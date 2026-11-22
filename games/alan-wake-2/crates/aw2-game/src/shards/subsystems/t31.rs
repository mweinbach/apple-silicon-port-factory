#![allow(unused_imports, unused_variables, unused_mut, unused_assignments, non_snake_case, dead_code, clippy::all, unreachable_code, unused_parens)]

#[inline(never)]
pub unsafe extern "win64" fn tpl_63880<const P0: u64, const P1: u64, const P2: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0; let mut ymm1_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 17>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x80u64)) as u64; r10 = (crate::simd_ops::ru64(r8)) as u64; rdi = rcx; rax = ((rdx as u32)) as u32 as u64; rdx = (crate::simd_ops::ru64(r10.wrapping_add(0x30u64))) as u64; rdx = (crate::simd_ops::ru64(rdx.wrapping_add(rax.wrapping_mul(8)))) as u64; rax = 0xffff; fl_l = ((rdx as u16)) as u16 as u64; fl_r = ((rax as u16)) as u16 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { bb = 2; continue; } bb = 1; continue; }
            1 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::lift_ops::trap(0x80000003, P2, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            2 => { rcx = (crate::simd_ops::ru64(r8.wrapping_add(0x10u64))) as u64; rax = (crate::simd_ops::ru64(r8.wrapping_add(0x8u64))) as u64; r11 = ((rcx.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = rcx; r9 = r11; r9 = ((r9).wrapping_sub(rcx)) as u64; r9 = ((((r9) as i64).wrapping_shr(1u32) as u64)) as u64; fl_l = r9; fl_r = r9; if (((((fl_l as u64)) & ((fl_r as u64))) as i64) <= 0) { bb = 8; continue; } bb = 3; continue; }
            3 => { bb = 4; continue; }
            4 => { r8 = r9; r8 = ((r8).wrapping_shr(1u32)) as u64; fl_l = (crate::simd_ops::ru16(rax.wrapping_add(r8.wrapping_mul(2)))) as u16 as u64; fl_r = ((rdx as u16)) as u16 as u64; rcx = ((rax.wrapping_add(r8.wrapping_mul(2))) as u64) as u64; if (((fl_l as u16)) >= ((fl_r as u16))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rax = (0xffffffffffffffffu64) as u64; rax = ((rax).wrapping_sub(r8)) as u64; fl_l = ((r9).wrapping_add(rax)) as u64; fl_r = fl_l; r9 = ((fl_l as u64)) as u64; rax = rcx.wrapping_add(0x2); bb = 7; continue; }
            6 => { r9 = r8; bb = 7; continue; }
            7 => { fl_l = r9; fl_r = r9; if (((((fl_l as u64)) & ((fl_r as u64))) as i64) > 0) { bb = 4; continue; } bb = 8; continue; }
            8 => { fl_l = rax; fl_r = r11; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 25; continue; } bb = 9; continue; }
            9 => { fl_l = ((rdx as u16)) as u16 as u64; fl_r = (crate::simd_ops::ru16(rax)) as u16 as u64; if (((fl_l as u16)) < ((fl_r as u16))) { bb = 25; continue; } bb = 10; continue; }
            10 => { r9 = (((rdx as u16)) as u32) as u32 as u64; r8 = r10.wrapping_add(0x40); crate::simd_ops::wu64(rsp.wrapping_add(0x90u64), rbx); rcx = rsp.wrapping_add(0x30); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdx); rbx = (crate::simd_ops::ru64(r8.wrapping_add(r9.wrapping_mul(8)).wrapping_add(0x48u64))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r11 = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; fl_l = r11; fl_r = r11; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 11; continue; }
            11 => { fl_l = ((r11).wrapping_add(rbx)) as u64; fl_r = fl_l; r11 = ((fl_l as u64)) as u64; bb = 12; continue; }
            12 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 14; continue; } bb = 13; continue; }
            13 => { fl_l = ((rax).wrapping_add(rbx)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; bb = 14; continue; }
            14 => { rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 16; continue; } bb = 15; continue; }
            15 => { fl_l = ((rcx).wrapping_add(rbx)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; bb = 16; continue; }
            16 => { rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 18; continue; } bb = 17; continue; }
            17 => { fl_l = ((rdx).wrapping_add(rbx)) as u64; fl_r = fl_l; rdx = ((fl_l as u64)) as u64; bb = 18; continue; }
            18 => { r8 = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 20; continue; } bb = 19; continue; }
            19 => { fl_l = ((r8).wrapping_add(rbx)) as u64; fl_r = fl_l; r8 = ((fl_l as u64)) as u64; bb = 20; continue; }
            20 => { r9 = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; fl_l = r9; fl_r = r9; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 22; continue; } bb = 21; continue; }
            21 => { fl_l = ((r9).wrapping_add(rbx)) as u64; fl_r = fl_l; r9 = ((fl_l as u64)) as u64; bb = 22; continue; }
            22 => { r10 = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; fl_l = r10; fl_r = r10; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 24; continue; } bb = 23; continue; }
            23 => { fl_l = ((r10).wrapping_add(rbx)) as u64; fl_r = fl_l; r10 = ((fl_l as u64)) as u64; bb = 24; continue; }
            24 => { crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rax); rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x24u64))) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), rbx); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x90u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), rax); rax = rdi; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), r11); crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rcx); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rdx); let y_addr: u64 = rsp.wrapping_add(0x30u64); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rdi; crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); xmm0 = ((crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64) as u128; ymm0_hi = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), r8); crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), r9); crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), r10); let y_addr: u64 = rsp.wrapping_add(0x50u64); xmm1 = crate::simd_ops::ru128(y_addr); ymm1_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rdi.wrapping_add(0x20u64); crate::simd_ops::wu128(y_addr, xmm1); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm1_hi); crate::simd_ops::wu64(rdi.wrapping_add(0x40u64), ((xmm0 as u64)) as u64); crate::simd_ops::wu8(rdi.wrapping_add(0x48u64), (0x1u8) as u8); rsp = ((rsp).wrapping_add(0x80u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            25 => { crate::simd_ops::wu8(rdi.wrapping_add(0x48u64), (0x0u8) as u8); rax = rdi; rsp = ((rsp).wrapping_add(0x80u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_63932<const P0: u64, const P1: u64, const P2: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 15>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { r11 = rsp; rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x70u64)) as u64; r14 = rdx; rdx = (crate::simd_ops::ru64(rcx)) as u64; rcx = ((r8 as u32)) as u32 as u64; rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x30u64))) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(rcx.wrapping_mul(8)))) as u64; rcx = 0xffff; fl_l = ((rax as u16)) as u16 as u64; fl_r = ((rcx as u16)) as u16 as u64; if (((fl_l as u16)) == ((fl_r as u16))) { bb = 20; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(r11.wrapping_add(0x10u64), rbx); r8 = rdx.wrapping_add(0x40); crate::simd_ops::wu64(r11.wrapping_add(0x18u64), rsi); rcx = r11.wrapping_sub(0x58); r9 = (((rax as u16)) as u32) as u32 as u64; crate::simd_ops::wu64(r11.wrapping_add(0x20u64), rdi); crate::simd_ops::wu64(rsp.wrapping_add(0x80u64), rax); rsi = (crate::simd_ops::ru64(r8.wrapping_add(r9.wrapping_mul(8)).wrapping_add(0x48u64))) as u64; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { fl_l = ((rdi).wrapping_add(rsi)) as u64; fl_r = fl_l; rdi = ((fl_l as u64)) as u64; bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { fl_l = ((rax).wrapping_add(rsi)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; bb = 5; continue; }
            5 => { rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 6; continue; }
            6 => { fl_l = ((rcx).wrapping_add(rsi)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; bb = 7; continue; }
            7 => { rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 8; continue; }
            8 => { fl_l = ((rdx).wrapping_add(rsi)) as u64; fl_r = fl_l; rdx = ((fl_l as u64)) as u64; bb = 9; continue; }
            9 => { r8 = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 10; continue; }
            10 => { fl_l = ((r8).wrapping_add(rsi)) as u64; fl_r = fl_l; r8 = ((fl_l as u64)) as u64; bb = 11; continue; }
            11 => { r9 = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; fl_l = r9; fl_r = r9; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 13; continue; } bb = 12; continue; }
            12 => { fl_l = ((r9).wrapping_add(rsi)) as u64; fl_r = fl_l; r9 = ((fl_l as u64)) as u64; bb = 13; continue; }
            13 => { r11 = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; fl_l = r11; fl_r = r11; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 15; continue; } bb = 14; continue; }
            14 => { fl_l = ((r11).wrapping_add(rsi)) as u64; fl_r = fl_l; r11 = ((fl_l as u64)) as u64; bb = 15; continue; }
            15 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 17; continue; } bb = 16; continue; }
            16 => { fl_l = ((rbx).wrapping_add(rsi)) as u64; fl_r = fl_l; rbx = ((fl_l as u64)) as u64; bb = 17; continue; }
            17 => { r10 = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; fl_l = r10; fl_r = r10; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 19; continue; } bb = 18; continue; }
            18 => { fl_l = ((r10).wrapping_add(rsi)) as u64; fl_r = fl_l; r10 = ((fl_l as u64)) as u64; bb = 19; continue; }
            19 => { crate::simd_ops::wu64(r14, rdi); crate::simd_ops::wu64(r14.wrapping_add(0x8u64), rax); rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x84u64))) as u32 as u64; crate::simd_ops::wu64(r14.wrapping_add(0x10u64), rcx); crate::simd_ops::wu64(r14.wrapping_add(0x18u64), rdx); crate::simd_ops::wu64(r14.wrapping_add(0x20u64), r8); crate::simd_ops::wu64(r14.wrapping_add(0x28u64), r9); crate::simd_ops::wu64(r14.wrapping_add(0x30u64), r11); crate::simd_ops::wu64(r14.wrapping_add(0x38u64), rbx); crate::simd_ops::wu64(r14.wrapping_add(0x40u64), r10); rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x98u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x88u64))) as u64; crate::simd_ops::wu64(r14.wrapping_add(0x48u64), rsi); rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x90u64))) as u64; crate::simd_ops::wu64(r14.wrapping_add(0x50u64), rax); rax = r14; rsp = ((rsp).wrapping_add(0x70u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            20 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, P2, rcx, rsp); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_63972<const P0: u64, const P1: u64, const P2: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rdi = rcx;
    rcx = ((rcx).wrapping_add(0x18u64)) as u64;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rdx = (crate::simd_ops::ru64(rdi.wrapping_add(P1))) as u64;
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(P1))) as u64;
    crate::simd_ops::wu64(rax, (0x0u64) as u64);
    rdx = (crate::simd_ops::ru64(rdx)) as u64;
    fl_l = rdx; fl_r = rdx;
    if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { rdx = (crate::simd_ops::ru64(rdi.wrapping_add(P1))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(P2, rax, rcx, rdx, r8, r9, rsp); }
    crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbx);
    loop { rbx = (crate::simd_ops::ru64(rdx)) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rdx = rbx; fl_l = rbx; fl_r = rbx; if !(((((fl_l as u64)) & ((fl_r as u64))) != 0)) { break; } }
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rdx = (crate::simd_ops::ru64(rdi.wrapping_add(P1))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return crate::simd_ops::tail_stk(P2, rax, rcx, rdx, r8, r9, rsp);
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_64008<const P0: u64, const P1: u64, const P2: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 33>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { r11 = rsp; rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rbp = r11.wrapping_sub(0x5f); rsp = ((rsp).wrapping_sub(0xf8u64)) as u64; rax = (((r8 as u16)) as u32) as u32 as u64; rbx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = (crate::simd_ops::ru64(rdx)) as u64; rbx = ((rbx).wrapping_add(rbx)) as u64; r9 = (crate::simd_ops::ru32(rax.wrapping_add(rbx.wrapping_mul(8)).wrapping_add(0x8u64))) as u32 as u64; rbx = (crate::simd_ops::ru64(rax.wrapping_add(rbx.wrapping_mul(8)))) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x7fu64), rbx); fl_l = r9; fl_r = r9; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 15; continue; } bb = 1; continue; }
            1 => { r10 = (crate::simd_ops::ru64(rcx)) as u64; rdx = ((crate::simd_ops::ru16(P0)) as u32) as u32 as u64; crate::simd_ops::wu64(r11.wrapping_add(0x8u64), rsi); rsi = ((rbx.wrapping_add(r9.wrapping_mul(2))) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x6fu64), r10); rax = (crate::simd_ops::ru64(r10)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), r10); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); rcx = (crate::simd_ops::ru64(rax.wrapping_add(0x20u64))) as u64; rcx = ((crate::simd_ops::ru16(rcx.wrapping_add(rdx.wrapping_mul(2)))) as u32) as u32 as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(r10.wrapping_add(0x8u64)))) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x8u64))) as u32 as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x79u64), rax); rax = (crate::simd_ops::ru64(rcx)) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x71u64), rax); fl_l = rbx; fl_r = rsi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 14; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::wu64(r11.wrapping_sub(0x18u64), rdi); crate::simd_ops::wu64(r11.wrapping_sub(0x20u64), r12); crate::simd_ops::wu64(r11.wrapping_sub(0x28u64), r13); crate::simd_ops::wu64(r11.wrapping_sub(0x30u64), r14); crate::simd_ops::wu64(r11.wrapping_sub(0x38u64), r15); bb = 3; continue; }
            3 => { rax = ((crate::simd_ops::ru16(rbx)) as u32) as u32 as u64; r12 = (crate::simd_ops::ru64(r10.wrapping_add(rax.wrapping_mul(8)).wrapping_add(0x88u64))) as u64; fl_l = r12; fl_r = r12; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rdx = (crate::simd_ops::ru32(r10.wrapping_add(rax.wrapping_mul(4)).wrapping_add(0x10088u64))) as u32 as u64; bb = 6; continue; }
            5 => { rdx = 0; fl_l = 0; fl_r = 0; bb = 6; continue; }
            6 => { r15 = ((rdx as u32)) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 12; continue; } bb = 7; continue; }
            7 => { crate::simd_ops::wu16(rbp.wrapping_sub(0x69u64), ((rax as u16)) as u16); rdx = rbp.wrapping_add(0x7); crate::simd_ops::wu64(rbp.wrapping_sub(0x61u64), r12); rcx = rbp.wrapping_sub(0x69); crate::simd_ops::wu64(rbp.wrapping_sub(0x59u64), r10); crate::simd_ops::wu64(rbp.wrapping_sub(0x51u64), r15); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r14 = 0; fl_l = 0; fl_r = 0; let y_addr: u64 = rax; xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rbp.wrapping_sub(0x49u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); fl_l = r15; fl_r = r15; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 8; continue; }
            8 => { r13 = (crate::simd_ops::ru64(rbp.wrapping_sub(0x31u64))) as u64; rdi = (crate::simd_ops::ru64(rbp.wrapping_sub(0x49u64))) as u64; rsi = (crate::simd_ops::ru64(rbp.wrapping_sub(0x39u64))) as u64; rbx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x41u64))) as u64; bb = 9; continue; }
            9 => { xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x28u64))) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rbp.wrapping_add(0x7u64), (xmm0) as u128); xmm0 = ((crate::simd_ops::ru64(rbp.wrapping_sub(0x71u64))) as u64) as u128; ymm0_hi = 0; crate::simd_ops::wu64(rbp.wrapping_add(0x17u64), ((xmm0 as u64)) as u64); crate::simd_ops::wu64(rbp.wrapping_sub(0x29u64), rdi); rdx = rbp.wrapping_add(0x7); crate::simd_ops::wu64(rbp.wrapping_sub(0x21u64), rbx); rcx = rbp.wrapping_sub(0x69); crate::simd_ops::wu64(rbp.wrapping_sub(0x19u64), rsi); crate::simd_ops::wu64(rbp.wrapping_sub(0x11u64), r13); let y_addr: u64 = rbp.wrapping_sub(0x29u64); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); crate::simd_ops::wu64(rbp.wrapping_sub(0x9u64), r12); crate::simd_ops::wu64(rbp.wrapping_sub(0x1u64), r14); xmm1 = (crate::simd_ops::ru128(rbp.wrapping_sub(0x9u64))) as u128; let y_addr: u64 = rbp.wrapping_sub(0x69u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); crate::simd_ops::wu128(rbp.wrapping_sub(0x49u64), (xmm1) as u128); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r14 = ((r14).wrapping_add(1)) as u64; fl_l = r14; fl_r = r15; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 9; continue; } bb = 10; continue; }
            10 => { rbx = (crate::simd_ops::ru64(rbp.wrapping_add(0x7fu64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; bb = 11; continue; }
            11 => { r10 = (crate::simd_ops::ru64(rbp.wrapping_add(0x6fu64))) as u64; bb = 12; continue; }
            12 => { rbx = ((rbx).wrapping_add(0x2u64)) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x7fu64), rbx); fl_l = rbx; fl_r = rsi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } bb = 13; continue; }
            13 => { r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0xd0u64))) as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0xd8u64))) as u64; r13 = (crate::simd_ops::ru64(rsp.wrapping_add(0xe0u64))) as u64; r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0xe8u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0xf0u64))) as u64; bb = 14; continue; }
            14 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x110u64))) as u64; bb = 15; continue; }
            15 => { rsp = ((rsp).wrapping_add(0xf8u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_64022<const P0: u64, const P1: u64, const P2: u64, const P3: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 15>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x68u64)) as u64; rbx = (crate::simd_ops::ru64(rcx)) as u64; rdi = 0; fl_l = 0; fl_r = 0; rax = (((rdx as u16)) as u32) as u32 as u64; r8 = (crate::simd_ops::ru64(rbx.wrapping_add(rax.wrapping_mul(8)).wrapping_add(P0))) as u64; fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(rax.wrapping_mul(4)).wrapping_add(P1))) as u32 as u64; bb = 3; continue; }
            2 => { rax = ((rdi as u32)) as u32 as u64; bb = 3; continue; }
            3 => { r9 = ((rax as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 9; continue; } bb = 4; continue; }
            4 => { rcx = rsp.wrapping_add(0x30); crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), r15); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbx); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; fl_l = r15; fl_r = r15; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 5; continue; }
            5 => { crate::simd_ops::wu64(rsp.wrapping_add(0x80u64), rbp); rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x88u64), rsi); rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x90u64), r14); r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; bb = 6; continue; }
            6 => { crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbp); rcx = rsp.wrapping_add(0x30); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rsi); rdx = rbx; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), r14); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rdi); let y_addr: u64 = rsp.wrapping_add(0x30u64); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rsp.wrapping_add(0x30u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdi = ((rdi).wrapping_add(1)) as u64; fl_l = rdi; fl_r = r15; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 6; continue; } bb = 7; continue; }
            7 => { r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x90u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x88u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; bb = 8; continue; }
            8 => { r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; bb = 9; continue; }
            9 => { rsp = ((rsp).wrapping_add(0x68u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_64023<const P0: u64, const P1: u64, const P2: u64, const P3: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { r11 = rsp; rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x60u64)) as u64; rax = (((r8 as u16)) as u32) as u32 as u64; rdi = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = (crate::simd_ops::ru64(rdx)) as u64; rdi = ((rdi).wrapping_add(rdi)) as u64; rdx = (crate::simd_ops::ru32(rax.wrapping_add(rdi.wrapping_mul(8)).wrapping_add(0x8u64))) as u32 as u64; rdi = (crate::simd_ops::ru64(rax.wrapping_add(rdi.wrapping_mul(8)))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 19; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(r11.wrapping_add(0x10u64), rbp); rbp = ((rdi.wrapping_add(rdx.wrapping_mul(2))) as u64) as u64; crate::simd_ops::wu64(r11.wrapping_add(0x18u64), rsi); rsi = (crate::simd_ops::ru64(rcx)) as u64; fl_l = rdi; fl_r = rbp; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 18; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::wu64(r11.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(r11.wrapping_sub(0x10u64), r12); crate::simd_ops::wu64(r11.wrapping_sub(0x18u64), r13); crate::simd_ops::wu64(r11.wrapping_sub(0x20u64), r14); crate::simd_ops::wu64(r11.wrapping_sub(0x28u64), r15); bb = 3; continue; }
            3 => { r11 = ((crate::simd_ops::ru16(rdi)) as u32) as u32 as u64; r12 = (crate::simd_ops::ru64(rsi.wrapping_add(r11.wrapping_mul(8)).wrapping_add(0x88u64))) as u64; fl_l = r12; fl_r = r12; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru32(rsi.wrapping_add(r11.wrapping_mul(4)).wrapping_add(0x10088u64))) as u32 as u64; bb = 6; continue; }
            5 => { rax = 0; fl_l = 0; fl_r = 0; bb = 6; continue; }
            6 => { r13 = ((rax as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 16; continue; } bb = 7; continue; }
            7 => { r8 = ((crate::simd_ops::ru16(P0)) as u32) as u32 as u64; rax = r11; rax = ((rax).wrapping_shl(5u32)) as u64; r14 = (crate::simd_ops::ru32(rax.wrapping_add(rsi).wrapping_add(0x180acu64))) as u32 as u64; r15 = (crate::simd_ops::ru64(rax.wrapping_add(rsi).wrapping_add(0x18098u64))) as u64; rdx = ((r14 as u32)) as u32 as u64; rcx = r15; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = ((((rax as u32)) as i32 as i64) as u64) as u64; fl_l = rcx; fl_r = (0xffffffffffffffffu64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 9; continue; } bb = 8; continue; }
            8 => { rax = r11.wrapping_add(0xc03); fl_l = ((rax).wrapping_shl(5u32)) as u64; rax = ((fl_l as u64)) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(rsi).wrapping_add(0x40u64))) as u64; rbx = (crate::simd_ops::ru32(rax.wrapping_add(rcx.wrapping_mul(4)))) as u32 as u64; bb = 10; continue; }
            9 => { rbx = 0; fl_l = 0; fl_r = 0; bb = 10; continue; }
            10 => { r8 = ((crate::simd_ops::ru16(P2)) as u32) as u32 as u64; rax = 0xc03; rdx = r14; rcx = r15; r11 = ((r11).wrapping_add(rax)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = ((((rax as u32)) as i32 as i64) as u64) as u64; fl_l = rcx; fl_r = (0xffffffffffffffffu64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 12; continue; } bb = 11; continue; }
            11 => { fl_l = ((r11).wrapping_shl(5u32)) as u64; r11 = ((fl_l as u64)) as u64; rax = (crate::simd_ops::ru64(r11.wrapping_add(rsi).wrapping_add(0x40u64))) as u64; r15 = (crate::simd_ops::ru32(rax.wrapping_add(rcx.wrapping_mul(4)))) as u32 as u64; bb = 13; continue; }
            12 => { r15 = 0; fl_l = 0; fl_r = 0; bb = 13; continue; }
            13 => { fl_l = rbx; fl_r = rbx; rax = ((rbx.wrapping_add(r12)) as u64) as u64; rbx = (if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { rax } else { rbx }) as u64; fl_l = r15; fl_r = r15; rax = ((r15.wrapping_add(r12)) as u64) as u64; r15 = (if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { rax } else { r15 }) as u64; r14 = 0; fl_l = 0; fl_r = 0; fl_l = r13; fl_r = r13; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 16; continue; } bb = 14; continue; }
            14 => { bb = 15; continue; }
            15 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbx); rcx = rsp.wrapping_add(0x20); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), r15); rdx = rsi; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), r12); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), r14); let y_addr: u64 = rsp.wrapping_add(0x20u64); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rsp.wrapping_add(0x20u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r14 = ((r14).wrapping_add(1)) as u64; fl_l = r14; fl_r = r13; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 15; continue; } bb = 16; continue; }
            16 => { rdi = ((rdi).wrapping_add(0x2u64)) as u64; fl_l = rdi; fl_r = rbp; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } bb = 17; continue; }
            17 => { r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; r13 = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; bb = 18; continue; }
            18 => { rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; bb = 19; continue; }
            19 => { rsp = ((rsp).wrapping_add(0x60u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_64030<const P0: u64, const P1: u64, const P2: u64, const P3: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 23>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = ((rsp).wrapping_sub(0xa8u64)) as u64; rbx = (crate::simd_ops::ru64(rcx)) as u64; rsi = 0; fl_l = 0; fl_r = 0; rax = (((rdx as u16)) as u32) as u32 as u64; r8 = (crate::simd_ops::ru64(rbx.wrapping_add(rax.wrapping_mul(8)).wrapping_add(P0))) as u64; fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(rax.wrapping_mul(4)).wrapping_add(P1))) as u32 as u64; bb = 3; continue; }
            2 => { rax = ((rsi as u32)) as u32 as u64; bb = 3; continue; }
            3 => { r9 = ((rax as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 9; continue; } bb = 4; continue; }
            4 => { rcx = rsp.wrapping_add(0x60); crate::simd_ops::wu64(rsp.wrapping_add(0xd0u64), r12); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbx); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; fl_l = r12; fl_r = r12; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 5; continue; }
            5 => { crate::simd_ops::wu64(rsp.wrapping_add(0xc0u64), rbp); rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x90u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xc8u64), rdi); rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xd8u64), r14); r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x88u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xa0u64), r15); r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; bb = 6; continue; }
            6 => { crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rdi); rcx = rsp.wrapping_add(0x60); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), r14); rdx = rbx; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), r15); let y_addr: u64 = rsp.wrapping_add(0x30u64); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rsi); xmm1 = ((crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64) as u128; let y_addr: u64 = rsp.wrapping_add(0x60u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); crate::simd_ops::wu64(rsp.wrapping_add(0x80u64), ((xmm1 as u64)) as u64); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rsi = ((rsi).wrapping_add(1)) as u64; fl_l = rsi; fl_r = r12; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 6; continue; } bb = 7; continue; }
            7 => { r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0xa0u64))) as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0xd8u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0xc8u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0xc0u64))) as u64; bb = 8; continue; }
            8 => { r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0xd0u64))) as u64; bb = 9; continue; }
            9 => { rsp = ((rsp).wrapping_add(0xa8u64)) as u64; rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_64033<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 23>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { r11 = rsp; rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0xb0u64)) as u64; rax = (((r8 as u16)) as u32) as u32 as u64; rdi = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = (crate::simd_ops::ru64(rdx)) as u64; rdi = ((rdi).wrapping_add(rdi)) as u64; rdx = (crate::simd_ops::ru32(rax.wrapping_add(rdi.wrapping_mul(8)).wrapping_add(0x8u64))) as u32 as u64; rdi = (crate::simd_ops::ru64(rax.wrapping_add(rdi.wrapping_mul(8)))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 23; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(r11.wrapping_sub(0x10u64), rbp); rbp = ((rdi.wrapping_add(rdx.wrapping_mul(2))) as u64) as u64; crate::simd_ops::wu64(r11.wrapping_sub(0x18u64), rsi); rsi = (crate::simd_ops::ru64(rcx)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xd8u64), rbp); fl_l = rdi; fl_r = rbp; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 22; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::wu64(r11.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(r11.wrapping_sub(0x20u64), r12); crate::simd_ops::wu64(r11.wrapping_sub(0x28u64), r13); crate::simd_ops::wu64(r11.wrapping_sub(0x30u64), r14); crate::simd_ops::wu64(r11.wrapping_sub(0x38u64), r15); bb = 3; continue; }
            3 => { r11 = ((crate::simd_ops::ru16(rdi)) as u32) as u32 as u64; r13 = (crate::simd_ops::ru64(rsi.wrapping_add(r11.wrapping_mul(8)).wrapping_add(0x88u64))) as u64; fl_l = r13; fl_r = r13; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru32(rsi.wrapping_add(r11.wrapping_mul(4)).wrapping_add(0x10088u64))) as u32 as u64; bb = 6; continue; }
            5 => { rax = 0; fl_l = 0; fl_r = 0; bb = 6; continue; }
            6 => { crate::simd_ops::wu64(rsp.wrapping_add(0xc8u64), rax); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 20; continue; } bb = 7; continue; }
            7 => { r8 = ((crate::simd_ops::ru16(P0)) as u32) as u32 as u64; rax = r11; rax = ((rax).wrapping_shl(5u32)) as u64; r14 = (crate::simd_ops::ru32(rax.wrapping_add(rsi).wrapping_add(0x180acu64))) as u32 as u64; r15 = (crate::simd_ops::ru64(rax.wrapping_add(rsi).wrapping_add(0x18098u64))) as u64; rdx = ((r14 as u32)) as u32 as u64; rcx = r15; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = ((((rax as u32)) as i32 as i64) as u64) as u64; fl_l = rcx; fl_r = (0xffffffffffffffffu64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 9; continue; } bb = 8; continue; }
            8 => { rax = r11.wrapping_add(0xc03); fl_l = ((rax).wrapping_shl(5u32)) as u64; rax = ((fl_l as u64)) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(rsi).wrapping_add(0x40u64))) as u64; rbx = (crate::simd_ops::ru32(rax.wrapping_add(rcx.wrapping_mul(4)))) as u32 as u64; bb = 10; continue; }
            9 => { rbx = 0; fl_l = 0; fl_r = 0; bb = 10; continue; }
            10 => { r8 = ((crate::simd_ops::ru16(P2)) as u32) as u32 as u64; rax = 0xc03; rdx = r14; rcx = r15; r11 = ((r11).wrapping_add(rax)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = ((((rax as u32)) as i32 as i64) as u64) as u64; fl_l = rcx; fl_r = (0xffffffffffffffffu64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 12; continue; } bb = 11; continue; }
            11 => { fl_l = ((r11).wrapping_shl(5u32)) as u64; r11 = ((fl_l as u64)) as u64; rax = (crate::simd_ops::ru64(r11.wrapping_add(rsi).wrapping_add(0x40u64))) as u64; r12 = (crate::simd_ops::ru32(rax.wrapping_add(rcx.wrapping_mul(4)))) as u32 as u64; bb = 13; continue; }
            12 => { r12 = 0; fl_l = 0; fl_r = 0; fl_l = ((r11).wrapping_shl(5u32)) as u64; r11 = ((fl_l as u64)) as u64; bb = 13; continue; }
            13 => { r8 = ((crate::simd_ops::ru16(P3)) as u32) as u32 as u64; rdx = r14; rcx = r15; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = ((((rax as u32)) as i32 as i64) as u64) as u64; fl_l = rcx; fl_r = (0xffffffffffffffffu64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 15; continue; } bb = 14; continue; }
            14 => { rax = (crate::simd_ops::ru64(r11.wrapping_add(rsi).wrapping_add(0x40u64))) as u64; r15 = (crate::simd_ops::ru32(rax.wrapping_add(rcx.wrapping_mul(4)))) as u32 as u64; bb = 16; continue; }
            15 => { r15 = 0; fl_l = 0; fl_r = 0; bb = 16; continue; }
            16 => { fl_l = rbx; fl_r = rbx; rax = ((rbx.wrapping_add(r13)) as u64) as u64; rbx = (if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { rax } else { rbx }) as u64; fl_l = r12; fl_r = r12; rax = ((r12.wrapping_add(r13)) as u64) as u64; r12 = (if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { rax } else { r12 }) as u64; fl_l = r15; fl_r = r15; rax = ((r15.wrapping_add(r13)) as u64) as u64; r15 = (if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { rax } else { r15 }) as u64; r14 = 0; fl_l = 0; fl_r = 0; fl_l = (crate::simd_ops::ru64(rsp.wrapping_add(0xc8u64))) as u64; fl_r = r14; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 20; continue; } bb = 17; continue; }
            17 => { rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0xc8u64))) as u64; bb = 18; continue; }
            18 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbx); rcx = rsp.wrapping_add(0x50); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), r12); rdx = rsi; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), r15); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), r13); let y_addr: u64 = rsp.wrapping_add(0x20u64); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), r14); xmm1 = ((crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64) as u128; let y_addr: u64 = rsp.wrapping_add(0x50u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), ((xmm1 as u64)) as u64); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r14 = ((r14).wrapping_add(1)) as u64; fl_l = r14; fl_r = rbp; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 18; continue; } bb = 19; continue; }
            19 => { rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0xd8u64))) as u64; bb = 20; continue; }
            20 => { rdi = ((rdi).wrapping_add(0x2u64)) as u64; fl_l = rdi; fl_r = rbp; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } bb = 21; continue; }
            21 => { r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x88u64))) as u64; r13 = (crate::simd_ops::ru64(rsp.wrapping_add(0x90u64))) as u64; r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x98u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0xc0u64))) as u64; bb = 22; continue; }
            22 => { rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0xa8u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0xa0u64))) as u64; bb = 23; continue; }
            23 => { rsp = ((rsp).wrapping_add(0xb0u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_64043<const P0: u64, const P1: u64, const P2: u64, const P3: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 29>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rbp = rsp.wrapping_sub(0x47); rsp = ((rsp).wrapping_sub(0xd0u64)) as u64; r8 = (crate::simd_ops::ru64(rcx)) as u64; rbx = 0; fl_l = 0; fl_r = 0; rax = (((rdx as u16)) as u32) as u32 as u64; rdi = (crate::simd_ops::ru64(r8.wrapping_add(rax.wrapping_mul(8)).wrapping_add(0x88u64))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru32(r8.wrapping_add(rax.wrapping_mul(4)).wrapping_add(0x10088u64))) as u32 as u64; bb = 3; continue; }
            2 => { rax = ((rbx as u32)) as u32 as u64; bb = 3; continue; }
            3 => { r9 = ((rax as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 9; continue; } bb = 4; continue; }
            4 => { r11 = (crate::simd_ops::ru64(r8.wrapping_add(0x8u64))) as u64; rax = (crate::simd_ops::ru64(r8)) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x11u64), r8); crate::simd_ops::wu64(rbp.wrapping_sub(0x29u64), r8); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r8); r8 = rdi; r10 = (crate::simd_ops::ru64(rax.wrapping_add(0x20u64))) as u64; rax = ((crate::simd_ops::ru16(P0)) as u32) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x100u64), r15); rcx = ((crate::simd_ops::ru16(r10.wrapping_add(rax.wrapping_mul(2)))) as u32) as u32 as u64; rcx = ((rcx).wrapping_add(rcx)) as u64; rax = (crate::simd_ops::ru32(r11.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x8u64))) as u32 as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x9u64), rax); rax = (crate::simd_ops::ru64(r11.wrapping_add(rcx.wrapping_mul(8)))) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x1u64), rax); rax = ((crate::simd_ops::ru16(P1)) as u32) as u32 as u64; rcx = ((crate::simd_ops::ru16(r10.wrapping_add(rax.wrapping_mul(2)))) as u32) as u32 as u64; rcx = ((rcx).wrapping_add(rcx)) as u64; rax = (crate::simd_ops::ru32(r11.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x8u64))) as u32 as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x21u64), rax); rax = (crate::simd_ops::ru64(r11.wrapping_add(rcx.wrapping_mul(8)))) as u64; rcx = rbp.wrapping_sub(0x59); crate::simd_ops::wu64(rbp.wrapping_sub(0x19u64), rax); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r15 = (crate::simd_ops::ru64(rbp.wrapping_sub(0x41u64))) as u64; fl_l = r15; fl_r = r15; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 5; continue; }
            5 => { rdi = (crate::simd_ops::ru64(rbp.wrapping_sub(0x31u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xf0u64), rsi); rsi = (crate::simd_ops::ru64(rbp.wrapping_sub(0x39u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xf8u64), r14); r14 = (crate::simd_ops::ru64(rbp.wrapping_sub(0x51u64))) as u64; bb = 6; continue; }
            6 => { xmm0 = (crate::simd_ops::ru128(rbp.wrapping_sub(0x29u64))) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rbp.wrapping_add(0x7u64), (xmm0) as u128); xmm0 = ((crate::simd_ops::ru64(rbp.wrapping_sub(0x19u64))) as u64) as u128; ymm0_hi = 0; crate::simd_ops::wu64(rbp.wrapping_add(0x17u64), ((xmm0 as u64)) as u64); xmm0 = (crate::simd_ops::ru128(rbp.wrapping_sub(0x11u64))) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rbp.wrapping_add(0x27u64), (xmm0) as u128); xmm0 = ((crate::simd_ops::ru64(rbp.wrapping_sub(0x1u64))) as u64) as u128; ymm0_hi = 0; crate::simd_ops::wu64(rbp.wrapping_add(0x37u64), ((xmm0 as u64)) as u64); crate::simd_ops::wu64(rbp.wrapping_sub(0x59u64), rsi); r8 = rbp.wrapping_add(0x7); crate::simd_ops::wu64(rbp.wrapping_sub(0x51u64), rdi); rdx = rbp.wrapping_add(0x27); crate::simd_ops::wu64(rbp.wrapping_sub(0x49u64), r14); rcx = rbp.wrapping_sub(0x59); crate::simd_ops::wu64(rbp.wrapping_sub(0x41u64), rbx); let y_addr: u64 = rbp.wrapping_sub(0x59u64); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rbp.wrapping_sub(0x59u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = ((rbx).wrapping_add(1)) as u64; fl_l = rbx; fl_r = r15; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 6; continue; } bb = 7; continue; }
            7 => { r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0xf8u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0xf0u64))) as u64; bb = 8; continue; }
            8 => { r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x100u64))) as u64; bb = 9; continue; }
            9 => { rsp = ((rsp).wrapping_add(0xd0u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_64047<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 27>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { r11 = rsp; rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0xd0u64)) as u64; rax = (((r8 as u16)) as u32) as u32 as u64; rdi = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = (crate::simd_ops::ru64(rdx)) as u64; rdi = ((rdi).wrapping_add(rdi)) as u64; r10 = (crate::simd_ops::ru32(rax.wrapping_add(rdi.wrapping_mul(8)).wrapping_add(0x8u64))) as u32 as u64; rdi = (crate::simd_ops::ru64(rax.wrapping_add(rdi.wrapping_mul(8)))) as u64; fl_l = r10; fl_r = r10; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 18; continue; } bb = 1; continue; }
            1 => { r9 = (crate::simd_ops::ru64(rcx)) as u64; crate::simd_ops::wu64(r11.wrapping_add(0x18u64), rsi); rsi = ((rdi.wrapping_add(r10.wrapping_mul(2))) as u64) as u64; crate::simd_ops::wu64(r11.wrapping_sub(0x18u64), r13); crate::simd_ops::wu64(r11.wrapping_sub(0x80u64), r9); r8 = (crate::simd_ops::ru64(r9.wrapping_add(0x8u64))) as u64; r13 = r9.wrapping_add(0x40); rax = (crate::simd_ops::ru64(r9)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), r9); rdx = (crate::simd_ops::ru64(rax.wrapping_add(0x20u64))) as u64; rax = ((crate::simd_ops::ru16(P0)) as u32) as u32 as u64; rcx = ((crate::simd_ops::ru16(rdx.wrapping_add(rax.wrapping_mul(2)))) as u32) as u32 as u64; rcx = ((rcx).wrapping_add(rcx)) as u64; rax = (crate::simd_ops::ru32(r8.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x8u64))) as u32 as u64; crate::simd_ops::wu64(r11.wrapping_sub(0x78u64), rax); rax = (crate::simd_ops::ru64(r8.wrapping_add(rcx.wrapping_mul(8)))) as u64; crate::simd_ops::wu64(r11.wrapping_sub(0x70u64), rax); rax = ((crate::simd_ops::ru16(P1)) as u32) as u32 as u64; rcx = ((crate::simd_ops::ru16(rdx.wrapping_add(rax.wrapping_mul(2)))) as u32) as u32 as u64; rcx = ((rcx).wrapping_add(rcx)) as u64; rax = (crate::simd_ops::ru32(r8.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x8u64))) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rax); rax = (crate::simd_ops::ru64(r8.wrapping_add(rcx.wrapping_mul(8)))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rax); fl_l = rdi; fl_r = rsi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 17; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::wu64(r11.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(r11.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(r11.wrapping_sub(0x10u64), r12); crate::simd_ops::wu64(r11.wrapping_sub(0x20u64), r14); crate::simd_ops::wu64(r11.wrapping_sub(0x28u64), r15); bb = 3; continue; }
            3 => { r11 = ((crate::simd_ops::ru16(rdi)) as u32) as u32 as u64; r15 = (crate::simd_ops::ru64(r13.wrapping_add(r11.wrapping_mul(8)).wrapping_add(0x48u64))) as u64; fl_l = r15; fl_r = r15; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru32(r13.wrapping_add(r11.wrapping_mul(4)).wrapping_add(0x10048u64))) as u32 as u64; bb = 6; continue; }
            5 => { rax = 0; fl_l = 0; fl_r = 0; bb = 6; continue; }
            6 => { r12 = ((rax as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 15; continue; } bb = 7; continue; }
            7 => { r8 = ((crate::simd_ops::ru16(P2)) as u32) as u32 as u64; rax = r11; rax = ((rax).wrapping_shl(5u32)) as u64; rbp = (crate::simd_ops::ru32(rax.wrapping_add(r13).wrapping_add(0x1806cu64))) as u32 as u64; r14 = (crate::simd_ops::ru64(rax.wrapping_add(r13).wrapping_add(0x18058u64))) as u64; rdx = ((rbp as u32)) as u32 as u64; rcx = r14; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = ((((rax as u32)) as i32 as i64) as u64) as u64; fl_l = rcx; fl_r = (0xffffffffffffffffu64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 9; continue; } bb = 8; continue; }
            8 => { rax = r11.wrapping_add(0xc03); fl_l = ((rax).wrapping_shl(5u32)) as u64; rax = ((fl_l as u64)) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(r13))) as u64; rbx = (crate::simd_ops::ru32(rax.wrapping_add(rcx.wrapping_mul(4)))) as u32 as u64; bb = 10; continue; }
            9 => { rbx = 0; fl_l = 0; fl_r = 0; bb = 10; continue; }
            10 => { r8 = ((crate::simd_ops::ru16(P4)) as u32) as u32 as u64; rax = 0xc03; rdx = rbp; rcx = r14; r11 = ((r11).wrapping_add(rax)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = ((((rax as u32)) as i32 as i64) as u64) as u64; fl_l = rcx; fl_r = (0xffffffffffffffffu64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 12; continue; } bb = 11; continue; }
            11 => { fl_l = ((r11).wrapping_shl(5u32)) as u64; r11 = ((fl_l as u64)) as u64; rax = (crate::simd_ops::ru64(r11.wrapping_add(r13))) as u64; r14 = (crate::simd_ops::ru32(rax.wrapping_add(rcx.wrapping_mul(4)))) as u32 as u64; bb = 13; continue; }
            12 => { r14 = 0; fl_l = 0; fl_r = 0; bb = 13; continue; }
            13 => { fl_l = rbx; fl_r = rbx; rax = ((r15.wrapping_add(rbx)) as u64) as u64; rbx = (if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { rax } else { rbx }) as u64; fl_l = r14; fl_r = r14; rax = ((r15.wrapping_add(r14)) as u64) as u64; r14 = (if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { rax } else { r14 }) as u64; rbp = 0; fl_l = 0; fl_r = 0; fl_l = r12; fl_r = r12; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 15; continue; } bb = 14; continue; }
            14 => { xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x40u64))) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rsp.wrapping_add(0x70u64), (xmm0) as u128); xmm0 = ((crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64) as u128; ymm0_hi = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x80u64), ((xmm0 as u64)) as u64); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x58u64))) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rsp.wrapping_add(0x90u64), (xmm0) as u128); xmm0 = ((crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64) as u128; ymm0_hi = 0; crate::simd_ops::wu64(rsp.wrapping_add(0xa0u64), ((xmm0 as u64)) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbx); r8 = rsp.wrapping_add(0x70); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), r14); rdx = rsp.wrapping_add(0x90); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), r15); rcx = rsp.wrapping_add(0x20); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rbp); let y_addr: u64 = rsp.wrapping_add(0x20u64); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rsp.wrapping_add(0x20u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbp = ((rbp).wrapping_add(1)) as u64; fl_l = rbp; fl_r = r12; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 14; continue; } bb = 15; continue; }
            15 => { rdi = ((rdi).wrapping_add(0x2u64)) as u64; fl_l = rdi; fl_r = rsi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } bb = 16; continue; }
            16 => { r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0xb0u64))) as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0xb8u64))) as u64; r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0xc8u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0xe8u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0xe0u64))) as u64; bb = 17; continue; }
            17 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0xf0u64))) as u64; r13 = (crate::simd_ops::ru64(rsp.wrapping_add(0xc0u64))) as u64; bb = 18; continue; }
            18 => { rsp = ((rsp).wrapping_add(0xd0u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_64057<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 15>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x70u64)) as u64; r8 = (crate::simd_ops::ru64(rcx)) as u64; rbx = 0; fl_l = 0; fl_r = 0; rax = (((rdx as u16)) as u32) as u32 as u64; r10 = (crate::simd_ops::ru64(r8.wrapping_add(rax.wrapping_mul(8)).wrapping_add(P0))) as u64; fl_l = r10; fl_r = r10; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru32(r8.wrapping_add(rax.wrapping_mul(4)).wrapping_add(P1))) as u32 as u64; bb = 3; continue; }
            2 => { rax = ((rbx as u32)) as u32 as u64; bb = 3; continue; }
            3 => { r9 = ((rax as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 9; continue; } bb = 4; continue; }
            4 => { rcx = ((crate::simd_ops::ru16(P2)) as u32) as u32 as u64; rax = (crate::simd_ops::ru64(r8.wrapping_add(P3))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r8); r8 = r10; crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), r14); crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), r15); r15 = (crate::simd_ops::ru64(rax.wrapping_add(rcx.wrapping_mul(8)))) as u64; rcx = rsp.wrapping_add(0x30); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; fl_l = r14; fl_r = r14; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 5; continue; }
            5 => { crate::simd_ops::wu64(rsp.wrapping_add(0x80u64), rbp); rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x88u64), rsi); rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x90u64), rdi); rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; bb = 6; continue; }
            6 => { crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rsi); rcx = rsp.wrapping_add(0x30); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rdi); rdx = r15; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rbx); let y_addr: u64 = rsp.wrapping_add(0x30u64); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rsp.wrapping_add(0x30u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = ((rbx).wrapping_add(1)) as u64; fl_l = rbx; fl_r = r14; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 6; continue; } bb = 7; continue; }
            7 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x90u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x88u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; bb = 8; continue; }
            8 => { r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; bb = 9; continue; }
            9 => { rsp = ((rsp).wrapping_add(0x70u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_64074<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 15>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { r11 = rsp; rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x70u64)) as u64; rax = (((r8 as u16)) as u32) as u32 as u64; rdi = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = (crate::simd_ops::ru64(rdx)) as u64; rdi = ((rdi).wrapping_add(rdi)) as u64; rdx = (crate::simd_ops::ru32(rax.wrapping_add(rdi.wrapping_mul(8)).wrapping_add(0x8u64))) as u32 as u64; rdi = (crate::simd_ops::ru64(rax.wrapping_add(rdi.wrapping_mul(8)))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 20; continue; } bb = 1; continue; }
            1 => { rcx = (crate::simd_ops::ru64(rcx)) as u64; rax = ((crate::simd_ops::ru16(P0)) as u32) as u32 as u64; crate::simd_ops::wu64(r11.wrapping_sub(0x10u64), rbp); rbp = ((rdi.wrapping_add(rdx.wrapping_mul(2))) as u64) as u64; crate::simd_ops::wu64(r11.wrapping_sub(0x18u64), rsi); rsi = (crate::simd_ops::ru64(rcx.wrapping_add(0x580e0u64))) as u64; crate::simd_ops::wu64(r11.wrapping_sub(0x30u64), r14); r14 = rcx.wrapping_add(0x40); crate::simd_ops::wu64(rsp.wrapping_add(0x88u64), r14); rsi = (crate::simd_ops::ru64(rsi.wrapping_add(rax.wrapping_mul(8)))) as u64; fl_l = rdi; fl_r = rbp; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 19; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::wu64(r11.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(r11.wrapping_sub(0x20u64), r12); crate::simd_ops::wu64(r11.wrapping_sub(0x28u64), r13); crate::simd_ops::wu64(r11.wrapping_sub(0x38u64), r15); bb = 3; continue; }
            3 => { r11 = ((crate::simd_ops::ru16(rdi)) as u32) as u32 as u64; r12 = (crate::simd_ops::ru64(r14.wrapping_add(r11.wrapping_mul(8)).wrapping_add(0x48u64))) as u64; fl_l = r12; fl_r = r12; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru32(r14.wrapping_add(r11.wrapping_mul(4)).wrapping_add(0x10048u64))) as u32 as u64; bb = 6; continue; }
            5 => { rax = 0; fl_l = 0; fl_r = 0; bb = 6; continue; }
            6 => { r13 = ((rax as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 17; continue; } bb = 7; continue; }
            7 => { r8 = ((crate::simd_ops::ru16(P1)) as u32) as u32 as u64; rax = r11; rax = ((rax).wrapping_shl(5u32)) as u64; r15 = (crate::simd_ops::ru32(rax.wrapping_add(r14).wrapping_add(0x1806cu64))) as u32 as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(r14).wrapping_add(0x18058u64))) as u64; rdx = ((r15 as u32)) as u32 as u64; rcx = rax; crate::simd_ops::wu64(rsp.wrapping_add(0x98u64), rax); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = ((((rax as u32)) as i32 as i64) as u64) as u64; fl_l = rcx; fl_r = (0xffffffffffffffffu64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 9; continue; } bb = 8; continue; }
            8 => { rax = r11.wrapping_add(0xc03); fl_l = ((rax).wrapping_shl(5u32)) as u64; rax = ((fl_l as u64)) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(r14))) as u64; rbx = (crate::simd_ops::ru32(rax.wrapping_add(rcx.wrapping_mul(4)))) as u32 as u64; bb = 10; continue; }
            9 => { rbx = 0; fl_l = 0; fl_r = 0; bb = 10; continue; }
            10 => { r8 = ((crate::simd_ops::ru16(P3)) as u32) as u32 as u64; rax = 0xc03; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x98u64))) as u64; rdx = r15; r11 = ((r11).wrapping_add(rax)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = ((((rax as u32)) as i32 as i64) as u64) as u64; fl_l = rcx; fl_r = (0xffffffffffffffffu64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 12; continue; } bb = 11; continue; }
            11 => { fl_l = ((r11).wrapping_shl(5u32)) as u64; r11 = ((fl_l as u64)) as u64; rax = (crate::simd_ops::ru64(r11.wrapping_add(r14))) as u64; r15 = (crate::simd_ops::ru32(rax.wrapping_add(rcx.wrapping_mul(4)))) as u32 as u64; bb = 13; continue; }
            12 => { r15 = 0; fl_l = 0; fl_r = 0; bb = 13; continue; }
            13 => { fl_l = rbx; fl_r = rbx; rax = ((rbx.wrapping_add(r12)) as u64) as u64; rbx = (if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { rax } else { rbx }) as u64; fl_l = r15; fl_r = r15; rax = ((r15.wrapping_add(r12)) as u64) as u64; r15 = (if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { rax } else { r15 }) as u64; r14 = 0; fl_l = 0; fl_r = 0; fl_l = r13; fl_r = r13; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 16; continue; } bb = 14; continue; }
            14 => { bb = 15; continue; }
            15 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbx); rcx = rsp.wrapping_add(0x20); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), r15); rdx = rsi; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), r12); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), r14); let y_addr: u64 = rsp.wrapping_add(0x20u64); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rsp.wrapping_add(0x20u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r14 = ((r14).wrapping_add(1)) as u64; fl_l = r14; fl_r = r13; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 15; continue; } bb = 16; continue; }
            16 => { r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x88u64))) as u64; bb = 17; continue; }
            17 => { rdi = ((rdi).wrapping_add(0x2u64)) as u64; fl_l = rdi; fl_r = rbp; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } bb = 18; continue; }
            18 => { r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; r13 = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; bb = 19; continue; }
            19 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; bb = 20; continue; }
            20 => { rsp = ((rsp).wrapping_add(0x70u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_64107<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 23>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0xa8u64)) as u64; r9 = (crate::simd_ops::ru64(rcx)) as u64; rdi = 0; fl_l = 0; fl_r = 0; rax = (((rdx as u16)) as u32) as u32 as u64; rsi = (crate::simd_ops::ru64(r9.wrapping_add(rax.wrapping_mul(8)).wrapping_add(0x88u64))) as u64; fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru32(r9.wrapping_add(rax.wrapping_mul(4)).wrapping_add(0x10088u64))) as u32 as u64; bb = 3; continue; }
            2 => { rax = ((rdi as u32)) as u32 as u64; bb = 3; continue; }
            3 => { r11 = ((rax as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 9; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(r9)) as u64; r8 = ((crate::simd_ops::ru16(P0)) as u32) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xc0u64), rbx); rbx = ((crate::simd_ops::ru16(P1)) as u32) as u32 as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xd0u64), r12); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), r9); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r9); rcx = ((crate::simd_ops::ru16(rcx.wrapping_add(r8.wrapping_mul(2)))) as u32) as u32 as u64; r8 = rsi; rcx = ((rcx).wrapping_shl(4u32)) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(r9.wrapping_add(0x8u64)))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xa0u64), r15); rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x8u64))) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rax); rax = (crate::simd_ops::ru64(rcx)) as u64; rcx = (crate::simd_ops::ru64(r9.wrapping_add(0x580e0u64))) as u64; r9 = ((r11 as u32)) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rax); rax = ((crate::simd_ops::ru16(P2)) as u32) as u32 as u64; rbx = (crate::simd_ops::ru64(rcx.wrapping_add(rbx.wrapping_mul(8)))) as u64; r12 = (crate::simd_ops::ru64(rcx.wrapping_add(rax.wrapping_mul(8)))) as u64; rcx = rsp.wrapping_add(0x70); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x88u64))) as u64; fl_l = r15; fl_r = r15; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 5; continue; }
            5 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x98u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xc8u64), rbp); rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x90u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xd8u64), r14); r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; bb = 6; continue; }
            6 => { xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x30u64))) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rsp.wrapping_add(0x50u64), (xmm0) as u128); xmm0 = ((crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64) as u128; ymm0_hi = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), ((xmm0 as u64)) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), rbp); rdx = rsp.wrapping_add(0x50); crate::simd_ops::wu64(rsp.wrapping_add(0x78u64), rsi); rcx = rsp.wrapping_add(0x70); crate::simd_ops::wu64(rsp.wrapping_add(0x80u64), r14); r9 = r12; crate::simd_ops::wu64(rsp.wrapping_add(0x88u64), rdi); r8 = rbx; let y_addr: u64 = rsp.wrapping_add(0x70u64); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rsp.wrapping_add(0x70u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdi = ((rdi).wrapping_add(1)) as u64; fl_l = rdi; fl_r = r15; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 6; continue; } bb = 7; continue; }
            7 => { r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0xd8u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0xc8u64))) as u64; bb = 8; continue; }
            8 => { r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0xd0u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0xc0u64))) as u64; r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0xa0u64))) as u64; bb = 9; continue; }
            9 => { rsp = ((rsp).wrapping_add(0xa8u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_64110<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 23>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0xb0u64)) as u64; r8 = (crate::simd_ops::ru64(rcx)) as u64; rdi = 0; fl_l = 0; fl_r = 0; rax = (((rdx as u16)) as u32) as u32 as u64; r10 = (crate::simd_ops::ru64(r8.wrapping_add(rax.wrapping_mul(8)).wrapping_add(P0))) as u64; fl_l = r10; fl_r = r10; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru32(r8.wrapping_add(rax.wrapping_mul(4)).wrapping_add(P1))) as u32 as u64; bb = 3; continue; }
            2 => { rax = ((rdi as u32)) as u32 as u64; bb = 3; continue; }
            3 => { r9 = ((rax as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 9; continue; } bb = 4; continue; }
            4 => { rcx = ((crate::simd_ops::ru16(P2)) as u32) as u32 as u64; rax = (crate::simd_ops::ru64(r8.wrapping_add(P3))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xd8u64), r12); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r8); r8 = r10; crate::simd_ops::wu64(rsp.wrapping_add(0xa0u64), r15); r12 = (crate::simd_ops::ru64(rax.wrapping_add(rcx.wrapping_mul(8)))) as u64; rcx = rsp.wrapping_add(0x60); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; fl_l = r15; fl_r = r15; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 5; continue; }
            5 => { crate::simd_ops::wu64(rsp.wrapping_add(0xc0u64), rbx); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xc8u64), rbp); rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x88u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xd0u64), rsi); rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x90u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xa8u64), r14); r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; bb = 6; continue; }
            6 => { crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbx); rcx = rsp.wrapping_add(0x60); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rbp); rdx = r12; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), r14); let y_addr: u64 = rsp.wrapping_add(0x30u64); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rdi); xmm1 = ((crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64) as u128; let y_addr: u64 = rsp.wrapping_add(0x60u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); crate::simd_ops::wu64(rsp.wrapping_add(0x80u64), ((xmm1 as u64)) as u64); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdi = ((rdi).wrapping_add(1)) as u64; fl_l = rdi; fl_r = r15; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 6; continue; } bb = 7; continue; }
            7 => { r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0xa8u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0xd0u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0xc8u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0xc0u64))) as u64; bb = 8; continue; }
            8 => { r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0xd8u64))) as u64; r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0xa0u64))) as u64; bb = 9; continue; }
            9 => { rsp = ((rsp).wrapping_add(0xb0u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_64118<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 25>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { r11 = rsp; rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0xc0u64)) as u64; rax = (((r8 as u16)) as u32) as u32 as u64; rdi = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = (crate::simd_ops::ru64(rdx)) as u64; rdi = ((rdi).wrapping_add(rdi)) as u64; r10 = (crate::simd_ops::ru32(rax.wrapping_add(rdi.wrapping_mul(8)).wrapping_add(0x8u64))) as u32 as u64; rdi = (crate::simd_ops::ru64(rax.wrapping_add(rdi.wrapping_mul(8)))) as u64; fl_l = r10; fl_r = r10; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 21; continue; } bb = 1; continue; }
            1 => { r9 = (crate::simd_ops::ru64(rcx)) as u64; rdx = ((crate::simd_ops::ru16(P0)) as u32) as u32 as u64; crate::simd_ops::wu64(r11.wrapping_sub(0x10u64), rbp); rbp = ((rdi.wrapping_add(r10.wrapping_mul(2))) as u64) as u64; crate::simd_ops::wu64(r11.wrapping_sub(0x18u64), rsi); rax = (crate::simd_ops::ru64(r9)) as u64; rsi = ((crate::simd_ops::ru16(P1)) as u32) as u32 as u64; crate::simd_ops::wu64(r11.wrapping_sub(0x30u64), r14); r14 = r9.wrapping_add(0x40); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), r9); rcx = (crate::simd_ops::ru64(rax.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xd8u64), r14); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rbp); rcx = ((crate::simd_ops::ru16(rcx.wrapping_add(rdx.wrapping_mul(2)))) as u32) as u32 as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(r9.wrapping_add(0x8u64)))) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x8u64))) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rax); rax = (crate::simd_ops::ru64(rcx)) as u64; rcx = (crate::simd_ops::ru64(r9.wrapping_add(0x580e0u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rax); rax = ((crate::simd_ops::ru16(P2)) as u32) as u32 as u64; rsi = (crate::simd_ops::ru64(rcx.wrapping_add(rsi.wrapping_mul(8)))) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(rax.wrapping_mul(8)))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); fl_l = rdi; fl_r = rbp; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 20; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::wu64(r11.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(r11.wrapping_sub(0x20u64), r12); crate::simd_ops::wu64(r11.wrapping_sub(0x28u64), r13); crate::simd_ops::wu64(r11.wrapping_sub(0x38u64), r15); bb = 3; continue; }
            3 => { r11 = ((crate::simd_ops::ru16(rdi)) as u32) as u32 as u64; r12 = (crate::simd_ops::ru64(r14.wrapping_add(r11.wrapping_mul(8)).wrapping_add(0x48u64))) as u64; fl_l = r12; fl_r = r12; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru32(r14.wrapping_add(r11.wrapping_mul(4)).wrapping_add(0x10048u64))) as u32 as u64; bb = 6; continue; }
            5 => { rax = 0; fl_l = 0; fl_r = 0; bb = 6; continue; }
            6 => { r13 = ((rax as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 18; continue; } bb = 7; continue; }
            7 => { r8 = ((crate::simd_ops::ru16(P3)) as u32) as u32 as u64; rax = r11; rax = ((rax).wrapping_shl(5u32)) as u64; r15 = (crate::simd_ops::ru32(rax.wrapping_add(r14).wrapping_add(0x1806cu64))) as u32 as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(r14).wrapping_add(0x18058u64))) as u64; rdx = ((r15 as u32)) as u32 as u64; rcx = rax; crate::simd_ops::wu64(rsp.wrapping_add(0xe8u64), rax); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = ((((rax as u32)) as i32 as i64) as u64) as u64; fl_l = rcx; fl_r = (0xffffffffffffffffu64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 9; continue; } bb = 8; continue; }
            8 => { rax = r11.wrapping_add(0xc03); fl_l = ((rax).wrapping_shl(5u32)) as u64; rax = ((fl_l as u64)) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(r14))) as u64; rbx = (crate::simd_ops::ru32(rax.wrapping_add(rcx.wrapping_mul(4)))) as u32 as u64; bb = 10; continue; }
            9 => { rbx = 0; fl_l = 0; fl_r = 0; bb = 10; continue; }
            10 => { r8 = ((crate::simd_ops::ru16(P5)) as u32) as u32 as u64; rax = 0xc03; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0xe8u64))) as u64; rdx = r15; r11 = ((r11).wrapping_add(rax)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = ((((rax as u32)) as i32 as i64) as u64) as u64; fl_l = rcx; fl_r = (0xffffffffffffffffu64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 12; continue; } bb = 11; continue; }
            11 => { fl_l = ((r11).wrapping_shl(5u32)) as u64; r11 = ((fl_l as u64)) as u64; rax = (crate::simd_ops::ru64(r11.wrapping_add(r14))) as u64; r15 = (crate::simd_ops::ru32(rax.wrapping_add(rcx.wrapping_mul(4)))) as u32 as u64; bb = 13; continue; }
            12 => { r15 = 0; fl_l = 0; fl_r = 0; bb = 13; continue; }
            13 => { fl_l = rbx; fl_r = rbx; rax = ((r12.wrapping_add(rbx)) as u64) as u64; rbx = (if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { rax } else { rbx }) as u64; fl_l = r15; fl_r = r15; rax = ((r12.wrapping_add(r15)) as u64) as u64; r15 = (if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { rax } else { r15 }) as u64; r14 = 0; fl_l = 0; fl_r = 0; fl_l = r13; fl_r = r13; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 17; continue; } bb = 14; continue; }
            14 => { rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; bb = 15; continue; }
            15 => { xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x30u64))) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rsp.wrapping_add(0x50u64), (xmm0) as u128); xmm0 = ((crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64) as u128; ymm0_hi = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), ((xmm0 as u64)) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), rbx); rdx = rsp.wrapping_add(0x50); crate::simd_ops::wu64(rsp.wrapping_add(0x78u64), r15); rcx = rsp.wrapping_add(0x70); crate::simd_ops::wu64(rsp.wrapping_add(0x80u64), r12); r9 = rbp; crate::simd_ops::wu64(rsp.wrapping_add(0x88u64), r14); r8 = rsi; let y_addr: u64 = rsp.wrapping_add(0x70u64); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rsp.wrapping_add(0x70u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r14 = ((r14).wrapping_add(1)) as u64; fl_l = r14; fl_r = r13; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 15; continue; } bb = 16; continue; }
            16 => { rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; bb = 17; continue; }
            17 => { r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0xd8u64))) as u64; bb = 18; continue; }
            18 => { rdi = ((rdi).wrapping_add(0x2u64)) as u64; fl_l = rdi; fl_r = rbp; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } bb = 19; continue; }
            19 => { r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x90u64))) as u64; r13 = (crate::simd_ops::ru64(rsp.wrapping_add(0xa0u64))) as u64; r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0xa8u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0xd0u64))) as u64; bb = 20; continue; }
            20 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0xb0u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0xb8u64))) as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x98u64))) as u64; bb = 21; continue; }
            21 => { rsp = ((rsp).wrapping_add(0xc0u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_64121<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 25>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { r11 = rsp; rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0xc0u64)) as u64; rax = (((r8 as u16)) as u32) as u32 as u64; rdi = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = (crate::simd_ops::ru64(rdx)) as u64; rdi = ((rdi).wrapping_add(rdi)) as u64; rdx = (crate::simd_ops::ru32(rax.wrapping_add(rdi.wrapping_mul(8)).wrapping_add(0x8u64))) as u32 as u64; rdi = (crate::simd_ops::ru64(rax.wrapping_add(rdi.wrapping_mul(8)))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 24; continue; } bb = 1; continue; }
            1 => { rcx = (crate::simd_ops::ru64(rcx)) as u64; rax = ((crate::simd_ops::ru16(P0)) as u32) as u32 as u64; crate::simd_ops::wu64(r11.wrapping_sub(0x10u64), rbp); rbp = ((rdi.wrapping_add(rdx.wrapping_mul(2))) as u64) as u64; crate::simd_ops::wu64(r11.wrapping_sub(0x18u64), rsi); rsi = (crate::simd_ops::ru64(rcx.wrapping_add(0x580e0u64))) as u64; crate::simd_ops::wu64(r11.wrapping_sub(0x30u64), r14); r14 = rcx.wrapping_add(0x40); crate::simd_ops::wu64(rsp.wrapping_add(0xe8u64), r14); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rbp); rsi = (crate::simd_ops::ru64(rsi.wrapping_add(rax.wrapping_mul(8)))) as u64; fl_l = rdi; fl_r = rbp; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 23; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::wu64(r11.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(r11.wrapping_sub(0x20u64), r12); crate::simd_ops::wu64(r11.wrapping_sub(0x28u64), r13); crate::simd_ops::wu64(r11.wrapping_sub(0x38u64), r15); bb = 3; continue; }
            3 => { r11 = ((crate::simd_ops::ru16(rdi)) as u32) as u32 as u64; r13 = (crate::simd_ops::ru64(r14.wrapping_add(r11.wrapping_mul(8)).wrapping_add(0x48u64))) as u64; fl_l = r13; fl_r = r13; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru32(r14.wrapping_add(r11.wrapping_mul(4)).wrapping_add(0x10048u64))) as u32 as u64; bb = 6; continue; }
            5 => { rax = 0; fl_l = 0; fl_r = 0; bb = 6; continue; }
            6 => { crate::simd_ops::wu64(rsp.wrapping_add(0xd8u64), rax); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 21; continue; } bb = 7; continue; }
            7 => { r8 = ((crate::simd_ops::ru16(P1)) as u32) as u32 as u64; rax = r11; rax = ((rax).wrapping_shl(5u32)) as u64; r12 = (crate::simd_ops::ru64(rax.wrapping_add(r14).wrapping_add(0x18058u64))) as u64; r15 = (crate::simd_ops::ru32(rax.wrapping_add(r14).wrapping_add(0x1806cu64))) as u32 as u64; rcx = r12; rdx = ((r15 as u32)) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r12); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = ((((rax as u32)) as i32 as i64) as u64) as u64; fl_l = rcx; fl_r = (0xffffffffffffffffu64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 9; continue; } bb = 8; continue; }
            8 => { rax = r11.wrapping_add(0xc03); fl_l = ((rax).wrapping_shl(5u32)) as u64; rax = ((fl_l as u64)) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(r14))) as u64; rbx = (crate::simd_ops::ru32(rax.wrapping_add(rcx.wrapping_mul(4)))) as u32 as u64; bb = 10; continue; }
            9 => { rbx = 0; fl_l = 0; fl_r = 0; bb = 10; continue; }
            10 => { r8 = ((crate::simd_ops::ru16(P3)) as u32) as u32 as u64; rax = 0xc03; rdx = r15; rcx = r12; r11 = ((r11).wrapping_add(rax)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = ((((rax as u32)) as i32 as i64) as u64) as u64; fl_l = rcx; fl_r = (0xffffffffffffffffu64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 12; continue; } bb = 11; continue; }
            11 => { fl_l = ((r11).wrapping_shl(5u32)) as u64; r11 = ((fl_l as u64)) as u64; rax = (crate::simd_ops::ru64(r11.wrapping_add(r14))) as u64; r12 = (crate::simd_ops::ru32(rax.wrapping_add(rcx.wrapping_mul(4)))) as u32 as u64; bb = 13; continue; }
            12 => { r12 = 0; fl_l = 0; fl_r = 0; fl_l = ((r11).wrapping_shl(5u32)) as u64; r11 = ((fl_l as u64)) as u64; bb = 13; continue; }
            13 => { r8 = ((crate::simd_ops::ru16(P4)) as u32) as u32 as u64; rdx = r15; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = ((((rax as u32)) as i32 as i64) as u64) as u64; fl_l = rcx; fl_r = (0xffffffffffffffffu64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 15; continue; } bb = 14; continue; }
            14 => { rax = (crate::simd_ops::ru64(r11.wrapping_add(r14))) as u64; r15 = (crate::simd_ops::ru32(rax.wrapping_add(rcx.wrapping_mul(4)))) as u32 as u64; bb = 16; continue; }
            15 => { r15 = 0; fl_l = 0; fl_r = 0; bb = 16; continue; }
            16 => { fl_l = rbx; fl_r = rbx; rax = ((rbx.wrapping_add(r13)) as u64) as u64; rbx = (if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { rax } else { rbx }) as u64; fl_l = r12; fl_r = r12; rax = ((r12.wrapping_add(r13)) as u64) as u64; r12 = (if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { rax } else { r12 }) as u64; fl_l = r15; fl_r = r15; rax = ((r15.wrapping_add(r13)) as u64) as u64; r15 = (if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { rax } else { r15 }) as u64; r14 = 0; fl_l = 0; fl_r = 0; fl_l = (crate::simd_ops::ru64(rsp.wrapping_add(0xd8u64))) as u64; fl_r = r14; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 20; continue; } bb = 17; continue; }
            17 => { rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0xd8u64))) as u64; bb = 18; continue; }
            18 => { crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbx); rcx = rsp.wrapping_add(0x60); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), r12); rdx = rsi; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), r15); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), r13); let y_addr: u64 = rsp.wrapping_add(0x30u64); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), r14); xmm1 = ((crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64) as u128; let y_addr: u64 = rsp.wrapping_add(0x60u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); crate::simd_ops::wu64(rsp.wrapping_add(0x80u64), ((xmm1 as u64)) as u64); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r14 = ((r14).wrapping_add(1)) as u64; fl_l = r14; fl_r = rbp; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 18; continue; } bb = 19; continue; }
            19 => { rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; bb = 20; continue; }
            20 => { r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0xe8u64))) as u64; bb = 21; continue; }
            21 => { rdi = ((rdi).wrapping_add(0x2u64)) as u64; fl_l = rdi; fl_r = rbp; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } bb = 22; continue; }
            22 => { r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x90u64))) as u64; r13 = (crate::simd_ops::ru64(rsp.wrapping_add(0xa0u64))) as u64; r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0xa8u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0xd0u64))) as u64; bb = 23; continue; }
            23 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0xb0u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0xb8u64))) as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x98u64))) as u64; bb = 24; continue; }
            24 => { rsp = ((rsp).wrapping_add(0xc0u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_64176<const P0: u64, const P1: u64, const P2: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0; let mut ymm1_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 73>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rbp = rsp.wrapping_sub(0x130); rsp = ((rsp).wrapping_sub(0x230u64)) as u64; rax = (((r8 as u16)) as u32) as u32 as u64; rbx = rcx; r8 = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = (crate::simd_ops::ru64(rdx)) as u64; r8 = ((r8).wrapping_add(r8)) as u64; rdi = (crate::simd_ops::ru32(rax.wrapping_add(r8.wrapping_mul(8)).wrapping_add(0x8u64))) as u32 as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(r8.wrapping_mul(8)))) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x168u64), rax); fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 31; continue; } bb = 1; continue; }
            1 => { rdx = rcx; rcx = rbp.wrapping_add(0x40); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r9 = (crate::simd_ops::ru64(rbp.wrapping_add(0x168u64))) as u64; r8 = (crate::simd_ops::ru64(rbx)) as u64; r8 = ((r8).wrapping_add(0x40u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x78u64), r8); rax = ((r9.wrapping_add(rdi.wrapping_mul(2))) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), rax); fl_l = r9; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 31; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::wu64(rsp.wrapping_add(0x250u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x228u64), r12); crate::simd_ops::wu64(rsp.wrapping_add(0x220u64), r13); crate::simd_ops::wu64(rsp.wrapping_add(0x218u64), r14); crate::simd_ops::wu64(rsp.wrapping_add(0x210u64), r15); bb = 3; continue; }
            3 => { r9 = ((crate::simd_ops::ru16(r9)) as u32) as u32 as u64; rcx = (crate::simd_ops::ru64(r8.wrapping_add(r9.wrapping_mul(8)).wrapping_add(0x48u64))) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x158u64), rcx); fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rcx = (crate::simd_ops::ru32(r8.wrapping_add(r9.wrapping_mul(4)).wrapping_add(0x10048u64))) as u32 as u64; bb = 5; continue; }
            5 => { crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rcx); fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 29; continue; } bb = 6; continue; }
            6 => { rcx = rbp.wrapping_sub(0x80); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru64(rbp.wrapping_sub(0x80u64))) as u64; rdx = (crate::simd_ops::ru64(rbp.wrapping_add(0x158u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 8; continue; } bb = 7; continue; }
            7 => { rcx = 0; fl_l = 0; fl_r = 0; bb = 9; continue; }
            8 => { rcx = ((rdx.wrapping_add(rax)) as u64) as u64; bb = 9; continue; }
            9 => { rax = (crate::simd_ops::ru64(rbp.wrapping_sub(0x78u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), rcx); crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), rax); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 10; continue; }
            10 => { fl_l = ((rax).wrapping_add(rdx)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), rax); bb = 11; continue; }
            11 => { r13 = (crate::simd_ops::ru64(rbp.wrapping_sub(0x70u64))) as u64; fl_l = r13; fl_r = r13; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 13; continue; } bb = 12; continue; }
            12 => { fl_l = ((r13).wrapping_add(rdx)) as u64; fl_r = fl_l; r13 = ((fl_l as u64)) as u64; bb = 13; continue; }
            13 => { r12 = (crate::simd_ops::ru64(rbp.wrapping_sub(0x68u64))) as u64; fl_l = r12; fl_r = r12; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 15; continue; } bb = 14; continue; }
            14 => { fl_l = ((r12).wrapping_add(rdx)) as u64; fl_r = fl_l; r12 = ((fl_l as u64)) as u64; bb = 15; continue; }
            15 => { r15 = (crate::simd_ops::ru64(rbp.wrapping_sub(0x60u64))) as u64; fl_l = r15; fl_r = r15; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 17; continue; } bb = 16; continue; }
            16 => { fl_l = ((r15).wrapping_add(rdx)) as u64; fl_r = fl_l; r15 = ((fl_l as u64)) as u64; bb = 17; continue; }
            17 => { r14 = (crate::simd_ops::ru64(rbp.wrapping_sub(0x58u64))) as u64; fl_l = r14; fl_r = r14; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 19; continue; } bb = 18; continue; }
            18 => { fl_l = ((r14).wrapping_add(rdx)) as u64; fl_r = fl_l; r14 = ((fl_l as u64)) as u64; bb = 19; continue; }
            19 => { rsi = (crate::simd_ops::ru64(rbp.wrapping_sub(0x50u64))) as u64; fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 21; continue; } bb = 20; continue; }
            20 => { fl_l = ((rsi).wrapping_add(rdx)) as u64; fl_r = fl_l; rsi = ((fl_l as u64)) as u64; bb = 21; continue; }
            21 => { rdi = (crate::simd_ops::ru64(rbp.wrapping_sub(0x48u64))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 23; continue; } bb = 22; continue; }
            22 => { fl_l = ((rdi).wrapping_add(rdx)) as u64; fl_r = fl_l; rdi = ((fl_l as u64)) as u64; bb = 23; continue; }
            23 => { rbx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x40u64))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 25; continue; } bb = 24; continue; }
            24 => { fl_l = ((rbx).wrapping_add(rdx)) as u64; fl_r = fl_l; rbx = ((fl_l as u64)) as u64; bb = 25; continue; }
            25 => { r8 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), r8); fl_l = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; fl_r = r8; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 28; continue; } bb = 26; continue; }
            26 => { bb = 27; continue; }
            27 => { xmm0 = (crate::simd_ops::ru128(rbp.wrapping_add(0x68u64))) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rbp.wrapping_sub(0x20u64), (xmm0) as u128); xmm0 = ((crate::simd_ops::ru64(rbp.wrapping_add(0x78u64))) as u64) as u128; ymm0_hi = 0; crate::simd_ops::wu64(rbp.wrapping_sub(0x10u64), ((xmm0 as u64)) as u64); xmm0 = (crate::simd_ops::ru128(rbp.wrapping_add(0x80u64))) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rbp, (xmm0) as u128); xmm0 = ((crate::simd_ops::ru64(rbp.wrapping_add(0x90u64))) as u64) as u128; ymm0_hi = 0; crate::simd_ops::wu64(rbp.wrapping_sub(0x78u64), rax); r9 = rbp.wrapping_sub(0x20); rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x40u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rax); rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x48u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rax); rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x50u64))) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x10u64), ((xmm0 as u64)) as u64); xmm0 = (crate::simd_ops::ru128(rbp.wrapping_add(0x98u64))) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rbp.wrapping_add(0x20u64), (xmm0) as u128); xmm0 = ((crate::simd_ops::ru64(rbp.wrapping_add(0xa8u64))) as u64) as u128; ymm0_hi = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x58u64))) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x30u64), ((xmm0 as u64)) as u64); crate::simd_ops::wu64(rbp.wrapping_sub(0x80u64), rcx); rcx = rbp.wrapping_add(0xb0); crate::simd_ops::wu64(rbp.wrapping_sub(0x38u64), rdx); rdx = rbp.wrapping_add(0x20); crate::simd_ops::wu64(rbp.wrapping_sub(0x30u64), r8); r8 = rbp; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax); rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x60u64))) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x70u64), r13); crate::simd_ops::wu64(rbp.wrapping_sub(0x68u64), r12); let y_addr: u64 = rbp.wrapping_sub(0x80u64); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rbp.wrapping_add(0xb0u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); crate::simd_ops::wu64(rbp.wrapping_sub(0x60u64), r15); crate::simd_ops::wu64(rbp.wrapping_sub(0x58u64), r14); crate::simd_ops::wu64(rbp.wrapping_sub(0x50u64), rsi); crate::simd_ops::wu64(rbp.wrapping_sub(0x48u64), rdi); let y_addr: u64 = rbp.wrapping_sub(0x60u64); xmm1 = crate::simd_ops::ru128(y_addr); ymm1_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rbp.wrapping_add(0xd0u64); crate::simd_ops::wu128(y_addr, xmm1); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm1_hi); xmm1 = ((crate::simd_ops::ru64(rbp.wrapping_sub(0x30u64))) as u64) as u128; ymm1_hi = 0; crate::simd_ops::wu64(rbp.wrapping_sub(0x40u64), rbx); xmm0 = (crate::simd_ops::ru128(rbp.wrapping_sub(0x40u64))) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rbp.wrapping_add(0xf0u64), (xmm0) as u128); crate::simd_ops::wu64(rbp.wrapping_add(0x100u64), ((xmm1 as u64)) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r8 = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; r8 = ((r8).wrapping_add(1)) as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rdx = (crate::simd_ops::ru64(rbp.wrapping_add(0x158u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), r8); fl_l = r8; fl_r = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 27; continue; } bb = 28; continue; }
            28 => { r8 = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; bb = 29; continue; }
            29 => { r9 = (crate::simd_ops::ru64(rbp.wrapping_add(0x168u64))) as u64; r9 = ((r9).wrapping_add(0x2u64)) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x168u64), r9); fl_l = r9; fl_r = rax; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } bb = 30; continue; }
            30 => { r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x210u64))) as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x218u64))) as u64; r13 = (crate::simd_ops::ru64(rsp.wrapping_add(0x220u64))) as u64; r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x228u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x250u64))) as u64; bb = 31; continue; }
            31 => { rsp = ((rsp).wrapping_add(0x230u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_64186<const P0: u64, const P1: u64, const P2: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rbx = rcx; r9 = ((rdx as u32)) as u32 as u64; rcx = (crate::simd_ops::ru64(r8)) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x20u64))) as u32 as u64; fl_l = r9; fl_r = rax; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 20; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x18u64))) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(r9.wrapping_mul(8)))) as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((r9 as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 20; continue; } bb = 2; continue; }
            2 => { rax = rdx; rcx = ((rcx).wrapping_shr(32u32)) as u64; rax = ((rax).wrapping_shr(32u32)) as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 20; continue; } bb = 3; continue; }
            3 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0xffffffff; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 20; continue; } bb = 4; continue; }
            4 => { xmm1 = (crate::simd_ops::ru128(r8)) as u128; xmm0 = ((crate::simd_ops::ru64(r8.wrapping_add(0x10u64))) as u64) as u128; ymm0_hi = 0; r11 = ((xmm1 as u64)) as u64; rax = 0xffff; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), ((xmm0 as u64)) as u64); r8 = (crate::simd_ops::ru64(r11.wrapping_add(0x30u64))) as u64; r8 = (crate::simd_ops::ru64(r8.wrapping_add(r9.wrapping_mul(8)))) as u64; fl_l = ((r8 as u16)) as u16 as u64; fl_r = ((rax as u16)) as u16 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { bb = 6; continue; } bb = 5; continue; }
            5 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::lift_ops::trap(0x80000003, P2, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            6 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rcx = (((xmm1 >> 64) as u64)) as u64; r10 = ((rax.wrapping_add(rcx.wrapping_mul(2))) as u64) as u64; rdx = r10; rdx = ((rdx).wrapping_sub(rax)) as u64; rdx = ((((rdx) as i64).wrapping_shr(1u32) as u64)) as u64; fl_l = rdx; fl_r = rdx; if (((((fl_l as u64)) & ((fl_r as u64))) as i64) <= 0) { bb = 12; continue; } bb = 7; continue; }
            7 => { bb = 8; continue; }
            8 => { r9 = rdx; r9 = ((r9).wrapping_shr(1u32)) as u64; fl_l = (crate::simd_ops::ru16(rax.wrapping_add(r9.wrapping_mul(2)))) as u16 as u64; fl_r = ((r8 as u16)) as u16 as u64; rcx = ((rax.wrapping_add(r9.wrapping_mul(2))) as u64) as u64; if (((fl_l as u16)) >= ((fl_r as u16))) { bb = 10; continue; } bb = 9; continue; }
            9 => { rax = (0xffffffffffffffffu64) as u64; rax = ((rax).wrapping_sub(r9)) as u64; fl_l = ((rdx).wrapping_add(rax)) as u64; fl_r = fl_l; rdx = ((fl_l as u64)) as u64; rax = rcx.wrapping_add(0x2); bb = 11; continue; }
            10 => { rdx = r9; bb = 11; continue; }
            11 => { fl_l = rdx; fl_r = rdx; if (((((fl_l as u64)) & ((fl_r as u64))) as i64) > 0) { bb = 8; continue; } bb = 12; continue; }
            12 => { fl_l = rax; fl_r = r10; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 19; continue; } bb = 13; continue; }
            13 => { fl_l = ((r8 as u16)) as u16 as u64; fl_r = (crate::simd_ops::ru16(rax)) as u16 as u64; if (((fl_l as u16)) < ((fl_r as u16))) { bb = 19; continue; } bb = 14; continue; }
            14 => { crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rsi); rdx = r11.wrapping_add(0x40); rax = (((r8 as u16)) as u32) as u32 as u64; rcx = rsp.wrapping_add(0x20); rsi = r8; crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rdi); rsi = ((rsi).wrapping_shr(32u32)) as u64; rdi = (crate::simd_ops::ru64(rdx.wrapping_add(rax.wrapping_mul(8)).wrapping_add(0x48u64))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 16; continue; } bb = 15; continue; }
            15 => { fl_l = ((rcx).wrapping_add(rdi)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; bb = 16; continue; }
            16 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 18; continue; } bb = 17; continue; }
            17 => { fl_l = ((rax).wrapping_add(rdi)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; bb = 18; continue; }
            18 => { crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax); rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rdi); rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rsi); rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rcx); let y_addr: u64 = rsp.wrapping_add(0x20u64); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rbx; crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); crate::simd_ops::wu8(rbx.wrapping_add(0x20u64), ((rax as u8)) as u8); rax = rbx; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            19 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; crate::simd_ops::wu8(rbx.wrapping_add(0x20u64), ((rax as u8)) as u8); rax = rbx; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            20 => { crate::simd_ops::wu8(rbx.wrapping_add(0x20u64), (0x0u8) as u8); rax = rbx; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_64194<const P0: u64, const P1: u64, const P2: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = ((rsp).wrapping_sub(0x60u64)) as u64; rsi = rdx; rdx = (crate::simd_ops::ru64(rcx)) as u64; rcx = ((r8 as u32)) as u32 as u64; rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x30u64))) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(rcx.wrapping_mul(8)))) as u64; rcx = 0xffff; fl_l = ((rax as u16)) as u16 as u64; fl_r = ((rcx as u16)) as u16 as u64; if (((fl_l as u16)) == ((fl_r as u16))) { bb = 18; continue; } bb = 1; continue; }
            1 => { r9 = (((rax as u16)) as u32) as u32 as u64; r8 = rdx.wrapping_add(0x40); crate::simd_ops::wu64(rsp.wrapping_add(0x78u64), rbx); rcx = rsp.wrapping_add(0x20); crate::simd_ops::wu64(rsp.wrapping_add(0x80u64), rdi); crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), rax); rdi = (crate::simd_ops::ru64(r8.wrapping_add(r9.wrapping_mul(8)).wrapping_add(0x48u64))) as u64; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { fl_l = ((rbx).wrapping_add(rdi)) as u64; fl_r = fl_l; rbx = ((fl_l as u64)) as u64; bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { fl_l = ((rax).wrapping_add(rdi)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; bb = 5; continue; }
            5 => { rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 6; continue; }
            6 => { fl_l = ((rcx).wrapping_add(rdi)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; bb = 7; continue; }
            7 => { rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 8; continue; }
            8 => { fl_l = ((rdx).wrapping_add(rdi)) as u64; fl_r = fl_l; rdx = ((fl_l as u64)) as u64; bb = 9; continue; }
            9 => { r8 = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 10; continue; }
            10 => { fl_l = ((r8).wrapping_add(rdi)) as u64; fl_r = fl_l; r8 = ((fl_l as u64)) as u64; bb = 11; continue; }
            11 => { r9 = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; fl_l = r9; fl_r = r9; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 13; continue; } bb = 12; continue; }
            12 => { fl_l = ((r9).wrapping_add(rdi)) as u64; fl_r = fl_l; r9 = ((fl_l as u64)) as u64; bb = 13; continue; }
            13 => { r10 = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; fl_l = r10; fl_r = r10; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 15; continue; } bb = 14; continue; }
            14 => { fl_l = ((r10).wrapping_add(rdi)) as u64; fl_r = fl_l; r10 = ((fl_l as u64)) as u64; bb = 15; continue; }
            15 => { r11 = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; fl_l = r11; fl_r = r11; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 17; continue; } bb = 16; continue; }
            16 => { fl_l = ((r11).wrapping_add(rdi)) as u64; fl_r = fl_l; r11 = ((fl_l as u64)) as u64; bb = 17; continue; }
            17 => { crate::simd_ops::wu64(rsi, rbx); crate::simd_ops::wu64(rsi.wrapping_add(0x8u64), rax); rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x74u64))) as u32 as u64; crate::simd_ops::wu64(rsi.wrapping_add(0x10u64), rcx); crate::simd_ops::wu64(rsi.wrapping_add(0x18u64), rdx); crate::simd_ops::wu64(rsi.wrapping_add(0x20u64), r8); crate::simd_ops::wu64(rsi.wrapping_add(0x28u64), r9); crate::simd_ops::wu64(rsi.wrapping_add(0x30u64), r10); crate::simd_ops::wu64(rsi.wrapping_add(0x38u64), r11); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; crate::simd_ops::wu64(rsi.wrapping_add(0x40u64), rdi); rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; crate::simd_ops::wu64(rsi.wrapping_add(0x48u64), rax); rax = rsi; rsp = ((rsp).wrapping_add(0x60u64)) as u64; rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            18 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, P2, rcx, rsp); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_64222<const P0: u64, const P1: u64, const P2: u64, const P3: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 27>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { r11 = rsp; rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0xd0u64)) as u64; rax = (((r8 as u16)) as u32) as u32 as u64; rbx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = (crate::simd_ops::ru64(rdx)) as u64; rbx = ((rbx).wrapping_add(rbx)) as u64; rdx = (crate::simd_ops::ru32(rax.wrapping_add(rbx.wrapping_mul(8)).wrapping_add(0x8u64))) as u32 as u64; rbx = (crate::simd_ops::ru64(rax.wrapping_add(rbx.wrapping_mul(8)))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xf8u64), rbx); fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 15; continue; } bb = 1; continue; }
            1 => { r8 = (crate::simd_ops::ru64(rcx)) as u64; rax = ((crate::simd_ops::ru16(P0)) as u32) as u32 as u64; crate::simd_ops::wu64(r11.wrapping_add(0x8u64), rbp); rbp = ((rbx.wrapping_add(rdx.wrapping_mul(2))) as u64) as u64; crate::simd_ops::wu64(r11.wrapping_sub(0x10u64), rsi); rcx = (crate::simd_ops::ru64(r8.wrapping_add(0x580e0u64))) as u64; rsi = ((crate::simd_ops::ru16(P1)) as u32) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xe8u64), r8); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rbp); rax = (crate::simd_ops::ru64(rcx.wrapping_add(rax.wrapping_mul(8)))) as u64; rsi = (crate::simd_ops::ru64(rcx.wrapping_add(rsi.wrapping_mul(8)))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); fl_l = rbx; fl_r = rbp; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 14; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::wu64(r11.wrapping_sub(0x18u64), rdi); crate::simd_ops::wu64(r11.wrapping_sub(0x20u64), r12); crate::simd_ops::wu64(r11.wrapping_sub(0x28u64), r13); crate::simd_ops::wu64(r11.wrapping_sub(0x30u64), r14); crate::simd_ops::wu64(r11.wrapping_sub(0x38u64), r15); bb = 3; continue; }
            3 => { rax = ((crate::simd_ops::ru16(rbx)) as u32) as u32 as u64; r12 = (crate::simd_ops::ru64(r8.wrapping_add(rax.wrapping_mul(8)).wrapping_add(0x88u64))) as u64; fl_l = r12; fl_r = r12; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rdx = (crate::simd_ops::ru32(r8.wrapping_add(rax.wrapping_mul(4)).wrapping_add(0x10088u64))) as u32 as u64; bb = 6; continue; }
            5 => { rdx = 0; fl_l = 0; fl_r = 0; bb = 6; continue; }
            6 => { r15 = ((rdx as u32)) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 12; continue; } bb = 7; continue; }
            7 => { crate::simd_ops::wu16(rsp.wrapping_add(0x30u64), ((rax as u16)) as u16); rdx = rsp.wrapping_add(0x70); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), r12); rcx = rsp.wrapping_add(0x30); crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), r8); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), r15); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r14 = 0; fl_l = 0; fl_r = 0; let y_addr: u64 = rax; xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rsp.wrapping_add(0x50u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); fl_l = r15; fl_r = r15; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 8; continue; }
            8 => { r13 = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; bb = 9; continue; }
            9 => { r8 = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; rcx = rsp.wrapping_add(0x30); crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), rdi); rdx = rsi; crate::simd_ops::wu64(rsp.wrapping_add(0x78u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x80u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x88u64), r13); let y_addr: u64 = rsp.wrapping_add(0x70u64); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); crate::simd_ops::wu64(rsp.wrapping_add(0x90u64), r12); crate::simd_ops::wu64(rsp.wrapping_add(0x98u64), r14); xmm1 = (crate::simd_ops::ru128(rsp.wrapping_add(0x90u64))) as u128; let y_addr: u64 = rsp.wrapping_add(0x30u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); crate::simd_ops::wu128(rsp.wrapping_add(0x50u64), (xmm1) as u128); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r14 = ((r14).wrapping_add(1)) as u64; fl_l = r14; fl_r = r15; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 9; continue; } bb = 10; continue; }
            10 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0xf8u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; bb = 11; continue; }
            11 => { r8 = (crate::simd_ops::ru64(rsp.wrapping_add(0xe8u64))) as u64; bb = 12; continue; }
            12 => { rbx = ((rbx).wrapping_add(0x2u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xf8u64), rbx); fl_l = rbx; fl_r = rbp; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } bb = 13; continue; }
            13 => { r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0xa0u64))) as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0xa8u64))) as u64; r13 = (crate::simd_ops::ru64(rsp.wrapping_add(0xb0u64))) as u64; r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0xb8u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0xc0u64))) as u64; bb = 14; continue; }
            14 => { rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0xe0u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0xc8u64))) as u64; bb = 15; continue; }
            15 => { rsp = ((rsp).wrapping_add(0xd0u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_64224<const P0: u64, const P1: u64, const P2: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0; let mut ymm1_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 31>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { r11 = rsp; rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = ((rsp).wrapping_sub(0xf0u64)) as u64; rax = (((r8 as u16)) as u32) as u32 as u64; rbp = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = (crate::simd_ops::ru64(rdx)) as u64; rbp = ((rbp).wrapping_add(rbp)) as u64; rdx = (crate::simd_ops::ru32(rax.wrapping_add(rbp.wrapping_mul(8)).wrapping_add(0x8u64))) as u32 as u64; rbp = (crate::simd_ops::ru64(rax.wrapping_add(rbp.wrapping_mul(8)))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rbp); fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 28; continue; } bb = 1; continue; }
            1 => { rcx = (crate::simd_ops::ru64(rcx)) as u64; r9 = ((rdx.wrapping_mul(2)) as u64) as u64; rax = ((crate::simd_ops::ru16(P0)) as u32) as u32 as u64; r9 = ((r9).wrapping_add(rbp)) as u64; crate::simd_ops::wu64(r11.wrapping_sub(0x28u64), r13); crate::simd_ops::wu64(rsp.wrapping_add(0x108u64), r9); r13 = (crate::simd_ops::ru64(rcx.wrapping_add(0x580e0u64))) as u64; r8 = rcx.wrapping_add(0x40); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), r8); r13 = (crate::simd_ops::ru64(r13.wrapping_add(rax.wrapping_mul(8)))) as u64; fl_l = rbp; fl_r = r9; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 27; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::wu64(r11.wrapping_add(0x8u64), rbx); rax = r9; crate::simd_ops::wu64(r11.wrapping_sub(0x10u64), rsi); crate::simd_ops::wu64(r11.wrapping_sub(0x18u64), rdi); crate::simd_ops::wu64(r11.wrapping_sub(0x20u64), r12); crate::simd_ops::wu64(r11.wrapping_sub(0x30u64), r14); crate::simd_ops::wu64(r11.wrapping_sub(0x38u64), r15); bb = 3; continue; }
            3 => { r9 = ((crate::simd_ops::ru16(rbp)) as u32) as u32 as u64; rbx = (crate::simd_ops::ru64(r8.wrapping_add(r9.wrapping_mul(8)).wrapping_add(0x48u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x118u64), rbx); fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rdx = (crate::simd_ops::ru32(r8.wrapping_add(r9.wrapping_mul(4)).wrapping_add(0x10048u64))) as u32 as u64; bb = 6; continue; }
            5 => { rdx = 0; fl_l = 0; fl_r = 0; bb = 6; continue; }
            6 => { rcx = ((rdx as u32)) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rcx); fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 25; continue; } bb = 7; continue; }
            7 => { rcx = rsp.wrapping_add(0x40); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 9; continue; } bb = 8; continue; }
            8 => { r12 = 0; fl_l = 0; fl_r = 0; bb = 10; continue; }
            9 => { r12 = ((rax.wrapping_add(rbx)) as u64) as u64; bb = 10; continue; }
            10 => { r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; fl_l = r15; fl_r = r15; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 11; continue; }
            11 => { fl_l = ((r15).wrapping_add(rbx)) as u64; fl_r = fl_l; r15 = ((fl_l as u64)) as u64; bb = 12; continue; }
            12 => { r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; fl_l = r14; fl_r = r14; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 14; continue; } bb = 13; continue; }
            13 => { fl_l = ((r14).wrapping_add(rbx)) as u64; fl_r = fl_l; r14 = ((fl_l as u64)) as u64; bb = 14; continue; }
            14 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 16; continue; } bb = 15; continue; }
            15 => { fl_l = ((rsi).wrapping_add(rbx)) as u64; fl_r = fl_l; rsi = ((fl_l as u64)) as u64; bb = 16; continue; }
            16 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 18; continue; } bb = 17; continue; }
            17 => { fl_l = ((rdi).wrapping_add(rbx)) as u64; fl_r = fl_l; rdi = ((fl_l as u64)) as u64; bb = 18; continue; }
            18 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 20; continue; } bb = 19; continue; }
            19 => { fl_l = ((rbx).wrapping_add(crate::simd_ops::ru64(rsp.wrapping_add(0x118u64)))) as u64; fl_r = fl_l; rbx = ((fl_l as u64)) as u64; bb = 20; continue; }
            20 => { rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); fl_l = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; fl_r = rax; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 24; continue; } bb = 21; continue; }
            21 => { rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x118u64))) as u64; bb = 22; continue; }
            22 => { crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), r12); rcx = rsp.wrapping_add(0x80); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), r15); rdx = r13; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), r14); crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rsi); let y_addr: u64 = rsp.wrapping_add(0x40u64); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), rdi); crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x78u64), rax); let y_addr: u64 = rsp.wrapping_add(0x60u64); xmm1 = crate::simd_ops::ru128(y_addr); ymm1_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rsp.wrapping_add(0x80u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); let y_addr: u64 = rsp.wrapping_add(0xa0u64); crate::simd_ops::wu128(y_addr, xmm1); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm1_hi); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; rax = ((rax).wrapping_add(1)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); fl_l = rax; fl_r = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 22; continue; } bb = 23; continue; }
            23 => { rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; bb = 24; continue; }
            24 => { r8 = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x108u64))) as u64; bb = 25; continue; }
            25 => { rbp = ((rbp).wrapping_add(0x2u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rbp); fl_l = rbp; fl_r = rax; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } bb = 26; continue; }
            26 => { r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0xc0u64))) as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0xc8u64))) as u64; r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0xd8u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0xe0u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0xe8u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x100u64))) as u64; bb = 27; continue; }
            27 => { r13 = (crate::simd_ops::ru64(rsp.wrapping_add(0xd0u64))) as u64; bb = 28; continue; }
            28 => { rsp = ((rsp).wrapping_add(0xf0u64)) as u64; rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_64227<const P0: u64, const P1: u64, const P2: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0; let mut ymm1_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 37>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { r11 = rsp; rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rbp = r11.wrapping_sub(0x28); rsp = ((rsp).wrapping_sub(0x118u64)) as u64; rax = (((r8 as u16)) as u32) as u32 as u64; r13 = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = (crate::simd_ops::ru64(rdx)) as u64; r13 = ((r13).wrapping_add(r13)) as u64; r8 = (crate::simd_ops::ru32(rax.wrapping_add(r13.wrapping_mul(8)).wrapping_add(0x8u64))) as u32 as u64; r13 = (crate::simd_ops::ru64(rax.wrapping_add(r13.wrapping_mul(8)))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), r13); fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 27; continue; } bb = 1; continue; }
            1 => { rdx = (crate::simd_ops::ru64(rcx)) as u64; r9 = ((r8.wrapping_mul(2)) as u64) as u64; rcx = ((crate::simd_ops::ru16(P0)) as u32) as u32 as u64; r9 = ((r9).wrapping_add(r13)) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x48u64), r9); rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x580e0u64))) as u64; r10 = rdx.wrapping_add(0x40); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), r10); rax = (crate::simd_ops::ru64(rax.wrapping_add(rcx.wrapping_mul(8)))) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x70u64), rax); fl_l = r13; fl_r = r9; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 27; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::wu64(r11.wrapping_add(0x8u64), rbx); rax = r9; crate::simd_ops::wu64(r11.wrapping_sub(0x18u64), rsi); crate::simd_ops::wu64(r11.wrapping_sub(0x20u64), rdi); crate::simd_ops::wu64(r11.wrapping_sub(0x28u64), r12); crate::simd_ops::wu64(r11.wrapping_sub(0x30u64), r14); crate::simd_ops::wu64(r11.wrapping_sub(0x38u64), r15); bb = 3; continue; }
            3 => { r9 = ((crate::simd_ops::ru16(r13)) as u32) as u32 as u64; rcx = (crate::simd_ops::ru64(r10.wrapping_add(r9.wrapping_mul(8)).wrapping_add(0x48u64))) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x38u64), rcx); fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rcx = (crate::simd_ops::ru32(r10.wrapping_add(r9.wrapping_mul(4)).wrapping_add(0x10048u64))) as u32 as u64; bb = 5; continue; }
            5 => { crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rcx); fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 25; continue; } bb = 6; continue; }
            6 => { r8 = r10; rcx = rsp.wrapping_add(0x48); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x38u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 7; continue; }
            7 => { fl_l = ((rax).wrapping_add(rcx)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; bb = 8; continue; }
            8 => { r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); fl_l = r12; fl_r = r12; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 9; continue; }
            9 => { fl_l = ((r12).wrapping_add(rcx)) as u64; fl_r = fl_l; r12 = ((fl_l as u64)) as u64; bb = 10; continue; }
            10 => { r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; fl_l = r15; fl_r = r15; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 11; continue; }
            11 => { fl_l = ((r15).wrapping_add(rcx)) as u64; fl_r = fl_l; r15 = ((fl_l as u64)) as u64; bb = 12; continue; }
            12 => { r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; fl_l = r14; fl_r = r14; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 14; continue; } bb = 13; continue; }
            13 => { fl_l = ((r14).wrapping_add(rcx)) as u64; fl_r = fl_l; r14 = ((fl_l as u64)) as u64; bb = 14; continue; }
            14 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 16; continue; } bb = 15; continue; }
            15 => { fl_l = ((rsi).wrapping_add(rcx)) as u64; fl_r = fl_l; rsi = ((fl_l as u64)) as u64; bb = 16; continue; }
            16 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 18; continue; } bb = 17; continue; }
            17 => { fl_l = ((rdi).wrapping_add(rcx)) as u64; fl_r = fl_l; rdi = ((fl_l as u64)) as u64; bb = 18; continue; }
            18 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 20; continue; } bb = 19; continue; }
            19 => { fl_l = ((rbx).wrapping_add(rcx)) as u64; fl_r = fl_l; rbx = ((fl_l as u64)) as u64; bb = 20; continue; }
            20 => { rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax); fl_l = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; fl_r = rax; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 24; continue; } bb = 21; continue; }
            21 => { r13 = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; bb = 22; continue; }
            22 => { rdx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x70u64))) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x80u64), rcx); rcx = rbp.wrapping_sub(0x60); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), r13); crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), r12); crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), r15); crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), r14); let y_addr: u64 = rsp.wrapping_add(0x48u64); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rbp.wrapping_sub(0x60u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), rdi); crate::simd_ops::wu64(rsp.wrapping_add(0x78u64), rbx); let y_addr: u64 = rsp.wrapping_add(0x68u64); xmm1 = crate::simd_ops::ru128(y_addr); ymm1_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); crate::simd_ops::wu64(rbp.wrapping_sub(0x78u64), rax); xmm0 = ((crate::simd_ops::ru64(rbp.wrapping_sub(0x78u64))) as u64) as u128; ymm0_hi = 0; crate::simd_ops::wu64(rbp.wrapping_sub(0x20u64), ((xmm0 as u64)) as u64); let y_addr: u64 = rbp.wrapping_sub(0x40u64); crate::simd_ops::wu128(y_addr, xmm1); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm1_hi); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x38u64))) as u64; rax = ((rax).wrapping_add(1)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax); fl_l = rax; fl_r = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 22; continue; } bb = 23; continue; }
            23 => { r13 = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; bb = 24; continue; }
            24 => { r10 = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x48u64))) as u64; bb = 25; continue; }
            25 => { r13 = ((r13).wrapping_add(0x2u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), r13); fl_l = r13; fl_r = rax; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } bb = 26; continue; }
            26 => { r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0xf0u64))) as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0xf8u64))) as u64; r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x100u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x108u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x110u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x130u64))) as u64; bb = 27; continue; }
            27 => { rsp = ((rsp).wrapping_add(0x118u64)) as u64; r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_64228<const P0: u64, const P1: u64, const P2: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 39>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { r11 = rsp; rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rbp = r11.wrapping_sub(0x38); rsp = ((rsp).wrapping_sub(0x128u64)) as u64; rax = (((r8 as u16)) as u32) as u32 as u64; r15 = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = (crate::simd_ops::ru64(rdx)) as u64; r15 = ((r15).wrapping_add(r15)) as u64; r10 = (crate::simd_ops::ru32(rax.wrapping_add(r15.wrapping_mul(8)).wrapping_add(0x8u64))) as u32 as u64; r15 = (crate::simd_ops::ru64(rax.wrapping_add(r15.wrapping_mul(8)))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), r15); fl_l = r10; fl_r = r10; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 26; continue; } bb = 1; continue; }
            1 => { r9 = (crate::simd_ops::ru64(rcx)) as u64; rdx = ((crate::simd_ops::ru16(P0)) as u32) as u32 as u64; crate::simd_ops::wu64(r11.wrapping_sub(0x30u64), r13); r13 = ((r15.wrapping_add(r10.wrapping_mul(2))) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), r9); rax = (crate::simd_ops::ru64(r9)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), r13); rcx = (crate::simd_ops::ru64(rax.wrapping_add(0x20u64))) as u64; rcx = ((crate::simd_ops::ru16(rcx.wrapping_add(rdx.wrapping_mul(2)))) as u32) as u32 as u64; rdx = r9.wrapping_add(0x40); rcx = ((rcx).wrapping_shl(4u32)) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(r9.wrapping_add(0x8u64)))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdx); rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x8u64))) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x78u64), rax); rax = (crate::simd_ops::ru64(rcx)) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x80u64), rax); fl_l = r15; fl_r = r13; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 25; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::wu64(r11.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(r11.wrapping_sub(0x18u64), rsi); crate::simd_ops::wu64(r11.wrapping_sub(0x20u64), rdi); crate::simd_ops::wu64(r11.wrapping_sub(0x28u64), r12); crate::simd_ops::wu64(r11.wrapping_sub(0x38u64), r14); bb = 3; continue; }
            3 => { r9 = ((crate::simd_ops::ru16(r15)) as u32) as u32 as u64; rbx = (crate::simd_ops::ru64(rdx.wrapping_add(r9.wrapping_mul(8)).wrapping_add(0x48u64))) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x48u64), rbx); fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rcx = (crate::simd_ops::ru32(rdx.wrapping_add(r9.wrapping_mul(4)).wrapping_add(0x10048u64))) as u32 as u64; bb = 6; continue; }
            5 => { rcx = 0; fl_l = 0; fl_r = 0; bb = 6; continue; }
            6 => { rax = ((rcx as u32)) as u32 as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x58u64), rax); fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 23; continue; } bb = 7; continue; }
            7 => { r8 = rdx; rcx = rsp.wrapping_add(0x40); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 9; continue; } bb = 8; continue; }
            8 => { r12 = 0; fl_l = 0; fl_r = 0; bb = 10; continue; }
            9 => { r12 = ((rax.wrapping_add(rbx)) as u64) as u64; bb = 10; continue; }
            10 => { r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; fl_l = r14; fl_r = r14; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 11; continue; }
            11 => { fl_l = ((r14).wrapping_add(rbx)) as u64; fl_r = fl_l; r14 = ((fl_l as u64)) as u64; bb = 12; continue; }
            12 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 14; continue; } bb = 13; continue; }
            13 => { fl_l = ((rsi).wrapping_add(rbx)) as u64; fl_r = fl_l; rsi = ((fl_l as u64)) as u64; bb = 14; continue; }
            14 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 16; continue; } bb = 15; continue; }
            15 => { fl_l = ((rdi).wrapping_add(rbx)) as u64; fl_r = fl_l; rdi = ((fl_l as u64)) as u64; bb = 16; continue; }
            16 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 18; continue; } bb = 17; continue; }
            17 => { fl_l = ((rbx).wrapping_add(crate::simd_ops::ru64(rbp.wrapping_add(0x48u64)))) as u64; fl_r = fl_l; rbx = ((fl_l as u64)) as u64; bb = 18; continue; }
            18 => { fl_l = (crate::simd_ops::ru64(rbp.wrapping_add(0x58u64))) as u64; fl_r = (0x0u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (0x0u64) as u64); if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 22; continue; } bb = 19; continue; }
            19 => { r13 = (crate::simd_ops::ru64(rbp.wrapping_add(0x48u64))) as u64; r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; bb = 20; continue; }
            20 => { xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x70u64))) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm0) as u128); xmm0 = ((crate::simd_ops::ru64(rbp.wrapping_sub(0x80u64))) as u64) as u128; ymm0_hi = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), ((xmm0 as u64)) as u64); crate::simd_ops::wu64(rbp.wrapping_sub(0x78u64), r12); rdx = rsp.wrapping_add(0x40); crate::simd_ops::wu64(rbp.wrapping_sub(0x70u64), r14); rcx = rbp.wrapping_sub(0x40); crate::simd_ops::wu64(rbp.wrapping_sub(0x68u64), rsi); crate::simd_ops::wu64(rbp.wrapping_sub(0x60u64), rdi); let y_addr: u64 = rbp.wrapping_sub(0x78u64); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rbp.wrapping_sub(0x40u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); crate::simd_ops::wu64(rbp.wrapping_sub(0x58u64), rbx); crate::simd_ops::wu64(rbp.wrapping_sub(0x50u64), r13); xmm1 = (crate::simd_ops::ru128(rbp.wrapping_sub(0x58u64))) as u128; crate::simd_ops::wu64(rbp.wrapping_sub(0x48u64), r15); xmm0 = ((crate::simd_ops::ru64(rbp.wrapping_sub(0x48u64))) as u64) as u128; ymm0_hi = 0; crate::simd_ops::wu64(rbp.wrapping_sub(0x10u64), ((xmm0 as u64)) as u64); crate::simd_ops::wu128(rbp.wrapping_sub(0x20u64), (xmm1) as u128); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r15 = ((r15).wrapping_add(1)) as u64; fl_l = r15; fl_r = (crate::simd_ops::ru64(rbp.wrapping_add(0x58u64))) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 20; continue; } bb = 21; continue; }
            21 => { r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; r13 = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; bb = 22; continue; }
            22 => { rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; bb = 23; continue; }
            23 => { r15 = ((r15).wrapping_add(0x2u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), r15); fl_l = r15; fl_r = r13; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } bb = 24; continue; }
            24 => { r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x100u64))) as u64; r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x110u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x118u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x120u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x140u64))) as u64; bb = 25; continue; }
            25 => { r13 = (crate::simd_ops::ru64(rsp.wrapping_add(0x108u64))) as u64; bb = 26; continue; }
            26 => { rsp = ((rsp).wrapping_add(0x128u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_64337<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdx = (((r9) as i64).wrapping_mul((0xf8u64) as i64) as u64) as u64; rcx = 0x11; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rdi); fl_l = rdx; fl_r = rdx; rdi = r9; rax = ((rdx.wrapping_add(P0)) as u64) as u64; rdx = 0x10; rcx = (if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { rax } else { rcx }) as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); rcx = rax; rdx = 0xdeadd00d; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); rax = 0; fl_l = 0; fl_r = 0; bb = 2; continue; }
            2 => { crate::simd_ops::wu64(rax, (0xf8u64) as u64); rsi = ((rax.wrapping_add(P0)) as u64) as u64; crate::simd_ops::wu32(rax.wrapping_add(P4), ((rdi as u32)) as u32); crate::simd_ops::wu32(rax.wrapping_add(P5), (0x8u32) as u32); fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 3; continue; }
            3 => { crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbx); rbx = rsi; bb = 4; continue; }
            4 => { rdx = 0; fl_l = 0; fl_r = 0; r8 = 0xf8; rcx = rbx; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); rcx = rbx; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); rbx = ((rbx).wrapping_add(0xf8u64)) as u64; fl_l = rdi; fl_r = (0x1u64) as u64; rdi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 4; continue; } bb = 5; continue; }
            5 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; bb = 6; continue; }
            6 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rax = rsi; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_64455<const P0: u64, const P1: u64, const P2: u64, const P3: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rsi = rdx;
    rdi = rcx;
    fl_l = rcx; fl_r = rdx;
    if !((((fl_l as u64)) == ((fl_r as u64)))) { crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbx); rdx = rcx; rbx = (crate::simd_ops::ru64(rcx)) as u64; r8 = (crate::simd_ops::ru64(rbx.wrapping_add(P0))) as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx.wrapping_add(P0), rbx); rdx = rsi; crate::simd_ops::wu64(rbx, rbx); rcx = rdi; crate::simd_ops::wu64(rbx.wrapping_add(P2), rbx); crate::simd_ops::wu64(rdi.wrapping_add(P0), (0x0u64) as u64); rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; }
    rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64;
    rax = rdi;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_64494<const P0: u64, const P1: u64, const P2: u64, const P3: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 15>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x68u64)) as u64; rbx = (crate::simd_ops::ru64(rcx)) as u64; rdi = 0; fl_l = 0; fl_r = 0; rax = (((rdx as u16)) as u32) as u32 as u64; r8 = (crate::simd_ops::ru64(rbx.wrapping_add(rax.wrapping_mul(8)).wrapping_add(P0))) as u64; fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(rax.wrapping_mul(4)).wrapping_add(P1))) as u32 as u64; bb = 3; continue; }
            2 => { rax = ((rdi as u32)) as u32 as u64; bb = 3; continue; }
            3 => { r9 = ((rax as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 9; continue; } bb = 4; continue; }
            4 => { rcx = rsp.wrapping_add(0x30); crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), r15); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbx); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; fl_l = r15; fl_r = r15; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 5; continue; }
            5 => { crate::simd_ops::wu64(rsp.wrapping_add(0x80u64), rbp); rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x88u64), rsi); rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x90u64), r14); r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; bb = 6; continue; }
            6 => { crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbp); rcx = rsp.wrapping_add(0x30); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rsi); rdx = rbx; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), r14); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rdi); let y_addr: u64 = rsp.wrapping_add(0x30u64); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rsp.wrapping_add(0x30u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdi = ((rdi).wrapping_add(1)) as u64; fl_l = rdi; fl_r = r15; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 6; continue; } bb = 7; continue; }
            7 => { r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x90u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x88u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; bb = 8; continue; }
            8 => { r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; bb = 9; continue; }
            9 => { rsp = ((rsp).wrapping_add(0x68u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_64499<const P0: u64, const P1: u64, const P2: u64, const P3: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { r11 = rsp; rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x60u64)) as u64; rax = (((r8 as u16)) as u32) as u32 as u64; rdi = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = (crate::simd_ops::ru64(rdx)) as u64; rdi = ((rdi).wrapping_add(rdi)) as u64; rdx = (crate::simd_ops::ru32(rax.wrapping_add(rdi.wrapping_mul(8)).wrapping_add(0x8u64))) as u32 as u64; rdi = (crate::simd_ops::ru64(rax.wrapping_add(rdi.wrapping_mul(8)))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 19; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(r11.wrapping_add(0x10u64), rbp); rbp = ((rdi.wrapping_add(rdx.wrapping_mul(2))) as u64) as u64; crate::simd_ops::wu64(r11.wrapping_add(0x18u64), rsi); rsi = (crate::simd_ops::ru64(rcx)) as u64; fl_l = rdi; fl_r = rbp; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 18; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::wu64(r11.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(r11.wrapping_sub(0x10u64), r12); crate::simd_ops::wu64(r11.wrapping_sub(0x18u64), r13); crate::simd_ops::wu64(r11.wrapping_sub(0x20u64), r14); crate::simd_ops::wu64(r11.wrapping_sub(0x28u64), r15); bb = 3; continue; }
            3 => { r11 = ((crate::simd_ops::ru16(rdi)) as u32) as u32 as u64; r12 = (crate::simd_ops::ru64(rsi.wrapping_add(r11.wrapping_mul(8)).wrapping_add(0x88u64))) as u64; fl_l = r12; fl_r = r12; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru32(rsi.wrapping_add(r11.wrapping_mul(4)).wrapping_add(0x10088u64))) as u32 as u64; bb = 6; continue; }
            5 => { rax = 0; fl_l = 0; fl_r = 0; bb = 6; continue; }
            6 => { r13 = ((rax as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 16; continue; } bb = 7; continue; }
            7 => { r8 = ((crate::simd_ops::ru16(P0)) as u32) as u32 as u64; rax = r11; rax = ((rax).wrapping_shl(5u32)) as u64; r14 = (crate::simd_ops::ru32(rax.wrapping_add(rsi).wrapping_add(0x180acu64))) as u32 as u64; r15 = (crate::simd_ops::ru64(rax.wrapping_add(rsi).wrapping_add(0x18098u64))) as u64; rdx = ((r14 as u32)) as u32 as u64; rcx = r15; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = ((((rax as u32)) as i32 as i64) as u64) as u64; fl_l = rcx; fl_r = (0xffffffffffffffffu64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 9; continue; } bb = 8; continue; }
            8 => { rax = r11.wrapping_add(0xc03); fl_l = ((rax).wrapping_shl(5u32)) as u64; rax = ((fl_l as u64)) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(rsi).wrapping_add(0x40u64))) as u64; rbx = (crate::simd_ops::ru32(rax.wrapping_add(rcx.wrapping_mul(4)))) as u32 as u64; bb = 10; continue; }
            9 => { rbx = 0; fl_l = 0; fl_r = 0; bb = 10; continue; }
            10 => { r8 = ((crate::simd_ops::ru16(P2)) as u32) as u32 as u64; rax = 0xc03; rdx = r14; rcx = r15; r11 = ((r11).wrapping_add(rax)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = ((((rax as u32)) as i32 as i64) as u64) as u64; fl_l = rcx; fl_r = (0xffffffffffffffffu64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 12; continue; } bb = 11; continue; }
            11 => { fl_l = ((r11).wrapping_shl(5u32)) as u64; r11 = ((fl_l as u64)) as u64; rax = (crate::simd_ops::ru64(r11.wrapping_add(rsi).wrapping_add(0x40u64))) as u64; r15 = (crate::simd_ops::ru32(rax.wrapping_add(rcx.wrapping_mul(4)))) as u32 as u64; bb = 13; continue; }
            12 => { r15 = 0; fl_l = 0; fl_r = 0; bb = 13; continue; }
            13 => { fl_l = rbx; fl_r = rbx; rax = ((rbx.wrapping_add(r12)) as u64) as u64; rbx = (if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { rax } else { rbx }) as u64; fl_l = r15; fl_r = r15; rax = ((r15.wrapping_add(r12)) as u64) as u64; r15 = (if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { rax } else { r15 }) as u64; r14 = 0; fl_l = 0; fl_r = 0; fl_l = r13; fl_r = r13; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 16; continue; } bb = 14; continue; }
            14 => { bb = 15; continue; }
            15 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbx); rcx = rsp.wrapping_add(0x20); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), r15); rdx = rsi; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), r12); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), r14); let y_addr: u64 = rsp.wrapping_add(0x20u64); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rsp.wrapping_add(0x20u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r14 = ((r14).wrapping_add(1)) as u64; fl_l = r14; fl_r = r13; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 15; continue; } bb = 16; continue; }
            16 => { rdi = ((rdi).wrapping_add(0x2u64)) as u64; fl_l = rdi; fl_r = rbp; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } bb = 17; continue; }
            17 => { r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; r13 = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; bb = 18; continue; }
            18 => { rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; bb = 19; continue; }
            19 => { rsp = ((rsp).wrapping_add(0x60u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_64515<const P0: u64, const P1: u64, const P2: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 17>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x80u64)) as u64; rdi = rcx; r9 = ((rdx as u32)) as u32 as u64; rcx = (crate::simd_ops::ru64(r8)) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x20u64))) as u32 as u64; fl_l = r9; fl_r = rax; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 26; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x18u64))) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(r9.wrapping_mul(8)))) as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((r9 as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 26; continue; } bb = 2; continue; }
            2 => { rax = rdx; rcx = ((rcx).wrapping_shr(32u32)) as u64; rax = ((rax).wrapping_shr(32u32)) as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 26; continue; } bb = 3; continue; }
            3 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0xffffffff; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 26; continue; } bb = 4; continue; }
            4 => { xmm1 = (crate::simd_ops::ru128(r8)) as u128; xmm0 = ((crate::simd_ops::ru64(r8.wrapping_add(0x10u64))) as u64) as u128; ymm0_hi = 0; r11 = ((xmm1 as u64)) as u64; rax = 0xffff; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), ((xmm0 as u64)) as u64); rdx = (crate::simd_ops::ru64(r11.wrapping_add(0x30u64))) as u64; rdx = (crate::simd_ops::ru64(rdx.wrapping_add(r9.wrapping_mul(8)))) as u64; fl_l = ((rdx as u16)) as u16 as u64; fl_r = ((rax as u16)) as u16 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { bb = 6; continue; } bb = 5; continue; }
            5 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::lift_ops::trap(0x80000003, P2, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            6 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rcx = (((xmm1 >> 64) as u64)) as u64; r10 = ((rax.wrapping_add(rcx.wrapping_mul(2))) as u64) as u64; r8 = r10; r8 = ((r8).wrapping_sub(rax)) as u64; r8 = ((((r8) as i64).wrapping_shr(1u32) as u64)) as u64; fl_l = r8; fl_r = r8; if (((((fl_l as u64)) & ((fl_r as u64))) as i64) <= 0) { bb = 12; continue; } bb = 7; continue; }
            7 => { bb = 8; continue; }
            8 => { r9 = r8; r9 = ((r9).wrapping_shr(1u32)) as u64; fl_l = (crate::simd_ops::ru16(rax.wrapping_add(r9.wrapping_mul(2)))) as u16 as u64; fl_r = ((rdx as u16)) as u16 as u64; rcx = ((rax.wrapping_add(r9.wrapping_mul(2))) as u64) as u64; if (((fl_l as u16)) >= ((fl_r as u16))) { bb = 10; continue; } bb = 9; continue; }
            9 => { rax = (0xffffffffffffffffu64) as u64; rax = ((rax).wrapping_sub(r9)) as u64; fl_l = ((r8).wrapping_add(rax)) as u64; fl_r = fl_l; r8 = ((fl_l as u64)) as u64; rax = rcx.wrapping_add(0x2); bb = 11; continue; }
            10 => { r8 = r9; bb = 11; continue; }
            11 => { fl_l = r8; fl_r = r8; if (((((fl_l as u64)) & ((fl_r as u64))) as i64) > 0) { bb = 8; continue; } bb = 12; continue; }
            12 => { fl_l = rax; fl_r = r10; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 25; continue; } bb = 13; continue; }
            13 => { fl_l = ((rdx as u16)) as u16 as u64; fl_r = (crate::simd_ops::ru16(rax)) as u16 as u64; if (((fl_l as u16)) < ((fl_r as u16))) { bb = 25; continue; } bb = 14; continue; }
            14 => { r9 = (((rdx as u16)) as u32) as u32 as u64; r8 = r11.wrapping_add(0x40); crate::simd_ops::wu64(rsp.wrapping_add(0x90u64), rbx); rcx = rsp.wrapping_add(0x40); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdx); rbx = (crate::simd_ops::ru64(r8.wrapping_add(r9.wrapping_mul(8)).wrapping_add(0x48u64))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r9 = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; fl_l = r9; fl_r = r9; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 16; continue; } bb = 15; continue; }
            15 => { fl_l = ((r9).wrapping_add(rbx)) as u64; fl_r = fl_l; r9 = ((fl_l as u64)) as u64; bb = 16; continue; }
            16 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 18; continue; } bb = 17; continue; }
            17 => { fl_l = ((rax).wrapping_add(rbx)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; bb = 18; continue; }
            18 => { rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 20; continue; } bb = 19; continue; }
            19 => { fl_l = ((rcx).wrapping_add(rbx)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; bb = 20; continue; }
            20 => { rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 22; continue; } bb = 21; continue; }
            21 => { fl_l = ((rdx).wrapping_add(rbx)) as u64; fl_r = fl_l; rdx = ((fl_l as u64)) as u64; bb = 22; continue; }
            22 => { r8 = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 24; continue; } bb = 23; continue; }
            23 => { fl_l = ((r8).wrapping_add(rbx)) as u64; fl_r = fl_l; r8 = ((fl_l as u64)) as u64; bb = 24; continue; }
            24 => { crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rax); rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x24u64))) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), rax); rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), rbx); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x90u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), r9); crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rcx); crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rdx); let y_addr: u64 = rsp.wrapping_add(0x40u64); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rdi; crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); xmm0 = ((crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64) as u128; ymm0_hi = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), r8); xmm1 = (crate::simd_ops::ru128(rsp.wrapping_add(0x60u64))) as u128; crate::simd_ops::wu128(rdi.wrapping_add(0x20u64), (xmm1) as u128); crate::simd_ops::wu64(rdi.wrapping_add(0x30u64), ((xmm0 as u64)) as u64); crate::simd_ops::wu8(rdi.wrapping_add(0x38u64), ((rax as u8)) as u8); rax = rdi; rsp = ((rsp).wrapping_add(0x80u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            25 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; crate::simd_ops::wu8(rdi.wrapping_add(0x38u64), ((rax as u8)) as u8); rax = rdi; rsp = ((rsp).wrapping_add(0x80u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            26 => { crate::simd_ops::wu8(rdi.wrapping_add(0x38u64), (0x0u8) as u8); rax = rdi; rsp = ((rsp).wrapping_add(0x80u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_64609<const P0: u64, const P1: u64, const P2: u64, const P3: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0; let mut ymm1_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 37>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { r11 = rsp; rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rbp = r11.wrapping_sub(0x28); rsp = ((rsp).wrapping_sub(0x118u64)) as u64; rax = (((r8 as u16)) as u32) as u32 as u64; r13 = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = (crate::simd_ops::ru64(rdx)) as u64; r13 = ((r13).wrapping_add(r13)) as u64; r9 = (crate::simd_ops::ru32(rax.wrapping_add(r13.wrapping_mul(8)).wrapping_add(0x8u64))) as u32 as u64; r13 = (crate::simd_ops::ru64(rax.wrapping_add(r13.wrapping_mul(8)))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), r13); fl_l = r9; fl_r = r9; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 27; continue; } bb = 1; continue; }
            1 => { r8 = (crate::simd_ops::ru64(rcx)) as u64; r9 = ((r9.wrapping_mul(2)) as u64) as u64; rax = ((crate::simd_ops::ru16(P0)) as u32) as u32 as u64; r9 = ((r9).wrapping_add(r13)) as u64; rcx = ((crate::simd_ops::ru16(P1)) as u32) as u32 as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x48u64), r9); rdx = (crate::simd_ops::ru64(r8.wrapping_add(0x580e0u64))) as u64; r8 = ((r8).wrapping_add(0x40u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), r8); rax = (crate::simd_ops::ru64(rdx.wrapping_add(rax.wrapping_mul(8)))) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x68u64), rax); rax = (crate::simd_ops::ru64(rdx.wrapping_add(rcx.wrapping_mul(8)))) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x70u64), rax); fl_l = r13; fl_r = r9; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 27; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::wu64(r11.wrapping_add(0x8u64), rbx); rax = r9; crate::simd_ops::wu64(r11.wrapping_sub(0x18u64), rsi); crate::simd_ops::wu64(r11.wrapping_sub(0x20u64), rdi); crate::simd_ops::wu64(r11.wrapping_sub(0x28u64), r12); crate::simd_ops::wu64(r11.wrapping_sub(0x30u64), r14); crate::simd_ops::wu64(r11.wrapping_sub(0x38u64), r15); bb = 3; continue; }
            3 => { r9 = ((crate::simd_ops::ru16(r13)) as u32) as u32 as u64; rcx = (crate::simd_ops::ru64(r8.wrapping_add(r9.wrapping_mul(8)).wrapping_add(0x48u64))) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x38u64), rcx); fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rcx = (crate::simd_ops::ru32(r8.wrapping_add(r9.wrapping_mul(4)).wrapping_add(0x10048u64))) as u32 as u64; bb = 5; continue; }
            5 => { crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rcx); fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 25; continue; } bb = 6; continue; }
            6 => { rcx = rsp.wrapping_add(0x48); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x38u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 7; continue; }
            7 => { fl_l = ((rax).wrapping_add(rcx)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; bb = 8; continue; }
            8 => { r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); fl_l = r12; fl_r = r12; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 9; continue; }
            9 => { fl_l = ((r12).wrapping_add(rcx)) as u64; fl_r = fl_l; r12 = ((fl_l as u64)) as u64; bb = 10; continue; }
            10 => { r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; fl_l = r15; fl_r = r15; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 11; continue; }
            11 => { fl_l = ((r15).wrapping_add(rcx)) as u64; fl_r = fl_l; r15 = ((fl_l as u64)) as u64; bb = 12; continue; }
            12 => { r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; fl_l = r14; fl_r = r14; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 14; continue; } bb = 13; continue; }
            13 => { fl_l = ((r14).wrapping_add(rcx)) as u64; fl_r = fl_l; r14 = ((fl_l as u64)) as u64; bb = 14; continue; }
            14 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 16; continue; } bb = 15; continue; }
            15 => { fl_l = ((rsi).wrapping_add(rcx)) as u64; fl_r = fl_l; rsi = ((fl_l as u64)) as u64; bb = 16; continue; }
            16 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 18; continue; } bb = 17; continue; }
            17 => { fl_l = ((rdi).wrapping_add(rcx)) as u64; fl_r = fl_l; rdi = ((fl_l as u64)) as u64; bb = 18; continue; }
            18 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 20; continue; } bb = 19; continue; }
            19 => { fl_l = ((rbx).wrapping_add(rcx)) as u64; fl_r = fl_l; rbx = ((fl_l as u64)) as u64; bb = 20; continue; }
            20 => { rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax); fl_l = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; fl_r = rax; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 24; continue; } bb = 21; continue; }
            21 => { r13 = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; bb = 22; continue; }
            22 => { r8 = (crate::simd_ops::ru64(rbp.wrapping_sub(0x70u64))) as u64; rdx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x68u64))) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x80u64), rcx); rcx = rbp.wrapping_sub(0x60); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), r13); crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), r12); crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), r15); crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), r14); let y_addr: u64 = rsp.wrapping_add(0x48u64); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rbp.wrapping_sub(0x60u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), rdi); crate::simd_ops::wu64(rsp.wrapping_add(0x78u64), rbx); let y_addr: u64 = rsp.wrapping_add(0x68u64); xmm1 = crate::simd_ops::ru128(y_addr); ymm1_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); crate::simd_ops::wu64(rbp.wrapping_sub(0x78u64), rax); xmm0 = ((crate::simd_ops::ru64(rbp.wrapping_sub(0x78u64))) as u64) as u128; ymm0_hi = 0; crate::simd_ops::wu64(rbp.wrapping_sub(0x20u64), ((xmm0 as u64)) as u64); let y_addr: u64 = rbp.wrapping_sub(0x40u64); crate::simd_ops::wu128(y_addr, xmm1); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm1_hi); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x38u64))) as u64; rax = ((rax).wrapping_add(1)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax); fl_l = rax; fl_r = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 22; continue; } bb = 23; continue; }
            23 => { r13 = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; bb = 24; continue; }
            24 => { r8 = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x48u64))) as u64; bb = 25; continue; }
            25 => { r13 = ((r13).wrapping_add(0x2u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), r13); fl_l = r13; fl_r = rax; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } bb = 26; continue; }
            26 => { r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0xf0u64))) as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0xf8u64))) as u64; r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x100u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x108u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x110u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x130u64))) as u64; bb = 27; continue; }
            27 => { rsp = ((rsp).wrapping_add(0x118u64)) as u64; r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_64713<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; fl_l = (crate::simd_ops::ru8(r8.wrapping_add(P0))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; rbx = r8; rbp = rdx; r14 = rcx; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 4; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rdi); bb = 2; continue; }
            2 => { r8 = (crate::simd_ops::ru64(rbx.wrapping_add(P1))) as u64; rdx = rbp; rcx = r14; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rsi = rbx; rbx = (crate::simd_ops::ru64(rbx)) as u64; rcx = ((rsi.wrapping_add(P3)) as u64) as u64; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rcx = ((rsi.wrapping_add(P5)) as u64) as u64; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rdx = rsi; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(P0))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 2; continue; } bb = 3; continue; }
            3 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; bb = 4; continue; }
            4 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_64835<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rsi = rdx; rbx = rcx; fl_l = rcx; fl_r = rdx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 5; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rdi); rdi = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rcx.wrapping_add(P0), rdi); crate::simd_ops::wu32(rcx.wrapping_add(P1), ((rdi as u32)) as u32); rdx = (crate::simd_ops::ru32(rdx.wrapping_add(P2))) as u32 as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu32(rbx.wrapping_add(P0), ((rdi as u32)) as u32); rax = (crate::simd_ops::ru32(rsi.wrapping_add(P1))) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(P4), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rsi.wrapping_add(P1))) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(P1), ((rax as u32)) as u32); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 4; continue; } bb = 2; continue; }
            2 => { bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru32(rsi.wrapping_add(P2))) as u32 as u64; rdx = 0; fl_l = 0; fl_r = 0; rax = (crate::simd_ops::ru32(rsi.wrapping_add(P0))) as u32 as u64; rax = ((rax).wrapping_add(rdi)) as u64; let d = (rcx) as u128; let n = ((rdx as u128) << 64) | (rax as u128); if d != 0 { rax = (n / d) as u64; rdx = (n % d) as u64; } rax = (crate::simd_ops::ru64(rsi)) as u64; rcx = (crate::simd_ops::ru64(rbx)) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(rdx.wrapping_mul(8)))) as u64; crate::simd_ops::wu64(rcx.wrapping_add(rdi.wrapping_mul(8)), rax); rdi = ((rdi).wrapping_add(1)) as u64; rax = (crate::simd_ops::ru32(rbx.wrapping_add(P1))) as u32 as u64; fl_l = rdi; fl_r = rax; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 3; continue; } bb = 4; continue; }
            4 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = rbx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            5 => { rax = rbx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_64850<const P0: u64, const P1: u64, const P2: u64, const P3: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0; let mut ymm1_hi: u128 = 0; let mut ymm2_hi: u128 = 0; let mut ymm3_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 45>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rbp = rsp.wrapping_sub(0x60); rsp = ((rsp).wrapping_sub(0x160u64)) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x78u64), (0xffffffffffffffffu64) as u64); rsi = rcx; fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0xffffffff; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 3; continue; } bb = 1; continue; }
            1 => { r9 = (crate::simd_ops::ru64(rbp.wrapping_add(0x78u64))) as u64; rax = rdx; r9 = ((r9).wrapping_shr(32u32)) as u64; rax = ((rax).wrapping_shr(32u32)) as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((r9 as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 3; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::wu8(rbp.wrapping_add(0x50u64), (0x0u8) as u8); rax = rbp.wrapping_sub(0x30); bb = 20; continue; }
            3 => { rcx = (crate::simd_ops::ru64(r8)) as u64; xmm0 = ((crate::simd_ops::ru64(r8.wrapping_add(0x10u64))) as u64) as u128; ymm0_hi = 0; r9 = ((rdx as u32)) as u32 as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x40u64), ((xmm0 as u64)) as u64); rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x20u64))) as u32 as u64; fl_l = r9; fl_r = rax; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 18; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x18u64))) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(r9.wrapping_mul(8)))) as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 18; continue; } bb = 5; continue; }
            5 => { rax = rdx; rcx = ((rcx).wrapping_shr(32u32)) as u64; rax = ((rax).wrapping_shr(32u32)) as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 18; continue; } bb = 6; continue; }
            6 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0xffffffff; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 18; continue; } bb = 7; continue; }
            7 => { xmm1 = (crate::simd_ops::ru128(r8)) as u128; ymm1_hi = 0; r10 = ((xmm1 as u64)) as u64; rax = 0xffff; crate::simd_ops::wu64(rbp.wrapping_sub(0x40u64), ((xmm0 as u64)) as u64); rbx = (crate::simd_ops::ru64(r10.wrapping_add(0x30u64))) as u64; rbx = (crate::simd_ops::ru64(rbx.wrapping_add(r9.wrapping_mul(8)))) as u64; fl_l = ((rbx as u16)) as u16 as u64; fl_r = ((rax as u16)) as u16 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { bb = 9; continue; } bb = 8; continue; }
            8 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::lift_ops::trap(0x80000003, P3, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            9 => { rax = (crate::simd_ops::ru64(rbp.wrapping_sub(0x40u64))) as u64; rcx = (((xmm1 >> 64) as u64)) as u64; r9 = ((rax.wrapping_add(rcx.wrapping_mul(2))) as u64) as u64; rdx = r9; rdx = ((rdx).wrapping_sub(rax)) as u64; rdx = ((((rdx) as i64).wrapping_shr(1u32) as u64)) as u64; fl_l = rdx; fl_r = rdx; if (((((fl_l as u64)) & ((fl_r as u64))) as i64) <= 0) { bb = 15; continue; } bb = 10; continue; }
            10 => { bb = 11; continue; }
            11 => { r8 = rdx; r8 = ((r8).wrapping_shr(1u32)) as u64; fl_l = (crate::simd_ops::ru16(rax.wrapping_add(r8.wrapping_mul(2)))) as u16 as u64; fl_r = ((rbx as u16)) as u16 as u64; rcx = ((rax.wrapping_add(r8.wrapping_mul(2))) as u64) as u64; if (((fl_l as u16)) >= ((fl_r as u16))) { bb = 13; continue; } bb = 12; continue; }
            12 => { rax = (0xffffffffffffffffu64) as u64; rax = ((rax).wrapping_sub(r8)) as u64; fl_l = ((rdx).wrapping_add(rax)) as u64; fl_r = fl_l; rdx = ((fl_l as u64)) as u64; rax = rcx.wrapping_add(0x2); bb = 14; continue; }
            13 => { rdx = r8; bb = 14; continue; }
            14 => { fl_l = rdx; fl_r = rdx; if (((((fl_l as u64)) & ((fl_r as u64))) as i64) > 0) { bb = 11; continue; } bb = 15; continue; }
            15 => { fl_l = rax; fl_r = r9; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 18; continue; } bb = 16; continue; }
            16 => { fl_l = ((rbx as u16)) as u16 as u64; fl_r = (crate::simd_ops::ru16(rax)) as u16 as u64; if (((fl_l as u16)) < ((fl_r as u16))) { bb = 18; continue; } bb = 17; continue; }
            17 => { r9 = (((rbx as u16)) as u32) as u32 as u64; r8 = r10.wrapping_add(0x40); crate::simd_ops::wu64(rsp.wrapping_add(0x170u64), rdi); rcx = rsp.wrapping_add(0x20); rdi = (crate::simd_ops::ru64(r8.wrapping_add(r9.wrapping_mul(8)).wrapping_add(0x48u64))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); let y_addr: u64 = rsp.wrapping_add(0x20u64); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rsp.wrapping_add(0x40u64); xmm1 = crate::simd_ops::ru128(y_addr); ymm1_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rbp.wrapping_sub(0x30u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); let y_addr: u64 = rsp.wrapping_add(0x60u64); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rbp.wrapping_sub(0x10u64); crate::simd_ops::wu128(y_addr, xmm1); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm1_hi); xmm1 = (crate::simd_ops::ru128(rbp.wrapping_sub(0x80u64))) as u128; ymm1_hi = 0; let y_addr: u64 = rbp.wrapping_add(0x10u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); crate::simd_ops::wu128(rbp.wrapping_add(0x30u64), (xmm1) as u128); r8 = rdi; rdx = rbp.wrapping_sub(0x30); rcx = rsp.wrapping_add(0x20); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm0 = (crate::simd_ops::ru128(rbp.wrapping_sub(0x80u64))) as u128; ymm0_hi = 0; let y_addr: u64 = rsp.wrapping_add(0x20u64); xmm1 = crate::simd_ops::ru128(y_addr); ymm1_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rsp.wrapping_add(0x40u64); xmm2 = crate::simd_ops::ru128(y_addr); ymm2_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rsp.wrapping_add(0x60u64); xmm3 = crate::simd_ops::ru128(y_addr); ymm3_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); crate::simd_ops::wu64(rbp.wrapping_add(0x40u64), rdi); rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x170u64))) as u64; fl_l = ((rbx).wrapping_shr(32u32)) as u64; rbx = ((fl_l as u64)) as u64; crate::simd_ops::wu128(rbp.wrapping_add(0x30u64), (xmm0) as u128); crate::simd_ops::wu64(rbp.wrapping_add(0x48u64), rbx); let y_addr: u64 = rbp.wrapping_add(0x30u64); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rbp.wrapping_sub(0x80u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); let y_addr: u64 = rsp.wrapping_add(0x20u64); crate::simd_ops::wu128(y_addr, xmm1); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm1_hi); let y_addr: u64 = rsp.wrapping_add(0x40u64); crate::simd_ops::wu128(y_addr, xmm2); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm2_hi); let y_addr: u64 = rsp.wrapping_add(0x60u64); crate::simd_ops::wu128(y_addr, xmm3); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm3_hi); crate::simd_ops::wu8(rbp.wrapping_sub(0x60u64), (0x1u8) as u8); bb = 19; continue; }
            18 => { crate::simd_ops::wu8(rbp.wrapping_sub(0x60u64), (0x0u8) as u8); bb = 19; continue; }
            19 => { rax = rsp.wrapping_add(0x20); bb = 20; continue; }
            20 => { let y_addr: u64 = rax; xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rax.wrapping_add(0x20u64); xmm1 = crate::simd_ops::ru128(y_addr); ymm1_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rsi; crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); let y_addr: u64 = rax.wrapping_add(0x40u64); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rsi.wrapping_add(0x20u64); crate::simd_ops::wu128(y_addr, xmm1); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm1_hi); let y_addr: u64 = rax.wrapping_add(0x60u64); xmm1 = crate::simd_ops::ru128(y_addr); ymm1_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rsi.wrapping_add(0x40u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); xmm0 = ((crate::simd_ops::ru64(rax.wrapping_add(0x80u64))) as u64) as u128; ymm0_hi = 0; let y_addr: u64 = rsi.wrapping_add(0x60u64); crate::simd_ops::wu128(y_addr, xmm1); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm1_hi); crate::simd_ops::wu64(rsi.wrapping_add(0x80u64), ((xmm0 as u64)) as u64); rax = rsi; r11 = rsp.wrapping_add(0x160); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x20u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x28u64))) as u64; rsp = r11; rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_64920<const P0: u64, const P1: u64, const P2: u64, const P3: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x58u64)) as u64; rsi = (crate::simd_ops::ru64(rcx)) as u64; r8 = rsp.wrapping_add(0x20); xmm0 = (crate::simd_ops::ru128(rcx.wrapping_add(0x8u64))) as u128; rdi = rdx; rcx = rsp.wrapping_add(0x30); rsi = ((rsi).wrapping_add(0x40u64)) as u64; rdx = rsi; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x30u64))) as u128; xmm1 = ((crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64) as u128; r11 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rdi, rsi); crate::simd_ops::wu128(rdi.wrapping_add(0x8u64), (xmm0) as u128); crate::simd_ops::wu64(rdi.wrapping_add(0x18u64), ((xmm1 as u64)) as u64); crate::simd_ops::wu64(rdi.wrapping_add(0x28u64), r11); crate::simd_ops::wu64(rdi.wrapping_add(0x30u64), r11); rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0x10u64))) as u64; fl_l = rdx; fl_r = (crate::simd_ops::ru64(rdi.wrapping_add(0x18u64))) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 14; continue; } bb = 1; continue; }
            1 => { rcx = ((crate::simd_ops::ru16(rdx)) as u32) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x78u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x80u64), r12); rax = (crate::simd_ops::ru64(rsi.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x48u64))) as u64; crate::simd_ops::wu64(rdi.wrapping_add(0x20u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x88u64), r14); crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), r15); fl_l = (crate::simd_ops::ru64(rsi.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x48u64))) as u64; fl_r = r11; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 3; continue; } bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru32(rsi.wrapping_add(rcx.wrapping_mul(4)).wrapping_add(0x10048u64))) as u32 as u64; bb = 4; continue; }
            3 => { rax = ((r11 as u32)) as u32 as u64; bb = 4; continue; }
            4 => { crate::simd_ops::wu64(rdi.wrapping_add(0x28u64), rax); rbp = ((crate::simd_ops::ru16(rdx)) as u32) as u32 as u64; r8 = ((crate::simd_ops::ru16(P1)) as u32) as u32 as u64; rax = ((rbp as u32)) as u32 as u64; rax = ((rax).wrapping_shl(5u32)) as u64; r15 = (crate::simd_ops::ru32(rax.wrapping_add(rsi).wrapping_add(0x1806cu64))) as u32 as u64; r12 = (crate::simd_ops::ru64(rax.wrapping_add(rsi).wrapping_add(0x18058u64))) as u64; rdx = ((r15 as u32)) as u32 as u64; rcx = r12; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = ((((rax as u32)) as i32 as i64) as u64) as u64; fl_l = rcx; fl_r = (0xffffffffffffffffu64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rax = rbp.wrapping_add(0xc03); fl_l = ((rax).wrapping_shl(5u32)) as u64; rax = ((fl_l as u64)) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(rsi))) as u64; rbx = (crate::simd_ops::ru32(rax.wrapping_add(rcx.wrapping_mul(4)))) as u32 as u64; bb = 7; continue; }
            6 => { rbx = r11; bb = 7; continue; }
            7 => { r8 = ((crate::simd_ops::ru16(P3)) as u32) as u32 as u64; rax = 0xc03; rdx = r15; rcx = r12; r14 = ((rax.wrapping_add(rbp)) as u64) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; rcx = ((((rax as u32)) as i32 as i64) as u64) as u64; fl_l = rcx; fl_r = (0xffffffffffffffffu64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 9; continue; } bb = 8; continue; }
            8 => { fl_l = ((r14).wrapping_shl(5u32)) as u64; r14 = ((fl_l as u64)) as u64; rax = (crate::simd_ops::ru64(r14.wrapping_add(rsi))) as u64; r11 = (crate::simd_ops::ru32(rax.wrapping_add(rcx.wrapping_mul(4)))) as u32 as u64; bb = 9; continue; }
            9 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x20u64))) as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x88u64))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 10; continue; }
            10 => { fl_l = ((rbx).wrapping_add(rax)) as u64; fl_r = fl_l; rbx = ((fl_l as u64)) as u64; bb = 11; continue; }
            11 => { fl_l = r11; fl_r = r11; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 13; continue; } bb = 12; continue; }
            12 => { fl_l = ((r11).wrapping_add(rax)) as u64; fl_r = fl_l; r11 = ((fl_l as u64)) as u64; bb = 13; continue; }
            13 => { crate::simd_ops::wu64(rdi.wrapping_add(0x38u64), rbx); crate::simd_ops::wu64(rdi.wrapping_add(0x40u64), r11); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; bb = 14; continue; }
            14 => { rax = rdi; rsp = ((rsp).wrapping_add(0x58u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_64921<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64, const P9: u64, const P10: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rdi = rcx; fl_l = rax; fl_r = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 7; continue; } bb = 1; continue; }
            1 => { r8 = ((crate::simd_ops::ru16(P2)) as u32) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rbx); rbx = (crate::simd_ops::ru64(rcx)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rbp); rbp = ((crate::simd_ops::ru16(rax)) as u32) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rsi); rsi = ((rbp as u32)) as u32 as u64; rsi = ((rsi).wrapping_shl(5u32)) as u64; rsi = ((rsi).wrapping_add(rbx)) as u64; rdx = (crate::simd_ops::ru32(rsi.wrapping_add(P3))) as u32 as u64; rcx = (crate::simd_ops::ru64(rsi.wrapping_add(P4))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = ((((rax as u32)) as i32 as i64) as u64) as u64; r11 = 0; fl_l = 0; fl_r = 0; fl_l = rcx; fl_r = (0xffffffffffffffffu64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 3; continue; } bb = 2; continue; }
            2 => { rax = rbp.wrapping_add(0xc03); fl_l = ((rax).wrapping_shl(5u32)) as u64; rax = ((fl_l as u64)) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(rbx))) as u64; rbx = (crate::simd_ops::ru32(rax.wrapping_add(rcx.wrapping_mul(4)))) as u32 as u64; bb = 4; continue; }
            3 => { rbx = r11; bb = 4; continue; }
            4 => { rdx = (crate::simd_ops::ru32(rsi.wrapping_add(P6))) as u32 as u64; r8 = ((crate::simd_ops::ru16(P7)) as u32) as u32 as u64; rcx = (crate::simd_ops::ru64(rsi.wrapping_add(P8))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0xffffffff; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rcx = (0xf1a9f1a9f1a9f1a9u64) as u64; rcx = (if (((fl_l as u32)) == ((fl_r as u32))) { r11 } else { rcx }) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { r11 = (crate::simd_ops::ru64(rdi.wrapping_add(P9))) as u64; fl_l = ((r11).wrapping_add(rbx)) as u64; fl_r = fl_l; r11 = ((fl_l as u64)) as u64; bb = 6; continue; }
            6 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r11); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rcx); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; crate::simd_ops::wu128(rdi.wrapping_add(P10), (xmm0) as u128); bb = 7; continue; }
            7 => { rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_65008<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64, const P9: u64, const P10: u64, const P11: u64, const P12: u64, const P13: u64, const P14: u64, const P15: u64, const P16: u64, const P17: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 21>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), r8); crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x70u64)) as u64; rdi = (((r9 as u16)) as u32) as u32 as u64; rbp = r8; rax = ((rdi as u32)) as u32 as u64; rax = ((rax).wrapping_shl(5u32)) as u64; rsi = (crate::simd_ops::ru32(rax.wrapping_add(r8).wrapping_add(0x1806cu64))) as u32 as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(r8).wrapping_add(0x18058u64))) as u64; rdx = ((rsi as u32)) as u32 as u64; r8 = ((crate::simd_ops::ru16(P0)) as u32) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rsi); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rdx = ((((rax as u32)) as i32 as i64) as u64) as u64; r11 = 0; fl_l = 0; fl_r = 0; fl_l = rdx; fl_r = (0xffffffffffffffffu64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = rdi.wrapping_add(0xc03); fl_l = ((rax).wrapping_shl(5u32)) as u64; rax = ((fl_l as u64)) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(rbp))) as u64; rbx = (crate::simd_ops::ru32(rax.wrapping_add(rdx.wrapping_mul(4)))) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbx); bb = 3; continue; }
            2 => { crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), r11); bb = 3; continue; }
            3 => { r8 = ((crate::simd_ops::ru16(P2)) as u32) as u32 as u64; rax = 0xc03; rdx = rsi; rbx = ((rdi.wrapping_add(rax)) as u64) as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rbx = ((rbx).wrapping_shl(5u32)) as u64; rdx = ((((rax as u32)) as i32 as i64) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbx); fl_l = rdx; fl_r = (0xffffffffffffffffu64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(rbp))) as u64; rbx = (crate::simd_ops::ru32(rax.wrapping_add(rdx.wrapping_mul(4)))) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rbx); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; bb = 6; continue; }
            5 => { crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), r11); bb = 6; continue; }
            6 => { r8 = ((crate::simd_ops::ru16(P3)) as u32) as u32 as u64; rdx = rsi; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rdx = ((((rax as u32)) as i32 as i64) as u64) as u64; rdi = ((rbx.wrapping_add(rbp)) as u64) as u64; fl_l = rdx; fl_r = (0xffffffffffffffffu64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 8; continue; } bb = 7; continue; }
            7 => { rax = (crate::simd_ops::ru64(rdi)) as u64; rbx = (crate::simd_ops::ru32(rax.wrapping_add(rdx.wrapping_mul(4)))) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rbx); bb = 9; continue; }
            8 => { crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), r11); bb = 9; continue; }
            9 => { r8 = ((crate::simd_ops::ru16(P4)) as u32) as u32 as u64; rdx = rsi; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rdx = ((((rax as u32)) as i32 as i64) as u64) as u64; fl_l = rdx; fl_r = (0xffffffffffffffffu64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 11; continue; } bb = 10; continue; }
            10 => { rax = (crate::simd_ops::ru64(rdi)) as u64; rbx = (crate::simd_ops::ru32(rax.wrapping_add(rdx.wrapping_mul(4)))) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rbx); bb = 12; continue; }
            11 => { crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), r11); bb = 12; continue; }
            12 => { r8 = ((crate::simd_ops::ru16(P5)) as u32) as u32 as u64; rdx = rsi; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rdx = ((((rax as u32)) as i32 as i64) as u64) as u64; fl_l = rdx; fl_r = (0xffffffffffffffffu64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 14; continue; } bb = 13; continue; }
            13 => { rax = (crate::simd_ops::ru64(rdi)) as u64; rbx = (crate::simd_ops::ru32(rax.wrapping_add(rdx.wrapping_mul(4)))) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rbx); bb = 15; continue; }
            14 => { crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), r11); bb = 15; continue; }
            15 => { r8 = ((crate::simd_ops::ru16(P6)) as u32) as u32 as u64; rdx = rsi; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rdx = ((((rax as u32)) as i32 as i64) as u64) as u64; fl_l = rdx; fl_r = (0xffffffffffffffffu64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 17; continue; } bb = 16; continue; }
            16 => { rax = (crate::simd_ops::ru64(rdi)) as u64; rbx = (crate::simd_ops::ru32(rax.wrapping_add(rdx.wrapping_mul(4)))) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rbx); bb = 18; continue; }
            17 => { crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), r11); bb = 18; continue; }
            18 => { r8 = ((crate::simd_ops::ru16(P7)) as u32) as u32 as u64; rdx = rsi; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rdx = ((((rax as u32)) as i32 as i64) as u64) as u64; fl_l = rdx; fl_r = (0xffffffffffffffffu64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 20; continue; } bb = 19; continue; }
            19 => { rax = (crate::simd_ops::ru64(rdi)) as u64; rbx = (crate::simd_ops::ru32(rax.wrapping_add(rdx.wrapping_mul(4)))) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), rbx); bb = 21; continue; }
            20 => { crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), r11); bb = 21; continue; }
            21 => { r8 = ((crate::simd_ops::ru16(P8)) as u32) as u32 as u64; rdx = rsi; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rdx = ((((rax as u32)) as i32 as i64) as u64) as u64; fl_l = rdx; fl_r = (0xffffffffffffffffu64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 23; continue; } bb = 22; continue; }
            22 => { rax = (crate::simd_ops::ru64(rdi)) as u64; rbx = (crate::simd_ops::ru32(rax.wrapping_add(rdx.wrapping_mul(4)))) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), rbx); bb = 24; continue; }
            23 => { crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), r11); bb = 24; continue; }
            24 => { r8 = ((crate::simd_ops::ru16(P9)) as u32) as u32 as u64; rdx = rsi; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rdx = ((((rax as u32)) as i32 as i64) as u64) as u64; fl_l = rdx; fl_r = (0xffffffffffffffffu64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 26; continue; } bb = 25; continue; }
            25 => { rax = (crate::simd_ops::ru64(rdi)) as u64; r13 = (crate::simd_ops::ru32(rax.wrapping_add(rdx.wrapping_mul(4)))) as u32 as u64; bb = 27; continue; }
            26 => { r13 = r11; bb = 27; continue; }
            27 => { r8 = ((crate::simd_ops::ru16(P10)) as u32) as u32 as u64; rdx = rsi; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rdx = ((((rax as u32)) as i32 as i64) as u64) as u64; fl_l = rdx; fl_r = (0xffffffffffffffffu64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 29; continue; } bb = 28; continue; }
            28 => { rax = (crate::simd_ops::ru64(rdi)) as u64; r12 = (crate::simd_ops::ru32(rax.wrapping_add(rdx.wrapping_mul(4)))) as u32 as u64; bb = 30; continue; }
            29 => { r12 = r11; bb = 30; continue; }
            30 => { r8 = ((crate::simd_ops::ru16(P11)) as u32) as u32 as u64; rdx = rsi; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rdx = ((((rax as u32)) as i32 as i64) as u64) as u64; fl_l = rdx; fl_r = (0xffffffffffffffffu64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 32; continue; } bb = 31; continue; }
            31 => { rax = (crate::simd_ops::ru64(rdi)) as u64; r15 = (crate::simd_ops::ru32(rax.wrapping_add(rdx.wrapping_mul(4)))) as u32 as u64; bb = 33; continue; }
            32 => { r15 = r11; bb = 33; continue; }
            33 => { r8 = ((crate::simd_ops::ru16(P12)) as u32) as u32 as u64; rdx = rsi; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rdx = ((((rax as u32)) as i32 as i64) as u64) as u64; fl_l = rdx; fl_r = (0xffffffffffffffffu64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 35; continue; } bb = 34; continue; }
            34 => { rax = (crate::simd_ops::ru64(rdi)) as u64; r14 = (crate::simd_ops::ru32(rax.wrapping_add(rdx.wrapping_mul(4)))) as u32 as u64; bb = 36; continue; }
            35 => { r14 = r11; bb = 36; continue; }
            36 => { r8 = ((crate::simd_ops::ru16(P13)) as u32) as u32 as u64; rdx = rsi; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rdx = ((((rax as u32)) as i32 as i64) as u64) as u64; fl_l = rdx; fl_r = (0xffffffffffffffffu64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 38; continue; } bb = 37; continue; }
            37 => { rax = (crate::simd_ops::ru64(rdi)) as u64; rbp = (crate::simd_ops::ru32(rax.wrapping_add(rdx.wrapping_mul(4)))) as u32 as u64; bb = 39; continue; }
            38 => { rbp = r11; bb = 39; continue; }
            39 => { r8 = ((crate::simd_ops::ru16(P14)) as u32) as u32 as u64; rdx = rsi; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rdx = ((((rax as u32)) as i32 as i64) as u64) as u64; fl_l = rdx; fl_r = (0xffffffffffffffffu64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 41; continue; } bb = 40; continue; }
            40 => { rax = (crate::simd_ops::ru64(rdi)) as u64; rsi = (crate::simd_ops::ru32(rax.wrapping_add(rdx.wrapping_mul(4)))) as u32 as u64; bb = 42; continue; }
            41 => { rsi = r11; bb = 42; continue; }
            42 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; r8 = ((crate::simd_ops::ru16(P15)) as u32) as u32 as u64; rdx = rbx; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rdx = ((((rax as u32)) as i32 as i64) as u64) as u64; fl_l = rdx; fl_r = (0xffffffffffffffffu64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 44; continue; } bb = 43; continue; }
            43 => { rax = (crate::simd_ops::ru64(rdi)) as u64; rdi = (crate::simd_ops::ru32(rax.wrapping_add(rdx.wrapping_mul(4)))) as u32 as u64; bb = 45; continue; }
            44 => { rdi = r11; bb = 45; continue; }
            45 => { r8 = ((crate::simd_ops::ru16(P16)) as u32) as u32 as u64; rdx = rbx; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); r8 = ((((rax as u32)) as i32 as i64) as u64) as u64; fl_l = r8; fl_r = (0xffffffffffffffffu64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 47; continue; } bb = 46; continue; }
            46 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0xc0u64))) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(rdx))) as u64; rbx = (crate::simd_ops::ru32(rax.wrapping_add(r8.wrapping_mul(4)))) as u32 as u64; bb = 48; continue; }
            47 => { rbx = r11; bb = 48; continue; }
            48 => { r8 = ((crate::simd_ops::ru16(P17)) as u32) as u32 as u64; rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rdx = ((((rax as u32)) as i32 as i64) as u64) as u64; fl_l = rdx; fl_r = (0xffffffffffffffffu64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 50; continue; } bb = 49; continue; }
            49 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0xc0u64))) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(rcx))) as u64; r11 = (crate::simd_ops::ru32(rax.wrapping_add(rdx.wrapping_mul(4)))) as u32 as u64; bb = 50; continue; }
            50 => { rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0xb0u64))) as u64; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu64(rcx, rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; crate::simd_ops::wu64(rcx.wrapping_add(0x8u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; crate::simd_ops::wu64(rcx.wrapping_add(0x10u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; crate::simd_ops::wu64(rcx.wrapping_add(0x18u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; crate::simd_ops::wu64(rcx.wrapping_add(0x20u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; crate::simd_ops::wu64(rcx.wrapping_add(0x28u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; crate::simd_ops::wu64(rcx.wrapping_add(0x30u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; crate::simd_ops::wu64(rcx.wrapping_add(0x38u64), rax); rax = rcx; crate::simd_ops::wu64(rcx.wrapping_add(0x78u64), rbx); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0xb8u64))) as u64; crate::simd_ops::wu64(rcx.wrapping_add(0x40u64), r13); crate::simd_ops::wu64(rcx.wrapping_add(0x48u64), r12); crate::simd_ops::wu64(rcx.wrapping_add(0x50u64), r15); crate::simd_ops::wu64(rcx.wrapping_add(0x58u64), r14); crate::simd_ops::wu64(rcx.wrapping_add(0x60u64), rbp); crate::simd_ops::wu64(rcx.wrapping_add(0x68u64), rsi); crate::simd_ops::wu64(rcx.wrapping_add(0x70u64), rdi); crate::simd_ops::wu64(rcx.wrapping_add(0x80u64), r11); rsp = ((rsp).wrapping_add(0x70u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_65139<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0; let mut ymm1_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 37>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { r11 = rsp; rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rbp = r11.wrapping_sub(0x28); rsp = ((rsp).wrapping_sub(0x118u64)) as u64; rax = (((r8 as u16)) as u32) as u32 as u64; r13 = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = (crate::simd_ops::ru64(rdx)) as u64; r13 = ((r13).wrapping_add(r13)) as u64; rdx = (crate::simd_ops::ru32(rax.wrapping_add(r13.wrapping_mul(8)).wrapping_add(0x8u64))) as u32 as u64; r13 = (crate::simd_ops::ru64(rax.wrapping_add(r13.wrapping_mul(8)))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), r13); fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 27; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(rcx)) as u64; r9 = ((rdx.wrapping_mul(2)) as u64) as u64; r9 = ((r9).wrapping_add(r13)) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x70u64), rax); crate::simd_ops::wu64(rbp.wrapping_add(0x48u64), r9); r8 = rax.wrapping_add(0x40); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), r8); fl_l = r13; fl_r = r9; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 27; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::wu64(r11.wrapping_add(0x8u64), rbx); rax = r9; crate::simd_ops::wu64(r11.wrapping_sub(0x18u64), rsi); crate::simd_ops::wu64(r11.wrapping_sub(0x20u64), rdi); crate::simd_ops::wu64(r11.wrapping_sub(0x28u64), r12); crate::simd_ops::wu64(r11.wrapping_sub(0x30u64), r14); crate::simd_ops::wu64(r11.wrapping_sub(0x38u64), r15); bb = 3; continue; }
            3 => { r9 = ((crate::simd_ops::ru16(r13)) as u32) as u32 as u64; rcx = (crate::simd_ops::ru64(r8.wrapping_add(r9.wrapping_mul(8)).wrapping_add(0x48u64))) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x38u64), rcx); fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rcx = (crate::simd_ops::ru32(r8.wrapping_add(r9.wrapping_mul(4)).wrapping_add(0x10048u64))) as u32 as u64; bb = 5; continue; }
            5 => { crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rcx); fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 25; continue; } bb = 6; continue; }
            6 => { rcx = rsp.wrapping_add(0x48); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x38u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 7; continue; }
            7 => { fl_l = ((rax).wrapping_add(rcx)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; bb = 8; continue; }
            8 => { r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); fl_l = r12; fl_r = r12; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 9; continue; }
            9 => { fl_l = ((r12).wrapping_add(rcx)) as u64; fl_r = fl_l; r12 = ((fl_l as u64)) as u64; bb = 10; continue; }
            10 => { r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; fl_l = r15; fl_r = r15; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 11; continue; }
            11 => { fl_l = ((r15).wrapping_add(rcx)) as u64; fl_r = fl_l; r15 = ((fl_l as u64)) as u64; bb = 12; continue; }
            12 => { r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; fl_l = r14; fl_r = r14; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 14; continue; } bb = 13; continue; }
            13 => { fl_l = ((r14).wrapping_add(rcx)) as u64; fl_r = fl_l; r14 = ((fl_l as u64)) as u64; bb = 14; continue; }
            14 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 16; continue; } bb = 15; continue; }
            15 => { fl_l = ((rsi).wrapping_add(rcx)) as u64; fl_r = fl_l; rsi = ((fl_l as u64)) as u64; bb = 16; continue; }
            16 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 18; continue; } bb = 17; continue; }
            17 => { fl_l = ((rdi).wrapping_add(rcx)) as u64; fl_r = fl_l; rdi = ((fl_l as u64)) as u64; bb = 18; continue; }
            18 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 20; continue; } bb = 19; continue; }
            19 => { fl_l = ((rbx).wrapping_add(rcx)) as u64; fl_r = fl_l; rbx = ((fl_l as u64)) as u64; bb = 20; continue; }
            20 => { rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax); fl_l = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; fl_r = rax; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 24; continue; } bb = 21; continue; }
            21 => { r13 = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; bb = 22; continue; }
            22 => { rdx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x70u64))) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x80u64), rcx); rcx = rbp.wrapping_sub(0x60); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), r13); crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), r12); crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), r15); crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), r14); let y_addr: u64 = rsp.wrapping_add(0x48u64); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rbp.wrapping_sub(0x60u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), rdi); crate::simd_ops::wu64(rsp.wrapping_add(0x78u64), rbx); let y_addr: u64 = rsp.wrapping_add(0x68u64); xmm1 = crate::simd_ops::ru128(y_addr); ymm1_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); crate::simd_ops::wu64(rbp.wrapping_sub(0x78u64), rax); xmm0 = ((crate::simd_ops::ru64(rbp.wrapping_sub(0x78u64))) as u64) as u128; ymm0_hi = 0; crate::simd_ops::wu64(rbp.wrapping_sub(0x20u64), ((xmm0 as u64)) as u64); let y_addr: u64 = rbp.wrapping_sub(0x40u64); crate::simd_ops::wu128(y_addr, xmm1); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm1_hi); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x38u64))) as u64; rax = ((rax).wrapping_add(1)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax); fl_l = rax; fl_r = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 22; continue; } bb = 23; continue; }
            23 => { r13 = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; bb = 24; continue; }
            24 => { r8 = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x48u64))) as u64; bb = 25; continue; }
            25 => { r13 = ((r13).wrapping_add(0x2u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), r13); fl_l = r13; fl_r = rax; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } bb = 26; continue; }
            26 => { r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0xf0u64))) as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0xf8u64))) as u64; r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x100u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x108u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x110u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x130u64))) as u64; bb = 27; continue; }
            27 => { rsp = ((rsp).wrapping_add(0x118u64)) as u64; r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_65181<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0; let mut ymm1_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 45>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { r11 = rsp; rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rbp = r11.wrapping_sub(0x68); rsp = ((rsp).wrapping_sub(0x160u64)) as u64; rax = (((r8 as u16)) as u32) as u32 as u64; r9 = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = (crate::simd_ops::ru64(rdx)) as u64; r9 = ((r9).wrapping_add(r9)) as u64; rdx = (crate::simd_ops::ru32(rax.wrapping_add(r9.wrapping_mul(8)).wrapping_add(0x8u64))) as u32 as u64; r9 = (crate::simd_ops::ru64(rax.wrapping_add(r9.wrapping_mul(8)))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), r9); fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 33; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(rcx)) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x40u64), rax); r8 = rax.wrapping_add(0x40); rax = ((r9.wrapping_add(rdx.wrapping_mul(2))) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), r8); crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rax); fl_l = r9; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 33; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::wu64(r11.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(r11.wrapping_sub(0x10u64), rsi); crate::simd_ops::wu64(r11.wrapping_sub(0x18u64), rdi); crate::simd_ops::wu64(r11.wrapping_sub(0x20u64), r12); crate::simd_ops::wu64(r11.wrapping_sub(0x28u64), r13); crate::simd_ops::wu64(r11.wrapping_sub(0x30u64), r14); crate::simd_ops::wu64(r11.wrapping_sub(0x38u64), r15); bb = 3; continue; }
            3 => { r9 = ((crate::simd_ops::ru16(r9)) as u32) as u32 as u64; rcx = (crate::simd_ops::ru64(r8.wrapping_add(r9.wrapping_mul(8)).wrapping_add(0x48u64))) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x78u64), rcx); fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rcx = (crate::simd_ops::ru32(r8.wrapping_add(r9.wrapping_mul(4)).wrapping_add(0x10048u64))) as u32 as u64; bb = 5; continue; }
            5 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rcx); fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 31; continue; } bb = 6; continue; }
            6 => { rcx = rsp.wrapping_add(0x60); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; r8 = (crate::simd_ops::ru64(rbp.wrapping_add(0x78u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 8; continue; } bb = 7; continue; }
            7 => { rdx = 0; fl_l = 0; fl_r = 0; bb = 9; continue; }
            8 => { rdx = ((rax.wrapping_add(r8)) as u64) as u64; bb = 9; continue; }
            9 => { rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rdx); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rcx); fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 10; continue; }
            10 => { fl_l = ((rcx).wrapping_add(r8)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rcx); bb = 11; continue; }
            11 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 13; continue; } bb = 12; continue; }
            12 => { fl_l = ((rax).wrapping_add(r8)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax); bb = 13; continue; }
            13 => { r13 = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; fl_l = r13; fl_r = r13; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 15; continue; } bb = 14; continue; }
            14 => { fl_l = ((r13).wrapping_add(r8)) as u64; fl_r = fl_l; r13 = ((fl_l as u64)) as u64; bb = 15; continue; }
            15 => { r12 = (crate::simd_ops::ru64(rbp.wrapping_sub(0x80u64))) as u64; fl_l = r12; fl_r = r12; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 17; continue; } bb = 16; continue; }
            16 => { fl_l = ((r12).wrapping_add(r8)) as u64; fl_r = fl_l; r12 = ((fl_l as u64)) as u64; bb = 17; continue; }
            17 => { r15 = (crate::simd_ops::ru64(rbp.wrapping_sub(0x78u64))) as u64; fl_l = r15; fl_r = r15; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 19; continue; } bb = 18; continue; }
            18 => { fl_l = ((r15).wrapping_add(r8)) as u64; fl_r = fl_l; r15 = ((fl_l as u64)) as u64; bb = 19; continue; }
            19 => { r14 = (crate::simd_ops::ru64(rbp.wrapping_sub(0x70u64))) as u64; fl_l = r14; fl_r = r14; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 21; continue; } bb = 20; continue; }
            20 => { fl_l = ((r14).wrapping_add(r8)) as u64; fl_r = fl_l; r14 = ((fl_l as u64)) as u64; bb = 21; continue; }
            21 => { rsi = (crate::simd_ops::ru64(rbp.wrapping_sub(0x68u64))) as u64; fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 23; continue; } bb = 22; continue; }
            22 => { fl_l = ((rsi).wrapping_add(r8)) as u64; fl_r = fl_l; rsi = ((fl_l as u64)) as u64; bb = 23; continue; }
            23 => { rdi = (crate::simd_ops::ru64(rbp.wrapping_sub(0x60u64))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 25; continue; } bb = 24; continue; }
            24 => { fl_l = ((rdi).wrapping_add(r8)) as u64; fl_r = fl_l; rdi = ((fl_l as u64)) as u64; bb = 25; continue; }
            25 => { rbx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x58u64))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 27; continue; } bb = 26; continue; }
            26 => { fl_l = ((rbx).wrapping_add(r8)) as u64; fl_r = fl_l; rbx = ((fl_l as u64)) as u64; bb = 27; continue; }
            27 => { r9 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rbp.wrapping_add(0x88u64), r9); fl_l = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; fl_r = r9; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 30; continue; } bb = 28; continue; }
            28 => { bb = 29; continue; }
            29 => { crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), rdx); rdx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x40u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), rcx); rcx = rbp.wrapping_sub(0x30); crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x78u64), r13); let y_addr: u64 = rsp.wrapping_add(0x60u64); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rbp.wrapping_sub(0x30u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); crate::simd_ops::wu64(rbp.wrapping_sub(0x80u64), r12); crate::simd_ops::wu64(rbp.wrapping_sub(0x78u64), r15); crate::simd_ops::wu64(rbp.wrapping_sub(0x70u64), r14); crate::simd_ops::wu64(rbp.wrapping_sub(0x68u64), rsi); let y_addr: u64 = rbp.wrapping_sub(0x80u64); xmm1 = crate::simd_ops::ru128(y_addr); ymm1_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); crate::simd_ops::wu64(rbp.wrapping_sub(0x60u64), rdi); crate::simd_ops::wu64(rbp.wrapping_sub(0x58u64), rbx); crate::simd_ops::wu64(rbp.wrapping_sub(0x50u64), r8); crate::simd_ops::wu64(rbp.wrapping_sub(0x48u64), r9); let y_addr: u64 = rbp.wrapping_sub(0x60u64); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rbp.wrapping_add(0x10u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); let y_addr: u64 = rbp.wrapping_sub(0x10u64); crate::simd_ops::wu128(y_addr, xmm1); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm1_hi); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r9 = (crate::simd_ops::ru64(rbp.wrapping_add(0x88u64))) as u64; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; r9 = ((r9).wrapping_add(1)) as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; r8 = (crate::simd_ops::ru64(rbp.wrapping_add(0x78u64))) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x88u64), r9); fl_l = r9; fl_r = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 29; continue; } bb = 30; continue; }
            30 => { r8 = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; bb = 31; continue; }
            31 => { r9 = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; r9 = ((r9).wrapping_add(0x2u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), r9); fl_l = r9; fl_r = rax; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } bb = 32; continue; }
            32 => { r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x130u64))) as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x138u64))) as u64; r13 = (crate::simd_ops::ru64(rsp.wrapping_add(0x140u64))) as u64; r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x148u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x150u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x158u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x170u64))) as u64; bb = 33; continue; }
            33 => { rsp = ((rsp).wrapping_add(0x160u64)) as u64; rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_65202<const P0: u64, const P1: u64, const P2: u64, const P3: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 41>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { r11 = rsp; rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rbp = r11.wrapping_sub(0x48); rsp = ((rsp).wrapping_sub(0x138u64)) as u64; rax = (((r8 as u16)) as u32) as u32 as u64; r15 = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = (crate::simd_ops::ru64(rdx)) as u64; r15 = ((r15).wrapping_add(r15)) as u64; r10 = (crate::simd_ops::ru32(rax.wrapping_add(r15.wrapping_mul(8)).wrapping_add(0x8u64))) as u32 as u64; r15 = (crate::simd_ops::ru64(rax.wrapping_add(r15.wrapping_mul(8)))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), r15); fl_l = r10; fl_r = r10; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 26; continue; } bb = 1; continue; }
            1 => { r9 = (crate::simd_ops::ru64(rcx)) as u64; rdx = ((crate::simd_ops::ru16(P0)) as u32) as u32 as u64; crate::simd_ops::wu64(r11.wrapping_sub(0x30u64), r13); r13 = ((r15.wrapping_add(r10.wrapping_mul(2))) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x80u64), r9); rax = (crate::simd_ops::ru64(r9)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), r13); rcx = (crate::simd_ops::ru64(rax.wrapping_add(0x20u64))) as u64; rcx = ((crate::simd_ops::ru16(rcx.wrapping_add(rdx.wrapping_mul(2)))) as u32) as u32 as u64; rdx = r9.wrapping_add(0x40); rcx = ((rcx).wrapping_shl(4u32)) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(r9.wrapping_add(0x8u64)))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdx); rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x8u64))) as u32 as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x78u64), rax); rax = (crate::simd_ops::ru64(rcx)) as u64; rcx = ((crate::simd_ops::ru16(P1)) as u32) as u32 as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x70u64), rax); rax = (crate::simd_ops::ru64(r9.wrapping_add(0x580e0u64))) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(rcx.wrapping_mul(8)))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rax); fl_l = r15; fl_r = r13; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 25; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::wu64(r11.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(r11.wrapping_sub(0x18u64), rsi); crate::simd_ops::wu64(r11.wrapping_sub(0x20u64), rdi); crate::simd_ops::wu64(r11.wrapping_sub(0x28u64), r12); crate::simd_ops::wu64(r11.wrapping_sub(0x38u64), r14); bb = 3; continue; }
            3 => { r9 = ((crate::simd_ops::ru16(r15)) as u32) as u32 as u64; rbx = (crate::simd_ops::ru64(rdx.wrapping_add(r9.wrapping_mul(8)).wrapping_add(0x48u64))) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x58u64), rbx); fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rcx = (crate::simd_ops::ru32(rdx.wrapping_add(r9.wrapping_mul(4)).wrapping_add(0x10048u64))) as u32 as u64; bb = 6; continue; }
            5 => { rcx = 0; fl_l = 0; fl_r = 0; bb = 6; continue; }
            6 => { rax = ((rcx as u32)) as u32 as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x68u64), rax); fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 23; continue; } bb = 7; continue; }
            7 => { r8 = rdx; rcx = rsp.wrapping_add(0x50); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 9; continue; } bb = 8; continue; }
            8 => { r12 = 0; fl_l = 0; fl_r = 0; bb = 10; continue; }
            9 => { r12 = ((rax.wrapping_add(rbx)) as u64) as u64; bb = 10; continue; }
            10 => { r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; fl_l = r14; fl_r = r14; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 11; continue; }
            11 => { fl_l = ((r14).wrapping_add(rbx)) as u64; fl_r = fl_l; r14 = ((fl_l as u64)) as u64; bb = 12; continue; }
            12 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 14; continue; } bb = 13; continue; }
            13 => { fl_l = ((rsi).wrapping_add(rbx)) as u64; fl_r = fl_l; rsi = ((fl_l as u64)) as u64; bb = 14; continue; }
            14 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 16; continue; } bb = 15; continue; }
            15 => { fl_l = ((rdi).wrapping_add(rbx)) as u64; fl_r = fl_l; rdi = ((fl_l as u64)) as u64; bb = 16; continue; }
            16 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 18; continue; } bb = 17; continue; }
            17 => { fl_l = ((rbx).wrapping_add(crate::simd_ops::ru64(rbp.wrapping_add(0x58u64)))) as u64; fl_r = fl_l; rbx = ((fl_l as u64)) as u64; bb = 18; continue; }
            18 => { fl_l = (crate::simd_ops::ru64(rbp.wrapping_add(0x68u64))) as u64; fl_r = (0x0u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (0x0u64) as u64); if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 22; continue; } bb = 19; continue; }
            19 => { r13 = (crate::simd_ops::ru64(rbp.wrapping_add(0x58u64))) as u64; r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; bb = 20; continue; }
            20 => { xmm0 = (crate::simd_ops::ru128(rbp.wrapping_sub(0x80u64))) as u128; ymm0_hi = 0; r8 = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rdx = rsp.wrapping_add(0x50); crate::simd_ops::wu128(rsp.wrapping_add(0x50u64), (xmm0) as u128); xmm0 = ((crate::simd_ops::ru64(rbp.wrapping_sub(0x70u64))) as u64) as u128; ymm0_hi = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), ((xmm0 as u64)) as u64); crate::simd_ops::wu64(rbp.wrapping_sub(0x68u64), r12); rcx = rbp.wrapping_sub(0x30); crate::simd_ops::wu64(rbp.wrapping_sub(0x60u64), r14); crate::simd_ops::wu64(rbp.wrapping_sub(0x58u64), rsi); crate::simd_ops::wu64(rbp.wrapping_sub(0x50u64), rdi); let y_addr: u64 = rbp.wrapping_sub(0x68u64); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rbp.wrapping_sub(0x30u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); crate::simd_ops::wu64(rbp.wrapping_sub(0x48u64), rbx); crate::simd_ops::wu64(rbp.wrapping_sub(0x40u64), r13); xmm1 = (crate::simd_ops::ru128(rbp.wrapping_sub(0x48u64))) as u128; crate::simd_ops::wu64(rbp.wrapping_sub(0x38u64), r15); xmm0 = ((crate::simd_ops::ru64(rbp.wrapping_sub(0x38u64))) as u64) as u128; ymm0_hi = 0; crate::simd_ops::wu64(rbp, ((xmm0 as u64)) as u64); crate::simd_ops::wu128(rbp.wrapping_sub(0x10u64), (xmm1) as u128); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r15 = ((r15).wrapping_add(1)) as u64; fl_l = r15; fl_r = (crate::simd_ops::ru64(rbp.wrapping_add(0x68u64))) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 20; continue; } bb = 21; continue; }
            21 => { r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; r13 = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; bb = 22; continue; }
            22 => { rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; bb = 23; continue; }
            23 => { r15 = ((r15).wrapping_add(0x2u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), r15); fl_l = r15; fl_r = r13; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } bb = 24; continue; }
            24 => { r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x110u64))) as u64; r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x120u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x128u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x130u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x150u64))) as u64; bb = 25; continue; }
            25 => { r13 = (crate::simd_ops::ru64(rsp.wrapping_add(0x118u64))) as u64; bb = 26; continue; }
            26 => { rsp = ((rsp).wrapping_add(0x138u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_65251<const P0: u64, const P1: u64, const P2: u64, const P3: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rdx; rdi = rcx; rsi = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; rcx = ((rsi as u32)) as u32 as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rax = (crate::simd_ops::ru32(rdi.wrapping_add(P1))) as u32 as u64; fl_l = rdx; fl_r = rax; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 3; continue; } bb = 1; continue; }
            1 => { rdx = ((rax as u32)) as u32 as u64; rdx = ((rdx).wrapping_shr(1u32)) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; fl_l = rdx; fl_r = rbx; rdx = (if (((fl_l as u64)) < ((fl_r as u64))) { rbx } else { rdx }) as u64; rcx = rdi; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); r8 = rbx; fl_l = r8; fl_r = rsi; r8 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 7; continue; } bb = 2; continue; }
            2 => { rcx = (crate::simd_ops::ru32(rdi.wrapping_add(P0))) as u32 as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rdi))) as u64; r8 = ((r8).wrapping_shl(4u32)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu32(rdi.wrapping_add(P0), ((rbx as u32)) as u32); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            3 => { fl_l = rbx; fl_r = rsi; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 6; continue; } bb = 4; continue; }
            4 => { r8 = rbx; fl_l = r8; fl_r = rsi; r8 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 7; continue; } bb = 5; continue; }
            5 => { rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rdi))) as u64; r8 = ((r8).wrapping_shl(4u32)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu32(rdi.wrapping_add(P0), ((rbx as u32)) as u32); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            6 => { if (((fl_l as u64)) == ((fl_r as u64))) { bb = 8; continue; } bb = 7; continue; }
            7 => { crate::simd_ops::wu32(rdi.wrapping_add(P0), ((rbx as u32)) as u32); bb = 8; continue; }
            8 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_65265<const P0: u64, const P1: u64, const P2: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 23>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x98u64)) as u64; rbx = (crate::simd_ops::ru64(rcx)) as u64; rsi = 0; fl_l = 0; fl_r = 0; rax = (((rdx as u16)) as u32) as u32 as u64; r14 = (crate::simd_ops::ru64(rbx.wrapping_add(rax.wrapping_mul(8)).wrapping_add(0x88u64))) as u64; fl_l = r14; fl_r = r14; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(rax.wrapping_mul(4)).wrapping_add(0x10088u64))) as u32 as u64; bb = 3; continue; }
            2 => { rax = ((rsi as u32)) as u32 as u64; bb = 3; continue; }
            3 => { rbp = ((rax as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 8; continue; } bb = 4; continue; }
            4 => { rcx = ((crate::simd_ops::ru16(P0)) as u32) as u32 as u64; rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x580e0u64))) as u64; crate::simd_ops::wu16(rsp.wrapping_add(0x50u64), ((rdx as u16)) as u16); rdx = rsp.wrapping_add(0x20); crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), r14); crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), rbx); rax = (crate::simd_ops::ru64(rax.wrapping_add(rcx.wrapping_mul(8)))) as u64; rcx = rsp.wrapping_add(0x50); crate::simd_ops::wu64(rsp.wrapping_add(0xc0u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), rbp); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); let y_addr: u64 = rax; xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rsp.wrapping_add(0x70u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); fl_l = rbp; fl_r = rbp; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 5; continue; }
            5 => { crate::simd_ops::wu64(rsp.wrapping_add(0xc8u64), rdi); rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xd0u64), r12); r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xd8u64), r13); r13 = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x90u64), r15); r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x88u64))) as u64; bb = 6; continue; }
            6 => { r8 = (crate::simd_ops::ru64(rsp.wrapping_add(0xc0u64))) as u64; rcx = rsp.wrapping_add(0x50); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rdx = rbx; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), r13); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), r12); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), r15); let y_addr: u64 = rsp.wrapping_add(0x20u64); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), r14); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rsi); xmm1 = (crate::simd_ops::ru128(rsp.wrapping_add(0x40u64))) as u128; let y_addr: u64 = rsp.wrapping_add(0x50u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); crate::simd_ops::wu128(rsp.wrapping_add(0x70u64), (xmm1) as u128); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rsi = ((rsi).wrapping_add(1)) as u64; fl_l = rsi; fl_r = rbp; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 6; continue; } bb = 7; continue; }
            7 => { r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x90u64))) as u64; r13 = (crate::simd_ops::ru64(rsp.wrapping_add(0xd8u64))) as u64; r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0xd0u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0xc8u64))) as u64; bb = 8; continue; }
            8 => { rsp = ((rsp).wrapping_add(0x98u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_65350<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x24u64))) as u32 as u64; rdi = rcx; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 17; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rsi); rsi = ((rax.wrapping_add(rax)) as u32) as u32 as u64; rax = 0x10; r10 = 0x6348; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; rax = (if (((fl_l as u32)) > ((fl_r as u32))) { (rdx as u32) } else { (rax as u32) }) as u32 as u64; fl_l = ((rsi as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; rsi = (if (((fl_l as u32)) < ((fl_r as u32))) { (rax as u32) } else { (rsi as u32) }) as u32 as u64; rax = ((crate::exe::tls_pointer() as u64)) as u64; rcx = (crate::simd_ops::ru64(rax)) as u64; rdx = ((rsi.wrapping_add(rsi.wrapping_mul(4))) as u64) as u64; rdx = ((rdx).wrapping_shl(4u32)) as u64; fl_l = ((rdx) & ((rdx).wrapping_sub(1))) as u64; rax = ((fl_l as u64)) as u64; r10 = (crate::simd_ops::ru64(r10.wrapping_add(rcx))) as u64; if (((fl_l as u64)) != 0) { bb = 6; continue; } bb = 2; continue; }
            2 => { rax = rdx.wrapping_sub(0x10); fl_l = rax; fl_r = (0x3f0u64) as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 6; continue; } bb = 3; continue; }
            3 => { rax = rdx; rax = ((rax).wrapping_shr(3u32)) as u64; r8 = (crate::simd_ops::ru64(r10.wrapping_add(rax.wrapping_mul(8)).wrapping_add(0x8u64))) as u64; rbx = (crate::simd_ops::ru64(r8.wrapping_add(0x10u64))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { r9 = 0; fl_l = 0; fl_r = 0; r8 = 0; fl_l = 0; fl_r = 0; rcx = r10; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); bb = 11; continue; }
            5 => { fl_l = (crate::simd_ops::lk_add_u32(r8.wrapping_add(0x18u64), (1) as u32)) as u32 as u64; rax = (crate::simd_ops::ru64(rbx)) as u64; crate::simd_ops::wu64(r8.wrapping_add(0x10u64), rax); bb = 12; continue; }
            6 => { rax = rdx.wrapping_sub(0x10); fl_l = rax; fl_r = (0x3f0u64) as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 10; continue; } bb = 7; continue; }
            7 => { rax = rdx; rax = ((rax).wrapping_shr(3u32)) as u64; rcx = (crate::simd_ops::ru64(r10.wrapping_add(rax.wrapping_mul(8)).wrapping_add(0x8u64))) as u64; rbx = (crate::simd_ops::ru64(rcx.wrapping_add(0x10u64))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 8; continue; }
            8 => { fl_l = ((rbx as u8)) as u8 as u64; fl_r = (0xfu8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 10; continue; } bb = 9; continue; }
            9 => { fl_l = (crate::simd_ops::lk_add_u32(rcx.wrapping_add(0x18u64), (1) as u32)) as u32 as u64; rax = (crate::simd_ops::ru64(rbx)) as u64; crate::simd_ops::wu64(rcx.wrapping_add(0x10u64), rax); bb = 12; continue; }
            10 => { r8 = 0x10; rcx = r10; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 11; continue; }
            11 => { rbx = rax; bb = 12; continue; }
            12 => { fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 14; continue; } bb = 13; continue; }
            13 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); rcx = rax; rdx = 0xdeadd00d; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); bb = 14; continue; }
            14 => { rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0x18u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 16; continue; } bb = 15; continue; }
            15 => { rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x20u64))) as u32 as u64; rcx = rbx; r8 = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; r8 = ((r8).wrapping_shl(4u32)) as u64; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); bb = 16; continue; }
            16 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x18u64))) as u64; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rdi.wrapping_add(0x18u64), rbx); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x24u64), ((rsi as u32)) as u32); rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; bb = 17; continue; }
            17 => { rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_65423<const P0: u64, const P1: u64, const P2: u64, const P3: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (0xfffffffffffffffu64) as u64; r9 = 0x1; let bit_in: u64 = rax; if bit_in != 0 { r8 = (((63u32.wrapping_sub(bit_in.leading_zeros())) as u64)) as u64; } fl_l = bit_in as u64; rsi = rcx; rax = ((r9).wrapping_shl((((r8) as u32) & 63))) as u64; fl_l = rdx; fl_r = rax; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rcx = P0; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, P3, rcx, rsp); return rax; }
            2 => { rax = rdx.wrapping_sub(0x1); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbx); rbx = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; rax = (((rax) | (r9))) as u64; let bit_in: u64 = rax; if bit_in != 0 { rcx = (((63u32.wrapping_sub(bit_in.leading_zeros())) as u64)) as u64; } fl_l = bit_in as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rdi); r8 = rbx; rcx = (((rcx as u32)).wrapping_add(1)) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), r14); rdi = ((r9).wrapping_shl((((rcx) as u32) & 63))) as u64; rcx = rsi.wrapping_add(0x18); rdx = ((rdi.wrapping_add(rdi)) as u64) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rax = rdi.wrapping_sub(0x1); crate::simd_ops::wu64(rsi.wrapping_add(0x38u64), rdi); crate::simd_ops::wu64(rsi.wrapping_add(0x30u64), rax); rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; rax = (crate::simd_ops::ru64(rax)) as u64; rcx = rax; fl_l = rax; fl_r = rbx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 17; continue; } bb = 3; continue; }
            3 => { crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rbp); rdi = (0x100000001b3u64) as u64; rbp = (0xcbf29ce484222325u64) as u64; bb = 4; continue; }
            4 => { rdx = ((crate::simd_ops::ru8(rax.wrapping_add(0x11u64))) as u32) as u32 as u64; r9 = ((crate::simd_ops::ru8(rax.wrapping_add(0x10u64))) as u32) as u32 as u64; r11 = (crate::simd_ops::ru64(rsi.wrapping_add(0x30u64))) as u64; r9 = (((r9) ^ (rbp))) as u64; rcx = (crate::simd_ops::ru64(rcx)) as u64; r9 = (((r9) as i64).wrapping_mul((rdi) as i64) as u64) as u64; r9 = (((r9) ^ (rdx))) as u64; rdx = ((crate::simd_ops::ru8(rax.wrapping_add(0x12u64))) as u32) as u32 as u64; r9 = (((r9) as i64).wrapping_mul((rdi) as i64) as u64) as u64; r9 = (((r9) ^ (rdx))) as u64; rdx = ((crate::simd_ops::ru8(rax.wrapping_add(0x13u64))) as u32) as u32 as u64; r9 = (((r9) as i64).wrapping_mul((rdi) as i64) as u64) as u64; r9 = (((r9) ^ (rdx))) as u64; rdx = ((crate::simd_ops::ru8(rax.wrapping_add(0x14u64))) as u32) as u32 as u64; r9 = (((r9) as i64).wrapping_mul((rdi) as i64) as u64) as u64; r9 = (((r9) ^ (rdx))) as u64; rdx = ((crate::simd_ops::ru8(rax.wrapping_add(0x15u64))) as u32) as u32 as u64; r9 = (((r9) as i64).wrapping_mul((rdi) as i64) as u64) as u64; r9 = (((r9) ^ (rdx))) as u64; rdx = ((crate::simd_ops::ru8(rax.wrapping_add(0x16u64))) as u32) as u32 as u64; r9 = (((r9) as i64).wrapping_mul((rdi) as i64) as u64) as u64; r9 = (((r9) ^ (rdx))) as u64; rdx = ((crate::simd_ops::ru8(rax.wrapping_add(0x17u64))) as u32) as u32 as u64; r9 = (((r9) as i64).wrapping_mul((rdi) as i64) as u64) as u64; r9 = (((r9) ^ (rdx))) as u64; r9 = (((r9) as i64).wrapping_mul((rdi) as i64) as u64) as u64; r11 = (((r11) & (r9))) as u64; r11 = ((r11).wrapping_shl(4u32)) as u64; r11 = ((r11).wrapping_add(crate::simd_ops::ru64(rsi.wrapping_add(0x18u64)))) as u64; r9 = (crate::simd_ops::ru64(r11)) as u64; fl_l = r9; fl_r = rbx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 6; continue; } bb = 5; continue; }
            5 => { crate::simd_ops::wu64(r11, rax); crate::simd_ops::wu64(r11.wrapping_add(0x8u64), rax); bb = 15; continue; }
            6 => { rdx = (crate::simd_ops::ru64(r11.wrapping_add(0x8u64))) as u64; r8 = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; fl_l = r8; fl_r = (crate::simd_ops::ru64(rdx.wrapping_add(0x10u64))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 10; continue; } bb = 7; continue; }
            7 => { r10 = (crate::simd_ops::ru64(rdx)) as u64; fl_l = r10; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 9; continue; } bb = 8; continue; }
            8 => { r9 = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(r9, rcx); r8 = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(r8, r10); rdx = (crate::simd_ops::ru64(r10.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(rdx, rax); crate::simd_ops::wu64(r10.wrapping_add(0x8u64), r8); crate::simd_ops::wu64(rcx.wrapping_add(0x8u64), r9); crate::simd_ops::wu64(rax.wrapping_add(0x8u64), rdx); bb = 9; continue; }
            9 => { crate::simd_ops::wu64(r11.wrapping_add(0x8u64), rax); bb = 15; continue; }
            10 => { fl_l = r9; fl_r = rdx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 14; continue; } bb = 11; continue; }
            11 => { bb = 12; continue; }
            12 => { rdx = (crate::simd_ops::ru64(rdx.wrapping_add(0x8u64))) as u64; fl_l = r8; fl_r = (crate::simd_ops::ru64(rdx.wrapping_add(0x10u64))) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 18; continue; } bb = 13; continue; }
            13 => { fl_l = r9; fl_r = rdx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 12; continue; } bb = 14; continue; }
            14 => { r10 = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(r10, rcx); r9 = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(r9, rdx); r8 = (crate::simd_ops::ru64(rdx.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(r8, rax); crate::simd_ops::wu64(rdx.wrapping_add(0x8u64), r9); crate::simd_ops::wu64(rcx.wrapping_add(0x8u64), r10); crate::simd_ops::wu64(rax.wrapping_add(0x8u64), r8); crate::simd_ops::wu64(r11, rax); bb = 15; continue; }
            15 => { rax = rcx; fl_l = rcx; fl_r = rbx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 4; continue; } bb = 16; continue; }
            16 => { rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; bb = 17; continue; }
            17 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            18 => { r10 = (crate::simd_ops::ru64(rdx)) as u64; r9 = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(r9, rcx); r8 = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(r8, r10); rdx = (crate::simd_ops::ru64(r10.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(rdx, rax); crate::simd_ops::wu64(r10.wrapping_add(0x8u64), r8); crate::simd_ops::wu64(rcx.wrapping_add(0x8u64), r9); crate::simd_ops::wu64(rax.wrapping_add(0x8u64), rdx); bb = 15; continue; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_65472<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdx = (((r9) as i64).wrapping_mul((0x1b0u64) as i64) as u64) as u64; rcx = 0x11; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rdi); fl_l = rdx; fl_r = rdx; rdi = r9; rax = ((rdx.wrapping_add(P0)) as u64) as u64; rdx = 0x10; rcx = (if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { rax } else { rcx }) as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); rcx = rax; rdx = 0xdeadd00d; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); rax = 0; fl_l = 0; fl_r = 0; bb = 2; continue; }
            2 => { crate::simd_ops::wu64(rax, (0x1b0u64) as u64); rsi = ((rax.wrapping_add(P0)) as u64) as u64; crate::simd_ops::wu32(rax.wrapping_add(P4), ((rdi as u32)) as u32); crate::simd_ops::wu32(rax.wrapping_add(P5), (0x8u32) as u32); fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 3; continue; }
            3 => { crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbx); rbx = rsi; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rbp); rbp = P6; bb = 4; continue; }
            4 => { rdx = 0; fl_l = 0; fl_r = 0; r8 = 0x1b0; rcx = rbx; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); rcx = rbx; rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx, rbp); rbx = ((rbx).wrapping_add(0x1b0u64)) as u64; fl_l = rdi; fl_r = (0x1u64) as u64; rdi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 4; continue; } bb = 5; continue; }
            5 => { rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; bb = 6; continue; }
            6 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rax = rsi; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_65568<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 25>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = ((rsp).wrapping_sub(0xb8u64)) as u64; rbx = (crate::simd_ops::ru64(rcx)) as u64; rsi = 0; fl_l = 0; fl_r = 0; rax = (((rdx as u16)) as u32) as u32 as u64; r8 = (crate::simd_ops::ru64(rbx.wrapping_add(rax.wrapping_mul(8)).wrapping_add(P0))) as u64; fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(rax.wrapping_mul(4)).wrapping_add(P1))) as u32 as u64; bb = 3; continue; }
            2 => { rax = ((rsi as u32)) as u32 as u64; bb = 3; continue; }
            3 => { r9 = ((rax as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 9; continue; } bb = 4; continue; }
            4 => { rcx = ((crate::simd_ops::ru16(P2)) as u32) as u32 as u64; rax = (crate::simd_ops::ru64(rbx.wrapping_add(P3))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xe0u64), r12); crate::simd_ops::wu64(rsp.wrapping_add(0xb0u64), r13); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbx); r13 = (crate::simd_ops::ru64(rax.wrapping_add(rcx.wrapping_mul(8)))) as u64; rcx = rsp.wrapping_add(0x60); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; fl_l = r12; fl_r = r12; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 5; continue; }
            5 => { crate::simd_ops::wu64(rsp.wrapping_add(0xd0u64), rbp); rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x90u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xd8u64), rdi); rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xa8u64), r14); r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x88u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xa0u64), r15); r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; bb = 6; continue; }
            6 => { crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rdi); rcx = rsp.wrapping_add(0x60); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), r14); r8 = r13; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rbp); rdx = rbx; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), r15); let y_addr: u64 = rsp.wrapping_add(0x30u64); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rsi); xmm1 = ((crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64) as u128; let y_addr: u64 = rsp.wrapping_add(0x60u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); crate::simd_ops::wu64(rsp.wrapping_add(0x80u64), ((xmm1 as u64)) as u64); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rsi = ((rsi).wrapping_add(1)) as u64; fl_l = rsi; fl_r = r12; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 6; continue; } bb = 7; continue; }
            7 => { r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0xa0u64))) as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0xa8u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0xd8u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0xd0u64))) as u64; bb = 8; continue; }
            8 => { r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0xe0u64))) as u64; r13 = (crate::simd_ops::ru64(rsp.wrapping_add(0xb0u64))) as u64; bb = 9; continue; }
            9 => { rsp = ((rsp).wrapping_add(0xb8u64)) as u64; rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_65573<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 25>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { r11 = rsp; rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0xc0u64)) as u64; rax = (((r8 as u16)) as u32) as u32 as u64; rdi = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = (crate::simd_ops::ru64(rdx)) as u64; rdi = ((rdi).wrapping_add(rdi)) as u64; rdx = (crate::simd_ops::ru32(rax.wrapping_add(rdi.wrapping_mul(8)).wrapping_add(0x8u64))) as u32 as u64; rdi = (crate::simd_ops::ru64(rax.wrapping_add(rdi.wrapping_mul(8)))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xe8u64), rdi); fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 23; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(r11.wrapping_sub(0x10u64), rbp); rbp = ((rdi.wrapping_add(rdx.wrapping_mul(2))) as u64) as u64; crate::simd_ops::wu64(r11.wrapping_sub(0x18u64), rsi); rsi = (crate::simd_ops::ru64(rcx)) as u64; rcx = ((crate::simd_ops::ru16(P0)) as u32) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbp); rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x580e0u64))) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(rcx.wrapping_mul(8)))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax); fl_l = rdi; fl_r = rbp; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 22; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::wu64(r11.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(r11.wrapping_sub(0x20u64), r12); crate::simd_ops::wu64(r11.wrapping_sub(0x28u64), r13); crate::simd_ops::wu64(r11.wrapping_sub(0x30u64), r14); crate::simd_ops::wu64(r11.wrapping_sub(0x38u64), r15); bb = 3; continue; }
            3 => { r11 = ((crate::simd_ops::ru16(rdi)) as u32) as u32 as u64; r13 = (crate::simd_ops::ru64(rsi.wrapping_add(r11.wrapping_mul(8)).wrapping_add(0x88u64))) as u64; fl_l = r13; fl_r = r13; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru32(rsi.wrapping_add(r11.wrapping_mul(4)).wrapping_add(0x10088u64))) as u32 as u64; bb = 6; continue; }
            5 => { rax = 0; fl_l = 0; fl_r = 0; bb = 6; continue; }
            6 => { crate::simd_ops::wu64(rsp.wrapping_add(0xd8u64), rax); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 20; continue; } bb = 7; continue; }
            7 => { r8 = ((crate::simd_ops::ru16(P1)) as u32) as u32 as u64; rax = r11; rax = ((rax).wrapping_shl(5u32)) as u64; r12 = (crate::simd_ops::ru64(rax.wrapping_add(rsi).wrapping_add(0x18098u64))) as u64; r15 = (crate::simd_ops::ru32(rax.wrapping_add(rsi).wrapping_add(0x180acu64))) as u32 as u64; rcx = r12; rdx = ((r15 as u32)) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r12); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = ((((rax as u32)) as i32 as i64) as u64) as u64; r14 = 0xc03; fl_l = rcx; fl_r = (0xffffffffffffffffu64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 9; continue; } bb = 8; continue; }
            8 => { rax = r11.wrapping_add(0xc03); fl_l = ((rax).wrapping_shl(5u32)) as u64; rax = ((fl_l as u64)) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(rsi).wrapping_add(0x40u64))) as u64; rbx = (crate::simd_ops::ru32(rax.wrapping_add(rcx.wrapping_mul(4)))) as u32 as u64; bb = 10; continue; }
            9 => { rbx = 0; fl_l = 0; fl_r = 0; bb = 10; continue; }
            10 => { r8 = ((crate::simd_ops::ru16(P3)) as u32) as u32 as u64; rdx = r15; rcx = r12; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r11 = ((r11).wrapping_add(r14)) as u64; rcx = ((((rax as u32)) as i32 as i64) as u64) as u64; r11 = ((r11).wrapping_shl(5u32)) as u64; fl_l = rcx; fl_r = (0xffffffffffffffffu64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 12; continue; } bb = 11; continue; }
            11 => { rax = (crate::simd_ops::ru64(r11.wrapping_add(rsi).wrapping_add(0x40u64))) as u64; r12 = (crate::simd_ops::ru32(rax.wrapping_add(rcx.wrapping_mul(4)))) as u32 as u64; bb = 13; continue; }
            12 => { r12 = 0; fl_l = 0; fl_r = 0; bb = 13; continue; }
            13 => { r8 = ((crate::simd_ops::ru16(P4)) as u32) as u32 as u64; rdx = r15; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = ((((rax as u32)) as i32 as i64) as u64) as u64; fl_l = rcx; fl_r = (0xffffffffffffffffu64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 15; continue; } bb = 14; continue; }
            14 => { rax = (crate::simd_ops::ru64(r11.wrapping_add(rsi).wrapping_add(0x40u64))) as u64; r15 = (crate::simd_ops::ru32(rax.wrapping_add(rcx.wrapping_mul(4)))) as u32 as u64; bb = 16; continue; }
            15 => { r15 = 0; fl_l = 0; fl_r = 0; bb = 16; continue; }
            16 => { fl_l = rbx; fl_r = rbx; rax = ((rbx.wrapping_add(r13)) as u64) as u64; rbx = (if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { rax } else { rbx }) as u64; fl_l = r12; fl_r = r12; rax = ((r12.wrapping_add(r13)) as u64) as u64; r12 = (if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { rax } else { r12 }) as u64; fl_l = r15; fl_r = r15; rax = ((r15.wrapping_add(r13)) as u64) as u64; r15 = (if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { rax } else { r15 }) as u64; r14 = 0; fl_l = 0; fl_r = 0; fl_l = (crate::simd_ops::ru64(rsp.wrapping_add(0xd8u64))) as u64; fl_r = r14; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 20; continue; } bb = 17; continue; }
            17 => { rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0xd8u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; bb = 18; continue; }
            18 => { crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rbx); rcx = rsp.wrapping_add(0x60); crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), r12); r8 = rdi; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), r15); rdx = rsi; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), r13); let y_addr: u64 = rsp.wrapping_add(0x38u64); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), r14); xmm1 = ((crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64) as u128; let y_addr: u64 = rsp.wrapping_add(0x60u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); crate::simd_ops::wu64(rsp.wrapping_add(0x80u64), ((xmm1 as u64)) as u64); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r14 = ((r14).wrapping_add(1)) as u64; fl_l = r14; fl_r = rbp; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 18; continue; } bb = 19; continue; }
            19 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0xe8u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; bb = 20; continue; }
            20 => { rdi = ((rdi).wrapping_add(0x2u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xe8u64), rdi); fl_l = rdi; fl_r = rbp; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } bb = 21; continue; }
            21 => { r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x90u64))) as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x98u64))) as u64; r13 = (crate::simd_ops::ru64(rsp.wrapping_add(0xa0u64))) as u64; r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0xa8u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0xd0u64))) as u64; bb = 22; continue; }
            22 => { rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0xb8u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0xb0u64))) as u64; bb = 23; continue; }
            23 => { rsp = ((rsp).wrapping_add(0xc0u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_65670<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rdi = (crate::simd_ops::ru64(rcx)) as u64; r14 = rdx; rbx = rcx; fl_l = rdi; fl_r = (crate::simd_ops::ru64(rdx)) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 8; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rsi); fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(rdi)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = rdi; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = rdi; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); bb = 3; continue; }
            3 => { rsi = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rbx, rsi); rdi = (crate::simd_ops::ru64(r14)) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(P1)) as u64; rcx = P1; rdx = rdi; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rdx = P3; r14 = rax; rcx = (crate::simd_ops::ru64(rax)) as u64; r8 = (crate::simd_ops::ru64(rcx.wrapping_add(P4))) as u64; rcx = rax; rax = crate::simd_ops::call_stk(r8, rax, rcx, rdx, r8, r9, rsp); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { r8 = P1; rdx = r14; rcx = rax; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rsi = rax; bb = 6; continue; }
            6 => { r9 = (crate::simd_ops::ru64(rdi)) as u64; rdx = rsp.wrapping_add(0x20); r8 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rcx = rdi; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rsi); rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(r9.wrapping_add(P6))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx, rsi); bb = 7; continue; }
            7 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rax = rbx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            8 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rax = rbx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_65672<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r9); crate::simd_ops::wu32(rsp.wrapping_add(0x18u64), ((r8 as u32)) as u32); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbp = rcx; rdi = rdx; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(0x338u64))) as u64; rdx = rsp.wrapping_add(0x50); rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x80u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru8(rsp.wrapping_add(0x50u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 3; continue; } bb = 1; continue; }
            1 => { rbx = (crate::simd_ops::ru64(rdi)) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 27; continue; } bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = rbx; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rdi, (0x0u64) as u64); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            3 => { rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x338u64))) as u64; rdx = rsp.wrapping_add(0x58); rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x40u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rdx = (crate::simd_ops::ru64(rdi)) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 14; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(P1)) as u64; rcx = P1; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x28u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = rax; rdx = (crate::simd_ops::ru64(rax)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rdx.wrapping_add(0x8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rsp.wrapping_add(0x58u64))) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 14; continue; } bb = 5; continue; }
            5 => { rbx = (crate::simd_ops::ru64(rdi)) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 13; continue; } bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = rbx.wrapping_sub(0x1); rax = rbx; rcx = (((rcx) & (0xfffffffffe000000u64))) as u64; rax = ((rax).wrapping_sub(rcx)) as u64; rax = ((rax).wrapping_shr(16u32)) as u64; rdx = rcx.wrapping_add(0x108); rax = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x4u64))) as u32 as u64; rdx = ((rdx).wrapping_sub(rax)) as u64; rax = ((crate::exe::teb_self() as u64)) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(rcx.wrapping_add(0x100u64))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 11; continue; } bb = 7; continue; }
            7 => { fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(0xeu64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 10; continue; } bb = 8; continue; }
            8 => { rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(rbx, rax); fl_l = (crate::simd_ops::lk_add_u32(rdx.wrapping_add(0x18u64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; crate::simd_ops::wu64(rdx.wrapping_add(0x20u64), rbx); if (((fl_l as u32)) != 0) { bb = 13; continue; } bb = 9; continue; }
            9 => { rcx = rdx; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 13; continue; }
            10 => { r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 12; continue; }
            11 => { r8 = 0; fl_l = 0; fl_r = 0; bb = 12; continue; }
            12 => { r9 = rbx; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 13; continue; }
            13 => { crate::simd_ops::wu64(rdi, (0x0u64) as u64); bb = 14; continue; }
            14 => { rbx = (crate::simd_ops::ru64(rdi)) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 26; continue; } bb = 15; continue; }
            15 => { rcx = (crate::simd_ops::ru32(rsp.wrapping_add(0x58u64))) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rsi); rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rcx = rax; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru32(rsp.wrapping_add(0x58u64))) as u32 as u64; rbx = rax; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); r8 = P1; rdx = rax; rcx = rbx; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(rdi)) as u64; rsi = rax; fl_l = rbx; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 25; continue; } bb = 16; continue; }
            16 => { fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 24; continue; } bb = 17; continue; }
            17 => { r8 = (crate::simd_ops::ru64(rbx)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(r8)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = rbx.wrapping_sub(0x1); rax = rbx; rcx = (((rcx) & (0xfffffffffe000000u64))) as u64; rax = ((rax).wrapping_sub(rcx)) as u64; rax = ((rax).wrapping_shr(16u32)) as u64; rdx = rcx.wrapping_add(0x108); rax = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x4u64))) as u32 as u64; rdx = ((rdx).wrapping_sub(rax)) as u64; rax = ((crate::exe::teb_self() as u64)) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(rcx.wrapping_add(0x100u64))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 22; continue; } bb = 18; continue; }
            18 => { fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(0xeu64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 21; continue; } bb = 19; continue; }
            19 => { rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(rbx, rax); fl_l = (crate::simd_ops::lk_add_u32(rdx.wrapping_add(0x18u64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; crate::simd_ops::wu64(rdx.wrapping_add(0x20u64), rbx); if (((fl_l as u32)) != 0) { bb = 24; continue; } bb = 20; continue; }
            20 => { rcx = rdx; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 24; continue; }
            21 => { r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 23; continue; }
            22 => { r8 = 0; fl_l = 0; fl_r = 0; bb = 23; continue; }
            23 => { r9 = rbx; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 24; continue; }
            24 => { crate::simd_ops::wu64(rdi, rsi); rbx = rsi; bb = 25; continue; }
            25 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; bb = 26; continue; }
            26 => { rax = (crate::simd_ops::ru64(rbx)) as u64; r8 = 0; fl_l = 0; fl_r = 0; rdx = rbp; rcx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x70u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 27; continue; }
            27 => { rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_65675<const P0: u64, const P1: u64, const P2: u64, const P3: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rdx; rdi = rcx; rsi = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(P1))) as u32 as u64; fl_l = rdx; fl_r = rax; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 3; continue; } bb = 1; continue; }
            1 => { rdx = ((rax as u32)) as u32 as u64; rdx = ((rdx).wrapping_shr(1u32)) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; fl_l = rdx; fl_r = rbx; rdx = (if (((fl_l as u64)) < ((fl_r as u64))) { rbx } else { rdx }) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); r8 = rbx; fl_l = r8; fl_r = rsi; r8 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 6; continue; } bb = 2; continue; }
            2 => { rcx = (crate::simd_ops::ru32(rdi.wrapping_add(P0))) as u32 as u64; rax = (crate::simd_ops::ru64(rdi)) as u64; rcx = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; r8 = ((r8).wrapping_shl(3u32)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu32(rdi.wrapping_add(P0), ((rbx as u32)) as u32); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            3 => { fl_l = rbx; fl_r = rsi; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 7; continue; } bb = 4; continue; }
            4 => { r8 = rbx; fl_l = r8; fl_r = rsi; r8 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rcx = ((rax.wrapping_add(rsi.wrapping_mul(8))) as u64) as u64; r8 = ((r8).wrapping_shl(3u32)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 6; continue; }
            6 => { crate::simd_ops::wu32(rdi.wrapping_add(P0), ((rbx as u32)) as u32); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            7 => { if (((fl_l as u64)) == ((fl_r as u64))) { bb = 9; continue; } bb = 8; continue; }
            8 => { crate::simd_ops::wu32(rcx.wrapping_add(P0), ((rbx as u32)) as u32); bb = 9; continue; }
            9 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_65756<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x28u64)) as u64; rdx = ((r9.wrapping_mul(8)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbx); fl_l = rdx; fl_r = rdx; rax = ((rdx.wrapping_add(P0)) as u64) as u64; rcx = 0x11; rdx = 0x10; rcx = (if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { rax } else { rcx }) as u64; rbx = r9; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); rcx = rax; rdx = 0xdeadd00d; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); rax = 0; fl_l = 0; fl_r = 0; bb = 2; continue; }
            2 => { crate::simd_ops::wu64(rax, (0x8u64) as u64); rdx = ((rax.wrapping_add(P0)) as u64) as u64; crate::simd_ops::wu32(rax.wrapping_add(P4), ((rbx as u32)) as u32); crate::simd_ops::wu32(rax.wrapping_add(P5), (0x8u32) as u32); fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rax = P6; rdi = rdx; rcx = rbx; while rcx != 0 { crate::simd_ops::wu64(rdi, rax); rdi = rdi.wrapping_add(8); rcx = rcx.wrapping_sub(1); } rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; bb = 4; continue; }
            4 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = rdx; rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_65793<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64, const P9: u64, const P10: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x40u64)) as u64;
    xmm0 = ((crate::simd_ops::ru64(P0)) as u64) as u128;
    crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), ((xmm0 as u64)) as u64);
    rdi = rdx;
    rax = P1;
    crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax);
    rdx = ((rcx.wrapping_add(P2)) as u64) as u64;
    rax = P3;
    rbx = rcx;
    r9 = P4;
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax);
    r8 = 0x2;
    rcx = rdi;
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    xmm0 = ((crate::simd_ops::ru64(P6)) as u64) as u128;
    crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), ((xmm0 as u64)) as u64);
    rax = P7;
    r8 = 0x2;
    crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax);
    rdx = ((rbx.wrapping_add(P8)) as u64) as u64;
    rax = P9;
    rcx = rdi;
    r9 = P10;
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax);
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64;
    rax = 0;
    fl_l = 0; fl_r = 0;
    rsp = ((rsp).wrapping_add(0x40u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_65875<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x40u64)) as u64;
    r9 = (crate::simd_ops::ru32(P0)) as u32 as u64;
    rdi = rdx;
    r8 = (crate::simd_ops::ru64(P1)) as u64;
    rdx = rcx;
    rbx = rcx;
    crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), (0x1u32) as u32);
    rcx = rdi;
    crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), (0x3u32) as u32);
    rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp);
    rdx = rdi;
    rcx = rbx;
    rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp);
    rdx = 0x2;
    rcx = rdi;
    r8 = ((rdx.wrapping_add(P4)) as u32) as u32 as u64;
    rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp);
    rdx = 0x2;
    rcx = rdi;
    r8 = ((rdx.wrapping_add(P4)) as u32) as u32 as u64;
    rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp);
    rcx = rdi;
    rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp);
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64;
    rax = 0;
    fl_l = 0; fl_r = 0;
    rsp = ((rsp).wrapping_add(0x40u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_65919<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; r9 = (crate::simd_ops::ru32(P0)) as u32 as u64; rsi = rdx; r8 = (crate::simd_ops::ru64(P1)) as u64; r14 = 0; fl_l = 0; fl_r = 0; rdx = rcx; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rbp); rbp = rcx; crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((r14 as u32)) as u32); rcx = rsi; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((r14 as u32)) as u32); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); r8 = 0; fl_l = 0; fl_r = 0; rdx = rbp; rcx = rsi; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru32(rsi.wrapping_add(0x328u64))) as u32 as u64; fl_r = ((r14 as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 2; continue; } bb = 1; continue; }
            1 => { rbx = (crate::simd_ops::ru64(rsi.wrapping_add(0x338u64))) as u64; rdx = rbp.wrapping_add(0x18); rcx = rbx; rax = (crate::simd_ops::ru64(rbx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x38u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = rbp.wrapping_add(0x1c); rcx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x38u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = rbp.wrapping_add(0x20); rcx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x38u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rax = rsi.wrapping_add(0x369); bb = 4; continue; }
            2 => { rax = rsi.wrapping_add(0x369); fl_l = (crate::simd_ops::ru8(rax)) as u8 as u64; fl_r = ((r14 as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 4; continue; } bb = 3; continue; }
            3 => { crate::simd_ops::wu64(rbp.wrapping_add(0x18u64), r14); crate::simd_ops::wu32(rbp.wrapping_add(0x20u64), ((r14 as u32)) as u32); bb = 4; continue; }
            4 => { rdi = rbp.wrapping_add(0x24); rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; fl_l = (crate::simd_ops::ru32(rsi.wrapping_add(0x328u64))) as u32 as u64; fl_r = ((r14 as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 6; continue; } bb = 5; continue; }
            5 => { rbx = (crate::simd_ops::ru64(rsi.wrapping_add(0x338u64))) as u64; rdx = rdi; rcx = rbx; rax = (crate::simd_ops::ru64(rbx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x38u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = rdi.wrapping_add(0x4); rcx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x38u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = rdi.wrapping_add(0x8); rcx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x38u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 8; continue; }
            6 => { fl_l = (crate::simd_ops::ru8(rax)) as u8 as u64; fl_r = ((r14 as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 8; continue; } bb = 7; continue; }
            7 => { crate::simd_ops::wu64(rdi, r14); crate::simd_ops::wu32(rdi.wrapping_add(0x8u64), ((r14 as u32)) as u32); bb = 8; continue; }
            8 => { rcx = rsi; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rax = 0; fl_l = 0; fl_r = 0; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_66008<const P0: u64, const P1: u64, const P2: u64, const P3: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbx); crate::simd_ops::wu32(rsp.wrapping_add(0x18u64), ((r8 as u32)) as u32); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbp = rcx; crate::simd_ops::wu32(rsp.wrapping_add(0x40u64), (0x0u32) as u32); rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rbx = rdx; rdx = rsp.wrapping_add(0x40); rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rdx = (((crate::simd_ops::ru32(rsp.wrapping_add(0x40u64))) as i32 as i64) as u64) as u64; rcx = rbx; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x40u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 4; continue; } bb = 1; continue; }
            1 => { rbx = (crate::simd_ops::ru64(rbx)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rdi); rdi = ((rax as u32)) as u32 as u64; bb = 2; continue; }
            2 => { rsi = (crate::simd_ops::ru64(rbp.wrapping_add(0x338u64))) as u64; rcx = rbx; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rsi)) as u64; rdx = rbx; rcx = rsi; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rbx = ((rbx).wrapping_add(0x20u64)) as u64; fl_l = rdi; fl_r = (0x1u64) as u64; rdi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 3; continue; }
            3 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; bb = 4; continue; }
            4 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_66024<const P0: u64, const P1: u64, const P2: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rdx; rdi = rcx; rsi = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(P1))) as u32 as u64; fl_l = rdx; fl_r = rax; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 5; continue; } bb = 1; continue; }
            1 => { rdx = ((rax as u32)) as u32 as u64; rdx = ((rdx).wrapping_shr(1u32)) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; fl_l = rdx; fl_r = rbx; rdx = (if (((fl_l as u64)) < ((fl_r as u64))) { rbx } else { rdx }) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); r8 = rbx; fl_l = r8; fl_r = rsi; r8 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 9; continue; } bb = 2; continue; }
            2 => { rcx = (crate::simd_ops::ru32(rdi.wrapping_add(P0))) as u32 as u64; rax = (crate::simd_ops::ru64(rdi)) as u64; rdx = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; rax = 0; fl_l = 0; fl_r = 0; bb = 3; continue; }
            3 => { crate::simd_ops::wu64(rdx, rax); rdx = ((rdx.wrapping_add(P0)) as u64) as u64; fl_l = r8; fl_r = (0x1u64) as u64; r8 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } bb = 4; continue; }
            4 => { crate::simd_ops::wu32(rdi.wrapping_add(P0), ((rbx as u32)) as u32); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            5 => { fl_l = rbx; fl_r = rsi; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 10; continue; } bb = 6; continue; }
            6 => { fl_l = rdx; fl_r = rsi; rdx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 9; continue; } bb = 7; continue; }
            7 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rcx = ((rax.wrapping_add(rsi.wrapping_mul(8))) as u64) as u64; rax = 0; fl_l = 0; fl_r = 0; bb = 8; continue; }
            8 => { crate::simd_ops::wu64(rcx, rax); rcx = ((rcx.wrapping_add(P0)) as u64) as u64; fl_l = rdx; fl_r = (0x1u64) as u64; rdx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 8; continue; } bb = 9; continue; }
            9 => { crate::simd_ops::wu32(rdi.wrapping_add(P0), ((rbx as u32)) as u32); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            10 => { if (((fl_l as u64)) == ((fl_r as u64))) { bb = 12; continue; } bb = 11; continue; }
            11 => { crate::simd_ops::wu32(rcx.wrapping_add(P0), ((rbx as u32)) as u32); bb = 12; continue; }
            12 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_66026<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rdx; rdi = rcx; rsi = (crate::simd_ops::ru32(rcx.wrapping_add(0x8u64))) as u32 as u64; rcx = ((rsi as u32)) as u32 as u64; rcx = ((rcx).wrapping_shl(5u32)) as u64; rax = (crate::simd_ops::ru32(rdi.wrapping_add(0xcu64))) as u32 as u64; fl_l = rdx; fl_r = rax; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 5; continue; } bb = 1; continue; }
            1 => { rdx = ((rax as u32)) as u32 as u64; rdx = ((rdx).wrapping_shr(1u32)) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; fl_l = rdx; fl_r = rbx; rdx = (if (((fl_l as u64)) < ((fl_r as u64))) { rbx } else { rdx }) as u64; rcx = rdi; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rdx = rbx; fl_l = rdx; fl_r = rsi; rdx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 11; continue; } bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x8u64))) as u32 as u64; rax = ((rax).wrapping_shl(5u32)) as u64; rax = ((rax).wrapping_add(crate::simd_ops::ru64(rdi))) as u64; rcx = 0; fl_l = 0; fl_r = 0; bb = 3; continue; }
            3 => { crate::simd_ops::wu64(rax, rcx); crate::simd_ops::wu32(rax.wrapping_add(0x8u64), ((rcx as u32)) as u32); crate::simd_ops::wu64(rax.wrapping_add(0x10u64), rcx); crate::simd_ops::wu8(rax.wrapping_add(0x18u64), ((rcx as u8)) as u8); rax = rax.wrapping_add(0x20); fl_l = rdx; fl_r = (0x1u64) as u64; rdx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } bb = 4; continue; }
            4 => { crate::simd_ops::wu32(rdi.wrapping_add(0x8u64), ((rbx as u32)) as u32); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            5 => { fl_l = rbx; fl_r = rsi; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 10; continue; } bb = 6; continue; }
            6 => { r8 = rbx; fl_l = r8; fl_r = rsi; r8 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 11; continue; } bb = 7; continue; }
            7 => { rdx = (crate::simd_ops::ru64(rdi)) as u64; rdx = ((rdx).wrapping_add(rcx)) as u64; rcx = 0; fl_l = 0; fl_r = 0; bb = 8; continue; }
            8 => { crate::simd_ops::wu64(rdx, rcx); crate::simd_ops::wu32(rdx.wrapping_add(0x8u64), ((rcx as u32)) as u32); crate::simd_ops::wu64(rdx.wrapping_add(0x10u64), rcx); crate::simd_ops::wu8(rdx.wrapping_add(0x18u64), ((rcx as u8)) as u8); rdx = rdx.wrapping_add(0x20); fl_l = r8; fl_r = (0x1u64) as u64; r8 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 8; continue; } bb = 9; continue; }
            9 => { crate::simd_ops::wu32(rdi.wrapping_add(0x8u64), ((rbx as u32)) as u32); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            10 => { if (((fl_l as u64)) == ((fl_r as u64))) { bb = 12; continue; } bb = 11; continue; }
            11 => { crate::simd_ops::wu32(rdi.wrapping_add(0x8u64), ((rbx as u32)) as u32); bb = 12; continue; }
            12 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_66027<const P0: u64, const P1: u64, const P2: u64, const P3: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rdx; rsi = rcx; rax = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rax = (0x2aaaaaaaaaaaaaabu64) as u64; let prod = (rax as i64 as i128).wrapping_mul((rcx) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; rdi = rdx; rdi = ((((rdi) as i64).wrapping_shr(3u32) as u64)) as u64; rax = rdi; rax = ((rax).wrapping_shr(63u32)) as u64; rdi = ((rdi).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru32(rsi.wrapping_add(P1))) as u32 as u64; fl_l = rbx; fl_r = rax; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rdx = ((rax as u32)) as u32 as u64; rdx = ((rdx).wrapping_shr(1u32)) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; fl_l = rdx; fl_r = rbx; rdx = (if (((fl_l as u64)) < ((fl_r as u64))) { rbx } else { rdx }) as u64; rcx = rsi; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rdx = rbx; rdx = ((rdx).wrapping_sub(rdi)) as u64; rax = (crate::simd_ops::ru32(rsi.wrapping_add(P0))) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rsi))) as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu32(rsi.wrapping_add(P0), ((rbx as u32)) as u32); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            2 => { fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rdx = rbx; rdx = ((rdx).wrapping_sub(rdi)) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rsi))) as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu32(rsi.wrapping_add(P0), ((rbx as u32)) as u32); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            4 => { if (((fl_l as u64)) == ((fl_r as u64))) { bb = 6; continue; } bb = 5; continue; }
            5 => { crate::simd_ops::wu32(rsi.wrapping_add(P0), ((rbx as u32)) as u32); bb = 6; continue; }
            6 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_66098<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0; let mut ymm1_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 29>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((r9 as u32)) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), r8); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdx); crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rbp = rsp.wrapping_sub(0x17); rsp = ((rsp).wrapping_sub(0xa8u64)) as u64; r9 = rdx; r14 = rcx; fl_l = (crate::simd_ops::ru64(rdx.wrapping_add(0x10u64))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 16; continue; } bb = 1; continue; }
            1 => { r8 = (crate::simd_ops::ru64(rcx)) as u64; fl_l = r8; fl_r = (crate::simd_ops::ru64(rcx.wrapping_add(0x10u64))) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 98; continue; } bb = 2; continue; }
            2 => { rdi = 0; fl_l = 0; fl_r = 0; rsi = (0xffffffffffffu64) as u64; rax = (0xcbf29ce484222325u64) as u64; r15 = 0x1000; r12 = (0x100000001b3u64) as u64; bb = 3; continue; }
            3 => { crate::simd_ops::wu64(rbp.wrapping_add(0x6fu64), rdi); fl_l = (crate::simd_ops::ru16(r8.wrapping_add(0xeu64))) as u16 as u64; fl_r = ((r15 as u16)) as u16 as u64; if ((((fl_l as u16)) & ((fl_r as u16))) != 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { r8 = (crate::simd_ops::ru64(r8.wrapping_add(0x8u64))) as u64; fl_l = (((r8) & (rsi))) as u64; fl_r = fl_l; r8 = ((fl_l as u64)) as u64; bb = 5; continue; }
            5 => { rcx = (0xffffffffffffffffu64) as u64; bb = 6; continue; }
            6 => { rcx = ((rcx).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru8(r8.wrapping_add(rcx))) as u8 as u64; fl_r = ((rdi as u8)) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 6; continue; } bb = 7; continue; }
            7 => { fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 9; continue; } bb = 8; continue; }
            8 => { rax = rdi; bb = 12; continue; }
            9 => { rdx = ((r8.wrapping_add(rcx)) as u64) as u64; fl_l = r8; fl_r = rdx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 12; continue; } bb = 10; continue; }
            10 => { bb = 11; continue; }
            11 => { rcx = (((crate::simd_ops::ru8(r8)) as i8 as i64) as u64) as u64; rcx = (((rcx) | (0x20u64))) as u64; rax = (((rax) ^ (rcx))) as u64; rax = (((rax) as i64).wrapping_mul((r12) as i64) as u64) as u64; r8 = ((r8).wrapping_add(1)) as u64; fl_l = r8; fl_r = rdx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 11; continue; } bb = 12; continue; }
            12 => { crate::simd_ops::wu64(rbp.wrapping_add(0x6fu64), rax); r8 = rbp.wrapping_add(0x6f); rdx = rbp.wrapping_sub(0x51); rcx = r9; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = (crate::simd_ops::ru64(rax)) as u64; rdx = ((rdx).wrapping_add(0x18u64)) as u64; rax = (crate::simd_ops::ru64(r14)) as u64; fl_l = (crate::simd_ops::ru16(rax.wrapping_add(0xeu64))) as u16 as u64; fl_r = ((r15 as u16)) as u16 as u64; if ((((fl_l as u16)) & ((fl_r as u16))) != 0) { bb = 14; continue; } bb = 13; continue; }
            13 => { rax = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; fl_l = (((rax) & (rsi))) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; bb = 14; continue; }
            14 => { r9 = rax; r8 = 0; fl_l = 0; fl_r = 0; rcx = r14.wrapping_add(0x20); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r8 = (crate::simd_ops::ru64(r14)) as u64; r8 = ((r8).wrapping_add(0x20u64)) as u64; crate::simd_ops::wu64(r14, r8); fl_l = r8; fl_r = (crate::simd_ops::ru64(r14.wrapping_add(0x10u64))) as u64; r9 = (crate::simd_ops::ru64(rbp.wrapping_add(0x67u64))) as u64; rax = (0xcbf29ce484222325u64) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } bb = 15; continue; }
            15 => { bb = 98; continue; }
            16 => { xmm0 = 0u128; ymm0_hi = 0; crate::simd_ops::wu128(rbp.wrapping_sub(0x61u64), (xmm0) as u128); r13 = 0; fl_l = 0; fl_r = 0; r15 = ((r13 as u32)) as u32 as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x7fu64), r13); crate::simd_ops::wu64(rbp.wrapping_sub(0x61u64), r13); rsi = ((r13 as u32)) as u32 as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x59u64), r13); rdi = ((r13 as u32)) as u32 as u64; rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x8u64))) as u64; rbx = (crate::simd_ops::ru64(rax)) as u64; fl_l = rbx; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 24; continue; } bb = 17; continue; }
            17 => { r14 = ((r13 as u32)) as u32 as u64; rdx = ((r13 as u32)) as u32 as u64; bb = 18; continue; }
            18 => { rax = ((rsi as u32)) as u32 as u64; rsi = ((r15.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rax = ((rdi as u32)) as u32 as u64; rcx = ((r15.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; fl_l = rsi; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 22; continue; } bb = 19; continue; }
            19 => { rsi = ((rsi).wrapping_sub(r15)) as u64; r14 = rsi; r14 = ((((r14) as i64).wrapping_shr(3u32) as u64)) as u64; rax = r14.wrapping_add(0x1); rcx = ((rcx).wrapping_sub(r15)) as u64; rcx = ((((rcx) as i64).wrapping_shr(3u32) as u64)) as u64; rdi = rcx; rdi = ((rdi).wrapping_shr(1u32)) as u64; rdi = ((rdi).wrapping_add(rcx)) as u64; fl_l = rdi; fl_r = rax; rdi = (if (((fl_l as u64)) < ((fl_r as u64))) { rax } else { rdi }) as u64; rcx = ((rdi.wrapping_mul(8)) as u64) as u64; rdx = 0x8; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r12 = rax; r8 = rsi; rdx = r15; rcx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = r15; fl_r = r15; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 21; continue; } bb = 20; continue; }
            20 => { rcx = r15; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 21; continue; }
            21 => { r15 = r12; crate::simd_ops::wu64(rbp.wrapping_sub(0x61u64), r12); crate::simd_ops::wu32(rbp.wrapping_sub(0x55u64), ((rdi as u32)) as u32); rdx = ((r14 as u32)) as u32 as u64; r9 = (crate::simd_ops::ru64(rbp.wrapping_add(0x67u64))) as u64; bb = 22; continue; }
            22 => { rax = ((r14 as u32)) as u32 as u64; crate::simd_ops::wu64(r15.wrapping_add(rax.wrapping_mul(8)), rbx); rsi = ((rdx.wrapping_add(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_sub(0x59u64), ((rsi as u32)) as u32); rax = (crate::simd_ops::ru64(rbx)) as u64; rbx = rax; r14 = ((rsi as u32)) as u32 as u64; rdx = ((rsi as u32)) as u32 as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(r9.wrapping_add(0x8u64))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 18; continue; } bb = 23; continue; }
            23 => { crate::simd_ops::wu64(rbp.wrapping_add(0x7fu64), r15); r14 = (crate::simd_ops::ru64(rbp.wrapping_add(0x5fu64))) as u64; bb = 24; continue; }
            24 => { xmm0 = 0u128; ymm0_hi = 0; crate::simd_ops::wu128(rbp.wrapping_sub(0x51u64), (xmm0) as u128); rdx = r13; crate::simd_ops::wu64(rbp.wrapping_add(0x6fu64), rdx); crate::simd_ops::wu64(rbp.wrapping_sub(0x51u64), rdx); rdx = ((r13 as u32)) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x77u64), ((rdx as u32)) as u32); crate::simd_ops::wu32(rbp.wrapping_sub(0x49u64), ((rdx as u32)) as u32); crate::simd_ops::wu32(rbp.wrapping_sub(0x45u64), ((r13 as u32)) as u32); rcx = (crate::simd_ops::ru64(r14)) as u64; fl_l = rcx; fl_r = (crate::simd_ops::ru64(r14.wrapping_add(0x10u64))) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 60; continue; } bb = 25; continue; }
            25 => { r11 = r14.wrapping_add(0x20); rdx = (0xcbf29ce484222325u64) as u64; r10 = 0x1000; r8 = (0xffffffffffffu64) as u64; r12 = (0x100000001b3u64) as u64; bb = 26; continue; }
            26 => { fl_l = (crate::simd_ops::ru16(rcx.wrapping_add(0xeu64))) as u16 as u64; fl_r = ((r10 as u16)) as u16 as u64; if ((((fl_l as u16)) & ((fl_r as u16))) != 0) { bb = 28; continue; } bb = 27; continue; }
            27 => { rcx = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; fl_l = (((rcx) & (r8))) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; bb = 28; continue; }
            28 => { rax = (0xffffffffffffffffu64) as u64; bb = 29; continue; }
            29 => { rax = ((rax).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(rax))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 29; continue; } bb = 30; continue; }
            30 => { fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 32; continue; } bb = 31; continue; }
            31 => { rbx = r13; bb = 35; continue; }
            32 => { rbx = rdx; rdx = ((rax.wrapping_add(rcx)) as u64) as u64; fl_l = rcx; fl_r = rdx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 35; continue; } bb = 33; continue; }
            33 => { bb = 34; continue; }
            34 => { rax = (((crate::simd_ops::ru8(rcx)) as i8 as i64) as u64) as u64; rax = (((rax) | (0x20u64))) as u64; rbx = (((rbx) ^ (rax))) as u64; rbx = (((rbx) as i64).wrapping_mul((r12) as i64) as u64) as u64; rcx = ((rcx).wrapping_add(1)) as u64; fl_l = rcx; fl_r = rdx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 34; continue; } bb = 35; continue; }
            35 => { rcx = rbx; rcx = (((rcx) & (crate::simd_ops::ru64(r9.wrapping_add(0x30u64))))) as u64; rcx = ((rcx).wrapping_add(rcx)) as u64; rax = (crate::simd_ops::ru64(r9.wrapping_add(0x18u64))) as u64; rdi = (crate::simd_ops::ru64(rax.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x8u64))) as u64; rdx = (crate::simd_ops::ru64(r9.wrapping_add(0x8u64))) as u64; fl_l = rdi; fl_r = rdx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 41; continue; } bb = 36; continue; }
            36 => { rax = (crate::simd_ops::ru64(rax.wrapping_add(rcx.wrapping_mul(8)))) as u64; fl_l = rbx; fl_r = (crate::simd_ops::ru64(rdi.wrapping_add(0x10u64))) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 42; continue; } bb = 37; continue; }
            37 => { bb = 38; continue; }
            38 => { fl_l = rdi; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 41; continue; } bb = 39; continue; }
            39 => { rdi = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; fl_l = rbx; fl_r = (crate::simd_ops::ru64(rdi.wrapping_add(0x10u64))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 38; continue; } bb = 40; continue; }
            40 => { bb = 42; continue; }
            41 => { rdi = r13; bb = 42; continue; }
            42 => { fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 55; continue; } bb = 43; continue; }
            43 => { fl_l = rdi; fl_r = rdx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 55; continue; } bb = 44; continue; }
            44 => { rax = (crate::simd_ops::ru64(r14)) as u64; fl_l = (crate::simd_ops::ru16(rax.wrapping_add(0xeu64))) as u16 as u64; fl_r = ((r10 as u16)) as u16 as u64; if ((((fl_l as u16)) & ((fl_r as u16))) != 0) { bb = 46; continue; } bb = 45; continue; }
            45 => { rax = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; fl_l = (((rax) & (r8))) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; bb = 46; continue; }
            46 => { rdx = rdi.wrapping_add(0x18); r9 = rax; r8 = 0; fl_l = 0; fl_r = 0; rcx = r11; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = r15; rcx = ((rsi as u32)) as u32 as u64; rdx = ((r15.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = r15; fl_r = rdx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 58; continue; } bb = 47; continue; }
            47 => { bb = 48; continue; }
            48 => { fl_l = (crate::simd_ops::ru64(rax)) as u64; fl_r = rdi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 52; continue; } bb = 49; continue; }
            49 => { rcx = ((rsi as u32)) as u32 as u64; rcx = ((rcx).wrapping_sub(1)) as u64; rcx = ((r15.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 51; continue; } bb = 50; continue; }
            50 => { rdx = (crate::simd_ops::ru64(rcx)) as u64; crate::simd_ops::wu64(rax, rdx); bb = 51; continue; }
            51 => { fl_l = (((rsi as u32)).wrapping_sub(1)) as u32 as u64; rsi = ((fl_l as u32)) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_sub(0x59u64), ((rsi as u32)) as u32); bb = 53; continue; }
            52 => { fl_l = ((rax).wrapping_add(0x8u64)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; bb = 53; continue; }
            53 => { rcx = ((rsi as u32)) as u32 as u64; rdx = ((r15.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = rax; fl_r = rdx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 48; continue; } bb = 54; continue; }
            54 => { bb = 58; continue; }
            55 => { rcx = rbp.wrapping_sub(0x51); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = rax; crate::simd_ops::wu64(rax, rbx); rax = (crate::simd_ops::ru64(r14)) as u64; rcx = 0x1000; fl_l = (crate::simd_ops::ru16(rax.wrapping_add(0xeu64))) as u16 as u64; fl_r = ((rcx as u16)) as u16 as u64; if ((((fl_l as u16)) & ((fl_r as u16))) != 0) { bb = 57; continue; } bb = 56; continue; }
            56 => { rax = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; rcx = (0xffffffffffffu64) as u64; fl_l = (((rax) & (rcx))) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; bb = 57; continue; }
            57 => { rdx = ((rdx).wrapping_add(0x8u64)) as u64; r9 = rax; r8 = 0; fl_l = 0; fl_r = 0; rcx = r14.wrapping_add(0x20); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 58; continue; }
            58 => { rcx = (crate::simd_ops::ru64(r14)) as u64; rcx = ((rcx).wrapping_add(0x20u64)) as u64; crate::simd_ops::wu64(r14, rcx); fl_l = rcx; fl_r = (crate::simd_ops::ru64(r14.wrapping_add(0x10u64))) as u64; r9 = (crate::simd_ops::ru64(rbp.wrapping_add(0x67u64))) as u64; r8 = (0xffffffffffffu64) as u64; r10 = 0x1000; r11 = r14.wrapping_add(0x20); rdx = (0xcbf29ce484222325u64) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 26; continue; } bb = 59; continue; }
            59 => { rax = (crate::simd_ops::ru32(rbp.wrapping_sub(0x49u64))) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x77u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru64(rbp.wrapping_sub(0x51u64))) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x6fu64), rax); bb = 60; continue; }
            60 => { xmm0 = 0u128; ymm0_hi = 0; crate::simd_ops::wu128(rbp.wrapping_sub(0x71u64), (xmm0) as u128); r14 = r13; crate::simd_ops::wu64(rbp.wrapping_sub(0x71u64), r13); rdi = ((r13 as u32)) as u32 as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x69u64), r13); r12 = ((r13 as u32)) as u32 as u64; rax = ((rsi as u32)) as u32 as u64; rsi = ((r15.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x41u64), rsi); rax = rsi; rax = ((rax).wrapping_sub(r15)) as u64; rax = ((((rax) as i64).wrapping_shr(3u32) as u64)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 62; continue; } bb = 61; continue; }
            61 => { r12 = rax; rcx = ((rax.wrapping_mul(8)) as u64) as u64; rdx = 0x8; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r14 = rax; crate::simd_ops::wu64(rbp.wrapping_sub(0x71u64), rax); crate::simd_ops::wu32(rbp.wrapping_sub(0x69u64), ((r13 as u32)) as u32); crate::simd_ops::wu32(rbp.wrapping_sub(0x65u64), ((r12 as u32)) as u32); bb = 62; continue; }
            62 => { rbx = r15; rax = ((r13 as u32)) as u32 as u64; fl_l = r15; fl_r = rsi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 70; continue; } bb = 63; continue; }
            63 => { rdi = ((r13 as u32)) as u32 as u64; rsi = ((r13 as u32)) as u32 as u64; rdx = ((r13 as u32)) as u32 as u64; r15 = (crate::simd_ops::ru64(rbp.wrapping_sub(0x41u64))) as u64; bb = 64; continue; }
            64 => { r13 = (crate::simd_ops::ru64(rbx)) as u64; rax = ((rdi as u32)) as u32 as u64; rdi = ((r14.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rax = ((r12 as u32)) as u32 as u64; rcx = ((r14.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; fl_l = rdi; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 68; continue; } bb = 65; continue; }
            65 => { rdi = ((rdi).wrapping_sub(r14)) as u64; rsi = rdi; rsi = ((((rsi) as i64).wrapping_shr(3u32) as u64)) as u64; rax = rsi.wrapping_add(0x1); rcx = ((rcx).wrapping_sub(r14)) as u64; rcx = ((((rcx) as i64).wrapping_shr(3u32) as u64)) as u64; r12 = rcx; r12 = ((r12).wrapping_shr(1u32)) as u64; r12 = ((r12).wrapping_add(rcx)) as u64; fl_l = r12; fl_r = rax; r12 = (if (((fl_l as u64)) < ((fl_r as u64))) { rax } else { r12 }) as u64; rcx = ((r12.wrapping_mul(8)) as u64) as u64; rdx = 0x8; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rbp.wrapping_sub(0x41u64), rax); r8 = rdi; rdx = r14; rcx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = r14; fl_r = r14; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 67; continue; } bb = 66; continue; }
            66 => { rcx = r14; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 67; continue; }
            67 => { r14 = (crate::simd_ops::ru64(rbp.wrapping_sub(0x41u64))) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x71u64), r14); crate::simd_ops::wu32(rbp.wrapping_sub(0x65u64), ((r12 as u32)) as u32); rdx = ((rsi as u32)) as u32 as u64; bb = 68; continue; }
            68 => { rcx = ((rsi as u32)) as u32 as u64; rax = (crate::simd_ops::ru64(r13.wrapping_add(0x10u64))) as u64; crate::simd_ops::wu64(r14.wrapping_add(rcx.wrapping_mul(8)), rax); rdi = ((rdx.wrapping_add(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_sub(0x69u64), ((rdi as u32)) as u32); rbx = ((rbx).wrapping_add(0x8u64)) as u64; rsi = ((rdi as u32)) as u32 as u64; rdx = ((rdi as u32)) as u32 as u64; fl_l = rbx; fl_r = r15; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 64; continue; } bb = 69; continue; }
            69 => { rax = ((rdi as u32)) as u32 as u64; r15 = (crate::simd_ops::ru64(rbp.wrapping_add(0x7fu64))) as u64; r13 = 0; fl_l = 0; fl_r = 0; bb = 70; continue; }
            70 => { rbx = r14; rcx = ((r14.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; r12 = (crate::simd_ops::ru64(rbp.wrapping_add(0x67u64))) as u64; fl_l = r14; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 88; continue; } bb = 71; continue; }
            71 => { bb = 72; continue; }
            72 => { r8 = (crate::simd_ops::ru64(rbx)) as u64; rcx = (crate::simd_ops::ru64(r12.wrapping_add(0x30u64))) as u64; rcx = (((rcx) & (r8))) as u64; rcx = ((rcx).wrapping_add(rcx)) as u64; rax = (crate::simd_ops::ru64(r12.wrapping_add(0x18u64))) as u64; rdx = (crate::simd_ops::ru64(rax.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x8u64))) as u64; fl_l = rdx; fl_r = (crate::simd_ops::ru64(r12.wrapping_add(0x8u64))) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 78; continue; } bb = 73; continue; }
            73 => { rax = (crate::simd_ops::ru64(rax.wrapping_add(rcx.wrapping_mul(8)))) as u64; fl_l = r8; fl_r = (crate::simd_ops::ru64(rdx.wrapping_add(0x10u64))) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 79; continue; } bb = 74; continue; }
            74 => { bb = 75; continue; }
            75 => { fl_l = rdx; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 78; continue; } bb = 76; continue; }
            76 => { rdx = (crate::simd_ops::ru64(rdx.wrapping_add(0x8u64))) as u64; fl_l = r8; fl_r = (crate::simd_ops::ru64(rdx.wrapping_add(0x10u64))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 75; continue; } bb = 77; continue; }
            77 => { bb = 79; continue; }
            78 => { rdx = r13; bb = 79; continue; }
            79 => { fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 87; continue; } bb = 80; continue; }
            80 => { rcx = (crate::simd_ops::ru64(r12.wrapping_add(0x30u64))) as u64; rcx = (((rcx) & (r8))) as u64; rcx = ((rcx).wrapping_add(rcx)) as u64; rax = (crate::simd_ops::ru64(r12.wrapping_add(0x18u64))) as u64; r8 = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; r9 = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; rax = (crate::simd_ops::ru64(r8)) as u64; fl_l = (crate::simd_ops::ru64(r9.wrapping_add(0x8u64))) as u64; fl_r = rdx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 84; continue; } bb = 81; continue; }
            81 => { fl_l = rax; fl_r = rdx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 83; continue; } bb = 82; continue; }
            82 => { rax = (crate::simd_ops::ru64(r12.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(r8, rax); crate::simd_ops::wu64(r9.wrapping_add(0x8u64), rax); bb = 86; continue; }
            83 => { rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(r9.wrapping_add(0x8u64), rax); bb = 86; continue; }
            84 => { fl_l = rax; fl_r = rdx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 86; continue; } bb = 85; continue; }
            85 => { rax = (crate::simd_ops::ru64(rdx)) as u64; crate::simd_ops::wu64(r8, rax); bb = 86; continue; }
            86 => { rcx = (crate::simd_ops::ru64(rdx)) as u64; crate::simd_ops::lk_sub_u64(r12.wrapping_add(0x10u64), (1) as u64); rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(rax, rcx); rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(rcx.wrapping_add(0x8u64), rax); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 87; continue; }
            87 => { rbx = ((rbx).wrapping_add(0x8u64)) as u64; rax = ((rdi as u32)) as u32 as u64; rcx = ((r14.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; fl_l = rbx; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 72; continue; } bb = 88; continue; }
            88 => { rdi = (crate::simd_ops::ru64(rbp.wrapping_add(0x6fu64))) as u64; rbx = rdi; rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x77u64))) as u32 as u64; rsi = (((rax) as i64).wrapping_mul((0x58u64) as i64) as u64) as u64; rsi = ((rsi).wrapping_add(rdi)) as u64; fl_l = rdi; fl_r = rsi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 92; continue; } bb = 89; continue; }
            89 => { bb = 90; continue; }
            90 => { let y_addr: u64 = rbx.wrapping_add(0x8u64); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rbp.wrapping_sub(0x31u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); let y_addr: u64 = rbx.wrapping_add(0x28u64); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rbp.wrapping_sub(0x11u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); xmm0 = ((crate::simd_ops::ru64(rbx.wrapping_add(0x48u64))) as u64) as u128; ymm0_hi = 0; crate::simd_ops::wu64(rbp.wrapping_add(0x67u64), ((xmm0 as u64)) as u64); rdi = (crate::simd_ops::ru32(rbx.wrapping_add(0x50u64))) as u32 as u64; r8 = rbx; rdx = rbp.wrapping_sub(0x41); rcx = r12; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru64(rax)) as u64; let y_addr: u64 = rbp.wrapping_sub(0x31u64); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rcx.wrapping_add(0x18u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); let y_addr: u64 = rbp.wrapping_sub(0x11u64); xmm1 = crate::simd_ops::ru128(y_addr); ymm1_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rcx.wrapping_add(0x38u64); crate::simd_ops::wu128(y_addr, xmm1); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm1_hi); xmm0 = ((crate::simd_ops::ru64(rbp.wrapping_add(0x67u64))) as u64) as u128; ymm0_hi = 0; crate::simd_ops::wu64(rcx.wrapping_add(0x58u64), ((xmm0 as u64)) as u64); crate::simd_ops::wu32(rcx.wrapping_add(0x60u64), ((rdi as u32)) as u32); rbx = ((rbx).wrapping_add(0x58u64)) as u64; fl_l = rbx; fl_r = rsi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 90; continue; } bb = 91; continue; }
            91 => { rdi = (crate::simd_ops::ru64(rbp.wrapping_add(0x6fu64))) as u64; bb = 92; continue; }
            92 => { fl_l = r14; fl_r = r14; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 94; continue; } bb = 93; continue; }
            93 => { rcx = r14; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 94; continue; }
            94 => { fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 96; continue; } bb = 95; continue; }
            95 => { rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 96; continue; }
            96 => { fl_l = r15; fl_r = r15; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 98; continue; } bb = 97; continue; }
            97 => { rcx = r15; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 98; continue; }
            98 => { rsp = ((rsp).wrapping_add(0xa8u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_66146<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r9); crate::simd_ops::wu32(rsp.wrapping_add(0x18u64), ((r8 as u32)) as u32); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x38u64)) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64; rbp = rcx; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; rdi = rdx; crate::simd_ops::wu32(rsp.wrapping_add(0x68u64), ((rax as u32)) as u32); rdx = rsp.wrapping_add(0x68); rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru32(rsp.wrapping_add(0x68u64))) as u32 as u64; fl_r = 0x0; if ((((fl_l as u32)) as i32) <= (((fl_r as u32)) as i32)) { bb = 8; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbx); rbx = (crate::simd_ops::ru64(rdi)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r14); r14 = (((crate::simd_ops::ru32(rdi.wrapping_add(P0))) as i32 as i64) as u64) as u64; fl_l = r14; fl_r = r14; if (((((fl_l as u64)) & ((fl_r as u64))) as i64) <= 0) { bb = 7; continue; } bb = 2; continue; }
            2 => { rdi = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rsi); bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x338u64))) as u64; rdx = rsp.wrapping_add(0x60); fl_l = (crate::simd_ops::ru64(rbx.wrapping_add(rdi.wrapping_mul(8)))) as u64; fl_r = (0x0u64) as u64; crate::simd_ops::wu8(rsp.wrapping_add(0x60u64), (if (((fl_l as u64)) != ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8); rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P3))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rdx = (crate::simd_ops::ru64(rbx.wrapping_add(rdi.wrapping_mul(8)))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(P4)) as u64; rcx = P4; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P5))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rsi = (crate::simd_ops::ru64(rbp.wrapping_add(0x338u64))) as u64; rcx = rax; rdx = (crate::simd_ops::ru64(rax)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu32(rsp.wrapping_add(0x50u64), ((rax as u32)) as u32); rdx = rsp.wrapping_add(0x50); rax = (crate::simd_ops::ru64(rsi)) as u64; rcx = rsi; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P6))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(rdi.wrapping_mul(8)))) as u64; r8 = 0; fl_l = 0; fl_r = 0; rdx = rbp; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P7))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 5; continue; }
            5 => { rdi = ((rdi).wrapping_add(1)) as u64; fl_l = rdi; fl_r = r14; if ((((fl_l as u64)) as i64) < (((fl_r as u64)) as i64)) { bb = 3; continue; } bb = 6; continue; }
            6 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; bb = 7; continue; }
            7 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; bb = 8; continue; }
            8 => { rsp = ((rsp).wrapping_add(0x38u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_66152<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64, const P9: u64, const P10: u64, const P11: u64, const P12: u64, const P13: u64, const P14: u64, const P15: u64, const P16: u64, const P17: u64, const P18: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 135>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdx); crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x410u64)) as u64; rbx = r9; r14 = r8; r11 = rdx; r15 = rcx; r13 = (crate::simd_ops::ru32(rsp.wrapping_add(0x460u64))) as u32 as u64; fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(0x54u64))) as u32 as u64; fl_r = ((r13 as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 51; continue; } bb = 1; continue; }
            1 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; rdi = 0; fl_l = 0; fl_r = 0; r10 = ((rdi as u32)) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rdi); crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rdi); rsi = (crate::simd_ops::ru64(rcx.wrapping_add(0x40u64))) as u64; fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 2; continue; }
            2 => { rdx = rbx; rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 3; continue; }
            3 => { fl_l = (crate::simd_ops::ru16(rax.wrapping_add(0x1eu64))) as u16 as u64; fl_r = ((rdi as u16)) as u16 as u64; if (((fl_l as u16)) == ((fl_r as u16))) { bb = 7; continue; } bb = 4; continue; }
            4 => { r10 = rax.wrapping_add(0x10); fl_l = r10; fl_r = r10; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 5; continue; }
            5 => { fl_l = (crate::simd_ops::ru16(r10.wrapping_add(0xeu64))) as u16 as u64; fl_r = (0x4u16) as u16 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { bb = 52; continue; } bb = 6; continue; }
            6 => { crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), r10); rax = (crate::simd_ops::ru64(rsi)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rax); rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 8; continue; }
            7 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; r10 = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; bb = 8; continue; }
            8 => { r11 = (crate::simd_ops::ru64(rsp.wrapping_add(0x448u64))) as u64; bb = 9; continue; }
            9 => { rdx = (crate::simd_ops::ru64(r15.wrapping_add(0x48u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 10; continue; }
            10 => { rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x8u64))) as u64; r10 = (crate::simd_ops::ru32(r15.wrapping_add(0x50u64))) as u32 as u64; r10 = ((r10).wrapping_shl(4u32)) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; r8 = (0xffffffffffffu64) as u64; rcx = (((rcx) & (r8))) as u64; r10 = ((r10).wrapping_add(rcx)) as u64; fl_l = (crate::simd_ops::ru16(r10.wrapping_add(0xeu64))) as u16 as u64; fl_r = (0x4u16) as u16 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { bb = 13; continue; } bb = 11; continue; }
            11 => { crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), r10); rax = (crate::simd_ops::ru64(rdx)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rax); bb = 22; continue; }
            12 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 21; continue; } bb = 13; continue; }
            13 => { crate::simd_ops::wu64(rsp.wrapping_add(0x1f0u64), (0x10000010u64) as u64); crate::simd_ops::wu8(rsp.wrapping_add(0x1f8u64), (0x0u8) as u8); fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 15; continue; } bb = 14; continue; }
            14 => { crate::simd_ops::wu32(rsp.wrapping_add(0x1f4u64), ((rdi as u32)) as u32); crate::simd_ops::wu8(rsp.wrapping_add(0x1f8u64), ((rbx as u8)) as u8); bb = 18; continue; }
            15 => { r9 = (0xffffffffffffffffu64) as u64; bb = 16; continue; }
            16 => { r9 = ((r9).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(r9))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 16; continue; } bb = 17; continue; }
            17 => { crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), r9); r8 = 0; fl_l = 0; fl_r = 0; rdx = rsp.wrapping_add(0x30); rcx = rsp.wrapping_add(0x1f0); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 18; continue; }
            18 => { rdx = (crate::simd_ops::ru32(r15.wrapping_add(0x38u64))) as u32 as u64; r8 = (crate::simd_ops::ru32(r15.wrapping_add(0x3cu64))) as u32 as u64; rcx = (((r8) as i64).wrapping_mul((0x88u64) as i64) as u64) as u64; rax = (((rdx) as i64).wrapping_mul((0x88u64) as i64) as u64) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 20; continue; } bb = 19; continue; }
            19 => { rax = rdx.wrapping_add(0x1); rdx = ((r8 as u32)) as u32 as u64; rdx = ((rdx).wrapping_shr(1u32)) as u64; rdx = ((rdx).wrapping_add(r8)) as u64; fl_l = rdx; fl_r = rax; rdx = (if (((fl_l as u64)) < ((fl_r as u64))) { rax } else { rdx }) as u64; rcx = r15.wrapping_add(0x30); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 20; continue; }
            20 => { rax = (crate::simd_ops::ru32(r15.wrapping_add(0x38u64))) as u32 as u64; rcx = (((rax) as i64).wrapping_mul((0x88u64) as i64) as u64) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(r15.wrapping_add(0x30u64)))) as u64; crate::simd_ops::lk_and_u32(rcx, (0xff000010u32) as u32); crate::simd_ops::lk_or_u32(rcx, (0x10u32) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x3u64), (0x10u8) as u8); crate::simd_ops::wu32(rcx.wrapping_add(0x4u64), ((rdi as u32)) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x8u64), (0x0u8) as u8); rdx = rsp.wrapping_add(0x1f0); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::lk_add_u32(r15.wrapping_add(0x38u64), (1) as u32); rcx = rsp.wrapping_add(0x1f0); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 51; continue; }
            21 => { r8 = (0xffffffffffffu64) as u64; bb = 22; continue; }
            22 => { r12 = (crate::simd_ops::ru32(r10)) as u32 as u64; fl_l = r12; fl_r = r14; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 24; continue; } bb = 23; continue; }
            23 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r14); r9 = rbx; r8 = P5; rdx = ((crate::simd_ops::ru8(P6)) as u32) as u32 as u64; rcx = rsp.wrapping_add(0x1f0); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = r15.wrapping_add(0x20); rdx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P8, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rsp.wrapping_add(0x1f0); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 51; continue; }
            24 => { rcx = (crate::simd_ops::ru32(r15.wrapping_add(0x54u64))) as u32 as u64; xmm0 = 0u128; ymm0_hi = 0; crate::simd_ops::wu128(rsp.wrapping_add(0x80u64), (xmm0) as u128); crate::simd_ops::wu64(rsp.wrapping_add(0x80u64), rdi); crate::simd_ops::wu64(rsp.wrapping_add(0x88u64), rdi); crate::simd_ops::wu8(rsp.wrapping_add(0x90u64), ((rdi as u8)) as u8); let y_addr: u64 = rsp.wrapping_add(0xa0u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); crate::simd_ops::wu64(rsp.wrapping_add(0xa0u64), rdi); crate::simd_ops::wu64(rsp.wrapping_add(0xa8u64), rdi); crate::simd_ops::wu64(rsp.wrapping_add(0xb0u64), rdi); crate::simd_ops::wu64(rsp.wrapping_add(0xb8u64), rdi); crate::simd_ops::wu64(rsp.wrapping_add(0xc0u64), rdi); r9 = rsp.wrapping_add(0x40); crate::simd_ops::wu64(rsp.wrapping_add(0xc8u64), r9); crate::simd_ops::wu32(rsp.wrapping_add(0xd0u64), ((rdi as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0xd4u64), ((rcx as u32)) as u32); rax = (crate::simd_ops::ru32(rsp.wrapping_add(0xd8u64))) as u32 as u64; rax = ((((rax as u32)) & (0xff000010u32))) as u32 as u64; fl_l = ((((rax as u32)) | (0x10u32))) as u32 as u64; fl_r = fl_l; rax = ((fl_l as u32)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0xd8u64), ((rax as u32)) as u32); crate::simd_ops::wu8(rsp.wrapping_add(0xdbu64), (0x10u8) as u8); crate::simd_ops::wu32(rsp.wrapping_add(0xdcu64), ((rdi as u32)) as u32); crate::simd_ops::wu8(rsp.wrapping_add(0xe0u64), ((rdi as u8)) as u8); rax = (crate::simd_ops::ru64(r15.wrapping_add(0x18u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x98u64), rax); r14 = rdi; bb = 25; continue; }
            25 => { fl_l = r14; fl_r = r12; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 36; continue; } bb = 26; continue; }
            26 => { crate::simd_ops::wu32(rsp.wrapping_add(0xd0u64), ((r14 as u32)) as u32); rsi = r14; rsi = ((rsi).wrapping_shl(5u32)) as u64; rsi = ((rsi).wrapping_add(r11)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x1f0u64), (0x10000010u64) as u64); crate::simd_ops::wu8(rsp.wrapping_add(0x1f8u64), (0x0u8) as u8); crate::simd_ops::wu64(rsp.wrapping_add(0x160u64), (0x10000010u64) as u64); crate::simd_ops::wu8(rsp.wrapping_add(0x168u64), (0x0u8) as u8); fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((r13 as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) >= (((fl_r as u32)) as i32)) { bb = 28; continue; } bb = 27; continue; }
            27 => { rax = rsp.wrapping_add(0x168); crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rdi); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x50u64))) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm0) as u128); r9 = 0; fl_l = 0; fl_r = 0; r8 = 0; fl_l = 0; fl_r = 0; rdx = rsp.wrapping_add(0x30); rcx = rsp.wrapping_add(0x1f0); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 33; continue; }
            28 => { rax = (crate::simd_ops::ru64(r9.wrapping_add(0x8u64))) as u64; rcx = ((r14 as u32)) as u32 as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rdx = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; rdx = (((rdx) & (r8))) as u64; rcx = ((rcx).wrapping_add(rdx)) as u64; r9 = (crate::simd_ops::ru64(r9)) as u64; r8 = P9; rdx = rsp.wrapping_add(0x1f0); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P10, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 33; continue; } bb = 29; continue; }
            29 => { r9 = (crate::simd_ops::ru32(rsp.wrapping_add(0x164u64))) as u32 as u64; rdx = ((crate::simd_ops::ru8(rsp.wrapping_add(0x163u64))) as u32) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0x80; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 31; continue; } bb = 30; continue; }
            30 => { rdx = ((rdx.wrapping_mul(8)) as u32) as u32 as u64; bb = 32; continue; }
            31 => { rdx = (((rdx as u32)).wrapping_shl(8u32)) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0x7c00; rdx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 32; continue; }
            32 => { rcx = (crate::simd_ops::ru32(rsp.wrapping_add(0x160u64))) as u32 as u64; rcx = ((((rcx as u32)) & (0xffffffu32))) as u32 as u64; rcx = (((rcx as u32)).wrapping_shl(3u32)) as u32 as u64; rax = rsp.wrapping_add(0x168); fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; rax = (if (((fl_l as u32)) > ((fl_r as u32))) { crate::simd_ops::ru64(rsp.wrapping_add(0x168u64)) } else { rax }) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), r9); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x60u64))) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm0) as u128); r8 = 0; fl_l = 0; fl_r = 0; rdx = rsp.wrapping_add(0x30); rcx = rsp.wrapping_add(0x1f0); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 33; continue; }
            33 => { rcx = rsp.wrapping_add(0x160); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = rdi; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbx); fl_l = (crate::simd_ops::ru32(rsp.wrapping_add(0x1f4u64))) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 35; continue; } bb = 34; continue; }
            34 => { rdx = rsp.wrapping_add(0x1f0); rcx = rsp.wrapping_add(0x30); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P11, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; bb = 35; continue; }
            35 => { rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P12, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rsi, rbx); rcx = rsp.wrapping_add(0x1f0); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((r14).wrapping_add(1)) as u64; r14 = ((fl_l as u64)) as u64; rcx = (crate::simd_ops::ru32(rsp.wrapping_add(0xd4u64))) as u32 as u64; r9 = (crate::simd_ops::ru64(rsp.wrapping_add(0xc8u64))) as u64; r8 = (0xffffffffffffu64) as u64; r11 = (crate::simd_ops::ru64(rsp.wrapping_add(0x448u64))) as u64; bb = 25; continue; }
            36 => { bb = 38; continue; }
            37 => { rdi = 0; fl_l = 0; fl_r = 0; r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x440u64))) as u64; bb = 38; continue; }
            38 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0xa0u64))) as u64; rax = (crate::simd_ops::ru32(rsp.wrapping_add(0xa8u64))) as u32 as u64; r14 = (((rax) as i64).wrapping_mul((0x108u64) as i64) as u64) as u64; r14 = ((r14).wrapping_add(rbx)) as u64; fl_l = rbx; fl_r = r14; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 50; continue; } bb = 39; continue; }
            39 => { bb = 40; continue; }
            40 => { crate::simd_ops::wu64(rsp.wrapping_add(0x300u64), (0x20000020u64) as u64); crate::simd_ops::wu8(rsp.wrapping_add(0x308u64), (0x0u8) as u8); rax = rsp.wrapping_add(0x300); fl_l = rax; fl_r = rbx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 47; continue; } bb = 41; continue; }
            41 => { r9 = (crate::simd_ops::ru32(rbx.wrapping_add(0x4u64))) as u32 as u64; rdx = rbx.wrapping_add(0x8); rcx = ((crate::simd_ops::ru8(rbx.wrapping_add(0x3u64))) as u32) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x80; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 43; continue; } bb = 42; continue; }
            42 => { rcx = ((rcx.wrapping_mul(8)) as u32) as u32 as u64; bb = 44; continue; }
            43 => { rcx = (((rcx as u32)).wrapping_shl(8u32)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x7c00; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 44; continue; }
            44 => { rax = (crate::simd_ops::ru32(rbx)) as u32 as u64; rax = ((((rax as u32)) & (0xffffffu32))) as u32 as u64; rax = (((rax as u32)).wrapping_shl(3u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 46; continue; } bb = 45; continue; }
            45 => { rdx = (crate::simd_ops::ru64(rdx)) as u64; bb = 46; continue; }
            46 => { crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), rdx); crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), r9); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x60u64))) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rsp.wrapping_add(0x50u64), (xmm0) as u128); r8 = 0; fl_l = 0; fl_r = 0; rdx = rsp.wrapping_add(0x50); rcx = rsp.wrapping_add(0x300); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 47; continue; }
            47 => { rdx = (crate::simd_ops::ru32(r15.wrapping_add(0x28u64))) as u32 as u64; r8 = (crate::simd_ops::ru32(r15.wrapping_add(0x2cu64))) as u32 as u64; rcx = (((r8) as i64).wrapping_mul((0x108u64) as i64) as u64) as u64; rax = (((rdx) as i64).wrapping_mul((0x108u64) as i64) as u64) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 49; continue; } bb = 48; continue; }
            48 => { rax = rdx.wrapping_add(0x1); rdx = ((r8 as u32)) as u32 as u64; rdx = ((rdx).wrapping_shr(1u32)) as u64; rdx = ((rdx).wrapping_add(r8)) as u64; fl_l = rdx; fl_r = rax; rdx = (if (((fl_l as u64)) < ((fl_r as u64))) { rax } else { rdx }) as u64; rcx = r15.wrapping_add(0x20); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P13, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 49; continue; }
            49 => { rax = (crate::simd_ops::ru32(r15.wrapping_add(0x28u64))) as u32 as u64; rcx = (((rax) as i64).wrapping_mul((0x108u64) as i64) as u64) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(r15.wrapping_add(0x20u64)))) as u64; crate::simd_ops::lk_and_u32(rcx, (0xff000020u32) as u32); crate::simd_ops::lk_or_u32(rcx, (0x20u32) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x3u64), (0x20u8) as u8); crate::simd_ops::wu32(rcx.wrapping_add(0x4u64), ((rdi as u32)) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x8u64), (0x0u8) as u8); rdx = rsp.wrapping_add(0x300); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::lk_add_u32(r15.wrapping_add(0x28u64), (1) as u32); rcx = rsp.wrapping_add(0x300); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = ((rbx).wrapping_add(0x108u64)) as u64; fl_l = rbx; fl_r = r14; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 40; continue; } bb = 50; continue; }
            50 => { rcx = rsp.wrapping_add(0xd8); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rsp.wrapping_add(0xb0); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P14, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rsp.wrapping_add(0xa0); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P15, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rsp.wrapping_add(0x80); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P15, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 51; continue; }
            51 => { r11 = rsp.wrapping_add(0x410); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x40u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x48u64))) as u64; rsp = r11; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            52 => { r8 = rbx; rdx = (crate::simd_ops::ru64(rsi)) as u64; rcx = rsp.wrapping_add(0x60); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P16, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P17, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::lift_ops::trap(0x80000003, P18, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_66164<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rdx; rdi = rcx; rsi = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; rcx = (crate::simd_ops::ru64(rcx)) as u64; rax = ((rcx.wrapping_add(rsi.wrapping_mul(8))) as u64) as u64; r8 = (crate::simd_ops::ru32(rdi.wrapping_add(P1))) as u32 as u64; fl_l = rdx; fl_r = r8; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 3; continue; } bb = 1; continue; }
            1 => { rdx = ((r8 as u32)) as u32 as u64; rdx = ((rdx).wrapping_shr(1u32)) as u64; rdx = ((rdx).wrapping_add(r8)) as u64; fl_l = rdx; fl_r = rbx; rdx = (if (((fl_l as u64)) < ((fl_r as u64))) { rbx } else { rdx }) as u64; rcx = rdi; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru32(rdi.wrapping_add(P0))) as u32 as u64; rax = (crate::simd_ops::ru64(rdi)) as u64; rcx = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; r8 = rbx; fl_l = r8; fl_r = rsi; r8 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 8; continue; } bb = 2; continue; }
            2 => { r8 = ((r8).wrapping_shl(3u32)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu32(rdi.wrapping_add(P0), ((rbx as u32)) as u32); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            3 => { fl_l = rbx; fl_r = rsi; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 6; continue; } bb = 4; continue; }
            4 => { r8 = rbx; fl_l = r8; fl_r = rsi; r8 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 8; continue; } bb = 5; continue; }
            5 => { r8 = ((r8).wrapping_shl(3u32)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = rax; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu32(rdi.wrapping_add(P0), ((rbx as u32)) as u32); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            6 => { if (((fl_l as u64)) == ((fl_r as u64))) { bb = 9; continue; } bb = 7; continue; }
            7 => { rdx = ((rcx.wrapping_add(rdx.wrapping_mul(8))) as u64) as u64; r8 = rax; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); bb = 8; continue; }
            8 => { crate::simd_ops::wu32(rdi.wrapping_add(P0), ((rbx as u32)) as u32); bb = 9; continue; }
            9 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_66193<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { fl_l = ((r8 as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 30; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((r9 as u32)) as u32); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rdi); r15 = rdx; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r12); rbp = rcx; r12 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rbx); rdi = ((r12 as u32)) as u32 as u64; r14 = ((((r8 as u32)) as i32 as i64) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rsi); bb = 2; continue; }
            2 => { rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x338u64))) as u64; rdx = rsp.wrapping_add(0x68); rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x80u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru8(rsp.wrapping_add(0x68u64))) as u8 as u64; fl_r = ((r12 as u8)) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 5; continue; } bb = 3; continue; }
            3 => { rbx = (crate::simd_ops::ru64(r15.wrapping_add(rdi.wrapping_mul(8)))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 28; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = rbx; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(r15.wrapping_add(rdi.wrapping_mul(8)), r12); bb = 28; continue; }
            5 => { rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x338u64))) as u64; rdx = rsp.wrapping_add(0x60); rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x40u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rdx = (crate::simd_ops::ru64(r15.wrapping_add(rdi.wrapping_mul(8)))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 16; continue; } bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru64(P1)) as u64; rcx = P1; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x28u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = rax; rdx = (crate::simd_ops::ru64(rax)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rdx.wrapping_add(0x8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rsp.wrapping_add(0x60u64))) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 16; continue; } bb = 7; continue; }
            7 => { rbx = (crate::simd_ops::ru64(r15.wrapping_add(rdi.wrapping_mul(8)))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 15; continue; } bb = 8; continue; }
            8 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = rbx.wrapping_sub(0x1); rax = rbx; rcx = (((rcx) & (0xfffffffffe000000u64))) as u64; rax = ((rax).wrapping_sub(rcx)) as u64; rax = ((rax).wrapping_shr(16u32)) as u64; rdx = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rdx = ((rdx).wrapping_shl(4u32)) as u64; rdx = ((rdx).wrapping_add(0x108u64)) as u64; rdx = ((rdx).wrapping_add(rcx)) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x4u64))) as u32 as u64; rdx = ((rdx).wrapping_sub(rax)) as u64; rax = ((crate::exe::teb_self() as u64)) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(rcx.wrapping_add(0x100u64))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 13; continue; } bb = 9; continue; }
            9 => { fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(0xeu64))) as u8 as u64; fl_r = ((r12 as u8)) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 12; continue; } bb = 10; continue; }
            10 => { rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(rbx, rax); fl_l = (crate::simd_ops::lk_add_u32(rdx.wrapping_add(0x18u64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; crate::simd_ops::wu64(rdx.wrapping_add(0x20u64), rbx); if (((fl_l as u32)) != 0) { bb = 15; continue; } bb = 11; continue; }
            11 => { rcx = rdx; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 15; continue; }
            12 => { r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 14; continue; }
            13 => { r8 = 0; fl_l = 0; fl_r = 0; bb = 14; continue; }
            14 => { r9 = rbx; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 15; continue; }
            15 => { crate::simd_ops::wu64(r15.wrapping_add(rdi.wrapping_mul(8)), r12); bb = 16; continue; }
            16 => { rbx = (crate::simd_ops::ru64(r15.wrapping_add(rdi.wrapping_mul(8)))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 27; continue; } bb = 17; continue; }
            17 => { rcx = (crate::simd_ops::ru32(rsp.wrapping_add(0x60u64))) as u32 as u64; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rcx = rax; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru32(rsp.wrapping_add(0x60u64))) as u32 as u64; rbx = rax; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); r8 = P1; rdx = rax; rcx = rbx; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(r15.wrapping_add(rdi.wrapping_mul(8)))) as u64; rsi = rax; fl_l = rbx; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 27; continue; } bb = 18; continue; }
            18 => { fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 26; continue; } bb = 19; continue; }
            19 => { r8 = (crate::simd_ops::ru64(rbx)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(r8)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = rbx.wrapping_sub(0x1); rax = rbx; rcx = (((rcx) & (0xfffffffffe000000u64))) as u64; rax = ((rax).wrapping_sub(rcx)) as u64; rax = ((rax).wrapping_shr(16u32)) as u64; rdx = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rdx = ((rdx).wrapping_shl(4u32)) as u64; rdx = ((rdx).wrapping_add(0x108u64)) as u64; rdx = ((rdx).wrapping_add(rcx)) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x4u64))) as u32 as u64; rdx = ((rdx).wrapping_sub(rax)) as u64; rax = ((crate::exe::teb_self() as u64)) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(rcx.wrapping_add(0x100u64))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 24; continue; } bb = 20; continue; }
            20 => { fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(0xeu64))) as u8 as u64; fl_r = ((r12 as u8)) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 23; continue; } bb = 21; continue; }
            21 => { rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(rbx, rax); fl_l = (crate::simd_ops::lk_add_u32(rdx.wrapping_add(0x18u64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; crate::simd_ops::wu64(rdx.wrapping_add(0x20u64), rbx); if (((fl_l as u32)) != 0) { bb = 26; continue; } bb = 22; continue; }
            22 => { rcx = rdx; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 26; continue; }
            23 => { r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 25; continue; }
            24 => { r8 = 0; fl_l = 0; fl_r = 0; bb = 25; continue; }
            25 => { r9 = rbx; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 26; continue; }
            26 => { crate::simd_ops::wu64(r15.wrapping_add(rdi.wrapping_mul(8)), rsi); rbx = rsi; bb = 27; continue; }
            27 => { rax = (crate::simd_ops::ru64(rbx)) as u64; r8 = 0; fl_l = 0; fl_r = 0; rdx = rbp; rcx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x80u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 28; continue; }
            28 => { rdi = ((rdi).wrapping_add(1)) as u64; fl_l = rdi; fl_r = r14; if ((((fl_l as u64)) as i64) < (((fl_r as u64)) as i64)) { bb = 2; continue; } bb = 29; continue; }
            29 => { r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); bb = 30; continue; }
            30 => { return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_66202<const P0: u64, const P1: u64, const P2: u64, const P3: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rdx; rdi = rcx; rsi = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(P1))) as u32 as u64; fl_l = rdx; fl_r = rax; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 3; continue; } bb = 1; continue; }
            1 => { rdx = ((rax as u32)) as u32 as u64; rdx = ((rdx).wrapping_shr(1u32)) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; fl_l = rdx; fl_r = rbx; rdx = (if (((fl_l as u64)) < ((fl_r as u64))) { rbx } else { rdx }) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); r8 = rbx; fl_l = r8; fl_r = rsi; r8 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 6; continue; } bb = 2; continue; }
            2 => { r8 = ((r8.wrapping_mul(8)) as u64) as u64; rcx = (crate::simd_ops::ru32(rdi.wrapping_add(P0))) as u32 as u64; rax = (crate::simd_ops::ru64(rdi)) as u64; rcx = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; rdx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu32(rdi.wrapping_add(P0), ((rbx as u32)) as u32); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            3 => { fl_l = rbx; fl_r = rsi; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 7; continue; } bb = 4; continue; }
            4 => { r8 = rbx; fl_l = r8; fl_r = rsi; r8 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 6; continue; } bb = 5; continue; }
            5 => { r8 = ((r8.wrapping_mul(8)) as u64) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rcx = ((rax.wrapping_add(rsi.wrapping_mul(8))) as u64) as u64; rdx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 6; continue; }
            6 => { crate::simd_ops::wu32(rdi.wrapping_add(P0), ((rbx as u32)) as u32); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            7 => { if (((fl_l as u64)) == ((fl_r as u64))) { bb = 9; continue; } bb = 8; continue; }
            8 => { crate::simd_ops::wu32(rcx.wrapping_add(P0), ((rbx as u32)) as u32); bb = 9; continue; }
            9 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_66331<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { fl_l = ((r8 as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 30; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((r9 as u32)) as u32); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rdi); r15 = rdx; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r12); rbp = rcx; r12 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rbx); rdi = ((r12 as u32)) as u32 as u64; r14 = ((((r8 as u32)) as i32 as i64) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rsi); bb = 2; continue; }
            2 => { rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x338u64))) as u64; rdx = rsp.wrapping_add(0x68); rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x80u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru8(rsp.wrapping_add(0x68u64))) as u8 as u64; fl_r = ((r12 as u8)) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 5; continue; } bb = 3; continue; }
            3 => { rbx = (crate::simd_ops::ru64(r15.wrapping_add(rdi.wrapping_mul(8)))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 28; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = rbx; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(r15.wrapping_add(rdi.wrapping_mul(8)), r12); bb = 28; continue; }
            5 => { rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x338u64))) as u64; rdx = rsp.wrapping_add(0x60); rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x40u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rdx = (crate::simd_ops::ru64(r15.wrapping_add(rdi.wrapping_mul(8)))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 16; continue; } bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru64(P1)) as u64; rcx = P1; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x28u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = rax; rdx = (crate::simd_ops::ru64(rax)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rdx.wrapping_add(0x8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rsp.wrapping_add(0x60u64))) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 16; continue; } bb = 7; continue; }
            7 => { rbx = (crate::simd_ops::ru64(r15.wrapping_add(rdi.wrapping_mul(8)))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 15; continue; } bb = 8; continue; }
            8 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = rbx.wrapping_sub(0x1); rax = rbx; rcx = (((rcx) & (0xfffffffffe000000u64))) as u64; rax = ((rax).wrapping_sub(rcx)) as u64; rax = ((rax).wrapping_shr(16u32)) as u64; rdx = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rdx = ((rdx).wrapping_shl(4u32)) as u64; rdx = ((rdx).wrapping_add(0x108u64)) as u64; rdx = ((rdx).wrapping_add(rcx)) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x4u64))) as u32 as u64; rdx = ((rdx).wrapping_sub(rax)) as u64; rax = ((crate::exe::teb_self() as u64)) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(rcx.wrapping_add(0x100u64))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 13; continue; } bb = 9; continue; }
            9 => { fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(0xeu64))) as u8 as u64; fl_r = ((r12 as u8)) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 12; continue; } bb = 10; continue; }
            10 => { rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(rbx, rax); fl_l = (crate::simd_ops::lk_add_u32(rdx.wrapping_add(0x18u64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; crate::simd_ops::wu64(rdx.wrapping_add(0x20u64), rbx); if (((fl_l as u32)) != 0) { bb = 15; continue; } bb = 11; continue; }
            11 => { rcx = rdx; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 15; continue; }
            12 => { r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 14; continue; }
            13 => { r8 = 0; fl_l = 0; fl_r = 0; bb = 14; continue; }
            14 => { r9 = rbx; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 15; continue; }
            15 => { crate::simd_ops::wu64(r15.wrapping_add(rdi.wrapping_mul(8)), r12); bb = 16; continue; }
            16 => { rbx = (crate::simd_ops::ru64(r15.wrapping_add(rdi.wrapping_mul(8)))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 27; continue; } bb = 17; continue; }
            17 => { rcx = (crate::simd_ops::ru32(rsp.wrapping_add(0x60u64))) as u32 as u64; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rcx = rax; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru32(rsp.wrapping_add(0x60u64))) as u32 as u64; rbx = rax; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); r8 = P1; rdx = rax; rcx = rbx; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(r15.wrapping_add(rdi.wrapping_mul(8)))) as u64; rsi = rax; fl_l = rbx; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 27; continue; } bb = 18; continue; }
            18 => { fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 26; continue; } bb = 19; continue; }
            19 => { r8 = (crate::simd_ops::ru64(rbx)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(r8)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = rbx.wrapping_sub(0x1); rax = rbx; rcx = (((rcx) & (0xfffffffffe000000u64))) as u64; rax = ((rax).wrapping_sub(rcx)) as u64; rax = ((rax).wrapping_shr(16u32)) as u64; rdx = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rdx = ((rdx).wrapping_shl(4u32)) as u64; rdx = ((rdx).wrapping_add(0x108u64)) as u64; rdx = ((rdx).wrapping_add(rcx)) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x4u64))) as u32 as u64; rdx = ((rdx).wrapping_sub(rax)) as u64; rax = ((crate::exe::teb_self() as u64)) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(rcx.wrapping_add(0x100u64))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 24; continue; } bb = 20; continue; }
            20 => { fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(0xeu64))) as u8 as u64; fl_r = ((r12 as u8)) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 23; continue; } bb = 21; continue; }
            21 => { rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(rbx, rax); fl_l = (crate::simd_ops::lk_add_u32(rdx.wrapping_add(0x18u64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; crate::simd_ops::wu64(rdx.wrapping_add(0x20u64), rbx); if (((fl_l as u32)) != 0) { bb = 26; continue; } bb = 22; continue; }
            22 => { rcx = rdx; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 26; continue; }
            23 => { r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 25; continue; }
            24 => { r8 = 0; fl_l = 0; fl_r = 0; bb = 25; continue; }
            25 => { r9 = rbx; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 26; continue; }
            26 => { crate::simd_ops::wu64(r15.wrapping_add(rdi.wrapping_mul(8)), rsi); rbx = rsi; bb = 27; continue; }
            27 => { rax = (crate::simd_ops::ru64(rbx)) as u64; r8 = 0; fl_l = 0; fl_r = 0; rdx = rbp; rcx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x70u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 28; continue; }
            28 => { rdi = ((rdi).wrapping_add(1)) as u64; fl_l = rdi; fl_r = r14; if ((((fl_l as u64)) as i64) < (((fl_r as u64)) as i64)) { bb = 2; continue; } bb = 29; continue; }
            29 => { r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); bb = 30; continue; }
            30 => { return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_66380<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdx = (((r9) as i64).wrapping_mul((0x128u64) as i64) as u64) as u64; rcx = 0x11; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rdi); fl_l = rdx; fl_r = rdx; rdi = r9; rax = ((rdx.wrapping_add(P0)) as u64) as u64; rdx = 0x10; rcx = (if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { rax } else { rcx }) as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); rcx = rax; rdx = 0xdeadd00d; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); rax = 0; fl_l = 0; fl_r = 0; bb = 2; continue; }
            2 => { crate::simd_ops::wu64(rax, (0x128u64) as u64); rsi = ((rax.wrapping_add(P0)) as u64) as u64; crate::simd_ops::wu32(rax.wrapping_add(P4), ((rdi as u32)) as u32); crate::simd_ops::wu32(rax.wrapping_add(P5), (0x8u32) as u32); fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 3; continue; }
            3 => { crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbx); rbx = rsi; bb = 4; continue; }
            4 => { rdx = 0; fl_l = 0; fl_r = 0; r8 = 0x128; rcx = rbx; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); rcx = rbx; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); rbx = ((rbx).wrapping_add(0x128u64)) as u64; fl_l = rdi; fl_r = (0x1u64) as u64; rdi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 4; continue; } bb = 5; continue; }
            5 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; bb = 6; continue; }
            6 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rax = rsi; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_66397<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rdx = ((r9.wrapping_add(r9.wrapping_mul(4))) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rsi); rdx = ((rdx).wrapping_shl(5u32)) as u64; rcx = 0x11; fl_l = rdx; fl_r = rdx; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), r14); rsi = r9; rax = rdx.wrapping_add(0x10); rdx = 0x10; rcx = (if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { rax } else { rcx }) as u64; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); r14 = 0; fl_l = 0; fl_r = 0; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp); rcx = rax; rdx = 0xdeadd00d; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); rax = ((r14 as u32)) as u32 as u64; bb = 2; continue; }
            2 => { crate::simd_ops::wu64(rax, (0xa0u64) as u64); rbp = rax.wrapping_add(0x10); crate::simd_ops::wu32(rax.wrapping_add(0x8u64), ((rsi as u32)) as u32); crate::simd_ops::wu32(rax.wrapping_add(0xcu64), (0x8u32) as u32); fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 3; continue; }
            3 => { crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rbx); rbx = rbp.wrapping_add(0x78); crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rdi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r15); r15 = P3; bb = 4; continue; }
            4 => { rdx = 0; fl_l = 0; fl_r = 0; rcx = rbx.wrapping_sub(0x78); r8 = 0xa0; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx.wrapping_sub(0x78u64), r15); rcx = rbx.wrapping_add(0x8); rax = (crate::simd_ops::ru32(rbx.wrapping_sub(0x50u64))) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_sub(0x4cu64), ((r14 as u32)) as u32); rax = ((((rax as u32)) & (0xff000008u32))) as u32 as u64; crate::simd_ops::wu8(rbx.wrapping_sub(0x48u64), ((r14 as u8)) as u8); rbx = rbx.wrapping_add(0xa0); rax = ((((rax as u32)) | (0x8u32))) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_sub(0xf0u64), ((rax as u32)) as u32); crate::simd_ops::wu8(rbx.wrapping_sub(0xedu64), (0x8u8) as u8); crate::simd_ops::wu64(rbx.wrapping_sub(0x98u64), r14); crate::simd_ops::wu64(rbx.wrapping_sub(0x90u64), r14); crate::simd_ops::wu64(rbx.wrapping_sub(0x88u64), r14); crate::simd_ops::wu64(rbx.wrapping_sub(0x80u64), r14); crate::simd_ops::wu64(rbx.wrapping_sub(0xa8u64), rcx); crate::simd_ops::wu32(rbx.wrapping_sub(0xa0u64), ((r14 as u32)) as u32); crate::simd_ops::wu32(rbx.wrapping_sub(0x9cu64), (0x4u32) as u32); fl_l = rsi; fl_r = (0x1u64) as u64; rsi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 4; continue; } bb = 5; continue; }
            5 => { r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; bb = 6; continue; }
            6 => { r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rax = rbp; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_66517<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rdx = ((r9.wrapping_add(r9.wrapping_mul(4))) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rsi); rdx = ((rdx).wrapping_shl(5u32)) as u64; rcx = 0x11; fl_l = rdx; fl_r = rdx; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), r14); rsi = r9; rax = rdx.wrapping_add(0x10); rdx = 0x10; rcx = (if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { rax } else { rcx }) as u64; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); r14 = 0; fl_l = 0; fl_r = 0; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp); rcx = rax; rdx = 0xdeadd00d; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); rax = ((r14 as u32)) as u32 as u64; bb = 2; continue; }
            2 => { crate::simd_ops::wu64(rax, (0xa0u64) as u64); rbp = rax.wrapping_add(0x10); crate::simd_ops::wu32(rax.wrapping_add(0x8u64), ((rsi as u32)) as u32); crate::simd_ops::wu32(rax.wrapping_add(0xcu64), (0x8u32) as u32); fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 3; continue; }
            3 => { crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rbx); rbx = rbp.wrapping_add(0x68); crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rdi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r15); r15 = P3; bb = 4; continue; }
            4 => { rdx = 0; fl_l = 0; fl_r = 0; rcx = rbx.wrapping_sub(0x68); r8 = 0xa0; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru32(rbx.wrapping_sub(0x50u64))) as u32 as u64; rcx = rbx.wrapping_add(0x8); crate::simd_ops::wu32(rbx.wrapping_sub(0x4cu64), ((r14 as u32)) as u32); rax = ((((rax as u32)) & (0xff000008u32))) as u32 as u64; crate::simd_ops::wu8(rbx.wrapping_sub(0x48u64), ((r14 as u8)) as u8); rbx = rbx.wrapping_add(0xa0); rax = ((((rax as u32)) | (0x8u32))) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_sub(0xf0u64), ((rax as u32)) as u32); crate::simd_ops::wu8(rbx.wrapping_sub(0xedu64), (0x8u8) as u8); crate::simd_ops::wu64(rbx.wrapping_sub(0x98u64), r14); crate::simd_ops::wu64(rbx.wrapping_sub(0x90u64), r14); crate::simd_ops::wu64(rbx.wrapping_sub(0x88u64), r14); crate::simd_ops::wu64(rbx.wrapping_sub(0x80u64), r14); crate::simd_ops::wu64(rbx.wrapping_sub(0xa8u64), rcx); crate::simd_ops::wu32(rbx.wrapping_sub(0xa0u64), ((r14 as u32)) as u32); crate::simd_ops::wu32(rbx.wrapping_sub(0x9cu64), (0x4u32) as u32); crate::simd_ops::wu64(rbx.wrapping_sub(0x108u64), r15); fl_l = rsi; fl_r = (0x1u64) as u64; rsi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 4; continue; } bb = 5; continue; }
            5 => { r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; bb = 6; continue; }
            6 => { r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rax = rbp; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_66544<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x40u64)) as u64;
    rbx = rdx;
    r11 = rcx;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    r10 = 0;
    fl_l = 0; fl_r = 0;
    rdi = P1;
    rax = P2;
    rcx = rbx;
    crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rax);
    rdx = r11.wrapping_add(0x10);
    crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), r10);
    r9 = P3;
    r8 = ((r10.wrapping_add(P4)) as u32) as u32 as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rdi);
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi);
    rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp);
    rax = P6;
    crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rax);
    rdx = r11.wrapping_add(0x20);
    crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), r10);
    r9 = P7;
    crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rdi);
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi);
    rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp);
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64;
    rax = 0;
    fl_l = 0; fl_r = 0;
    rsp = ((rsp).wrapping_add(0x40u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_66547<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64, const P9: u64, const P10: u64, const P11: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rdx = (((r9) as i64).wrapping_mul((0xb8u64) as i64) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rsi); rcx = 0x11; fl_l = rdx; fl_r = rdx; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), r14); rsi = r9; rax = ((rdx.wrapping_add(P0)) as u64) as u64; rdx = 0x10; rcx = (if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { rax } else { rcx }) as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); r14 = 0; fl_l = 0; fl_r = 0; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); rcx = rax; rdx = 0xdeadd00d; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); rax = ((r14 as u32)) as u32 as u64; bb = 2; continue; }
            2 => { crate::simd_ops::wu64(rax, (0xb8u64) as u64); rbp = ((rax.wrapping_add(P0)) as u64) as u64; crate::simd_ops::wu32(rax.wrapping_add(P4), ((rsi as u32)) as u32); crate::simd_ops::wu32(rax.wrapping_add(P5), (0x8u32) as u32); fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 3; continue; }
            3 => { crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rbx); rbx = rbp.wrapping_add(0x90); crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rdi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r15); r15 = P6; bb = 4; continue; }
            4 => { rdx = 0; fl_l = 0; fl_r = 0; rcx = ((rbx.wrapping_add(P7)) as u64) as u64; r8 = 0xb8; rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx.wrapping_add(P7), r15); rcx = ((rbx.wrapping_add(P4)) as u64) as u64; rax = (crate::simd_ops::ru32(rbx.wrapping_add(P9))) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(P10), ((r14 as u32)) as u32); rax = ((((rax as u32)) & (0xff000008u32))) as u32 as u64; crate::simd_ops::wu8(rbx.wrapping_add(P11), ((r14 as u8)) as u8); rbx = rbx.wrapping_add(0xb8); rax = ((((rax as u32)) | (0x8u32))) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_sub(0x108u64), ((rax as u32)) as u32); crate::simd_ops::wu8(rbx.wrapping_sub(0x105u64), (0x8u8) as u8); crate::simd_ops::wu64(rbx.wrapping_sub(0xb0u64), r14); crate::simd_ops::wu64(rbx.wrapping_sub(0xa8u64), r14); crate::simd_ops::wu64(rbx.wrapping_sub(0xa0u64), r14); crate::simd_ops::wu64(rbx.wrapping_sub(0x98u64), r14); crate::simd_ops::wu64(rbx.wrapping_sub(0xc0u64), rcx); crate::simd_ops::wu32(rbx.wrapping_sub(0xb8u64), ((r14 as u32)) as u32); crate::simd_ops::wu32(rbx.wrapping_sub(0xb4u64), (0x4u32) as u32); fl_l = rsi; fl_r = (0x1u64) as u64; rsi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 4; continue; } bb = 5; continue; }
            5 => { r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; bb = 6; continue; }
            6 => { r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rax = rbp; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_66600<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64, const P9: u64, const P10: u64, const P11: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rdx = (((r9) as i64).wrapping_mul((0xa8u64) as i64) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rsi); rcx = 0x11; fl_l = rdx; fl_r = rdx; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), r14); rsi = r9; rax = ((rdx.wrapping_add(P0)) as u64) as u64; rdx = 0x10; rcx = (if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { rax } else { rcx }) as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); r14 = 0; fl_l = 0; fl_r = 0; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); rcx = rax; rdx = 0xdeadd00d; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); rax = ((r14 as u32)) as u32 as u64; bb = 2; continue; }
            2 => { crate::simd_ops::wu64(rax, (0xa8u64) as u64); rbp = ((rax.wrapping_add(P0)) as u64) as u64; crate::simd_ops::wu32(rax.wrapping_add(P4), ((rsi as u32)) as u32); crate::simd_ops::wu32(rax.wrapping_add(P5), (0x8u32) as u32); fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 3; continue; }
            3 => { crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rbx); rbx = rbp.wrapping_add(0x80); crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rdi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r15); r15 = P6; bb = 4; continue; }
            4 => { rdx = 0; fl_l = 0; fl_r = 0; rcx = ((rbx.wrapping_add(P7)) as u64) as u64; r8 = 0xa8; rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx.wrapping_add(P7), r15); rcx = ((rbx.wrapping_add(P4)) as u64) as u64; rax = (crate::simd_ops::ru32(rbx.wrapping_add(P9))) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(P10), ((r14 as u32)) as u32); rax = ((((rax as u32)) & (0xff000008u32))) as u32 as u64; crate::simd_ops::wu8(rbx.wrapping_add(P11), ((r14 as u8)) as u8); rbx = rbx.wrapping_add(0xa8); rax = ((((rax as u32)) | (0x8u32))) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_sub(0xf8u64), ((rax as u32)) as u32); crate::simd_ops::wu8(rbx.wrapping_sub(0xf5u64), (0x8u8) as u8); crate::simd_ops::wu64(rbx.wrapping_sub(0xa0u64), r14); crate::simd_ops::wu64(rbx.wrapping_sub(0x98u64), r14); crate::simd_ops::wu64(rbx.wrapping_sub(0x90u64), r14); crate::simd_ops::wu64(rbx.wrapping_sub(0x88u64), r14); crate::simd_ops::wu64(rbx.wrapping_sub(0xb0u64), rcx); crate::simd_ops::wu32(rbx.wrapping_sub(0xa8u64), ((r14 as u32)) as u32); crate::simd_ops::wu32(rbx.wrapping_sub(0xa4u64), (0x4u32) as u32); fl_l = rsi; fl_r = (0x1u64) as u64; rsi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 4; continue; } bb = 5; continue; }
            5 => { r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; bb = 6; continue; }
            6 => { r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rax = rbp; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_66744<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdx = ((r9.wrapping_add(r9.wrapping_mul(8))) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rdi); rdx = ((rdx).wrapping_shl(5u32)) as u64; rcx = 0x11; fl_l = rdx; fl_r = rdx; rdi = r9; rax = ((rdx.wrapping_add(P0)) as u64) as u64; rdx = 0x10; rcx = (if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { rax } else { rcx }) as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); rcx = rax; rdx = 0xdeadd00d; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); rax = 0; fl_l = 0; fl_r = 0; bb = 2; continue; }
            2 => { crate::simd_ops::wu64(rax, (0x120u64) as u64); rsi = ((rax.wrapping_add(P0)) as u64) as u64; crate::simd_ops::wu32(rax.wrapping_add(P4), ((rdi as u32)) as u32); crate::simd_ops::wu32(rax.wrapping_add(P5), (0x8u32) as u32); fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 3; continue; }
            3 => { crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbx); rbx = rsi; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rbp); rbp = P6; bb = 4; continue; }
            4 => { rdx = 0; fl_l = 0; fl_r = 0; r8 = 0x120; rcx = rbx; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); rcx = rbx; rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx, rbp); rbx = ((rbx).wrapping_add(0x120u64)) as u64; fl_l = rdi; fl_r = (0x1u64) as u64; rdi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 4; continue; } bb = 5; continue; }
            5 => { rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; bb = 6; continue; }
            6 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rax = rsi; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_66780<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64, const P9: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x40u64)) as u64;
    r9 = (crate::simd_ops::ru32(P0)) as u32 as u64;
    rdi = rdx;
    r8 = (crate::simd_ops::ru64(P1)) as u64;
    rdx = rcx;
    rbx = rcx;
    crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), (0x1u32) as u32);
    rcx = rdi;
    crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), (0x1u32) as u32);
    rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp);
    rdx = rdi;
    rcx = rbx;
    rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp);
    rdx = (crate::simd_ops::ru64(rdi.wrapping_add(P4))) as u64;
    rax = (crate::simd_ops::ru32(rbx.wrapping_add(P5))) as u32 as u64;
    rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P6))) as u64;
    crate::simd_ops::wu32(rcx, ((rax as u32)) as u32);
    crate::simd_ops::lk_add_u64(rdx.wrapping_add(P6), (0x4u64) as u64);
    rdx = ((rbx.wrapping_add(P7)) as u64) as u64;
    rcx = (crate::simd_ops::ru64(rdi.wrapping_add(P4))) as u64;
    rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp);
    rcx = rdi;
    rax = crate::simd_ops::call_stk(P9, rax, rcx, rdx, r8, r9, rsp);
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64;
    rax = 0;
    fl_l = 0; fl_r = 0;
    rsp = ((rsp).wrapping_add(0x40u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_66781<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdx = (((r9) as i64).wrapping_mul((0x118u64) as i64) as u64) as u64; rcx = 0x11; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rdi); fl_l = rdx; fl_r = rdx; rdi = r9; rax = ((rdx.wrapping_add(P0)) as u64) as u64; rdx = 0x10; rcx = (if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { rax } else { rcx }) as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); rcx = rax; rdx = 0xdeadd00d; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); rax = 0; fl_l = 0; fl_r = 0; bb = 2; continue; }
            2 => { crate::simd_ops::wu64(rax, (0x118u64) as u64); rsi = ((rax.wrapping_add(P0)) as u64) as u64; crate::simd_ops::wu32(rax.wrapping_add(P4), ((rdi as u32)) as u32); crate::simd_ops::wu32(rax.wrapping_add(P5), (0x8u32) as u32); fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 3; continue; }
            3 => { crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbx); rbx = rsi; bb = 4; continue; }
            4 => { rdx = 0; fl_l = 0; fl_r = 0; r8 = 0x118; rcx = rbx; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); rcx = rbx; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); rbx = ((rbx).wrapping_add(0x118u64)) as u64; fl_l = rdi; fl_r = (0x1u64) as u64; rdi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 4; continue; } bb = 5; continue; }
            5 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; bb = 6; continue; }
            6 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rax = rsi; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_66876<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64, const P9: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x40u64)) as u64;
    rdi = rdx;
    rbx = rcx;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rax = P1;
    rcx = rdi;
    crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rax);
    rdx = ((rbx.wrapping_add(P2)) as u64) as u64;
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), (0x2u32) as u32);
    crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax);
    r9 = P3;
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax);
    r8 = ((rax.wrapping_add(P4)) as u32) as u32 as u64;
    rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp);
    rax = P6;
    r8 = 0x2;
    crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rax);
    rdx = ((rbx.wrapping_add(P7)) as u64) as u64;
    rax = P8;
    crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), (0x2u32) as u32);
    crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax);
    r9 = P9;
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax);
    rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp);
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64;
    rax = 0;
    fl_l = 0; fl_r = 0;
    rsp = ((rsp).wrapping_add(0x40u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_66973<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64, const P9: u64, const P10: u64, const P11: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rdx = (((r9) as i64).wrapping_mul((0xa8u64) as i64) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rsi); rcx = 0x11; fl_l = rdx; fl_r = rdx; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), r14); rsi = r9; rax = ((rdx.wrapping_add(P0)) as u64) as u64; rdx = 0x10; rcx = (if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { rax } else { rcx }) as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); r14 = 0; fl_l = 0; fl_r = 0; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); rcx = rax; rdx = 0xdeadd00d; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); rax = ((r14 as u32)) as u32 as u64; bb = 2; continue; }
            2 => { crate::simd_ops::wu64(rax, (0xa8u64) as u64); rbp = ((rax.wrapping_add(P0)) as u64) as u64; crate::simd_ops::wu32(rax.wrapping_add(P4), ((rsi as u32)) as u32); crate::simd_ops::wu32(rax.wrapping_add(P5), (0x8u32) as u32); fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 3; continue; }
            3 => { crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rbx); rbx = rbp.wrapping_add(0x78); crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rdi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r15); r15 = P6; bb = 4; continue; }
            4 => { rdx = 0; fl_l = 0; fl_r = 0; rcx = ((rbx.wrapping_add(P7)) as u64) as u64; r8 = 0xa8; rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru32(rbx.wrapping_add(P9))) as u32 as u64; rcx = ((rbx.wrapping_add(P4)) as u64) as u64; crate::simd_ops::wu32(rbx.wrapping_add(P10), ((r14 as u32)) as u32); rax = ((((rax as u32)) & (0xff000008u32))) as u32 as u64; crate::simd_ops::wu8(rbx.wrapping_add(P11), ((r14 as u8)) as u8); rbx = rbx.wrapping_add(0xa8); rax = ((((rax as u32)) | (0x8u32))) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_sub(0xf8u64), ((rax as u32)) as u32); crate::simd_ops::wu8(rbx.wrapping_sub(0xf5u64), (0x8u8) as u8); crate::simd_ops::wu64(rbx.wrapping_sub(0xa0u64), r14); crate::simd_ops::wu64(rbx.wrapping_sub(0x98u64), r14); crate::simd_ops::wu64(rbx.wrapping_sub(0x90u64), r14); crate::simd_ops::wu64(rbx.wrapping_sub(0x88u64), r14); crate::simd_ops::wu64(rbx.wrapping_sub(0xb0u64), rcx); crate::simd_ops::wu32(rbx.wrapping_sub(0xa8u64), ((r14 as u32)) as u32); crate::simd_ops::wu32(rbx.wrapping_sub(0xa4u64), (0x4u32) as u32); crate::simd_ops::wu64(rbx.wrapping_sub(0x120u64), r15); fl_l = rsi; fl_r = (0x1u64) as u64; rsi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 4; continue; } bb = 5; continue; }
            5 => { r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; bb = 6; continue; }
            6 => { r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rax = rbp; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_66989<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x40u64)) as u64;
    rdi = rdx;
    rbx = rcx;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    r10 = 0;
    fl_l = 0; fl_r = 0;
    rax = P1;
    crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rax);
    rdx = ((rbx.wrapping_add(P2)) as u64) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), r10);
    r9 = P3;
    crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), r10);
    rcx = rdi;
    r8 = ((r10.wrapping_add(P4)) as u32) as u32 as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r10);
    rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp);
    rax = P6;
    crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rax);
    rdx = ((rbx.wrapping_add(P7)) as u64) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), r10);
    r9 = P8;
    crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), r10);
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r10);
    rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp);
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64;
    rax = 0;
    fl_l = 0; fl_r = 0;
    rsp = ((rsp).wrapping_add(0x40u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_66992<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64, const P9: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    crate::simd_ops::wu8(rsp.wrapping_add(0x18u64), ((r8 as u8)) as u8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x40u64)) as u64;
    r9 = (crate::simd_ops::ru32(P0)) as u32 as u64;
    rdi = rdx;
    r8 = (crate::simd_ops::ru64(P1)) as u64;
    rdx = rcx;
    rbx = rcx;
    crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), (0x1u32) as u32);
    rcx = rdi;
    crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), (0x3u32) as u32);
    rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp);
    rdx = rdi;
    rcx = rbx;
    rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp);
    rax = (crate::simd_ops::ru32(rbx.wrapping_add(P4))) as u32 as u64;
    rdx = rsp.wrapping_add(0x60);
    rcx = (crate::simd_ops::ru64(rdi.wrapping_add(P5))) as u64;
    crate::simd_ops::wu32(rsp.wrapping_add(0x60u64), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P6))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp);
    rcx = ((rbx.wrapping_add(P7)) as u64) as u64;
    rdx = rdi;
    rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp);
    rcx = rdi;
    rax = crate::simd_ops::call_stk(P9, rax, rcx, rdx, r8, r9, rsp);
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64;
    rax = 0;
    fl_l = 0; fl_r = 0;
    rsp = ((rsp).wrapping_add(0x40u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_66993<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64, const P9: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x40u64)) as u64;
    r9 = (crate::simd_ops::ru32(P0)) as u32 as u64;
    rdi = rdx;
    r8 = (crate::simd_ops::ru64(P1)) as u64;
    rdx = rcx;
    rbx = rcx;
    crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), (0x1u32) as u32);
    rcx = rdi;
    crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), (0x3u32) as u32);
    rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp);
    rdx = rdi;
    rcx = rbx;
    rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp);
    r8 = (crate::simd_ops::ru64(rdi.wrapping_add(P4))) as u64;
    rdx = rdi;
    rax = (crate::simd_ops::ru32(rbx.wrapping_add(P5))) as u32 as u64;
    rcx = (crate::simd_ops::ru64(r8.wrapping_add(P6))) as u64;
    crate::simd_ops::wu32(rcx, ((rax as u32)) as u32);
    rcx = ((rbx.wrapping_add(P7)) as u64) as u64;
    crate::simd_ops::lk_add_u64(r8.wrapping_add(P6), (0x4u64) as u64);
    rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp);
    rcx = rdi;
    rax = crate::simd_ops::call_stk(P9, rax, rcx, rdx, r8, r9, rsp);
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64;
    rax = 0;
    fl_l = 0; fl_r = 0;
    rsp = ((rsp).wrapping_add(0x40u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_67013<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x40u64)) as u64;
    r9 = (crate::simd_ops::ru32(P0)) as u32 as u64;
    rdi = rdx;
    r8 = (crate::simd_ops::ru64(P1)) as u64;
    rdx = rcx;
    rbx = rcx;
    crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), (0x1u32) as u32);
    rcx = rdi;
    crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), (0x1u32) as u32);
    rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp);
    rdx = rdi;
    rcx = rbx;
    rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp);
    rdx = (crate::simd_ops::ru64(rdi.wrapping_add(P4))) as u64;
    rax = (crate::simd_ops::ru32(rbx.wrapping_add(P5))) as u32 as u64;
    rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P6))) as u64;
    crate::simd_ops::wu32(rcx, ((rax as u32)) as u32);
    crate::simd_ops::lk_add_u64(rdx.wrapping_add(P6), (0x4u64) as u64);
    rdx = (crate::simd_ops::ru64(rdi.wrapping_add(P4))) as u64;
    rax = (crate::simd_ops::ru32(rbx.wrapping_add(P7))) as u32 as u64;
    rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P6))) as u64;
    crate::simd_ops::wu32(rcx, ((rax as u32)) as u32);
    rcx = rdi;
    crate::simd_ops::lk_add_u64(rdx.wrapping_add(P6), (0x4u64) as u64);
    rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp);
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64;
    rax = 0;
    fl_l = 0; fl_r = 0;
    rsp = ((rsp).wrapping_add(0x40u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_67091<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdx = (((r9) as i64).wrapping_mul((0x138u64) as i64) as u64) as u64; rcx = 0x11; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rdi); fl_l = rdx; fl_r = rdx; rdi = r9; rax = ((rdx.wrapping_add(P0)) as u64) as u64; rdx = 0x10; rcx = (if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { rax } else { rcx }) as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); rcx = rax; rdx = 0xdeadd00d; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); rax = 0; fl_l = 0; fl_r = 0; bb = 2; continue; }
            2 => { crate::simd_ops::wu64(rax, (0x138u64) as u64); rsi = ((rax.wrapping_add(P0)) as u64) as u64; crate::simd_ops::wu32(rax.wrapping_add(P4), ((rdi as u32)) as u32); crate::simd_ops::wu32(rax.wrapping_add(P5), (0x8u32) as u32); fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 3; continue; }
            3 => { crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbx); rbx = rsi; bb = 4; continue; }
            4 => { rdx = 0; fl_l = 0; fl_r = 0; r8 = 0x138; rcx = rbx; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); rcx = rbx; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); rbx = ((rbx).wrapping_add(0x138u64)) as u64; fl_l = rdi; fl_r = (0x1u64) as u64; rdi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 4; continue; } bb = 5; continue; }
            5 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; bb = 6; continue; }
            6 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rax = rsi; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_67183<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64, const P9: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x40u64)) as u64;
    rdi = rdx;
    rbx = rcx;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rax = P1;
    rcx = rdi;
    crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rax);
    rdx = ((rbx.wrapping_add(P2)) as u64) as u64;
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), (0x1u32) as u32);
    crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax);
    r9 = P3;
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax);
    r8 = ((rax.wrapping_add(P4)) as u32) as u32 as u64;
    rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp);
    rax = P6;
    r8 = 0x2;
    crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rax);
    rdx = ((rbx.wrapping_add(P7)) as u64) as u64;
    rax = P8;
    crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), (0x2u32) as u32);
    crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax);
    r9 = P9;
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax);
    rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp);
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64;
    rax = 0;
    fl_l = 0; fl_r = 0;
    rsp = ((rsp).wrapping_add(0x40u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_67234<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64, const P9: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x40u64)) as u64;
    rdi = rdx;
    rbx = rcx;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rax = P1;
    rcx = rdi;
    crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rax);
    rdx = ((rbx.wrapping_add(P2)) as u64) as u64;
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), (0x1u32) as u32);
    crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax);
    r9 = P3;
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax);
    r8 = ((rax.wrapping_add(P4)) as u32) as u32 as u64;
    rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp);
    rax = P6;
    crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rax);
    rdx = ((rbx.wrapping_add(P7)) as u64) as u64;
    rax = P8;
    crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), (0x2u32) as u32);
    crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax);
    r9 = P9;
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax);
    rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp);
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64;
    rax = 0;
    fl_l = 0; fl_r = 0;
    rsp = ((rsp).wrapping_add(0x40u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_67268<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x40u64)) as u64;
    r9 = (crate::simd_ops::ru32(P0)) as u32 as u64;
    rdi = rdx;
    r8 = (crate::simd_ops::ru64(P1)) as u64;
    rdx = rcx;
    rbx = rcx;
    crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), (0x1u32) as u32);
    rcx = rdi;
    crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), (0x2u32) as u32);
    rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp);
    rdx = rdi;
    rcx = rbx;
    rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp);
    rdx = ((rbx.wrapping_add(P4)) as u64) as u64;
    rcx = rdi;
    rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp);
    rdx = ((rbx.wrapping_add(P6)) as u64) as u64;
    r8 = 0x2;
    rcx = rdi;
    rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp);
    rcx = rdi;
    rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp);
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64;
    rax = 0;
    fl_l = 0; fl_r = 0;
    rsp = ((rsp).wrapping_add(0x40u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_67527<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x40u64)) as u64;
    xmm0 = ((crate::simd_ops::ru64(P0)) as u64) as u128;
    rdi = rdx;
    rsi = P1;
    crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), ((xmm0 as u64)) as u64);
    rbx = rcx;
    crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rsi);
    rdx = ((rcx.wrapping_add(P2)) as u64) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi);
    r9 = P3;
    r8 = 0;
    fl_l = 0; fl_r = 0;
    rcx = rdi;
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    xmm0 = ((crate::simd_ops::ru64(P5)) as u64) as u128;
    crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), ((xmm0 as u64)) as u64);
    crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rsi);
    rdx = ((rbx.wrapping_add(P6)) as u64) as u64;
    r9 = P7;
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi);
    r8 = 0;
    fl_l = 0; fl_r = 0;
    rcx = rdi;
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64;
    rax = 0;
    fl_l = 0; fl_r = 0;
    rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64;
    rsp = ((rsp).wrapping_add(0x40u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_67546<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x40u64)) as u64;
    r9 = (crate::simd_ops::ru32(P0)) as u32 as u64;
    rdi = rdx;
    r8 = (crate::simd_ops::ru64(P1)) as u64;
    rdx = rcx;
    rbx = rcx;
    crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), (0x1u32) as u32);
    rcx = rdi;
    crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), (0x1u32) as u32);
    rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp);
    rdx = rdi;
    rcx = rbx;
    rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp);
    rdx = ((rbx.wrapping_add(P4)) as u64) as u64;
    r8 = 0x1;
    rcx = rdi;
    rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp);
    rdx = ((rbx.wrapping_add(P6)) as u64) as u64;
    r8 = 0x1;
    rcx = rdi;
    rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp);
    rcx = rdi;
    rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp);
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64;
    rax = 0;
    fl_l = 0; fl_r = 0;
    rsp = ((rsp).wrapping_add(0x40u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_67717<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rdx; rsi = rcx; rax = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; rcx = (((rax) as i64).wrapping_mul((0x78u64) as i64) as u64) as u64; rax = (0x8888888888888889u64) as u64; let prod = (rax as i64 as i128).wrapping_mul((rcx) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; rdi = ((rcx.wrapping_add(rdx)) as u64) as u64; rdi = ((((rdi) as i64).wrapping_shr(6u32) as u64)) as u64; rax = rdi; rax = ((rax).wrapping_shr(63u32)) as u64; rdi = ((rdi).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru32(rsi.wrapping_add(P1))) as u32 as u64; fl_l = rbx; fl_r = rax; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rdx = ((rax as u32)) as u32 as u64; rdx = ((rdx).wrapping_shr(1u32)) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; fl_l = rdx; fl_r = rbx; rdx = (if (((fl_l as u64)) < ((fl_r as u64))) { rbx } else { rdx }) as u64; rcx = rsi; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rdx = rbx; rdx = ((rdx).wrapping_sub(rdi)) as u64; rax = (crate::simd_ops::ru32(rsi.wrapping_add(P0))) as u32 as u64; rcx = (((rax) as i64).wrapping_mul((0x78u64) as i64) as u64) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rsi))) as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu32(rsi.wrapping_add(P0), ((rbx as u32)) as u32); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            2 => { fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rdx = rbx; rdx = ((rdx).wrapping_sub(rdi)) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rsi))) as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu32(rsi.wrapping_add(P0), ((rbx as u32)) as u32); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            4 => { if (((fl_l as u64)) == ((fl_r as u64))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru64(rsi)) as u64; r8 = ((rax.wrapping_add(rcx)) as u64) as u64; rdx = (((rbx) as i64).wrapping_mul((0x78u64) as i64) as u64) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu32(rsi.wrapping_add(P0), ((rbx as u32)) as u32); bb = 6; continue; }
            6 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_67834<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    crate::simd_ops::wu8(rsp.wrapping_add(0x18u64), ((r8 as u8)) as u8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x40u64)) as u64;
    r9 = (crate::simd_ops::ru32(P0)) as u32 as u64;
    rdi = rdx;
    r8 = (crate::simd_ops::ru64(P1)) as u64;
    rax = 0;
    fl_l = 0; fl_r = 0;
    rdx = rcx;
    crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((rax as u32)) as u32);
    rbx = rcx;
    crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rax as u32)) as u32);
    rcx = rdi;
    rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp);
    rdx = rdi;
    rcx = rbx;
    rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp);
    rax = (crate::simd_ops::ru32(rbx.wrapping_add(P4))) as u32 as u64;
    rdx = rsp.wrapping_add(0x60);
    rcx = (crate::simd_ops::ru64(rdi.wrapping_add(P5))) as u64;
    crate::simd_ops::wu32(rsp.wrapping_add(0x60u64), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P6))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp);
    rcx = rdi;
    rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp);
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64;
    rax = 0;
    fl_l = 0; fl_r = 0;
    rsp = ((rsp).wrapping_add(0x40u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_67896<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64, const P9: u64, const P10: u64, const P11: u64, const P12: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x50u64)) as u64; r9 = (crate::simd_ops::ru32(P0)) as u32 as u64; rbx = rdx; r8 = (crate::simd_ops::ru64(P1)) as u64; rbp = 0; fl_l = 0; fl_r = 0; rdx = rcx; crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), rsi); rsi = rcx; crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((rbp as u32)) as u32); rcx = rbx; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rbp as u32)) as u32); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rdx = rbx; rcx = rsi; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rcx = ((rsi.wrapping_add(P4)) as u64) as u64; r14 = P5; fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(P6))) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 2; continue; } bb = 1; continue; }
            1 => { rdx = rcx; rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P7))) as u64; rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); rdi = ((rbx.wrapping_add(P9)) as u64) as u64; bb = 4; continue; }
            2 => { rdi = ((rbx.wrapping_add(P9)) as u64) as u64; fl_l = (crate::simd_ops::ru8(rdi)) as u8 as u64; fl_r = ((rbp as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 4; continue; } bb = 3; continue; }
            3 => { r9 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), r14); r8 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rbp); rdx = rsp.wrapping_add(0x40); rax = crate::simd_ops::call_stk(P10, rax, rcx, rdx, r8, r9, rsp); bb = 4; continue; }
            4 => { rcx = ((rsi.wrapping_add(P11)) as u64) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x328u64))) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 6; continue; } bb = 5; continue; }
            5 => { rdx = rcx; rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x338u64))) as u64; rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); bb = 8; continue; }
            6 => { fl_l = (crate::simd_ops::ru8(rdi)) as u8 as u64; fl_r = ((rbp as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 8; continue; } bb = 7; continue; }
            7 => { r9 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), r14); r8 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rbp); rdx = rsp.wrapping_add(0x40); rax = crate::simd_ops::call_stk(P10, rax, rcx, rdx, r8, r9, rsp); bb = 8; continue; }
            8 => { rcx = rbx; rax = crate::simd_ops::call_stk(P12, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rax = 0; fl_l = 0; fl_r = 0; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; rsp = ((rsp).wrapping_add(0x50u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_67902<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x40u64)) as u64;
    r9 = (crate::simd_ops::ru32(P0)) as u32 as u64;
    rdi = rdx;
    r8 = (crate::simd_ops::ru64(P1)) as u64;
    rax = 0;
    fl_l = 0; fl_r = 0;
    rdx = rcx;
    crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((rax as u32)) as u32);
    rbx = rcx;
    crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rax as u32)) as u32);
    rcx = rdi;
    rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp);
    rdx = rdi;
    rcx = rbx;
    rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp);
    rcx = (crate::simd_ops::ru64(rdi.wrapping_add(P4))) as u64;
    rdx = ((rbx.wrapping_add(P5)) as u64) as u64;
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P6))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp);
    rcx = (crate::simd_ops::ru64(rdi.wrapping_add(P4))) as u64;
    rdx = ((rbx.wrapping_add(P7)) as u64) as u64;
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P6))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp);
    rcx = rdi;
    rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp);
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64;
    rax = 0;
    fl_l = 0; fl_r = 0;
    rsp = ((rsp).wrapping_add(0x40u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_67903<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x40u64)) as u64;
    r9 = (crate::simd_ops::ru32(P0)) as u32 as u64;
    rdi = rdx;
    r8 = (crate::simd_ops::ru64(P1)) as u64;
    rax = 0;
    fl_l = 0; fl_r = 0;
    rdx = rcx;
    crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((rax as u32)) as u32);
    rbx = rcx;
    crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rax as u32)) as u32);
    rcx = rdi;
    rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp);
    rdx = rdi;
    rcx = rbx;
    rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp);
    rcx = (crate::simd_ops::ru64(rdi.wrapping_add(P4))) as u64;
    rdx = ((rbx.wrapping_add(P5)) as u64) as u64;
    rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp);
    rcx = (crate::simd_ops::ru64(rdi.wrapping_add(P4))) as u64;
    rdx = ((rbx.wrapping_add(P7)) as u64) as u64;
    rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp);
    rcx = rdi;
    rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp);
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64;
    rax = 0;
    fl_l = 0; fl_r = 0;
    rsp = ((rsp).wrapping_add(0x40u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_68175<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64, const P9: u64, const P10: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu8(rsp.wrapping_add(0x18u64), ((r8 as u8)) as u8); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; r8 = (crate::simd_ops::ru64(P0)) as u64; rsi = rdx; rdx = rcx; crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), (0x1u32) as u32); rbx = rcx; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), (0x1u32) as u32); rcx = rsi; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rsi.wrapping_add(P2))) as u64; rdx = rbx; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P3))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru32(rbx.wrapping_add(P4))) as u32 as u64; rdx = rsp.wrapping_add(0x60); rcx = (crate::simd_ops::ru64(rsi.wrapping_add(P2))) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x60u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P5))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru32(rsp.wrapping_add(0x60u64))) as u32 as u64; fl_r = 0x0; if ((((fl_l as u32)) as i32) <= (((fl_r as u32)) as i32)) { bb = 5; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(P4))) as u32 as u64; rbx = (crate::simd_ops::ru64(rbx.wrapping_add(P6))) as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 5; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rdi); rdi = ((rax as u32)) as u32 as u64; bb = 3; continue; }
            3 => { rdx = rsi; rcx = rbx; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); rbx = ((rbx).wrapping_add(0x8u64)) as u64; fl_l = rdi; fl_r = (0x1u64) as u64; rdi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } bb = 4; continue; }
            4 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; bb = 5; continue; }
            5 => { rdx = (crate::simd_ops::ru32(rsi.wrapping_add(P6))) as u32 as u64; rax = (crate::simd_ops::ru64(rsi)) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rcx = ((rdx.wrapping_add(rdx.wrapping_mul(2))) as u64) as u64; rcx = ((rcx).wrapping_add(rcx)) as u64; rcx = (crate::simd_ops::ru32(rax.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(P8))) as u32 as u64; rax = ((rdx.wrapping_add(P9)) as u32) as u32 as u64; crate::simd_ops::wu32(rsi.wrapping_add(P6), ((rax as u32)) as u32); rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rsi.wrapping_add(P10), ((rcx as u32)) as u32); rsp = ((rsp).wrapping_add(0x40u64)) as u64; rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_68314<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64, const P9: u64, const P10: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; r9 = (crate::simd_ops::ru32(P0)) as u32 as u64; rsi = rdx; r8 = (crate::simd_ops::ru64(P1)) as u64; rdx = rcx; rbp = rcx; crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), (0x1u32) as u32); rcx = rsi; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), (0x1u32) as u32); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); r9 = (crate::simd_ops::ru32(P3)) as u32 as u64; rdx = rbp; r8 = (crate::simd_ops::ru64(P4)) as u64; rcx = rsi; crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), (0x1u32) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), (0x1u32) as u32); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rsi.wrapping_add(P5))) as u64; rdx = rbp.wrapping_add(0x8); rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); rcx = rsi; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rsi.wrapping_add(P5))) as u64; rdx = (crate::simd_ops::ru32(rbp.wrapping_add(0x58u64))) as u32 as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(P8))) as u64; crate::simd_ops::wu32(rax, ((rdx as u32)) as u32); crate::simd_ops::lk_add_u64(rcx.wrapping_add(P8), (0x4u64) as u64); fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rdx = (crate::simd_ops::ru64(rbp.wrapping_add(0x50u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rbx); rbx = (((crate::simd_ops::ru32(rbp.wrapping_add(0x58u64))) as i32 as i64) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rdi); rdi = (crate::simd_ops::ru64(rsi.wrapping_add(P9))) as u64; rbx = ((rbx).wrapping_shl(2u32)) as u64; r8 = rbx; rcx = (crate::simd_ops::ru64(rdi.wrapping_add(P8))) as u64; rax = crate::simd_ops::call_stk(P10, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::lk_add_u64(rdi.wrapping_add(P8), (rbx) as u64)) as u64; fl_r = fl_l; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; bb = 2; continue; }
            2 => { rcx = rsi; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rax = 0; fl_l = 0; fl_r = 0; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_68339<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64, const P9: u64, const P10: u64, const P11: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu8(rsp.wrapping_add(0x18u64), ((r8 as u8)) as u8); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; r8 = (crate::simd_ops::ru64(P0)) as u64; rsi = rdx; rdx = rcx; crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), (0x1u32) as u32); rbx = rcx; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), (0x1u32) as u32); rcx = rsi; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rdx = rsi; rcx = rbx; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rsi.wrapping_add(P3))) as u64; rdx = rsp.wrapping_add(0x60); crate::simd_ops::wu32(rsp.wrapping_add(0x60u64), (0x0u32) as u32); rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P4))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rdx = (((crate::simd_ops::ru32(rsp.wrapping_add(0x60u64))) as i32 as i64) as u64) as u64; rcx = ((rbx.wrapping_add(P5)) as u64) as u64; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x60u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 4; continue; } bb = 1; continue; }
            1 => { rbx = (crate::simd_ops::ru64(rbx.wrapping_add(P5))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rdi); rdi = ((rax as u32)) as u32 as u64; bb = 2; continue; }
            2 => { rdx = rsi; rcx = rbx; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); rbx = ((rbx).wrapping_add(0x8u64)) as u64; fl_l = rdi; fl_r = (0x1u64) as u64; rdi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 3; continue; }
            3 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; bb = 4; continue; }
            4 => { rdx = (crate::simd_ops::ru32(rsi.wrapping_add(P8))) as u32 as u64; rax = (crate::simd_ops::ru64(rsi)) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rcx = ((rdx.wrapping_add(rdx.wrapping_mul(2))) as u64) as u64; rcx = ((rcx).wrapping_add(rcx)) as u64; rcx = (crate::simd_ops::ru32(rax.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(P9))) as u32 as u64; rax = ((rdx.wrapping_add(P10)) as u32) as u32 as u64; crate::simd_ops::wu32(rsi.wrapping_add(P8), ((rax as u32)) as u32); rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rsi.wrapping_add(P11), ((rcx as u32)) as u32); rsp = ((rsp).wrapping_add(0x40u64)) as u64; rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_68340<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64, const P9: u64, const P10: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; r8 = (crate::simd_ops::ru64(P0)) as u64; rsi = rdx; rdx = rcx; crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), (0x1u32) as u32); rbx = rcx; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), (0x1u32) as u32); rcx = rsi; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rdx = rsi; rcx = rbx; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru32(rsi.wrapping_add(P3))) as u32 as u64; rax = ((rcx as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_shr(1u32)) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 8; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(P4))) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rdi); rdi = (crate::simd_ops::ru64(rbx.wrapping_add(P5))) as u64; rbx = ((rdi.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; fl_l = ((rcx as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 7; continue; } bb = 2; continue; }
            2 => { fl_l = rdi; fl_r = rbx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 7; continue; } bb = 3; continue; }
            3 => { bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru32(rsi.wrapping_add(P3))) as u32 as u64; rax = (((rax as u32)).wrapping_shr(1u32)) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { rdx = rsi; rcx = rdi; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); bb = 6; continue; }
            6 => { rdi = ((rdi).wrapping_add(0x8u64)) as u64; fl_l = rdi; fl_r = rbx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 4; continue; } bb = 7; continue; }
            7 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; bb = 8; continue; }
            8 => { rdx = (crate::simd_ops::ru32(rsi.wrapping_add(P7))) as u32 as u64; rax = (crate::simd_ops::ru64(rsi)) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rcx = ((rdx.wrapping_add(rdx.wrapping_mul(2))) as u64) as u64; rcx = ((rcx).wrapping_add(rcx)) as u64; rcx = (crate::simd_ops::ru32(rax.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(P8))) as u32 as u64; rax = ((rdx.wrapping_add(P9)) as u32) as u32 as u64; crate::simd_ops::wu32(rsi.wrapping_add(P7), ((rax as u32)) as u32); rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rsi.wrapping_add(P10), ((rcx as u32)) as u32); rsp = ((rsp).wrapping_add(0x40u64)) as u64; rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_68342<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64, const P9: u64, const P10: u64, const P11: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu8(rsp.wrapping_add(0x18u64), ((r8 as u8)) as u8); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; r8 = (crate::simd_ops::ru64(P0)) as u64; rsi = rdx; rdx = rcx; crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), (0x1u32) as u32); rbx = rcx; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), (0x1u32) as u32); rcx = rsi; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rdx = rsi; rcx = rbx; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru32(rbx.wrapping_add(P3))) as u32 as u64; rdx = rsp.wrapping_add(0x60); rcx = (crate::simd_ops::ru64(rsi.wrapping_add(P4))) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x60u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P5))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru32(rsp.wrapping_add(0x60u64))) as u32 as u64; fl_r = 0x0; if ((((fl_l as u32)) as i32) <= (((fl_r as u32)) as i32)) { bb = 5; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(P3))) as u32 as u64; rbx = (crate::simd_ops::ru64(rbx.wrapping_add(P6))) as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 5; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rdi); rdi = ((rax as u32)) as u32 as u64; bb = 3; continue; }
            3 => { rdx = rsi; rcx = rbx; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); rbx = ((rbx).wrapping_add(0x8u64)) as u64; fl_l = rdi; fl_r = (0x1u64) as u64; rdi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } bb = 4; continue; }
            4 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; bb = 5; continue; }
            5 => { rdx = (crate::simd_ops::ru32(rsi.wrapping_add(P8))) as u32 as u64; rax = (crate::simd_ops::ru64(rsi)) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rcx = ((rdx.wrapping_add(rdx.wrapping_mul(2))) as u64) as u64; rcx = ((rcx).wrapping_add(rcx)) as u64; rcx = (crate::simd_ops::ru32(rax.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(P9))) as u32 as u64; rax = ((rdx.wrapping_add(P10)) as u32) as u32 as u64; crate::simd_ops::wu32(rsi.wrapping_add(P8), ((rax as u32)) as u32); rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rsi.wrapping_add(P11), ((rcx as u32)) as u32); rsp = ((rsp).wrapping_add(0x40u64)) as u64; rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_68343<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64, const P9: u64, const P10: u64, const P11: u64, const P12: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; r9 = (crate::simd_ops::ru32(P0)) as u32 as u64; rsi = rdx; r8 = (crate::simd_ops::ru64(P1)) as u64; rdx = rcx; rbx = rcx; crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), (0x1u32) as u32); rcx = rsi; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), (0x1u32) as u32); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); r9 = (crate::simd_ops::ru32(P3)) as u32 as u64; rdx = rbx; r8 = (crate::simd_ops::ru64(P4)) as u64; rcx = rsi; crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), (0x1u32) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), (0x1u32) as u32); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rsi.wrapping_add(P5))) as u64; rdx = ((rbx.wrapping_add(P6)) as u64) as u64; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); rcx = rsi; rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rsi.wrapping_add(P5))) as u64; rdx = (crate::simd_ops::ru32(rbx.wrapping_add(P9))) as u32 as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(P10))) as u64; crate::simd_ops::wu32(rax, ((rdx as u32)) as u32); crate::simd_ops::lk_add_u64(rcx.wrapping_add(P10), (0x4u64) as u64); fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 5; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(P9))) as u32 as u64; rbx = (crate::simd_ops::ru64(rbx.wrapping_add(P11))) as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 5; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rdi); rdi = ((rax as u32)) as u32 as u64; bb = 3; continue; }
            3 => { rdx = rsi; rcx = rbx; rax = crate::simd_ops::call_stk(P12, rax, rcx, rdx, r8, r9, rsp); rbx = ((rbx).wrapping_add(0x8u64)) as u64; fl_l = rdi; fl_r = (0x1u64) as u64; rdi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } bb = 4; continue; }
            4 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; bb = 5; continue; }
            5 => { rcx = rsi; rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rax = 0; fl_l = 0; fl_r = 0; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_68352<const P0: u64, const P1: u64, const P2: u64, const P3: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx);
    crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x30u64)) as u64;
    rbx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    rsi = rdx;
    rax = (crate::simd_ops::ru32(rcx.wrapping_add(P1))) as u32 as u64;
    rdi = ((rbx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64;
    fl_l = rbx; fl_r = rdi;
    if (((fl_l as u64)) == ((fl_r as u64))) { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rax = 0; fl_l = 0; fl_r = 0; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rbp);
    rbp = 0;
    fl_l = 0; fl_r = 0;
    loop { crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rbp); r9 = P2; r8 = 0x1; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbp); rdx = rbx; rcx = rsi; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rbx = ((rbx).wrapping_add(0x8u64)) as u64; fl_l = rbx; fl_r = rdi; if !((((fl_l as u64)) != ((fl_r as u64)))) { break; } }
    rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64;
    rax = 0;
    fl_l = 0; fl_r = 0;
    rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64;
    rsp = ((rsp).wrapping_add(0x30u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_68372<const P0: u64, const P1: u64, const P2: u64, const P3: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rdx; rdi = rcx; rsi = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; rcx = ((rsi as u32)) as u32 as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rax = (crate::simd_ops::ru32(rdi.wrapping_add(P1))) as u32 as u64; fl_l = rdx; fl_r = rax; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 5; continue; } bb = 1; continue; }
            1 => { rdx = ((rax as u32)) as u32 as u64; rdx = ((rdx).wrapping_shr(1u32)) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; fl_l = rdx; fl_r = rbx; rdx = (if (((fl_l as u64)) < ((fl_r as u64))) { rbx } else { rdx }) as u64; rcx = rdi; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rdx = rbx; fl_l = rdx; fl_r = rsi; rdx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 11; continue; } bb = 2; continue; }
            2 => { rcx = (crate::simd_ops::ru32(rdi.wrapping_add(P0))) as u32 as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rdi))) as u64; rax = 0; fl_l = 0; fl_r = 0; bb = 3; continue; }
            3 => { crate::simd_ops::wu64(rcx, rax); crate::simd_ops::wu64(rcx.wrapping_add(P0), rax); rcx = ((rcx.wrapping_add(P3)) as u64) as u64; fl_l = rdx; fl_r = (0x1u64) as u64; rdx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } bb = 4; continue; }
            4 => { crate::simd_ops::wu32(rdi.wrapping_add(P0), ((rbx as u32)) as u32); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            5 => { fl_l = rbx; fl_r = rsi; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 10; continue; } bb = 6; continue; }
            6 => { fl_l = rdx; fl_r = rsi; rdx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 11; continue; } bb = 7; continue; }
            7 => { rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rdi))) as u64; rax = 0; fl_l = 0; fl_r = 0; bb = 8; continue; }
            8 => { crate::simd_ops::wu64(rcx, rax); crate::simd_ops::wu64(rcx.wrapping_add(P0), rax); rcx = ((rcx.wrapping_add(P3)) as u64) as u64; fl_l = rdx; fl_r = (0x1u64) as u64; rdx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 8; continue; } bb = 9; continue; }
            9 => { crate::simd_ops::wu32(rdi.wrapping_add(P0), ((rbx as u32)) as u32); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            10 => { if (((fl_l as u64)) == ((fl_r as u64))) { bb = 12; continue; } bb = 11; continue; }
            11 => { crate::simd_ops::wu32(rdi.wrapping_add(P0), ((rbx as u32)) as u32); bb = 12; continue; }
            12 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_68473<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64, const P9: u64, const P10: u64, const P11: u64, const P12: u64, const P13: u64, const P14: u64, const P15: u64, const P16: u64, const P17: u64, const P18: u64, const P19: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi, mut r12, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 121>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x3b0u64)) as u64; rsi = r9; rdi = r8; rbx = rcx; r12 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), r12); rdx = r8; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 40; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru16(rax.wrapping_add(0x1eu64))) as u16 as u64; fl_r = ((r12 as u16)) as u16 as u64; if (((fl_l as u16)) == ((fl_r as u16))) { bb = 40; continue; } bb = 2; continue; }
            2 => { fl_l = ((rax).wrapping_add(0x10u64)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; if (((fl_l as u64)) == 0) { bb = 40; continue; } bb = 3; continue; }
            3 => { rdx = (crate::simd_ops::ru64(rbx)) as u64; fl_l = (crate::simd_ops::ru16(rax.wrapping_add(0xeu64))) as u16 as u64; fl_r = (0x3u16) as u16 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { bb = 59; continue; } bb = 4; continue; }
            4 => { crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdx); xmm0 = 0u128; ymm0_hi = 0; crate::simd_ops::wu128(rsp.wrapping_add(0x70u64), (xmm0) as u128); crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), r12); crate::simd_ops::wu64(rsp.wrapping_add(0x78u64), r12); crate::simd_ops::wu8(rsp.wrapping_add(0x80u64), ((r12 as u8)) as u8); let y_addr: u64 = rsp.wrapping_add(0x90u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); crate::simd_ops::wu64(rsp.wrapping_add(0x90u64), r12); crate::simd_ops::wu64(rsp.wrapping_add(0x98u64), r12); crate::simd_ops::wu64(rsp.wrapping_add(0xa0u64), r12); crate::simd_ops::wu64(rsp.wrapping_add(0xa8u64), r12); rax = rsp.wrapping_add(0x20); crate::simd_ops::wu64(rsp.wrapping_add(0xb0u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0xb8u64), r12); crate::simd_ops::wu32(rsp.wrapping_add(0xc0u64), ((r12 as u32)) as u32); rbx = (0xffffffffffffffffu64) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0xc4u64), ((rbx as u32)) as u32); rax = (crate::simd_ops::ru32(rsp.wrapping_add(0xc8u64))) as u32 as u64; rax = ((((rax as u32)) & (0xff000010u32))) as u32 as u64; rax = ((((rax as u32)) | (0x10u32))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0xc8u64), ((rax as u32)) as u32); crate::simd_ops::wu8(rsp.wrapping_add(0xcbu64), (0x10u8) as u8); crate::simd_ops::wu32(rsp.wrapping_add(0xccu64), ((r12 as u32)) as u32); crate::simd_ops::wu8(rsp.wrapping_add(0xd0u64), ((r12 as u8)) as u8); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x408u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x88u64), rax); r8 = 0; fl_l = 0; fl_r = 0; rdx = rsp.wrapping_add(0x70); rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rsp.wrapping_add(0xc8); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru32(rsp.wrapping_add(0xa8u64))) as u32 as u64; rdi = (((rax) as i64).wrapping_mul((0x88u64) as i64) as u64) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0xa0u64))) as u64; rdi = ((rdi).wrapping_add(rbx)) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 8; continue; } bb = 5; continue; }
            5 => { bb = 6; continue; }
            6 => { rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = ((rbx).wrapping_add(0x88u64)) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 6; continue; } bb = 7; continue; }
            7 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0xa0u64))) as u64; bb = 8; continue; }
            8 => { fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 16; continue; } bb = 9; continue; }
            9 => { rbx = ((rbx).wrapping_sub(1)) as u64; rbx = (((rbx) & (0xfffffffffe000000u64))) as u64; rax = ((crate::exe::teb_self() as u64)) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(rbx.wrapping_add(0x100u64))) as u64; rcx = (rcx & !0xffu64) | (((if (((fl_l as u64)) == ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); r9 = (crate::simd_ops::ru64(rsp.wrapping_add(0xa0u64))) as u64; rax = r9; rax = ((rax).wrapping_sub(rbx)) as u64; rax = ((rax).wrapping_shr(16u32)) as u64; rdx = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rdx = ((rdx).wrapping_shl(4u32)) as u64; rdx = ((rdx).wrapping_add(0x108u64)) as u64; rdx = ((rdx).wrapping_add(rbx)) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x4u64))) as u32 as u64; rdx = ((rdx).wrapping_sub(rax)) as u64; fl_l = ((rcx as u8)) as u8 as u64; fl_r = ((rcx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 14; continue; } bb = 10; continue; }
            10 => { fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(0xeu64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 13; continue; } bb = 11; continue; }
            11 => { rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(r9, rax); crate::simd_ops::wu64(rdx.wrapping_add(0x20u64), r9); fl_l = (crate::simd_ops::lk_add_u32(rdx.wrapping_add(0x18u64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; if (((fl_l as u32)) != 0) { bb = 16; continue; } bb = 12; continue; }
            12 => { rcx = rdx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 16; continue; }
            13 => { r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 15; continue; }
            14 => { r8 = 0; fl_l = 0; fl_r = 0; bb = 15; continue; }
            15 => { rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 16; continue; }
            16 => { crate::simd_ops::wu64(rsp.wrapping_add(0xa0u64), r12); crate::simd_ops::wu64(rsp.wrapping_add(0xa8u64), r12); rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x98u64))) as u32 as u64; rdi = (((rax) as i64).wrapping_mul((0x108u64) as i64) as u64) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x90u64))) as u64; rdi = ((rdi).wrapping_add(rbx)) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 20; continue; } bb = 17; continue; }
            17 => { bb = 18; continue; }
            18 => { rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = ((rbx).wrapping_add(0x108u64)) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 18; continue; } bb = 19; continue; }
            19 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x90u64))) as u64; bb = 20; continue; }
            20 => { fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 28; continue; } bb = 21; continue; }
            21 => { rbx = ((rbx).wrapping_sub(1)) as u64; rbx = (((rbx) & (0xfffffffffe000000u64))) as u64; rax = ((crate::exe::teb_self() as u64)) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(rbx.wrapping_add(0x100u64))) as u64; rcx = (rcx & !0xffu64) | (((if (((fl_l as u64)) == ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); r9 = (crate::simd_ops::ru64(rsp.wrapping_add(0x90u64))) as u64; rax = r9; rax = ((rax).wrapping_sub(rbx)) as u64; rax = ((rax).wrapping_shr(16u32)) as u64; rdx = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rdx = ((rdx).wrapping_shl(4u32)) as u64; rdx = ((rdx).wrapping_add(0x108u64)) as u64; rdx = ((rdx).wrapping_add(rbx)) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x4u64))) as u32 as u64; rdx = ((rdx).wrapping_sub(rax)) as u64; fl_l = ((rcx as u8)) as u8 as u64; fl_r = ((rcx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 26; continue; } bb = 22; continue; }
            22 => { fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(0xeu64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 25; continue; } bb = 23; continue; }
            23 => { rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(r9, rax); crate::simd_ops::wu64(rdx.wrapping_add(0x20u64), r9); fl_l = (crate::simd_ops::lk_add_u32(rdx.wrapping_add(0x18u64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; if (((fl_l as u32)) != 0) { bb = 28; continue; } bb = 24; continue; }
            24 => { rcx = rdx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 28; continue; }
            25 => { r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 27; continue; }
            26 => { r8 = 0; fl_l = 0; fl_r = 0; bb = 27; continue; }
            27 => { rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 28; continue; }
            28 => { crate::simd_ops::wu64(rsp.wrapping_add(0x90u64), r12); crate::simd_ops::wu64(rsp.wrapping_add(0x98u64), r12); rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x78u64))) as u32 as u64; rdi = (((rax) as i64).wrapping_mul((0x108u64) as i64) as u64) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rdi = ((rdi).wrapping_add(rbx)) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 31; continue; } bb = 29; continue; }
            29 => { rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = ((rbx).wrapping_add(0x108u64)) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 29; continue; } bb = 30; continue; }
            30 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; bb = 31; continue; }
            31 => { fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 39; continue; } bb = 32; continue; }
            32 => { rbx = ((rbx).wrapping_sub(1)) as u64; rbx = (((rbx) & (0xfffffffffe000000u64))) as u64; rax = ((crate::exe::teb_self() as u64)) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(rbx.wrapping_add(0x100u64))) as u64; rcx = (rcx & !0xffu64) | (((if (((fl_l as u64)) == ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); r9 = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rax = r9; rax = ((rax).wrapping_sub(rbx)) as u64; rax = ((rax).wrapping_shr(16u32)) as u64; rdx = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rdx = ((rdx).wrapping_shl(4u32)) as u64; rdx = ((rdx).wrapping_add(0x108u64)) as u64; rdx = ((rdx).wrapping_add(rbx)) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x4u64))) as u32 as u64; rdx = ((rdx).wrapping_sub(rax)) as u64; fl_l = ((rcx as u8)) as u8 as u64; fl_r = ((rcx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 37; continue; } bb = 33; continue; }
            33 => { fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(0xeu64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 36; continue; } bb = 34; continue; }
            34 => { rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(r9, rax); crate::simd_ops::wu64(rdx.wrapping_add(0x20u64), r9); fl_l = (crate::simd_ops::lk_add_u32(rdx.wrapping_add(0x18u64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; if (((fl_l as u32)) != 0) { bb = 39; continue; } bb = 35; continue; }
            35 => { rcx = rdx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 39; continue; }
            36 => { r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 38; continue; }
            37 => { r8 = 0; fl_l = 0; fl_r = 0; bb = 38; continue; }
            38 => { rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 39; continue; }
            39 => { bb = 58; continue; }
            40 => { r9 = rdi; r8 = P5; rdx = ((crate::simd_ops::ru8(P6)) as u32) as u32 as u64; rcx = rsp.wrapping_add(0x70); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = rax; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x3f8u64))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P8, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rsp.wrapping_add(0x70); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = 0; fl_l = 0; fl_r = 0; r8 = 0x310; rcx = rsp.wrapping_add(0x70); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P9, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = rsp.wrapping_add(0x80); crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), rdx); crate::simd_ops::wu32(rsp.wrapping_add(0x7cu64), (0x10u32) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x388u64), (0x6u32) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x390u64), r12); rbx = (0xffffffffffffffffu64) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x398u64), ((rbx as u32)) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x3a0u64), r12); crate::simd_ops::wu64(rsp.wrapping_add(0x380u64), r12); xmm0 = 0u128; ymm0_hi = 0; let y_addr: u64 = rsp.wrapping_add(0x80u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); crate::simd_ops::wu128(rsp.wrapping_add(0xa0u64), (xmm0) as u128); rax = ((r12.wrapping_add(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x78u64), ((rax as u32)) as u32); r14 = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; r14 = ((r14).wrapping_shl(4u32)) as u64; r14 = ((r14).wrapping_add(rdx)) as u64; crate::simd_ops::wu64(r14.wrapping_sub(0x30u64), r12); rax = P10; crate::simd_ops::wu64(r14.wrapping_sub(0x28u64), rax); crate::simd_ops::wu64(r14.wrapping_sub(0x20u64), r12); crate::simd_ops::wu64(r14.wrapping_sub(0x18u64), r12); crate::simd_ops::wu32(r14.wrapping_sub(0x8u64), ((rbx as u32)) as u32); crate::simd_ops::wu64(r14.wrapping_sub(0x10u64), r12); rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x390u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 43; continue; } bb = 41; continue; }
            41 => { rcx = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 43; continue; } bb = 42; continue; }
            42 => { rax = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(r14.wrapping_sub(0x18u64), rax); bb = 43; continue; }
            43 => { crate::simd_ops::wu32(rsp.wrapping_add(0x3a8u64), ((r12 as u32)) as u32); r15 = (crate::simd_ops::ru64(P11)) as u64; r9 = (crate::simd_ops::ru32(rsp.wrapping_add(0x78u64))) as u32 as u64; r8 = (crate::simd_ops::ru32(rsp.wrapping_add(0x7cu64))) as u32 as u64; rcx = ((r8.wrapping_add(r8.wrapping_mul(2))) as u64) as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rax = ((r9.wrapping_add(r9.wrapping_mul(2))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 45; continue; } bb = 44; continue; }
            44 => { rax = r9.wrapping_add(0x1); rdx = ((r8 as u32)) as u32 as u64; rdx = ((rdx).wrapping_shr(1u32)) as u64; rdx = ((rdx).wrapping_add(r8)) as u64; fl_l = rdx; fl_r = rax; rdx = (if (((fl_l as u64)) < ((fl_r as u64))) { rax } else { rdx }) as u64; rcx = rsp.wrapping_add(0x70); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P12, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r9 = (crate::simd_ops::ru32(rsp.wrapping_add(0x78u64))) as u32 as u64; bb = 45; continue; }
            45 => { rax = ((r9 as u32)) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rsp.wrapping_add(0x70u64)))) as u64; xmm0 = 0u128; ymm0_hi = 0; let y_addr: u64 = rcx; crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); crate::simd_ops::wu128(rcx.wrapping_add(0x20u64), (xmm0) as u128); rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x78u64))) as u32 as u64; rax = (((rax as u32)).wrapping_add(1)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x78u64), ((rax as u32)) as u32); r14 = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; r14 = ((r14).wrapping_shl(4u32)) as u64; r14 = ((r14).wrapping_add(crate::simd_ops::ru64(rsp.wrapping_add(0x70u64)))) as u64; crate::simd_ops::wu64(r14.wrapping_sub(0x30u64), rsi); crate::simd_ops::wu64(r14.wrapping_sub(0x28u64), r15); crate::simd_ops::wu32(r14.wrapping_sub(0x20u64), (0x1u32) as u32); crate::simd_ops::wu32(r14.wrapping_sub(0x1cu64), (0x1u32) as u32); crate::simd_ops::wu64(r14.wrapping_sub(0x18u64), r12); crate::simd_ops::wu32(r14.wrapping_sub(0x8u64), ((rbx as u32)) as u32); crate::simd_ops::wu64(r14.wrapping_sub(0x10u64), r12); rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x390u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 48; continue; } bb = 46; continue; }
            46 => { rcx = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 48; continue; } bb = 47; continue; }
            47 => { rax = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(r14.wrapping_sub(0x18u64), rax); bb = 48; continue; }
            48 => { rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x3a8u64))) as u32 as u64; rax = (((rax as u32)).wrapping_shr(1u32)) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 50; continue; } bb = 49; continue; }
            49 => { crate::simd_ops::wu64(rsi, r12); crate::simd_ops::wu64(rsi.wrapping_add(0x8u64), r12); bb = 50; continue; }
            50 => { rdx = (crate::simd_ops::ru32(rsp.wrapping_add(0x78u64))) as u32 as u64; rcx = ((rdx.wrapping_add(rdx.wrapping_mul(2))) as u64) as u64; rcx = ((rcx).wrapping_add(rcx)) as u64; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rcx = (crate::simd_ops::ru32(rax.wrapping_add(rcx.wrapping_mul(8)).wrapping_sub(0x8u64))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x398u64), ((rcx as u32)) as u32); rdx = (((rdx as u32)).wrapping_sub(1)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x78u64), ((rdx as u32)) as u32); rcx = rsp.wrapping_add(0x70); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P13, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), (0x10000010u64) as u64); crate::simd_ops::wu8(rsp.wrapping_add(0x78u64), (0x0u8) as u8); fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 53; continue; } bb = 51; continue; }
            51 => { crate::simd_ops::wu32(rsp.wrapping_add(0x74u64), ((r12 as u32)) as u32); crate::simd_ops::wu8(rsp.wrapping_add(0x78u64), ((rdi as u8)) as u8); bb = 55; continue; }
            52 => { bb = 53; continue; }
            53 => { rbx = ((rbx).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru8(rdi.wrapping_add(rbx))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 53; continue; } bb = 54; continue; }
            54 => { crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rdi); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rbx); r9 = rbx; r8 = 0; fl_l = 0; fl_r = 0; rdx = rsp.wrapping_add(0x40); rcx = rsp.wrapping_add(0x70); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P14, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 55; continue; }
            55 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x400u64))) as u64; rdx = (crate::simd_ops::ru32(rbx.wrapping_add(0x8u64))) as u32 as u64; r8 = (crate::simd_ops::ru32(rbx.wrapping_add(0xcu64))) as u32 as u64; rcx = (((r8) as i64).wrapping_mul((0x88u64) as i64) as u64) as u64; rax = (((rdx) as i64).wrapping_mul((0x88u64) as i64) as u64) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 57; continue; } bb = 56; continue; }
            56 => { rax = rdx.wrapping_add(0x1); rdx = ((r8 as u32)) as u32 as u64; rdx = ((rdx).wrapping_shr(1u32)) as u64; rdx = ((rdx).wrapping_add(r8)) as u64; fl_l = rdx; fl_r = rax; rdx = (if (((fl_l as u64)) < ((fl_r as u64))) { rax } else { rdx }) as u64; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P15, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 57; continue; }
            57 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x8u64))) as u32 as u64; rcx = (((rax) as i64).wrapping_mul((0x88u64) as i64) as u64) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rbx))) as u64; crate::simd_ops::lk_and_u32(rcx, (0xff000010u32) as u32); crate::simd_ops::lk_or_u32(rcx, (0x10u32) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x3u64), (0x10u8) as u8); crate::simd_ops::wu32(rcx.wrapping_add(0x4u64), ((r12 as u32)) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x8u64), (0x0u8) as u8); rdx = rsp.wrapping_add(0x70); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P16, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::lk_add_u32(rbx.wrapping_add(0x8u64), (1) as u32); rcx = rsp.wrapping_add(0x70); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 58; continue; }
            58 => { r11 = rsp.wrapping_add(0x3b0); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x20u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x28u64))) as u64; rdi = (crate::simd_ops::ru64(r11.wrapping_add(0x30u64))) as u64; rsp = r11; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            59 => { r8 = rdi; rcx = rsp.wrapping_add(0x40); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P17, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P18, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::lift_ops::trap(0x80000003, P19, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_68520<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x40u64)) as u64;
    r9 = (crate::simd_ops::ru32(P0)) as u32 as u64;
    rdi = rdx;
    r8 = (crate::simd_ops::ru64(P1)) as u64;
    rdx = rcx;
    rbx = rcx;
    crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), (0x1u32) as u32);
    rcx = rdi;
    crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), (0x1u32) as u32);
    rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp);
    rcx = (crate::simd_ops::ru64(rdi.wrapping_add(P3))) as u64;
    rdx = rbx;
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P4))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp);
    rcx = (crate::simd_ops::ru64(rdi.wrapping_add(P3))) as u64;
    rdx = ((rbx.wrapping_add(P5)) as u64) as u64;
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P6))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp);
    rcx = (crate::simd_ops::ru64(rdi.wrapping_add(P3))) as u64;
    rdx = ((rbx.wrapping_add(P7)) as u64) as u64;
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P6))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp);
    rcx = rdi;
    rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp);
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64;
    rax = 0;
    fl_l = 0; fl_r = 0;
    rsp = ((rsp).wrapping_add(0x40u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_68527<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64, const P9: u64, const P10: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x50u64)) as u64; r9 = (crate::simd_ops::ru32(P0)) as u32 as u64; rbx = rdx; r8 = (crate::simd_ops::ru64(P1)) as u64; rdx = rcx; crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), rsi); rsi = rcx; rcx = rbx; crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), (0x1u32) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), (0x1u32) as u32); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(P3))) as u32 as u64; fl_r = 0x1; rbp = P4; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 2; continue; } bb = 1; continue; }
            1 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P5))) as u64; rdx = rsi; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P6))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rdi = ((rbx.wrapping_add(P7)) as u64) as u64; bb = 4; continue; }
            2 => { rdi = ((rbx.wrapping_add(P7)) as u64) as u64; fl_l = (crate::simd_ops::ru8(rdi)) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 4; continue; } bb = 3; continue; }
            3 => { r9 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rbp); r8 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), (0x0u64) as u64); rdx = rsp.wrapping_add(0x40); rcx = rsi; rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); bb = 4; continue; }
            4 => { fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(P3))) as u32 as u64; fl_r = 0x1; r10 = ((rsi.wrapping_add(P9)) as u64) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 6; continue; } bb = 5; continue; }
            5 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P5))) as u64; rdx = r10; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P6))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 8; continue; }
            6 => { fl_l = (crate::simd_ops::ru8(rdi)) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 8; continue; } bb = 7; continue; }
            7 => { r9 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rbp); r8 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), (0x0u64) as u64); rdx = rsp.wrapping_add(0x40); rcx = r10; rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); bb = 8; continue; }
            8 => { rcx = rbx; rax = crate::simd_ops::call_stk(P10, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rax = 0; fl_l = 0; fl_r = 0; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rsp = ((rsp).wrapping_add(0x50u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_68528<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64, const P9: u64, const P10: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x50u64)) as u64; r9 = (crate::simd_ops::ru32(P0)) as u32 as u64; rbx = rdx; r8 = (crate::simd_ops::ru64(P1)) as u64; rdx = rcx; crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), rsi); rsi = rcx; rcx = rbx; crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), (0x1u32) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), (0x1u32) as u32); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(P3))) as u32 as u64; fl_r = 0x1; rbp = P4; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 2; continue; } bb = 1; continue; }
            1 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P5))) as u64; rdx = rsi; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); rdi = ((rbx.wrapping_add(P7)) as u64) as u64; bb = 4; continue; }
            2 => { rdi = ((rbx.wrapping_add(P7)) as u64) as u64; fl_l = (crate::simd_ops::ru8(rdi)) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 4; continue; } bb = 3; continue; }
            3 => { r9 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rbp); r8 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), (0x0u64) as u64); rdx = rsp.wrapping_add(0x40); rcx = rsi; rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); bb = 4; continue; }
            4 => { fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(P3))) as u32 as u64; fl_r = 0x1; rcx = ((rsi.wrapping_add(P9)) as u64) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 6; continue; } bb = 5; continue; }
            5 => { rdx = rcx; rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P5))) as u64; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); bb = 8; continue; }
            6 => { fl_l = (crate::simd_ops::ru8(rdi)) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 8; continue; } bb = 7; continue; }
            7 => { r9 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rbp); r8 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), (0x0u64) as u64); rdx = rsp.wrapping_add(0x40); rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); bb = 8; continue; }
            8 => { rcx = rbx; rax = crate::simd_ops::call_stk(P10, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rax = 0; fl_l = 0; fl_r = 0; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rsp = ((rsp).wrapping_add(0x50u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_68532<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x40u64)) as u64;
    r8 = (crate::simd_ops::ru64(P0)) as u64;
    rdi = rdx;
    rdx = rcx;
    crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), (0x1u32) as u32);
    rbx = rcx;
    crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), (0x1u32) as u32);
    rcx = rdi;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rcx = (crate::simd_ops::ru64(rdi.wrapping_add(P2))) as u64;
    rdx = rbx;
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp);
    rcx = (crate::simd_ops::ru64(rdi.wrapping_add(P2))) as u64;
    rdx = ((rbx.wrapping_add(P3)) as u64) as u64;
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp);
    rdx = (crate::simd_ops::ru32(rdi.wrapping_add(P3))) as u32 as u64;
    rax = (crate::simd_ops::ru64(rdi)) as u64;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64;
    rcx = ((rdx.wrapping_add(rdx.wrapping_mul(2))) as u64) as u64;
    rcx = ((rcx).wrapping_add(rcx)) as u64;
    rcx = (crate::simd_ops::ru32(rax.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(P4))) as u32 as u64;
    rax = ((rdx.wrapping_add(P5)) as u32) as u32 as u64;
    crate::simd_ops::wu32(rdi.wrapping_add(P3), ((rax as u32)) as u32);
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu32(rdi.wrapping_add(P6), ((rcx as u32)) as u32);
    rsp = ((rsp).wrapping_add(0x40u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_68533<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x40u64)) as u64;
    r8 = (crate::simd_ops::ru64(P0)) as u64;
    rdi = rdx;
    rdx = rcx;
    crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), (0x1u32) as u32);
    rbx = rcx;
    crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), (0x1u32) as u32);
    rcx = rdi;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rcx = (crate::simd_ops::ru64(rdi.wrapping_add(P2))) as u64;
    rdx = rbx;
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P3))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp);
    rcx = (crate::simd_ops::ru64(rdi.wrapping_add(P2))) as u64;
    rdx = ((rbx.wrapping_add(P4)) as u64) as u64;
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P3))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp);
    rdx = (crate::simd_ops::ru32(rdi.wrapping_add(P4))) as u32 as u64;
    rax = (crate::simd_ops::ru64(rdi)) as u64;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64;
    rcx = ((rdx.wrapping_add(rdx.wrapping_mul(2))) as u64) as u64;
    rcx = ((rcx).wrapping_add(rcx)) as u64;
    rcx = (crate::simd_ops::ru32(rax.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(P5))) as u32 as u64;
    rax = ((rdx.wrapping_add(P6)) as u32) as u32 as u64;
    crate::simd_ops::wu32(rdi.wrapping_add(P4), ((rax as u32)) as u32);
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu32(rdi.wrapping_add(P7), ((rcx as u32)) as u32);
    rsp = ((rsp).wrapping_add(0x40u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_68679<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); crate::simd_ops::wu32(rsp.wrapping_add(0x18u64), ((r8 as u32)) as u32); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; fl_l = (crate::simd_ops::ru64(rdx)) as u64; fl_r = (0x0u64) as u64; rsi = rcx; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rdi = rdx; crate::simd_ops::wu8(rsp.wrapping_add(0x40u64), (if (((fl_l as u64)) != ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8); rdx = rsp.wrapping_add(0x40); rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rdx = (crate::simd_ops::ru64(rdi)) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(P2)) as u64; rcx = P2; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbx); rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P3))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(rsi.wrapping_add(P0))) as u64; rcx = rax; rdx = (crate::simd_ops::ru64(rax)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rdx.wrapping_add(P4))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu32(rsp.wrapping_add(0x40u64), ((rax as u32)) as u32); rdx = rsp.wrapping_add(0x40); rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P5))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rdi)) as u64; r8 = 0; fl_l = 0; fl_r = 0; rdx = rsi; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P6))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; bb = 2; continue; }
            2 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_68695<const P0: u64, const P1: u64, const P2: u64, const P3: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rdx; rdi = rcx; rsi = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(P1))) as u32 as u64; fl_l = rdx; fl_r = rax; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 3; continue; } bb = 1; continue; }
            1 => { rdx = ((rax as u32)) as u32 as u64; rdx = ((rdx).wrapping_shr(1u32)) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; fl_l = rdx; fl_r = rbx; rdx = (if (((fl_l as u64)) < ((fl_r as u64))) { rbx } else { rdx }) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); r8 = rbx; fl_l = r8; fl_r = rsi; r8 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 7; continue; } bb = 2; continue; }
            2 => { rcx = (crate::simd_ops::ru32(rdi.wrapping_add(P0))) as u32 as u64; rax = (crate::simd_ops::ru64(rdi)) as u64; rcx = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; bb = 6; continue; }
            3 => { fl_l = rbx; fl_r = rsi; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 8; continue; } bb = 4; continue; }
            4 => { r8 = rbx; fl_l = r8; fl_r = rsi; r8 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 7; continue; } bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rcx = ((rax.wrapping_add(rsi.wrapping_mul(8))) as u64) as u64; bb = 6; continue; }
            6 => { r8 = ((r8.wrapping_mul(8)) as u64) as u64; rdx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 7; continue; }
            7 => { crate::simd_ops::wu32(rdi.wrapping_add(P0), ((rbx as u32)) as u32); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            8 => { if (((fl_l as u64)) == ((fl_r as u64))) { bb = 10; continue; } bb = 9; continue; }
            9 => { crate::simd_ops::wu32(rcx.wrapping_add(P0), ((rbx as u32)) as u32); bb = 10; continue; }
            10 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_68731<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64, const P9: u64, const P10: u64, const P11: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 27>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), r8); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdx); crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rbp = rsp.wrapping_sub(0x1f); rsp = ((rsp).wrapping_sub(0xa0u64)) as u64; r14 = rdx; r15 = rcx; fl_l = (crate::simd_ops::ru64(rdx.wrapping_add(0x8u64))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 20; continue; } bb = 1; continue; }
            1 => { r8 = (crate::simd_ops::ru64(rcx)) as u64; fl_l = r8; fl_r = (crate::simd_ops::ru64(rcx.wrapping_add(0x10u64))) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 121; continue; } bb = 2; continue; }
            2 => { rsi = (0xffffffffffffu64) as u64; r12 = 0x1000; r13 = 0; fl_l = 0; fl_r = 0; rdi = (0x249249249249249u64) as u64; bb = 3; continue; }
            3 => { fl_l = (crate::simd_ops::ru16(r8.wrapping_add(0xeu64))) as u16 as u64; fl_r = ((r12 as u16)) as u16 as u64; if ((((fl_l as u16)) & ((fl_r as u16))) != 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { r8 = (crate::simd_ops::ru64(r8.wrapping_add(0x8u64))) as u64; fl_l = (((r8) & (rsi))) as u64; fl_r = fl_l; r8 = ((fl_l as u64)) as u64; bb = 5; continue; }
            5 => { crate::simd_ops::wu64(rbp.wrapping_sub(0x21u64), r8); rax = (0xffffffffffffffffu64) as u64; bb = 6; continue; }
            6 => { rax = ((rax).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru8(r8.wrapping_add(rax))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 6; continue; } bb = 7; continue; }
            7 => { crate::simd_ops::wu64(rbp.wrapping_sub(0x19u64), rax); xmm0 = (crate::simd_ops::ru128(rbp.wrapping_sub(0x21u64))) as u128; crate::simd_ops::wu128(rbp.wrapping_sub(0x11u64), (xmm0) as u128); rdx = rbp.wrapping_add(0x67); rcx = rbp.wrapping_sub(0x11); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = (crate::simd_ops::ru64(r14)) as u64; rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x1u64), rax); crate::simd_ops::wu32(rbp.wrapping_add(0x7u64), ((r13 as u32)) as u32); rcx = rbx; rdx = (crate::simd_ops::ru64(rbp.wrapping_add(0x67u64))) as u64; fl_l = (crate::simd_ops::ru8(rax.wrapping_add(0x19u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 12; continue; } bb = 8; continue; }
            8 => { crate::simd_ops::wu64(rbp.wrapping_sub(0x1u64), rax); fl_l = (crate::simd_ops::ru64(rax.wrapping_add(0x20u64))) as u64; fl_r = rdx; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 10; continue; } bb = 9; continue; }
            9 => { crate::simd_ops::wu32(rbp.wrapping_add(0x7u64), ((r13 as u32)) as u32); rax = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; bb = 11; continue; }
            10 => { crate::simd_ops::wu32(rbp.wrapping_add(0x7u64), (0x1u32) as u32); rcx = rax; rax = (crate::simd_ops::ru64(rax)) as u64; bb = 11; continue; }
            11 => { fl_l = (crate::simd_ops::ru8(rax.wrapping_add(0x19u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 8; continue; } bb = 12; continue; }
            12 => { fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0x19u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 14; continue; } bb = 13; continue; }
            13 => { fl_l = rdx; fl_r = (crate::simd_ops::ru64(rcx.wrapping_add(0x20u64))) as u64; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 16; continue; } bb = 14; continue; }
            14 => { fl_l = (crate::simd_ops::ru64(r14.wrapping_add(0x8u64))) as u64; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 122; continue; } bb = 15; continue; }
            15 => { crate::simd_ops::wu64(rbp.wrapping_sub(0x31u64), r14); crate::simd_ops::wu64(rbp.wrapping_sub(0x29u64), r13); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r8 = rax; rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x67u64))) as u64; crate::simd_ops::wu64(r8.wrapping_add(0x20u64), rax); crate::simd_ops::lk_and_u32(r8.wrapping_add(0x28u64), (0xff000008u32) as u32); crate::simd_ops::lk_or_u32(r8.wrapping_add(0x28u64), (0x8u32) as u32); crate::simd_ops::wu8(r8.wrapping_add(0x2bu64), (0x8u8) as u8); crate::simd_ops::wu32(r8.wrapping_add(0x2cu64), ((r13 as u32)) as u32); crate::simd_ops::wu8(r8.wrapping_add(0x30u64), (0x0u8) as u8); crate::simd_ops::wu64(r8, rbx); crate::simd_ops::wu64(r8.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(r8.wrapping_add(0x10u64), rbx); crate::simd_ops::wu16(r8.wrapping_add(0x18u64), (0x0u16) as u16); crate::simd_ops::wu64(rbp.wrapping_sub(0x29u64), r13); xmm0 = (crate::simd_ops::ru128(rbp.wrapping_sub(0x1u64))) as u128; crate::simd_ops::wu128(rbp.wrapping_sub(0x11u64), (xmm0) as u128); rdx = rbp.wrapping_sub(0x11); rcx = r14; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rax; bb = 16; continue; }
            16 => { rdx = rcx.wrapping_add(0x28); rcx = r15.wrapping_add(0x20); rax = (crate::simd_ops::ru64(r15)) as u64; fl_l = (crate::simd_ops::ru16(rax.wrapping_add(0xeu64))) as u16 as u64; fl_r = ((r12 as u16)) as u16 as u64; if ((((fl_l as u16)) & ((fl_r as u16))) != 0) { bb = 18; continue; } bb = 17; continue; }
            17 => { rax = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; fl_l = (((rax) & (rsi))) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; bb = 18; continue; }
            18 => { r9 = rax; r8 = 0; fl_l = 0; fl_r = 0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r8 = (crate::simd_ops::ru64(r15)) as u64; r8 = ((r8).wrapping_add(0x20u64)) as u64; crate::simd_ops::wu64(r15, r8); fl_l = r8; fl_r = (crate::simd_ops::ru64(r15.wrapping_add(0x10u64))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } bb = 19; continue; }
            19 => { bb = 121; continue; }
            20 => { xmm0 = 0u128; crate::simd_ops::wu128(rbp.wrapping_sub(0x31u64), (xmm0) as u128); r13 = 0; fl_l = 0; fl_r = 0; rsi = ((r13 as u32)) as u32 as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x31u64), r13); rdi = ((r13 as u32)) as u32 as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x29u64), r13); r14 = ((r13 as u32)) as u32 as u64; rdx = (crate::simd_ops::ru64(rbp.wrapping_add(0x67u64))) as u64; rax = (crate::simd_ops::ru64(rdx)) as u64; rbx = (crate::simd_ops::ru64(rax)) as u64; fl_l = rbx; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 36; continue; } bb = 21; continue; }
            21 => { bb = 22; continue; }
            22 => { rax = ((rdi as u32)) as u32 as u64; r15 = ((rsi.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rax = ((r14 as u32)) as u32 as u64; rcx = ((rsi.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; fl_l = r15; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 26; continue; } bb = 23; continue; }
            23 => { r15 = ((r15).wrapping_sub(rsi)) as u64; rdi = r15; rdi = ((((rdi) as i64).wrapping_shr(3u32) as u64)) as u64; rax = rdi.wrapping_add(0x1); rcx = ((rcx).wrapping_sub(rsi)) as u64; rcx = ((((rcx) as i64).wrapping_shr(3u32) as u64)) as u64; r14 = rcx; r14 = ((r14).wrapping_shr(1u32)) as u64; r14 = ((r14).wrapping_add(rcx)) as u64; fl_l = r14; fl_r = rax; r14 = (if (((fl_l as u64)) < ((fl_r as u64))) { rax } else { r14 }) as u64; rcx = ((r14.wrapping_mul(8)) as u64) as u64; rdx = 0x8; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r12 = rax; r8 = r15; rdx = rsi; rcx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 25; continue; } bb = 24; continue; }
            24 => { rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 25; continue; }
            25 => { rsi = r12; crate::simd_ops::wu64(rbp.wrapping_sub(0x31u64), r12); crate::simd_ops::wu32(rbp.wrapping_sub(0x25u64), ((r14 as u32)) as u32); rdx = (crate::simd_ops::ru64(rbp.wrapping_add(0x67u64))) as u64; bb = 26; continue; }
            26 => { rax = ((rdi as u32)) as u32 as u64; crate::simd_ops::wu64(rsi.wrapping_add(rax.wrapping_mul(8)), rbx); rdi = (((rdi as u32)).wrapping_add(1)) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_sub(0x29u64), ((rdi as u32)) as u32); rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; fl_l = (crate::simd_ops::ru8(rax.wrapping_add(0x19u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 31; continue; } bb = 27; continue; }
            27 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; fl_l = (crate::simd_ops::ru8(rax.wrapping_add(0x19u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 30; continue; } bb = 28; continue; }
            28 => { fl_l = rbx; fl_r = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 30; continue; } bb = 29; continue; }
            29 => { rbx = rax; rax = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; fl_l = (crate::simd_ops::ru8(rax.wrapping_add(0x19u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 28; continue; } bb = 30; continue; }
            30 => { rbx = rax; bb = 34; continue; }
            31 => { rbx = rax; rcx = (crate::simd_ops::ru64(rax)) as u64; fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0x19u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 34; continue; } bb = 32; continue; }
            32 => { bb = 33; continue; }
            33 => { rbx = rcx; rax = (crate::simd_ops::ru64(rcx)) as u64; rcx = rax; fl_l = (crate::simd_ops::ru8(rax.wrapping_add(0x19u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 33; continue; } bb = 34; continue; }
            34 => { fl_l = rbx; fl_r = (crate::simd_ops::ru64(rdx)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 22; continue; } bb = 35; continue; }
            35 => { r15 = (crate::simd_ops::ru64(rbp.wrapping_add(0x5fu64))) as u64; bb = 36; continue; }
            36 => { xmm0 = 0u128; crate::simd_ops::wu128(rbp.wrapping_sub(0x21u64), (xmm0) as u128); crate::simd_ops::wu64(rbp.wrapping_sub(0x21u64), r13); crate::simd_ops::wu64(rbp.wrapping_sub(0x19u64), r13); rcx = (crate::simd_ops::ru64(r15)) as u64; fl_l = rcx; fl_r = (crate::simd_ops::ru64(r15.wrapping_add(0x10u64))) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 67; continue; } bb = 37; continue; }
            37 => { r12 = 0x1000; r13 = (0xffffffffffffu64) as u64; bb = 38; continue; }
            38 => { fl_l = (crate::simd_ops::ru16(rcx.wrapping_add(0xeu64))) as u16 as u64; fl_r = ((r12 as u16)) as u16 as u64; if ((((fl_l as u16)) & ((fl_r as u16))) != 0) { bb = 40; continue; } bb = 39; continue; }
            39 => { rcx = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; fl_l = (((rcx) & (r13))) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; bb = 40; continue; }
            40 => { crate::simd_ops::wu64(rbp.wrapping_sub(0x41u64), rcx); rax = (0xffffffffffffffffu64) as u64; bb = 41; continue; }
            41 => { rax = ((rax).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(rax))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 41; continue; } bb = 42; continue; }
            42 => { crate::simd_ops::wu64(rbp.wrapping_sub(0x39u64), rax); xmm0 = (crate::simd_ops::ru128(rbp.wrapping_sub(0x41u64))) as u128; crate::simd_ops::wu128(rbp.wrapping_sub(0x11u64), (xmm0) as u128); rdx = rbp.wrapping_add(0x6f); rcx = rbp.wrapping_sub(0x11); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x67u64))) as u64; rbx = (crate::simd_ops::ru64(rax)) as u64; rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; rcx = rbx; r14 = (crate::simd_ops::ru64(rbp.wrapping_add(0x6fu64))) as u64; fl_l = (crate::simd_ops::ru8(rax.wrapping_add(0x19u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 48; continue; } bb = 43; continue; }
            43 => { bb = 44; continue; }
            44 => { fl_l = (crate::simd_ops::ru64(rax.wrapping_add(0x20u64))) as u64; fl_r = r14; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 46; continue; } bb = 45; continue; }
            45 => { rax = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; bb = 47; continue; }
            46 => { rbx = rax; rax = (crate::simd_ops::ru64(rax)) as u64; bb = 47; continue; }
            47 => { fl_l = (crate::simd_ops::ru8(rax.wrapping_add(0x19u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 44; continue; } bb = 48; continue; }
            48 => { fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(0x19u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 62; continue; } bb = 49; continue; }
            49 => { fl_l = r14; fl_r = (crate::simd_ops::ru64(rbx.wrapping_add(0x20u64))) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 62; continue; } bb = 50; continue; }
            50 => { fl_l = rbx; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 62; continue; } bb = 51; continue; }
            51 => { rdx = rbx.wrapping_add(0x28); rax = (crate::simd_ops::ru64(r15)) as u64; fl_l = (crate::simd_ops::ru16(rax.wrapping_add(0xeu64))) as u16 as u64; fl_r = ((r12 as u16)) as u16 as u64; if ((((fl_l as u16)) & ((fl_r as u16))) != 0) { bb = 53; continue; } bb = 52; continue; }
            52 => { rax = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; fl_l = (((rax) & (r13))) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; bb = 53; continue; }
            53 => { r9 = rax; r8 = 0; fl_l = 0; fl_r = 0; rcx = r15.wrapping_add(0x20); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = rsi; rcx = ((rdi as u32)) as u32 as u64; rdx = ((rsi.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = rsi; fl_r = rdx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 65; continue; } bb = 54; continue; }
            54 => { bb = 55; continue; }
            55 => { fl_l = (crate::simd_ops::ru64(rax)) as u64; fl_r = rbx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 59; continue; } bb = 56; continue; }
            56 => { rcx = ((rdi as u32)) as u32 as u64; rcx = ((rcx).wrapping_sub(1)) as u64; rcx = ((rsi.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 58; continue; } bb = 57; continue; }
            57 => { rdx = (crate::simd_ops::ru64(rcx)) as u64; crate::simd_ops::wu64(rax, rdx); bb = 58; continue; }
            58 => { fl_l = (((rdi as u32)).wrapping_sub(1)) as u32 as u64; rdi = ((fl_l as u32)) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_sub(0x29u64), ((rdi as u32)) as u32); bb = 60; continue; }
            59 => { fl_l = ((rax).wrapping_add(0x8u64)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; bb = 60; continue; }
            60 => { rcx = ((rdi as u32)) as u32 as u64; rdx = ((rsi.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = rax; fl_r = rdx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 55; continue; } bb = 61; continue; }
            61 => { bb = 65; continue; }
            62 => { rcx = rbp.wrapping_sub(0x21); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = rax; crate::simd_ops::wu64(rax, r14); rax = (crate::simd_ops::ru64(r15)) as u64; fl_l = (crate::simd_ops::ru16(rax.wrapping_add(0xeu64))) as u16 as u64; fl_r = ((r12 as u16)) as u16 as u64; if ((((fl_l as u16)) & ((fl_r as u16))) != 0) { bb = 64; continue; } bb = 63; continue; }
            63 => { rax = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; fl_l = (((rax) & (r13))) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; bb = 64; continue; }
            64 => { rdx = ((rdx).wrapping_add(0x8u64)) as u64; r9 = rax; r8 = 0; fl_l = 0; fl_r = 0; rcx = r15.wrapping_add(0x20); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 65; continue; }
            65 => { rcx = (crate::simd_ops::ru64(r15)) as u64; rcx = ((rcx).wrapping_add(0x20u64)) as u64; crate::simd_ops::wu64(r15, rcx); fl_l = rcx; fl_r = (crate::simd_ops::ru64(r15.wrapping_add(0x10u64))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 38; continue; } bb = 66; continue; }
            66 => { r13 = 0; fl_l = 0; fl_r = 0; bb = 67; continue; }
            67 => { xmm0 = 0u128; crate::simd_ops::wu128(rbp.wrapping_sub(0x41u64), (xmm0) as u128); r15 = r13; crate::simd_ops::wu64(rbp.wrapping_sub(0x41u64), r13); crate::simd_ops::wu64(rbp.wrapping_sub(0x39u64), (0x0u64) as u64); r14 = ((r13 as u32)) as u32 as u64; rax = ((rdi as u32)) as u32 as u64; rdi = ((rsi.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x6fu64), rdi); rax = rdi; rax = ((rax).wrapping_sub(rsi)) as u64; rax = ((((rax) as i64).wrapping_shr(3u32) as u64)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 69; continue; } bb = 68; continue; }
            68 => { r14 = r13; fl_l = ((r13 as u8)) as u8 as u64; fl_r = ((r13 as u8)) as u8 as u64; r14 = (if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { rax } else { r14 }) as u64; rcx = ((r14.wrapping_mul(8)) as u64) as u64; rdx = 0x8; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r15 = rax; crate::simd_ops::wu64(rbp.wrapping_sub(0x41u64), rax); crate::simd_ops::wu32(rbp.wrapping_sub(0x39u64), ((r13 as u32)) as u32); crate::simd_ops::wu32(rbp.wrapping_sub(0x35u64), ((r14 as u32)) as u32); bb = 69; continue; }
            69 => { rbx = rsi; rcx = ((r13 as u32)) as u32 as u64; fl_l = rsi; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 77; continue; } bb = 70; continue; }
            70 => { rdi = ((r13 as u32)) as u32 as u64; rdx = ((r13 as u32)) as u32 as u64; r13 = (crate::simd_ops::ru64(rbp.wrapping_sub(0x41u64))) as u64; rsi = (crate::simd_ops::ru64(rbp.wrapping_add(0x6fu64))) as u64; bb = 71; continue; }
            71 => { r15 = (crate::simd_ops::ru64(rbx)) as u64; rax = ((rcx as u32)) as u32 as u64; r12 = ((rax.wrapping_mul(8)) as u64) as u64; r12 = ((r12).wrapping_add(r13)) as u64; rax = ((r14 as u32)) as u32 as u64; rcx = ((rax.wrapping_mul(8)) as u64) as u64; rcx = ((rcx).wrapping_add(r13)) as u64; fl_l = r12; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 75; continue; } bb = 72; continue; }
            72 => { r12 = ((r12).wrapping_sub(r13)) as u64; rdi = r12; rdi = ((((rdi) as i64).wrapping_shr(3u32) as u64)) as u64; rax = rdi.wrapping_add(0x1); rcx = ((rcx).wrapping_sub(r13)) as u64; rcx = ((((rcx) as i64).wrapping_shr(3u32) as u64)) as u64; r14 = rcx; r14 = ((r14).wrapping_shr(1u32)) as u64; r14 = ((r14).wrapping_add(rcx)) as u64; fl_l = r14; fl_r = rax; r14 = (if (((fl_l as u64)) < ((fl_r as u64))) { rax } else { r14 }) as u64; rcx = ((r14.wrapping_mul(8)) as u64) as u64; rdx = 0x8; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rbp.wrapping_add(0x6fu64), rax); r8 = r12; rdx = r13; rcx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = r13; fl_r = r13; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 74; continue; } bb = 73; continue; }
            73 => { rcx = r13; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 74; continue; }
            74 => { r13 = (crate::simd_ops::ru64(rbp.wrapping_add(0x6fu64))) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x41u64), r13); crate::simd_ops::wu32(rbp.wrapping_sub(0x35u64), ((r14 as u32)) as u32); rdx = ((rdi as u32)) as u32 as u64; bb = 75; continue; }
            75 => { rcx = ((rdi as u32)) as u32 as u64; rax = (crate::simd_ops::ru64(r15.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(r13.wrapping_add(rcx.wrapping_mul(8)), rax); rcx = ((rdx.wrapping_add(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_sub(0x39u64), ((rcx as u32)) as u32); rbx = ((rbx).wrapping_add(0x8u64)) as u64; rdi = ((rcx as u32)) as u32 as u64; rdx = ((rcx as u32)) as u32 as u64; fl_l = rbx; fl_r = rsi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 71; continue; } bb = 76; continue; }
            76 => { rsi = (crate::simd_ops::ru64(rbp.wrapping_sub(0x31u64))) as u64; r13 = 0; fl_l = 0; fl_r = 0; r15 = (crate::simd_ops::ru64(rbp.wrapping_sub(0x41u64))) as u64; bb = 77; continue; }
            77 => { rbx = r15; rax = ((rcx as u32)) as u32 as u64; rdi = ((r15.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; fl_l = r15; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 97; continue; } bb = 78; continue; }
            78 => { rsi = (crate::simd_ops::ru64(rbp.wrapping_add(0x67u64))) as u64; bb = 79; continue; }
            79 => { r10 = (crate::simd_ops::ru64(rsi)) as u64; rcx = (crate::simd_ops::ru64(r10.wrapping_add(0x8u64))) as u64; rax = rcx; rdx = r10; fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0x19u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 87; continue; } bb = 80; continue; }
            80 => { r9 = (crate::simd_ops::ru64(rbx)) as u64; bb = 81; continue; }
            81 => { r8 = (crate::simd_ops::ru64(rax.wrapping_add(0x20u64))) as u64; fl_l = r8; fl_r = r9; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 83; continue; } bb = 82; continue; }
            82 => { rax = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; bb = 86; continue; }
            83 => { fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(0x19u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 85; continue; } bb = 84; continue; }
            84 => { fl_l = r9; fl_r = r8; rdx = (if (((fl_l as u64)) < ((fl_r as u64))) { rax } else { rdx }) as u64; bb = 85; continue; }
            85 => { r10 = rax; rax = (crate::simd_ops::ru64(rax)) as u64; bb = 86; continue; }
            86 => { fl_l = (crate::simd_ops::ru8(rax.wrapping_add(0x19u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 81; continue; } bb = 87; continue; }
            87 => { fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(0x19u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 89; continue; } bb = 88; continue; }
            88 => { rcx = (crate::simd_ops::ru64(rdx)) as u64; bb = 89; continue; }
            89 => { fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0x19u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 95; continue; } bb = 90; continue; }
            90 => { rax = (crate::simd_ops::ru64(rbx)) as u64; bb = 91; continue; }
            91 => { fl_l = rax; fl_r = (crate::simd_ops::ru64(rcx.wrapping_add(0x20u64))) as u64; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 93; continue; } bb = 92; continue; }
            92 => { rdx = rcx; rcx = (crate::simd_ops::ru64(rcx)) as u64; bb = 94; continue; }
            93 => { rcx = (crate::simd_ops::ru64(rcx.wrapping_add(0x10u64))) as u64; bb = 94; continue; }
            94 => { fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0x19u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 91; continue; } bb = 95; continue; }
            95 => { crate::simd_ops::wu64(rbp.wrapping_sub(0x11u64), r10); crate::simd_ops::wu64(rbp.wrapping_sub(0x9u64), rdx); xmm0 = (crate::simd_ops::ru128(rbp.wrapping_sub(0x11u64))) as u128; crate::simd_ops::wu128(rbp.wrapping_sub(0x1u64), (xmm0) as u128); rdx = rbp.wrapping_sub(0x1); rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P8, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = ((rbx).wrapping_add(0x8u64)) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 79; continue; } bb = 96; continue; }
            96 => { rsi = (crate::simd_ops::ru64(rbp.wrapping_sub(0x31u64))) as u64; bb = 97; continue; }
            97 => { rdi = (crate::simd_ops::ru64(rbp.wrapping_sub(0x21u64))) as u64; rbx = rdi; rax = (crate::simd_ops::ru32(rbp.wrapping_sub(0x19u64))) as u32 as u64; r14 = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; r14 = ((r14).wrapping_shl(4u32)) as u64; r14 = ((r14).wrapping_add(rdi)) as u64; fl_l = rdi; fl_r = r14; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 112; continue; } bb = 98; continue; }
            98 => { rdx = (0x249249249249249u64) as u64; bb = 99; continue; }
            99 => { r8 = (crate::simd_ops::ru64(rbp.wrapping_add(0x67u64))) as u64; rdi = (crate::simd_ops::ru64(r8)) as u64; rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x1u64), rax); crate::simd_ops::wu32(rbp.wrapping_add(0x7u64), ((r13 as u32)) as u32); rcx = rdi; fl_l = (crate::simd_ops::ru8(rax.wrapping_add(0x19u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 106; continue; } bb = 100; continue; }
            100 => { rdx = (crate::simd_ops::ru64(rbx)) as u64; bb = 101; continue; }
            101 => { crate::simd_ops::wu64(rbp.wrapping_sub(0x1u64), rax); fl_l = (crate::simd_ops::ru64(rax.wrapping_add(0x20u64))) as u64; fl_r = rdx; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 103; continue; } bb = 102; continue; }
            102 => { crate::simd_ops::wu32(rbp.wrapping_add(0x7u64), ((r13 as u32)) as u32); rax = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; bb = 104; continue; }
            103 => { crate::simd_ops::wu32(rbp.wrapping_add(0x7u64), (0x1u32) as u32); rcx = rax; rax = (crate::simd_ops::ru64(rax)) as u64; bb = 104; continue; }
            104 => { fl_l = (crate::simd_ops::ru8(rax.wrapping_add(0x19u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 101; continue; } bb = 105; continue; }
            105 => { rdx = (0x249249249249249u64) as u64; bb = 106; continue; }
            106 => { fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0x19u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 108; continue; } bb = 107; continue; }
            107 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x20u64))) as u64; fl_l = (crate::simd_ops::ru64(rbx)) as u64; fl_r = rax; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 110; continue; } bb = 108; continue; }
            108 => { fl_l = (crate::simd_ops::ru64(r8.wrapping_add(0x8u64))) as u64; fl_r = rdx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 123; continue; } bb = 109; continue; }
            109 => { crate::simd_ops::wu64(rbp.wrapping_sub(0x11u64), r8); crate::simd_ops::wu64(rbp.wrapping_sub(0x9u64), r13); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r8 = rax; rax = (crate::simd_ops::ru64(rbx)) as u64; crate::simd_ops::wu64(r8.wrapping_add(0x20u64), rax); crate::simd_ops::lk_and_u32(r8.wrapping_add(0x28u64), (0xff000008u32) as u32); crate::simd_ops::lk_or_u32(r8.wrapping_add(0x28u64), (0x8u32) as u32); crate::simd_ops::wu8(r8.wrapping_add(0x2bu64), (0x8u8) as u8); crate::simd_ops::wu32(r8.wrapping_add(0x2cu64), ((r13 as u32)) as u32); crate::simd_ops::wu8(r8.wrapping_add(0x30u64), (0x0u8) as u8); crate::simd_ops::wu64(r8, rdi); crate::simd_ops::wu64(r8.wrapping_add(0x8u64), rdi); crate::simd_ops::wu64(r8.wrapping_add(0x10u64), rdi); crate::simd_ops::wu16(r8.wrapping_add(0x18u64), (0x0u16) as u16); crate::simd_ops::wu64(rbp.wrapping_sub(0x9u64), r13); xmm0 = (crate::simd_ops::ru128(rbp.wrapping_sub(0x1u64))) as u128; crate::simd_ops::wu128(rbp.wrapping_sub(0x1u64), (xmm0) as u128); rdx = rbp.wrapping_sub(0x1); rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x67u64))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rax; bb = 110; continue; }
            110 => { rcx = ((rcx).wrapping_add(0x28u64)) as u64; rdx = rbx.wrapping_add(0x8); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P9, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = ((rbx).wrapping_add(0x50u64)) as u64; fl_l = rbx; fl_r = r14; rdx = (0x249249249249249u64) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 99; continue; } bb = 111; continue; }
            111 => { rdi = (crate::simd_ops::ru64(rbp.wrapping_sub(0x21u64))) as u64; bb = 112; continue; }
            112 => { fl_l = r15; fl_r = r15; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 114; continue; } bb = 113; continue; }
            113 => { rcx = r15; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 114; continue; }
            114 => { fl_l = rdi; fl_r = r14; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 117; continue; } bb = 115; continue; }
            115 => { rbx = rdi; bb = 116; continue; }
            116 => { rcx = rbx.wrapping_add(0x8); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P10, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = ((rbx).wrapping_add(0x50u64)) as u64; fl_l = rbx; fl_r = r14; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 116; continue; } bb = 117; continue; }
            117 => { fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 119; continue; } bb = 118; continue; }
            118 => { rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 119; continue; }
            119 => { fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 121; continue; } bb = 120; continue; }
            120 => { rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 121; continue; }
            121 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0xf8u64))) as u64; rsp = ((rsp).wrapping_add(0xa0u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            122 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P11, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 123; continue; }
            123 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P11, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_69732<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x40u64)) as u64;
    r8 = (crate::simd_ops::ru64(P0)) as u64;
    rdi = rdx;
    rdx = rcx;
    crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), (0x1u32) as u32);
    rbx = rcx;
    crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), (0x1u32) as u32);
    rcx = rdi;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rcx = (crate::simd_ops::ru64(rdi.wrapping_add(P2))) as u64;
    rdx = rbx;
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P3))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp);
    rcx = (crate::simd_ops::ru64(rdi.wrapping_add(P2))) as u64;
    rdx = ((rbx.wrapping_add(P4)) as u64) as u64;
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P4))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp);
    rdx = (crate::simd_ops::ru32(rdi.wrapping_add(P5))) as u32 as u64;
    rax = (crate::simd_ops::ru64(rdi)) as u64;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64;
    rcx = ((rdx.wrapping_add(rdx.wrapping_mul(2))) as u64) as u64;
    rcx = ((rcx).wrapping_add(rcx)) as u64;
    rcx = (crate::simd_ops::ru32(rax.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(P6))) as u32 as u64;
    rax = ((rdx.wrapping_add(P7)) as u32) as u32 as u64;
    crate::simd_ops::wu32(rdi.wrapping_add(P5), ((rax as u32)) as u32);
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu32(rdi.wrapping_add(P8), ((rcx as u32)) as u32);
    rsp = ((rsp).wrapping_add(0x40u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_69810<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = rdx; rsi = rcx; rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x8u64))) as u32 as u64; rcx = (((rax) as i64).wrapping_mul((0x4cu64) as i64) as u64) as u64; rax = (0x6bca1af286bca1bu64) as u64; let prod = (rax as i64 as i128).wrapping_mul((rcx) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; rbx = rdx; rbx = ((((rbx) as i64).wrapping_shr(1u32) as u64)) as u64; rax = rbx; rax = ((rax).wrapping_shr(63u32)) as u64; rbx = ((rbx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru32(rsi.wrapping_add(0xcu64))) as u32 as u64; fl_l = rdi; fl_r = rax; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 5; continue; } bb = 1; continue; }
            1 => { rdx = ((rax as u32)) as u32 as u64; rdx = ((rdx).wrapping_shr(1u32)) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; fl_l = rdx; fl_r = rdi; rdx = (if (((fl_l as u64)) < ((fl_r as u64))) { rdi } else { rdx }) as u64; rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rdi; fl_l = rcx; fl_r = rbx; rcx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 13; continue; } bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru32(rsi.wrapping_add(0x8u64))) as u32 as u64; rax = (((rax) as i64).wrapping_mul((0x4cu64) as i64) as u64) as u64; fl_l = ((rax).wrapping_add(crate::simd_ops::ru64(rsi))) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; bb = 3; continue; }
            3 => { xmm0 = 0u128; ymm0_hi = 0; rdx = 0; fl_l = 0; fl_r = 0; let y_addr: u64 = rax; crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); let y_addr: u64 = rax.wrapping_add(0x20u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); crate::simd_ops::wu64(rax.wrapping_add(0x40u64), rdx); crate::simd_ops::wu32(rax.wrapping_add(0x48u64), ((rdx as u32)) as u32); crate::simd_ops::wu64(rax, (0x8u64) as u64); crate::simd_ops::wu8(rax.wrapping_add(0x3u64), (0x8u8) as u8); crate::simd_ops::wu8(rax.wrapping_add(0x8u64), ((rdx as u8)) as u8); rax = rax.wrapping_add(0x4c); fl_l = rcx; fl_r = (0x1u64) as u64; rcx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } bb = 4; continue; }
            4 => { bb = 13; continue; }
            5 => { fl_l = rdi; fl_r = rbx; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 10; continue; } bb = 6; continue; }
            6 => { rdx = rdi; fl_l = rdx; fl_r = rbx; rdx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 13; continue; } bb = 7; continue; }
            7 => { fl_l = ((rcx).wrapping_add(crate::simd_ops::ru64(rsi))) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; bb = 8; continue; }
            8 => { xmm0 = 0u128; ymm0_hi = 0; rax = 0; fl_l = 0; fl_r = 0; let y_addr: u64 = rcx; crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); let y_addr: u64 = rcx.wrapping_add(0x20u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); crate::simd_ops::wu64(rcx.wrapping_add(0x40u64), rax); crate::simd_ops::wu32(rcx.wrapping_add(0x48u64), ((rax as u32)) as u32); crate::simd_ops::wu64(rcx, (0x8u64) as u64); crate::simd_ops::wu8(rcx.wrapping_add(0x3u64), (0x8u8) as u8); crate::simd_ops::wu8(rcx.wrapping_add(0x8u64), ((rax as u8)) as u8); rcx = rcx.wrapping_add(0x4c); fl_l = rdx; fl_r = (0x1u64) as u64; rdx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 8; continue; } bb = 9; continue; }
            9 => { bb = 13; continue; }
            10 => { if (((fl_l as u64)) == ((fl_r as u64))) { bb = 14; continue; } bb = 11; continue; }
            11 => { rax = (crate::simd_ops::ru64(rsi)) as u64; rbp = ((rax.wrapping_add(rcx)) as u64) as u64; rbx = (((rdi) as i64).wrapping_mul((0x4cu64) as i64) as u64) as u64; rbx = ((rbx).wrapping_add(rax)) as u64; fl_l = rbx; fl_r = rbp; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 13; continue; } bb = 12; continue; }
            12 => { rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = ((rbx).wrapping_add(0x4cu64)) as u64; fl_l = rbx; fl_r = rbp; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 12; continue; } bb = 13; continue; }
            13 => { crate::simd_ops::wu32(rsi.wrapping_add(0x8u64), ((rdi as u32)) as u32); bb = 14; continue; }
            14 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_69838<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x8u64))) as u32 as u64; rdi = rdx; r15 = rcx; rbp = r8; r10 = (0x2aaaaaaaaaaaaaabu64) as u64; rcx = rbp; rcx = ((rcx).wrapping_sub(rdi)) as u64; rdx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = r10; rbx = (crate::simd_ops::ru64(r15)) as u64; r9 = ((rdx.wrapping_mul(8)) as u64) as u64; let prod = (rax as i64 as i128).wrapping_mul((r9) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; rax = r10; r12 = rdx; let prod = (rax as i64 as i128).wrapping_mul((rcx) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; r12 = ((((r12) as i64).wrapping_shr(2u32) as u64)) as u64; r8 = r12; r14 = rdx; r14 = ((((r14) as i64).wrapping_shr(2u32) as u64)) as u64; rax = r14; r8 = ((r8).wrapping_shr(63u32)) as u64; rax = ((rax).wrapping_shr(63u32)) as u64; r12 = ((r12).wrapping_add(r8)) as u64; r14 = ((r14).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru32(r15.wrapping_add(0xcu64))) as u32 as u64; fl_l = r14; fl_r = rax; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 8; continue; } bb = 1; continue; }
            1 => { rcx = ((r9.wrapping_add(rbx)) as u64) as u64; rsi = 0; fl_l = 0; fl_r = 0; fl_l = rbx; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 4; continue; } bb = 2; continue; }
            2 => { bb = 3; continue; }
            3 => { rax = rsi; let tmp_xchg: u64 = crate::simd_ops::atomic_xchg_u64(rbx.wrapping_add(0x8u64), rax); rax = (tmp_xchg) as u64; rbx = ((rbx).wrapping_add(0x18u64)) as u64; fl_l = rbx; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru32(r15.wrapping_add(0xcu64))) as u32 as u64; rcx = r15; rdx = ((rax as u32)) as u32 as u64; crate::simd_ops::wu32(r15.wrapping_add(0x8u64), ((rsi as u32)) as u32); rdx = ((rdx).wrapping_shr(1u32)) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; fl_l = rdx; fl_r = r14; rdx = (if (((fl_l as u64)) < ((fl_r as u64))) { r14 } else { rdx }) as u64; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(r15)) as u64; fl_l = rdi; fl_r = rbp; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 7; continue; } bb = 5; continue; }
            5 => { bb = 6; continue; }
            6 => { rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rbx, rsi); crate::simd_ops::wu64(rbx.wrapping_add(0x8u64), rax); rdx = rdi; crate::simd_ops::wu32(rbx.wrapping_add(0x10u64), ((rax as u32)) as u32); rcx = rbx; crate::simd_ops::wu8(rbx.wrapping_add(0x14u64), ((rax as u8)) as u8); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rbx = ((rbx).wrapping_add(0x18u64)) as u64; rdi = ((rdi).wrapping_add(0x18u64)) as u64; fl_l = rdi; fl_r = rbp; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 6; continue; } bb = 7; continue; }
            7 => { crate::simd_ops::wu32(r15.wrapping_add(0x8u64), ((r14 as u32)) as u32); bb = 17; continue; }
            8 => { fl_l = r14; fl_r = r12; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 18; continue; } bb = 9; continue; }
            9 => { crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), r13); rax = ((r12.wrapping_add(r12.wrapping_mul(2))) as u64) as u64; r13 = ((rdi.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; fl_l = r12; fl_r = r12; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 13; continue; } bb = 10; continue; }
            10 => { rdx = rdi; rcx = rbx; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rbx = ((rbx).wrapping_add(0x18u64)) as u64; rsi = r12.wrapping_sub(0x1); fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 13; continue; } bb = 11; continue; }
            11 => { bb = 12; continue; }
            12 => { rdi = ((rdi).wrapping_add(0x18u64)) as u64; rcx = rbx; rdx = rdi; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rbx = ((rbx).wrapping_add(0x18u64)) as u64; fl_l = rsi; fl_r = (0x1u64) as u64; rsi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 12; continue; } bb = 13; continue; }
            13 => { rbp = r14; fl_l = rbp; fl_r = r12; rbp = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 16; continue; } bb = 14; continue; }
            14 => { r13 = ((r13).wrapping_sub(rbx)) as u64; rdi = rbx.wrapping_add(0x10); rsi = 0; fl_l = 0; fl_r = 0; bb = 15; continue; }
            15 => { rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rdi.wrapping_sub(0x10u64), rsi); crate::simd_ops::wu64(rdi.wrapping_sub(0x8u64), rax); rcx = rdi.wrapping_sub(0x10); crate::simd_ops::wu32(rdi, ((rax as u32)) as u32); rdx = r13.wrapping_sub(0x10); rdx = ((rdx).wrapping_add(rdi)) as u64; crate::simd_ops::wu8(rdi.wrapping_add(0x4u64), ((rax as u8)) as u8); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rdi = ((rdi).wrapping_add(0x18u64)) as u64; fl_l = rbp; fl_r = (0x1u64) as u64; rbp = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 15; continue; } bb = 16; continue; }
            16 => { r13 = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; crate::simd_ops::wu32(r15.wrapping_add(0x8u64), ((r14 as u32)) as u32); bb = 17; continue; }
            17 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            18 => { fl_l = rdi; fl_r = rbp; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 21; continue; } bb = 19; continue; }
            19 => { bb = 20; continue; }
            20 => { rdx = rdi; rcx = rbx; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rbx = ((rbx).wrapping_add(0x18u64)) as u64; rdi = ((rdi).wrapping_add(0x18u64)) as u64; fl_l = rdi; fl_r = rbp; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 20; continue; } bb = 21; continue; }
            21 => { rax = (crate::simd_ops::ru32(r15.wrapping_add(0x8u64))) as u32 as u64; rdx = (crate::simd_ops::ru64(r15)) as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; r8 = ((rdx.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; rax = ((r14.wrapping_add(r14.wrapping_mul(2))) as u64) as u64; rcx = ((rdx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; fl_l = rcx; fl_r = r8; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 7; continue; } bb = 22; continue; }
            22 => { rsi = 0; fl_l = 0; fl_r = 0; bb = 23; continue; }
            23 => { rax = rsi; let tmp_xchg: u64 = crate::simd_ops::atomic_xchg_u64(rcx.wrapping_add(0x8u64), rax); rax = (tmp_xchg) as u64; rcx = ((rcx).wrapping_add(0x18u64)) as u64; fl_l = rcx; fl_r = r8; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 23; continue; } bb = 24; continue; }
            24 => { crate::simd_ops::wu32(r15.wrapping_add(0x8u64), ((r14 as u32)) as u32); bb = 17; continue; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_70013<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x40u64)) as u64;
    r8 = (crate::simd_ops::ru64(P0)) as u64;
    rdi = rdx;
    rsi = 0;
    fl_l = 0; fl_r = 0;
    rdx = rcx;
    rbx = rcx;
    crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((rsi as u32)) as u32);
    rcx = rdi;
    crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rsi as u32)) as u32);
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rdx = rdi;
    rcx = rbx;
    rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp);
    crate::simd_ops::wu64(rbx.wrapping_add(P3), rsi);
    crate::simd_ops::wu32(rbx.wrapping_add(P4), ((rsi as u32)) as u32);
    rdx = (crate::simd_ops::ru32(rdi.wrapping_add(P3))) as u32 as u64;
    rax = (crate::simd_ops::ru64(rdi)) as u64;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64;
    rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64;
    rcx = ((rdx.wrapping_add(rdx.wrapping_mul(2))) as u64) as u64;
    rcx = ((rcx).wrapping_add(rcx)) as u64;
    rcx = (crate::simd_ops::ru32(rax.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(P5))) as u32 as u64;
    rax = ((rdx.wrapping_add(P6)) as u32) as u32 as u64;
    crate::simd_ops::wu32(rdi.wrapping_add(P3), ((rax as u32)) as u32);
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu32(rdi.wrapping_add(P7), ((rcx as u32)) as u32);
    rsp = ((rsp).wrapping_add(0x40u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_70014<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x40u64)) as u64;
    r9 = (crate::simd_ops::ru32(P0)) as u32 as u64;
    rdi = rdx;
    r8 = (crate::simd_ops::ru64(P1)) as u64;
    rax = 0;
    fl_l = 0; fl_r = 0;
    rdx = rcx;
    crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((rax as u32)) as u32);
    rbx = rcx;
    crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rax as u32)) as u32);
    rcx = rdi;
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    rdx = rdi;
    rcx = rbx;
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    r8 = (crate::simd_ops::ru64(rdi.wrapping_add(P4))) as u64;
    xmm0 = ((crate::simd_ops::ru64(rbx.wrapping_add(P5))) as u64) as u128;
    rcx = (crate::simd_ops::ru64(r8.wrapping_add(P6))) as u64;
    crate::simd_ops::wu64(rcx, ((xmm0 as u64)) as u64);
    rax = (crate::simd_ops::ru32(rbx.wrapping_add(P7))) as u32 as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(P5), ((rax as u32)) as u32);
    rcx = rdi;
    crate::simd_ops::lk_add_u64(r8.wrapping_add(P6), (0xcu64) as u64);
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P8, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64;
    rax = 0;
    fl_l = 0; fl_r = 0;
    rsp = ((rsp).wrapping_add(0x40u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_70062<const P0: u64, const P1: u64, const P2: u64, const P3: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx);
    crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x30u64)) as u64;
    rbx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    rsi = rdx;
    rax = (crate::simd_ops::ru32(rcx.wrapping_add(P1))) as u32 as u64;
    rdi = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64;
    rdi = ((rdi).wrapping_shl(5u32)) as u64;
    rdi = ((rdi).wrapping_add(rbx)) as u64;
    fl_l = rbx; fl_r = rdi;
    if (((fl_l as u64)) == ((fl_r as u64))) { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rax = 0; fl_l = 0; fl_r = 0; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rbp);
    rbp = 0;
    fl_l = 0; fl_r = 0;
    loop { crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rbp); r9 = P2; r8 = 0x7; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbp); rdx = rbx; rcx = rsi; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rbx = ((rbx).wrapping_add(0x60u64)) as u64; fl_l = rbx; fl_r = rdi; if !((((fl_l as u64)) != ((fl_r as u64)))) { break; } }
    rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64;
    rax = 0;
    fl_l = 0; fl_r = 0;
    rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64;
    rsp = ((rsp).wrapping_add(0x30u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_70232<const P0: u64, const P1: u64, const P2: u64, const P3: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx);
    crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x30u64)) as u64;
    rbx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    rsi = rdx;
    rax = (crate::simd_ops::ru32(rcx.wrapping_add(P1))) as u32 as u64;
    rdi = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64;
    rdi = ((rdi).wrapping_shl(5u32)) as u64;
    rdi = ((rdi).wrapping_add(rbx)) as u64;
    fl_l = rbx; fl_r = rdi;
    if (((fl_l as u64)) == ((fl_r as u64))) { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rax = 0; fl_l = 0; fl_r = 0; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rbp);
    rbp = 0;
    fl_l = 0; fl_r = 0;
    loop { crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rbp); r9 = P2; r8 = 0x1; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbp); rdx = rbx; rcx = rsi; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rbx = ((rbx).wrapping_add(0x60u64)) as u64; fl_l = rbx; fl_r = rdi; if !((((fl_l as u64)) != ((fl_r as u64)))) { break; } }
    rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64;
    rax = 0;
    fl_l = 0; fl_r = 0;
    rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64;
    rsp = ((rsp).wrapping_add(0x30u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_70397<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64, const P9: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x40u64)) as u64;
    r9 = (crate::simd_ops::ru32(P0)) as u32 as u64;
    rdi = rdx;
    r8 = (crate::simd_ops::ru64(P1)) as u64;
    rdx = rcx;
    rbx = rcx;
    crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), (0x1u32) as u32);
    rcx = rdi;
    crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), (0x1u32) as u32);
    rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp);
    rdx = rdi;
    rcx = rbx;
    rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp);
    rcx = ((rbx.wrapping_add(P4)) as u64) as u64;
    rdx = rdi;
    rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp);
    rcx = (crate::simd_ops::ru64(rdi.wrapping_add(P6))) as u64;
    rdx = ((rbx.wrapping_add(P7)) as u64) as u64;
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P8))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp);
    rcx = rdi;
    rax = crate::simd_ops::call_stk(P9, rax, rcx, rdx, r8, r9, rsp);
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64;
    rax = 0;
    fl_l = 0; fl_r = 0;
    rsp = ((rsp).wrapping_add(0x40u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_70565<const P0: u64, const P1: u64, const P2: u64, const P3: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx);
    crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x30u64)) as u64;
    rbx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    rsi = rdx;
    rax = (crate::simd_ops::ru32(rcx.wrapping_add(P1))) as u32 as u64;
    rdi = ((rbx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64;
    fl_l = rbx; fl_r = rdi;
    if (((fl_l as u64)) == ((fl_r as u64))) { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rax = 0; fl_l = 0; fl_r = 0; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rbp);
    rbp = 0;
    fl_l = 0; fl_r = 0;
    loop { crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rbp); r9 = P2; r8 = 0x2; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbp); rdx = rbx; rcx = rsi; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rbx = ((rbx).wrapping_add(0x8u64)) as u64; fl_l = rbx; fl_r = rdi; if !((((fl_l as u64)) != ((fl_r as u64)))) { break; } }
    rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64;
    rax = 0;
    fl_l = 0; fl_r = 0;
    rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64;
    rsp = ((rsp).wrapping_add(0x30u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_70728<const P0: u64, const P1: u64, const P2: u64, const P3: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx);
    crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x30u64)) as u64;
    rbx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    rsi = rdx;
    rax = (crate::simd_ops::ru32(rcx.wrapping_add(P1))) as u32 as u64;
    rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64;
    rdi = ((rbx.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64;
    fl_l = rbx; fl_r = rdi;
    if (((fl_l as u64)) == ((fl_r as u64))) { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rax = 0; fl_l = 0; fl_r = 0; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rbp);
    rbp = 0;
    fl_l = 0; fl_r = 0;
    loop { crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rbp); r9 = P2; r8 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbp); rdx = rbx; rcx = rsi; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rbx = ((rbx).wrapping_add(0x18u64)) as u64; fl_l = rbx; fl_r = rdi; if !((((fl_l as u64)) != ((fl_r as u64)))) { break; } }
    rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64;
    rax = 0;
    fl_l = 0; fl_r = 0;
    rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64;
    rsp = ((rsp).wrapping_add(0x30u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_70783<const P0: u64, const P1: u64, const P2: u64, const P3: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx);
    crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x30u64)) as u64;
    rbx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    rsi = rdx;
    rax = (crate::simd_ops::ru32(rcx.wrapping_add(P1))) as u32 as u64;
    rdi = (((rax) as i64).wrapping_mul((0x88u64) as i64) as u64) as u64;
    rdi = ((rdi).wrapping_add(rbx)) as u64;
    fl_l = rbx; fl_r = rdi;
    if (((fl_l as u64)) == ((fl_r as u64))) { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rax = 0; fl_l = 0; fl_r = 0; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rbp);
    rbp = 0;
    fl_l = 0; fl_r = 0;
    loop { crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rbp); r9 = P2; r8 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbp); rdx = rbx; rcx = rsi; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rbx = ((rbx).wrapping_add(0x88u64)) as u64; fl_l = rbx; fl_r = rdi; if !((((fl_l as u64)) != ((fl_r as u64)))) { break; } }
    rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64;
    rax = 0;
    fl_l = 0; fl_r = 0;
    rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64;
    rsp = ((rsp).wrapping_add(0x30u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_70790<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64, const P9: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu8(rsp.wrapping_add(0x18u64), ((r8 as u8)) as u8); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; r9 = (crate::simd_ops::ru32(P0)) as u32 as u64; rsi = rdx; r8 = (crate::simd_ops::ru64(P1)) as u64; rax = 0; fl_l = 0; fl_r = 0; rdx = rcx; crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((rax as u32)) as u32); rbx = rcx; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rax as u32)) as u32); rcx = rsi; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rdx = rsi; rcx = rbx; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru32(rbx.wrapping_add(P4))) as u32 as u64; rdx = rsp.wrapping_add(0x60); rcx = (crate::simd_ops::ru64(rsi.wrapping_add(P5))) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x60u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P6))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru32(rsp.wrapping_add(0x60u64))) as u32 as u64; fl_r = 0x0; if ((((fl_l as u32)) as i32) <= (((fl_r as u32)) as i32)) { bb = 5; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(P4))) as u32 as u64; rbx = (crate::simd_ops::ru64(rbx.wrapping_add(P7))) as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 5; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rdi); rdi = ((rax as u32)) as u32 as u64; bb = 3; continue; }
            3 => { rdx = rsi; rcx = rbx; rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); rbx = ((rbx).wrapping_add(0x8u64)) as u64; fl_l = rdi; fl_r = (0x1u64) as u64; rdi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } bb = 4; continue; }
            4 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; bb = 5; continue; }
            5 => { rcx = rsi; rax = crate::simd_ops::call_stk(P9, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rax = 0; fl_l = 0; fl_r = 0; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_70791<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64, const P9: u64, const P10: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu8(rsp.wrapping_add(0x18u64), ((r8 as u8)) as u8); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; r8 = (crate::simd_ops::ru64(P0)) as u64; rsi = rdx; rax = 0; fl_l = 0; fl_r = 0; rdx = rcx; rbx = rcx; crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((rax as u32)) as u32); rcx = rsi; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rax as u32)) as u32); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rdx = rsi; rcx = rbx; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru32(rbx.wrapping_add(P3))) as u32 as u64; rdx = rsp.wrapping_add(0x60); rcx = (crate::simd_ops::ru64(rsi.wrapping_add(P4))) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x60u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P5))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru32(rsp.wrapping_add(0x60u64))) as u32 as u64; fl_r = 0x0; if ((((fl_l as u32)) as i32) <= (((fl_r as u32)) as i32)) { bb = 5; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(P3))) as u32 as u64; rbx = (crate::simd_ops::ru64(rbx.wrapping_add(P6))) as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 5; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rdi); rdi = ((rax as u32)) as u32 as u64; bb = 3; continue; }
            3 => { rdx = rsi; rcx = rbx; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); rbx = ((rbx).wrapping_add(0x8u64)) as u64; fl_l = rdi; fl_r = (0x1u64) as u64; rdi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } bb = 4; continue; }
            4 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; bb = 5; continue; }
            5 => { rdx = (crate::simd_ops::ru32(rsi.wrapping_add(P6))) as u32 as u64; rax = (crate::simd_ops::ru64(rsi)) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rcx = ((rdx.wrapping_add(rdx.wrapping_mul(2))) as u64) as u64; rcx = ((rcx).wrapping_add(rcx)) as u64; rcx = (crate::simd_ops::ru32(rax.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(P8))) as u32 as u64; rax = ((rdx.wrapping_add(P9)) as u32) as u32 as u64; crate::simd_ops::wu32(rsi.wrapping_add(P6), ((rax as u32)) as u32); rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rsi.wrapping_add(P10), ((rcx as u32)) as u32); rsp = ((rsp).wrapping_add(0x40u64)) as u64; rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_70792<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64, const P9: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; r9 = (crate::simd_ops::ru32(P0)) as u32 as u64; rsi = rdx; r8 = (crate::simd_ops::ru64(P1)) as u64; rax = 0; fl_l = 0; fl_r = 0; rdx = rcx; crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((rax as u32)) as u32); rbx = rcx; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rax as u32)) as u32); rcx = rsi; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rdx = rsi; rcx = rbx; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rsi.wrapping_add(P4))) as u64; rdx = (crate::simd_ops::ru32(rbx.wrapping_add(P5))) as u32 as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(P6))) as u64; crate::simd_ops::wu32(rax, ((rdx as u32)) as u32); crate::simd_ops::lk_add_u64(rcx.wrapping_add(P6), (0x4u64) as u64); fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 5; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(P5))) as u32 as u64; rbx = (crate::simd_ops::ru64(rbx.wrapping_add(P7))) as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 5; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rdi); rdi = ((rax as u32)) as u32 as u64; bb = 3; continue; }
            3 => { rdx = rsi; rcx = rbx; rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); rbx = ((rbx).wrapping_add(0x8u64)) as u64; fl_l = rdi; fl_r = (0x1u64) as u64; rdi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } bb = 4; continue; }
            4 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; bb = 5; continue; }
            5 => { rcx = rsi; rax = crate::simd_ops::call_stk(P9, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rax = 0; fl_l = 0; fl_r = 0; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_70858<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64, const P9: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x40u64)) as u64;
    r9 = (crate::simd_ops::ru32(P0)) as u32 as u64;
    rdi = rdx;
    r8 = (crate::simd_ops::ru64(P1)) as u64;
    rax = 0;
    fl_l = 0; fl_r = 0;
    rdx = rcx;
    crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((rax as u32)) as u32);
    rbx = rcx;
    crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rax as u32)) as u32);
    rcx = rdi;
    rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp);
    rdx = rdi;
    rcx = rbx;
    rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp);
    rcx = (crate::simd_ops::ru64(rdi.wrapping_add(P4))) as u64;
    rdx = ((rbx.wrapping_add(P5)) as u64) as u64;
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P6))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp);
    rcx = (crate::simd_ops::ru64(rdi.wrapping_add(P4))) as u64;
    rdx = ((rbx.wrapping_add(P7)) as u64) as u64;
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P8))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp);
    rcx = rdi;
    rax = crate::simd_ops::call_stk(P9, rax, rcx, rdx, r8, r9, rsp);
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64;
    rax = 0;
    fl_l = 0; fl_r = 0;
    rsp = ((rsp).wrapping_add(0x40u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_70894<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64, const P9: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x40u64)) as u64;
    r9 = (crate::simd_ops::ru32(P0)) as u32 as u64;
    rdi = rdx;
    r8 = (crate::simd_ops::ru64(P1)) as u64;
    rax = 0;
    fl_l = 0; fl_r = 0;
    rdx = rcx;
    crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((rax as u32)) as u32);
    rbx = rcx;
    crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rax as u32)) as u32);
    rcx = rdi;
    rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp);
    rdx = rdi;
    rcx = rbx;
    rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp);
    rcx = (crate::simd_ops::ru64(rdi.wrapping_add(P4))) as u64;
    rdx = ((rbx.wrapping_add(P5)) as u64) as u64;
    rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp);
    rdx = (crate::simd_ops::ru64(rdi.wrapping_add(P4))) as u64;
    rax = (crate::simd_ops::ru32(rbx.wrapping_add(P7))) as u32 as u64;
    rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P8))) as u64;
    crate::simd_ops::wu32(rcx, ((rax as u32)) as u32);
    rcx = rdi;
    crate::simd_ops::lk_add_u64(rdx.wrapping_add(P8), (0x4u64) as u64);
    rax = crate::simd_ops::call_stk(P9, rax, rcx, rdx, r8, r9, rsp);
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64;
    rax = 0;
    fl_l = 0; fl_r = 0;
    rsp = ((rsp).wrapping_add(0x40u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_71031<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64, const P9: u64, const P10: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; r9 = (crate::simd_ops::ru32(P0)) as u32 as u64; rdi = rdx; r8 = (crate::simd_ops::ru64(P1)) as u64; rbp = 0; fl_l = 0; fl_r = 0; rdx = rcx; crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((rbp as u32)) as u32); rsi = rcx; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rbp as u32)) as u32); rcx = rdi; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru32(rdi.wrapping_add(P3))) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 2; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rbx); rdx = rsi; rbx = (crate::simd_ops::ru64(rdi.wrapping_add(P4))) as u64; rcx = rbx; rax = (crate::simd_ops::ru64(rbx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P5))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = ((rsi.wrapping_add(P6)) as u64) as u64; rcx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P5))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = ((rsi.wrapping_add(P7)) as u64) as u64; rcx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P5))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rax = ((rdi.wrapping_add(P8)) as u64) as u64; bb = 4; continue; }
            2 => { rax = ((rdi.wrapping_add(P8)) as u64) as u64; fl_l = (crate::simd_ops::ru8(rax)) as u8 as u64; fl_r = ((rbp as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 4; continue; } bb = 3; continue; }
            3 => { crate::simd_ops::wu64(rsi, rbp); crate::simd_ops::wu32(rsi.wrapping_add(P7), ((rbp as u32)) as u32); bb = 4; continue; }
            4 => { fl_l = (crate::simd_ops::ru32(rdi.wrapping_add(P3))) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 6; continue; } bb = 5; continue; }
            5 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(P4))) as u64; rdx = ((rsi.wrapping_add(P9)) as u64) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P5))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 8; continue; }
            6 => { fl_l = (crate::simd_ops::ru8(rax)) as u8 as u64; fl_r = ((rbp as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 8; continue; } bb = 7; continue; }
            7 => { crate::simd_ops::wu32(rsi.wrapping_add(P9), ((rbp as u32)) as u32); bb = 8; continue; }
            8 => { rcx = rdi; rax = crate::simd_ops::call_stk(P10, rax, rcx, rdx, r8, r9, rsp); rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rax = 0; fl_l = 0; fl_r = 0; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_71136<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rdx; rsi = rcx; rax = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rax = (0x6666666666666667u64) as u64; let prod = (rax as i64 as i128).wrapping_mul((rcx) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; rdi = rdx; rdi = ((((rdi) as i64).wrapping_shr(5u32) as u64)) as u64; rax = rdi; rax = ((rax).wrapping_shr(63u32)) as u64; rdi = ((rdi).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru32(rsi.wrapping_add(P1))) as u32 as u64; fl_l = rbx; fl_r = rax; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rdx = ((rax as u32)) as u32 as u64; rdx = ((rdx).wrapping_shr(1u32)) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; fl_l = rdx; fl_r = rbx; rdx = (if (((fl_l as u64)) < ((fl_r as u64))) { rbx } else { rdx }) as u64; rcx = rsi; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rdx = rbx; rdx = ((rdx).wrapping_sub(rdi)) as u64; rax = (crate::simd_ops::ru32(rsi.wrapping_add(P0))) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rsi))) as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu32(rsi.wrapping_add(P0), ((rbx as u32)) as u32); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            2 => { fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rdx = rbx; rdx = ((rdx).wrapping_sub(rdi)) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rsi))) as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu32(rsi.wrapping_add(P0), ((rbx as u32)) as u32); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            4 => { if (((fl_l as u64)) == ((fl_r as u64))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru64(rsi)) as u64; r8 = ((rax.wrapping_add(rcx)) as u64) as u64; rdx = ((rbx.wrapping_add(rbx.wrapping_mul(4))) as u64) as u64; rdx = ((rdx).wrapping_shl(4u32)) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu32(rsi.wrapping_add(P0), ((rbx as u32)) as u32); bb = 6; continue; }
            6 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_71150<const P0: u64, const P1: u64, const P2: u64, const P3: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rdx; rdi = rcx; rsi = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; rcx = ((rsi as u32)) as u32 as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rax = (crate::simd_ops::ru32(rdi.wrapping_add(P1))) as u32 as u64; fl_l = rdx; fl_r = rax; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 5; continue; } bb = 1; continue; }
            1 => { rdx = ((rax as u32)) as u32 as u64; rdx = ((rdx).wrapping_shr(1u32)) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; fl_l = rdx; fl_r = rbx; rdx = (if (((fl_l as u64)) < ((fl_r as u64))) { rbx } else { rdx }) as u64; rcx = rdi; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rdx = rbx; fl_l = rdx; fl_r = rsi; rdx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 11; continue; } bb = 2; continue; }
            2 => { rcx = (crate::simd_ops::ru32(rdi.wrapping_add(P0))) as u32 as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rdi))) as u64; rax = 0; fl_l = 0; fl_r = 0; bb = 3; continue; }
            3 => { crate::simd_ops::wu64(rcx.wrapping_add(P0), rax); crate::simd_ops::wu64(rcx, rax); rcx = ((rcx.wrapping_add(P3)) as u64) as u64; fl_l = rdx; fl_r = (0x1u64) as u64; rdx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } bb = 4; continue; }
            4 => { crate::simd_ops::wu32(rdi.wrapping_add(P0), ((rbx as u32)) as u32); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            5 => { fl_l = rbx; fl_r = rsi; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 10; continue; } bb = 6; continue; }
            6 => { fl_l = rdx; fl_r = rsi; rdx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 11; continue; } bb = 7; continue; }
            7 => { rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rdi))) as u64; rax = 0; fl_l = 0; fl_r = 0; bb = 8; continue; }
            8 => { crate::simd_ops::wu64(rcx.wrapping_add(P0), rax); crate::simd_ops::wu64(rcx, rax); rcx = ((rcx.wrapping_add(P3)) as u64) as u64; fl_l = rdx; fl_r = (0x1u64) as u64; rdx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 8; continue; } bb = 9; continue; }
            9 => { crate::simd_ops::wu32(rdi.wrapping_add(P0), ((rbx as u32)) as u32); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            10 => { if (((fl_l as u64)) == ((fl_r as u64))) { bb = 12; continue; } bb = 11; continue; }
            11 => { crate::simd_ops::wu32(rdi.wrapping_add(P0), ((rbx as u32)) as u32); bb = 12; continue; }
            12 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_71152<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rdx; rdi = rcx; rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x8u64))) as u32 as u64; r8 = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; r8 = ((r8.wrapping_mul(8)) as u64) as u64; rax = (0x2aaaaaaaaaaaaaabu64) as u64; let prod = (rax as i64 as i128).wrapping_mul((r8) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; rsi = rdx; rsi = ((((rsi) as i64).wrapping_shr(2u32) as u64)) as u64; rax = rsi; rax = ((rax).wrapping_shr(63u32)) as u64; rsi = ((rsi).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(0xcu64))) as u32 as u64; fl_l = rbx; fl_r = rax; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 5; continue; } bb = 1; continue; }
            1 => { rdx = ((rax as u32)) as u32 as u64; rdx = ((rdx).wrapping_shr(1u32)) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; fl_l = rdx; fl_r = rbx; rdx = (if (((fl_l as u64)) < ((fl_r as u64))) { rbx } else { rdx }) as u64; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); r8 = rbx; fl_l = r8; fl_r = rsi; r8 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 14; continue; } bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x8u64))) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = (crate::simd_ops::ru64(rdi)) as u64; rdx = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; rax = 0; fl_l = 0; fl_r = 0; bb = 3; continue; }
            3 => { crate::simd_ops::wu64(rdx, rax); crate::simd_ops::wu64(rdx.wrapping_add(0x8u64), rax); crate::simd_ops::wu32(rdx.wrapping_add(0x10u64), ((rax as u32)) as u32); crate::simd_ops::wu8(rdx.wrapping_add(0x14u64), ((rax as u8)) as u8); rdx = rdx.wrapping_add(0x18); fl_l = r8; fl_r = (0x1u64) as u64; r8 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } bb = 4; continue; }
            4 => { crate::simd_ops::wu32(rdi.wrapping_add(0x8u64), ((rbx as u32)) as u32); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            5 => { fl_l = rbx; fl_r = rsi; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 10; continue; } bb = 6; continue; }
            6 => { rdx = rbx; fl_l = rdx; fl_r = rsi; rdx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 14; continue; } bb = 7; continue; }
            7 => { rcx = (crate::simd_ops::ru64(rcx)) as u64; rcx = ((rcx).wrapping_add(r8)) as u64; rax = 0; fl_l = 0; fl_r = 0; bb = 8; continue; }
            8 => { crate::simd_ops::wu64(rcx, rax); crate::simd_ops::wu64(rcx.wrapping_add(0x8u64), rax); crate::simd_ops::wu32(rcx.wrapping_add(0x10u64), ((rax as u32)) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x14u64), ((rax as u8)) as u8); rcx = rcx.wrapping_add(0x18); fl_l = rdx; fl_r = (0x1u64) as u64; rdx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 8; continue; } bb = 9; continue; }
            9 => { crate::simd_ops::wu32(rdi.wrapping_add(0x8u64), ((rbx as u32)) as u32); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            10 => { if (((fl_l as u64)) == ((fl_r as u64))) { bb = 15; continue; } bb = 11; continue; }
            11 => { rcx = (crate::simd_ops::ru64(rcx)) as u64; r8 = ((r8).wrapping_add(rcx)) as u64; rax = ((rbx.wrapping_add(rbx.wrapping_mul(2))) as u64) as u64; rdx = ((rcx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; fl_l = rdx; fl_r = r8; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 14; continue; } bb = 12; continue; }
            12 => { rax = 0; fl_l = 0; fl_r = 0; bb = 13; continue; }
            13 => { rcx = rax; let tmp_xchg: u64 = crate::simd_ops::atomic_xchg_u64(rdx.wrapping_add(0x8u64), rcx); rcx = (tmp_xchg) as u64; rdx = ((rdx).wrapping_add(0x18u64)) as u64; fl_l = rdx; fl_r = r8; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 13; continue; } bb = 14; continue; }
            14 => { crate::simd_ops::wu32(rdi.wrapping_add(0x8u64), ((rbx as u32)) as u32); bb = 15; continue; }
            15 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}
