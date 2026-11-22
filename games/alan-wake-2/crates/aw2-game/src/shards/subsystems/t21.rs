#![allow(unused_imports, unused_variables, unused_mut, unused_assignments, non_snake_case, dead_code, clippy::all, unreachable_code, unused_parens)]

#[inline(never)]
pub unsafe extern "win64" fn tpl_31841<const P0: u64, const P1: u64, const P2: u64, const P3: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rcx;
    rdx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    fl_l = rdx; fl_r = rdx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rcx = (crate::simd_ops::ru64(P1)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64, rax, rcx, rdx, r8, r9, rsp); }
    rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P3))) as u64;
    fl_l = rdx; fl_r = rdx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rcx = (crate::simd_ops::ru64(P1)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64, rax, rcx, rdx, r8, r9, rsp); }
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_31930<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 19>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rbp = rsp.wrapping_sub(0x57); rsp = ((rsp).wrapping_sub(0x90u64)) as u64; rbx = rdx; rdi = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rbp.wrapping_add(0x2fu64), rdi); crate::simd_ops::wu32(rbp.wrapping_add(0x3fu64), (0x2u32) as u32); crate::simd_ops::wu64(rbp.wrapping_add(0x47u64), rdi); rax = P0; crate::simd_ops::wu64(rbp.wrapping_add(0x1fu64), rax); crate::simd_ops::wu64(rbp.wrapping_add(0x4fu64), rcx); xmm0 = 0u128; crate::simd_ops::wu128(rbp.wrapping_sub(0x1u64), (xmm0) as u128); crate::simd_ops::wu64(rbp.wrapping_add(0xfu64), rdi); crate::simd_ops::wu32(rbp.wrapping_add(0x17u64), ((rdi as u32)) as u32); rax = rbp.wrapping_sub(0x9); crate::simd_ops::wu64(rbp.wrapping_add(0x27u64), rax); crate::simd_ops::wu32(rbp.wrapping_sub(0x9u64), (0x28u32) as u32); crate::simd_ops::wu64(rbp.wrapping_add(0x37u64), rdi); rcx = rbp.wrapping_add(0x1f); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x3fu64))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rax as u32)) as u32); r9 = (crate::simd_ops::ru64(rbp.wrapping_add(0x47u64))) as u64; r8 = (crate::simd_ops::ru32(rbp.wrapping_add(0x3bu64))) as u32 as u64; rdx = rbx; rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x27u64))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x27u64))) as u64; rdx = (crate::simd_ops::ru64(rcx.wrapping_add(P3))) as u64; rax = (crate::simd_ops::ru32(rcx)) as u32 as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 1; continue; } fl_l = (((rax as u32)).wrapping_add(0x28u32)) as u32 as u64; fl_r = fl_l; rax = ((fl_l as u32)) as u32 as u64; crate::simd_ops::wu32(rcx, ((rax as u32)) as u32); rax = ((rdi as u32)) as u32 as u64; bb = 2; continue; }
            1 => { rax = ((rax).wrapping_add(rdx)) as u64; crate::simd_ops::wu64(rax, (0xffffffffffffffffu64) as u64); crate::simd_ops::wu64(rax.wrapping_add(P3), rdi); crate::simd_ops::wu32(rax.wrapping_add(P4), ((rdi as u32)) as u32); crate::simd_ops::wu64(rax.wrapping_add(P5), rdi); crate::simd_ops::wu64(rax.wrapping_add(P6), rdi); fl_l = (crate::simd_ops::lk_add_u32(rcx, (0x28u32) as u32)) as u32 as u64; fl_r = fl_l; bb = 2; continue; }
            2 => { crate::simd_ops::wu64(rbp.wrapping_add(0x2fu64), rax); crate::simd_ops::wu64(rbp.wrapping_add(0x37u64), rdi); rcx = rbp.wrapping_add(0x1f); rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x1fu64))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm((crate::simd_ops::ru64(rax.wrapping_add(P3))) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r11 = rsp.wrapping_add(0x90); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x10u64))) as u64; rdi = (crate::simd_ops::ru64(r11.wrapping_add(0x18u64))) as u64; rsp = r11; rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_31939<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64, const P9: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    crate::simd_ops::forward_incoming_to_outgoing();
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = (crate::simd_ops::ru32(rdx)) as u32 as u64; r9 = ((rcx as u32)) as u32 as u64; rax = (((rax as u32)).swap_bytes()) as u32 as u64; crate::simd_ops::wu32(rdx, ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64; rax = (((rax as u32)).swap_bytes()) as u32 as u64; crate::simd_ops::wu32(rdx.wrapping_add(P0), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdx.wrapping_add(P1))) as u32 as u64; rax = (((rax as u32)).swap_bytes()) as u32 as u64; crate::simd_ops::wu32(rdx.wrapping_add(P1), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdx.wrapping_add(P2))) as u32 as u64; rax = (((rax as u32)).swap_bytes()) as u32 as u64; crate::simd_ops::wu32(rdx.wrapping_add(P2), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdx.wrapping_add(P3))) as u32 as u64; rax = (((rax as u32)).swap_bytes()) as u32 as u64; crate::simd_ops::wu32(rdx.wrapping_add(P3), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdx.wrapping_add(P4))) as u32 as u64; rax = (((rax as u32)).swap_bytes()) as u32 as u64; crate::simd_ops::wu32(rdx.wrapping_add(P4), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdx.wrapping_add(P5))) as u32 as u64; rax = (((rax as u32)).swap_bytes()) as u32 as u64; crate::simd_ops::wu32(rdx.wrapping_add(P5), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdx.wrapping_add(P6))) as u32 as u64; rax = (((rax as u32)).swap_bytes()) as u32 as u64; crate::simd_ops::wu32(rdx.wrapping_add(P6), ((rax as u32)) as u32); rcx = (crate::simd_ops::ru32(rdx.wrapping_add(P7))) as u32 as u64; fl_l = ((r9 as u32)) as u32 as u64; fl_r = ((r9 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 1; continue; } rax = (crate::simd_ops::ru32(rdx.wrapping_add(P8))) as u32 as u64; rcx = (((rcx as u32)).swap_bytes()) as u32 as u64; crate::simd_ops::wu32(rdx.wrapping_add(P7), ((rcx as u32)) as u32); rax = (((rax as u32)).swap_bytes()) as u32 as u64; crate::simd_ops::wu32(rdx.wrapping_add(P8), ((rax as u32)) as u32); fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 4; continue; } rax = (rax as u32 as i32 as i64) as u64; rdx = ((rdx).wrapping_add(0x24u64)) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8(P9, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            1 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 2; continue; } r8 = 0; fl_l = 0; fl_r = 0; bb = 3; continue; }
            2 => { rax = (((crate::simd_ops::ru32(rdx.wrapping_add(P8))) as i32 as i64) as u64) as u64; r8 = ((rdx.wrapping_add(P8)) as u64) as u64; fl_l = ((r8).wrapping_add(rax)) as u64; fl_r = fl_l; r8 = ((fl_l as u64)) as u64; bb = 3; continue; }
            3 => { rax = ((rcx as u32)) as u32 as u64; rax = (((rax as u32)).swap_bytes()) as u32 as u64; crate::simd_ops::wu32(rdx.wrapping_add(P7), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdx.wrapping_add(P8))) as u32 as u64; rax = (((rax as u32)).swap_bytes()) as u32 as u64; crate::simd_ops::wu32(rdx.wrapping_add(P8), ((rax as u32)) as u32); fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 4; continue; } rdx = r8; rcx = ((r9 as u32)) as u32 as u64; crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8(P9, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            4 => { return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_32086<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 41>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rbp = rsp.wrapping_sub(0x30); rsp = ((rsp).wrapping_sub(0x130u64)) as u64; rbx = r9; r14 = r8; rdi = rdx; rdx = (crate::simd_ops::ru64(r9)) as u64; rsi = (crate::simd_ops::ru64(rdx.wrapping_add(0xa0u64))) as u64; fl_l = (crate::simd_ops::ru64(rsi.wrapping_add(0x120u64))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } crate::simd_ops::wu8(rdi, (0x0u8) as u8); bb = 15; continue; }
            1 => { r9 = r14; r8 = rsi; rcx = rbp.wrapping_add(0x10); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x70u64))) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x50u64), rax); crate::simd_ops::wu64(rbp.wrapping_sub(0x48u64), r14); crate::simd_ops::wu8(rbp.wrapping_sub(0x40u64), (0x0u8) as u8); xmm0 = (crate::simd_ops::ru128(rbx.wrapping_add(0x2cu64))) as u128; crate::simd_ops::wu128(rbp.wrapping_sub(0x3cu64), (xmm0) as u128); xmm1 = ((crate::simd_ops::ru64(rbx.wrapping_add(0x3cu64))) as u64) as u128; crate::simd_ops::wu64(rbp.wrapping_sub(0x2cu64), ((xmm1 as u64)) as u64); r14 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rbp.wrapping_sub(0x20u64), r14); rax = 0xffff; crate::simd_ops::wu16(rbp.wrapping_sub(0x18u64), ((rax as u16)) as u16); crate::simd_ops::wu64(rbp.wrapping_sub(0x10u64), r14); crate::simd_ops::wu16(rbp.wrapping_sub(0x8u64), ((rax as u16)) as u16); crate::simd_ops::wu64(rbp, r14); crate::simd_ops::wu16(rbp.wrapping_add(0x8u64), ((rax as u16)) as u16); crate::simd_ops::wu16(rbp.wrapping_sub(0x24u64), (0x101u16) as u16); crate::simd_ops::wu8(rbp.wrapping_sub(0x22u64), (0x1u8) as u8); rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x10u64))) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(0x78u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rcx); rax = rbp.wrapping_sub(0x50); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rax); rdx = (crate::simd_ops::ru64(rbp.wrapping_add(0x18u64))) as u64; rcx = rsp.wrapping_add(0x40); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r8 = rbx.wrapping_add(0x2c); rdx = (crate::simd_ops::ru64(rbp.wrapping_add(0x18u64))) as u64; rcx = rsp.wrapping_add(0x60); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rbp.wrapping_sub(0x68u64), r14); rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } rbx = ((r14 as u32)) as u32 as u64; rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x120u64))) as u64; fl_l = (crate::simd_ops::ru32(rax.wrapping_add(0x18u64))) as u32 as u64; fl_r = ((r14 as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 6; continue; } bb = 2; continue; }
            2 => { rcx = ((rbx as u32)) as u32 as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rax.wrapping_add(0x10u64)))) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x60u64), rax); rcx = ((crate::simd_ops::ru16(rcx.wrapping_add(0x8u64))) as u32) as u32 as u64; crate::simd_ops::wu16(rbp.wrapping_sub(0x58u64), ((rcx as u16)) as u16); rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rdx = (crate::simd_ops::ru32(rax.wrapping_add(0x38u64))) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rcx.wrapping_add(0x14u64))) as u32 as u64; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 5; continue; } fl_l = ((rdx as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rcx.wrapping_add(0x1cu64))) as u32 as u64; if ((((fl_l as u32)) as i32) > (((fl_r as u32)) as i32)) { bb = 5; continue; } rdx = (crate::simd_ops::ru32(rax.wrapping_add(0x3cu64))) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rcx.wrapping_add(0x18u64))) as u32 as u64; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 5; continue; } fl_l = ((rdx as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rcx.wrapping_add(0x20u64))) as u32 as u64; if ((((fl_l as u32)) as i32) > (((fl_r as u32)) as i32)) { bb = 5; continue; } rdx = rbp.wrapping_add(0x68); rcx = rsp.wrapping_add(0x40); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = (crate::simd_ops::ru8(rax)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 7; continue; } rcx = (crate::simd_ops::ru32(rsp.wrapping_add(0x48u64))) as u32 as u64; rcx = ((rcx).wrapping_add(rcx)) as u64; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rdx = (crate::simd_ops::ru64(rax.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x18u64))) as u64; rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x54u64))) as u32 as u64; rax = ((rax).wrapping_add(rax)) as u64; xmm0 = (crate::simd_ops::ru128(rbp.wrapping_sub(0x60u64))) as u128; crate::simd_ops::wu128(rdx.wrapping_add(rax.wrapping_mul(8)), (xmm0) as u128); rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x54u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rsp.wrapping_add(0x58u64))) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 3; continue; } fl_l = (((rax as u32)).wrapping_add(1)) as u32 as u64; rax = ((fl_l as u32)) as u32 as u64; bb = 4; continue; }
            3 => { rax = ((r14 as u32)) as u32 as u64; bb = 4; continue; }
            4 => { crate::simd_ops::wu32(rsp.wrapping_add(0x54u64), ((rax as u32)) as u32); rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } r9 = rsp.wrapping_add(0x20); r8 = rbp.wrapping_sub(0x60); rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rcx = rsp.wrapping_add(0x60); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 7; continue; } bb = 5; continue; }
            5 => { rbx = (((rbx as u32)).wrapping_add(1)) as u32 as u64; rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x120u64))) as u64; fl_l = ((rbx as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rax.wrapping_add(0x18u64))) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 2; continue; } bb = 6; continue; }
            6 => { rcx = rsp.wrapping_add(0x30); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 10; continue; } crate::simd_ops::wu8(rdi, (0x0u8) as u8); rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; bb = 9; continue; }
            7 => { rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; bb = 8; continue; }
            8 => { crate::simd_ops::wu8(rdi, (0x0u8) as u8); bb = 9; continue; }
            9 => { fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 13; continue; } rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x68u64))) as u32 as u64; fl_l = ((rax).wrapping_add(rax)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; crate::simd_ops::wu8(rcx.wrapping_add(rax.wrapping_mul(8)).wrapping_add(0x24u64), (0x0u8) as u8); bb = 13; continue; }
            10 => { fl_l = (crate::simd_ops::ru8(rbp.wrapping_sub(0x40u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 11; continue; } crate::simd_ops::wu8(rdi, (0x0u8) as u8); rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; bb = 9; continue; }
            11 => { crate::simd_ops::wu8(rdi, (0x1u8) as u8); rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; bb = 9; continue; }
            12 => { crate::simd_ops::wu8(rdi, (0x0u8) as u8); fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 14; continue; } rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x68u64))) as u32 as u64; fl_l = ((rax).wrapping_add(rax)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; crate::simd_ops::wu8(rdx.wrapping_add(rax.wrapping_mul(8)).wrapping_add(0x24u64), (0x0u8) as u8); bb = 13; continue; }
            13 => { rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; bb = 14; continue; }
            14 => { fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 15; continue; } rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x48u64))) as u32 as u64; fl_l = ((rax).wrapping_add(rax)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; crate::simd_ops::wu8(rcx.wrapping_add(rax.wrapping_mul(8)).wrapping_add(0x24u64), (0x0u8) as u8); bb = 15; continue; }
            15 => { rax = rdi; r11 = rsp.wrapping_add(0x130); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x20u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x28u64))) as u64; rsp = r11; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_32103<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    rcx = (crate::simd_ops::ru64(rcx)) as u64;
    fl_l = rcx; fl_r = rcx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64, rax, rcx, rdx, r8, r9, rsp); }
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_32111<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64, const P9: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; fl_r = 0x1; rdi = rdx; rbx = rcx; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 3; continue; } rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(P2))) as u32 as u64; fl_r = 0x7fffffff; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 1; continue; } crate::simd_ops::wu32(rbx.wrapping_add(P0), (0x2u32) as u32); crate::simd_ops::wu32(rbx.wrapping_add(P3), (0x2u32) as u32); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            1 => { fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 2; continue; } rdi = (crate::simd_ops::ru64(rcx.wrapping_add(P4))) as u64; bb = 2; continue; }
            2 => { xmm0 = ((crate::simd_ops::ru32(rbx.wrapping_add(P5))) as u32) as u128; r8 = ((rbx.wrapping_add(P6)) as u64) as u64; xmm1 = ((crate::simd_ops::ru32(rbx.wrapping_add(P7))) as u32) as u128; rdx = rsp.wrapping_add(0x30); rcx = ((rcx).wrapping_add(0x58u64)) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((xmm0 as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x34u64), ((xmm1 as u32)) as u32); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P8, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = rdi; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P9, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu32(rbx.wrapping_add(P3), (0x2u32) as u32); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            3 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; crate::simd_ops::wu32(rcx.wrapping_add(P3), (0x2u32) as u32); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_32140<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rdi = rdx; rsi = rcx; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 4; continue; } rcx = (crate::simd_ops::ru64(P1)) as u64; rdx = ((rbx.wrapping_add(P2)) as u32) as u32 as u64; r8 = 0; fl_l = 0; fl_r = 0; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(rax.wrapping_add(P3))) as u64, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rax); rbx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 1; continue; } crate::simd_ops::wu32(rax.wrapping_add(P4), (0x1u32) as u32); rax = P5; crate::simd_ops::wu64(rbx, rax); bb = 2; continue; }
            1 => { rbx = 0; fl_l = 0; fl_r = 0; bb = 2; continue; }
            2 => { rcx = (crate::simd_ops::ru64(rsi.wrapping_add(P0))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); bb = 3; continue; }
            3 => { crate::simd_ops::wu64(rsi.wrapping_add(P0), rbx); fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rcx = rbx; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rsi.wrapping_add(P0))) as u64; crate::simd_ops::wu64(rdi, rax); rax = rdi; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            5 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rax = rdi; crate::simd_ops::wu64(rdi, rbx); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_32166<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rdx; rsi = rcx; rdi = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; fl_l = rdx; fl_r = rdi; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 5; continue; } rbp = rdi; fl_l = rbp; fl_r = rdx; rbp = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rdi = ((rdi.wrapping_add(P1)) as u64) as u64; rdi = ((rax.wrapping_add(rdi.wrapping_mul(8))) as u64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 3; continue; } bb = 1; continue; }
            1 => { rcx = (crate::simd_ops::ru64(rdi)) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 2; continue; }
            2 => { rdi = ((rdi).wrapping_sub(0x8u64)) as u64; fl_l = rbp; fl_r = (0x1u64) as u64; rbp = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rsi.wrapping_add(P3))) as u64; rax = ((rax).wrapping_shr(1u32)) as u64; fl_l = rbx; fl_r = rax; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 4; continue; } r8 = rbx; rdx = rsi; rcx = rsi; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); bb = 4; continue; }
            4 => { crate::simd_ops::wu64(rsi.wrapping_add(P0), rbx); bb = 7; continue; }
            5 => { fl_l = rbx; fl_r = (crate::simd_ops::ru64(rcx.wrapping_add(P3))) as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 6; continue; } r8 = rbx; r8 = ((r8).wrapping_shr(2u32)) as u64; r8 = ((r8).wrapping_add(rbx)) as u64; rdx = rsi; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); bb = 6; continue; }
            6 => { crate::simd_ops::wu64(rsi.wrapping_add(P0), rbx); fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 7; continue; } rax = (crate::simd_ops::ru64(rsi)) as u64; rcx = ((rax.wrapping_add(rdi.wrapping_mul(8))) as u64) as u64; fl_l = rbx; fl_r = rdi; rbx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 7; continue; } rbx = ((rbx).wrapping_shl(3u32)) as u64; r8 = rbx; rdx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); bb = 7; continue; }
            7 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_32169<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 17>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x70u64)) as u64; fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(0x20u64))) as u32 as u64; fl_r = 0x0; rax = 0xffff; r15 = (crate::simd_ops::ru64(rcx.wrapping_add(0x68u64))) as u64; xmm0 = 0u128; crate::simd_ops::wu32(rsp.wrapping_add(0x60u64), ((xmm0 as u32)) as u32); rbx = rcx; crate::simd_ops::wu16(rsp.wrapping_add(0x58u64), ((rax as u16)) as u16); crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), (0x0u64) as u64); if (((fl_l as u32)) == ((fl_r as u32))) { bb = 9; continue; } bb = 1; continue; }
            1 => { rdx = rsp.wrapping_add(0x50); rcx = rbx.wrapping_add(0x10); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm2 = ((crate::simd_ops::ru32(rsp.wrapping_add(0x60u64))) as u32) as u128; r9 = rbx.wrapping_add(0x30); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; rdx = rsp.wrapping_add(0x50); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = r15; fl_r = r15; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } rdi = (crate::simd_ops::ru64(rbx.wrapping_add(0x68u64))) as u64; rcx = (crate::simd_ops::ru32(rdi.wrapping_add(0x10u64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rdi.wrapping_add(0x14u64))) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 2; continue; } r8 = (crate::simd_ops::ru32(rdi.wrapping_add(0x8u64))) as u32 as u64; rdx = (crate::simd_ops::ru64(rdi)) as u64; bb = 4; continue; }
            2 => { rdx = (crate::simd_ops::ru32(rdi.wrapping_add(0x8u64))) as u32 as u64; r8 = 0; fl_l = 0; fl_r = 0; rcx = (crate::simd_ops::ru64(rdi)) as u64; rax = ((rdx as u32)) as u32 as u64; rax = ((rax).wrapping_add(rax)) as u64; rsi = (crate::simd_ops::ru64(rcx.wrapping_add(rax.wrapping_mul(8)).wrapping_add(0x18u64))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } r8 = (crate::simd_ops::ru32(rdi.wrapping_add(0x10u64))) as u32 as u64; rdx = rsi; r8 = ((r8).wrapping_shl(4u32)) as u64; rcx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru64(P4)) as u64; rdx = rsi; rax = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm((crate::simd_ops::ru64(rax.wrapping_add(0x60u64))) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 3; continue; }
            3 => { r8 = (crate::simd_ops::ru32(rdi.wrapping_add(0x8u64))) as u32 as u64; rdx = (crate::simd_ops::ru64(rdi)) as u64; rcx = (crate::simd_ops::ru32(rdi.wrapping_add(0x10u64))) as u32 as u64; rax = r8.wrapping_add(0x2); rax = ((rax).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(rax.wrapping_mul(8)))) as u32 as u64; fl_l = (((rax as u32)).wrapping_shr(4u32)) as u32 as u64; rax = ((fl_l as u32)) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x14u64), ((rax as u32)) as u32); bb = 4; continue; }
            4 => { xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x50u64))) as u128; rcx = ((rcx).wrapping_add(rcx)) as u64; rax = ((r8 as u32)) as u32 as u64; rax = ((rax).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru64(rdx.wrapping_add(rax.wrapping_mul(8)).wrapping_add(0x18u64))) as u64; crate::simd_ops::wu128(rax.wrapping_add(rcx.wrapping_mul(8)), (xmm0) as u128); fl_l = (crate::simd_ops::lk_add_u32(rdi.wrapping_add(0x10u64), (1) as u32)) as u32 as u64; bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; xmm0 = ((crate::simd_ops::ru32(rax.wrapping_add(0x14u64))) as u32) as u128; fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rax.wrapping_add(0x50u64))) as u32 as u64; if !(f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { bb = 9; continue; } fl_l = (crate::simd_ops::ru8(rax.wrapping_add(0x54u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 6; continue; } xmm0 = (crate::simd_ops::ru128(rax.wrapping_add(0x58u64))) as u128; rdx = rsp.wrapping_add(0x20); rcx = rbx; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 10; continue; } bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; fl_l = (crate::simd_ops::ru8(rax.wrapping_add(0x55u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 7; continue; } xmm0 = (crate::simd_ops::ru128(rax.wrapping_add(0x68u64))) as u128; rdx = rsp.wrapping_add(0x30); rcx = rbx; crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm0) as u128); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 10; continue; } bb = 7; continue; }
            7 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; fl_l = (crate::simd_ops::ru8(rax.wrapping_add(0x56u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 8; continue; } xmm0 = (crate::simd_ops::ru128(rax.wrapping_add(0x78u64))) as u128; rdx = rsp.wrapping_add(0x40); rcx = rbx; crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm0) as u128); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 10; continue; } bb = 8; continue; }
            8 => { fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x20u64))) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 1; continue; } bb = 9; continue; }
            9 => { rax = 0; fl_l = 0; fl_r = 0; bb = 10; continue; }
            10 => { r11 = rsp.wrapping_add(0x70); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x20u64))) as u64; rbp = (crate::simd_ops::ru64(r11.wrapping_add(0x28u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x30u64))) as u64; rsp = r11; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            11 => { rax = 0x3; bb = 10; continue; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_32171<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 21>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x70u64)) as u64; r8 = (crate::simd_ops::ru32(rcx.wrapping_add(0x20u64))) as u32 as u64; r12 = 0; fl_l = 0; fl_r = 0; rax = 0xffff; r15 = (crate::simd_ops::ru64(rcx.wrapping_add(0x68u64))) as u64; rdi = rcx; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), r12); crate::simd_ops::wu16(rsp.wrapping_add(0x30u64), ((rax as u16)) as u16); fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(0x24u64))) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 9; continue; } bb = 1; continue; }
            1 => { rdx = (crate::simd_ops::ru32(rdi.wrapping_add(0x18u64))) as u32 as u64; rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x10u64))) as u64; rdx = ((rdx).wrapping_add(rdx)) as u64; rcx = ((r8 as u32)) as u32 as u64; rcx = ((rcx).wrapping_add(rcx)) as u64; fl_l = ((r8 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rdi.wrapping_add(0x28u64))) as u32 as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(rdx.wrapping_mul(8)).wrapping_add(0x18u64))) as u64; rdx = rsp.wrapping_add(0x28); xmm0 = (crate::simd_ops::ru128(rax.wrapping_add(rcx.wrapping_mul(8)))) as u128; rax = ((r8.wrapping_add(0x1u64)) as u32) as u32 as u64; rax = (if (((fl_l as u32)) == ((fl_r as u32))) { (r12 as u32) } else { (rax as u32) }) as u32 as u64; r8 = rdi.wrapping_add(0x30); crate::simd_ops::wu32(rdi.wrapping_add(0x20u64), ((rax as u32)) as u32); rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu128(rsp.wrapping_add(0x28u64), (xmm0) as u128); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = r15; fl_r = r15; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } rsi = (crate::simd_ops::ru64(rdi.wrapping_add(0x68u64))) as u64; rcx = (crate::simd_ops::ru32(rsi.wrapping_add(0x10u64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rsi.wrapping_add(0x14u64))) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 2; continue; } r8 = (crate::simd_ops::ru32(rsi.wrapping_add(0x8u64))) as u32 as u64; rdx = (crate::simd_ops::ru64(rsi)) as u64; bb = 4; continue; }
            2 => { rdx = (crate::simd_ops::ru32(rsi.wrapping_add(0x8u64))) as u32 as u64; r8 = 0; fl_l = 0; fl_r = 0; rcx = (crate::simd_ops::ru64(rsi)) as u64; rax = ((rdx as u32)) as u32 as u64; rax = ((rax).wrapping_add(rax)) as u64; rbp = (crate::simd_ops::ru64(rcx.wrapping_add(rax.wrapping_mul(8)).wrapping_add(0x18u64))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } fl_l = rbp; fl_r = rbp; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } r8 = (crate::simd_ops::ru32(rsi.wrapping_add(0x10u64))) as u32 as u64; rdx = rbp; r8 = ((r8).wrapping_shl(4u32)) as u64; rcx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru64(P3)) as u64; rdx = rbp; rax = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm((crate::simd_ops::ru64(rax.wrapping_add(0x60u64))) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 3; continue; }
            3 => { r8 = (crate::simd_ops::ru32(rsi.wrapping_add(0x8u64))) as u32 as u64; rdx = (crate::simd_ops::ru64(rsi)) as u64; rcx = (crate::simd_ops::ru32(rsi.wrapping_add(0x10u64))) as u32 as u64; rax = r8.wrapping_add(0x2); rax = ((rax).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(rax.wrapping_mul(8)))) as u32 as u64; fl_l = (((rax as u32)).wrapping_shr(4u32)) as u32 as u64; rax = ((fl_l as u32)) as u32 as u64; crate::simd_ops::wu32(rsi.wrapping_add(0x14u64), ((rax as u32)) as u32); bb = 4; continue; }
            4 => { xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x28u64))) as u128; rcx = ((rcx).wrapping_add(rcx)) as u64; rax = ((r8 as u32)) as u32 as u64; rax = ((rax).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru64(rdx.wrapping_add(rax.wrapping_mul(8)).wrapping_add(0x18u64))) as u64; crate::simd_ops::wu128(rax.wrapping_add(rcx.wrapping_mul(8)), (xmm0) as u128); fl_l = (crate::simd_ops::lk_add_u32(rsi.wrapping_add(0x10u64), (1) as u32)) as u32 as u64; bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; fl_l = (crate::simd_ops::ru8(rax.wrapping_add(0x10u64))) as u8 as u64; fl_r = ((r12 as u8)) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 9; continue; } fl_l = (crate::simd_ops::ru8(rax.wrapping_add(0x2cu64))) as u8 as u64; fl_r = ((r12 as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 6; continue; } xmm0 = (crate::simd_ops::ru128(rax.wrapping_add(0x30u64))) as u128; rdx = (crate::simd_ops::ru64(rdi)) as u64; r9 = rsp.wrapping_add(0xb0); r8 = rsp.wrapping_add(0x38); rcx = rdi.wrapping_add(0x30); crate::simd_ops::wu128(rsp.wrapping_add(0x38u64), (xmm0) as u128); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 10; continue; } fl_l = (crate::simd_ops::ru8(rsp.wrapping_add(0xb0u64))) as u8 as u64; fl_r = ((r12 as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 6; continue; } rdx = rsp.wrapping_add(0xc8); rcx = rdi.wrapping_add(0x10); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = (crate::simd_ops::ru8(rax)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 12; continue; } rdx = (crate::simd_ops::ru32(rdi.wrapping_add(0x18u64))) as u32 as u64; rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x10u64))) as u64; rdx = ((rdx).wrapping_add(rdx)) as u64; rcx = (crate::simd_ops::ru32(rdi.wrapping_add(0x24u64))) as u32 as u64; xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x38u64))) as u128; rcx = ((rcx).wrapping_add(rcx)) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(rdx.wrapping_mul(8)).wrapping_add(0x18u64))) as u64; crate::simd_ops::wu128(rax.wrapping_add(rcx.wrapping_mul(8)), (xmm0) as u128); rcx = (crate::simd_ops::ru32(rdi.wrapping_add(0x24u64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rdi.wrapping_add(0x28u64))) as u32 as u64; rax = ((rcx.wrapping_add(0x1u64)) as u32) as u32 as u64; rax = (if (((fl_l as u32)) == ((fl_r as u32))) { (r12 as u32) } else { (rax as u32) }) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x24u64), ((rax as u32)) as u32); bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; fl_l = (crate::simd_ops::ru8(rax.wrapping_add(0x2du64))) as u8 as u64; fl_r = ((r12 as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 7; continue; } xmm0 = (crate::simd_ops::ru128(rax.wrapping_add(0x40u64))) as u128; rdx = (crate::simd_ops::ru64(rdi)) as u64; r9 = rsp.wrapping_add(0xb8); r8 = rsp.wrapping_add(0x48); rcx = rdi.wrapping_add(0x30); crate::simd_ops::wu128(rsp.wrapping_add(0x48u64), (xmm0) as u128); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 10; continue; } fl_l = (crate::simd_ops::ru8(rsp.wrapping_add(0xb8u64))) as u8 as u64; fl_r = ((r12 as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 7; continue; } rdx = rsp.wrapping_add(0x20); rcx = rdi.wrapping_add(0x10); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = (crate::simd_ops::ru8(rax)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 12; continue; } rdx = (crate::simd_ops::ru32(rdi.wrapping_add(0x18u64))) as u32 as u64; rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x10u64))) as u64; rdx = ((rdx).wrapping_add(rdx)) as u64; rcx = (crate::simd_ops::ru32(rdi.wrapping_add(0x24u64))) as u32 as u64; xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x48u64))) as u128; rcx = ((rcx).wrapping_add(rcx)) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(rdx.wrapping_mul(8)).wrapping_add(0x18u64))) as u64; crate::simd_ops::wu128(rax.wrapping_add(rcx.wrapping_mul(8)), (xmm0) as u128); rcx = (crate::simd_ops::ru32(rdi.wrapping_add(0x24u64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rdi.wrapping_add(0x28u64))) as u32 as u64; rax = ((rcx.wrapping_add(0x1u64)) as u32) as u32 as u64; rax = (if (((fl_l as u32)) == ((fl_r as u32))) { (r12 as u32) } else { (rax as u32) }) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x24u64), ((rax as u32)) as u32); bb = 7; continue; }
            7 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; fl_l = (crate::simd_ops::ru8(rax.wrapping_add(0x2eu64))) as u8 as u64; fl_r = ((r12 as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 8; continue; } xmm0 = (crate::simd_ops::ru128(rax.wrapping_add(0x50u64))) as u128; rdx = (crate::simd_ops::ru64(rdi)) as u64; r9 = rsp.wrapping_add(0xc0); r8 = rsp.wrapping_add(0x58); rcx = rdi.wrapping_add(0x30); crate::simd_ops::wu128(rsp.wrapping_add(0x58u64), (xmm0) as u128); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 10; continue; } fl_l = (crate::simd_ops::ru8(rsp.wrapping_add(0xc0u64))) as u8 as u64; fl_r = ((r12 as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 8; continue; } rdx = rsp.wrapping_add(0x21); rcx = rdi.wrapping_add(0x10); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = (crate::simd_ops::ru8(rax)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 12; continue; } rdx = (crate::simd_ops::ru32(rdi.wrapping_add(0x18u64))) as u32 as u64; rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x10u64))) as u64; rdx = ((rdx).wrapping_add(rdx)) as u64; rcx = (crate::simd_ops::ru32(rdi.wrapping_add(0x24u64))) as u32 as u64; xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x58u64))) as u128; rcx = ((rcx).wrapping_add(rcx)) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(rdx.wrapping_mul(8)).wrapping_add(0x18u64))) as u64; crate::simd_ops::wu128(rax.wrapping_add(rcx.wrapping_mul(8)), (xmm0) as u128); rcx = (crate::simd_ops::ru32(rdi.wrapping_add(0x24u64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rdi.wrapping_add(0x28u64))) as u32 as u64; rax = ((rcx.wrapping_add(0x1u64)) as u32) as u32 as u64; rax = (if (((fl_l as u32)) == ((fl_r as u32))) { (r12 as u32) } else { (rax as u32) }) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x24u64), ((rax as u32)) as u32); bb = 8; continue; }
            8 => { r8 = (crate::simd_ops::ru32(rdi.wrapping_add(0x20u64))) as u32 as u64; fl_l = (crate::simd_ops::ru32(rdi.wrapping_add(0x24u64))) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 1; continue; } bb = 9; continue; }
            9 => { rax = 0; fl_l = 0; fl_r = 0; bb = 10; continue; }
            10 => { rsp = ((rsp).wrapping_add(0x70u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            11 => { rax = 0x3; rsp = ((rsp).wrapping_add(0x70u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            12 => { rax = 0x2; rsp = ((rsp).wrapping_add(0x70u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_32214<const P0: u64, const P1: u64, const P2: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rcx;
    rdi = rdx;
    rcx = (crate::simd_ops::ru64(rdx)) as u64;
    fl_l = rcx; fl_r = rcx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); }
    rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P1))) as u64;
    fl_l = rcx; fl_r = rcx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); }
    rax = (crate::simd_ops::ru64(rdi)) as u64;
    crate::simd_ops::wu64(rbx.wrapping_add(P1), rax);
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_32264<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64, const P9: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 17>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { r11 = rsp; crate::simd_ops::wu64(r11.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x80u64)) as u64; rbx = rcx; rcx = (crate::simd_ops::ru64(rcx)) as u64; fl_l = ((rdx as u8)) as u8 as u64; fl_r = ((rdx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 6; continue; } fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 8; continue; } rax = r11.wrapping_sub(0x58); crate::simd_ops::wu64(r11.wrapping_add(0x18u64), rax); rdi = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(r11.wrapping_sub(0x58u64), rdi); crate::simd_ops::wu64(r11.wrapping_sub(0x48u64), rdi); crate::simd_ops::wu64(r11.wrapping_sub(0x38u64), rdi); crate::simd_ops::wu64(r11.wrapping_sub(0x28u64), rdi); crate::simd_ops::wu64(r11.wrapping_sub(0x30u64), rdi); crate::simd_ops::wu32(rsp.wrapping_add(0x48u64), (0xa5ff00u32) as u32); crate::simd_ops::wu64(r11.wrapping_sub(0x50u64), rdi); xmm0 = 0u128; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128); crate::simd_ops::wu64(r11.wrapping_sub(0x20u64), rdi); crate::simd_ops::wu32(rsp.wrapping_add(0x70u64), (0x2u32) as u32); rdx = r11.wrapping_sub(0x68); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru64(P1)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; r8 = 0; fl_l = 0; fl_r = 0; rdx = 0x110; (rax, xmm0) = crate::simd_ops::call_stk_xmm((crate::simd_ops::ru64(rax.wrapping_add(0x48u64))) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rsp.wrapping_add(0xa0u64), rax); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 1; continue; } rcx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdi = rax; bb = 1; continue; }
            1 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 2; continue; }
            2 => { crate::simd_ops::wu64(rbx, rdi); r8 = rsp.wrapping_add(0x20); rdx = rsp.wrapping_add(0x98); rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; rcx = (crate::simd_ops::ru64(rbx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru64(rbx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } rcx = (crate::simd_ops::ru64(P1)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm((crate::simd_ops::ru64(rax.wrapping_add(0x60u64))) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 3; continue; }
            3 => { rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } rcx = (crate::simd_ops::ru64(P1)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm((crate::simd_ops::ru64(rax.wrapping_add(0x60u64))) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 4; continue; }
            4 => { rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 5; continue; }
            5 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x90u64))) as u64; rsp = ((rsp).wrapping_add(0x80u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            6 => { fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(0x18u64))) as u32 as u64; fl_r = 0xffffffff; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 8; continue; } (rax, xmm0) = crate::simd_ops::call_stk_xmm(P8, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru64(rbx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P9, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru64(rbx)) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 7; continue; }
            7 => { rdi = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rbx, rdi); bb = 8; continue; }
            8 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x90u64))) as u64; rsp = ((rsp).wrapping_add(0x80u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_32297<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    rax = (crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_32332<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64, const P9: u64, const P10: u64, const P11: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x30u64)) as u64;
    rbx = rcx;
    rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    fl_l = rcx; fl_r = rcx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); }
    rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P2))) as u64;
    fl_l = rcx; fl_r = rcx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); }
    rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P4))) as u64;
    rdx = 0;
    fl_l = 0; fl_r = 0;
    xmm0 = 0u128;
    crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), (0x0u32) as u32);
    rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x28u64))) as u32 as u64;
    xmm1 = 0u128;
    xmm0 = crate::simd_ops::vunpcklps(xmm0, xmm1);
    crate::simd_ops::wu64(rcx.wrapping_add(P5), ((xmm0 as u64)) as u64);
    crate::simd_ops::wu32(rcx.wrapping_add(P6), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru64(rbx.wrapping_add(P4))) as u64;
    crate::simd_ops::wu64(rbx.wrapping_add(P7), rdx);
    xmm0 = ((crate::simd_ops::ru64(rcx.wrapping_add(P8))) as u64) as u128;
    crate::simd_ops::wu64(rax.wrapping_add(P9), ((xmm0 as u64)) as u64);
    rax = (crate::simd_ops::ru64(rbx.wrapping_add(P4))) as u64;
    crate::simd_ops::wu8(rax.wrapping_add(P10), ((rdx as u8)) as u8);
    rax = (crate::simd_ops::ru64(rbx.wrapping_add(P4))) as u64;
    crate::simd_ops::wu32(rax.wrapping_add(P11), ((rdx as u32)) as u32);
    rsp = ((rsp).wrapping_add(0x30u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_32406<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    crate::simd_ops::forward_incoming_to_outgoing();
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    xmm0 = ((crate::simd_ops::ru64(rdx)) as u64) as u128;
    crate::simd_ops::wu64(rcx.wrapping_add(P0), ((xmm0 as u64)) as u64);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P1))) as u32 as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(P2), ((rax as u32)) as u32);
    crate::simd_ops::wu32(rcx.wrapping_add(P3), ((r8 as u32)) as u32);
    xmm0 = ((crate::simd_ops::ru64(r9)) as u64) as u128;
    crate::simd_ops::wu64(rcx.wrapping_add(P4), ((xmm0 as u64)) as u64);
    crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(P5, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::set_xmm_ret(c_x0); return c_rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_32423<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = ((rcx.wrapping_add(P0)) as u64) as u64; rdi = rcx; rcx = rbx; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 1; continue; } rax = (crate::simd_ops::ru64(rdi.wrapping_add(P2))) as u64; rcx = rbx; rdi = (crate::simd_ops::ru64(rax.wrapping_add(P3))) as u64; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = rdi; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            1 => { rcx = ((rax as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, P6, rcx, rsp); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_32425<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbp = rcx; rbx = ((rcx.wrapping_add(P0)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbx); rcx = rbx; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 4; continue; } rcx = 0; fl_l = 0; fl_r = 0; rsi = ((rcx as u32)) as u32 as u64; fl_l = (crate::simd_ops::ru32(rbp.wrapping_add(0x38u64))) as u32 as u64; fl_r = 0x1000; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 1; continue; } rax = (crate::simd_ops::ru64(rbp.wrapping_add(0xf0u64))) as u64; rsi = (crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64; bb = 1; continue; }
            1 => { rdi = (crate::simd_ops::ru64(rbp.wrapping_add(0x90u64))) as u64; rax = rbp.wrapping_add(0x88); rbp = ((rax.wrapping_add(P3)) as u64) as u64; fl_l = rax; fl_r = rax; rbp = (if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { rcx } else { rbp }) as u64; bb = 2; continue; }
            2 => { fl_l = rdi; fl_r = rbp; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 3; continue; } rax = (crate::simd_ops::ru64(rdi)) as u64; rcx = rdi; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(rax.wrapping_add(P4))) as u64, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rsi).wrapping_add(rax)) as u64; fl_r = fl_l; rsi = ((fl_l as u64)) as u64; rdi = (crate::simd_ops::ru64(rdi.wrapping_add(P5))) as u64; bb = 2; continue; }
            3 => { rcx = rbx; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); rax = rsi; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            4 => { rcx = ((rax as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, P8, rcx, rsp); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_32457(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = 0xffffffff;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_32490<const P0: u64, const P1: u64, const P2: u64, const P3: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rbx = rdx; rdi = rcx; rdx = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; fl_l = rbx; fl_r = (crate::simd_ops::ru64(rcx.wrapping_add(P2))) as u64; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 1; continue; } rax = ((rax).wrapping_shr(1u32)) as u64; fl_l = rbx; fl_r = rax; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 2; continue; } r8 = rbx; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rdi.wrapping_add(P2), rbx); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            1 => { fl_l = rbx; fl_r = rax; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 2; continue; } r8 = rbx; r8 = ((r8).wrapping_shr(2u32)) as u64; r8 = ((r8).wrapping_add(rbx)) as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rdi.wrapping_add(P2), rbx); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            2 => { crate::simd_ops::wu64(rcx.wrapping_add(P2), rbx); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_32518<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rdi = rdx;
    r9 = ((rcx.wrapping_add(P0)) as u64) as u64;
    rdx = ((rcx.wrapping_add(P1)) as u64) as u64;
    rbx = rcx;
    r8 = ((rcx.wrapping_add(P2)) as u64) as u64;
    rcx = rdi;
    rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp);
    r9 = ((rbx.wrapping_add(P4)) as u64) as u64;
    rcx = rdi;
    rdx = ((rbx.wrapping_add(P5)) as u64) as u64;
    r8 = ((rbx.wrapping_add(P6)) as u64) as u64;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return crate::simd_ops::tail_stk(P3, rax, rcx, rdx, r8, r9, rsp);
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_32519<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    r9 = 0x1;
    r8 = rcx;
    fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(P0))) as u8 as u64; fl_r = ((r9 as u8)) as u8 as u64;
    if !((((fl_l as u8)) == ((fl_r as u8)))) { rax = (crate::simd_ops::ru32(rcx.wrapping_add(P1))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0xffffffff; rdx = ((rax.wrapping_add(P2)) as u32) as u32 as u64; rdx = (if (((fl_l as u32)) == ((fl_r as u32))) { (r9 as u32) } else { (rdx as u32) }) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(P1), ((rdx as u32)) as u32); crate::simd_ops::wu8(rcx.wrapping_add(P0), ((r9 as u8)) as u8); }
    fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(P3))) as u8 as u64; fl_r = ((r9 as u8)) as u8 as u64;
    if !((((fl_l as u8)) == ((fl_r as u8)))) { rax = (crate::simd_ops::ru32(rcx.wrapping_add(P4))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0xffffffff; rcx = ((rax.wrapping_add(P2)) as u32) as u32 as u64; rcx = (if (((fl_l as u32)) == ((fl_r as u32))) { (r9 as u32) } else { (rcx as u32) }) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(P4), ((rcx as u32)) as u32); crate::simd_ops::wu8(r8.wrapping_add(P3), ((r9 as u8)) as u8); }
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_32520<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64, const P9: u64, const P10: u64, const P11: u64, const P12: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; r9 = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; rbx = rcx; fl_l = ((r9 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rdx)) as u32 as u64; rdi = rdx; r9 = (if ((((fl_l as u32)) as i32) >= (((fl_r as u32)) as i32)) { crate::simd_ops::ru32(rdx) } else { (r9 as u32) }) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(P0), ((r9 as u32)) as u32); r8 = (crate::simd_ops::ru32(rdx.wrapping_add(P1))) as u32 as u64; fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(P2))) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; r8 = (if ((((fl_l as u32)) as i32) >= (((fl_r as u32)) as i32)) { crate::simd_ops::ru32(rcx.wrapping_add(P2)) } else { (r8 as u32) }) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(P2), ((r8 as u32)) as u32); rdx = (crate::simd_ops::ru32(rcx.wrapping_add(P3))) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rdi.wrapping_add(P4))) as u32 as u64; rdx = (if ((((fl_l as u32)) as i32) >= (((fl_r as u32)) as i32)) { crate::simd_ops::ru32(rdi.wrapping_add(P4)) } else { (rdx as u32) }) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(P3), ((rdx as u32)) as u32); rcx = (crate::simd_ops::ru32(rdi.wrapping_add(P5))) as u32 as u64; fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(P6))) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; rcx = (if ((((fl_l as u32)) as i32) >= (((fl_r as u32)) as i32)) { crate::simd_ops::ru32(rbx.wrapping_add(P6)) } else { (rcx as u32) }) as u32 as u64; r8 = (((r8 as u32)).wrapping_sub((r9 as u32))) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(P6), ((rcx as u32)) as u32); r8 = (((r8 as u32)).wrapping_add(1)) as u32 as u64; rcx = (((rcx as u32)).wrapping_sub((rdx as u32))) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(P7), ((r8 as u32)) as u32); rcx = (((rcx as u32)).wrapping_add(1)) as u32 as u64; r8 = ((rbx.wrapping_add(P8)) as u64) as u64; crate::simd_ops::wu32(rbx.wrapping_add(P9), ((rcx as u32)) as u32); rdx = ((rbx.wrapping_add(P10)) as u64) as u64; rcx = rdi; crate::simd_ops::wu8(rbx.wrapping_add(P11), (0x1u8) as u8); r9 = rbx; rax = crate::simd_ops::call_stk(P12, rax, rcx, rdx, r8, r9, rsp); r9 = rbx.wrapping_add(0xc); rcx = rdi; rdx = rbx.wrapping_add(0x88); r8 = rbx.wrapping_add(0xa5); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(P12, rax, rcx, rdx, r8, r9, rsp); }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_32524<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = ((rcx.wrapping_add(P0)) as u64) as u64;
    rdi = rdx;
    r8 = (crate::simd_ops::ru64(rbx.wrapping_add(P1))) as u64;
    rdx = rbx;
    r8 = ((r8).wrapping_add(1)) as u64;
    rcx = rbx;
    rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp);
    rax = (crate::simd_ops::ru64(rbx)) as u64;
    rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P1))) as u64;
    rbx = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64;
    rax = (crate::simd_ops::ru64(rdi)) as u64;
    fl_l = rax; fl_r = rax;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rcx = rax; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rdi)) as u64; }
    crate::simd_ops::wu64(rbx.wrapping_add(P4), rax);
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_32568<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rdi = ((rdx as u32)) as u32 as u64;
    rbx = rcx;
    rax = P0;
    crate::simd_ops::wu64(rcx, rax);
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    fl_l = ((rdi as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64;
    if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { rax = rbx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    fl_l = ((rdi as u8)) as u8 as u64; fl_r = (0x4u8) as u8 as u64;
    if !(((((fl_l as u8)) & ((fl_r as u8))) != 0)) { rcx = (crate::simd_ops::ru64(P2)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = rbx; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(rax.wrapping_add(P3))) as u64, rax, rcx, rdx, r8, r9, rsp); rax = rbx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    rdx = 0x18;
    rcx = rbx;
    rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp);
    rax = rbx;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_32619<const P0: u64, const P1: u64, const P2: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rcx;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rcx = (crate::simd_ops::ru64(P1)) as u64;
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    rdx = (crate::simd_ops::ru64(rbx)) as u64;
    rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_32624<const P0: u64, const P1: u64, const P2: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = 0;
    fl_l = 0; fl_r = 0;
    rdi = rcx;
    fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; fl_r = ((rbx as u32)) as u32 as u64;
    if (((fl_l as u32)) <= ((fl_r as u32))) { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    loop { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(P1))) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(rbx.wrapping_mul(8)))) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rbx = (((rbx as u32)).wrapping_add(1)) as u32 as u64; fl_l = ((rbx as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rdi.wrapping_add(P0))) as u32 as u64; if !((((fl_l as u32)) < ((fl_r as u32)))) { break; } }
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_32636<const P0: u64, const P1: u64, const P2: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rcx;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rcx = ((rbx.wrapping_add(P1)) as u64) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return crate::simd_ops::tail_stk(P2, rax, rcx, rdx, r8, r9, rsp);
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_32674<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    crate::simd_ops::forward_incoming_to_outgoing();
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = (crate::simd_ops::ru32(r8)) as u32 as u64;
    rcx = ((rdx as u32)) as u32 as u64;
    rax = (((rax as u32)).swap_bytes()) as u32 as u64;
    rdx = ((r8.wrapping_add(P0)) as u64) as u64;
    crate::simd_ops::wu32(r8, ((rax as u32)) as u32);
    crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8(P1, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_32704<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = (crate::simd_ops::ru32(r8)) as u32 as u64;
    rax = (((rax as u32)).swap_bytes()) as u32 as u64;
    crate::simd_ops::wu32(r8, ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(r8.wrapping_add(P0))) as u32 as u64;
    rax = (((rax as u32)).swap_bytes()) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P0), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(r8.wrapping_add(P1))) as u32 as u64;
    rax = (((rax as u32)).swap_bytes()) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P1), ((rax as u32)) as u32);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_32881<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P1)) as u64, rax, rcx, rdx, r8, r9, rsp);
    fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64;
    rax = (rax & !0xffu64) | (((if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { 1u8 } else { 0u8 }) as u8 as u64));
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_32884<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64, rax, rcx, rdx, r8, r9, rsp);
    rax = (rax as u32 as i32 as i64) as u64;
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_33077<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    crate::simd_ops::forward_incoming_to_outgoing();
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    r8 = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64;
    r9 = rdx;
    rax = (crate::simd_ops::ru32(rdx)) as u32 as u64;
    r8 = ((((r8 as u32)) & (0xfu32))) as u32 as u64;
    r8 = ((r8).wrapping_shl(4u32)) as u64;
    rax = ((((rax as u32)) & (0xfu32))) as u32 as u64;
    r8 = (((r8) | (rax))) as u64;
    rdx = 0x3f;
    r8 = ((r8).wrapping_sub(1)) as u64;
    fl_l = r8; fl_r = rdx;
    rdx = (if (((fl_l as u64)) < ((fl_r as u64))) { r8 } else { rdx }) as u64;
    r8 = r9;
    crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8(P1, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_33124<const P0: u64, const P1: u64, const P2: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rsi = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    rdi = rdx;
    rbx = rcx;
    fl_l = rsi; fl_r = rsi;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rcx = rsi; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rdi)) as u64; rdx = rsi; rcx = rdi; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64, rax, rcx, rdx, r8, r9, rsp); }
    rax = (crate::simd_ops::ru64(rbx)) as u64;
    rdx = 0;
    fl_l = 0; fl_r = 0;
    rcx = rbx;
    rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(rax)) as u64, rax, rcx, rdx, r8, r9, rsp);
    rax = (crate::simd_ops::ru64(rdi)) as u64;
    rdx = rbx;
    rcx = rdi;
    rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64, rax, rcx, rdx, r8, r9, rsp);
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rax = 0x1;
    rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_33145<const P0: u64, const P1: u64, const P2: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = rdx; rbx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rdx, rbx); rsi = r8; crate::simd_ops::wu64(rdx.wrapping_add(P0), rbx); xmm0 = 0u128; crate::simd_ops::wu64(rdx.wrapping_add(P1), rbx); rcx = rdi; rdx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64; rdx = ((rdx).wrapping_sub(crate::simd_ops::ru64(r8))) as u64; r8 = rsp.wrapping_add(0x38); rdx = ((((rdx) as i64).wrapping_shr(2u32) as u64)) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x38u64), ((xmm0 as u32)) as u32); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r8 = (crate::simd_ops::ru64(rdi)) as u64; rax = (crate::simd_ops::ru64(rdi.wrapping_add(P0))) as u64; rax = ((rax).wrapping_sub(r8)) as u64; rax = ((((rax) as i64).wrapping_shr(2u32) as u64)) as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(rsi)) as u64; rdx = ((rbx as u32)) as u32 as u64; rbx = (((rbx as u32)).wrapping_add(1)) as u32 as u64; rcx = (crate::simd_ops::ru32(rax.wrapping_add(rdx.wrapping_mul(4)))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(rdx.wrapping_mul(4)), ((rcx as u32)) as u32); r8 = (crate::simd_ops::ru64(rdi)) as u64; rax = (crate::simd_ops::ru64(rdi.wrapping_add(P0))) as u64; rax = ((rax).wrapping_sub(r8)) as u64; rax = ((((rax) as i64).wrapping_shr(2u32) as u64)) as u64; fl_l = ((rbx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 1; continue; } bb = 2; continue; }
            2 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_33147<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = (crate::simd_ops::ru64(rcx)) as u64;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rcx = rax;
    rdx = rbx;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_33149<const P0: u64, const P1: u64, const P2: u64, const P3: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rdi = rcx;
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    fl_l = rax; fl_r = rax;
    if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    fl_l = (crate::simd_ops::lk_add_u32(rax, (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l;
    if (((fl_l as u32)) != 0) { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    rbx = (crate::simd_ops::ru64(rcx)) as u64;
    fl_l = rbx; fl_r = rbx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rcx = rbx; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rcx = rax; rdx = rbx; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); }
    rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp);
    rcx = rax;
    rdx = (crate::simd_ops::ru64(rdi.wrapping_add(P0))) as u64;
    rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp);
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_33151<const P0: u64, const P1: u64, const P2: u64, const P3: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rsi = rcx;
    rdi = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    rbx = (crate::simd_ops::ru64(rcx)) as u64;
    fl_l = rbx; fl_r = rdi;
    if (((fl_l as u64)) >= ((fl_r as u64))) { rbx = (crate::simd_ops::ru64(rsi)) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rcx = rax; rdx = rbx; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    loop { rcx = rbx; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rbx = ((rbx).wrapping_add(0x10u64)) as u64; fl_l = rbx; fl_r = rdi; if !((((fl_l as u64)) < ((fl_r as u64)))) { break; } }
    rbx = (crate::simd_ops::ru64(rsi)) as u64;
    rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp);
    rcx = rax;
    rdx = rbx;
    rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp);
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_33154<const P0: u64, const P1: u64, const P2: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(P0))) as u8 as u64; fl_r = (0x0u8) as u8 as u64;
    if !((((fl_l as u8)) == ((fl_r as u8)))) { rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
    rax = (rax & !0xffu64) | (((0u8) as u8 as u64));
    fl_l = 0; fl_r = 0;
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_33160<const P0: u64, const P1: u64, const P2: u64, const P3: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rcx;
    r8 = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    rdx = (crate::simd_ops::ru64(rcx)) as u64;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rbx = (crate::simd_ops::ru64(rbx)) as u64;
    rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp);
    rcx = rax;
    rdx = rbx;
    rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_33165<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    xmm0 = ((crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32) as u128;
    rbx = rcx;
    fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(P1)) as u32 as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(P2), ((xmm1 as u32)) as u32);
    if !((f32::from_bits((fl_l as u32)) != f32::from_bits((fl_r as u32)) && !f32::from_bits((fl_l as u32)).is_nan() && !f32::from_bits((fl_r as u32)).is_nan())) { xmm0 = ((crate::simd_ops::ru32(P3)) as u32) as u128; } else { xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm1 as u32)).to_bits() as u128); xmm1 = ((crate::simd_ops::ru32(P3)) as u32) as u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(crate::simd_ops::ru32(rcx))).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) / f32::from_bits(xmm0 as u32)).to_bits() as u128); xmm0 = ((crate::simd_ops::ru32(P4)) as u32) as u128; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); }
    rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P6))) as u64;
    xmm1 = 0u128;
    xmm1 = (xmm1 & !0xffff_ffff_ffff_ffffu128) | (((f32::from_bits(xmm0 as u32)) as f64).to_bits() as u128);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return crate::simd_ops::tail_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_33166<const P0: u64, const P1: u64, const P2: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    crate::simd_ops::forward_incoming_to_outgoing();
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    crate::simd_ops::wu32(rcx.wrapping_add(P0), ((xmm1 as u32)) as u32);
    rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64;
    xmm1 = (xmm1 & !0xffff_ffff_ffff_ffffu128) | (((f32::from_bits(xmm1 as u32)) as f64).to_bits() as u128);
    crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(P2, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::set_xmm_ret(c_x0); return c_rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_33180<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = 0;
    fl_l = 0; fl_r = 0;
    rdi = rcx;
    fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; fl_r = ((rbx as u32)) as u32 as u64;
    if (((fl_l as u32)) <= ((fl_r as u32))) { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    loop { rcx = (crate::simd_ops::ru64(rdi)) as u64; rax = ((rbx as u32)) as u32 as u64; rax = ((rax).wrapping_add(rax)) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(rax.wrapping_mul(8)))) as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rbx = (((rbx as u32)).wrapping_add(1)) as u32 as u64; fl_l = ((rbx as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rdi.wrapping_add(P0))) as u32 as u64; if !((((fl_l as u32)) < ((fl_r as u32)))) { break; } }
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_33181<const P0: u64, const P1: u64, const P2: u64, const P3: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { fl_l = rdx; fl_r = r8; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 4; continue; } crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rsi = r8; rbx = rdx; bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(P0))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } fl_l = (crate::simd_ops::lk_add_u32(rax, (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; if (((fl_l as u32)) != 0) { bb = 3; continue; } rdi = (crate::simd_ops::ru64(rbx)) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } rcx = rdi; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rcx = rax; rdx = rdi; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 2; continue; }
            2 => { rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rcx = rax; rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P0))) as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 3; continue; }
            3 => { rbx = ((rbx).wrapping_add(0x10u64)) as u64; fl_l = rbx; fl_r = rsi; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 1; continue; } rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); bb = 4; continue; }
            4 => { return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_33183<const P0: u64, const P1: u64, const P2: u64, const P3: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rdi = rcx;
    rbx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rcx = rax;
    rdx = rbx;
    rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp);
    rbx = (crate::simd_ops::ru64(rdi.wrapping_add(P3))) as u64;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rcx = rax;
    rdx = rbx;
    rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp);
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_33185<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    crate::simd_ops::forward_incoming_to_outgoing();
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    crate::simd_ops::wu64(rcx.wrapping_add(P0), ((xmm1 as u64)) as u64);
    crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(P1, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::set_xmm_ret(c_x0); return c_rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_33217<const P0: u64, const P1: u64, const P2: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = rdx; rbx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rdx, rbx); rsi = r8; crate::simd_ops::wu64(rdx.wrapping_add(P0), rbx); rcx = rdi; crate::simd_ops::wu64(rdx.wrapping_add(P1), rbx); rdx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64; rdx = ((rdx).wrapping_sub(crate::simd_ops::ru64(r8))) as u64; r8 = rsp.wrapping_add(0x38); rdx = ((((rdx) as i64).wrapping_shr(3u32) as u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rbx); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); r8 = (crate::simd_ops::ru64(rdi)) as u64; rax = (crate::simd_ops::ru64(rdi.wrapping_add(P0))) as u64; rax = ((rax).wrapping_sub(r8)) as u64; rax = ((((rax) as i64).wrapping_shr(3u32) as u64)) as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = ((rbx as u32)) as u32 as u64; rbx = (((rbx as u32)).wrapping_add(1)) as u32 as u64; rdx = ((rax.wrapping_mul(8)) as u64) as u64; rax = (crate::simd_ops::ru64(rsi)) as u64; rcx = (crate::simd_ops::ru64(rdx.wrapping_add(rax))) as u64; crate::simd_ops::wu64(rdx.wrapping_add(r8), rcx); r8 = (crate::simd_ops::ru64(rdi)) as u64; rax = (crate::simd_ops::ru64(rdi.wrapping_add(P0))) as u64; rax = ((rax).wrapping_sub(r8)) as u64; rax = ((((rax) as i64).wrapping_shr(3u32) as u64)) as u64; fl_l = ((rbx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 1; continue; } bb = 2; continue; }
            2 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_33227<const P0: u64, const P1: u64, const P2: u64, const P3: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    r8 = P0;
    r9 = 0x15;
    rdx = ((r9.wrapping_add(P1)) as u32) as u32 as u64;
    rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64, rax, rcx, rdx, r8, r9, rsp);
    crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax);
    fl_l = rax; fl_r = rax;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rcx = P3; crate::simd_ops::wu64(rax, rcx); rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_33232<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    rax = (crate::simd_ops::ru32(rax.wrapping_add(P0))) as u32 as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_33265<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x50u64)) as u64;
    rax = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64;
    xmm2 = 0u128;
    xmm3 = ((crate::simd_ops::ru32(rcx.wrapping_add(P1))) as u32) as u128;
    rbx = rcx;
    crate::simd_ops::wu32(rcx.wrapping_add(P2), ((xmm1 as u32)) as u32);
    rcx = rsp.wrapping_add(0x20);
    xmm2 = (xmm2 & !0xffff_ffff_ffff_ffffu128) | (((f32::from_bits(xmm1 as u32)) as f64).to_bits() as u128);
    xmm1 = 0u128;
    xmm3 = crate::simd_ops::vcvtps2pd(xmm3);
    xmm1 = (xmm1 & !0xffff_ffff_ffff_ffffu128) | ((((rax) as i64) as f64).to_bits() as u128);
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P4))) as u64;
    rdx = rsp.wrapping_add(0x20);
    r8 = 0;
    fl_l = 0; fl_r = 0;
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    rsp = ((rsp).wrapping_add(0x50u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_33266<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x50u64)) as u64;
    rax = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64;
    xmm3 = 0u128;
    xmm2 = ((crate::simd_ops::ru32(rcx.wrapping_add(P1))) as u32) as u128;
    rbx = rcx;
    crate::simd_ops::wu32(rcx.wrapping_add(P2), ((xmm1 as u32)) as u32);
    rcx = rsp.wrapping_add(0x20);
    xmm3 = (xmm3 & !0xffff_ffff_ffff_ffffu128) | (((f32::from_bits(xmm1 as u32)) as f64).to_bits() as u128);
    xmm1 = 0u128;
    xmm2 = crate::simd_ops::vcvtps2pd(xmm2);
    xmm1 = (xmm1 & !0xffff_ffff_ffff_ffffu128) | ((((rax) as i64) as f64).to_bits() as u128);
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P4))) as u64;
    rdx = rsp.wrapping_add(0x20);
    r8 = 0;
    fl_l = 0; fl_r = 0;
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    rsp = ((rsp).wrapping_add(0x50u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_33275<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    r8 = P0;
    r9 = 0x15;
    rdx = ((r9.wrapping_add(P1)) as u32) as u32 as u64;
    rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64, rax, rcx, rdx, r8, r9, rsp);
    crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax);
    rcx = rax;
    fl_l = rax; fl_r = rax;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rax = P3; crate::simd_ops::wu64(rcx, rax); rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rcx.wrapping_add(P2), rax); crate::simd_ops::wu64(rcx.wrapping_add(P4), rax); crate::simd_ops::wu64(rcx.wrapping_add(P5), rax); rax = rcx; rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_33277<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64, const P9: u64, const P10: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rdi = r8; rbx = rdx; rbp = rcx; crate::simd_ops::wu64(rcx.wrapping_add(P0), r9); crate::simd_ops::wu64(rcx.wrapping_add(P1), rdx); crate::simd_ops::wu8(r9.wrapping_add(P2), (0x1u8) as u8); rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; crate::simd_ops::wu8(rax.wrapping_add(P3), (0x1u8) as u8); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rdx); crate::simd_ops::wu16(rsp.wrapping_add(0x38u64), (0x0u16) as u16); rcx = rsp.wrapping_add(0x30); rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rbx)) as u64; rsi = 0x34; r9 = ((rsi as u32)) as u32 as u64; r8 = P5; rdx = ((rsi.wrapping_add(P6)) as u32) as u32 as u64; rcx = rbx; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(rax.wrapping_add(P7))) as u64, rax, rcx, rdx, r8, r9, rsp); r14 = rax; crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), rax); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 1; continue; } rdx = (crate::simd_ops::ru64(rdi)) as u64; rcx = rdi; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(rdx.wrapping_add(P7))) as u64, rax, rcx, rdx, r8, r9, rsp); rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; rbx = ((crate::simd_ops::ru8(rdi.wrapping_add(P8))) as u32) as u32 as u64; r9 = (crate::simd_ops::ru64(rax)) as u64; rcx = rax; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(r9.wrapping_add(P0))) as u64, rax, rcx, rdx, r8, r9, rsp); r9 = (((rax as u16)) as u32) as u32 as u64; crate::simd_ops::wu8(rsp.wrapping_add(0x20u64), (0x0u8) as u8); r8 = ((rbx as u32)) as u32 as u64; rdx = (crate::simd_ops::ru32(rdi)) as u32 as u64; rcx = r14; rax = crate::simd_ops::call_stk(P9, rax, rcx, rdx, r8, r9, rsp); bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rbp.wrapping_add(0x8u64), rax); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } rax = crate::simd_ops::call_stk(P10, rax, rcx, rdx, r8, r9, rsp); rcx = 0x1; fl_l = (crate::simd_ops::ru8(rax.wrapping_add(P7))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; rsi = (if (((fl_l as u8)) == ((fl_r as u8))) { (rcx as u32) } else { (rsi as u32) }) as u32 as u64; bb = 2; continue; }
            2 => { rax = ((rsi as u32)) as u32 as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_33286<const P0: u64, const P1: u64, const P2: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut xmm6: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x40u64)) as u64;
    rbx = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm6) as u128);
    rsi = ((r8 as u32)) as u32 as u64;
    xmm6 = (xmm1) as u128;
    rdi = rcx;
    fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; fl_r = ((rbx as u32)) as u32 as u64;
    if (((fl_l as u32)) <= ((fl_r as u32))) { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x30u64))) as u128; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    loop { rax = ((rbx as u32)) as u32 as u64; rdx = 0x3; rcx = (((rax) as i64).wrapping_mul((0xb88u64) as i64) as u64) as u64; xmm3 = (xmm6) as u128; r8 = ((rdx.wrapping_add(P1)) as u32) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rsi as u32)) as u32); rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rdi))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = (((rbx as u32)).wrapping_add(1)) as u32 as u64; fl_l = ((rbx as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rdi.wrapping_add(P0))) as u32 as u64; if !((((fl_l as u32)) < ((fl_r as u32)))) { break; } }
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64;
    rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64;
    xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x30u64))) as u128;
    rsp = ((rsp).wrapping_add(0x40u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_33297<const P0: u64, const P1: u64, const P2: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rsp = ((rsp).wrapping_sub(0x38u64)) as u64;
    xmm0 = ((crate::simd_ops::ru32(P0)) as u32) as u128;
    rax = P1;
    crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((xmm0 as u32)) as u32);
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax);
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    rsp = ((rsp).wrapping_add(0x38u64)) as u64;
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_33305<const P0: u64, const P1: u64, const P2: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rsp = ((rsp).wrapping_sub(0x38u64)) as u64;
    xmm3 = ((crate::simd_ops::ru32(P0)) as u32) as u128;
    xmm2 = ((crate::simd_ops::ru32(P1)) as u32) as u128;
    crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), r9);
    crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((r8 as u32)) as u32);
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    rsp = ((rsp).wrapping_add(0x38u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_33336<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = 0; fl_l = 0; fl_r = 0; rbp = ((r8 as u32)) as u32 as u64; r14 = ((rdx as u32)) as u32 as u64; rsi = rcx; rbx = (rbx & !0xffu64) | (((0x1u8) as u8 as u64)); fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rcx = ((rdi as u32)) as u32 as u64; r8 = ((rbp as u32)) as u32 as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rdx = ((r14 as u32)) as u32 as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rsi))) as u64; rcx = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rbx = (rbx & !0xffu64) | ((((((rbx as u8)) & ((rax as u8)))) as u8 as u64)); rdi = (((rdi as u32)).wrapping_add(1)) as u32 as u64; fl_l = ((rdi as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rsi.wrapping_add(P0))) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 1; continue; } bb = 2; continue; }
            2 => { rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rax = (((rbx as u8)) as u32) as u32 as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_33344<const P0: u64, const P1: u64, const P2: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
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
    if !(((((fl_l as u8)) & ((fl_r as u8))) == 0)) { rdx = 0xb0; rcx = rdi; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); }
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rax = rdi;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_33366<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu64(rcx.wrapping_add(P0), rax);
    crate::simd_ops::wu64(rcx.wrapping_add(P1), rax);
    crate::simd_ops::wu64(rcx.wrapping_add(P2), rax);
    crate::simd_ops::wu64(rcx.wrapping_add(P3), rax);
    crate::simd_ops::wu64(rcx.wrapping_add(P4), rax);
    crate::simd_ops::wu64(rcx.wrapping_add(P5), rax);
    crate::simd_ops::wu64(rcx.wrapping_add(P6), rax);
    crate::simd_ops::wu64(rcx.wrapping_add(P7), rax);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_33371<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64, const P9: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut xmm4: u128 = 0; let mut xmm5: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 0>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) * f32::from_bits(crate::simd_ops::ru32(rcx.wrapping_add(P0)))).to_bits() as u128); xmm0 = 0u128; xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) / f32::from_bits(xmm1 as u32)).to_bits() as u128); fl_l = ((xmm2 as u32)) as u32 as u64; fl_r = ((xmm0 as u32)) as u32 as u64; if !(f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { bb = 1; continue; } xmm0 = (xmm2) as u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((if f32::from_bits(xmm0 as u32) < f32::from_bits(crate::simd_ops::ru32(P1)) { f32::from_bits(xmm0 as u32) } else { f32::from_bits(crate::simd_ops::ru32(P1)) }).to_bits() as u128); bb = 1; continue; }
            1 => { xmm5 = ((crate::simd_ops::ru64(P2)) as u64) as u128; xmm3 = 0u128; xmm1 = ((crate::simd_ops::ru32(rsp.wrapping_add(0x30u64))) as u32) as u128; xmm4 = (xmm5) as u128; xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(crate::simd_ops::ru32(P3))).to_bits() as u128); xmm3 = (xmm3 & !0xffff_ffff_ffff_ffffu128) | (((f32::from_bits(xmm0 as u32)) as f64).to_bits() as u128); xmm4 = (xmm4 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm4 as u64) - f64::from_bits(xmm3 as u64)).to_bits() as u128); xmm0 = (xmm3) as u128; xmm0 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm0 as u64) * f64::from_bits(crate::simd_ops::ru64(P4))).to_bits() as u128); xmm2 = (xmm4) as u128; xmm2 = (xmm2 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm2 as u64) * f64::from_bits(crate::simd_ops::ru64(P5))).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm0 as u64) * f64::from_bits(xmm4 as u64)).to_bits() as u128); xmm2 = (xmm2 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm2 as u64) * f64::from_bits(xmm4 as u64)).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm0 as u64) + f64::from_bits(xmm3 as u64)).to_bits() as u128); xmm2 = (xmm2 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm2 as u64) + f64::from_bits(xmm3 as u64)).to_bits() as u128); crate::simd_ops::wu64(rcx.wrapping_add(P6), ((xmm0 as u64)) as u64); xmm0 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm0 as u64) + f64::from_bits(xmm0 as u64)).to_bits() as u128); xmm2 = (xmm2 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm2 as u64) * f64::from_bits(xmm4 as u64)).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm0 as u64) - f64::from_bits(xmm5 as u64)).to_bits() as u128); xmm2 = (xmm2 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm2 as u64) * f64::from_bits(crate::simd_ops::ru64(P7))).to_bits() as u128); crate::simd_ops::wu64(rcx.wrapping_add(P8), ((xmm0 as u64)) as u64); xmm0 = crate::simd_ops::vcvtps2pd(xmm1); xmm2 = (xmm2 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm2 as u64) + f64::from_bits(xmm5 as u64)).to_bits() as u128); xmm2 = (xmm2 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm2 as u64) * f64::from_bits(xmm0 as u64)).to_bits() as u128); crate::simd_ops::wu64(rcx.wrapping_add(P9), ((xmm2 as u64)) as u64); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_33406<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    fl_l = (crate::simd_ops::ru8(P0)) as u8 as u64; fl_r = (0x0u8) as u8 as u64;
    if !((((fl_l as u8)) == ((fl_r as u8)))) { rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu8(P0, (0x0u8) as u8); }
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_33474<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = (crate::simd_ops::ru64(P0)) as u64;
    fl_l = rcx; fl_r = rcx;
    rax = (if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { rcx } else { rax }) as u64;
    crate::simd_ops::wu64(P1, rax);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_33481<const P0: u64, const P1: u64, const P2: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    fl_l = (crate::simd_ops::ru8(P0)) as u8 as u64; fl_r = (0x0u8) as u8 as u64;
    if !((((fl_l as u8)) == ((fl_r as u8)))) { rcx = P1; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P2)) as u64, rax, rcx, rdx, r8, r9, rsp); rax = 0; fl_l = 0; fl_r = 0; rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
    rax = 0x80010002;
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_33482<const P0: u64, const P1: u64, const P2: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    crate::simd_ops::forward_incoming_to_outgoing();
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    fl_l = (crate::simd_ops::ru8(P0)) as u8 as u64; fl_r = (0x0u8) as u8 as u64;
    if !((((fl_l as u8)) == ((fl_r as u8)))) { rcx = P1; crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((crate::simd_ops::ru64(P2)) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_33583<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64, const P9: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut xmm4: u128 = 0; let mut xmm5: u128 = 0; let mut xmm6: u128 = 0; let mut xmm7: u128 = 0; let mut xmm8: u128 = 0; let mut xmm9: u128 = 0; let mut xmm10: u128 = 0; let mut xmm11: u128 = 0; let mut xmm12: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 19>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = rsp; rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x90u64)) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0xc0u64))) as u64; xmm0 = 0u128; crate::simd_ops::wu128(rax.wrapping_sub(0x18u64), (xmm6) as u128); xmm6 = (xmm2) as u128; crate::simd_ops::wu128(rax.wrapping_sub(0x28u64), (xmm7) as u128); xmm7 = (xmm1) as u128; crate::simd_ops::wu128(rax.wrapping_sub(0x38u64), (xmm8) as u128); crate::simd_ops::wu128(rax.wrapping_sub(0x48u64), (xmm9) as u128); crate::simd_ops::wu128(rax.wrapping_sub(0x58u64), (xmm10) as u128); xmm10 = (xmm3) as u128; crate::simd_ops::wu128(rax.wrapping_sub(0x68u64), (xmm11) as u128); crate::simd_ops::wu128(rax.wrapping_sub(0x78u64), (xmm12) as u128); rax = ((rcx as u32)) as u32 as u64; xmm0 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | ((((rax) as i64) as f64).to_bits() as u128); xmm7 = (xmm7 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm7 as u64) / f64::from_bits(xmm0 as u64)).to_bits() as u128); xmm7 = (xmm7 & !0xffff_ffff_ffff_ffffu128) | ((if f64::from_bits(xmm7 as u64) > f64::from_bits(crate::simd_ops::ru64(P0)) { f64::from_bits(xmm7 as u64) } else { f64::from_bits(crate::simd_ops::ru64(P0)) }).to_bits() as u128); xmm7 = (xmm7 & !0xffff_ffff_ffff_ffffu128) | ((if f64::from_bits(xmm7 as u64) < f64::from_bits(crate::simd_ops::ru64(P1)) { f64::from_bits(xmm7 as u64) } else { f64::from_bits(crate::simd_ops::ru64(P1)) }).to_bits() as u128); xmm0 = (xmm7) as u128; xmm0 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm0 as u64) * f64::from_bits(crate::simd_ops::ru64(P2))).to_bits() as u128); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm8 = ((crate::simd_ops::ru64(P4)) as u64) as u128; xmm1 = (xmm0) as u128; xmm1 = (xmm1 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm1 as u64) + f64::from_bits(xmm8 as u64)).to_bits() as u128); xmm9 = (xmm8) as u128; xmm9 = (xmm9 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm9 as u64) / f64::from_bits(xmm6 as u64)).to_bits() as u128); xmm11 = (xmm0) as u128; xmm12 = (xmm0) as u128; xmm9 = (xmm9 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm9 as u64) * f64::from_bits(xmm1 as u64)).to_bits() as u128); xmm1 = (xmm10) as u128; xmm1 = (xmm1 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm1 as u64) * f64::from_bits(crate::simd_ops::ru64(P5))).to_bits() as u128); xmm11 = (xmm11 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm11 as u64) * f64::from_bits(xmm0 as u64)).to_bits() as u128); xmm0 = ((crate::simd_ops::ru64(P6)) as u64) as u128; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((xmm0 as u64)) as u64; fl_r = ((xmm8 as u64)) as u64; xmm10 = (xmm10 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm10 as u64) * f64::from_bits(crate::simd_ops::ru64(P8))).to_bits() as u128); xmm5 = (xmm8) as u128; xmm5 = (xmm5 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm5 as u64) - f64::from_bits(xmm7 as u64)).to_bits() as u128); xmm10 = (xmm10) & (crate::simd_ops::ru128(P9)); if !(f64::from_bits(fl_l) >= f64::from_bits(fl_r)) { bb = 1; continue; } xmm3 = (xmm0) as u128; xmm4 = (xmm8) as u128; xmm4 = (xmm4 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm4 as u64) - f64::from_bits(xmm10 as u64)).to_bits() as u128); xmm3 = (xmm3 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm3 as u64) * f64::from_bits(xmm9 as u64)).to_bits() as u128); xmm7 = (xmm7 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm7 as u64) * f64::from_bits(xmm3 as u64)).to_bits() as u128); xmm7 = (xmm7 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm7 as u64) + f64::from_bits(xmm5 as u64)).to_bits() as u128); xmm7 = (xmm7 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm7 as u64) * f64::from_bits(xmm10 as u64)).to_bits() as u128); xmm4 = (xmm4 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm4 as u64) + f64::from_bits(xmm7 as u64)).to_bits() as u128); xmm7 = (xmm8) as u128; xmm4 = (xmm4 & !0xffff_ffff_ffff_ffffu128) | ((if f64::from_bits(xmm4 as u64) > f64::from_bits(xmm8 as u64) { f64::from_bits(xmm4 as u64) } else { f64::from_bits(xmm8 as u64) }).to_bits() as u128); xmm4 = (xmm4 & !0xffff_ffff_ffff_ffffu128) | ((if f64::from_bits(xmm4 as u64) < f64::from_bits(xmm0 as u64) { f64::from_bits(xmm4 as u64) } else { f64::from_bits(xmm0 as u64) }).to_bits() as u128); bb = 2; continue; }
            1 => { xmm1 = (xmm8) as u128; xmm3 = (xmm9) as u128; xmm1 = (xmm1 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm1 as u64) / f64::from_bits(xmm0 as u64)).to_bits() as u128); xmm4 = (xmm8) as u128; xmm0 = (xmm1) as u128; xmm0 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm0 as u64) * f64::from_bits(xmm9 as u64)).to_bits() as u128); xmm7 = (xmm7 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm7 as u64) * f64::from_bits(xmm0 as u64)).to_bits() as u128); xmm9 = (xmm0) as u128; xmm0 = (xmm8) as u128; xmm0 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm0 as u64) - f64::from_bits(xmm10 as u64)).to_bits() as u128); xmm7 = (xmm7 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm7 as u64) + f64::from_bits(xmm5 as u64)).to_bits() as u128); xmm7 = (xmm7 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm7 as u64) * f64::from_bits(xmm10 as u64)).to_bits() as u128); xmm7 = (xmm7 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm7 as u64) + f64::from_bits(xmm0 as u64)).to_bits() as u128); xmm7 = (xmm7 & !0xffff_ffff_ffff_ffffu128) | ((if f64::from_bits(xmm7 as u64) > f64::from_bits(xmm8 as u64) { f64::from_bits(xmm7 as u64) } else { f64::from_bits(xmm8 as u64) }).to_bits() as u128); xmm7 = (xmm7 & !0xffff_ffff_ffff_ffffu128) | ((if f64::from_bits(xmm7 as u64) < f64::from_bits(xmm1 as u64) { f64::from_bits(xmm7 as u64) } else { f64::from_bits(xmm1 as u64) }).to_bits() as u128); bb = 2; continue; }
            2 => { xmm9 = (xmm9 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm9 as u64) * f64::from_bits(xmm12 as u64)).to_bits() as u128); r11 = rsp.wrapping_add(0x90); rax = (0x3ff0000000000000u64) as u64; xmm6 = (crate::simd_ops::ru128(r11.wrapping_sub(0x10u64))) as u128; xmm2 = (xmm11) as u128; xmm10 = (crate::simd_ops::ru128(r11.wrapping_sub(0x50u64))) as u128; xmm2 = (xmm2 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm2 as u64) + f64::from_bits(xmm11 as u64)).to_bits() as u128); xmm3 = (xmm3 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm3 as u64) * f64::from_bits(xmm12 as u64)).to_bits() as u128); crate::simd_ops::wu64(rbx.wrapping_add(0x18u64), rax); xmm12 = (crate::simd_ops::ru128(r11.wrapping_sub(0x70u64))) as u128; xmm0 = (xmm9) as u128; xmm0 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm0 as u64) + f64::from_bits(xmm7 as u64)).to_bits() as u128); xmm1 = (xmm3) as u128; xmm1 = (xmm1 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm1 as u64) + f64::from_bits(xmm4 as u64)).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm0 as u64) + f64::from_bits(xmm11 as u64)).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm1 as u64) + f64::from_bits(xmm11 as u64)).to_bits() as u128); xmm8 = (xmm8 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm8 as u64) / f64::from_bits(xmm0 as u64)).to_bits() as u128); xmm0 = (xmm4) as u128; xmm0 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm0 as u64) + f64::from_bits(xmm4 as u64)).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm1 as u64) * f64::from_bits(xmm8 as u64)).to_bits() as u128); xmm4 = (xmm4 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm4 as u64) - f64::from_bits(xmm3 as u64)).to_bits() as u128); crate::simd_ops::wu64(rbx, ((xmm1 as u64)) as u64); xmm1 = (xmm2) as u128; xmm1 = (xmm1 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm1 as u64) - f64::from_bits(xmm0 as u64)).to_bits() as u128); xmm0 = (xmm7) as u128; xmm0 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm0 as u64) + f64::from_bits(xmm7 as u64)).to_bits() as u128); xmm7 = (xmm7 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm7 as u64) - f64::from_bits(xmm9 as u64)).to_bits() as u128); xmm9 = (crate::simd_ops::ru128(r11.wrapping_sub(0x40u64))) as u128; xmm4 = (xmm4 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm4 as u64) + f64::from_bits(xmm11 as u64)).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm1 as u64) * f64::from_bits(xmm8 as u64)).to_bits() as u128); xmm7 = (xmm7 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm7 as u64) + f64::from_bits(xmm11 as u64)).to_bits() as u128); xmm11 = (crate::simd_ops::ru128(r11.wrapping_sub(0x60u64))) as u128; xmm4 = (xmm4 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm4 as u64) * f64::from_bits(xmm8 as u64)).to_bits() as u128); xmm2 = (xmm2 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm2 as u64) - f64::from_bits(xmm0 as u64)).to_bits() as u128); crate::simd_ops::wu64(rbx.wrapping_add(0x8u64), ((xmm1 as u64)) as u64); xmm7 = (xmm7 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm7 as u64) * f64::from_bits(xmm8 as u64)).to_bits() as u128); xmm2 = (xmm2 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm2 as u64) * f64::from_bits(xmm8 as u64)).to_bits() as u128); xmm8 = (crate::simd_ops::ru128(r11.wrapping_sub(0x30u64))) as u128; crate::simd_ops::wu64(rbx.wrapping_add(0x28u64), ((xmm7 as u64)) as u64); xmm7 = (crate::simd_ops::ru128(rsp.wrapping_add(0x70u64))) as u128; crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), ((xmm4 as u64)) as u64); crate::simd_ops::wu64(rbx.wrapping_add(0x20u64), ((xmm2 as u64)) as u64); rsp = r11; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_33625<const P0: u64, const P1: u64, const P2: u64, const P3: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rsp = ((rsp).wrapping_sub(0x58u64)) as u64;
    xmm0 = (crate::simd_ops::ru128(rcx.wrapping_add(P0))) as u128;
    rax = rdx;
    rdx = rsp.wrapping_add(0x20);
    xmm1 = (crate::simd_ops::ru128(rcx.wrapping_add(P1))) as u128;
    crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128);
    xmm0 = ((crate::simd_ops::ru64(rcx.wrapping_add(P2))) as u64) as u128;
    rcx = rax;
    crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), ((xmm0 as u64)) as u64);
    crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm1) as u128);
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    rsp = ((rsp).wrapping_add(0x58u64)) as u64;
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_33661<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 15>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rbp = rsp; rsp = ((rsp).wrapping_sub(0x50u64)) as u64; xmm1 = (crate::simd_ops::ru128(rcx.wrapping_add(0x28u64))) as u128; rbx = rdx; rdi = rcx; rdx = (crate::simd_ops::ru64(rcx.wrapping_add(0x18u64))) as u64; r14 = ((r8 as u32)) as u32 as u64; rcx = rbx; crate::simd_ops::wu128(rbp.wrapping_sub(0x18u64), (xmm1) as u128); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x58u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 1; continue; } fl_l = (crate::simd_ops::lk_add_u32(rax, (1) as u32)) as u32 as u64; bb = 1; continue; }
            1 => { r15 = 0; fl_l = 0; fl_r = 0; rsi = P1; crate::simd_ops::wu64(rbp.wrapping_add(0x30u64), r15); fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(0x48u64))) as u8 as u64; fl_r = ((r15 as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 4; continue; } r8 = (crate::simd_ops::ru64(rbx)) as u64; rax = (crate::simd_ops::ru32(r8.wrapping_add(0x10u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 2; continue; } fl_l = (((rax as u32)).wrapping_sub(1)) as u32 as u64; rax = ((fl_l as u32)) as u32 as u64; rdx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = (crate::simd_ops::ru64(r8.wrapping_add(0x8u64))) as u64; rdx = (crate::simd_ops::ru64(rax.wrapping_add(rdx.wrapping_mul(8)))) as u64; bb = 3; continue; }
            2 => { rdx = rsi; bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x28u64))) as u64; r8 = rbp.wrapping_add(0x30); rax = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm((crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 4; continue; } rdx = (crate::simd_ops::ru64(rbp.wrapping_add(0x30u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } fl_l = (crate::simd_ops::ru8(rdx)) as u8 as u64; fl_r = ((r15 as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 4; continue; } rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x20u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } rax = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm((crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x30u64))) as u64; rdx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm((crate::simd_ops::ru64(rbp.wrapping_sub(0x10u64))) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 4; continue; }
            4 => { rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm1 = (crate::simd_ops::ru128(rdi.wrapping_add(0x48u64))) as u128; rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0x38u64))) as u64; rcx = rbx; crate::simd_ops::wu128(rbp.wrapping_sub(0x18u64), (xmm1) as u128); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x58u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } fl_l = (crate::simd_ops::lk_add_u32(rax, (1) as u32)) as u32 as u64; bb = 5; continue; }
            5 => { crate::simd_ops::wu64(rbp.wrapping_add(0x30u64), r15); fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(0x48u64))) as u8 as u64; fl_r = ((r15 as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 8; continue; } r8 = (crate::simd_ops::ru64(rbx)) as u64; rax = (crate::simd_ops::ru32(r8.wrapping_add(0x10u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 6; continue; } fl_l = (((rax as u32)).wrapping_sub(1)) as u32 as u64; rax = ((fl_l as u32)) as u32 as u64; rdx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = (crate::simd_ops::ru64(r8.wrapping_add(0x8u64))) as u64; rdx = (crate::simd_ops::ru64(rax.wrapping_add(rdx.wrapping_mul(8)))) as u64; bb = 7; continue; }
            6 => { rdx = rsi; bb = 7; continue; }
            7 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x28u64))) as u64; r8 = rbp.wrapping_add(0x30); rax = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm((crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 8; continue; } rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x30u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } fl_l = (crate::simd_ops::ru8(rcx)) as u8 as u64; fl_r = ((r15 as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 8; continue; } rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x38u64))) as u64; r9 = P3; r8 = rbp.wrapping_add(0x40); crate::simd_ops::wu32(rbp.wrapping_add(0x40u64), ((r15 as u32)) as u32); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = ((crate::simd_ops::ru8(rbp.wrapping_add(0x40u64))) as u32) as u32 as u64; rdx = rbp.wrapping_add(0x40); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu8(rbp.wrapping_add(0x40u64), ((rax as u8)) as u8); (rax, xmm0) = crate::simd_ops::call_stk_xmm((crate::simd_ops::ru64(rbp.wrapping_sub(0x10u64))) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 8; continue; }
            8 => { rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm1 = (crate::simd_ops::ru128(rdi.wrapping_add(0x68u64))) as u128; rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0x58u64))) as u64; rcx = rbx; crate::simd_ops::wu128(rbp.wrapping_sub(0x18u64), (xmm1) as u128); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x58u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } fl_l = (crate::simd_ops::lk_add_u32(rax, (1) as u32)) as u32 as u64; bb = 9; continue; }
            9 => { crate::simd_ops::wu64(rbp.wrapping_add(0x30u64), r15); fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(0x48u64))) as u8 as u64; fl_r = ((r15 as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 12; continue; } r8 = (crate::simd_ops::ru64(rbx)) as u64; rax = (crate::simd_ops::ru32(r8.wrapping_add(0x10u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 10; continue; } fl_l = (((rax as u32)).wrapping_sub(1)) as u32 as u64; rax = ((fl_l as u32)) as u32 as u64; rdx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = (crate::simd_ops::ru64(r8.wrapping_add(0x8u64))) as u64; rdx = (crate::simd_ops::ru64(rax.wrapping_add(rdx.wrapping_mul(8)))) as u64; bb = 11; continue; }
            10 => { rdx = rsi; bb = 11; continue; }
            11 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x28u64))) as u64; r8 = rbp.wrapping_add(0x30); rax = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm((crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 12; continue; } rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x30u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } fl_l = (crate::simd_ops::ru8(rcx)) as u8 as u64; fl_r = ((r15 as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 12; continue; } r8 = 0xa; crate::simd_ops::wu64(rbp.wrapping_add(0x38u64), rcx); rdx = rbp.wrapping_add(0x38); (rax, xmm0) = crate::simd_ops::call_stk_xmm((crate::simd_ops::ru64(P5)) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x30u64))) as u64; rdx = (((rax as u8)) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm((crate::simd_ops::ru64(rbp.wrapping_sub(0x10u64))) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 12; continue; }
            12 => { rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm1 = (crate::simd_ops::ru128(rdi.wrapping_add(0x88u64))) as u128; rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0x78u64))) as u64; rcx = rbx; crate::simd_ops::wu128(rbp.wrapping_sub(0x18u64), (xmm1) as u128); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x58u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 13; continue; } fl_l = (crate::simd_ops::lk_add_u32(rax, (1) as u32)) as u32 as u64; bb = 13; continue; }
            13 => { crate::simd_ops::wu64(rbp.wrapping_add(0x30u64), r15); fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(0x48u64))) as u8 as u64; fl_r = ((r15 as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 16; continue; } r8 = (crate::simd_ops::ru64(rbx)) as u64; rax = (crate::simd_ops::ru32(r8.wrapping_add(0x10u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 14; continue; } fl_l = (((rax as u32)).wrapping_sub(1)) as u32 as u64; rax = ((fl_l as u32)) as u32 as u64; rdx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = (crate::simd_ops::ru64(r8.wrapping_add(0x8u64))) as u64; rdx = (crate::simd_ops::ru64(rax.wrapping_add(rdx.wrapping_mul(8)))) as u64; bb = 15; continue; }
            14 => { rdx = rsi; bb = 15; continue; }
            15 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x28u64))) as u64; r8 = rbp.wrapping_add(0x30); rax = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm((crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 16; continue; } rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x30u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 16; continue; } fl_l = (crate::simd_ops::ru8(rcx)) as u8 as u64; fl_r = ((r15 as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 16; continue; } r8 = 0xa; crate::simd_ops::wu64(rbp.wrapping_add(0x38u64), rcx); rdx = rbp.wrapping_add(0x38); (rax, xmm0) = crate::simd_ops::call_stk_xmm((crate::simd_ops::ru64(P5)) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x30u64))) as u64; rdx = (((rax as u8)) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm((crate::simd_ops::ru64(rbp.wrapping_sub(0x10u64))) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 16; continue; }
            16 => { rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm1 = (crate::simd_ops::ru128(rdi.wrapping_add(0xa8u64))) as u128; rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0x98u64))) as u64; rcx = rbx; crate::simd_ops::wu128(rbp.wrapping_sub(0x18u64), (xmm1) as u128); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x58u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 17; continue; } fl_l = (crate::simd_ops::lk_add_u32(rax, (1) as u32)) as u32 as u64; bb = 17; continue; }
            17 => { crate::simd_ops::wu64(rbp.wrapping_add(0x30u64), r15); fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(0x48u64))) as u8 as u64; fl_r = ((r15 as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 19; continue; } r8 = (crate::simd_ops::ru64(rbx)) as u64; rax = (crate::simd_ops::ru32(r8.wrapping_add(0x10u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 18; continue; } fl_l = (((rax as u32)).wrapping_sub(1)) as u32 as u64; rax = ((fl_l as u32)) as u32 as u64; rdx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = (crate::simd_ops::ru64(r8.wrapping_add(0x8u64))) as u64; rsi = (crate::simd_ops::ru64(rax.wrapping_add(rdx.wrapping_mul(8)))) as u64; bb = 18; continue; }
            18 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x28u64))) as u64; r8 = rbp.wrapping_add(0x30); rdx = rsi; rax = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm((crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 19; continue; } rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x30u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 19; continue; } fl_l = (crate::simd_ops::ru8(rcx)) as u8 as u64; fl_r = ((r15 as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 19; continue; } rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x38u64))) as u64; r9 = P6; r8 = rbp.wrapping_add(0x40); crate::simd_ops::wu32(rbp.wrapping_add(0x40u64), ((r15 as u32)) as u32); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = ((crate::simd_ops::ru8(rbp.wrapping_add(0x40u64))) as u32) as u32 as u64; rdx = rbp.wrapping_add(0x40); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu8(rbp.wrapping_add(0x40u64), ((rax as u8)) as u8); (rax, xmm0) = crate::simd_ops::call_stk_xmm((crate::simd_ops::ru64(rbp.wrapping_sub(0x10u64))) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 19; continue; }
            19 => { rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x98u64))) as u64; rax = ((r14.wrapping_add(0x8u64)) as u32) as u32 as u64; rsp = ((rsp).wrapping_add(0x50u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_33698<const P0: u64, const P1: u64, const P2: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 41>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x140u64)) as u64; rdi = rcx; r14 = r9; rcx = rsp.wrapping_add(0x60); rsi = r8; rbp = rdx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x170u64))) as u64; rdx = rsp.wrapping_add(0x20); xmm0 = 0u128; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); r8 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rsi); rcx = rsp.wrapping_add(0x60); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbp); crate::simd_ops::wu128(rsp.wrapping_add(0x48u64), (xmm0) as u128); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), r14); crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rbx); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm0 = 0u128; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); r8 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rsi); rdx = rsp.wrapping_add(0x20); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbp); rcx = rsp.wrapping_add(0x60); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), r14); crate::simd_ops::wu128(rsp.wrapping_add(0x48u64), (xmm0) as u128); crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rbx); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r11 = rsp.wrapping_add(0x140); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x10u64))) as u64; rbp = (crate::simd_ops::ru64(r11.wrapping_add(0x18u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x20u64))) as u64; rdi = (crate::simd_ops::ru64(r11.wrapping_add(0x28u64))) as u64; rsp = r11; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_33715<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    crate::simd_ops::forward_incoming_to_outgoing();
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = (crate::simd_ops::ru64(rdx)) as u64;
    rcx = rdx;
    rdx = r8;
    crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_33716<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rbx = rdx; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; r14 = r9; rdx = rbp; r15 = r8; rsi = rcx; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64, rax, rcx, rdx, r8, r9, rsp); rdi = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 1; continue; } r9 = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; r8 = r15; r10 = (crate::simd_ops::ru64(rsi)) as u64; rdx = rax; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), r9); rcx = rsi; r9 = r14; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbp); rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(r10.wrapping_add(P1))) as u64, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 1; continue; } rax = P2; crate::simd_ops::wu64(rbx.wrapping_add(P3), rdi); crate::simd_ops::wu64(rbx, rax); crate::simd_ops::wu64(rbx.wrapping_add(P4), rdi); bb = 2; continue; }
            1 => { rax = P5; crate::simd_ops::wu64(rbx, rax); rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rbx.wrapping_add(P3), rax); crate::simd_ops::wu64(rbx.wrapping_add(P4), rax); bb = 2; continue; }
            2 => { rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rax = rbx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_33726<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r10) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rsp = ((rsp).wrapping_sub(0x58u64)) as u64;
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64;
    rcx = rsp.wrapping_add(0x30);
    r10 = r8;
    r8 = rdx;
    rdx = r10;
    xmm0 = (crate::simd_ops::ru128(rax)) as u128;
    xmm1 = ((crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64) as u128;
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x88u64))) as u64;
    crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm0) as u128);
    crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), ((xmm1 as u64)) as u64);
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax);
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    rsp = ((rsp).wrapping_add(0x58u64)) as u64;
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_33759<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbp = ((rdx as u32)) as u32 as u64; rdi = rcx; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(rdi.wrapping_add(P1))) as u64; rsi = rax; rax = (crate::simd_ops::ru32(rdi.wrapping_add(P2))) as u32 as u64; r8 = rsi; r9 = rbx; r10 = ((rsi.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; fl_l = r10; fl_r = rsi; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 2; continue; } rcx = 0; fl_l = 0; fl_r = 0; rdx = ((rax as u32)) as u32 as u64; fl_l = rsi; fl_r = r10; rdx = (if (((fl_l as u64)) > ((fl_r as u64))) { (rcx as u32) } else { (rdx as u32) }) as u32 as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = ((rcx).wrapping_add(1)) as u64; crate::simd_ops::wu64(r8, rax); r8 = ((r8.wrapping_add(P3)) as u64) as u64; rbx = ((rbx.wrapping_add(P3)) as u64) as u64; fl_l = rcx; fl_r = rdx; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 1; continue; } rbx = (crate::simd_ops::ru64(rdi.wrapping_add(P1))) as u64; rax = (crate::simd_ops::ru32(rdi.wrapping_add(P2))) as u32 as u64; r9 = rbx; bb = 2; continue; }
            2 => { rcx = ((r9.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; fl_l = r9; fl_r = rcx; rbx = (if (((fl_l as u64)) >= ((fl_r as u64))) { r9 } else { rbx }) as u64; fl_l = (crate::simd_ops::ru32(rdi.wrapping_add(P4))) as u32 as u64; fl_r = 0x0; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 4; continue; } fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } crate::simd_ops::wu8(rdi.wrapping_add(P5), (0x0u8) as u8); bb = 4; continue; }
            3 => { fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P6)) as u64, rax, rcx, rdx, r8, r9, rsp); rdx = rbx; rcx = rax; r8 = (crate::simd_ops::ru64(rax)) as u64; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(r8.wrapping_add(P7))) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 4; continue; }
            4 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu32(rdi.wrapping_add(P4), ((rbp as u32)) as u32); rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; crate::simd_ops::wu64(rdi.wrapping_add(P1), rsi); rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_33761<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x30u64)) as u64;
    rax = (crate::simd_ops::ru64(r8)) as u64;
    rbx = r8;
    r9 = rdx;
    r8 = 0x80000000;
    rcx = rbx;
    rdi = rdx;
    rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(rax)) as u64, rax, rcx, rdx, r8, r9, rsp);
    rax = P0;
    crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rbx);
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax);
    rdx = rsp.wrapping_add(0x20);
    rax = (crate::simd_ops::ru64(rdi)) as u64;
    rcx = rdi;
    rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64, rax, rcx, rdx, r8, r9, rsp);
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64;
    rsp = ((rsp).wrapping_add(0x30u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_33763<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; rbx = r8; fl_l = ((((rax as u32)) >> (31u32)) & 1) as u64; rax = (((rax as u32)) & !((1u32) << (31u32))) as u32 as u64; rbp = ((rdx as u32)) as u32 as u64; rsi = rcx; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 1; continue; } rdx = ((rbp as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 1; continue; }
            1 => { rdi = (crate::simd_ops::ru64(rsi.wrapping_add(P2))) as u64; rdx = 0; fl_l = 0; fl_r = 0; rax = (crate::simd_ops::ru32(rsi.wrapping_add(P3))) as u32 as u64; r8 = ((rdi.wrapping_add(rbp.wrapping_mul(8))) as u64) as u64; r9 = ((rdi.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; r10 = r8; r10 = ((r10).wrapping_sub(r9)) as u64; r10 = ((r10).wrapping_add(0x7u64)) as u64; r10 = ((r10).wrapping_shr(3u32)) as u64; fl_l = r9; fl_r = r8; r10 = (if (((fl_l as u64)) > ((fl_r as u64))) { rdx } else { r10 }) as u64; fl_l = r10; fl_r = r10; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } fl_l = r10; fl_r = (0x2u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 4; continue; } r11 = (crate::simd_ops::ru64(rbx)) as u64; rax = ((r9.wrapping_add(P4)) as u64) as u64; rax = ((rax.wrapping_add(r10.wrapping_mul(8))) as u64) as u64; fl_l = r9; fl_r = rbx; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 2; continue; } fl_l = rax; fl_r = rbx; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 4; continue; } bb = 2; continue; }
            2 => { r8 = r10; fl_l = (((r8) & (0xfffffffffffffffeu64))) as u64; fl_r = fl_l; r8 = ((fl_l as u64)) as u64; bb = 3; continue; }
            3 => { rdx = ((rdx).wrapping_add(0x2u64)) as u64; fl_l = rdx; fl_r = r8; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 3; continue; } r8 = ((r8.wrapping_mul(8)) as u64) as u64; rdi = r9; rcx = r8; rax = r11; rcx = ((rcx).wrapping_shr(3u32)) as u64; while rcx != 0 { crate::simd_ops::wu64(rdi, rax); rdi = rdi.wrapping_add(8); rcx = rcx.wrapping_sub(1); } fl_l = ((r9).wrapping_add(r8)) as u64; fl_r = fl_l; r9 = ((fl_l as u64)) as u64; bb = 4; continue; }
            4 => { fl_l = rdx; fl_r = r10; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = ((rdx).wrapping_add(1)) as u64; crate::simd_ops::wu64(r9, rax); r9 = ((r9.wrapping_add(P5)) as u64) as u64; fl_l = rdx; fl_r = r10; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 5; continue; } bb = 6; continue; }
            6 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu32(rsi.wrapping_add(P3), ((rbp as u32)) as u32); rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_33765<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x3cu64))) as u32 as u64; rbx = r8; fl_l = ((((rax as u32)) >> (31u32)) & 1) as u64; rax = (((rax as u32)) & !((1u32) << (31u32))) as u32 as u64; rbp = ((rdx as u32)) as u32 as u64; rsi = rcx; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 1; continue; } rdx = ((rbp as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); bb = 1; continue; }
            1 => { rdi = (crate::simd_ops::ru64(rsi.wrapping_add(0x30u64))) as u64; rdx = 0; fl_l = 0; fl_r = 0; rax = (crate::simd_ops::ru32(rsi.wrapping_add(0x38u64))) as u32 as u64; r8 = ((rdi.wrapping_add(rbp.wrapping_mul(8))) as u64) as u64; r9 = ((rdi.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; r10 = r8; r10 = ((r10).wrapping_sub(r9)) as u64; r10 = ((r10).wrapping_add(0x7u64)) as u64; r10 = ((r10).wrapping_shr(3u32)) as u64; fl_l = r9; fl_r = r8; r10 = (if (((fl_l as u64)) > ((fl_r as u64))) { rdx } else { r10 }) as u64; fl_l = r10; fl_r = r10; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } fl_l = r10; fl_r = (0x2u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 4; continue; } r11 = (crate::simd_ops::ru64(rbx)) as u64; rax = r9.wrapping_sub(0x8); rax = ((rax.wrapping_add(r10.wrapping_mul(8))) as u64) as u64; fl_l = r9; fl_r = rbx; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 2; continue; } fl_l = rax; fl_r = rbx; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 4; continue; } bb = 2; continue; }
            2 => { r8 = r10; fl_l = (((r8) & (0xfffffffffffffffeu64))) as u64; fl_r = fl_l; r8 = ((fl_l as u64)) as u64; bb = 3; continue; }
            3 => { rdx = ((rdx).wrapping_add(0x2u64)) as u64; fl_l = rdx; fl_r = r8; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 3; continue; } r8 = ((r8.wrapping_mul(8)) as u64) as u64; rdi = r9; rcx = r8; rax = r11; rcx = ((rcx).wrapping_shr(3u32)) as u64; while rcx != 0 { crate::simd_ops::wu64(rdi, rax); rdi = rdi.wrapping_add(8); rcx = rcx.wrapping_sub(1); } fl_l = ((r9).wrapping_add(r8)) as u64; fl_r = fl_l; r9 = ((fl_l as u64)) as u64; bb = 4; continue; }
            4 => { fl_l = rdx; fl_r = r10; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = ((rdx).wrapping_add(1)) as u64; crate::simd_ops::wu64(r9, rax); r9 = r9.wrapping_add(0x8); fl_l = rdx; fl_r = r10; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 5; continue; } bb = 6; continue; }
            6 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu32(rsi.wrapping_add(0x38u64), ((rbp as u32)) as u32); rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_33792<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rdi) = (0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; r11 = r8; r10 = 0; fl_l = 0; fl_r = 0; r8 = (crate::simd_ops::ru32(rcx.wrapping_add(0x24u64))) as u32 as u64; rdi = rdx; rbx = rcx; fl_l = ((r8 as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 4; continue; } r9 = (crate::simd_ops::ru64(rdx)) as u64; rcx = r9; rcx = ((rcx).wrapping_shl(32u32)) as u64; rcx = (!(rcx)) as u64; rcx = ((rcx).wrapping_add(r9)) as u64; rax = rcx; rax = ((rax).wrapping_shr(22u32)) as u64; rcx = (((rcx) ^ (rax))) as u64; rax = rcx; rax = ((rax).wrapping_shl(13u32)) as u64; rax = (!(rax)) as u64; rcx = ((rcx).wrapping_add(rax)) as u64; rax = rcx; rax = ((rax).wrapping_shr(8u32)) as u64; rax = (((rax) ^ (rcx))) as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rax = rcx; rax = ((rax).wrapping_shr(15u32)) as u64; rcx = (((rcx) ^ (rax))) as u64; rax = rcx; rax = ((rax).wrapping_shl(27u32)) as u64; rax = (!(rax)) as u64; rcx = ((rcx).wrapping_add(rax)) as u64; rax = ((r8.wrapping_sub(0x1u64)) as u32) as u32 as u64; r10 = rcx; r10 = ((r10).wrapping_shr(31u32)) as u64; r10 = ((((r10 as u32)) ^ ((rcx as u32)))) as u32 as u64; r10 = ((((r10 as u32)) & ((rax as u32)))) as u32 as u64; rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x18u64))) as u64; rdx = (crate::simd_ops::ru32(rax.wrapping_add(r10.wrapping_mul(4)))) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0xffffffff; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 3; continue; } r8 = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; bb = 1; continue; }
            1 => { rax = ((rdx as u32)) as u32 as u64; rax = ((rax).wrapping_add(rax)) as u64; rcx = ((rdx as u32)) as u32 as u64; fl_l = (crate::simd_ops::ru64(r8.wrapping_add(rax.wrapping_mul(8)))) as u64; fl_r = r9; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 2; continue; } rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; rdx = (crate::simd_ops::ru32(rax.wrapping_add(rcx.wrapping_mul(4)))) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0xffffffff; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 1; continue; } bb = 2; continue; }
            2 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0xffffffff; bb = 3; continue; }
            3 => { rax = (rax & !0xffu64) | (((if (((fl_l as u32)) != ((fl_r as u32))) { 1u8 } else { 0u8 }) as u8 as u64)); crate::simd_ops::wu8(r11, ((rax as u8)) as u8); if (((fl_l as u32)) == ((fl_r as u32))) { bb = 5; continue; } rax = ((rdx as u32)) as u32 as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rax = ((rax).wrapping_add(crate::simd_ops::ru64(rbx.wrapping_add(0x8u64)))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            4 => { crate::simd_ops::wu8(r11, ((r10 as u8)) as u8); bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x20u64))) as u32 as u64; fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x34u64))) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 9; continue; } r8 = (crate::simd_ops::ru32(rbx.wrapping_add(0x24u64))) as u32 as u64; fl_l = ((r8 as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 6; continue; } rdx = ((r8.wrapping_add(0x10u64)) as u32) as u32 as u64; bb = 7; continue; }
            6 => { rdx = ((r8.wrapping_add(r8)) as u32) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 8; continue; } bb = 7; continue; }
            7 => { rcx = rbx; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); r8 = (crate::simd_ops::ru32(rbx.wrapping_add(0x24u64))) as u32 as u64; bb = 8; continue; }
            8 => { rcx = (crate::simd_ops::ru64(rdi)) as u64; rax = rcx; rax = ((rax).wrapping_shl(32u32)) as u64; rax = (!(rax)) as u64; rcx = ((rcx).wrapping_add(rax)) as u64; rax = rcx; rax = ((rax).wrapping_shr(22u32)) as u64; rcx = (((rcx) ^ (rax))) as u64; rax = rcx; rax = ((rax).wrapping_shl(13u32)) as u64; rax = (!(rax)) as u64; rcx = ((rcx).wrapping_add(rax)) as u64; rax = rcx; rax = ((rax).wrapping_shr(8u32)) as u64; rax = (((rax) ^ (rcx))) as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rax = rcx; rax = ((rax).wrapping_shr(15u32)) as u64; rcx = (((rcx) ^ (rax))) as u64; rax = rcx; rax = ((rax).wrapping_shl(27u32)) as u64; rax = (!(rax)) as u64; rcx = ((rcx).wrapping_add(rax)) as u64; rax = ((r8.wrapping_sub(0x1u64)) as u32) as u32 as u64; r10 = rcx; r10 = ((r10).wrapping_shr(31u32)) as u64; r10 = ((((r10 as u32)) ^ ((rcx as u32)))) as u32 as u64; fl_l = ((((r10 as u32)) & ((rax as u32)))) as u32 as u64; fl_r = fl_l; r10 = ((fl_l as u32)) as u32 as u64; bb = 9; continue; }
            9 => { r8 = (crate::simd_ops::ru32(rbx.wrapping_add(0x2cu64))) as u32 as u64; rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x18u64))) as u64; rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; rax = ((r8.wrapping_add(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x2cu64), ((rax as u32)) as u32); rax = ((r8 as u32)) as u32 as u64; rcx = (crate::simd_ops::ru32(rcx.wrapping_add(r10.wrapping_mul(4)))) as u32 as u64; crate::simd_ops::wu32(rdx.wrapping_add(r8.wrapping_mul(4)), ((rcx as u32)) as u32); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x18u64))) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; crate::simd_ops::wu32(rcx.wrapping_add(r10.wrapping_mul(4)), ((r8 as u32)) as u32); crate::simd_ops::lk_add_u32(rbx.wrapping_add(0x34u64), (1) as u32); crate::simd_ops::lk_add_u32(rbx.wrapping_add(0x30u64), (1) as u32); rax = ((rax).wrapping_add(crate::simd_ops::ru64(rbx.wrapping_add(0x8u64)))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_33824<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(0xcu64))) as u32 as u64; rbx = r8; fl_l = ((((rax as u32)) >> (31u32)) & 1) as u64; rax = (((rax as u32)) & !((1u32) << (31u32))) as u32 as u64; rbp = ((rdx as u32)) as u32 as u64; rsi = rcx; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 1; continue; } rdx = ((rbp as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); bb = 1; continue; }
            1 => { rdi = (crate::simd_ops::ru64(rsi)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rax = (crate::simd_ops::ru32(rsi.wrapping_add(0x8u64))) as u32 as u64; r8 = ((rdi.wrapping_add(rbp.wrapping_mul(8))) as u64) as u64; r9 = ((rdi.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; r10 = r8; r10 = ((r10).wrapping_sub(r9)) as u64; r10 = ((r10).wrapping_add(0x7u64)) as u64; r10 = ((r10).wrapping_shr(3u32)) as u64; fl_l = r9; fl_r = r8; r10 = (if (((fl_l as u64)) > ((fl_r as u64))) { rdx } else { r10 }) as u64; fl_l = r10; fl_r = r10; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } fl_l = r10; fl_r = (0x2u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 4; continue; } r11 = (crate::simd_ops::ru64(rbx)) as u64; rax = r9.wrapping_sub(0x8); rax = ((rax.wrapping_add(r10.wrapping_mul(8))) as u64) as u64; fl_l = r9; fl_r = rbx; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 2; continue; } fl_l = rax; fl_r = rbx; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 4; continue; } bb = 2; continue; }
            2 => { r8 = r10; fl_l = (((r8) & (0xfffffffffffffffeu64))) as u64; fl_r = fl_l; r8 = ((fl_l as u64)) as u64; bb = 3; continue; }
            3 => { rdx = ((rdx).wrapping_add(0x2u64)) as u64; fl_l = rdx; fl_r = r8; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 3; continue; } r8 = ((r8.wrapping_mul(8)) as u64) as u64; rdi = r9; rcx = r8; rax = r11; rcx = ((rcx).wrapping_shr(3u32)) as u64; while rcx != 0 { crate::simd_ops::wu64(rdi, rax); rdi = rdi.wrapping_add(8); rcx = rcx.wrapping_sub(1); } fl_l = ((r9).wrapping_add(r8)) as u64; fl_r = fl_l; r9 = ((fl_l as u64)) as u64; bb = 4; continue; }
            4 => { fl_l = rdx; fl_r = r10; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = ((rdx).wrapping_add(1)) as u64; crate::simd_ops::wu64(r9, rax); r9 = r9.wrapping_add(0x8); fl_l = rdx; fl_r = r10; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 5; continue; } bb = 6; continue; }
            6 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu32(rsi.wrapping_add(0x8u64), ((rbp as u32)) as u32); rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_33831<const P0: u64, const P1: u64, const P2: u64, const P3: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rax = P0;
    rbx = rcx;
    crate::simd_ops::wu64(rcx, rax);
    rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64;
    fl_l = rcx; fl_r = rcx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P2)) as u64, rax, rcx, rdx, r8, r9, rsp); }
    rax = P3;
    crate::simd_ops::wu64(rbx, rax);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_33840<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x38u64)) as u64;
    rax = ((crate::simd_ops::ru8(rsp.wrapping_add(0x60u64))) as u32) as u32 as u64;
    crate::simd_ops::wu8(rsp.wrapping_add(0x28u64), ((rax as u8)) as u8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r9);
    r9 = ((r8 as u32)) as u32 as u64;
    r8 = rdx;
    rdx = rcx;
    rcx = (rcx & !0xffu64) | (((0x1u8) as u8 as u64));
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x38u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_33841<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    crate::simd_ops::wu32(rsp.wrapping_add(0x10u64), ((xmm1 as u32)) as u32);
    rsp = ((rsp).wrapping_sub(0x38u64)) as u64;
    crate::simd_ops::wu8(rsp.wrapping_add(0x28u64), ((r9 as u8)) as u8);
    rdx = rcx;
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r8);
    r9 = 0x1;
    r8 = rsp.wrapping_add(0x48);
    rcx = 0;
    fl_l = 0; fl_r = 0;
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    rsp = ((rsp).wrapping_add(0x38u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_33890<const P0: u64, const P1: u64, const P2: u64, const P3: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    fl_l = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; fl_r = (0x0u64) as u64;
    r9 = rcx;
    if !((((fl_l as u64)) == ((fl_r as u64)))) { r8 = (crate::simd_ops::ru64(rdx.wrapping_add(P1))) as u64; r8 = ((r8).wrapping_add(0xfu64)) as u64; fl_l = (((r8) & (0xfffffffffffffff0u64))) as u64; fl_r = fl_l; r8 = ((fl_l as u64)) as u64; rax = ((r8.wrapping_add(P2)) as u64) as u64; crate::simd_ops::wu64(rdx.wrapping_add(P1), rax); crate::simd_ops::wu64(rcx.wrapping_add(P0), r8); }
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(P1))) as u64;
    r8 = 0;
    fl_l = 0; fl_r = 0;
    rcx = (crate::simd_ops::ru32(rax)) as u32 as u64;
    rax = ((rax).wrapping_add(0x4u64)) as u64;
    crate::simd_ops::wu64(rdx.wrapping_add(P1), rax);
    fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64;
    rax = (if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { r8 } else { rax }) as u64;
    crate::simd_ops::wu64(r9.wrapping_add(P3), rax);
    fl_l = (crate::simd_ops::lk_add_u64(rdx.wrapping_add(P1), (rcx) as u64)) as u64; fl_r = fl_l;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_33891<const P0: u64, const P1: u64, const P2: u64, const P3: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rax = rdx;
    r8 = ((rcx.wrapping_add(P0)) as u64) as u64;
    rdx = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64;
    r9 = P2;
    rbx = rcx;
    rcx = rax;
    rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp);
    crate::simd_ops::wu64(rbx.wrapping_add(P1), rax);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_33892<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64, const P9: u64, const P10: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; r8 = 0; fl_l = 0; fl_r = 0; r10 = rdx; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 1; continue; } rdx = 0x2; rbx = rax; crate::simd_ops::wu16(rax.wrapping_add(P0), ((rdx as u16)) as u16); rdx = P1; crate::simd_ops::wu64(rax, rdx); rdx = P2; crate::simd_ops::wu64(rax.wrapping_add(P3), rdx); rax = (crate::simd_ops::ru64(rcx)) as u64; bb = 2; continue; }
            1 => { rbx = r8; bb = 2; continue; }
            2 => { rax = ((rax).wrapping_add(0x70u64)) as u64; crate::simd_ops::wu64(rcx, rax); fl_l = (crate::simd_ops::ru64(rbx.wrapping_add(P4))) as u64; fl_r = r8; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 3; continue; } rcx = (crate::simd_ops::ru64(r10.wrapping_add(P5))) as u64; rcx = ((rcx).wrapping_add(0xfu64)) as u64; fl_l = (((rcx) & (0xfffffffffffffff0u64))) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; rax = ((rcx.wrapping_add(P6)) as u64) as u64; crate::simd_ops::wu64(r10.wrapping_add(P5), rax); crate::simd_ops::wu64(rbx.wrapping_add(P4), rcx); bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(r10.wrapping_add(P5))) as u64; r9 = P7; rcx = (crate::simd_ops::ru32(rax)) as u32 as u64; rax = ((rax).wrapping_add(0x4u64)) as u64; crate::simd_ops::wu64(r10.wrapping_add(P5), rax); fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; rax = (if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { r8 } else { rax }) as u64; r8 = ((rbx.wrapping_add(P3)) as u64) as u64; crate::simd_ops::wu64(rbx.wrapping_add(P8), rax); crate::simd_ops::lk_add_u64(r10.wrapping_add(P5), (rcx) as u64); rcx = r10; rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P9))) as u64; rax = crate::simd_ops::call_stk(P10, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx.wrapping_add(P9), rax); rax = rbx; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_33894<const P0: u64, const P1: u64, const P2: u64, const P3: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    crate::simd_ops::forward_incoming_to_outgoing();
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    crate::simd_ops::wu32(rax.wrapping_add(P1), ((xmm1 as u32)) as u32);
    rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P2))) as u64;
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((crate::simd_ops::ru64(rax.wrapping_add(P3))) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::set_xmm_ret(c_x0); return c_rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_33896<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    rcx = ((crate::simd_ops::ru16(rax.wrapping_add(P1))) as u32) as u32 as u64;
    rax = rdx;
    crate::simd_ops::wu16(rdx, ((rcx as u16)) as u16);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_33897<const P0: u64, const P1: u64, const P2: u64, const P3: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    crate::simd_ops::forward_incoming_to_outgoing();
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    r8 = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    rax = ((crate::simd_ops::ru16(rdx)) as u32) as u32 as u64;
    crate::simd_ops::wu16(r8.wrapping_add(P1), ((rax as u16)) as u16);
    rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P2))) as u64;
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((crate::simd_ops::ru64(rax.wrapping_add(P3))) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_33898<const P0: u64, const P1: u64, const P2: u64, const P3: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    crate::simd_ops::forward_incoming_to_outgoing();
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    fl_l = ((r8 as u8)) as u8 as u64; fl_r = ((r8 as u8)) as u8 as u64;
    if !(((((fl_l as u8)) & ((fl_r as u8))) == 0)) { fl_l = (crate::simd_ops::lk_or_u16(rax.wrapping_add(P1), ((rdx as u16)) as u16)) as u16 as u64; fl_r = fl_l; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P2))) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((crate::simd_ops::ru64(rax.wrapping_add(P3))) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
    rdx = (rdx & !0xffffu64) | (((!((rdx as u16))) as u16 as u64));
    fl_l = (crate::simd_ops::lk_and_u16(rax.wrapping_add(P1), ((rdx as u16)) as u16)) as u16 as u64; fl_r = fl_l;
    rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P2))) as u64;
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((crate::simd_ops::ru64(rax.wrapping_add(P3))) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_33902<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    rbx = rdx;
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64, rax, rcx, rdx, r8, r9, rsp);
    rax = rbx;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_33904<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    xmm0 = ((crate::simd_ops::ru32(rax)) as u32) as u128;
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_33911<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rcx = P0; rbx = rdx; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 2; continue; } rcx = 0; fl_l = 0; fl_r = 0; rdx = P2; r8 = ((rcx as u32)) as u32 as u64; bb = 1; continue; }
            1 => { rax = ((crate::simd_ops::ru8(rdx.wrapping_add(r8))) as u32) as u32 as u64; r8 = ((r8).wrapping_add(1)) as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (crate::simd_ops::ru8(rbx.wrapping_add(r8).wrapping_add(P3))) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 3; continue; } fl_l = r8; fl_r = (0x8u64) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } bb = 2; continue; }
            2 => { rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            3 => { rdx = P4; bb = 4; continue; }
            4 => { rax = ((crate::simd_ops::ru8(rbx.wrapping_add(rcx))) as u32) as u32 as u64; rcx = ((rcx).wrapping_add(1)) as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (crate::simd_ops::ru8(rdx.wrapping_add(rcx).wrapping_add(P3))) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 5; continue; } fl_l = rcx; fl_r = (0x7u64) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 4; continue; } rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            5 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_33933<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    xmm0 = (crate::simd_ops::ru128(rax.wrapping_add(P1))) as u128;
    xmm1 = ((crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64) as u128;
    rax = (crate::simd_ops::ru32(rax.wrapping_add(P3))) as u32 as u64;
    crate::simd_ops::wu128(rdx, (xmm0) as u128);
    crate::simd_ops::wu64(rdx.wrapping_add(P4), ((xmm1 as u64)) as u64);
    crate::simd_ops::wu32(rdx.wrapping_add(P5), ((rax as u32)) as u32);
    rax = rdx;
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_33934<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    crate::simd_ops::forward_incoming_to_outgoing();
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    r8 = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    xmm0 = (crate::simd_ops::ru128(rdx)) as u128;
    crate::simd_ops::wu128(r8.wrapping_add(P1), (xmm0) as u128);
    xmm1 = ((crate::simd_ops::ru64(rdx.wrapping_add(P2))) as u64) as u128;
    crate::simd_ops::wu64(r8.wrapping_add(P3), ((xmm1 as u64)) as u64);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P4))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P5), ((rax as u32)) as u32);
    crate::simd_ops::wu8(rcx.wrapping_add(P6), (0x1u8) as u8);
    rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P7))) as u64;
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((crate::simd_ops::ru64(rax.wrapping_add(P8))) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::set_xmm_ret(c_x0); return c_rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_33948<const P0: u64, const P1: u64, const P2: u64, const P3: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    fl_l = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; fl_r = (0x0u64) as u64;
    rbx = rdx;
    rdi = rcx;
    if !((((fl_l as u64)) == ((fl_r as u64)))) { rax = (crate::simd_ops::ru64(rdx)) as u64; rcx = rbx; rdx = 0x10; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rbx)) as u64; r8 = 0x80; rdx = (crate::simd_ops::ru64(rdi.wrapping_add(P0))) as u64; rcx = rbx; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64, rax, rcx, rdx, r8, r9, rsp); }
    rax = (crate::simd_ops::ru64(rbx)) as u64;
    rcx = rbx;
    rdx = (crate::simd_ops::ru64(rdi.wrapping_add(P3))) as u64;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return crate::simd_ops::tail_stk((crate::simd_ops::ru64(rax.wrapping_add(P3))) as u64, rax, rcx, rdx, r8, r9, rsp);
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_33949<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    xmm0 = (crate::simd_ops::ru128(rax.wrapping_add(P1))) as u128;
    xmm1 = ((crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64) as u128;
    rax = (crate::simd_ops::ru32(rax.wrapping_add(P0))) as u32 as u64;
    crate::simd_ops::wu128(rdx, (xmm0) as u128);
    crate::simd_ops::wu64(rdx.wrapping_add(P3), ((xmm1 as u64)) as u64);
    crate::simd_ops::wu32(rdx.wrapping_add(P4), ((rax as u32)) as u32);
    rax = rdx;
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_33950<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    crate::simd_ops::forward_incoming_to_outgoing();
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    r8 = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    xmm0 = (crate::simd_ops::ru128(rdx)) as u128;
    crate::simd_ops::wu128(r8.wrapping_add(P1), (xmm0) as u128);
    xmm1 = ((crate::simd_ops::ru64(rdx.wrapping_add(P2))) as u64) as u128;
    crate::simd_ops::wu64(r8.wrapping_add(P3), ((xmm1 as u64)) as u64);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P4))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P0), ((rax as u32)) as u32);
    rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P3))) as u64;
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((crate::simd_ops::ru64(rax.wrapping_add(P5))) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::set_xmm_ret(c_x0); return c_rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_33951<const P0: u64, const P1: u64, const P2: u64, const P3: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rdi) = (0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x30u64)) as u64;
    rax = (crate::simd_ops::ru64(rdx)) as u64;
    r10 = r9;
    r9 = ((rcx.wrapping_add(P0)) as u64) as u64;
    crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), (0x80u32) as u32);
    rdi = rcx;
    r11 = r8;
    rbx = rdx;
    rcx = P1;
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rcx);
    r8 = r10;
    rcx = rbx;
    rdx = r11;
    rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64, rax, rcx, rdx, r8, r9, rsp);
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64;
    fl_l = rax; fl_r = rax;
    crate::simd_ops::wu64(rdi.wrapping_add(P3), rax);
    rax = (rax & !0xffu64) | (((if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { 1u8 } else { 0u8 }) as u8 as u64));
    rsp = ((rsp).wrapping_add(0x30u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_33959<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    r8 = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    rax = ((crate::simd_ops::ru16(rdx)) as u32) as u32 as u64;
    crate::simd_ops::wu16(r8.wrapping_add(P1), ((rax as u16)) as u16);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_33973<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rsp = ((rsp).wrapping_sub(0x48u64)) as u64;
    rcx = (crate::simd_ops::ru64(rdx)) as u64;
    rdx = rsp.wrapping_add(0x20);
    xmm0 = (crate::simd_ops::ru128(P0)) as u128;
    crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128);
    crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (0x0u64) as u64);
    crate::simd_ops::wu32(rsp.wrapping_add(0x38u64), (0x0u32) as u32);
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    (rax, xmm0) = crate::simd_ops::call_stk_xmm((crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    rsp = ((rsp).wrapping_add(0x48u64)) as u64;
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_33982<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 21>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0xa0u64)) as u64; rbx = rdx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r8 = 0x1; rdx = P1; rcx = rax; rdi = rax; r9 = (crate::simd_ops::ru64(rax)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm((crate::simd_ops::ru64(r9)) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = 0; fl_l = 0; fl_r = 0; rcx = rsp.wrapping_add(0x20); r8 = 0x80; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm3 = ((crate::simd_ops::ru32(rbx.wrapping_add(P3))) as u32) as u128; r8 = P4; xmm3 = crate::simd_ops::vcvtps2pd(xmm3); rdx = 0x80; rcx = rsp.wrapping_add(0x20); r9 = ((xmm3 as u64)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm((crate::simd_ops::ru64(P5)) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = (crate::simd_ops::ru8(rsp.wrapping_add(0x20u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 2; continue; } rax = (crate::simd_ops::ru64(rdi)) as u64; rcx = rsp.wrapping_add(0x20); r8 = (0xffffffffffffffffu64) as u64; bb = 1; continue; }
            1 => { r8 = ((r8).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(r8))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 1; continue; } rdx = rsp.wrapping_add(0x20); rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm((crate::simd_ops::ru64(rax)) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 2; continue; }
            2 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0xb0u64))) as u64; rsp = ((rsp).wrapping_add(0xa0u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_34001<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64, const P9: u64, const P10: u64, const P11: u64, const P12: u64, const P13: u64, const P14: u64, const P15: u64, const P16: u64, const P17: u64, const P18: u64, const P19: u64, const P20: u64, const P21: u64, const P22: u64, const P23: u64, const P24: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 21>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), r8); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rbp = rsp; rsp = ((rsp).wrapping_sub(0x70u64)) as u64; rax = (crate::simd_ops::ru64(rdx)) as u64; rcx = rdx; r12 = r9; rdi = r8; r13 = rdx; (rax, xmm0) = crate::simd_ops::call_stk_xmm((crate::simd_ops::ru64(rax.wrapping_add(0xa0u64))) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = (crate::simd_ops::ru64(rbp.wrapping_add(0x60u64))) as u64; rdx = P0; rcx = rbx; r15 = rax; r8 = (crate::simd_ops::ru32(rax.wrapping_add(0x8u64))) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r9 = (crate::simd_ops::ru64(rbx)) as u64; rdx = rbp.wrapping_add(0x60); r8 = 0x1; crate::simd_ops::wu8(rbp.wrapping_add(0x60u64), (0x0u8) as u8); rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm((crate::simd_ops::ru64(r9)) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru64(r12)) as u64; rdx = P2; r8 = (crate::simd_ops::ru64(rbx.wrapping_add(0x20u64))) as u64; rcx = r12; (rax, xmm0) = crate::simd_ops::call_stk_xmm((crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rsi = 0; fl_l = 0; fl_r = 0; rdx = P0; crate::simd_ops::wu64(rbx.wrapping_add(0x18u64), rsi); rcx = rbx; r8 = (crate::simd_ops::ru32(r15.wrapping_add(0xcu64))) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru64(rbx)) as u64; r8 = ((rsi.wrapping_add(0x1u64)) as u32) as u32 as u64; rdx = rbp.wrapping_add(0x60); crate::simd_ops::wu8(rbp.wrapping_add(0x60u64), ((rsi as u8)) as u8); rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm((crate::simd_ops::ru64(rax)) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru64(r12)) as u64; rdx = P3; r8 = (crate::simd_ops::ru64(rbx.wrapping_add(0x20u64))) as u64; rcx = r12; (rax, xmm0) = crate::simd_ops::call_stk_xmm((crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rbx.wrapping_add(0x18u64), rsi); rdx = rbp.wrapping_add(0x48); rax = (crate::simd_ops::ru64(r13)) as u64; rcx = r13; (rax, xmm0) = crate::simd_ops::call_stk_xmm((crate::simd_ops::ru64(rax.wrapping_add(0x1c0u64))) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r14 = (crate::simd_ops::ru32(r15.wrapping_add(0xcu64))) as u32 as u64; fl_l = ((r14 as u32)) as u32 as u64; fl_r = ((r14 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 8; continue; } rsi = (crate::simd_ops::ru64(r15.wrapping_add(0x10u64))) as u64; r14 = (((r14 as u32)).wrapping_sub(1)) as u32 as u64; r14 = (((r14 as u32)).wrapping_shr(5u32)) as u32 as u64; r14 = (((r14 as u32)).wrapping_add(1)) as u32 as u64; fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } rdi = 0; fl_l = 0; fl_r = 0; fl_l = ((r14 as u32)) as u32 as u64; fl_r = ((r14 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 4; continue; } bb = 1; continue; }
            1 => { fl_l = ((rdi as u32)) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 2; continue; } fl_l = ((rdi as u8)) as u8 as u64; fl_r = (0x7u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 2; continue; } r8 = 0x4; rdx = P4; bb = 3; continue; }
            2 => { r8 = 0x1; rdx = P5; bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm((crate::simd_ops::ru64(rax)) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r8 = (crate::simd_ops::ru32(rsi)) as u32 as u64; rdx = P0; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdi = (((rdi as u32)).wrapping_add(1)) as u32 as u64; rsi = ((rsi).wrapping_add(0x4u64)) as u64; fl_l = ((rdi as u32)) as u32 as u64; fl_r = ((r14 as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 1; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = rbp.wrapping_add(0x60); r8 = 0x1; crate::simd_ops::wu8(rbp.wrapping_add(0x60u64), (0x0u8) as u8); rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm((crate::simd_ops::ru64(rax)) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru64(r12)) as u64; rdx = P6; r8 = (crate::simd_ops::ru64(rbx.wrapping_add(0x20u64))) as u64; rcx = r12; (rax, xmm0) = crate::simd_ops::call_stk_xmm((crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdi = (crate::simd_ops::ru64(rbp.wrapping_add(0x50u64))) as u64; rsi = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rbx.wrapping_add(0x18u64), rsi); bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru32(r15.wrapping_add(0x8u64))) as u32 as u64; r14 = P7; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), r14); r9 = r15.wrapping_add(0x18); crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), (0x6u32) as u32); r8 = P8; rdx = rbx; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rax as u32)) as u32); rcx = r12; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P9, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = ((crate::simd_ops::ru16(rbp.wrapping_add(0x48u64))) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x2u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 6; continue; } rax = (crate::simd_ops::ru32(r15.wrapping_add(0x8u64))) as u32 as u64; r9 = r15.wrapping_add(0x28); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), r14); r8 = P10; crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), (0x6u32) as u32); rdx = rbx; rcx = r12; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rax as u32)) as u32); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P9, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = ((crate::simd_ops::ru16(rbp.wrapping_add(0x48u64))) as u32) as u32 as u64; bb = 6; continue; }
            6 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x4u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 7; continue; } rax = P11; rdx = rbx; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); r9 = r15.wrapping_add(0x38); rax = (crate::simd_ops::ru32(r15.wrapping_add(0x8u64))) as u32 as u64; r8 = P12; crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), (0x6u32) as u32); rcx = r12; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rax as u32)) as u32); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P9, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = ((crate::simd_ops::ru16(rbp.wrapping_add(0x48u64))) as u32) as u32 as u64; bb = 7; continue; }
            7 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x8u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 8; continue; } rax = P13; rdx = rbx; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); r9 = r15.wrapping_add(0x48); rax = (crate::simd_ops::ru32(r15.wrapping_add(0x8u64))) as u32 as u64; r8 = P14; crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), (0x6u32) as u32); rcx = r12; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rax as u32)) as u32); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P15, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 8; continue; }
            8 => { rax = (crate::simd_ops::ru64(r15)) as u64; rcx = r15; (rax, xmm0) = crate::simd_ops::call_stk_xmm((crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru64(r13)) as u64; rcx = r13; (rax, xmm0) = crate::simd_ops::call_stk_xmm((crate::simd_ops::ru64(rax.wrapping_add(0x178u64))) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = P0; rcx = rbx; r8 = ((rax as u32)) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = rbp.wrapping_add(0x60); r8 = 0x1; crate::simd_ops::wu8(rbp.wrapping_add(0x60u64), (0x0u8) as u8); rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm((crate::simd_ops::ru64(rax)) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru64(r12)) as u64; rdx = P16; r8 = (crate::simd_ops::ru64(rbx.wrapping_add(0x20u64))) as u64; rcx = r12; (rax, xmm0) = crate::simd_ops::call_stk_xmm((crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rbx.wrapping_add(0x18u64), (0x0u64) as u64); rdx = rbp.wrapping_add(0x58); rax = (crate::simd_ops::ru64(r13)) as u64; rcx = r13; (rax, xmm0) = crate::simd_ops::call_stk_xmm((crate::simd_ops::ru64(rax.wrapping_add(0x170u64))) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r9 = ((crate::simd_ops::ru16(rbp.wrapping_add(0x58u64))) as u32) as u32 as u64; rax = P17; r8 = P18; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); rdx = rbx; rcx = r12; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P19, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r9 = ((crate::simd_ops::ru16(rbp.wrapping_add(0x48u64))) as u32) as u32 as u64; rax = P20; r8 = P21; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); rdx = rbx; rcx = r12; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P19, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru64(r13)) as u64; r8 = rbp.wrapping_sub(0x30); rdx = rbp.wrapping_sub(0x28); rcx = r13; (rax, xmm0) = crate::simd_ops::call_stk_xmm((crate::simd_ops::ru64(rax.wrapping_add(0x108u64))) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm0 = ((crate::simd_ops::ru32(rbp.wrapping_sub(0x28u64))) as u32) as u128; rax = rbp.wrapping_sub(0x18); xmm1 = ((crate::simd_ops::ru32(rbp.wrapping_sub(0x24u64))) as u32) as u128; r9 = P22; crate::simd_ops::wu32(rbp.wrapping_sub(0x18u64), ((xmm0 as u32)) as u32); r8 = rbx; xmm0 = ((crate::simd_ops::ru32(rbp.wrapping_sub(0x20u64))) as u32) as u128; rdx = rdi; crate::simd_ops::wu32(rbp.wrapping_sub(0x14u64), ((xmm1 as u32)) as u32); rcx = r12; xmm1 = ((crate::simd_ops::ru32(rbp.wrapping_sub(0x30u64))) as u32) as u128; crate::simd_ops::wu32(rbp.wrapping_sub(0x10u64), ((xmm0 as u32)) as u32); crate::simd_ops::wu32(rbp.wrapping_sub(0xcu64), ((xmm1 as u32)) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P23, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r9 = rdi; r8 = rbx; rdx = r12; rcx = r13; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P24, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0xb0u64))) as u64; rsp = ((rsp).wrapping_add(0x70u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_34013<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r10) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 0>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    r10 = r9;
    rcx = rdx;
    r9 = r8;
    rdx = r10;
    r8 = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64;
    let _ = &stack; return crate::simd_ops::tail_stk(P0, rax, rcx, rdx, r8, r9, rsp);
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_34032<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = (crate::simd_ops::ru32(rdx)) as u32 as u64;
    crate::simd_ops::wu32(rcx, ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(P0), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P1))) as u32 as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(P1), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P2))) as u32 as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(P2), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P3))) as u32 as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(P3), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P4))) as u32 as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(P4), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P5))) as u32 as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(P5), ((rax as u32)) as u32);
    rax = rcx;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_34036<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = (crate::simd_ops::ru32(rdx)) as u32 as u64;
    crate::simd_ops::wu32(rcx, ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(P0), ((rax as u32)) as u32);
    rax = rcx;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_34040<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rax = (crate::simd_ops::ru64(rdx)) as u64;
    r9 = rdx;
    rbx = rcx;
    rdx = rcx;
    rcx = r9;
    rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64, rax, rcx, rdx, r8, r9, rsp);
    rax = rbx;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_34041<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rax = (crate::simd_ops::ru64(rdx)) as u64;
    r8 = rdx;
    rbx = rcx;
    rdx = rcx;
    rcx = r8;
    rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64, rax, rcx, rdx, r8, r9, rsp);
    rax = rbx;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_34042<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    rax = ((crate::simd_ops::ru16(rdx)) as u32) as u32 as u64;
    rdx = rsp.wrapping_add(0x30);
    crate::simd_ops::wu16(rsp.wrapping_add(0x30u64), ((rax as u16)) as u16);
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_34044(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    xmm0 = ((crate::simd_ops::ru32(rcx)) as u32) as u128;
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_34045<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = (crate::simd_ops::ru32(rdx)) as u32 as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(P0), ((rax as u32)) as u32);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_34105<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64;
    fl_l = ((rax as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64;
    rax = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_34179<const P0: u64, const P1: u64, const P2: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; fl_r = 0x0;
    if !((((fl_l as u32)) <= ((fl_r as u32)))) { crate::simd_ops::wu32(rcx.wrapping_add(P1), ((xmm1 as u32)) as u32); crate::simd_ops::wu8(rcx.wrapping_add(P2), (0x1u8) as u8); }
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_34181<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    crate::simd_ops::forward_incoming_to_outgoing();
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((crate::simd_ops::ru64(P0)) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::set_xmm_ret(c_x0); return c_rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_34214<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = rsp; crate::simd_ops::wu64(rax.wrapping_add(P0), rbx); crate::simd_ops::wu64(rax.wrapping_add(P1), rbp); crate::simd_ops::wu64(rax.wrapping_add(P2), rsi); crate::simd_ops::wu64(rax.wrapping_add(P3), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = (crate::simd_ops::ru64(r9.wrapping_add(P4))) as u64; rsi = rdx; r14 = r8; rbp = rcx; rdx = r9; rcx = rsi; rdi = r9; r8 = ((rbx.wrapping_add(P5)) as u64) as u64; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x4u64))) as u32 as u64; rax = (rax & !0xffu64) | ((((((rax as u8)) & (0x66u8))) as u8 as u64)); fl_l = 0; fl_r = ((rax as u8)) as u8 as u64; rax = (rax & !0xffu64) | (((0u8.wrapping_sub(fl_r as u8)) as u8 as u64)); rax = 0x1; r8 = (if (((fl_l as u8)) < ((fl_r as u8))) { 0xffffffffu32 } else { 0u32 }) as u32 as u64; r8 = (0u32.wrapping_sub((r8 as u32))) as u32 as u64; r8 = (((r8 as u32)).wrapping_add((rax as u32))) as u32 as u64; fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(P5))) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 1; continue; } r9 = rdi; r8 = r14; rdx = rsi; rcx = rbp; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); bb = 1; continue; }
            1 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_34229<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = P0; rdi = P0; bb = 3; continue; }
            1 => { rax = (crate::simd_ops::ru64(rbx)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } let _ = P1; rax = crate::simd_ops::call_stk(rax, rax, rcx, rdx, r8, r9, rsp); bb = 2; continue; }
            2 => { fl_l = ((rbx).wrapping_add(0x8u64)) as u64; fl_r = fl_l; rbx = ((fl_l as u64)) as u64; bb = 3; continue; }
            3 => { fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 1; continue; } rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_34252(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 0>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rax = ((crate::simd_ops::ru8(rsp.wrapping_add(0x8u64))) as u32) as u32 as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_34256<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    crate::simd_ops::forward_incoming_to_outgoing();
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    fl_l = rdx; fl_r = rdx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rcx = rdx; crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8(P0, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_34257<const P0: u64, const P1: u64, const P2: u64, const P3: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut ymm0_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x40u64)) as u64;
    rcx = (crate::simd_ops::ru64(P0)) as u64;
    rbx = rdx;
    rdx = 0x1;
    rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64;
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    let y_addr: u64 = rbx; xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16));
    let y_addr: u64 = rsp.wrapping_add(0x20u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi);
    rcx = rsp.wrapping_add(0x20);
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    rsp = ((rsp).wrapping_add(0x40u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_34259(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    crate::simd_ops::wu64(rcx, (0x0u64) as u64);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_34265<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = (crate::simd_ops::ru64(P0)) as u64;
    rax = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_34268<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    crate::simd_ops::wu8(rcx.wrapping_add(P0), (0x0u8) as u8);
    rax = rcx;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_34269<const P0: u64, const P1: u64, const P2: u64, const P3: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rax = P0;
    crate::simd_ops::wu64(rcx, rax);
    rbx = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64;
    fl_l = rbx; fl_r = rbx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rcx = rbx; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rcx = rbx; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); }
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_34312<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rax = (crate::simd_ops::ru64(rdx)) as u64;
    rbx = rcx;
    crate::simd_ops::wu64(rcx, rax);
    rdx = ((rdx).wrapping_add(0x8u64)) as u64;
    rcx = ((rcx).wrapping_add(0x8u64)) as u64;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rax = rbx;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_34313<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    crate::simd_ops::forward_incoming_to_outgoing();
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rcx = (crate::simd_ops::ru64(rcx)) as u64;
    fl_l = rcx; fl_r = rcx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rdx = 0x1c0; crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8(P0, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_34318(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = rcx;
    fl_l = rcx; fl_r = rcx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rax = ((rcx.wrapping_add(rdx)) as u64) as u64; }
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_34319<const P0: u64, const P1: u64, const P2: u64, const P3: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    r11 = rdx;
    rbx = (((r8 as u16)) as u32) as u32 as u64;
    r8 = ((crate::simd_ops::ru16(P0)) as u32) as u32 as u64;
    rax = ((rbx as u32)) as u32 as u64;
    rax = ((rax).wrapping_shl(5u32)) as u64;
    rdx = (crate::simd_ops::ru32(rax.wrapping_add(rdx).wrapping_add(P1))) as u32 as u64;
    rcx = (crate::simd_ops::ru64(rax.wrapping_add(r11).wrapping_add(0x18058u64))) as u64;
    rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp);
    rcx = ((((rax as u32)) as i32 as i64) as u64) as u64;
    fl_l = rcx; fl_r = (0xffffffffffffffffu64) as u64;
    if !((((fl_l as u64)) == ((fl_r as u64)))) { rax = ((rbx.wrapping_add(P3)) as u64) as u64; rax = ((rax).wrapping_shl(5u32)) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(r11))) as u64; rax = (crate::simd_ops::ru32(rax.wrapping_add(rcx.wrapping_mul(4)))) as u32 as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    rax = 0;
    fl_l = 0; fl_r = 0;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_34321<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    crate::simd_ops::wu8(rcx.wrapping_add(P0), (0x1u8) as u8);
    crate::simd_ops::wu32(rcx.wrapping_add(P1), ((rdx as u32)) as u32);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_34323<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    crate::simd_ops::wu64(rax.wrapping_add(P0), (0x0u64) as u64);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_34353<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = rdx; rsi = rcx; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rcx); r8 = rsp.wrapping_add(0x30); rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rbp = rax; r10 = (crate::simd_ops::ru32(rdi.wrapping_add(0x30u64))) as u32 as u64; r9 = (crate::simd_ops::ru32(rdi.wrapping_add(0x34u64))) as u32 as u64; r8 = ((r9 as u32)) as u32 as u64; r8 = ((r8).wrapping_shl(4u32)) as u64; rcx = ((r10 as u32)) as u32 as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; fl_l = rcx; fl_r = r8; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } rcx = r10.wrapping_add(0x1); rdx = ((r9 as u32)) as u32 as u64; rdx = ((rdx).wrapping_shr(1u32)) as u64; rdx = ((rdx).wrapping_add(r9)) as u64; fl_l = rdx; fl_r = rcx; rdx = (if (((fl_l as u64)) < ((fl_r as u64))) { rcx } else { rdx }) as u64; rcx = rdi.wrapping_add(0x28); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x30u64))) as u32 as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rax = ((rax).wrapping_add(crate::simd_ops::ru64(rdi.wrapping_add(0x28u64)))) as u64; crate::simd_ops::wu64(rax, rbp); rcx = P2; crate::simd_ops::wu64(rax.wrapping_add(0x8u64), rcx); crate::simd_ops::lk_add_u32(rdi.wrapping_add(0x30u64), (1) as u32); rdx = (crate::simd_ops::ru32(rdi.wrapping_add(0x40u64))) as u32 as u64; r8 = (crate::simd_ops::ru32(rdi.wrapping_add(0x44u64))) as u32 as u64; rcx = ((r8 as u32)) as u32 as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rax = ((rdx as u32)) as u32 as u64; rax = ((rax).wrapping_shl(4u32)) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } rax = rdx.wrapping_add(0x1); rdx = ((r8 as u32)) as u32 as u64; rdx = ((rdx).wrapping_shr(1u32)) as u64; rdx = ((rdx).wrapping_add(r8)) as u64; fl_l = rdx; fl_r = rax; rdx = (if (((fl_l as u64)) < ((fl_r as u64))) { rax } else { rdx }) as u64; rcx = rdi.wrapping_add(0x38); rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x40u64))) as u32 as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rax = ((rax).wrapping_add(crate::simd_ops::ru64(rdi.wrapping_add(0x38u64)))) as u64; rcx = P4; crate::simd_ops::wu64(rax, rcx); crate::simd_ops::wu64(rax.wrapping_add(0x8u64), rbp); crate::simd_ops::lk_add_u32(rdi.wrapping_add(0x40u64), (1) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rsi); rbx = rdi.wrapping_add(0x68); r8 = rsp.wrapping_add(0x30); rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rdi = rax; r8 = (crate::simd_ops::ru32(rbx.wrapping_add(0x8u64))) as u32 as u64; rax = (crate::simd_ops::ru32(rbx.wrapping_add(0xcu64))) as u32 as u64; rdx = ((rax as u32)) as u32 as u64; rdx = ((rdx).wrapping_shl(4u32)) as u64; rcx = ((r8 as u32)) as u32 as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; fl_l = rcx; fl_r = rdx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } rcx = r8.wrapping_add(0x1); rdx = ((rax as u32)) as u32 as u64; rdx = ((rdx).wrapping_shr(1u32)) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; fl_l = rdx; fl_r = rcx; rdx = (if (((fl_l as u64)) < ((fl_r as u64))) { rcx } else { rdx }) as u64; rcx = rbx; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x8u64))) as u32 as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rax = ((rax).wrapping_add(crate::simd_ops::ru64(rbx))) as u64; crate::simd_ops::wu64(rax, rdi); rcx = P5; crate::simd_ops::wu64(rax.wrapping_add(0x8u64), rcx); crate::simd_ops::lk_add_u32(rbx.wrapping_add(0x8u64), (1) as u32); rdx = (crate::simd_ops::ru32(rbx.wrapping_add(0x18u64))) as u32 as u64; r8 = (crate::simd_ops::ru32(rbx.wrapping_add(0x1cu64))) as u32 as u64; rcx = ((r8 as u32)) as u32 as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rax = ((rdx as u32)) as u32 as u64; rax = ((rax).wrapping_shl(4u32)) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 4; continue; } rax = rdx.wrapping_add(0x1); rdx = ((r8 as u32)) as u32 as u64; rdx = ((rdx).wrapping_shr(1u32)) as u64; rdx = ((rdx).wrapping_add(r8)) as u64; fl_l = rdx; fl_r = rax; rdx = (if (((fl_l as u64)) < ((fl_r as u64))) { rax } else { rdx }) as u64; rcx = rbx.wrapping_add(0x10); rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x18u64))) as u32 as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rax = ((rax).wrapping_add(crate::simd_ops::ru64(rbx.wrapping_add(0x10u64)))) as u64; rcx = P6; crate::simd_ops::wu64(rax, rcx); crate::simd_ops::wu64(rax.wrapping_add(0x8u64), rdi); crate::simd_ops::lk_add_u32(rbx.wrapping_add(0x18u64), (1) as u32); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_34362<const P0: u64, const P1: u64, const P2: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x40u64)) as u64;
    r8 = ((crate::simd_ops::ru16(P0)) as u32) as u32 as u64;
    rcx = rsp.wrapping_add(0x20);
    rbx = rdx;
    rdx = (crate::simd_ops::ru64(P1)) as u64;
    rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp);
    rax = ((crate::simd_ops::ru8(rsp.wrapping_add(0x38u64))) as u32) as u32 as u64;
    fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64;
    if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { rsp = ((rsp).wrapping_add(0x40u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    fl_l = (crate::simd_ops::ru8(rsp.wrapping_add(0x30u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64;
    if !((((fl_l as u8)) != ((fl_r as u8)))) { rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(rbx, rcx); }
    rsp = ((rsp).wrapping_add(0x40u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_34373<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rcx;
    rax = P0;
    crate::simd_ops::wu64(rcx, rax);
    rax = (crate::simd_ops::ru64(rdx)) as u64;
    crate::simd_ops::wu64(rcx.wrapping_add(P1), rax);
    rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp);
    crate::simd_ops::wu64(rbx.wrapping_add(P3), rax);
    crate::simd_ops::wu16(rbx.wrapping_add(P4), (0x0u16) as u16);
    rax = P5;
    crate::simd_ops::wu64(rbx, rax);
    rax = rbx;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_34374<const P0: u64, const P1: u64, const P2: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l) = (crate::simd_ops::get_rax_in(), 0u64);
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    crate::simd_ops::wu64(rdx, rax);
    crate::simd_ops::wu8(rcx.wrapping_add(P1), (0x1u8) as u8);
    crate::simd_ops::wu32(rdx.wrapping_add(P2), ((r8 as u32)) as u32);
    fl_l = (crate::simd_ops::atomic_add_u32(rax, 1)) as u32 as u64;
    rax = rdx;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_34387<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64, const P9: u64, const P10: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(P1))) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 1; continue; } fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(P2))) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 4; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru32(P3)) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(P1), ((rax as u32)) as u32); fl_l = (crate::simd_ops::ru64(rbx.wrapping_add(P4))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (0x10000u64) as u64); r8 = rsp.wrapping_add(0x30); rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx.wrapping_add(P4), rax); bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(P6))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 3; continue; } crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (0x10000u64) as u64); r8 = rsp.wrapping_add(0x30); rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx.wrapping_add(P6), rax); bb = 3; continue; }
            3 => { crate::simd_ops::wu32(rax.wrapping_add(P7), (0x1u32) as u32); rcx = (crate::simd_ops::ru64(rax.wrapping_add(P8))) as u64; crate::simd_ops::wu64(rax.wrapping_add(P9), rcx); crate::simd_ops::wu64(rax.wrapping_add(P10), (0x0u64) as u64); bb = 4; continue; }
            4 => { rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_34393(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut ymm0_hi: u128 = 0;
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let y_addr: u64 = rcx; xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16));
    rax = ((r8 as u32)) as u32 as u64;
    fl_l = ((rax).wrapping_shl(5u32)) as u64;
    rax = ((fl_l as u64)) as u64;
    let y_addr: u64 = rax.wrapping_add(rdx); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi);
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_34394<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    xmm0 = (crate::simd_ops::ru128(rcx)) as u128;
    rax = ((r8 as u32)) as u32 as u64;
    r8 = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64;
    crate::simd_ops::wu128(rdx.wrapping_add(r8.wrapping_mul(8)), (xmm0) as u128);
    xmm1 = ((crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64) as u128;
    crate::simd_ops::wu64(rdx.wrapping_add(r8.wrapping_mul(8)).wrapping_add(P0), ((xmm1 as u64)) as u64);
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_34395<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    xmm0 = (crate::simd_ops::ru128(rcx)) as u128;
    rax = ((r8 as u32)) as u32 as u64;
    r8 = (((rax) as i64).wrapping_mul((0x1cu64) as i64) as u64) as u64;
    crate::simd_ops::wu128(r8.wrapping_add(rdx), (xmm0) as u128);
    xmm1 = ((crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64) as u128;
    crate::simd_ops::wu64(r8.wrapping_add(rdx).wrapping_add(P0), ((xmm1 as u64)) as u64);
    rax = (crate::simd_ops::ru32(rcx.wrapping_add(P1))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(rdx).wrapping_add(P1), ((rax as u32)) as u32);
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_34396<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = (crate::simd_ops::ru64(P0)) as u64;
    crate::simd_ops::wu8(rax.wrapping_add(P1), (0x1u8) as u8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_34423<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    crate::simd_ops::forward_incoming_to_outgoing();
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rcx = (crate::simd_ops::ru64(rcx)) as u64;
    rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8(P1, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_34429<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (0u64, 0u64, 0u64);
    fl_l = (crate::simd_ops::ru16(rdx.wrapping_add(P0))) as u16 as u64; fl_r = (0x0u16) as u16 as u64;
    rax = (rax & !0xffu64) | (((if (((fl_l as u16)) != ((fl_r as u16))) { 1u8 } else { 0u8 }) as u8 as u64));
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_34443(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    crate::simd_ops::wu64(rcx, rdx);
    rax = rcx;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_34448<const P0: u64, const P1: u64, const P2: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    xmm0 = (crate::simd_ops::ru128(rdx)) as u128;
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu128(rcx, (xmm0) as u128);
    crate::simd_ops::wu64(rcx.wrapping_add(P0), rax);
    crate::simd_ops::wu64(rcx.wrapping_add(P1), rax);
    crate::simd_ops::wu64(rcx.wrapping_add(P2), rax);
    rax = rcx;
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_34449<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
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
    let _ = &stack; return crate::simd_ops::tail_stk(P1, rax, rcx, rdx, r8, r9, rsp);
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_34469<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rax = P0;
    rbx = rcx;
    crate::simd_ops::wu64(rcx, rax);
    rax = (crate::simd_ops::ru64(rdx)) as u64;
    crate::simd_ops::wu64(rcx.wrapping_add(P1), rax);
    rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp);
    crate::simd_ops::wu64(rbx.wrapping_add(P3), rax);
    rax = rbx;
    crate::simd_ops::wu16(rbx.wrapping_add(P4), (0x0u16) as u16);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_34471<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rdi = rcx;
    rbx = ((rdx as u32)) as u32 as u64;
    rcx = ((rcx).wrapping_add(0x10u64)) as u64;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    fl_l = ((rbx as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64;
    if !(((((fl_l as u8)) & ((fl_r as u8))) == 0)) { rdx = 0x20; rcx = rdi; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); }
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rax = rdi;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_34472<const P0: u64, const P1: u64, const P2: u64, const P3: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l) = (crate::simd_ops::get_rax_in(), 0u64);
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    crate::simd_ops::wu64(rdx, rax);
    crate::simd_ops::wu8(rcx.wrapping_add(P1), (0x1u8) as u8);
    crate::simd_ops::wu32(rdx.wrapping_add(P2), (0x0u32) as u32);
    fl_l = (crate::simd_ops::atomic_add_u32(rax.wrapping_add(P3), 1)) as u32 as u64;
    rax = rdx;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_34474<const P0: u64, const P1: u64, const P2: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l) = (crate::simd_ops::get_rax_in(), 0u64);
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    crate::simd_ops::wu64(rdx, rax);
    crate::simd_ops::wu8(rcx.wrapping_add(P1), (0x1u8) as u8);
    crate::simd_ops::wu32(rdx.wrapping_add(P2), (0x0u32) as u32);
    fl_l = (crate::simd_ops::atomic_add_u32(rax, 1)) as u32 as u64;
    rax = rdx;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_34482<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P1)) as u64, rax, rcx, rdx, r8, r9, rsp);
    fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64;
    rax = (rax & !0xffu64) | (((if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { 1u8 } else { 0u8 }) as u8 as u64));
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_34520<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    crate::simd_ops::forward_incoming_to_outgoing();
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    fl_l = (crate::simd_ops::ru8(rcx)) as u8 as u64; fl_r = (0x0u8) as u8 as u64;
    if !((((fl_l as u8)) == ((fl_r as u8)))) { rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((crate::simd_ops::ru64(P1)) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_34584<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rax = P0;
    rbx = rcx;
    crate::simd_ops::wu64(rcx, rax);
    xmm0 = (crate::simd_ops::ru128(rdx)) as u128;
    crate::simd_ops::wu128(rcx.wrapping_add(P1), (xmm0) as u128);
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    crate::simd_ops::wu64(rbx.wrapping_add(P3), rax);
    rax = rbx;
    crate::simd_ops::wu16(rbx.wrapping_add(P4), (0x0u16) as u16);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_34585<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rdi = rcx;
    rbx = ((rdx as u32)) as u32 as u64;
    rcx = ((rcx).wrapping_add(0x18u64)) as u64;
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
pub unsafe extern "win64" fn tpl_34602<const P0: u64, const P1: u64, const P2: u64, const P3: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; rbx = rcx; r8 = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; rcx = r8; r9 = ((rax.wrapping_mul(8)) as u64) as u64; rax = ((r8.wrapping_add(r9)) as u64) as u64; fl_l = r8; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru64(rcx)) as u64; fl_r = rdx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 2; continue; } rcx = ((rcx).wrapping_add(0x8u64)) as u64; fl_l = rcx; fl_r = rax; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } bb = 2; continue; }
            2 => { rdx = ((rcx.wrapping_add(P2)) as u64) as u64; r8 = ((r8).wrapping_sub(rdx)) as u64; r8 = ((r8).wrapping_add(r9)) as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::lk_sub_u32(rbx.wrapping_add(P0), (1) as u32); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_34603<const P0: u64, const P1: u64, const P2: u64, const P3: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rdi = rdx;
    rbx = ((rcx.wrapping_add(P0)) as u64) as u64;
    rdx = (crate::simd_ops::ru32(rbx.wrapping_add(P1))) as u32 as u64;
    rcx = (crate::simd_ops::ru32(rbx.wrapping_add(P2))) as u32 as u64;
    rcx = ((rcx).wrapping_shl(3u32)) as u64;
    rax = ((rdx.wrapping_mul(8)) as u64) as u64;
    fl_l = rax; fl_r = rcx;
    if !((((fl_l as u64)) != ((fl_r as u64)))) { rdx = ((rdx).wrapping_add(1)) as u64; rcx = rbx; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); }
    rcx = (crate::simd_ops::ru32(rbx.wrapping_add(P1))) as u32 as u64;
    rax = (crate::simd_ops::ru64(rbx)) as u64;
    crate::simd_ops::wu64(rax.wrapping_add(rcx.wrapping_mul(8)), rdi);
    crate::simd_ops::lk_add_u32(rbx.wrapping_add(P1), (1) as u32);
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_34605<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { xmm1 = (crate::simd_ops::ru128(rcx)) as u128; rcx = (crate::simd_ops::ru64(rcx)) as u64; r10 = r9; rax = (((xmm1 >> 64) as u64)) as u64; rcx = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; rax = (((xmm1 >> 64) as u64)) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 4; continue; } r11 = P0; bb = 1; continue; }
            1 => { r9 = (crate::simd_ops::ru64(rax)) as u64; r8 = r9; r8 = ((r8).wrapping_shr(32u32)) as u64; fl_l = ((r9 as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 3; continue; } fl_l = ((r8 as u32)) as u32 as u64; fl_r = ((r9 as u32)) as u32 as u64; r8 = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; r9 = ((r8 as u32)) as u32 as u64; bb = 2; continue; }
            2 => { r8 = ((rdx as u32)) as u32 as u64; rdx = (((rdx as u32)).wrapping_add(1)) as u32 as u64; crate::simd_ops::wu64(r10.wrapping_add(r8.wrapping_mul(8)), r11); fl_l = r9; fl_r = (0x1u64) as u64; r9 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 3; continue; }
            3 => { rax = ((rax).wrapping_add(0x8u64)) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } bb = 4; continue; }
            4 => { return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_34607<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
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
    loop { rcx = rbx; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rbx = ((rbx).wrapping_add(0x18u64)) as u64; fl_l = rdi; fl_r = (0x1u64) as u64; rdi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if !((((fl_l as u64)) != ((fl_r as u64)))) { break; } }
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_34610<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut ymm0_hi: u128 = 0;
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let y_addr: u64 = rcx; xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16));
    rax = ((r8 as u32)) as u32 as u64;
    r8 = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64;
    let y_addr: u64 = rdx.wrapping_add(r8.wrapping_mul(8)); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi);
    xmm1 = ((crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64) as u128;
    crate::simd_ops::wu64(rdx.wrapping_add(r8.wrapping_mul(8)).wrapping_add(P0), ((xmm1 as u64)) as u64);
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_34611<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    xmm0 = ((crate::simd_ops::ru64(rcx)) as u64) as u128;
    rax = ((r8 as u32)) as u32 as u64;
    r8 = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64;
    crate::simd_ops::wu64(rdx.wrapping_add(r8.wrapping_mul(4)), ((xmm0 as u64)) as u64);
    rax = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64;
    crate::simd_ops::wu32(rdx.wrapping_add(r8.wrapping_mul(4)).wrapping_add(P0), ((rax as u32)) as u32);
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_34612(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    xmm0 = ((crate::simd_ops::ru64(rcx)) as u64) as u128;
    rax = ((r8 as u32)) as u32 as u64;
    crate::simd_ops::wu64(rdx.wrapping_add(rax.wrapping_mul(8)), ((xmm0 as u64)) as u64);
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_34613(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    xmm0 = ((crate::simd_ops::ru32(rcx)) as u32) as u128;
    rax = ((r8 as u32)) as u32 as u64;
    crate::simd_ops::wu32(rdx.wrapping_add(rax.wrapping_mul(4)), ((xmm0 as u32)) as u32);
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_34615(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
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
    loop { rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = rbx; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(rax)) as u64, rax, rcx, rdx, r8, r9, rsp); rbx = ((rbx).wrapping_sub(0xffffffffffffff80u64)) as u64; fl_l = rdi; fl_r = (0x1u64) as u64; rdi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if !((((fl_l as u64)) != ((fl_r as u64)))) { break; } }
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_34624<const P0: u64, const P1: u64, const P2: u64, const P3: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rdx; rdx = (crate::simd_ops::ru64(rcx.wrapping_add(0x10u64))) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; r12 = ((rax.wrapping_add(rdx.wrapping_mul(8))) as u64) as u64; rbp = (crate::simd_ops::ru64(r12.wrapping_add(0x10u64))) as u64; rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x58u64))) as u64; rcx = rax; rcx = ((rcx).wrapping_shr(32u32)) as u64; r14 = 0; fl_l = 0; fl_r = 0; fl_l = ((rbp as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 1; continue; } rax = rbp; rax = ((rax).wrapping_shr(32u32)) as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; rcx = ((r14 as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rcx = 0x1; bb = 2; continue; }
            2 => { crate::simd_ops::lk_add_u64(rbx.wrapping_add(0x60u64), (rcx) as u64); crate::simd_ops::wu64(rbx.wrapping_add(0x58u64), rbp); rdi = ((crate::simd_ops::ru16(P0)) as u32) as u32 as u64; rsi = rbx.wrapping_add(0x158); rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } rax = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; rax = ((rax).wrapping_add(1)) as u64; fl_l = rax; fl_r = (0x4000u64) as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rcx = rsi; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 4; continue; }
            4 => { rcx = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu32(rcx.wrapping_add(rax.wrapping_mul(4)).wrapping_add(0x10u64), ((rbp as u32)) as u32); rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; crate::simd_ops::lk_add_u64(rax.wrapping_add(0x8u64), (1) as u64); crate::simd_ops::lk_add_u64(rsi.wrapping_add(0x10u64), (1) as u64); rbp = rbx.wrapping_add(0x170); rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x8u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } rax = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; rax = ((rax).wrapping_add(1)) as u64; fl_l = rax; fl_r = (0x8000u64) as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rcx = rbp; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 6; continue; }
            6 => { rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x8u64))) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu16(rcx.wrapping_add(rax.wrapping_mul(2)).wrapping_add(0x10u64), ((rdi as u16)) as u16); rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x8u64))) as u64; crate::simd_ops::lk_add_u64(rax.wrapping_add(0x8u64), (1) as u64); crate::simd_ops::lk_add_u64(rbp.wrapping_add(0x10u64), (1) as u64); r15 = (crate::simd_ops::ru64(r12.wrapping_add(0x10u64))) as u64; rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x58u64))) as u64; rcx = rax; rcx = ((rcx).wrapping_shr(32u32)) as u64; fl_l = ((r15 as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 7; continue; } rax = r15; rax = ((rax).wrapping_shr(32u32)) as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; rcx = r14; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 8; continue; } bb = 7; continue; }
            7 => { rcx = 0x1; bb = 8; continue; }
            8 => { crate::simd_ops::lk_add_u64(rbx.wrapping_add(0x60u64), (rcx) as u64); crate::simd_ops::wu64(rbx.wrapping_add(0x58u64), r15); rdi = ((crate::simd_ops::ru16(P2)) as u32) as u32 as u64; rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } rax = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; rax = ((rax).wrapping_add(1)) as u64; fl_l = rax; fl_r = (0x4000u64) as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 10; continue; } bb = 9; continue; }
            9 => { rcx = rsi; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 10; continue; }
            10 => { rcx = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu32(rcx.wrapping_add(rax.wrapping_mul(4)).wrapping_add(0x10u64), ((r15 as u32)) as u32); rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; crate::simd_ops::lk_add_u64(rax.wrapping_add(0x8u64), (1) as u64); crate::simd_ops::lk_add_u64(rsi.wrapping_add(0x10u64), (1) as u64); rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x8u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } rax = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; rax = ((rax).wrapping_add(1)) as u64; fl_l = rax; fl_r = (0x8000u64) as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 12; continue; } bb = 11; continue; }
            11 => { rcx = rbp; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 12; continue; }
            12 => { rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x8u64))) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu16(rcx.wrapping_add(rax.wrapping_mul(2)).wrapping_add(0x10u64), ((rdi as u16)) as u16); rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x8u64))) as u64; crate::simd_ops::lk_add_u64(rax.wrapping_add(0x8u64), (1) as u64); crate::simd_ops::lk_add_u64(rbp.wrapping_add(0x10u64), (1) as u64); rdi = (crate::simd_ops::ru64(r12.wrapping_add(0x10u64))) as u64; rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x58u64))) as u64; rcx = rax; rcx = ((rcx).wrapping_shr(32u32)) as u64; fl_l = ((rdi as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 13; continue; } rax = rdi; rax = ((rax).wrapping_shr(32u32)) as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 14; continue; } bb = 13; continue; }
            13 => { r14 = 0x1; bb = 14; continue; }
            14 => { crate::simd_ops::lk_add_u64(rbx.wrapping_add(0x60u64), (r14) as u64); crate::simd_ops::wu64(rbx.wrapping_add(0x58u64), rdi); rbx = ((crate::simd_ops::ru16(P3)) as u32) as u32 as u64; rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 15; continue; } rax = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; rax = ((rax).wrapping_add(1)) as u64; fl_l = rax; fl_r = (0x4000u64) as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 16; continue; } bb = 15; continue; }
            15 => { rcx = rsi; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 16; continue; }
            16 => { rcx = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu32(rcx.wrapping_add(rax.wrapping_mul(4)).wrapping_add(0x10u64), ((rdi as u32)) as u32); rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; crate::simd_ops::lk_add_u64(rax.wrapping_add(0x8u64), (1) as u64); crate::simd_ops::lk_add_u64(rsi.wrapping_add(0x10u64), (1) as u64); rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x8u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 17; continue; } rax = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; rax = ((rax).wrapping_add(1)) as u64; fl_l = rax; fl_r = (0x8000u64) as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 18; continue; } bb = 17; continue; }
            17 => { rcx = rbp; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 18; continue; }
            18 => { rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x8u64))) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu16(rcx.wrapping_add(rax.wrapping_mul(2)).wrapping_add(0x10u64), ((rbx as u16)) as u16); rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x8u64))) as u64; crate::simd_ops::lk_add_u64(rax.wrapping_add(0x8u64), (1) as u64); crate::simd_ops::lk_add_u64(rbp.wrapping_add(0x10u64), (1) as u64); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_34627<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rdx = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64;
    fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64;
    if !(((((fl_l as u32)) & ((fl_r as u32))) == 0)) { fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0x1; rdx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if !((((fl_l as u32)) == ((fl_r as u32)))) { fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0x1; if !((((fl_l as u32)) == ((fl_r as u32)))) { xmm0 = ((crate::simd_ops::ru32(P1)) as u32) as u128; crate::simd_ops::set_xmm_ret(xmm0); return rax; } xmm0 = ((crate::simd_ops::ru32(P2)) as u32) as u128; crate::simd_ops::set_xmm_ret(xmm0); return rax; } xmm0 = ((crate::simd_ops::ru32(P3)) as u32) as u128; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
    xmm0 = ((crate::simd_ops::ru32(P4)) as u32) as u128;
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_34633<const P0: u64, const P1: u64, const P2: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x5;
    rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64;
    if !((((fl_l as u32)) == ((fl_r as u32)))) { fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x1; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if !((((fl_l as u32)) == ((fl_r as u32)))) { fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x1; if !((((fl_l as u32)) == ((fl_r as u32)))) { xmm0 = 0u128; crate::simd_ops::set_xmm_ret(xmm0); return rax; } xmm0 = ((crate::simd_ops::ru32(P0)) as u32) as u128; crate::simd_ops::set_xmm_ret(xmm0); return rax; } xmm0 = ((crate::simd_ops::ru32(P1)) as u32) as u128; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
    xmm0 = ((crate::simd_ops::ru32(P2)) as u32) as u128;
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_34635<const P0: u64, const P1: u64, const P2: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x38u64)) as u64;
    rcx = 0;
    fl_l = 0; fl_r = 0;
    r9 = P0;
    rax = r8;
    crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rcx);
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rcx);
    rdx = ((rdx).wrapping_add(0x8u64)) as u64;
    r8 = ((rcx.wrapping_add(P1)) as u32) as u32 as u64;
    rcx = rax;
    rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x38u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_34638<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    xmm0 = ((crate::simd_ops::ru64(rdx)) as u64) as u128;
    crate::simd_ops::wu64(r8, ((xmm0 as u64)) as u64);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P0), ((rax as u32)) as u32);
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_34654<const P0: u64, const P1: u64, const P2: u64, const P3: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rcx;
    crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rcx);
    crate::simd_ops::lk_and_u32(rcx, (0xff000010u32) as u32);
    crate::simd_ops::lk_or_u32(rcx, (0x10u32) as u32);
    crate::simd_ops::wu8(rcx.wrapping_add(P0), (0x10u8) as u8);
    crate::simd_ops::wu32(rcx.wrapping_add(P1), (0x0u32) as u32);
    crate::simd_ops::wu8(rcx.wrapping_add(P2), (0x0u8) as u8);
    rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp);
    rax = rbx;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_34667<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut ymm0_hi: u128 = 0;
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let y_addr: u64 = rcx; xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16));
    rax = ((r8 as u32)) as u32 as u64;
    r8 = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64;
    fl_l = ((r8).wrapping_add(r8)) as u64; fl_r = fl_l;
    r8 = ((fl_l as u64)) as u64;
    let y_addr: u64 = rdx.wrapping_add(r8.wrapping_mul(8)); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi);
    xmm1 = (crate::simd_ops::ru128(rcx.wrapping_add(P0))) as u128;
    crate::simd_ops::wu128(rdx.wrapping_add(r8.wrapping_mul(8)).wrapping_add(P0), (xmm1) as u128);
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_34670<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    fl_l = rdx; fl_r = rdx;
    if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { return rax; }
    rax = 0;
    fl_l = 0; fl_r = 0;
    loop { crate::simd_ops::wu64(rcx.wrapping_add(P0), rax); rcx = ((rcx.wrapping_add(P1)) as u64) as u64; fl_l = rdx; fl_r = (0x1u64) as u64; rdx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if !((((fl_l as u64)) != ((fl_r as u64)))) { break; } }
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_34671<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut ymm0_hi: u128 = 0; let mut ymm1_hi: u128 = 0;
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let y_addr: u64 = rcx; xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16));
    rax = ((r8 as u32)) as u32 as u64;
    fl_l = ((rax).wrapping_shl(6u32)) as u64;
    rax = ((fl_l as u64)) as u64;
    let y_addr: u64 = rax.wrapping_add(rdx); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi);
    let y_addr: u64 = rcx.wrapping_add(P0); xmm1 = crate::simd_ops::ru128(y_addr); ymm1_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16));
    let y_addr: u64 = rax.wrapping_add(rdx).wrapping_add(P0); crate::simd_ops::wu128(y_addr, xmm1); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm1_hi);
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_34676<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    fl_l = rdx; fl_r = rdx;
    if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { return rax; }
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rdi = rdx;
    rbx = ((rcx.wrapping_add(P0)) as u64) as u64;
    loop { rcx = rbx; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rbx = ((rbx).wrapping_add(0x50u64)) as u64; fl_l = rdi; fl_r = (0x1u64) as u64; rdi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if !((((fl_l as u64)) != ((fl_r as u64)))) { break; } }
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_34678<const P0: u64, const P1: u64, const P2: u64, const P3: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    crate::simd_ops::forward_incoming_to_outgoing();
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = ((r8 as u32)) as u32 as u64;
    r9 = rcx;
    r8 = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64;
    rax = 0;
    fl_l = 0; fl_r = 0;
    rcx = ((rdx.wrapping_add(r8.wrapping_mul(8))) as u64) as u64;
    rdx = r9;
    crate::simd_ops::wu64(rcx, (0x0u64) as u64);
    crate::simd_ops::wu64(rcx.wrapping_add(P0), rax);
    crate::simd_ops::wu32(rcx.wrapping_add(P1), ((rax as u32)) as u32);
    crate::simd_ops::wu8(rcx.wrapping_add(P2), ((rax as u8)) as u8);
    crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8(P3, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_34679<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    fl_l = rdx; fl_r = rdx;
    if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { return rax; }
    rax = ((rcx.wrapping_add(P0)) as u64) as u64;
    loop { rcx = 0; fl_l = 0; fl_r = 0; let tmp_xchg: u64 = crate::simd_ops::atomic_xchg_u64(rax, rcx); rcx = (tmp_xchg) as u64; rax = ((rax.wrapping_add(P1)) as u64) as u64; fl_l = rdx; fl_r = (0x1u64) as u64; rdx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if !((((fl_l as u64)) != ((fl_r as u64)))) { break; } }
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_34684<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
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
    loop { rcx = rbx; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rbx = ((rbx).wrapping_add(0x10u64)) as u64; fl_l = rdi; fl_r = (0x1u64) as u64; rdi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if !((((fl_l as u64)) != ((fl_r as u64)))) { break; } }
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_34693<const P0: u64, const P1: u64, const P2: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    xmm0 = (crate::simd_ops::ru128(rdx)) as u128;
    xmm1 = ((crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64) as u128;
    crate::simd_ops::wu128(rcx, (xmm0) as u128);
    crate::simd_ops::wu64(rcx.wrapping_add(P0), ((xmm1 as u64)) as u64);
    crate::simd_ops::wu64(rcx.wrapping_add(P1), r8);
    rax = rcx;
    crate::simd_ops::wu64(rcx.wrapping_add(P2), r9);
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_34694<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    xmm0 = (crate::simd_ops::ru128(rcx)) as u128;
    xmm1 = ((crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64) as u128;
    crate::simd_ops::wu128(rdx, (xmm0) as u128);
    crate::simd_ops::wu64(rdx.wrapping_add(P0), ((xmm1 as u64)) as u64);
    rax = rdx;
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_34695<const P0: u64, const P1: u64, const P2: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64;
    rcx = (crate::simd_ops::ru64(rax.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(P2))) as u64;
    rax = rdx;
    crate::simd_ops::wu64(rdx, rcx);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_34696<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    crate::simd_ops::wu64(rcx, rdx);
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    crate::simd_ops::wu64(rcx.wrapping_add(P1), rax);
    rax = rcx;
    crate::simd_ops::wu64(rcx.wrapping_add(P2), (0x0u64) as u64);
    xmm0 = (crate::simd_ops::ru128(rdx.wrapping_add(P3))) as u128;
    crate::simd_ops::wu128(rcx.wrapping_add(P0), (xmm0) as u128);
    xmm1 = ((crate::simd_ops::ru64(rdx.wrapping_add(P4))) as u64) as u128;
    crate::simd_ops::wu64(rcx.wrapping_add(P5), ((xmm1 as u64)) as u64);
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_34697<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    xmm0 = (crate::simd_ops::ru128(rcx.wrapping_add(P1))) as u128;
    xmm1 = ((crate::simd_ops::ru64(rcx.wrapping_add(P2))) as u64) as u128;
    crate::simd_ops::wu64(rdx.wrapping_add(P3), rax);
    rax = rdx;
    crate::simd_ops::wu128(rdx.wrapping_add(P0), (xmm0) as u128);
    crate::simd_ops::wu64(rdx.wrapping_add(P4), ((xmm1 as u64)) as u64);
    crate::simd_ops::wu64(rdx, rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(P5), (0x0u64) as u64);
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_34703<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    xmm0 = (crate::simd_ops::ru128(r8)) as u128;
    xmm1 = ((crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64) as u128;
    rbx = rcx;
    crate::simd_ops::wu64(rcx, rdx);
    crate::simd_ops::wu128(rcx.wrapping_add(P1), (xmm0) as u128);
    crate::simd_ops::wu64(rcx.wrapping_add(P2), ((xmm1 as u64)) as u64);
    rcx = 0;
    fl_l = 0; fl_r = 0;
    r9 = rdx;
    crate::simd_ops::wu64(rbx.wrapping_add(P3), rcx);
    crate::simd_ops::wu64(rbx.wrapping_add(P4), rcx);
    rax = (crate::simd_ops::ru64(rbx.wrapping_add(P0))) as u64;
    fl_l = rax; fl_r = (crate::simd_ops::ru64(rbx.wrapping_add(P2))) as u64;
    if (((fl_l as u64)) == ((fl_r as u64))) { rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P8, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = rbx; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
    rdx = ((crate::simd_ops::ru16(rax)) as u32) as u32 as u64;
    rax = (crate::simd_ops::ru64(r9.wrapping_add(rdx.wrapping_mul(8)).wrapping_add(P5))) as u64;
    crate::simd_ops::wu64(rbx.wrapping_add(P6), rax);
    fl_l = (crate::simd_ops::ru64(r9.wrapping_add(rdx.wrapping_mul(8)).wrapping_add(P5))) as u64; fl_r = rcx;
    if !((((fl_l as u64)) == ((fl_r as u64)))) { rcx = (crate::simd_ops::ru32(r9.wrapping_add(rdx.wrapping_mul(4)).wrapping_add(P7))) as u32 as u64; }
    rax = ((rcx as u32)) as u32 as u64;
    crate::simd_ops::wu64(rbx.wrapping_add(P3), rax);
    rcx = rbx;
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P8, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    rax = rbx;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_34704<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    crate::simd_ops::forward_incoming_to_outgoing();
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rax = ((rax).wrapping_add(0x2u64)) as u64; crate::simd_ops::wu64(rcx.wrapping_add(P0), rax); r9 = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; fl_l = rax; fl_r = r9; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 3; continue; } bb = 1; continue; }
            1 => { rdx = (crate::simd_ops::ru64(rcx.wrapping_add(P2))) as u64; r8 = ((crate::simd_ops::ru16(rax)) as u32) as u32 as u64; fl_l = (crate::simd_ops::ru64(rdx.wrapping_add(r8.wrapping_mul(8)).wrapping_add(P3))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 2; continue; } fl_l = (crate::simd_ops::ru32(rdx.wrapping_add(r8.wrapping_mul(4)).wrapping_add(P4))) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 3; continue; } bb = 2; continue; }
            2 => { rax = ((rax).wrapping_add(0x2u64)) as u64; crate::simd_ops::wu64(rcx.wrapping_add(P0), rax); fl_l = rax; fl_r = r9; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; fl_l = rax; fl_r = r9; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 5; continue; } rdx = ((crate::simd_ops::ru16(rax)) as u32) as u32 as u64; r8 = (crate::simd_ops::ru64(rcx)) as u64; rax = (crate::simd_ops::ru64(r8.wrapping_add(rdx.wrapping_mul(8)).wrapping_add(P3))) as u64; crate::simd_ops::wu64(rcx.wrapping_add(P5), rax); fl_l = (crate::simd_ops::ru64(r8.wrapping_add(rdx.wrapping_mul(8)).wrapping_add(P3))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 4; continue; } rax = (crate::simd_ops::ru32(r8.wrapping_add(rdx.wrapping_mul(4)).wrapping_add(P4))) as u32 as u64; crate::simd_ops::wu64(rcx.wrapping_add(P6), rax); crate::simd_ops::wu64(rcx.wrapping_add(P7), (0x0u64) as u64); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8(P8, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            4 => { rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rcx.wrapping_add(P6), rax); crate::simd_ops::wu64(rcx.wrapping_add(P7), (0x0u64) as u64); bb = 5; continue; }
            5 => { crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8(P8, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_34718<const P0: u64, const P1: u64, const P2: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x48u64)) as u64; rcx = rsp.wrapping_add(0x20); rax = crate::simd_ops::call_stk_caught(P0, rax, rcx, rdx, r8, r9, rsp); rcx = rax; rax = crate::simd_ops::call_stk_caught(P1, rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, P2, rcx, rsp); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_34722(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
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
    loop { rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = rbx; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(rax)) as u64, rax, rcx, rdx, r8, r9, rsp); rbx = ((rbx).wrapping_add(0xb0u64)) as u64; fl_l = rdi; fl_r = (0x1u64) as u64; rdi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if !((((fl_l as u64)) != ((fl_r as u64)))) { break; } }
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_34723<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_34731<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    xmm0 = ((crate::simd_ops::ru32(r8.wrapping_add(P0))) as u32) as u128;
    xmm1 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) + f32::from_bits(crate::simd_ops::ru32(rdx))).to_bits() as u128);
    crate::simd_ops::wu32(rdx, ((xmm1 as u32)) as u32);
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_34732<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 0>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); fl_l = (crate::simd_ops::ru8(rsp.wrapping_add(0x28u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; rbx = rdx; r10 = rcx; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 4; continue; } r11 = (crate::simd_ops::ru64(r9.wrapping_add(P0))) as u64; fl_l = r11; fl_r = (0xffffffffffffffffu64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 4; continue; } rax = (crate::simd_ops::ru64(r8.wrapping_add(P1))) as u64; rdx = (crate::simd_ops::ru32(r8.wrapping_add(P2))) as u32 as u64; xmm0 = 0u128; rcx = ((rax.wrapping_add(rdx.wrapping_mul(8))) as u64) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 3; continue; } r8 = (crate::simd_ops::ru64(r10.wrapping_add(P1))) as u64; bb = 1; continue; }
            1 => { rdx = (crate::simd_ops::ru64(rax)) as u64; rdx = ((rdx).wrapping_shl(7u32)) as u64; fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(r8).wrapping_add(P3))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 2; continue; } xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) + f32::from_bits(crate::simd_ops::ru32(rdx.wrapping_add(r8).wrapping_add(P4)))).to_bits() as u128); bb = 2; continue; }
            2 => { rax = ((rax).wrapping_add(0x8u64)) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(P5))) as u64; rcx = (((r11) as i64).wrapping_mul((0x34u64) as i64) as u64) as u64; xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) + f32::from_bits(crate::simd_ops::ru32(rcx.wrapping_add(rax).wrapping_add(P6)))).to_bits() as u128); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x8u64))) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            4 => { rax = (crate::simd_ops::ru64(r8.wrapping_add(P1))) as u64; rcx = (crate::simd_ops::ru32(r8.wrapping_add(P2))) as u32 as u64; xmm0 = 0u128; rcx = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 7; continue; } r8 = (crate::simd_ops::ru64(r10.wrapping_add(P1))) as u64; bb = 5; continue; }
            5 => { rdx = (crate::simd_ops::ru64(rax)) as u64; rdx = ((rdx).wrapping_shl(7u32)) as u64; fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(r8).wrapping_add(P3))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 6; continue; } xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) + f32::from_bits(crate::simd_ops::ru32(rdx.wrapping_add(r8).wrapping_add(P4)))).to_bits() as u128); bb = 6; continue; }
            6 => { rax = ((rax).wrapping_add(0x8u64)) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 5; continue; } bb = 7; continue; }
            7 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(P1))) as u64; rcx = (((crate::simd_ops::ru64(r9)) as i64).wrapping_mul((0x34u64) as i64) as u64) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x8u64))) as u64; xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) + f32::from_bits(crate::simd_ops::ru32(rcx.wrapping_add(rax).wrapping_add(P6)))).to_bits() as u128); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_34735(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    xmm0 = ((crate::simd_ops::ru32(rdx)) as u32) as u128;
    xmm1 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) + f32::from_bits(crate::simd_ops::ru32(r8))).to_bits() as u128);
    crate::simd_ops::wu32(rdx, ((xmm1 as u32)) as u32);
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_34741<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    xmm0 = ((crate::simd_ops::ru32(r8.wrapping_add(P0))) as u32) as u128;
    xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) - f32::from_bits(crate::simd_ops::ru32(P1))).to_bits() as u128);
    xmm1 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) + f32::from_bits(crate::simd_ops::ru32(rdx))).to_bits() as u128);
    crate::simd_ops::wu32(rdx, ((xmm1 as u32)) as u32);
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_34743<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l) = (crate::simd_ops::get_rax_in(), 0u64);
    r8 = (crate::simd_ops::ru64(rdx)) as u64;
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    fl_l = ((r8).wrapping_shl(6u32)) as u64;
    r8 = ((fl_l as u64)) as u64;
    rax = (crate::simd_ops::ru32(r8.wrapping_add(rax).wrapping_add(P1))) as u32 as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_34744<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l) = (crate::simd_ops::get_rax_in(), 0u64);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    r8 = (crate::simd_ops::ru64(rdx)) as u64;
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    fl_l = ((r8).wrapping_shl(6u32)) as u64;
    r8 = ((fl_l as u64)) as u64;
    xmm0 = ((crate::simd_ops::ru32(r8.wrapping_add(rax).wrapping_add(P1))) as u32) as u128;
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_34746<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut r10, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64; r10 = r9; r8 = (crate::simd_ops::ru32(r8.wrapping_add(P1))) as u32 as u64; xmm0 = 0u128; r8 = ((rax.wrapping_add(r8.wrapping_mul(8))) as u64) as u64; fl_l = rax; fl_r = r8; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 3; continue; } r9 = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; bb = 1; continue; }
            1 => { rcx = (((crate::simd_ops::ru64(rax)) as i64).wrapping_mul((0x8cu64) as i64) as u64) as u64; fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(r9).wrapping_add(P2))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 2; continue; } xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) + f32::from_bits(crate::simd_ops::ru32(rcx.wrapping_add(r9).wrapping_add(P3)))).to_bits() as u128); bb = 2; continue; }
            2 => { rax = ((rax).wrapping_add(0x8u64)) as u64; fl_l = rax; fl_r = r8; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru64(r10)) as u64; rax = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; fl_l = ((rcx).wrapping_shl(6u32)) as u64; rcx = ((fl_l as u64)) as u64; xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) + f32::from_bits(crate::simd_ops::ru32(rcx.wrapping_add(rax).wrapping_add(P4)))).to_bits() as u128); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_34747<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = (crate::simd_ops::ru32(r8.wrapping_add(P0))) as u32 as u64;
    fl_l = (crate::simd_ops::lk_add_u32(rdx, ((rax as u32)) as u32)) as u32 as u64; fl_r = fl_l;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_34748<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64; r10 = 0; fl_l = 0; fl_r = 0; r8 = (crate::simd_ops::ru32(r8.wrapping_add(P1))) as u32 as u64; r11 = r9; r8 = ((rax.wrapping_add(r8.wrapping_mul(8))) as u64) as u64; fl_l = rax; fl_r = r8; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 3; continue; } r9 = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; bb = 1; continue; }
            1 => { rcx = (((crate::simd_ops::ru64(rax)) as i64).wrapping_mul((0x8cu64) as i64) as u64) as u64; fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(r9).wrapping_add(P2))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 2; continue; } fl_l = (((r10 as u32)).wrapping_add(crate::simd_ops::ru32(rcx.wrapping_add(r9).wrapping_add(P3)))) as u32 as u64; fl_r = fl_l; r10 = ((fl_l as u32)) as u32 as u64; bb = 2; continue; }
            2 => { rax = ((rax).wrapping_add(0x8u64)) as u64; fl_l = rax; fl_r = r8; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru64(r11)) as u64; rax = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; rcx = ((rcx).wrapping_shl(6u32)) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(rax).wrapping_add(P4))) as u32 as u64; fl_l = (((rax as u32)).wrapping_add((r10 as u32))) as u32 as u64; fl_r = fl_l; rax = ((fl_l as u32)) as u32 as u64; return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_34750<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = ((crate::simd_ops::ru8(r8.wrapping_add(P0))) as u32) as u32 as u64;
    fl_l = (crate::simd_ops::lk_or_u8(rdx, ((rax as u8)) as u8)) as u8 as u64; fl_r = fl_l;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_34757<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut r10, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64; r10 = r9; r8 = (crate::simd_ops::ru32(r8.wrapping_add(P1))) as u32 as u64; xmm0 = 0u128; r8 = ((rax.wrapping_add(r8.wrapping_mul(8))) as u64) as u64; fl_l = rax; fl_r = r8; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 3; continue; } r9 = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; bb = 1; continue; }
            1 => { rcx = (((crate::simd_ops::ru64(rax)) as i64).wrapping_mul((0x98u64) as i64) as u64) as u64; fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(r9).wrapping_add(P2))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 2; continue; } xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) + f32::from_bits(crate::simd_ops::ru32(rcx.wrapping_add(r9).wrapping_add(P3)))).to_bits() as u128); bb = 2; continue; }
            2 => { rax = ((rax).wrapping_add(0x8u64)) as u64; fl_l = rax; fl_r = r8; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } bb = 3; continue; }
            3 => { rcx = (((crate::simd_ops::ru64(r10)) as i64).wrapping_mul((0x4cu64) as i64) as u64) as u64; rax = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) + f32::from_bits(crate::simd_ops::ru32(rcx.wrapping_add(rax).wrapping_add(P4)))).to_bits() as u128); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_34758<const P0: u64, const P1: u64, const P2: u64, const P3: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut r10, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = (crate::simd_ops::ru64(r8)) as u64; r8 = (crate::simd_ops::ru32(r8.wrapping_add(P0))) as u32 as u64; xmm0 = 0u128; r8 = ((rax.wrapping_add(r8.wrapping_mul(8))) as u64) as u64; fl_l = rax; fl_r = r8; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 3; continue; } r10 = (crate::simd_ops::ru64(rcx)) as u64; bb = 1; continue; }
            1 => { rcx = (((crate::simd_ops::ru64(rax)) as i64).wrapping_mul((0xe8u64) as i64) as u64) as u64; fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(r10).wrapping_add(P1))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 2; continue; } xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) + f32::from_bits(crate::simd_ops::ru32(rcx.wrapping_add(r10).wrapping_add(P2)))).to_bits() as u128); bb = 2; continue; }
            2 => { rax = ((rax).wrapping_add(0x8u64)) as u64; fl_l = rax; fl_r = r8; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } bb = 3; continue; }
            3 => { rcx = (((crate::simd_ops::ru64(r9)) as i64).wrapping_mul((0x9cu64) as i64) as u64) as u64; rax = (crate::simd_ops::ru64(rdx)) as u64; xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) + f32::from_bits(crate::simd_ops::ru32(rcx.wrapping_add(rax).wrapping_add(P3)))).to_bits() as u128); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_34775<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut ymm0_hi: u128 = 0;
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let y_addr: u64 = rcx; xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16));
    rax = ((r8 as u32)) as u32 as u64;
    r8 = (((rax) as i64).wrapping_mul((0x2cu64) as i64) as u64) as u64;
    let y_addr: u64 = r8.wrapping_add(rdx); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi);
    xmm1 = ((crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64) as u128;
    crate::simd_ops::wu64(r8.wrapping_add(rdx).wrapping_add(P0), ((xmm1 as u64)) as u64);
    rax = (crate::simd_ops::ru32(rcx.wrapping_add(P1))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(rdx).wrapping_add(P1), ((rax as u32)) as u32);
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_34795<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64, const P9: u64, const P10: u64, const P11: u64, const P12: u64, const P13: u64, const P14: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 21>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rbp = rsp; rsp = ((rsp).wrapping_sub(0x80u64)) as u64; r12 = rcx; r15 = rcx.wrapping_add(0x8); xmm0 = (crate::simd_ops::ru128(P0)) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rbp.wrapping_sub(0x60u64), (xmm0) as u128); r8 = rbp.wrapping_sub(0x60); rdx = rbp.wrapping_sub(0x40); rcx = r15.wrapping_add(0x18); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rbp.wrapping_sub(0x60u64), r15); rax = (crate::simd_ops::ru64(rbp.wrapping_sub(0x38u64))) as u64; xmm0 = (crate::simd_ops::ru128(rax.wrapping_add(0x10u64))) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rbp.wrapping_sub(0x58u64), (xmm0) as u128); rdx = rbp.wrapping_sub(0x20); rcx = rbp.wrapping_sub(0x60); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rsi = (crate::simd_ops::ru64(rbp.wrapping_sub(0x20u64))) as u64; rcx = rsi.wrapping_add(0x48); xmm0 = (crate::simd_ops::ru128(P3)) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rbp.wrapping_sub(0x40u64), (xmm0) as u128); rdx = rbp.wrapping_sub(0x40); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r14 = rax; rbx = ((crate::simd_ops::ru16(rax)) as u32) as u32 as u64; rdx = (0x2aaaaaaaaaaaaaabu64) as u64; r9 = 0xffff; fl_l = ((rbx as u16)) as u16 as u64; fl_r = ((r9 as u16)) as u16 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { bb = 2; continue; } rdi = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; xmm0 = (crate::simd_ops::ru128(P3)) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rbp.wrapping_sub(0x40u64), (xmm0) as u128); rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x18u64))) as u32 as u64; r8 = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; r8 = ((r8).wrapping_shl(4u32)) as u64; rax = rdx; let prod = (rax as i64 as i128).wrapping_mul((r8) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; rbx = rdx; rbx = ((((rbx) as i64).wrapping_shr(3u32) as u64)) as u64; rax = rbx; rax = ((rax).wrapping_shr(63u32)) as u64; rbx = ((rbx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x1cu64))) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; fl_l = r8; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } rdx = rbx.wrapping_add(0x1); rcx = rdi.wrapping_add(0x10); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x18u64))) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rdi.wrapping_add(0x10u64)))) as u64; xmm0 = 0u128; ymm0_hi = 0; let y_addr: u64 = rcx; crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); crate::simd_ops::wu128(rcx.wrapping_add(0x20u64), (xmm0) as u128); crate::simd_ops::lk_add_u32(rdi.wrapping_add(0x18u64), (1) as u32); rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x18u64))) as u32 as u64; rdx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rdx = ((rdx).wrapping_shl(4u32)) as u64; fl_l = ((rdx).wrapping_add(crate::simd_ops::ru64(rdi.wrapping_add(0x10u64)))) as u64; fl_r = fl_l; rdx = ((fl_l as u64)) as u64; xmm0 = (crate::simd_ops::ru128(rbp.wrapping_sub(0x40u64))) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rdx.wrapping_sub(0x30u64), (xmm0) as u128); crate::simd_ops::wu32(rdx.wrapping_sub(0xeu64), (0xffffffffu32) as u32); rax = (crate::simd_ops::ru64(rbp.wrapping_sub(0x10u64))) as u64; crate::simd_ops::wu64(rdx.wrapping_sub(0x20u64), rax); crate::simd_ops::wu64(rdx.wrapping_sub(0x18u64), rbx); crate::simd_ops::wu16(r14, ((rbx as u16)) as u16); bb = 2; continue; }
            2 => { r9 = 0x2; r8 = (((rbx as u16)) as u32) as u32 as u64; rdx = ((crate::simd_ops::ru16(rbp.wrapping_sub(0x16u64))) as u32) as u32 as u64; rcx = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = rbp.wrapping_sub(0x20); rcx = rbp.wrapping_sub(0x60); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rsi = (crate::simd_ops::ru64(rbp.wrapping_sub(0x20u64))) as u64; rcx = rsi.wrapping_add(0x48); xmm0 = (crate::simd_ops::ru128(P8)) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rbp.wrapping_sub(0x40u64), (xmm0) as u128); rdx = rbp.wrapping_sub(0x40); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r14 = rax; rbx = ((crate::simd_ops::ru16(rax)) as u32) as u32 as u64; rax = 0xffff; fl_l = ((rbx as u16)) as u16 as u64; fl_r = ((rax as u16)) as u16 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { bb = 4; continue; } rdi = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; xmm0 = (crate::simd_ops::ru128(P8)) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rbp.wrapping_sub(0x40u64), (xmm0) as u128); rcx = (crate::simd_ops::ru32(rdi.wrapping_add(0x18u64))) as u32 as u64; r8 = ((rcx.wrapping_add(rcx.wrapping_mul(2))) as u64) as u64; r8 = ((r8).wrapping_shl(4u32)) as u64; rax = (0x2aaaaaaaaaaaaaabu64) as u64; let prod = (rax as i64 as i128).wrapping_mul((r8) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; rbx = rdx; rbx = ((((rbx) as i64).wrapping_shr(3u32) as u64)) as u64; rax = rbx; rax = ((rax).wrapping_shr(63u32)) as u64; rbx = ((rbx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x1cu64))) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; fl_l = r8; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } rdx = rbx.wrapping_add(0x1); rcx = rdi.wrapping_add(0x10); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x18u64))) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rdi.wrapping_add(0x10u64)))) as u64; xmm0 = 0u128; ymm0_hi = 0; let y_addr: u64 = rcx; crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); crate::simd_ops::wu128(rcx.wrapping_add(0x20u64), (xmm0) as u128); crate::simd_ops::lk_add_u32(rdi.wrapping_add(0x18u64), (1) as u32); rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x18u64))) as u32 as u64; rdx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rdx = ((rdx).wrapping_shl(4u32)) as u64; fl_l = ((rdx).wrapping_add(crate::simd_ops::ru64(rdi.wrapping_add(0x10u64)))) as u64; fl_r = fl_l; rdx = ((fl_l as u64)) as u64; xmm0 = (crate::simd_ops::ru128(rbp.wrapping_sub(0x40u64))) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rdx.wrapping_sub(0x30u64), (xmm0) as u128); crate::simd_ops::wu32(rdx.wrapping_sub(0xeu64), (0xffffffffu32) as u32); rax = (crate::simd_ops::ru64(rbp.wrapping_sub(0x10u64))) as u64; crate::simd_ops::wu64(rdx.wrapping_sub(0x20u64), rax); crate::simd_ops::wu64(rdx.wrapping_sub(0x18u64), rbx); crate::simd_ops::wu16(r14, ((rbx as u16)) as u16); bb = 4; continue; }
            4 => { r9 = 0x2; r8 = (((rbx as u16)) as u32) as u32 as u64; rdx = ((crate::simd_ops::ru16(rbp.wrapping_sub(0x16u64))) as u32) as u32 as u64; rcx = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm0 = (crate::simd_ops::ru128(P9)) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rbp.wrapping_sub(0x40u64), (xmm0) as u128); r8 = rbp.wrapping_sub(0x40); rdx = rbp.wrapping_sub(0x60); rcx = r15.wrapping_add(0x18); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rbp.wrapping_sub(0x40u64), r15); rax = (crate::simd_ops::ru64(rbp.wrapping_sub(0x58u64))) as u64; xmm0 = (crate::simd_ops::ru128(rax.wrapping_add(0x10u64))) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rbp.wrapping_sub(0x38u64), (xmm0) as u128); rdx = rbp.wrapping_sub(0x20); rcx = rbp.wrapping_sub(0x40); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P10, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = (crate::simd_ops::ru64(rax)) as u64; r14 = ((crate::simd_ops::ru16(rax.wrapping_add(0xau64))) as u32) as u32 as u64; xmm0 = (crate::simd_ops::ru128(rax.wrapping_add(0x10u64))) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rbp.wrapping_sub(0x60u64), (xmm0) as u128); rcx = rbx.wrapping_add(0x48); xmm1 = (crate::simd_ops::ru128(P11)) as u128; crate::simd_ops::wu128(rbp.wrapping_sub(0x40u64), (xmm1) as u128); rdx = rbp.wrapping_sub(0x40); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r15 = rax; rdi = ((crate::simd_ops::ru16(rax)) as u32) as u32 as u64; rsi = rbx.wrapping_add(0x8); r13 = 0xffff; fl_l = ((rdi as u16)) as u16 as u64; fl_r = ((r13 as u16)) as u16 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { bb = 6; continue; } rbx = (crate::simd_ops::ru64(rsi)) as u64; xmm0 = (crate::simd_ops::ru128(P11)) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rbp.wrapping_sub(0x40u64), (xmm0) as u128); rcx = (crate::simd_ops::ru32(rbx.wrapping_add(0x18u64))) as u32 as u64; r8 = ((rcx.wrapping_add(rcx.wrapping_mul(2))) as u64) as u64; r8 = ((r8).wrapping_shl(4u32)) as u64; rax = (0x2aaaaaaaaaaaaaabu64) as u64; let prod = (rax as i64 as i128).wrapping_mul((r8) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; rdi = rdx; rdi = ((((rdi) as i64).wrapping_shr(3u32) as u64)) as u64; rax = rdi; rax = ((rax).wrapping_shr(63u32)) as u64; rdi = ((rdi).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x1cu64))) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; fl_l = r8; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 5; continue; } rdx = rdi.wrapping_add(0x1); rcx = rbx.wrapping_add(0x10); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x18u64))) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rbx.wrapping_add(0x10u64)))) as u64; xmm0 = 0u128; ymm0_hi = 0; let y_addr: u64 = rcx; crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); crate::simd_ops::wu128(rcx.wrapping_add(0x20u64), (xmm0) as u128); crate::simd_ops::lk_add_u32(rbx.wrapping_add(0x18u64), (1) as u32); rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x18u64))) as u32 as u64; rdx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rdx = ((rdx).wrapping_shl(4u32)) as u64; fl_l = ((rdx).wrapping_add(crate::simd_ops::ru64(rbx.wrapping_add(0x10u64)))) as u64; fl_r = fl_l; rdx = ((fl_l as u64)) as u64; xmm0 = (crate::simd_ops::ru128(rbp.wrapping_sub(0x40u64))) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rdx.wrapping_sub(0x30u64), (xmm0) as u128); crate::simd_ops::wu32(rdx.wrapping_sub(0xeu64), (0xffffffffu32) as u32); rax = (crate::simd_ops::ru64(rbp.wrapping_sub(0x60u64))) as u64; crate::simd_ops::wu64(rdx.wrapping_sub(0x20u64), rax); crate::simd_ops::wu64(rdx.wrapping_sub(0x18u64), rdi); crate::simd_ops::wu16(r15, ((rdi as u16)) as u16); bb = 6; continue; }
            6 => { r9 = 0x2; r8 = (((rdi as u16)) as u32) as u32 as u64; rdx = (((r14 as u16)) as u32) as u32 as u64; rcx = (crate::simd_ops::ru64(rsi)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru32(P12)) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_sub(0x60u64), ((rax as u32)) as u32); rax = P13; crate::simd_ops::wu64(rbp.wrapping_sub(0x58u64), rax); rcx = (crate::simd_ops::ru64(r12)) as u64; rcx = ((rcx).wrapping_add(0xc0u64)) as u64; r8 = rbp.wrapping_sub(0x60); rdx = rbp.wrapping_add(0x30); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P14, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r11 = rsp.wrapping_add(0x80); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x40u64))) as u64; rdi = (crate::simd_ops::ru64(r11.wrapping_add(0x48u64))) as u64; rsp = r11; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_34799<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    fl_l = rdx; fl_r = rdx;
    if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { return rax; }
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rdi = rdx;
    rbx = ((rcx.wrapping_add(P0)) as u64) as u64;
    loop { rcx = rbx; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rbx = ((rbx).wrapping_add(0x30u64)) as u64; fl_l = rdi; fl_r = (0x1u64) as u64; rdi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if !((((fl_l as u64)) != ((fl_r as u64)))) { break; } }
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_34809(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
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
    loop { rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = rbx; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(rax)) as u64, rax, rcx, rdx, r8, r9, rsp); rbx = ((rbx).wrapping_add(0x120u64)) as u64; fl_l = rdi; fl_r = (0x1u64) as u64; rdi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if !((((fl_l as u64)) != ((fl_r as u64)))) { break; } }
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_34824<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
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
    loop { rcx = ((rbx.wrapping_add(P0)) as u64) as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rcx = rbx; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rbx = ((rbx).wrapping_add(0x68u64)) as u64; fl_l = rdi; fl_r = (0x1u64) as u64; rdi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if !((((fl_l as u64)) != ((fl_r as u64)))) { break; } }
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_34833<const P0: u64, const P1: u64, const P2: u64, const P3: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = (crate::simd_ops::ru64(P0)) as u64; rax = rcx; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } rdx = ((crate::simd_ops::ru8(rax.wrapping_add(P1))) as u32) as u32 as u64; rcx = ((rcx).wrapping_add(0x8u64)) as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0x80; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 1; continue; } rdx = ((rdx.wrapping_mul(8)) as u32) as u32 as u64; bb = 2; continue; }
            1 => { rdx = (((rdx as u32)).wrapping_shl(8u32)) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0x7c00; rdx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru32(rax)) as u32 as u64; rax = ((((rax as u32)) & (0xffffffu32))) as u32 as u64; rax = (((rax as u32)).wrapping_shl(3u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 3; continue; } rcx = (crate::simd_ops::ru64(rcx)) as u64; bb = 3; continue; }
            3 => { fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 4; continue; } rax = 0; fl_l = 0; fl_r = 0; bb = 5; continue; }
            4 => { rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 5; continue; }
            5 => { r9 = 0; fl_l = 0; fl_r = 0; r8 = (0x100000001u64) as u64; rdx = ((rax as u32)) as u32 as u64; rcx = rbx; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(P3, rax, rcx, rdx, r8, r9, rsp); }
            6 => { rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_34835<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbx);
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rdx;
    rdi = rcx;
    xmm0 = (crate::simd_ops::ru128(rdx)) as u128;
    crate::simd_ops::wu128(rcx, (xmm0) as u128);
    rcx = ((rcx).wrapping_add(0x10u64)) as u64;
    rsi = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu64(rcx, rsi);
    crate::simd_ops::wu64(rcx.wrapping_add(P0), rsi);
    rdx = ((rdx).wrapping_add(0x10u64)) as u64;
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    rcx = ((rdi.wrapping_add(P2)) as u64) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rcx);
    crate::simd_ops::lk_and_u32(rcx, (0xff000004u32) as u32);
    crate::simd_ops::lk_or_u32(rcx, (0x4u32) as u32);
    crate::simd_ops::wu8(rcx.wrapping_add(P3), (0x4u8) as u8);
    crate::simd_ops::wu32(rcx.wrapping_add(P4), ((rsi as u32)) as u32);
    crate::simd_ops::wu8(rcx.wrapping_add(P0), ((rsi as u8)) as u8);
    rdx = ((rbx.wrapping_add(P2)) as u64) as u64;
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    rax = rdi;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64;
    rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_34837<const P0: u64, const P1: u64, const P2: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    xmm0 = (crate::simd_ops::ru128(rdx)) as u128;
    crate::simd_ops::wu128(rcx, (xmm0) as u128);
    rbx = rdx;
    rdi = rcx;
    rcx = ((rcx).wrapping_add(0x10u64)) as u64;
    rdx = ((rdx).wrapping_add(0x10u64)) as u64;
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    rdx = ((rbx.wrapping_add(P1)) as u64) as u64;
    rcx = ((rdi.wrapping_add(P1)) as u64) as u64;
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rax = rdi;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_34840<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbx);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rdx;
    rdi = rcx;
    xmm0 = (crate::simd_ops::ru128(rdx)) as u128;
    crate::simd_ops::wu128(rcx, (xmm0) as u128);
    rcx = ((rcx).wrapping_add(0x10u64)) as u64;
    rdx = ((rdx).wrapping_add(0x10u64)) as u64;
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    rcx = ((rdi.wrapping_add(P1)) as u64) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rcx);
    crate::simd_ops::lk_and_u32(rcx, (0xff000004u32) as u32);
    crate::simd_ops::lk_or_u32(rcx, (0x4u32) as u32);
    crate::simd_ops::wu8(rcx.wrapping_add(P2), (0x4u8) as u8);
    crate::simd_ops::wu32(rcx.wrapping_add(P3), (0x0u32) as u32);
    crate::simd_ops::wu8(rcx.wrapping_add(P4), (0x0u8) as u8);
    rdx = ((rbx.wrapping_add(P1)) as u64) as u64;
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    rax = rdi;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_34842<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = (crate::simd_ops::ru64(rdx)) as u64;
    r8 = ((crate::simd_ops::ru16(P0)) as u32) as u32 as u64;
    rdx = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64;
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(r8.wrapping_mul(8)))) as u64;
    crate::simd_ops::wu64(rcx, rax);
    rax = rcx;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_34843<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rsi = rcx;
    rbx = ((crate::simd_ops::ru16(P0)) as u32) as u32 as u64;
    rdi = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64;
    r8 = (crate::simd_ops::ru32(rdi.wrapping_add(P2))) as u32 as u64;
    rdx = (crate::simd_ops::ru32(rdi.wrapping_add(P3))) as u32 as u64;
    rdx = ((rdx).wrapping_add(rdx)) as u64;
    rax = ((r8.wrapping_add(r8)) as u64) as u64;
    fl_l = rax; fl_r = rdx;
    if !((((fl_l as u64)) != ((fl_r as u64)))) { rdx = ((r8.wrapping_add(P4)) as u64) as u64; rcx = ((rdi.wrapping_add(P5)) as u64) as u64; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); }
    rcx = (crate::simd_ops::ru32(rdi.wrapping_add(P2))) as u32 as u64;
    rax = (crate::simd_ops::ru64(rdi.wrapping_add(P5))) as u64;
    crate::simd_ops::wu16(rax.wrapping_add(rcx.wrapping_mul(2)), ((rbx as u16)) as u16);
    crate::simd_ops::lk_add_u32(rdi.wrapping_add(P2), (1) as u32);
    rax = (crate::simd_ops::ru64(rsi.wrapping_add(P1))) as u64;
    crate::simd_ops::wu8(rax.wrapping_add(P7), (0x1u8) as u8);
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_34844<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 55>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rbp = rsp.wrapping_sub(0x90); rsp = ((rsp).wrapping_sub(0x190u64)) as u64; rsi = rcx; rax = P0; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (0x4u64) as u64); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdi = rax; r15 = P2; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), r15); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x30u64))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm0) as u128); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (0xcu64) as u64); rax = rsp.wrapping_add(0x40); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rax); rbx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x78u64), (0x7ffu64) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rdi); crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), (0x7ffu64) as u64); r14 = P3; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), r14); crate::simd_ops::wu64(rbp.wrapping_sub(0x80u64), rdi); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x30u64))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm0) as u128); xmm1 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm1) as u128); r9 = ((rbx as u32)) as u32 as u64; r8 = rsp.wrapping_add(0x30); rdx = rsp.wrapping_add(0x20); rcx = rsp.wrapping_add(0x50); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; r11 = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; rax = r11; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rax = ((rax).wrapping_sub(rcx)) as u64; r10 = ((rbx as u32)) as u32 as u64; fl_l = r11; fl_r = rcx; r10 = (if (((fl_l as u64)) > ((fl_r as u64))) { rax } else { r10 }) as u64; r9 = ((rdx.wrapping_add(rcx)) as u64) as u64; fl_l = rdx; fl_r = r10; r10 = (if (((fl_l as u64)) < ((fl_r as u64))) { rdx } else { r10 }) as u64; r8 = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rcx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x80u64))) as u64; fl_l = r8; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } fl_l = ((rcx).wrapping_add(r10)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x80u64), rcx); r8 = rbp.wrapping_sub(0x78); crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), r8); crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), (0x100u64) as u64); bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), r14); rax = r11; rax = ((rax).wrapping_sub(r9)) as u64; rdx = rbx; fl_l = r11; fl_r = r9; rdx = (if (((fl_l as u64)) > ((fl_r as u64))) { rax } else { rdx }) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), r9); fl_l = rdx; fl_r = rdx; rdx = (if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { rbx } else { rdx }) as u64; fl_l = r8; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } rax = ((rcx.wrapping_add(rdx)) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x80u64), rax); rax = rbp.wrapping_sub(0x78); crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), (0x100u64) as u64); bb = 2; continue; }
            2 => { crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), rbx); crate::simd_ops::wu8(rcx, (0x0u8) as u8); rdx = (0xffffffffffffffffu64) as u64; bb = 3; continue; }
            3 => { rdx = ((rdx).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru8(rdi.wrapping_add(rdx))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 3; continue; } rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rsi, rbx); crate::simd_ops::wu64(rsi.wrapping_add(0x8u64), rax); crate::simd_ops::wu64(rsi.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsi.wrapping_add(0x18u64), rbx); rcx = rsi.wrapping_add(0x20); crate::simd_ops::wu64(rbp.wrapping_add(0xc8u64), rcx); crate::simd_ops::lk_and_u32(rcx, (0xff000004u32) as u32); crate::simd_ops::lk_or_u32(rcx, (0x4u32) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x3u64), (0x4u8) as u8); crate::simd_ops::wu32(rcx.wrapping_add(0x4u64), ((rbx as u32)) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x8u64), (0x0u8) as u8); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r15); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (0x1du64) as u64); r9 = 0x1d; r8 = 0; fl_l = 0; fl_r = 0; rdx = rsp.wrapping_add(0x20); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = rsi; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x1d0u64))) as u64; rsp = ((rsp).wrapping_add(0x190u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_34845<const P0: u64, const P1: u64, const P2: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rax = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64;
    rbx = rdx;
    rdi = rcx;
    fl_l = rdx; fl_r = rax;
    if !((((fl_l as u64)) < ((fl_r as u64)))) { rdx = ((rdx).wrapping_add(1)) as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); }
    rax = ((rbx.wrapping_add(rbx.wrapping_mul(4))) as u64) as u64;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rax = ((rax).wrapping_shl(5u32)) as u64;
    rax = ((rax).wrapping_add(crate::simd_ops::ru64(rdi.wrapping_add(P2)))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_34846<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x30u64)) as u64;
    rbx = r8;
    rdi = rcx;
    r8 = rdx;
    xmm0 = 0u128;
    fl_l = rbx; fl_r = rbx;
    if !((((((fl_l as u64)) & ((fl_r as u64))) as i64) < 0)) { xmm0 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | ((((rbx) as i64) as f64).to_bits() as u128); } else { rax = rbx; rcx = rbx; rcx = ((rcx).wrapping_shr(1u32)) as u64; rax = ((((rax as u32)) & (0x1u32))) as u32 as u64; fl_l = (((rcx) | (rax))) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; xmm0 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | ((((rcx) as i64) as f64).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm0 as u64) + f64::from_bits(xmm0 as u64)).to_bits() as u128); }
    r9 = rsp.wrapping_add(0x40);
    crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), (0x0u32) as u32);
    rdx = rdi;
    rcx = r8;
    crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), ((xmm0 as u64)) as u64);
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    rdx = rbx;
    rcx = rdi;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64;
    rsp = ((rsp).wrapping_add(0x30u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return crate::simd_ops::tail_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_34847<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 55>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rbp = rsp.wrapping_sub(0x90); rsp = ((rsp).wrapping_sub(0x190u64)) as u64; rsi = rcx; rax = P0; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (0x4u64) as u64); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdi = rax; r15 = P2; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), r15); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x30u64))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm0) as u128); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (0xcu64) as u64); rax = rsp.wrapping_add(0x40); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rax); rbx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x78u64), (0x7ffu64) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rdi); crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), (0x7ffu64) as u64); r14 = P3; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), r14); crate::simd_ops::wu64(rbp.wrapping_sub(0x80u64), rdi); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x30u64))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm0) as u128); xmm1 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm1) as u128); r9 = ((rbx as u32)) as u32 as u64; r8 = rsp.wrapping_add(0x30); rdx = rsp.wrapping_add(0x20); rcx = rsp.wrapping_add(0x50); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; r11 = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; rax = r11; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rax = ((rax).wrapping_sub(rcx)) as u64; r10 = ((rbx as u32)) as u32 as u64; fl_l = r11; fl_r = rcx; r10 = (if (((fl_l as u64)) > ((fl_r as u64))) { rax } else { r10 }) as u64; r9 = ((rdx.wrapping_add(rcx)) as u64) as u64; fl_l = rdx; fl_r = r10; r10 = (if (((fl_l as u64)) < ((fl_r as u64))) { rdx } else { r10 }) as u64; r8 = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rcx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x80u64))) as u64; fl_l = r8; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } fl_l = ((rcx).wrapping_add(r10)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x80u64), rcx); r8 = rbp.wrapping_sub(0x78); crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), r8); crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), (0x100u64) as u64); bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), r14); rax = r11; rax = ((rax).wrapping_sub(r9)) as u64; rdx = rbx; fl_l = r11; fl_r = r9; rdx = (if (((fl_l as u64)) > ((fl_r as u64))) { rax } else { rdx }) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), r9); fl_l = rdx; fl_r = rdx; rdx = (if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { rbx } else { rdx }) as u64; fl_l = r8; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } rax = ((rcx.wrapping_add(rdx)) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x80u64), rax); rax = rbp.wrapping_sub(0x78); crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), (0x100u64) as u64); bb = 2; continue; }
            2 => { crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), rbx); crate::simd_ops::wu8(rcx, (0x0u8) as u8); rdx = (0xffffffffffffffffu64) as u64; bb = 3; continue; }
            3 => { rdx = ((rdx).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru8(rdi.wrapping_add(rdx))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 3; continue; } rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rsi, rbx); crate::simd_ops::wu64(rsi.wrapping_add(0x8u64), rax); crate::simd_ops::wu64(rsi.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsi.wrapping_add(0x18u64), rbx); rcx = rsi.wrapping_add(0x20); crate::simd_ops::wu64(rbp.wrapping_add(0xc8u64), rcx); crate::simd_ops::lk_and_u32(rcx, (0xff000004u32) as u32); crate::simd_ops::lk_or_u32(rcx, (0x4u32) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x3u64), (0x4u8) as u8); crate::simd_ops::wu32(rcx.wrapping_add(0x4u64), ((rbx as u32)) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x8u64), (0x0u8) as u8); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r15); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (0x1eu64) as u64); r9 = 0x1e; r8 = 0; fl_l = 0; fl_r = 0; rdx = rsp.wrapping_add(0x20); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = rsi; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x1d0u64))) as u64; rsp = ((rsp).wrapping_add(0x190u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_34869<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64, const P9: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x60u64)) as u64; r8 = ((crate::simd_ops::ru8(P0)) as u32) as u32 as u64; rbx = rdx; rdi = rcx; fl_l = ((r8 as u32)) as u32 as u64; fl_r = 0x80; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 1; continue; } r8 = ((r8.wrapping_mul(8)) as u32) as u32 as u64; bb = 2; continue; }
            1 => { r8 = (((r8 as u32)).wrapping_shl(8u32)) as u32 as u64; fl_l = ((r8 as u32)) as u32 as u64; fl_r = 0x7c00; r8 = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 2; continue; }
            2 => { xmm0 = (crate::simd_ops::ru128(rdx)) as u128; rax = (crate::simd_ops::ru32(P1)) as u32 as u64; rsi = P2; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(0x10u64))) as u64; rax = ((((rax as u32)) & (0xffffffu32))) as u32 as u64; rbp = (crate::simd_ops::ru32(P3)) as u32 as u64; rax = (((rax as u32)).wrapping_shl(3u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), (0xffffffffffffffffu64) as u64); crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm0) as u128); rsi = (if (((fl_l as u32)) > ((fl_r as u32))) { crate::simd_ops::ru64(P2) } else { rsi }) as u64; xmm0 = ((crate::simd_ops::ru64(rdx.wrapping_add(0x10u64))) as u64) as u128; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), ((xmm0 as u64)) as u64); fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0xffffffff; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 3; continue; } rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rax = ((rax).wrapping_shr(32u32)) as u64; rcx = ((rcx).wrapping_shr(32u32)) as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 7; continue; } bb = 3; continue; }
            3 => { xmm0 = (crate::simd_ops::ru128(rdx)) as u128; xmm1 = ((crate::simd_ops::ru64(rdx.wrapping_add(0x10u64))) as u64) as u128; rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0x10u64))) as u64; r8 = rsp.wrapping_add(0x20); rcx = rsp.wrapping_add(0x70); crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), ((xmm1 as u64)) as u64); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0xffffffff; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 7; continue; } rdi = (crate::simd_ops::ru64(rbx)) as u64; rcx = ((rax as u32)) as u32 as u64; rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x30u64))) as u64; r11 = (crate::simd_ops::ru64(rax.wrapping_add(rcx.wrapping_mul(8)))) as u64; bb = 4; continue; }
            4 => { rax = 0xffff; fl_l = ((r11 as u16)) as u16 as u64; fl_r = ((rax as u16)) as u16 as u64; if (((fl_l as u16)) == ((fl_r as u16))) { bb = 8; continue; } r8 = ((crate::simd_ops::ru16(P5)) as u32) as u32 as u64; rbx = (((r11 as u16)) as u32) as u32 as u64; rax = ((rbx as u32)) as u32 as u64; rax = ((rax).wrapping_shl(5u32)) as u64; rsi = (crate::simd_ops::ru64(rdi.wrapping_add(rbx.wrapping_mul(8)).wrapping_add(0x88u64))) as u64; rdx = (crate::simd_ops::ru32(rax.wrapping_add(rdi).wrapping_add(0x180acu64))) as u32 as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(rdi).wrapping_add(0x18098u64))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = ((((rax as u32)) as i32 as i64) as u64) as u64; r11 = ((r11).wrapping_shr(32u32)) as u64; fl_l = rcx; fl_r = (0xffffffffffffffffu64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 5; continue; } rax = rbx.wrapping_add(0xc03); fl_l = ((rax).wrapping_shl(5u32)) as u64; rax = ((fl_l as u64)) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(rdi).wrapping_add(0x40u64))) as u64; rcx = (crate::simd_ops::ru32(rax.wrapping_add(rcx.wrapping_mul(4)))) as u32 as u64; bb = 6; continue; }
            5 => { rcx = 0; fl_l = 0; fl_r = 0; bb = 6; continue; }
            6 => { rax = ((rsi.wrapping_add(rcx)) as u64) as u64; fl_l = rcx; fl_r = rcx; rcx = (if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { rax } else { rcx }) as u64; rax = (((r11) as i64).wrapping_mul((0x6cu64) as i64) as u64) as u64; fl_l = ((rcx).wrapping_add(rax)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; bb = 10; continue; }
            7 => { r8 = rsp.wrapping_add(0x40); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); rdx = rsp.wrapping_add(0x20); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rbp); rcx = rsp.wrapping_add(0x70); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0xffffffff; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 9; continue; } crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), (0xffffffffffffffffu64) as u64); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rax = (if (((fl_l as u32)) != ((fl_r as u32))) { rcx } else { rax }) as u64; rdx = ((rcx as u32)) as u32 as u64; crate::simd_ops::wu64(rdi.wrapping_add(0x10u64), rax); rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x30u64))) as u64; r11 = (crate::simd_ops::ru64(rax.wrapping_add(rdx.wrapping_mul(8)))) as u64; bb = 4; continue; }
            8 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm((crate::simd_ops::ru64(P8)) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::lift_ops::trap(0x80000003, P9, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            9 => { rcx = 0; fl_l = 0; fl_r = 0; bb = 10; continue; }
            10 => { r11 = rsp.wrapping_add(0x60); rax = rcx; rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x18u64))) as u64; rbp = (crate::simd_ops::ru64(r11.wrapping_add(0x20u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x28u64))) as u64; rsp = r11; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_34871<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; r8 = (crate::simd_ops::ru64(rcx)) as u64; rbx = rdx; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), (0xffffffffffffffffu64) as u64); rdi = rcx; fl_l = ((r8 as u32)) as u32 as u64; fl_r = 0xffffffff; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 1; continue; } rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rax = ((rax).wrapping_shr(32u32)) as u64; r8 = ((r8).wrapping_shr(32u32)) as u64; fl_l = ((r8 as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 2; continue; } bb = 1; continue; }
            1 => { xmm0 = (crate::simd_ops::ru128(rdx)) as u128; xmm1 = ((crate::simd_ops::ru64(rdx.wrapping_add(0x10u64))) as u64) as u128; rdx = (crate::simd_ops::ru64(rcx)) as u64; r8 = rsp.wrapping_add(0x20); rcx = rsp.wrapping_add(0x50); crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), ((xmm1 as u64)) as u64); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r8 = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; fl_l = ((r8 as u32)) as u32 as u64; fl_r = 0xffffffff; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 5; continue; } bb = 2; continue; }
            2 => { rdx = ((crate::simd_ops::ru8(P1)) as u32) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0x80; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 3; continue; } rdx = ((rdx.wrapping_mul(8)) as u32) as u32 as u64; bb = 4; continue; }
            3 => { rdx = (((rdx as u32)).wrapping_shl(8u32)) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0x7c00; rdx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 4; continue; }
            4 => { rcx = (crate::simd_ops::ru32(P2)) as u32 as u64; rax = P3; rcx = ((((rcx as u32)) & (0xffffffu32))) as u32 as u64; r8 = rbx; rcx = (((rcx as u32)).wrapping_shl(3u32)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; rdx = rsp.wrapping_add(0x20); rcx = rsp.wrapping_add(0x50); rax = (if (((fl_l as u32)) > ((fl_r as u32))) { crate::simd_ops::ru64(P3) } else { rax }) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); rax = (crate::simd_ops::ru32(P4)) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r8 = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; fl_l = ((r8 as u32)) as u32 as u64; fl_r = 0xffffffff; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 6; continue; } crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), (0xffffffffffffffffu64) as u64); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rax = (if (((fl_l as u32)) != ((fl_r as u32))) { r8 } else { rax }) as u64; crate::simd_ops::wu64(rdi, rax); bb = 5; continue; }
            5 => { rdx = rsp.wrapping_add(0x20); rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = ((rax).wrapping_shl(6u32)) as u64; rax = ((rax).wrapping_add(crate::simd_ops::ru64(rsp.wrapping_add(0x20u64)))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            6 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rax = 0; fl_l = 0; fl_r = 0; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_34885<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    crate::simd_ops::forward_incoming_to_outgoing();
    let (mut rax, mut r10) = (crate::simd_ops::get_rax_in(), 0u64);
    r10 = (crate::simd_ops::ru64(rcx)) as u64;
    crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((crate::simd_ops::ru64(r10.wrapping_add(P0))) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_34889<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 55>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rbp = rsp.wrapping_sub(0x90); rsp = ((rsp).wrapping_sub(0x190u64)) as u64; rsi = rcx; rax = P0; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (0x4u64) as u64); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdi = rax; r15 = P2; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), r15); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x30u64))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm0) as u128); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (0xcu64) as u64); rax = rsp.wrapping_add(0x40); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rax); rbx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x78u64), (0x7ffu64) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rdi); crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), (0x7ffu64) as u64); r14 = P3; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), r14); crate::simd_ops::wu64(rbp.wrapping_sub(0x80u64), rdi); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x30u64))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm0) as u128); xmm1 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm1) as u128); r9 = ((rbx as u32)) as u32 as u64; r8 = rsp.wrapping_add(0x30); rdx = rsp.wrapping_add(0x20); rcx = rsp.wrapping_add(0x50); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; r11 = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; rax = r11; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rax = ((rax).wrapping_sub(rcx)) as u64; r10 = ((rbx as u32)) as u32 as u64; fl_l = r11; fl_r = rcx; r10 = (if (((fl_l as u64)) > ((fl_r as u64))) { rax } else { r10 }) as u64; r9 = ((rdx.wrapping_add(rcx)) as u64) as u64; fl_l = rdx; fl_r = r10; r10 = (if (((fl_l as u64)) < ((fl_r as u64))) { rdx } else { r10 }) as u64; r8 = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rcx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x80u64))) as u64; fl_l = r8; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } fl_l = ((rcx).wrapping_add(r10)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x80u64), rcx); r8 = rbp.wrapping_sub(0x78); crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), r8); crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), (0x100u64) as u64); bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), r14); rax = r11; rax = ((rax).wrapping_sub(r9)) as u64; rdx = rbx; fl_l = r11; fl_r = r9; rdx = (if (((fl_l as u64)) > ((fl_r as u64))) { rax } else { rdx }) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), r9); fl_l = rdx; fl_r = rdx; rdx = (if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { rbx } else { rdx }) as u64; fl_l = r8; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } rax = ((rcx.wrapping_add(rdx)) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x80u64), rax); rax = rbp.wrapping_sub(0x78); crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), (0x100u64) as u64); bb = 2; continue; }
            2 => { crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), rbx); crate::simd_ops::wu8(rcx, (0x0u8) as u8); rdx = (0xffffffffffffffffu64) as u64; bb = 3; continue; }
            3 => { rdx = ((rdx).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru8(rdi.wrapping_add(rdx))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 3; continue; } rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rsi, rbx); crate::simd_ops::wu64(rsi.wrapping_add(0x8u64), rax); crate::simd_ops::wu64(rsi.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsi.wrapping_add(0x18u64), rbx); rcx = rsi.wrapping_add(0x20); crate::simd_ops::wu64(rbp.wrapping_add(0xc8u64), rcx); crate::simd_ops::lk_and_u32(rcx, (0xff000004u32) as u32); crate::simd_ops::lk_or_u32(rcx, (0x4u32) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x3u64), (0x4u8) as u8); crate::simd_ops::wu32(rcx.wrapping_add(0x4u64), ((rbx as u32)) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x8u64), (0x0u8) as u8); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r15); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (0x15u64) as u64); r9 = 0x15; r8 = 0; fl_l = 0; fl_r = 0; rdx = rsp.wrapping_add(0x20); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = rsi; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x1d0u64))) as u64; rsp = ((rsp).wrapping_add(0x190u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_34890<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 55>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rbp = rsp.wrapping_sub(0x90); rsp = ((rsp).wrapping_sub(0x190u64)) as u64; rsi = rcx; rax = P0; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (0x4u64) as u64); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdi = rax; r15 = P2; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), r15); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x30u64))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm0) as u128); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (0xcu64) as u64); rax = rsp.wrapping_add(0x40); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rax); rbx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x78u64), (0x7ffu64) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rdi); crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), (0x7ffu64) as u64); r14 = P3; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), r14); crate::simd_ops::wu64(rbp.wrapping_sub(0x80u64), rdi); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x30u64))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm0) as u128); xmm1 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm1) as u128); r9 = ((rbx as u32)) as u32 as u64; r8 = rsp.wrapping_add(0x30); rdx = rsp.wrapping_add(0x20); rcx = rsp.wrapping_add(0x50); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; r11 = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; rax = r11; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rax = ((rax).wrapping_sub(rcx)) as u64; r10 = ((rbx as u32)) as u32 as u64; fl_l = r11; fl_r = rcx; r10 = (if (((fl_l as u64)) > ((fl_r as u64))) { rax } else { r10 }) as u64; r9 = ((rdx.wrapping_add(rcx)) as u64) as u64; fl_l = rdx; fl_r = r10; r10 = (if (((fl_l as u64)) < ((fl_r as u64))) { rdx } else { r10 }) as u64; r8 = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rcx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x80u64))) as u64; fl_l = r8; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } fl_l = ((rcx).wrapping_add(r10)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x80u64), rcx); r8 = rbp.wrapping_sub(0x78); crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), r8); crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), (0x100u64) as u64); bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), r14); rax = r11; rax = ((rax).wrapping_sub(r9)) as u64; rdx = rbx; fl_l = r11; fl_r = r9; rdx = (if (((fl_l as u64)) > ((fl_r as u64))) { rax } else { rdx }) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), r9); fl_l = rdx; fl_r = rdx; rdx = (if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { rbx } else { rdx }) as u64; fl_l = r8; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } rax = ((rcx.wrapping_add(rdx)) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x80u64), rax); rax = rbp.wrapping_sub(0x78); crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), (0x100u64) as u64); bb = 2; continue; }
            2 => { crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), rbx); crate::simd_ops::wu8(rcx, (0x0u8) as u8); rdx = (0xffffffffffffffffu64) as u64; bb = 3; continue; }
            3 => { rdx = ((rdx).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru8(rdi.wrapping_add(rdx))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 3; continue; } rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rsi, rbx); crate::simd_ops::wu64(rsi.wrapping_add(0x8u64), rax); crate::simd_ops::wu64(rsi.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsi.wrapping_add(0x18u64), rbx); rcx = rsi.wrapping_add(0x20); crate::simd_ops::wu64(rbp.wrapping_add(0xc8u64), rcx); crate::simd_ops::lk_and_u32(rcx, (0xff000004u32) as u32); crate::simd_ops::lk_or_u32(rcx, (0x4u32) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x3u64), (0x4u8) as u8); crate::simd_ops::wu32(rcx.wrapping_add(0x4u64), ((rbx as u32)) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x8u64), (0x0u8) as u8); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r15); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (0x1cu64) as u64); r9 = 0x1c; r8 = 0; fl_l = 0; fl_r = 0; rdx = rsp.wrapping_add(0x20); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = rsi; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x1d0u64))) as u64; rsp = ((rsp).wrapping_add(0x190u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_34893<const P0: u64, const P1: u64, const P2: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rax = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64;
    rbx = rdx;
    rdi = rcx;
    fl_l = rdx; fl_r = rax;
    if !((((fl_l as u64)) < ((fl_r as u64)))) { rdx = ((rdx).wrapping_add(1)) as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); }
    rax = ((rbx.wrapping_add(rbx.wrapping_mul(4))) as u64) as u64;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rax = ((rax).wrapping_shl(4u32)) as u64;
    rax = ((rax).wrapping_add(crate::simd_ops::ru64(rdi.wrapping_add(P2)))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_34894<const P0: u64, const P1: u64, const P2: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rax = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64;
    rbx = rdx;
    rdi = rcx;
    fl_l = rdx; fl_r = rax;
    if !((((fl_l as u64)) < ((fl_r as u64)))) { rdx = ((rdx).wrapping_add(1)) as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); }
    rax = ((rbx.wrapping_add(rbx.wrapping_mul(2))) as u64) as u64;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rax = ((rax).wrapping_shl(4u32)) as u64;
    rax = ((rax).wrapping_add(crate::simd_ops::ru64(rdi.wrapping_add(P2)))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_34895<const P0: u64, const P1: u64, const P2: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rax = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64;
    rbx = rdx;
    rdi = rcx;
    fl_l = rdx; fl_r = rax;
    if !((((fl_l as u64)) < ((fl_r as u64)))) { rdx = ((rdx).wrapping_add(1)) as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); }
    rbx = ((rbx).wrapping_shl(6u32)) as u64;
    rbx = ((rbx).wrapping_add(crate::simd_ops::ru64(rdi.wrapping_add(P2)))) as u64;
    rax = rbx;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_34924<const P0: u64, const P1: u64, const P2: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rax = (crate::simd_ops::ru32(P0)) as u32 as u64;
    rbx = rdx;
    fl_l = rdx; fl_r = rax;
    if !((((fl_l as u64)) < ((fl_r as u64)))) { rdx = ((rdx).wrapping_add(1)) as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); }
    rax = ((rbx.wrapping_add(rbx.wrapping_mul(2))) as u64) as u64;
    rax = ((rax).wrapping_shl(4u32)) as u64;
    rax = ((rax).wrapping_add(crate::simd_ops::ru64(P2))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_34925<const P0: u64, const P1: u64, const P2: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x30u64)) as u64;
    rbx = r8;
    r8 = rdx;
    xmm0 = 0u128;
    fl_l = rbx; fl_r = rbx;
    if !((((((fl_l as u64)) & ((fl_r as u64))) as i64) < 0)) { xmm0 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | ((((rbx) as i64) as f64).to_bits() as u128); } else { rax = rbx; rcx = rbx; rcx = ((rcx).wrapping_shr(1u32)) as u64; rax = ((((rax as u32)) & (0x1u32))) as u32 as u64; fl_l = (((rcx) | (rax))) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; xmm0 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | ((((rcx) as i64) as f64).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm0 as u64) + f64::from_bits(xmm0 as u64)).to_bits() as u128); }
    r9 = rsp.wrapping_add(0x40);
    crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), (0x0u32) as u32);
    rdx = P0;
    rcx = r8;
    crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), ((xmm0 as u64)) as u64);
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    rdx = rbx;
    rsp = ((rsp).wrapping_add(0x30u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return crate::simd_ops::tail_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_34947<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 1>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); r8 = ((crate::simd_ops::ru8(rcx.wrapping_add(P0))) as u32) as u32 as u64; r11 = ((rcx.wrapping_add(P1)) as u64) as u64; fl_l = ((r8 as u32)) as u32 as u64; fl_r = 0x80; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 1; continue; } rdx = ((r8.wrapping_mul(8)) as u32) as u32 as u64; bb = 2; continue; }
            1 => { rdx = ((r8 as u32)) as u32 as u64; rdx = (((rdx as u32)).wrapping_shl(8u32)) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0x7c00; rdx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru32(rcx)) as u32 as u64; rax = ((((rax as u32)) & (0xffffffu32))) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); rbx = r11; rdi = ((rax.wrapping_mul(8)) as u32) as u32 as u64; fl_l = ((rdi as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 3; continue; } rbx = (crate::simd_ops::ru64(r11)) as u64; bb = 3; continue; }
            3 => { rdx = (crate::simd_ops::ru32(rcx.wrapping_add(P2))) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); fl_l = rdx; fl_r = (0x1u64) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 5; continue; } rcx = P3; fl_l = rbx; fl_r = rcx; rbx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; rsi = P4; bb = 4; continue; }
            4 => { rax = ((crate::simd_ops::ru8(rbx.wrapping_add(rcx))) as u32) as u32 as u64; rdx = ((rdx).wrapping_sub(1)) as u64; r9 = ((crate::simd_ops::ru8(rax.wrapping_add(rsi))) as u32) as u32 as u64; rax = ((crate::simd_ops::ru8(rcx)) as u32) as u32 as u64; fl_l = ((r9 as u8)) as u8 as u64; fl_r = (crate::simd_ops::ru8(rax.wrapping_add(rsi))) as u8 as u64; if ((((fl_l as u8)) as i8) < (((fl_r as u8)) as i8)) { bb = 5; continue; } if ((((fl_l as u8)) as i8) > (((fl_r as u8)) as i8)) { bb = 5; continue; } rcx = ((rcx).wrapping_add(1)) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 4; continue; } rax = P5; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x18u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x10u64))) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            5 => { fl_l = ((r8 as u32)) as u32 as u64; fl_r = 0x80; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 6; continue; } r8 = ((r8.wrapping_mul(8)) as u32) as u32 as u64; bb = 7; continue; }
            6 => { r8 = (((r8 as u32)).wrapping_shl(8u32)) as u32 as u64; fl_l = (((r8 as u32)).wrapping_add(0xffff8400u32)) as u32 as u64; fl_r = fl_l; r8 = ((fl_l as u32)) as u32 as u64; bb = 7; continue; }
            7 => { rax = r11; fl_l = ((rdi as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 8; continue; } rax = (crate::simd_ops::ru64(r11)) as u64; bb = 8; continue; }
            8 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x18u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x10u64))) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_34953<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64, const P9: u64, const P10: u64, const P11: u64, const P12: u64, const P13: u64, const P14: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 55>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rbp = rsp.wrapping_sub(0x90); rsp = ((rsp).wrapping_sub(0x190u64)) as u64; r15 = P0; crate::simd_ops::wu64(rbp.wrapping_add(0xc0u64), r15); rax = P1; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (0x4u64) as u64); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdi = rax; r14 = P3; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), r14); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x30u64))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm0) as u128); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (0xcu64) as u64); rax = rsp.wrapping_add(0x40); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rax); rbx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x78u64), (0x7ffu64) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rdi); crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), (0x7ffu64) as u64); rsi = P4; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rsi); crate::simd_ops::wu64(rbp.wrapping_sub(0x80u64), rdi); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x30u64))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm0) as u128); xmm1 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm1) as u128); r9 = ((rbx as u32)) as u32 as u64; r8 = rsp.wrapping_add(0x30); rdx = rsp.wrapping_add(0x20); rcx = rsp.wrapping_add(0x50); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; r11 = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; rax = r11; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rax = ((rax).wrapping_sub(rcx)) as u64; r10 = ((rbx as u32)) as u32 as u64; fl_l = r11; fl_r = rcx; r10 = (if (((fl_l as u64)) > ((fl_r as u64))) { rax } else { r10 }) as u64; r9 = ((rdx.wrapping_add(rcx)) as u64) as u64; fl_l = rdx; fl_r = r10; r10 = (if (((fl_l as u64)) < ((fl_r as u64))) { rdx } else { r10 }) as u64; r8 = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rcx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x80u64))) as u64; fl_l = r8; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } fl_l = ((rcx).wrapping_add(r10)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x80u64), rcx); r8 = rbp.wrapping_sub(0x78); crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), r8); crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), (0x100u64) as u64); bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rsi); rax = r11; rax = ((rax).wrapping_sub(r9)) as u64; rdx = rbx; fl_l = r11; fl_r = r9; rdx = (if (((fl_l as u64)) > ((fl_r as u64))) { rax } else { rdx }) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), r9); fl_l = rdx; fl_r = rdx; rdx = (if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { rbx } else { rdx }) as u64; fl_l = r8; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } rax = ((rcx.wrapping_add(rdx)) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x80u64), rax); rax = rbp.wrapping_sub(0x78); crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), (0x100u64) as u64); bb = 2; continue; }
            2 => { crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), rbx); crate::simd_ops::wu8(rcx, (0x0u8) as u8); rdx = (0xffffffffffffffffu64) as u64; bb = 3; continue; }
            3 => { rdx = ((rdx).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru8(rdi.wrapping_add(rdx))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 3; continue; } rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(P0, rbx); crate::simd_ops::wu64(P7, rax); crate::simd_ops::wu64(P8, rbx); crate::simd_ops::wu64(P9, rbx); rcx = P10; crate::simd_ops::wu64(rbp.wrapping_add(0xc8u64), rcx); rax = (crate::simd_ops::ru32(P10)) as u32 as u64; rax = ((((rax as u32)) & (0xff000004u32))) as u32 as u64; rax = ((((rax as u32)) | (0x4u32))) as u32 as u64; crate::simd_ops::wu32(P10, ((rax as u32)) as u32); crate::simd_ops::wu8(P11, (0x4u8) as u8); crate::simd_ops::wu32(P12, ((rbx as u32)) as u32); crate::simd_ops::wu8(P13, (0x0u8) as u8); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r14); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (0x22u64) as u64); r9 = 0x22; r8 = 0; fl_l = 0; fl_r = 0; rdx = rsp.wrapping_add(0x20); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P14, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = r15; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x1d0u64))) as u64; rsp = ((rsp).wrapping_add(0x190u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_34970<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rbx = rcx; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = rbx; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x2; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 3; continue; } rbx = (crate::simd_ops::ru64(rbx.wrapping_add(P1))) as u64; rcx = rbx; rax = (crate::simd_ops::ru64(rbx)) as u64; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 1; continue; } bb = 2; continue; }
            2 => { rax = rbx; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            3 => { rax = 0; fl_l = 0; fl_r = 0; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_34971<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x58u64)) as u64;
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    r9 = rdx;
    r8 = P0;
    rdx = rsp.wrapping_add(0x20);
    rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64, rax, rcx, rdx, r8, r9, rsp);
    rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64;
    fl_l = rcx; fl_r = rcx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = P2; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rdx); crate::simd_ops::wu8(rsp.wrapping_add(0x40u64), (0x1u8) as u8); rdx = P3; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rdx); crate::simd_ops::wu32(rsp.wrapping_add(0x48u64), (0x0u32) as u32); r8 = rsp.wrapping_add(0x30); rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(rax.wrapping_add(P4))) as u64, rax, rcx, rdx, r8, r9, rsp); }
    rcx = rsp.wrapping_add(0x20);
    rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P5)) as u64, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x58u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_34978<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 19>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x80u64)) as u64; rbx = rcx.wrapping_add(0x8); xmm0 = (crate::simd_ops::ru128(P0)) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128); rcx = rbx.wrapping_add(0x18); r8 = rsp.wrapping_add(0x20); rdx = rsp.wrapping_add(0x30); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rbx); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; xmm0 = (crate::simd_ops::ru128(rax.wrapping_add(0x10u64))) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rsp.wrapping_add(0x48u64), (xmm0) as u128); rdx = rsp.wrapping_add(0x60); rcx = rsp.wrapping_add(0x40); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = (crate::simd_ops::ru64(rax)) as u64; rbp = ((crate::simd_ops::ru16(rax.wrapping_add(0xau64))) as u32) as u32 as u64; xmm0 = (crate::simd_ops::ru128(rax.wrapping_add(0x10u64))) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm0) as u128); rcx = rbx.wrapping_add(0x48); xmm1 = (crate::simd_ops::ru128(P3)) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm1) as u128); rdx = rsp.wrapping_add(0x30); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r14 = rax; rdi = ((crate::simd_ops::ru16(rax)) as u32) as u32 as u64; r15 = 0xffff; fl_l = ((rdi as u16)) as u16 as u64; fl_r = ((r15 as u16)) as u16 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { bb = 2; continue; } rsi = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; xmm0 = (crate::simd_ops::ru128(P3)) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128); rax = (crate::simd_ops::ru32(rsi.wrapping_add(0x18u64))) as u32 as u64; r8 = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; r8 = ((r8).wrapping_shl(4u32)) as u64; rax = (0x2aaaaaaaaaaaaaabu64) as u64; let prod = (rax as i64 as i128).wrapping_mul((r8) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; rdi = rdx; rdi = ((((rdi) as i64).wrapping_shr(3u32) as u64)) as u64; rax = rdi; rax = ((rax).wrapping_shr(63u32)) as u64; rdi = ((rdi).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru32(rsi.wrapping_add(0x1cu64))) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; fl_l = r8; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } rdx = rdi.wrapping_add(0x1); rcx = rsi.wrapping_add(0x10); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru32(rsi.wrapping_add(0x18u64))) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rsi.wrapping_add(0x10u64)))) as u64; xmm0 = 0u128; ymm0_hi = 0; let y_addr: u64 = rcx; crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); crate::simd_ops::wu128(rcx.wrapping_add(0x20u64), (xmm0) as u128); crate::simd_ops::lk_add_u32(rsi.wrapping_add(0x18u64), (1) as u32); rax = (crate::simd_ops::ru32(rsi.wrapping_add(0x18u64))) as u32 as u64; rdx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rdx = ((rdx).wrapping_shl(4u32)) as u64; fl_l = ((rdx).wrapping_add(crate::simd_ops::ru64(rsi.wrapping_add(0x10u64)))) as u64; fl_r = fl_l; rdx = ((fl_l as u64)) as u64; xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rdx.wrapping_sub(0x30u64), (xmm0) as u128); crate::simd_ops::wu32(rdx.wrapping_sub(0xeu64), (0xffffffffu32) as u32); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; crate::simd_ops::wu64(rdx.wrapping_sub(0x20u64), rax); crate::simd_ops::wu64(rdx.wrapping_sub(0x18u64), rdi); crate::simd_ops::wu16(r14, ((rdi as u16)) as u16); bb = 2; continue; }
            2 => { r9 = 0x1; r8 = (((rbp as u16)) as u32) as u32 as u64; rdx = (((rdi as u16)) as u32) as u32 as u64; rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r11 = rsp.wrapping_add(0x80); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x20u64))) as u64; rbp = (crate::simd_ops::ru64(r11.wrapping_add(0x28u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x30u64))) as u64; rsp = r11; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_34992<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    crate::simd_ops::forward_incoming_to_outgoing();
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = rcx;
    rcx = ((r8 as u32)) as u32 as u64;
    rcx = ((rcx).wrapping_shl(6u32)) as u64;
    fl_l = ((rcx).wrapping_add(rdx)) as u64; fl_r = fl_l;
    rcx = ((fl_l as u64)) as u64;
    rdx = rax;
    crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8(P0, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_34993(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
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
    loop { rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = rbx; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(rax)) as u64, rax, rcx, rdx, r8, r9, rsp); rbx = ((rbx).wrapping_add(0x40u64)) as u64; fl_l = rdi; fl_r = (0x1u64) as u64; rdi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if !((((fl_l as u64)) != ((fl_r as u64)))) { break; } }
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_34994<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = ((crate::exe::tls_pointer() as u64)) as u64; rbx = rcx; rcx = 0x74; rax = (crate::simd_ops::ru64(rax)) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(rax))) as u32 as u64; fl_l = (crate::simd_ops::ru32(P0)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) > (((fl_r as u32)) as i32)) { bb = 2; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(P1, rbx); rax = P2; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            2 => { rcx = P0; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru32(P0)) as u32 as u64; fl_r = 0xffffffff; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 1; continue; } rcx = P4; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rcx = P0; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); bb = 1; continue; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_34995(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = ((crate::simd_ops::ru8(rcx)) as u32) as u32 as u64;
    r8 = ((r8 as u32)) as u32 as u64;
    crate::simd_ops::wu8(r8.wrapping_add(rdx), ((rax as u8)) as u8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_34996<const P0: u64, const P1: u64, const P2: u64, const P3: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x58u64)) as u64; r9 = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; r8 = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rdx = r9; rdx = ((rdx).wrapping_shr(32u32)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), (0xffffffffffffffffu64) as u64); fl_l = ((r9 as u32)) as u32 as u64; fl_r = 0xffffffff; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 1; continue; } rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rax = ((rax).wrapping_shr(32u32)) as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rcx = (crate::simd_ops::ru64(r8.wrapping_add(P1))) as u64; rax = (crate::simd_ops::ru64(r8.wrapping_add(P2))) as u64; r8 = (crate::simd_ops::ru64(rcx.wrapping_add(rax.wrapping_mul(8)).wrapping_add(P3))) as u64; fl_l = ((r9 as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 2; continue; } r8 = ((r8).wrapping_shr(32u32)) as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 2; continue; } rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); rsp = ((rsp).wrapping_add(0x58u64)) as u64; return rax; }
            2 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; rsp = ((rsp).wrapping_add(0x58u64)) as u64; return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_34998<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    crate::simd_ops::wu64(rcx, rdx);
    rax = rcx;
    crate::simd_ops::wu64(rcx.wrapping_add(P0), r8);
    crate::simd_ops::wu64(rcx.wrapping_add(P1), r9);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_35001<const P0: u64, const P1: u64, const P2: u64, const P3: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x58u64)) as u64; r9 = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; r8 = (crate::simd_ops::ru64(rcx)) as u64; rdx = r9; rdx = ((rdx).wrapping_shr(32u32)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), (0xffffffffffffffffu64) as u64); fl_l = ((r9 as u32)) as u32 as u64; fl_r = 0xffffffff; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 1; continue; } rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rax = ((rax).wrapping_shr(32u32)) as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 1; continue; } rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; rsp = ((rsp).wrapping_add(0x58u64)) as u64; return rax; }
            1 => { rcx = (crate::simd_ops::ru64(r8.wrapping_add(P1))) as u64; rax = (crate::simd_ops::ru64(r8.wrapping_add(P2))) as u64; r8 = (crate::simd_ops::ru64(rcx.wrapping_add(rax.wrapping_mul(8)).wrapping_add(P3))) as u64; fl_l = ((r9 as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 2; continue; } r8 = ((r8).wrapping_shr(32u32)) as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 2; continue; } rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); rsp = ((rsp).wrapping_add(0x58u64)) as u64; return rax; }
            2 => { rax = (rax & !0xffu64) | (((0x0u8) as u8 as u64)); rsp = ((rsp).wrapping_add(0x58u64)) as u64; return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_35009<const P0: u64, const P1: u64, const P2: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x38u64)) as u64;
    rax = (crate::simd_ops::ru32(P0)) as u32 as u64;
    r8 = rsp.wrapping_add(0x20);
    rcx = (crate::simd_ops::ru64(rcx)) as u64;
    rdx = rsp.wrapping_add(0x40);
    crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rax as u32)) as u32);
    rcx = ((rcx).wrapping_add(0xc0u64)) as u64;
    rax = P1;
    crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax);
    rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x38u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_35012(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    r9 = (crate::simd_ops::ru64(rcx)) as u64;
    rax = ((r8 as u32)) as u32 as u64;
    crate::simd_ops::wu64(rcx, (0x0u64) as u64);
    crate::simd_ops::wu64(rdx.wrapping_add(rax.wrapping_mul(8)), r9);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_35013<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rsi = rdx; r14 = rcx; rbx = 0; fl_l = 0; fl_r = 0; bb = 1; continue; }
            1 => { rdi = (crate::simd_ops::ru64(r14.wrapping_add(rbx.wrapping_mul(8)))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } rax = (crate::simd_ops::ru64(rdi)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = rdi; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(rax)) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = rdi; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); bb = 2; continue; }
            2 => { rbx = ((rbx).wrapping_add(1)) as u64; fl_l = rbx; fl_r = rsi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); bb = 3; continue; }
            3 => { return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_35016(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
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
    loop { rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = rbx; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(rax)) as u64, rax, rcx, rdx, r8, r9, rsp); rbx = ((rbx).wrapping_add(0x10u64)) as u64; fl_l = rdi; fl_r = (0x1u64) as u64; rdi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if !((((fl_l as u64)) != ((fl_r as u64)))) { break; } }
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_35031<const P0: u64, const P1: u64, const P2: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rdx; rdi = rcx; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rax = ((rax as u32)) as u32 as u64; r14 = (((rax) as i64).wrapping_mul((0x188u64) as i64) as u64) as u64; rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0x10u64))) as u64; rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; rbp = (crate::simd_ops::ru64(rax.wrapping_add(rdx.wrapping_mul(8)).wrapping_add(0x10u64))) as u64; rax = (crate::simd_ops::ru64(r14.wrapping_add(rbx).wrapping_add(0x58148u64))) as u64; rcx = rax; rcx = ((rcx).wrapping_shr(32u32)) as u64; fl_l = ((rbp as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 1; continue; } rax = rbp; rax = ((rax).wrapping_shr(32u32)) as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 1; continue; } rcx = 0; fl_l = 0; fl_r = 0; bb = 2; continue; }
            1 => { rcx = 0x1; bb = 2; continue; }
            2 => { crate::simd_ops::lk_add_u64(r14.wrapping_add(rbx).wrapping_add(0x58150u64), (rcx) as u64); crate::simd_ops::wu64(r14.wrapping_add(rbx).wrapping_add(0x58148u64), rbp); rdi = ((crate::simd_ops::ru16(P1)) as u32) as u32 as u64; rsi = rbx.wrapping_add(0x58248); rsi = ((rsi).wrapping_add(r14)) as u64; rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } rax = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; rax = ((rax).wrapping_add(1)) as u64; fl_l = rax; fl_r = (0x4000u64) as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rcx = rsi; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 4; continue; }
            4 => { rcx = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu32(rcx.wrapping_add(rax.wrapping_mul(4)).wrapping_add(0x10u64), ((rbp as u32)) as u32); rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; crate::simd_ops::lk_add_u64(rax.wrapping_add(0x8u64), (1) as u64); crate::simd_ops::lk_add_u64(rsi.wrapping_add(0x10u64), (1) as u64); rbx = ((rbx).wrapping_add(0x58260u64)) as u64; rbx = ((rbx).wrapping_add(r14)) as u64; rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } rax = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; rax = ((rax).wrapping_add(1)) as u64; fl_l = rax; fl_r = (0x8000u64) as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rcx = rbx; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 6; continue; }
            6 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu16(rcx.wrapping_add(rax.wrapping_mul(2)).wrapping_add(0x10u64), ((rdi as u16)) as u16); rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; crate::simd_ops::lk_add_u64(rax.wrapping_add(0x8u64), (1) as u64); crate::simd_ops::lk_add_u64(rbx.wrapping_add(0x10u64), (1) as u64); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_35032<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x40u64)) as u64;
    rdi = rcx;
    rbx = ((rdx.wrapping_add(P0)) as u64) as u64;
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    rax = ((rax as u32)) as u32 as u64;
    rcx = (((rax) as i64).wrapping_mul((0x188u64) as i64) as u64) as u64;
    rcx = ((rcx).wrapping_add(rbx)) as u64;
    crate::simd_ops::wu8(rsp.wrapping_add(0x50u64), (0x0u8) as u8);
    rdx = (crate::simd_ops::ru64(rdi.wrapping_add(P2))) as u64;
    rax = (crate::simd_ops::ru64(rdi.wrapping_add(P3))) as u64;
    rdx = (crate::simd_ops::ru64(rax.wrapping_add(rdx.wrapping_mul(8)).wrapping_add(P2))) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdx);
    crate::simd_ops::wu8(rsp.wrapping_add(0x28u64), (0x0u8) as u8);
    xmm0 = ((crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64) as u128;
    crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), ((xmm0 as u64)) as u64);
    rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x28u64))) as u32 as u64;
    crate::simd_ops::wu32(rsp.wrapping_add(0x38u64), ((rax as u32)) as u32);
    r8 = rsp.wrapping_add(0x50);
    rdx = rsp.wrapping_add(0x30);
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64;
    rsp = ((rsp).wrapping_add(0x40u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_35037<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    crate::simd_ops::wu64(r8.wrapping_add(P0), rax);
    rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P1))) as u32) as u32 as u64;
    crate::simd_ops::wu8(r8.wrapping_add(P1), ((rax as u8)) as u8);
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(P2))) as u64;
    crate::simd_ops::wu64(r8.wrapping_add(P2), rax);
    xmm0 = (crate::simd_ops::ru128(rdx.wrapping_add(P3))) as u128;
    crate::simd_ops::wu128(r8.wrapping_add(P3), (xmm0) as u128);
    xmm1 = ((crate::simd_ops::ru64(rdx.wrapping_add(P4))) as u64) as u128;
    crate::simd_ops::wu64(r8.wrapping_add(P4), ((xmm1 as u64)) as u64);
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_35038<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = (crate::simd_ops::ru32(rcx)) as u32 as u64;
    r8 = ((r8 as u32)) as u32 as u64;
    rdx = ((rdx.wrapping_add(r8.wrapping_mul(4))) as u64) as u64;
    crate::simd_ops::wu32(rdx.wrapping_add(r8), ((rax as u32)) as u32);
    rax = ((crate::simd_ops::ru8(rcx.wrapping_add(P0))) as u32) as u32 as u64;
    crate::simd_ops::wu8(rdx.wrapping_add(r8).wrapping_add(P0), ((rax as u8)) as u8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_35039<const P0: u64, const P1: u64, const P2: u64, const P3: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = ((r8 as u32)) as u32 as u64;
    r8 = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64;
    rax = P0;
    crate::simd_ops::wu64(rdx.wrapping_add(r8.wrapping_mul(8)), rax);
    rax = (crate::simd_ops::ru32(rcx.wrapping_add(P1))) as u32 as u64;
    crate::simd_ops::wu32(rdx.wrapping_add(r8.wrapping_mul(8)).wrapping_add(P1), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rcx.wrapping_add(P2))) as u32 as u64;
    crate::simd_ops::wu32(rdx.wrapping_add(r8.wrapping_mul(8)).wrapping_add(P2), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rcx.wrapping_add(P3))) as u32 as u64;
    crate::simd_ops::wu32(rdx.wrapping_add(r8.wrapping_mul(8)).wrapping_add(P3), ((rax as u32)) as u32);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_35043<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    crate::simd_ops::forward_incoming_to_outgoing();
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rdx = ((rcx.wrapping_add(P0)) as u64) as u64;
    fl_l = ((rcx).wrapping_add(0x50u64)) as u64; fl_r = fl_l;
    rcx = ((fl_l as u64)) as u64;
    crate::simd_ops::wu32(rdx, ((xmm1 as u32)) as u32);
    crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(P1, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::set_xmm_ret(c_x0); return c_rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_35044<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    crate::simd_ops::wu32(rsp.wrapping_add(0x10u64), ((xmm1 as u32)) as u32);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    rcx = ((rcx).wrapping_add(0x50u64)) as u64;
    rdx = rsp.wrapping_add(0x38);
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_35047<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    crate::simd_ops::wu32(rsp.wrapping_add(0x10u64), ((xmm1 as u32)) as u32);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(P0), ((xmm1 as u32)) as u32);
    rcx = ((rcx).wrapping_add(0x50u64)) as u64;
    rdx = rsp.wrapping_add(0x38);
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_35048<const P0: u64, const P1: u64, const P2: u64, const P3: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    crate::simd_ops::forward_incoming_to_outgoing();
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    xmm0 = ((crate::simd_ops::ru64(rdx)) as u64) as u128;
    crate::simd_ops::wu64(rcx.wrapping_add(P0), ((xmm0 as u64)) as u64);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P1))) as u32 as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(P2), ((rax as u32)) as u32);
    fl_l = ((rcx).wrapping_add(0x50u64)) as u64; fl_r = fl_l;
    rcx = ((fl_l as u64)) as u64;
    crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(P3, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::set_xmm_ret(c_x0); return c_rax;
}
