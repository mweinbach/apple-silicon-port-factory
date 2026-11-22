#![allow(unused_imports, unused_variables, unused_mut, unused_assignments, non_snake_case, dead_code, clippy::all, unreachable_code, unused_parens)]

#[inline(never)]
pub unsafe extern "win64" fn tpl_35049<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64, const P9: u64, const P10: u64, const P11: u64, const P12: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    crate::simd_ops::forward_incoming_to_outgoing();
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    xmm0 = ((crate::simd_ops::ru64(rdx)) as u64) as u128;
    crate::simd_ops::wu64(rcx.wrapping_add(P0), ((xmm0 as u64)) as u64);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P1))) as u32 as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(P2), ((rax as u32)) as u32);
    xmm0 = ((crate::simd_ops::ru64(rdx.wrapping_add(P3))) as u64) as u128;
    crate::simd_ops::wu64(rcx.wrapping_add(P4), ((xmm0 as u64)) as u64);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P5))) as u32 as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(P6), ((rax as u32)) as u32);
    xmm0 = ((crate::simd_ops::ru64(rdx.wrapping_add(P7))) as u64) as u128;
    crate::simd_ops::wu64(rcx.wrapping_add(P8), ((xmm0 as u64)) as u64);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(P9), ((rax as u32)) as u32);
    xmm0 = ((crate::simd_ops::ru64(rdx.wrapping_add(P10))) as u64) as u128;
    crate::simd_ops::wu64(rcx.wrapping_add(P11), ((xmm0 as u64)) as u64);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x2cu64))) as u32 as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(0x4cu64), ((rax as u32)) as u32);
    fl_l = ((rcx).wrapping_add(0x50u64)) as u64; fl_r = fl_l;
    rcx = ((fl_l as u64)) as u64;
    crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(P12, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::set_xmm_ret(c_x0); return c_rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_35056<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    crate::simd_ops::forward_incoming_to_outgoing();
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    r9 = rcx;
    rax = ((r8 as u32)) as u32 as u64;
    r8 = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64;
    rcx = ((rdx.wrapping_add(r8.wrapping_mul(8))) as u64) as u64;
    rdx = r9;
    crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8(P0, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_35059<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    r8 = ((r8 as u32)) as u32 as u64;
    rax = P0;
    r8 = ((r8).wrapping_shl(5u32)) as u64;
    r8 = ((r8).wrapping_add(rdx)) as u64;
    rdx = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu64(r8, rax);
    crate::simd_ops::wu64(r8.wrapping_add(P1), rdx);
    crate::simd_ops::wu64(r8.wrapping_add(P2), rdx);
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64;
    crate::simd_ops::wu64(r8.wrapping_add(P1), rax);
    rax = (crate::simd_ops::ru32(rcx.wrapping_add(P2))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P2), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rcx.wrapping_add(P3))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P3), ((rax as u32)) as u32);
    crate::simd_ops::wu64(rcx.wrapping_add(P1), rdx);
    crate::simd_ops::wu64(rcx.wrapping_add(P2), rdx);
    rax = (crate::simd_ops::ru32(rcx.wrapping_add(P4))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P4), ((rax as u32)) as u32);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_35060(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
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
    loop { rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = rbx; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(rax)) as u64, rax, rcx, rdx, r8, r9, rsp); rbx = ((rbx).wrapping_add(0x20u64)) as u64; fl_l = rdi; fl_r = (0x1u64) as u64; rdi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if !((((fl_l as u64)) != ((fl_r as u64)))) { break; } }
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_35061<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 55>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rbp = rsp.wrapping_sub(0x90); rsp = ((rsp).wrapping_sub(0x190u64)) as u64; rsi = rcx; rax = P0; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (0x4u64) as u64); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdi = rax; r15 = P2; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), r15); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x30u64))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm0) as u128); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (0xcu64) as u64); rax = rsp.wrapping_add(0x40); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rax); rbx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x78u64), (0x7ffu64) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rdi); crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), (0x7ffu64) as u64); r14 = P3; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), r14); crate::simd_ops::wu64(rbp.wrapping_sub(0x80u64), rdi); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x30u64))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm0) as u128); xmm1 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm1) as u128); r9 = ((rbx as u32)) as u32 as u64; r8 = rsp.wrapping_add(0x30); rdx = rsp.wrapping_add(0x20); rcx = rsp.wrapping_add(0x50); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; r11 = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; rax = r11; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rax = ((rax).wrapping_sub(rcx)) as u64; r10 = ((rbx as u32)) as u32 as u64; fl_l = r11; fl_r = rcx; r10 = (if (((fl_l as u64)) > ((fl_r as u64))) { rax } else { r10 }) as u64; r9 = ((rdx.wrapping_add(rcx)) as u64) as u64; fl_l = rdx; fl_r = r10; r10 = (if (((fl_l as u64)) < ((fl_r as u64))) { rdx } else { r10 }) as u64; r8 = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rcx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x80u64))) as u64; fl_l = r8; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } fl_l = ((rcx).wrapping_add(r10)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x80u64), rcx); r8 = rbp.wrapping_sub(0x78); crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), r8); crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), (0x100u64) as u64); bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), r14); rax = r11; rax = ((rax).wrapping_sub(r9)) as u64; rdx = rbx; fl_l = r11; fl_r = r9; rdx = (if (((fl_l as u64)) > ((fl_r as u64))) { rax } else { rdx }) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), r9); fl_l = rdx; fl_r = rdx; rdx = (if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { rbx } else { rdx }) as u64; fl_l = r8; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } rax = ((rcx.wrapping_add(rdx)) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x80u64), rax); rax = rbp.wrapping_sub(0x78); crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), (0x100u64) as u64); bb = 2; continue; }
            2 => { crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), rbx); crate::simd_ops::wu8(rcx, (0x0u8) as u8); rdx = (0xffffffffffffffffu64) as u64; bb = 3; continue; }
            3 => { rdx = ((rdx).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru8(rdi.wrapping_add(rdx))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 3; continue; } rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rsi, rbx); crate::simd_ops::wu64(rsi.wrapping_add(0x8u64), rax); crate::simd_ops::wu64(rsi.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsi.wrapping_add(0x18u64), rbx); rcx = rsi.wrapping_add(0x20); crate::simd_ops::wu64(rbp.wrapping_add(0xc8u64), rcx); crate::simd_ops::lk_and_u32(rcx, (0xff000004u32) as u32); crate::simd_ops::lk_or_u32(rcx, (0x4u32) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x3u64), (0x4u8) as u8); crate::simd_ops::wu32(rcx.wrapping_add(0x4u64), ((rbx as u32)) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x8u64), (0x0u8) as u8); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r15); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (0x10u64) as u64); r9 = 0x10; r8 = 0; fl_l = 0; fl_r = 0; rdx = rsp.wrapping_add(0x20); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = rsi; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x1d0u64))) as u64; rsp = ((rsp).wrapping_add(0x190u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_35062<const P0: u64, const P1: u64, const P2: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rcx;
    rcx = ((rcx).wrapping_add(0x20u64)) as u64;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rcx = ((rbx.wrapping_add(P1)) as u64) as u64;
    rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_35063<const P0: u64, const P1: u64, const P2: u64, const P3: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    r9 = rdx;
    rax = (((r8 as u16)) as u32) as u32 as u64;
    r8 = ((crate::simd_ops::ru16(P0)) as u32) as u32 as u64;
    rax = ((rax).wrapping_shl(5u32)) as u64;
    rdx = (crate::simd_ops::ru32(rax.wrapping_add(rdx).wrapping_add(P1))) as u32 as u64;
    rcx = (crate::simd_ops::ru64(rax.wrapping_add(r9).wrapping_add(P2))) as u64;
    rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp);
    fl_l = ((rax as u32)) as u32 as u64; fl_r = 0xffffffff;
    if !((((fl_l as u32)) == ((fl_r as u32)))) { rax = (0xf1a9f1a9f1a9f1a9u64) as u64; rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
    rax = 0;
    fl_l = 0; fl_r = 0;
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_35064<const P0: u64, const P1: u64, const P2: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P0))) as u32) as u32 as u64;
    rcx = (crate::simd_ops::ru64(P1)) as u64;
    rax = (((rax) as i64).wrapping_mul((0xd0u64) as i64) as u64) as u64;
    fl_l = ((rax).wrapping_add(crate::simd_ops::ru64(rcx))) as u64; fl_r = fl_l;
    rax = ((fl_l as u64)) as u64;
    rax = ((crate::simd_ops::ru8(rax.wrapping_add(P2))) as u32) as u32 as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_35065<const P0: u64, const P1: u64, const P2: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
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
    rbx = ((rbx).wrapping_shl(4u32)) as u64;
    rbx = ((rbx).wrapping_add(crate::simd_ops::ru64(rdi.wrapping_add(P2)))) as u64;
    rax = rbx;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_35082<const P0: u64, const P1: u64, const P2: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    crate::simd_ops::forward_incoming_to_outgoing();
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    rcx = ((r8.wrapping_add(P1)) as u64) as u64;
    fl_l = ((rdx).wrapping_add(0x10u64)) as u64; fl_r = fl_l;
    rdx = ((fl_l as u64)) as u64;
    crate::simd_ops::wu64(r8.wrapping_add(P0), rax);
    crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8(P2, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_35097(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
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
    loop { rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = rbx; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(rax)) as u64, rax, rcx, rdx, r8, r9, rsp); rbx = ((rbx).wrapping_add(0x98u64)) as u64; fl_l = rdi; fl_r = (0x1u64) as u64; rdi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if !((((fl_l as u64)) != ((fl_r as u64)))) { break; } }
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_35099<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut ymm0_hi: u128 = 0; let mut ymm1_hi: u128 = 0;
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let y_addr: u64 = rcx; xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16));
    rax = ((r8 as u32)) as u32 as u64;
    r8 = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64;
    fl_l = ((r8).wrapping_add(r8)) as u64; fl_r = fl_l;
    r8 = ((fl_l as u64)) as u64;
    let y_addr: u64 = rdx.wrapping_add(r8.wrapping_mul(8)); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi);
    let y_addr: u64 = rcx.wrapping_add(P0); xmm1 = crate::simd_ops::ru128(y_addr); ymm1_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16));
    let y_addr: u64 = rdx.wrapping_add(r8.wrapping_mul(8)).wrapping_add(P0); crate::simd_ops::wu128(y_addr, xmm1); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm1_hi);
    xmm0 = (crate::simd_ops::ru128(rcx.wrapping_add(P1))) as u128; ymm0_hi = 0;
    crate::simd_ops::wu128(rdx.wrapping_add(r8.wrapping_mul(8)).wrapping_add(P1), (xmm0) as u128);
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_35102<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rdx; rdi = rcx; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rcx); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rcx); crate::simd_ops::lk_and_u32(rcx, (0xff000004u32) as u32); crate::simd_ops::lk_or_u32(rcx, (0x4u32) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x3u64), (0x4u8) as u8); rsi = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rcx.wrapping_add(0x4u64), ((rsi as u32)) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x8u64), ((rsi as u8)) as u8); rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rcx = rdi.wrapping_add(0x28); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rcx); crate::simd_ops::lk_and_u32(rcx, (0xff000004u32) as u32); crate::simd_ops::lk_or_u32(rcx, (0x4u32) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x3u64), (0x4u8) as u8); crate::simd_ops::wu32(rcx.wrapping_add(0x4u64), ((rsi as u32)) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x8u64), ((rsi as u8)) as u8); rdx = rbx.wrapping_add(0x28); rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rcx = rdi.wrapping_add(0x50); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rcx); crate::simd_ops::lk_and_u32(rcx, (0xff000004u32) as u32); crate::simd_ops::lk_or_u32(rcx, (0x4u32) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x3u64), (0x4u8) as u8); crate::simd_ops::wu32(rcx.wrapping_add(0x4u64), ((rsi as u32)) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x8u64), ((rsi as u8)) as u8); rdx = rbx.wrapping_add(0x50); rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rcx = rdi.wrapping_add(0x78); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rcx); crate::simd_ops::lk_and_u32(rcx, (0xff000004u32) as u32); crate::simd_ops::lk_or_u32(rcx, (0x4u32) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x3u64), (0x4u8) as u8); crate::simd_ops::wu32(rcx.wrapping_add(0x4u64), ((rsi as u32)) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x8u64), ((rsi as u8)) as u8); rdx = rbx.wrapping_add(0x78); rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rax = ((crate::simd_ops::ru8(rbx.wrapping_add(0xa0u64))) as u32) as u32 as u64; crate::simd_ops::wu8(rdi.wrapping_add(0xa0u64), ((rax as u8)) as u8); rax = ((crate::simd_ops::ru8(rbx.wrapping_add(0xa1u64))) as u32) as u32 as u64; crate::simd_ops::wu8(rdi.wrapping_add(0xa1u64), ((rax as u8)) as u8); rax = ((crate::simd_ops::ru16(rbx.wrapping_add(0xa2u64))) as u32) as u32 as u64; crate::simd_ops::wu16(rdi.wrapping_add(0xa2u64), ((rax as u16)) as u16); rax = (crate::simd_ops::ru64(rbx.wrapping_add(0xa8u64))) as u64; crate::simd_ops::wu64(rdi.wrapping_add(0xa8u64), rax); rax = (crate::simd_ops::ru64(rbx.wrapping_add(0xb0u64))) as u64; crate::simd_ops::wu64(rdi.wrapping_add(0xb0u64), rax); rax = (crate::simd_ops::ru64(rbx.wrapping_add(0xb8u64))) as u64; crate::simd_ops::wu64(rdi.wrapping_add(0xb8u64), rax); rax = (crate::simd_ops::ru64(rbx.wrapping_add(0xc0u64))) as u64; crate::simd_ops::wu64(rdi.wrapping_add(0xc0u64), rax); rcx = rdi.wrapping_add(0xc8); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rcx); crate::simd_ops::lk_and_u32(rcx, (0xff000004u32) as u32); crate::simd_ops::lk_or_u32(rcx, (0x4u32) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x3u64), (0x4u8) as u8); crate::simd_ops::wu32(rcx.wrapping_add(0x4u64), ((rsi as u32)) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x8u64), ((rsi as u8)) as u8); rdx = rbx.wrapping_add(0xc8); rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rax = rdi; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_35103<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rdx;
    rdi = rcx;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rax = (crate::simd_ops::ru32(rbx.wrapping_add(P1))) as u32 as u64;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    crate::simd_ops::wu32(rdi.wrapping_add(P1), ((rax as u32)) as u32);
    rax = rdi;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_35123<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rax = (crate::simd_ops::ru64(rdx)) as u64;
    crate::simd_ops::wu64(rdx, (0x0u64) as u64);
    crate::simd_ops::wu64(rcx, rax);
    rax = rcx;
    xmm0 = (crate::simd_ops::ru128(rdx.wrapping_add(P0))) as u128;
    crate::simd_ops::wu128(rcx.wrapping_add(P0), (xmm0) as u128);
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_35131<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = r8;
    rdi = rcx;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rax = (crate::simd_ops::ru32(rbx)) as u32 as u64;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    crate::simd_ops::wu32(rdi.wrapping_add(P1), ((rax as u32)) as u32);
    rax = rdi;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_35132<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = (crate::simd_ops::ru32(rdx)) as u32 as u64;
    crate::simd_ops::wu32(rcx, ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(r8)) as u32 as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(P0), ((rax as u32)) as u32);
    rax = rcx;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_35169<const P0: u64, const P1: u64, const P2: u64, const P3: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = (crate::simd_ops::ru64(rdx)) as u64;
    r9 = rdx;
    r9 = ((r9).wrapping_sub(crate::simd_ops::ru64(r8))) as u64;
    rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    crate::simd_ops::wu64(r9.wrapping_add(rcx), rax);
    r9 = ((rdx.wrapping_add(P0)) as u64) as u64;
    rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P1))) as u32) as u32 as u64;
    fl_l = r9; fl_r = (crate::simd_ops::ru64(r8)) as u64;
    r9 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64;
    rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    crate::simd_ops::wu8(r9.wrapping_add(rcx).wrapping_add(P2), ((rax as u8)) as u8);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64;
    crate::simd_ops::wu32(r9.wrapping_add(rcx), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P3))) as u32 as u64;
    crate::simd_ops::wu32(r9.wrapping_add(rcx).wrapping_add(P0), ((rax as u32)) as u32);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_35170<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rax = (crate::simd_ops::ru64(rdx)) as u64;
    crate::simd_ops::wu64(r8, rax);
    xmm0 = ((crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64) as u128;
    crate::simd_ops::wu64(r8.wrapping_add(P0), ((xmm0 as u64)) as u64);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P1))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P1), ((rax as u32)) as u32);
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_35171<const P0: u64, const P1: u64, const P2: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    r9 = rdx;
    r9 = ((r9).wrapping_sub(crate::simd_ops::ru64(r8))) as u64;
    rax = (crate::simd_ops::ru64(rdx)) as u64;
    crate::simd_ops::wu64(r9.wrapping_add(rcx), rax);
    r9 = ((rdx.wrapping_add(P0)) as u64) as u64;
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64;
    fl_l = r9; fl_r = (crate::simd_ops::ru64(r8)) as u64;
    r9 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64;
    rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    crate::simd_ops::wu32(r9.wrapping_add(rcx), ((rax as u32)) as u32);
    rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P1))) as u32) as u32 as u64;
    crate::simd_ops::wu8(r9.wrapping_add(rcx).wrapping_add(P2), ((rax as u8)) as u8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_35172<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = (crate::simd_ops::ru64(rdx)) as u64;
    crate::simd_ops::wu64(r8, rax);
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    crate::simd_ops::wu64(r8.wrapping_add(P0), rax);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_35174<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = (crate::simd_ops::ru64(rdx)) as u64;
    r9 = rdx;
    r9 = ((r9).wrapping_sub(crate::simd_ops::ru64(r8))) as u64;
    rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    crate::simd_ops::wu64(r9.wrapping_add(rcx), rax);
    r9 = ((rdx.wrapping_add(P0)) as u64) as u64;
    rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P0))) as u32) as u32 as u64;
    fl_l = r9; fl_r = (crate::simd_ops::ru64(r8)) as u64;
    r9 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64;
    rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    crate::simd_ops::wu8(r9.wrapping_add(rcx), ((rax as u8)) as u8);
    rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P1))) as u32) as u32 as u64;
    crate::simd_ops::wu8(r9.wrapping_add(rcx).wrapping_add(P2), ((rax as u8)) as u8);
    rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P3))) as u32) as u32 as u64;
    crate::simd_ops::wu8(r9.wrapping_add(rcx).wrapping_add(P4), ((rax as u8)) as u8);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P5))) as u32 as u64;
    crate::simd_ops::wu32(r9.wrapping_add(rcx).wrapping_add(P6), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P7))) as u32 as u64;
    crate::simd_ops::wu32(r9.wrapping_add(rcx).wrapping_add(P0), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P8))) as u32 as u64;
    crate::simd_ops::wu32(r9.wrapping_add(rcx).wrapping_add(P5), ((rax as u32)) as u32);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_35176<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x38u64)) as u64;
    rax = (crate::simd_ops::ru64(rdx)) as u64;
    r9 = rdx;
    rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    r9 = ((r9).wrapping_sub(crate::simd_ops::ru64(r8))) as u64;
    crate::simd_ops::wu64(r9.wrapping_add(rcx), rax);
    rcx = ((rdx.wrapping_add(P0)) as u64) as u64;
    rax = rcx;
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rcx);
    rax = ((rax).wrapping_sub(crate::simd_ops::ru64(r8))) as u64;
    rdx = rsp.wrapping_add(0x20);
    rax = ((rax).wrapping_add(crate::simd_ops::ru64(r8.wrapping_add(P0)))) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax);
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x38u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_35177<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rdx;
    rcx = ((rdx.wrapping_add(P0)) as u64) as u64;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rcx = ((rbx.wrapping_add(P2)) as u64) as u64;
    rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp);
    rcx = ((rbx.wrapping_add(P4)) as u64) as u64;
    rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp);
    rcx = ((rbx.wrapping_add(P5)) as u64) as u64;
    rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp);
    rcx = ((rbx.wrapping_add(P6)) as u64) as u64;
    rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_35179<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64, const P9: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = rdx; rsi = 0; fl_l = 0; fl_r = 0; fl_l = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64; fl_r = ((rsi as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rbx = rsi; rbx = (((rbx) as i64).wrapping_mul((crate::simd_ops::ru64(rdi.wrapping_add(P1))) as i64) as u64) as u64; rbx = ((rbx).wrapping_add(rdi)) as u64; rcx = ((rbx.wrapping_add(P2)) as u64) as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rcx = ((rbx.wrapping_add(P4)) as u64) as u64; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rcx = ((rbx.wrapping_add(P6)) as u64) as u64; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rcx = ((rbx.wrapping_add(P7)) as u64) as u64; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rcx = ((rbx.wrapping_add(P8)) as u64) as u64; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rsi = ((rsi).wrapping_add(1)) as u64; rax = (crate::simd_ops::ru32(rdi.wrapping_add(P0))) as u32 as u64; fl_l = rsi; fl_r = rax; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 1; continue; } bb = 2; continue; }
            2 => { rcx = rdi; rax = crate::simd_ops::call_stk(P9, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); bb = 3; continue; }
            3 => { return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_35180<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    fl_l = rdx; fl_r = rdx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rdx; rcx = ((rdx.wrapping_add(P0)) as u64) as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rcx = ((rbx.wrapping_add(P2)) as u64) as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rcx = ((rbx.wrapping_add(P4)) as u64) as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rcx = ((rbx.wrapping_add(P5)) as u64) as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rcx = ((rbx.wrapping_add(P6)) as u64) as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rcx = rbx; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); }
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_35181<const P0: u64, const P1: u64, const P2: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x38u64)) as u64;
    rax = (crate::simd_ops::ru64(rdx)) as u64;
    r9 = rdx;
    rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    r9 = ((r9).wrapping_sub(crate::simd_ops::ru64(r8))) as u64;
    crate::simd_ops::wu64(r9.wrapping_add(rcx), rax);
    rcx = ((rdx.wrapping_add(P1)) as u64) as u64;
    rax = rcx;
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rcx);
    rax = ((rax).wrapping_sub(crate::simd_ops::ru64(r8))) as u64;
    rdx = rsp.wrapping_add(0x20);
    rax = ((rax).wrapping_add(crate::simd_ops::ru64(r8.wrapping_add(P0)))) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax);
    rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x38u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_35184<const P0: u64, const P1: u64, const P2: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    fl_l = rdx; fl_r = rdx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rcx = ((rdx.wrapping_add(P0)) as u64) as u64; rbx = rdx; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rcx = rbx; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); }
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_35185<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rax = (crate::simd_ops::ru64(rdx)) as u64;
    r9 = rdx;
    rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    rdx = ((rdx).wrapping_add(0x8u64)) as u64;
    r9 = ((r9).wrapping_sub(crate::simd_ops::ru64(r8))) as u64;
    crate::simd_ops::wu64(r9.wrapping_add(rcx), rax);
    xmm0 = ((crate::simd_ops::ru64(rdx)) as u64) as u128;
    rcx = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64;
    fl_l = rdx; fl_r = (crate::simd_ops::ru64(r8)) as u64;
    rdx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64;
    rax = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    crate::simd_ops::wu64(rdx.wrapping_add(rax), ((xmm0 as u64)) as u64);
    crate::simd_ops::wu32(rdx.wrapping_add(rax).wrapping_add(P0), ((rcx as u32)) as u32);
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_35186<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu64(rdx, rax);
    crate::simd_ops::wu64(rdx.wrapping_add(P0), rax);
    crate::simd_ops::wu32(rdx.wrapping_add(P1), ((rax as u32)) as u32);
    rax = rdx;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_35187<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    r9 = rdx;
    r9 = ((r9).wrapping_sub(crate::simd_ops::ru64(r8))) as u64;
    rax = (crate::simd_ops::ru64(rdx)) as u64;
    crate::simd_ops::wu64(r9.wrapping_add(rcx), rax);
    r9 = ((rdx.wrapping_add(P0)) as u64) as u64;
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64;
    fl_l = r9; fl_r = (crate::simd_ops::ru64(r8)) as u64;
    r9 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64;
    rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    crate::simd_ops::wu32(r9.wrapping_add(rcx), ((rax as u32)) as u32);
    rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P1))) as u32) as u32 as u64;
    crate::simd_ops::wu8(r9.wrapping_add(rcx).wrapping_add(P2), ((rax as u8)) as u8);
    rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P3))) as u32) as u32 as u64;
    crate::simd_ops::wu8(r9.wrapping_add(rcx).wrapping_add(P4), ((rax as u8)) as u8);
    rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P5))) as u32) as u32 as u64;
    crate::simd_ops::wu8(r9.wrapping_add(rcx).wrapping_add(P6), ((rax as u8)) as u8);
    rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P7))) as u32) as u32 as u64;
    crate::simd_ops::wu8(r9.wrapping_add(rcx).wrapping_add(P0), ((rax as u8)) as u8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_35190<const P0: u64, const P1: u64, const P2: u64, const P3: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    fl_l = rdx; fl_r = rdx;
    if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { return rax; }
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rdx;
    rdi = 0;
    fl_l = 0; fl_r = 0;
    fl_l = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64;
    if (((fl_l as u32)) <= ((fl_r as u32))) { rcx = rbx; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    loop { rcx = rdi; rcx = (((rcx) as i64).wrapping_mul((crate::simd_ops::ru64(rbx.wrapping_add(P1))) as i64) as u64) as u64; rcx = ((rcx).wrapping_add(0x8u64)) as u64; rcx = ((rcx).wrapping_add(rbx)) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rdi = ((rdi).wrapping_add(1)) as u64; rax = (crate::simd_ops::ru32(rbx.wrapping_add(P0))) as u32 as u64; fl_l = rdi; fl_r = rax; if !((((fl_l as u64)) < ((fl_r as u64)))) { break; } }
    rcx = rbx;
    rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp);
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_35191<const P0: u64, const P1: u64, const P2: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    fl_l = rdx; fl_r = rdx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rdx; rcx = ((rdx.wrapping_add(P0)) as u64) as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rcx = rbx; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); }
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_35192<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut ymm0_hi: u128 = 0;
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    r9 = rdx;
    rax = (crate::simd_ops::ru64(rdx)) as u64;
    r9 = ((r9).wrapping_sub(crate::simd_ops::ru64(r8))) as u64;
    crate::simd_ops::wu64(r9.wrapping_add(rcx), rax);
    rcx = ((rdx.wrapping_add(P1)) as u64) as u64;
    let y_addr: u64 = rcx; xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16));
    fl_l = rcx; fl_r = (crate::simd_ops::ru64(r8)) as u64;
    rcx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64;
    rax = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    let y_addr: u64 = rcx.wrapping_add(rax); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi);
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_35195<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rcx = r8;
    rdi = r8;
    rbx = rdx;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rdx = rbx;
    rcx = rdi;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return crate::simd_ops::tail_stk(P0, rax, rcx, rdx, r8, r9, rsp);
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_35197<const P0: u64, const P1: u64, const P2: u64, const P3: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    fl_l = rdx; fl_r = rdx;
    if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { return rax; }
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rdx;
    rdi = 0;
    fl_l = 0; fl_r = 0;
    fl_l = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64;
    if (((fl_l as u32)) <= ((fl_r as u32))) { rcx = rbx; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    loop { rcx = rdi; rcx = (((rcx) as i64).wrapping_mul((crate::simd_ops::ru64(rbx.wrapping_add(P1))) as i64) as u64) as u64; rcx = ((rcx).wrapping_add(rbx)) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rdi = ((rdi).wrapping_add(1)) as u64; rax = (crate::simd_ops::ru32(rbx.wrapping_add(P0))) as u32 as u64; fl_l = rdi; fl_r = rax; if !((((fl_l as u64)) < ((fl_r as u64)))) { break; } }
    rcx = rbx;
    rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp);
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_35198<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    fl_l = rdx; fl_r = rdx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rdx; rcx = rdx; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rcx = rbx; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); }
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_35199<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x40u64)) as u64;
    rbx = r8;
    crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), (0x1u32) as u32);
    r8 = (crate::simd_ops::ru64(P0)) as u64;
    rcx = rbx;
    crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), (0x1u32) as u32);
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rdx = (crate::simd_ops::ru32(rbx.wrapping_add(P2))) as u32 as u64;
    rax = (crate::simd_ops::ru64(rbx)) as u64;
    rcx = ((rdx.wrapping_add(rdx.wrapping_mul(2))) as u64) as u64;
    rcx = ((rcx).wrapping_add(rcx)) as u64;
    rcx = (crate::simd_ops::ru32(rax.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(P3))) as u32 as u64;
    rax = ((rdx.wrapping_add(P4)) as u32) as u32 as u64;
    crate::simd_ops::wu32(rbx.wrapping_add(P2), ((rax as u32)) as u32);
    crate::simd_ops::wu32(rbx.wrapping_add(P5), ((rcx as u32)) as u32);
    rsp = ((rsp).wrapping_add(0x40u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_35200<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rcx = r8;
    rdi = r8;
    rbx = rdx;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rax = (crate::simd_ops::ru64(rbx.wrapping_add(P1))) as u64;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    crate::simd_ops::wu64(rdi.wrapping_add(P1), rax);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_35205<const P0: u64, const P1: u64, const P2: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x38u64)) as u64;
    rcx = 0;
    fl_l = 0; fl_r = 0;
    r9 = P0;
    rax = r8;
    crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rcx);
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rcx);
    rdx = ((rdx).wrapping_add(0x30u64)) as u64;
    r8 = ((rcx.wrapping_add(P1)) as u32) as u32 as u64;
    rcx = rax;
    rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x38u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_35207<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    crate::simd_ops::forward_incoming_to_outgoing();
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    r9 = rdx;
    rax = (crate::simd_ops::ru32(rdx)) as u32 as u64;
    rdx = ((rdx).wrapping_add(0x4u64)) as u64;
    r9 = ((r9).wrapping_sub(crate::simd_ops::ru64(r8))) as u64;
    crate::simd_ops::wu32(r9.wrapping_add(rcx), ((rax as u32)) as u32);
    rcx = rdx;
    rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(r8))) as u64;
    fl_l = ((rcx).wrapping_add(crate::simd_ops::ru64(r8.wrapping_add(P0)))) as u64; fl_r = fl_l;
    rcx = ((fl_l as u64)) as u64;
    crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8(P1, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_35212<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x40u64)) as u64;
    rbx = r8;
    crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), (0x1u32) as u32);
    r8 = (crate::simd_ops::ru64(P0)) as u64;
    rcx = rbx;
    crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), (0x3u32) as u32);
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rdx = (crate::simd_ops::ru32(rbx.wrapping_add(P2))) as u32 as u64;
    rax = (crate::simd_ops::ru64(rbx)) as u64;
    rcx = ((rdx.wrapping_add(rdx.wrapping_mul(2))) as u64) as u64;
    rcx = ((rcx).wrapping_add(rcx)) as u64;
    rcx = (crate::simd_ops::ru32(rax.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(P3))) as u32 as u64;
    rax = ((rdx.wrapping_add(P4)) as u32) as u32 as u64;
    crate::simd_ops::wu32(rbx.wrapping_add(P2), ((rax as u32)) as u32);
    crate::simd_ops::wu32(rbx.wrapping_add(P5), ((rcx as u32)) as u32);
    rsp = ((rsp).wrapping_add(0x40u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_35214<const P0: u64, const P1: u64, const P2: u64, const P3: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rdx;
    rcx = ((rdx.wrapping_add(P0)) as u64) as u64;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rcx = ((rbx.wrapping_add(P2)) as u64) as u64;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rcx = rbx;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return crate::simd_ops::tail_stk(P3, rax, rcx, rdx, r8, r9, rsp);
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_35216<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = rdx; rsi = 0; fl_l = 0; fl_r = 0; fl_l = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64; fl_r = ((rsi as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rbx = rsi; rbx = (((rbx) as i64).wrapping_mul((crate::simd_ops::ru64(rdi.wrapping_add(P1))) as i64) as u64) as u64; rbx = ((rbx).wrapping_add(rdi)) as u64; rcx = ((rbx.wrapping_add(P2)) as u64) as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rcx = ((rbx.wrapping_add(P4)) as u64) as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rcx = rbx; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rsi = ((rsi).wrapping_add(1)) as u64; rax = (crate::simd_ops::ru32(rdi.wrapping_add(P0))) as u32 as u64; fl_l = rsi; fl_r = rax; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 1; continue; } bb = 2; continue; }
            2 => { rcx = rdi; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); bb = 3; continue; }
            3 => { return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_35217<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    fl_l = rdx; fl_r = rdx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rdx; rcx = ((rdx.wrapping_add(P0)) as u64) as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rcx = ((rbx.wrapping_add(P2)) as u64) as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rcx = rbx; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rcx = rbx; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); }
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_35218<const P0: u64, const P1: u64, const P2: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    r9 = rdx;
    rax = (crate::simd_ops::ru32(rdx)) as u32 as u64;
    r9 = ((r9).wrapping_sub(crate::simd_ops::ru64(r8))) as u64;
    crate::simd_ops::wu32(r9.wrapping_add(rcx), ((rax as u32)) as u32);
    rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(r8))) as u64;
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P1))) as u32 as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(rdx).wrapping_add(P1), ((rax as u32)) as u32);
    rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(r8))) as u64;
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(rdx).wrapping_add(P0), ((rax as u32)) as u32);
    rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    fl_l = rcx; fl_r = (crate::simd_ops::ru64(r8)) as u64;
    rcx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64;
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P2))) as u32 as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(rdx).wrapping_add(P2), ((rax as u32)) as u32);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_35219<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x40u64)) as u64;
    rbx = r8;
    crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), (0x1u32) as u32);
    r8 = (crate::simd_ops::ru64(P0)) as u64;
    rcx = rbx;
    crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), (0x2u32) as u32);
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rdx = (crate::simd_ops::ru32(rbx.wrapping_add(P2))) as u32 as u64;
    rax = (crate::simd_ops::ru64(rbx)) as u64;
    rcx = ((rdx.wrapping_add(rdx.wrapping_mul(2))) as u64) as u64;
    rcx = ((rcx).wrapping_add(rcx)) as u64;
    rcx = (crate::simd_ops::ru32(rax.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(P3))) as u32 as u64;
    rax = ((rdx.wrapping_add(P4)) as u32) as u32 as u64;
    crate::simd_ops::wu32(rbx.wrapping_add(P2), ((rax as u32)) as u32);
    crate::simd_ops::wu32(rbx.wrapping_add(P5), ((rcx as u32)) as u32);
    rsp = ((rsp).wrapping_add(0x40u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_35220<const P0: u64, const P1: u64, const P2: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x38u64)) as u64;
    rcx = 0;
    fl_l = 0; fl_r = 0;
    r9 = P0;
    rax = r8;
    crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rcx);
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rcx);
    rdx = ((rdx).wrapping_add(0x10u64)) as u64;
    r8 = ((rcx.wrapping_add(P1)) as u32) as u32 as u64;
    rcx = rax;
    rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x38u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_35226<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    xmm0 = (crate::simd_ops::ru128(rdx)) as u128;
    crate::simd_ops::wu128(r8, (xmm0) as u128);
    xmm1 = ((crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64) as u128;
    crate::simd_ops::wu64(r8.wrapping_add(P0), ((xmm1 as u64)) as u64);
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_35228<const P0: u64, const P1: u64, const P2: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x38u64)) as u64;
    rcx = 0;
    fl_l = 0; fl_r = 0;
    r9 = P0;
    rax = r8;
    crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rcx);
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rcx);
    rdx = ((rdx).wrapping_add(0x40u64)) as u64;
    r8 = ((rcx.wrapping_add(P1)) as u32) as u32 as u64;
    rcx = rax;
    rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x38u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_35230<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rax = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    fl_l = rax; fl_r = (crate::simd_ops::ru64(r8)) as u64;
    rax = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64;
    xmm0 = ((crate::simd_ops::ru64(rdx)) as u64) as u128;
    rcx = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64;
    crate::simd_ops::wu64(rax.wrapping_add(rdx), ((xmm0 as u64)) as u64);
    crate::simd_ops::wu32(rax.wrapping_add(rdx).wrapping_add(P0), ((rcx as u32)) as u32);
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_35233<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    fl_l = rcx; fl_r = (crate::simd_ops::ru64(r8)) as u64;
    rcx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64;
    rax = ((crate::simd_ops::ru8(rdx)) as u32) as u32 as u64;
    crate::simd_ops::wu8(rcx.wrapping_add(rdx), ((rax as u8)) as u8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_35235<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x40u64)) as u64;
    rbx = r8;
    crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), (0x2u32) as u32);
    r8 = (crate::simd_ops::ru64(P0)) as u64;
    rcx = rbx;
    crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), (0x6u32) as u32);
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rdx = (crate::simd_ops::ru32(rbx.wrapping_add(P2))) as u32 as u64;
    rax = (crate::simd_ops::ru64(rbx)) as u64;
    rcx = ((rdx.wrapping_add(rdx.wrapping_mul(2))) as u64) as u64;
    rcx = ((rcx).wrapping_add(rcx)) as u64;
    rcx = (crate::simd_ops::ru32(rax.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(P3))) as u32 as u64;
    rax = ((rdx.wrapping_add(P4)) as u32) as u32 as u64;
    crate::simd_ops::wu32(rbx.wrapping_add(P2), ((rax as u32)) as u32);
    crate::simd_ops::wu32(rbx.wrapping_add(P5), ((rcx as u32)) as u32);
    rsp = ((rsp).wrapping_add(0x40u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_35237<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut ymm0_hi: u128 = 0;
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rax = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    let y_addr: u64 = rdx; xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16));
    fl_l = rax; fl_r = (crate::simd_ops::ru64(r8)) as u64;
    rax = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64;
    let y_addr: u64 = rax.wrapping_add(rdx); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi);
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_35238<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x40u64)) as u64;
    rbx = r8;
    crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), (0x3u32) as u32);
    r8 = (crate::simd_ops::ru64(P0)) as u64;
    rcx = rbx;
    crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), (0x3u32) as u32);
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rdx = (crate::simd_ops::ru32(rbx.wrapping_add(P2))) as u32 as u64;
    rax = (crate::simd_ops::ru64(rbx)) as u64;
    rcx = ((rdx.wrapping_add(rdx.wrapping_mul(2))) as u64) as u64;
    rcx = ((rcx).wrapping_add(rcx)) as u64;
    rcx = (crate::simd_ops::ru32(rax.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(P3))) as u32 as u64;
    rax = ((rdx.wrapping_add(P4)) as u32) as u32 as u64;
    crate::simd_ops::wu32(rbx.wrapping_add(P2), ((rax as u32)) as u32);
    crate::simd_ops::wu32(rbx.wrapping_add(P5), ((rcx as u32)) as u32);
    rsp = ((rsp).wrapping_add(0x40u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_35239(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut ymm0_hi: u128 = 0;
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let y_addr: u64 = rdx; xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16));
    let y_addr: u64 = r8; crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi);
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_35240<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x40u64)) as u64;
    rbx = r8;
    crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), (0x3u32) as u32);
    r8 = (crate::simd_ops::ru64(P0)) as u64;
    rcx = rbx;
    crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), (0x9u32) as u32);
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rdx = (crate::simd_ops::ru32(rbx.wrapping_add(P2))) as u32 as u64;
    rax = (crate::simd_ops::ru64(rbx)) as u64;
    rcx = ((rdx.wrapping_add(rdx.wrapping_mul(2))) as u64) as u64;
    rcx = ((rcx).wrapping_add(rcx)) as u64;
    rcx = (crate::simd_ops::ru32(rax.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(P3))) as u32 as u64;
    rax = ((rdx.wrapping_add(P4)) as u32) as u32 as u64;
    crate::simd_ops::wu32(rbx.wrapping_add(P2), ((rax as u32)) as u32);
    crate::simd_ops::wu32(rbx.wrapping_add(P5), ((rcx as u32)) as u32);
    rsp = ((rsp).wrapping_add(0x40u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_35252(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    r8 = ((r8 as u32)) as u32 as u64;
    crate::simd_ops::wu64(rdx.wrapping_add(r8.wrapping_mul(8)), rax);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_35253(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = (crate::simd_ops::ru64(rdx)) as u64;
    crate::simd_ops::wu64(rcx, rax);
    rax = rcx;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_35266<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
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
    loop { rcx = rbx; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rbx = ((rbx).wrapping_add(0x18u64)) as u64; fl_l = rdi; fl_r = (0x1u64) as u64; rdi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if !((((fl_l as u64)) != ((fl_r as u64)))) { break; } }
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_35267<const P0: u64, const P1: u64, const P2: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P0))) as u32) as u32 as u64;
    crate::simd_ops::wu8(rcx.wrapping_add(P0), ((rax as u8)) as u8);
    fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64;
    if !(((((fl_l as u8)) & ((fl_r as u8))) == 0)) { rax = ((crate::simd_ops::ru8(rdx)) as u32) as u32 as u64; crate::simd_ops::wu8(rcx, ((rax as u8)) as u8); } else { rax = (crate::simd_ops::ru32(rdx)) as u32 as u64; crate::simd_ops::wu32(rcx, ((rax as u32)) as u32); }
    rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P1))) as u32) as u32 as u64;
    crate::simd_ops::wu8(rcx.wrapping_add(P1), ((rax as u8)) as u8);
    fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64;
    if !(((((fl_l as u8)) & ((fl_r as u8))) == 0)) { rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P2))) as u32) as u32 as u64; crate::simd_ops::wu8(rcx.wrapping_add(P2), ((rax as u8)) as u8); rax = rcx; return rax; }
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P2))) as u32 as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(P2), ((rax as u32)) as u32);
    rax = rcx;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_35273<const P0: u64, const P1: u64, const P2: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P0))) as u32) as u32 as u64;
    crate::simd_ops::wu8(rcx.wrapping_add(P0), ((rax as u8)) as u8);
    fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64;
    if !(((((fl_l as u8)) & ((fl_r as u8))) == 0)) { rax = ((crate::simd_ops::ru8(rdx)) as u32) as u32 as u64; crate::simd_ops::wu8(rcx, ((rax as u8)) as u8); } else { rax = (crate::simd_ops::ru32(rdx)) as u32 as u64; crate::simd_ops::wu32(rcx, ((rax as u32)) as u32); }
    rax = ((crate::simd_ops::ru8(r8.wrapping_add(P0))) as u32) as u32 as u64;
    crate::simd_ops::wu8(rcx.wrapping_add(P1), ((rax as u8)) as u8);
    fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64;
    if !(((((fl_l as u8)) & ((fl_r as u8))) == 0)) { rax = ((crate::simd_ops::ru8(r8)) as u32) as u32 as u64; crate::simd_ops::wu8(rcx.wrapping_add(P2), ((rax as u8)) as u8); rax = rcx; return rax; }
    rax = (crate::simd_ops::ru32(r8)) as u32 as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(P2), ((rax as u32)) as u32);
    rax = rcx;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_35274<const P0: u64, const P1: u64, const P2: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = (crate::simd_ops::ru64(rdx)) as u64;
    crate::simd_ops::wu64(rcx, rax);
    rax = ((crate::simd_ops::ru8(r8.wrapping_add(P0))) as u32) as u32 as u64;
    crate::simd_ops::wu8(rcx.wrapping_add(P1), ((rax as u8)) as u8);
    fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64;
    if !(((((fl_l as u8)) & ((fl_r as u8))) == 0)) { rax = ((crate::simd_ops::ru8(r8)) as u32) as u32 as u64; crate::simd_ops::wu8(rcx.wrapping_add(P2), ((rax as u8)) as u8); rax = rcx; return rax; }
    rax = (crate::simd_ops::ru32(r8)) as u32 as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(P2), ((rax as u32)) as u32);
    rax = rcx;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_35285<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 55>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rbp = rsp.wrapping_sub(0x90); rsp = ((rsp).wrapping_sub(0x190u64)) as u64; rsi = rcx; rax = P0; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (0x4u64) as u64); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdi = rax; r15 = P2; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), r15); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x30u64))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm0) as u128); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (0xcu64) as u64); rax = rsp.wrapping_add(0x40); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rax); rbx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x78u64), (0x7ffu64) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rdi); crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), (0x7ffu64) as u64); r14 = P3; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), r14); crate::simd_ops::wu64(rbp.wrapping_sub(0x80u64), rdi); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x30u64))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm0) as u128); xmm1 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm1) as u128); r9 = ((rbx as u32)) as u32 as u64; r8 = rsp.wrapping_add(0x30); rdx = rsp.wrapping_add(0x20); rcx = rsp.wrapping_add(0x50); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; r11 = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; rax = r11; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rax = ((rax).wrapping_sub(rcx)) as u64; r10 = ((rbx as u32)) as u32 as u64; fl_l = r11; fl_r = rcx; r10 = (if (((fl_l as u64)) > ((fl_r as u64))) { rax } else { r10 }) as u64; r9 = ((rdx.wrapping_add(rcx)) as u64) as u64; fl_l = rdx; fl_r = r10; r10 = (if (((fl_l as u64)) < ((fl_r as u64))) { rdx } else { r10 }) as u64; r8 = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rcx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x80u64))) as u64; fl_l = r8; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } fl_l = ((rcx).wrapping_add(r10)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x80u64), rcx); r8 = rbp.wrapping_sub(0x78); crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), r8); crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), (0x100u64) as u64); bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), r14); rax = r11; rax = ((rax).wrapping_sub(r9)) as u64; rdx = rbx; fl_l = r11; fl_r = r9; rdx = (if (((fl_l as u64)) > ((fl_r as u64))) { rax } else { rdx }) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), r9); fl_l = rdx; fl_r = rdx; rdx = (if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { rbx } else { rdx }) as u64; fl_l = r8; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } rax = ((rcx.wrapping_add(rdx)) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x80u64), rax); rax = rbp.wrapping_sub(0x78); crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), (0x100u64) as u64); bb = 2; continue; }
            2 => { crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), rbx); crate::simd_ops::wu8(rcx, (0x0u8) as u8); rdx = (0xffffffffffffffffu64) as u64; bb = 3; continue; }
            3 => { rdx = ((rdx).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru8(rdi.wrapping_add(rdx))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 3; continue; } rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rsi, rbx); crate::simd_ops::wu64(rsi.wrapping_add(0x8u64), rax); crate::simd_ops::wu64(rsi.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsi.wrapping_add(0x18u64), rbx); rcx = rsi.wrapping_add(0x20); crate::simd_ops::wu64(rbp.wrapping_add(0xc8u64), rcx); crate::simd_ops::lk_and_u32(rcx, (0xff000004u32) as u32); crate::simd_ops::lk_or_u32(rcx, (0x4u32) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x3u64), (0x4u8) as u8); crate::simd_ops::wu32(rcx.wrapping_add(0x4u64), ((rbx as u32)) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x8u64), (0x0u8) as u8); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r15); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (0x1fu64) as u64); r9 = 0x1f; r8 = 0; fl_l = 0; fl_r = 0; rdx = rsp.wrapping_add(0x20); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = rsi; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x1d0u64))) as u64; rsp = ((rsp).wrapping_add(0x190u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_35320<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
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
    loop { rcx = rbx; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rbx = ((rbx).wrapping_add(0x30u64)) as u64; fl_l = rdi; fl_r = (0x1u64) as u64; rdi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if !((((fl_l as u64)) != ((fl_r as u64)))) { break; } }
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_35321<const P0: u64, const P1: u64, const P2: u64, const P3: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = ((r8 as u32)) as u32 as u64;
    r8 = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64;
    rax = P0;
    crate::simd_ops::wu64(rdx.wrapping_add(r8.wrapping_mul(8)), rax);
    rax = ((crate::simd_ops::ru8(rcx.wrapping_add(P1))) as u32) as u32 as u64;
    crate::simd_ops::wu8(rdx.wrapping_add(r8.wrapping_mul(8)).wrapping_add(P1), ((rax as u8)) as u8);
    rax = (crate::simd_ops::ru32(rcx.wrapping_add(P2))) as u32 as u64;
    crate::simd_ops::wu32(rdx.wrapping_add(r8.wrapping_mul(8)).wrapping_add(P2), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rcx.wrapping_add(P3))) as u32 as u64;
    crate::simd_ops::wu32(rdx.wrapping_add(r8.wrapping_mul(8)).wrapping_add(P3), ((rax as u32)) as u32);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_35322(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
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
    loop { rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = rbx; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(rax)) as u64, rax, rcx, rdx, r8, r9, rsp); rbx = ((rbx).wrapping_add(0x18u64)) as u64; fl_l = rdi; fl_r = (0x1u64) as u64; rdi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if !((((fl_l as u64)) != ((fl_r as u64)))) { break; } }
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_35323(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    xmm0 = (crate::simd_ops::ru128(rcx)) as u128;
    rax = ((r8 as u32)) as u32 as u64;
    fl_l = ((rax).wrapping_add(rax)) as u64; fl_r = fl_l;
    rax = ((fl_l as u64)) as u64;
    crate::simd_ops::wu128(rdx.wrapping_add(rax.wrapping_mul(8)), (xmm0) as u128);
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_35325<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    crate::simd_ops::forward_incoming_to_outgoing();
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    r9 = rcx;
    rax = ((r8 as u32)) as u32 as u64;
    rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64;
    rcx = ((rcx).wrapping_shl(5u32)) as u64;
    fl_l = ((rcx).wrapping_add(rdx)) as u64; fl_r = fl_l;
    rcx = ((fl_l as u64)) as u64;
    rdx = r9;
    crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8(P0, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_35328<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
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
    loop { rcx = rbx; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rbx = ((rbx).wrapping_add(0x40u64)) as u64; fl_l = rdi; fl_r = (0x1u64) as u64; rdi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if !((((fl_l as u64)) != ((fl_r as u64)))) { break; } }
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_35331(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = ((crate::simd_ops::ru16(rcx)) as u32) as u32 as u64;
    r8 = ((r8 as u32)) as u32 as u64;
    crate::simd_ops::wu16(rdx.wrapping_add(r8.wrapping_mul(2)), ((rax as u16)) as u16);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_35332(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = (crate::simd_ops::ru32(rcx)) as u32 as u64;
    r8 = ((r8 as u32)) as u32 as u64;
    crate::simd_ops::wu32(rdx.wrapping_add(r8.wrapping_mul(4)), ((rax as u32)) as u32);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_35335<const P0: u64, const P1: u64, const P2: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut ymm0_hi: u128 = 0;
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rax = P0;
    crate::simd_ops::wu64(rdx, rax);
    rax = rdx;
    let y_addr: u64 = rcx.wrapping_add(P1); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16));
    let y_addr: u64 = rdx.wrapping_add(P1); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi);
    xmm1 = (crate::simd_ops::ru128(rcx.wrapping_add(P2))) as u128;
    crate::simd_ops::wu128(rdx.wrapping_add(P2), (xmm1) as u128);
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_35336<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    crate::simd_ops::forward_incoming_to_outgoing();
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    fl_l = ((rdx as u8)) as u8 as u64; fl_r = ((rdx as u8)) as u8 as u64;
    if !(((((fl_l as u8)) & ((fl_r as u8))) == 0)) { rdx = 0x38; crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8(P0, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_35375<const P0: u64, const P1: u64, const P2: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut ymm0_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x50u64)) as u64;
    let y_addr: u64 = rcx; xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16));
    xmm1 = (crate::simd_ops::ru128(rcx.wrapping_add(P0))) as u128;
    rbx = (crate::simd_ops::ru64(r8)) as u64;
    rcx = rsp.wrapping_add(0x20);
    rbx = ((rbx).wrapping_shl(6u32)) as u64;
    rbx = ((rbx).wrapping_add(crate::simd_ops::ru64(rdx.wrapping_add(P0)))) as u64;
    rdx = r9;
    let y_addr: u64 = rsp.wrapping_add(0x20u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi);
    crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm1) as u128);
    r8 = (crate::simd_ops::ru32(rbx)) as u32 as u64;
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) + f32::from_bits(crate::simd_ops::ru32(rbx.wrapping_add(P2)))).to_bits() as u128); ymm0_hi = 0;
    rsp = ((rsp).wrapping_add(0x50u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_35376<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l) = (crate::simd_ops::get_rax_in(), 0u64);
    r8 = (crate::simd_ops::ru64(rdx)) as u64;
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    fl_l = ((r8).wrapping_shl(6u32)) as u64;
    r8 = ((fl_l as u64)) as u64;
    rax = ((crate::simd_ops::ru8(r8.wrapping_add(rax).wrapping_add(P1))) as u32) as u32 as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_35378<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rdx = (((crate::simd_ops::ru64(rdx)) as i64).wrapping_mul((0x3cu64) as i64) as u64) as u64;
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    xmm0 = ((crate::simd_ops::ru32(rdx.wrapping_add(rax).wrapping_add(P1))) as u32) as u128;
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_35384<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rdx = (((crate::simd_ops::ru64(rdx)) as i64).wrapping_mul((0x3cu64) as i64) as u64) as u64;
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    rax = ((crate::simd_ops::ru8(rdx.wrapping_add(rax).wrapping_add(P1))) as u32) as u32 as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_35386<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rdx = (((crate::simd_ops::ru64(rdx)) as i64).wrapping_mul((0x5cu64) as i64) as u64) as u64;
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    rax = ((crate::simd_ops::ru8(rdx.wrapping_add(rax).wrapping_add(P0))) as u32) as u32 as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_35390<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rdx = (((crate::simd_ops::ru64(rdx)) as i64).wrapping_mul((0x5cu64) as i64) as u64) as u64;
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    xmm0 = ((crate::simd_ops::ru32(rdx.wrapping_add(rax).wrapping_add(P0))) as u32) as u128;
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_35391<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rdx = (((crate::simd_ops::ru64(rdx)) as i64).wrapping_mul((0x5cu64) as i64) as u64) as u64;
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(rax).wrapping_add(P0))) as u32 as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_35394<const P0: u64, const P1: u64, const P2: u64, const P3: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut ymm1_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdx);
    rsp = ((rsp).wrapping_sub(0x48u64)) as u64;
    let y_addr: u64 = rcx; xmm1 = crate::simd_ops::ru128(y_addr); ymm1_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16));
    xmm0 = (ymm1_hi) as u128;
    let y_addr: u64 = rsp.wrapping_add(0x20u64); crate::simd_ops::wu128(y_addr, xmm1); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm1_hi);
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64;
    rdx = (((xmm0 >> 64) as u64)) as u64;
    fl_l = (crate::simd_ops::ru32(rax.wrapping_add(rdx.wrapping_mul(4)))) as u32 as u64; fl_r = 0x0;
    if !((((fl_l as u32)) == ((fl_r as u32)))) { rax = (((xmm1 >> 64) as u64)) as u64; rdx = ((rdx.wrapping_add(rdx.wrapping_mul(2))) as u64) as u64; crate::simd_ops::wu8(rax.wrapping_add(rdx.wrapping_mul(4)).wrapping_add(P0), (0x0u8) as u8); rax = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; rdx = (crate::simd_ops::ru64(rcx.wrapping_add(P2))) as u64; rcx = rsp.wrapping_add(0x58); rdx = (crate::simd_ops::ru64(rdx.wrapping_add(rax.wrapping_mul(8)).wrapping_add(P2))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); }
    rsp = ((rsp).wrapping_add(0x48u64)) as u64;
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_35396<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64, const P9: u64, const P10: u64, const P11: u64, const P12: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 19>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rbp = rsp; rsp = ((rsp).wrapping_sub(0x70u64)) as u64; rbx = rcx; rcx = (crate::simd_ops::ru64(rcx)) as u64; rcx = ((rcx).wrapping_add(0xc0u64)) as u64; rax = (crate::simd_ops::ru32(P0)) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_sub(0x50u64), ((rax as u32)) as u32); rax = P1; crate::simd_ops::wu64(rbp.wrapping_sub(0x48u64), rax); r8 = rbp.wrapping_sub(0x50); rdx = rbp.wrapping_add(0x30); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = ((rbx).wrapping_add(0x8u64)) as u64; xmm0 = (crate::simd_ops::ru128(P3)) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rbp.wrapping_sub(0x50u64), (xmm0) as u128); rcx = rbx.wrapping_add(0x18); r8 = rbp.wrapping_sub(0x50); rdx = rbp.wrapping_sub(0x30); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rbp.wrapping_sub(0x50u64), rbx); rax = (crate::simd_ops::ru64(rbp.wrapping_sub(0x28u64))) as u64; xmm0 = (crate::simd_ops::ru128(rax.wrapping_add(0x10u64))) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rbp.wrapping_sub(0x48u64), (xmm0) as u128); rdx = rbp.wrapping_sub(0x20); rcx = rbp.wrapping_sub(0x50); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rsi = (crate::simd_ops::ru64(rbp.wrapping_sub(0x20u64))) as u64; rcx = rsi.wrapping_add(0x48); xmm0 = (crate::simd_ops::ru128(P6)) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rbp.wrapping_sub(0x30u64), (xmm0) as u128); rdx = rbp.wrapping_sub(0x30); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r14 = rax; rbx = ((crate::simd_ops::ru16(rax)) as u32) as u32 as u64; r15 = 0xffff; r12 = (0x2aaaaaaaaaaaaaabu64) as u64; fl_l = ((rbx as u16)) as u16 as u64; fl_r = ((r15 as u16)) as u16 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { bb = 2; continue; } rdi = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; xmm0 = (crate::simd_ops::ru128(P6)) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rbp.wrapping_sub(0x30u64), (xmm0) as u128); rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x18u64))) as u32 as u64; r8 = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; r8 = ((r8).wrapping_shl(4u32)) as u64; rax = r12; let prod = (rax as i64 as i128).wrapping_mul((r8) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; rbx = rdx; rbx = ((((rbx) as i64).wrapping_shr(3u32) as u64)) as u64; rax = rbx; rax = ((rax).wrapping_shr(63u32)) as u64; rbx = ((rbx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x1cu64))) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; fl_l = r8; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } rdx = rbx.wrapping_add(0x1); rcx = rdi.wrapping_add(0x10); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P8, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x18u64))) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rdi.wrapping_add(0x10u64)))) as u64; xmm0 = 0u128; ymm0_hi = 0; let y_addr: u64 = rcx; crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); crate::simd_ops::wu128(rcx.wrapping_add(0x20u64), (xmm0) as u128); crate::simd_ops::lk_add_u32(rdi.wrapping_add(0x18u64), (1) as u32); rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x18u64))) as u32 as u64; rdx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rdx = ((rdx).wrapping_shl(4u32)) as u64; fl_l = ((rdx).wrapping_add(crate::simd_ops::ru64(rdi.wrapping_add(0x10u64)))) as u64; fl_r = fl_l; rdx = ((fl_l as u64)) as u64; xmm0 = (crate::simd_ops::ru128(rbp.wrapping_sub(0x30u64))) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rdx.wrapping_sub(0x30u64), (xmm0) as u128); crate::simd_ops::wu32(rdx.wrapping_sub(0xeu64), (0xffffffffu32) as u32); rax = (crate::simd_ops::ru64(rbp.wrapping_sub(0x10u64))) as u64; crate::simd_ops::wu64(rdx.wrapping_sub(0x20u64), rax); crate::simd_ops::wu64(rdx.wrapping_sub(0x18u64), rbx); crate::simd_ops::wu16(r14, ((rbx as u16)) as u16); bb = 2; continue; }
            2 => { r9 = 0x2; r8 = (((rbx as u16)) as u32) as u32 as u64; rdx = ((crate::simd_ops::ru16(rbp.wrapping_sub(0x16u64))) as u32) as u32 as u64; rcx = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P9, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = rbp.wrapping_sub(0x20); rcx = rbp.wrapping_sub(0x50); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P10, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rsi = (crate::simd_ops::ru64(rbp.wrapping_sub(0x20u64))) as u64; rcx = rsi.wrapping_add(0x48); xmm0 = (crate::simd_ops::ru128(P11)) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rbp.wrapping_sub(0x30u64), (xmm0) as u128); rdx = rbp.wrapping_sub(0x30); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r14 = rax; rbx = ((crate::simd_ops::ru16(rax)) as u32) as u32 as u64; fl_l = ((rbx as u16)) as u16 as u64; fl_r = ((r15 as u16)) as u16 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { bb = 4; continue; } rdi = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; xmm0 = (crate::simd_ops::ru128(P11)) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rbp.wrapping_sub(0x30u64), (xmm0) as u128); rcx = (crate::simd_ops::ru32(rdi.wrapping_add(0x18u64))) as u32 as u64; r8 = ((rcx.wrapping_add(rcx.wrapping_mul(2))) as u64) as u64; r8 = ((r8).wrapping_shl(4u32)) as u64; rax = r12; let prod = (rax as i64 as i128).wrapping_mul((r8) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; rbx = rdx; rbx = ((((rbx) as i64).wrapping_shr(3u32) as u64)) as u64; rax = rbx; rax = ((rax).wrapping_shr(63u32)) as u64; rbx = ((rbx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x1cu64))) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; fl_l = r8; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } rdx = rbx.wrapping_add(0x1); rcx = rdi.wrapping_add(0x10); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P8, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x18u64))) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rdi.wrapping_add(0x10u64)))) as u64; xmm0 = 0u128; ymm0_hi = 0; let y_addr: u64 = rcx; crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); crate::simd_ops::wu128(rcx.wrapping_add(0x20u64), (xmm0) as u128); crate::simd_ops::lk_add_u32(rdi.wrapping_add(0x18u64), (1) as u32); rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x18u64))) as u32 as u64; rdx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rdx = ((rdx).wrapping_shl(4u32)) as u64; fl_l = ((rdx).wrapping_add(crate::simd_ops::ru64(rdi.wrapping_add(0x10u64)))) as u64; fl_r = fl_l; rdx = ((fl_l as u64)) as u64; xmm0 = (crate::simd_ops::ru128(rbp.wrapping_sub(0x30u64))) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rdx.wrapping_sub(0x30u64), (xmm0) as u128); crate::simd_ops::wu32(rdx.wrapping_sub(0xeu64), (0xffffffffu32) as u32); rax = (crate::simd_ops::ru64(rbp.wrapping_sub(0x10u64))) as u64; crate::simd_ops::wu64(rdx.wrapping_sub(0x20u64), rax); crate::simd_ops::wu64(rdx.wrapping_sub(0x18u64), rbx); crate::simd_ops::wu16(r14, ((rbx as u16)) as u16); bb = 4; continue; }
            4 => { r9 = 0x2; r8 = (((rbx as u16)) as u32) as u32 as u64; rdx = ((crate::simd_ops::ru16(rbp.wrapping_sub(0x16u64))) as u32) as u32 as u64; rcx = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P9, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = rbp.wrapping_sub(0x20); rcx = rbp.wrapping_sub(0x50); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P12, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r11 = rsp.wrapping_add(0x70); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x40u64))) as u64; rsp = r11; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_35403<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = ((r8 as u32)) as u32 as u64;
    r8 = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64;
    rax = (crate::simd_ops::ru32(rcx)) as u32 as u64;
    crate::simd_ops::wu32(rdx.wrapping_add(r8.wrapping_mul(2)), ((rax as u32)) as u32);
    rax = ((crate::simd_ops::ru16(rcx.wrapping_add(P0))) as u32) as u32 as u64;
    crate::simd_ops::wu16(rdx.wrapping_add(r8.wrapping_mul(2)).wrapping_add(P0), ((rax as u16)) as u16);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_35411<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (0u64, 0u64, 0u64);
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    fl_l = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; fl_r = rax;
    rax = (rax & !0xffu64) | (((if (((fl_l as u64)) == ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64));
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_35414<const P0: u64, const P1: u64, const P2: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    fl_l = rdx; fl_r = rdx;
    if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { return rax; }
    rax = ((rcx.wrapping_add(P0)) as u64) as u64;
    rcx = 0;
    fl_l = 0; fl_r = 0;
    loop { crate::simd_ops::wu64(rax.wrapping_add(P1), rcx); rax = ((rax.wrapping_add(P2)) as u64) as u64; fl_l = rdx; fl_r = (0x1u64) as u64; rdx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if !((((fl_l as u64)) != ((fl_r as u64)))) { break; } }
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_35429<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rcx;
    rax = P0;
    crate::simd_ops::wu64(rcx, rax);
    xmm0 = (crate::simd_ops::ru128(rdx)) as u128;
    crate::simd_ops::wu128(rcx.wrapping_add(P1), (xmm0) as u128);
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    crate::simd_ops::wu64(rbx.wrapping_add(P3), rax);
    crate::simd_ops::wu16(rbx.wrapping_add(P4), (0x0u16) as u16);
    rax = P5;
    crate::simd_ops::wu64(rbx, rax);
    rax = rbx;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_35463<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64, const P9: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 21>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x80u64)) as u64; r14 = rcx; rbx = rcx.wrapping_add(0x8); xmm0 = (crate::simd_ops::ru128(P0)) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128); rcx = rbx.wrapping_add(0x18); r8 = rsp.wrapping_add(0x20); rdx = rsp.wrapping_add(0x30); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rbx); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; xmm0 = (crate::simd_ops::ru128(rax.wrapping_add(0x10u64))) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rsp.wrapping_add(0x48u64), (xmm0) as u128); rdx = rsp.wrapping_add(0x60); rcx = rsp.wrapping_add(0x40); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = (crate::simd_ops::ru64(rax)) as u64; rbp = ((crate::simd_ops::ru16(rax.wrapping_add(0xau64))) as u32) as u32 as u64; xmm0 = (crate::simd_ops::ru128(rax.wrapping_add(0x10u64))) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm0) as u128); rcx = rbx.wrapping_add(0x48); xmm1 = (crate::simd_ops::ru128(P3)) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm1) as u128); rdx = rsp.wrapping_add(0x30); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r15 = rax; rdi = ((crate::simd_ops::ru16(rax)) as u32) as u32 as u64; r12 = 0xffff; fl_l = ((rdi as u16)) as u16 as u64; fl_r = ((r12 as u16)) as u16 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { bb = 2; continue; } rsi = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; xmm0 = (crate::simd_ops::ru128(P3)) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128); rax = (crate::simd_ops::ru32(rsi.wrapping_add(0x18u64))) as u32 as u64; r8 = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; r8 = ((r8).wrapping_shl(4u32)) as u64; rax = (0x2aaaaaaaaaaaaaabu64) as u64; let prod = (rax as i64 as i128).wrapping_mul((r8) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; rdi = rdx; rdi = ((((rdi) as i64).wrapping_shr(3u32) as u64)) as u64; rax = rdi; rax = ((rax).wrapping_shr(63u32)) as u64; rdi = ((rdi).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru32(rsi.wrapping_add(0x1cu64))) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; fl_l = r8; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } rdx = rdi.wrapping_add(0x1); rcx = rsi.wrapping_add(0x10); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru32(rsi.wrapping_add(0x18u64))) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rsi.wrapping_add(0x10u64)))) as u64; xmm0 = 0u128; ymm0_hi = 0; let y_addr: u64 = rcx; crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); crate::simd_ops::wu128(rcx.wrapping_add(0x20u64), (xmm0) as u128); crate::simd_ops::lk_add_u32(rsi.wrapping_add(0x18u64), (1) as u32); rax = (crate::simd_ops::ru32(rsi.wrapping_add(0x18u64))) as u32 as u64; rdx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rdx = ((rdx).wrapping_shl(4u32)) as u64; fl_l = ((rdx).wrapping_add(crate::simd_ops::ru64(rsi.wrapping_add(0x10u64)))) as u64; fl_r = fl_l; rdx = ((fl_l as u64)) as u64; xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rdx.wrapping_sub(0x30u64), (xmm0) as u128); crate::simd_ops::wu32(rdx.wrapping_sub(0xeu64), (0xffffffffu32) as u32); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; crate::simd_ops::wu64(rdx.wrapping_sub(0x20u64), rax); crate::simd_ops::wu64(rdx.wrapping_sub(0x18u64), rdi); crate::simd_ops::wu16(r15, ((rdi as u16)) as u16); bb = 2; continue; }
            2 => { r9 = 0x1; r8 = (((rbp as u16)) as u32) as u32 as u64; rdx = (((rdi as u16)) as u32) as u32 as u64; rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru32(P7)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rax as u32)) as u32); rax = P8; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax); rcx = (crate::simd_ops::ru64(r14)) as u64; rcx = ((rcx).wrapping_add(0xc0u64)) as u64; r8 = rsp.wrapping_add(0x20); rdx = rsp.wrapping_add(0xb0); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P9, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r11 = rsp.wrapping_add(0x80); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x38u64))) as u64; rbp = (crate::simd_ops::ru64(r11.wrapping_add(0x40u64))) as u64; rsp = r11; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_35484<const P0: u64, const P1: u64, const P2: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rax = ((r8 as u32)) as u32 as u64;
    r8 = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64;
    rax = P0;
    crate::simd_ops::wu64(rdx.wrapping_add(r8.wrapping_mul(8)), rax);
    xmm0 = ((crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64) as u128;
    crate::simd_ops::wu64(rdx.wrapping_add(r8.wrapping_mul(8)).wrapping_add(P1), ((xmm0 as u64)) as u64);
    rax = (crate::simd_ops::ru32(rcx.wrapping_add(P2))) as u32 as u64;
    crate::simd_ops::wu32(rdx.wrapping_add(r8.wrapping_mul(8)).wrapping_add(P2), ((rax as u32)) as u32);
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_35502<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    xmm0 = ((crate::simd_ops::ru32(P0)) as u32) as u128;
    xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(crate::simd_ops::ru32(P1))).to_bits() as u128);
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_35505<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut ymm0_hi: u128 = 0; let mut ymm1_hi: u128 = 0;
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let y_addr: u64 = rcx; xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16));
    rax = ((r8 as u32)) as u32 as u64;
    r8 = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64;
    fl_l = ((r8).wrapping_shl(5u32)) as u64;
    r8 = ((fl_l as u64)) as u64;
    let y_addr: u64 = r8.wrapping_add(rdx); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi);
    let y_addr: u64 = rcx.wrapping_add(P0); xmm1 = crate::simd_ops::ru128(y_addr); ymm1_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16));
    let y_addr: u64 = r8.wrapping_add(rdx).wrapping_add(P0); crate::simd_ops::wu128(y_addr, xmm1); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm1_hi);
    let y_addr: u64 = rcx.wrapping_add(P1); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16));
    let y_addr: u64 = r8.wrapping_add(rdx).wrapping_add(P1); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi);
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_35507<const P0: u64, const P1: u64, const P2: u64, const P3: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (0u64, 0u64, 0u64);
    rax = (crate::simd_ops::ru64(P0)) as u64;
    fl_l = (crate::simd_ops::ru64(rdx)) as u64; fl_r = rax;
    if (((fl_l as u64)) != ((fl_r as u64))) { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; return rax; }
    rax = (crate::simd_ops::ru64(P1)) as u64;
    fl_l = (crate::simd_ops::ru64(rdx.wrapping_add(P2))) as u64; fl_r = rax;
    if (((fl_l as u64)) != ((fl_r as u64))) { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; return rax; }
    fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(P3))) as u8 as u64; fl_r = (0x2u8) as u8 as u64;
    if !((((fl_l as u8)) == ((fl_r as u8)))) { rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); return rax; }
    rax = (rax & !0xffu64) | (((0u8) as u8 as u64));
    fl_l = 0; fl_r = 0;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_35527<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 23>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x90u64)) as u64; rbx = rdx; r12 = (crate::simd_ops::ru64(rcx)) as u64; xmm0 = (crate::simd_ops::ru128(rcx.wrapping_add(0x8u64))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128); r8 = rsp.wrapping_add(0x20); rdx = r12.wrapping_add(0x40); rcx = rsp.wrapping_add(0x30); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm1 = (crate::simd_ops::ru128(rsp.wrapping_add(0x30u64))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x58u64), (xmm1) as u128); xmm0 = ((crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64) as u128; crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), ((xmm0 as u64)) as u64); rax = (((xmm1 >> 64) as u64)) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 10; continue; } rax = ((crate::simd_ops::ru16(rax)) as u32) as u32 as u64; r13 = (crate::simd_ops::ru64(r12.wrapping_add(rax.wrapping_mul(8)).wrapping_add(0x88u64))) as u64; fl_l = r13; fl_r = (0x0u64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 1; continue; } rcx = (crate::simd_ops::ru32(r12.wrapping_add(rax.wrapping_mul(4)).wrapping_add(0x10088u64))) as u32 as u64; bb = 2; continue; }
            1 => { rcx = 0; fl_l = 0; fl_r = 0; bb = 2; continue; }
            2 => { r15 = ((rcx as u32)) as u32 as u64; r14 = 0; fl_l = 0; fl_r = 0; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 10; continue; } rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; bb = 3; continue; }
            3 => { rdx = (crate::simd_ops::ru64(r13.wrapping_add(r14.wrapping_mul(8)).wrapping_add(0x10u64))) as u64; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r14 = ((r14).wrapping_add(1)) as u64; fl_l = r14; fl_r = r15; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 3; continue; } rdi = ((rdi).wrapping_add(0x2u64)) as u64; fl_l = rdi; fl_r = rbp; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 9; continue; } bb = 4; continue; }
            4 => { rax = ((crate::simd_ops::ru16(rdi)) as u32) as u32 as u64; fl_l = (crate::simd_ops::ru64(rsi.wrapping_add(rax.wrapping_mul(8)).wrapping_add(0x48u64))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 5; continue; } fl_l = (crate::simd_ops::ru32(rsi.wrapping_add(rax.wrapping_mul(4)).wrapping_add(0x10048u64))) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rdi = ((rdi).wrapping_add(0x2u64)) as u64; fl_l = rdi; fl_r = rbp; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 4; continue; } bb = 9; continue; }
            6 => { fl_l = rdi; fl_r = rbp; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 9; continue; } rax = ((crate::simd_ops::ru16(rdi)) as u32) as u32 as u64; r13 = (crate::simd_ops::ru64(r12.wrapping_add(rax.wrapping_mul(8)).wrapping_add(0x88u64))) as u64; fl_l = r13; fl_r = r13; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } rcx = (crate::simd_ops::ru32(r12.wrapping_add(rax.wrapping_mul(4)).wrapping_add(0x10088u64))) as u32 as u64; bb = 8; continue; }
            7 => { rcx = 0; fl_l = 0; fl_r = 0; bb = 8; continue; }
            8 => { r15 = ((rcx as u32)) as u32 as u64; r14 = 0; fl_l = 0; fl_r = 0; bb = 9; continue; }
            9 => { fl_l = r14; fl_r = r15; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 3; continue; } bb = 10; continue; }
            10 => { r11 = rsp.wrapping_add(0x90); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(r11.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x40u64))) as u64; rsp = r11; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_35536<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
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
    loop { rcx = rbx; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rbx = ((rbx).wrapping_add(0x38u64)) as u64; fl_l = rdi; fl_r = (0x1u64) as u64; rdi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if !((((fl_l as u64)) != ((fl_r as u64)))) { break; } }
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_35537<const P0: u64, const P1: u64, const P2: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
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
    rax = (((rbx) as i64).wrapping_mul((0xe0u64) as i64) as u64) as u64;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rax = ((rax).wrapping_add(crate::simd_ops::ru64(rdi.wrapping_add(P2)))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_35548<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64, const P9: u64, const P10: u64, const P11: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x60u64)) as u64; rdx = ((crate::simd_ops::ru8(P0)) as u32) as u32 as u64; rbx = rcx; fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0x80; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 1; continue; } rdx = ((rdx.wrapping_mul(8)) as u32) as u32 as u64; bb = 2; continue; }
            1 => { rdx = (((rdx as u32)).wrapping_shl(8u32)) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0x7c00; rdx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 2; continue; }
            2 => { rcx = (crate::simd_ops::ru32(P1)) as u32 as u64; rax = P2; rcx = ((((rcx as u32)) & (0xffffffu32))) as u32 as u64; r8 = rbx; rcx = (((rcx as u32)).wrapping_shl(3u32)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; rdx = rsp.wrapping_add(0x20); rcx = rsp.wrapping_add(0x70); rax = (if (((fl_l as u32)) > ((fl_r as u32))) { crate::simd_ops::ru64(P2) } else { rax }) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); rax = (crate::simd_ops::ru32(P3)) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r8 = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; fl_l = ((r8 as u32)) as u32 as u64; fl_r = 0xffffffff; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 5; continue; } rdx = rsp.wrapping_add(0x40); rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; r8 = (crate::simd_ops::ru64(rbx)) as u64; rdx = ((crate::simd_ops::ru16(P6)) as u32) as u32 as u64; xmm0 = ((crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64) as u128; r9 = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = (crate::simd_ops::ru64(r8)) as u64; r9 = ((r9).wrapping_shl(6u32)) as u64; r9 = ((r9).wrapping_add(crate::simd_ops::ru64(rsp.wrapping_add(0x40u64)))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), ((xmm0 as u64)) as u64); rcx = (crate::simd_ops::ru64(rax.wrapping_add(0x20u64))) as u64; rdx = ((crate::simd_ops::ru16(rcx.wrapping_add(rdx.wrapping_mul(2)))) as u32) as u32 as u64; rdx = ((rdx).wrapping_shl(4u32)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), r8); rdx = ((rdx).wrapping_add(crate::simd_ops::ru64(r8.wrapping_add(0x8u64)))) as u64; rcx = (crate::simd_ops::ru64(rdx)) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x8u64))) as u32 as u64; rdx = ((crate::simd_ops::ru8(P7)) as u32) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rcx); fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0x80; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 3; continue; } rdx = ((rdx.wrapping_mul(8)) as u32) as u32 as u64; bb = 4; continue; }
            3 => { rdx = (((rdx as u32)).wrapping_shl(8u32)) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0x7c00; rdx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 4; continue; }
            4 => { rcx = (crate::simd_ops::ru32(P8)) as u32 as u64; rax = P9; rcx = ((((rcx as u32)) & (0xffffffu32))) as u32 as u64; r8 = rsp.wrapping_add(0x40); rcx = (((rcx as u32)).wrapping_shl(3u32)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; rdx = r9; rcx = rsp.wrapping_add(0x20); rax = (if (((fl_l as u32)) > ((fl_r as u32))) { crate::simd_ops::ru64(P9) } else { rax }) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); rax = (crate::simd_ops::ru32(P10)) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P11, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rsp = ((rsp).wrapping_add(0x60u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            5 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; rsp = ((rsp).wrapping_add(0x60u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_35557<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 27>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0xd0u64)) as u64; rdi = rcx; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), (0x0u32) as u32); fl_l = (crate::simd_ops::ru8(P0)) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 1; continue; } rax = (crate::simd_ops::ru64(P1)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax); rax = rsp.wrapping_add(0x30); crate::simd_ops::wu64(rsp.wrapping_add(0xe0u64), rax); rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x30u64))) as u32 as u64; rax = ((((rax as u32)) & (0xff000008u32))) as u32 as u64; rax = ((((rax as u32)) | (0x8u32))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((rax as u32)) as u32); crate::simd_ops::wu8(rsp.wrapping_add(0x33u64), (0x8u8) as u8); crate::simd_ops::wu32(rsp.wrapping_add(0x34u64), (0x0u32) as u32); crate::simd_ops::wu8(rsp.wrapping_add(0x38u64), (0x0u8) as u8); rdx = P2; rcx = rsp.wrapping_add(0x30); rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rax = rsp.wrapping_add(0x28); rbx = 0x1; bb = 2; continue; }
            1 => { rcx = rsp.wrapping_add(0x78); rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rbx = 0x2; bb = 2; continue; }
            2 => { crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rbx as u32)) as u32); rcx = (crate::simd_ops::ru64(rax)) as u64; crate::simd_ops::wu64(rdi, rcx); rcx = ((rdi.wrapping_add(P5)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xe0u64), rcx); crate::simd_ops::lk_and_u32(rcx, (0xff000008u32) as u32); crate::simd_ops::lk_or_u32(rcx, (0x8u32) as u32); crate::simd_ops::wu8(rcx.wrapping_add(P6), (0x8u8) as u8); crate::simd_ops::wu32(rcx.wrapping_add(P7), (0x0u32) as u32); crate::simd_ops::wu8(rcx.wrapping_add(P5), (0x0u8) as u8); rdx = ((rax.wrapping_add(P5)) as u64) as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rbx = ((((rbx as u32)) | (0x4u32))) as u32 as u64; fl_l = ((rbx as u8)) as u8 as u64; fl_r = (0x2u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 3; continue; } rbx = ((((rbx as u32)) & (0xfffffffdu32))) as u32 as u64; rcx = rsp.wrapping_add(0x80); rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); bb = 3; continue; }
            3 => { fl_l = ((rbx as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 4; continue; } rcx = rsp.wrapping_add(0x30); rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); bb = 4; continue; }
            4 => { rax = rdi; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0xe8u64))) as u64; rsp = ((rsp).wrapping_add(0xd0u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_35558<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    crate::simd_ops::wu32(rcx, (0x0u32) as u32);
    rax = rcx;
    crate::simd_ops::wu8(rcx.wrapping_add(P0), (0x0u8) as u8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_35616<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rdx; rdi = ((rcx.wrapping_add(P0)) as u64) as u64; r9 = (crate::simd_ops::ru32(rdi.wrapping_add(P1))) as u32 as u64; rdx = (crate::simd_ops::ru64(rdi)) as u64; rcx = ((rdx.wrapping_add(r9.wrapping_mul(8))) as u64) as u64; rax = rdx; fl_l = rdx; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru64(rax)) as u64; fl_r = rbx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 4; continue; } rax = ((rax).wrapping_add(0x8u64)) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } bb = 2; continue; }
            2 => { r8 = (crate::simd_ops::ru32(rdi.wrapping_add(P2))) as u32 as u64; rax = ((rdx.wrapping_add(r8.wrapping_mul(8))) as u64) as u64; fl_l = rcx; fl_r = rax; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } rax = ((r9.wrapping_add(P3)) as u64) as u64; rdx = ((r8 as u32)) as u32 as u64; rdx = ((rdx).wrapping_shr(1u32)) as u64; rdx = ((rdx).wrapping_add(r8)) as u64; fl_l = rdx; fl_r = rax; rdx = (if (((fl_l as u64)) < ((fl_r as u64))) { rax } else { rdx }) as u64; rcx = rdi; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru32(rdi.wrapping_add(P1))) as u32 as u64; rax = (crate::simd_ops::ru64(rdi)) as u64; crate::simd_ops::wu64(rax.wrapping_add(rcx.wrapping_mul(8)), rbx); fl_l = (crate::simd_ops::lk_add_u32(rdi.wrapping_add(P1), (1) as u32)) as u32 as u64; bb = 4; continue; }
            4 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_35630<const P0: u64, const P1: u64, const P2: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rsp = ((rsp).wrapping_sub(0x48u64)) as u64;
    rax = rdx;
    rdx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64;
    rdx = (crate::simd_ops::ru64(rcx.wrapping_add(rdx.wrapping_mul(8)).wrapping_add(P0))) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdx);
    crate::simd_ops::wu8(rsp.wrapping_add(0x28u64), (0x0u8) as u8);
    xmm0 = ((crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64) as u128;
    crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), ((xmm0 as u64)) as u64);
    rcx = (crate::simd_ops::ru32(rsp.wrapping_add(0x28u64))) as u32 as u64;
    crate::simd_ops::wu32(rsp.wrapping_add(0x38u64), ((rcx as u32)) as u32);
    rdx = rsp.wrapping_add(0x30);
    rcx = rax;
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    rsp = ((rsp).wrapping_add(0x48u64)) as u64;
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_35658<const P0: u64, const P1: u64, const P2: u64, const P3: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rcx;
    crate::simd_ops::wu64(rcx, rdx);
    crate::simd_ops::wu64(rcx.wrapping_add(P0), r8);
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    crate::simd_ops::wu64(rax, rax);
    crate::simd_ops::wu64(rax.wrapping_add(P0), rax);
    crate::simd_ops::wu64(rax.wrapping_add(P2), rax);
    crate::simd_ops::wu16(rax.wrapping_add(P3), (0x101u16) as u16);
    rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P0))) as u64;
    crate::simd_ops::wu64(rdx, rax);
    rax = rbx;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_35661<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x30u64)) as u64;
    rbx = rdx;
    rdi = rcx;
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu64(rcx, rax);
    crate::simd_ops::wu64(rcx.wrapping_add(P0), rax);
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rcx);
    crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rcx);
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    crate::simd_ops::wu64(rax, rax);
    crate::simd_ops::wu64(rax.wrapping_add(P0), rax);
    crate::simd_ops::wu64(rax.wrapping_add(P2), rax);
    crate::simd_ops::wu16(rax.wrapping_add(P3), (0x101u16) as u16);
    crate::simd_ops::wu64(rdi, rax);
    rdx = rbx;
    rcx = rdi;
    rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp);
    rax = rdi;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64;
    rsp = ((rsp).wrapping_add(0x30u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_35665<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = (crate::simd_ops::ru64(rdx)) as u64;
    r8 = (crate::simd_ops::ru64(rcx)) as u64;
    crate::simd_ops::wu64(rcx, rax);
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    crate::simd_ops::wu64(rdx, r8);
    r8 = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    crate::simd_ops::wu64(rcx.wrapping_add(P0), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(P0), r8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_35666<const P0: u64, const P1: u64, const P2: u64, const P3: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rax = 0;
    fl_l = 0; fl_r = 0;
    rbx = rdx;
    crate::simd_ops::wu64(rcx, rax);
    rdi = rcx;
    crate::simd_ops::wu64(rcx.wrapping_add(P0), rax);
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    crate::simd_ops::wu64(rax, rax);
    crate::simd_ops::wu64(rax.wrapping_add(P0), rax);
    crate::simd_ops::wu64(rax.wrapping_add(P2), rax);
    crate::simd_ops::wu16(rax.wrapping_add(P3), (0x101u16) as u16);
    crate::simd_ops::wu64(rdi, rax);
    rdx = (crate::simd_ops::ru64(rbx)) as u64;
    crate::simd_ops::wu64(rdi, rdx);
    crate::simd_ops::wu64(rbx, rax);
    rax = (crate::simd_ops::ru64(rbx.wrapping_add(P0))) as u64;
    rcx = (crate::simd_ops::ru64(rdi.wrapping_add(P0))) as u64;
    crate::simd_ops::wu64(rdi.wrapping_add(P0), rax);
    rax = rdi;
    crate::simd_ops::wu64(rbx.wrapping_add(P0), rcx);
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_35671<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rcx;
    crate::simd_ops::wu64(rcx, rdx);
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu64(rcx.wrapping_add(P0), rax);
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    crate::simd_ops::wu64(rbx.wrapping_add(P0), rax);
    rax = rbx;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_35672(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = (0x1c71c71c71c71c7u64) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_35675<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    crate::simd_ops::wu64(rcx.wrapping_add(P0), (0x0u64) as u64);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_35677<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rdx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(P1))) as u8 as u64; fl_r = (0x0u8) as u8 as u64;
    if !((((fl_l as u8)) != ((fl_r as u8)))) { loop { rax = rdx; rdx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(P1))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if !((((fl_l as u8)) == ((fl_r as u8)))) { break; } } return rax; }
    rax = rcx;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_35678<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rdx = (crate::simd_ops::ru64(rcx)) as u64;
    fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(P0))) as u8 as u64; fl_r = (0x0u8) as u8 as u64;
    if !((((fl_l as u8)) != ((fl_r as u8)))) { loop { rcx = (crate::simd_ops::ru64(rdx)) as u64; rax = rdx; rdx = rcx; fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(P0))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if !((((fl_l as u8)) == ((fl_r as u8)))) { break; } } return rax; }
    rax = rcx;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_35680(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    rcx = (crate::simd_ops::ru64(rax)) as u64;
    rax = rdx;
    crate::simd_ops::wu64(rdx, rcx);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_35682<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r10) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    r10 = rdx;
    crate::simd_ops::wu64(rdx, rax);
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rax = r10;
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_35683(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (0u64, 0u64, 0u64);
    rax = (crate::simd_ops::ru64(rdx)) as u64;
    fl_l = (crate::simd_ops::ru64(rcx)) as u64; fl_r = rax;
    rax = (rax & !0xffu64) | (((if (((fl_l as u64)) == ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64));
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_35685<const P0: u64, const P1: u64, const P2: u64, const P3: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rcx;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    crate::simd_ops::wu64(rax, rax);
    crate::simd_ops::wu64(rax.wrapping_add(P1), rax);
    crate::simd_ops::wu64(rax.wrapping_add(P2), rax);
    crate::simd_ops::wu16(rax.wrapping_add(P3), (0x101u16) as u16);
    crate::simd_ops::wu64(rbx, rax);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_35688<const P0: u64, const P1: u64, const P2: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
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
    rax = ((rax).wrapping_shl(5u32)) as u64;
    rax = ((rax).wrapping_add(crate::simd_ops::ru64(rdi.wrapping_add(P2)))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_35702(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
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
    loop { rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = rbx; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(rax)) as u64, rax, rcx, rdx, r8, r9, rsp); rbx = ((rbx).wrapping_add(0x90u64)) as u64; fl_l = rdi; fl_r = (0x1u64) as u64; rdi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if !((((fl_l as u64)) != ((fl_r as u64)))) { break; } }
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_35718<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    r8 = ((r8 as u32)) as u32 as u64;
    r8 = ((r8).wrapping_shl(5u32)) as u64;
    r8 = ((r8).wrapping_add(rdx)) as u64;
    rdx = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu64(r8, rdx);
    crate::simd_ops::wu64(r8.wrapping_add(P0), rdx);
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    crate::simd_ops::wu64(r8, rax);
    rax = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P0), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rcx.wrapping_add(P1))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P1), ((rax as u32)) as u32);
    crate::simd_ops::wu64(rcx, rdx);
    crate::simd_ops::wu64(rcx.wrapping_add(P0), rdx);
    crate::simd_ops::wu64(r8.wrapping_add(P2), rdx);
    crate::simd_ops::wu64(r8.wrapping_add(P3), rdx);
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P2))) as u64;
    crate::simd_ops::wu64(r8.wrapping_add(P2), rax);
    rax = (crate::simd_ops::ru32(rcx.wrapping_add(P3))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P3), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rcx.wrapping_add(P4))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P4), ((rax as u32)) as u32);
    crate::simd_ops::wu64(rcx.wrapping_add(P2), rdx);
    crate::simd_ops::wu64(rcx.wrapping_add(P3), rdx);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_35719<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
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
    loop { rcx = ((rbx.wrapping_add(P0)) as u64) as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rcx = rbx; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rbx = ((rbx).wrapping_add(0x20u64)) as u64; fl_l = rdi; fl_r = (0x1u64) as u64; rdi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if !((((fl_l as u64)) != ((fl_r as u64)))) { break; } }
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_35748<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    crate::simd_ops::forward_incoming_to_outgoing();
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    r9 = rcx;
    rax = ((r8 as u32)) as u32 as u64;
    rcx = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64;
    rcx = ((rcx).wrapping_shl(5u32)) as u64;
    fl_l = ((rcx).wrapping_add(rdx)) as u64; fl_r = fl_l;
    rcx = ((fl_l as u64)) as u64;
    rdx = r9;
    crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8(P0, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_35751<const P0: u64, const P1: u64, const P2: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut ymm0_hi: u128 = 0; let mut ymm1_hi: u128 = 0;
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let y_addr: u64 = rcx; xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16));
    rax = ((r8 as u32)) as u32 as u64;
    r8 = (((rax) as i64).wrapping_mul((0x70u64) as i64) as u64) as u64;
    let y_addr: u64 = r8.wrapping_add(rdx); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi);
    let y_addr: u64 = rcx.wrapping_add(P0); xmm1 = crate::simd_ops::ru128(y_addr); ymm1_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16));
    let y_addr: u64 = r8.wrapping_add(rdx).wrapping_add(P0); crate::simd_ops::wu128(y_addr, xmm1); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm1_hi);
    let y_addr: u64 = rcx.wrapping_add(P1); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16));
    let y_addr: u64 = r8.wrapping_add(rdx).wrapping_add(P1); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi);
    xmm1 = (crate::simd_ops::ru128(rcx.wrapping_add(P2))) as u128; ymm1_hi = 0;
    crate::simd_ops::wu128(r8.wrapping_add(rdx).wrapping_add(P2), (xmm1) as u128);
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_35758<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; r15 = r9; r12 = r8; rbp = ((rdx as u32)) as u32 as u64; rbx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; r10 = (crate::simd_ops::ru64(rcx)) as u64; rdi = ((rbx.wrapping_add(r10.wrapping_mul(8))) as u64) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 4; continue; } r13 = 0; fl_l = 0; fl_r = 0; bb = 1; continue; }
            1 => { rsi = (crate::simd_ops::ru64(rbx)) as u64; r14 = rsi; r14 = ((r14).wrapping_shr(32u32)) as u64; fl_l = ((rsi as u32)) as u32 as u64; fl_r = ((r14 as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 3; continue; } bb = 2; continue; }
            2 => { rdx = ((rsi as u32)) as u32 as u64; rdx = ((rdx).wrapping_shl(4u32)) as u64; rdx = ((rdx).wrapping_add(r12)) as u64; rcx = ((rbp as u32)) as u32 as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rcx = ((rcx).wrapping_add(r15)) as u64; rbp = (((rbp as u32)).wrapping_add(1)) as u32 as u64; crate::simd_ops::wu64(rcx, r13); crate::simd_ops::wu64(rcx.wrapping_add(P0), r13); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rsi = (((rsi as u32)).wrapping_add(1)) as u32 as u64; fl_l = ((rsi as u32)) as u32 as u64; fl_r = ((r14 as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 2; continue; } bb = 3; continue; }
            3 => { rbx = ((rbx).wrapping_add(0x8u64)) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } bb = 4; continue; }
            4 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_35759<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    r9 = rcx;
    rcx = ((r8 as u32)) as u32 as u64;
    rcx = ((rcx).wrapping_shl(4u32)) as u64;
    rcx = ((rcx).wrapping_add(rdx)) as u64;
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu64(rcx, rax);
    crate::simd_ops::wu64(rcx.wrapping_add(P0), rax);
    rdx = r9;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_35765<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rax = P0;
    crate::simd_ops::wu64(rdx, rax);
    rax = rdx;
    xmm0 = ((crate::simd_ops::ru32(rcx.wrapping_add(P1))) as u32) as u128;
    crate::simd_ops::wu32(rdx.wrapping_add(P1), ((xmm0 as u32)) as u32);
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_35766<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64, const P9: u64, const P10: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rbx = rdx; rdi = rcx; rcx = rbx; rdx = 0x1; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r10 = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 1; continue; } rdx = ((rax.wrapping_add(P1)) as u32) as u32 as u64; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x2; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 3; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(P3))) as u64; rcx = rsp.wrapping_add(0x20); r9 = r10; rdx = (crate::simd_ops::ru64(rax)) as u64; rdx = (crate::simd_ops::ru64(rdx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = (crate::simd_ops::ru8(rsp.wrapping_add(0x38u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 2; continue; } xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; xmm1 = ((crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64) as u128; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rax = rdi; crate::simd_ops::wu128(rdi, (xmm0) as u128); crate::simd_ops::wu64(rdi.wrapping_add(P5), ((xmm1 as u64)) as u64); rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            2 => { r8 = P6; rdx = 0x1; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::lift_ops::trap(0x80000003, P9, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            3 => { rdx = 0x1; rcx = rbx; r8 = ((rdx.wrapping_add(P1)) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P8, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::lift_ops::trap(0x80000003, P10, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_35767<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64, const P9: u64, const P10: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0; let mut ymm1_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 15>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x70u64)) as u64; rbx = rdx; rdi = rcx; rcx = rbx; rdx = 0x1; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r10 = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 1; continue; } rdx = ((rax.wrapping_add(P1)) as u32) as u32 as u64; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x2; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 3; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(P3))) as u64; rcx = rsp.wrapping_add(0x20); r9 = r10; rdx = (crate::simd_ops::ru64(rax)) as u64; rdx = (crate::simd_ops::ru64(rdx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = (crate::simd_ops::ru8(rsp.wrapping_add(0x60u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 2; continue; } let y_addr: u64 = rsp.wrapping_add(0x20u64); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rsp.wrapping_add(0x40u64); xmm1 = crate::simd_ops::ru128(y_addr); ymm1_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rdi; crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); let y_addr: u64 = rdi.wrapping_add(P5); crate::simd_ops::wu128(y_addr, xmm1); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm1_hi); rax = rdi; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; rsp = ((rsp).wrapping_add(0x70u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            2 => { r8 = P6; rdx = 0x1; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::lift_ops::trap(0x80000003, P9, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            3 => { rdx = 0x1; rcx = rbx; r8 = ((rdx.wrapping_add(P1)) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P8, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::lift_ops::trap(0x80000003, P10, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_35768<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64, const P9: u64, const P10: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x50u64)) as u64; rbx = rdx; rdi = rcx; rcx = rbx; rdx = 0x1; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r10 = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 1; continue; } rdx = ((rax.wrapping_add(P1)) as u32) as u32 as u64; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x2; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 3; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(P3))) as u64; rcx = rsp.wrapping_add(0x20); r9 = r10; rdx = (crate::simd_ops::ru64(rax)) as u64; rdx = (crate::simd_ops::ru64(rdx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = (crate::simd_ops::ru8(rsp.wrapping_add(0x48u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 2; continue; } let y_addr: u64 = rsp.wrapping_add(0x20u64); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); xmm1 = ((crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64) as u128; let y_addr: u64 = rdi; crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); crate::simd_ops::wu64(rdi.wrapping_add(P5), ((xmm1 as u64)) as u64); rax = rdi; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rsp = ((rsp).wrapping_add(0x50u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            2 => { r8 = P6; rdx = 0x1; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::lift_ops::trap(0x80000003, P9, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            3 => { rdx = 0x1; rcx = rbx; r8 = ((rdx.wrapping_add(P1)) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P8, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::lift_ops::trap(0x80000003, P10, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_35769<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64, const P9: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x50u64)) as u64; rbx = rdx; rdi = rcx; rcx = rbx; rdx = 0x1; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r10 = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 1; continue; } rdx = ((rax.wrapping_add(P1)) as u32) as u32 as u64; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x2; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 3; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(P3))) as u64; rcx = rsp.wrapping_add(0x20); r9 = r10; rdx = (crate::simd_ops::ru64(rax)) as u64; rdx = (crate::simd_ops::ru64(rdx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = (crate::simd_ops::ru8(rsp.wrapping_add(0x40u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 2; continue; } let y_addr: u64 = rsp.wrapping_add(0x20u64); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rdi; crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); rax = rdi; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rsp = ((rsp).wrapping_add(0x50u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            2 => { r8 = P5; rdx = 0x1; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::lift_ops::trap(0x80000003, P8, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            3 => { rdx = 0x1; rcx = rbx; r8 = ((rdx.wrapping_add(P1)) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::lift_ops::trap(0x80000003, P9, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_35770<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64, const P9: u64, const P10: u64, const P11: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x60u64)) as u64; rdi = rdx; rbx = rcx; rcx = rdi; rdx = 0x1; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r10 = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 1; continue; } rdx = ((rax.wrapping_add(P1)) as u32) as u32 as u64; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x2; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 3; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(P3))) as u64; rcx = rsp.wrapping_add(0x20); r9 = r10; rdx = (crate::simd_ops::ru64(rax)) as u64; rdx = (crate::simd_ops::ru64(rdx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = (crate::simd_ops::ru8(rsp.wrapping_add(0x58u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 2; continue; } let y_addr: u64 = rsp.wrapping_add(0x20u64); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); xmm1 = (crate::simd_ops::ru128(rsp.wrapping_add(0x40u64))) as u128; let y_addr: u64 = rbx; crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); xmm0 = ((crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rbx.wrapping_add(P5), (xmm1) as u128); crate::simd_ops::wu64(rbx.wrapping_add(P6), ((xmm0 as u64)) as u64); rax = rbx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rsp = ((rsp).wrapping_add(0x60u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            2 => { r8 = P7; rdx = 0x1; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P8, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::lift_ops::trap(0x80000003, P10, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            3 => { rdx = 0x1; rcx = rdi; r8 = ((rdx.wrapping_add(P1)) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P9, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::lift_ops::trap(0x80000003, P11, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_35777<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64, const P9: u64, const P10: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x60u64)) as u64; rbx = rdx; rdi = rcx; rcx = rbx; rdx = 0x1; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r10 = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 1; continue; } rdx = ((rax.wrapping_add(P1)) as u32) as u32 as u64; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x2; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 3; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(P3))) as u64; rcx = rsp.wrapping_add(0x20); r9 = r10; rdx = (crate::simd_ops::ru64(rax)) as u64; rdx = (crate::simd_ops::ru64(rdx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = (crate::simd_ops::ru8(rsp.wrapping_add(0x50u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 2; continue; } let y_addr: u64 = rsp.wrapping_add(0x20u64); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); xmm1 = (crate::simd_ops::ru128(rsp.wrapping_add(0x40u64))) as u128; let y_addr: u64 = rdi; crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); crate::simd_ops::wu128(rdi.wrapping_add(P5), (xmm1) as u128); rax = rdi; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rsp = ((rsp).wrapping_add(0x60u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            2 => { r8 = P6; rdx = 0x1; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::lift_ops::trap(0x80000003, P9, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            3 => { rdx = 0x1; rcx = rbx; r8 = ((rdx.wrapping_add(P1)) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P8, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::lift_ops::trap(0x80000003, P10, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_35782<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; r10 = rdx; rbx = rcx; rcx = r10; rdx = 0x1; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); r11 = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 1; continue; } rdx = ((rax.wrapping_add(P1)) as u32) as u32 as u64; rcx = r10; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x2; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(r10.wrapping_add(P3))) as u64; r9 = r11; rcx = rbx; rdx = (crate::simd_ops::ru64(rax)) as u64; rdx = (crate::simd_ops::ru64(rdx)) as u64; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rax = rbx; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            2 => { rdx = 0x1; rcx = r10; r8 = ((rdx.wrapping_add(P1)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, P6, rcx, rsp); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_35786<const P0: u64, const P1: u64, const P2: u64, const P3: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    r8 = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    rdx = 0;
    fl_l = 0; fl_r = 0;
    rax = (crate::simd_ops::ru64(P1)) as u64;
    fl_l = (crate::simd_ops::ru8(rax.wrapping_add(P2))) as u8 as u64; fl_r = ((rdx as u8)) as u8 as u64;
    rax = 0x1;
    crate::simd_ops::wu32(r8.wrapping_add(P3), (0x1u32) as u32);
    rdx = (rdx & !0xffu64) | (((if (((fl_l as u8)) != ((fl_r as u8))) { 1u8 } else { 0u8 }) as u8 as u64));
    crate::simd_ops::wu32(r8, ((rdx as u32)) as u32);
    fl_l = (crate::simd_ops::lk_add_u64(rcx.wrapping_add(P0), (0x18u64) as u64)) as u64; fl_r = fl_l;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_35794<const P0: u64, const P1: u64, const P2: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    rcx = (crate::simd_ops::ru64(P1)) as u64;
    rdx = (crate::simd_ops::ru64(rax)) as u64;
    rdx = (crate::simd_ops::ru64(rdx)) as u64;
    rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp);
    rax = 0;
    fl_l = 0; fl_r = 0;
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_35812<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64, const P9: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut xmm6: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm6) as u128); r8 = P0; xmm6 = ((crate::simd_ops::ru32(P1)) as u32) as u128; rdi = rdx; rbx = rcx; xmm3 = (xmm6) as u128; rdx = 0xffffffff; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm3 = (xmm6) as u128; r8 = P3; rdx = 0xffffffff; rcx = rbx; crate::simd_ops::wu32(rdi.wrapping_add(P4), ((xmm0 as u32)) as u32); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm3 = (xmm6) as u128; r8 = P5; rdx = 0xffffffff; rcx = rbx; crate::simd_ops::wu32(rdi.wrapping_add(P6), ((xmm0 as u32)) as u32); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm3 = (xmm6) as u128; r8 = P7; rdx = 0xffffffff; rcx = rbx; crate::simd_ops::wu32(rdi.wrapping_add(P8), ((xmm0 as u32)) as u32); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; crate::simd_ops::wu32(rdi.wrapping_add(P9), ((xmm0 as u32)) as u32); rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_35827<const P0: u64, const P1: u64, const P2: u64, const P3: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 25>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rbp = rsp.wrapping_sub(0x37); rsp = ((rsp).wrapping_sub(0xa0u64)) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x78u64))) as u64; rsi = r8; r9 = ((crate::simd_ops::ru16(P0)) as u32) as u32 as u64; r8 = rbp.wrapping_sub(0x49); r14 = rdx; rcx = (crate::simd_ops::ru64(rax)) as u64; r11 = (crate::simd_ops::ru64(rcx)) as u64; rax = (crate::simd_ops::ru64(r11)) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(0x20u64))) as u64; rcx = ((crate::simd_ops::ru16(rcx.wrapping_add(r9.wrapping_mul(2)))) as u32) as u32 as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(r11.wrapping_add(0x8u64)))) as u64; r11 = ((r11).wrapping_add(0x40u64)) as u64; rdx = r11; rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x8u64))) as u32 as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x31u64), rax); rax = (crate::simd_ops::ru64(rcx)) as u64; rcx = rbp.wrapping_sub(0x39); crate::simd_ops::wu64(rbp.wrapping_sub(0x29u64), rax); xmm0 = (crate::simd_ops::ru128(rbp.wrapping_sub(0x31u64))) as u128; crate::simd_ops::wu128(rbp.wrapping_sub(0x49u64), (xmm0) as u128); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm1 = (crate::simd_ops::ru128(rbp.wrapping_sub(0x39u64))) as u128; xmm0 = ((crate::simd_ops::ru64(rbp.wrapping_sub(0x29u64))) as u64) as u128; r15 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rbp.wrapping_sub(0x19u64), r11); rax = (((xmm1 >> 64) as u64)) as u64; crate::simd_ops::wu128(rbp.wrapping_sub(0x11u64), (xmm1) as u128); crate::simd_ops::wu64(rbp.wrapping_sub(0x1u64), ((xmm0 as u64)) as u64); crate::simd_ops::wu64(rbp.wrapping_add(0xfu64), r15); crate::simd_ops::wu64(rbp.wrapping_add(0x17u64), r15); fl_l = rax; fl_r = (crate::simd_ops::ru64(rbp.wrapping_sub(0x29u64))) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 3; continue; } rcx = ((crate::simd_ops::ru16(rax)) as u32) as u32 as u64; rax = (crate::simd_ops::ru64(r11.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x48u64))) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x7u64), rax); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 1; continue; } rax = (crate::simd_ops::ru32(r11.wrapping_add(rcx.wrapping_mul(4)).wrapping_add(0x10048u64))) as u32 as u64; bb = 2; continue; }
            1 => { rax = ((r15 as u32)) as u32 as u64; bb = 2; continue; }
            2 => { crate::simd_ops::wu64(rbp.wrapping_add(0xfu64), rax); bb = 3; continue; }
            3 => { rcx = rbp.wrapping_sub(0x19); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = (crate::simd_ops::ru64(rbp.wrapping_add(0x17u64))) as u64; rdi = (crate::simd_ops::ru64(rbp.wrapping_add(0xfu64))) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 13; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x1fu64))) as u64; rcx = (((rbx) as i64).wrapping_mul((0x118u64) as i64) as u64) as u64; rcx = (crate::simd_ops::ru32(rcx.wrapping_add(rax).wrapping_add(0x18u64))) as u32 as u64; fl_l = (crate::simd_ops::ru32(r14)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 6; continue; } rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x27u64))) as u64; rcx = (crate::simd_ops::ru32(rsi.wrapping_add(0x8u64))) as u32 as u64; r8 = ((rax.wrapping_add(rbx.wrapping_mul(8))) as u64) as u64; rax = (crate::simd_ops::ru64(rsi)) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x67u64), r8); rdx = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; rax = (crate::simd_ops::ru32(rsi.wrapping_add(0xcu64))) as u32 as u64; rax = ((((rax as u32)) & (0x3fffffffu32))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 5; continue; } crate::simd_ops::wu64(rbp.wrapping_add(0x7fu64), rdx); r8 = rbp.wrapping_add(0x67); rdx = rbp.wrapping_add(0x7f); rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 6; continue; }
            5 => { rax = ((rcx.wrapping_add(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu32(rsi.wrapping_add(0x8u64), ((rax as u32)) as u32); crate::simd_ops::wu64(rdx, r8); bb = 6; continue; }
            6 => { rbx = ((rbx).wrapping_add(1)) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x17u64), rbx); fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 4; continue; } rcx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x9u64))) as u64; r8 = (crate::simd_ops::ru64(rbp.wrapping_sub(0x1u64))) as u64; rcx = ((rcx).wrapping_add(0x2u64)) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x9u64), rcx); fl_l = rcx; fl_r = r8; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 12; continue; } rdx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x11u64))) as u64; bb = 7; continue; }
            7 => { rax = ((crate::simd_ops::ru16(rcx)) as u32) as u32 as u64; fl_l = (crate::simd_ops::ru64(rdx.wrapping_add(rax.wrapping_mul(8)).wrapping_add(0x48u64))) as u64; fl_r = r15; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 8; continue; } fl_l = (crate::simd_ops::ru32(rdx.wrapping_add(rax.wrapping_mul(4)).wrapping_add(0x10048u64))) as u32 as u64; fl_r = ((r15 as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 9; continue; } bb = 8; continue; }
            8 => { rcx = ((rcx).wrapping_add(0x2u64)) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x9u64), rcx); fl_l = rcx; fl_r = r8; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 7; continue; } bb = 12; continue; }
            9 => { fl_l = rcx; fl_r = r8; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 12; continue; } rdx = ((crate::simd_ops::ru16(rcx)) as u32) as u32 as u64; rcx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x19u64))) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(rdx.wrapping_mul(8)).wrapping_add(0x48u64))) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x7u64), rax); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } rax = (crate::simd_ops::ru32(rcx.wrapping_add(rdx.wrapping_mul(4)).wrapping_add(0x10048u64))) as u32 as u64; bb = 11; continue; }
            10 => { rax = ((r15 as u32)) as u32 as u64; bb = 11; continue; }
            11 => { crate::simd_ops::wu64(rbp.wrapping_add(0xfu64), rax); crate::simd_ops::wu64(rbp.wrapping_add(0x17u64), r15); bb = 12; continue; }
            12 => { rcx = rbp.wrapping_sub(0x19); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = (crate::simd_ops::ru64(rbp.wrapping_add(0x17u64))) as u64; rdi = (crate::simd_ops::ru64(rbp.wrapping_add(0xfu64))) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 4; continue; } bb = 13; continue; }
            13 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0xd8u64))) as u64; rsp = ((rsp).wrapping_add(0xa0u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_35834<const P0: u64, const P1: u64, const P2: u64, const P3: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 25>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rbp = rsp.wrapping_sub(0x37); rsp = ((rsp).wrapping_sub(0xa0u64)) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x78u64))) as u64; rsi = r8; r9 = ((crate::simd_ops::ru16(P0)) as u32) as u32 as u64; r8 = rbp.wrapping_sub(0x49); r14 = rdx; rcx = (crate::simd_ops::ru64(rax)) as u64; r11 = (crate::simd_ops::ru64(rcx)) as u64; rax = (crate::simd_ops::ru64(r11)) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(0x20u64))) as u64; rcx = ((crate::simd_ops::ru16(rcx.wrapping_add(r9.wrapping_mul(2)))) as u32) as u32 as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(r11.wrapping_add(0x8u64)))) as u64; r11 = ((r11).wrapping_add(0x40u64)) as u64; rdx = r11; rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x8u64))) as u32 as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x31u64), rax); rax = (crate::simd_ops::ru64(rcx)) as u64; rcx = rbp.wrapping_sub(0x39); crate::simd_ops::wu64(rbp.wrapping_sub(0x29u64), rax); xmm0 = (crate::simd_ops::ru128(rbp.wrapping_sub(0x31u64))) as u128; crate::simd_ops::wu128(rbp.wrapping_sub(0x49u64), (xmm0) as u128); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm1 = (crate::simd_ops::ru128(rbp.wrapping_sub(0x39u64))) as u128; xmm0 = ((crate::simd_ops::ru64(rbp.wrapping_sub(0x29u64))) as u64) as u128; r15 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rbp.wrapping_sub(0x19u64), r11); rax = (((xmm1 >> 64) as u64)) as u64; crate::simd_ops::wu128(rbp.wrapping_sub(0x11u64), (xmm1) as u128); crate::simd_ops::wu64(rbp.wrapping_sub(0x1u64), ((xmm0 as u64)) as u64); crate::simd_ops::wu64(rbp.wrapping_add(0xfu64), r15); crate::simd_ops::wu64(rbp.wrapping_add(0x17u64), r15); fl_l = rax; fl_r = (crate::simd_ops::ru64(rbp.wrapping_sub(0x29u64))) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 3; continue; } rcx = ((crate::simd_ops::ru16(rax)) as u32) as u32 as u64; rax = (crate::simd_ops::ru64(r11.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x48u64))) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x7u64), rax); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 1; continue; } rax = (crate::simd_ops::ru32(r11.wrapping_add(rcx.wrapping_mul(4)).wrapping_add(0x10048u64))) as u32 as u64; bb = 2; continue; }
            1 => { rax = ((r15 as u32)) as u32 as u64; bb = 2; continue; }
            2 => { crate::simd_ops::wu64(rbp.wrapping_add(0xfu64), rax); bb = 3; continue; }
            3 => { rcx = rbp.wrapping_sub(0x19); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = (crate::simd_ops::ru64(rbp.wrapping_add(0x17u64))) as u64; rdi = (crate::simd_ops::ru64(rbp.wrapping_add(0xfu64))) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 13; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x1fu64))) as u64; rcx = (((rbx) as i64).wrapping_mul((0x54u64) as i64) as u64) as u64; rcx = (crate::simd_ops::ru32(rcx.wrapping_add(rax).wrapping_add(0x50u64))) as u32 as u64; fl_l = (crate::simd_ops::ru32(r14)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 6; continue; } rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x27u64))) as u64; rcx = (crate::simd_ops::ru32(rsi.wrapping_add(0x8u64))) as u32 as u64; r8 = ((rax.wrapping_add(rbx.wrapping_mul(8))) as u64) as u64; rax = (crate::simd_ops::ru64(rsi)) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x67u64), r8); rdx = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; rax = (crate::simd_ops::ru32(rsi.wrapping_add(0xcu64))) as u32 as u64; rax = ((((rax as u32)) & (0x3fffffffu32))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 5; continue; } crate::simd_ops::wu64(rbp.wrapping_add(0x7fu64), rdx); r8 = rbp.wrapping_add(0x67); rdx = rbp.wrapping_add(0x7f); rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 6; continue; }
            5 => { rax = ((rcx.wrapping_add(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu32(rsi.wrapping_add(0x8u64), ((rax as u32)) as u32); crate::simd_ops::wu64(rdx, r8); bb = 6; continue; }
            6 => { rbx = ((rbx).wrapping_add(1)) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x17u64), rbx); fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 4; continue; } rcx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x9u64))) as u64; r8 = (crate::simd_ops::ru64(rbp.wrapping_sub(0x1u64))) as u64; rcx = ((rcx).wrapping_add(0x2u64)) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x9u64), rcx); fl_l = rcx; fl_r = r8; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 12; continue; } rdx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x11u64))) as u64; bb = 7; continue; }
            7 => { rax = ((crate::simd_ops::ru16(rcx)) as u32) as u32 as u64; fl_l = (crate::simd_ops::ru64(rdx.wrapping_add(rax.wrapping_mul(8)).wrapping_add(0x48u64))) as u64; fl_r = r15; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 8; continue; } fl_l = (crate::simd_ops::ru32(rdx.wrapping_add(rax.wrapping_mul(4)).wrapping_add(0x10048u64))) as u32 as u64; fl_r = ((r15 as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 9; continue; } bb = 8; continue; }
            8 => { rcx = ((rcx).wrapping_add(0x2u64)) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x9u64), rcx); fl_l = rcx; fl_r = r8; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 7; continue; } bb = 12; continue; }
            9 => { fl_l = rcx; fl_r = r8; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 12; continue; } rdx = ((crate::simd_ops::ru16(rcx)) as u32) as u32 as u64; rcx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x19u64))) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(rdx.wrapping_mul(8)).wrapping_add(0x48u64))) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x7u64), rax); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } rax = (crate::simd_ops::ru32(rcx.wrapping_add(rdx.wrapping_mul(4)).wrapping_add(0x10048u64))) as u32 as u64; bb = 11; continue; }
            10 => { rax = ((r15 as u32)) as u32 as u64; bb = 11; continue; }
            11 => { crate::simd_ops::wu64(rbp.wrapping_add(0xfu64), rax); crate::simd_ops::wu64(rbp.wrapping_add(0x17u64), r15); bb = 12; continue; }
            12 => { rcx = rbp.wrapping_sub(0x19); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = (crate::simd_ops::ru64(rbp.wrapping_add(0x17u64))) as u64; rdi = (crate::simd_ops::ru64(rbp.wrapping_add(0xfu64))) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 4; continue; } bb = 13; continue; }
            13 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0xd8u64))) as u64; rsp = ((rsp).wrapping_add(0xa0u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_35839<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rdx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    rax = 0x1;
    crate::simd_ops::wu32(rdx, (0x1u32) as u32);
    crate::simd_ops::wu32(rdx.wrapping_add(P1), (0x1u32) as u32);
    fl_l = (crate::simd_ops::lk_add_u64(rcx.wrapping_add(P0), (0x18u64) as u64)) as u64; fl_r = fl_l;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_35860(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64;
    if !((((((fl_l as u32)) & ((fl_r as u32))) as i32) >= 0)) { rax = 0; fl_l = 0; fl_r = 0; return rax; }
    rax = 0x8;
    fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64;
    rdx = (if ((((fl_l as u32)) as i32) > (((fl_r as u32)) as i32)) { (rax as u32) } else { (rdx as u32) }) as u32 as u64;
    rax = ((rdx as u32)) as u32 as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_35861(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64;
    if !((((((fl_l as u32)) & ((fl_r as u32))) as i32) >= 0)) { rax = 0; fl_l = 0; fl_r = 0; return rax; }
    rax = 0x9;
    fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64;
    rdx = (if ((((fl_l as u32)) as i32) > (((fl_r as u32)) as i32)) { (rax as u32) } else { (rdx as u32) }) as u32 as u64;
    rax = ((rdx as u32)) as u32 as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_35862(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (0u64, 0u64, 0u64);
    rax = (crate::simd_ops::ru32(r8)) as u32 as u64;
    fl_l = (crate::simd_ops::ru32(rdx)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64;
    rax = (rax & !0xffu64) | (((if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { 1u8 } else { 0u8 }) as u8 as u64));
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_35874<const P0: u64, const P1: u64, const P2: u64, const P3: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rcx;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    r8 = (crate::simd_ops::ru64(rbx.wrapping_add(P1))) as u64;
    rdx = 0;
    fl_l = 0; fl_r = 0;
    fl_l = (crate::simd_ops::ru8(rax.wrapping_add(P2))) as u8 as u64; fl_r = ((rdx as u8)) as u8 as u64;
    rax = 0x1;
    crate::simd_ops::wu32(r8.wrapping_add(P3), (0x1u32) as u32);
    rdx = (rdx & !0xffu64) | (((if (((fl_l as u8)) != ((fl_r as u8))) { 1u8 } else { 0u8 }) as u8 as u64));
    crate::simd_ops::wu32(r8, ((rdx as u32)) as u32);
    crate::simd_ops::lk_add_u64(rbx.wrapping_add(P1), (0x18u64) as u64);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_35880<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64, const P9: u64, const P10: u64, const P11: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x28u64)) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rdx = (crate::simd_ops::ru64(rax)) as u64; rax = (crate::simd_ops::ru64(rdx)) as u64; rdx = ((crate::simd_ops::ru16(P1)) as u32) as u32 as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64; r8 = (crate::simd_ops::ru64(rax.wrapping_add(rdx.wrapping_mul(8)))) as u64; fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } rax = (crate::simd_ops::ru64(rcx.wrapping_add(P3))) as u64; rdx = (crate::simd_ops::ru64(rcx.wrapping_add(P4))) as u64; fl_l = rax; fl_r = rdx; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 3; continue; } r9 = P5; fl_l = rax; fl_r = r9; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 3; continue; } r9 = (crate::simd_ops::ru32(rax.wrapping_add(P3))) as u32 as u64; fl_l = ((r9 as u32)) as u32 as u64; fl_r = 0xffffffff; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 3; continue; } fl_l = ((r9 as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 4; continue; } fl_l = ((r9 as u32)) as u32 as u64; fl_r = ((r9 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 2; continue; } fl_l = ((r9 as u32)) as u32 as u64; fl_r = ((r9 as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 1; continue; } fl_l = (crate::simd_ops::ru32(rax)) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); crate::simd_ops::wu8(r8.wrapping_add(P6), ((rax as u8)) as u8); rax = 0; fl_l = 0; fl_r = 0; rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
            2 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; crate::simd_ops::wu8(r8.wrapping_add(P6), ((rax as u8)) as u8); rax = 0; fl_l = 0; fl_r = 0; rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
            3 => { rax = 0; fl_l = 0; fl_r = 0; fl_l = (crate::simd_ops::ru8(r8.wrapping_add(P6))) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; crate::simd_ops::wu32(rdx.wrapping_add(P3), (0x1u32) as u32); rax = (rax & !0xffu64) | (((if (((fl_l as u8)) != ((fl_r as u8))) { 1u8 } else { 0u8 }) as u8 as u64)); crate::simd_ops::wu32(rdx, ((rax as u32)) as u32); rax = 0x1; crate::simd_ops::lk_add_u64(rcx.wrapping_add(P4), (0x18u64) as u64); rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
            4 => { rdx = 0x1; r8 = ((rdx as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, P10, rcx, rsp); return rax; }
            5 => { rdx = P8; rax = crate::simd_ops::call_stk(P9, rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, P11, rcx, rsp); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_35890<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x28u64)) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; r8 = rcx; rdx = (crate::simd_ops::ru64(rax)) as u64; rax = (crate::simd_ops::ru64(rdx)) as u64; rdx = ((crate::simd_ops::ru16(P1)) as u32) as u32 as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(rdx.wrapping_mul(8)))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 1; continue; } rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rax = 0; fl_l = 0; fl_r = 0; rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
            1 => { rdx = P4; rcx = r8; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, P6, rcx, rsp); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_35893<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64, const P9: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rbx = rcx; rdx = (crate::simd_ops::ru64(rax)) as u64; rax = (crate::simd_ops::ru64(rdx)) as u64; rdx = ((crate::simd_ops::ru16(P1)) as u32) as u32 as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64; r8 = (crate::simd_ops::ru64(rax.wrapping_add(rdx.wrapping_mul(8)))) as u64; fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 1; continue; } rdx = 0x1; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P4))) as u64; rcx = 0; fl_l = 0; fl_r = 0; fl_l = (crate::simd_ops::ru8(rax.wrapping_add(P5))) as u8 as u64; fl_r = ((rcx as u8)) as u8 as u64; rax = 0x1; crate::simd_ops::wu32(rdx.wrapping_add(P6), (0x1u32) as u32); rcx = (rcx & !0xffu64) | (((if (((fl_l as u8)) != ((fl_r as u8))) { 1u8 } else { 0u8 }) as u8 as u64)); crate::simd_ops::wu32(rdx, ((rcx as u32)) as u32); crate::simd_ops::lk_add_u64(rbx.wrapping_add(P4), (0x18u64) as u64); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            1 => { rdx = P7; rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, P9, rcx, rsp); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_35899<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64, const P9: u64, const P10: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x28u64)) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; r8 = ((crate::simd_ops::ru16(P1)) as u32) as u32 as u64; rdx = (crate::simd_ops::ru64(rax)) as u64; rax = (crate::simd_ops::ru64(rdx)) as u64; rdx = (crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64; rax = (crate::simd_ops::ru64(rdx.wrapping_add(r8.wrapping_mul(8)))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 1; continue; } xmm2 = (crate::simd_ops::ru128(rax.wrapping_add(P3))) as u128; rax = (crate::simd_ops::ru64(rcx.wrapping_add(P4))) as u64; crate::simd_ops::wu32(rax, ((xmm2 as u32)) as u32); crate::simd_ops::wu32(rax.wrapping_add(P5), (((xmm2 >> 32) as u32) as u32) as u32); crate::simd_ops::wu32(rax.wrapping_add(P4), (((xmm2 >> 64) as u32) as u32) as u32); crate::simd_ops::wu32(rax.wrapping_add(P6), (((xmm2 >> 96) as u32) as u32) as u32); crate::simd_ops::wu32(rax.wrapping_add(P7), (0x4u32) as u32); rax = 0x1; crate::simd_ops::lk_add_u64(rcx.wrapping_add(P4), (0x18u64) as u64); rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
            1 => { rdx = P8; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P9, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::lift_ops::trap(0x80000003, P10, rcx, rsp); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_35913<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rcx; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P1))) as u64; r9 = rax; rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P2))) as u64; rdx = ((rdx).wrapping_add(0x18u64)) as u64; fl_l = rdx; fl_r = rcx; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 3; continue; } rax = P3; fl_l = rdx; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 3; continue; } r8 = (crate::simd_ops::ru32(rdx.wrapping_add(P1))) as u32 as u64; fl_l = ((r8 as u32)) as u32 as u64; fl_r = 0xffffffff; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 3; continue; } fl_l = ((r8 as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 4; continue; } rax = ((r8 as u32)) as u32 as u64; fl_l = ((r8 as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 2; continue; } fl_l = ((rax as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 1; continue; } fl_l = (crate::simd_ops::ru32(rdx)) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); crate::simd_ops::wu8(r9.wrapping_add(P4), ((rax as u8)) as u8); rax = 0; fl_l = 0; fl_r = 0; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            2 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; crate::simd_ops::wu8(r9.wrapping_add(P4), ((rax as u8)) as u8); rax = 0; fl_l = 0; fl_r = 0; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            3 => { rax = 0; fl_l = 0; fl_r = 0; fl_l = (crate::simd_ops::ru8(r9.wrapping_add(P4))) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; crate::simd_ops::wu32(rcx.wrapping_add(P1), (0x1u32) as u32); rax = (rax & !0xffu64) | (((if (((fl_l as u8)) != ((fl_r as u8))) { 1u8 } else { 0u8 }) as u8 as u64)); crate::simd_ops::wu32(rcx, ((rax as u32)) as u32); rax = 0x1; crate::simd_ops::lk_add_u64(rbx.wrapping_add(P2), (0x18u64) as u64); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            4 => { rdx = 0x2; rcx = rbx; r8 = ((rdx.wrapping_add(P5)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, P7, rcx, rsp); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_35915<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64, const P9: u64, const P10: u64, const P11: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rcx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P1))) as u64; r8 = rax; rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P2))) as u64; rdx = ((rdx).wrapping_add(0x18u64)) as u64; fl_l = rdx; fl_r = rcx; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 1; continue; } rax = P3; fl_l = rdx; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 1; continue; } fl_l = (crate::simd_ops::ru32(rdx.wrapping_add(P1))) as u32 as u64; fl_r = 0xffffffff; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 1; continue; } rax = 0; fl_l = 0; fl_r = 0; fl_l = (crate::simd_ops::ru32(rdx.wrapping_add(P1))) as u32 as u64; fl_r = 0x4; rax = (if (((fl_l as u32)) == ((fl_r as u32))) { rdx } else { rax }) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } xmm1 = ((crate::simd_ops::ru32(rax)) as u32) as u128; xmm0 = ((crate::simd_ops::ru32(rax.wrapping_add(P4))) as u32) as u128; xmm3 = ((crate::simd_ops::ru32(rax.wrapping_add(P2))) as u32) as u128; xmm0 = crate::simd_ops::vunpcklps(xmm1, xmm0); crate::simd_ops::wu64(r8.wrapping_add(P5), ((xmm0 as u64)) as u64); crate::simd_ops::wu32(r8.wrapping_add(P6), ((xmm3 as u32)) as u32); rax = 0; fl_l = 0; fl_r = 0; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            1 => { rax = (crate::simd_ops::ru32(r8.wrapping_add(P5))) as u32 as u64; xmm1 = ((crate::simd_ops::ru32(r8.wrapping_add(P6))) as u32) as u128; xmm0 = ((crate::simd_ops::ru32(r8.wrapping_add(P7))) as u32) as u128; crate::simd_ops::wu32(rcx, ((rax as u32)) as u32); rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rcx.wrapping_add(P8), ((rax as u32)) as u32); rax = 0x1; crate::simd_ops::wu32(rcx.wrapping_add(P4), ((xmm0 as u32)) as u32); crate::simd_ops::wu32(rcx.wrapping_add(P2), ((xmm1 as u32)) as u32); crate::simd_ops::wu32(rcx.wrapping_add(P1), (0x4u32) as u32); crate::simd_ops::lk_add_u64(rbx.wrapping_add(P2), (0x18u64) as u64); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            2 => { rdx = 0x2; rcx = rbx; r8 = ((rdx.wrapping_add(P9)) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P10, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::lift_ops::trap(0x80000003, P11, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_35925<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x28u64)) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; r8 = rcx; rdx = (crate::simd_ops::ru64(rax)) as u64; rax = (crate::simd_ops::ru64(rdx)) as u64; rdx = ((crate::simd_ops::ru16(P1)) as u32) as u32 as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(rdx.wrapping_mul(8)))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } rcx = 0x3fff; fl_l = (crate::simd_ops::ru16(rax.wrapping_add(P3))) as u16 as u64; fl_r = ((rcx as u16)) as u16 as u64; if ((((fl_l as u16)) & ((fl_r as u16))) == 0) { bb = 1; continue; } rcx = (0x3fffffffffffc000u64) as u64; fl_l = (crate::simd_ops::ru64(rax.wrapping_add(P3))) as u64; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 1; continue; } rax = (crate::simd_ops::ru64(rax.wrapping_add(P3))) as u64; rcx = (crate::simd_ops::ru64(r8.wrapping_add(P4))) as u64; crate::simd_ops::wu64(rcx, rax); rax = 0x1; crate::simd_ops::wu32(rcx.wrapping_add(P5), (0x2u32) as u32); crate::simd_ops::lk_add_u64(r8.wrapping_add(P4), (0x18u64) as u64); rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
            1 => { rax = (crate::simd_ops::ru64(r8.wrapping_add(P4))) as u64; crate::simd_ops::wu32(rax.wrapping_add(P5), (0x0u32) as u32); rax = 0x1; crate::simd_ops::lk_add_u64(r8.wrapping_add(P4), (0x18u64) as u64); rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
            2 => { rdx = P6; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, P8, rcx, rsp); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_35926<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x28u64)) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; r8 = rcx; rdx = (crate::simd_ops::ru64(rax)) as u64; rax = (crate::simd_ops::ru64(rdx)) as u64; rdx = ((crate::simd_ops::ru16(P1)) as u32) as u32 as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64; r9 = (crate::simd_ops::ru64(rax.wrapping_add(rdx.wrapping_mul(8)))) as u64; fl_l = r9; fl_r = r9; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 1; continue; } rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P3))) as u64; rax = 0; fl_l = 0; fl_r = 0; fl_l = (crate::simd_ops::ru8(r9.wrapping_add(P4))) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; rax = (rax & !0xffu64) | (((if (((fl_l as u8)) != ((fl_r as u8))) { 1u8 } else { 0u8 }) as u8 as u64)); crate::simd_ops::wu32(rcx, ((rax as u32)) as u32); rax = 0x1; crate::simd_ops::wu32(rcx.wrapping_add(P5), (0x1u32) as u32); crate::simd_ops::lk_add_u64(r8.wrapping_add(P3), (0x18u64) as u64); rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
            1 => { rdx = P6; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, P8, rcx, rsp); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_35928<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64, const P9: u64, const P10: u64, const P11: u64, const P12: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x28u64)) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; r8 = rcx; rdx = (crate::simd_ops::ru64(rax)) as u64; rax = (crate::simd_ops::ru64(rdx)) as u64; rdx = ((crate::simd_ops::ru16(P1)) as u32) as u32 as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(rdx.wrapping_mul(8)))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 1; continue; } xmm1 = ((crate::simd_ops::ru32(rax.wrapping_add(P3))) as u32) as u128; xmm0 = ((crate::simd_ops::ru32(rax.wrapping_add(P4))) as u32) as u128; rax = (crate::simd_ops::ru32(rax.wrapping_add(P5))) as u32 as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P6))) as u64; crate::simd_ops::wu32(rcx, ((rax as u32)) as u32); rax = 0x1; crate::simd_ops::wu32(rcx.wrapping_add(P7), ((xmm0 as u32)) as u32); crate::simd_ops::wu32(rcx.wrapping_add(P6), ((xmm1 as u32)) as u32); crate::simd_ops::wu32(rcx.wrapping_add(P8), (0x0u32) as u32); crate::simd_ops::wu32(rcx.wrapping_add(P9), (0x4u32) as u32); crate::simd_ops::lk_add_u64(r8.wrapping_add(P6), (0x18u64) as u64); rsp = ((rsp).wrapping_add(0x28u64)) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            1 => { rdx = P10; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P11, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::lift_ops::trap(0x80000003, P12, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_35948<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rcx; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P1))) as u64; rcx = rax; rdx = ((rdx).wrapping_add(0x18u64)) as u64; fl_l = rdx; fl_r = (crate::simd_ops::ru64(rbx.wrapping_add(P2))) as u64; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 3; continue; } rax = P3; fl_l = rdx; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 3; continue; } fl_l = (crate::simd_ops::ru32(rdx.wrapping_add(P1))) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 3; continue; } rax = (crate::simd_ops::ru32(rdx.wrapping_add(P1))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 2; continue; } fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 1; continue; } fl_l = (crate::simd_ops::ru32(rdx)) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); crate::simd_ops::wu8(rcx, ((rax as u8)) as u8); rax = 0; fl_l = 0; fl_r = 0; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            2 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; crate::simd_ops::wu8(rcx, ((rax as u8)) as u8); rax = 0; fl_l = 0; fl_r = 0; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            3 => { rdx = 0x2; rcx = rbx; r8 = ((rdx.wrapping_add(P4)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, P6, rcx, rsp); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_35972<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64, const P9: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 15>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x70u64)) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x10u64))) as u64; rbx = P0; rdi = rcx; r9 = rbx; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; fl_l = rax; fl_r = rcx; r9 = (if (((fl_l as u64)) < ((fl_r as u64))) { rax } else { r9 }) as u64; fl_l = (crate::simd_ops::ru32(r9.wrapping_add(0x10u64))) as u32 as u64; fl_r = 0x2; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 1; continue; } r9 = 0; fl_l = 0; fl_r = 0; bb = 2; continue; }
            1 => { r9 = (crate::simd_ops::ru64(r9)) as u64; fl_l = r9; fl_r = r9; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { fl_l = rax; fl_r = rcx; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 6; continue; } fl_l = rax; fl_r = rbx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 6; continue; } fl_l = (crate::simd_ops::ru32(rax.wrapping_add(0x10u64))) as u32 as u64; fl_r = 0x2; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 6; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x78u64))) as u64; rcx = rsp.wrapping_add(0x40); rdx = (crate::simd_ops::ru64(rax)) as u64; rdx = (crate::simd_ops::ru64(rdx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = (crate::simd_ops::ru8(rsp.wrapping_add(0x60u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 7; continue; } rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x10u64))) as u64; rax = ((rax).wrapping_add(0x18u64)) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; rbx = (if (((fl_l as u64)) < ((fl_r as u64))) { rax } else { rbx }) as u64; fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x10u64))) as u32 as u64; fl_r = 0x3; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 4; continue; } rdx = rsp.wrapping_add(0x20); rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { xmm1 = ((crate::simd_ops::ru64(rbx)) as u64) as u128; let y_addr: u64 = rsp.wrapping_add(0x40u64); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); xmm1 = crate::simd_ops::vcvtpd2ps(xmm1); let y_addr: u64 = rsp.wrapping_add(0x40u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); rcx = rsp.wrapping_add(0x40); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; rax = 0; fl_l = 0; fl_r = 0; rsp = ((rsp).wrapping_add(0x70u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            5 => { rdx = 0x2; rcx = rdi; r8 = ((rdx.wrapping_add(0x1u64)) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::lift_ops::trap(0x80000003, P7, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            6 => { rdx = 0x1; rcx = rdi; r8 = ((rdx.wrapping_add(0x1u64)) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::lift_ops::trap(0x80000003, P8, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            7 => { r8 = P5; rdx = 0x1; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::lift_ops::trap(0x80000003, P9, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_35976<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64, const P9: u64, const P10: u64, const P11: u64, const P12: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rdx = P1; rbx = rcx; r9 = rdx; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P2))) as u64; fl_l = rax; fl_r = rcx; r9 = (if (((fl_l as u64)) < ((fl_r as u64))) { rax } else { r9 }) as u64; fl_l = (crate::simd_ops::ru32(r9.wrapping_add(P0))) as u32 as u64; fl_r = 0x2; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 1; continue; } r9 = 0; fl_l = 0; fl_r = 0; bb = 2; continue; }
            1 => { r9 = (crate::simd_ops::ru64(r9)) as u64; fl_l = r9; fl_r = r9; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { fl_l = rax; fl_r = rcx; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 5; continue; } fl_l = rax; fl_r = rdx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 5; continue; } fl_l = (crate::simd_ops::ru32(rax.wrapping_add(P0))) as u32 as u64; fl_r = 0x2; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 5; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(P3))) as u64; rcx = rsp.wrapping_add(0x20); rdx = (crate::simd_ops::ru64(rax)) as u64; rdx = (crate::simd_ops::ru64(rdx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = (crate::simd_ops::ru8(rsp.wrapping_add(0x38u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 4; continue; } rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P2))) as u64; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rcx = ((rcx).wrapping_shl(7u32)) as u64; xmm0 = ((crate::simd_ops::ru32(rcx.wrapping_add(rax).wrapping_add(P5))) as u32) as u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(crate::simd_ops::ru32(P6))).to_bits() as u128); xmm1 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | (((f32::from_bits(xmm0 as u32)) as f64).to_bits() as u128); crate::simd_ops::wu64(rdx, ((xmm1 as u64)) as u64); crate::simd_ops::wu32(rdx.wrapping_add(P0), (0x3u32) as u32); rax = 0x1; crate::simd_ops::lk_add_u64(rbx.wrapping_add(P2), (0x18u64) as u64); rsp = ((rsp).wrapping_add(0x40u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            4 => { r8 = P7; rdx = 0x1; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P8, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::lift_ops::trap(0x80000003, P11, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            5 => { rdx = 0x1; rcx = rbx; r8 = ((rdx.wrapping_add(P9)) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P10, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::lift_ops::trap(0x80000003, P12, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_35998<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    crate::simd_ops::wu8(rax.wrapping_add(P1), (0x1u8) as u8);
    rax = 0;
    fl_l = 0; fl_r = 0;
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_36001<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; r8 = P0; rbx = rcx; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P2))) as u64; r9 = rax; rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P3))) as u64; rdx = ((rdx).wrapping_add(0x18u64)) as u64; fl_l = rdx; fl_r = rcx; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 3; continue; } rax = P4; fl_l = rdx; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 3; continue; } r8 = (crate::simd_ops::ru32(rdx.wrapping_add(P2))) as u32 as u64; fl_l = ((r8 as u32)) as u32 as u64; fl_r = 0xffffffff; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 3; continue; } fl_l = ((r8 as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 4; continue; } rax = ((r8 as u32)) as u32 as u64; fl_l = ((r8 as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 2; continue; } fl_l = ((rax as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 1; continue; } fl_l = (crate::simd_ops::ru32(rdx)) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); crate::simd_ops::wu8(r9.wrapping_add(P5), ((rax as u8)) as u8); rax = 0; fl_l = 0; fl_r = 0; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            2 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; crate::simd_ops::wu8(r9.wrapping_add(P5), ((rax as u8)) as u8); rax = 0; fl_l = 0; fl_r = 0; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            3 => { rax = 0; fl_l = 0; fl_r = 0; fl_l = (crate::simd_ops::ru8(r9.wrapping_add(P5))) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; crate::simd_ops::wu32(rcx.wrapping_add(P2), (0x1u32) as u32); rax = (rax & !0xffu64) | (((if (((fl_l as u8)) != ((fl_r as u8))) { 1u8 } else { 0u8 }) as u8 as u64)); crate::simd_ops::wu32(rcx, ((rax as u32)) as u32); rax = 0x1; crate::simd_ops::lk_add_u64(rbx.wrapping_add(P3), (0x18u64) as u64); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            4 => { rdx = 0x2; rcx = rbx; r8 = ((rdx.wrapping_add(P6)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, P8, rcx, rsp); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_36013<const P0: u64, const P1: u64, const P2: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 0>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (0xffffffffffffffffu64) as u64);
    rcx = (crate::simd_ops::ru64(rax)) as u64;
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    rcx = ((crate::simd_ops::ru16(P1)) as u32) as u32 as u64;
    rax = (crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64;
    rdx = (crate::simd_ops::ru64(rax.wrapping_add(rcx.wrapping_mul(8)))) as u64;
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x8u64))) as u64;
    crate::simd_ops::wu64(rdx, rax);
    rax = 0;
    fl_l = 0; fl_r = 0;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_36051<const P0: u64, const P1: u64, const P2: u64, const P3: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rbx = rcx; rcx = rsp.wrapping_add(0x20); rdx = (crate::simd_ops::ru64(rax)) as u64; rdx = (crate::simd_ops::ru64(rdx)) as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru8(rsp.wrapping_add(0x38u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 1; continue; } rax = (((crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as i64).wrapping_mul((0x38u64) as i64) as u64) as u64; rcx = 0x3fff; rax = ((rax).wrapping_add(crate::simd_ops::ru64(rsp.wrapping_add(0x20u64)))) as u64; fl_l = (crate::simd_ops::ru16(rax)) as u16 as u64; fl_r = ((rcx as u16)) as u16 as u64; if ((((fl_l as u16)) & ((fl_r as u16))) == 0) { bb = 1; continue; } rcx = (0x3fffffffffffc000u64) as u64; fl_l = (crate::simd_ops::ru64(rax)) as u64; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 1; continue; } rax = (crate::simd_ops::ru64(rax)) as u64; rcx = ((rbx.wrapping_add(P2)) as u64) as u64; rdx = (crate::simd_ops::ru64(rcx)) as u64; r8 = rcx; fl_l = rax; fl_r = (0x100u64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 2; continue; } crate::simd_ops::wu64(rdx, rax); rax = 0x1; crate::simd_ops::wu32(rdx.wrapping_add(P3), (0x2u32) as u32); crate::simd_ops::lk_add_u64(rcx, (0x18u64) as u64); rsp = ((rsp).wrapping_add(0x40u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            1 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P2))) as u64; rcx = ((rbx.wrapping_add(P2)) as u64) as u64; r8 = rcx; bb = 2; continue; }
            2 => { crate::simd_ops::wu32(rdx.wrapping_add(P3), (0x0u32) as u32); rax = (crate::simd_ops::ru64(r8)) as u64; rax = ((rax).wrapping_add(0x18u64)) as u64; crate::simd_ops::wu64(rcx, rax); rax = 0x1; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_36053<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64, const P9: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rdx = (crate::simd_ops::ru64(rax)) as u64; rax = (crate::simd_ops::ru64(rdx)) as u64; rbx = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64; rax = ((crate::simd_ops::ru16(P2)) as u32) as u32 as u64; fl_l = (crate::simd_ops::ru64(rbx.wrapping_add(rax.wrapping_mul(8)))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 4; continue; } rbx = (crate::simd_ops::ru64(rbx.wrapping_add(rax.wrapping_mul(8)))) as u64; fl_l = (crate::simd_ops::ru8(rbx)) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 4; continue; } r8 = P3; rdx = 0x1; rax = crate::simd_ops::call_stk_caught(P4, rax, rcx, rdx, r8, r9, rsp); rdi = ((rax as u32)) as u32 as u64; r8 = (crate::simd_ops::ru32(rbx.wrapping_add(P5))) as u32 as u64; rax = (crate::simd_ops::ru64(rbx.wrapping_add(P6))) as u64; rdx = ((rax.wrapping_add(r8.wrapping_mul(4))) as u64) as u64; rcx = rax; fl_l = rax; fl_r = rdx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru32(rcx)) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 4; continue; } rcx = ((rcx).wrapping_add(0x4u64)) as u64; fl_l = rcx; fl_r = rdx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } bb = 2; continue; }
            2 => { rcx = (crate::simd_ops::ru32(rbx.wrapping_add(P7))) as u32 as u64; rax = ((rax.wrapping_add(rcx.wrapping_mul(4))) as u64) as u64; fl_l = rdx; fl_r = rax; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } rax = ((r8.wrapping_add(P8)) as u64) as u64; rdx = ((rcx as u32)) as u32 as u64; rdx = ((rdx).wrapping_shr(1u32)) as u64; rdx = ((rdx).wrapping_add(rcx)) as u64; fl_l = rdx; fl_r = rax; rdx = (if (((fl_l as u64)) < ((fl_r as u64))) { rax } else { rdx }) as u64; rcx = ((rbx.wrapping_add(P6)) as u64) as u64; rax = crate::simd_ops::call_stk_caught(P9, rax, rcx, rdx, r8, r9, rsp); bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru32(rbx.wrapping_add(P5))) as u32 as u64; rax = (crate::simd_ops::ru64(rbx.wrapping_add(P6))) as u64; crate::simd_ops::wu32(rax.wrapping_add(rcx.wrapping_mul(4)), ((rdi as u32)) as u32); fl_l = (crate::simd_ops::lk_add_u32(rbx.wrapping_add(P5), (1) as u32)) as u32 as u64; bb = 4; continue; }
            4 => { rax = 0; fl_l = 0; fl_r = 0; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_36086<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x28u64)) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; r8 = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; rdx = (crate::simd_ops::ru64(rax)) as u64; rax = (crate::simd_ops::ru64(rdx)) as u64; rdx = ((crate::simd_ops::ru16(P2)) as u32) as u32 as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(P3))) as u64; r9 = (crate::simd_ops::ru64(rax.wrapping_add(rdx.wrapping_mul(8)))) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(P4))) as u64; fl_l = rax; fl_r = r8; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 3; continue; } rdx = P5; fl_l = rax; fl_r = rdx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 3; continue; } rdx = (crate::simd_ops::ru32(rax.wrapping_add(P4))) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0xffffffff; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 3; continue; } fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 4; continue; } fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 2; continue; } fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 1; continue; } fl_l = (crate::simd_ops::ru32(rax)) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); crate::simd_ops::wu8(r9, ((rax as u8)) as u8); rax = 0; fl_l = 0; fl_r = 0; rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
            2 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; crate::simd_ops::wu8(r9, ((rax as u8)) as u8); rax = 0; fl_l = 0; fl_r = 0; rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
            3 => { rax = 0; fl_l = 0; fl_r = 0; fl_l = (crate::simd_ops::ru8(r9)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; crate::simd_ops::wu32(r8.wrapping_add(P4), (0x1u32) as u32); rax = (rax & !0xffu64) | (((if (((fl_l as u8)) != ((fl_r as u8))) { 1u8 } else { 0u8 }) as u8 as u64)); crate::simd_ops::wu32(r8, ((rax as u32)) as u32); rax = 0x1; crate::simd_ops::lk_add_u64(rcx.wrapping_add(P1), (0x18u64) as u64); rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
            4 => { rdx = 0x1; r8 = ((rdx as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, P7, rcx, rsp); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_36121<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64, const P9: u64, const P10: u64, const P11: u64, const P12: u64, const P13: u64, const P14: u64, const P15: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rdx = 0x1; rdi = rcx; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); r10 = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 1; continue; } rdx = ((rax.wrapping_add(0x1u64)) as u32) as u32 as u64; rcx = rdi; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x2; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 7; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), r14); r14 = (crate::simd_ops::ru64(rdi.wrapping_add(0x78u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), r10); rbx = (crate::simd_ops::ru64(r14.wrapping_add(0x8u64))) as u64; rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x38u64))) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(rbx.wrapping_add(0x30u64))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } r11 = (crate::simd_ops::ru64(rbx.wrapping_add(0x40u64))) as u64; bb = 3; continue; }
            2 => { r8 = rsp.wrapping_add(0x50); rcx = rbx; rdx = rsp.wrapping_add(0x20); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); r11 = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; fl_l = r11; fl_r = (0xffffffffffffffffu64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 8; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x40u64))) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rax = ((rax).wrapping_add(crate::simd_ops::ru64(rbx.wrapping_add(0x58u64)))) as u64; r11 = ((r11).wrapping_shl(4u32)) as u64; r11 = ((r11).wrapping_add(crate::simd_ops::ru64(rbx.wrapping_add(0x58u64)))) as u64; fl_l = r11; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 8; continue; } r11 = (crate::simd_ops::ru64(r11.wrapping_add(0x8u64))) as u64; fl_l = ((r11 as u32)) as u32 as u64; fl_r = 0xffffffff; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 8; continue; } rax = (crate::simd_ops::ru64(r14)) as u64; rcx = ((r11 as u32)) as u32 as u64; rbx = (crate::simd_ops::ru64(rax)) as u64; rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x30u64))) as u64; rdx = (crate::simd_ops::ru64(rax.wrapping_add(rcx.wrapping_mul(8)))) as u64; rax = 0xffff; fl_l = ((rdx as u16)) as u16 as u64; fl_r = ((rax as u16)) as u16 as u64; if (((fl_l as u16)) == ((fl_r as u16))) { bb = 6; continue; } r8 = ((crate::simd_ops::ru16(P3)) as u32) as u32 as u64; rax = (((rdx as u16)) as u32) as u32 as u64; rax = ((rax).wrapping_shl(5u32)) as u64; rdx = (crate::simd_ops::ru32(rax.wrapping_add(rbx).wrapping_add(0x180a8u64))) as u32 as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(rbx).wrapping_add(0x18090u64))) as u64; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0xffffffff; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 4; continue; } rdx = r11; rcx = rbx; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); bb = 5; continue; }
            4 => { rax = 0; fl_l = 0; fl_r = 0; bb = 5; continue; }
            5 => { fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            6 => { rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P6)) as u64, rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, P10, rcx, rsp); return rax; }
            7 => { rdx = 0x1; rcx = rdi; r8 = ((rdx.wrapping_add(0x1u64)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, P11, rcx, rsp); return rax; }
            8 => { rdx = P8; rcx = rdi; rax = crate::simd_ops::call_stk(P9, rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, P12, rcx, rsp); return rax; }
            9 => { crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), r14); r14 = (crate::simd_ops::ru64(rdi.wrapping_add(0x78u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), r10); rbx = (crate::simd_ops::ru64(r14.wrapping_add(0x8u64))) as u64; rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x38u64))) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(rbx.wrapping_add(0x30u64))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } r11 = (crate::simd_ops::ru64(rbx.wrapping_add(0x40u64))) as u64; bb = 3; continue; }
            10 => { r8 = rsp.wrapping_add(0x50); rcx = rbx; rdx = rsp.wrapping_add(0x20); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); r11 = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; fl_l = r11; fl_r = (0xffffffffffffffffu64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 8; continue; } bb = 11; continue; }
            11 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x40u64))) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rax = ((rax).wrapping_add(crate::simd_ops::ru64(rbx.wrapping_add(0x58u64)))) as u64; r11 = ((r11).wrapping_shl(4u32)) as u64; r11 = ((r11).wrapping_add(crate::simd_ops::ru64(rbx.wrapping_add(0x58u64)))) as u64; fl_l = r11; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 8; continue; } r11 = (crate::simd_ops::ru64(r11.wrapping_add(0x8u64))) as u64; fl_l = ((r11 as u32)) as u32 as u64; fl_r = 0xffffffff; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 8; continue; } rax = (crate::simd_ops::ru64(r14)) as u64; rcx = ((r11 as u32)) as u32 as u64; rbx = (crate::simd_ops::ru64(rax)) as u64; rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x30u64))) as u64; rdx = (crate::simd_ops::ru64(rax.wrapping_add(rcx.wrapping_mul(8)))) as u64; rax = 0xffff; fl_l = ((rdx as u16)) as u16 as u64; fl_r = ((rax as u16)) as u16 as u64; if (((fl_l as u16)) == ((fl_r as u16))) { bb = 6; continue; } r8 = ((crate::simd_ops::ru16(P3)) as u32) as u32 as u64; rax = (((rdx as u16)) as u32) as u32 as u64; rax = ((rax).wrapping_shl(5u32)) as u64; rdx = (crate::simd_ops::ru32(rax.wrapping_add(rbx).wrapping_add(0x180a8u64))) as u32 as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(rbx).wrapping_add(0x18090u64))) as u64; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0xffffffff; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 4; continue; } rdx = r11; rcx = rbx; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); bb = 5; continue; }
            12 => { rax = 0; fl_l = 0; fl_r = 0; bb = 13; continue; }
            13 => { fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            14 => { rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P6)) as u64, rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, P13, rcx, rsp); return rax; }
            15 => { rdx = 0x1; rcx = rdi; r8 = ((rdx.wrapping_add(0x1u64)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, P14, rcx, rsp); return rax; }
            16 => { rdx = P8; rcx = rdi; rax = crate::simd_ops::call_stk(P9, rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, P15, rcx, rsp); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_36123(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
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
    loop { rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = rbx; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(rax)) as u64, rax, rcx, rdx, r8, r9, rsp); rbx = ((rbx).wrapping_add(0x8u64)) as u64; fl_l = rdi; fl_r = (0x1u64) as u64; rdi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if !((((fl_l as u64)) != ((fl_r as u64)))) { break; } }
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_36133<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x60u64)) as u64;
    xmm0 = (crate::simd_ops::ru128(P0)) as u128;
    rbx = ((rcx.wrapping_add(P1)) as u64) as u64;
    rdi = rcx;
    rcx = ((rbx.wrapping_add(P2)) as u64) as u64;
    r8 = rsp.wrapping_add(0x20);
    rdx = rsp.wrapping_add(0x40);
    crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128);
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64;
    rdx = rsp.wrapping_add(0x40);
    rcx = rsp.wrapping_add(0x20);
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbx);
    xmm0 = (crate::simd_ops::ru128(rax.wrapping_add(P4))) as u128;
    crate::simd_ops::wu128(rsp.wrapping_add(0x28u64), (xmm0) as u128);
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    rdx = rdi;
    rcx = rsp.wrapping_add(0x40);
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64;
    rsp = ((rsp).wrapping_add(0x60u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_36135(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 0>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (0xffffffffffffffffu64) as u64);
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x8u64))) as u64;
    crate::simd_ops::wu64(rcx, rax);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_36138<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
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
    loop { rcx = rbx; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rbx = ((rbx).wrapping_add(0x28u64)) as u64; fl_l = rdi; fl_r = (0x1u64) as u64; rdi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if !((((fl_l as u64)) != ((fl_r as u64)))) { break; } }
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_36174<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rax = (crate::simd_ops::ru64(rdx)) as u64;
    rdx = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64;
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    fl_l = ((rdx).wrapping_add(rdx)) as u64; fl_r = fl_l;
    rdx = ((fl_l as u64)) as u64;
    xmm0 = ((crate::simd_ops::ru32(rax.wrapping_add(rdx.wrapping_mul(8)).wrapping_add(P0))) as u32) as u128;
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_36179<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64, const P9: u64, const P10: u64, const P11: u64, const P12: u64, const P13: u64, const P14: u64, const P15: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 15>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x70u64)) as u64; rsi = rcx; rbx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rbx); rbp = ((crate::simd_ops::ru16(P0)) as u32) as u32 as u64; rdi = ((rbp as u32)) as u32 as u64; rdi = ((rdi).wrapping_shl(6u32)) as u64; rdi = ((rdi).wrapping_add(crate::simd_ops::ru64(rcx.wrapping_add(0x90u64)))) as u64; rax = (crate::simd_ops::ru32(P1)) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x3cu64), ((rax as u32)) as u32); rax = P2; crate::simd_ops::wu64(rdi.wrapping_add(0x30u64), rax); crate::simd_ops::wu8(rdi.wrapping_add(0x38u64), ((rbx as u8)) as u8); rdx = 0x6348; rax = ((crate::exe::tls_pointer() as u64)) as u64; rcx = (crate::simd_ops::ru64(rax)) as u64; rcx = (crate::simd_ops::ru64(rdx.wrapping_add(rcx))) as u64; r8 = (crate::simd_ops::ru64(rcx.wrapping_add(0x18u64))) as u64; rdx = (crate::simd_ops::ru64(r8.wrapping_add(0x10u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 1; continue; } r9 = 0; fl_l = 0; fl_r = 0; r8 = 0; fl_l = 0; fl_r = 0; rdx = ((rbx.wrapping_add(0x10u64)) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 2; continue; } (rax, xmm0) = crate::simd_ops::call_stk_xmm((crate::simd_ops::ru64(P4)) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rax; rdx = 0xdeadd00d; (rax, xmm0) = crate::simd_ops::call_stk_xmm((crate::simd_ops::ru64(P5)) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 3; continue; }
            1 => { fl_l = (crate::simd_ops::lk_add_u32(r8.wrapping_add(0x18u64), (1) as u32)) as u32 as u64; rax = (crate::simd_ops::ru64(rdx)) as u64; crate::simd_ops::wu64(r8.wrapping_add(0x10u64), rax); bb = 2; continue; }
            2 => { rbx = rdx; bb = 3; continue; }
            3 => { rdx = rsp.wrapping_add(0x20); rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rdi, rbx); rax = (crate::simd_ops::ru64(rsi.wrapping_add(0xa0u64))) as u64; crate::simd_ops::wu64(rax.wrapping_add(rbp.wrapping_mul(8)), rbx); fl_l = (crate::simd_ops::ru32(rsp.wrapping_add(0x2cu64))) as u32 as u64; fl_r = 0x40000000; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 7; continue; } r10 = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; fl_l = r10; fl_r = r10; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } r10 = ((r10).wrapping_sub(1)) as u64; r10 = (((r10) & (0xfffffffffe000000u64))) as u64; rax = ((crate::exe::teb_self() as u64)) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(r10.wrapping_add(0x100u64))) as u64; rdx = (rdx & !0xffu64) | (((if (((fl_l as u64)) == ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); r9 = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; rax = r9; rax = ((rax).wrapping_sub(r10)) as u64; rax = ((rax).wrapping_shr(16u32)) as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rcx = r10.wrapping_add(0x108); rcx = ((rcx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x4u64))) as u32 as u64; rcx = ((rcx).wrapping_sub(rax)) as u64; fl_l = ((rdx as u8)) as u8 as u64; fl_r = ((rdx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 5; continue; } fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0xeu64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 4; continue; } rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(r9, rax); crate::simd_ops::wu64(rcx.wrapping_add(0x20u64), r9); fl_l = (crate::simd_ops::lk_add_u32(rcx.wrapping_add(0x18u64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; if (((fl_l as u32)) != 0) { bb = 7; continue; } (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 7; continue; }
            4 => { r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 6; continue; }
            5 => { r8 = 0; fl_l = 0; fl_r = 0; bb = 6; continue; }
            6 => { rdx = rcx; rcx = r10; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P8, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 7; continue; }
            7 => { rbx = rsi.wrapping_add(0x8); xmm0 = (crate::simd_ops::ru128(P9)) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128); rcx = rbx.wrapping_add(0x18); r8 = rsp.wrapping_add(0x20); rdx = rsp.wrapping_add(0x40); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P10, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rbx); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; xmm0 = (crate::simd_ops::ru128(rax.wrapping_add(0x10u64))) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rsp.wrapping_add(0x58u64), (xmm0) as u128); rdx = rsp.wrapping_add(0x20); rcx = rsp.wrapping_add(0x50); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P11, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; rcx = rsi.wrapping_add(0x48); xmm0 = (crate::simd_ops::ru128(P12)) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm0) as u128); rdx = rsp.wrapping_add(0x40); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P13, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r14 = rax; rbx = ((crate::simd_ops::ru16(rax)) as u32) as u32 as u64; rbp = 0xffff; fl_l = ((rbx as u16)) as u16 as u64; fl_r = ((rbp as u16)) as u16 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { bb = 9; continue; } rdi = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; xmm0 = (crate::simd_ops::ru128(P12)) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm0) as u128); rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x18u64))) as u32 as u64; r8 = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; r8 = ((r8).wrapping_shl(4u32)) as u64; rax = (0x2aaaaaaaaaaaaaabu64) as u64; let prod = (rax as i64 as i128).wrapping_mul((r8) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; rbx = rdx; rbx = ((((rbx) as i64).wrapping_shr(3u32) as u64)) as u64; rax = rbx; rax = ((rax).wrapping_shr(63u32)) as u64; rbx = ((rbx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x1cu64))) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; fl_l = r8; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 8; continue; } rdx = rbx.wrapping_add(0x1); rcx = rdi.wrapping_add(0x10); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P14, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 8; continue; }
            8 => { rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x18u64))) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rdi.wrapping_add(0x10u64)))) as u64; xmm0 = 0u128; ymm0_hi = 0; let y_addr: u64 = rcx; crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); crate::simd_ops::wu128(rcx.wrapping_add(0x20u64), (xmm0) as u128); crate::simd_ops::lk_add_u32(rdi.wrapping_add(0x18u64), (1) as u32); rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x18u64))) as u32 as u64; rdx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rdx = ((rdx).wrapping_shl(4u32)) as u64; fl_l = ((rdx).wrapping_add(crate::simd_ops::ru64(rdi.wrapping_add(0x10u64)))) as u64; fl_r = fl_l; rdx = ((fl_l as u64)) as u64; xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x40u64))) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rdx.wrapping_sub(0x30u64), (xmm0) as u128); crate::simd_ops::wu32(rdx.wrapping_sub(0xeu64), (0xffffffffu32) as u32); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu64(rdx.wrapping_sub(0x20u64), rax); crate::simd_ops::wu64(rdx.wrapping_sub(0x18u64), rbx); crate::simd_ops::wu16(r14, ((rbx as u16)) as u16); bb = 9; continue; }
            9 => { r9 = 0x2; r8 = (((rbx as u16)) as u32) as u32 as u64; rdx = ((crate::simd_ops::ru16(rsp.wrapping_add(0x2au64))) as u32) as u32 as u64; rcx = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; r11 = rsp.wrapping_add(0x70); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x10u64))) as u64; rbp = (crate::simd_ops::ru64(r11.wrapping_add(0x18u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x20u64))) as u64; rdi = (crate::simd_ops::ru64(r11.wrapping_add(0x28u64))) as u64; rsp = r11; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk_xmm(P15, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_36181<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 55>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rbp = rsp.wrapping_sub(0x90); rsp = ((rsp).wrapping_sub(0x190u64)) as u64; rsi = rcx; rax = P0; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (0x4u64) as u64); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdi = rax; r15 = P2; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), r15); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x30u64))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm0) as u128); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (0xcu64) as u64); rax = rsp.wrapping_add(0x40); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rax); rbx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x78u64), (0x7ffu64) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rdi); crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), (0x7ffu64) as u64); r14 = P3; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), r14); crate::simd_ops::wu64(rbp.wrapping_sub(0x80u64), rdi); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x30u64))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm0) as u128); xmm1 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm1) as u128); r9 = ((rbx as u32)) as u32 as u64; r8 = rsp.wrapping_add(0x30); rdx = rsp.wrapping_add(0x20); rcx = rsp.wrapping_add(0x50); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; r11 = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; rax = r11; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rax = ((rax).wrapping_sub(rcx)) as u64; r10 = ((rbx as u32)) as u32 as u64; fl_l = r11; fl_r = rcx; r10 = (if (((fl_l as u64)) > ((fl_r as u64))) { rax } else { r10 }) as u64; r9 = ((rdx.wrapping_add(rcx)) as u64) as u64; fl_l = rdx; fl_r = r10; r10 = (if (((fl_l as u64)) < ((fl_r as u64))) { rdx } else { r10 }) as u64; r8 = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rcx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x80u64))) as u64; fl_l = r8; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } fl_l = ((rcx).wrapping_add(r10)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x80u64), rcx); r8 = rbp.wrapping_sub(0x78); crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), r8); crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), (0x100u64) as u64); bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), r14); rax = r11; rax = ((rax).wrapping_sub(r9)) as u64; rdx = rbx; fl_l = r11; fl_r = r9; rdx = (if (((fl_l as u64)) > ((fl_r as u64))) { rax } else { rdx }) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), r9); fl_l = rdx; fl_r = rdx; rdx = (if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { rbx } else { rdx }) as u64; fl_l = r8; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } rax = ((rcx.wrapping_add(rdx)) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x80u64), rax); rax = rbp.wrapping_sub(0x78); crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), (0x100u64) as u64); bb = 2; continue; }
            2 => { crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), rbx); crate::simd_ops::wu8(rcx, (0x0u8) as u8); rdx = (0xffffffffffffffffu64) as u64; bb = 3; continue; }
            3 => { rdx = ((rdx).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru8(rdi.wrapping_add(rdx))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 3; continue; } rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rsi, rbx); crate::simd_ops::wu64(rsi.wrapping_add(0x8u64), rax); crate::simd_ops::wu64(rsi.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsi.wrapping_add(0x18u64), rbx); rcx = rsi.wrapping_add(0x20); crate::simd_ops::wu64(rbp.wrapping_add(0xc8u64), rcx); crate::simd_ops::lk_and_u32(rcx, (0xff000004u32) as u32); crate::simd_ops::lk_or_u32(rcx, (0x4u32) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x3u64), (0x4u8) as u8); crate::simd_ops::wu32(rcx.wrapping_add(0x4u64), ((rbx as u32)) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x8u64), (0x0u8) as u8); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r15); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (0x23u64) as u64); r9 = 0x23; r8 = 0; fl_l = 0; fl_r = 0; rdx = rsp.wrapping_add(0x20); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = rsi; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x1d0u64))) as u64; rsp = ((rsp).wrapping_add(0x190u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_36186<const P0: u64, const P1: u64, const P2: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
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
    rax = (((rbx) as i64).wrapping_mul((0x70u64) as i64) as u64) as u64;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rax = ((rax).wrapping_add(crate::simd_ops::ru64(rdi.wrapping_add(P2)))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_36188<const P0: u64, const P1: u64, const P2: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
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
    rbx = ((rbx).wrapping_shl(5u32)) as u64;
    rbx = ((rbx).wrapping_add(crate::simd_ops::ru64(rdi.wrapping_add(P2)))) as u64;
    rax = rbx;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_36194<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (0u64, 0u64, 0u64);
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    rcx = (crate::simd_ops::ru32(rax)) as u32 as u64;
    fl_l = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64;
    rax = (rax & !0xffu64) | (((if (((fl_l as u32)) == ((fl_r as u32))) { 1u8 } else { 0u8 }) as u8 as u64));
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_36233(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (0u64, 0u64, 0u64);
    rax = (crate::simd_ops::ru32(rdx)) as u32 as u64;
    r8 = (crate::simd_ops::ru64(rcx)) as u64;
    fl_l = (crate::simd_ops::ru32(r8)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64;
    rax = (rax & !0xffu64) | (((if (((fl_l as u32)) == ((fl_r as u32))) { 1u8 } else { 0u8 }) as u8 as u64));
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_36272(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (0u64, 0u64, 0u64);
    fl_l = (crate::simd_ops::ru32(rcx)) as u32 as u64; fl_r = 0x0;
    rax = (rax & !0xffu64) | (((if (((fl_l as u32)) != ((fl_r as u32))) { 1u8 } else { 0u8 }) as u8 as u64));
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_36277<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    crate::simd_ops::wu64(rcx, rdx);
    rax = rcx;
    crate::simd_ops::wu64(rcx.wrapping_add(P0), rdx);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_36278<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    crate::simd_ops::wu64(rcx, rax);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_36279<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    crate::simd_ops::wu64(rcx, rdx);
    rax = rcx;
    crate::simd_ops::wu64(rcx.wrapping_add(P0), (0x0u64) as u64);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_36282(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = (0x492492492492492u64) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_36283<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    crate::simd_ops::forward_incoming_to_outgoing();
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    fl_l = rdx; fl_r = rdx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rcx = rdx; crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8(P0, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_36284<const P0: u64, const P1: u64, const P2: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x28u64)) as u64; rax = (0x492492492492492u64) as u64; fl_l = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 1; continue; } rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
            1 => { rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, P2, rcx, rsp); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_36287(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l) = (crate::simd_ops::get_rax_in(), 0u64);
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    rax = ((rax).wrapping_sub(crate::simd_ops::ru64(rdx))) as u64;
    fl_l = ((((rax) as i64).wrapping_shr(4u32) as u64)) as u64;
    rax = ((fl_l as u64)) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_36289<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64;
    rax = (((rax as u32)).wrapping_sub(1)) as u32 as u64;
    rax = ((rax).wrapping_shl(4u32)) as u64;
    fl_l = ((rax).wrapping_add(crate::simd_ops::ru64(rcx))) as u64; fl_r = fl_l;
    rax = ((fl_l as u64)) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_36292<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rax = 0;
    fl_l = 0; fl_r = 0;
    rbx = rcx;
    crate::simd_ops::wu64(rcx, rax);
    crate::simd_ops::wu64(rcx.wrapping_add(P0), rax);
    rdx = ((rax.wrapping_add(P0)) as u32) as u32 as u64;
    rcx = ((rax.wrapping_add(P1)) as u32) as u32 as u64;
    rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp);
    crate::simd_ops::wu64(rax, rax);
    crate::simd_ops::wu64(rax.wrapping_add(P0), rax);
    crate::simd_ops::wu64(rax.wrapping_add(P3), rax);
    crate::simd_ops::wu16(rax.wrapping_add(P4), (0x101u16) as u16);
    crate::simd_ops::wu64(rbx, rax);
    rax = rbx;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_36293<const P0: u64, const P1: u64, const P2: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; fl_r = 0x40000000;
    rbx = rcx;
    crate::simd_ops::wu32(rcx.wrapping_add(P1), (0x0u32) as u32);
    if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    rcx = (crate::simd_ops::ru64(rcx)) as u64;
    fl_l = rcx; fl_r = rcx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); }
    crate::simd_ops::lk_and_u32(rbx.wrapping_add(P0), (0xc0000000u32) as u32);
    crate::simd_ops::wu64(rbx, (0x0u64) as u64);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_36294<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 0>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); r9 = (crate::simd_ops::ru64(rcx)) as u64; rbx = rcx; rax = (crate::simd_ops::ru64(r9.wrapping_add(P0))) as u64; fl_l = (crate::simd_ops::ru8(rax.wrapping_add(P1))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 5; continue; } r10 = (crate::simd_ops::ru32(r8)) as u32 as u64; bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru32(rax.wrapping_add(P2))) as u32 as u64; fl_r = ((r10 as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 3; continue; } if ((((fl_l as u32)) as i32) > (((fl_r as u32)) as i32)) { bb = 2; continue; } rcx = (crate::simd_ops::ru32(r8.wrapping_add(P3))) as u32 as u64; fl_l = (crate::simd_ops::ru32(rax.wrapping_add(P4))) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 3; continue; } bb = 2; continue; }
            2 => { r9 = rax; rax = (crate::simd_ops::ru64(rax)) as u64; bb = 4; continue; }
            3 => { rax = (crate::simd_ops::ru64(rax.wrapping_add(P5))) as u64; bb = 4; continue; }
            4 => { fl_l = (crate::simd_ops::ru8(rax.wrapping_add(P1))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 1; continue; } bb = 5; continue; }
            5 => { fl_l = (crate::simd_ops::ru8(r9.wrapping_add(P1))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 7; continue; } rax = (crate::simd_ops::ru32(r8)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(r9.wrapping_add(P2))) as u32 as u64; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 7; continue; } if ((((fl_l as u32)) as i32) > (((fl_r as u32)) as i32)) { bb = 6; continue; } rax = (crate::simd_ops::ru32(r9.wrapping_add(P4))) as u32 as u64; fl_l = (crate::simd_ops::ru32(r8.wrapping_add(P3))) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 7; continue; } bb = 6; continue; }
            6 => { crate::simd_ops::wu64(rdx, r9); rax = rdx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x8u64))) as u64; return rax; }
            7 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(rdx, rax); rax = rdx; return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_36339<const P0: u64, const P1: u64, const P2: u64, const P3: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { r8 = (((crate::simd_ops::ru64(rcx.wrapping_add(P0))) as i64).wrapping_mul((0x198u64) as i64) as u64) as u64; rcx = (crate::simd_ops::ru64(rcx)) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(r8).wrapping_add(P1))) as u64; rcx = (crate::simd_ops::ru32(rcx.wrapping_add(r8).wrapping_add(P2))) as u32 as u64; rcx = (((rcx) as i64).wrapping_mul((0xf0u64) as i64) as u64) as u64; rcx = ((rcx).wrapping_add(rax)) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru32(rax.wrapping_add(P3))) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 3; continue; } rax = ((rax).wrapping_add(0xf0u64)) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } bb = 2; continue; }
            2 => { rax = 0; fl_l = 0; fl_r = 0; bb = 3; continue; }
            3 => { return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_36355<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rbx = rcx; rax = P0; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), (0x3fu64) as u64); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x30u64))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm0) as u128); rcx = rsp.wrapping_add(0x30); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::lk_and_u32(rbx, (0xff000010u32) as u32); crate::simd_ops::lk_or_u32(rbx, (0x10u32) as u32); crate::simd_ops::wu8(rbx.wrapping_add(P2), (0x10u8) as u8); crate::simd_ops::wu32(rbx.wrapping_add(P3), (0x0u32) as u32); rcx = ((rbx.wrapping_add(P4)) as u64) as u64; crate::simd_ops::wu8(rcx, (0x0u8) as u8); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 2; continue; } crate::simd_ops::wu32(rbx.wrapping_add(P3), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rbx)) as u32 as u64; rax = ((((rax as u32)) & (0xffffffu32))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x10; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 1; continue; } rcx = (crate::simd_ops::ru64(rcx)) as u64; bb = 1; continue; }
            1 => { crate::simd_ops::wu8(rcx, (0x0u8) as u8); bb = 4; continue; }
            2 => { r9 = (0xffffffffffffffffu64) as u64; bb = 3; continue; }
            3 => { r9 = ((r9).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru8(rax.wrapping_add(r9))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 3; continue; } crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), r9); r8 = 0; fl_l = 0; fl_r = 0; rdx = rsp.wrapping_add(0x30); rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 4; continue; }
            4 => { rax = rbx; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_36358<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    crate::simd_ops::forward_incoming_to_outgoing();
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = rcx;
    rcx = ((r8 as u32)) as u32 as u64;
    rcx = ((rcx).wrapping_shl(4u32)) as u64;
    fl_l = ((rcx).wrapping_add(rdx)) as u64; fl_r = fl_l;
    rcx = ((fl_l as u64)) as u64;
    rdx = rax;
    crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8(P0, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_36359<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64, const P9: u64, const P10: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rdi = rdx; rsi = rcx; xmm0 = (crate::simd_ops::ru128(rdx)) as u128; crate::simd_ops::wu128(rcx, (xmm0) as u128); rcx = ((rcx).wrapping_add(0x10u64)) as u64; rbp = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rcx, rbp); crate::simd_ops::wu64(rcx.wrapping_add(P0), rbp); rdx = ((rdx).wrapping_add(0x10u64)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = ((rsi.wrapping_add(P2)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rcx); crate::simd_ops::lk_and_u32(rcx, (0xff000004u32) as u32); crate::simd_ops::lk_or_u32(rcx, (0x4u32) as u32); crate::simd_ops::wu8(rcx.wrapping_add(P3), (0x4u8) as u8); crate::simd_ops::wu32(rcx.wrapping_add(P4), ((rbp as u32)) as u32); crate::simd_ops::wu8(rcx.wrapping_add(P0), ((rbp as u8)) as u8); rdx = ((rdi.wrapping_add(P2)) as u64) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = ((rsi.wrapping_add(P6)) as u64) as u64; crate::simd_ops::wu64(rbx, rbp); crate::simd_ops::wu64(rbx.wrapping_add(P0), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rbx); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rax, rax); crate::simd_ops::wu64(rax.wrapping_add(P0), rax); crate::simd_ops::wu64(rax.wrapping_add(P8), rax); crate::simd_ops::wu16(rax.wrapping_add(P9), (0x101u16) as u16); crate::simd_ops::wu64(rbx, rax); rdx = ((rdi.wrapping_add(P6)) as u64) as u64; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P10, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = rsi; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_36360<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
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
    rdx = ((rbx.wrapping_add(P3)) as u64) as u64;
    rcx = ((rdi.wrapping_add(P3)) as u64) as u64;
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rax = rdi;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_36361<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64, const P9: u64, const P10: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rdx; rdi = rcx; xmm0 = (crate::simd_ops::ru128(rdx)) as u128; crate::simd_ops::wu128(rcx, (xmm0) as u128); rcx = ((rcx).wrapping_add(0x10u64)) as u64; rdx = ((rdx).wrapping_add(0x10u64)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = ((rdi.wrapping_add(P1)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rcx); crate::simd_ops::lk_and_u32(rcx, (0xff000004u32) as u32); crate::simd_ops::lk_or_u32(rcx, (0x4u32) as u32); crate::simd_ops::wu8(rcx.wrapping_add(P2), (0x4u8) as u8); crate::simd_ops::wu32(rcx.wrapping_add(P3), (0x0u32) as u32); crate::simd_ops::wu8(rcx.wrapping_add(P4), (0x0u8) as u8); rdx = ((rbx.wrapping_add(P1)) as u64) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rdi.wrapping_add(P6), (0x0u64) as u64); crate::simd_ops::wu64(rdi.wrapping_add(P7), (0x0u64) as u64); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P8, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rax, rax); crate::simd_ops::wu64(rax.wrapping_add(P4), rax); crate::simd_ops::wu64(rax.wrapping_add(P9), rax); crate::simd_ops::wu16(rax.wrapping_add(P10), (0x101u16) as u16); crate::simd_ops::wu64(rdi.wrapping_add(P6), rax); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P6))) as u64; crate::simd_ops::wu64(rdi.wrapping_add(P6), rcx); crate::simd_ops::wu64(rbx.wrapping_add(P6), rax); rcx = (crate::simd_ops::ru64(rdi.wrapping_add(P7))) as u64; rax = (crate::simd_ops::ru64(rbx.wrapping_add(P7))) as u64; crate::simd_ops::wu64(rdi.wrapping_add(P7), rax); crate::simd_ops::wu64(rbx.wrapping_add(P7), rcx); rax = rdi; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_36362<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { fl_l = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } rax = 0; fl_l = 0; fl_r = 0; return rax; }
            1 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; r8 = 0; fl_l = 0; fl_r = 0; rax = (crate::simd_ops::ru64(rax)) as u64; fl_l = (crate::simd_ops::ru8(rax.wrapping_add(P2))) as u8 as u64; fl_r = ((r8 as u8)) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 8; continue; } bb = 2; continue; }
            2 => { rcx = (crate::simd_ops::ru64(rax.wrapping_add(P3))) as u64; fl_l = rcx; fl_r = r8; rcx = (if (((fl_l as u64)) <= ((fl_r as u64))) { r8 } else { rcx }) as u64; r8 = rcx; rcx = (crate::simd_ops::ru64(rax.wrapping_add(P4))) as u64; fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(P2))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 5; continue; } rcx = (crate::simd_ops::ru64(rax.wrapping_add(P5))) as u64; fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(P2))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 4; continue; } bb = 3; continue; }
            3 => { fl_l = rax; fl_r = (crate::simd_ops::ru64(rcx.wrapping_add(P4))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 4; continue; } rax = rcx; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P5))) as u64; fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(P2))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 3; continue; } bb = 4; continue; }
            4 => { rax = rcx; bb = 7; continue; }
            5 => { rdx = (crate::simd_ops::ru64(rcx)) as u64; rax = rcx; fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(P2))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 7; continue; } bb = 6; continue; }
            6 => { rcx = (crate::simd_ops::ru64(rdx)) as u64; rax = rdx; rdx = rcx; fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(P2))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 6; continue; } bb = 7; continue; }
            7 => { fl_l = (crate::simd_ops::ru8(rax.wrapping_add(P2))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 2; continue; } bb = 8; continue; }
            8 => { rax = ((r8.wrapping_add(P6)) as u64) as u64; return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_36363<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; r11 = ((rcx.wrapping_add(P0)) as u64) as u64; r9 = 0; fl_l = 0; fl_r = 0; rcx = (crate::simd_ops::ru64(r11)) as u64; rbx = rdx; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), r9); r10 = (crate::simd_ops::ru64(rcx)) as u64; rax = r10; fl_l = r10; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 7; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64; fl_r = r9; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 8; continue; } rdx = (crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64; fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(P3))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 4; continue; } rdx = (crate::simd_ops::ru64(rax.wrapping_add(P4))) as u64; fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(P3))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 3; continue; } bb = 2; continue; }
            2 => { fl_l = rax; fl_r = (crate::simd_ops::ru64(rdx.wrapping_add(P2))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } rax = rdx; rdx = (crate::simd_ops::ru64(rdx.wrapping_add(P4))) as u64; fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(P3))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 2; continue; } bb = 3; continue; }
            3 => { rax = rdx; bb = 6; continue; }
            4 => { r8 = (crate::simd_ops::ru64(rdx)) as u64; rax = rdx; fl_l = (crate::simd_ops::ru8(r8.wrapping_add(P3))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rdx = (crate::simd_ops::ru64(r8)) as u64; rax = r8; r8 = rdx; fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(P3))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 5; continue; } bb = 6; continue; }
            6 => { fl_l = rax; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), r9); bb = 7; continue; }
            7 => { r9 = rsp.wrapping_add(0x40); r8 = rbx; rdx = rsp.wrapping_add(0x20); rcx = r11; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            8 => { fl_l = ((r9).wrapping_add(1)) as u64; r9 = ((fl_l as u64)) as u64; rax = r10; bb = 1; continue; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_36364<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = rdx; rbp = rcx; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rsi = rax; rbx = 0; fl_l = 0; fl_r = 0; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 1; continue; }
            1 => { r8 = (crate::simd_ops::ru64(rbp.wrapping_add(0x48u64))) as u64; rdx = (crate::simd_ops::ru64(r8)) as u64; fl_l = rdx; fl_r = r8; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 8; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru64(rdx.wrapping_add(0x28u64))) as u64; fl_r = rbx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 10; continue; } rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x10u64))) as u64; fl_l = (crate::simd_ops::ru8(rax.wrapping_add(0x19u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 5; continue; } rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x8u64))) as u64; fl_l = (crate::simd_ops::ru8(rax.wrapping_add(0x19u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 4; continue; } bb = 3; continue; }
            3 => { fl_l = rdx; fl_r = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 4; continue; } rdx = rax; rax = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; fl_l = (crate::simd_ops::ru8(rax.wrapping_add(0x19u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 3; continue; } bb = 4; continue; }
            4 => { rdx = rax; bb = 7; continue; }
            5 => { rdx = rax; rcx = (crate::simd_ops::ru64(rax)) as u64; fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0x19u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 7; continue; } bb = 6; continue; }
            6 => { rdx = rcx; rax = (crate::simd_ops::ru64(rcx)) as u64; rcx = rax; fl_l = (crate::simd_ops::ru8(rax.wrapping_add(0x19u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 6; continue; } bb = 7; continue; }
            7 => { fl_l = rdx; fl_r = r8; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 8; continue; }
            8 => { rdx = (crate::simd_ops::ru32(rdi.wrapping_add(0x8u64))) as u32 as u64; r8 = (crate::simd_ops::ru32(rdi.wrapping_add(0xcu64))) as u32 as u64; rcx = ((r8.wrapping_mul(8)) as u64) as u64; rax = ((rdx.wrapping_mul(8)) as u64) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 9; continue; } rax = rdx.wrapping_add(0x1); rdx = ((r8 as u32)) as u32 as u64; rdx = ((rdx).wrapping_shr(1u32)) as u64; rdx = ((rdx).wrapping_add(r8)) as u64; fl_l = rdx; fl_r = rax; rdx = (if (((fl_l as u64)) < ((fl_r as u64))) { rax } else { rdx }) as u64; rcx = rdi; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 9; continue; }
            9 => { rcx = (crate::simd_ops::ru32(rdi.wrapping_add(0x8u64))) as u32 as u64; rax = (crate::simd_ops::ru64(rdi)) as u64; crate::simd_ops::wu64(rax.wrapping_add(rcx.wrapping_mul(8)), rbx); fl_l = (crate::simd_ops::lk_add_u32(rdi.wrapping_add(0x8u64), (1) as u32)) as u32 as u64; bb = 10; continue; }
            10 => { rbx = ((rbx).wrapping_add(1)) as u64; fl_l = rbx; fl_r = rsi; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 1; continue; } bb = 11; continue; }
            11 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_36365<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx) = (0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; r10 = (crate::simd_ops::ru64(rcx.wrapping_add(0x48u64))) as u64; rbx = rcx.wrapping_add(0x48); r11 = rdx; r8 = r10; r9 = 0x3fff; rcx = (crate::simd_ops::ru64(r10.wrapping_add(0x8u64))) as u64; fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0x19u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 5; continue; } rdx = ((crate::simd_ops::ru16(rdx)) as u32) as u32 as u64; fl_l = ((((rdx as u16)) & ((r9 as u16)))) as u16 as u64; fl_r = fl_l; rdx = (rdx & !0xffffu64) | ((((fl_l as u16)) as u16 as u64)); bb = 1; continue; }
            1 => { rax = ((crate::simd_ops::ru16(rcx.wrapping_add(0x20u64))) as u32) as u32 as u64; rax = (rax & !0xffffu64) | ((((((rax as u16)) & ((r9 as u16)))) as u16 as u64)); fl_l = ((rax as u16)) as u16 as u64; fl_r = ((rdx as u16)) as u16 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { bb = 2; continue; } rax = (crate::simd_ops::ru64(r11)) as u64; fl_l = (crate::simd_ops::ru64(rcx.wrapping_add(0x20u64))) as u64; fl_r = rax; bb = 2; continue; }
            2 => { rax = (rax & !0xffu64) | (((if (((fl_l as u64)) < ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 3; continue; } rcx = (crate::simd_ops::ru64(rcx.wrapping_add(0x10u64))) as u64; bb = 4; continue; }
            3 => { r8 = rcx; rcx = (crate::simd_ops::ru64(rcx)) as u64; bb = 4; continue; }
            4 => { fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0x19u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 1; continue; } bb = 5; continue; }
            5 => { fl_l = (crate::simd_ops::ru8(r8.wrapping_add(0x19u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 7; continue; } rax = ((crate::simd_ops::ru16(r11)) as u32) as u32 as u64; rcx = ((crate::simd_ops::ru16(r8.wrapping_add(0x20u64))) as u32) as u32 as u64; rax = (rax & !0xffffu64) | ((((((rax as u16)) & ((r9 as u16)))) as u16 as u64)); rcx = (rcx & !0xffffu64) | ((((((rcx as u16)) & ((r9 as u16)))) as u16 as u64)); fl_l = ((rax as u16)) as u16 as u64; fl_r = ((rcx as u16)) as u16 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { bb = 6; continue; } rax = (crate::simd_ops::ru64(r8.wrapping_add(0x20u64))) as u64; fl_l = (crate::simd_ops::ru64(r11)) as u64; fl_r = rax; bb = 6; continue; }
            6 => { rax = (rax & !0xffu64) | (((if (((fl_l as u64)) < ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 7; continue; } fl_l = r8; fl_r = r10; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 7; continue; } rax = (crate::simd_ops::ru64(r8.wrapping_add(0x28u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            7 => { r9 = (crate::simd_ops::ru64(r10)) as u64; r8 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), r8); rax = r9; fl_l = r9; fl_r = r10; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 14; continue; } bb = 8; continue; }
            8 => { fl_l = (crate::simd_ops::ru64(rax.wrapping_add(0x28u64))) as u64; fl_r = r8; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 15; continue; } rcx = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0x19u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 11; continue; } rcx = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0x19u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 10; continue; } bb = 9; continue; }
            9 => { fl_l = rax; fl_r = (crate::simd_ops::ru64(rcx.wrapping_add(0x10u64))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 10; continue; } rax = rcx; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0x19u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 9; continue; } bb = 10; continue; }
            10 => { rax = rcx; bb = 13; continue; }
            11 => { rdx = (crate::simd_ops::ru64(rcx)) as u64; rax = rcx; fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(0x19u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 13; continue; } bb = 12; continue; }
            12 => { rcx = (crate::simd_ops::ru64(rdx)) as u64; rax = rdx; rdx = rcx; fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0x19u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 12; continue; } bb = 13; continue; }
            13 => { fl_l = rax; fl_r = r10; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 8; continue; } crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), r8); bb = 14; continue; }
            14 => { r9 = rsp.wrapping_add(0x40); r8 = r11; rdx = rsp.wrapping_add(0x20); rcx = rbx; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            15 => { fl_l = ((r8).wrapping_add(1)) as u64; r8 = ((fl_l as u64)) as u64; rax = r9; bb = 8; continue; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_36366<const P0: u64, const P1: u64, const P2: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x30u64)) as u64;
    r10 = rdx;
    rdi = rcx;
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    rbx = rax;
    xmm0 = 0u128;
    fl_l = rax; fl_r = rax;
    if !((((((fl_l as u64)) & ((fl_r as u64))) as i64) < 0)) { xmm0 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | ((((rax) as i64) as f64).to_bits() as u128); } else { rcx = rbx; rax = ((((rax as u32)) & (0x1u32))) as u32 as u64; rcx = ((rcx).wrapping_shr(1u32)) as u64; fl_l = (((rcx) | (rax))) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; xmm0 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | ((((rcx) as i64) as f64).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm0 as u64) + f64::from_bits(xmm0 as u64)).to_bits() as u128); }
    r9 = rsp.wrapping_add(0x40);
    crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), (0x0u32) as u32);
    rdx = rdi;
    rcx = r10;
    crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), ((xmm0 as u64)) as u64);
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    rdx = rbx;
    rcx = rdi;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64;
    rsp = ((rsp).wrapping_add(0x30u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return crate::simd_ops::tail_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_36367<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rcx;
    rcx = ((rcx).wrapping_add(0x48u64)) as u64;
    rdx = rcx;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rcx = ((rbx.wrapping_add(P1)) as u64) as u64;
    rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp);
    rcx = ((rbx.wrapping_add(P3)) as u64) as u64;
    rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_36371<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    xmm2 = (crate::simd_ops::ru128(rdx.wrapping_add(P0))) as u128;
    xmm0 = crate::simd_ops::vaddps(xmm2, crate::simd_ops::ru128(r8));
    xmm1 = 0u128;
    xmm0 = crate::simd_ops::vblendps(xmm0, xmm1, 0x2u8);
    crate::simd_ops::wu128(rcx, (xmm0) as u128);
    rax = rcx;
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_36372(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    xmm0 = (crate::simd_ops::ru128(rdx)) as u128;
    xmm1 = 0u128;
    xmm2 = crate::simd_ops::vblendps(xmm0, xmm1, 0x2u8);
    crate::simd_ops::wu128(rcx, (xmm2) as u128);
    rax = rcx;
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_36383<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64, const P9: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r10, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x48u64)) as u64; rcx = (crate::simd_ops::ru64(r8)) as u64; rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rax); xmm0 = ((crate::simd_ops::ru64(rdx)) as u64) as u128; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), ((xmm0 as u64)) as u64); rax = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((rax as u32)) as u32); r8 = rsp.wrapping_add(0x30); rdx = rsp.wrapping_add(0x20); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = (crate::simd_ops::ru32(rsp.wrapping_add(0x3cu64))) as u32 as u64; fl_r = 0x40000000; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 4; continue; } r10 = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; fl_l = r10; fl_r = r10; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } r10 = ((r10).wrapping_sub(1)) as u64; r10 = (((r10) & (0xfffffffffe000000u64))) as u64; rax = ((crate::exe::teb_self() as u64)) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(r10.wrapping_add(P2))) as u64; rdx = (rdx & !0xffu64) | (((if (((fl_l as u64)) == ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); r9 = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = r9; rax = ((rax).wrapping_sub(r10)) as u64; rax = ((rax).wrapping_shr(16u32)) as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rcx = ((r10.wrapping_add(P3)) as u64) as u64; rcx = ((rcx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(P4))) as u32 as u64; rcx = ((rcx).wrapping_sub(rax)) as u64; fl_l = ((rdx as u8)) as u8 as u64; fl_r = ((rdx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 2; continue; } fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(P5))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 1; continue; } rax = (crate::simd_ops::ru64(rcx.wrapping_add(P6))) as u64; crate::simd_ops::wu64(r9, rax); crate::simd_ops::wu64(rcx.wrapping_add(P6), r9); fl_l = (crate::simd_ops::lk_add_u32(rcx.wrapping_add(P7), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; if (((fl_l as u32)) != 0) { bb = 4; continue; } (rax, xmm0) = crate::simd_ops::call_stk_xmm(P8, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 4; continue; }
            1 => { r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 3; continue; }
            2 => { r8 = 0; fl_l = 0; fl_r = 0; bb = 3; continue; }
            3 => { rdx = rcx; rcx = r10; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P9, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 4; continue; }
            4 => { rsp = ((rsp).wrapping_add(0x48u64)) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_36385<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    crate::simd_ops::forward_incoming_to_outgoing();
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rcx = (crate::simd_ops::ru64(rcx)) as u64;
    fl_l = rcx; fl_r = rcx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rdx = 0x68; crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8(P0, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_36390<const P0: u64, const P1: u64, const P2: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 0>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64;
    crate::simd_ops::wu64(rcx.wrapping_add(P0), rax);
    rax = rcx;
    crate::simd_ops::wu64(rcx, rdx);
    crate::simd_ops::wu64(rcx.wrapping_add(P1), r8);
    crate::simd_ops::wu64(rcx.wrapping_add(P2), r9);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_36401(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (0u64, 0u64, 0u64);
    fl_l = (crate::simd_ops::ru32(rcx)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64;
    rax = (rax & !0xffu64) | (((if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { 1u8 } else { 0u8 }) as u8 as u64));
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_36473<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    crate::simd_ops::wu64(rcx, rax);
    rax = rcx;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_36497<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = (crate::simd_ops::ru32(rdx)) as u32 as u64;
    r9 = rdx;
    rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    r9 = ((r9).wrapping_sub(crate::simd_ops::ru64(r8))) as u64;
    crate::simd_ops::wu32(r9.wrapping_add(rcx), ((rax as u32)) as u32);
    rax = ((rdx.wrapping_add(P1)) as u64) as u64;
    rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    rdx = rax;
    fl_l = rdx; fl_r = (crate::simd_ops::ru64(r8)) as u64;
    rdx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64;
    rax = ((crate::simd_ops::ru8(rax)) as u32) as u32 as u64;
    crate::simd_ops::wu8(rdx.wrapping_add(rcx), ((rax as u8)) as u8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_36515<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
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
    loop { rcx = rbx; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rbx = ((rbx).wrapping_add(0x20u64)) as u64; fl_l = rdi; fl_r = (0x1u64) as u64; rdi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if !((((fl_l as u64)) != ((fl_r as u64)))) { break; } }
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_36528<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdx);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx);
    rsp = ((rsp).wrapping_sub(0x38u64)) as u64;
    rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64;
    rcx = 0x4;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax);
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64;
    rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64;
    rcx = (crate::simd_ops::ru32(rcx)) as u32 as u64;
    crate::simd_ops::wu32(rax, ((rcx as u32)) as u32);
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64;
    rsp = ((rsp).wrapping_add(0x38u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_36544<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdx);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rcx = P1;
    crate::simd_ops::wu64(rax, rcx);
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64;
    rax = (crate::simd_ops::ru64(rax)) as u64;
    rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    crate::simd_ops::wu64(rcx.wrapping_add(P2), rax);
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rax = ((rax).wrapping_add(0x18u64)) as u64;
    rcx = rax;
    rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp);
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rcx = P4;
    crate::simd_ops::wu64(rax, rcx);
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_36549(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (0x0u64) as u64);
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    crate::simd_ops::wu64(rsp, rax);
    rax = (crate::simd_ops::ru64(rsp)) as u64;
    rax = (crate::simd_ops::ru64(rax)) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rax);
    rax = (crate::simd_ops::ru64(rsp)) as u64;
    rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x8u64))) as u64;
    crate::simd_ops::wu64(rax, rcx);
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x10u64))) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rax);
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x18u64))) as u64;
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_36550<const P0: u64, const P1: u64, const P2: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdx);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx);
    rsp = ((rsp).wrapping_sub(0x38u64)) as u64;
    rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64;
    rcx = 0x10;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax);
    rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64;
    rcx = P2;
    crate::simd_ops::wu64(rax, rcx);
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64;
    rsp = ((rsp).wrapping_add(0x38u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_36551<const P0: u64, const P1: u64, const P2: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx);
    rsp = ((rsp).wrapping_sub(0x38u64)) as u64;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax);
    rdx = P1;
    rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64;
    rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x38u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_36552<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 0>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx);
    rax = P0;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_36553(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 3>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx);
    rsp = ((rsp).wrapping_sub(0x18u64)) as u64;
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64;
    rax = ((rax).wrapping_add(0x8u64)) as u64;
    crate::simd_ops::wu64(rsp, rax);
    rax = (crate::simd_ops::ru64(rsp)) as u64;
    rsp = ((rsp).wrapping_add(0x18u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_36554<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu8(rsp.wrapping_add(0x10u64), ((rdx as u8)) as u8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    rax = ((crate::simd_ops::ru8(rsp.wrapping_add(0x38u64))) as u32) as u32 as u64;
    fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64;
    if !(((((fl_l as u32)) & ((fl_r as u32))) == 0)) { rdx = 0x10; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); }
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_36555<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu32(rsp.wrapping_add(0x18u64), ((r8 as u32)) as u32);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdx);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx);
    rsp = ((rsp).wrapping_sub(0x38u64)) as u64;
    crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), (0x0u32) as u32);
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64;
    rax = ((rax).wrapping_add(0x18u64)) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax);
    r8 = (crate::simd_ops::ru32(rsp.wrapping_add(0x50u64))) as u32 as u64;
    rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64;
    rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x20u64))) as u32 as u64;
    rax = ((((rax as u32)) | (0x1u32))) as u32 as u64;
    crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64;
    rsp = ((rsp).wrapping_add(0x38u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_36589<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdx); crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x58u64)) as u64; rax = rsp.wrapping_add(0x40); rdi = rax; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; rcx = 0x10; while rcx != 0 { crate::simd_ops::wu8(rdi, crate::simd_ops::ru8(rsi)); rdi = rdi.wrapping_add(1); rsi = rsi.wrapping_add(1); rcx = rcx.wrapping_sub(1); } rdx = rsp.wrapping_add(0x40); rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm0 = (xmm0 & !0xffff_ffffu128) | (((f64::from_bits(xmm0 as u64)) as f32).to_bits() as u128); crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((xmm0 as u32)) as u32); xmm0 = ((crate::simd_ops::ru32(P1)) as u32) as u128; fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rsp.wrapping_add(0x30u64))) as u32 as u64; if !(f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 1; continue; } xmm0 = ((crate::simd_ops::ru32(P2)) as u32) as u128; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((xmm0 as u32)) as u32); xmm3 = ((crate::simd_ops::ru32(P3)) as u32) as u128; xmm2 = ((crate::simd_ops::ru32(P1)) as u32) as u128; xmm1 = 0u128; xmm0 = ((crate::simd_ops::ru32(rsp.wrapping_add(0x30u64))) as u32) as u128; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu32(rsp.wrapping_add(0x34u64), ((xmm0 as u32)) as u32); bb = 2; continue; }
            1 => { xmm0 = ((crate::simd_ops::ru32(P5)) as u32) as u128; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((xmm0 as u32)) as u32); xmm3 = ((crate::simd_ops::ru32(P2)) as u32) as u128; xmm2 = ((crate::simd_ops::ru32(P6)) as u32) as u128; xmm1 = ((crate::simd_ops::ru32(P1)) as u32) as u128; xmm0 = ((crate::simd_ops::ru32(rsp.wrapping_add(0x30u64))) as u32) as u128; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu32(rsp.wrapping_add(0x34u64), ((xmm0 as u32)) as u32); bb = 2; continue; }
            2 => { xmm0 = ((crate::simd_ops::ru32(rsp.wrapping_add(0x34u64))) as u32) as u128; rsp = ((rsp).wrapping_add(0x58u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_36628<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 21>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x68u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (0x4000004u64) as u64); crate::simd_ops::wu8(rsp.wrapping_add(0x38u64), (0x0u8) as u8); rax = P0; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (0x1au64) as u64); r9 = 0x1a; r8 = 0; fl_l = 0; fl_r = 0; rdx = rsp.wrapping_add(0x20); rcx = rsp.wrapping_add(0x30); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru32(rsp.wrapping_add(0x34u64))) as u32 as u64; rcx = (crate::simd_ops::ru32(rsp.wrapping_add(0x30u64))) as u32 as u64; rax = ((rcx as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_shr(24u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x80; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 1; continue; } rax = ((rax.wrapping_mul(8)) as u32) as u32 as u64; bb = 2; continue; }
            1 => { rax = (((rax as u32)).wrapping_shl(8u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x7c00; rax = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 2; continue; }
            2 => { rcx = ((((rcx as u32)) & (0xffffffu32))) as u32 as u64; rcx = (((rcx as u32)).wrapping_shl(3u32)) as u32 as u64; r15 = rsp.wrapping_add(0x38); fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; r15 = (if (((fl_l as u32)) > ((fl_r as u32))) { crate::simd_ops::ru64(rsp.wrapping_add(0x38u64)) } else { r15 }) as u64; fl_l = rbx; fl_r = (0x20u64) as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 7; continue; } fl_l = rbx; fl_r = (0x10u64) as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 6; continue; } fl_l = rbx; fl_r = (0x8u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 3; continue; } r8 = (0x9ae16a3b2f90404fu64) as u64; r9 = ((r8.wrapping_add(rbx.wrapping_mul(2))) as u64) as u64; r8 = (crate::simd_ops::ru64(r15)) as u64; rax = (0x651e95c4d06fbfb1u64) as u64; r8 = ((r8).wrapping_sub(rax)) as u64; rdx = (crate::simd_ops::ru64(rbx.wrapping_add(r15).wrapping_sub(0x8u64))) as u64; rdx = ((rdx).rotate_right(37u32)) as u64; rdx = (((rdx) as i64).wrapping_mul((r9) as i64) as u64) as u64; rdx = ((rdx).wrapping_add(r8)) as u64; r8 = ((r8).rotate_right(25u32)) as u64; r8 = ((r8).wrapping_add(crate::simd_ops::ru64(rbx.wrapping_add(r15).wrapping_sub(0x8u64)))) as u64; r8 = (((r8) as i64).wrapping_mul((r9) as i64) as u64) as u64; rcx = r8; rcx = (((rcx) ^ (rdx))) as u64; rcx = (((rcx) as i64).wrapping_mul((r9) as i64) as u64) as u64; rax = rcx; rax = ((rax).wrapping_shr(47u32)) as u64; rcx = (((rcx) ^ (rax))) as u64; rcx = (((rcx) ^ (r8))) as u64; rcx = (((rcx) as i64).wrapping_mul((r9) as i64) as u64) as u64; rax = rcx; rax = ((rax).wrapping_shr(47u32)) as u64; rcx = (((rcx) ^ (rax))) as u64; rcx = (((rcx) as i64).wrapping_mul((r9) as i64) as u64) as u64; bb = 10; continue; }
            3 => { fl_l = rbx; fl_r = (0x4u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 4; continue; } r8 = (0x9ae16a3b2f90404fu64) as u64; r8 = ((r8.wrapping_add(rbx.wrapping_mul(2))) as u64) as u64; rax = (crate::simd_ops::ru32(r15)) as u32 as u64; rdx = (crate::simd_ops::ru32(rbx.wrapping_add(r15).wrapping_sub(0x4u64))) as u32 as u64; rcx = ((rbx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rcx = (((rcx) ^ (rdx))) as u64; rcx = (((rcx) as i64).wrapping_mul((r8) as i64) as u64) as u64; rax = rcx; rax = ((rax).wrapping_shr(47u32)) as u64; rcx = (((rcx) ^ (rax))) as u64; rcx = (((rcx) ^ (rdx))) as u64; rcx = (((rcx) as i64).wrapping_mul((r8) as i64) as u64) as u64; rax = rcx; rax = ((rax).wrapping_shr(47u32)) as u64; rcx = (((rcx) ^ (rax))) as u64; rcx = (((rcx) as i64).wrapping_mul((r8) as i64) as u64) as u64; bb = 10; continue; }
            4 => { fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } rax = rbx; rax = ((rax).wrapping_shr(1u32)) as u64; rdx = ((crate::simd_ops::ru8(rax.wrapping_add(r15))) as u32) as u32 as u64; rax = (0xe16a3b2f90404f00u64) as u64; rdx = (((rdx) as i64).wrapping_mul((rax) as i64) as u64) as u64; rax = ((crate::simd_ops::ru8(r15)) as u32) as u32 as u64; r8 = (0x9ae16a3b2f90404fu64) as u64; rax = (((rax) as i64).wrapping_mul((r8) as i64) as u64) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; rax = ((crate::simd_ops::ru8(rbx.wrapping_add(r15).wrapping_sub(0x1u64))) as u32) as u32 as u64; rax = ((rbx.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = ((rax as u32)) as u32 as u64; rcx = (0xc3a5c85c97cb3127u64) as u64; rax = (((rax) as i64).wrapping_mul((rcx) as i64) as u64) as u64; rdx = (((rdx) ^ (rax))) as u64; rcx = rdx; rcx = ((rcx).wrapping_shr(47u32)) as u64; rcx = (((rcx) ^ (rdx))) as u64; rcx = (((rcx) as i64).wrapping_mul((r8) as i64) as u64) as u64; bb = 10; continue; }
            5 => { rcx = (0x9ae16a3b2f90404fu64) as u64; bb = 10; continue; }
            6 => { r8 = (0x9ae16a3b2f90404fu64) as u64; r11 = ((r8.wrapping_add(rbx.wrapping_mul(2))) as u64) as u64; r12 = (0xb492b66fbe98f273u64) as u64; r9 = (crate::simd_ops::ru64(r15)) as u64; r9 = (((r9) as i64).wrapping_mul((r12) as i64) as u64) as u64; rdx = (crate::simd_ops::ru64(r15.wrapping_add(0x8u64))) as u64; r10 = r11; r10 = (((r10) as i64).wrapping_mul((crate::simd_ops::ru64(rbx.wrapping_add(r15).wrapping_sub(0x8u64))) as i64) as u64) as u64; rcx = (crate::simd_ops::ru64(rbx.wrapping_add(r15).wrapping_sub(0x10u64))) as u64; rcx = (((rcx) as i64).wrapping_mul((r8) as i64) as u64) as u64; r8 = ((r8).wrapping_add(rdx)) as u64; r8 = ((r8).rotate_right(18u32)) as u64; r8 = ((r8).wrapping_add(r10)) as u64; r8 = ((r8).wrapping_add(r9)) as u64; rax = ((rdx.wrapping_add(r9)) as u64) as u64; rax = ((rax).rotate_right(43u32)) as u64; r10 = ((r10).rotate_right(30u32)) as u64; rcx = ((rcx).wrapping_add(rax)) as u64; rcx = ((rcx).wrapping_add(r10)) as u64; rcx = (((rcx) ^ (r8))) as u64; rcx = (((rcx) as i64).wrapping_mul((r11) as i64) as u64) as u64; rax = rcx; rax = ((rax).wrapping_shr(47u32)) as u64; rcx = (((rcx) ^ (rax))) as u64; rcx = (((rcx) ^ (r8))) as u64; rcx = (((rcx) as i64).wrapping_mul((r11) as i64) as u64) as u64; rax = rcx; rax = ((rax).wrapping_shr(47u32)) as u64; rcx = (((rcx) ^ (rax))) as u64; rcx = (((rcx) as i64).wrapping_mul((r11) as i64) as u64) as u64; bb = 10; continue; }
            7 => { r13 = ((rbx.wrapping_add(r15)) as u64) as u64; fl_l = rbx; fl_r = (0x40u64) as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 8; continue; } r8 = (0x9ae16a3b2f90404fu64) as u64; r14 = ((r8.wrapping_add(rbx.wrapping_mul(2))) as u64) as u64; r11 = (crate::simd_ops::ru64(r13.wrapping_sub(0x18u64))) as u64; r10 = (crate::simd_ops::ru64(r15.wrapping_add(0x10u64))) as u64; r10 = (((r10) as i64).wrapping_mul((r8) as i64) as u64) as u64; rax = (crate::simd_ops::ru64(r15.wrapping_add(0x18u64))) as u64; r9 = ((rax.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rdi = r14; rdi = (((rdi) as i64).wrapping_mul((crate::simd_ops::ru64(r13.wrapping_sub(0x10u64))) as i64) as u64) as u64; rdx = (crate::simd_ops::ru64(r15)) as u64; rdx = (((rdx) as i64).wrapping_mul((r8) as i64) as u64) as u64; rdx = ((rdx).wrapping_add(crate::simd_ops::ru64(r13.wrapping_sub(0x8u64)))) as u64; rax = (crate::simd_ops::ru64(r15.wrapping_add(0x8u64))) as u64; rax = ((rax).rotate_right(30u32)) as u64; rax = ((rax).wrapping_add(r11)) as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rax = rdx; rax = ((rax).rotate_right(43u32)) as u64; rcx = ((rcx).wrapping_add(rax)) as u64; r8 = (crate::simd_ops::ru64(r13.wrapping_sub(0x20u64))) as u64; r8 = (((r8) ^ (rdx))) as u64; r8 = ((r8).wrapping_add(1)) as u64; r8 = ((r8).wrapping_add(r9)) as u64; rdx = ((r8.wrapping_add(rcx)) as u64) as u64; rdx = (((rdx) as i64).wrapping_mul((r14) as i64) as u64) as u64; rdx = ((rdx).swap_bytes()) as u64; rdx = ((rdx).wrapping_add(rdi)) as u64; rax = ((r9.wrapping_add(r10)) as u64) as u64; r9 = rax; r9 = ((r9).rotate_right(42u32)) as u64; r9 = ((r9).wrapping_add(r11)) as u64; rcx = ((rax.wrapping_add(r11)) as u64) as u64; rax = ((rdx.wrapping_add(r8)) as u64) as u64; rax = (((rax) as i64).wrapping_mul((r14) as i64) as u64) as u64; rax = ((rax).swap_bytes()) as u64; rax = ((rax).wrapping_add(rcx)) as u64; rax = ((rax).wrapping_add(r9)) as u64; rax = ((rax).wrapping_add(crate::simd_ops::ru64(r13.wrapping_sub(0x8u64)))) as u64; rax = (((rax) as i64).wrapping_mul((r14) as i64) as u64) as u64; rax = ((rax).swap_bytes()) as u64; rax = ((rax).wrapping_add(crate::simd_ops::ru64(r15.wrapping_add(0x8u64)))) as u64; rcx = ((rcx).wrapping_add(rax)) as u64; rcx = (((rcx) as i64).wrapping_mul((r14) as i64) as u64) as u64; rcx = ((rcx).wrapping_add(rdi)) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(r13.wrapping_sub(0x20u64)))) as u64; rax = rcx; rax = ((rax).wrapping_shr(47u32)) as u64; rcx = (((rcx) ^ (rax))) as u64; rcx = (((rcx) as i64).wrapping_mul((r14) as i64) as u64) as u64; fl_l = ((rcx).wrapping_add(r9)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; bb = 10; continue; }
            8 => { rdi = (crate::simd_ops::ru64(r13.wrapping_sub(0x28u64))) as u64; rbp = (crate::simd_ops::ru64(r13.wrapping_sub(0x38u64))) as u64; rbp = ((rbp).wrapping_add(crate::simd_ops::ru64(r13.wrapping_sub(0x10u64)))) as u64; r14 = (crate::simd_ops::ru64(r13.wrapping_sub(0x30u64))) as u64; r14 = ((r14).wrapping_add(rbx)) as u64; r14 = (((r14) ^ (crate::simd_ops::ru64(r13.wrapping_sub(0x18u64))))) as u64; rdx = (0x9ddfea08eb382d69u64) as u64; r14 = (((r14) as i64).wrapping_mul((rdx) as i64) as u64) as u64; rax = r14; rax = ((rax).wrapping_shr(47u32)) as u64; r14 = (((r14) ^ (rax))) as u64; r14 = (((r14) ^ (crate::simd_ops::ru64(r13.wrapping_sub(0x18u64))))) as u64; r14 = (((r14) as i64).wrapping_mul((rdx) as i64) as u64) as u64; rax = r14; rax = ((rax).wrapping_shr(47u32)) as u64; r14 = (((r14) ^ (rax))) as u64; r14 = (((r14) as i64).wrapping_mul((rdx) as i64) as u64) as u64; rax = (crate::simd_ops::ru64(r13.wrapping_sub(0x30u64))) as u64; r10 = (crate::simd_ops::ru64(r13.wrapping_sub(0x40u64))) as u64; r10 = ((r10).wrapping_add(rbx)) as u64; r9 = ((rdi.wrapping_add(r10)) as u64) as u64; r9 = ((r9).wrapping_add(r14)) as u64; r8 = r10; rax = ((rax).wrapping_add(crate::simd_ops::ru64(r13.wrapping_sub(0x38u64)))) as u64; r10 = ((r10).wrapping_add(rax)) as u64; r9 = ((r9).rotate_right(21u32)) as u64; rax = r10; rax = ((rax).rotate_right(44u32)) as u64; rax = ((rax).wrapping_add(r9)) as u64; rax = ((rax).wrapping_add(r8)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xc0u64), rax); r9 = ((rdi.wrapping_add(r10)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xb8u64), r9); r11 = (crate::simd_ops::ru64(r13.wrapping_sub(0x8u64))) as u64; rax = (crate::simd_ops::ru64(r13.wrapping_sub(0x10u64))) as u64; rcx = (crate::simd_ops::ru64(r13.wrapping_sub(0x18u64))) as u64; r10 = (crate::simd_ops::ru64(r13.wrapping_sub(0x20u64))) as u64; r12 = (0xb492b66fbe98f273u64) as u64; r10 = ((r10).wrapping_add(r12)) as u64; r10 = ((r10).wrapping_add(rbp)) as u64; r9 = ((r11.wrapping_add(r10)) as u64) as u64; r9 = ((r9).wrapping_add(rdi)) as u64; r13 = r10; rax = ((rax).wrapping_add(rcx)) as u64; r10 = ((r10).wrapping_add(rax)) as u64; r9 = ((r9).rotate_right(21u32)) as u64; rax = r10; rax = ((rax).rotate_right(44u32)) as u64; r9 = ((r9).wrapping_add(rax)) as u64; r13 = ((r13).wrapping_add(r9)) as u64; r8 = ((r11.wrapping_add(r10)) as u64) as u64; rdi = (((rdi) as i64).wrapping_mul((r12) as i64) as u64) as u64; rdi = ((rdi).wrapping_add(crate::simd_ops::ru64(r15))) as u64; rbx = ((rbx).wrapping_sub(1)) as u64; fl_l = (((rbx) & (0xffffffffffffffc0u64))) as u64; fl_r = fl_l; rbx = ((fl_l as u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xc8u64), rbx); rax = r15.wrapping_add(0x30); crate::simd_ops::wu64(rsp.wrapping_add(0xb0u64), rax); r15 = rdi; bb = 9; continue; }
            9 => { r11 = (crate::simd_ops::ru64(rax.wrapping_sub(0x28u64))) as u64; rsi = ((r11.wrapping_add(rbp)) as u64) as u64; rsi = ((rsi).wrapping_add(r15)) as u64; rsi = ((rsi).wrapping_add(crate::simd_ops::ru64(rsp.wrapping_add(0xb8u64)))) as u64; rdi = (crate::simd_ops::ru64(rax)) as u64; rbp = ((rbp).wrapping_add(rdi)) as u64; rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0xc0u64))) as u64; rbp = ((rbp).wrapping_add(rdx)) as u64; rsi = ((rsi).rotate_right(37u32)) as u64; rsi = (((rsi) as i64).wrapping_mul((r12) as i64) as u64) as u64; rsi = (((rsi) ^ (r13))) as u64; rbx = (crate::simd_ops::ru64(rax.wrapping_sub(0x8u64))) as u64; rbp = ((rbp).rotate_right(42u32)) as u64; rbp = (((rbp) as i64).wrapping_mul((r12) as i64) as u64) as u64; rbp = ((rbp).wrapping_add(rbx)) as u64; rbp = ((rbp).wrapping_add(crate::simd_ops::ru64(rsp.wrapping_add(0xb8u64)))) as u64; r15 = ((r14.wrapping_add(r8)) as u64) as u64; r15 = ((r15).rotate_right(33u32)) as u64; r15 = (((r15) as i64).wrapping_mul((r12) as i64) as u64) as u64; rcx = ((rsi.wrapping_add(r8)) as u64) as u64; rdx = (((rdx) as i64).wrapping_mul((r12) as i64) as u64) as u64; r9 = (crate::simd_ops::ru64(rax.wrapping_sub(0x18u64))) as u64; r10 = (crate::simd_ops::ru64(rax.wrapping_sub(0x20u64))) as u64; r8 = (crate::simd_ops::ru64(rax.wrapping_sub(0x30u64))) as u64; r8 = ((r8).wrapping_add(rdx)) as u64; rdx = ((r9.wrapping_add(r8)) as u64) as u64; rdx = ((rdx).wrapping_add(rcx)) as u64; rcx = r8; rax = ((r10.wrapping_add(r11)) as u64) as u64; r8 = ((r8).wrapping_add(rax)) as u64; rdx = ((rdx).rotate_right(21u32)) as u64; r11 = r8; r11 = ((r11).rotate_right(44u32)) as u64; r11 = ((r11).wrapping_add(rdx)) as u64; r11 = ((r11).wrapping_add(rcx)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xc0u64), r11); rax = ((r9.wrapping_add(r8)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xb8u64), rax); rdx = ((r10.wrapping_add(rbp)) as u64) as u64; r9 = ((r15.wrapping_add(r13)) as u64) as u64; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xb0u64))) as u64; r10 = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; r9 = ((r9).wrapping_add(crate::simd_ops::ru64(rax.wrapping_sub(0x10u64)))) as u64; r8 = ((r10.wrapping_add(r9)) as u64) as u64; r8 = ((r8).wrapping_add(rdx)) as u64; r13 = r9; rax = ((rdi.wrapping_add(rbx)) as u64) as u64; r9 = ((r9).wrapping_add(rax)) as u64; r8 = ((r8).rotate_right(21u32)) as u64; rax = r9; rax = ((rax).rotate_right(44u32)) as u64; r8 = ((r8).wrapping_add(rax)) as u64; r13 = ((r13).wrapping_add(r8)) as u64; r8 = ((r10.wrapping_add(r9)) as u64) as u64; r14 = rsi; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xb0u64))) as u64; rax = ((rax).wrapping_add(0x40u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xb0u64), rax); fl_l = (crate::simd_ops::ru64(rsp.wrapping_add(0xc8u64))) as u64; fl_r = (0x40u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xc8u64), ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64); if (((fl_l as u64)) != ((fl_r as u64))) { bb = 9; continue; } rcx = r13; rcx = (((rcx) ^ (r11))) as u64; r10 = (0x9ddfea08eb382d69u64) as u64; rcx = (((rcx) as i64).wrapping_mul((r10) as i64) as u64) as u64; rax = rcx; rax = ((rax).wrapping_shr(47u32)) as u64; rcx = (((rcx) ^ (rax))) as u64; r13 = (((r13) ^ (rcx))) as u64; r13 = (((r13) as i64).wrapping_mul((r10) as i64) as u64) as u64; rcx = r13; rcx = ((rcx).wrapping_shr(47u32)) as u64; rcx = (((rcx) ^ (r13))) as u64; rcx = (((rcx) as i64).wrapping_mul((r10) as i64) as u64) as u64; rcx = ((rcx).wrapping_add(r15)) as u64; rdx = r8; rdx = (((rdx) ^ (crate::simd_ops::ru64(rsp.wrapping_add(0xb8u64))))) as u64; rdx = (((rdx) as i64).wrapping_mul((r10) as i64) as u64) as u64; rax = rdx; rax = ((rax).wrapping_shr(47u32)) as u64; rdx = (((rdx) ^ (rax))) as u64; r8 = (((r8) ^ (rdx))) as u64; r8 = (((r8) as i64).wrapping_mul((r10) as i64) as u64) as u64; rax = r8; rax = ((rax).wrapping_shr(47u32)) as u64; rax = (((rax) ^ (r8))) as u64; rax = (((rax) as i64).wrapping_mul((r10) as i64) as u64) as u64; rdx = rbp; rdx = ((rdx).wrapping_shr(47u32)) as u64; rdx = (((rdx) ^ (rbp))) as u64; rdx = (((rdx) as i64).wrapping_mul((r12) as i64) as u64) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; rdx = ((rdx).wrapping_add(rsi)) as u64; rdx = (((rdx) ^ (rcx))) as u64; rdx = (((rdx) as i64).wrapping_mul((r10) as i64) as u64) as u64; rax = rdx; rax = ((rax).wrapping_shr(47u32)) as u64; rdx = (((rdx) ^ (rax))) as u64; rcx = (((rcx) ^ (rdx))) as u64; rcx = (((rcx) as i64).wrapping_mul((r10) as i64) as u64) as u64; rax = rcx; rax = ((rax).wrapping_shr(47u32)) as u64; rcx = (((rcx) ^ (rax))) as u64; rcx = (((rcx) as i64).wrapping_mul((r10) as i64) as u64) as u64; bb = 10; continue; }
            10 => { rax = (0xde5fb9d2630458e9u64) as u64; let prod = (rax as u128).wrapping_mul((rcx) as u128); rax = prod as u64; rdx = (prod >> 64) as u64; r9 = ((rdx.wrapping_add(rax)) as u64) as u64; r8 = rsp.wrapping_add(0x30); rdx = rsp.wrapping_add(0x20); rcx = P2; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; fl_l = (crate::simd_ops::ru8(rsp.wrapping_add(0x28u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 11; continue; } rbx = ((rdi.wrapping_add(rdi.wrapping_mul(2))) as u64) as u64; rbx = ((rbx).wrapping_shl(4u32)) as u64; rbx = ((rbx).wrapping_add(crate::simd_ops::ru64(P4))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xb0u64), rbx); crate::simd_ops::lk_and_u32(rbx, (0xff000004u32) as u32); crate::simd_ops::lk_or_u32(rbx, (0x4u32) as u32); crate::simd_ops::wu8(rbx.wrapping_add(0x3u64), (0x4u8) as u8); crate::simd_ops::wu32(rbx.wrapping_add(0x4u64), (0x0u32) as u32); crate::simd_ops::wu8(rbx.wrapping_add(0x8u64), (0x0u8) as u8); rdx = rsp.wrapping_add(0x30); rcx = rbx; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx.wrapping_add(0x28u64), (0x0u64) as u64); bb = 11; continue; }
            11 => { rax = ((rdi.wrapping_add(rdi.wrapping_mul(2))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rax = ((rax).wrapping_add(crate::simd_ops::ru64(P4))) as u64; rcx = P6; crate::simd_ops::wu64(rax.wrapping_add(0x28u64), rcx); rcx = rsp.wrapping_add(0x30); rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); rax = 0; fl_l = 0; fl_r = 0; rsp = ((rsp).wrapping_add(0x68u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_36629<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 21>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x68u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (0x4000004u64) as u64); crate::simd_ops::wu8(rsp.wrapping_add(0x38u64), (0x0u8) as u8); rax = P0; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (0xcu64) as u64); r9 = 0xc; r8 = 0; fl_l = 0; fl_r = 0; rdx = rsp.wrapping_add(0x20); rcx = rsp.wrapping_add(0x30); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru32(rsp.wrapping_add(0x34u64))) as u32 as u64; rcx = (crate::simd_ops::ru32(rsp.wrapping_add(0x30u64))) as u32 as u64; rax = ((rcx as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_shr(24u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x80; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 1; continue; } rax = ((rax.wrapping_mul(8)) as u32) as u32 as u64; bb = 2; continue; }
            1 => { rax = (((rax as u32)).wrapping_shl(8u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x7c00; rax = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 2; continue; }
            2 => { rcx = ((((rcx as u32)) & (0xffffffu32))) as u32 as u64; rcx = (((rcx as u32)).wrapping_shl(3u32)) as u32 as u64; r15 = rsp.wrapping_add(0x38); fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; r15 = (if (((fl_l as u32)) > ((fl_r as u32))) { crate::simd_ops::ru64(rsp.wrapping_add(0x38u64)) } else { r15 }) as u64; fl_l = rbx; fl_r = (0x20u64) as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 7; continue; } fl_l = rbx; fl_r = (0x10u64) as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 6; continue; } fl_l = rbx; fl_r = (0x8u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 3; continue; } r8 = (0x9ae16a3b2f90404fu64) as u64; r9 = ((r8.wrapping_add(rbx.wrapping_mul(2))) as u64) as u64; r8 = (crate::simd_ops::ru64(r15)) as u64; rax = (0x651e95c4d06fbfb1u64) as u64; r8 = ((r8).wrapping_sub(rax)) as u64; rdx = (crate::simd_ops::ru64(rbx.wrapping_add(r15).wrapping_sub(0x8u64))) as u64; rdx = ((rdx).rotate_right(37u32)) as u64; rdx = (((rdx) as i64).wrapping_mul((r9) as i64) as u64) as u64; rdx = ((rdx).wrapping_add(r8)) as u64; r8 = ((r8).rotate_right(25u32)) as u64; r8 = ((r8).wrapping_add(crate::simd_ops::ru64(rbx.wrapping_add(r15).wrapping_sub(0x8u64)))) as u64; r8 = (((r8) as i64).wrapping_mul((r9) as i64) as u64) as u64; rcx = r8; rcx = (((rcx) ^ (rdx))) as u64; rcx = (((rcx) as i64).wrapping_mul((r9) as i64) as u64) as u64; rax = rcx; rax = ((rax).wrapping_shr(47u32)) as u64; rcx = (((rcx) ^ (rax))) as u64; rcx = (((rcx) ^ (r8))) as u64; rcx = (((rcx) as i64).wrapping_mul((r9) as i64) as u64) as u64; rax = rcx; rax = ((rax).wrapping_shr(47u32)) as u64; rcx = (((rcx) ^ (rax))) as u64; rcx = (((rcx) as i64).wrapping_mul((r9) as i64) as u64) as u64; bb = 10; continue; }
            3 => { fl_l = rbx; fl_r = (0x4u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 4; continue; } r8 = (0x9ae16a3b2f90404fu64) as u64; r8 = ((r8.wrapping_add(rbx.wrapping_mul(2))) as u64) as u64; rax = (crate::simd_ops::ru32(r15)) as u32 as u64; rdx = (crate::simd_ops::ru32(rbx.wrapping_add(r15).wrapping_sub(0x4u64))) as u32 as u64; rcx = ((rbx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rcx = (((rcx) ^ (rdx))) as u64; rcx = (((rcx) as i64).wrapping_mul((r8) as i64) as u64) as u64; rax = rcx; rax = ((rax).wrapping_shr(47u32)) as u64; rcx = (((rcx) ^ (rax))) as u64; rcx = (((rcx) ^ (rdx))) as u64; rcx = (((rcx) as i64).wrapping_mul((r8) as i64) as u64) as u64; rax = rcx; rax = ((rax).wrapping_shr(47u32)) as u64; rcx = (((rcx) ^ (rax))) as u64; rcx = (((rcx) as i64).wrapping_mul((r8) as i64) as u64) as u64; bb = 10; continue; }
            4 => { fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } rax = rbx; rax = ((rax).wrapping_shr(1u32)) as u64; rdx = ((crate::simd_ops::ru8(rax.wrapping_add(r15))) as u32) as u32 as u64; rax = (0xe16a3b2f90404f00u64) as u64; rdx = (((rdx) as i64).wrapping_mul((rax) as i64) as u64) as u64; rax = ((crate::simd_ops::ru8(r15)) as u32) as u32 as u64; r8 = (0x9ae16a3b2f90404fu64) as u64; rax = (((rax) as i64).wrapping_mul((r8) as i64) as u64) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; rax = ((crate::simd_ops::ru8(rbx.wrapping_add(r15).wrapping_sub(0x1u64))) as u32) as u32 as u64; rax = ((rbx.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = ((rax as u32)) as u32 as u64; rcx = (0xc3a5c85c97cb3127u64) as u64; rax = (((rax) as i64).wrapping_mul((rcx) as i64) as u64) as u64; rdx = (((rdx) ^ (rax))) as u64; rcx = rdx; rcx = ((rcx).wrapping_shr(47u32)) as u64; rcx = (((rcx) ^ (rdx))) as u64; rcx = (((rcx) as i64).wrapping_mul((r8) as i64) as u64) as u64; bb = 10; continue; }
            5 => { rcx = (0x9ae16a3b2f90404fu64) as u64; bb = 10; continue; }
            6 => { r8 = (0x9ae16a3b2f90404fu64) as u64; r11 = ((r8.wrapping_add(rbx.wrapping_mul(2))) as u64) as u64; r12 = (0xb492b66fbe98f273u64) as u64; r9 = (crate::simd_ops::ru64(r15)) as u64; r9 = (((r9) as i64).wrapping_mul((r12) as i64) as u64) as u64; rdx = (crate::simd_ops::ru64(r15.wrapping_add(0x8u64))) as u64; r10 = r11; r10 = (((r10) as i64).wrapping_mul((crate::simd_ops::ru64(rbx.wrapping_add(r15).wrapping_sub(0x8u64))) as i64) as u64) as u64; rcx = (crate::simd_ops::ru64(rbx.wrapping_add(r15).wrapping_sub(0x10u64))) as u64; rcx = (((rcx) as i64).wrapping_mul((r8) as i64) as u64) as u64; r8 = ((r8).wrapping_add(rdx)) as u64; r8 = ((r8).rotate_right(18u32)) as u64; r8 = ((r8).wrapping_add(r10)) as u64; r8 = ((r8).wrapping_add(r9)) as u64; rax = ((rdx.wrapping_add(r9)) as u64) as u64; rax = ((rax).rotate_right(43u32)) as u64; r10 = ((r10).rotate_right(30u32)) as u64; rcx = ((rcx).wrapping_add(rax)) as u64; rcx = ((rcx).wrapping_add(r10)) as u64; rcx = (((rcx) ^ (r8))) as u64; rcx = (((rcx) as i64).wrapping_mul((r11) as i64) as u64) as u64; rax = rcx; rax = ((rax).wrapping_shr(47u32)) as u64; rcx = (((rcx) ^ (rax))) as u64; rcx = (((rcx) ^ (r8))) as u64; rcx = (((rcx) as i64).wrapping_mul((r11) as i64) as u64) as u64; rax = rcx; rax = ((rax).wrapping_shr(47u32)) as u64; rcx = (((rcx) ^ (rax))) as u64; rcx = (((rcx) as i64).wrapping_mul((r11) as i64) as u64) as u64; bb = 10; continue; }
            7 => { r13 = ((rbx.wrapping_add(r15)) as u64) as u64; fl_l = rbx; fl_r = (0x40u64) as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 8; continue; } r8 = (0x9ae16a3b2f90404fu64) as u64; r14 = ((r8.wrapping_add(rbx.wrapping_mul(2))) as u64) as u64; r11 = (crate::simd_ops::ru64(r13.wrapping_sub(0x18u64))) as u64; r10 = (crate::simd_ops::ru64(r15.wrapping_add(0x10u64))) as u64; r10 = (((r10) as i64).wrapping_mul((r8) as i64) as u64) as u64; rax = (crate::simd_ops::ru64(r15.wrapping_add(0x18u64))) as u64; r9 = ((rax.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rdi = r14; rdi = (((rdi) as i64).wrapping_mul((crate::simd_ops::ru64(r13.wrapping_sub(0x10u64))) as i64) as u64) as u64; rdx = (crate::simd_ops::ru64(r15)) as u64; rdx = (((rdx) as i64).wrapping_mul((r8) as i64) as u64) as u64; rdx = ((rdx).wrapping_add(crate::simd_ops::ru64(r13.wrapping_sub(0x8u64)))) as u64; rax = (crate::simd_ops::ru64(r15.wrapping_add(0x8u64))) as u64; rax = ((rax).rotate_right(30u32)) as u64; rax = ((rax).wrapping_add(r11)) as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rax = rdx; rax = ((rax).rotate_right(43u32)) as u64; rcx = ((rcx).wrapping_add(rax)) as u64; r8 = (crate::simd_ops::ru64(r13.wrapping_sub(0x20u64))) as u64; r8 = (((r8) ^ (rdx))) as u64; r8 = ((r8).wrapping_add(1)) as u64; r8 = ((r8).wrapping_add(r9)) as u64; rdx = ((r8.wrapping_add(rcx)) as u64) as u64; rdx = (((rdx) as i64).wrapping_mul((r14) as i64) as u64) as u64; rdx = ((rdx).swap_bytes()) as u64; rdx = ((rdx).wrapping_add(rdi)) as u64; rax = ((r9.wrapping_add(r10)) as u64) as u64; r9 = rax; r9 = ((r9).rotate_right(42u32)) as u64; r9 = ((r9).wrapping_add(r11)) as u64; rcx = ((rax.wrapping_add(r11)) as u64) as u64; rax = ((rdx.wrapping_add(r8)) as u64) as u64; rax = (((rax) as i64).wrapping_mul((r14) as i64) as u64) as u64; rax = ((rax).swap_bytes()) as u64; rax = ((rax).wrapping_add(rcx)) as u64; rax = ((rax).wrapping_add(r9)) as u64; rax = ((rax).wrapping_add(crate::simd_ops::ru64(r13.wrapping_sub(0x8u64)))) as u64; rax = (((rax) as i64).wrapping_mul((r14) as i64) as u64) as u64; rax = ((rax).swap_bytes()) as u64; rax = ((rax).wrapping_add(crate::simd_ops::ru64(r15.wrapping_add(0x8u64)))) as u64; rcx = ((rcx).wrapping_add(rax)) as u64; rcx = (((rcx) as i64).wrapping_mul((r14) as i64) as u64) as u64; rcx = ((rcx).wrapping_add(rdi)) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(r13.wrapping_sub(0x20u64)))) as u64; rax = rcx; rax = ((rax).wrapping_shr(47u32)) as u64; rcx = (((rcx) ^ (rax))) as u64; rcx = (((rcx) as i64).wrapping_mul((r14) as i64) as u64) as u64; fl_l = ((rcx).wrapping_add(r9)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; bb = 10; continue; }
            8 => { rdi = (crate::simd_ops::ru64(r13.wrapping_sub(0x28u64))) as u64; rbp = (crate::simd_ops::ru64(r13.wrapping_sub(0x38u64))) as u64; rbp = ((rbp).wrapping_add(crate::simd_ops::ru64(r13.wrapping_sub(0x10u64)))) as u64; r14 = (crate::simd_ops::ru64(r13.wrapping_sub(0x30u64))) as u64; r14 = ((r14).wrapping_add(rbx)) as u64; r14 = (((r14) ^ (crate::simd_ops::ru64(r13.wrapping_sub(0x18u64))))) as u64; rdx = (0x9ddfea08eb382d69u64) as u64; r14 = (((r14) as i64).wrapping_mul((rdx) as i64) as u64) as u64; rax = r14; rax = ((rax).wrapping_shr(47u32)) as u64; r14 = (((r14) ^ (rax))) as u64; r14 = (((r14) ^ (crate::simd_ops::ru64(r13.wrapping_sub(0x18u64))))) as u64; r14 = (((r14) as i64).wrapping_mul((rdx) as i64) as u64) as u64; rax = r14; rax = ((rax).wrapping_shr(47u32)) as u64; r14 = (((r14) ^ (rax))) as u64; r14 = (((r14) as i64).wrapping_mul((rdx) as i64) as u64) as u64; rax = (crate::simd_ops::ru64(r13.wrapping_sub(0x30u64))) as u64; r10 = (crate::simd_ops::ru64(r13.wrapping_sub(0x40u64))) as u64; r10 = ((r10).wrapping_add(rbx)) as u64; r9 = ((rdi.wrapping_add(r10)) as u64) as u64; r9 = ((r9).wrapping_add(r14)) as u64; r8 = r10; rax = ((rax).wrapping_add(crate::simd_ops::ru64(r13.wrapping_sub(0x38u64)))) as u64; r10 = ((r10).wrapping_add(rax)) as u64; r9 = ((r9).rotate_right(21u32)) as u64; rax = r10; rax = ((rax).rotate_right(44u32)) as u64; rax = ((rax).wrapping_add(r9)) as u64; rax = ((rax).wrapping_add(r8)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xc0u64), rax); r9 = ((rdi.wrapping_add(r10)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xb8u64), r9); r11 = (crate::simd_ops::ru64(r13.wrapping_sub(0x8u64))) as u64; rax = (crate::simd_ops::ru64(r13.wrapping_sub(0x10u64))) as u64; rcx = (crate::simd_ops::ru64(r13.wrapping_sub(0x18u64))) as u64; r10 = (crate::simd_ops::ru64(r13.wrapping_sub(0x20u64))) as u64; r12 = (0xb492b66fbe98f273u64) as u64; r10 = ((r10).wrapping_add(r12)) as u64; r10 = ((r10).wrapping_add(rbp)) as u64; r9 = ((r11.wrapping_add(r10)) as u64) as u64; r9 = ((r9).wrapping_add(rdi)) as u64; r13 = r10; rax = ((rax).wrapping_add(rcx)) as u64; r10 = ((r10).wrapping_add(rax)) as u64; r9 = ((r9).rotate_right(21u32)) as u64; rax = r10; rax = ((rax).rotate_right(44u32)) as u64; r9 = ((r9).wrapping_add(rax)) as u64; r13 = ((r13).wrapping_add(r9)) as u64; r8 = ((r11.wrapping_add(r10)) as u64) as u64; rdi = (((rdi) as i64).wrapping_mul((r12) as i64) as u64) as u64; rdi = ((rdi).wrapping_add(crate::simd_ops::ru64(r15))) as u64; rbx = ((rbx).wrapping_sub(1)) as u64; fl_l = (((rbx) & (0xffffffffffffffc0u64))) as u64; fl_r = fl_l; rbx = ((fl_l as u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xc8u64), rbx); rax = r15.wrapping_add(0x30); crate::simd_ops::wu64(rsp.wrapping_add(0xb0u64), rax); r15 = rdi; bb = 9; continue; }
            9 => { r11 = (crate::simd_ops::ru64(rax.wrapping_sub(0x28u64))) as u64; rsi = ((r11.wrapping_add(rbp)) as u64) as u64; rsi = ((rsi).wrapping_add(r15)) as u64; rsi = ((rsi).wrapping_add(crate::simd_ops::ru64(rsp.wrapping_add(0xb8u64)))) as u64; rdi = (crate::simd_ops::ru64(rax)) as u64; rbp = ((rbp).wrapping_add(rdi)) as u64; rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0xc0u64))) as u64; rbp = ((rbp).wrapping_add(rdx)) as u64; rsi = ((rsi).rotate_right(37u32)) as u64; rsi = (((rsi) as i64).wrapping_mul((r12) as i64) as u64) as u64; rsi = (((rsi) ^ (r13))) as u64; rbx = (crate::simd_ops::ru64(rax.wrapping_sub(0x8u64))) as u64; rbp = ((rbp).rotate_right(42u32)) as u64; rbp = (((rbp) as i64).wrapping_mul((r12) as i64) as u64) as u64; rbp = ((rbp).wrapping_add(rbx)) as u64; rbp = ((rbp).wrapping_add(crate::simd_ops::ru64(rsp.wrapping_add(0xb8u64)))) as u64; r15 = ((r14.wrapping_add(r8)) as u64) as u64; r15 = ((r15).rotate_right(33u32)) as u64; r15 = (((r15) as i64).wrapping_mul((r12) as i64) as u64) as u64; rcx = ((rsi.wrapping_add(r8)) as u64) as u64; rdx = (((rdx) as i64).wrapping_mul((r12) as i64) as u64) as u64; r9 = (crate::simd_ops::ru64(rax.wrapping_sub(0x18u64))) as u64; r10 = (crate::simd_ops::ru64(rax.wrapping_sub(0x20u64))) as u64; r8 = (crate::simd_ops::ru64(rax.wrapping_sub(0x30u64))) as u64; r8 = ((r8).wrapping_add(rdx)) as u64; rdx = ((r9.wrapping_add(r8)) as u64) as u64; rdx = ((rdx).wrapping_add(rcx)) as u64; rcx = r8; rax = ((r10.wrapping_add(r11)) as u64) as u64; r8 = ((r8).wrapping_add(rax)) as u64; rdx = ((rdx).rotate_right(21u32)) as u64; r11 = r8; r11 = ((r11).rotate_right(44u32)) as u64; r11 = ((r11).wrapping_add(rdx)) as u64; r11 = ((r11).wrapping_add(rcx)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xc0u64), r11); rax = ((r9.wrapping_add(r8)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xb8u64), rax); rdx = ((r10.wrapping_add(rbp)) as u64) as u64; r9 = ((r15.wrapping_add(r13)) as u64) as u64; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xb0u64))) as u64; r10 = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; r9 = ((r9).wrapping_add(crate::simd_ops::ru64(rax.wrapping_sub(0x10u64)))) as u64; r8 = ((r10.wrapping_add(r9)) as u64) as u64; r8 = ((r8).wrapping_add(rdx)) as u64; r13 = r9; rax = ((rdi.wrapping_add(rbx)) as u64) as u64; r9 = ((r9).wrapping_add(rax)) as u64; r8 = ((r8).rotate_right(21u32)) as u64; rax = r9; rax = ((rax).rotate_right(44u32)) as u64; r8 = ((r8).wrapping_add(rax)) as u64; r13 = ((r13).wrapping_add(r8)) as u64; r8 = ((r10.wrapping_add(r9)) as u64) as u64; r14 = rsi; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xb0u64))) as u64; rax = ((rax).wrapping_add(0x40u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xb0u64), rax); fl_l = (crate::simd_ops::ru64(rsp.wrapping_add(0xc8u64))) as u64; fl_r = (0x40u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xc8u64), ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64); if (((fl_l as u64)) != ((fl_r as u64))) { bb = 9; continue; } rcx = r13; rcx = (((rcx) ^ (r11))) as u64; r10 = (0x9ddfea08eb382d69u64) as u64; rcx = (((rcx) as i64).wrapping_mul((r10) as i64) as u64) as u64; rax = rcx; rax = ((rax).wrapping_shr(47u32)) as u64; rcx = (((rcx) ^ (rax))) as u64; r13 = (((r13) ^ (rcx))) as u64; r13 = (((r13) as i64).wrapping_mul((r10) as i64) as u64) as u64; rcx = r13; rcx = ((rcx).wrapping_shr(47u32)) as u64; rcx = (((rcx) ^ (r13))) as u64; rcx = (((rcx) as i64).wrapping_mul((r10) as i64) as u64) as u64; rcx = ((rcx).wrapping_add(r15)) as u64; rdx = r8; rdx = (((rdx) ^ (crate::simd_ops::ru64(rsp.wrapping_add(0xb8u64))))) as u64; rdx = (((rdx) as i64).wrapping_mul((r10) as i64) as u64) as u64; rax = rdx; rax = ((rax).wrapping_shr(47u32)) as u64; rdx = (((rdx) ^ (rax))) as u64; r8 = (((r8) ^ (rdx))) as u64; r8 = (((r8) as i64).wrapping_mul((r10) as i64) as u64) as u64; rax = r8; rax = ((rax).wrapping_shr(47u32)) as u64; rax = (((rax) ^ (r8))) as u64; rax = (((rax) as i64).wrapping_mul((r10) as i64) as u64) as u64; rdx = rbp; rdx = ((rdx).wrapping_shr(47u32)) as u64; rdx = (((rdx) ^ (rbp))) as u64; rdx = (((rdx) as i64).wrapping_mul((r12) as i64) as u64) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; rdx = ((rdx).wrapping_add(rsi)) as u64; rdx = (((rdx) ^ (rcx))) as u64; rdx = (((rdx) as i64).wrapping_mul((r10) as i64) as u64) as u64; rax = rdx; rax = ((rax).wrapping_shr(47u32)) as u64; rdx = (((rdx) ^ (rax))) as u64; rcx = (((rcx) ^ (rdx))) as u64; rcx = (((rcx) as i64).wrapping_mul((r10) as i64) as u64) as u64; rax = rcx; rax = ((rax).wrapping_shr(47u32)) as u64; rcx = (((rcx) ^ (rax))) as u64; rcx = (((rcx) as i64).wrapping_mul((r10) as i64) as u64) as u64; bb = 10; continue; }
            10 => { rax = (0xde5fb9d2630458e9u64) as u64; let prod = (rax as u128).wrapping_mul((rcx) as u128); rax = prod as u64; rdx = (prod >> 64) as u64; r9 = ((rdx.wrapping_add(rax)) as u64) as u64; r8 = rsp.wrapping_add(0x30); rdx = rsp.wrapping_add(0x20); rcx = P2; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; fl_l = (crate::simd_ops::ru8(rsp.wrapping_add(0x28u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 11; continue; } rbx = ((rdi.wrapping_add(rdi.wrapping_mul(2))) as u64) as u64; rbx = ((rbx).wrapping_shl(4u32)) as u64; rbx = ((rbx).wrapping_add(crate::simd_ops::ru64(P4))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xb0u64), rbx); crate::simd_ops::lk_and_u32(rbx, (0xff000004u32) as u32); crate::simd_ops::lk_or_u32(rbx, (0x4u32) as u32); crate::simd_ops::wu8(rbx.wrapping_add(0x3u64), (0x4u8) as u8); crate::simd_ops::wu32(rbx.wrapping_add(0x4u64), (0x0u32) as u32); crate::simd_ops::wu8(rbx.wrapping_add(0x8u64), (0x0u8) as u8); rdx = rsp.wrapping_add(0x30); rcx = rbx; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx.wrapping_add(0x28u64), (0x0u64) as u64); bb = 11; continue; }
            11 => { rax = ((rdi.wrapping_add(rdi.wrapping_mul(2))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rax = ((rax).wrapping_add(crate::simd_ops::ru64(P4))) as u64; rcx = P6; crate::simd_ops::wu64(rax.wrapping_add(0x28u64), rcx); rcx = rsp.wrapping_add(0x30); rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); rax = 0; fl_l = 0; fl_r = 0; rsp = ((rsp).wrapping_add(0x68u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_36630<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 21>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x68u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (0x4000004u64) as u64); crate::simd_ops::wu8(rsp.wrapping_add(0x38u64), (0x0u8) as u8); rax = P0; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (0xbu64) as u64); r9 = 0xb; r8 = 0; fl_l = 0; fl_r = 0; rdx = rsp.wrapping_add(0x20); rcx = rsp.wrapping_add(0x30); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru32(rsp.wrapping_add(0x34u64))) as u32 as u64; rcx = (crate::simd_ops::ru32(rsp.wrapping_add(0x30u64))) as u32 as u64; rax = ((rcx as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_shr(24u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x80; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 1; continue; } rax = ((rax.wrapping_mul(8)) as u32) as u32 as u64; bb = 2; continue; }
            1 => { rax = (((rax as u32)).wrapping_shl(8u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x7c00; rax = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 2; continue; }
            2 => { rcx = ((((rcx as u32)) & (0xffffffu32))) as u32 as u64; rcx = (((rcx as u32)).wrapping_shl(3u32)) as u32 as u64; r15 = rsp.wrapping_add(0x38); fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; r15 = (if (((fl_l as u32)) > ((fl_r as u32))) { crate::simd_ops::ru64(rsp.wrapping_add(0x38u64)) } else { r15 }) as u64; fl_l = rbx; fl_r = (0x20u64) as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 7; continue; } fl_l = rbx; fl_r = (0x10u64) as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 6; continue; } fl_l = rbx; fl_r = (0x8u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 3; continue; } r8 = (0x9ae16a3b2f90404fu64) as u64; r9 = ((r8.wrapping_add(rbx.wrapping_mul(2))) as u64) as u64; r8 = (crate::simd_ops::ru64(r15)) as u64; rax = (0x651e95c4d06fbfb1u64) as u64; r8 = ((r8).wrapping_sub(rax)) as u64; rdx = (crate::simd_ops::ru64(rbx.wrapping_add(r15).wrapping_sub(0x8u64))) as u64; rdx = ((rdx).rotate_right(37u32)) as u64; rdx = (((rdx) as i64).wrapping_mul((r9) as i64) as u64) as u64; rdx = ((rdx).wrapping_add(r8)) as u64; r8 = ((r8).rotate_right(25u32)) as u64; r8 = ((r8).wrapping_add(crate::simd_ops::ru64(rbx.wrapping_add(r15).wrapping_sub(0x8u64)))) as u64; r8 = (((r8) as i64).wrapping_mul((r9) as i64) as u64) as u64; rcx = r8; rcx = (((rcx) ^ (rdx))) as u64; rcx = (((rcx) as i64).wrapping_mul((r9) as i64) as u64) as u64; rax = rcx; rax = ((rax).wrapping_shr(47u32)) as u64; rcx = (((rcx) ^ (rax))) as u64; rcx = (((rcx) ^ (r8))) as u64; rcx = (((rcx) as i64).wrapping_mul((r9) as i64) as u64) as u64; rax = rcx; rax = ((rax).wrapping_shr(47u32)) as u64; rcx = (((rcx) ^ (rax))) as u64; rcx = (((rcx) as i64).wrapping_mul((r9) as i64) as u64) as u64; bb = 10; continue; }
            3 => { fl_l = rbx; fl_r = (0x4u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 4; continue; } r8 = (0x9ae16a3b2f90404fu64) as u64; r8 = ((r8.wrapping_add(rbx.wrapping_mul(2))) as u64) as u64; rax = (crate::simd_ops::ru32(r15)) as u32 as u64; rdx = (crate::simd_ops::ru32(rbx.wrapping_add(r15).wrapping_sub(0x4u64))) as u32 as u64; rcx = ((rbx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rcx = (((rcx) ^ (rdx))) as u64; rcx = (((rcx) as i64).wrapping_mul((r8) as i64) as u64) as u64; rax = rcx; rax = ((rax).wrapping_shr(47u32)) as u64; rcx = (((rcx) ^ (rax))) as u64; rcx = (((rcx) ^ (rdx))) as u64; rcx = (((rcx) as i64).wrapping_mul((r8) as i64) as u64) as u64; rax = rcx; rax = ((rax).wrapping_shr(47u32)) as u64; rcx = (((rcx) ^ (rax))) as u64; rcx = (((rcx) as i64).wrapping_mul((r8) as i64) as u64) as u64; bb = 10; continue; }
            4 => { fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } rax = rbx; rax = ((rax).wrapping_shr(1u32)) as u64; rdx = ((crate::simd_ops::ru8(rax.wrapping_add(r15))) as u32) as u32 as u64; rax = (0xe16a3b2f90404f00u64) as u64; rdx = (((rdx) as i64).wrapping_mul((rax) as i64) as u64) as u64; rax = ((crate::simd_ops::ru8(r15)) as u32) as u32 as u64; r8 = (0x9ae16a3b2f90404fu64) as u64; rax = (((rax) as i64).wrapping_mul((r8) as i64) as u64) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; rax = ((crate::simd_ops::ru8(rbx.wrapping_add(r15).wrapping_sub(0x1u64))) as u32) as u32 as u64; rax = ((rbx.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = ((rax as u32)) as u32 as u64; rcx = (0xc3a5c85c97cb3127u64) as u64; rax = (((rax) as i64).wrapping_mul((rcx) as i64) as u64) as u64; rdx = (((rdx) ^ (rax))) as u64; rcx = rdx; rcx = ((rcx).wrapping_shr(47u32)) as u64; rcx = (((rcx) ^ (rdx))) as u64; rcx = (((rcx) as i64).wrapping_mul((r8) as i64) as u64) as u64; bb = 10; continue; }
            5 => { rcx = (0x9ae16a3b2f90404fu64) as u64; bb = 10; continue; }
            6 => { r8 = (0x9ae16a3b2f90404fu64) as u64; r11 = ((r8.wrapping_add(rbx.wrapping_mul(2))) as u64) as u64; r12 = (0xb492b66fbe98f273u64) as u64; r9 = (crate::simd_ops::ru64(r15)) as u64; r9 = (((r9) as i64).wrapping_mul((r12) as i64) as u64) as u64; rdx = (crate::simd_ops::ru64(r15.wrapping_add(0x8u64))) as u64; r10 = r11; r10 = (((r10) as i64).wrapping_mul((crate::simd_ops::ru64(rbx.wrapping_add(r15).wrapping_sub(0x8u64))) as i64) as u64) as u64; rcx = (crate::simd_ops::ru64(rbx.wrapping_add(r15).wrapping_sub(0x10u64))) as u64; rcx = (((rcx) as i64).wrapping_mul((r8) as i64) as u64) as u64; r8 = ((r8).wrapping_add(rdx)) as u64; r8 = ((r8).rotate_right(18u32)) as u64; r8 = ((r8).wrapping_add(r10)) as u64; r8 = ((r8).wrapping_add(r9)) as u64; rax = ((rdx.wrapping_add(r9)) as u64) as u64; rax = ((rax).rotate_right(43u32)) as u64; r10 = ((r10).rotate_right(30u32)) as u64; rcx = ((rcx).wrapping_add(rax)) as u64; rcx = ((rcx).wrapping_add(r10)) as u64; rcx = (((rcx) ^ (r8))) as u64; rcx = (((rcx) as i64).wrapping_mul((r11) as i64) as u64) as u64; rax = rcx; rax = ((rax).wrapping_shr(47u32)) as u64; rcx = (((rcx) ^ (rax))) as u64; rcx = (((rcx) ^ (r8))) as u64; rcx = (((rcx) as i64).wrapping_mul((r11) as i64) as u64) as u64; rax = rcx; rax = ((rax).wrapping_shr(47u32)) as u64; rcx = (((rcx) ^ (rax))) as u64; rcx = (((rcx) as i64).wrapping_mul((r11) as i64) as u64) as u64; bb = 10; continue; }
            7 => { r13 = ((rbx.wrapping_add(r15)) as u64) as u64; fl_l = rbx; fl_r = (0x40u64) as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 8; continue; } r8 = (0x9ae16a3b2f90404fu64) as u64; r14 = ((r8.wrapping_add(rbx.wrapping_mul(2))) as u64) as u64; r11 = (crate::simd_ops::ru64(r13.wrapping_sub(0x18u64))) as u64; r10 = (crate::simd_ops::ru64(r15.wrapping_add(0x10u64))) as u64; r10 = (((r10) as i64).wrapping_mul((r8) as i64) as u64) as u64; rax = (crate::simd_ops::ru64(r15.wrapping_add(0x18u64))) as u64; r9 = ((rax.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rdi = r14; rdi = (((rdi) as i64).wrapping_mul((crate::simd_ops::ru64(r13.wrapping_sub(0x10u64))) as i64) as u64) as u64; rdx = (crate::simd_ops::ru64(r15)) as u64; rdx = (((rdx) as i64).wrapping_mul((r8) as i64) as u64) as u64; rdx = ((rdx).wrapping_add(crate::simd_ops::ru64(r13.wrapping_sub(0x8u64)))) as u64; rax = (crate::simd_ops::ru64(r15.wrapping_add(0x8u64))) as u64; rax = ((rax).rotate_right(30u32)) as u64; rax = ((rax).wrapping_add(r11)) as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rax = rdx; rax = ((rax).rotate_right(43u32)) as u64; rcx = ((rcx).wrapping_add(rax)) as u64; r8 = (crate::simd_ops::ru64(r13.wrapping_sub(0x20u64))) as u64; r8 = (((r8) ^ (rdx))) as u64; r8 = ((r8).wrapping_add(1)) as u64; r8 = ((r8).wrapping_add(r9)) as u64; rdx = ((r8.wrapping_add(rcx)) as u64) as u64; rdx = (((rdx) as i64).wrapping_mul((r14) as i64) as u64) as u64; rdx = ((rdx).swap_bytes()) as u64; rdx = ((rdx).wrapping_add(rdi)) as u64; rax = ((r9.wrapping_add(r10)) as u64) as u64; r9 = rax; r9 = ((r9).rotate_right(42u32)) as u64; r9 = ((r9).wrapping_add(r11)) as u64; rcx = ((rax.wrapping_add(r11)) as u64) as u64; rax = ((rdx.wrapping_add(r8)) as u64) as u64; rax = (((rax) as i64).wrapping_mul((r14) as i64) as u64) as u64; rax = ((rax).swap_bytes()) as u64; rax = ((rax).wrapping_add(rcx)) as u64; rax = ((rax).wrapping_add(r9)) as u64; rax = ((rax).wrapping_add(crate::simd_ops::ru64(r13.wrapping_sub(0x8u64)))) as u64; rax = (((rax) as i64).wrapping_mul((r14) as i64) as u64) as u64; rax = ((rax).swap_bytes()) as u64; rax = ((rax).wrapping_add(crate::simd_ops::ru64(r15.wrapping_add(0x8u64)))) as u64; rcx = ((rcx).wrapping_add(rax)) as u64; rcx = (((rcx) as i64).wrapping_mul((r14) as i64) as u64) as u64; rcx = ((rcx).wrapping_add(rdi)) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(r13.wrapping_sub(0x20u64)))) as u64; rax = rcx; rax = ((rax).wrapping_shr(47u32)) as u64; rcx = (((rcx) ^ (rax))) as u64; rcx = (((rcx) as i64).wrapping_mul((r14) as i64) as u64) as u64; fl_l = ((rcx).wrapping_add(r9)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; bb = 10; continue; }
            8 => { rdi = (crate::simd_ops::ru64(r13.wrapping_sub(0x28u64))) as u64; rbp = (crate::simd_ops::ru64(r13.wrapping_sub(0x38u64))) as u64; rbp = ((rbp).wrapping_add(crate::simd_ops::ru64(r13.wrapping_sub(0x10u64)))) as u64; r14 = (crate::simd_ops::ru64(r13.wrapping_sub(0x30u64))) as u64; r14 = ((r14).wrapping_add(rbx)) as u64; r14 = (((r14) ^ (crate::simd_ops::ru64(r13.wrapping_sub(0x18u64))))) as u64; rdx = (0x9ddfea08eb382d69u64) as u64; r14 = (((r14) as i64).wrapping_mul((rdx) as i64) as u64) as u64; rax = r14; rax = ((rax).wrapping_shr(47u32)) as u64; r14 = (((r14) ^ (rax))) as u64; r14 = (((r14) ^ (crate::simd_ops::ru64(r13.wrapping_sub(0x18u64))))) as u64; r14 = (((r14) as i64).wrapping_mul((rdx) as i64) as u64) as u64; rax = r14; rax = ((rax).wrapping_shr(47u32)) as u64; r14 = (((r14) ^ (rax))) as u64; r14 = (((r14) as i64).wrapping_mul((rdx) as i64) as u64) as u64; rax = (crate::simd_ops::ru64(r13.wrapping_sub(0x30u64))) as u64; r10 = (crate::simd_ops::ru64(r13.wrapping_sub(0x40u64))) as u64; r10 = ((r10).wrapping_add(rbx)) as u64; r9 = ((rdi.wrapping_add(r10)) as u64) as u64; r9 = ((r9).wrapping_add(r14)) as u64; r8 = r10; rax = ((rax).wrapping_add(crate::simd_ops::ru64(r13.wrapping_sub(0x38u64)))) as u64; r10 = ((r10).wrapping_add(rax)) as u64; r9 = ((r9).rotate_right(21u32)) as u64; rax = r10; rax = ((rax).rotate_right(44u32)) as u64; rax = ((rax).wrapping_add(r9)) as u64; rax = ((rax).wrapping_add(r8)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xc0u64), rax); r9 = ((rdi.wrapping_add(r10)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xb8u64), r9); r11 = (crate::simd_ops::ru64(r13.wrapping_sub(0x8u64))) as u64; rax = (crate::simd_ops::ru64(r13.wrapping_sub(0x10u64))) as u64; rcx = (crate::simd_ops::ru64(r13.wrapping_sub(0x18u64))) as u64; r10 = (crate::simd_ops::ru64(r13.wrapping_sub(0x20u64))) as u64; r12 = (0xb492b66fbe98f273u64) as u64; r10 = ((r10).wrapping_add(r12)) as u64; r10 = ((r10).wrapping_add(rbp)) as u64; r9 = ((r11.wrapping_add(r10)) as u64) as u64; r9 = ((r9).wrapping_add(rdi)) as u64; r13 = r10; rax = ((rax).wrapping_add(rcx)) as u64; r10 = ((r10).wrapping_add(rax)) as u64; r9 = ((r9).rotate_right(21u32)) as u64; rax = r10; rax = ((rax).rotate_right(44u32)) as u64; r9 = ((r9).wrapping_add(rax)) as u64; r13 = ((r13).wrapping_add(r9)) as u64; r8 = ((r11.wrapping_add(r10)) as u64) as u64; rdi = (((rdi) as i64).wrapping_mul((r12) as i64) as u64) as u64; rdi = ((rdi).wrapping_add(crate::simd_ops::ru64(r15))) as u64; rbx = ((rbx).wrapping_sub(1)) as u64; fl_l = (((rbx) & (0xffffffffffffffc0u64))) as u64; fl_r = fl_l; rbx = ((fl_l as u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xc8u64), rbx); rax = r15.wrapping_add(0x30); crate::simd_ops::wu64(rsp.wrapping_add(0xb0u64), rax); r15 = rdi; bb = 9; continue; }
            9 => { r11 = (crate::simd_ops::ru64(rax.wrapping_sub(0x28u64))) as u64; rsi = ((r11.wrapping_add(rbp)) as u64) as u64; rsi = ((rsi).wrapping_add(r15)) as u64; rsi = ((rsi).wrapping_add(crate::simd_ops::ru64(rsp.wrapping_add(0xb8u64)))) as u64; rdi = (crate::simd_ops::ru64(rax)) as u64; rbp = ((rbp).wrapping_add(rdi)) as u64; rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0xc0u64))) as u64; rbp = ((rbp).wrapping_add(rdx)) as u64; rsi = ((rsi).rotate_right(37u32)) as u64; rsi = (((rsi) as i64).wrapping_mul((r12) as i64) as u64) as u64; rsi = (((rsi) ^ (r13))) as u64; rbx = (crate::simd_ops::ru64(rax.wrapping_sub(0x8u64))) as u64; rbp = ((rbp).rotate_right(42u32)) as u64; rbp = (((rbp) as i64).wrapping_mul((r12) as i64) as u64) as u64; rbp = ((rbp).wrapping_add(rbx)) as u64; rbp = ((rbp).wrapping_add(crate::simd_ops::ru64(rsp.wrapping_add(0xb8u64)))) as u64; r15 = ((r14.wrapping_add(r8)) as u64) as u64; r15 = ((r15).rotate_right(33u32)) as u64; r15 = (((r15) as i64).wrapping_mul((r12) as i64) as u64) as u64; rcx = ((rsi.wrapping_add(r8)) as u64) as u64; rdx = (((rdx) as i64).wrapping_mul((r12) as i64) as u64) as u64; r9 = (crate::simd_ops::ru64(rax.wrapping_sub(0x18u64))) as u64; r10 = (crate::simd_ops::ru64(rax.wrapping_sub(0x20u64))) as u64; r8 = (crate::simd_ops::ru64(rax.wrapping_sub(0x30u64))) as u64; r8 = ((r8).wrapping_add(rdx)) as u64; rdx = ((r9.wrapping_add(r8)) as u64) as u64; rdx = ((rdx).wrapping_add(rcx)) as u64; rcx = r8; rax = ((r10.wrapping_add(r11)) as u64) as u64; r8 = ((r8).wrapping_add(rax)) as u64; rdx = ((rdx).rotate_right(21u32)) as u64; r11 = r8; r11 = ((r11).rotate_right(44u32)) as u64; r11 = ((r11).wrapping_add(rdx)) as u64; r11 = ((r11).wrapping_add(rcx)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xc0u64), r11); rax = ((r9.wrapping_add(r8)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xb8u64), rax); rdx = ((r10.wrapping_add(rbp)) as u64) as u64; r9 = ((r15.wrapping_add(r13)) as u64) as u64; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xb0u64))) as u64; r10 = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; r9 = ((r9).wrapping_add(crate::simd_ops::ru64(rax.wrapping_sub(0x10u64)))) as u64; r8 = ((r10.wrapping_add(r9)) as u64) as u64; r8 = ((r8).wrapping_add(rdx)) as u64; r13 = r9; rax = ((rdi.wrapping_add(rbx)) as u64) as u64; r9 = ((r9).wrapping_add(rax)) as u64; r8 = ((r8).rotate_right(21u32)) as u64; rax = r9; rax = ((rax).rotate_right(44u32)) as u64; r8 = ((r8).wrapping_add(rax)) as u64; r13 = ((r13).wrapping_add(r8)) as u64; r8 = ((r10.wrapping_add(r9)) as u64) as u64; r14 = rsi; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xb0u64))) as u64; rax = ((rax).wrapping_add(0x40u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xb0u64), rax); fl_l = (crate::simd_ops::ru64(rsp.wrapping_add(0xc8u64))) as u64; fl_r = (0x40u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xc8u64), ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64); if (((fl_l as u64)) != ((fl_r as u64))) { bb = 9; continue; } rcx = r13; rcx = (((rcx) ^ (r11))) as u64; r10 = (0x9ddfea08eb382d69u64) as u64; rcx = (((rcx) as i64).wrapping_mul((r10) as i64) as u64) as u64; rax = rcx; rax = ((rax).wrapping_shr(47u32)) as u64; rcx = (((rcx) ^ (rax))) as u64; r13 = (((r13) ^ (rcx))) as u64; r13 = (((r13) as i64).wrapping_mul((r10) as i64) as u64) as u64; rcx = r13; rcx = ((rcx).wrapping_shr(47u32)) as u64; rcx = (((rcx) ^ (r13))) as u64; rcx = (((rcx) as i64).wrapping_mul((r10) as i64) as u64) as u64; rcx = ((rcx).wrapping_add(r15)) as u64; rdx = r8; rdx = (((rdx) ^ (crate::simd_ops::ru64(rsp.wrapping_add(0xb8u64))))) as u64; rdx = (((rdx) as i64).wrapping_mul((r10) as i64) as u64) as u64; rax = rdx; rax = ((rax).wrapping_shr(47u32)) as u64; rdx = (((rdx) ^ (rax))) as u64; r8 = (((r8) ^ (rdx))) as u64; r8 = (((r8) as i64).wrapping_mul((r10) as i64) as u64) as u64; rax = r8; rax = ((rax).wrapping_shr(47u32)) as u64; rax = (((rax) ^ (r8))) as u64; rax = (((rax) as i64).wrapping_mul((r10) as i64) as u64) as u64; rdx = rbp; rdx = ((rdx).wrapping_shr(47u32)) as u64; rdx = (((rdx) ^ (rbp))) as u64; rdx = (((rdx) as i64).wrapping_mul((r12) as i64) as u64) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; rdx = ((rdx).wrapping_add(rsi)) as u64; rdx = (((rdx) ^ (rcx))) as u64; rdx = (((rdx) as i64).wrapping_mul((r10) as i64) as u64) as u64; rax = rdx; rax = ((rax).wrapping_shr(47u32)) as u64; rdx = (((rdx) ^ (rax))) as u64; rcx = (((rcx) ^ (rdx))) as u64; rcx = (((rcx) as i64).wrapping_mul((r10) as i64) as u64) as u64; rax = rcx; rax = ((rax).wrapping_shr(47u32)) as u64; rcx = (((rcx) ^ (rax))) as u64; rcx = (((rcx) as i64).wrapping_mul((r10) as i64) as u64) as u64; bb = 10; continue; }
            10 => { rax = (0xde5fb9d2630458e9u64) as u64; let prod = (rax as u128).wrapping_mul((rcx) as u128); rax = prod as u64; rdx = (prod >> 64) as u64; r9 = ((rdx.wrapping_add(rax)) as u64) as u64; r8 = rsp.wrapping_add(0x30); rdx = rsp.wrapping_add(0x20); rcx = P2; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; fl_l = (crate::simd_ops::ru8(rsp.wrapping_add(0x28u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 11; continue; } rbx = ((rdi.wrapping_add(rdi.wrapping_mul(2))) as u64) as u64; rbx = ((rbx).wrapping_shl(4u32)) as u64; rbx = ((rbx).wrapping_add(crate::simd_ops::ru64(P4))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xb0u64), rbx); crate::simd_ops::lk_and_u32(rbx, (0xff000004u32) as u32); crate::simd_ops::lk_or_u32(rbx, (0x4u32) as u32); crate::simd_ops::wu8(rbx.wrapping_add(0x3u64), (0x4u8) as u8); crate::simd_ops::wu32(rbx.wrapping_add(0x4u64), (0x0u32) as u32); crate::simd_ops::wu8(rbx.wrapping_add(0x8u64), (0x0u8) as u8); rdx = rsp.wrapping_add(0x30); rcx = rbx; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx.wrapping_add(0x28u64), (0x0u64) as u64); bb = 11; continue; }
            11 => { rax = ((rdi.wrapping_add(rdi.wrapping_mul(2))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rax = ((rax).wrapping_add(crate::simd_ops::ru64(P4))) as u64; rcx = P6; crate::simd_ops::wu64(rax.wrapping_add(0x28u64), rcx); rcx = rsp.wrapping_add(0x30); rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); rax = 0; fl_l = 0; fl_r = 0; rsp = ((rsp).wrapping_add(0x68u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_36631<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 21>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x68u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (0x4000004u64) as u64); crate::simd_ops::wu8(rsp.wrapping_add(0x38u64), (0x0u8) as u8); rax = P0; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (0x16u64) as u64); r9 = 0x16; r8 = 0; fl_l = 0; fl_r = 0; rdx = rsp.wrapping_add(0x20); rcx = rsp.wrapping_add(0x30); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru32(rsp.wrapping_add(0x34u64))) as u32 as u64; rcx = (crate::simd_ops::ru32(rsp.wrapping_add(0x30u64))) as u32 as u64; rax = ((rcx as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_shr(24u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x80; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 1; continue; } rax = ((rax.wrapping_mul(8)) as u32) as u32 as u64; bb = 2; continue; }
            1 => { rax = (((rax as u32)).wrapping_shl(8u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x7c00; rax = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 2; continue; }
            2 => { rcx = ((((rcx as u32)) & (0xffffffu32))) as u32 as u64; rcx = (((rcx as u32)).wrapping_shl(3u32)) as u32 as u64; r15 = rsp.wrapping_add(0x38); fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; r15 = (if (((fl_l as u32)) > ((fl_r as u32))) { crate::simd_ops::ru64(rsp.wrapping_add(0x38u64)) } else { r15 }) as u64; fl_l = rbx; fl_r = (0x20u64) as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 7; continue; } fl_l = rbx; fl_r = (0x10u64) as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 6; continue; } fl_l = rbx; fl_r = (0x8u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 3; continue; } r8 = (0x9ae16a3b2f90404fu64) as u64; r9 = ((r8.wrapping_add(rbx.wrapping_mul(2))) as u64) as u64; r8 = (crate::simd_ops::ru64(r15)) as u64; rax = (0x651e95c4d06fbfb1u64) as u64; r8 = ((r8).wrapping_sub(rax)) as u64; rdx = (crate::simd_ops::ru64(rbx.wrapping_add(r15).wrapping_sub(0x8u64))) as u64; rdx = ((rdx).rotate_right(37u32)) as u64; rdx = (((rdx) as i64).wrapping_mul((r9) as i64) as u64) as u64; rdx = ((rdx).wrapping_add(r8)) as u64; r8 = ((r8).rotate_right(25u32)) as u64; r8 = ((r8).wrapping_add(crate::simd_ops::ru64(rbx.wrapping_add(r15).wrapping_sub(0x8u64)))) as u64; r8 = (((r8) as i64).wrapping_mul((r9) as i64) as u64) as u64; rcx = r8; rcx = (((rcx) ^ (rdx))) as u64; rcx = (((rcx) as i64).wrapping_mul((r9) as i64) as u64) as u64; rax = rcx; rax = ((rax).wrapping_shr(47u32)) as u64; rcx = (((rcx) ^ (rax))) as u64; rcx = (((rcx) ^ (r8))) as u64; rcx = (((rcx) as i64).wrapping_mul((r9) as i64) as u64) as u64; rax = rcx; rax = ((rax).wrapping_shr(47u32)) as u64; rcx = (((rcx) ^ (rax))) as u64; rcx = (((rcx) as i64).wrapping_mul((r9) as i64) as u64) as u64; bb = 10; continue; }
            3 => { fl_l = rbx; fl_r = (0x4u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 4; continue; } r8 = (0x9ae16a3b2f90404fu64) as u64; r8 = ((r8.wrapping_add(rbx.wrapping_mul(2))) as u64) as u64; rax = (crate::simd_ops::ru32(r15)) as u32 as u64; rdx = (crate::simd_ops::ru32(rbx.wrapping_add(r15).wrapping_sub(0x4u64))) as u32 as u64; rcx = ((rbx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rcx = (((rcx) ^ (rdx))) as u64; rcx = (((rcx) as i64).wrapping_mul((r8) as i64) as u64) as u64; rax = rcx; rax = ((rax).wrapping_shr(47u32)) as u64; rcx = (((rcx) ^ (rax))) as u64; rcx = (((rcx) ^ (rdx))) as u64; rcx = (((rcx) as i64).wrapping_mul((r8) as i64) as u64) as u64; rax = rcx; rax = ((rax).wrapping_shr(47u32)) as u64; rcx = (((rcx) ^ (rax))) as u64; rcx = (((rcx) as i64).wrapping_mul((r8) as i64) as u64) as u64; bb = 10; continue; }
            4 => { fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } rax = rbx; rax = ((rax).wrapping_shr(1u32)) as u64; rdx = ((crate::simd_ops::ru8(rax.wrapping_add(r15))) as u32) as u32 as u64; rax = (0xe16a3b2f90404f00u64) as u64; rdx = (((rdx) as i64).wrapping_mul((rax) as i64) as u64) as u64; rax = ((crate::simd_ops::ru8(r15)) as u32) as u32 as u64; r8 = (0x9ae16a3b2f90404fu64) as u64; rax = (((rax) as i64).wrapping_mul((r8) as i64) as u64) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; rax = ((crate::simd_ops::ru8(rbx.wrapping_add(r15).wrapping_sub(0x1u64))) as u32) as u32 as u64; rax = ((rbx.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = ((rax as u32)) as u32 as u64; rcx = (0xc3a5c85c97cb3127u64) as u64; rax = (((rax) as i64).wrapping_mul((rcx) as i64) as u64) as u64; rdx = (((rdx) ^ (rax))) as u64; rcx = rdx; rcx = ((rcx).wrapping_shr(47u32)) as u64; rcx = (((rcx) ^ (rdx))) as u64; rcx = (((rcx) as i64).wrapping_mul((r8) as i64) as u64) as u64; bb = 10; continue; }
            5 => { rcx = (0x9ae16a3b2f90404fu64) as u64; bb = 10; continue; }
            6 => { r8 = (0x9ae16a3b2f90404fu64) as u64; r11 = ((r8.wrapping_add(rbx.wrapping_mul(2))) as u64) as u64; r12 = (0xb492b66fbe98f273u64) as u64; r9 = (crate::simd_ops::ru64(r15)) as u64; r9 = (((r9) as i64).wrapping_mul((r12) as i64) as u64) as u64; rdx = (crate::simd_ops::ru64(r15.wrapping_add(0x8u64))) as u64; r10 = r11; r10 = (((r10) as i64).wrapping_mul((crate::simd_ops::ru64(rbx.wrapping_add(r15).wrapping_sub(0x8u64))) as i64) as u64) as u64; rcx = (crate::simd_ops::ru64(rbx.wrapping_add(r15).wrapping_sub(0x10u64))) as u64; rcx = (((rcx) as i64).wrapping_mul((r8) as i64) as u64) as u64; r8 = ((r8).wrapping_add(rdx)) as u64; r8 = ((r8).rotate_right(18u32)) as u64; r8 = ((r8).wrapping_add(r10)) as u64; r8 = ((r8).wrapping_add(r9)) as u64; rax = ((rdx.wrapping_add(r9)) as u64) as u64; rax = ((rax).rotate_right(43u32)) as u64; r10 = ((r10).rotate_right(30u32)) as u64; rcx = ((rcx).wrapping_add(rax)) as u64; rcx = ((rcx).wrapping_add(r10)) as u64; rcx = (((rcx) ^ (r8))) as u64; rcx = (((rcx) as i64).wrapping_mul((r11) as i64) as u64) as u64; rax = rcx; rax = ((rax).wrapping_shr(47u32)) as u64; rcx = (((rcx) ^ (rax))) as u64; rcx = (((rcx) ^ (r8))) as u64; rcx = (((rcx) as i64).wrapping_mul((r11) as i64) as u64) as u64; rax = rcx; rax = ((rax).wrapping_shr(47u32)) as u64; rcx = (((rcx) ^ (rax))) as u64; rcx = (((rcx) as i64).wrapping_mul((r11) as i64) as u64) as u64; bb = 10; continue; }
            7 => { r13 = ((rbx.wrapping_add(r15)) as u64) as u64; fl_l = rbx; fl_r = (0x40u64) as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 8; continue; } r8 = (0x9ae16a3b2f90404fu64) as u64; r14 = ((r8.wrapping_add(rbx.wrapping_mul(2))) as u64) as u64; r11 = (crate::simd_ops::ru64(r13.wrapping_sub(0x18u64))) as u64; r10 = (crate::simd_ops::ru64(r15.wrapping_add(0x10u64))) as u64; r10 = (((r10) as i64).wrapping_mul((r8) as i64) as u64) as u64; rax = (crate::simd_ops::ru64(r15.wrapping_add(0x18u64))) as u64; r9 = ((rax.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rdi = r14; rdi = (((rdi) as i64).wrapping_mul((crate::simd_ops::ru64(r13.wrapping_sub(0x10u64))) as i64) as u64) as u64; rdx = (crate::simd_ops::ru64(r15)) as u64; rdx = (((rdx) as i64).wrapping_mul((r8) as i64) as u64) as u64; rdx = ((rdx).wrapping_add(crate::simd_ops::ru64(r13.wrapping_sub(0x8u64)))) as u64; rax = (crate::simd_ops::ru64(r15.wrapping_add(0x8u64))) as u64; rax = ((rax).rotate_right(30u32)) as u64; rax = ((rax).wrapping_add(r11)) as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rax = rdx; rax = ((rax).rotate_right(43u32)) as u64; rcx = ((rcx).wrapping_add(rax)) as u64; r8 = (crate::simd_ops::ru64(r13.wrapping_sub(0x20u64))) as u64; r8 = (((r8) ^ (rdx))) as u64; r8 = ((r8).wrapping_add(1)) as u64; r8 = ((r8).wrapping_add(r9)) as u64; rdx = ((r8.wrapping_add(rcx)) as u64) as u64; rdx = (((rdx) as i64).wrapping_mul((r14) as i64) as u64) as u64; rdx = ((rdx).swap_bytes()) as u64; rdx = ((rdx).wrapping_add(rdi)) as u64; rax = ((r9.wrapping_add(r10)) as u64) as u64; r9 = rax; r9 = ((r9).rotate_right(42u32)) as u64; r9 = ((r9).wrapping_add(r11)) as u64; rcx = ((rax.wrapping_add(r11)) as u64) as u64; rax = ((rdx.wrapping_add(r8)) as u64) as u64; rax = (((rax) as i64).wrapping_mul((r14) as i64) as u64) as u64; rax = ((rax).swap_bytes()) as u64; rax = ((rax).wrapping_add(rcx)) as u64; rax = ((rax).wrapping_add(r9)) as u64; rax = ((rax).wrapping_add(crate::simd_ops::ru64(r13.wrapping_sub(0x8u64)))) as u64; rax = (((rax) as i64).wrapping_mul((r14) as i64) as u64) as u64; rax = ((rax).swap_bytes()) as u64; rax = ((rax).wrapping_add(crate::simd_ops::ru64(r15.wrapping_add(0x8u64)))) as u64; rcx = ((rcx).wrapping_add(rax)) as u64; rcx = (((rcx) as i64).wrapping_mul((r14) as i64) as u64) as u64; rcx = ((rcx).wrapping_add(rdi)) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(r13.wrapping_sub(0x20u64)))) as u64; rax = rcx; rax = ((rax).wrapping_shr(47u32)) as u64; rcx = (((rcx) ^ (rax))) as u64; rcx = (((rcx) as i64).wrapping_mul((r14) as i64) as u64) as u64; fl_l = ((rcx).wrapping_add(r9)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; bb = 10; continue; }
            8 => { rdi = (crate::simd_ops::ru64(r13.wrapping_sub(0x28u64))) as u64; rbp = (crate::simd_ops::ru64(r13.wrapping_sub(0x38u64))) as u64; rbp = ((rbp).wrapping_add(crate::simd_ops::ru64(r13.wrapping_sub(0x10u64)))) as u64; r14 = (crate::simd_ops::ru64(r13.wrapping_sub(0x30u64))) as u64; r14 = ((r14).wrapping_add(rbx)) as u64; r14 = (((r14) ^ (crate::simd_ops::ru64(r13.wrapping_sub(0x18u64))))) as u64; rdx = (0x9ddfea08eb382d69u64) as u64; r14 = (((r14) as i64).wrapping_mul((rdx) as i64) as u64) as u64; rax = r14; rax = ((rax).wrapping_shr(47u32)) as u64; r14 = (((r14) ^ (rax))) as u64; r14 = (((r14) ^ (crate::simd_ops::ru64(r13.wrapping_sub(0x18u64))))) as u64; r14 = (((r14) as i64).wrapping_mul((rdx) as i64) as u64) as u64; rax = r14; rax = ((rax).wrapping_shr(47u32)) as u64; r14 = (((r14) ^ (rax))) as u64; r14 = (((r14) as i64).wrapping_mul((rdx) as i64) as u64) as u64; rax = (crate::simd_ops::ru64(r13.wrapping_sub(0x30u64))) as u64; r10 = (crate::simd_ops::ru64(r13.wrapping_sub(0x40u64))) as u64; r10 = ((r10).wrapping_add(rbx)) as u64; r9 = ((rdi.wrapping_add(r10)) as u64) as u64; r9 = ((r9).wrapping_add(r14)) as u64; r8 = r10; rax = ((rax).wrapping_add(crate::simd_ops::ru64(r13.wrapping_sub(0x38u64)))) as u64; r10 = ((r10).wrapping_add(rax)) as u64; r9 = ((r9).rotate_right(21u32)) as u64; rax = r10; rax = ((rax).rotate_right(44u32)) as u64; rax = ((rax).wrapping_add(r9)) as u64; rax = ((rax).wrapping_add(r8)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xc0u64), rax); r9 = ((rdi.wrapping_add(r10)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xb8u64), r9); r11 = (crate::simd_ops::ru64(r13.wrapping_sub(0x8u64))) as u64; rax = (crate::simd_ops::ru64(r13.wrapping_sub(0x10u64))) as u64; rcx = (crate::simd_ops::ru64(r13.wrapping_sub(0x18u64))) as u64; r10 = (crate::simd_ops::ru64(r13.wrapping_sub(0x20u64))) as u64; r12 = (0xb492b66fbe98f273u64) as u64; r10 = ((r10).wrapping_add(r12)) as u64; r10 = ((r10).wrapping_add(rbp)) as u64; r9 = ((r11.wrapping_add(r10)) as u64) as u64; r9 = ((r9).wrapping_add(rdi)) as u64; r13 = r10; rax = ((rax).wrapping_add(rcx)) as u64; r10 = ((r10).wrapping_add(rax)) as u64; r9 = ((r9).rotate_right(21u32)) as u64; rax = r10; rax = ((rax).rotate_right(44u32)) as u64; r9 = ((r9).wrapping_add(rax)) as u64; r13 = ((r13).wrapping_add(r9)) as u64; r8 = ((r11.wrapping_add(r10)) as u64) as u64; rdi = (((rdi) as i64).wrapping_mul((r12) as i64) as u64) as u64; rdi = ((rdi).wrapping_add(crate::simd_ops::ru64(r15))) as u64; rbx = ((rbx).wrapping_sub(1)) as u64; fl_l = (((rbx) & (0xffffffffffffffc0u64))) as u64; fl_r = fl_l; rbx = ((fl_l as u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xc8u64), rbx); rax = r15.wrapping_add(0x30); crate::simd_ops::wu64(rsp.wrapping_add(0xb0u64), rax); r15 = rdi; bb = 9; continue; }
            9 => { r11 = (crate::simd_ops::ru64(rax.wrapping_sub(0x28u64))) as u64; rsi = ((r11.wrapping_add(rbp)) as u64) as u64; rsi = ((rsi).wrapping_add(r15)) as u64; rsi = ((rsi).wrapping_add(crate::simd_ops::ru64(rsp.wrapping_add(0xb8u64)))) as u64; rdi = (crate::simd_ops::ru64(rax)) as u64; rbp = ((rbp).wrapping_add(rdi)) as u64; rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0xc0u64))) as u64; rbp = ((rbp).wrapping_add(rdx)) as u64; rsi = ((rsi).rotate_right(37u32)) as u64; rsi = (((rsi) as i64).wrapping_mul((r12) as i64) as u64) as u64; rsi = (((rsi) ^ (r13))) as u64; rbx = (crate::simd_ops::ru64(rax.wrapping_sub(0x8u64))) as u64; rbp = ((rbp).rotate_right(42u32)) as u64; rbp = (((rbp) as i64).wrapping_mul((r12) as i64) as u64) as u64; rbp = ((rbp).wrapping_add(rbx)) as u64; rbp = ((rbp).wrapping_add(crate::simd_ops::ru64(rsp.wrapping_add(0xb8u64)))) as u64; r15 = ((r14.wrapping_add(r8)) as u64) as u64; r15 = ((r15).rotate_right(33u32)) as u64; r15 = (((r15) as i64).wrapping_mul((r12) as i64) as u64) as u64; rcx = ((rsi.wrapping_add(r8)) as u64) as u64; rdx = (((rdx) as i64).wrapping_mul((r12) as i64) as u64) as u64; r9 = (crate::simd_ops::ru64(rax.wrapping_sub(0x18u64))) as u64; r10 = (crate::simd_ops::ru64(rax.wrapping_sub(0x20u64))) as u64; r8 = (crate::simd_ops::ru64(rax.wrapping_sub(0x30u64))) as u64; r8 = ((r8).wrapping_add(rdx)) as u64; rdx = ((r9.wrapping_add(r8)) as u64) as u64; rdx = ((rdx).wrapping_add(rcx)) as u64; rcx = r8; rax = ((r10.wrapping_add(r11)) as u64) as u64; r8 = ((r8).wrapping_add(rax)) as u64; rdx = ((rdx).rotate_right(21u32)) as u64; r11 = r8; r11 = ((r11).rotate_right(44u32)) as u64; r11 = ((r11).wrapping_add(rdx)) as u64; r11 = ((r11).wrapping_add(rcx)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xc0u64), r11); rax = ((r9.wrapping_add(r8)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xb8u64), rax); rdx = ((r10.wrapping_add(rbp)) as u64) as u64; r9 = ((r15.wrapping_add(r13)) as u64) as u64; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xb0u64))) as u64; r10 = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; r9 = ((r9).wrapping_add(crate::simd_ops::ru64(rax.wrapping_sub(0x10u64)))) as u64; r8 = ((r10.wrapping_add(r9)) as u64) as u64; r8 = ((r8).wrapping_add(rdx)) as u64; r13 = r9; rax = ((rdi.wrapping_add(rbx)) as u64) as u64; r9 = ((r9).wrapping_add(rax)) as u64; r8 = ((r8).rotate_right(21u32)) as u64; rax = r9; rax = ((rax).rotate_right(44u32)) as u64; r8 = ((r8).wrapping_add(rax)) as u64; r13 = ((r13).wrapping_add(r8)) as u64; r8 = ((r10.wrapping_add(r9)) as u64) as u64; r14 = rsi; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xb0u64))) as u64; rax = ((rax).wrapping_add(0x40u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xb0u64), rax); fl_l = (crate::simd_ops::ru64(rsp.wrapping_add(0xc8u64))) as u64; fl_r = (0x40u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xc8u64), ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64); if (((fl_l as u64)) != ((fl_r as u64))) { bb = 9; continue; } rcx = r13; rcx = (((rcx) ^ (r11))) as u64; r10 = (0x9ddfea08eb382d69u64) as u64; rcx = (((rcx) as i64).wrapping_mul((r10) as i64) as u64) as u64; rax = rcx; rax = ((rax).wrapping_shr(47u32)) as u64; rcx = (((rcx) ^ (rax))) as u64; r13 = (((r13) ^ (rcx))) as u64; r13 = (((r13) as i64).wrapping_mul((r10) as i64) as u64) as u64; rcx = r13; rcx = ((rcx).wrapping_shr(47u32)) as u64; rcx = (((rcx) ^ (r13))) as u64; rcx = (((rcx) as i64).wrapping_mul((r10) as i64) as u64) as u64; rcx = ((rcx).wrapping_add(r15)) as u64; rdx = r8; rdx = (((rdx) ^ (crate::simd_ops::ru64(rsp.wrapping_add(0xb8u64))))) as u64; rdx = (((rdx) as i64).wrapping_mul((r10) as i64) as u64) as u64; rax = rdx; rax = ((rax).wrapping_shr(47u32)) as u64; rdx = (((rdx) ^ (rax))) as u64; r8 = (((r8) ^ (rdx))) as u64; r8 = (((r8) as i64).wrapping_mul((r10) as i64) as u64) as u64; rax = r8; rax = ((rax).wrapping_shr(47u32)) as u64; rax = (((rax) ^ (r8))) as u64; rax = (((rax) as i64).wrapping_mul((r10) as i64) as u64) as u64; rdx = rbp; rdx = ((rdx).wrapping_shr(47u32)) as u64; rdx = (((rdx) ^ (rbp))) as u64; rdx = (((rdx) as i64).wrapping_mul((r12) as i64) as u64) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; rdx = ((rdx).wrapping_add(rsi)) as u64; rdx = (((rdx) ^ (rcx))) as u64; rdx = (((rdx) as i64).wrapping_mul((r10) as i64) as u64) as u64; rax = rdx; rax = ((rax).wrapping_shr(47u32)) as u64; rdx = (((rdx) ^ (rax))) as u64; rcx = (((rcx) ^ (rdx))) as u64; rcx = (((rcx) as i64).wrapping_mul((r10) as i64) as u64) as u64; rax = rcx; rax = ((rax).wrapping_shr(47u32)) as u64; rcx = (((rcx) ^ (rax))) as u64; rcx = (((rcx) as i64).wrapping_mul((r10) as i64) as u64) as u64; bb = 10; continue; }
            10 => { rax = (0xde5fb9d2630458e9u64) as u64; let prod = (rax as u128).wrapping_mul((rcx) as u128); rax = prod as u64; rdx = (prod >> 64) as u64; r9 = ((rdx.wrapping_add(rax)) as u64) as u64; r8 = rsp.wrapping_add(0x30); rdx = rsp.wrapping_add(0x20); rcx = P2; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; fl_l = (crate::simd_ops::ru8(rsp.wrapping_add(0x28u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 11; continue; } rbx = ((rdi.wrapping_add(rdi.wrapping_mul(2))) as u64) as u64; rbx = ((rbx).wrapping_shl(4u32)) as u64; rbx = ((rbx).wrapping_add(crate::simd_ops::ru64(P4))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xb0u64), rbx); crate::simd_ops::lk_and_u32(rbx, (0xff000004u32) as u32); crate::simd_ops::lk_or_u32(rbx, (0x4u32) as u32); crate::simd_ops::wu8(rbx.wrapping_add(0x3u64), (0x4u8) as u8); crate::simd_ops::wu32(rbx.wrapping_add(0x4u64), (0x0u32) as u32); crate::simd_ops::wu8(rbx.wrapping_add(0x8u64), (0x0u8) as u8); rdx = rsp.wrapping_add(0x30); rcx = rbx; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx.wrapping_add(0x28u64), (0x0u64) as u64); bb = 11; continue; }
            11 => { rax = ((rdi.wrapping_add(rdi.wrapping_mul(2))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rax = ((rax).wrapping_add(crate::simd_ops::ru64(P4))) as u64; rcx = P6; crate::simd_ops::wu64(rax.wrapping_add(0x28u64), rcx); rcx = rsp.wrapping_add(0x30); rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); rax = 0; fl_l = 0; fl_r = 0; rsp = ((rsp).wrapping_add(0x68u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_36632<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 21>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x68u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (0x4000004u64) as u64); crate::simd_ops::wu8(rsp.wrapping_add(0x38u64), (0x0u8) as u8); rax = P0; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (0x1du64) as u64); r9 = 0x1d; r8 = 0; fl_l = 0; fl_r = 0; rdx = rsp.wrapping_add(0x20); rcx = rsp.wrapping_add(0x30); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru32(rsp.wrapping_add(0x34u64))) as u32 as u64; rcx = (crate::simd_ops::ru32(rsp.wrapping_add(0x30u64))) as u32 as u64; rax = ((rcx as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_shr(24u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x80; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 1; continue; } rax = ((rax.wrapping_mul(8)) as u32) as u32 as u64; bb = 2; continue; }
            1 => { rax = (((rax as u32)).wrapping_shl(8u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x7c00; rax = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 2; continue; }
            2 => { rcx = ((((rcx as u32)) & (0xffffffu32))) as u32 as u64; rcx = (((rcx as u32)).wrapping_shl(3u32)) as u32 as u64; r15 = rsp.wrapping_add(0x38); fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; r15 = (if (((fl_l as u32)) > ((fl_r as u32))) { crate::simd_ops::ru64(rsp.wrapping_add(0x38u64)) } else { r15 }) as u64; fl_l = rbx; fl_r = (0x20u64) as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 7; continue; } fl_l = rbx; fl_r = (0x10u64) as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 6; continue; } fl_l = rbx; fl_r = (0x8u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 3; continue; } r8 = (0x9ae16a3b2f90404fu64) as u64; r9 = ((r8.wrapping_add(rbx.wrapping_mul(2))) as u64) as u64; r8 = (crate::simd_ops::ru64(r15)) as u64; rax = (0x651e95c4d06fbfb1u64) as u64; r8 = ((r8).wrapping_sub(rax)) as u64; rdx = (crate::simd_ops::ru64(rbx.wrapping_add(r15).wrapping_sub(0x8u64))) as u64; rdx = ((rdx).rotate_right(37u32)) as u64; rdx = (((rdx) as i64).wrapping_mul((r9) as i64) as u64) as u64; rdx = ((rdx).wrapping_add(r8)) as u64; r8 = ((r8).rotate_right(25u32)) as u64; r8 = ((r8).wrapping_add(crate::simd_ops::ru64(rbx.wrapping_add(r15).wrapping_sub(0x8u64)))) as u64; r8 = (((r8) as i64).wrapping_mul((r9) as i64) as u64) as u64; rcx = r8; rcx = (((rcx) ^ (rdx))) as u64; rcx = (((rcx) as i64).wrapping_mul((r9) as i64) as u64) as u64; rax = rcx; rax = ((rax).wrapping_shr(47u32)) as u64; rcx = (((rcx) ^ (rax))) as u64; rcx = (((rcx) ^ (r8))) as u64; rcx = (((rcx) as i64).wrapping_mul((r9) as i64) as u64) as u64; rax = rcx; rax = ((rax).wrapping_shr(47u32)) as u64; rcx = (((rcx) ^ (rax))) as u64; rcx = (((rcx) as i64).wrapping_mul((r9) as i64) as u64) as u64; bb = 10; continue; }
            3 => { fl_l = rbx; fl_r = (0x4u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 4; continue; } r8 = (0x9ae16a3b2f90404fu64) as u64; r8 = ((r8.wrapping_add(rbx.wrapping_mul(2))) as u64) as u64; rax = (crate::simd_ops::ru32(r15)) as u32 as u64; rdx = (crate::simd_ops::ru32(rbx.wrapping_add(r15).wrapping_sub(0x4u64))) as u32 as u64; rcx = ((rbx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rcx = (((rcx) ^ (rdx))) as u64; rcx = (((rcx) as i64).wrapping_mul((r8) as i64) as u64) as u64; rax = rcx; rax = ((rax).wrapping_shr(47u32)) as u64; rcx = (((rcx) ^ (rax))) as u64; rcx = (((rcx) ^ (rdx))) as u64; rcx = (((rcx) as i64).wrapping_mul((r8) as i64) as u64) as u64; rax = rcx; rax = ((rax).wrapping_shr(47u32)) as u64; rcx = (((rcx) ^ (rax))) as u64; rcx = (((rcx) as i64).wrapping_mul((r8) as i64) as u64) as u64; bb = 10; continue; }
            4 => { fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } rax = rbx; rax = ((rax).wrapping_shr(1u32)) as u64; rdx = ((crate::simd_ops::ru8(rax.wrapping_add(r15))) as u32) as u32 as u64; rax = (0xe16a3b2f90404f00u64) as u64; rdx = (((rdx) as i64).wrapping_mul((rax) as i64) as u64) as u64; rax = ((crate::simd_ops::ru8(r15)) as u32) as u32 as u64; r8 = (0x9ae16a3b2f90404fu64) as u64; rax = (((rax) as i64).wrapping_mul((r8) as i64) as u64) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; rax = ((crate::simd_ops::ru8(rbx.wrapping_add(r15).wrapping_sub(0x1u64))) as u32) as u32 as u64; rax = ((rbx.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = ((rax as u32)) as u32 as u64; rcx = (0xc3a5c85c97cb3127u64) as u64; rax = (((rax) as i64).wrapping_mul((rcx) as i64) as u64) as u64; rdx = (((rdx) ^ (rax))) as u64; rcx = rdx; rcx = ((rcx).wrapping_shr(47u32)) as u64; rcx = (((rcx) ^ (rdx))) as u64; rcx = (((rcx) as i64).wrapping_mul((r8) as i64) as u64) as u64; bb = 10; continue; }
            5 => { rcx = (0x9ae16a3b2f90404fu64) as u64; bb = 10; continue; }
            6 => { r8 = (0x9ae16a3b2f90404fu64) as u64; r11 = ((r8.wrapping_add(rbx.wrapping_mul(2))) as u64) as u64; r12 = (0xb492b66fbe98f273u64) as u64; r9 = (crate::simd_ops::ru64(r15)) as u64; r9 = (((r9) as i64).wrapping_mul((r12) as i64) as u64) as u64; rdx = (crate::simd_ops::ru64(r15.wrapping_add(0x8u64))) as u64; r10 = r11; r10 = (((r10) as i64).wrapping_mul((crate::simd_ops::ru64(rbx.wrapping_add(r15).wrapping_sub(0x8u64))) as i64) as u64) as u64; rcx = (crate::simd_ops::ru64(rbx.wrapping_add(r15).wrapping_sub(0x10u64))) as u64; rcx = (((rcx) as i64).wrapping_mul((r8) as i64) as u64) as u64; r8 = ((r8).wrapping_add(rdx)) as u64; r8 = ((r8).rotate_right(18u32)) as u64; r8 = ((r8).wrapping_add(r10)) as u64; r8 = ((r8).wrapping_add(r9)) as u64; rax = ((rdx.wrapping_add(r9)) as u64) as u64; rax = ((rax).rotate_right(43u32)) as u64; r10 = ((r10).rotate_right(30u32)) as u64; rcx = ((rcx).wrapping_add(rax)) as u64; rcx = ((rcx).wrapping_add(r10)) as u64; rcx = (((rcx) ^ (r8))) as u64; rcx = (((rcx) as i64).wrapping_mul((r11) as i64) as u64) as u64; rax = rcx; rax = ((rax).wrapping_shr(47u32)) as u64; rcx = (((rcx) ^ (rax))) as u64; rcx = (((rcx) ^ (r8))) as u64; rcx = (((rcx) as i64).wrapping_mul((r11) as i64) as u64) as u64; rax = rcx; rax = ((rax).wrapping_shr(47u32)) as u64; rcx = (((rcx) ^ (rax))) as u64; rcx = (((rcx) as i64).wrapping_mul((r11) as i64) as u64) as u64; bb = 10; continue; }
            7 => { r13 = ((rbx.wrapping_add(r15)) as u64) as u64; fl_l = rbx; fl_r = (0x40u64) as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 8; continue; } r8 = (0x9ae16a3b2f90404fu64) as u64; r14 = ((r8.wrapping_add(rbx.wrapping_mul(2))) as u64) as u64; r11 = (crate::simd_ops::ru64(r13.wrapping_sub(0x18u64))) as u64; r10 = (crate::simd_ops::ru64(r15.wrapping_add(0x10u64))) as u64; r10 = (((r10) as i64).wrapping_mul((r8) as i64) as u64) as u64; rax = (crate::simd_ops::ru64(r15.wrapping_add(0x18u64))) as u64; r9 = ((rax.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rdi = r14; rdi = (((rdi) as i64).wrapping_mul((crate::simd_ops::ru64(r13.wrapping_sub(0x10u64))) as i64) as u64) as u64; rdx = (crate::simd_ops::ru64(r15)) as u64; rdx = (((rdx) as i64).wrapping_mul((r8) as i64) as u64) as u64; rdx = ((rdx).wrapping_add(crate::simd_ops::ru64(r13.wrapping_sub(0x8u64)))) as u64; rax = (crate::simd_ops::ru64(r15.wrapping_add(0x8u64))) as u64; rax = ((rax).rotate_right(30u32)) as u64; rax = ((rax).wrapping_add(r11)) as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rax = rdx; rax = ((rax).rotate_right(43u32)) as u64; rcx = ((rcx).wrapping_add(rax)) as u64; r8 = (crate::simd_ops::ru64(r13.wrapping_sub(0x20u64))) as u64; r8 = (((r8) ^ (rdx))) as u64; r8 = ((r8).wrapping_add(1)) as u64; r8 = ((r8).wrapping_add(r9)) as u64; rdx = ((r8.wrapping_add(rcx)) as u64) as u64; rdx = (((rdx) as i64).wrapping_mul((r14) as i64) as u64) as u64; rdx = ((rdx).swap_bytes()) as u64; rdx = ((rdx).wrapping_add(rdi)) as u64; rax = ((r9.wrapping_add(r10)) as u64) as u64; r9 = rax; r9 = ((r9).rotate_right(42u32)) as u64; r9 = ((r9).wrapping_add(r11)) as u64; rcx = ((rax.wrapping_add(r11)) as u64) as u64; rax = ((rdx.wrapping_add(r8)) as u64) as u64; rax = (((rax) as i64).wrapping_mul((r14) as i64) as u64) as u64; rax = ((rax).swap_bytes()) as u64; rax = ((rax).wrapping_add(rcx)) as u64; rax = ((rax).wrapping_add(r9)) as u64; rax = ((rax).wrapping_add(crate::simd_ops::ru64(r13.wrapping_sub(0x8u64)))) as u64; rax = (((rax) as i64).wrapping_mul((r14) as i64) as u64) as u64; rax = ((rax).swap_bytes()) as u64; rax = ((rax).wrapping_add(crate::simd_ops::ru64(r15.wrapping_add(0x8u64)))) as u64; rcx = ((rcx).wrapping_add(rax)) as u64; rcx = (((rcx) as i64).wrapping_mul((r14) as i64) as u64) as u64; rcx = ((rcx).wrapping_add(rdi)) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(r13.wrapping_sub(0x20u64)))) as u64; rax = rcx; rax = ((rax).wrapping_shr(47u32)) as u64; rcx = (((rcx) ^ (rax))) as u64; rcx = (((rcx) as i64).wrapping_mul((r14) as i64) as u64) as u64; fl_l = ((rcx).wrapping_add(r9)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; bb = 10; continue; }
            8 => { rdi = (crate::simd_ops::ru64(r13.wrapping_sub(0x28u64))) as u64; rbp = (crate::simd_ops::ru64(r13.wrapping_sub(0x38u64))) as u64; rbp = ((rbp).wrapping_add(crate::simd_ops::ru64(r13.wrapping_sub(0x10u64)))) as u64; r14 = (crate::simd_ops::ru64(r13.wrapping_sub(0x30u64))) as u64; r14 = ((r14).wrapping_add(rbx)) as u64; r14 = (((r14) ^ (crate::simd_ops::ru64(r13.wrapping_sub(0x18u64))))) as u64; rdx = (0x9ddfea08eb382d69u64) as u64; r14 = (((r14) as i64).wrapping_mul((rdx) as i64) as u64) as u64; rax = r14; rax = ((rax).wrapping_shr(47u32)) as u64; r14 = (((r14) ^ (rax))) as u64; r14 = (((r14) ^ (crate::simd_ops::ru64(r13.wrapping_sub(0x18u64))))) as u64; r14 = (((r14) as i64).wrapping_mul((rdx) as i64) as u64) as u64; rax = r14; rax = ((rax).wrapping_shr(47u32)) as u64; r14 = (((r14) ^ (rax))) as u64; r14 = (((r14) as i64).wrapping_mul((rdx) as i64) as u64) as u64; rax = (crate::simd_ops::ru64(r13.wrapping_sub(0x30u64))) as u64; r10 = (crate::simd_ops::ru64(r13.wrapping_sub(0x40u64))) as u64; r10 = ((r10).wrapping_add(rbx)) as u64; r9 = ((rdi.wrapping_add(r10)) as u64) as u64; r9 = ((r9).wrapping_add(r14)) as u64; r8 = r10; rax = ((rax).wrapping_add(crate::simd_ops::ru64(r13.wrapping_sub(0x38u64)))) as u64; r10 = ((r10).wrapping_add(rax)) as u64; r9 = ((r9).rotate_right(21u32)) as u64; rax = r10; rax = ((rax).rotate_right(44u32)) as u64; rax = ((rax).wrapping_add(r9)) as u64; rax = ((rax).wrapping_add(r8)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xc0u64), rax); r9 = ((rdi.wrapping_add(r10)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xb8u64), r9); r11 = (crate::simd_ops::ru64(r13.wrapping_sub(0x8u64))) as u64; rax = (crate::simd_ops::ru64(r13.wrapping_sub(0x10u64))) as u64; rcx = (crate::simd_ops::ru64(r13.wrapping_sub(0x18u64))) as u64; r10 = (crate::simd_ops::ru64(r13.wrapping_sub(0x20u64))) as u64; r12 = (0xb492b66fbe98f273u64) as u64; r10 = ((r10).wrapping_add(r12)) as u64; r10 = ((r10).wrapping_add(rbp)) as u64; r9 = ((r11.wrapping_add(r10)) as u64) as u64; r9 = ((r9).wrapping_add(rdi)) as u64; r13 = r10; rax = ((rax).wrapping_add(rcx)) as u64; r10 = ((r10).wrapping_add(rax)) as u64; r9 = ((r9).rotate_right(21u32)) as u64; rax = r10; rax = ((rax).rotate_right(44u32)) as u64; r9 = ((r9).wrapping_add(rax)) as u64; r13 = ((r13).wrapping_add(r9)) as u64; r8 = ((r11.wrapping_add(r10)) as u64) as u64; rdi = (((rdi) as i64).wrapping_mul((r12) as i64) as u64) as u64; rdi = ((rdi).wrapping_add(crate::simd_ops::ru64(r15))) as u64; rbx = ((rbx).wrapping_sub(1)) as u64; fl_l = (((rbx) & (0xffffffffffffffc0u64))) as u64; fl_r = fl_l; rbx = ((fl_l as u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xc8u64), rbx); rax = r15.wrapping_add(0x30); crate::simd_ops::wu64(rsp.wrapping_add(0xb0u64), rax); r15 = rdi; bb = 9; continue; }
            9 => { r11 = (crate::simd_ops::ru64(rax.wrapping_sub(0x28u64))) as u64; rsi = ((r11.wrapping_add(rbp)) as u64) as u64; rsi = ((rsi).wrapping_add(r15)) as u64; rsi = ((rsi).wrapping_add(crate::simd_ops::ru64(rsp.wrapping_add(0xb8u64)))) as u64; rdi = (crate::simd_ops::ru64(rax)) as u64; rbp = ((rbp).wrapping_add(rdi)) as u64; rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0xc0u64))) as u64; rbp = ((rbp).wrapping_add(rdx)) as u64; rsi = ((rsi).rotate_right(37u32)) as u64; rsi = (((rsi) as i64).wrapping_mul((r12) as i64) as u64) as u64; rsi = (((rsi) ^ (r13))) as u64; rbx = (crate::simd_ops::ru64(rax.wrapping_sub(0x8u64))) as u64; rbp = ((rbp).rotate_right(42u32)) as u64; rbp = (((rbp) as i64).wrapping_mul((r12) as i64) as u64) as u64; rbp = ((rbp).wrapping_add(rbx)) as u64; rbp = ((rbp).wrapping_add(crate::simd_ops::ru64(rsp.wrapping_add(0xb8u64)))) as u64; r15 = ((r14.wrapping_add(r8)) as u64) as u64; r15 = ((r15).rotate_right(33u32)) as u64; r15 = (((r15) as i64).wrapping_mul((r12) as i64) as u64) as u64; rcx = ((rsi.wrapping_add(r8)) as u64) as u64; rdx = (((rdx) as i64).wrapping_mul((r12) as i64) as u64) as u64; r9 = (crate::simd_ops::ru64(rax.wrapping_sub(0x18u64))) as u64; r10 = (crate::simd_ops::ru64(rax.wrapping_sub(0x20u64))) as u64; r8 = (crate::simd_ops::ru64(rax.wrapping_sub(0x30u64))) as u64; r8 = ((r8).wrapping_add(rdx)) as u64; rdx = ((r9.wrapping_add(r8)) as u64) as u64; rdx = ((rdx).wrapping_add(rcx)) as u64; rcx = r8; rax = ((r10.wrapping_add(r11)) as u64) as u64; r8 = ((r8).wrapping_add(rax)) as u64; rdx = ((rdx).rotate_right(21u32)) as u64; r11 = r8; r11 = ((r11).rotate_right(44u32)) as u64; r11 = ((r11).wrapping_add(rdx)) as u64; r11 = ((r11).wrapping_add(rcx)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xc0u64), r11); rax = ((r9.wrapping_add(r8)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xb8u64), rax); rdx = ((r10.wrapping_add(rbp)) as u64) as u64; r9 = ((r15.wrapping_add(r13)) as u64) as u64; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xb0u64))) as u64; r10 = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; r9 = ((r9).wrapping_add(crate::simd_ops::ru64(rax.wrapping_sub(0x10u64)))) as u64; r8 = ((r10.wrapping_add(r9)) as u64) as u64; r8 = ((r8).wrapping_add(rdx)) as u64; r13 = r9; rax = ((rdi.wrapping_add(rbx)) as u64) as u64; r9 = ((r9).wrapping_add(rax)) as u64; r8 = ((r8).rotate_right(21u32)) as u64; rax = r9; rax = ((rax).rotate_right(44u32)) as u64; r8 = ((r8).wrapping_add(rax)) as u64; r13 = ((r13).wrapping_add(r8)) as u64; r8 = ((r10.wrapping_add(r9)) as u64) as u64; r14 = rsi; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xb0u64))) as u64; rax = ((rax).wrapping_add(0x40u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xb0u64), rax); fl_l = (crate::simd_ops::ru64(rsp.wrapping_add(0xc8u64))) as u64; fl_r = (0x40u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xc8u64), ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64); if (((fl_l as u64)) != ((fl_r as u64))) { bb = 9; continue; } rcx = r13; rcx = (((rcx) ^ (r11))) as u64; r10 = (0x9ddfea08eb382d69u64) as u64; rcx = (((rcx) as i64).wrapping_mul((r10) as i64) as u64) as u64; rax = rcx; rax = ((rax).wrapping_shr(47u32)) as u64; rcx = (((rcx) ^ (rax))) as u64; r13 = (((r13) ^ (rcx))) as u64; r13 = (((r13) as i64).wrapping_mul((r10) as i64) as u64) as u64; rcx = r13; rcx = ((rcx).wrapping_shr(47u32)) as u64; rcx = (((rcx) ^ (r13))) as u64; rcx = (((rcx) as i64).wrapping_mul((r10) as i64) as u64) as u64; rcx = ((rcx).wrapping_add(r15)) as u64; rdx = r8; rdx = (((rdx) ^ (crate::simd_ops::ru64(rsp.wrapping_add(0xb8u64))))) as u64; rdx = (((rdx) as i64).wrapping_mul((r10) as i64) as u64) as u64; rax = rdx; rax = ((rax).wrapping_shr(47u32)) as u64; rdx = (((rdx) ^ (rax))) as u64; r8 = (((r8) ^ (rdx))) as u64; r8 = (((r8) as i64).wrapping_mul((r10) as i64) as u64) as u64; rax = r8; rax = ((rax).wrapping_shr(47u32)) as u64; rax = (((rax) ^ (r8))) as u64; rax = (((rax) as i64).wrapping_mul((r10) as i64) as u64) as u64; rdx = rbp; rdx = ((rdx).wrapping_shr(47u32)) as u64; rdx = (((rdx) ^ (rbp))) as u64; rdx = (((rdx) as i64).wrapping_mul((r12) as i64) as u64) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; rdx = ((rdx).wrapping_add(rsi)) as u64; rdx = (((rdx) ^ (rcx))) as u64; rdx = (((rdx) as i64).wrapping_mul((r10) as i64) as u64) as u64; rax = rdx; rax = ((rax).wrapping_shr(47u32)) as u64; rdx = (((rdx) ^ (rax))) as u64; rcx = (((rcx) ^ (rdx))) as u64; rcx = (((rcx) as i64).wrapping_mul((r10) as i64) as u64) as u64; rax = rcx; rax = ((rax).wrapping_shr(47u32)) as u64; rcx = (((rcx) ^ (rax))) as u64; rcx = (((rcx) as i64).wrapping_mul((r10) as i64) as u64) as u64; bb = 10; continue; }
            10 => { rax = (0xde5fb9d2630458e9u64) as u64; let prod = (rax as u128).wrapping_mul((rcx) as u128); rax = prod as u64; rdx = (prod >> 64) as u64; r9 = ((rdx.wrapping_add(rax)) as u64) as u64; r8 = rsp.wrapping_add(0x30); rdx = rsp.wrapping_add(0x20); rcx = P2; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; fl_l = (crate::simd_ops::ru8(rsp.wrapping_add(0x28u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 11; continue; } rbx = ((rdi.wrapping_add(rdi.wrapping_mul(2))) as u64) as u64; rbx = ((rbx).wrapping_shl(4u32)) as u64; rbx = ((rbx).wrapping_add(crate::simd_ops::ru64(P4))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xb0u64), rbx); crate::simd_ops::lk_and_u32(rbx, (0xff000004u32) as u32); crate::simd_ops::lk_or_u32(rbx, (0x4u32) as u32); crate::simd_ops::wu8(rbx.wrapping_add(0x3u64), (0x4u8) as u8); crate::simd_ops::wu32(rbx.wrapping_add(0x4u64), (0x0u32) as u32); crate::simd_ops::wu8(rbx.wrapping_add(0x8u64), (0x0u8) as u8); rdx = rsp.wrapping_add(0x30); rcx = rbx; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx.wrapping_add(0x28u64), (0x0u64) as u64); bb = 11; continue; }
            11 => { rax = ((rdi.wrapping_add(rdi.wrapping_mul(2))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rax = ((rax).wrapping_add(crate::simd_ops::ru64(P4))) as u64; rcx = P6; crate::simd_ops::wu64(rax.wrapping_add(0x28u64), rcx); rcx = rsp.wrapping_add(0x30); rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); rax = 0; fl_l = 0; fl_r = 0; rsp = ((rsp).wrapping_add(0x68u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_36633<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 21>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x68u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (0x4000004u64) as u64); crate::simd_ops::wu8(rsp.wrapping_add(0x38u64), (0x0u8) as u8); rax = P0; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (0x19u64) as u64); r9 = 0x19; r8 = 0; fl_l = 0; fl_r = 0; rdx = rsp.wrapping_add(0x20); rcx = rsp.wrapping_add(0x30); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru32(rsp.wrapping_add(0x34u64))) as u32 as u64; rcx = (crate::simd_ops::ru32(rsp.wrapping_add(0x30u64))) as u32 as u64; rax = ((rcx as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_shr(24u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x80; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 1; continue; } rax = ((rax.wrapping_mul(8)) as u32) as u32 as u64; bb = 2; continue; }
            1 => { rax = (((rax as u32)).wrapping_shl(8u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x7c00; rax = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 2; continue; }
            2 => { rcx = ((((rcx as u32)) & (0xffffffu32))) as u32 as u64; rcx = (((rcx as u32)).wrapping_shl(3u32)) as u32 as u64; r15 = rsp.wrapping_add(0x38); fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; r15 = (if (((fl_l as u32)) > ((fl_r as u32))) { crate::simd_ops::ru64(rsp.wrapping_add(0x38u64)) } else { r15 }) as u64; fl_l = rbx; fl_r = (0x20u64) as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 7; continue; } fl_l = rbx; fl_r = (0x10u64) as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 6; continue; } fl_l = rbx; fl_r = (0x8u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 3; continue; } r8 = (0x9ae16a3b2f90404fu64) as u64; r9 = ((r8.wrapping_add(rbx.wrapping_mul(2))) as u64) as u64; r8 = (crate::simd_ops::ru64(r15)) as u64; rax = (0x651e95c4d06fbfb1u64) as u64; r8 = ((r8).wrapping_sub(rax)) as u64; rdx = (crate::simd_ops::ru64(rbx.wrapping_add(r15).wrapping_sub(0x8u64))) as u64; rdx = ((rdx).rotate_right(37u32)) as u64; rdx = (((rdx) as i64).wrapping_mul((r9) as i64) as u64) as u64; rdx = ((rdx).wrapping_add(r8)) as u64; r8 = ((r8).rotate_right(25u32)) as u64; r8 = ((r8).wrapping_add(crate::simd_ops::ru64(rbx.wrapping_add(r15).wrapping_sub(0x8u64)))) as u64; r8 = (((r8) as i64).wrapping_mul((r9) as i64) as u64) as u64; rcx = r8; rcx = (((rcx) ^ (rdx))) as u64; rcx = (((rcx) as i64).wrapping_mul((r9) as i64) as u64) as u64; rax = rcx; rax = ((rax).wrapping_shr(47u32)) as u64; rcx = (((rcx) ^ (rax))) as u64; rcx = (((rcx) ^ (r8))) as u64; rcx = (((rcx) as i64).wrapping_mul((r9) as i64) as u64) as u64; rax = rcx; rax = ((rax).wrapping_shr(47u32)) as u64; rcx = (((rcx) ^ (rax))) as u64; rcx = (((rcx) as i64).wrapping_mul((r9) as i64) as u64) as u64; bb = 10; continue; }
            3 => { fl_l = rbx; fl_r = (0x4u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 4; continue; } r8 = (0x9ae16a3b2f90404fu64) as u64; r8 = ((r8.wrapping_add(rbx.wrapping_mul(2))) as u64) as u64; rax = (crate::simd_ops::ru32(r15)) as u32 as u64; rdx = (crate::simd_ops::ru32(rbx.wrapping_add(r15).wrapping_sub(0x4u64))) as u32 as u64; rcx = ((rbx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rcx = (((rcx) ^ (rdx))) as u64; rcx = (((rcx) as i64).wrapping_mul((r8) as i64) as u64) as u64; rax = rcx; rax = ((rax).wrapping_shr(47u32)) as u64; rcx = (((rcx) ^ (rax))) as u64; rcx = (((rcx) ^ (rdx))) as u64; rcx = (((rcx) as i64).wrapping_mul((r8) as i64) as u64) as u64; rax = rcx; rax = ((rax).wrapping_shr(47u32)) as u64; rcx = (((rcx) ^ (rax))) as u64; rcx = (((rcx) as i64).wrapping_mul((r8) as i64) as u64) as u64; bb = 10; continue; }
            4 => { fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } rax = rbx; rax = ((rax).wrapping_shr(1u32)) as u64; rdx = ((crate::simd_ops::ru8(rax.wrapping_add(r15))) as u32) as u32 as u64; rax = (0xe16a3b2f90404f00u64) as u64; rdx = (((rdx) as i64).wrapping_mul((rax) as i64) as u64) as u64; rax = ((crate::simd_ops::ru8(r15)) as u32) as u32 as u64; r8 = (0x9ae16a3b2f90404fu64) as u64; rax = (((rax) as i64).wrapping_mul((r8) as i64) as u64) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; rax = ((crate::simd_ops::ru8(rbx.wrapping_add(r15).wrapping_sub(0x1u64))) as u32) as u32 as u64; rax = ((rbx.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = ((rax as u32)) as u32 as u64; rcx = (0xc3a5c85c97cb3127u64) as u64; rax = (((rax) as i64).wrapping_mul((rcx) as i64) as u64) as u64; rdx = (((rdx) ^ (rax))) as u64; rcx = rdx; rcx = ((rcx).wrapping_shr(47u32)) as u64; rcx = (((rcx) ^ (rdx))) as u64; rcx = (((rcx) as i64).wrapping_mul((r8) as i64) as u64) as u64; bb = 10; continue; }
            5 => { rcx = (0x9ae16a3b2f90404fu64) as u64; bb = 10; continue; }
            6 => { r8 = (0x9ae16a3b2f90404fu64) as u64; r11 = ((r8.wrapping_add(rbx.wrapping_mul(2))) as u64) as u64; r12 = (0xb492b66fbe98f273u64) as u64; r9 = (crate::simd_ops::ru64(r15)) as u64; r9 = (((r9) as i64).wrapping_mul((r12) as i64) as u64) as u64; rdx = (crate::simd_ops::ru64(r15.wrapping_add(0x8u64))) as u64; r10 = r11; r10 = (((r10) as i64).wrapping_mul((crate::simd_ops::ru64(rbx.wrapping_add(r15).wrapping_sub(0x8u64))) as i64) as u64) as u64; rcx = (crate::simd_ops::ru64(rbx.wrapping_add(r15).wrapping_sub(0x10u64))) as u64; rcx = (((rcx) as i64).wrapping_mul((r8) as i64) as u64) as u64; r8 = ((r8).wrapping_add(rdx)) as u64; r8 = ((r8).rotate_right(18u32)) as u64; r8 = ((r8).wrapping_add(r10)) as u64; r8 = ((r8).wrapping_add(r9)) as u64; rax = ((rdx.wrapping_add(r9)) as u64) as u64; rax = ((rax).rotate_right(43u32)) as u64; r10 = ((r10).rotate_right(30u32)) as u64; rcx = ((rcx).wrapping_add(rax)) as u64; rcx = ((rcx).wrapping_add(r10)) as u64; rcx = (((rcx) ^ (r8))) as u64; rcx = (((rcx) as i64).wrapping_mul((r11) as i64) as u64) as u64; rax = rcx; rax = ((rax).wrapping_shr(47u32)) as u64; rcx = (((rcx) ^ (rax))) as u64; rcx = (((rcx) ^ (r8))) as u64; rcx = (((rcx) as i64).wrapping_mul((r11) as i64) as u64) as u64; rax = rcx; rax = ((rax).wrapping_shr(47u32)) as u64; rcx = (((rcx) ^ (rax))) as u64; rcx = (((rcx) as i64).wrapping_mul((r11) as i64) as u64) as u64; bb = 10; continue; }
            7 => { r13 = ((rbx.wrapping_add(r15)) as u64) as u64; fl_l = rbx; fl_r = (0x40u64) as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 8; continue; } r8 = (0x9ae16a3b2f90404fu64) as u64; r14 = ((r8.wrapping_add(rbx.wrapping_mul(2))) as u64) as u64; r11 = (crate::simd_ops::ru64(r13.wrapping_sub(0x18u64))) as u64; r10 = (crate::simd_ops::ru64(r15.wrapping_add(0x10u64))) as u64; r10 = (((r10) as i64).wrapping_mul((r8) as i64) as u64) as u64; rax = (crate::simd_ops::ru64(r15.wrapping_add(0x18u64))) as u64; r9 = ((rax.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rdi = r14; rdi = (((rdi) as i64).wrapping_mul((crate::simd_ops::ru64(r13.wrapping_sub(0x10u64))) as i64) as u64) as u64; rdx = (crate::simd_ops::ru64(r15)) as u64; rdx = (((rdx) as i64).wrapping_mul((r8) as i64) as u64) as u64; rdx = ((rdx).wrapping_add(crate::simd_ops::ru64(r13.wrapping_sub(0x8u64)))) as u64; rax = (crate::simd_ops::ru64(r15.wrapping_add(0x8u64))) as u64; rax = ((rax).rotate_right(30u32)) as u64; rax = ((rax).wrapping_add(r11)) as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rax = rdx; rax = ((rax).rotate_right(43u32)) as u64; rcx = ((rcx).wrapping_add(rax)) as u64; r8 = (crate::simd_ops::ru64(r13.wrapping_sub(0x20u64))) as u64; r8 = (((r8) ^ (rdx))) as u64; r8 = ((r8).wrapping_add(1)) as u64; r8 = ((r8).wrapping_add(r9)) as u64; rdx = ((r8.wrapping_add(rcx)) as u64) as u64; rdx = (((rdx) as i64).wrapping_mul((r14) as i64) as u64) as u64; rdx = ((rdx).swap_bytes()) as u64; rdx = ((rdx).wrapping_add(rdi)) as u64; rax = ((r9.wrapping_add(r10)) as u64) as u64; r9 = rax; r9 = ((r9).rotate_right(42u32)) as u64; r9 = ((r9).wrapping_add(r11)) as u64; rcx = ((rax.wrapping_add(r11)) as u64) as u64; rax = ((rdx.wrapping_add(r8)) as u64) as u64; rax = (((rax) as i64).wrapping_mul((r14) as i64) as u64) as u64; rax = ((rax).swap_bytes()) as u64; rax = ((rax).wrapping_add(rcx)) as u64; rax = ((rax).wrapping_add(r9)) as u64; rax = ((rax).wrapping_add(crate::simd_ops::ru64(r13.wrapping_sub(0x8u64)))) as u64; rax = (((rax) as i64).wrapping_mul((r14) as i64) as u64) as u64; rax = ((rax).swap_bytes()) as u64; rax = ((rax).wrapping_add(crate::simd_ops::ru64(r15.wrapping_add(0x8u64)))) as u64; rcx = ((rcx).wrapping_add(rax)) as u64; rcx = (((rcx) as i64).wrapping_mul((r14) as i64) as u64) as u64; rcx = ((rcx).wrapping_add(rdi)) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(r13.wrapping_sub(0x20u64)))) as u64; rax = rcx; rax = ((rax).wrapping_shr(47u32)) as u64; rcx = (((rcx) ^ (rax))) as u64; rcx = (((rcx) as i64).wrapping_mul((r14) as i64) as u64) as u64; fl_l = ((rcx).wrapping_add(r9)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; bb = 10; continue; }
            8 => { rdi = (crate::simd_ops::ru64(r13.wrapping_sub(0x28u64))) as u64; rbp = (crate::simd_ops::ru64(r13.wrapping_sub(0x38u64))) as u64; rbp = ((rbp).wrapping_add(crate::simd_ops::ru64(r13.wrapping_sub(0x10u64)))) as u64; r14 = (crate::simd_ops::ru64(r13.wrapping_sub(0x30u64))) as u64; r14 = ((r14).wrapping_add(rbx)) as u64; r14 = (((r14) ^ (crate::simd_ops::ru64(r13.wrapping_sub(0x18u64))))) as u64; rdx = (0x9ddfea08eb382d69u64) as u64; r14 = (((r14) as i64).wrapping_mul((rdx) as i64) as u64) as u64; rax = r14; rax = ((rax).wrapping_shr(47u32)) as u64; r14 = (((r14) ^ (rax))) as u64; r14 = (((r14) ^ (crate::simd_ops::ru64(r13.wrapping_sub(0x18u64))))) as u64; r14 = (((r14) as i64).wrapping_mul((rdx) as i64) as u64) as u64; rax = r14; rax = ((rax).wrapping_shr(47u32)) as u64; r14 = (((r14) ^ (rax))) as u64; r14 = (((r14) as i64).wrapping_mul((rdx) as i64) as u64) as u64; rax = (crate::simd_ops::ru64(r13.wrapping_sub(0x30u64))) as u64; r10 = (crate::simd_ops::ru64(r13.wrapping_sub(0x40u64))) as u64; r10 = ((r10).wrapping_add(rbx)) as u64; r9 = ((rdi.wrapping_add(r10)) as u64) as u64; r9 = ((r9).wrapping_add(r14)) as u64; r8 = r10; rax = ((rax).wrapping_add(crate::simd_ops::ru64(r13.wrapping_sub(0x38u64)))) as u64; r10 = ((r10).wrapping_add(rax)) as u64; r9 = ((r9).rotate_right(21u32)) as u64; rax = r10; rax = ((rax).rotate_right(44u32)) as u64; rax = ((rax).wrapping_add(r9)) as u64; rax = ((rax).wrapping_add(r8)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xc0u64), rax); r9 = ((rdi.wrapping_add(r10)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xb8u64), r9); r11 = (crate::simd_ops::ru64(r13.wrapping_sub(0x8u64))) as u64; rax = (crate::simd_ops::ru64(r13.wrapping_sub(0x10u64))) as u64; rcx = (crate::simd_ops::ru64(r13.wrapping_sub(0x18u64))) as u64; r10 = (crate::simd_ops::ru64(r13.wrapping_sub(0x20u64))) as u64; r12 = (0xb492b66fbe98f273u64) as u64; r10 = ((r10).wrapping_add(r12)) as u64; r10 = ((r10).wrapping_add(rbp)) as u64; r9 = ((r11.wrapping_add(r10)) as u64) as u64; r9 = ((r9).wrapping_add(rdi)) as u64; r13 = r10; rax = ((rax).wrapping_add(rcx)) as u64; r10 = ((r10).wrapping_add(rax)) as u64; r9 = ((r9).rotate_right(21u32)) as u64; rax = r10; rax = ((rax).rotate_right(44u32)) as u64; r9 = ((r9).wrapping_add(rax)) as u64; r13 = ((r13).wrapping_add(r9)) as u64; r8 = ((r11.wrapping_add(r10)) as u64) as u64; rdi = (((rdi) as i64).wrapping_mul((r12) as i64) as u64) as u64; rdi = ((rdi).wrapping_add(crate::simd_ops::ru64(r15))) as u64; rbx = ((rbx).wrapping_sub(1)) as u64; fl_l = (((rbx) & (0xffffffffffffffc0u64))) as u64; fl_r = fl_l; rbx = ((fl_l as u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xc8u64), rbx); rax = r15.wrapping_add(0x30); crate::simd_ops::wu64(rsp.wrapping_add(0xb0u64), rax); r15 = rdi; bb = 9; continue; }
            9 => { r11 = (crate::simd_ops::ru64(rax.wrapping_sub(0x28u64))) as u64; rsi = ((r11.wrapping_add(rbp)) as u64) as u64; rsi = ((rsi).wrapping_add(r15)) as u64; rsi = ((rsi).wrapping_add(crate::simd_ops::ru64(rsp.wrapping_add(0xb8u64)))) as u64; rdi = (crate::simd_ops::ru64(rax)) as u64; rbp = ((rbp).wrapping_add(rdi)) as u64; rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0xc0u64))) as u64; rbp = ((rbp).wrapping_add(rdx)) as u64; rsi = ((rsi).rotate_right(37u32)) as u64; rsi = (((rsi) as i64).wrapping_mul((r12) as i64) as u64) as u64; rsi = (((rsi) ^ (r13))) as u64; rbx = (crate::simd_ops::ru64(rax.wrapping_sub(0x8u64))) as u64; rbp = ((rbp).rotate_right(42u32)) as u64; rbp = (((rbp) as i64).wrapping_mul((r12) as i64) as u64) as u64; rbp = ((rbp).wrapping_add(rbx)) as u64; rbp = ((rbp).wrapping_add(crate::simd_ops::ru64(rsp.wrapping_add(0xb8u64)))) as u64; r15 = ((r14.wrapping_add(r8)) as u64) as u64; r15 = ((r15).rotate_right(33u32)) as u64; r15 = (((r15) as i64).wrapping_mul((r12) as i64) as u64) as u64; rcx = ((rsi.wrapping_add(r8)) as u64) as u64; rdx = (((rdx) as i64).wrapping_mul((r12) as i64) as u64) as u64; r9 = (crate::simd_ops::ru64(rax.wrapping_sub(0x18u64))) as u64; r10 = (crate::simd_ops::ru64(rax.wrapping_sub(0x20u64))) as u64; r8 = (crate::simd_ops::ru64(rax.wrapping_sub(0x30u64))) as u64; r8 = ((r8).wrapping_add(rdx)) as u64; rdx = ((r9.wrapping_add(r8)) as u64) as u64; rdx = ((rdx).wrapping_add(rcx)) as u64; rcx = r8; rax = ((r10.wrapping_add(r11)) as u64) as u64; r8 = ((r8).wrapping_add(rax)) as u64; rdx = ((rdx).rotate_right(21u32)) as u64; r11 = r8; r11 = ((r11).rotate_right(44u32)) as u64; r11 = ((r11).wrapping_add(rdx)) as u64; r11 = ((r11).wrapping_add(rcx)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xc0u64), r11); rax = ((r9.wrapping_add(r8)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xb8u64), rax); rdx = ((r10.wrapping_add(rbp)) as u64) as u64; r9 = ((r15.wrapping_add(r13)) as u64) as u64; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xb0u64))) as u64; r10 = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; r9 = ((r9).wrapping_add(crate::simd_ops::ru64(rax.wrapping_sub(0x10u64)))) as u64; r8 = ((r10.wrapping_add(r9)) as u64) as u64; r8 = ((r8).wrapping_add(rdx)) as u64; r13 = r9; rax = ((rdi.wrapping_add(rbx)) as u64) as u64; r9 = ((r9).wrapping_add(rax)) as u64; r8 = ((r8).rotate_right(21u32)) as u64; rax = r9; rax = ((rax).rotate_right(44u32)) as u64; r8 = ((r8).wrapping_add(rax)) as u64; r13 = ((r13).wrapping_add(r8)) as u64; r8 = ((r10.wrapping_add(r9)) as u64) as u64; r14 = rsi; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xb0u64))) as u64; rax = ((rax).wrapping_add(0x40u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xb0u64), rax); fl_l = (crate::simd_ops::ru64(rsp.wrapping_add(0xc8u64))) as u64; fl_r = (0x40u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xc8u64), ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64); if (((fl_l as u64)) != ((fl_r as u64))) { bb = 9; continue; } rcx = r13; rcx = (((rcx) ^ (r11))) as u64; r10 = (0x9ddfea08eb382d69u64) as u64; rcx = (((rcx) as i64).wrapping_mul((r10) as i64) as u64) as u64; rax = rcx; rax = ((rax).wrapping_shr(47u32)) as u64; rcx = (((rcx) ^ (rax))) as u64; r13 = (((r13) ^ (rcx))) as u64; r13 = (((r13) as i64).wrapping_mul((r10) as i64) as u64) as u64; rcx = r13; rcx = ((rcx).wrapping_shr(47u32)) as u64; rcx = (((rcx) ^ (r13))) as u64; rcx = (((rcx) as i64).wrapping_mul((r10) as i64) as u64) as u64; rcx = ((rcx).wrapping_add(r15)) as u64; rdx = r8; rdx = (((rdx) ^ (crate::simd_ops::ru64(rsp.wrapping_add(0xb8u64))))) as u64; rdx = (((rdx) as i64).wrapping_mul((r10) as i64) as u64) as u64; rax = rdx; rax = ((rax).wrapping_shr(47u32)) as u64; rdx = (((rdx) ^ (rax))) as u64; r8 = (((r8) ^ (rdx))) as u64; r8 = (((r8) as i64).wrapping_mul((r10) as i64) as u64) as u64; rax = r8; rax = ((rax).wrapping_shr(47u32)) as u64; rax = (((rax) ^ (r8))) as u64; rax = (((rax) as i64).wrapping_mul((r10) as i64) as u64) as u64; rdx = rbp; rdx = ((rdx).wrapping_shr(47u32)) as u64; rdx = (((rdx) ^ (rbp))) as u64; rdx = (((rdx) as i64).wrapping_mul((r12) as i64) as u64) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; rdx = ((rdx).wrapping_add(rsi)) as u64; rdx = (((rdx) ^ (rcx))) as u64; rdx = (((rdx) as i64).wrapping_mul((r10) as i64) as u64) as u64; rax = rdx; rax = ((rax).wrapping_shr(47u32)) as u64; rdx = (((rdx) ^ (rax))) as u64; rcx = (((rcx) ^ (rdx))) as u64; rcx = (((rcx) as i64).wrapping_mul((r10) as i64) as u64) as u64; rax = rcx; rax = ((rax).wrapping_shr(47u32)) as u64; rcx = (((rcx) ^ (rax))) as u64; rcx = (((rcx) as i64).wrapping_mul((r10) as i64) as u64) as u64; bb = 10; continue; }
            10 => { rax = (0xde5fb9d2630458e9u64) as u64; let prod = (rax as u128).wrapping_mul((rcx) as u128); rax = prod as u64; rdx = (prod >> 64) as u64; r9 = ((rdx.wrapping_add(rax)) as u64) as u64; r8 = rsp.wrapping_add(0x30); rdx = rsp.wrapping_add(0x20); rcx = P2; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; fl_l = (crate::simd_ops::ru8(rsp.wrapping_add(0x28u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 11; continue; } rbx = ((rdi.wrapping_add(rdi.wrapping_mul(2))) as u64) as u64; rbx = ((rbx).wrapping_shl(4u32)) as u64; rbx = ((rbx).wrapping_add(crate::simd_ops::ru64(P4))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xb0u64), rbx); crate::simd_ops::lk_and_u32(rbx, (0xff000004u32) as u32); crate::simd_ops::lk_or_u32(rbx, (0x4u32) as u32); crate::simd_ops::wu8(rbx.wrapping_add(0x3u64), (0x4u8) as u8); crate::simd_ops::wu32(rbx.wrapping_add(0x4u64), (0x0u32) as u32); crate::simd_ops::wu8(rbx.wrapping_add(0x8u64), (0x0u8) as u8); rdx = rsp.wrapping_add(0x30); rcx = rbx; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx.wrapping_add(0x28u64), (0x0u64) as u64); bb = 11; continue; }
            11 => { rax = ((rdi.wrapping_add(rdi.wrapping_mul(2))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rax = ((rax).wrapping_add(crate::simd_ops::ru64(P4))) as u64; rcx = P6; crate::simd_ops::wu64(rax.wrapping_add(0x28u64), rcx); rcx = rsp.wrapping_add(0x30); rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); rax = 0; fl_l = 0; fl_r = 0; rsp = ((rsp).wrapping_add(0x68u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_36634<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 21>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x68u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (0x4000004u64) as u64); crate::simd_ops::wu8(rsp.wrapping_add(0x38u64), (0x0u8) as u8); rax = P0; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (0x18u64) as u64); r9 = 0x18; r8 = 0; fl_l = 0; fl_r = 0; rdx = rsp.wrapping_add(0x20); rcx = rsp.wrapping_add(0x30); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru32(rsp.wrapping_add(0x34u64))) as u32 as u64; rcx = (crate::simd_ops::ru32(rsp.wrapping_add(0x30u64))) as u32 as u64; rax = ((rcx as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_shr(24u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x80; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 1; continue; } rax = ((rax.wrapping_mul(8)) as u32) as u32 as u64; bb = 2; continue; }
            1 => { rax = (((rax as u32)).wrapping_shl(8u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x7c00; rax = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 2; continue; }
            2 => { rcx = ((((rcx as u32)) & (0xffffffu32))) as u32 as u64; rcx = (((rcx as u32)).wrapping_shl(3u32)) as u32 as u64; r15 = rsp.wrapping_add(0x38); fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; r15 = (if (((fl_l as u32)) > ((fl_r as u32))) { crate::simd_ops::ru64(rsp.wrapping_add(0x38u64)) } else { r15 }) as u64; fl_l = rbx; fl_r = (0x20u64) as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 7; continue; } fl_l = rbx; fl_r = (0x10u64) as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 6; continue; } fl_l = rbx; fl_r = (0x8u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 3; continue; } r8 = (0x9ae16a3b2f90404fu64) as u64; r9 = ((r8.wrapping_add(rbx.wrapping_mul(2))) as u64) as u64; r8 = (crate::simd_ops::ru64(r15)) as u64; rax = (0x651e95c4d06fbfb1u64) as u64; r8 = ((r8).wrapping_sub(rax)) as u64; rdx = (crate::simd_ops::ru64(rbx.wrapping_add(r15).wrapping_sub(0x8u64))) as u64; rdx = ((rdx).rotate_right(37u32)) as u64; rdx = (((rdx) as i64).wrapping_mul((r9) as i64) as u64) as u64; rdx = ((rdx).wrapping_add(r8)) as u64; r8 = ((r8).rotate_right(25u32)) as u64; r8 = ((r8).wrapping_add(crate::simd_ops::ru64(rbx.wrapping_add(r15).wrapping_sub(0x8u64)))) as u64; r8 = (((r8) as i64).wrapping_mul((r9) as i64) as u64) as u64; rcx = r8; rcx = (((rcx) ^ (rdx))) as u64; rcx = (((rcx) as i64).wrapping_mul((r9) as i64) as u64) as u64; rax = rcx; rax = ((rax).wrapping_shr(47u32)) as u64; rcx = (((rcx) ^ (rax))) as u64; rcx = (((rcx) ^ (r8))) as u64; rcx = (((rcx) as i64).wrapping_mul((r9) as i64) as u64) as u64; rax = rcx; rax = ((rax).wrapping_shr(47u32)) as u64; rcx = (((rcx) ^ (rax))) as u64; rcx = (((rcx) as i64).wrapping_mul((r9) as i64) as u64) as u64; bb = 10; continue; }
            3 => { fl_l = rbx; fl_r = (0x4u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 4; continue; } r8 = (0x9ae16a3b2f90404fu64) as u64; r8 = ((r8.wrapping_add(rbx.wrapping_mul(2))) as u64) as u64; rax = (crate::simd_ops::ru32(r15)) as u32 as u64; rdx = (crate::simd_ops::ru32(rbx.wrapping_add(r15).wrapping_sub(0x4u64))) as u32 as u64; rcx = ((rbx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rcx = (((rcx) ^ (rdx))) as u64; rcx = (((rcx) as i64).wrapping_mul((r8) as i64) as u64) as u64; rax = rcx; rax = ((rax).wrapping_shr(47u32)) as u64; rcx = (((rcx) ^ (rax))) as u64; rcx = (((rcx) ^ (rdx))) as u64; rcx = (((rcx) as i64).wrapping_mul((r8) as i64) as u64) as u64; rax = rcx; rax = ((rax).wrapping_shr(47u32)) as u64; rcx = (((rcx) ^ (rax))) as u64; rcx = (((rcx) as i64).wrapping_mul((r8) as i64) as u64) as u64; bb = 10; continue; }
            4 => { fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } rax = rbx; rax = ((rax).wrapping_shr(1u32)) as u64; rdx = ((crate::simd_ops::ru8(rax.wrapping_add(r15))) as u32) as u32 as u64; rax = (0xe16a3b2f90404f00u64) as u64; rdx = (((rdx) as i64).wrapping_mul((rax) as i64) as u64) as u64; rax = ((crate::simd_ops::ru8(r15)) as u32) as u32 as u64; r8 = (0x9ae16a3b2f90404fu64) as u64; rax = (((rax) as i64).wrapping_mul((r8) as i64) as u64) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; rax = ((crate::simd_ops::ru8(rbx.wrapping_add(r15).wrapping_sub(0x1u64))) as u32) as u32 as u64; rax = ((rbx.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = ((rax as u32)) as u32 as u64; rcx = (0xc3a5c85c97cb3127u64) as u64; rax = (((rax) as i64).wrapping_mul((rcx) as i64) as u64) as u64; rdx = (((rdx) ^ (rax))) as u64; rcx = rdx; rcx = ((rcx).wrapping_shr(47u32)) as u64; rcx = (((rcx) ^ (rdx))) as u64; rcx = (((rcx) as i64).wrapping_mul((r8) as i64) as u64) as u64; bb = 10; continue; }
            5 => { rcx = (0x9ae16a3b2f90404fu64) as u64; bb = 10; continue; }
            6 => { r8 = (0x9ae16a3b2f90404fu64) as u64; r11 = ((r8.wrapping_add(rbx.wrapping_mul(2))) as u64) as u64; r12 = (0xb492b66fbe98f273u64) as u64; r9 = (crate::simd_ops::ru64(r15)) as u64; r9 = (((r9) as i64).wrapping_mul((r12) as i64) as u64) as u64; rdx = (crate::simd_ops::ru64(r15.wrapping_add(0x8u64))) as u64; r10 = r11; r10 = (((r10) as i64).wrapping_mul((crate::simd_ops::ru64(rbx.wrapping_add(r15).wrapping_sub(0x8u64))) as i64) as u64) as u64; rcx = (crate::simd_ops::ru64(rbx.wrapping_add(r15).wrapping_sub(0x10u64))) as u64; rcx = (((rcx) as i64).wrapping_mul((r8) as i64) as u64) as u64; r8 = ((r8).wrapping_add(rdx)) as u64; r8 = ((r8).rotate_right(18u32)) as u64; r8 = ((r8).wrapping_add(r10)) as u64; r8 = ((r8).wrapping_add(r9)) as u64; rax = ((rdx.wrapping_add(r9)) as u64) as u64; rax = ((rax).rotate_right(43u32)) as u64; r10 = ((r10).rotate_right(30u32)) as u64; rcx = ((rcx).wrapping_add(rax)) as u64; rcx = ((rcx).wrapping_add(r10)) as u64; rcx = (((rcx) ^ (r8))) as u64; rcx = (((rcx) as i64).wrapping_mul((r11) as i64) as u64) as u64; rax = rcx; rax = ((rax).wrapping_shr(47u32)) as u64; rcx = (((rcx) ^ (rax))) as u64; rcx = (((rcx) ^ (r8))) as u64; rcx = (((rcx) as i64).wrapping_mul((r11) as i64) as u64) as u64; rax = rcx; rax = ((rax).wrapping_shr(47u32)) as u64; rcx = (((rcx) ^ (rax))) as u64; rcx = (((rcx) as i64).wrapping_mul((r11) as i64) as u64) as u64; bb = 10; continue; }
            7 => { r13 = ((rbx.wrapping_add(r15)) as u64) as u64; fl_l = rbx; fl_r = (0x40u64) as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 8; continue; } r8 = (0x9ae16a3b2f90404fu64) as u64; r14 = ((r8.wrapping_add(rbx.wrapping_mul(2))) as u64) as u64; r11 = (crate::simd_ops::ru64(r13.wrapping_sub(0x18u64))) as u64; r10 = (crate::simd_ops::ru64(r15.wrapping_add(0x10u64))) as u64; r10 = (((r10) as i64).wrapping_mul((r8) as i64) as u64) as u64; rax = (crate::simd_ops::ru64(r15.wrapping_add(0x18u64))) as u64; r9 = ((rax.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rdi = r14; rdi = (((rdi) as i64).wrapping_mul((crate::simd_ops::ru64(r13.wrapping_sub(0x10u64))) as i64) as u64) as u64; rdx = (crate::simd_ops::ru64(r15)) as u64; rdx = (((rdx) as i64).wrapping_mul((r8) as i64) as u64) as u64; rdx = ((rdx).wrapping_add(crate::simd_ops::ru64(r13.wrapping_sub(0x8u64)))) as u64; rax = (crate::simd_ops::ru64(r15.wrapping_add(0x8u64))) as u64; rax = ((rax).rotate_right(30u32)) as u64; rax = ((rax).wrapping_add(r11)) as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rax = rdx; rax = ((rax).rotate_right(43u32)) as u64; rcx = ((rcx).wrapping_add(rax)) as u64; r8 = (crate::simd_ops::ru64(r13.wrapping_sub(0x20u64))) as u64; r8 = (((r8) ^ (rdx))) as u64; r8 = ((r8).wrapping_add(1)) as u64; r8 = ((r8).wrapping_add(r9)) as u64; rdx = ((r8.wrapping_add(rcx)) as u64) as u64; rdx = (((rdx) as i64).wrapping_mul((r14) as i64) as u64) as u64; rdx = ((rdx).swap_bytes()) as u64; rdx = ((rdx).wrapping_add(rdi)) as u64; rax = ((r9.wrapping_add(r10)) as u64) as u64; r9 = rax; r9 = ((r9).rotate_right(42u32)) as u64; r9 = ((r9).wrapping_add(r11)) as u64; rcx = ((rax.wrapping_add(r11)) as u64) as u64; rax = ((rdx.wrapping_add(r8)) as u64) as u64; rax = (((rax) as i64).wrapping_mul((r14) as i64) as u64) as u64; rax = ((rax).swap_bytes()) as u64; rax = ((rax).wrapping_add(rcx)) as u64; rax = ((rax).wrapping_add(r9)) as u64; rax = ((rax).wrapping_add(crate::simd_ops::ru64(r13.wrapping_sub(0x8u64)))) as u64; rax = (((rax) as i64).wrapping_mul((r14) as i64) as u64) as u64; rax = ((rax).swap_bytes()) as u64; rax = ((rax).wrapping_add(crate::simd_ops::ru64(r15.wrapping_add(0x8u64)))) as u64; rcx = ((rcx).wrapping_add(rax)) as u64; rcx = (((rcx) as i64).wrapping_mul((r14) as i64) as u64) as u64; rcx = ((rcx).wrapping_add(rdi)) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(r13.wrapping_sub(0x20u64)))) as u64; rax = rcx; rax = ((rax).wrapping_shr(47u32)) as u64; rcx = (((rcx) ^ (rax))) as u64; rcx = (((rcx) as i64).wrapping_mul((r14) as i64) as u64) as u64; fl_l = ((rcx).wrapping_add(r9)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; bb = 10; continue; }
            8 => { rdi = (crate::simd_ops::ru64(r13.wrapping_sub(0x28u64))) as u64; rbp = (crate::simd_ops::ru64(r13.wrapping_sub(0x38u64))) as u64; rbp = ((rbp).wrapping_add(crate::simd_ops::ru64(r13.wrapping_sub(0x10u64)))) as u64; r14 = (crate::simd_ops::ru64(r13.wrapping_sub(0x30u64))) as u64; r14 = ((r14).wrapping_add(rbx)) as u64; r14 = (((r14) ^ (crate::simd_ops::ru64(r13.wrapping_sub(0x18u64))))) as u64; rdx = (0x9ddfea08eb382d69u64) as u64; r14 = (((r14) as i64).wrapping_mul((rdx) as i64) as u64) as u64; rax = r14; rax = ((rax).wrapping_shr(47u32)) as u64; r14 = (((r14) ^ (rax))) as u64; r14 = (((r14) ^ (crate::simd_ops::ru64(r13.wrapping_sub(0x18u64))))) as u64; r14 = (((r14) as i64).wrapping_mul((rdx) as i64) as u64) as u64; rax = r14; rax = ((rax).wrapping_shr(47u32)) as u64; r14 = (((r14) ^ (rax))) as u64; r14 = (((r14) as i64).wrapping_mul((rdx) as i64) as u64) as u64; rax = (crate::simd_ops::ru64(r13.wrapping_sub(0x30u64))) as u64; r10 = (crate::simd_ops::ru64(r13.wrapping_sub(0x40u64))) as u64; r10 = ((r10).wrapping_add(rbx)) as u64; r9 = ((rdi.wrapping_add(r10)) as u64) as u64; r9 = ((r9).wrapping_add(r14)) as u64; r8 = r10; rax = ((rax).wrapping_add(crate::simd_ops::ru64(r13.wrapping_sub(0x38u64)))) as u64; r10 = ((r10).wrapping_add(rax)) as u64; r9 = ((r9).rotate_right(21u32)) as u64; rax = r10; rax = ((rax).rotate_right(44u32)) as u64; rax = ((rax).wrapping_add(r9)) as u64; rax = ((rax).wrapping_add(r8)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xc0u64), rax); r9 = ((rdi.wrapping_add(r10)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xb8u64), r9); r11 = (crate::simd_ops::ru64(r13.wrapping_sub(0x8u64))) as u64; rax = (crate::simd_ops::ru64(r13.wrapping_sub(0x10u64))) as u64; rcx = (crate::simd_ops::ru64(r13.wrapping_sub(0x18u64))) as u64; r10 = (crate::simd_ops::ru64(r13.wrapping_sub(0x20u64))) as u64; r12 = (0xb492b66fbe98f273u64) as u64; r10 = ((r10).wrapping_add(r12)) as u64; r10 = ((r10).wrapping_add(rbp)) as u64; r9 = ((r11.wrapping_add(r10)) as u64) as u64; r9 = ((r9).wrapping_add(rdi)) as u64; r13 = r10; rax = ((rax).wrapping_add(rcx)) as u64; r10 = ((r10).wrapping_add(rax)) as u64; r9 = ((r9).rotate_right(21u32)) as u64; rax = r10; rax = ((rax).rotate_right(44u32)) as u64; r9 = ((r9).wrapping_add(rax)) as u64; r13 = ((r13).wrapping_add(r9)) as u64; r8 = ((r11.wrapping_add(r10)) as u64) as u64; rdi = (((rdi) as i64).wrapping_mul((r12) as i64) as u64) as u64; rdi = ((rdi).wrapping_add(crate::simd_ops::ru64(r15))) as u64; rbx = ((rbx).wrapping_sub(1)) as u64; fl_l = (((rbx) & (0xffffffffffffffc0u64))) as u64; fl_r = fl_l; rbx = ((fl_l as u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xc8u64), rbx); rax = r15.wrapping_add(0x30); crate::simd_ops::wu64(rsp.wrapping_add(0xb0u64), rax); r15 = rdi; bb = 9; continue; }
            9 => { r11 = (crate::simd_ops::ru64(rax.wrapping_sub(0x28u64))) as u64; rsi = ((r11.wrapping_add(rbp)) as u64) as u64; rsi = ((rsi).wrapping_add(r15)) as u64; rsi = ((rsi).wrapping_add(crate::simd_ops::ru64(rsp.wrapping_add(0xb8u64)))) as u64; rdi = (crate::simd_ops::ru64(rax)) as u64; rbp = ((rbp).wrapping_add(rdi)) as u64; rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0xc0u64))) as u64; rbp = ((rbp).wrapping_add(rdx)) as u64; rsi = ((rsi).rotate_right(37u32)) as u64; rsi = (((rsi) as i64).wrapping_mul((r12) as i64) as u64) as u64; rsi = (((rsi) ^ (r13))) as u64; rbx = (crate::simd_ops::ru64(rax.wrapping_sub(0x8u64))) as u64; rbp = ((rbp).rotate_right(42u32)) as u64; rbp = (((rbp) as i64).wrapping_mul((r12) as i64) as u64) as u64; rbp = ((rbp).wrapping_add(rbx)) as u64; rbp = ((rbp).wrapping_add(crate::simd_ops::ru64(rsp.wrapping_add(0xb8u64)))) as u64; r15 = ((r14.wrapping_add(r8)) as u64) as u64; r15 = ((r15).rotate_right(33u32)) as u64; r15 = (((r15) as i64).wrapping_mul((r12) as i64) as u64) as u64; rcx = ((rsi.wrapping_add(r8)) as u64) as u64; rdx = (((rdx) as i64).wrapping_mul((r12) as i64) as u64) as u64; r9 = (crate::simd_ops::ru64(rax.wrapping_sub(0x18u64))) as u64; r10 = (crate::simd_ops::ru64(rax.wrapping_sub(0x20u64))) as u64; r8 = (crate::simd_ops::ru64(rax.wrapping_sub(0x30u64))) as u64; r8 = ((r8).wrapping_add(rdx)) as u64; rdx = ((r9.wrapping_add(r8)) as u64) as u64; rdx = ((rdx).wrapping_add(rcx)) as u64; rcx = r8; rax = ((r10.wrapping_add(r11)) as u64) as u64; r8 = ((r8).wrapping_add(rax)) as u64; rdx = ((rdx).rotate_right(21u32)) as u64; r11 = r8; r11 = ((r11).rotate_right(44u32)) as u64; r11 = ((r11).wrapping_add(rdx)) as u64; r11 = ((r11).wrapping_add(rcx)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xc0u64), r11); rax = ((r9.wrapping_add(r8)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xb8u64), rax); rdx = ((r10.wrapping_add(rbp)) as u64) as u64; r9 = ((r15.wrapping_add(r13)) as u64) as u64; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xb0u64))) as u64; r10 = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; r9 = ((r9).wrapping_add(crate::simd_ops::ru64(rax.wrapping_sub(0x10u64)))) as u64; r8 = ((r10.wrapping_add(r9)) as u64) as u64; r8 = ((r8).wrapping_add(rdx)) as u64; r13 = r9; rax = ((rdi.wrapping_add(rbx)) as u64) as u64; r9 = ((r9).wrapping_add(rax)) as u64; r8 = ((r8).rotate_right(21u32)) as u64; rax = r9; rax = ((rax).rotate_right(44u32)) as u64; r8 = ((r8).wrapping_add(rax)) as u64; r13 = ((r13).wrapping_add(r8)) as u64; r8 = ((r10.wrapping_add(r9)) as u64) as u64; r14 = rsi; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xb0u64))) as u64; rax = ((rax).wrapping_add(0x40u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xb0u64), rax); fl_l = (crate::simd_ops::ru64(rsp.wrapping_add(0xc8u64))) as u64; fl_r = (0x40u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xc8u64), ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64); if (((fl_l as u64)) != ((fl_r as u64))) { bb = 9; continue; } rcx = r13; rcx = (((rcx) ^ (r11))) as u64; r10 = (0x9ddfea08eb382d69u64) as u64; rcx = (((rcx) as i64).wrapping_mul((r10) as i64) as u64) as u64; rax = rcx; rax = ((rax).wrapping_shr(47u32)) as u64; rcx = (((rcx) ^ (rax))) as u64; r13 = (((r13) ^ (rcx))) as u64; r13 = (((r13) as i64).wrapping_mul((r10) as i64) as u64) as u64; rcx = r13; rcx = ((rcx).wrapping_shr(47u32)) as u64; rcx = (((rcx) ^ (r13))) as u64; rcx = (((rcx) as i64).wrapping_mul((r10) as i64) as u64) as u64; rcx = ((rcx).wrapping_add(r15)) as u64; rdx = r8; rdx = (((rdx) ^ (crate::simd_ops::ru64(rsp.wrapping_add(0xb8u64))))) as u64; rdx = (((rdx) as i64).wrapping_mul((r10) as i64) as u64) as u64; rax = rdx; rax = ((rax).wrapping_shr(47u32)) as u64; rdx = (((rdx) ^ (rax))) as u64; r8 = (((r8) ^ (rdx))) as u64; r8 = (((r8) as i64).wrapping_mul((r10) as i64) as u64) as u64; rax = r8; rax = ((rax).wrapping_shr(47u32)) as u64; rax = (((rax) ^ (r8))) as u64; rax = (((rax) as i64).wrapping_mul((r10) as i64) as u64) as u64; rdx = rbp; rdx = ((rdx).wrapping_shr(47u32)) as u64; rdx = (((rdx) ^ (rbp))) as u64; rdx = (((rdx) as i64).wrapping_mul((r12) as i64) as u64) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; rdx = ((rdx).wrapping_add(rsi)) as u64; rdx = (((rdx) ^ (rcx))) as u64; rdx = (((rdx) as i64).wrapping_mul((r10) as i64) as u64) as u64; rax = rdx; rax = ((rax).wrapping_shr(47u32)) as u64; rdx = (((rdx) ^ (rax))) as u64; rcx = (((rcx) ^ (rdx))) as u64; rcx = (((rcx) as i64).wrapping_mul((r10) as i64) as u64) as u64; rax = rcx; rax = ((rax).wrapping_shr(47u32)) as u64; rcx = (((rcx) ^ (rax))) as u64; rcx = (((rcx) as i64).wrapping_mul((r10) as i64) as u64) as u64; bb = 10; continue; }
            10 => { rax = (0xde5fb9d2630458e9u64) as u64; let prod = (rax as u128).wrapping_mul((rcx) as u128); rax = prod as u64; rdx = (prod >> 64) as u64; r9 = ((rdx.wrapping_add(rax)) as u64) as u64; r8 = rsp.wrapping_add(0x30); rdx = rsp.wrapping_add(0x20); rcx = P2; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; fl_l = (crate::simd_ops::ru8(rsp.wrapping_add(0x28u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 11; continue; } rbx = ((rdi.wrapping_add(rdi.wrapping_mul(2))) as u64) as u64; rbx = ((rbx).wrapping_shl(4u32)) as u64; rbx = ((rbx).wrapping_add(crate::simd_ops::ru64(P4))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xb0u64), rbx); crate::simd_ops::lk_and_u32(rbx, (0xff000004u32) as u32); crate::simd_ops::lk_or_u32(rbx, (0x4u32) as u32); crate::simd_ops::wu8(rbx.wrapping_add(0x3u64), (0x4u8) as u8); crate::simd_ops::wu32(rbx.wrapping_add(0x4u64), (0x0u32) as u32); crate::simd_ops::wu8(rbx.wrapping_add(0x8u64), (0x0u8) as u8); rdx = rsp.wrapping_add(0x30); rcx = rbx; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx.wrapping_add(0x28u64), (0x0u64) as u64); bb = 11; continue; }
            11 => { rax = ((rdi.wrapping_add(rdi.wrapping_mul(2))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rax = ((rax).wrapping_add(crate::simd_ops::ru64(P4))) as u64; rcx = P6; crate::simd_ops::wu64(rax.wrapping_add(0x28u64), rcx); rcx = rsp.wrapping_add(0x30); rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); rax = 0; fl_l = 0; fl_r = 0; rsp = ((rsp).wrapping_add(0x68u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_36635<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 21>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x68u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (0x4000004u64) as u64); crate::simd_ops::wu8(rsp.wrapping_add(0x38u64), (0x0u8) as u8); rax = P0; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (0x17u64) as u64); r9 = 0x17; r8 = 0; fl_l = 0; fl_r = 0; rdx = rsp.wrapping_add(0x20); rcx = rsp.wrapping_add(0x30); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru32(rsp.wrapping_add(0x34u64))) as u32 as u64; rcx = (crate::simd_ops::ru32(rsp.wrapping_add(0x30u64))) as u32 as u64; rax = ((rcx as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_shr(24u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x80; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 1; continue; } rax = ((rax.wrapping_mul(8)) as u32) as u32 as u64; bb = 2; continue; }
            1 => { rax = (((rax as u32)).wrapping_shl(8u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x7c00; rax = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 2; continue; }
            2 => { rcx = ((((rcx as u32)) & (0xffffffu32))) as u32 as u64; rcx = (((rcx as u32)).wrapping_shl(3u32)) as u32 as u64; r15 = rsp.wrapping_add(0x38); fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; r15 = (if (((fl_l as u32)) > ((fl_r as u32))) { crate::simd_ops::ru64(rsp.wrapping_add(0x38u64)) } else { r15 }) as u64; fl_l = rbx; fl_r = (0x20u64) as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 7; continue; } fl_l = rbx; fl_r = (0x10u64) as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 6; continue; } fl_l = rbx; fl_r = (0x8u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 3; continue; } r8 = (0x9ae16a3b2f90404fu64) as u64; r9 = ((r8.wrapping_add(rbx.wrapping_mul(2))) as u64) as u64; r8 = (crate::simd_ops::ru64(r15)) as u64; rax = (0x651e95c4d06fbfb1u64) as u64; r8 = ((r8).wrapping_sub(rax)) as u64; rdx = (crate::simd_ops::ru64(rbx.wrapping_add(r15).wrapping_sub(0x8u64))) as u64; rdx = ((rdx).rotate_right(37u32)) as u64; rdx = (((rdx) as i64).wrapping_mul((r9) as i64) as u64) as u64; rdx = ((rdx).wrapping_add(r8)) as u64; r8 = ((r8).rotate_right(25u32)) as u64; r8 = ((r8).wrapping_add(crate::simd_ops::ru64(rbx.wrapping_add(r15).wrapping_sub(0x8u64)))) as u64; r8 = (((r8) as i64).wrapping_mul((r9) as i64) as u64) as u64; rcx = r8; rcx = (((rcx) ^ (rdx))) as u64; rcx = (((rcx) as i64).wrapping_mul((r9) as i64) as u64) as u64; rax = rcx; rax = ((rax).wrapping_shr(47u32)) as u64; rcx = (((rcx) ^ (rax))) as u64; rcx = (((rcx) ^ (r8))) as u64; rcx = (((rcx) as i64).wrapping_mul((r9) as i64) as u64) as u64; rax = rcx; rax = ((rax).wrapping_shr(47u32)) as u64; rcx = (((rcx) ^ (rax))) as u64; rcx = (((rcx) as i64).wrapping_mul((r9) as i64) as u64) as u64; bb = 10; continue; }
            3 => { fl_l = rbx; fl_r = (0x4u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 4; continue; } r8 = (0x9ae16a3b2f90404fu64) as u64; r8 = ((r8.wrapping_add(rbx.wrapping_mul(2))) as u64) as u64; rax = (crate::simd_ops::ru32(r15)) as u32 as u64; rdx = (crate::simd_ops::ru32(rbx.wrapping_add(r15).wrapping_sub(0x4u64))) as u32 as u64; rcx = ((rbx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rcx = (((rcx) ^ (rdx))) as u64; rcx = (((rcx) as i64).wrapping_mul((r8) as i64) as u64) as u64; rax = rcx; rax = ((rax).wrapping_shr(47u32)) as u64; rcx = (((rcx) ^ (rax))) as u64; rcx = (((rcx) ^ (rdx))) as u64; rcx = (((rcx) as i64).wrapping_mul((r8) as i64) as u64) as u64; rax = rcx; rax = ((rax).wrapping_shr(47u32)) as u64; rcx = (((rcx) ^ (rax))) as u64; rcx = (((rcx) as i64).wrapping_mul((r8) as i64) as u64) as u64; bb = 10; continue; }
            4 => { fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } rax = rbx; rax = ((rax).wrapping_shr(1u32)) as u64; rdx = ((crate::simd_ops::ru8(rax.wrapping_add(r15))) as u32) as u32 as u64; rax = (0xe16a3b2f90404f00u64) as u64; rdx = (((rdx) as i64).wrapping_mul((rax) as i64) as u64) as u64; rax = ((crate::simd_ops::ru8(r15)) as u32) as u32 as u64; r8 = (0x9ae16a3b2f90404fu64) as u64; rax = (((rax) as i64).wrapping_mul((r8) as i64) as u64) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; rax = ((crate::simd_ops::ru8(rbx.wrapping_add(r15).wrapping_sub(0x1u64))) as u32) as u32 as u64; rax = ((rbx.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = ((rax as u32)) as u32 as u64; rcx = (0xc3a5c85c97cb3127u64) as u64; rax = (((rax) as i64).wrapping_mul((rcx) as i64) as u64) as u64; rdx = (((rdx) ^ (rax))) as u64; rcx = rdx; rcx = ((rcx).wrapping_shr(47u32)) as u64; rcx = (((rcx) ^ (rdx))) as u64; rcx = (((rcx) as i64).wrapping_mul((r8) as i64) as u64) as u64; bb = 10; continue; }
            5 => { rcx = (0x9ae16a3b2f90404fu64) as u64; bb = 10; continue; }
            6 => { r8 = (0x9ae16a3b2f90404fu64) as u64; r11 = ((r8.wrapping_add(rbx.wrapping_mul(2))) as u64) as u64; r12 = (0xb492b66fbe98f273u64) as u64; r9 = (crate::simd_ops::ru64(r15)) as u64; r9 = (((r9) as i64).wrapping_mul((r12) as i64) as u64) as u64; rdx = (crate::simd_ops::ru64(r15.wrapping_add(0x8u64))) as u64; r10 = r11; r10 = (((r10) as i64).wrapping_mul((crate::simd_ops::ru64(rbx.wrapping_add(r15).wrapping_sub(0x8u64))) as i64) as u64) as u64; rcx = (crate::simd_ops::ru64(rbx.wrapping_add(r15).wrapping_sub(0x10u64))) as u64; rcx = (((rcx) as i64).wrapping_mul((r8) as i64) as u64) as u64; r8 = ((r8).wrapping_add(rdx)) as u64; r8 = ((r8).rotate_right(18u32)) as u64; r8 = ((r8).wrapping_add(r10)) as u64; r8 = ((r8).wrapping_add(r9)) as u64; rax = ((rdx.wrapping_add(r9)) as u64) as u64; rax = ((rax).rotate_right(43u32)) as u64; r10 = ((r10).rotate_right(30u32)) as u64; rcx = ((rcx).wrapping_add(rax)) as u64; rcx = ((rcx).wrapping_add(r10)) as u64; rcx = (((rcx) ^ (r8))) as u64; rcx = (((rcx) as i64).wrapping_mul((r11) as i64) as u64) as u64; rax = rcx; rax = ((rax).wrapping_shr(47u32)) as u64; rcx = (((rcx) ^ (rax))) as u64; rcx = (((rcx) ^ (r8))) as u64; rcx = (((rcx) as i64).wrapping_mul((r11) as i64) as u64) as u64; rax = rcx; rax = ((rax).wrapping_shr(47u32)) as u64; rcx = (((rcx) ^ (rax))) as u64; rcx = (((rcx) as i64).wrapping_mul((r11) as i64) as u64) as u64; bb = 10; continue; }
            7 => { r13 = ((rbx.wrapping_add(r15)) as u64) as u64; fl_l = rbx; fl_r = (0x40u64) as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 8; continue; } r8 = (0x9ae16a3b2f90404fu64) as u64; r14 = ((r8.wrapping_add(rbx.wrapping_mul(2))) as u64) as u64; r11 = (crate::simd_ops::ru64(r13.wrapping_sub(0x18u64))) as u64; r10 = (crate::simd_ops::ru64(r15.wrapping_add(0x10u64))) as u64; r10 = (((r10) as i64).wrapping_mul((r8) as i64) as u64) as u64; rax = (crate::simd_ops::ru64(r15.wrapping_add(0x18u64))) as u64; r9 = ((rax.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rdi = r14; rdi = (((rdi) as i64).wrapping_mul((crate::simd_ops::ru64(r13.wrapping_sub(0x10u64))) as i64) as u64) as u64; rdx = (crate::simd_ops::ru64(r15)) as u64; rdx = (((rdx) as i64).wrapping_mul((r8) as i64) as u64) as u64; rdx = ((rdx).wrapping_add(crate::simd_ops::ru64(r13.wrapping_sub(0x8u64)))) as u64; rax = (crate::simd_ops::ru64(r15.wrapping_add(0x8u64))) as u64; rax = ((rax).rotate_right(30u32)) as u64; rax = ((rax).wrapping_add(r11)) as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rax = rdx; rax = ((rax).rotate_right(43u32)) as u64; rcx = ((rcx).wrapping_add(rax)) as u64; r8 = (crate::simd_ops::ru64(r13.wrapping_sub(0x20u64))) as u64; r8 = (((r8) ^ (rdx))) as u64; r8 = ((r8).wrapping_add(1)) as u64; r8 = ((r8).wrapping_add(r9)) as u64; rdx = ((r8.wrapping_add(rcx)) as u64) as u64; rdx = (((rdx) as i64).wrapping_mul((r14) as i64) as u64) as u64; rdx = ((rdx).swap_bytes()) as u64; rdx = ((rdx).wrapping_add(rdi)) as u64; rax = ((r9.wrapping_add(r10)) as u64) as u64; r9 = rax; r9 = ((r9).rotate_right(42u32)) as u64; r9 = ((r9).wrapping_add(r11)) as u64; rcx = ((rax.wrapping_add(r11)) as u64) as u64; rax = ((rdx.wrapping_add(r8)) as u64) as u64; rax = (((rax) as i64).wrapping_mul((r14) as i64) as u64) as u64; rax = ((rax).swap_bytes()) as u64; rax = ((rax).wrapping_add(rcx)) as u64; rax = ((rax).wrapping_add(r9)) as u64; rax = ((rax).wrapping_add(crate::simd_ops::ru64(r13.wrapping_sub(0x8u64)))) as u64; rax = (((rax) as i64).wrapping_mul((r14) as i64) as u64) as u64; rax = ((rax).swap_bytes()) as u64; rax = ((rax).wrapping_add(crate::simd_ops::ru64(r15.wrapping_add(0x8u64)))) as u64; rcx = ((rcx).wrapping_add(rax)) as u64; rcx = (((rcx) as i64).wrapping_mul((r14) as i64) as u64) as u64; rcx = ((rcx).wrapping_add(rdi)) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(r13.wrapping_sub(0x20u64)))) as u64; rax = rcx; rax = ((rax).wrapping_shr(47u32)) as u64; rcx = (((rcx) ^ (rax))) as u64; rcx = (((rcx) as i64).wrapping_mul((r14) as i64) as u64) as u64; fl_l = ((rcx).wrapping_add(r9)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; bb = 10; continue; }
            8 => { rdi = (crate::simd_ops::ru64(r13.wrapping_sub(0x28u64))) as u64; rbp = (crate::simd_ops::ru64(r13.wrapping_sub(0x38u64))) as u64; rbp = ((rbp).wrapping_add(crate::simd_ops::ru64(r13.wrapping_sub(0x10u64)))) as u64; r14 = (crate::simd_ops::ru64(r13.wrapping_sub(0x30u64))) as u64; r14 = ((r14).wrapping_add(rbx)) as u64; r14 = (((r14) ^ (crate::simd_ops::ru64(r13.wrapping_sub(0x18u64))))) as u64; rdx = (0x9ddfea08eb382d69u64) as u64; r14 = (((r14) as i64).wrapping_mul((rdx) as i64) as u64) as u64; rax = r14; rax = ((rax).wrapping_shr(47u32)) as u64; r14 = (((r14) ^ (rax))) as u64; r14 = (((r14) ^ (crate::simd_ops::ru64(r13.wrapping_sub(0x18u64))))) as u64; r14 = (((r14) as i64).wrapping_mul((rdx) as i64) as u64) as u64; rax = r14; rax = ((rax).wrapping_shr(47u32)) as u64; r14 = (((r14) ^ (rax))) as u64; r14 = (((r14) as i64).wrapping_mul((rdx) as i64) as u64) as u64; rax = (crate::simd_ops::ru64(r13.wrapping_sub(0x30u64))) as u64; r10 = (crate::simd_ops::ru64(r13.wrapping_sub(0x40u64))) as u64; r10 = ((r10).wrapping_add(rbx)) as u64; r9 = ((rdi.wrapping_add(r10)) as u64) as u64; r9 = ((r9).wrapping_add(r14)) as u64; r8 = r10; rax = ((rax).wrapping_add(crate::simd_ops::ru64(r13.wrapping_sub(0x38u64)))) as u64; r10 = ((r10).wrapping_add(rax)) as u64; r9 = ((r9).rotate_right(21u32)) as u64; rax = r10; rax = ((rax).rotate_right(44u32)) as u64; rax = ((rax).wrapping_add(r9)) as u64; rax = ((rax).wrapping_add(r8)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xc0u64), rax); r9 = ((rdi.wrapping_add(r10)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xb8u64), r9); r11 = (crate::simd_ops::ru64(r13.wrapping_sub(0x8u64))) as u64; rax = (crate::simd_ops::ru64(r13.wrapping_sub(0x10u64))) as u64; rcx = (crate::simd_ops::ru64(r13.wrapping_sub(0x18u64))) as u64; r10 = (crate::simd_ops::ru64(r13.wrapping_sub(0x20u64))) as u64; r12 = (0xb492b66fbe98f273u64) as u64; r10 = ((r10).wrapping_add(r12)) as u64; r10 = ((r10).wrapping_add(rbp)) as u64; r9 = ((r11.wrapping_add(r10)) as u64) as u64; r9 = ((r9).wrapping_add(rdi)) as u64; r13 = r10; rax = ((rax).wrapping_add(rcx)) as u64; r10 = ((r10).wrapping_add(rax)) as u64; r9 = ((r9).rotate_right(21u32)) as u64; rax = r10; rax = ((rax).rotate_right(44u32)) as u64; r9 = ((r9).wrapping_add(rax)) as u64; r13 = ((r13).wrapping_add(r9)) as u64; r8 = ((r11.wrapping_add(r10)) as u64) as u64; rdi = (((rdi) as i64).wrapping_mul((r12) as i64) as u64) as u64; rdi = ((rdi).wrapping_add(crate::simd_ops::ru64(r15))) as u64; rbx = ((rbx).wrapping_sub(1)) as u64; fl_l = (((rbx) & (0xffffffffffffffc0u64))) as u64; fl_r = fl_l; rbx = ((fl_l as u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xc8u64), rbx); rax = r15.wrapping_add(0x30); crate::simd_ops::wu64(rsp.wrapping_add(0xb0u64), rax); r15 = rdi; bb = 9; continue; }
            9 => { r11 = (crate::simd_ops::ru64(rax.wrapping_sub(0x28u64))) as u64; rsi = ((r11.wrapping_add(rbp)) as u64) as u64; rsi = ((rsi).wrapping_add(r15)) as u64; rsi = ((rsi).wrapping_add(crate::simd_ops::ru64(rsp.wrapping_add(0xb8u64)))) as u64; rdi = (crate::simd_ops::ru64(rax)) as u64; rbp = ((rbp).wrapping_add(rdi)) as u64; rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0xc0u64))) as u64; rbp = ((rbp).wrapping_add(rdx)) as u64; rsi = ((rsi).rotate_right(37u32)) as u64; rsi = (((rsi) as i64).wrapping_mul((r12) as i64) as u64) as u64; rsi = (((rsi) ^ (r13))) as u64; rbx = (crate::simd_ops::ru64(rax.wrapping_sub(0x8u64))) as u64; rbp = ((rbp).rotate_right(42u32)) as u64; rbp = (((rbp) as i64).wrapping_mul((r12) as i64) as u64) as u64; rbp = ((rbp).wrapping_add(rbx)) as u64; rbp = ((rbp).wrapping_add(crate::simd_ops::ru64(rsp.wrapping_add(0xb8u64)))) as u64; r15 = ((r14.wrapping_add(r8)) as u64) as u64; r15 = ((r15).rotate_right(33u32)) as u64; r15 = (((r15) as i64).wrapping_mul((r12) as i64) as u64) as u64; rcx = ((rsi.wrapping_add(r8)) as u64) as u64; rdx = (((rdx) as i64).wrapping_mul((r12) as i64) as u64) as u64; r9 = (crate::simd_ops::ru64(rax.wrapping_sub(0x18u64))) as u64; r10 = (crate::simd_ops::ru64(rax.wrapping_sub(0x20u64))) as u64; r8 = (crate::simd_ops::ru64(rax.wrapping_sub(0x30u64))) as u64; r8 = ((r8).wrapping_add(rdx)) as u64; rdx = ((r9.wrapping_add(r8)) as u64) as u64; rdx = ((rdx).wrapping_add(rcx)) as u64; rcx = r8; rax = ((r10.wrapping_add(r11)) as u64) as u64; r8 = ((r8).wrapping_add(rax)) as u64; rdx = ((rdx).rotate_right(21u32)) as u64; r11 = r8; r11 = ((r11).rotate_right(44u32)) as u64; r11 = ((r11).wrapping_add(rdx)) as u64; r11 = ((r11).wrapping_add(rcx)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xc0u64), r11); rax = ((r9.wrapping_add(r8)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xb8u64), rax); rdx = ((r10.wrapping_add(rbp)) as u64) as u64; r9 = ((r15.wrapping_add(r13)) as u64) as u64; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xb0u64))) as u64; r10 = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; r9 = ((r9).wrapping_add(crate::simd_ops::ru64(rax.wrapping_sub(0x10u64)))) as u64; r8 = ((r10.wrapping_add(r9)) as u64) as u64; r8 = ((r8).wrapping_add(rdx)) as u64; r13 = r9; rax = ((rdi.wrapping_add(rbx)) as u64) as u64; r9 = ((r9).wrapping_add(rax)) as u64; r8 = ((r8).rotate_right(21u32)) as u64; rax = r9; rax = ((rax).rotate_right(44u32)) as u64; r8 = ((r8).wrapping_add(rax)) as u64; r13 = ((r13).wrapping_add(r8)) as u64; r8 = ((r10.wrapping_add(r9)) as u64) as u64; r14 = rsi; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xb0u64))) as u64; rax = ((rax).wrapping_add(0x40u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xb0u64), rax); fl_l = (crate::simd_ops::ru64(rsp.wrapping_add(0xc8u64))) as u64; fl_r = (0x40u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xc8u64), ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64); if (((fl_l as u64)) != ((fl_r as u64))) { bb = 9; continue; } rcx = r13; rcx = (((rcx) ^ (r11))) as u64; r10 = (0x9ddfea08eb382d69u64) as u64; rcx = (((rcx) as i64).wrapping_mul((r10) as i64) as u64) as u64; rax = rcx; rax = ((rax).wrapping_shr(47u32)) as u64; rcx = (((rcx) ^ (rax))) as u64; r13 = (((r13) ^ (rcx))) as u64; r13 = (((r13) as i64).wrapping_mul((r10) as i64) as u64) as u64; rcx = r13; rcx = ((rcx).wrapping_shr(47u32)) as u64; rcx = (((rcx) ^ (r13))) as u64; rcx = (((rcx) as i64).wrapping_mul((r10) as i64) as u64) as u64; rcx = ((rcx).wrapping_add(r15)) as u64; rdx = r8; rdx = (((rdx) ^ (crate::simd_ops::ru64(rsp.wrapping_add(0xb8u64))))) as u64; rdx = (((rdx) as i64).wrapping_mul((r10) as i64) as u64) as u64; rax = rdx; rax = ((rax).wrapping_shr(47u32)) as u64; rdx = (((rdx) ^ (rax))) as u64; r8 = (((r8) ^ (rdx))) as u64; r8 = (((r8) as i64).wrapping_mul((r10) as i64) as u64) as u64; rax = r8; rax = ((rax).wrapping_shr(47u32)) as u64; rax = (((rax) ^ (r8))) as u64; rax = (((rax) as i64).wrapping_mul((r10) as i64) as u64) as u64; rdx = rbp; rdx = ((rdx).wrapping_shr(47u32)) as u64; rdx = (((rdx) ^ (rbp))) as u64; rdx = (((rdx) as i64).wrapping_mul((r12) as i64) as u64) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; rdx = ((rdx).wrapping_add(rsi)) as u64; rdx = (((rdx) ^ (rcx))) as u64; rdx = (((rdx) as i64).wrapping_mul((r10) as i64) as u64) as u64; rax = rdx; rax = ((rax).wrapping_shr(47u32)) as u64; rdx = (((rdx) ^ (rax))) as u64; rcx = (((rcx) ^ (rdx))) as u64; rcx = (((rcx) as i64).wrapping_mul((r10) as i64) as u64) as u64; rax = rcx; rax = ((rax).wrapping_shr(47u32)) as u64; rcx = (((rcx) ^ (rax))) as u64; rcx = (((rcx) as i64).wrapping_mul((r10) as i64) as u64) as u64; bb = 10; continue; }
            10 => { rax = (0xde5fb9d2630458e9u64) as u64; let prod = (rax as u128).wrapping_mul((rcx) as u128); rax = prod as u64; rdx = (prod >> 64) as u64; r9 = ((rdx.wrapping_add(rax)) as u64) as u64; r8 = rsp.wrapping_add(0x30); rdx = rsp.wrapping_add(0x20); rcx = P2; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; fl_l = (crate::simd_ops::ru8(rsp.wrapping_add(0x28u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 11; continue; } rbx = ((rdi.wrapping_add(rdi.wrapping_mul(2))) as u64) as u64; rbx = ((rbx).wrapping_shl(4u32)) as u64; rbx = ((rbx).wrapping_add(crate::simd_ops::ru64(P4))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xb0u64), rbx); crate::simd_ops::lk_and_u32(rbx, (0xff000004u32) as u32); crate::simd_ops::lk_or_u32(rbx, (0x4u32) as u32); crate::simd_ops::wu8(rbx.wrapping_add(0x3u64), (0x4u8) as u8); crate::simd_ops::wu32(rbx.wrapping_add(0x4u64), (0x0u32) as u32); crate::simd_ops::wu8(rbx.wrapping_add(0x8u64), (0x0u8) as u8); rdx = rsp.wrapping_add(0x30); rcx = rbx; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx.wrapping_add(0x28u64), (0x0u64) as u64); bb = 11; continue; }
            11 => { rax = ((rdi.wrapping_add(rdi.wrapping_mul(2))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rax = ((rax).wrapping_add(crate::simd_ops::ru64(P4))) as u64; rcx = P6; crate::simd_ops::wu64(rax.wrapping_add(0x28u64), rcx); rcx = rsp.wrapping_add(0x30); rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); rax = 0; fl_l = 0; fl_r = 0; rsp = ((rsp).wrapping_add(0x68u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_36636<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 21>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x68u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (0x4000004u64) as u64); crate::simd_ops::wu8(rsp.wrapping_add(0x38u64), (0x0u8) as u8); rax = P0; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (0x14u64) as u64); r9 = 0x14; r8 = 0; fl_l = 0; fl_r = 0; rdx = rsp.wrapping_add(0x20); rcx = rsp.wrapping_add(0x30); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru32(rsp.wrapping_add(0x34u64))) as u32 as u64; rcx = (crate::simd_ops::ru32(rsp.wrapping_add(0x30u64))) as u32 as u64; rax = ((rcx as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_shr(24u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x80; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 1; continue; } rax = ((rax.wrapping_mul(8)) as u32) as u32 as u64; bb = 2; continue; }
            1 => { rax = (((rax as u32)).wrapping_shl(8u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x7c00; rax = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 2; continue; }
            2 => { rcx = ((((rcx as u32)) & (0xffffffu32))) as u32 as u64; rcx = (((rcx as u32)).wrapping_shl(3u32)) as u32 as u64; r15 = rsp.wrapping_add(0x38); fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; r15 = (if (((fl_l as u32)) > ((fl_r as u32))) { crate::simd_ops::ru64(rsp.wrapping_add(0x38u64)) } else { r15 }) as u64; fl_l = rbx; fl_r = (0x20u64) as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 7; continue; } fl_l = rbx; fl_r = (0x10u64) as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 6; continue; } fl_l = rbx; fl_r = (0x8u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 3; continue; } r8 = (0x9ae16a3b2f90404fu64) as u64; r9 = ((r8.wrapping_add(rbx.wrapping_mul(2))) as u64) as u64; r8 = (crate::simd_ops::ru64(r15)) as u64; rax = (0x651e95c4d06fbfb1u64) as u64; r8 = ((r8).wrapping_sub(rax)) as u64; rdx = (crate::simd_ops::ru64(rbx.wrapping_add(r15).wrapping_sub(0x8u64))) as u64; rdx = ((rdx).rotate_right(37u32)) as u64; rdx = (((rdx) as i64).wrapping_mul((r9) as i64) as u64) as u64; rdx = ((rdx).wrapping_add(r8)) as u64; r8 = ((r8).rotate_right(25u32)) as u64; r8 = ((r8).wrapping_add(crate::simd_ops::ru64(rbx.wrapping_add(r15).wrapping_sub(0x8u64)))) as u64; r8 = (((r8) as i64).wrapping_mul((r9) as i64) as u64) as u64; rcx = r8; rcx = (((rcx) ^ (rdx))) as u64; rcx = (((rcx) as i64).wrapping_mul((r9) as i64) as u64) as u64; rax = rcx; rax = ((rax).wrapping_shr(47u32)) as u64; rcx = (((rcx) ^ (rax))) as u64; rcx = (((rcx) ^ (r8))) as u64; rcx = (((rcx) as i64).wrapping_mul((r9) as i64) as u64) as u64; rax = rcx; rax = ((rax).wrapping_shr(47u32)) as u64; rcx = (((rcx) ^ (rax))) as u64; rcx = (((rcx) as i64).wrapping_mul((r9) as i64) as u64) as u64; bb = 10; continue; }
            3 => { fl_l = rbx; fl_r = (0x4u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 4; continue; } r8 = (0x9ae16a3b2f90404fu64) as u64; r8 = ((r8.wrapping_add(rbx.wrapping_mul(2))) as u64) as u64; rax = (crate::simd_ops::ru32(r15)) as u32 as u64; rdx = (crate::simd_ops::ru32(rbx.wrapping_add(r15).wrapping_sub(0x4u64))) as u32 as u64; rcx = ((rbx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rcx = (((rcx) ^ (rdx))) as u64; rcx = (((rcx) as i64).wrapping_mul((r8) as i64) as u64) as u64; rax = rcx; rax = ((rax).wrapping_shr(47u32)) as u64; rcx = (((rcx) ^ (rax))) as u64; rcx = (((rcx) ^ (rdx))) as u64; rcx = (((rcx) as i64).wrapping_mul((r8) as i64) as u64) as u64; rax = rcx; rax = ((rax).wrapping_shr(47u32)) as u64; rcx = (((rcx) ^ (rax))) as u64; rcx = (((rcx) as i64).wrapping_mul((r8) as i64) as u64) as u64; bb = 10; continue; }
            4 => { fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } rax = rbx; rax = ((rax).wrapping_shr(1u32)) as u64; rdx = ((crate::simd_ops::ru8(rax.wrapping_add(r15))) as u32) as u32 as u64; rax = (0xe16a3b2f90404f00u64) as u64; rdx = (((rdx) as i64).wrapping_mul((rax) as i64) as u64) as u64; rax = ((crate::simd_ops::ru8(r15)) as u32) as u32 as u64; r8 = (0x9ae16a3b2f90404fu64) as u64; rax = (((rax) as i64).wrapping_mul((r8) as i64) as u64) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; rax = ((crate::simd_ops::ru8(rbx.wrapping_add(r15).wrapping_sub(0x1u64))) as u32) as u32 as u64; rax = ((rbx.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = ((rax as u32)) as u32 as u64; rcx = (0xc3a5c85c97cb3127u64) as u64; rax = (((rax) as i64).wrapping_mul((rcx) as i64) as u64) as u64; rdx = (((rdx) ^ (rax))) as u64; rcx = rdx; rcx = ((rcx).wrapping_shr(47u32)) as u64; rcx = (((rcx) ^ (rdx))) as u64; rcx = (((rcx) as i64).wrapping_mul((r8) as i64) as u64) as u64; bb = 10; continue; }
            5 => { rcx = (0x9ae16a3b2f90404fu64) as u64; bb = 10; continue; }
            6 => { r8 = (0x9ae16a3b2f90404fu64) as u64; r11 = ((r8.wrapping_add(rbx.wrapping_mul(2))) as u64) as u64; r12 = (0xb492b66fbe98f273u64) as u64; r9 = (crate::simd_ops::ru64(r15)) as u64; r9 = (((r9) as i64).wrapping_mul((r12) as i64) as u64) as u64; rdx = (crate::simd_ops::ru64(r15.wrapping_add(0x8u64))) as u64; r10 = r11; r10 = (((r10) as i64).wrapping_mul((crate::simd_ops::ru64(rbx.wrapping_add(r15).wrapping_sub(0x8u64))) as i64) as u64) as u64; rcx = (crate::simd_ops::ru64(rbx.wrapping_add(r15).wrapping_sub(0x10u64))) as u64; rcx = (((rcx) as i64).wrapping_mul((r8) as i64) as u64) as u64; r8 = ((r8).wrapping_add(rdx)) as u64; r8 = ((r8).rotate_right(18u32)) as u64; r8 = ((r8).wrapping_add(r10)) as u64; r8 = ((r8).wrapping_add(r9)) as u64; rax = ((rdx.wrapping_add(r9)) as u64) as u64; rax = ((rax).rotate_right(43u32)) as u64; r10 = ((r10).rotate_right(30u32)) as u64; rcx = ((rcx).wrapping_add(rax)) as u64; rcx = ((rcx).wrapping_add(r10)) as u64; rcx = (((rcx) ^ (r8))) as u64; rcx = (((rcx) as i64).wrapping_mul((r11) as i64) as u64) as u64; rax = rcx; rax = ((rax).wrapping_shr(47u32)) as u64; rcx = (((rcx) ^ (rax))) as u64; rcx = (((rcx) ^ (r8))) as u64; rcx = (((rcx) as i64).wrapping_mul((r11) as i64) as u64) as u64; rax = rcx; rax = ((rax).wrapping_shr(47u32)) as u64; rcx = (((rcx) ^ (rax))) as u64; rcx = (((rcx) as i64).wrapping_mul((r11) as i64) as u64) as u64; bb = 10; continue; }
            7 => { r13 = ((rbx.wrapping_add(r15)) as u64) as u64; fl_l = rbx; fl_r = (0x40u64) as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 8; continue; } r8 = (0x9ae16a3b2f90404fu64) as u64; r14 = ((r8.wrapping_add(rbx.wrapping_mul(2))) as u64) as u64; r11 = (crate::simd_ops::ru64(r13.wrapping_sub(0x18u64))) as u64; r10 = (crate::simd_ops::ru64(r15.wrapping_add(0x10u64))) as u64; r10 = (((r10) as i64).wrapping_mul((r8) as i64) as u64) as u64; rax = (crate::simd_ops::ru64(r15.wrapping_add(0x18u64))) as u64; r9 = ((rax.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rdi = r14; rdi = (((rdi) as i64).wrapping_mul((crate::simd_ops::ru64(r13.wrapping_sub(0x10u64))) as i64) as u64) as u64; rdx = (crate::simd_ops::ru64(r15)) as u64; rdx = (((rdx) as i64).wrapping_mul((r8) as i64) as u64) as u64; rdx = ((rdx).wrapping_add(crate::simd_ops::ru64(r13.wrapping_sub(0x8u64)))) as u64; rax = (crate::simd_ops::ru64(r15.wrapping_add(0x8u64))) as u64; rax = ((rax).rotate_right(30u32)) as u64; rax = ((rax).wrapping_add(r11)) as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rax = rdx; rax = ((rax).rotate_right(43u32)) as u64; rcx = ((rcx).wrapping_add(rax)) as u64; r8 = (crate::simd_ops::ru64(r13.wrapping_sub(0x20u64))) as u64; r8 = (((r8) ^ (rdx))) as u64; r8 = ((r8).wrapping_add(1)) as u64; r8 = ((r8).wrapping_add(r9)) as u64; rdx = ((r8.wrapping_add(rcx)) as u64) as u64; rdx = (((rdx) as i64).wrapping_mul((r14) as i64) as u64) as u64; rdx = ((rdx).swap_bytes()) as u64; rdx = ((rdx).wrapping_add(rdi)) as u64; rax = ((r9.wrapping_add(r10)) as u64) as u64; r9 = rax; r9 = ((r9).rotate_right(42u32)) as u64; r9 = ((r9).wrapping_add(r11)) as u64; rcx = ((rax.wrapping_add(r11)) as u64) as u64; rax = ((rdx.wrapping_add(r8)) as u64) as u64; rax = (((rax) as i64).wrapping_mul((r14) as i64) as u64) as u64; rax = ((rax).swap_bytes()) as u64; rax = ((rax).wrapping_add(rcx)) as u64; rax = ((rax).wrapping_add(r9)) as u64; rax = ((rax).wrapping_add(crate::simd_ops::ru64(r13.wrapping_sub(0x8u64)))) as u64; rax = (((rax) as i64).wrapping_mul((r14) as i64) as u64) as u64; rax = ((rax).swap_bytes()) as u64; rax = ((rax).wrapping_add(crate::simd_ops::ru64(r15.wrapping_add(0x8u64)))) as u64; rcx = ((rcx).wrapping_add(rax)) as u64; rcx = (((rcx) as i64).wrapping_mul((r14) as i64) as u64) as u64; rcx = ((rcx).wrapping_add(rdi)) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(r13.wrapping_sub(0x20u64)))) as u64; rax = rcx; rax = ((rax).wrapping_shr(47u32)) as u64; rcx = (((rcx) ^ (rax))) as u64; rcx = (((rcx) as i64).wrapping_mul((r14) as i64) as u64) as u64; fl_l = ((rcx).wrapping_add(r9)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; bb = 10; continue; }
            8 => { rdi = (crate::simd_ops::ru64(r13.wrapping_sub(0x28u64))) as u64; rbp = (crate::simd_ops::ru64(r13.wrapping_sub(0x38u64))) as u64; rbp = ((rbp).wrapping_add(crate::simd_ops::ru64(r13.wrapping_sub(0x10u64)))) as u64; r14 = (crate::simd_ops::ru64(r13.wrapping_sub(0x30u64))) as u64; r14 = ((r14).wrapping_add(rbx)) as u64; r14 = (((r14) ^ (crate::simd_ops::ru64(r13.wrapping_sub(0x18u64))))) as u64; rdx = (0x9ddfea08eb382d69u64) as u64; r14 = (((r14) as i64).wrapping_mul((rdx) as i64) as u64) as u64; rax = r14; rax = ((rax).wrapping_shr(47u32)) as u64; r14 = (((r14) ^ (rax))) as u64; r14 = (((r14) ^ (crate::simd_ops::ru64(r13.wrapping_sub(0x18u64))))) as u64; r14 = (((r14) as i64).wrapping_mul((rdx) as i64) as u64) as u64; rax = r14; rax = ((rax).wrapping_shr(47u32)) as u64; r14 = (((r14) ^ (rax))) as u64; r14 = (((r14) as i64).wrapping_mul((rdx) as i64) as u64) as u64; rax = (crate::simd_ops::ru64(r13.wrapping_sub(0x30u64))) as u64; r10 = (crate::simd_ops::ru64(r13.wrapping_sub(0x40u64))) as u64; r10 = ((r10).wrapping_add(rbx)) as u64; r9 = ((rdi.wrapping_add(r10)) as u64) as u64; r9 = ((r9).wrapping_add(r14)) as u64; r8 = r10; rax = ((rax).wrapping_add(crate::simd_ops::ru64(r13.wrapping_sub(0x38u64)))) as u64; r10 = ((r10).wrapping_add(rax)) as u64; r9 = ((r9).rotate_right(21u32)) as u64; rax = r10; rax = ((rax).rotate_right(44u32)) as u64; rax = ((rax).wrapping_add(r9)) as u64; rax = ((rax).wrapping_add(r8)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xc0u64), rax); r9 = ((rdi.wrapping_add(r10)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xb8u64), r9); r11 = (crate::simd_ops::ru64(r13.wrapping_sub(0x8u64))) as u64; rax = (crate::simd_ops::ru64(r13.wrapping_sub(0x10u64))) as u64; rcx = (crate::simd_ops::ru64(r13.wrapping_sub(0x18u64))) as u64; r10 = (crate::simd_ops::ru64(r13.wrapping_sub(0x20u64))) as u64; r12 = (0xb492b66fbe98f273u64) as u64; r10 = ((r10).wrapping_add(r12)) as u64; r10 = ((r10).wrapping_add(rbp)) as u64; r9 = ((r11.wrapping_add(r10)) as u64) as u64; r9 = ((r9).wrapping_add(rdi)) as u64; r13 = r10; rax = ((rax).wrapping_add(rcx)) as u64; r10 = ((r10).wrapping_add(rax)) as u64; r9 = ((r9).rotate_right(21u32)) as u64; rax = r10; rax = ((rax).rotate_right(44u32)) as u64; r9 = ((r9).wrapping_add(rax)) as u64; r13 = ((r13).wrapping_add(r9)) as u64; r8 = ((r11.wrapping_add(r10)) as u64) as u64; rdi = (((rdi) as i64).wrapping_mul((r12) as i64) as u64) as u64; rdi = ((rdi).wrapping_add(crate::simd_ops::ru64(r15))) as u64; rbx = ((rbx).wrapping_sub(1)) as u64; fl_l = (((rbx) & (0xffffffffffffffc0u64))) as u64; fl_r = fl_l; rbx = ((fl_l as u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xc8u64), rbx); rax = r15.wrapping_add(0x30); crate::simd_ops::wu64(rsp.wrapping_add(0xb0u64), rax); r15 = rdi; bb = 9; continue; }
            9 => { r11 = (crate::simd_ops::ru64(rax.wrapping_sub(0x28u64))) as u64; rsi = ((r11.wrapping_add(rbp)) as u64) as u64; rsi = ((rsi).wrapping_add(r15)) as u64; rsi = ((rsi).wrapping_add(crate::simd_ops::ru64(rsp.wrapping_add(0xb8u64)))) as u64; rdi = (crate::simd_ops::ru64(rax)) as u64; rbp = ((rbp).wrapping_add(rdi)) as u64; rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0xc0u64))) as u64; rbp = ((rbp).wrapping_add(rdx)) as u64; rsi = ((rsi).rotate_right(37u32)) as u64; rsi = (((rsi) as i64).wrapping_mul((r12) as i64) as u64) as u64; rsi = (((rsi) ^ (r13))) as u64; rbx = (crate::simd_ops::ru64(rax.wrapping_sub(0x8u64))) as u64; rbp = ((rbp).rotate_right(42u32)) as u64; rbp = (((rbp) as i64).wrapping_mul((r12) as i64) as u64) as u64; rbp = ((rbp).wrapping_add(rbx)) as u64; rbp = ((rbp).wrapping_add(crate::simd_ops::ru64(rsp.wrapping_add(0xb8u64)))) as u64; r15 = ((r14.wrapping_add(r8)) as u64) as u64; r15 = ((r15).rotate_right(33u32)) as u64; r15 = (((r15) as i64).wrapping_mul((r12) as i64) as u64) as u64; rcx = ((rsi.wrapping_add(r8)) as u64) as u64; rdx = (((rdx) as i64).wrapping_mul((r12) as i64) as u64) as u64; r9 = (crate::simd_ops::ru64(rax.wrapping_sub(0x18u64))) as u64; r10 = (crate::simd_ops::ru64(rax.wrapping_sub(0x20u64))) as u64; r8 = (crate::simd_ops::ru64(rax.wrapping_sub(0x30u64))) as u64; r8 = ((r8).wrapping_add(rdx)) as u64; rdx = ((r9.wrapping_add(r8)) as u64) as u64; rdx = ((rdx).wrapping_add(rcx)) as u64; rcx = r8; rax = ((r10.wrapping_add(r11)) as u64) as u64; r8 = ((r8).wrapping_add(rax)) as u64; rdx = ((rdx).rotate_right(21u32)) as u64; r11 = r8; r11 = ((r11).rotate_right(44u32)) as u64; r11 = ((r11).wrapping_add(rdx)) as u64; r11 = ((r11).wrapping_add(rcx)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xc0u64), r11); rax = ((r9.wrapping_add(r8)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xb8u64), rax); rdx = ((r10.wrapping_add(rbp)) as u64) as u64; r9 = ((r15.wrapping_add(r13)) as u64) as u64; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xb0u64))) as u64; r10 = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; r9 = ((r9).wrapping_add(crate::simd_ops::ru64(rax.wrapping_sub(0x10u64)))) as u64; r8 = ((r10.wrapping_add(r9)) as u64) as u64; r8 = ((r8).wrapping_add(rdx)) as u64; r13 = r9; rax = ((rdi.wrapping_add(rbx)) as u64) as u64; r9 = ((r9).wrapping_add(rax)) as u64; r8 = ((r8).rotate_right(21u32)) as u64; rax = r9; rax = ((rax).rotate_right(44u32)) as u64; r8 = ((r8).wrapping_add(rax)) as u64; r13 = ((r13).wrapping_add(r8)) as u64; r8 = ((r10.wrapping_add(r9)) as u64) as u64; r14 = rsi; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xb0u64))) as u64; rax = ((rax).wrapping_add(0x40u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xb0u64), rax); fl_l = (crate::simd_ops::ru64(rsp.wrapping_add(0xc8u64))) as u64; fl_r = (0x40u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xc8u64), ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64); if (((fl_l as u64)) != ((fl_r as u64))) { bb = 9; continue; } rcx = r13; rcx = (((rcx) ^ (r11))) as u64; r10 = (0x9ddfea08eb382d69u64) as u64; rcx = (((rcx) as i64).wrapping_mul((r10) as i64) as u64) as u64; rax = rcx; rax = ((rax).wrapping_shr(47u32)) as u64; rcx = (((rcx) ^ (rax))) as u64; r13 = (((r13) ^ (rcx))) as u64; r13 = (((r13) as i64).wrapping_mul((r10) as i64) as u64) as u64; rcx = r13; rcx = ((rcx).wrapping_shr(47u32)) as u64; rcx = (((rcx) ^ (r13))) as u64; rcx = (((rcx) as i64).wrapping_mul((r10) as i64) as u64) as u64; rcx = ((rcx).wrapping_add(r15)) as u64; rdx = r8; rdx = (((rdx) ^ (crate::simd_ops::ru64(rsp.wrapping_add(0xb8u64))))) as u64; rdx = (((rdx) as i64).wrapping_mul((r10) as i64) as u64) as u64; rax = rdx; rax = ((rax).wrapping_shr(47u32)) as u64; rdx = (((rdx) ^ (rax))) as u64; r8 = (((r8) ^ (rdx))) as u64; r8 = (((r8) as i64).wrapping_mul((r10) as i64) as u64) as u64; rax = r8; rax = ((rax).wrapping_shr(47u32)) as u64; rax = (((rax) ^ (r8))) as u64; rax = (((rax) as i64).wrapping_mul((r10) as i64) as u64) as u64; rdx = rbp; rdx = ((rdx).wrapping_shr(47u32)) as u64; rdx = (((rdx) ^ (rbp))) as u64; rdx = (((rdx) as i64).wrapping_mul((r12) as i64) as u64) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; rdx = ((rdx).wrapping_add(rsi)) as u64; rdx = (((rdx) ^ (rcx))) as u64; rdx = (((rdx) as i64).wrapping_mul((r10) as i64) as u64) as u64; rax = rdx; rax = ((rax).wrapping_shr(47u32)) as u64; rdx = (((rdx) ^ (rax))) as u64; rcx = (((rcx) ^ (rdx))) as u64; rcx = (((rcx) as i64).wrapping_mul((r10) as i64) as u64) as u64; rax = rcx; rax = ((rax).wrapping_shr(47u32)) as u64; rcx = (((rcx) ^ (rax))) as u64; rcx = (((rcx) as i64).wrapping_mul((r10) as i64) as u64) as u64; bb = 10; continue; }
            10 => { rax = (0xde5fb9d2630458e9u64) as u64; let prod = (rax as u128).wrapping_mul((rcx) as u128); rax = prod as u64; rdx = (prod >> 64) as u64; r9 = ((rdx.wrapping_add(rax)) as u64) as u64; r8 = rsp.wrapping_add(0x30); rdx = rsp.wrapping_add(0x20); rcx = P2; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; fl_l = (crate::simd_ops::ru8(rsp.wrapping_add(0x28u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 11; continue; } rbx = ((rdi.wrapping_add(rdi.wrapping_mul(2))) as u64) as u64; rbx = ((rbx).wrapping_shl(4u32)) as u64; rbx = ((rbx).wrapping_add(crate::simd_ops::ru64(P4))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xb0u64), rbx); crate::simd_ops::lk_and_u32(rbx, (0xff000004u32) as u32); crate::simd_ops::lk_or_u32(rbx, (0x4u32) as u32); crate::simd_ops::wu8(rbx.wrapping_add(0x3u64), (0x4u8) as u8); crate::simd_ops::wu32(rbx.wrapping_add(0x4u64), (0x0u32) as u32); crate::simd_ops::wu8(rbx.wrapping_add(0x8u64), (0x0u8) as u8); rdx = rsp.wrapping_add(0x30); rcx = rbx; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx.wrapping_add(0x28u64), (0x0u64) as u64); bb = 11; continue; }
            11 => { rax = ((rdi.wrapping_add(rdi.wrapping_mul(2))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rax = ((rax).wrapping_add(crate::simd_ops::ru64(P4))) as u64; rcx = P6; crate::simd_ops::wu64(rax.wrapping_add(0x28u64), rcx); rcx = rsp.wrapping_add(0x30); rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); rax = 0; fl_l = 0; fl_r = 0; rsp = ((rsp).wrapping_add(0x68u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_36637<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 21>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x68u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (0x4000004u64) as u64); crate::simd_ops::wu8(rsp.wrapping_add(0x38u64), (0x0u8) as u8); rax = P0; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (0x13u64) as u64); r9 = 0x13; r8 = 0; fl_l = 0; fl_r = 0; rdx = rsp.wrapping_add(0x20); rcx = rsp.wrapping_add(0x30); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru32(rsp.wrapping_add(0x34u64))) as u32 as u64; rcx = (crate::simd_ops::ru32(rsp.wrapping_add(0x30u64))) as u32 as u64; rax = ((rcx as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_shr(24u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x80; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 1; continue; } rax = ((rax.wrapping_mul(8)) as u32) as u32 as u64; bb = 2; continue; }
            1 => { rax = (((rax as u32)).wrapping_shl(8u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x7c00; rax = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 2; continue; }
            2 => { rcx = ((((rcx as u32)) & (0xffffffu32))) as u32 as u64; rcx = (((rcx as u32)).wrapping_shl(3u32)) as u32 as u64; r15 = rsp.wrapping_add(0x38); fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; r15 = (if (((fl_l as u32)) > ((fl_r as u32))) { crate::simd_ops::ru64(rsp.wrapping_add(0x38u64)) } else { r15 }) as u64; fl_l = rbx; fl_r = (0x20u64) as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 7; continue; } fl_l = rbx; fl_r = (0x10u64) as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 6; continue; } fl_l = rbx; fl_r = (0x8u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 3; continue; } r8 = (0x9ae16a3b2f90404fu64) as u64; r9 = ((r8.wrapping_add(rbx.wrapping_mul(2))) as u64) as u64; r8 = (crate::simd_ops::ru64(r15)) as u64; rax = (0x651e95c4d06fbfb1u64) as u64; r8 = ((r8).wrapping_sub(rax)) as u64; rdx = (crate::simd_ops::ru64(rbx.wrapping_add(r15).wrapping_sub(0x8u64))) as u64; rdx = ((rdx).rotate_right(37u32)) as u64; rdx = (((rdx) as i64).wrapping_mul((r9) as i64) as u64) as u64; rdx = ((rdx).wrapping_add(r8)) as u64; r8 = ((r8).rotate_right(25u32)) as u64; r8 = ((r8).wrapping_add(crate::simd_ops::ru64(rbx.wrapping_add(r15).wrapping_sub(0x8u64)))) as u64; r8 = (((r8) as i64).wrapping_mul((r9) as i64) as u64) as u64; rcx = r8; rcx = (((rcx) ^ (rdx))) as u64; rcx = (((rcx) as i64).wrapping_mul((r9) as i64) as u64) as u64; rax = rcx; rax = ((rax).wrapping_shr(47u32)) as u64; rcx = (((rcx) ^ (rax))) as u64; rcx = (((rcx) ^ (r8))) as u64; rcx = (((rcx) as i64).wrapping_mul((r9) as i64) as u64) as u64; rax = rcx; rax = ((rax).wrapping_shr(47u32)) as u64; rcx = (((rcx) ^ (rax))) as u64; rcx = (((rcx) as i64).wrapping_mul((r9) as i64) as u64) as u64; bb = 10; continue; }
            3 => { fl_l = rbx; fl_r = (0x4u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 4; continue; } r8 = (0x9ae16a3b2f90404fu64) as u64; r8 = ((r8.wrapping_add(rbx.wrapping_mul(2))) as u64) as u64; rax = (crate::simd_ops::ru32(r15)) as u32 as u64; rdx = (crate::simd_ops::ru32(rbx.wrapping_add(r15).wrapping_sub(0x4u64))) as u32 as u64; rcx = ((rbx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rcx = (((rcx) ^ (rdx))) as u64; rcx = (((rcx) as i64).wrapping_mul((r8) as i64) as u64) as u64; rax = rcx; rax = ((rax).wrapping_shr(47u32)) as u64; rcx = (((rcx) ^ (rax))) as u64; rcx = (((rcx) ^ (rdx))) as u64; rcx = (((rcx) as i64).wrapping_mul((r8) as i64) as u64) as u64; rax = rcx; rax = ((rax).wrapping_shr(47u32)) as u64; rcx = (((rcx) ^ (rax))) as u64; rcx = (((rcx) as i64).wrapping_mul((r8) as i64) as u64) as u64; bb = 10; continue; }
            4 => { fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } rax = rbx; rax = ((rax).wrapping_shr(1u32)) as u64; rdx = ((crate::simd_ops::ru8(rax.wrapping_add(r15))) as u32) as u32 as u64; rax = (0xe16a3b2f90404f00u64) as u64; rdx = (((rdx) as i64).wrapping_mul((rax) as i64) as u64) as u64; rax = ((crate::simd_ops::ru8(r15)) as u32) as u32 as u64; r8 = (0x9ae16a3b2f90404fu64) as u64; rax = (((rax) as i64).wrapping_mul((r8) as i64) as u64) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; rax = ((crate::simd_ops::ru8(rbx.wrapping_add(r15).wrapping_sub(0x1u64))) as u32) as u32 as u64; rax = ((rbx.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = ((rax as u32)) as u32 as u64; rcx = (0xc3a5c85c97cb3127u64) as u64; rax = (((rax) as i64).wrapping_mul((rcx) as i64) as u64) as u64; rdx = (((rdx) ^ (rax))) as u64; rcx = rdx; rcx = ((rcx).wrapping_shr(47u32)) as u64; rcx = (((rcx) ^ (rdx))) as u64; rcx = (((rcx) as i64).wrapping_mul((r8) as i64) as u64) as u64; bb = 10; continue; }
            5 => { rcx = (0x9ae16a3b2f90404fu64) as u64; bb = 10; continue; }
            6 => { r8 = (0x9ae16a3b2f90404fu64) as u64; r11 = ((r8.wrapping_add(rbx.wrapping_mul(2))) as u64) as u64; r12 = (0xb492b66fbe98f273u64) as u64; r9 = (crate::simd_ops::ru64(r15)) as u64; r9 = (((r9) as i64).wrapping_mul((r12) as i64) as u64) as u64; rdx = (crate::simd_ops::ru64(r15.wrapping_add(0x8u64))) as u64; r10 = r11; r10 = (((r10) as i64).wrapping_mul((crate::simd_ops::ru64(rbx.wrapping_add(r15).wrapping_sub(0x8u64))) as i64) as u64) as u64; rcx = (crate::simd_ops::ru64(rbx.wrapping_add(r15).wrapping_sub(0x10u64))) as u64; rcx = (((rcx) as i64).wrapping_mul((r8) as i64) as u64) as u64; r8 = ((r8).wrapping_add(rdx)) as u64; r8 = ((r8).rotate_right(18u32)) as u64; r8 = ((r8).wrapping_add(r10)) as u64; r8 = ((r8).wrapping_add(r9)) as u64; rax = ((rdx.wrapping_add(r9)) as u64) as u64; rax = ((rax).rotate_right(43u32)) as u64; r10 = ((r10).rotate_right(30u32)) as u64; rcx = ((rcx).wrapping_add(rax)) as u64; rcx = ((rcx).wrapping_add(r10)) as u64; rcx = (((rcx) ^ (r8))) as u64; rcx = (((rcx) as i64).wrapping_mul((r11) as i64) as u64) as u64; rax = rcx; rax = ((rax).wrapping_shr(47u32)) as u64; rcx = (((rcx) ^ (rax))) as u64; rcx = (((rcx) ^ (r8))) as u64; rcx = (((rcx) as i64).wrapping_mul((r11) as i64) as u64) as u64; rax = rcx; rax = ((rax).wrapping_shr(47u32)) as u64; rcx = (((rcx) ^ (rax))) as u64; rcx = (((rcx) as i64).wrapping_mul((r11) as i64) as u64) as u64; bb = 10; continue; }
            7 => { r13 = ((rbx.wrapping_add(r15)) as u64) as u64; fl_l = rbx; fl_r = (0x40u64) as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 8; continue; } r8 = (0x9ae16a3b2f90404fu64) as u64; r14 = ((r8.wrapping_add(rbx.wrapping_mul(2))) as u64) as u64; r11 = (crate::simd_ops::ru64(r13.wrapping_sub(0x18u64))) as u64; r10 = (crate::simd_ops::ru64(r15.wrapping_add(0x10u64))) as u64; r10 = (((r10) as i64).wrapping_mul((r8) as i64) as u64) as u64; rax = (crate::simd_ops::ru64(r15.wrapping_add(0x18u64))) as u64; r9 = ((rax.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rdi = r14; rdi = (((rdi) as i64).wrapping_mul((crate::simd_ops::ru64(r13.wrapping_sub(0x10u64))) as i64) as u64) as u64; rdx = (crate::simd_ops::ru64(r15)) as u64; rdx = (((rdx) as i64).wrapping_mul((r8) as i64) as u64) as u64; rdx = ((rdx).wrapping_add(crate::simd_ops::ru64(r13.wrapping_sub(0x8u64)))) as u64; rax = (crate::simd_ops::ru64(r15.wrapping_add(0x8u64))) as u64; rax = ((rax).rotate_right(30u32)) as u64; rax = ((rax).wrapping_add(r11)) as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rax = rdx; rax = ((rax).rotate_right(43u32)) as u64; rcx = ((rcx).wrapping_add(rax)) as u64; r8 = (crate::simd_ops::ru64(r13.wrapping_sub(0x20u64))) as u64; r8 = (((r8) ^ (rdx))) as u64; r8 = ((r8).wrapping_add(1)) as u64; r8 = ((r8).wrapping_add(r9)) as u64; rdx = ((r8.wrapping_add(rcx)) as u64) as u64; rdx = (((rdx) as i64).wrapping_mul((r14) as i64) as u64) as u64; rdx = ((rdx).swap_bytes()) as u64; rdx = ((rdx).wrapping_add(rdi)) as u64; rax = ((r9.wrapping_add(r10)) as u64) as u64; r9 = rax; r9 = ((r9).rotate_right(42u32)) as u64; r9 = ((r9).wrapping_add(r11)) as u64; rcx = ((rax.wrapping_add(r11)) as u64) as u64; rax = ((rdx.wrapping_add(r8)) as u64) as u64; rax = (((rax) as i64).wrapping_mul((r14) as i64) as u64) as u64; rax = ((rax).swap_bytes()) as u64; rax = ((rax).wrapping_add(rcx)) as u64; rax = ((rax).wrapping_add(r9)) as u64; rax = ((rax).wrapping_add(crate::simd_ops::ru64(r13.wrapping_sub(0x8u64)))) as u64; rax = (((rax) as i64).wrapping_mul((r14) as i64) as u64) as u64; rax = ((rax).swap_bytes()) as u64; rax = ((rax).wrapping_add(crate::simd_ops::ru64(r15.wrapping_add(0x8u64)))) as u64; rcx = ((rcx).wrapping_add(rax)) as u64; rcx = (((rcx) as i64).wrapping_mul((r14) as i64) as u64) as u64; rcx = ((rcx).wrapping_add(rdi)) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(r13.wrapping_sub(0x20u64)))) as u64; rax = rcx; rax = ((rax).wrapping_shr(47u32)) as u64; rcx = (((rcx) ^ (rax))) as u64; rcx = (((rcx) as i64).wrapping_mul((r14) as i64) as u64) as u64; fl_l = ((rcx).wrapping_add(r9)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; bb = 10; continue; }
            8 => { rdi = (crate::simd_ops::ru64(r13.wrapping_sub(0x28u64))) as u64; rbp = (crate::simd_ops::ru64(r13.wrapping_sub(0x38u64))) as u64; rbp = ((rbp).wrapping_add(crate::simd_ops::ru64(r13.wrapping_sub(0x10u64)))) as u64; r14 = (crate::simd_ops::ru64(r13.wrapping_sub(0x30u64))) as u64; r14 = ((r14).wrapping_add(rbx)) as u64; r14 = (((r14) ^ (crate::simd_ops::ru64(r13.wrapping_sub(0x18u64))))) as u64; rdx = (0x9ddfea08eb382d69u64) as u64; r14 = (((r14) as i64).wrapping_mul((rdx) as i64) as u64) as u64; rax = r14; rax = ((rax).wrapping_shr(47u32)) as u64; r14 = (((r14) ^ (rax))) as u64; r14 = (((r14) ^ (crate::simd_ops::ru64(r13.wrapping_sub(0x18u64))))) as u64; r14 = (((r14) as i64).wrapping_mul((rdx) as i64) as u64) as u64; rax = r14; rax = ((rax).wrapping_shr(47u32)) as u64; r14 = (((r14) ^ (rax))) as u64; r14 = (((r14) as i64).wrapping_mul((rdx) as i64) as u64) as u64; rax = (crate::simd_ops::ru64(r13.wrapping_sub(0x30u64))) as u64; r10 = (crate::simd_ops::ru64(r13.wrapping_sub(0x40u64))) as u64; r10 = ((r10).wrapping_add(rbx)) as u64; r9 = ((rdi.wrapping_add(r10)) as u64) as u64; r9 = ((r9).wrapping_add(r14)) as u64; r8 = r10; rax = ((rax).wrapping_add(crate::simd_ops::ru64(r13.wrapping_sub(0x38u64)))) as u64; r10 = ((r10).wrapping_add(rax)) as u64; r9 = ((r9).rotate_right(21u32)) as u64; rax = r10; rax = ((rax).rotate_right(44u32)) as u64; rax = ((rax).wrapping_add(r9)) as u64; rax = ((rax).wrapping_add(r8)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xc0u64), rax); r9 = ((rdi.wrapping_add(r10)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xb8u64), r9); r11 = (crate::simd_ops::ru64(r13.wrapping_sub(0x8u64))) as u64; rax = (crate::simd_ops::ru64(r13.wrapping_sub(0x10u64))) as u64; rcx = (crate::simd_ops::ru64(r13.wrapping_sub(0x18u64))) as u64; r10 = (crate::simd_ops::ru64(r13.wrapping_sub(0x20u64))) as u64; r12 = (0xb492b66fbe98f273u64) as u64; r10 = ((r10).wrapping_add(r12)) as u64; r10 = ((r10).wrapping_add(rbp)) as u64; r9 = ((r11.wrapping_add(r10)) as u64) as u64; r9 = ((r9).wrapping_add(rdi)) as u64; r13 = r10; rax = ((rax).wrapping_add(rcx)) as u64; r10 = ((r10).wrapping_add(rax)) as u64; r9 = ((r9).rotate_right(21u32)) as u64; rax = r10; rax = ((rax).rotate_right(44u32)) as u64; r9 = ((r9).wrapping_add(rax)) as u64; r13 = ((r13).wrapping_add(r9)) as u64; r8 = ((r11.wrapping_add(r10)) as u64) as u64; rdi = (((rdi) as i64).wrapping_mul((r12) as i64) as u64) as u64; rdi = ((rdi).wrapping_add(crate::simd_ops::ru64(r15))) as u64; rbx = ((rbx).wrapping_sub(1)) as u64; fl_l = (((rbx) & (0xffffffffffffffc0u64))) as u64; fl_r = fl_l; rbx = ((fl_l as u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xc8u64), rbx); rax = r15.wrapping_add(0x30); crate::simd_ops::wu64(rsp.wrapping_add(0xb0u64), rax); r15 = rdi; bb = 9; continue; }
            9 => { r11 = (crate::simd_ops::ru64(rax.wrapping_sub(0x28u64))) as u64; rsi = ((r11.wrapping_add(rbp)) as u64) as u64; rsi = ((rsi).wrapping_add(r15)) as u64; rsi = ((rsi).wrapping_add(crate::simd_ops::ru64(rsp.wrapping_add(0xb8u64)))) as u64; rdi = (crate::simd_ops::ru64(rax)) as u64; rbp = ((rbp).wrapping_add(rdi)) as u64; rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0xc0u64))) as u64; rbp = ((rbp).wrapping_add(rdx)) as u64; rsi = ((rsi).rotate_right(37u32)) as u64; rsi = (((rsi) as i64).wrapping_mul((r12) as i64) as u64) as u64; rsi = (((rsi) ^ (r13))) as u64; rbx = (crate::simd_ops::ru64(rax.wrapping_sub(0x8u64))) as u64; rbp = ((rbp).rotate_right(42u32)) as u64; rbp = (((rbp) as i64).wrapping_mul((r12) as i64) as u64) as u64; rbp = ((rbp).wrapping_add(rbx)) as u64; rbp = ((rbp).wrapping_add(crate::simd_ops::ru64(rsp.wrapping_add(0xb8u64)))) as u64; r15 = ((r14.wrapping_add(r8)) as u64) as u64; r15 = ((r15).rotate_right(33u32)) as u64; r15 = (((r15) as i64).wrapping_mul((r12) as i64) as u64) as u64; rcx = ((rsi.wrapping_add(r8)) as u64) as u64; rdx = (((rdx) as i64).wrapping_mul((r12) as i64) as u64) as u64; r9 = (crate::simd_ops::ru64(rax.wrapping_sub(0x18u64))) as u64; r10 = (crate::simd_ops::ru64(rax.wrapping_sub(0x20u64))) as u64; r8 = (crate::simd_ops::ru64(rax.wrapping_sub(0x30u64))) as u64; r8 = ((r8).wrapping_add(rdx)) as u64; rdx = ((r9.wrapping_add(r8)) as u64) as u64; rdx = ((rdx).wrapping_add(rcx)) as u64; rcx = r8; rax = ((r10.wrapping_add(r11)) as u64) as u64; r8 = ((r8).wrapping_add(rax)) as u64; rdx = ((rdx).rotate_right(21u32)) as u64; r11 = r8; r11 = ((r11).rotate_right(44u32)) as u64; r11 = ((r11).wrapping_add(rdx)) as u64; r11 = ((r11).wrapping_add(rcx)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xc0u64), r11); rax = ((r9.wrapping_add(r8)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xb8u64), rax); rdx = ((r10.wrapping_add(rbp)) as u64) as u64; r9 = ((r15.wrapping_add(r13)) as u64) as u64; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xb0u64))) as u64; r10 = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; r9 = ((r9).wrapping_add(crate::simd_ops::ru64(rax.wrapping_sub(0x10u64)))) as u64; r8 = ((r10.wrapping_add(r9)) as u64) as u64; r8 = ((r8).wrapping_add(rdx)) as u64; r13 = r9; rax = ((rdi.wrapping_add(rbx)) as u64) as u64; r9 = ((r9).wrapping_add(rax)) as u64; r8 = ((r8).rotate_right(21u32)) as u64; rax = r9; rax = ((rax).rotate_right(44u32)) as u64; r8 = ((r8).wrapping_add(rax)) as u64; r13 = ((r13).wrapping_add(r8)) as u64; r8 = ((r10.wrapping_add(r9)) as u64) as u64; r14 = rsi; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xb0u64))) as u64; rax = ((rax).wrapping_add(0x40u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xb0u64), rax); fl_l = (crate::simd_ops::ru64(rsp.wrapping_add(0xc8u64))) as u64; fl_r = (0x40u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xc8u64), ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64); if (((fl_l as u64)) != ((fl_r as u64))) { bb = 9; continue; } rcx = r13; rcx = (((rcx) ^ (r11))) as u64; r10 = (0x9ddfea08eb382d69u64) as u64; rcx = (((rcx) as i64).wrapping_mul((r10) as i64) as u64) as u64; rax = rcx; rax = ((rax).wrapping_shr(47u32)) as u64; rcx = (((rcx) ^ (rax))) as u64; r13 = (((r13) ^ (rcx))) as u64; r13 = (((r13) as i64).wrapping_mul((r10) as i64) as u64) as u64; rcx = r13; rcx = ((rcx).wrapping_shr(47u32)) as u64; rcx = (((rcx) ^ (r13))) as u64; rcx = (((rcx) as i64).wrapping_mul((r10) as i64) as u64) as u64; rcx = ((rcx).wrapping_add(r15)) as u64; rdx = r8; rdx = (((rdx) ^ (crate::simd_ops::ru64(rsp.wrapping_add(0xb8u64))))) as u64; rdx = (((rdx) as i64).wrapping_mul((r10) as i64) as u64) as u64; rax = rdx; rax = ((rax).wrapping_shr(47u32)) as u64; rdx = (((rdx) ^ (rax))) as u64; r8 = (((r8) ^ (rdx))) as u64; r8 = (((r8) as i64).wrapping_mul((r10) as i64) as u64) as u64; rax = r8; rax = ((rax).wrapping_shr(47u32)) as u64; rax = (((rax) ^ (r8))) as u64; rax = (((rax) as i64).wrapping_mul((r10) as i64) as u64) as u64; rdx = rbp; rdx = ((rdx).wrapping_shr(47u32)) as u64; rdx = (((rdx) ^ (rbp))) as u64; rdx = (((rdx) as i64).wrapping_mul((r12) as i64) as u64) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; rdx = ((rdx).wrapping_add(rsi)) as u64; rdx = (((rdx) ^ (rcx))) as u64; rdx = (((rdx) as i64).wrapping_mul((r10) as i64) as u64) as u64; rax = rdx; rax = ((rax).wrapping_shr(47u32)) as u64; rdx = (((rdx) ^ (rax))) as u64; rcx = (((rcx) ^ (rdx))) as u64; rcx = (((rcx) as i64).wrapping_mul((r10) as i64) as u64) as u64; rax = rcx; rax = ((rax).wrapping_shr(47u32)) as u64; rcx = (((rcx) ^ (rax))) as u64; rcx = (((rcx) as i64).wrapping_mul((r10) as i64) as u64) as u64; bb = 10; continue; }
            10 => { rax = (0xde5fb9d2630458e9u64) as u64; let prod = (rax as u128).wrapping_mul((rcx) as u128); rax = prod as u64; rdx = (prod >> 64) as u64; r9 = ((rdx.wrapping_add(rax)) as u64) as u64; r8 = rsp.wrapping_add(0x30); rdx = rsp.wrapping_add(0x20); rcx = P2; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; fl_l = (crate::simd_ops::ru8(rsp.wrapping_add(0x28u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 11; continue; } rbx = ((rdi.wrapping_add(rdi.wrapping_mul(2))) as u64) as u64; rbx = ((rbx).wrapping_shl(4u32)) as u64; rbx = ((rbx).wrapping_add(crate::simd_ops::ru64(P4))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xb0u64), rbx); crate::simd_ops::lk_and_u32(rbx, (0xff000004u32) as u32); crate::simd_ops::lk_or_u32(rbx, (0x4u32) as u32); crate::simd_ops::wu8(rbx.wrapping_add(0x3u64), (0x4u8) as u8); crate::simd_ops::wu32(rbx.wrapping_add(0x4u64), (0x0u32) as u32); crate::simd_ops::wu8(rbx.wrapping_add(0x8u64), (0x0u8) as u8); rdx = rsp.wrapping_add(0x30); rcx = rbx; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx.wrapping_add(0x28u64), (0x0u64) as u64); bb = 11; continue; }
            11 => { rax = ((rdi.wrapping_add(rdi.wrapping_mul(2))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rax = ((rax).wrapping_add(crate::simd_ops::ru64(P4))) as u64; rcx = P6; crate::simd_ops::wu64(rax.wrapping_add(0x28u64), rcx); rcx = rsp.wrapping_add(0x30); rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); rax = 0; fl_l = 0; fl_r = 0; rsp = ((rsp).wrapping_add(0x68u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_36638<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 21>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x68u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (0x4000004u64) as u64); crate::simd_ops::wu8(rsp.wrapping_add(0x38u64), (0x0u8) as u8); rax = P0; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (0xfu64) as u64); r9 = 0xf; r8 = 0; fl_l = 0; fl_r = 0; rdx = rsp.wrapping_add(0x20); rcx = rsp.wrapping_add(0x30); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru32(rsp.wrapping_add(0x34u64))) as u32 as u64; rcx = (crate::simd_ops::ru32(rsp.wrapping_add(0x30u64))) as u32 as u64; rax = ((rcx as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_shr(24u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x80; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 1; continue; } rax = ((rax.wrapping_mul(8)) as u32) as u32 as u64; bb = 2; continue; }
            1 => { rax = (((rax as u32)).wrapping_shl(8u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x7c00; rax = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 2; continue; }
            2 => { rcx = ((((rcx as u32)) & (0xffffffu32))) as u32 as u64; rcx = (((rcx as u32)).wrapping_shl(3u32)) as u32 as u64; r15 = rsp.wrapping_add(0x38); fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; r15 = (if (((fl_l as u32)) > ((fl_r as u32))) { crate::simd_ops::ru64(rsp.wrapping_add(0x38u64)) } else { r15 }) as u64; fl_l = rbx; fl_r = (0x20u64) as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 7; continue; } fl_l = rbx; fl_r = (0x10u64) as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 6; continue; } fl_l = rbx; fl_r = (0x8u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 3; continue; } r8 = (0x9ae16a3b2f90404fu64) as u64; r9 = ((r8.wrapping_add(rbx.wrapping_mul(2))) as u64) as u64; r8 = (crate::simd_ops::ru64(r15)) as u64; rax = (0x651e95c4d06fbfb1u64) as u64; r8 = ((r8).wrapping_sub(rax)) as u64; rdx = (crate::simd_ops::ru64(rbx.wrapping_add(r15).wrapping_sub(0x8u64))) as u64; rdx = ((rdx).rotate_right(37u32)) as u64; rdx = (((rdx) as i64).wrapping_mul((r9) as i64) as u64) as u64; rdx = ((rdx).wrapping_add(r8)) as u64; r8 = ((r8).rotate_right(25u32)) as u64; r8 = ((r8).wrapping_add(crate::simd_ops::ru64(rbx.wrapping_add(r15).wrapping_sub(0x8u64)))) as u64; r8 = (((r8) as i64).wrapping_mul((r9) as i64) as u64) as u64; rcx = r8; rcx = (((rcx) ^ (rdx))) as u64; rcx = (((rcx) as i64).wrapping_mul((r9) as i64) as u64) as u64; rax = rcx; rax = ((rax).wrapping_shr(47u32)) as u64; rcx = (((rcx) ^ (rax))) as u64; rcx = (((rcx) ^ (r8))) as u64; rcx = (((rcx) as i64).wrapping_mul((r9) as i64) as u64) as u64; rax = rcx; rax = ((rax).wrapping_shr(47u32)) as u64; rcx = (((rcx) ^ (rax))) as u64; rcx = (((rcx) as i64).wrapping_mul((r9) as i64) as u64) as u64; bb = 10; continue; }
            3 => { fl_l = rbx; fl_r = (0x4u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 4; continue; } r8 = (0x9ae16a3b2f90404fu64) as u64; r8 = ((r8.wrapping_add(rbx.wrapping_mul(2))) as u64) as u64; rax = (crate::simd_ops::ru32(r15)) as u32 as u64; rdx = (crate::simd_ops::ru32(rbx.wrapping_add(r15).wrapping_sub(0x4u64))) as u32 as u64; rcx = ((rbx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rcx = (((rcx) ^ (rdx))) as u64; rcx = (((rcx) as i64).wrapping_mul((r8) as i64) as u64) as u64; rax = rcx; rax = ((rax).wrapping_shr(47u32)) as u64; rcx = (((rcx) ^ (rax))) as u64; rcx = (((rcx) ^ (rdx))) as u64; rcx = (((rcx) as i64).wrapping_mul((r8) as i64) as u64) as u64; rax = rcx; rax = ((rax).wrapping_shr(47u32)) as u64; rcx = (((rcx) ^ (rax))) as u64; rcx = (((rcx) as i64).wrapping_mul((r8) as i64) as u64) as u64; bb = 10; continue; }
            4 => { fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } rax = rbx; rax = ((rax).wrapping_shr(1u32)) as u64; rdx = ((crate::simd_ops::ru8(rax.wrapping_add(r15))) as u32) as u32 as u64; rax = (0xe16a3b2f90404f00u64) as u64; rdx = (((rdx) as i64).wrapping_mul((rax) as i64) as u64) as u64; rax = ((crate::simd_ops::ru8(r15)) as u32) as u32 as u64; r8 = (0x9ae16a3b2f90404fu64) as u64; rax = (((rax) as i64).wrapping_mul((r8) as i64) as u64) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; rax = ((crate::simd_ops::ru8(rbx.wrapping_add(r15).wrapping_sub(0x1u64))) as u32) as u32 as u64; rax = ((rbx.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = ((rax as u32)) as u32 as u64; rcx = (0xc3a5c85c97cb3127u64) as u64; rax = (((rax) as i64).wrapping_mul((rcx) as i64) as u64) as u64; rdx = (((rdx) ^ (rax))) as u64; rcx = rdx; rcx = ((rcx).wrapping_shr(47u32)) as u64; rcx = (((rcx) ^ (rdx))) as u64; rcx = (((rcx) as i64).wrapping_mul((r8) as i64) as u64) as u64; bb = 10; continue; }
            5 => { rcx = (0x9ae16a3b2f90404fu64) as u64; bb = 10; continue; }
            6 => { r8 = (0x9ae16a3b2f90404fu64) as u64; r11 = ((r8.wrapping_add(rbx.wrapping_mul(2))) as u64) as u64; r12 = (0xb492b66fbe98f273u64) as u64; r9 = (crate::simd_ops::ru64(r15)) as u64; r9 = (((r9) as i64).wrapping_mul((r12) as i64) as u64) as u64; rdx = (crate::simd_ops::ru64(r15.wrapping_add(0x8u64))) as u64; r10 = r11; r10 = (((r10) as i64).wrapping_mul((crate::simd_ops::ru64(rbx.wrapping_add(r15).wrapping_sub(0x8u64))) as i64) as u64) as u64; rcx = (crate::simd_ops::ru64(rbx.wrapping_add(r15).wrapping_sub(0x10u64))) as u64; rcx = (((rcx) as i64).wrapping_mul((r8) as i64) as u64) as u64; r8 = ((r8).wrapping_add(rdx)) as u64; r8 = ((r8).rotate_right(18u32)) as u64; r8 = ((r8).wrapping_add(r10)) as u64; r8 = ((r8).wrapping_add(r9)) as u64; rax = ((rdx.wrapping_add(r9)) as u64) as u64; rax = ((rax).rotate_right(43u32)) as u64; r10 = ((r10).rotate_right(30u32)) as u64; rcx = ((rcx).wrapping_add(rax)) as u64; rcx = ((rcx).wrapping_add(r10)) as u64; rcx = (((rcx) ^ (r8))) as u64; rcx = (((rcx) as i64).wrapping_mul((r11) as i64) as u64) as u64; rax = rcx; rax = ((rax).wrapping_shr(47u32)) as u64; rcx = (((rcx) ^ (rax))) as u64; rcx = (((rcx) ^ (r8))) as u64; rcx = (((rcx) as i64).wrapping_mul((r11) as i64) as u64) as u64; rax = rcx; rax = ((rax).wrapping_shr(47u32)) as u64; rcx = (((rcx) ^ (rax))) as u64; rcx = (((rcx) as i64).wrapping_mul((r11) as i64) as u64) as u64; bb = 10; continue; }
            7 => { r13 = ((rbx.wrapping_add(r15)) as u64) as u64; fl_l = rbx; fl_r = (0x40u64) as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 8; continue; } r8 = (0x9ae16a3b2f90404fu64) as u64; r14 = ((r8.wrapping_add(rbx.wrapping_mul(2))) as u64) as u64; r11 = (crate::simd_ops::ru64(r13.wrapping_sub(0x18u64))) as u64; r10 = (crate::simd_ops::ru64(r15.wrapping_add(0x10u64))) as u64; r10 = (((r10) as i64).wrapping_mul((r8) as i64) as u64) as u64; rax = (crate::simd_ops::ru64(r15.wrapping_add(0x18u64))) as u64; r9 = ((rax.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rdi = r14; rdi = (((rdi) as i64).wrapping_mul((crate::simd_ops::ru64(r13.wrapping_sub(0x10u64))) as i64) as u64) as u64; rdx = (crate::simd_ops::ru64(r15)) as u64; rdx = (((rdx) as i64).wrapping_mul((r8) as i64) as u64) as u64; rdx = ((rdx).wrapping_add(crate::simd_ops::ru64(r13.wrapping_sub(0x8u64)))) as u64; rax = (crate::simd_ops::ru64(r15.wrapping_add(0x8u64))) as u64; rax = ((rax).rotate_right(30u32)) as u64; rax = ((rax).wrapping_add(r11)) as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rax = rdx; rax = ((rax).rotate_right(43u32)) as u64; rcx = ((rcx).wrapping_add(rax)) as u64; r8 = (crate::simd_ops::ru64(r13.wrapping_sub(0x20u64))) as u64; r8 = (((r8) ^ (rdx))) as u64; r8 = ((r8).wrapping_add(1)) as u64; r8 = ((r8).wrapping_add(r9)) as u64; rdx = ((r8.wrapping_add(rcx)) as u64) as u64; rdx = (((rdx) as i64).wrapping_mul((r14) as i64) as u64) as u64; rdx = ((rdx).swap_bytes()) as u64; rdx = ((rdx).wrapping_add(rdi)) as u64; rax = ((r9.wrapping_add(r10)) as u64) as u64; r9 = rax; r9 = ((r9).rotate_right(42u32)) as u64; r9 = ((r9).wrapping_add(r11)) as u64; rcx = ((rax.wrapping_add(r11)) as u64) as u64; rax = ((rdx.wrapping_add(r8)) as u64) as u64; rax = (((rax) as i64).wrapping_mul((r14) as i64) as u64) as u64; rax = ((rax).swap_bytes()) as u64; rax = ((rax).wrapping_add(rcx)) as u64; rax = ((rax).wrapping_add(r9)) as u64; rax = ((rax).wrapping_add(crate::simd_ops::ru64(r13.wrapping_sub(0x8u64)))) as u64; rax = (((rax) as i64).wrapping_mul((r14) as i64) as u64) as u64; rax = ((rax).swap_bytes()) as u64; rax = ((rax).wrapping_add(crate::simd_ops::ru64(r15.wrapping_add(0x8u64)))) as u64; rcx = ((rcx).wrapping_add(rax)) as u64; rcx = (((rcx) as i64).wrapping_mul((r14) as i64) as u64) as u64; rcx = ((rcx).wrapping_add(rdi)) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(r13.wrapping_sub(0x20u64)))) as u64; rax = rcx; rax = ((rax).wrapping_shr(47u32)) as u64; rcx = (((rcx) ^ (rax))) as u64; rcx = (((rcx) as i64).wrapping_mul((r14) as i64) as u64) as u64; fl_l = ((rcx).wrapping_add(r9)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; bb = 10; continue; }
            8 => { rdi = (crate::simd_ops::ru64(r13.wrapping_sub(0x28u64))) as u64; rbp = (crate::simd_ops::ru64(r13.wrapping_sub(0x38u64))) as u64; rbp = ((rbp).wrapping_add(crate::simd_ops::ru64(r13.wrapping_sub(0x10u64)))) as u64; r14 = (crate::simd_ops::ru64(r13.wrapping_sub(0x30u64))) as u64; r14 = ((r14).wrapping_add(rbx)) as u64; r14 = (((r14) ^ (crate::simd_ops::ru64(r13.wrapping_sub(0x18u64))))) as u64; rdx = (0x9ddfea08eb382d69u64) as u64; r14 = (((r14) as i64).wrapping_mul((rdx) as i64) as u64) as u64; rax = r14; rax = ((rax).wrapping_shr(47u32)) as u64; r14 = (((r14) ^ (rax))) as u64; r14 = (((r14) ^ (crate::simd_ops::ru64(r13.wrapping_sub(0x18u64))))) as u64; r14 = (((r14) as i64).wrapping_mul((rdx) as i64) as u64) as u64; rax = r14; rax = ((rax).wrapping_shr(47u32)) as u64; r14 = (((r14) ^ (rax))) as u64; r14 = (((r14) as i64).wrapping_mul((rdx) as i64) as u64) as u64; rax = (crate::simd_ops::ru64(r13.wrapping_sub(0x30u64))) as u64; r10 = (crate::simd_ops::ru64(r13.wrapping_sub(0x40u64))) as u64; r10 = ((r10).wrapping_add(rbx)) as u64; r9 = ((rdi.wrapping_add(r10)) as u64) as u64; r9 = ((r9).wrapping_add(r14)) as u64; r8 = r10; rax = ((rax).wrapping_add(crate::simd_ops::ru64(r13.wrapping_sub(0x38u64)))) as u64; r10 = ((r10).wrapping_add(rax)) as u64; r9 = ((r9).rotate_right(21u32)) as u64; rax = r10; rax = ((rax).rotate_right(44u32)) as u64; rax = ((rax).wrapping_add(r9)) as u64; rax = ((rax).wrapping_add(r8)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xc0u64), rax); r9 = ((rdi.wrapping_add(r10)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xb8u64), r9); r11 = (crate::simd_ops::ru64(r13.wrapping_sub(0x8u64))) as u64; rax = (crate::simd_ops::ru64(r13.wrapping_sub(0x10u64))) as u64; rcx = (crate::simd_ops::ru64(r13.wrapping_sub(0x18u64))) as u64; r10 = (crate::simd_ops::ru64(r13.wrapping_sub(0x20u64))) as u64; r12 = (0xb492b66fbe98f273u64) as u64; r10 = ((r10).wrapping_add(r12)) as u64; r10 = ((r10).wrapping_add(rbp)) as u64; r9 = ((r11.wrapping_add(r10)) as u64) as u64; r9 = ((r9).wrapping_add(rdi)) as u64; r13 = r10; rax = ((rax).wrapping_add(rcx)) as u64; r10 = ((r10).wrapping_add(rax)) as u64; r9 = ((r9).rotate_right(21u32)) as u64; rax = r10; rax = ((rax).rotate_right(44u32)) as u64; r9 = ((r9).wrapping_add(rax)) as u64; r13 = ((r13).wrapping_add(r9)) as u64; r8 = ((r11.wrapping_add(r10)) as u64) as u64; rdi = (((rdi) as i64).wrapping_mul((r12) as i64) as u64) as u64; rdi = ((rdi).wrapping_add(crate::simd_ops::ru64(r15))) as u64; rbx = ((rbx).wrapping_sub(1)) as u64; fl_l = (((rbx) & (0xffffffffffffffc0u64))) as u64; fl_r = fl_l; rbx = ((fl_l as u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xc8u64), rbx); rax = r15.wrapping_add(0x30); crate::simd_ops::wu64(rsp.wrapping_add(0xb0u64), rax); r15 = rdi; bb = 9; continue; }
            9 => { r11 = (crate::simd_ops::ru64(rax.wrapping_sub(0x28u64))) as u64; rsi = ((r11.wrapping_add(rbp)) as u64) as u64; rsi = ((rsi).wrapping_add(r15)) as u64; rsi = ((rsi).wrapping_add(crate::simd_ops::ru64(rsp.wrapping_add(0xb8u64)))) as u64; rdi = (crate::simd_ops::ru64(rax)) as u64; rbp = ((rbp).wrapping_add(rdi)) as u64; rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0xc0u64))) as u64; rbp = ((rbp).wrapping_add(rdx)) as u64; rsi = ((rsi).rotate_right(37u32)) as u64; rsi = (((rsi) as i64).wrapping_mul((r12) as i64) as u64) as u64; rsi = (((rsi) ^ (r13))) as u64; rbx = (crate::simd_ops::ru64(rax.wrapping_sub(0x8u64))) as u64; rbp = ((rbp).rotate_right(42u32)) as u64; rbp = (((rbp) as i64).wrapping_mul((r12) as i64) as u64) as u64; rbp = ((rbp).wrapping_add(rbx)) as u64; rbp = ((rbp).wrapping_add(crate::simd_ops::ru64(rsp.wrapping_add(0xb8u64)))) as u64; r15 = ((r14.wrapping_add(r8)) as u64) as u64; r15 = ((r15).rotate_right(33u32)) as u64; r15 = (((r15) as i64).wrapping_mul((r12) as i64) as u64) as u64; rcx = ((rsi.wrapping_add(r8)) as u64) as u64; rdx = (((rdx) as i64).wrapping_mul((r12) as i64) as u64) as u64; r9 = (crate::simd_ops::ru64(rax.wrapping_sub(0x18u64))) as u64; r10 = (crate::simd_ops::ru64(rax.wrapping_sub(0x20u64))) as u64; r8 = (crate::simd_ops::ru64(rax.wrapping_sub(0x30u64))) as u64; r8 = ((r8).wrapping_add(rdx)) as u64; rdx = ((r9.wrapping_add(r8)) as u64) as u64; rdx = ((rdx).wrapping_add(rcx)) as u64; rcx = r8; rax = ((r10.wrapping_add(r11)) as u64) as u64; r8 = ((r8).wrapping_add(rax)) as u64; rdx = ((rdx).rotate_right(21u32)) as u64; r11 = r8; r11 = ((r11).rotate_right(44u32)) as u64; r11 = ((r11).wrapping_add(rdx)) as u64; r11 = ((r11).wrapping_add(rcx)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xc0u64), r11); rax = ((r9.wrapping_add(r8)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xb8u64), rax); rdx = ((r10.wrapping_add(rbp)) as u64) as u64; r9 = ((r15.wrapping_add(r13)) as u64) as u64; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xb0u64))) as u64; r10 = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; r9 = ((r9).wrapping_add(crate::simd_ops::ru64(rax.wrapping_sub(0x10u64)))) as u64; r8 = ((r10.wrapping_add(r9)) as u64) as u64; r8 = ((r8).wrapping_add(rdx)) as u64; r13 = r9; rax = ((rdi.wrapping_add(rbx)) as u64) as u64; r9 = ((r9).wrapping_add(rax)) as u64; r8 = ((r8).rotate_right(21u32)) as u64; rax = r9; rax = ((rax).rotate_right(44u32)) as u64; r8 = ((r8).wrapping_add(rax)) as u64; r13 = ((r13).wrapping_add(r8)) as u64; r8 = ((r10.wrapping_add(r9)) as u64) as u64; r14 = rsi; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xb0u64))) as u64; rax = ((rax).wrapping_add(0x40u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xb0u64), rax); fl_l = (crate::simd_ops::ru64(rsp.wrapping_add(0xc8u64))) as u64; fl_r = (0x40u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xc8u64), ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64); if (((fl_l as u64)) != ((fl_r as u64))) { bb = 9; continue; } rcx = r13; rcx = (((rcx) ^ (r11))) as u64; r10 = (0x9ddfea08eb382d69u64) as u64; rcx = (((rcx) as i64).wrapping_mul((r10) as i64) as u64) as u64; rax = rcx; rax = ((rax).wrapping_shr(47u32)) as u64; rcx = (((rcx) ^ (rax))) as u64; r13 = (((r13) ^ (rcx))) as u64; r13 = (((r13) as i64).wrapping_mul((r10) as i64) as u64) as u64; rcx = r13; rcx = ((rcx).wrapping_shr(47u32)) as u64; rcx = (((rcx) ^ (r13))) as u64; rcx = (((rcx) as i64).wrapping_mul((r10) as i64) as u64) as u64; rcx = ((rcx).wrapping_add(r15)) as u64; rdx = r8; rdx = (((rdx) ^ (crate::simd_ops::ru64(rsp.wrapping_add(0xb8u64))))) as u64; rdx = (((rdx) as i64).wrapping_mul((r10) as i64) as u64) as u64; rax = rdx; rax = ((rax).wrapping_shr(47u32)) as u64; rdx = (((rdx) ^ (rax))) as u64; r8 = (((r8) ^ (rdx))) as u64; r8 = (((r8) as i64).wrapping_mul((r10) as i64) as u64) as u64; rax = r8; rax = ((rax).wrapping_shr(47u32)) as u64; rax = (((rax) ^ (r8))) as u64; rax = (((rax) as i64).wrapping_mul((r10) as i64) as u64) as u64; rdx = rbp; rdx = ((rdx).wrapping_shr(47u32)) as u64; rdx = (((rdx) ^ (rbp))) as u64; rdx = (((rdx) as i64).wrapping_mul((r12) as i64) as u64) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; rdx = ((rdx).wrapping_add(rsi)) as u64; rdx = (((rdx) ^ (rcx))) as u64; rdx = (((rdx) as i64).wrapping_mul((r10) as i64) as u64) as u64; rax = rdx; rax = ((rax).wrapping_shr(47u32)) as u64; rdx = (((rdx) ^ (rax))) as u64; rcx = (((rcx) ^ (rdx))) as u64; rcx = (((rcx) as i64).wrapping_mul((r10) as i64) as u64) as u64; rax = rcx; rax = ((rax).wrapping_shr(47u32)) as u64; rcx = (((rcx) ^ (rax))) as u64; rcx = (((rcx) as i64).wrapping_mul((r10) as i64) as u64) as u64; bb = 10; continue; }
            10 => { rax = (0xde5fb9d2630458e9u64) as u64; let prod = (rax as u128).wrapping_mul((rcx) as u128); rax = prod as u64; rdx = (prod >> 64) as u64; r9 = ((rdx.wrapping_add(rax)) as u64) as u64; r8 = rsp.wrapping_add(0x30); rdx = rsp.wrapping_add(0x20); rcx = P2; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; fl_l = (crate::simd_ops::ru8(rsp.wrapping_add(0x28u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 11; continue; } rbx = ((rdi.wrapping_add(rdi.wrapping_mul(2))) as u64) as u64; rbx = ((rbx).wrapping_shl(4u32)) as u64; rbx = ((rbx).wrapping_add(crate::simd_ops::ru64(P4))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xb0u64), rbx); crate::simd_ops::lk_and_u32(rbx, (0xff000004u32) as u32); crate::simd_ops::lk_or_u32(rbx, (0x4u32) as u32); crate::simd_ops::wu8(rbx.wrapping_add(0x3u64), (0x4u8) as u8); crate::simd_ops::wu32(rbx.wrapping_add(0x4u64), (0x0u32) as u32); crate::simd_ops::wu8(rbx.wrapping_add(0x8u64), (0x0u8) as u8); rdx = rsp.wrapping_add(0x30); rcx = rbx; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx.wrapping_add(0x28u64), (0x0u64) as u64); bb = 11; continue; }
            11 => { rax = ((rdi.wrapping_add(rdi.wrapping_mul(2))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rax = ((rax).wrapping_add(crate::simd_ops::ru64(P4))) as u64; rcx = P6; crate::simd_ops::wu64(rax.wrapping_add(0x28u64), rcx); rcx = rsp.wrapping_add(0x30); rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); rax = 0; fl_l = 0; fl_r = 0; rsp = ((rsp).wrapping_add(0x68u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_36639<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 21>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x68u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (0x4000004u64) as u64); crate::simd_ops::wu8(rsp.wrapping_add(0x38u64), (0x0u8) as u8); rax = P0; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (0x11u64) as u64); r9 = 0x11; r8 = 0; fl_l = 0; fl_r = 0; rdx = rsp.wrapping_add(0x20); rcx = rsp.wrapping_add(0x30); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru32(rsp.wrapping_add(0x34u64))) as u32 as u64; rcx = (crate::simd_ops::ru32(rsp.wrapping_add(0x30u64))) as u32 as u64; rax = ((rcx as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_shr(24u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x80; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 1; continue; } rax = ((rax.wrapping_mul(8)) as u32) as u32 as u64; bb = 2; continue; }
            1 => { rax = (((rax as u32)).wrapping_shl(8u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x7c00; rax = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 2; continue; }
            2 => { rcx = ((((rcx as u32)) & (0xffffffu32))) as u32 as u64; rcx = (((rcx as u32)).wrapping_shl(3u32)) as u32 as u64; r15 = rsp.wrapping_add(0x38); fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; r15 = (if (((fl_l as u32)) > ((fl_r as u32))) { crate::simd_ops::ru64(rsp.wrapping_add(0x38u64)) } else { r15 }) as u64; fl_l = rbx; fl_r = (0x20u64) as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 7; continue; } fl_l = rbx; fl_r = (0x10u64) as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 6; continue; } fl_l = rbx; fl_r = (0x8u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 3; continue; } r8 = (0x9ae16a3b2f90404fu64) as u64; r9 = ((r8.wrapping_add(rbx.wrapping_mul(2))) as u64) as u64; r8 = (crate::simd_ops::ru64(r15)) as u64; rax = (0x651e95c4d06fbfb1u64) as u64; r8 = ((r8).wrapping_sub(rax)) as u64; rdx = (crate::simd_ops::ru64(rbx.wrapping_add(r15).wrapping_sub(0x8u64))) as u64; rdx = ((rdx).rotate_right(37u32)) as u64; rdx = (((rdx) as i64).wrapping_mul((r9) as i64) as u64) as u64; rdx = ((rdx).wrapping_add(r8)) as u64; r8 = ((r8).rotate_right(25u32)) as u64; r8 = ((r8).wrapping_add(crate::simd_ops::ru64(rbx.wrapping_add(r15).wrapping_sub(0x8u64)))) as u64; r8 = (((r8) as i64).wrapping_mul((r9) as i64) as u64) as u64; rcx = r8; rcx = (((rcx) ^ (rdx))) as u64; rcx = (((rcx) as i64).wrapping_mul((r9) as i64) as u64) as u64; rax = rcx; rax = ((rax).wrapping_shr(47u32)) as u64; rcx = (((rcx) ^ (rax))) as u64; rcx = (((rcx) ^ (r8))) as u64; rcx = (((rcx) as i64).wrapping_mul((r9) as i64) as u64) as u64; rax = rcx; rax = ((rax).wrapping_shr(47u32)) as u64; rcx = (((rcx) ^ (rax))) as u64; rcx = (((rcx) as i64).wrapping_mul((r9) as i64) as u64) as u64; bb = 10; continue; }
            3 => { fl_l = rbx; fl_r = (0x4u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 4; continue; } r8 = (0x9ae16a3b2f90404fu64) as u64; r8 = ((r8.wrapping_add(rbx.wrapping_mul(2))) as u64) as u64; rax = (crate::simd_ops::ru32(r15)) as u32 as u64; rdx = (crate::simd_ops::ru32(rbx.wrapping_add(r15).wrapping_sub(0x4u64))) as u32 as u64; rcx = ((rbx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rcx = (((rcx) ^ (rdx))) as u64; rcx = (((rcx) as i64).wrapping_mul((r8) as i64) as u64) as u64; rax = rcx; rax = ((rax).wrapping_shr(47u32)) as u64; rcx = (((rcx) ^ (rax))) as u64; rcx = (((rcx) ^ (rdx))) as u64; rcx = (((rcx) as i64).wrapping_mul((r8) as i64) as u64) as u64; rax = rcx; rax = ((rax).wrapping_shr(47u32)) as u64; rcx = (((rcx) ^ (rax))) as u64; rcx = (((rcx) as i64).wrapping_mul((r8) as i64) as u64) as u64; bb = 10; continue; }
            4 => { fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } rax = rbx; rax = ((rax).wrapping_shr(1u32)) as u64; rdx = ((crate::simd_ops::ru8(rax.wrapping_add(r15))) as u32) as u32 as u64; rax = (0xe16a3b2f90404f00u64) as u64; rdx = (((rdx) as i64).wrapping_mul((rax) as i64) as u64) as u64; rax = ((crate::simd_ops::ru8(r15)) as u32) as u32 as u64; r8 = (0x9ae16a3b2f90404fu64) as u64; rax = (((rax) as i64).wrapping_mul((r8) as i64) as u64) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; rax = ((crate::simd_ops::ru8(rbx.wrapping_add(r15).wrapping_sub(0x1u64))) as u32) as u32 as u64; rax = ((rbx.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = ((rax as u32)) as u32 as u64; rcx = (0xc3a5c85c97cb3127u64) as u64; rax = (((rax) as i64).wrapping_mul((rcx) as i64) as u64) as u64; rdx = (((rdx) ^ (rax))) as u64; rcx = rdx; rcx = ((rcx).wrapping_shr(47u32)) as u64; rcx = (((rcx) ^ (rdx))) as u64; rcx = (((rcx) as i64).wrapping_mul((r8) as i64) as u64) as u64; bb = 10; continue; }
            5 => { rcx = (0x9ae16a3b2f90404fu64) as u64; bb = 10; continue; }
            6 => { r8 = (0x9ae16a3b2f90404fu64) as u64; r11 = ((r8.wrapping_add(rbx.wrapping_mul(2))) as u64) as u64; r12 = (0xb492b66fbe98f273u64) as u64; r9 = (crate::simd_ops::ru64(r15)) as u64; r9 = (((r9) as i64).wrapping_mul((r12) as i64) as u64) as u64; rdx = (crate::simd_ops::ru64(r15.wrapping_add(0x8u64))) as u64; r10 = r11; r10 = (((r10) as i64).wrapping_mul((crate::simd_ops::ru64(rbx.wrapping_add(r15).wrapping_sub(0x8u64))) as i64) as u64) as u64; rcx = (crate::simd_ops::ru64(rbx.wrapping_add(r15).wrapping_sub(0x10u64))) as u64; rcx = (((rcx) as i64).wrapping_mul((r8) as i64) as u64) as u64; r8 = ((r8).wrapping_add(rdx)) as u64; r8 = ((r8).rotate_right(18u32)) as u64; r8 = ((r8).wrapping_add(r10)) as u64; r8 = ((r8).wrapping_add(r9)) as u64; rax = ((rdx.wrapping_add(r9)) as u64) as u64; rax = ((rax).rotate_right(43u32)) as u64; r10 = ((r10).rotate_right(30u32)) as u64; rcx = ((rcx).wrapping_add(rax)) as u64; rcx = ((rcx).wrapping_add(r10)) as u64; rcx = (((rcx) ^ (r8))) as u64; rcx = (((rcx) as i64).wrapping_mul((r11) as i64) as u64) as u64; rax = rcx; rax = ((rax).wrapping_shr(47u32)) as u64; rcx = (((rcx) ^ (rax))) as u64; rcx = (((rcx) ^ (r8))) as u64; rcx = (((rcx) as i64).wrapping_mul((r11) as i64) as u64) as u64; rax = rcx; rax = ((rax).wrapping_shr(47u32)) as u64; rcx = (((rcx) ^ (rax))) as u64; rcx = (((rcx) as i64).wrapping_mul((r11) as i64) as u64) as u64; bb = 10; continue; }
            7 => { r13 = ((rbx.wrapping_add(r15)) as u64) as u64; fl_l = rbx; fl_r = (0x40u64) as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 8; continue; } r8 = (0x9ae16a3b2f90404fu64) as u64; r14 = ((r8.wrapping_add(rbx.wrapping_mul(2))) as u64) as u64; r11 = (crate::simd_ops::ru64(r13.wrapping_sub(0x18u64))) as u64; r10 = (crate::simd_ops::ru64(r15.wrapping_add(0x10u64))) as u64; r10 = (((r10) as i64).wrapping_mul((r8) as i64) as u64) as u64; rax = (crate::simd_ops::ru64(r15.wrapping_add(0x18u64))) as u64; r9 = ((rax.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rdi = r14; rdi = (((rdi) as i64).wrapping_mul((crate::simd_ops::ru64(r13.wrapping_sub(0x10u64))) as i64) as u64) as u64; rdx = (crate::simd_ops::ru64(r15)) as u64; rdx = (((rdx) as i64).wrapping_mul((r8) as i64) as u64) as u64; rdx = ((rdx).wrapping_add(crate::simd_ops::ru64(r13.wrapping_sub(0x8u64)))) as u64; rax = (crate::simd_ops::ru64(r15.wrapping_add(0x8u64))) as u64; rax = ((rax).rotate_right(30u32)) as u64; rax = ((rax).wrapping_add(r11)) as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rax = rdx; rax = ((rax).rotate_right(43u32)) as u64; rcx = ((rcx).wrapping_add(rax)) as u64; r8 = (crate::simd_ops::ru64(r13.wrapping_sub(0x20u64))) as u64; r8 = (((r8) ^ (rdx))) as u64; r8 = ((r8).wrapping_add(1)) as u64; r8 = ((r8).wrapping_add(r9)) as u64; rdx = ((r8.wrapping_add(rcx)) as u64) as u64; rdx = (((rdx) as i64).wrapping_mul((r14) as i64) as u64) as u64; rdx = ((rdx).swap_bytes()) as u64; rdx = ((rdx).wrapping_add(rdi)) as u64; rax = ((r9.wrapping_add(r10)) as u64) as u64; r9 = rax; r9 = ((r9).rotate_right(42u32)) as u64; r9 = ((r9).wrapping_add(r11)) as u64; rcx = ((rax.wrapping_add(r11)) as u64) as u64; rax = ((rdx.wrapping_add(r8)) as u64) as u64; rax = (((rax) as i64).wrapping_mul((r14) as i64) as u64) as u64; rax = ((rax).swap_bytes()) as u64; rax = ((rax).wrapping_add(rcx)) as u64; rax = ((rax).wrapping_add(r9)) as u64; rax = ((rax).wrapping_add(crate::simd_ops::ru64(r13.wrapping_sub(0x8u64)))) as u64; rax = (((rax) as i64).wrapping_mul((r14) as i64) as u64) as u64; rax = ((rax).swap_bytes()) as u64; rax = ((rax).wrapping_add(crate::simd_ops::ru64(r15.wrapping_add(0x8u64)))) as u64; rcx = ((rcx).wrapping_add(rax)) as u64; rcx = (((rcx) as i64).wrapping_mul((r14) as i64) as u64) as u64; rcx = ((rcx).wrapping_add(rdi)) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(r13.wrapping_sub(0x20u64)))) as u64; rax = rcx; rax = ((rax).wrapping_shr(47u32)) as u64; rcx = (((rcx) ^ (rax))) as u64; rcx = (((rcx) as i64).wrapping_mul((r14) as i64) as u64) as u64; fl_l = ((rcx).wrapping_add(r9)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; bb = 10; continue; }
            8 => { rdi = (crate::simd_ops::ru64(r13.wrapping_sub(0x28u64))) as u64; rbp = (crate::simd_ops::ru64(r13.wrapping_sub(0x38u64))) as u64; rbp = ((rbp).wrapping_add(crate::simd_ops::ru64(r13.wrapping_sub(0x10u64)))) as u64; r14 = (crate::simd_ops::ru64(r13.wrapping_sub(0x30u64))) as u64; r14 = ((r14).wrapping_add(rbx)) as u64; r14 = (((r14) ^ (crate::simd_ops::ru64(r13.wrapping_sub(0x18u64))))) as u64; rdx = (0x9ddfea08eb382d69u64) as u64; r14 = (((r14) as i64).wrapping_mul((rdx) as i64) as u64) as u64; rax = r14; rax = ((rax).wrapping_shr(47u32)) as u64; r14 = (((r14) ^ (rax))) as u64; r14 = (((r14) ^ (crate::simd_ops::ru64(r13.wrapping_sub(0x18u64))))) as u64; r14 = (((r14) as i64).wrapping_mul((rdx) as i64) as u64) as u64; rax = r14; rax = ((rax).wrapping_shr(47u32)) as u64; r14 = (((r14) ^ (rax))) as u64; r14 = (((r14) as i64).wrapping_mul((rdx) as i64) as u64) as u64; rax = (crate::simd_ops::ru64(r13.wrapping_sub(0x30u64))) as u64; r10 = (crate::simd_ops::ru64(r13.wrapping_sub(0x40u64))) as u64; r10 = ((r10).wrapping_add(rbx)) as u64; r9 = ((rdi.wrapping_add(r10)) as u64) as u64; r9 = ((r9).wrapping_add(r14)) as u64; r8 = r10; rax = ((rax).wrapping_add(crate::simd_ops::ru64(r13.wrapping_sub(0x38u64)))) as u64; r10 = ((r10).wrapping_add(rax)) as u64; r9 = ((r9).rotate_right(21u32)) as u64; rax = r10; rax = ((rax).rotate_right(44u32)) as u64; rax = ((rax).wrapping_add(r9)) as u64; rax = ((rax).wrapping_add(r8)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xc0u64), rax); r9 = ((rdi.wrapping_add(r10)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xb8u64), r9); r11 = (crate::simd_ops::ru64(r13.wrapping_sub(0x8u64))) as u64; rax = (crate::simd_ops::ru64(r13.wrapping_sub(0x10u64))) as u64; rcx = (crate::simd_ops::ru64(r13.wrapping_sub(0x18u64))) as u64; r10 = (crate::simd_ops::ru64(r13.wrapping_sub(0x20u64))) as u64; r12 = (0xb492b66fbe98f273u64) as u64; r10 = ((r10).wrapping_add(r12)) as u64; r10 = ((r10).wrapping_add(rbp)) as u64; r9 = ((r11.wrapping_add(r10)) as u64) as u64; r9 = ((r9).wrapping_add(rdi)) as u64; r13 = r10; rax = ((rax).wrapping_add(rcx)) as u64; r10 = ((r10).wrapping_add(rax)) as u64; r9 = ((r9).rotate_right(21u32)) as u64; rax = r10; rax = ((rax).rotate_right(44u32)) as u64; r9 = ((r9).wrapping_add(rax)) as u64; r13 = ((r13).wrapping_add(r9)) as u64; r8 = ((r11.wrapping_add(r10)) as u64) as u64; rdi = (((rdi) as i64).wrapping_mul((r12) as i64) as u64) as u64; rdi = ((rdi).wrapping_add(crate::simd_ops::ru64(r15))) as u64; rbx = ((rbx).wrapping_sub(1)) as u64; fl_l = (((rbx) & (0xffffffffffffffc0u64))) as u64; fl_r = fl_l; rbx = ((fl_l as u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xc8u64), rbx); rax = r15.wrapping_add(0x30); crate::simd_ops::wu64(rsp.wrapping_add(0xb0u64), rax); r15 = rdi; bb = 9; continue; }
            9 => { r11 = (crate::simd_ops::ru64(rax.wrapping_sub(0x28u64))) as u64; rsi = ((r11.wrapping_add(rbp)) as u64) as u64; rsi = ((rsi).wrapping_add(r15)) as u64; rsi = ((rsi).wrapping_add(crate::simd_ops::ru64(rsp.wrapping_add(0xb8u64)))) as u64; rdi = (crate::simd_ops::ru64(rax)) as u64; rbp = ((rbp).wrapping_add(rdi)) as u64; rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0xc0u64))) as u64; rbp = ((rbp).wrapping_add(rdx)) as u64; rsi = ((rsi).rotate_right(37u32)) as u64; rsi = (((rsi) as i64).wrapping_mul((r12) as i64) as u64) as u64; rsi = (((rsi) ^ (r13))) as u64; rbx = (crate::simd_ops::ru64(rax.wrapping_sub(0x8u64))) as u64; rbp = ((rbp).rotate_right(42u32)) as u64; rbp = (((rbp) as i64).wrapping_mul((r12) as i64) as u64) as u64; rbp = ((rbp).wrapping_add(rbx)) as u64; rbp = ((rbp).wrapping_add(crate::simd_ops::ru64(rsp.wrapping_add(0xb8u64)))) as u64; r15 = ((r14.wrapping_add(r8)) as u64) as u64; r15 = ((r15).rotate_right(33u32)) as u64; r15 = (((r15) as i64).wrapping_mul((r12) as i64) as u64) as u64; rcx = ((rsi.wrapping_add(r8)) as u64) as u64; rdx = (((rdx) as i64).wrapping_mul((r12) as i64) as u64) as u64; r9 = (crate::simd_ops::ru64(rax.wrapping_sub(0x18u64))) as u64; r10 = (crate::simd_ops::ru64(rax.wrapping_sub(0x20u64))) as u64; r8 = (crate::simd_ops::ru64(rax.wrapping_sub(0x30u64))) as u64; r8 = ((r8).wrapping_add(rdx)) as u64; rdx = ((r9.wrapping_add(r8)) as u64) as u64; rdx = ((rdx).wrapping_add(rcx)) as u64; rcx = r8; rax = ((r10.wrapping_add(r11)) as u64) as u64; r8 = ((r8).wrapping_add(rax)) as u64; rdx = ((rdx).rotate_right(21u32)) as u64; r11 = r8; r11 = ((r11).rotate_right(44u32)) as u64; r11 = ((r11).wrapping_add(rdx)) as u64; r11 = ((r11).wrapping_add(rcx)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xc0u64), r11); rax = ((r9.wrapping_add(r8)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xb8u64), rax); rdx = ((r10.wrapping_add(rbp)) as u64) as u64; r9 = ((r15.wrapping_add(r13)) as u64) as u64; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xb0u64))) as u64; r10 = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; r9 = ((r9).wrapping_add(crate::simd_ops::ru64(rax.wrapping_sub(0x10u64)))) as u64; r8 = ((r10.wrapping_add(r9)) as u64) as u64; r8 = ((r8).wrapping_add(rdx)) as u64; r13 = r9; rax = ((rdi.wrapping_add(rbx)) as u64) as u64; r9 = ((r9).wrapping_add(rax)) as u64; r8 = ((r8).rotate_right(21u32)) as u64; rax = r9; rax = ((rax).rotate_right(44u32)) as u64; r8 = ((r8).wrapping_add(rax)) as u64; r13 = ((r13).wrapping_add(r8)) as u64; r8 = ((r10.wrapping_add(r9)) as u64) as u64; r14 = rsi; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xb0u64))) as u64; rax = ((rax).wrapping_add(0x40u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xb0u64), rax); fl_l = (crate::simd_ops::ru64(rsp.wrapping_add(0xc8u64))) as u64; fl_r = (0x40u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xc8u64), ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64); if (((fl_l as u64)) != ((fl_r as u64))) { bb = 9; continue; } rcx = r13; rcx = (((rcx) ^ (r11))) as u64; r10 = (0x9ddfea08eb382d69u64) as u64; rcx = (((rcx) as i64).wrapping_mul((r10) as i64) as u64) as u64; rax = rcx; rax = ((rax).wrapping_shr(47u32)) as u64; rcx = (((rcx) ^ (rax))) as u64; r13 = (((r13) ^ (rcx))) as u64; r13 = (((r13) as i64).wrapping_mul((r10) as i64) as u64) as u64; rcx = r13; rcx = ((rcx).wrapping_shr(47u32)) as u64; rcx = (((rcx) ^ (r13))) as u64; rcx = (((rcx) as i64).wrapping_mul((r10) as i64) as u64) as u64; rcx = ((rcx).wrapping_add(r15)) as u64; rdx = r8; rdx = (((rdx) ^ (crate::simd_ops::ru64(rsp.wrapping_add(0xb8u64))))) as u64; rdx = (((rdx) as i64).wrapping_mul((r10) as i64) as u64) as u64; rax = rdx; rax = ((rax).wrapping_shr(47u32)) as u64; rdx = (((rdx) ^ (rax))) as u64; r8 = (((r8) ^ (rdx))) as u64; r8 = (((r8) as i64).wrapping_mul((r10) as i64) as u64) as u64; rax = r8; rax = ((rax).wrapping_shr(47u32)) as u64; rax = (((rax) ^ (r8))) as u64; rax = (((rax) as i64).wrapping_mul((r10) as i64) as u64) as u64; rdx = rbp; rdx = ((rdx).wrapping_shr(47u32)) as u64; rdx = (((rdx) ^ (rbp))) as u64; rdx = (((rdx) as i64).wrapping_mul((r12) as i64) as u64) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; rdx = ((rdx).wrapping_add(rsi)) as u64; rdx = (((rdx) ^ (rcx))) as u64; rdx = (((rdx) as i64).wrapping_mul((r10) as i64) as u64) as u64; rax = rdx; rax = ((rax).wrapping_shr(47u32)) as u64; rdx = (((rdx) ^ (rax))) as u64; rcx = (((rcx) ^ (rdx))) as u64; rcx = (((rcx) as i64).wrapping_mul((r10) as i64) as u64) as u64; rax = rcx; rax = ((rax).wrapping_shr(47u32)) as u64; rcx = (((rcx) ^ (rax))) as u64; rcx = (((rcx) as i64).wrapping_mul((r10) as i64) as u64) as u64; bb = 10; continue; }
            10 => { rax = (0xde5fb9d2630458e9u64) as u64; let prod = (rax as u128).wrapping_mul((rcx) as u128); rax = prod as u64; rdx = (prod >> 64) as u64; r9 = ((rdx.wrapping_add(rax)) as u64) as u64; r8 = rsp.wrapping_add(0x30); rdx = rsp.wrapping_add(0x20); rcx = P2; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; fl_l = (crate::simd_ops::ru8(rsp.wrapping_add(0x28u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 11; continue; } rbx = ((rdi.wrapping_add(rdi.wrapping_mul(2))) as u64) as u64; rbx = ((rbx).wrapping_shl(4u32)) as u64; rbx = ((rbx).wrapping_add(crate::simd_ops::ru64(P4))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xb0u64), rbx); crate::simd_ops::lk_and_u32(rbx, (0xff000004u32) as u32); crate::simd_ops::lk_or_u32(rbx, (0x4u32) as u32); crate::simd_ops::wu8(rbx.wrapping_add(0x3u64), (0x4u8) as u8); crate::simd_ops::wu32(rbx.wrapping_add(0x4u64), (0x0u32) as u32); crate::simd_ops::wu8(rbx.wrapping_add(0x8u64), (0x0u8) as u8); rdx = rsp.wrapping_add(0x30); rcx = rbx; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx.wrapping_add(0x28u64), (0x0u64) as u64); bb = 11; continue; }
            11 => { rax = ((rdi.wrapping_add(rdi.wrapping_mul(2))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rax = ((rax).wrapping_add(crate::simd_ops::ru64(P4))) as u64; rcx = P6; crate::simd_ops::wu64(rax.wrapping_add(0x28u64), rcx); rcx = rsp.wrapping_add(0x30); rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); rax = 0; fl_l = 0; fl_r = 0; rsp = ((rsp).wrapping_add(0x68u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_36640<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 21>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x68u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (0x4000004u64) as u64); crate::simd_ops::wu8(rsp.wrapping_add(0x38u64), (0x0u8) as u8); rax = P0; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (0xdu64) as u64); r9 = 0xd; r8 = 0; fl_l = 0; fl_r = 0; rdx = rsp.wrapping_add(0x20); rcx = rsp.wrapping_add(0x30); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru32(rsp.wrapping_add(0x34u64))) as u32 as u64; rcx = (crate::simd_ops::ru32(rsp.wrapping_add(0x30u64))) as u32 as u64; rax = ((rcx as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_shr(24u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x80; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 1; continue; } rax = ((rax.wrapping_mul(8)) as u32) as u32 as u64; bb = 2; continue; }
            1 => { rax = (((rax as u32)).wrapping_shl(8u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x7c00; rax = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 2; continue; }
            2 => { rcx = ((((rcx as u32)) & (0xffffffu32))) as u32 as u64; rcx = (((rcx as u32)).wrapping_shl(3u32)) as u32 as u64; r15 = rsp.wrapping_add(0x38); fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; r15 = (if (((fl_l as u32)) > ((fl_r as u32))) { crate::simd_ops::ru64(rsp.wrapping_add(0x38u64)) } else { r15 }) as u64; fl_l = rbx; fl_r = (0x20u64) as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 7; continue; } fl_l = rbx; fl_r = (0x10u64) as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 6; continue; } fl_l = rbx; fl_r = (0x8u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 3; continue; } r8 = (0x9ae16a3b2f90404fu64) as u64; r9 = ((r8.wrapping_add(rbx.wrapping_mul(2))) as u64) as u64; r8 = (crate::simd_ops::ru64(r15)) as u64; rax = (0x651e95c4d06fbfb1u64) as u64; r8 = ((r8).wrapping_sub(rax)) as u64; rdx = (crate::simd_ops::ru64(rbx.wrapping_add(r15).wrapping_sub(0x8u64))) as u64; rdx = ((rdx).rotate_right(37u32)) as u64; rdx = (((rdx) as i64).wrapping_mul((r9) as i64) as u64) as u64; rdx = ((rdx).wrapping_add(r8)) as u64; r8 = ((r8).rotate_right(25u32)) as u64; r8 = ((r8).wrapping_add(crate::simd_ops::ru64(rbx.wrapping_add(r15).wrapping_sub(0x8u64)))) as u64; r8 = (((r8) as i64).wrapping_mul((r9) as i64) as u64) as u64; rcx = r8; rcx = (((rcx) ^ (rdx))) as u64; rcx = (((rcx) as i64).wrapping_mul((r9) as i64) as u64) as u64; rax = rcx; rax = ((rax).wrapping_shr(47u32)) as u64; rcx = (((rcx) ^ (rax))) as u64; rcx = (((rcx) ^ (r8))) as u64; rcx = (((rcx) as i64).wrapping_mul((r9) as i64) as u64) as u64; rax = rcx; rax = ((rax).wrapping_shr(47u32)) as u64; rcx = (((rcx) ^ (rax))) as u64; rcx = (((rcx) as i64).wrapping_mul((r9) as i64) as u64) as u64; bb = 10; continue; }
            3 => { fl_l = rbx; fl_r = (0x4u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 4; continue; } r8 = (0x9ae16a3b2f90404fu64) as u64; r8 = ((r8.wrapping_add(rbx.wrapping_mul(2))) as u64) as u64; rax = (crate::simd_ops::ru32(r15)) as u32 as u64; rdx = (crate::simd_ops::ru32(rbx.wrapping_add(r15).wrapping_sub(0x4u64))) as u32 as u64; rcx = ((rbx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rcx = (((rcx) ^ (rdx))) as u64; rcx = (((rcx) as i64).wrapping_mul((r8) as i64) as u64) as u64; rax = rcx; rax = ((rax).wrapping_shr(47u32)) as u64; rcx = (((rcx) ^ (rax))) as u64; rcx = (((rcx) ^ (rdx))) as u64; rcx = (((rcx) as i64).wrapping_mul((r8) as i64) as u64) as u64; rax = rcx; rax = ((rax).wrapping_shr(47u32)) as u64; rcx = (((rcx) ^ (rax))) as u64; rcx = (((rcx) as i64).wrapping_mul((r8) as i64) as u64) as u64; bb = 10; continue; }
            4 => { fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } rax = rbx; rax = ((rax).wrapping_shr(1u32)) as u64; rdx = ((crate::simd_ops::ru8(rax.wrapping_add(r15))) as u32) as u32 as u64; rax = (0xe16a3b2f90404f00u64) as u64; rdx = (((rdx) as i64).wrapping_mul((rax) as i64) as u64) as u64; rax = ((crate::simd_ops::ru8(r15)) as u32) as u32 as u64; r8 = (0x9ae16a3b2f90404fu64) as u64; rax = (((rax) as i64).wrapping_mul((r8) as i64) as u64) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; rax = ((crate::simd_ops::ru8(rbx.wrapping_add(r15).wrapping_sub(0x1u64))) as u32) as u32 as u64; rax = ((rbx.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = ((rax as u32)) as u32 as u64; rcx = (0xc3a5c85c97cb3127u64) as u64; rax = (((rax) as i64).wrapping_mul((rcx) as i64) as u64) as u64; rdx = (((rdx) ^ (rax))) as u64; rcx = rdx; rcx = ((rcx).wrapping_shr(47u32)) as u64; rcx = (((rcx) ^ (rdx))) as u64; rcx = (((rcx) as i64).wrapping_mul((r8) as i64) as u64) as u64; bb = 10; continue; }
            5 => { rcx = (0x9ae16a3b2f90404fu64) as u64; bb = 10; continue; }
            6 => { r8 = (0x9ae16a3b2f90404fu64) as u64; r11 = ((r8.wrapping_add(rbx.wrapping_mul(2))) as u64) as u64; r12 = (0xb492b66fbe98f273u64) as u64; r9 = (crate::simd_ops::ru64(r15)) as u64; r9 = (((r9) as i64).wrapping_mul((r12) as i64) as u64) as u64; rdx = (crate::simd_ops::ru64(r15.wrapping_add(0x8u64))) as u64; r10 = r11; r10 = (((r10) as i64).wrapping_mul((crate::simd_ops::ru64(rbx.wrapping_add(r15).wrapping_sub(0x8u64))) as i64) as u64) as u64; rcx = (crate::simd_ops::ru64(rbx.wrapping_add(r15).wrapping_sub(0x10u64))) as u64; rcx = (((rcx) as i64).wrapping_mul((r8) as i64) as u64) as u64; r8 = ((r8).wrapping_add(rdx)) as u64; r8 = ((r8).rotate_right(18u32)) as u64; r8 = ((r8).wrapping_add(r10)) as u64; r8 = ((r8).wrapping_add(r9)) as u64; rax = ((rdx.wrapping_add(r9)) as u64) as u64; rax = ((rax).rotate_right(43u32)) as u64; r10 = ((r10).rotate_right(30u32)) as u64; rcx = ((rcx).wrapping_add(rax)) as u64; rcx = ((rcx).wrapping_add(r10)) as u64; rcx = (((rcx) ^ (r8))) as u64; rcx = (((rcx) as i64).wrapping_mul((r11) as i64) as u64) as u64; rax = rcx; rax = ((rax).wrapping_shr(47u32)) as u64; rcx = (((rcx) ^ (rax))) as u64; rcx = (((rcx) ^ (r8))) as u64; rcx = (((rcx) as i64).wrapping_mul((r11) as i64) as u64) as u64; rax = rcx; rax = ((rax).wrapping_shr(47u32)) as u64; rcx = (((rcx) ^ (rax))) as u64; rcx = (((rcx) as i64).wrapping_mul((r11) as i64) as u64) as u64; bb = 10; continue; }
            7 => { r13 = ((rbx.wrapping_add(r15)) as u64) as u64; fl_l = rbx; fl_r = (0x40u64) as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 8; continue; } r8 = (0x9ae16a3b2f90404fu64) as u64; r14 = ((r8.wrapping_add(rbx.wrapping_mul(2))) as u64) as u64; r11 = (crate::simd_ops::ru64(r13.wrapping_sub(0x18u64))) as u64; r10 = (crate::simd_ops::ru64(r15.wrapping_add(0x10u64))) as u64; r10 = (((r10) as i64).wrapping_mul((r8) as i64) as u64) as u64; rax = (crate::simd_ops::ru64(r15.wrapping_add(0x18u64))) as u64; r9 = ((rax.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rdi = r14; rdi = (((rdi) as i64).wrapping_mul((crate::simd_ops::ru64(r13.wrapping_sub(0x10u64))) as i64) as u64) as u64; rdx = (crate::simd_ops::ru64(r15)) as u64; rdx = (((rdx) as i64).wrapping_mul((r8) as i64) as u64) as u64; rdx = ((rdx).wrapping_add(crate::simd_ops::ru64(r13.wrapping_sub(0x8u64)))) as u64; rax = (crate::simd_ops::ru64(r15.wrapping_add(0x8u64))) as u64; rax = ((rax).rotate_right(30u32)) as u64; rax = ((rax).wrapping_add(r11)) as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rax = rdx; rax = ((rax).rotate_right(43u32)) as u64; rcx = ((rcx).wrapping_add(rax)) as u64; r8 = (crate::simd_ops::ru64(r13.wrapping_sub(0x20u64))) as u64; r8 = (((r8) ^ (rdx))) as u64; r8 = ((r8).wrapping_add(1)) as u64; r8 = ((r8).wrapping_add(r9)) as u64; rdx = ((r8.wrapping_add(rcx)) as u64) as u64; rdx = (((rdx) as i64).wrapping_mul((r14) as i64) as u64) as u64; rdx = ((rdx).swap_bytes()) as u64; rdx = ((rdx).wrapping_add(rdi)) as u64; rax = ((r9.wrapping_add(r10)) as u64) as u64; r9 = rax; r9 = ((r9).rotate_right(42u32)) as u64; r9 = ((r9).wrapping_add(r11)) as u64; rcx = ((rax.wrapping_add(r11)) as u64) as u64; rax = ((rdx.wrapping_add(r8)) as u64) as u64; rax = (((rax) as i64).wrapping_mul((r14) as i64) as u64) as u64; rax = ((rax).swap_bytes()) as u64; rax = ((rax).wrapping_add(rcx)) as u64; rax = ((rax).wrapping_add(r9)) as u64; rax = ((rax).wrapping_add(crate::simd_ops::ru64(r13.wrapping_sub(0x8u64)))) as u64; rax = (((rax) as i64).wrapping_mul((r14) as i64) as u64) as u64; rax = ((rax).swap_bytes()) as u64; rax = ((rax).wrapping_add(crate::simd_ops::ru64(r15.wrapping_add(0x8u64)))) as u64; rcx = ((rcx).wrapping_add(rax)) as u64; rcx = (((rcx) as i64).wrapping_mul((r14) as i64) as u64) as u64; rcx = ((rcx).wrapping_add(rdi)) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(r13.wrapping_sub(0x20u64)))) as u64; rax = rcx; rax = ((rax).wrapping_shr(47u32)) as u64; rcx = (((rcx) ^ (rax))) as u64; rcx = (((rcx) as i64).wrapping_mul((r14) as i64) as u64) as u64; fl_l = ((rcx).wrapping_add(r9)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; bb = 10; continue; }
            8 => { rdi = (crate::simd_ops::ru64(r13.wrapping_sub(0x28u64))) as u64; rbp = (crate::simd_ops::ru64(r13.wrapping_sub(0x38u64))) as u64; rbp = ((rbp).wrapping_add(crate::simd_ops::ru64(r13.wrapping_sub(0x10u64)))) as u64; r14 = (crate::simd_ops::ru64(r13.wrapping_sub(0x30u64))) as u64; r14 = ((r14).wrapping_add(rbx)) as u64; r14 = (((r14) ^ (crate::simd_ops::ru64(r13.wrapping_sub(0x18u64))))) as u64; rdx = (0x9ddfea08eb382d69u64) as u64; r14 = (((r14) as i64).wrapping_mul((rdx) as i64) as u64) as u64; rax = r14; rax = ((rax).wrapping_shr(47u32)) as u64; r14 = (((r14) ^ (rax))) as u64; r14 = (((r14) ^ (crate::simd_ops::ru64(r13.wrapping_sub(0x18u64))))) as u64; r14 = (((r14) as i64).wrapping_mul((rdx) as i64) as u64) as u64; rax = r14; rax = ((rax).wrapping_shr(47u32)) as u64; r14 = (((r14) ^ (rax))) as u64; r14 = (((r14) as i64).wrapping_mul((rdx) as i64) as u64) as u64; rax = (crate::simd_ops::ru64(r13.wrapping_sub(0x30u64))) as u64; r10 = (crate::simd_ops::ru64(r13.wrapping_sub(0x40u64))) as u64; r10 = ((r10).wrapping_add(rbx)) as u64; r9 = ((rdi.wrapping_add(r10)) as u64) as u64; r9 = ((r9).wrapping_add(r14)) as u64; r8 = r10; rax = ((rax).wrapping_add(crate::simd_ops::ru64(r13.wrapping_sub(0x38u64)))) as u64; r10 = ((r10).wrapping_add(rax)) as u64; r9 = ((r9).rotate_right(21u32)) as u64; rax = r10; rax = ((rax).rotate_right(44u32)) as u64; rax = ((rax).wrapping_add(r9)) as u64; rax = ((rax).wrapping_add(r8)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xc0u64), rax); r9 = ((rdi.wrapping_add(r10)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xb8u64), r9); r11 = (crate::simd_ops::ru64(r13.wrapping_sub(0x8u64))) as u64; rax = (crate::simd_ops::ru64(r13.wrapping_sub(0x10u64))) as u64; rcx = (crate::simd_ops::ru64(r13.wrapping_sub(0x18u64))) as u64; r10 = (crate::simd_ops::ru64(r13.wrapping_sub(0x20u64))) as u64; r12 = (0xb492b66fbe98f273u64) as u64; r10 = ((r10).wrapping_add(r12)) as u64; r10 = ((r10).wrapping_add(rbp)) as u64; r9 = ((r11.wrapping_add(r10)) as u64) as u64; r9 = ((r9).wrapping_add(rdi)) as u64; r13 = r10; rax = ((rax).wrapping_add(rcx)) as u64; r10 = ((r10).wrapping_add(rax)) as u64; r9 = ((r9).rotate_right(21u32)) as u64; rax = r10; rax = ((rax).rotate_right(44u32)) as u64; r9 = ((r9).wrapping_add(rax)) as u64; r13 = ((r13).wrapping_add(r9)) as u64; r8 = ((r11.wrapping_add(r10)) as u64) as u64; rdi = (((rdi) as i64).wrapping_mul((r12) as i64) as u64) as u64; rdi = ((rdi).wrapping_add(crate::simd_ops::ru64(r15))) as u64; rbx = ((rbx).wrapping_sub(1)) as u64; fl_l = (((rbx) & (0xffffffffffffffc0u64))) as u64; fl_r = fl_l; rbx = ((fl_l as u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xc8u64), rbx); rax = r15.wrapping_add(0x30); crate::simd_ops::wu64(rsp.wrapping_add(0xb0u64), rax); r15 = rdi; bb = 9; continue; }
            9 => { r11 = (crate::simd_ops::ru64(rax.wrapping_sub(0x28u64))) as u64; rsi = ((r11.wrapping_add(rbp)) as u64) as u64; rsi = ((rsi).wrapping_add(r15)) as u64; rsi = ((rsi).wrapping_add(crate::simd_ops::ru64(rsp.wrapping_add(0xb8u64)))) as u64; rdi = (crate::simd_ops::ru64(rax)) as u64; rbp = ((rbp).wrapping_add(rdi)) as u64; rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0xc0u64))) as u64; rbp = ((rbp).wrapping_add(rdx)) as u64; rsi = ((rsi).rotate_right(37u32)) as u64; rsi = (((rsi) as i64).wrapping_mul((r12) as i64) as u64) as u64; rsi = (((rsi) ^ (r13))) as u64; rbx = (crate::simd_ops::ru64(rax.wrapping_sub(0x8u64))) as u64; rbp = ((rbp).rotate_right(42u32)) as u64; rbp = (((rbp) as i64).wrapping_mul((r12) as i64) as u64) as u64; rbp = ((rbp).wrapping_add(rbx)) as u64; rbp = ((rbp).wrapping_add(crate::simd_ops::ru64(rsp.wrapping_add(0xb8u64)))) as u64; r15 = ((r14.wrapping_add(r8)) as u64) as u64; r15 = ((r15).rotate_right(33u32)) as u64; r15 = (((r15) as i64).wrapping_mul((r12) as i64) as u64) as u64; rcx = ((rsi.wrapping_add(r8)) as u64) as u64; rdx = (((rdx) as i64).wrapping_mul((r12) as i64) as u64) as u64; r9 = (crate::simd_ops::ru64(rax.wrapping_sub(0x18u64))) as u64; r10 = (crate::simd_ops::ru64(rax.wrapping_sub(0x20u64))) as u64; r8 = (crate::simd_ops::ru64(rax.wrapping_sub(0x30u64))) as u64; r8 = ((r8).wrapping_add(rdx)) as u64; rdx = ((r9.wrapping_add(r8)) as u64) as u64; rdx = ((rdx).wrapping_add(rcx)) as u64; rcx = r8; rax = ((r10.wrapping_add(r11)) as u64) as u64; r8 = ((r8).wrapping_add(rax)) as u64; rdx = ((rdx).rotate_right(21u32)) as u64; r11 = r8; r11 = ((r11).rotate_right(44u32)) as u64; r11 = ((r11).wrapping_add(rdx)) as u64; r11 = ((r11).wrapping_add(rcx)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xc0u64), r11); rax = ((r9.wrapping_add(r8)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xb8u64), rax); rdx = ((r10.wrapping_add(rbp)) as u64) as u64; r9 = ((r15.wrapping_add(r13)) as u64) as u64; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xb0u64))) as u64; r10 = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; r9 = ((r9).wrapping_add(crate::simd_ops::ru64(rax.wrapping_sub(0x10u64)))) as u64; r8 = ((r10.wrapping_add(r9)) as u64) as u64; r8 = ((r8).wrapping_add(rdx)) as u64; r13 = r9; rax = ((rdi.wrapping_add(rbx)) as u64) as u64; r9 = ((r9).wrapping_add(rax)) as u64; r8 = ((r8).rotate_right(21u32)) as u64; rax = r9; rax = ((rax).rotate_right(44u32)) as u64; r8 = ((r8).wrapping_add(rax)) as u64; r13 = ((r13).wrapping_add(r8)) as u64; r8 = ((r10.wrapping_add(r9)) as u64) as u64; r14 = rsi; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xb0u64))) as u64; rax = ((rax).wrapping_add(0x40u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xb0u64), rax); fl_l = (crate::simd_ops::ru64(rsp.wrapping_add(0xc8u64))) as u64; fl_r = (0x40u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xc8u64), ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64); if (((fl_l as u64)) != ((fl_r as u64))) { bb = 9; continue; } rcx = r13; rcx = (((rcx) ^ (r11))) as u64; r10 = (0x9ddfea08eb382d69u64) as u64; rcx = (((rcx) as i64).wrapping_mul((r10) as i64) as u64) as u64; rax = rcx; rax = ((rax).wrapping_shr(47u32)) as u64; rcx = (((rcx) ^ (rax))) as u64; r13 = (((r13) ^ (rcx))) as u64; r13 = (((r13) as i64).wrapping_mul((r10) as i64) as u64) as u64; rcx = r13; rcx = ((rcx).wrapping_shr(47u32)) as u64; rcx = (((rcx) ^ (r13))) as u64; rcx = (((rcx) as i64).wrapping_mul((r10) as i64) as u64) as u64; rcx = ((rcx).wrapping_add(r15)) as u64; rdx = r8; rdx = (((rdx) ^ (crate::simd_ops::ru64(rsp.wrapping_add(0xb8u64))))) as u64; rdx = (((rdx) as i64).wrapping_mul((r10) as i64) as u64) as u64; rax = rdx; rax = ((rax).wrapping_shr(47u32)) as u64; rdx = (((rdx) ^ (rax))) as u64; r8 = (((r8) ^ (rdx))) as u64; r8 = (((r8) as i64).wrapping_mul((r10) as i64) as u64) as u64; rax = r8; rax = ((rax).wrapping_shr(47u32)) as u64; rax = (((rax) ^ (r8))) as u64; rax = (((rax) as i64).wrapping_mul((r10) as i64) as u64) as u64; rdx = rbp; rdx = ((rdx).wrapping_shr(47u32)) as u64; rdx = (((rdx) ^ (rbp))) as u64; rdx = (((rdx) as i64).wrapping_mul((r12) as i64) as u64) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; rdx = ((rdx).wrapping_add(rsi)) as u64; rdx = (((rdx) ^ (rcx))) as u64; rdx = (((rdx) as i64).wrapping_mul((r10) as i64) as u64) as u64; rax = rdx; rax = ((rax).wrapping_shr(47u32)) as u64; rdx = (((rdx) ^ (rax))) as u64; rcx = (((rcx) ^ (rdx))) as u64; rcx = (((rcx) as i64).wrapping_mul((r10) as i64) as u64) as u64; rax = rcx; rax = ((rax).wrapping_shr(47u32)) as u64; rcx = (((rcx) ^ (rax))) as u64; rcx = (((rcx) as i64).wrapping_mul((r10) as i64) as u64) as u64; bb = 10; continue; }
            10 => { rax = (0xde5fb9d2630458e9u64) as u64; let prod = (rax as u128).wrapping_mul((rcx) as u128); rax = prod as u64; rdx = (prod >> 64) as u64; r9 = ((rdx.wrapping_add(rax)) as u64) as u64; r8 = rsp.wrapping_add(0x30); rdx = rsp.wrapping_add(0x20); rcx = P2; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; fl_l = (crate::simd_ops::ru8(rsp.wrapping_add(0x28u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 11; continue; } rbx = ((rdi.wrapping_add(rdi.wrapping_mul(2))) as u64) as u64; rbx = ((rbx).wrapping_shl(4u32)) as u64; rbx = ((rbx).wrapping_add(crate::simd_ops::ru64(P4))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xb0u64), rbx); crate::simd_ops::lk_and_u32(rbx, (0xff000004u32) as u32); crate::simd_ops::lk_or_u32(rbx, (0x4u32) as u32); crate::simd_ops::wu8(rbx.wrapping_add(0x3u64), (0x4u8) as u8); crate::simd_ops::wu32(rbx.wrapping_add(0x4u64), (0x0u32) as u32); crate::simd_ops::wu8(rbx.wrapping_add(0x8u64), (0x0u8) as u8); rdx = rsp.wrapping_add(0x30); rcx = rbx; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx.wrapping_add(0x28u64), (0x0u64) as u64); bb = 11; continue; }
            11 => { rax = ((rdi.wrapping_add(rdi.wrapping_mul(2))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rax = ((rax).wrapping_add(crate::simd_ops::ru64(P4))) as u64; rcx = P6; crate::simd_ops::wu64(rax.wrapping_add(0x28u64), rcx); rcx = rsp.wrapping_add(0x30); rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); rax = 0; fl_l = 0; fl_r = 0; rsp = ((rsp).wrapping_add(0x68u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_36641<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 21>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x68u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (0x4000004u64) as u64); crate::simd_ops::wu8(rsp.wrapping_add(0x38u64), (0x0u8) as u8); rax = P0; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (0x10u64) as u64); r9 = 0x10; r8 = 0; fl_l = 0; fl_r = 0; rdx = rsp.wrapping_add(0x20); rcx = rsp.wrapping_add(0x30); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru32(rsp.wrapping_add(0x34u64))) as u32 as u64; rcx = (crate::simd_ops::ru32(rsp.wrapping_add(0x30u64))) as u32 as u64; rax = ((rcx as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_shr(24u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x80; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 1; continue; } rax = ((rax.wrapping_mul(8)) as u32) as u32 as u64; bb = 2; continue; }
            1 => { rax = (((rax as u32)).wrapping_shl(8u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x7c00; rax = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 2; continue; }
            2 => { rcx = ((((rcx as u32)) & (0xffffffu32))) as u32 as u64; rcx = (((rcx as u32)).wrapping_shl(3u32)) as u32 as u64; r15 = rsp.wrapping_add(0x38); fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; r15 = (if (((fl_l as u32)) > ((fl_r as u32))) { crate::simd_ops::ru64(rsp.wrapping_add(0x38u64)) } else { r15 }) as u64; fl_l = rbx; fl_r = (0x20u64) as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 7; continue; } fl_l = rbx; fl_r = (0x10u64) as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 6; continue; } fl_l = rbx; fl_r = (0x8u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 3; continue; } r8 = (0x9ae16a3b2f90404fu64) as u64; r9 = ((r8.wrapping_add(rbx.wrapping_mul(2))) as u64) as u64; r8 = (crate::simd_ops::ru64(r15)) as u64; rax = (0x651e95c4d06fbfb1u64) as u64; r8 = ((r8).wrapping_sub(rax)) as u64; rdx = (crate::simd_ops::ru64(rbx.wrapping_add(r15).wrapping_sub(0x8u64))) as u64; rdx = ((rdx).rotate_right(37u32)) as u64; rdx = (((rdx) as i64).wrapping_mul((r9) as i64) as u64) as u64; rdx = ((rdx).wrapping_add(r8)) as u64; r8 = ((r8).rotate_right(25u32)) as u64; r8 = ((r8).wrapping_add(crate::simd_ops::ru64(rbx.wrapping_add(r15).wrapping_sub(0x8u64)))) as u64; r8 = (((r8) as i64).wrapping_mul((r9) as i64) as u64) as u64; rcx = r8; rcx = (((rcx) ^ (rdx))) as u64; rcx = (((rcx) as i64).wrapping_mul((r9) as i64) as u64) as u64; rax = rcx; rax = ((rax).wrapping_shr(47u32)) as u64; rcx = (((rcx) ^ (rax))) as u64; rcx = (((rcx) ^ (r8))) as u64; rcx = (((rcx) as i64).wrapping_mul((r9) as i64) as u64) as u64; rax = rcx; rax = ((rax).wrapping_shr(47u32)) as u64; rcx = (((rcx) ^ (rax))) as u64; rcx = (((rcx) as i64).wrapping_mul((r9) as i64) as u64) as u64; bb = 10; continue; }
            3 => { fl_l = rbx; fl_r = (0x4u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 4; continue; } r8 = (0x9ae16a3b2f90404fu64) as u64; r8 = ((r8.wrapping_add(rbx.wrapping_mul(2))) as u64) as u64; rax = (crate::simd_ops::ru32(r15)) as u32 as u64; rdx = (crate::simd_ops::ru32(rbx.wrapping_add(r15).wrapping_sub(0x4u64))) as u32 as u64; rcx = ((rbx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rcx = (((rcx) ^ (rdx))) as u64; rcx = (((rcx) as i64).wrapping_mul((r8) as i64) as u64) as u64; rax = rcx; rax = ((rax).wrapping_shr(47u32)) as u64; rcx = (((rcx) ^ (rax))) as u64; rcx = (((rcx) ^ (rdx))) as u64; rcx = (((rcx) as i64).wrapping_mul((r8) as i64) as u64) as u64; rax = rcx; rax = ((rax).wrapping_shr(47u32)) as u64; rcx = (((rcx) ^ (rax))) as u64; rcx = (((rcx) as i64).wrapping_mul((r8) as i64) as u64) as u64; bb = 10; continue; }
            4 => { fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } rax = rbx; rax = ((rax).wrapping_shr(1u32)) as u64; rdx = ((crate::simd_ops::ru8(rax.wrapping_add(r15))) as u32) as u32 as u64; rax = (0xe16a3b2f90404f00u64) as u64; rdx = (((rdx) as i64).wrapping_mul((rax) as i64) as u64) as u64; rax = ((crate::simd_ops::ru8(r15)) as u32) as u32 as u64; r8 = (0x9ae16a3b2f90404fu64) as u64; rax = (((rax) as i64).wrapping_mul((r8) as i64) as u64) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; rax = ((crate::simd_ops::ru8(rbx.wrapping_add(r15).wrapping_sub(0x1u64))) as u32) as u32 as u64; rax = ((rbx.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = ((rax as u32)) as u32 as u64; rcx = (0xc3a5c85c97cb3127u64) as u64; rax = (((rax) as i64).wrapping_mul((rcx) as i64) as u64) as u64; rdx = (((rdx) ^ (rax))) as u64; rcx = rdx; rcx = ((rcx).wrapping_shr(47u32)) as u64; rcx = (((rcx) ^ (rdx))) as u64; rcx = (((rcx) as i64).wrapping_mul((r8) as i64) as u64) as u64; bb = 10; continue; }
            5 => { rcx = (0x9ae16a3b2f90404fu64) as u64; bb = 10; continue; }
            6 => { r8 = (0x9ae16a3b2f90404fu64) as u64; r11 = ((r8.wrapping_add(rbx.wrapping_mul(2))) as u64) as u64; r12 = (0xb492b66fbe98f273u64) as u64; r9 = (crate::simd_ops::ru64(r15)) as u64; r9 = (((r9) as i64).wrapping_mul((r12) as i64) as u64) as u64; rdx = (crate::simd_ops::ru64(r15.wrapping_add(0x8u64))) as u64; r10 = r11; r10 = (((r10) as i64).wrapping_mul((crate::simd_ops::ru64(rbx.wrapping_add(r15).wrapping_sub(0x8u64))) as i64) as u64) as u64; rcx = (crate::simd_ops::ru64(rbx.wrapping_add(r15).wrapping_sub(0x10u64))) as u64; rcx = (((rcx) as i64).wrapping_mul((r8) as i64) as u64) as u64; r8 = ((r8).wrapping_add(rdx)) as u64; r8 = ((r8).rotate_right(18u32)) as u64; r8 = ((r8).wrapping_add(r10)) as u64; r8 = ((r8).wrapping_add(r9)) as u64; rax = ((rdx.wrapping_add(r9)) as u64) as u64; rax = ((rax).rotate_right(43u32)) as u64; r10 = ((r10).rotate_right(30u32)) as u64; rcx = ((rcx).wrapping_add(rax)) as u64; rcx = ((rcx).wrapping_add(r10)) as u64; rcx = (((rcx) ^ (r8))) as u64; rcx = (((rcx) as i64).wrapping_mul((r11) as i64) as u64) as u64; rax = rcx; rax = ((rax).wrapping_shr(47u32)) as u64; rcx = (((rcx) ^ (rax))) as u64; rcx = (((rcx) ^ (r8))) as u64; rcx = (((rcx) as i64).wrapping_mul((r11) as i64) as u64) as u64; rax = rcx; rax = ((rax).wrapping_shr(47u32)) as u64; rcx = (((rcx) ^ (rax))) as u64; rcx = (((rcx) as i64).wrapping_mul((r11) as i64) as u64) as u64; bb = 10; continue; }
            7 => { r13 = ((rbx.wrapping_add(r15)) as u64) as u64; fl_l = rbx; fl_r = (0x40u64) as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 8; continue; } r8 = (0x9ae16a3b2f90404fu64) as u64; r14 = ((r8.wrapping_add(rbx.wrapping_mul(2))) as u64) as u64; r11 = (crate::simd_ops::ru64(r13.wrapping_sub(0x18u64))) as u64; r10 = (crate::simd_ops::ru64(r15.wrapping_add(0x10u64))) as u64; r10 = (((r10) as i64).wrapping_mul((r8) as i64) as u64) as u64; rax = (crate::simd_ops::ru64(r15.wrapping_add(0x18u64))) as u64; r9 = ((rax.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rdi = r14; rdi = (((rdi) as i64).wrapping_mul((crate::simd_ops::ru64(r13.wrapping_sub(0x10u64))) as i64) as u64) as u64; rdx = (crate::simd_ops::ru64(r15)) as u64; rdx = (((rdx) as i64).wrapping_mul((r8) as i64) as u64) as u64; rdx = ((rdx).wrapping_add(crate::simd_ops::ru64(r13.wrapping_sub(0x8u64)))) as u64; rax = (crate::simd_ops::ru64(r15.wrapping_add(0x8u64))) as u64; rax = ((rax).rotate_right(30u32)) as u64; rax = ((rax).wrapping_add(r11)) as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rax = rdx; rax = ((rax).rotate_right(43u32)) as u64; rcx = ((rcx).wrapping_add(rax)) as u64; r8 = (crate::simd_ops::ru64(r13.wrapping_sub(0x20u64))) as u64; r8 = (((r8) ^ (rdx))) as u64; r8 = ((r8).wrapping_add(1)) as u64; r8 = ((r8).wrapping_add(r9)) as u64; rdx = ((r8.wrapping_add(rcx)) as u64) as u64; rdx = (((rdx) as i64).wrapping_mul((r14) as i64) as u64) as u64; rdx = ((rdx).swap_bytes()) as u64; rdx = ((rdx).wrapping_add(rdi)) as u64; rax = ((r9.wrapping_add(r10)) as u64) as u64; r9 = rax; r9 = ((r9).rotate_right(42u32)) as u64; r9 = ((r9).wrapping_add(r11)) as u64; rcx = ((rax.wrapping_add(r11)) as u64) as u64; rax = ((rdx.wrapping_add(r8)) as u64) as u64; rax = (((rax) as i64).wrapping_mul((r14) as i64) as u64) as u64; rax = ((rax).swap_bytes()) as u64; rax = ((rax).wrapping_add(rcx)) as u64; rax = ((rax).wrapping_add(r9)) as u64; rax = ((rax).wrapping_add(crate::simd_ops::ru64(r13.wrapping_sub(0x8u64)))) as u64; rax = (((rax) as i64).wrapping_mul((r14) as i64) as u64) as u64; rax = ((rax).swap_bytes()) as u64; rax = ((rax).wrapping_add(crate::simd_ops::ru64(r15.wrapping_add(0x8u64)))) as u64; rcx = ((rcx).wrapping_add(rax)) as u64; rcx = (((rcx) as i64).wrapping_mul((r14) as i64) as u64) as u64; rcx = ((rcx).wrapping_add(rdi)) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(r13.wrapping_sub(0x20u64)))) as u64; rax = rcx; rax = ((rax).wrapping_shr(47u32)) as u64; rcx = (((rcx) ^ (rax))) as u64; rcx = (((rcx) as i64).wrapping_mul((r14) as i64) as u64) as u64; fl_l = ((rcx).wrapping_add(r9)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; bb = 10; continue; }
            8 => { rdi = (crate::simd_ops::ru64(r13.wrapping_sub(0x28u64))) as u64; rbp = (crate::simd_ops::ru64(r13.wrapping_sub(0x38u64))) as u64; rbp = ((rbp).wrapping_add(crate::simd_ops::ru64(r13.wrapping_sub(0x10u64)))) as u64; r14 = (crate::simd_ops::ru64(r13.wrapping_sub(0x30u64))) as u64; r14 = ((r14).wrapping_add(rbx)) as u64; r14 = (((r14) ^ (crate::simd_ops::ru64(r13.wrapping_sub(0x18u64))))) as u64; rdx = (0x9ddfea08eb382d69u64) as u64; r14 = (((r14) as i64).wrapping_mul((rdx) as i64) as u64) as u64; rax = r14; rax = ((rax).wrapping_shr(47u32)) as u64; r14 = (((r14) ^ (rax))) as u64; r14 = (((r14) ^ (crate::simd_ops::ru64(r13.wrapping_sub(0x18u64))))) as u64; r14 = (((r14) as i64).wrapping_mul((rdx) as i64) as u64) as u64; rax = r14; rax = ((rax).wrapping_shr(47u32)) as u64; r14 = (((r14) ^ (rax))) as u64; r14 = (((r14) as i64).wrapping_mul((rdx) as i64) as u64) as u64; rax = (crate::simd_ops::ru64(r13.wrapping_sub(0x30u64))) as u64; r10 = (crate::simd_ops::ru64(r13.wrapping_sub(0x40u64))) as u64; r10 = ((r10).wrapping_add(rbx)) as u64; r9 = ((rdi.wrapping_add(r10)) as u64) as u64; r9 = ((r9).wrapping_add(r14)) as u64; r8 = r10; rax = ((rax).wrapping_add(crate::simd_ops::ru64(r13.wrapping_sub(0x38u64)))) as u64; r10 = ((r10).wrapping_add(rax)) as u64; r9 = ((r9).rotate_right(21u32)) as u64; rax = r10; rax = ((rax).rotate_right(44u32)) as u64; rax = ((rax).wrapping_add(r9)) as u64; rax = ((rax).wrapping_add(r8)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xc0u64), rax); r9 = ((rdi.wrapping_add(r10)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xb8u64), r9); r11 = (crate::simd_ops::ru64(r13.wrapping_sub(0x8u64))) as u64; rax = (crate::simd_ops::ru64(r13.wrapping_sub(0x10u64))) as u64; rcx = (crate::simd_ops::ru64(r13.wrapping_sub(0x18u64))) as u64; r10 = (crate::simd_ops::ru64(r13.wrapping_sub(0x20u64))) as u64; r12 = (0xb492b66fbe98f273u64) as u64; r10 = ((r10).wrapping_add(r12)) as u64; r10 = ((r10).wrapping_add(rbp)) as u64; r9 = ((r11.wrapping_add(r10)) as u64) as u64; r9 = ((r9).wrapping_add(rdi)) as u64; r13 = r10; rax = ((rax).wrapping_add(rcx)) as u64; r10 = ((r10).wrapping_add(rax)) as u64; r9 = ((r9).rotate_right(21u32)) as u64; rax = r10; rax = ((rax).rotate_right(44u32)) as u64; r9 = ((r9).wrapping_add(rax)) as u64; r13 = ((r13).wrapping_add(r9)) as u64; r8 = ((r11.wrapping_add(r10)) as u64) as u64; rdi = (((rdi) as i64).wrapping_mul((r12) as i64) as u64) as u64; rdi = ((rdi).wrapping_add(crate::simd_ops::ru64(r15))) as u64; rbx = ((rbx).wrapping_sub(1)) as u64; fl_l = (((rbx) & (0xffffffffffffffc0u64))) as u64; fl_r = fl_l; rbx = ((fl_l as u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xc8u64), rbx); rax = r15.wrapping_add(0x30); crate::simd_ops::wu64(rsp.wrapping_add(0xb0u64), rax); r15 = rdi; bb = 9; continue; }
            9 => { r11 = (crate::simd_ops::ru64(rax.wrapping_sub(0x28u64))) as u64; rsi = ((r11.wrapping_add(rbp)) as u64) as u64; rsi = ((rsi).wrapping_add(r15)) as u64; rsi = ((rsi).wrapping_add(crate::simd_ops::ru64(rsp.wrapping_add(0xb8u64)))) as u64; rdi = (crate::simd_ops::ru64(rax)) as u64; rbp = ((rbp).wrapping_add(rdi)) as u64; rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0xc0u64))) as u64; rbp = ((rbp).wrapping_add(rdx)) as u64; rsi = ((rsi).rotate_right(37u32)) as u64; rsi = (((rsi) as i64).wrapping_mul((r12) as i64) as u64) as u64; rsi = (((rsi) ^ (r13))) as u64; rbx = (crate::simd_ops::ru64(rax.wrapping_sub(0x8u64))) as u64; rbp = ((rbp).rotate_right(42u32)) as u64; rbp = (((rbp) as i64).wrapping_mul((r12) as i64) as u64) as u64; rbp = ((rbp).wrapping_add(rbx)) as u64; rbp = ((rbp).wrapping_add(crate::simd_ops::ru64(rsp.wrapping_add(0xb8u64)))) as u64; r15 = ((r14.wrapping_add(r8)) as u64) as u64; r15 = ((r15).rotate_right(33u32)) as u64; r15 = (((r15) as i64).wrapping_mul((r12) as i64) as u64) as u64; rcx = ((rsi.wrapping_add(r8)) as u64) as u64; rdx = (((rdx) as i64).wrapping_mul((r12) as i64) as u64) as u64; r9 = (crate::simd_ops::ru64(rax.wrapping_sub(0x18u64))) as u64; r10 = (crate::simd_ops::ru64(rax.wrapping_sub(0x20u64))) as u64; r8 = (crate::simd_ops::ru64(rax.wrapping_sub(0x30u64))) as u64; r8 = ((r8).wrapping_add(rdx)) as u64; rdx = ((r9.wrapping_add(r8)) as u64) as u64; rdx = ((rdx).wrapping_add(rcx)) as u64; rcx = r8; rax = ((r10.wrapping_add(r11)) as u64) as u64; r8 = ((r8).wrapping_add(rax)) as u64; rdx = ((rdx).rotate_right(21u32)) as u64; r11 = r8; r11 = ((r11).rotate_right(44u32)) as u64; r11 = ((r11).wrapping_add(rdx)) as u64; r11 = ((r11).wrapping_add(rcx)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xc0u64), r11); rax = ((r9.wrapping_add(r8)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xb8u64), rax); rdx = ((r10.wrapping_add(rbp)) as u64) as u64; r9 = ((r15.wrapping_add(r13)) as u64) as u64; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xb0u64))) as u64; r10 = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; r9 = ((r9).wrapping_add(crate::simd_ops::ru64(rax.wrapping_sub(0x10u64)))) as u64; r8 = ((r10.wrapping_add(r9)) as u64) as u64; r8 = ((r8).wrapping_add(rdx)) as u64; r13 = r9; rax = ((rdi.wrapping_add(rbx)) as u64) as u64; r9 = ((r9).wrapping_add(rax)) as u64; r8 = ((r8).rotate_right(21u32)) as u64; rax = r9; rax = ((rax).rotate_right(44u32)) as u64; r8 = ((r8).wrapping_add(rax)) as u64; r13 = ((r13).wrapping_add(r8)) as u64; r8 = ((r10.wrapping_add(r9)) as u64) as u64; r14 = rsi; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xb0u64))) as u64; rax = ((rax).wrapping_add(0x40u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xb0u64), rax); fl_l = (crate::simd_ops::ru64(rsp.wrapping_add(0xc8u64))) as u64; fl_r = (0x40u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xc8u64), ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64); if (((fl_l as u64)) != ((fl_r as u64))) { bb = 9; continue; } rcx = r13; rcx = (((rcx) ^ (r11))) as u64; r10 = (0x9ddfea08eb382d69u64) as u64; rcx = (((rcx) as i64).wrapping_mul((r10) as i64) as u64) as u64; rax = rcx; rax = ((rax).wrapping_shr(47u32)) as u64; rcx = (((rcx) ^ (rax))) as u64; r13 = (((r13) ^ (rcx))) as u64; r13 = (((r13) as i64).wrapping_mul((r10) as i64) as u64) as u64; rcx = r13; rcx = ((rcx).wrapping_shr(47u32)) as u64; rcx = (((rcx) ^ (r13))) as u64; rcx = (((rcx) as i64).wrapping_mul((r10) as i64) as u64) as u64; rcx = ((rcx).wrapping_add(r15)) as u64; rdx = r8; rdx = (((rdx) ^ (crate::simd_ops::ru64(rsp.wrapping_add(0xb8u64))))) as u64; rdx = (((rdx) as i64).wrapping_mul((r10) as i64) as u64) as u64; rax = rdx; rax = ((rax).wrapping_shr(47u32)) as u64; rdx = (((rdx) ^ (rax))) as u64; r8 = (((r8) ^ (rdx))) as u64; r8 = (((r8) as i64).wrapping_mul((r10) as i64) as u64) as u64; rax = r8; rax = ((rax).wrapping_shr(47u32)) as u64; rax = (((rax) ^ (r8))) as u64; rax = (((rax) as i64).wrapping_mul((r10) as i64) as u64) as u64; rdx = rbp; rdx = ((rdx).wrapping_shr(47u32)) as u64; rdx = (((rdx) ^ (rbp))) as u64; rdx = (((rdx) as i64).wrapping_mul((r12) as i64) as u64) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; rdx = ((rdx).wrapping_add(rsi)) as u64; rdx = (((rdx) ^ (rcx))) as u64; rdx = (((rdx) as i64).wrapping_mul((r10) as i64) as u64) as u64; rax = rdx; rax = ((rax).wrapping_shr(47u32)) as u64; rdx = (((rdx) ^ (rax))) as u64; rcx = (((rcx) ^ (rdx))) as u64; rcx = (((rcx) as i64).wrapping_mul((r10) as i64) as u64) as u64; rax = rcx; rax = ((rax).wrapping_shr(47u32)) as u64; rcx = (((rcx) ^ (rax))) as u64; rcx = (((rcx) as i64).wrapping_mul((r10) as i64) as u64) as u64; bb = 10; continue; }
            10 => { rax = (0xde5fb9d2630458e9u64) as u64; let prod = (rax as u128).wrapping_mul((rcx) as u128); rax = prod as u64; rdx = (prod >> 64) as u64; r9 = ((rdx.wrapping_add(rax)) as u64) as u64; r8 = rsp.wrapping_add(0x30); rdx = rsp.wrapping_add(0x20); rcx = P2; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; fl_l = (crate::simd_ops::ru8(rsp.wrapping_add(0x28u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 11; continue; } rbx = ((rdi.wrapping_add(rdi.wrapping_mul(2))) as u64) as u64; rbx = ((rbx).wrapping_shl(4u32)) as u64; rbx = ((rbx).wrapping_add(crate::simd_ops::ru64(P4))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xb0u64), rbx); crate::simd_ops::lk_and_u32(rbx, (0xff000004u32) as u32); crate::simd_ops::lk_or_u32(rbx, (0x4u32) as u32); crate::simd_ops::wu8(rbx.wrapping_add(0x3u64), (0x4u8) as u8); crate::simd_ops::wu32(rbx.wrapping_add(0x4u64), (0x0u32) as u32); crate::simd_ops::wu8(rbx.wrapping_add(0x8u64), (0x0u8) as u8); rdx = rsp.wrapping_add(0x30); rcx = rbx; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx.wrapping_add(0x28u64), (0x0u64) as u64); bb = 11; continue; }
            11 => { rax = ((rdi.wrapping_add(rdi.wrapping_mul(2))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rax = ((rax).wrapping_add(crate::simd_ops::ru64(P4))) as u64; rcx = P6; crate::simd_ops::wu64(rax.wrapping_add(0x28u64), rcx); rcx = rsp.wrapping_add(0x30); rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); rax = 0; fl_l = 0; fl_r = 0; rsp = ((rsp).wrapping_add(0x68u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_36642<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 21>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x68u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (0x4000004u64) as u64); crate::simd_ops::wu8(rsp.wrapping_add(0x38u64), (0x0u8) as u8); rax = P0; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (0xeu64) as u64); r9 = 0xe; r8 = 0; fl_l = 0; fl_r = 0; rdx = rsp.wrapping_add(0x20); rcx = rsp.wrapping_add(0x30); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru32(rsp.wrapping_add(0x34u64))) as u32 as u64; rcx = (crate::simd_ops::ru32(rsp.wrapping_add(0x30u64))) as u32 as u64; rax = ((rcx as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_shr(24u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x80; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 1; continue; } rax = ((rax.wrapping_mul(8)) as u32) as u32 as u64; bb = 2; continue; }
            1 => { rax = (((rax as u32)).wrapping_shl(8u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x7c00; rax = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 2; continue; }
            2 => { rcx = ((((rcx as u32)) & (0xffffffu32))) as u32 as u64; rcx = (((rcx as u32)).wrapping_shl(3u32)) as u32 as u64; r15 = rsp.wrapping_add(0x38); fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; r15 = (if (((fl_l as u32)) > ((fl_r as u32))) { crate::simd_ops::ru64(rsp.wrapping_add(0x38u64)) } else { r15 }) as u64; fl_l = rbx; fl_r = (0x20u64) as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 7; continue; } fl_l = rbx; fl_r = (0x10u64) as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 6; continue; } fl_l = rbx; fl_r = (0x8u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 3; continue; } r8 = (0x9ae16a3b2f90404fu64) as u64; r9 = ((r8.wrapping_add(rbx.wrapping_mul(2))) as u64) as u64; r8 = (crate::simd_ops::ru64(r15)) as u64; rax = (0x651e95c4d06fbfb1u64) as u64; r8 = ((r8).wrapping_sub(rax)) as u64; rdx = (crate::simd_ops::ru64(rbx.wrapping_add(r15).wrapping_sub(0x8u64))) as u64; rdx = ((rdx).rotate_right(37u32)) as u64; rdx = (((rdx) as i64).wrapping_mul((r9) as i64) as u64) as u64; rdx = ((rdx).wrapping_add(r8)) as u64; r8 = ((r8).rotate_right(25u32)) as u64; r8 = ((r8).wrapping_add(crate::simd_ops::ru64(rbx.wrapping_add(r15).wrapping_sub(0x8u64)))) as u64; r8 = (((r8) as i64).wrapping_mul((r9) as i64) as u64) as u64; rcx = r8; rcx = (((rcx) ^ (rdx))) as u64; rcx = (((rcx) as i64).wrapping_mul((r9) as i64) as u64) as u64; rax = rcx; rax = ((rax).wrapping_shr(47u32)) as u64; rcx = (((rcx) ^ (rax))) as u64; rcx = (((rcx) ^ (r8))) as u64; rcx = (((rcx) as i64).wrapping_mul((r9) as i64) as u64) as u64; rax = rcx; rax = ((rax).wrapping_shr(47u32)) as u64; rcx = (((rcx) ^ (rax))) as u64; rcx = (((rcx) as i64).wrapping_mul((r9) as i64) as u64) as u64; bb = 10; continue; }
            3 => { fl_l = rbx; fl_r = (0x4u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 4; continue; } r8 = (0x9ae16a3b2f90404fu64) as u64; r8 = ((r8.wrapping_add(rbx.wrapping_mul(2))) as u64) as u64; rax = (crate::simd_ops::ru32(r15)) as u32 as u64; rdx = (crate::simd_ops::ru32(rbx.wrapping_add(r15).wrapping_sub(0x4u64))) as u32 as u64; rcx = ((rbx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rcx = (((rcx) ^ (rdx))) as u64; rcx = (((rcx) as i64).wrapping_mul((r8) as i64) as u64) as u64; rax = rcx; rax = ((rax).wrapping_shr(47u32)) as u64; rcx = (((rcx) ^ (rax))) as u64; rcx = (((rcx) ^ (rdx))) as u64; rcx = (((rcx) as i64).wrapping_mul((r8) as i64) as u64) as u64; rax = rcx; rax = ((rax).wrapping_shr(47u32)) as u64; rcx = (((rcx) ^ (rax))) as u64; rcx = (((rcx) as i64).wrapping_mul((r8) as i64) as u64) as u64; bb = 10; continue; }
            4 => { fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } rax = rbx; rax = ((rax).wrapping_shr(1u32)) as u64; rdx = ((crate::simd_ops::ru8(rax.wrapping_add(r15))) as u32) as u32 as u64; rax = (0xe16a3b2f90404f00u64) as u64; rdx = (((rdx) as i64).wrapping_mul((rax) as i64) as u64) as u64; rax = ((crate::simd_ops::ru8(r15)) as u32) as u32 as u64; r8 = (0x9ae16a3b2f90404fu64) as u64; rax = (((rax) as i64).wrapping_mul((r8) as i64) as u64) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; rax = ((crate::simd_ops::ru8(rbx.wrapping_add(r15).wrapping_sub(0x1u64))) as u32) as u32 as u64; rax = ((rbx.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = ((rax as u32)) as u32 as u64; rcx = (0xc3a5c85c97cb3127u64) as u64; rax = (((rax) as i64).wrapping_mul((rcx) as i64) as u64) as u64; rdx = (((rdx) ^ (rax))) as u64; rcx = rdx; rcx = ((rcx).wrapping_shr(47u32)) as u64; rcx = (((rcx) ^ (rdx))) as u64; rcx = (((rcx) as i64).wrapping_mul((r8) as i64) as u64) as u64; bb = 10; continue; }
            5 => { rcx = (0x9ae16a3b2f90404fu64) as u64; bb = 10; continue; }
            6 => { r8 = (0x9ae16a3b2f90404fu64) as u64; r11 = ((r8.wrapping_add(rbx.wrapping_mul(2))) as u64) as u64; r12 = (0xb492b66fbe98f273u64) as u64; r9 = (crate::simd_ops::ru64(r15)) as u64; r9 = (((r9) as i64).wrapping_mul((r12) as i64) as u64) as u64; rdx = (crate::simd_ops::ru64(r15.wrapping_add(0x8u64))) as u64; r10 = r11; r10 = (((r10) as i64).wrapping_mul((crate::simd_ops::ru64(rbx.wrapping_add(r15).wrapping_sub(0x8u64))) as i64) as u64) as u64; rcx = (crate::simd_ops::ru64(rbx.wrapping_add(r15).wrapping_sub(0x10u64))) as u64; rcx = (((rcx) as i64).wrapping_mul((r8) as i64) as u64) as u64; r8 = ((r8).wrapping_add(rdx)) as u64; r8 = ((r8).rotate_right(18u32)) as u64; r8 = ((r8).wrapping_add(r10)) as u64; r8 = ((r8).wrapping_add(r9)) as u64; rax = ((rdx.wrapping_add(r9)) as u64) as u64; rax = ((rax).rotate_right(43u32)) as u64; r10 = ((r10).rotate_right(30u32)) as u64; rcx = ((rcx).wrapping_add(rax)) as u64; rcx = ((rcx).wrapping_add(r10)) as u64; rcx = (((rcx) ^ (r8))) as u64; rcx = (((rcx) as i64).wrapping_mul((r11) as i64) as u64) as u64; rax = rcx; rax = ((rax).wrapping_shr(47u32)) as u64; rcx = (((rcx) ^ (rax))) as u64; rcx = (((rcx) ^ (r8))) as u64; rcx = (((rcx) as i64).wrapping_mul((r11) as i64) as u64) as u64; rax = rcx; rax = ((rax).wrapping_shr(47u32)) as u64; rcx = (((rcx) ^ (rax))) as u64; rcx = (((rcx) as i64).wrapping_mul((r11) as i64) as u64) as u64; bb = 10; continue; }
            7 => { r13 = ((rbx.wrapping_add(r15)) as u64) as u64; fl_l = rbx; fl_r = (0x40u64) as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 8; continue; } r8 = (0x9ae16a3b2f90404fu64) as u64; r14 = ((r8.wrapping_add(rbx.wrapping_mul(2))) as u64) as u64; r11 = (crate::simd_ops::ru64(r13.wrapping_sub(0x18u64))) as u64; r10 = (crate::simd_ops::ru64(r15.wrapping_add(0x10u64))) as u64; r10 = (((r10) as i64).wrapping_mul((r8) as i64) as u64) as u64; rax = (crate::simd_ops::ru64(r15.wrapping_add(0x18u64))) as u64; r9 = ((rax.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rdi = r14; rdi = (((rdi) as i64).wrapping_mul((crate::simd_ops::ru64(r13.wrapping_sub(0x10u64))) as i64) as u64) as u64; rdx = (crate::simd_ops::ru64(r15)) as u64; rdx = (((rdx) as i64).wrapping_mul((r8) as i64) as u64) as u64; rdx = ((rdx).wrapping_add(crate::simd_ops::ru64(r13.wrapping_sub(0x8u64)))) as u64; rax = (crate::simd_ops::ru64(r15.wrapping_add(0x8u64))) as u64; rax = ((rax).rotate_right(30u32)) as u64; rax = ((rax).wrapping_add(r11)) as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rax = rdx; rax = ((rax).rotate_right(43u32)) as u64; rcx = ((rcx).wrapping_add(rax)) as u64; r8 = (crate::simd_ops::ru64(r13.wrapping_sub(0x20u64))) as u64; r8 = (((r8) ^ (rdx))) as u64; r8 = ((r8).wrapping_add(1)) as u64; r8 = ((r8).wrapping_add(r9)) as u64; rdx = ((r8.wrapping_add(rcx)) as u64) as u64; rdx = (((rdx) as i64).wrapping_mul((r14) as i64) as u64) as u64; rdx = ((rdx).swap_bytes()) as u64; rdx = ((rdx).wrapping_add(rdi)) as u64; rax = ((r9.wrapping_add(r10)) as u64) as u64; r9 = rax; r9 = ((r9).rotate_right(42u32)) as u64; r9 = ((r9).wrapping_add(r11)) as u64; rcx = ((rax.wrapping_add(r11)) as u64) as u64; rax = ((rdx.wrapping_add(r8)) as u64) as u64; rax = (((rax) as i64).wrapping_mul((r14) as i64) as u64) as u64; rax = ((rax).swap_bytes()) as u64; rax = ((rax).wrapping_add(rcx)) as u64; rax = ((rax).wrapping_add(r9)) as u64; rax = ((rax).wrapping_add(crate::simd_ops::ru64(r13.wrapping_sub(0x8u64)))) as u64; rax = (((rax) as i64).wrapping_mul((r14) as i64) as u64) as u64; rax = ((rax).swap_bytes()) as u64; rax = ((rax).wrapping_add(crate::simd_ops::ru64(r15.wrapping_add(0x8u64)))) as u64; rcx = ((rcx).wrapping_add(rax)) as u64; rcx = (((rcx) as i64).wrapping_mul((r14) as i64) as u64) as u64; rcx = ((rcx).wrapping_add(rdi)) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(r13.wrapping_sub(0x20u64)))) as u64; rax = rcx; rax = ((rax).wrapping_shr(47u32)) as u64; rcx = (((rcx) ^ (rax))) as u64; rcx = (((rcx) as i64).wrapping_mul((r14) as i64) as u64) as u64; fl_l = ((rcx).wrapping_add(r9)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; bb = 10; continue; }
            8 => { rdi = (crate::simd_ops::ru64(r13.wrapping_sub(0x28u64))) as u64; rbp = (crate::simd_ops::ru64(r13.wrapping_sub(0x38u64))) as u64; rbp = ((rbp).wrapping_add(crate::simd_ops::ru64(r13.wrapping_sub(0x10u64)))) as u64; r14 = (crate::simd_ops::ru64(r13.wrapping_sub(0x30u64))) as u64; r14 = ((r14).wrapping_add(rbx)) as u64; r14 = (((r14) ^ (crate::simd_ops::ru64(r13.wrapping_sub(0x18u64))))) as u64; rdx = (0x9ddfea08eb382d69u64) as u64; r14 = (((r14) as i64).wrapping_mul((rdx) as i64) as u64) as u64; rax = r14; rax = ((rax).wrapping_shr(47u32)) as u64; r14 = (((r14) ^ (rax))) as u64; r14 = (((r14) ^ (crate::simd_ops::ru64(r13.wrapping_sub(0x18u64))))) as u64; r14 = (((r14) as i64).wrapping_mul((rdx) as i64) as u64) as u64; rax = r14; rax = ((rax).wrapping_shr(47u32)) as u64; r14 = (((r14) ^ (rax))) as u64; r14 = (((r14) as i64).wrapping_mul((rdx) as i64) as u64) as u64; rax = (crate::simd_ops::ru64(r13.wrapping_sub(0x30u64))) as u64; r10 = (crate::simd_ops::ru64(r13.wrapping_sub(0x40u64))) as u64; r10 = ((r10).wrapping_add(rbx)) as u64; r9 = ((rdi.wrapping_add(r10)) as u64) as u64; r9 = ((r9).wrapping_add(r14)) as u64; r8 = r10; rax = ((rax).wrapping_add(crate::simd_ops::ru64(r13.wrapping_sub(0x38u64)))) as u64; r10 = ((r10).wrapping_add(rax)) as u64; r9 = ((r9).rotate_right(21u32)) as u64; rax = r10; rax = ((rax).rotate_right(44u32)) as u64; rax = ((rax).wrapping_add(r9)) as u64; rax = ((rax).wrapping_add(r8)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xc0u64), rax); r9 = ((rdi.wrapping_add(r10)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xb8u64), r9); r11 = (crate::simd_ops::ru64(r13.wrapping_sub(0x8u64))) as u64; rax = (crate::simd_ops::ru64(r13.wrapping_sub(0x10u64))) as u64; rcx = (crate::simd_ops::ru64(r13.wrapping_sub(0x18u64))) as u64; r10 = (crate::simd_ops::ru64(r13.wrapping_sub(0x20u64))) as u64; r12 = (0xb492b66fbe98f273u64) as u64; r10 = ((r10).wrapping_add(r12)) as u64; r10 = ((r10).wrapping_add(rbp)) as u64; r9 = ((r11.wrapping_add(r10)) as u64) as u64; r9 = ((r9).wrapping_add(rdi)) as u64; r13 = r10; rax = ((rax).wrapping_add(rcx)) as u64; r10 = ((r10).wrapping_add(rax)) as u64; r9 = ((r9).rotate_right(21u32)) as u64; rax = r10; rax = ((rax).rotate_right(44u32)) as u64; r9 = ((r9).wrapping_add(rax)) as u64; r13 = ((r13).wrapping_add(r9)) as u64; r8 = ((r11.wrapping_add(r10)) as u64) as u64; rdi = (((rdi) as i64).wrapping_mul((r12) as i64) as u64) as u64; rdi = ((rdi).wrapping_add(crate::simd_ops::ru64(r15))) as u64; rbx = ((rbx).wrapping_sub(1)) as u64; fl_l = (((rbx) & (0xffffffffffffffc0u64))) as u64; fl_r = fl_l; rbx = ((fl_l as u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xc8u64), rbx); rax = r15.wrapping_add(0x30); crate::simd_ops::wu64(rsp.wrapping_add(0xb0u64), rax); r15 = rdi; bb = 9; continue; }
            9 => { r11 = (crate::simd_ops::ru64(rax.wrapping_sub(0x28u64))) as u64; rsi = ((r11.wrapping_add(rbp)) as u64) as u64; rsi = ((rsi).wrapping_add(r15)) as u64; rsi = ((rsi).wrapping_add(crate::simd_ops::ru64(rsp.wrapping_add(0xb8u64)))) as u64; rdi = (crate::simd_ops::ru64(rax)) as u64; rbp = ((rbp).wrapping_add(rdi)) as u64; rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0xc0u64))) as u64; rbp = ((rbp).wrapping_add(rdx)) as u64; rsi = ((rsi).rotate_right(37u32)) as u64; rsi = (((rsi) as i64).wrapping_mul((r12) as i64) as u64) as u64; rsi = (((rsi) ^ (r13))) as u64; rbx = (crate::simd_ops::ru64(rax.wrapping_sub(0x8u64))) as u64; rbp = ((rbp).rotate_right(42u32)) as u64; rbp = (((rbp) as i64).wrapping_mul((r12) as i64) as u64) as u64; rbp = ((rbp).wrapping_add(rbx)) as u64; rbp = ((rbp).wrapping_add(crate::simd_ops::ru64(rsp.wrapping_add(0xb8u64)))) as u64; r15 = ((r14.wrapping_add(r8)) as u64) as u64; r15 = ((r15).rotate_right(33u32)) as u64; r15 = (((r15) as i64).wrapping_mul((r12) as i64) as u64) as u64; rcx = ((rsi.wrapping_add(r8)) as u64) as u64; rdx = (((rdx) as i64).wrapping_mul((r12) as i64) as u64) as u64; r9 = (crate::simd_ops::ru64(rax.wrapping_sub(0x18u64))) as u64; r10 = (crate::simd_ops::ru64(rax.wrapping_sub(0x20u64))) as u64; r8 = (crate::simd_ops::ru64(rax.wrapping_sub(0x30u64))) as u64; r8 = ((r8).wrapping_add(rdx)) as u64; rdx = ((r9.wrapping_add(r8)) as u64) as u64; rdx = ((rdx).wrapping_add(rcx)) as u64; rcx = r8; rax = ((r10.wrapping_add(r11)) as u64) as u64; r8 = ((r8).wrapping_add(rax)) as u64; rdx = ((rdx).rotate_right(21u32)) as u64; r11 = r8; r11 = ((r11).rotate_right(44u32)) as u64; r11 = ((r11).wrapping_add(rdx)) as u64; r11 = ((r11).wrapping_add(rcx)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xc0u64), r11); rax = ((r9.wrapping_add(r8)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xb8u64), rax); rdx = ((r10.wrapping_add(rbp)) as u64) as u64; r9 = ((r15.wrapping_add(r13)) as u64) as u64; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xb0u64))) as u64; r10 = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; r9 = ((r9).wrapping_add(crate::simd_ops::ru64(rax.wrapping_sub(0x10u64)))) as u64; r8 = ((r10.wrapping_add(r9)) as u64) as u64; r8 = ((r8).wrapping_add(rdx)) as u64; r13 = r9; rax = ((rdi.wrapping_add(rbx)) as u64) as u64; r9 = ((r9).wrapping_add(rax)) as u64; r8 = ((r8).rotate_right(21u32)) as u64; rax = r9; rax = ((rax).rotate_right(44u32)) as u64; r8 = ((r8).wrapping_add(rax)) as u64; r13 = ((r13).wrapping_add(r8)) as u64; r8 = ((r10.wrapping_add(r9)) as u64) as u64; r14 = rsi; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xb0u64))) as u64; rax = ((rax).wrapping_add(0x40u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xb0u64), rax); fl_l = (crate::simd_ops::ru64(rsp.wrapping_add(0xc8u64))) as u64; fl_r = (0x40u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xc8u64), ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64); if (((fl_l as u64)) != ((fl_r as u64))) { bb = 9; continue; } rcx = r13; rcx = (((rcx) ^ (r11))) as u64; r10 = (0x9ddfea08eb382d69u64) as u64; rcx = (((rcx) as i64).wrapping_mul((r10) as i64) as u64) as u64; rax = rcx; rax = ((rax).wrapping_shr(47u32)) as u64; rcx = (((rcx) ^ (rax))) as u64; r13 = (((r13) ^ (rcx))) as u64; r13 = (((r13) as i64).wrapping_mul((r10) as i64) as u64) as u64; rcx = r13; rcx = ((rcx).wrapping_shr(47u32)) as u64; rcx = (((rcx) ^ (r13))) as u64; rcx = (((rcx) as i64).wrapping_mul((r10) as i64) as u64) as u64; rcx = ((rcx).wrapping_add(r15)) as u64; rdx = r8; rdx = (((rdx) ^ (crate::simd_ops::ru64(rsp.wrapping_add(0xb8u64))))) as u64; rdx = (((rdx) as i64).wrapping_mul((r10) as i64) as u64) as u64; rax = rdx; rax = ((rax).wrapping_shr(47u32)) as u64; rdx = (((rdx) ^ (rax))) as u64; r8 = (((r8) ^ (rdx))) as u64; r8 = (((r8) as i64).wrapping_mul((r10) as i64) as u64) as u64; rax = r8; rax = ((rax).wrapping_shr(47u32)) as u64; rax = (((rax) ^ (r8))) as u64; rax = (((rax) as i64).wrapping_mul((r10) as i64) as u64) as u64; rdx = rbp; rdx = ((rdx).wrapping_shr(47u32)) as u64; rdx = (((rdx) ^ (rbp))) as u64; rdx = (((rdx) as i64).wrapping_mul((r12) as i64) as u64) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; rdx = ((rdx).wrapping_add(rsi)) as u64; rdx = (((rdx) ^ (rcx))) as u64; rdx = (((rdx) as i64).wrapping_mul((r10) as i64) as u64) as u64; rax = rdx; rax = ((rax).wrapping_shr(47u32)) as u64; rdx = (((rdx) ^ (rax))) as u64; rcx = (((rcx) ^ (rdx))) as u64; rcx = (((rcx) as i64).wrapping_mul((r10) as i64) as u64) as u64; rax = rcx; rax = ((rax).wrapping_shr(47u32)) as u64; rcx = (((rcx) ^ (rax))) as u64; rcx = (((rcx) as i64).wrapping_mul((r10) as i64) as u64) as u64; bb = 10; continue; }
            10 => { rax = (0xde5fb9d2630458e9u64) as u64; let prod = (rax as u128).wrapping_mul((rcx) as u128); rax = prod as u64; rdx = (prod >> 64) as u64; r9 = ((rdx.wrapping_add(rax)) as u64) as u64; r8 = rsp.wrapping_add(0x30); rdx = rsp.wrapping_add(0x20); rcx = P2; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; fl_l = (crate::simd_ops::ru8(rsp.wrapping_add(0x28u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 11; continue; } rbx = ((rdi.wrapping_add(rdi.wrapping_mul(2))) as u64) as u64; rbx = ((rbx).wrapping_shl(4u32)) as u64; rbx = ((rbx).wrapping_add(crate::simd_ops::ru64(P4))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xb0u64), rbx); crate::simd_ops::lk_and_u32(rbx, (0xff000004u32) as u32); crate::simd_ops::lk_or_u32(rbx, (0x4u32) as u32); crate::simd_ops::wu8(rbx.wrapping_add(0x3u64), (0x4u8) as u8); crate::simd_ops::wu32(rbx.wrapping_add(0x4u64), (0x0u32) as u32); crate::simd_ops::wu8(rbx.wrapping_add(0x8u64), (0x0u8) as u8); rdx = rsp.wrapping_add(0x30); rcx = rbx; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx.wrapping_add(0x28u64), (0x0u64) as u64); bb = 11; continue; }
            11 => { rax = ((rdi.wrapping_add(rdi.wrapping_mul(2))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rax = ((rax).wrapping_add(crate::simd_ops::ru64(P4))) as u64; rcx = P6; crate::simd_ops::wu64(rax.wrapping_add(0x28u64), rcx); rcx = rsp.wrapping_add(0x30); rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); rax = 0; fl_l = 0; fl_r = 0; rsp = ((rsp).wrapping_add(0x68u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_36643<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 21>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x68u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (0x4000004u64) as u64); crate::simd_ops::wu8(rsp.wrapping_add(0x38u64), (0x0u8) as u8); rax = P0; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (0xau64) as u64); r9 = 0xa; r8 = 0; fl_l = 0; fl_r = 0; rdx = rsp.wrapping_add(0x20); rcx = rsp.wrapping_add(0x30); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru32(rsp.wrapping_add(0x34u64))) as u32 as u64; rcx = (crate::simd_ops::ru32(rsp.wrapping_add(0x30u64))) as u32 as u64; rax = ((rcx as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_shr(24u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x80; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 1; continue; } rax = ((rax.wrapping_mul(8)) as u32) as u32 as u64; bb = 2; continue; }
            1 => { rax = (((rax as u32)).wrapping_shl(8u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x7c00; rax = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 2; continue; }
            2 => { rcx = ((((rcx as u32)) & (0xffffffu32))) as u32 as u64; rcx = (((rcx as u32)).wrapping_shl(3u32)) as u32 as u64; r15 = rsp.wrapping_add(0x38); fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; r15 = (if (((fl_l as u32)) > ((fl_r as u32))) { crate::simd_ops::ru64(rsp.wrapping_add(0x38u64)) } else { r15 }) as u64; fl_l = rbx; fl_r = (0x20u64) as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 7; continue; } fl_l = rbx; fl_r = (0x10u64) as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 6; continue; } fl_l = rbx; fl_r = (0x8u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 3; continue; } r8 = (0x9ae16a3b2f90404fu64) as u64; r9 = ((r8.wrapping_add(rbx.wrapping_mul(2))) as u64) as u64; r8 = (crate::simd_ops::ru64(r15)) as u64; rax = (0x651e95c4d06fbfb1u64) as u64; r8 = ((r8).wrapping_sub(rax)) as u64; rdx = (crate::simd_ops::ru64(rbx.wrapping_add(r15).wrapping_sub(0x8u64))) as u64; rdx = ((rdx).rotate_right(37u32)) as u64; rdx = (((rdx) as i64).wrapping_mul((r9) as i64) as u64) as u64; rdx = ((rdx).wrapping_add(r8)) as u64; r8 = ((r8).rotate_right(25u32)) as u64; r8 = ((r8).wrapping_add(crate::simd_ops::ru64(rbx.wrapping_add(r15).wrapping_sub(0x8u64)))) as u64; r8 = (((r8) as i64).wrapping_mul((r9) as i64) as u64) as u64; rcx = r8; rcx = (((rcx) ^ (rdx))) as u64; rcx = (((rcx) as i64).wrapping_mul((r9) as i64) as u64) as u64; rax = rcx; rax = ((rax).wrapping_shr(47u32)) as u64; rcx = (((rcx) ^ (rax))) as u64; rcx = (((rcx) ^ (r8))) as u64; rcx = (((rcx) as i64).wrapping_mul((r9) as i64) as u64) as u64; rax = rcx; rax = ((rax).wrapping_shr(47u32)) as u64; rcx = (((rcx) ^ (rax))) as u64; rcx = (((rcx) as i64).wrapping_mul((r9) as i64) as u64) as u64; bb = 10; continue; }
            3 => { fl_l = rbx; fl_r = (0x4u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 4; continue; } r8 = (0x9ae16a3b2f90404fu64) as u64; r8 = ((r8.wrapping_add(rbx.wrapping_mul(2))) as u64) as u64; rax = (crate::simd_ops::ru32(r15)) as u32 as u64; rdx = (crate::simd_ops::ru32(rbx.wrapping_add(r15).wrapping_sub(0x4u64))) as u32 as u64; rcx = ((rbx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rcx = (((rcx) ^ (rdx))) as u64; rcx = (((rcx) as i64).wrapping_mul((r8) as i64) as u64) as u64; rax = rcx; rax = ((rax).wrapping_shr(47u32)) as u64; rcx = (((rcx) ^ (rax))) as u64; rcx = (((rcx) ^ (rdx))) as u64; rcx = (((rcx) as i64).wrapping_mul((r8) as i64) as u64) as u64; rax = rcx; rax = ((rax).wrapping_shr(47u32)) as u64; rcx = (((rcx) ^ (rax))) as u64; rcx = (((rcx) as i64).wrapping_mul((r8) as i64) as u64) as u64; bb = 10; continue; }
            4 => { fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } rax = rbx; rax = ((rax).wrapping_shr(1u32)) as u64; rdx = ((crate::simd_ops::ru8(rax.wrapping_add(r15))) as u32) as u32 as u64; rax = (0xe16a3b2f90404f00u64) as u64; rdx = (((rdx) as i64).wrapping_mul((rax) as i64) as u64) as u64; rax = ((crate::simd_ops::ru8(r15)) as u32) as u32 as u64; r8 = (0x9ae16a3b2f90404fu64) as u64; rax = (((rax) as i64).wrapping_mul((r8) as i64) as u64) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; rax = ((crate::simd_ops::ru8(rbx.wrapping_add(r15).wrapping_sub(0x1u64))) as u32) as u32 as u64; rax = ((rbx.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = ((rax as u32)) as u32 as u64; rcx = (0xc3a5c85c97cb3127u64) as u64; rax = (((rax) as i64).wrapping_mul((rcx) as i64) as u64) as u64; rdx = (((rdx) ^ (rax))) as u64; rcx = rdx; rcx = ((rcx).wrapping_shr(47u32)) as u64; rcx = (((rcx) ^ (rdx))) as u64; rcx = (((rcx) as i64).wrapping_mul((r8) as i64) as u64) as u64; bb = 10; continue; }
            5 => { rcx = (0x9ae16a3b2f90404fu64) as u64; bb = 10; continue; }
            6 => { r8 = (0x9ae16a3b2f90404fu64) as u64; r11 = ((r8.wrapping_add(rbx.wrapping_mul(2))) as u64) as u64; r12 = (0xb492b66fbe98f273u64) as u64; r9 = (crate::simd_ops::ru64(r15)) as u64; r9 = (((r9) as i64).wrapping_mul((r12) as i64) as u64) as u64; rdx = (crate::simd_ops::ru64(r15.wrapping_add(0x8u64))) as u64; r10 = r11; r10 = (((r10) as i64).wrapping_mul((crate::simd_ops::ru64(rbx.wrapping_add(r15).wrapping_sub(0x8u64))) as i64) as u64) as u64; rcx = (crate::simd_ops::ru64(rbx.wrapping_add(r15).wrapping_sub(0x10u64))) as u64; rcx = (((rcx) as i64).wrapping_mul((r8) as i64) as u64) as u64; r8 = ((r8).wrapping_add(rdx)) as u64; r8 = ((r8).rotate_right(18u32)) as u64; r8 = ((r8).wrapping_add(r10)) as u64; r8 = ((r8).wrapping_add(r9)) as u64; rax = ((rdx.wrapping_add(r9)) as u64) as u64; rax = ((rax).rotate_right(43u32)) as u64; r10 = ((r10).rotate_right(30u32)) as u64; rcx = ((rcx).wrapping_add(rax)) as u64; rcx = ((rcx).wrapping_add(r10)) as u64; rcx = (((rcx) ^ (r8))) as u64; rcx = (((rcx) as i64).wrapping_mul((r11) as i64) as u64) as u64; rax = rcx; rax = ((rax).wrapping_shr(47u32)) as u64; rcx = (((rcx) ^ (rax))) as u64; rcx = (((rcx) ^ (r8))) as u64; rcx = (((rcx) as i64).wrapping_mul((r11) as i64) as u64) as u64; rax = rcx; rax = ((rax).wrapping_shr(47u32)) as u64; rcx = (((rcx) ^ (rax))) as u64; rcx = (((rcx) as i64).wrapping_mul((r11) as i64) as u64) as u64; bb = 10; continue; }
            7 => { r13 = ((rbx.wrapping_add(r15)) as u64) as u64; fl_l = rbx; fl_r = (0x40u64) as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 8; continue; } r8 = (0x9ae16a3b2f90404fu64) as u64; r14 = ((r8.wrapping_add(rbx.wrapping_mul(2))) as u64) as u64; r11 = (crate::simd_ops::ru64(r13.wrapping_sub(0x18u64))) as u64; r10 = (crate::simd_ops::ru64(r15.wrapping_add(0x10u64))) as u64; r10 = (((r10) as i64).wrapping_mul((r8) as i64) as u64) as u64; rax = (crate::simd_ops::ru64(r15.wrapping_add(0x18u64))) as u64; r9 = ((rax.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rdi = r14; rdi = (((rdi) as i64).wrapping_mul((crate::simd_ops::ru64(r13.wrapping_sub(0x10u64))) as i64) as u64) as u64; rdx = (crate::simd_ops::ru64(r15)) as u64; rdx = (((rdx) as i64).wrapping_mul((r8) as i64) as u64) as u64; rdx = ((rdx).wrapping_add(crate::simd_ops::ru64(r13.wrapping_sub(0x8u64)))) as u64; rax = (crate::simd_ops::ru64(r15.wrapping_add(0x8u64))) as u64; rax = ((rax).rotate_right(30u32)) as u64; rax = ((rax).wrapping_add(r11)) as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rax = rdx; rax = ((rax).rotate_right(43u32)) as u64; rcx = ((rcx).wrapping_add(rax)) as u64; r8 = (crate::simd_ops::ru64(r13.wrapping_sub(0x20u64))) as u64; r8 = (((r8) ^ (rdx))) as u64; r8 = ((r8).wrapping_add(1)) as u64; r8 = ((r8).wrapping_add(r9)) as u64; rdx = ((r8.wrapping_add(rcx)) as u64) as u64; rdx = (((rdx) as i64).wrapping_mul((r14) as i64) as u64) as u64; rdx = ((rdx).swap_bytes()) as u64; rdx = ((rdx).wrapping_add(rdi)) as u64; rax = ((r9.wrapping_add(r10)) as u64) as u64; r9 = rax; r9 = ((r9).rotate_right(42u32)) as u64; r9 = ((r9).wrapping_add(r11)) as u64; rcx = ((rax.wrapping_add(r11)) as u64) as u64; rax = ((rdx.wrapping_add(r8)) as u64) as u64; rax = (((rax) as i64).wrapping_mul((r14) as i64) as u64) as u64; rax = ((rax).swap_bytes()) as u64; rax = ((rax).wrapping_add(rcx)) as u64; rax = ((rax).wrapping_add(r9)) as u64; rax = ((rax).wrapping_add(crate::simd_ops::ru64(r13.wrapping_sub(0x8u64)))) as u64; rax = (((rax) as i64).wrapping_mul((r14) as i64) as u64) as u64; rax = ((rax).swap_bytes()) as u64; rax = ((rax).wrapping_add(crate::simd_ops::ru64(r15.wrapping_add(0x8u64)))) as u64; rcx = ((rcx).wrapping_add(rax)) as u64; rcx = (((rcx) as i64).wrapping_mul((r14) as i64) as u64) as u64; rcx = ((rcx).wrapping_add(rdi)) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(r13.wrapping_sub(0x20u64)))) as u64; rax = rcx; rax = ((rax).wrapping_shr(47u32)) as u64; rcx = (((rcx) ^ (rax))) as u64; rcx = (((rcx) as i64).wrapping_mul((r14) as i64) as u64) as u64; fl_l = ((rcx).wrapping_add(r9)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; bb = 10; continue; }
            8 => { rdi = (crate::simd_ops::ru64(r13.wrapping_sub(0x28u64))) as u64; rbp = (crate::simd_ops::ru64(r13.wrapping_sub(0x38u64))) as u64; rbp = ((rbp).wrapping_add(crate::simd_ops::ru64(r13.wrapping_sub(0x10u64)))) as u64; r14 = (crate::simd_ops::ru64(r13.wrapping_sub(0x30u64))) as u64; r14 = ((r14).wrapping_add(rbx)) as u64; r14 = (((r14) ^ (crate::simd_ops::ru64(r13.wrapping_sub(0x18u64))))) as u64; rdx = (0x9ddfea08eb382d69u64) as u64; r14 = (((r14) as i64).wrapping_mul((rdx) as i64) as u64) as u64; rax = r14; rax = ((rax).wrapping_shr(47u32)) as u64; r14 = (((r14) ^ (rax))) as u64; r14 = (((r14) ^ (crate::simd_ops::ru64(r13.wrapping_sub(0x18u64))))) as u64; r14 = (((r14) as i64).wrapping_mul((rdx) as i64) as u64) as u64; rax = r14; rax = ((rax).wrapping_shr(47u32)) as u64; r14 = (((r14) ^ (rax))) as u64; r14 = (((r14) as i64).wrapping_mul((rdx) as i64) as u64) as u64; rax = (crate::simd_ops::ru64(r13.wrapping_sub(0x30u64))) as u64; r10 = (crate::simd_ops::ru64(r13.wrapping_sub(0x40u64))) as u64; r10 = ((r10).wrapping_add(rbx)) as u64; r9 = ((rdi.wrapping_add(r10)) as u64) as u64; r9 = ((r9).wrapping_add(r14)) as u64; r8 = r10; rax = ((rax).wrapping_add(crate::simd_ops::ru64(r13.wrapping_sub(0x38u64)))) as u64; r10 = ((r10).wrapping_add(rax)) as u64; r9 = ((r9).rotate_right(21u32)) as u64; rax = r10; rax = ((rax).rotate_right(44u32)) as u64; rax = ((rax).wrapping_add(r9)) as u64; rax = ((rax).wrapping_add(r8)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xc0u64), rax); r9 = ((rdi.wrapping_add(r10)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xb8u64), r9); r11 = (crate::simd_ops::ru64(r13.wrapping_sub(0x8u64))) as u64; rax = (crate::simd_ops::ru64(r13.wrapping_sub(0x10u64))) as u64; rcx = (crate::simd_ops::ru64(r13.wrapping_sub(0x18u64))) as u64; r10 = (crate::simd_ops::ru64(r13.wrapping_sub(0x20u64))) as u64; r12 = (0xb492b66fbe98f273u64) as u64; r10 = ((r10).wrapping_add(r12)) as u64; r10 = ((r10).wrapping_add(rbp)) as u64; r9 = ((r11.wrapping_add(r10)) as u64) as u64; r9 = ((r9).wrapping_add(rdi)) as u64; r13 = r10; rax = ((rax).wrapping_add(rcx)) as u64; r10 = ((r10).wrapping_add(rax)) as u64; r9 = ((r9).rotate_right(21u32)) as u64; rax = r10; rax = ((rax).rotate_right(44u32)) as u64; r9 = ((r9).wrapping_add(rax)) as u64; r13 = ((r13).wrapping_add(r9)) as u64; r8 = ((r11.wrapping_add(r10)) as u64) as u64; rdi = (((rdi) as i64).wrapping_mul((r12) as i64) as u64) as u64; rdi = ((rdi).wrapping_add(crate::simd_ops::ru64(r15))) as u64; rbx = ((rbx).wrapping_sub(1)) as u64; fl_l = (((rbx) & (0xffffffffffffffc0u64))) as u64; fl_r = fl_l; rbx = ((fl_l as u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xc8u64), rbx); rax = r15.wrapping_add(0x30); crate::simd_ops::wu64(rsp.wrapping_add(0xb0u64), rax); r15 = rdi; bb = 9; continue; }
            9 => { r11 = (crate::simd_ops::ru64(rax.wrapping_sub(0x28u64))) as u64; rsi = ((r11.wrapping_add(rbp)) as u64) as u64; rsi = ((rsi).wrapping_add(r15)) as u64; rsi = ((rsi).wrapping_add(crate::simd_ops::ru64(rsp.wrapping_add(0xb8u64)))) as u64; rdi = (crate::simd_ops::ru64(rax)) as u64; rbp = ((rbp).wrapping_add(rdi)) as u64; rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0xc0u64))) as u64; rbp = ((rbp).wrapping_add(rdx)) as u64; rsi = ((rsi).rotate_right(37u32)) as u64; rsi = (((rsi) as i64).wrapping_mul((r12) as i64) as u64) as u64; rsi = (((rsi) ^ (r13))) as u64; rbx = (crate::simd_ops::ru64(rax.wrapping_sub(0x8u64))) as u64; rbp = ((rbp).rotate_right(42u32)) as u64; rbp = (((rbp) as i64).wrapping_mul((r12) as i64) as u64) as u64; rbp = ((rbp).wrapping_add(rbx)) as u64; rbp = ((rbp).wrapping_add(crate::simd_ops::ru64(rsp.wrapping_add(0xb8u64)))) as u64; r15 = ((r14.wrapping_add(r8)) as u64) as u64; r15 = ((r15).rotate_right(33u32)) as u64; r15 = (((r15) as i64).wrapping_mul((r12) as i64) as u64) as u64; rcx = ((rsi.wrapping_add(r8)) as u64) as u64; rdx = (((rdx) as i64).wrapping_mul((r12) as i64) as u64) as u64; r9 = (crate::simd_ops::ru64(rax.wrapping_sub(0x18u64))) as u64; r10 = (crate::simd_ops::ru64(rax.wrapping_sub(0x20u64))) as u64; r8 = (crate::simd_ops::ru64(rax.wrapping_sub(0x30u64))) as u64; r8 = ((r8).wrapping_add(rdx)) as u64; rdx = ((r9.wrapping_add(r8)) as u64) as u64; rdx = ((rdx).wrapping_add(rcx)) as u64; rcx = r8; rax = ((r10.wrapping_add(r11)) as u64) as u64; r8 = ((r8).wrapping_add(rax)) as u64; rdx = ((rdx).rotate_right(21u32)) as u64; r11 = r8; r11 = ((r11).rotate_right(44u32)) as u64; r11 = ((r11).wrapping_add(rdx)) as u64; r11 = ((r11).wrapping_add(rcx)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xc0u64), r11); rax = ((r9.wrapping_add(r8)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xb8u64), rax); rdx = ((r10.wrapping_add(rbp)) as u64) as u64; r9 = ((r15.wrapping_add(r13)) as u64) as u64; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xb0u64))) as u64; r10 = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; r9 = ((r9).wrapping_add(crate::simd_ops::ru64(rax.wrapping_sub(0x10u64)))) as u64; r8 = ((r10.wrapping_add(r9)) as u64) as u64; r8 = ((r8).wrapping_add(rdx)) as u64; r13 = r9; rax = ((rdi.wrapping_add(rbx)) as u64) as u64; r9 = ((r9).wrapping_add(rax)) as u64; r8 = ((r8).rotate_right(21u32)) as u64; rax = r9; rax = ((rax).rotate_right(44u32)) as u64; r8 = ((r8).wrapping_add(rax)) as u64; r13 = ((r13).wrapping_add(r8)) as u64; r8 = ((r10.wrapping_add(r9)) as u64) as u64; r14 = rsi; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xb0u64))) as u64; rax = ((rax).wrapping_add(0x40u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xb0u64), rax); fl_l = (crate::simd_ops::ru64(rsp.wrapping_add(0xc8u64))) as u64; fl_r = (0x40u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xc8u64), ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64); if (((fl_l as u64)) != ((fl_r as u64))) { bb = 9; continue; } rcx = r13; rcx = (((rcx) ^ (r11))) as u64; r10 = (0x9ddfea08eb382d69u64) as u64; rcx = (((rcx) as i64).wrapping_mul((r10) as i64) as u64) as u64; rax = rcx; rax = ((rax).wrapping_shr(47u32)) as u64; rcx = (((rcx) ^ (rax))) as u64; r13 = (((r13) ^ (rcx))) as u64; r13 = (((r13) as i64).wrapping_mul((r10) as i64) as u64) as u64; rcx = r13; rcx = ((rcx).wrapping_shr(47u32)) as u64; rcx = (((rcx) ^ (r13))) as u64; rcx = (((rcx) as i64).wrapping_mul((r10) as i64) as u64) as u64; rcx = ((rcx).wrapping_add(r15)) as u64; rdx = r8; rdx = (((rdx) ^ (crate::simd_ops::ru64(rsp.wrapping_add(0xb8u64))))) as u64; rdx = (((rdx) as i64).wrapping_mul((r10) as i64) as u64) as u64; rax = rdx; rax = ((rax).wrapping_shr(47u32)) as u64; rdx = (((rdx) ^ (rax))) as u64; r8 = (((r8) ^ (rdx))) as u64; r8 = (((r8) as i64).wrapping_mul((r10) as i64) as u64) as u64; rax = r8; rax = ((rax).wrapping_shr(47u32)) as u64; rax = (((rax) ^ (r8))) as u64; rax = (((rax) as i64).wrapping_mul((r10) as i64) as u64) as u64; rdx = rbp; rdx = ((rdx).wrapping_shr(47u32)) as u64; rdx = (((rdx) ^ (rbp))) as u64; rdx = (((rdx) as i64).wrapping_mul((r12) as i64) as u64) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; rdx = ((rdx).wrapping_add(rsi)) as u64; rdx = (((rdx) ^ (rcx))) as u64; rdx = (((rdx) as i64).wrapping_mul((r10) as i64) as u64) as u64; rax = rdx; rax = ((rax).wrapping_shr(47u32)) as u64; rdx = (((rdx) ^ (rax))) as u64; rcx = (((rcx) ^ (rdx))) as u64; rcx = (((rcx) as i64).wrapping_mul((r10) as i64) as u64) as u64; rax = rcx; rax = ((rax).wrapping_shr(47u32)) as u64; rcx = (((rcx) ^ (rax))) as u64; rcx = (((rcx) as i64).wrapping_mul((r10) as i64) as u64) as u64; bb = 10; continue; }
            10 => { rax = (0xde5fb9d2630458e9u64) as u64; let prod = (rax as u128).wrapping_mul((rcx) as u128); rax = prod as u64; rdx = (prod >> 64) as u64; r9 = ((rdx.wrapping_add(rax)) as u64) as u64; r8 = rsp.wrapping_add(0x30); rdx = rsp.wrapping_add(0x20); rcx = P2; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; fl_l = (crate::simd_ops::ru8(rsp.wrapping_add(0x28u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 11; continue; } rbx = ((rdi.wrapping_add(rdi.wrapping_mul(2))) as u64) as u64; rbx = ((rbx).wrapping_shl(4u32)) as u64; rbx = ((rbx).wrapping_add(crate::simd_ops::ru64(P4))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xb0u64), rbx); crate::simd_ops::lk_and_u32(rbx, (0xff000004u32) as u32); crate::simd_ops::lk_or_u32(rbx, (0x4u32) as u32); crate::simd_ops::wu8(rbx.wrapping_add(0x3u64), (0x4u8) as u8); crate::simd_ops::wu32(rbx.wrapping_add(0x4u64), (0x0u32) as u32); crate::simd_ops::wu8(rbx.wrapping_add(0x8u64), (0x0u8) as u8); rdx = rsp.wrapping_add(0x30); rcx = rbx; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx.wrapping_add(0x28u64), (0x0u64) as u64); bb = 11; continue; }
            11 => { rax = ((rdi.wrapping_add(rdi.wrapping_mul(2))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rax = ((rax).wrapping_add(crate::simd_ops::ru64(P4))) as u64; rcx = P6; crate::simd_ops::wu64(rax.wrapping_add(0x28u64), rcx); rcx = rsp.wrapping_add(0x30); rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); rax = 0; fl_l = 0; fl_r = 0; rsp = ((rsp).wrapping_add(0x68u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_36644<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 21>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x68u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (0x4000004u64) as u64); crate::simd_ops::wu8(rsp.wrapping_add(0x38u64), (0x0u8) as u8); rax = P0; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (0x1cu64) as u64); r9 = 0x1c; r8 = 0; fl_l = 0; fl_r = 0; rdx = rsp.wrapping_add(0x20); rcx = rsp.wrapping_add(0x30); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru32(rsp.wrapping_add(0x34u64))) as u32 as u64; rcx = (crate::simd_ops::ru32(rsp.wrapping_add(0x30u64))) as u32 as u64; rax = ((rcx as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_shr(24u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x80; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 1; continue; } rax = ((rax.wrapping_mul(8)) as u32) as u32 as u64; bb = 2; continue; }
            1 => { rax = (((rax as u32)).wrapping_shl(8u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x7c00; rax = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 2; continue; }
            2 => { rcx = ((((rcx as u32)) & (0xffffffu32))) as u32 as u64; rcx = (((rcx as u32)).wrapping_shl(3u32)) as u32 as u64; r15 = rsp.wrapping_add(0x38); fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; r15 = (if (((fl_l as u32)) > ((fl_r as u32))) { crate::simd_ops::ru64(rsp.wrapping_add(0x38u64)) } else { r15 }) as u64; fl_l = rbx; fl_r = (0x20u64) as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 7; continue; } fl_l = rbx; fl_r = (0x10u64) as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 6; continue; } fl_l = rbx; fl_r = (0x8u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 3; continue; } r8 = (0x9ae16a3b2f90404fu64) as u64; r9 = ((r8.wrapping_add(rbx.wrapping_mul(2))) as u64) as u64; r8 = (crate::simd_ops::ru64(r15)) as u64; rax = (0x651e95c4d06fbfb1u64) as u64; r8 = ((r8).wrapping_sub(rax)) as u64; rdx = (crate::simd_ops::ru64(rbx.wrapping_add(r15).wrapping_sub(0x8u64))) as u64; rdx = ((rdx).rotate_right(37u32)) as u64; rdx = (((rdx) as i64).wrapping_mul((r9) as i64) as u64) as u64; rdx = ((rdx).wrapping_add(r8)) as u64; r8 = ((r8).rotate_right(25u32)) as u64; r8 = ((r8).wrapping_add(crate::simd_ops::ru64(rbx.wrapping_add(r15).wrapping_sub(0x8u64)))) as u64; r8 = (((r8) as i64).wrapping_mul((r9) as i64) as u64) as u64; rcx = r8; rcx = (((rcx) ^ (rdx))) as u64; rcx = (((rcx) as i64).wrapping_mul((r9) as i64) as u64) as u64; rax = rcx; rax = ((rax).wrapping_shr(47u32)) as u64; rcx = (((rcx) ^ (rax))) as u64; rcx = (((rcx) ^ (r8))) as u64; rcx = (((rcx) as i64).wrapping_mul((r9) as i64) as u64) as u64; rax = rcx; rax = ((rax).wrapping_shr(47u32)) as u64; rcx = (((rcx) ^ (rax))) as u64; rcx = (((rcx) as i64).wrapping_mul((r9) as i64) as u64) as u64; bb = 10; continue; }
            3 => { fl_l = rbx; fl_r = (0x4u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 4; continue; } r8 = (0x9ae16a3b2f90404fu64) as u64; r8 = ((r8.wrapping_add(rbx.wrapping_mul(2))) as u64) as u64; rax = (crate::simd_ops::ru32(r15)) as u32 as u64; rdx = (crate::simd_ops::ru32(rbx.wrapping_add(r15).wrapping_sub(0x4u64))) as u32 as u64; rcx = ((rbx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rcx = (((rcx) ^ (rdx))) as u64; rcx = (((rcx) as i64).wrapping_mul((r8) as i64) as u64) as u64; rax = rcx; rax = ((rax).wrapping_shr(47u32)) as u64; rcx = (((rcx) ^ (rax))) as u64; rcx = (((rcx) ^ (rdx))) as u64; rcx = (((rcx) as i64).wrapping_mul((r8) as i64) as u64) as u64; rax = rcx; rax = ((rax).wrapping_shr(47u32)) as u64; rcx = (((rcx) ^ (rax))) as u64; rcx = (((rcx) as i64).wrapping_mul((r8) as i64) as u64) as u64; bb = 10; continue; }
            4 => { fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } rax = rbx; rax = ((rax).wrapping_shr(1u32)) as u64; rdx = ((crate::simd_ops::ru8(rax.wrapping_add(r15))) as u32) as u32 as u64; rax = (0xe16a3b2f90404f00u64) as u64; rdx = (((rdx) as i64).wrapping_mul((rax) as i64) as u64) as u64; rax = ((crate::simd_ops::ru8(r15)) as u32) as u32 as u64; r8 = (0x9ae16a3b2f90404fu64) as u64; rax = (((rax) as i64).wrapping_mul((r8) as i64) as u64) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; rax = ((crate::simd_ops::ru8(rbx.wrapping_add(r15).wrapping_sub(0x1u64))) as u32) as u32 as u64; rax = ((rbx.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = ((rax as u32)) as u32 as u64; rcx = (0xc3a5c85c97cb3127u64) as u64; rax = (((rax) as i64).wrapping_mul((rcx) as i64) as u64) as u64; rdx = (((rdx) ^ (rax))) as u64; rcx = rdx; rcx = ((rcx).wrapping_shr(47u32)) as u64; rcx = (((rcx) ^ (rdx))) as u64; rcx = (((rcx) as i64).wrapping_mul((r8) as i64) as u64) as u64; bb = 10; continue; }
            5 => { rcx = (0x9ae16a3b2f90404fu64) as u64; bb = 10; continue; }
            6 => { r8 = (0x9ae16a3b2f90404fu64) as u64; r11 = ((r8.wrapping_add(rbx.wrapping_mul(2))) as u64) as u64; r12 = (0xb492b66fbe98f273u64) as u64; r9 = (crate::simd_ops::ru64(r15)) as u64; r9 = (((r9) as i64).wrapping_mul((r12) as i64) as u64) as u64; rdx = (crate::simd_ops::ru64(r15.wrapping_add(0x8u64))) as u64; r10 = r11; r10 = (((r10) as i64).wrapping_mul((crate::simd_ops::ru64(rbx.wrapping_add(r15).wrapping_sub(0x8u64))) as i64) as u64) as u64; rcx = (crate::simd_ops::ru64(rbx.wrapping_add(r15).wrapping_sub(0x10u64))) as u64; rcx = (((rcx) as i64).wrapping_mul((r8) as i64) as u64) as u64; r8 = ((r8).wrapping_add(rdx)) as u64; r8 = ((r8).rotate_right(18u32)) as u64; r8 = ((r8).wrapping_add(r10)) as u64; r8 = ((r8).wrapping_add(r9)) as u64; rax = ((rdx.wrapping_add(r9)) as u64) as u64; rax = ((rax).rotate_right(43u32)) as u64; r10 = ((r10).rotate_right(30u32)) as u64; rcx = ((rcx).wrapping_add(rax)) as u64; rcx = ((rcx).wrapping_add(r10)) as u64; rcx = (((rcx) ^ (r8))) as u64; rcx = (((rcx) as i64).wrapping_mul((r11) as i64) as u64) as u64; rax = rcx; rax = ((rax).wrapping_shr(47u32)) as u64; rcx = (((rcx) ^ (rax))) as u64; rcx = (((rcx) ^ (r8))) as u64; rcx = (((rcx) as i64).wrapping_mul((r11) as i64) as u64) as u64; rax = rcx; rax = ((rax).wrapping_shr(47u32)) as u64; rcx = (((rcx) ^ (rax))) as u64; rcx = (((rcx) as i64).wrapping_mul((r11) as i64) as u64) as u64; bb = 10; continue; }
            7 => { r13 = ((rbx.wrapping_add(r15)) as u64) as u64; fl_l = rbx; fl_r = (0x40u64) as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 8; continue; } r8 = (0x9ae16a3b2f90404fu64) as u64; r14 = ((r8.wrapping_add(rbx.wrapping_mul(2))) as u64) as u64; r11 = (crate::simd_ops::ru64(r13.wrapping_sub(0x18u64))) as u64; r10 = (crate::simd_ops::ru64(r15.wrapping_add(0x10u64))) as u64; r10 = (((r10) as i64).wrapping_mul((r8) as i64) as u64) as u64; rax = (crate::simd_ops::ru64(r15.wrapping_add(0x18u64))) as u64; r9 = ((rax.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rdi = r14; rdi = (((rdi) as i64).wrapping_mul((crate::simd_ops::ru64(r13.wrapping_sub(0x10u64))) as i64) as u64) as u64; rdx = (crate::simd_ops::ru64(r15)) as u64; rdx = (((rdx) as i64).wrapping_mul((r8) as i64) as u64) as u64; rdx = ((rdx).wrapping_add(crate::simd_ops::ru64(r13.wrapping_sub(0x8u64)))) as u64; rax = (crate::simd_ops::ru64(r15.wrapping_add(0x8u64))) as u64; rax = ((rax).rotate_right(30u32)) as u64; rax = ((rax).wrapping_add(r11)) as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rax = rdx; rax = ((rax).rotate_right(43u32)) as u64; rcx = ((rcx).wrapping_add(rax)) as u64; r8 = (crate::simd_ops::ru64(r13.wrapping_sub(0x20u64))) as u64; r8 = (((r8) ^ (rdx))) as u64; r8 = ((r8).wrapping_add(1)) as u64; r8 = ((r8).wrapping_add(r9)) as u64; rdx = ((r8.wrapping_add(rcx)) as u64) as u64; rdx = (((rdx) as i64).wrapping_mul((r14) as i64) as u64) as u64; rdx = ((rdx).swap_bytes()) as u64; rdx = ((rdx).wrapping_add(rdi)) as u64; rax = ((r9.wrapping_add(r10)) as u64) as u64; r9 = rax; r9 = ((r9).rotate_right(42u32)) as u64; r9 = ((r9).wrapping_add(r11)) as u64; rcx = ((rax.wrapping_add(r11)) as u64) as u64; rax = ((rdx.wrapping_add(r8)) as u64) as u64; rax = (((rax) as i64).wrapping_mul((r14) as i64) as u64) as u64; rax = ((rax).swap_bytes()) as u64; rax = ((rax).wrapping_add(rcx)) as u64; rax = ((rax).wrapping_add(r9)) as u64; rax = ((rax).wrapping_add(crate::simd_ops::ru64(r13.wrapping_sub(0x8u64)))) as u64; rax = (((rax) as i64).wrapping_mul((r14) as i64) as u64) as u64; rax = ((rax).swap_bytes()) as u64; rax = ((rax).wrapping_add(crate::simd_ops::ru64(r15.wrapping_add(0x8u64)))) as u64; rcx = ((rcx).wrapping_add(rax)) as u64; rcx = (((rcx) as i64).wrapping_mul((r14) as i64) as u64) as u64; rcx = ((rcx).wrapping_add(rdi)) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(r13.wrapping_sub(0x20u64)))) as u64; rax = rcx; rax = ((rax).wrapping_shr(47u32)) as u64; rcx = (((rcx) ^ (rax))) as u64; rcx = (((rcx) as i64).wrapping_mul((r14) as i64) as u64) as u64; fl_l = ((rcx).wrapping_add(r9)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; bb = 10; continue; }
            8 => { rdi = (crate::simd_ops::ru64(r13.wrapping_sub(0x28u64))) as u64; rbp = (crate::simd_ops::ru64(r13.wrapping_sub(0x38u64))) as u64; rbp = ((rbp).wrapping_add(crate::simd_ops::ru64(r13.wrapping_sub(0x10u64)))) as u64; r14 = (crate::simd_ops::ru64(r13.wrapping_sub(0x30u64))) as u64; r14 = ((r14).wrapping_add(rbx)) as u64; r14 = (((r14) ^ (crate::simd_ops::ru64(r13.wrapping_sub(0x18u64))))) as u64; rdx = (0x9ddfea08eb382d69u64) as u64; r14 = (((r14) as i64).wrapping_mul((rdx) as i64) as u64) as u64; rax = r14; rax = ((rax).wrapping_shr(47u32)) as u64; r14 = (((r14) ^ (rax))) as u64; r14 = (((r14) ^ (crate::simd_ops::ru64(r13.wrapping_sub(0x18u64))))) as u64; r14 = (((r14) as i64).wrapping_mul((rdx) as i64) as u64) as u64; rax = r14; rax = ((rax).wrapping_shr(47u32)) as u64; r14 = (((r14) ^ (rax))) as u64; r14 = (((r14) as i64).wrapping_mul((rdx) as i64) as u64) as u64; rax = (crate::simd_ops::ru64(r13.wrapping_sub(0x30u64))) as u64; r10 = (crate::simd_ops::ru64(r13.wrapping_sub(0x40u64))) as u64; r10 = ((r10).wrapping_add(rbx)) as u64; r9 = ((rdi.wrapping_add(r10)) as u64) as u64; r9 = ((r9).wrapping_add(r14)) as u64; r8 = r10; rax = ((rax).wrapping_add(crate::simd_ops::ru64(r13.wrapping_sub(0x38u64)))) as u64; r10 = ((r10).wrapping_add(rax)) as u64; r9 = ((r9).rotate_right(21u32)) as u64; rax = r10; rax = ((rax).rotate_right(44u32)) as u64; rax = ((rax).wrapping_add(r9)) as u64; rax = ((rax).wrapping_add(r8)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xc0u64), rax); r9 = ((rdi.wrapping_add(r10)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xb8u64), r9); r11 = (crate::simd_ops::ru64(r13.wrapping_sub(0x8u64))) as u64; rax = (crate::simd_ops::ru64(r13.wrapping_sub(0x10u64))) as u64; rcx = (crate::simd_ops::ru64(r13.wrapping_sub(0x18u64))) as u64; r10 = (crate::simd_ops::ru64(r13.wrapping_sub(0x20u64))) as u64; r12 = (0xb492b66fbe98f273u64) as u64; r10 = ((r10).wrapping_add(r12)) as u64; r10 = ((r10).wrapping_add(rbp)) as u64; r9 = ((r11.wrapping_add(r10)) as u64) as u64; r9 = ((r9).wrapping_add(rdi)) as u64; r13 = r10; rax = ((rax).wrapping_add(rcx)) as u64; r10 = ((r10).wrapping_add(rax)) as u64; r9 = ((r9).rotate_right(21u32)) as u64; rax = r10; rax = ((rax).rotate_right(44u32)) as u64; r9 = ((r9).wrapping_add(rax)) as u64; r13 = ((r13).wrapping_add(r9)) as u64; r8 = ((r11.wrapping_add(r10)) as u64) as u64; rdi = (((rdi) as i64).wrapping_mul((r12) as i64) as u64) as u64; rdi = ((rdi).wrapping_add(crate::simd_ops::ru64(r15))) as u64; rbx = ((rbx).wrapping_sub(1)) as u64; fl_l = (((rbx) & (0xffffffffffffffc0u64))) as u64; fl_r = fl_l; rbx = ((fl_l as u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xc8u64), rbx); rax = r15.wrapping_add(0x30); crate::simd_ops::wu64(rsp.wrapping_add(0xb0u64), rax); r15 = rdi; bb = 9; continue; }
            9 => { r11 = (crate::simd_ops::ru64(rax.wrapping_sub(0x28u64))) as u64; rsi = ((r11.wrapping_add(rbp)) as u64) as u64; rsi = ((rsi).wrapping_add(r15)) as u64; rsi = ((rsi).wrapping_add(crate::simd_ops::ru64(rsp.wrapping_add(0xb8u64)))) as u64; rdi = (crate::simd_ops::ru64(rax)) as u64; rbp = ((rbp).wrapping_add(rdi)) as u64; rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0xc0u64))) as u64; rbp = ((rbp).wrapping_add(rdx)) as u64; rsi = ((rsi).rotate_right(37u32)) as u64; rsi = (((rsi) as i64).wrapping_mul((r12) as i64) as u64) as u64; rsi = (((rsi) ^ (r13))) as u64; rbx = (crate::simd_ops::ru64(rax.wrapping_sub(0x8u64))) as u64; rbp = ((rbp).rotate_right(42u32)) as u64; rbp = (((rbp) as i64).wrapping_mul((r12) as i64) as u64) as u64; rbp = ((rbp).wrapping_add(rbx)) as u64; rbp = ((rbp).wrapping_add(crate::simd_ops::ru64(rsp.wrapping_add(0xb8u64)))) as u64; r15 = ((r14.wrapping_add(r8)) as u64) as u64; r15 = ((r15).rotate_right(33u32)) as u64; r15 = (((r15) as i64).wrapping_mul((r12) as i64) as u64) as u64; rcx = ((rsi.wrapping_add(r8)) as u64) as u64; rdx = (((rdx) as i64).wrapping_mul((r12) as i64) as u64) as u64; r9 = (crate::simd_ops::ru64(rax.wrapping_sub(0x18u64))) as u64; r10 = (crate::simd_ops::ru64(rax.wrapping_sub(0x20u64))) as u64; r8 = (crate::simd_ops::ru64(rax.wrapping_sub(0x30u64))) as u64; r8 = ((r8).wrapping_add(rdx)) as u64; rdx = ((r9.wrapping_add(r8)) as u64) as u64; rdx = ((rdx).wrapping_add(rcx)) as u64; rcx = r8; rax = ((r10.wrapping_add(r11)) as u64) as u64; r8 = ((r8).wrapping_add(rax)) as u64; rdx = ((rdx).rotate_right(21u32)) as u64; r11 = r8; r11 = ((r11).rotate_right(44u32)) as u64; r11 = ((r11).wrapping_add(rdx)) as u64; r11 = ((r11).wrapping_add(rcx)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xc0u64), r11); rax = ((r9.wrapping_add(r8)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xb8u64), rax); rdx = ((r10.wrapping_add(rbp)) as u64) as u64; r9 = ((r15.wrapping_add(r13)) as u64) as u64; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xb0u64))) as u64; r10 = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; r9 = ((r9).wrapping_add(crate::simd_ops::ru64(rax.wrapping_sub(0x10u64)))) as u64; r8 = ((r10.wrapping_add(r9)) as u64) as u64; r8 = ((r8).wrapping_add(rdx)) as u64; r13 = r9; rax = ((rdi.wrapping_add(rbx)) as u64) as u64; r9 = ((r9).wrapping_add(rax)) as u64; r8 = ((r8).rotate_right(21u32)) as u64; rax = r9; rax = ((rax).rotate_right(44u32)) as u64; r8 = ((r8).wrapping_add(rax)) as u64; r13 = ((r13).wrapping_add(r8)) as u64; r8 = ((r10.wrapping_add(r9)) as u64) as u64; r14 = rsi; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xb0u64))) as u64; rax = ((rax).wrapping_add(0x40u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xb0u64), rax); fl_l = (crate::simd_ops::ru64(rsp.wrapping_add(0xc8u64))) as u64; fl_r = (0x40u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xc8u64), ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64); if (((fl_l as u64)) != ((fl_r as u64))) { bb = 9; continue; } rcx = r13; rcx = (((rcx) ^ (r11))) as u64; r10 = (0x9ddfea08eb382d69u64) as u64; rcx = (((rcx) as i64).wrapping_mul((r10) as i64) as u64) as u64; rax = rcx; rax = ((rax).wrapping_shr(47u32)) as u64; rcx = (((rcx) ^ (rax))) as u64; r13 = (((r13) ^ (rcx))) as u64; r13 = (((r13) as i64).wrapping_mul((r10) as i64) as u64) as u64; rcx = r13; rcx = ((rcx).wrapping_shr(47u32)) as u64; rcx = (((rcx) ^ (r13))) as u64; rcx = (((rcx) as i64).wrapping_mul((r10) as i64) as u64) as u64; rcx = ((rcx).wrapping_add(r15)) as u64; rdx = r8; rdx = (((rdx) ^ (crate::simd_ops::ru64(rsp.wrapping_add(0xb8u64))))) as u64; rdx = (((rdx) as i64).wrapping_mul((r10) as i64) as u64) as u64; rax = rdx; rax = ((rax).wrapping_shr(47u32)) as u64; rdx = (((rdx) ^ (rax))) as u64; r8 = (((r8) ^ (rdx))) as u64; r8 = (((r8) as i64).wrapping_mul((r10) as i64) as u64) as u64; rax = r8; rax = ((rax).wrapping_shr(47u32)) as u64; rax = (((rax) ^ (r8))) as u64; rax = (((rax) as i64).wrapping_mul((r10) as i64) as u64) as u64; rdx = rbp; rdx = ((rdx).wrapping_shr(47u32)) as u64; rdx = (((rdx) ^ (rbp))) as u64; rdx = (((rdx) as i64).wrapping_mul((r12) as i64) as u64) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; rdx = ((rdx).wrapping_add(rsi)) as u64; rdx = (((rdx) ^ (rcx))) as u64; rdx = (((rdx) as i64).wrapping_mul((r10) as i64) as u64) as u64; rax = rdx; rax = ((rax).wrapping_shr(47u32)) as u64; rdx = (((rdx) ^ (rax))) as u64; rcx = (((rcx) ^ (rdx))) as u64; rcx = (((rcx) as i64).wrapping_mul((r10) as i64) as u64) as u64; rax = rcx; rax = ((rax).wrapping_shr(47u32)) as u64; rcx = (((rcx) ^ (rax))) as u64; rcx = (((rcx) as i64).wrapping_mul((r10) as i64) as u64) as u64; bb = 10; continue; }
            10 => { rax = (0xde5fb9d2630458e9u64) as u64; let prod = (rax as u128).wrapping_mul((rcx) as u128); rax = prod as u64; rdx = (prod >> 64) as u64; r9 = ((rdx.wrapping_add(rax)) as u64) as u64; r8 = rsp.wrapping_add(0x30); rdx = rsp.wrapping_add(0x20); rcx = P2; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; fl_l = (crate::simd_ops::ru8(rsp.wrapping_add(0x28u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 11; continue; } rbx = ((rdi.wrapping_add(rdi.wrapping_mul(2))) as u64) as u64; rbx = ((rbx).wrapping_shl(4u32)) as u64; rbx = ((rbx).wrapping_add(crate::simd_ops::ru64(P4))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xb0u64), rbx); crate::simd_ops::lk_and_u32(rbx, (0xff000004u32) as u32); crate::simd_ops::lk_or_u32(rbx, (0x4u32) as u32); crate::simd_ops::wu8(rbx.wrapping_add(0x3u64), (0x4u8) as u8); crate::simd_ops::wu32(rbx.wrapping_add(0x4u64), (0x0u32) as u32); crate::simd_ops::wu8(rbx.wrapping_add(0x8u64), (0x0u8) as u8); rdx = rsp.wrapping_add(0x30); rcx = rbx; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx.wrapping_add(0x28u64), (0x0u64) as u64); bb = 11; continue; }
            11 => { rax = ((rdi.wrapping_add(rdi.wrapping_mul(2))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rax = ((rax).wrapping_add(crate::simd_ops::ru64(P4))) as u64; rcx = P6; crate::simd_ops::wu64(rax.wrapping_add(0x28u64), rcx); rcx = rsp.wrapping_add(0x30); rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); rax = 0; fl_l = 0; fl_r = 0; rsp = ((rsp).wrapping_add(0x68u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_36645<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 21>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x68u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (0x4000004u64) as u64); crate::simd_ops::wu8(rsp.wrapping_add(0x38u64), (0x0u8) as u8); rax = P0; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (0x15u64) as u64); r9 = 0x15; r8 = 0; fl_l = 0; fl_r = 0; rdx = rsp.wrapping_add(0x20); rcx = rsp.wrapping_add(0x30); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru32(rsp.wrapping_add(0x34u64))) as u32 as u64; rcx = (crate::simd_ops::ru32(rsp.wrapping_add(0x30u64))) as u32 as u64; rax = ((rcx as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_shr(24u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x80; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 1; continue; } rax = ((rax.wrapping_mul(8)) as u32) as u32 as u64; bb = 2; continue; }
            1 => { rax = (((rax as u32)).wrapping_shl(8u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x7c00; rax = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 2; continue; }
            2 => { rcx = ((((rcx as u32)) & (0xffffffu32))) as u32 as u64; rcx = (((rcx as u32)).wrapping_shl(3u32)) as u32 as u64; r15 = rsp.wrapping_add(0x38); fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; r15 = (if (((fl_l as u32)) > ((fl_r as u32))) { crate::simd_ops::ru64(rsp.wrapping_add(0x38u64)) } else { r15 }) as u64; fl_l = rbx; fl_r = (0x20u64) as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 7; continue; } fl_l = rbx; fl_r = (0x10u64) as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 6; continue; } fl_l = rbx; fl_r = (0x8u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 3; continue; } r8 = (0x9ae16a3b2f90404fu64) as u64; r9 = ((r8.wrapping_add(rbx.wrapping_mul(2))) as u64) as u64; r8 = (crate::simd_ops::ru64(r15)) as u64; rax = (0x651e95c4d06fbfb1u64) as u64; r8 = ((r8).wrapping_sub(rax)) as u64; rdx = (crate::simd_ops::ru64(rbx.wrapping_add(r15).wrapping_sub(0x8u64))) as u64; rdx = ((rdx).rotate_right(37u32)) as u64; rdx = (((rdx) as i64).wrapping_mul((r9) as i64) as u64) as u64; rdx = ((rdx).wrapping_add(r8)) as u64; r8 = ((r8).rotate_right(25u32)) as u64; r8 = ((r8).wrapping_add(crate::simd_ops::ru64(rbx.wrapping_add(r15).wrapping_sub(0x8u64)))) as u64; r8 = (((r8) as i64).wrapping_mul((r9) as i64) as u64) as u64; rcx = r8; rcx = (((rcx) ^ (rdx))) as u64; rcx = (((rcx) as i64).wrapping_mul((r9) as i64) as u64) as u64; rax = rcx; rax = ((rax).wrapping_shr(47u32)) as u64; rcx = (((rcx) ^ (rax))) as u64; rcx = (((rcx) ^ (r8))) as u64; rcx = (((rcx) as i64).wrapping_mul((r9) as i64) as u64) as u64; rax = rcx; rax = ((rax).wrapping_shr(47u32)) as u64; rcx = (((rcx) ^ (rax))) as u64; rcx = (((rcx) as i64).wrapping_mul((r9) as i64) as u64) as u64; bb = 10; continue; }
            3 => { fl_l = rbx; fl_r = (0x4u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 4; continue; } r8 = (0x9ae16a3b2f90404fu64) as u64; r8 = ((r8.wrapping_add(rbx.wrapping_mul(2))) as u64) as u64; rax = (crate::simd_ops::ru32(r15)) as u32 as u64; rdx = (crate::simd_ops::ru32(rbx.wrapping_add(r15).wrapping_sub(0x4u64))) as u32 as u64; rcx = ((rbx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rcx = (((rcx) ^ (rdx))) as u64; rcx = (((rcx) as i64).wrapping_mul((r8) as i64) as u64) as u64; rax = rcx; rax = ((rax).wrapping_shr(47u32)) as u64; rcx = (((rcx) ^ (rax))) as u64; rcx = (((rcx) ^ (rdx))) as u64; rcx = (((rcx) as i64).wrapping_mul((r8) as i64) as u64) as u64; rax = rcx; rax = ((rax).wrapping_shr(47u32)) as u64; rcx = (((rcx) ^ (rax))) as u64; rcx = (((rcx) as i64).wrapping_mul((r8) as i64) as u64) as u64; bb = 10; continue; }
            4 => { fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } rax = rbx; rax = ((rax).wrapping_shr(1u32)) as u64; rdx = ((crate::simd_ops::ru8(rax.wrapping_add(r15))) as u32) as u32 as u64; rax = (0xe16a3b2f90404f00u64) as u64; rdx = (((rdx) as i64).wrapping_mul((rax) as i64) as u64) as u64; rax = ((crate::simd_ops::ru8(r15)) as u32) as u32 as u64; r8 = (0x9ae16a3b2f90404fu64) as u64; rax = (((rax) as i64).wrapping_mul((r8) as i64) as u64) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; rax = ((crate::simd_ops::ru8(rbx.wrapping_add(r15).wrapping_sub(0x1u64))) as u32) as u32 as u64; rax = ((rbx.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = ((rax as u32)) as u32 as u64; rcx = (0xc3a5c85c97cb3127u64) as u64; rax = (((rax) as i64).wrapping_mul((rcx) as i64) as u64) as u64; rdx = (((rdx) ^ (rax))) as u64; rcx = rdx; rcx = ((rcx).wrapping_shr(47u32)) as u64; rcx = (((rcx) ^ (rdx))) as u64; rcx = (((rcx) as i64).wrapping_mul((r8) as i64) as u64) as u64; bb = 10; continue; }
            5 => { rcx = (0x9ae16a3b2f90404fu64) as u64; bb = 10; continue; }
            6 => { r8 = (0x9ae16a3b2f90404fu64) as u64; r11 = ((r8.wrapping_add(rbx.wrapping_mul(2))) as u64) as u64; r12 = (0xb492b66fbe98f273u64) as u64; r9 = (crate::simd_ops::ru64(r15)) as u64; r9 = (((r9) as i64).wrapping_mul((r12) as i64) as u64) as u64; rdx = (crate::simd_ops::ru64(r15.wrapping_add(0x8u64))) as u64; r10 = r11; r10 = (((r10) as i64).wrapping_mul((crate::simd_ops::ru64(rbx.wrapping_add(r15).wrapping_sub(0x8u64))) as i64) as u64) as u64; rcx = (crate::simd_ops::ru64(rbx.wrapping_add(r15).wrapping_sub(0x10u64))) as u64; rcx = (((rcx) as i64).wrapping_mul((r8) as i64) as u64) as u64; r8 = ((r8).wrapping_add(rdx)) as u64; r8 = ((r8).rotate_right(18u32)) as u64; r8 = ((r8).wrapping_add(r10)) as u64; r8 = ((r8).wrapping_add(r9)) as u64; rax = ((rdx.wrapping_add(r9)) as u64) as u64; rax = ((rax).rotate_right(43u32)) as u64; r10 = ((r10).rotate_right(30u32)) as u64; rcx = ((rcx).wrapping_add(rax)) as u64; rcx = ((rcx).wrapping_add(r10)) as u64; rcx = (((rcx) ^ (r8))) as u64; rcx = (((rcx) as i64).wrapping_mul((r11) as i64) as u64) as u64; rax = rcx; rax = ((rax).wrapping_shr(47u32)) as u64; rcx = (((rcx) ^ (rax))) as u64; rcx = (((rcx) ^ (r8))) as u64; rcx = (((rcx) as i64).wrapping_mul((r11) as i64) as u64) as u64; rax = rcx; rax = ((rax).wrapping_shr(47u32)) as u64; rcx = (((rcx) ^ (rax))) as u64; rcx = (((rcx) as i64).wrapping_mul((r11) as i64) as u64) as u64; bb = 10; continue; }
            7 => { r13 = ((rbx.wrapping_add(r15)) as u64) as u64; fl_l = rbx; fl_r = (0x40u64) as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 8; continue; } r8 = (0x9ae16a3b2f90404fu64) as u64; r14 = ((r8.wrapping_add(rbx.wrapping_mul(2))) as u64) as u64; r11 = (crate::simd_ops::ru64(r13.wrapping_sub(0x18u64))) as u64; r10 = (crate::simd_ops::ru64(r15.wrapping_add(0x10u64))) as u64; r10 = (((r10) as i64).wrapping_mul((r8) as i64) as u64) as u64; rax = (crate::simd_ops::ru64(r15.wrapping_add(0x18u64))) as u64; r9 = ((rax.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rdi = r14; rdi = (((rdi) as i64).wrapping_mul((crate::simd_ops::ru64(r13.wrapping_sub(0x10u64))) as i64) as u64) as u64; rdx = (crate::simd_ops::ru64(r15)) as u64; rdx = (((rdx) as i64).wrapping_mul((r8) as i64) as u64) as u64; rdx = ((rdx).wrapping_add(crate::simd_ops::ru64(r13.wrapping_sub(0x8u64)))) as u64; rax = (crate::simd_ops::ru64(r15.wrapping_add(0x8u64))) as u64; rax = ((rax).rotate_right(30u32)) as u64; rax = ((rax).wrapping_add(r11)) as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rax = rdx; rax = ((rax).rotate_right(43u32)) as u64; rcx = ((rcx).wrapping_add(rax)) as u64; r8 = (crate::simd_ops::ru64(r13.wrapping_sub(0x20u64))) as u64; r8 = (((r8) ^ (rdx))) as u64; r8 = ((r8).wrapping_add(1)) as u64; r8 = ((r8).wrapping_add(r9)) as u64; rdx = ((r8.wrapping_add(rcx)) as u64) as u64; rdx = (((rdx) as i64).wrapping_mul((r14) as i64) as u64) as u64; rdx = ((rdx).swap_bytes()) as u64; rdx = ((rdx).wrapping_add(rdi)) as u64; rax = ((r9.wrapping_add(r10)) as u64) as u64; r9 = rax; r9 = ((r9).rotate_right(42u32)) as u64; r9 = ((r9).wrapping_add(r11)) as u64; rcx = ((rax.wrapping_add(r11)) as u64) as u64; rax = ((rdx.wrapping_add(r8)) as u64) as u64; rax = (((rax) as i64).wrapping_mul((r14) as i64) as u64) as u64; rax = ((rax).swap_bytes()) as u64; rax = ((rax).wrapping_add(rcx)) as u64; rax = ((rax).wrapping_add(r9)) as u64; rax = ((rax).wrapping_add(crate::simd_ops::ru64(r13.wrapping_sub(0x8u64)))) as u64; rax = (((rax) as i64).wrapping_mul((r14) as i64) as u64) as u64; rax = ((rax).swap_bytes()) as u64; rax = ((rax).wrapping_add(crate::simd_ops::ru64(r15.wrapping_add(0x8u64)))) as u64; rcx = ((rcx).wrapping_add(rax)) as u64; rcx = (((rcx) as i64).wrapping_mul((r14) as i64) as u64) as u64; rcx = ((rcx).wrapping_add(rdi)) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(r13.wrapping_sub(0x20u64)))) as u64; rax = rcx; rax = ((rax).wrapping_shr(47u32)) as u64; rcx = (((rcx) ^ (rax))) as u64; rcx = (((rcx) as i64).wrapping_mul((r14) as i64) as u64) as u64; fl_l = ((rcx).wrapping_add(r9)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; bb = 10; continue; }
            8 => { rdi = (crate::simd_ops::ru64(r13.wrapping_sub(0x28u64))) as u64; rbp = (crate::simd_ops::ru64(r13.wrapping_sub(0x38u64))) as u64; rbp = ((rbp).wrapping_add(crate::simd_ops::ru64(r13.wrapping_sub(0x10u64)))) as u64; r14 = (crate::simd_ops::ru64(r13.wrapping_sub(0x30u64))) as u64; r14 = ((r14).wrapping_add(rbx)) as u64; r14 = (((r14) ^ (crate::simd_ops::ru64(r13.wrapping_sub(0x18u64))))) as u64; rdx = (0x9ddfea08eb382d69u64) as u64; r14 = (((r14) as i64).wrapping_mul((rdx) as i64) as u64) as u64; rax = r14; rax = ((rax).wrapping_shr(47u32)) as u64; r14 = (((r14) ^ (rax))) as u64; r14 = (((r14) ^ (crate::simd_ops::ru64(r13.wrapping_sub(0x18u64))))) as u64; r14 = (((r14) as i64).wrapping_mul((rdx) as i64) as u64) as u64; rax = r14; rax = ((rax).wrapping_shr(47u32)) as u64; r14 = (((r14) ^ (rax))) as u64; r14 = (((r14) as i64).wrapping_mul((rdx) as i64) as u64) as u64; rax = (crate::simd_ops::ru64(r13.wrapping_sub(0x30u64))) as u64; r10 = (crate::simd_ops::ru64(r13.wrapping_sub(0x40u64))) as u64; r10 = ((r10).wrapping_add(rbx)) as u64; r9 = ((rdi.wrapping_add(r10)) as u64) as u64; r9 = ((r9).wrapping_add(r14)) as u64; r8 = r10; rax = ((rax).wrapping_add(crate::simd_ops::ru64(r13.wrapping_sub(0x38u64)))) as u64; r10 = ((r10).wrapping_add(rax)) as u64; r9 = ((r9).rotate_right(21u32)) as u64; rax = r10; rax = ((rax).rotate_right(44u32)) as u64; rax = ((rax).wrapping_add(r9)) as u64; rax = ((rax).wrapping_add(r8)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xc0u64), rax); r9 = ((rdi.wrapping_add(r10)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xb8u64), r9); r11 = (crate::simd_ops::ru64(r13.wrapping_sub(0x8u64))) as u64; rax = (crate::simd_ops::ru64(r13.wrapping_sub(0x10u64))) as u64; rcx = (crate::simd_ops::ru64(r13.wrapping_sub(0x18u64))) as u64; r10 = (crate::simd_ops::ru64(r13.wrapping_sub(0x20u64))) as u64; r12 = (0xb492b66fbe98f273u64) as u64; r10 = ((r10).wrapping_add(r12)) as u64; r10 = ((r10).wrapping_add(rbp)) as u64; r9 = ((r11.wrapping_add(r10)) as u64) as u64; r9 = ((r9).wrapping_add(rdi)) as u64; r13 = r10; rax = ((rax).wrapping_add(rcx)) as u64; r10 = ((r10).wrapping_add(rax)) as u64; r9 = ((r9).rotate_right(21u32)) as u64; rax = r10; rax = ((rax).rotate_right(44u32)) as u64; r9 = ((r9).wrapping_add(rax)) as u64; r13 = ((r13).wrapping_add(r9)) as u64; r8 = ((r11.wrapping_add(r10)) as u64) as u64; rdi = (((rdi) as i64).wrapping_mul((r12) as i64) as u64) as u64; rdi = ((rdi).wrapping_add(crate::simd_ops::ru64(r15))) as u64; rbx = ((rbx).wrapping_sub(1)) as u64; fl_l = (((rbx) & (0xffffffffffffffc0u64))) as u64; fl_r = fl_l; rbx = ((fl_l as u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xc8u64), rbx); rax = r15.wrapping_add(0x30); crate::simd_ops::wu64(rsp.wrapping_add(0xb0u64), rax); r15 = rdi; bb = 9; continue; }
            9 => { r11 = (crate::simd_ops::ru64(rax.wrapping_sub(0x28u64))) as u64; rsi = ((r11.wrapping_add(rbp)) as u64) as u64; rsi = ((rsi).wrapping_add(r15)) as u64; rsi = ((rsi).wrapping_add(crate::simd_ops::ru64(rsp.wrapping_add(0xb8u64)))) as u64; rdi = (crate::simd_ops::ru64(rax)) as u64; rbp = ((rbp).wrapping_add(rdi)) as u64; rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0xc0u64))) as u64; rbp = ((rbp).wrapping_add(rdx)) as u64; rsi = ((rsi).rotate_right(37u32)) as u64; rsi = (((rsi) as i64).wrapping_mul((r12) as i64) as u64) as u64; rsi = (((rsi) ^ (r13))) as u64; rbx = (crate::simd_ops::ru64(rax.wrapping_sub(0x8u64))) as u64; rbp = ((rbp).rotate_right(42u32)) as u64; rbp = (((rbp) as i64).wrapping_mul((r12) as i64) as u64) as u64; rbp = ((rbp).wrapping_add(rbx)) as u64; rbp = ((rbp).wrapping_add(crate::simd_ops::ru64(rsp.wrapping_add(0xb8u64)))) as u64; r15 = ((r14.wrapping_add(r8)) as u64) as u64; r15 = ((r15).rotate_right(33u32)) as u64; r15 = (((r15) as i64).wrapping_mul((r12) as i64) as u64) as u64; rcx = ((rsi.wrapping_add(r8)) as u64) as u64; rdx = (((rdx) as i64).wrapping_mul((r12) as i64) as u64) as u64; r9 = (crate::simd_ops::ru64(rax.wrapping_sub(0x18u64))) as u64; r10 = (crate::simd_ops::ru64(rax.wrapping_sub(0x20u64))) as u64; r8 = (crate::simd_ops::ru64(rax.wrapping_sub(0x30u64))) as u64; r8 = ((r8).wrapping_add(rdx)) as u64; rdx = ((r9.wrapping_add(r8)) as u64) as u64; rdx = ((rdx).wrapping_add(rcx)) as u64; rcx = r8; rax = ((r10.wrapping_add(r11)) as u64) as u64; r8 = ((r8).wrapping_add(rax)) as u64; rdx = ((rdx).rotate_right(21u32)) as u64; r11 = r8; r11 = ((r11).rotate_right(44u32)) as u64; r11 = ((r11).wrapping_add(rdx)) as u64; r11 = ((r11).wrapping_add(rcx)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xc0u64), r11); rax = ((r9.wrapping_add(r8)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xb8u64), rax); rdx = ((r10.wrapping_add(rbp)) as u64) as u64; r9 = ((r15.wrapping_add(r13)) as u64) as u64; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xb0u64))) as u64; r10 = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; r9 = ((r9).wrapping_add(crate::simd_ops::ru64(rax.wrapping_sub(0x10u64)))) as u64; r8 = ((r10.wrapping_add(r9)) as u64) as u64; r8 = ((r8).wrapping_add(rdx)) as u64; r13 = r9; rax = ((rdi.wrapping_add(rbx)) as u64) as u64; r9 = ((r9).wrapping_add(rax)) as u64; r8 = ((r8).rotate_right(21u32)) as u64; rax = r9; rax = ((rax).rotate_right(44u32)) as u64; r8 = ((r8).wrapping_add(rax)) as u64; r13 = ((r13).wrapping_add(r8)) as u64; r8 = ((r10.wrapping_add(r9)) as u64) as u64; r14 = rsi; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xb0u64))) as u64; rax = ((rax).wrapping_add(0x40u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xb0u64), rax); fl_l = (crate::simd_ops::ru64(rsp.wrapping_add(0xc8u64))) as u64; fl_r = (0x40u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xc8u64), ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64); if (((fl_l as u64)) != ((fl_r as u64))) { bb = 9; continue; } rcx = r13; rcx = (((rcx) ^ (r11))) as u64; r10 = (0x9ddfea08eb382d69u64) as u64; rcx = (((rcx) as i64).wrapping_mul((r10) as i64) as u64) as u64; rax = rcx; rax = ((rax).wrapping_shr(47u32)) as u64; rcx = (((rcx) ^ (rax))) as u64; r13 = (((r13) ^ (rcx))) as u64; r13 = (((r13) as i64).wrapping_mul((r10) as i64) as u64) as u64; rcx = r13; rcx = ((rcx).wrapping_shr(47u32)) as u64; rcx = (((rcx) ^ (r13))) as u64; rcx = (((rcx) as i64).wrapping_mul((r10) as i64) as u64) as u64; rcx = ((rcx).wrapping_add(r15)) as u64; rdx = r8; rdx = (((rdx) ^ (crate::simd_ops::ru64(rsp.wrapping_add(0xb8u64))))) as u64; rdx = (((rdx) as i64).wrapping_mul((r10) as i64) as u64) as u64; rax = rdx; rax = ((rax).wrapping_shr(47u32)) as u64; rdx = (((rdx) ^ (rax))) as u64; r8 = (((r8) ^ (rdx))) as u64; r8 = (((r8) as i64).wrapping_mul((r10) as i64) as u64) as u64; rax = r8; rax = ((rax).wrapping_shr(47u32)) as u64; rax = (((rax) ^ (r8))) as u64; rax = (((rax) as i64).wrapping_mul((r10) as i64) as u64) as u64; rdx = rbp; rdx = ((rdx).wrapping_shr(47u32)) as u64; rdx = (((rdx) ^ (rbp))) as u64; rdx = (((rdx) as i64).wrapping_mul((r12) as i64) as u64) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; rdx = ((rdx).wrapping_add(rsi)) as u64; rdx = (((rdx) ^ (rcx))) as u64; rdx = (((rdx) as i64).wrapping_mul((r10) as i64) as u64) as u64; rax = rdx; rax = ((rax).wrapping_shr(47u32)) as u64; rdx = (((rdx) ^ (rax))) as u64; rcx = (((rcx) ^ (rdx))) as u64; rcx = (((rcx) as i64).wrapping_mul((r10) as i64) as u64) as u64; rax = rcx; rax = ((rax).wrapping_shr(47u32)) as u64; rcx = (((rcx) ^ (rax))) as u64; rcx = (((rcx) as i64).wrapping_mul((r10) as i64) as u64) as u64; bb = 10; continue; }
            10 => { rax = (0xde5fb9d2630458e9u64) as u64; let prod = (rax as u128).wrapping_mul((rcx) as u128); rax = prod as u64; rdx = (prod >> 64) as u64; r9 = ((rdx.wrapping_add(rax)) as u64) as u64; r8 = rsp.wrapping_add(0x30); rdx = rsp.wrapping_add(0x20); rcx = P2; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; fl_l = (crate::simd_ops::ru8(rsp.wrapping_add(0x28u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 11; continue; } rbx = ((rdi.wrapping_add(rdi.wrapping_mul(2))) as u64) as u64; rbx = ((rbx).wrapping_shl(4u32)) as u64; rbx = ((rbx).wrapping_add(crate::simd_ops::ru64(P4))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xb0u64), rbx); crate::simd_ops::lk_and_u32(rbx, (0xff000004u32) as u32); crate::simd_ops::lk_or_u32(rbx, (0x4u32) as u32); crate::simd_ops::wu8(rbx.wrapping_add(0x3u64), (0x4u8) as u8); crate::simd_ops::wu32(rbx.wrapping_add(0x4u64), (0x0u32) as u32); crate::simd_ops::wu8(rbx.wrapping_add(0x8u64), (0x0u8) as u8); rdx = rsp.wrapping_add(0x30); rcx = rbx; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx.wrapping_add(0x28u64), (0x0u64) as u64); bb = 11; continue; }
            11 => { rax = ((rdi.wrapping_add(rdi.wrapping_mul(2))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rax = ((rax).wrapping_add(crate::simd_ops::ru64(P4))) as u64; rcx = P6; crate::simd_ops::wu64(rax.wrapping_add(0x28u64), rcx); rcx = rsp.wrapping_add(0x30); rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); rax = 0; fl_l = 0; fl_r = 0; rsp = ((rsp).wrapping_add(0x68u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_36646<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 21>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x68u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (0x4000004u64) as u64); crate::simd_ops::wu8(rsp.wrapping_add(0x38u64), (0x0u8) as u8); rax = P0; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (0x9u64) as u64); r9 = 0x9; r8 = 0; fl_l = 0; fl_r = 0; rdx = rsp.wrapping_add(0x20); rcx = rsp.wrapping_add(0x30); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru32(rsp.wrapping_add(0x34u64))) as u32 as u64; rcx = (crate::simd_ops::ru32(rsp.wrapping_add(0x30u64))) as u32 as u64; rax = ((rcx as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_shr(24u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x80; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 1; continue; } rax = ((rax.wrapping_mul(8)) as u32) as u32 as u64; bb = 2; continue; }
            1 => { rax = (((rax as u32)).wrapping_shl(8u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x7c00; rax = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 2; continue; }
            2 => { rcx = ((((rcx as u32)) & (0xffffffu32))) as u32 as u64; rcx = (((rcx as u32)).wrapping_shl(3u32)) as u32 as u64; r15 = rsp.wrapping_add(0x38); fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; r15 = (if (((fl_l as u32)) > ((fl_r as u32))) { crate::simd_ops::ru64(rsp.wrapping_add(0x38u64)) } else { r15 }) as u64; fl_l = rbx; fl_r = (0x20u64) as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 7; continue; } fl_l = rbx; fl_r = (0x10u64) as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 6; continue; } fl_l = rbx; fl_r = (0x8u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 3; continue; } r8 = (0x9ae16a3b2f90404fu64) as u64; r9 = ((r8.wrapping_add(rbx.wrapping_mul(2))) as u64) as u64; r8 = (crate::simd_ops::ru64(r15)) as u64; rax = (0x651e95c4d06fbfb1u64) as u64; r8 = ((r8).wrapping_sub(rax)) as u64; rdx = (crate::simd_ops::ru64(rbx.wrapping_add(r15).wrapping_sub(0x8u64))) as u64; rdx = ((rdx).rotate_right(37u32)) as u64; rdx = (((rdx) as i64).wrapping_mul((r9) as i64) as u64) as u64; rdx = ((rdx).wrapping_add(r8)) as u64; r8 = ((r8).rotate_right(25u32)) as u64; r8 = ((r8).wrapping_add(crate::simd_ops::ru64(rbx.wrapping_add(r15).wrapping_sub(0x8u64)))) as u64; r8 = (((r8) as i64).wrapping_mul((r9) as i64) as u64) as u64; rcx = r8; rcx = (((rcx) ^ (rdx))) as u64; rcx = (((rcx) as i64).wrapping_mul((r9) as i64) as u64) as u64; rax = rcx; rax = ((rax).wrapping_shr(47u32)) as u64; rcx = (((rcx) ^ (rax))) as u64; rcx = (((rcx) ^ (r8))) as u64; rcx = (((rcx) as i64).wrapping_mul((r9) as i64) as u64) as u64; rax = rcx; rax = ((rax).wrapping_shr(47u32)) as u64; rcx = (((rcx) ^ (rax))) as u64; rcx = (((rcx) as i64).wrapping_mul((r9) as i64) as u64) as u64; bb = 10; continue; }
            3 => { fl_l = rbx; fl_r = (0x4u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 4; continue; } r8 = (0x9ae16a3b2f90404fu64) as u64; r8 = ((r8.wrapping_add(rbx.wrapping_mul(2))) as u64) as u64; rax = (crate::simd_ops::ru32(r15)) as u32 as u64; rdx = (crate::simd_ops::ru32(rbx.wrapping_add(r15).wrapping_sub(0x4u64))) as u32 as u64; rcx = ((rbx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rcx = (((rcx) ^ (rdx))) as u64; rcx = (((rcx) as i64).wrapping_mul((r8) as i64) as u64) as u64; rax = rcx; rax = ((rax).wrapping_shr(47u32)) as u64; rcx = (((rcx) ^ (rax))) as u64; rcx = (((rcx) ^ (rdx))) as u64; rcx = (((rcx) as i64).wrapping_mul((r8) as i64) as u64) as u64; rax = rcx; rax = ((rax).wrapping_shr(47u32)) as u64; rcx = (((rcx) ^ (rax))) as u64; rcx = (((rcx) as i64).wrapping_mul((r8) as i64) as u64) as u64; bb = 10; continue; }
            4 => { fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } rax = rbx; rax = ((rax).wrapping_shr(1u32)) as u64; rdx = ((crate::simd_ops::ru8(rax.wrapping_add(r15))) as u32) as u32 as u64; rax = (0xe16a3b2f90404f00u64) as u64; rdx = (((rdx) as i64).wrapping_mul((rax) as i64) as u64) as u64; rax = ((crate::simd_ops::ru8(r15)) as u32) as u32 as u64; r8 = (0x9ae16a3b2f90404fu64) as u64; rax = (((rax) as i64).wrapping_mul((r8) as i64) as u64) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; rax = ((crate::simd_ops::ru8(rbx.wrapping_add(r15).wrapping_sub(0x1u64))) as u32) as u32 as u64; rax = ((rbx.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = ((rax as u32)) as u32 as u64; rcx = (0xc3a5c85c97cb3127u64) as u64; rax = (((rax) as i64).wrapping_mul((rcx) as i64) as u64) as u64; rdx = (((rdx) ^ (rax))) as u64; rcx = rdx; rcx = ((rcx).wrapping_shr(47u32)) as u64; rcx = (((rcx) ^ (rdx))) as u64; rcx = (((rcx) as i64).wrapping_mul((r8) as i64) as u64) as u64; bb = 10; continue; }
            5 => { rcx = (0x9ae16a3b2f90404fu64) as u64; bb = 10; continue; }
            6 => { r8 = (0x9ae16a3b2f90404fu64) as u64; r11 = ((r8.wrapping_add(rbx.wrapping_mul(2))) as u64) as u64; r12 = (0xb492b66fbe98f273u64) as u64; r9 = (crate::simd_ops::ru64(r15)) as u64; r9 = (((r9) as i64).wrapping_mul((r12) as i64) as u64) as u64; rdx = (crate::simd_ops::ru64(r15.wrapping_add(0x8u64))) as u64; r10 = r11; r10 = (((r10) as i64).wrapping_mul((crate::simd_ops::ru64(rbx.wrapping_add(r15).wrapping_sub(0x8u64))) as i64) as u64) as u64; rcx = (crate::simd_ops::ru64(rbx.wrapping_add(r15).wrapping_sub(0x10u64))) as u64; rcx = (((rcx) as i64).wrapping_mul((r8) as i64) as u64) as u64; r8 = ((r8).wrapping_add(rdx)) as u64; r8 = ((r8).rotate_right(18u32)) as u64; r8 = ((r8).wrapping_add(r10)) as u64; r8 = ((r8).wrapping_add(r9)) as u64; rax = ((rdx.wrapping_add(r9)) as u64) as u64; rax = ((rax).rotate_right(43u32)) as u64; r10 = ((r10).rotate_right(30u32)) as u64; rcx = ((rcx).wrapping_add(rax)) as u64; rcx = ((rcx).wrapping_add(r10)) as u64; rcx = (((rcx) ^ (r8))) as u64; rcx = (((rcx) as i64).wrapping_mul((r11) as i64) as u64) as u64; rax = rcx; rax = ((rax).wrapping_shr(47u32)) as u64; rcx = (((rcx) ^ (rax))) as u64; rcx = (((rcx) ^ (r8))) as u64; rcx = (((rcx) as i64).wrapping_mul((r11) as i64) as u64) as u64; rax = rcx; rax = ((rax).wrapping_shr(47u32)) as u64; rcx = (((rcx) ^ (rax))) as u64; rcx = (((rcx) as i64).wrapping_mul((r11) as i64) as u64) as u64; bb = 10; continue; }
            7 => { r13 = ((rbx.wrapping_add(r15)) as u64) as u64; fl_l = rbx; fl_r = (0x40u64) as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 8; continue; } r8 = (0x9ae16a3b2f90404fu64) as u64; r14 = ((r8.wrapping_add(rbx.wrapping_mul(2))) as u64) as u64; r11 = (crate::simd_ops::ru64(r13.wrapping_sub(0x18u64))) as u64; r10 = (crate::simd_ops::ru64(r15.wrapping_add(0x10u64))) as u64; r10 = (((r10) as i64).wrapping_mul((r8) as i64) as u64) as u64; rax = (crate::simd_ops::ru64(r15.wrapping_add(0x18u64))) as u64; r9 = ((rax.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rdi = r14; rdi = (((rdi) as i64).wrapping_mul((crate::simd_ops::ru64(r13.wrapping_sub(0x10u64))) as i64) as u64) as u64; rdx = (crate::simd_ops::ru64(r15)) as u64; rdx = (((rdx) as i64).wrapping_mul((r8) as i64) as u64) as u64; rdx = ((rdx).wrapping_add(crate::simd_ops::ru64(r13.wrapping_sub(0x8u64)))) as u64; rax = (crate::simd_ops::ru64(r15.wrapping_add(0x8u64))) as u64; rax = ((rax).rotate_right(30u32)) as u64; rax = ((rax).wrapping_add(r11)) as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rax = rdx; rax = ((rax).rotate_right(43u32)) as u64; rcx = ((rcx).wrapping_add(rax)) as u64; r8 = (crate::simd_ops::ru64(r13.wrapping_sub(0x20u64))) as u64; r8 = (((r8) ^ (rdx))) as u64; r8 = ((r8).wrapping_add(1)) as u64; r8 = ((r8).wrapping_add(r9)) as u64; rdx = ((r8.wrapping_add(rcx)) as u64) as u64; rdx = (((rdx) as i64).wrapping_mul((r14) as i64) as u64) as u64; rdx = ((rdx).swap_bytes()) as u64; rdx = ((rdx).wrapping_add(rdi)) as u64; rax = ((r9.wrapping_add(r10)) as u64) as u64; r9 = rax; r9 = ((r9).rotate_right(42u32)) as u64; r9 = ((r9).wrapping_add(r11)) as u64; rcx = ((rax.wrapping_add(r11)) as u64) as u64; rax = ((rdx.wrapping_add(r8)) as u64) as u64; rax = (((rax) as i64).wrapping_mul((r14) as i64) as u64) as u64; rax = ((rax).swap_bytes()) as u64; rax = ((rax).wrapping_add(rcx)) as u64; rax = ((rax).wrapping_add(r9)) as u64; rax = ((rax).wrapping_add(crate::simd_ops::ru64(r13.wrapping_sub(0x8u64)))) as u64; rax = (((rax) as i64).wrapping_mul((r14) as i64) as u64) as u64; rax = ((rax).swap_bytes()) as u64; rax = ((rax).wrapping_add(crate::simd_ops::ru64(r15.wrapping_add(0x8u64)))) as u64; rcx = ((rcx).wrapping_add(rax)) as u64; rcx = (((rcx) as i64).wrapping_mul((r14) as i64) as u64) as u64; rcx = ((rcx).wrapping_add(rdi)) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(r13.wrapping_sub(0x20u64)))) as u64; rax = rcx; rax = ((rax).wrapping_shr(47u32)) as u64; rcx = (((rcx) ^ (rax))) as u64; rcx = (((rcx) as i64).wrapping_mul((r14) as i64) as u64) as u64; fl_l = ((rcx).wrapping_add(r9)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; bb = 10; continue; }
            8 => { rdi = (crate::simd_ops::ru64(r13.wrapping_sub(0x28u64))) as u64; rbp = (crate::simd_ops::ru64(r13.wrapping_sub(0x38u64))) as u64; rbp = ((rbp).wrapping_add(crate::simd_ops::ru64(r13.wrapping_sub(0x10u64)))) as u64; r14 = (crate::simd_ops::ru64(r13.wrapping_sub(0x30u64))) as u64; r14 = ((r14).wrapping_add(rbx)) as u64; r14 = (((r14) ^ (crate::simd_ops::ru64(r13.wrapping_sub(0x18u64))))) as u64; rdx = (0x9ddfea08eb382d69u64) as u64; r14 = (((r14) as i64).wrapping_mul((rdx) as i64) as u64) as u64; rax = r14; rax = ((rax).wrapping_shr(47u32)) as u64; r14 = (((r14) ^ (rax))) as u64; r14 = (((r14) ^ (crate::simd_ops::ru64(r13.wrapping_sub(0x18u64))))) as u64; r14 = (((r14) as i64).wrapping_mul((rdx) as i64) as u64) as u64; rax = r14; rax = ((rax).wrapping_shr(47u32)) as u64; r14 = (((r14) ^ (rax))) as u64; r14 = (((r14) as i64).wrapping_mul((rdx) as i64) as u64) as u64; rax = (crate::simd_ops::ru64(r13.wrapping_sub(0x30u64))) as u64; r10 = (crate::simd_ops::ru64(r13.wrapping_sub(0x40u64))) as u64; r10 = ((r10).wrapping_add(rbx)) as u64; r9 = ((rdi.wrapping_add(r10)) as u64) as u64; r9 = ((r9).wrapping_add(r14)) as u64; r8 = r10; rax = ((rax).wrapping_add(crate::simd_ops::ru64(r13.wrapping_sub(0x38u64)))) as u64; r10 = ((r10).wrapping_add(rax)) as u64; r9 = ((r9).rotate_right(21u32)) as u64; rax = r10; rax = ((rax).rotate_right(44u32)) as u64; rax = ((rax).wrapping_add(r9)) as u64; rax = ((rax).wrapping_add(r8)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xc0u64), rax); r9 = ((rdi.wrapping_add(r10)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xb8u64), r9); r11 = (crate::simd_ops::ru64(r13.wrapping_sub(0x8u64))) as u64; rax = (crate::simd_ops::ru64(r13.wrapping_sub(0x10u64))) as u64; rcx = (crate::simd_ops::ru64(r13.wrapping_sub(0x18u64))) as u64; r10 = (crate::simd_ops::ru64(r13.wrapping_sub(0x20u64))) as u64; r12 = (0xb492b66fbe98f273u64) as u64; r10 = ((r10).wrapping_add(r12)) as u64; r10 = ((r10).wrapping_add(rbp)) as u64; r9 = ((r11.wrapping_add(r10)) as u64) as u64; r9 = ((r9).wrapping_add(rdi)) as u64; r13 = r10; rax = ((rax).wrapping_add(rcx)) as u64; r10 = ((r10).wrapping_add(rax)) as u64; r9 = ((r9).rotate_right(21u32)) as u64; rax = r10; rax = ((rax).rotate_right(44u32)) as u64; r9 = ((r9).wrapping_add(rax)) as u64; r13 = ((r13).wrapping_add(r9)) as u64; r8 = ((r11.wrapping_add(r10)) as u64) as u64; rdi = (((rdi) as i64).wrapping_mul((r12) as i64) as u64) as u64; rdi = ((rdi).wrapping_add(crate::simd_ops::ru64(r15))) as u64; rbx = ((rbx).wrapping_sub(1)) as u64; fl_l = (((rbx) & (0xffffffffffffffc0u64))) as u64; fl_r = fl_l; rbx = ((fl_l as u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xc8u64), rbx); rax = r15.wrapping_add(0x30); crate::simd_ops::wu64(rsp.wrapping_add(0xb0u64), rax); r15 = rdi; bb = 9; continue; }
            9 => { r11 = (crate::simd_ops::ru64(rax.wrapping_sub(0x28u64))) as u64; rsi = ((r11.wrapping_add(rbp)) as u64) as u64; rsi = ((rsi).wrapping_add(r15)) as u64; rsi = ((rsi).wrapping_add(crate::simd_ops::ru64(rsp.wrapping_add(0xb8u64)))) as u64; rdi = (crate::simd_ops::ru64(rax)) as u64; rbp = ((rbp).wrapping_add(rdi)) as u64; rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0xc0u64))) as u64; rbp = ((rbp).wrapping_add(rdx)) as u64; rsi = ((rsi).rotate_right(37u32)) as u64; rsi = (((rsi) as i64).wrapping_mul((r12) as i64) as u64) as u64; rsi = (((rsi) ^ (r13))) as u64; rbx = (crate::simd_ops::ru64(rax.wrapping_sub(0x8u64))) as u64; rbp = ((rbp).rotate_right(42u32)) as u64; rbp = (((rbp) as i64).wrapping_mul((r12) as i64) as u64) as u64; rbp = ((rbp).wrapping_add(rbx)) as u64; rbp = ((rbp).wrapping_add(crate::simd_ops::ru64(rsp.wrapping_add(0xb8u64)))) as u64; r15 = ((r14.wrapping_add(r8)) as u64) as u64; r15 = ((r15).rotate_right(33u32)) as u64; r15 = (((r15) as i64).wrapping_mul((r12) as i64) as u64) as u64; rcx = ((rsi.wrapping_add(r8)) as u64) as u64; rdx = (((rdx) as i64).wrapping_mul((r12) as i64) as u64) as u64; r9 = (crate::simd_ops::ru64(rax.wrapping_sub(0x18u64))) as u64; r10 = (crate::simd_ops::ru64(rax.wrapping_sub(0x20u64))) as u64; r8 = (crate::simd_ops::ru64(rax.wrapping_sub(0x30u64))) as u64; r8 = ((r8).wrapping_add(rdx)) as u64; rdx = ((r9.wrapping_add(r8)) as u64) as u64; rdx = ((rdx).wrapping_add(rcx)) as u64; rcx = r8; rax = ((r10.wrapping_add(r11)) as u64) as u64; r8 = ((r8).wrapping_add(rax)) as u64; rdx = ((rdx).rotate_right(21u32)) as u64; r11 = r8; r11 = ((r11).rotate_right(44u32)) as u64; r11 = ((r11).wrapping_add(rdx)) as u64; r11 = ((r11).wrapping_add(rcx)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xc0u64), r11); rax = ((r9.wrapping_add(r8)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xb8u64), rax); rdx = ((r10.wrapping_add(rbp)) as u64) as u64; r9 = ((r15.wrapping_add(r13)) as u64) as u64; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xb0u64))) as u64; r10 = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; r9 = ((r9).wrapping_add(crate::simd_ops::ru64(rax.wrapping_sub(0x10u64)))) as u64; r8 = ((r10.wrapping_add(r9)) as u64) as u64; r8 = ((r8).wrapping_add(rdx)) as u64; r13 = r9; rax = ((rdi.wrapping_add(rbx)) as u64) as u64; r9 = ((r9).wrapping_add(rax)) as u64; r8 = ((r8).rotate_right(21u32)) as u64; rax = r9; rax = ((rax).rotate_right(44u32)) as u64; r8 = ((r8).wrapping_add(rax)) as u64; r13 = ((r13).wrapping_add(r8)) as u64; r8 = ((r10.wrapping_add(r9)) as u64) as u64; r14 = rsi; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xb0u64))) as u64; rax = ((rax).wrapping_add(0x40u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xb0u64), rax); fl_l = (crate::simd_ops::ru64(rsp.wrapping_add(0xc8u64))) as u64; fl_r = (0x40u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xc8u64), ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64); if (((fl_l as u64)) != ((fl_r as u64))) { bb = 9; continue; } rcx = r13; rcx = (((rcx) ^ (r11))) as u64; r10 = (0x9ddfea08eb382d69u64) as u64; rcx = (((rcx) as i64).wrapping_mul((r10) as i64) as u64) as u64; rax = rcx; rax = ((rax).wrapping_shr(47u32)) as u64; rcx = (((rcx) ^ (rax))) as u64; r13 = (((r13) ^ (rcx))) as u64; r13 = (((r13) as i64).wrapping_mul((r10) as i64) as u64) as u64; rcx = r13; rcx = ((rcx).wrapping_shr(47u32)) as u64; rcx = (((rcx) ^ (r13))) as u64; rcx = (((rcx) as i64).wrapping_mul((r10) as i64) as u64) as u64; rcx = ((rcx).wrapping_add(r15)) as u64; rdx = r8; rdx = (((rdx) ^ (crate::simd_ops::ru64(rsp.wrapping_add(0xb8u64))))) as u64; rdx = (((rdx) as i64).wrapping_mul((r10) as i64) as u64) as u64; rax = rdx; rax = ((rax).wrapping_shr(47u32)) as u64; rdx = (((rdx) ^ (rax))) as u64; r8 = (((r8) ^ (rdx))) as u64; r8 = (((r8) as i64).wrapping_mul((r10) as i64) as u64) as u64; rax = r8; rax = ((rax).wrapping_shr(47u32)) as u64; rax = (((rax) ^ (r8))) as u64; rax = (((rax) as i64).wrapping_mul((r10) as i64) as u64) as u64; rdx = rbp; rdx = ((rdx).wrapping_shr(47u32)) as u64; rdx = (((rdx) ^ (rbp))) as u64; rdx = (((rdx) as i64).wrapping_mul((r12) as i64) as u64) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; rdx = ((rdx).wrapping_add(rsi)) as u64; rdx = (((rdx) ^ (rcx))) as u64; rdx = (((rdx) as i64).wrapping_mul((r10) as i64) as u64) as u64; rax = rdx; rax = ((rax).wrapping_shr(47u32)) as u64; rdx = (((rdx) ^ (rax))) as u64; rcx = (((rcx) ^ (rdx))) as u64; rcx = (((rcx) as i64).wrapping_mul((r10) as i64) as u64) as u64; rax = rcx; rax = ((rax).wrapping_shr(47u32)) as u64; rcx = (((rcx) ^ (rax))) as u64; rcx = (((rcx) as i64).wrapping_mul((r10) as i64) as u64) as u64; bb = 10; continue; }
            10 => { rax = (0xde5fb9d2630458e9u64) as u64; let prod = (rax as u128).wrapping_mul((rcx) as u128); rax = prod as u64; rdx = (prod >> 64) as u64; r9 = ((rdx.wrapping_add(rax)) as u64) as u64; r8 = rsp.wrapping_add(0x30); rdx = rsp.wrapping_add(0x20); rcx = P2; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; fl_l = (crate::simd_ops::ru8(rsp.wrapping_add(0x28u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 11; continue; } rbx = ((rdi.wrapping_add(rdi.wrapping_mul(2))) as u64) as u64; rbx = ((rbx).wrapping_shl(4u32)) as u64; rbx = ((rbx).wrapping_add(crate::simd_ops::ru64(P4))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xb0u64), rbx); crate::simd_ops::lk_and_u32(rbx, (0xff000004u32) as u32); crate::simd_ops::lk_or_u32(rbx, (0x4u32) as u32); crate::simd_ops::wu8(rbx.wrapping_add(0x3u64), (0x4u8) as u8); crate::simd_ops::wu32(rbx.wrapping_add(0x4u64), (0x0u32) as u32); crate::simd_ops::wu8(rbx.wrapping_add(0x8u64), (0x0u8) as u8); rdx = rsp.wrapping_add(0x30); rcx = rbx; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx.wrapping_add(0x28u64), (0x0u64) as u64); bb = 11; continue; }
            11 => { rax = ((rdi.wrapping_add(rdi.wrapping_mul(2))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rax = ((rax).wrapping_add(crate::simd_ops::ru64(P4))) as u64; rcx = P6; crate::simd_ops::wu64(rax.wrapping_add(0x28u64), rcx); rcx = rsp.wrapping_add(0x30); rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); rax = 0; fl_l = 0; fl_r = 0; rsp = ((rsp).wrapping_add(0x68u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_36648<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 21>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x68u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (0x4000004u64) as u64); crate::simd_ops::wu8(rsp.wrapping_add(0x38u64), (0x0u8) as u8); rax = P0; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (0x8u64) as u64); r9 = 0x8; r8 = 0; fl_l = 0; fl_r = 0; rdx = rsp.wrapping_add(0x20); rcx = rsp.wrapping_add(0x30); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru32(rsp.wrapping_add(0x34u64))) as u32 as u64; rcx = (crate::simd_ops::ru32(rsp.wrapping_add(0x30u64))) as u32 as u64; rax = ((rcx as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_shr(24u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x80; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 1; continue; } rax = ((rax.wrapping_mul(8)) as u32) as u32 as u64; bb = 2; continue; }
            1 => { rax = (((rax as u32)).wrapping_shl(8u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x7c00; rax = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 2; continue; }
            2 => { rcx = ((((rcx as u32)) & (0xffffffu32))) as u32 as u64; rcx = (((rcx as u32)).wrapping_shl(3u32)) as u32 as u64; r15 = rsp.wrapping_add(0x38); fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; r15 = (if (((fl_l as u32)) > ((fl_r as u32))) { crate::simd_ops::ru64(rsp.wrapping_add(0x38u64)) } else { r15 }) as u64; fl_l = rbx; fl_r = (0x20u64) as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 7; continue; } fl_l = rbx; fl_r = (0x10u64) as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 6; continue; } fl_l = rbx; fl_r = (0x8u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 3; continue; } r8 = (0x9ae16a3b2f90404fu64) as u64; r9 = ((r8.wrapping_add(rbx.wrapping_mul(2))) as u64) as u64; r8 = (crate::simd_ops::ru64(r15)) as u64; rax = (0x651e95c4d06fbfb1u64) as u64; r8 = ((r8).wrapping_sub(rax)) as u64; rdx = (crate::simd_ops::ru64(rbx.wrapping_add(r15).wrapping_sub(0x8u64))) as u64; rdx = ((rdx).rotate_right(37u32)) as u64; rdx = (((rdx) as i64).wrapping_mul((r9) as i64) as u64) as u64; rdx = ((rdx).wrapping_add(r8)) as u64; r8 = ((r8).rotate_right(25u32)) as u64; r8 = ((r8).wrapping_add(crate::simd_ops::ru64(rbx.wrapping_add(r15).wrapping_sub(0x8u64)))) as u64; r8 = (((r8) as i64).wrapping_mul((r9) as i64) as u64) as u64; rcx = r8; rcx = (((rcx) ^ (rdx))) as u64; rcx = (((rcx) as i64).wrapping_mul((r9) as i64) as u64) as u64; rax = rcx; rax = ((rax).wrapping_shr(47u32)) as u64; rcx = (((rcx) ^ (rax))) as u64; rcx = (((rcx) ^ (r8))) as u64; rcx = (((rcx) as i64).wrapping_mul((r9) as i64) as u64) as u64; rax = rcx; rax = ((rax).wrapping_shr(47u32)) as u64; rcx = (((rcx) ^ (rax))) as u64; rcx = (((rcx) as i64).wrapping_mul((r9) as i64) as u64) as u64; bb = 10; continue; }
            3 => { fl_l = rbx; fl_r = (0x4u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 4; continue; } r8 = (0x9ae16a3b2f90404fu64) as u64; r8 = ((r8.wrapping_add(rbx.wrapping_mul(2))) as u64) as u64; rax = (crate::simd_ops::ru32(r15)) as u32 as u64; rdx = (crate::simd_ops::ru32(rbx.wrapping_add(r15).wrapping_sub(0x4u64))) as u32 as u64; rcx = ((rbx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rcx = (((rcx) ^ (rdx))) as u64; rcx = (((rcx) as i64).wrapping_mul((r8) as i64) as u64) as u64; rax = rcx; rax = ((rax).wrapping_shr(47u32)) as u64; rcx = (((rcx) ^ (rax))) as u64; rcx = (((rcx) ^ (rdx))) as u64; rcx = (((rcx) as i64).wrapping_mul((r8) as i64) as u64) as u64; rax = rcx; rax = ((rax).wrapping_shr(47u32)) as u64; rcx = (((rcx) ^ (rax))) as u64; rcx = (((rcx) as i64).wrapping_mul((r8) as i64) as u64) as u64; bb = 10; continue; }
            4 => { fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } rax = rbx; rax = ((rax).wrapping_shr(1u32)) as u64; rdx = ((crate::simd_ops::ru8(rax.wrapping_add(r15))) as u32) as u32 as u64; rax = (0xe16a3b2f90404f00u64) as u64; rdx = (((rdx) as i64).wrapping_mul((rax) as i64) as u64) as u64; rax = ((crate::simd_ops::ru8(r15)) as u32) as u32 as u64; r8 = (0x9ae16a3b2f90404fu64) as u64; rax = (((rax) as i64).wrapping_mul((r8) as i64) as u64) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; rax = ((crate::simd_ops::ru8(rbx.wrapping_add(r15).wrapping_sub(0x1u64))) as u32) as u32 as u64; rax = ((rbx.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = ((rax as u32)) as u32 as u64; rcx = (0xc3a5c85c97cb3127u64) as u64; rax = (((rax) as i64).wrapping_mul((rcx) as i64) as u64) as u64; rdx = (((rdx) ^ (rax))) as u64; rcx = rdx; rcx = ((rcx).wrapping_shr(47u32)) as u64; rcx = (((rcx) ^ (rdx))) as u64; rcx = (((rcx) as i64).wrapping_mul((r8) as i64) as u64) as u64; bb = 10; continue; }
            5 => { rcx = (0x9ae16a3b2f90404fu64) as u64; bb = 10; continue; }
            6 => { r8 = (0x9ae16a3b2f90404fu64) as u64; r11 = ((r8.wrapping_add(rbx.wrapping_mul(2))) as u64) as u64; r12 = (0xb492b66fbe98f273u64) as u64; r9 = (crate::simd_ops::ru64(r15)) as u64; r9 = (((r9) as i64).wrapping_mul((r12) as i64) as u64) as u64; rdx = (crate::simd_ops::ru64(r15.wrapping_add(0x8u64))) as u64; r10 = r11; r10 = (((r10) as i64).wrapping_mul((crate::simd_ops::ru64(rbx.wrapping_add(r15).wrapping_sub(0x8u64))) as i64) as u64) as u64; rcx = (crate::simd_ops::ru64(rbx.wrapping_add(r15).wrapping_sub(0x10u64))) as u64; rcx = (((rcx) as i64).wrapping_mul((r8) as i64) as u64) as u64; r8 = ((r8).wrapping_add(rdx)) as u64; r8 = ((r8).rotate_right(18u32)) as u64; r8 = ((r8).wrapping_add(r10)) as u64; r8 = ((r8).wrapping_add(r9)) as u64; rax = ((rdx.wrapping_add(r9)) as u64) as u64; rax = ((rax).rotate_right(43u32)) as u64; r10 = ((r10).rotate_right(30u32)) as u64; rcx = ((rcx).wrapping_add(rax)) as u64; rcx = ((rcx).wrapping_add(r10)) as u64; rcx = (((rcx) ^ (r8))) as u64; rcx = (((rcx) as i64).wrapping_mul((r11) as i64) as u64) as u64; rax = rcx; rax = ((rax).wrapping_shr(47u32)) as u64; rcx = (((rcx) ^ (rax))) as u64; rcx = (((rcx) ^ (r8))) as u64; rcx = (((rcx) as i64).wrapping_mul((r11) as i64) as u64) as u64; rax = rcx; rax = ((rax).wrapping_shr(47u32)) as u64; rcx = (((rcx) ^ (rax))) as u64; rcx = (((rcx) as i64).wrapping_mul((r11) as i64) as u64) as u64; bb = 10; continue; }
            7 => { r13 = ((rbx.wrapping_add(r15)) as u64) as u64; fl_l = rbx; fl_r = (0x40u64) as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 8; continue; } r8 = (0x9ae16a3b2f90404fu64) as u64; r14 = ((r8.wrapping_add(rbx.wrapping_mul(2))) as u64) as u64; r11 = (crate::simd_ops::ru64(r13.wrapping_sub(0x18u64))) as u64; r10 = (crate::simd_ops::ru64(r15.wrapping_add(0x10u64))) as u64; r10 = (((r10) as i64).wrapping_mul((r8) as i64) as u64) as u64; rax = (crate::simd_ops::ru64(r15.wrapping_add(0x18u64))) as u64; r9 = ((rax.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rdi = r14; rdi = (((rdi) as i64).wrapping_mul((crate::simd_ops::ru64(r13.wrapping_sub(0x10u64))) as i64) as u64) as u64; rdx = (crate::simd_ops::ru64(r15)) as u64; rdx = (((rdx) as i64).wrapping_mul((r8) as i64) as u64) as u64; rdx = ((rdx).wrapping_add(crate::simd_ops::ru64(r13.wrapping_sub(0x8u64)))) as u64; rax = (crate::simd_ops::ru64(r15.wrapping_add(0x8u64))) as u64; rax = ((rax).rotate_right(30u32)) as u64; rax = ((rax).wrapping_add(r11)) as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rax = rdx; rax = ((rax).rotate_right(43u32)) as u64; rcx = ((rcx).wrapping_add(rax)) as u64; r8 = (crate::simd_ops::ru64(r13.wrapping_sub(0x20u64))) as u64; r8 = (((r8) ^ (rdx))) as u64; r8 = ((r8).wrapping_add(1)) as u64; r8 = ((r8).wrapping_add(r9)) as u64; rdx = ((r8.wrapping_add(rcx)) as u64) as u64; rdx = (((rdx) as i64).wrapping_mul((r14) as i64) as u64) as u64; rdx = ((rdx).swap_bytes()) as u64; rdx = ((rdx).wrapping_add(rdi)) as u64; rax = ((r9.wrapping_add(r10)) as u64) as u64; r9 = rax; r9 = ((r9).rotate_right(42u32)) as u64; r9 = ((r9).wrapping_add(r11)) as u64; rcx = ((rax.wrapping_add(r11)) as u64) as u64; rax = ((rdx.wrapping_add(r8)) as u64) as u64; rax = (((rax) as i64).wrapping_mul((r14) as i64) as u64) as u64; rax = ((rax).swap_bytes()) as u64; rax = ((rax).wrapping_add(rcx)) as u64; rax = ((rax).wrapping_add(r9)) as u64; rax = ((rax).wrapping_add(crate::simd_ops::ru64(r13.wrapping_sub(0x8u64)))) as u64; rax = (((rax) as i64).wrapping_mul((r14) as i64) as u64) as u64; rax = ((rax).swap_bytes()) as u64; rax = ((rax).wrapping_add(crate::simd_ops::ru64(r15.wrapping_add(0x8u64)))) as u64; rcx = ((rcx).wrapping_add(rax)) as u64; rcx = (((rcx) as i64).wrapping_mul((r14) as i64) as u64) as u64; rcx = ((rcx).wrapping_add(rdi)) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(r13.wrapping_sub(0x20u64)))) as u64; rax = rcx; rax = ((rax).wrapping_shr(47u32)) as u64; rcx = (((rcx) ^ (rax))) as u64; rcx = (((rcx) as i64).wrapping_mul((r14) as i64) as u64) as u64; fl_l = ((rcx).wrapping_add(r9)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; bb = 10; continue; }
            8 => { rdi = (crate::simd_ops::ru64(r13.wrapping_sub(0x28u64))) as u64; rbp = (crate::simd_ops::ru64(r13.wrapping_sub(0x38u64))) as u64; rbp = ((rbp).wrapping_add(crate::simd_ops::ru64(r13.wrapping_sub(0x10u64)))) as u64; r14 = (crate::simd_ops::ru64(r13.wrapping_sub(0x30u64))) as u64; r14 = ((r14).wrapping_add(rbx)) as u64; r14 = (((r14) ^ (crate::simd_ops::ru64(r13.wrapping_sub(0x18u64))))) as u64; rdx = (0x9ddfea08eb382d69u64) as u64; r14 = (((r14) as i64).wrapping_mul((rdx) as i64) as u64) as u64; rax = r14; rax = ((rax).wrapping_shr(47u32)) as u64; r14 = (((r14) ^ (rax))) as u64; r14 = (((r14) ^ (crate::simd_ops::ru64(r13.wrapping_sub(0x18u64))))) as u64; r14 = (((r14) as i64).wrapping_mul((rdx) as i64) as u64) as u64; rax = r14; rax = ((rax).wrapping_shr(47u32)) as u64; r14 = (((r14) ^ (rax))) as u64; r14 = (((r14) as i64).wrapping_mul((rdx) as i64) as u64) as u64; rax = (crate::simd_ops::ru64(r13.wrapping_sub(0x30u64))) as u64; r10 = (crate::simd_ops::ru64(r13.wrapping_sub(0x40u64))) as u64; r10 = ((r10).wrapping_add(rbx)) as u64; r9 = ((rdi.wrapping_add(r10)) as u64) as u64; r9 = ((r9).wrapping_add(r14)) as u64; r8 = r10; rax = ((rax).wrapping_add(crate::simd_ops::ru64(r13.wrapping_sub(0x38u64)))) as u64; r10 = ((r10).wrapping_add(rax)) as u64; r9 = ((r9).rotate_right(21u32)) as u64; rax = r10; rax = ((rax).rotate_right(44u32)) as u64; rax = ((rax).wrapping_add(r9)) as u64; rax = ((rax).wrapping_add(r8)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xc0u64), rax); r9 = ((rdi.wrapping_add(r10)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xb8u64), r9); r11 = (crate::simd_ops::ru64(r13.wrapping_sub(0x8u64))) as u64; rax = (crate::simd_ops::ru64(r13.wrapping_sub(0x10u64))) as u64; rcx = (crate::simd_ops::ru64(r13.wrapping_sub(0x18u64))) as u64; r10 = (crate::simd_ops::ru64(r13.wrapping_sub(0x20u64))) as u64; r12 = (0xb492b66fbe98f273u64) as u64; r10 = ((r10).wrapping_add(r12)) as u64; r10 = ((r10).wrapping_add(rbp)) as u64; r9 = ((r11.wrapping_add(r10)) as u64) as u64; r9 = ((r9).wrapping_add(rdi)) as u64; r13 = r10; rax = ((rax).wrapping_add(rcx)) as u64; r10 = ((r10).wrapping_add(rax)) as u64; r9 = ((r9).rotate_right(21u32)) as u64; rax = r10; rax = ((rax).rotate_right(44u32)) as u64; r9 = ((r9).wrapping_add(rax)) as u64; r13 = ((r13).wrapping_add(r9)) as u64; r8 = ((r11.wrapping_add(r10)) as u64) as u64; rdi = (((rdi) as i64).wrapping_mul((r12) as i64) as u64) as u64; rdi = ((rdi).wrapping_add(crate::simd_ops::ru64(r15))) as u64; rbx = ((rbx).wrapping_sub(1)) as u64; fl_l = (((rbx) & (0xffffffffffffffc0u64))) as u64; fl_r = fl_l; rbx = ((fl_l as u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xc8u64), rbx); rax = r15.wrapping_add(0x30); crate::simd_ops::wu64(rsp.wrapping_add(0xb0u64), rax); r15 = rdi; bb = 9; continue; }
            9 => { r11 = (crate::simd_ops::ru64(rax.wrapping_sub(0x28u64))) as u64; rsi = ((r11.wrapping_add(rbp)) as u64) as u64; rsi = ((rsi).wrapping_add(r15)) as u64; rsi = ((rsi).wrapping_add(crate::simd_ops::ru64(rsp.wrapping_add(0xb8u64)))) as u64; rdi = (crate::simd_ops::ru64(rax)) as u64; rbp = ((rbp).wrapping_add(rdi)) as u64; rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0xc0u64))) as u64; rbp = ((rbp).wrapping_add(rdx)) as u64; rsi = ((rsi).rotate_right(37u32)) as u64; rsi = (((rsi) as i64).wrapping_mul((r12) as i64) as u64) as u64; rsi = (((rsi) ^ (r13))) as u64; rbx = (crate::simd_ops::ru64(rax.wrapping_sub(0x8u64))) as u64; rbp = ((rbp).rotate_right(42u32)) as u64; rbp = (((rbp) as i64).wrapping_mul((r12) as i64) as u64) as u64; rbp = ((rbp).wrapping_add(rbx)) as u64; rbp = ((rbp).wrapping_add(crate::simd_ops::ru64(rsp.wrapping_add(0xb8u64)))) as u64; r15 = ((r14.wrapping_add(r8)) as u64) as u64; r15 = ((r15).rotate_right(33u32)) as u64; r15 = (((r15) as i64).wrapping_mul((r12) as i64) as u64) as u64; rcx = ((rsi.wrapping_add(r8)) as u64) as u64; rdx = (((rdx) as i64).wrapping_mul((r12) as i64) as u64) as u64; r9 = (crate::simd_ops::ru64(rax.wrapping_sub(0x18u64))) as u64; r10 = (crate::simd_ops::ru64(rax.wrapping_sub(0x20u64))) as u64; r8 = (crate::simd_ops::ru64(rax.wrapping_sub(0x30u64))) as u64; r8 = ((r8).wrapping_add(rdx)) as u64; rdx = ((r9.wrapping_add(r8)) as u64) as u64; rdx = ((rdx).wrapping_add(rcx)) as u64; rcx = r8; rax = ((r10.wrapping_add(r11)) as u64) as u64; r8 = ((r8).wrapping_add(rax)) as u64; rdx = ((rdx).rotate_right(21u32)) as u64; r11 = r8; r11 = ((r11).rotate_right(44u32)) as u64; r11 = ((r11).wrapping_add(rdx)) as u64; r11 = ((r11).wrapping_add(rcx)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xc0u64), r11); rax = ((r9.wrapping_add(r8)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xb8u64), rax); rdx = ((r10.wrapping_add(rbp)) as u64) as u64; r9 = ((r15.wrapping_add(r13)) as u64) as u64; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xb0u64))) as u64; r10 = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; r9 = ((r9).wrapping_add(crate::simd_ops::ru64(rax.wrapping_sub(0x10u64)))) as u64; r8 = ((r10.wrapping_add(r9)) as u64) as u64; r8 = ((r8).wrapping_add(rdx)) as u64; r13 = r9; rax = ((rdi.wrapping_add(rbx)) as u64) as u64; r9 = ((r9).wrapping_add(rax)) as u64; r8 = ((r8).rotate_right(21u32)) as u64; rax = r9; rax = ((rax).rotate_right(44u32)) as u64; r8 = ((r8).wrapping_add(rax)) as u64; r13 = ((r13).wrapping_add(r8)) as u64; r8 = ((r10.wrapping_add(r9)) as u64) as u64; r14 = rsi; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xb0u64))) as u64; rax = ((rax).wrapping_add(0x40u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xb0u64), rax); fl_l = (crate::simd_ops::ru64(rsp.wrapping_add(0xc8u64))) as u64; fl_r = (0x40u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xc8u64), ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64); if (((fl_l as u64)) != ((fl_r as u64))) { bb = 9; continue; } rcx = r13; rcx = (((rcx) ^ (r11))) as u64; r10 = (0x9ddfea08eb382d69u64) as u64; rcx = (((rcx) as i64).wrapping_mul((r10) as i64) as u64) as u64; rax = rcx; rax = ((rax).wrapping_shr(47u32)) as u64; rcx = (((rcx) ^ (rax))) as u64; r13 = (((r13) ^ (rcx))) as u64; r13 = (((r13) as i64).wrapping_mul((r10) as i64) as u64) as u64; rcx = r13; rcx = ((rcx).wrapping_shr(47u32)) as u64; rcx = (((rcx) ^ (r13))) as u64; rcx = (((rcx) as i64).wrapping_mul((r10) as i64) as u64) as u64; rcx = ((rcx).wrapping_add(r15)) as u64; rdx = r8; rdx = (((rdx) ^ (crate::simd_ops::ru64(rsp.wrapping_add(0xb8u64))))) as u64; rdx = (((rdx) as i64).wrapping_mul((r10) as i64) as u64) as u64; rax = rdx; rax = ((rax).wrapping_shr(47u32)) as u64; rdx = (((rdx) ^ (rax))) as u64; r8 = (((r8) ^ (rdx))) as u64; r8 = (((r8) as i64).wrapping_mul((r10) as i64) as u64) as u64; rax = r8; rax = ((rax).wrapping_shr(47u32)) as u64; rax = (((rax) ^ (r8))) as u64; rax = (((rax) as i64).wrapping_mul((r10) as i64) as u64) as u64; rdx = rbp; rdx = ((rdx).wrapping_shr(47u32)) as u64; rdx = (((rdx) ^ (rbp))) as u64; rdx = (((rdx) as i64).wrapping_mul((r12) as i64) as u64) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; rdx = ((rdx).wrapping_add(rsi)) as u64; rdx = (((rdx) ^ (rcx))) as u64; rdx = (((rdx) as i64).wrapping_mul((r10) as i64) as u64) as u64; rax = rdx; rax = ((rax).wrapping_shr(47u32)) as u64; rdx = (((rdx) ^ (rax))) as u64; rcx = (((rcx) ^ (rdx))) as u64; rcx = (((rcx) as i64).wrapping_mul((r10) as i64) as u64) as u64; rax = rcx; rax = ((rax).wrapping_shr(47u32)) as u64; rcx = (((rcx) ^ (rax))) as u64; rcx = (((rcx) as i64).wrapping_mul((r10) as i64) as u64) as u64; bb = 10; continue; }
            10 => { rax = (0xde5fb9d2630458e9u64) as u64; let prod = (rax as u128).wrapping_mul((rcx) as u128); rax = prod as u64; rdx = (prod >> 64) as u64; r9 = ((rdx.wrapping_add(rax)) as u64) as u64; r8 = rsp.wrapping_add(0x30); rdx = rsp.wrapping_add(0x20); rcx = P2; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; fl_l = (crate::simd_ops::ru8(rsp.wrapping_add(0x28u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 11; continue; } rbx = ((rdi.wrapping_add(rdi.wrapping_mul(2))) as u64) as u64; rbx = ((rbx).wrapping_shl(4u32)) as u64; rbx = ((rbx).wrapping_add(crate::simd_ops::ru64(P4))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xb0u64), rbx); crate::simd_ops::lk_and_u32(rbx, (0xff000004u32) as u32); crate::simd_ops::lk_or_u32(rbx, (0x4u32) as u32); crate::simd_ops::wu8(rbx.wrapping_add(0x3u64), (0x4u8) as u8); crate::simd_ops::wu32(rbx.wrapping_add(0x4u64), (0x0u32) as u32); crate::simd_ops::wu8(rbx.wrapping_add(0x8u64), (0x0u8) as u8); rdx = rsp.wrapping_add(0x30); rcx = rbx; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx.wrapping_add(0x28u64), (0x0u64) as u64); bb = 11; continue; }
            11 => { rax = ((rdi.wrapping_add(rdi.wrapping_mul(2))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rax = ((rax).wrapping_add(crate::simd_ops::ru64(P4))) as u64; rcx = P6; crate::simd_ops::wu64(rax.wrapping_add(0x28u64), rcx); rcx = rsp.wrapping_add(0x30); rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); rax = 0; fl_l = 0; fl_r = 0; rsp = ((rsp).wrapping_add(0x68u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_36649<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 21>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x68u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (0x4000004u64) as u64); crate::simd_ops::wu8(rsp.wrapping_add(0x38u64), (0x0u8) as u8); rax = P0; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (0x7u64) as u64); r9 = 0x7; r8 = 0; fl_l = 0; fl_r = 0; rdx = rsp.wrapping_add(0x20); rcx = rsp.wrapping_add(0x30); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru32(rsp.wrapping_add(0x34u64))) as u32 as u64; rcx = (crate::simd_ops::ru32(rsp.wrapping_add(0x30u64))) as u32 as u64; rax = ((rcx as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_shr(24u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x80; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 1; continue; } rax = ((rax.wrapping_mul(8)) as u32) as u32 as u64; bb = 2; continue; }
            1 => { rax = (((rax as u32)).wrapping_shl(8u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x7c00; rax = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 2; continue; }
            2 => { rcx = ((((rcx as u32)) & (0xffffffu32))) as u32 as u64; rcx = (((rcx as u32)).wrapping_shl(3u32)) as u32 as u64; r15 = rsp.wrapping_add(0x38); fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; r15 = (if (((fl_l as u32)) > ((fl_r as u32))) { crate::simd_ops::ru64(rsp.wrapping_add(0x38u64)) } else { r15 }) as u64; fl_l = rbx; fl_r = (0x20u64) as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 7; continue; } fl_l = rbx; fl_r = (0x10u64) as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 6; continue; } fl_l = rbx; fl_r = (0x8u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 3; continue; } r8 = (0x9ae16a3b2f90404fu64) as u64; r9 = ((r8.wrapping_add(rbx.wrapping_mul(2))) as u64) as u64; r8 = (crate::simd_ops::ru64(r15)) as u64; rax = (0x651e95c4d06fbfb1u64) as u64; r8 = ((r8).wrapping_sub(rax)) as u64; rdx = (crate::simd_ops::ru64(rbx.wrapping_add(r15).wrapping_sub(0x8u64))) as u64; rdx = ((rdx).rotate_right(37u32)) as u64; rdx = (((rdx) as i64).wrapping_mul((r9) as i64) as u64) as u64; rdx = ((rdx).wrapping_add(r8)) as u64; r8 = ((r8).rotate_right(25u32)) as u64; r8 = ((r8).wrapping_add(crate::simd_ops::ru64(rbx.wrapping_add(r15).wrapping_sub(0x8u64)))) as u64; r8 = (((r8) as i64).wrapping_mul((r9) as i64) as u64) as u64; rcx = r8; rcx = (((rcx) ^ (rdx))) as u64; rcx = (((rcx) as i64).wrapping_mul((r9) as i64) as u64) as u64; rax = rcx; rax = ((rax).wrapping_shr(47u32)) as u64; rcx = (((rcx) ^ (rax))) as u64; rcx = (((rcx) ^ (r8))) as u64; rcx = (((rcx) as i64).wrapping_mul((r9) as i64) as u64) as u64; rax = rcx; rax = ((rax).wrapping_shr(47u32)) as u64; rcx = (((rcx) ^ (rax))) as u64; rcx = (((rcx) as i64).wrapping_mul((r9) as i64) as u64) as u64; bb = 10; continue; }
            3 => { fl_l = rbx; fl_r = (0x4u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 4; continue; } r8 = (0x9ae16a3b2f90404fu64) as u64; r8 = ((r8.wrapping_add(rbx.wrapping_mul(2))) as u64) as u64; rax = (crate::simd_ops::ru32(r15)) as u32 as u64; rdx = (crate::simd_ops::ru32(rbx.wrapping_add(r15).wrapping_sub(0x4u64))) as u32 as u64; rcx = ((rbx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rcx = (((rcx) ^ (rdx))) as u64; rcx = (((rcx) as i64).wrapping_mul((r8) as i64) as u64) as u64; rax = rcx; rax = ((rax).wrapping_shr(47u32)) as u64; rcx = (((rcx) ^ (rax))) as u64; rcx = (((rcx) ^ (rdx))) as u64; rcx = (((rcx) as i64).wrapping_mul((r8) as i64) as u64) as u64; rax = rcx; rax = ((rax).wrapping_shr(47u32)) as u64; rcx = (((rcx) ^ (rax))) as u64; rcx = (((rcx) as i64).wrapping_mul((r8) as i64) as u64) as u64; bb = 10; continue; }
            4 => { fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } rax = rbx; rax = ((rax).wrapping_shr(1u32)) as u64; rdx = ((crate::simd_ops::ru8(rax.wrapping_add(r15))) as u32) as u32 as u64; rax = (0xe16a3b2f90404f00u64) as u64; rdx = (((rdx) as i64).wrapping_mul((rax) as i64) as u64) as u64; rax = ((crate::simd_ops::ru8(r15)) as u32) as u32 as u64; r8 = (0x9ae16a3b2f90404fu64) as u64; rax = (((rax) as i64).wrapping_mul((r8) as i64) as u64) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; rax = ((crate::simd_ops::ru8(rbx.wrapping_add(r15).wrapping_sub(0x1u64))) as u32) as u32 as u64; rax = ((rbx.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = ((rax as u32)) as u32 as u64; rcx = (0xc3a5c85c97cb3127u64) as u64; rax = (((rax) as i64).wrapping_mul((rcx) as i64) as u64) as u64; rdx = (((rdx) ^ (rax))) as u64; rcx = rdx; rcx = ((rcx).wrapping_shr(47u32)) as u64; rcx = (((rcx) ^ (rdx))) as u64; rcx = (((rcx) as i64).wrapping_mul((r8) as i64) as u64) as u64; bb = 10; continue; }
            5 => { rcx = (0x9ae16a3b2f90404fu64) as u64; bb = 10; continue; }
            6 => { r8 = (0x9ae16a3b2f90404fu64) as u64; r11 = ((r8.wrapping_add(rbx.wrapping_mul(2))) as u64) as u64; r12 = (0xb492b66fbe98f273u64) as u64; r9 = (crate::simd_ops::ru64(r15)) as u64; r9 = (((r9) as i64).wrapping_mul((r12) as i64) as u64) as u64; rdx = (crate::simd_ops::ru64(r15.wrapping_add(0x8u64))) as u64; r10 = r11; r10 = (((r10) as i64).wrapping_mul((crate::simd_ops::ru64(rbx.wrapping_add(r15).wrapping_sub(0x8u64))) as i64) as u64) as u64; rcx = (crate::simd_ops::ru64(rbx.wrapping_add(r15).wrapping_sub(0x10u64))) as u64; rcx = (((rcx) as i64).wrapping_mul((r8) as i64) as u64) as u64; r8 = ((r8).wrapping_add(rdx)) as u64; r8 = ((r8).rotate_right(18u32)) as u64; r8 = ((r8).wrapping_add(r10)) as u64; r8 = ((r8).wrapping_add(r9)) as u64; rax = ((rdx.wrapping_add(r9)) as u64) as u64; rax = ((rax).rotate_right(43u32)) as u64; r10 = ((r10).rotate_right(30u32)) as u64; rcx = ((rcx).wrapping_add(rax)) as u64; rcx = ((rcx).wrapping_add(r10)) as u64; rcx = (((rcx) ^ (r8))) as u64; rcx = (((rcx) as i64).wrapping_mul((r11) as i64) as u64) as u64; rax = rcx; rax = ((rax).wrapping_shr(47u32)) as u64; rcx = (((rcx) ^ (rax))) as u64; rcx = (((rcx) ^ (r8))) as u64; rcx = (((rcx) as i64).wrapping_mul((r11) as i64) as u64) as u64; rax = rcx; rax = ((rax).wrapping_shr(47u32)) as u64; rcx = (((rcx) ^ (rax))) as u64; rcx = (((rcx) as i64).wrapping_mul((r11) as i64) as u64) as u64; bb = 10; continue; }
            7 => { r13 = ((rbx.wrapping_add(r15)) as u64) as u64; fl_l = rbx; fl_r = (0x40u64) as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 8; continue; } r8 = (0x9ae16a3b2f90404fu64) as u64; r14 = ((r8.wrapping_add(rbx.wrapping_mul(2))) as u64) as u64; r11 = (crate::simd_ops::ru64(r13.wrapping_sub(0x18u64))) as u64; r10 = (crate::simd_ops::ru64(r15.wrapping_add(0x10u64))) as u64; r10 = (((r10) as i64).wrapping_mul((r8) as i64) as u64) as u64; rax = (crate::simd_ops::ru64(r15.wrapping_add(0x18u64))) as u64; r9 = ((rax.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rdi = r14; rdi = (((rdi) as i64).wrapping_mul((crate::simd_ops::ru64(r13.wrapping_sub(0x10u64))) as i64) as u64) as u64; rdx = (crate::simd_ops::ru64(r15)) as u64; rdx = (((rdx) as i64).wrapping_mul((r8) as i64) as u64) as u64; rdx = ((rdx).wrapping_add(crate::simd_ops::ru64(r13.wrapping_sub(0x8u64)))) as u64; rax = (crate::simd_ops::ru64(r15.wrapping_add(0x8u64))) as u64; rax = ((rax).rotate_right(30u32)) as u64; rax = ((rax).wrapping_add(r11)) as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rax = rdx; rax = ((rax).rotate_right(43u32)) as u64; rcx = ((rcx).wrapping_add(rax)) as u64; r8 = (crate::simd_ops::ru64(r13.wrapping_sub(0x20u64))) as u64; r8 = (((r8) ^ (rdx))) as u64; r8 = ((r8).wrapping_add(1)) as u64; r8 = ((r8).wrapping_add(r9)) as u64; rdx = ((r8.wrapping_add(rcx)) as u64) as u64; rdx = (((rdx) as i64).wrapping_mul((r14) as i64) as u64) as u64; rdx = ((rdx).swap_bytes()) as u64; rdx = ((rdx).wrapping_add(rdi)) as u64; rax = ((r9.wrapping_add(r10)) as u64) as u64; r9 = rax; r9 = ((r9).rotate_right(42u32)) as u64; r9 = ((r9).wrapping_add(r11)) as u64; rcx = ((rax.wrapping_add(r11)) as u64) as u64; rax = ((rdx.wrapping_add(r8)) as u64) as u64; rax = (((rax) as i64).wrapping_mul((r14) as i64) as u64) as u64; rax = ((rax).swap_bytes()) as u64; rax = ((rax).wrapping_add(rcx)) as u64; rax = ((rax).wrapping_add(r9)) as u64; rax = ((rax).wrapping_add(crate::simd_ops::ru64(r13.wrapping_sub(0x8u64)))) as u64; rax = (((rax) as i64).wrapping_mul((r14) as i64) as u64) as u64; rax = ((rax).swap_bytes()) as u64; rax = ((rax).wrapping_add(crate::simd_ops::ru64(r15.wrapping_add(0x8u64)))) as u64; rcx = ((rcx).wrapping_add(rax)) as u64; rcx = (((rcx) as i64).wrapping_mul((r14) as i64) as u64) as u64; rcx = ((rcx).wrapping_add(rdi)) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(r13.wrapping_sub(0x20u64)))) as u64; rax = rcx; rax = ((rax).wrapping_shr(47u32)) as u64; rcx = (((rcx) ^ (rax))) as u64; rcx = (((rcx) as i64).wrapping_mul((r14) as i64) as u64) as u64; fl_l = ((rcx).wrapping_add(r9)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; bb = 10; continue; }
            8 => { rdi = (crate::simd_ops::ru64(r13.wrapping_sub(0x28u64))) as u64; rbp = (crate::simd_ops::ru64(r13.wrapping_sub(0x38u64))) as u64; rbp = ((rbp).wrapping_add(crate::simd_ops::ru64(r13.wrapping_sub(0x10u64)))) as u64; r14 = (crate::simd_ops::ru64(r13.wrapping_sub(0x30u64))) as u64; r14 = ((r14).wrapping_add(rbx)) as u64; r14 = (((r14) ^ (crate::simd_ops::ru64(r13.wrapping_sub(0x18u64))))) as u64; rdx = (0x9ddfea08eb382d69u64) as u64; r14 = (((r14) as i64).wrapping_mul((rdx) as i64) as u64) as u64; rax = r14; rax = ((rax).wrapping_shr(47u32)) as u64; r14 = (((r14) ^ (rax))) as u64; r14 = (((r14) ^ (crate::simd_ops::ru64(r13.wrapping_sub(0x18u64))))) as u64; r14 = (((r14) as i64).wrapping_mul((rdx) as i64) as u64) as u64; rax = r14; rax = ((rax).wrapping_shr(47u32)) as u64; r14 = (((r14) ^ (rax))) as u64; r14 = (((r14) as i64).wrapping_mul((rdx) as i64) as u64) as u64; rax = (crate::simd_ops::ru64(r13.wrapping_sub(0x30u64))) as u64; r10 = (crate::simd_ops::ru64(r13.wrapping_sub(0x40u64))) as u64; r10 = ((r10).wrapping_add(rbx)) as u64; r9 = ((rdi.wrapping_add(r10)) as u64) as u64; r9 = ((r9).wrapping_add(r14)) as u64; r8 = r10; rax = ((rax).wrapping_add(crate::simd_ops::ru64(r13.wrapping_sub(0x38u64)))) as u64; r10 = ((r10).wrapping_add(rax)) as u64; r9 = ((r9).rotate_right(21u32)) as u64; rax = r10; rax = ((rax).rotate_right(44u32)) as u64; rax = ((rax).wrapping_add(r9)) as u64; rax = ((rax).wrapping_add(r8)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xc0u64), rax); r9 = ((rdi.wrapping_add(r10)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xb8u64), r9); r11 = (crate::simd_ops::ru64(r13.wrapping_sub(0x8u64))) as u64; rax = (crate::simd_ops::ru64(r13.wrapping_sub(0x10u64))) as u64; rcx = (crate::simd_ops::ru64(r13.wrapping_sub(0x18u64))) as u64; r10 = (crate::simd_ops::ru64(r13.wrapping_sub(0x20u64))) as u64; r12 = (0xb492b66fbe98f273u64) as u64; r10 = ((r10).wrapping_add(r12)) as u64; r10 = ((r10).wrapping_add(rbp)) as u64; r9 = ((r11.wrapping_add(r10)) as u64) as u64; r9 = ((r9).wrapping_add(rdi)) as u64; r13 = r10; rax = ((rax).wrapping_add(rcx)) as u64; r10 = ((r10).wrapping_add(rax)) as u64; r9 = ((r9).rotate_right(21u32)) as u64; rax = r10; rax = ((rax).rotate_right(44u32)) as u64; r9 = ((r9).wrapping_add(rax)) as u64; r13 = ((r13).wrapping_add(r9)) as u64; r8 = ((r11.wrapping_add(r10)) as u64) as u64; rdi = (((rdi) as i64).wrapping_mul((r12) as i64) as u64) as u64; rdi = ((rdi).wrapping_add(crate::simd_ops::ru64(r15))) as u64; rbx = ((rbx).wrapping_sub(1)) as u64; fl_l = (((rbx) & (0xffffffffffffffc0u64))) as u64; fl_r = fl_l; rbx = ((fl_l as u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xc8u64), rbx); rax = r15.wrapping_add(0x30); crate::simd_ops::wu64(rsp.wrapping_add(0xb0u64), rax); r15 = rdi; bb = 9; continue; }
            9 => { r11 = (crate::simd_ops::ru64(rax.wrapping_sub(0x28u64))) as u64; rsi = ((r11.wrapping_add(rbp)) as u64) as u64; rsi = ((rsi).wrapping_add(r15)) as u64; rsi = ((rsi).wrapping_add(crate::simd_ops::ru64(rsp.wrapping_add(0xb8u64)))) as u64; rdi = (crate::simd_ops::ru64(rax)) as u64; rbp = ((rbp).wrapping_add(rdi)) as u64; rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0xc0u64))) as u64; rbp = ((rbp).wrapping_add(rdx)) as u64; rsi = ((rsi).rotate_right(37u32)) as u64; rsi = (((rsi) as i64).wrapping_mul((r12) as i64) as u64) as u64; rsi = (((rsi) ^ (r13))) as u64; rbx = (crate::simd_ops::ru64(rax.wrapping_sub(0x8u64))) as u64; rbp = ((rbp).rotate_right(42u32)) as u64; rbp = (((rbp) as i64).wrapping_mul((r12) as i64) as u64) as u64; rbp = ((rbp).wrapping_add(rbx)) as u64; rbp = ((rbp).wrapping_add(crate::simd_ops::ru64(rsp.wrapping_add(0xb8u64)))) as u64; r15 = ((r14.wrapping_add(r8)) as u64) as u64; r15 = ((r15).rotate_right(33u32)) as u64; r15 = (((r15) as i64).wrapping_mul((r12) as i64) as u64) as u64; rcx = ((rsi.wrapping_add(r8)) as u64) as u64; rdx = (((rdx) as i64).wrapping_mul((r12) as i64) as u64) as u64; r9 = (crate::simd_ops::ru64(rax.wrapping_sub(0x18u64))) as u64; r10 = (crate::simd_ops::ru64(rax.wrapping_sub(0x20u64))) as u64; r8 = (crate::simd_ops::ru64(rax.wrapping_sub(0x30u64))) as u64; r8 = ((r8).wrapping_add(rdx)) as u64; rdx = ((r9.wrapping_add(r8)) as u64) as u64; rdx = ((rdx).wrapping_add(rcx)) as u64; rcx = r8; rax = ((r10.wrapping_add(r11)) as u64) as u64; r8 = ((r8).wrapping_add(rax)) as u64; rdx = ((rdx).rotate_right(21u32)) as u64; r11 = r8; r11 = ((r11).rotate_right(44u32)) as u64; r11 = ((r11).wrapping_add(rdx)) as u64; r11 = ((r11).wrapping_add(rcx)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xc0u64), r11); rax = ((r9.wrapping_add(r8)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xb8u64), rax); rdx = ((r10.wrapping_add(rbp)) as u64) as u64; r9 = ((r15.wrapping_add(r13)) as u64) as u64; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xb0u64))) as u64; r10 = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; r9 = ((r9).wrapping_add(crate::simd_ops::ru64(rax.wrapping_sub(0x10u64)))) as u64; r8 = ((r10.wrapping_add(r9)) as u64) as u64; r8 = ((r8).wrapping_add(rdx)) as u64; r13 = r9; rax = ((rdi.wrapping_add(rbx)) as u64) as u64; r9 = ((r9).wrapping_add(rax)) as u64; r8 = ((r8).rotate_right(21u32)) as u64; rax = r9; rax = ((rax).rotate_right(44u32)) as u64; r8 = ((r8).wrapping_add(rax)) as u64; r13 = ((r13).wrapping_add(r8)) as u64; r8 = ((r10.wrapping_add(r9)) as u64) as u64; r14 = rsi; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xb0u64))) as u64; rax = ((rax).wrapping_add(0x40u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xb0u64), rax); fl_l = (crate::simd_ops::ru64(rsp.wrapping_add(0xc8u64))) as u64; fl_r = (0x40u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xc8u64), ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64); if (((fl_l as u64)) != ((fl_r as u64))) { bb = 9; continue; } rcx = r13; rcx = (((rcx) ^ (r11))) as u64; r10 = (0x9ddfea08eb382d69u64) as u64; rcx = (((rcx) as i64).wrapping_mul((r10) as i64) as u64) as u64; rax = rcx; rax = ((rax).wrapping_shr(47u32)) as u64; rcx = (((rcx) ^ (rax))) as u64; r13 = (((r13) ^ (rcx))) as u64; r13 = (((r13) as i64).wrapping_mul((r10) as i64) as u64) as u64; rcx = r13; rcx = ((rcx).wrapping_shr(47u32)) as u64; rcx = (((rcx) ^ (r13))) as u64; rcx = (((rcx) as i64).wrapping_mul((r10) as i64) as u64) as u64; rcx = ((rcx).wrapping_add(r15)) as u64; rdx = r8; rdx = (((rdx) ^ (crate::simd_ops::ru64(rsp.wrapping_add(0xb8u64))))) as u64; rdx = (((rdx) as i64).wrapping_mul((r10) as i64) as u64) as u64; rax = rdx; rax = ((rax).wrapping_shr(47u32)) as u64; rdx = (((rdx) ^ (rax))) as u64; r8 = (((r8) ^ (rdx))) as u64; r8 = (((r8) as i64).wrapping_mul((r10) as i64) as u64) as u64; rax = r8; rax = ((rax).wrapping_shr(47u32)) as u64; rax = (((rax) ^ (r8))) as u64; rax = (((rax) as i64).wrapping_mul((r10) as i64) as u64) as u64; rdx = rbp; rdx = ((rdx).wrapping_shr(47u32)) as u64; rdx = (((rdx) ^ (rbp))) as u64; rdx = (((rdx) as i64).wrapping_mul((r12) as i64) as u64) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; rdx = ((rdx).wrapping_add(rsi)) as u64; rdx = (((rdx) ^ (rcx))) as u64; rdx = (((rdx) as i64).wrapping_mul((r10) as i64) as u64) as u64; rax = rdx; rax = ((rax).wrapping_shr(47u32)) as u64; rdx = (((rdx) ^ (rax))) as u64; rcx = (((rcx) ^ (rdx))) as u64; rcx = (((rcx) as i64).wrapping_mul((r10) as i64) as u64) as u64; rax = rcx; rax = ((rax).wrapping_shr(47u32)) as u64; rcx = (((rcx) ^ (rax))) as u64; rcx = (((rcx) as i64).wrapping_mul((r10) as i64) as u64) as u64; bb = 10; continue; }
            10 => { rax = (0xde5fb9d2630458e9u64) as u64; let prod = (rax as u128).wrapping_mul((rcx) as u128); rax = prod as u64; rdx = (prod >> 64) as u64; r9 = ((rdx.wrapping_add(rax)) as u64) as u64; r8 = rsp.wrapping_add(0x30); rdx = rsp.wrapping_add(0x20); rcx = P2; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; fl_l = (crate::simd_ops::ru8(rsp.wrapping_add(0x28u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 11; continue; } rbx = ((rdi.wrapping_add(rdi.wrapping_mul(2))) as u64) as u64; rbx = ((rbx).wrapping_shl(4u32)) as u64; rbx = ((rbx).wrapping_add(crate::simd_ops::ru64(P4))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xb0u64), rbx); crate::simd_ops::lk_and_u32(rbx, (0xff000004u32) as u32); crate::simd_ops::lk_or_u32(rbx, (0x4u32) as u32); crate::simd_ops::wu8(rbx.wrapping_add(0x3u64), (0x4u8) as u8); crate::simd_ops::wu32(rbx.wrapping_add(0x4u64), (0x0u32) as u32); crate::simd_ops::wu8(rbx.wrapping_add(0x8u64), (0x0u8) as u8); rdx = rsp.wrapping_add(0x30); rcx = rbx; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx.wrapping_add(0x28u64), (0x0u64) as u64); bb = 11; continue; }
            11 => { rax = ((rdi.wrapping_add(rdi.wrapping_mul(2))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rax = ((rax).wrapping_add(crate::simd_ops::ru64(P4))) as u64; rcx = P6; crate::simd_ops::wu64(rax.wrapping_add(0x28u64), rcx); rcx = rsp.wrapping_add(0x30); rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); rax = 0; fl_l = 0; fl_r = 0; rsp = ((rsp).wrapping_add(0x68u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_36650<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 21>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x68u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (0x4000004u64) as u64); crate::simd_ops::wu8(rsp.wrapping_add(0x38u64), (0x0u8) as u8); rax = P0; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (0x6u64) as u64); r9 = 0x6; r8 = 0; fl_l = 0; fl_r = 0; rdx = rsp.wrapping_add(0x20); rcx = rsp.wrapping_add(0x30); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru32(rsp.wrapping_add(0x34u64))) as u32 as u64; rcx = (crate::simd_ops::ru32(rsp.wrapping_add(0x30u64))) as u32 as u64; rax = ((rcx as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_shr(24u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x80; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 1; continue; } rax = ((rax.wrapping_mul(8)) as u32) as u32 as u64; bb = 2; continue; }
            1 => { rax = (((rax as u32)).wrapping_shl(8u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x7c00; rax = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 2; continue; }
            2 => { rcx = ((((rcx as u32)) & (0xffffffu32))) as u32 as u64; rcx = (((rcx as u32)).wrapping_shl(3u32)) as u32 as u64; r15 = rsp.wrapping_add(0x38); fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; r15 = (if (((fl_l as u32)) > ((fl_r as u32))) { crate::simd_ops::ru64(rsp.wrapping_add(0x38u64)) } else { r15 }) as u64; fl_l = rbx; fl_r = (0x20u64) as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 7; continue; } fl_l = rbx; fl_r = (0x10u64) as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 6; continue; } fl_l = rbx; fl_r = (0x8u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 3; continue; } r8 = (0x9ae16a3b2f90404fu64) as u64; r9 = ((r8.wrapping_add(rbx.wrapping_mul(2))) as u64) as u64; r8 = (crate::simd_ops::ru64(r15)) as u64; rax = (0x651e95c4d06fbfb1u64) as u64; r8 = ((r8).wrapping_sub(rax)) as u64; rdx = (crate::simd_ops::ru64(rbx.wrapping_add(r15).wrapping_sub(0x8u64))) as u64; rdx = ((rdx).rotate_right(37u32)) as u64; rdx = (((rdx) as i64).wrapping_mul((r9) as i64) as u64) as u64; rdx = ((rdx).wrapping_add(r8)) as u64; r8 = ((r8).rotate_right(25u32)) as u64; r8 = ((r8).wrapping_add(crate::simd_ops::ru64(rbx.wrapping_add(r15).wrapping_sub(0x8u64)))) as u64; r8 = (((r8) as i64).wrapping_mul((r9) as i64) as u64) as u64; rcx = r8; rcx = (((rcx) ^ (rdx))) as u64; rcx = (((rcx) as i64).wrapping_mul((r9) as i64) as u64) as u64; rax = rcx; rax = ((rax).wrapping_shr(47u32)) as u64; rcx = (((rcx) ^ (rax))) as u64; rcx = (((rcx) ^ (r8))) as u64; rcx = (((rcx) as i64).wrapping_mul((r9) as i64) as u64) as u64; rax = rcx; rax = ((rax).wrapping_shr(47u32)) as u64; rcx = (((rcx) ^ (rax))) as u64; rcx = (((rcx) as i64).wrapping_mul((r9) as i64) as u64) as u64; bb = 10; continue; }
            3 => { fl_l = rbx; fl_r = (0x4u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 4; continue; } r8 = (0x9ae16a3b2f90404fu64) as u64; r8 = ((r8.wrapping_add(rbx.wrapping_mul(2))) as u64) as u64; rax = (crate::simd_ops::ru32(r15)) as u32 as u64; rdx = (crate::simd_ops::ru32(rbx.wrapping_add(r15).wrapping_sub(0x4u64))) as u32 as u64; rcx = ((rbx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rcx = (((rcx) ^ (rdx))) as u64; rcx = (((rcx) as i64).wrapping_mul((r8) as i64) as u64) as u64; rax = rcx; rax = ((rax).wrapping_shr(47u32)) as u64; rcx = (((rcx) ^ (rax))) as u64; rcx = (((rcx) ^ (rdx))) as u64; rcx = (((rcx) as i64).wrapping_mul((r8) as i64) as u64) as u64; rax = rcx; rax = ((rax).wrapping_shr(47u32)) as u64; rcx = (((rcx) ^ (rax))) as u64; rcx = (((rcx) as i64).wrapping_mul((r8) as i64) as u64) as u64; bb = 10; continue; }
            4 => { fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } rax = rbx; rax = ((rax).wrapping_shr(1u32)) as u64; rdx = ((crate::simd_ops::ru8(rax.wrapping_add(r15))) as u32) as u32 as u64; rax = (0xe16a3b2f90404f00u64) as u64; rdx = (((rdx) as i64).wrapping_mul((rax) as i64) as u64) as u64; rax = ((crate::simd_ops::ru8(r15)) as u32) as u32 as u64; r8 = (0x9ae16a3b2f90404fu64) as u64; rax = (((rax) as i64).wrapping_mul((r8) as i64) as u64) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; rax = ((crate::simd_ops::ru8(rbx.wrapping_add(r15).wrapping_sub(0x1u64))) as u32) as u32 as u64; rax = ((rbx.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = ((rax as u32)) as u32 as u64; rcx = (0xc3a5c85c97cb3127u64) as u64; rax = (((rax) as i64).wrapping_mul((rcx) as i64) as u64) as u64; rdx = (((rdx) ^ (rax))) as u64; rcx = rdx; rcx = ((rcx).wrapping_shr(47u32)) as u64; rcx = (((rcx) ^ (rdx))) as u64; rcx = (((rcx) as i64).wrapping_mul((r8) as i64) as u64) as u64; bb = 10; continue; }
            5 => { rcx = (0x9ae16a3b2f90404fu64) as u64; bb = 10; continue; }
            6 => { r8 = (0x9ae16a3b2f90404fu64) as u64; r11 = ((r8.wrapping_add(rbx.wrapping_mul(2))) as u64) as u64; r12 = (0xb492b66fbe98f273u64) as u64; r9 = (crate::simd_ops::ru64(r15)) as u64; r9 = (((r9) as i64).wrapping_mul((r12) as i64) as u64) as u64; rdx = (crate::simd_ops::ru64(r15.wrapping_add(0x8u64))) as u64; r10 = r11; r10 = (((r10) as i64).wrapping_mul((crate::simd_ops::ru64(rbx.wrapping_add(r15).wrapping_sub(0x8u64))) as i64) as u64) as u64; rcx = (crate::simd_ops::ru64(rbx.wrapping_add(r15).wrapping_sub(0x10u64))) as u64; rcx = (((rcx) as i64).wrapping_mul((r8) as i64) as u64) as u64; r8 = ((r8).wrapping_add(rdx)) as u64; r8 = ((r8).rotate_right(18u32)) as u64; r8 = ((r8).wrapping_add(r10)) as u64; r8 = ((r8).wrapping_add(r9)) as u64; rax = ((rdx.wrapping_add(r9)) as u64) as u64; rax = ((rax).rotate_right(43u32)) as u64; r10 = ((r10).rotate_right(30u32)) as u64; rcx = ((rcx).wrapping_add(rax)) as u64; rcx = ((rcx).wrapping_add(r10)) as u64; rcx = (((rcx) ^ (r8))) as u64; rcx = (((rcx) as i64).wrapping_mul((r11) as i64) as u64) as u64; rax = rcx; rax = ((rax).wrapping_shr(47u32)) as u64; rcx = (((rcx) ^ (rax))) as u64; rcx = (((rcx) ^ (r8))) as u64; rcx = (((rcx) as i64).wrapping_mul((r11) as i64) as u64) as u64; rax = rcx; rax = ((rax).wrapping_shr(47u32)) as u64; rcx = (((rcx) ^ (rax))) as u64; rcx = (((rcx) as i64).wrapping_mul((r11) as i64) as u64) as u64; bb = 10; continue; }
            7 => { r13 = ((rbx.wrapping_add(r15)) as u64) as u64; fl_l = rbx; fl_r = (0x40u64) as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 8; continue; } r8 = (0x9ae16a3b2f90404fu64) as u64; r14 = ((r8.wrapping_add(rbx.wrapping_mul(2))) as u64) as u64; r11 = (crate::simd_ops::ru64(r13.wrapping_sub(0x18u64))) as u64; r10 = (crate::simd_ops::ru64(r15.wrapping_add(0x10u64))) as u64; r10 = (((r10) as i64).wrapping_mul((r8) as i64) as u64) as u64; rax = (crate::simd_ops::ru64(r15.wrapping_add(0x18u64))) as u64; r9 = ((rax.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rdi = r14; rdi = (((rdi) as i64).wrapping_mul((crate::simd_ops::ru64(r13.wrapping_sub(0x10u64))) as i64) as u64) as u64; rdx = (crate::simd_ops::ru64(r15)) as u64; rdx = (((rdx) as i64).wrapping_mul((r8) as i64) as u64) as u64; rdx = ((rdx).wrapping_add(crate::simd_ops::ru64(r13.wrapping_sub(0x8u64)))) as u64; rax = (crate::simd_ops::ru64(r15.wrapping_add(0x8u64))) as u64; rax = ((rax).rotate_right(30u32)) as u64; rax = ((rax).wrapping_add(r11)) as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rax = rdx; rax = ((rax).rotate_right(43u32)) as u64; rcx = ((rcx).wrapping_add(rax)) as u64; r8 = (crate::simd_ops::ru64(r13.wrapping_sub(0x20u64))) as u64; r8 = (((r8) ^ (rdx))) as u64; r8 = ((r8).wrapping_add(1)) as u64; r8 = ((r8).wrapping_add(r9)) as u64; rdx = ((r8.wrapping_add(rcx)) as u64) as u64; rdx = (((rdx) as i64).wrapping_mul((r14) as i64) as u64) as u64; rdx = ((rdx).swap_bytes()) as u64; rdx = ((rdx).wrapping_add(rdi)) as u64; rax = ((r9.wrapping_add(r10)) as u64) as u64; r9 = rax; r9 = ((r9).rotate_right(42u32)) as u64; r9 = ((r9).wrapping_add(r11)) as u64; rcx = ((rax.wrapping_add(r11)) as u64) as u64; rax = ((rdx.wrapping_add(r8)) as u64) as u64; rax = (((rax) as i64).wrapping_mul((r14) as i64) as u64) as u64; rax = ((rax).swap_bytes()) as u64; rax = ((rax).wrapping_add(rcx)) as u64; rax = ((rax).wrapping_add(r9)) as u64; rax = ((rax).wrapping_add(crate::simd_ops::ru64(r13.wrapping_sub(0x8u64)))) as u64; rax = (((rax) as i64).wrapping_mul((r14) as i64) as u64) as u64; rax = ((rax).swap_bytes()) as u64; rax = ((rax).wrapping_add(crate::simd_ops::ru64(r15.wrapping_add(0x8u64)))) as u64; rcx = ((rcx).wrapping_add(rax)) as u64; rcx = (((rcx) as i64).wrapping_mul((r14) as i64) as u64) as u64; rcx = ((rcx).wrapping_add(rdi)) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(r13.wrapping_sub(0x20u64)))) as u64; rax = rcx; rax = ((rax).wrapping_shr(47u32)) as u64; rcx = (((rcx) ^ (rax))) as u64; rcx = (((rcx) as i64).wrapping_mul((r14) as i64) as u64) as u64; fl_l = ((rcx).wrapping_add(r9)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; bb = 10; continue; }
            8 => { rdi = (crate::simd_ops::ru64(r13.wrapping_sub(0x28u64))) as u64; rbp = (crate::simd_ops::ru64(r13.wrapping_sub(0x38u64))) as u64; rbp = ((rbp).wrapping_add(crate::simd_ops::ru64(r13.wrapping_sub(0x10u64)))) as u64; r14 = (crate::simd_ops::ru64(r13.wrapping_sub(0x30u64))) as u64; r14 = ((r14).wrapping_add(rbx)) as u64; r14 = (((r14) ^ (crate::simd_ops::ru64(r13.wrapping_sub(0x18u64))))) as u64; rdx = (0x9ddfea08eb382d69u64) as u64; r14 = (((r14) as i64).wrapping_mul((rdx) as i64) as u64) as u64; rax = r14; rax = ((rax).wrapping_shr(47u32)) as u64; r14 = (((r14) ^ (rax))) as u64; r14 = (((r14) ^ (crate::simd_ops::ru64(r13.wrapping_sub(0x18u64))))) as u64; r14 = (((r14) as i64).wrapping_mul((rdx) as i64) as u64) as u64; rax = r14; rax = ((rax).wrapping_shr(47u32)) as u64; r14 = (((r14) ^ (rax))) as u64; r14 = (((r14) as i64).wrapping_mul((rdx) as i64) as u64) as u64; rax = (crate::simd_ops::ru64(r13.wrapping_sub(0x30u64))) as u64; r10 = (crate::simd_ops::ru64(r13.wrapping_sub(0x40u64))) as u64; r10 = ((r10).wrapping_add(rbx)) as u64; r9 = ((rdi.wrapping_add(r10)) as u64) as u64; r9 = ((r9).wrapping_add(r14)) as u64; r8 = r10; rax = ((rax).wrapping_add(crate::simd_ops::ru64(r13.wrapping_sub(0x38u64)))) as u64; r10 = ((r10).wrapping_add(rax)) as u64; r9 = ((r9).rotate_right(21u32)) as u64; rax = r10; rax = ((rax).rotate_right(44u32)) as u64; rax = ((rax).wrapping_add(r9)) as u64; rax = ((rax).wrapping_add(r8)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xc0u64), rax); r9 = ((rdi.wrapping_add(r10)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xb8u64), r9); r11 = (crate::simd_ops::ru64(r13.wrapping_sub(0x8u64))) as u64; rax = (crate::simd_ops::ru64(r13.wrapping_sub(0x10u64))) as u64; rcx = (crate::simd_ops::ru64(r13.wrapping_sub(0x18u64))) as u64; r10 = (crate::simd_ops::ru64(r13.wrapping_sub(0x20u64))) as u64; r12 = (0xb492b66fbe98f273u64) as u64; r10 = ((r10).wrapping_add(r12)) as u64; r10 = ((r10).wrapping_add(rbp)) as u64; r9 = ((r11.wrapping_add(r10)) as u64) as u64; r9 = ((r9).wrapping_add(rdi)) as u64; r13 = r10; rax = ((rax).wrapping_add(rcx)) as u64; r10 = ((r10).wrapping_add(rax)) as u64; r9 = ((r9).rotate_right(21u32)) as u64; rax = r10; rax = ((rax).rotate_right(44u32)) as u64; r9 = ((r9).wrapping_add(rax)) as u64; r13 = ((r13).wrapping_add(r9)) as u64; r8 = ((r11.wrapping_add(r10)) as u64) as u64; rdi = (((rdi) as i64).wrapping_mul((r12) as i64) as u64) as u64; rdi = ((rdi).wrapping_add(crate::simd_ops::ru64(r15))) as u64; rbx = ((rbx).wrapping_sub(1)) as u64; fl_l = (((rbx) & (0xffffffffffffffc0u64))) as u64; fl_r = fl_l; rbx = ((fl_l as u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xc8u64), rbx); rax = r15.wrapping_add(0x30); crate::simd_ops::wu64(rsp.wrapping_add(0xb0u64), rax); r15 = rdi; bb = 9; continue; }
            9 => { r11 = (crate::simd_ops::ru64(rax.wrapping_sub(0x28u64))) as u64; rsi = ((r11.wrapping_add(rbp)) as u64) as u64; rsi = ((rsi).wrapping_add(r15)) as u64; rsi = ((rsi).wrapping_add(crate::simd_ops::ru64(rsp.wrapping_add(0xb8u64)))) as u64; rdi = (crate::simd_ops::ru64(rax)) as u64; rbp = ((rbp).wrapping_add(rdi)) as u64; rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0xc0u64))) as u64; rbp = ((rbp).wrapping_add(rdx)) as u64; rsi = ((rsi).rotate_right(37u32)) as u64; rsi = (((rsi) as i64).wrapping_mul((r12) as i64) as u64) as u64; rsi = (((rsi) ^ (r13))) as u64; rbx = (crate::simd_ops::ru64(rax.wrapping_sub(0x8u64))) as u64; rbp = ((rbp).rotate_right(42u32)) as u64; rbp = (((rbp) as i64).wrapping_mul((r12) as i64) as u64) as u64; rbp = ((rbp).wrapping_add(rbx)) as u64; rbp = ((rbp).wrapping_add(crate::simd_ops::ru64(rsp.wrapping_add(0xb8u64)))) as u64; r15 = ((r14.wrapping_add(r8)) as u64) as u64; r15 = ((r15).rotate_right(33u32)) as u64; r15 = (((r15) as i64).wrapping_mul((r12) as i64) as u64) as u64; rcx = ((rsi.wrapping_add(r8)) as u64) as u64; rdx = (((rdx) as i64).wrapping_mul((r12) as i64) as u64) as u64; r9 = (crate::simd_ops::ru64(rax.wrapping_sub(0x18u64))) as u64; r10 = (crate::simd_ops::ru64(rax.wrapping_sub(0x20u64))) as u64; r8 = (crate::simd_ops::ru64(rax.wrapping_sub(0x30u64))) as u64; r8 = ((r8).wrapping_add(rdx)) as u64; rdx = ((r9.wrapping_add(r8)) as u64) as u64; rdx = ((rdx).wrapping_add(rcx)) as u64; rcx = r8; rax = ((r10.wrapping_add(r11)) as u64) as u64; r8 = ((r8).wrapping_add(rax)) as u64; rdx = ((rdx).rotate_right(21u32)) as u64; r11 = r8; r11 = ((r11).rotate_right(44u32)) as u64; r11 = ((r11).wrapping_add(rdx)) as u64; r11 = ((r11).wrapping_add(rcx)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xc0u64), r11); rax = ((r9.wrapping_add(r8)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xb8u64), rax); rdx = ((r10.wrapping_add(rbp)) as u64) as u64; r9 = ((r15.wrapping_add(r13)) as u64) as u64; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xb0u64))) as u64; r10 = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; r9 = ((r9).wrapping_add(crate::simd_ops::ru64(rax.wrapping_sub(0x10u64)))) as u64; r8 = ((r10.wrapping_add(r9)) as u64) as u64; r8 = ((r8).wrapping_add(rdx)) as u64; r13 = r9; rax = ((rdi.wrapping_add(rbx)) as u64) as u64; r9 = ((r9).wrapping_add(rax)) as u64; r8 = ((r8).rotate_right(21u32)) as u64; rax = r9; rax = ((rax).rotate_right(44u32)) as u64; r8 = ((r8).wrapping_add(rax)) as u64; r13 = ((r13).wrapping_add(r8)) as u64; r8 = ((r10.wrapping_add(r9)) as u64) as u64; r14 = rsi; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xb0u64))) as u64; rax = ((rax).wrapping_add(0x40u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xb0u64), rax); fl_l = (crate::simd_ops::ru64(rsp.wrapping_add(0xc8u64))) as u64; fl_r = (0x40u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xc8u64), ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64); if (((fl_l as u64)) != ((fl_r as u64))) { bb = 9; continue; } rcx = r13; rcx = (((rcx) ^ (r11))) as u64; r10 = (0x9ddfea08eb382d69u64) as u64; rcx = (((rcx) as i64).wrapping_mul((r10) as i64) as u64) as u64; rax = rcx; rax = ((rax).wrapping_shr(47u32)) as u64; rcx = (((rcx) ^ (rax))) as u64; r13 = (((r13) ^ (rcx))) as u64; r13 = (((r13) as i64).wrapping_mul((r10) as i64) as u64) as u64; rcx = r13; rcx = ((rcx).wrapping_shr(47u32)) as u64; rcx = (((rcx) ^ (r13))) as u64; rcx = (((rcx) as i64).wrapping_mul((r10) as i64) as u64) as u64; rcx = ((rcx).wrapping_add(r15)) as u64; rdx = r8; rdx = (((rdx) ^ (crate::simd_ops::ru64(rsp.wrapping_add(0xb8u64))))) as u64; rdx = (((rdx) as i64).wrapping_mul((r10) as i64) as u64) as u64; rax = rdx; rax = ((rax).wrapping_shr(47u32)) as u64; rdx = (((rdx) ^ (rax))) as u64; r8 = (((r8) ^ (rdx))) as u64; r8 = (((r8) as i64).wrapping_mul((r10) as i64) as u64) as u64; rax = r8; rax = ((rax).wrapping_shr(47u32)) as u64; rax = (((rax) ^ (r8))) as u64; rax = (((rax) as i64).wrapping_mul((r10) as i64) as u64) as u64; rdx = rbp; rdx = ((rdx).wrapping_shr(47u32)) as u64; rdx = (((rdx) ^ (rbp))) as u64; rdx = (((rdx) as i64).wrapping_mul((r12) as i64) as u64) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; rdx = ((rdx).wrapping_add(rsi)) as u64; rdx = (((rdx) ^ (rcx))) as u64; rdx = (((rdx) as i64).wrapping_mul((r10) as i64) as u64) as u64; rax = rdx; rax = ((rax).wrapping_shr(47u32)) as u64; rdx = (((rdx) ^ (rax))) as u64; rcx = (((rcx) ^ (rdx))) as u64; rcx = (((rcx) as i64).wrapping_mul((r10) as i64) as u64) as u64; rax = rcx; rax = ((rax).wrapping_shr(47u32)) as u64; rcx = (((rcx) ^ (rax))) as u64; rcx = (((rcx) as i64).wrapping_mul((r10) as i64) as u64) as u64; bb = 10; continue; }
            10 => { rax = (0xde5fb9d2630458e9u64) as u64; let prod = (rax as u128).wrapping_mul((rcx) as u128); rax = prod as u64; rdx = (prod >> 64) as u64; r9 = ((rdx.wrapping_add(rax)) as u64) as u64; r8 = rsp.wrapping_add(0x30); rdx = rsp.wrapping_add(0x20); rcx = P2; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; fl_l = (crate::simd_ops::ru8(rsp.wrapping_add(0x28u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 11; continue; } rbx = ((rdi.wrapping_add(rdi.wrapping_mul(2))) as u64) as u64; rbx = ((rbx).wrapping_shl(4u32)) as u64; rbx = ((rbx).wrapping_add(crate::simd_ops::ru64(P4))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xb0u64), rbx); crate::simd_ops::lk_and_u32(rbx, (0xff000004u32) as u32); crate::simd_ops::lk_or_u32(rbx, (0x4u32) as u32); crate::simd_ops::wu8(rbx.wrapping_add(0x3u64), (0x4u8) as u8); crate::simd_ops::wu32(rbx.wrapping_add(0x4u64), (0x0u32) as u32); crate::simd_ops::wu8(rbx.wrapping_add(0x8u64), (0x0u8) as u8); rdx = rsp.wrapping_add(0x30); rcx = rbx; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx.wrapping_add(0x28u64), (0x0u64) as u64); bb = 11; continue; }
            11 => { rax = ((rdi.wrapping_add(rdi.wrapping_mul(2))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rax = ((rax).wrapping_add(crate::simd_ops::ru64(P4))) as u64; rcx = P6; crate::simd_ops::wu64(rax.wrapping_add(0x28u64), rcx); rcx = rsp.wrapping_add(0x30); rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); rax = 0; fl_l = 0; fl_r = 0; rsp = ((rsp).wrapping_add(0x68u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1419e6b10(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x140466950;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut fl_k: u32 = 0;
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; r10 = (crate::simd_ops::ru64(rcx.wrapping_add(0x48u64))) as u64; rbx = ((rcx.wrapping_add(0x48u64)) as u64) as u64; r11 = (rdx) as u64; r8 = (r10) as u64; r9 = (0x3fffu32) as u32 as u64; rcx = (crate::simd_ops::ru64(r10.wrapping_add(0x8u64))) as u64; fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0x19u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 8; continue; } bb = 1; continue; }
            1 => { rdx = ((crate::simd_ops::ru16(rdx)) as u32) as u32 as u64; fl_l = ((((rdx as u16)) & ((r9 as u16)))) as u16 as u64; fl_r = fl_l; rdx = (rdx & !0xffffu64) | ((((fl_l as u16)) as u16 as u64)); fl_k = 3; bb = 2; continue; }
            2 => { rax = ((crate::simd_ops::ru16(rcx.wrapping_add(0x20u64))) as u32) as u32 as u64; rax = (rax & !0xffffu64) | ((((((rax as u16)) & ((r9 as u16)))) as u16 as u64)); fl_l = ((rax as u16)) as u16 as u64; fl_r = ((rdx as u16)) as u16 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { fl_k = 0; bb = 4; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(r11)) as u64; fl_l = (crate::simd_ops::ru64(rcx.wrapping_add(0x20u64))) as u64 as u64; fl_r = (rax) as u64 as u64; fl_k = 1; bb = 4; continue; }
            4 => { rax = (rax & !0xffu64) | (((if (match fl_k { 0 => (((fl_l as u16)) < ((fl_r as u16))), 1 => (((fl_l as u64)) < ((fl_r as u64))), _ => false }) { 1u8 } else { 0u8 }) as u8 as u64)); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { rcx = (crate::simd_ops::ru64(rcx.wrapping_add(0x10u64))) as u64; bb = 7; continue; }
            6 => { r8 = (rcx) as u64; rcx = (crate::simd_ops::ru64(rcx)) as u64; bb = 7; continue; }
            7 => { fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0x19u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { fl_k = 2; bb = 2; continue; } bb = 8; continue; }
            8 => { fl_l = (crate::simd_ops::ru8(r8.wrapping_add(0x19u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { fl_k = 2; bb = 14; continue; } bb = 9; continue; }
            9 => { rax = ((crate::simd_ops::ru16(r11)) as u32) as u32 as u64; rcx = ((crate::simd_ops::ru16(r8.wrapping_add(0x20u64))) as u32) as u32 as u64; rax = (rax & !0xffffu64) | ((((((rax as u16)) & ((r9 as u16)))) as u16 as u64)); rcx = (rcx & !0xffffu64) | ((((((rcx as u16)) & ((r9 as u16)))) as u16 as u64)); fl_l = ((rax as u16)) as u16 as u64; fl_r = ((rcx as u16)) as u16 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { fl_k = 0; bb = 11; continue; } bb = 10; continue; }
            10 => { rax = (crate::simd_ops::ru64(r8.wrapping_add(0x20u64))) as u64; fl_l = (crate::simd_ops::ru64(r11)) as u64 as u64; fl_r = (rax) as u64 as u64; fl_k = 1; bb = 11; continue; }
            11 => { rax = (rax & !0xffu64) | (((if (match fl_k { 0 => (((fl_l as u16)) < ((fl_r as u16))), 1 => (((fl_l as u64)) < ((fl_r as u64))), _ => false }) { 1u8 } else { 0u8 }) as u8 as u64)); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { fl_k = 4; bb = 14; continue; } bb = 12; continue; }
            12 => { fl_l = (r8) as u64 as u64; fl_r = (r10) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { fl_k = 1; bb = 14; continue; } bb = 13; continue; }
            13 => { rax = (crate::simd_ops::ru64(r8.wrapping_add(0x28u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rbx = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            14 => { r9 = (crate::simd_ops::ru64(r10)) as u64; r8 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), (r8) as u64); rax = (r9) as u64; fl_l = (r9) as u64 as u64; fl_r = (r10) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 28; continue; } bb = 15; continue; }
            15 => { fl_k = 1; bb = 16; continue; }
            16 => { fl_l = (crate::simd_ops::ru64(rax.wrapping_add(0x28u64))) as u64 as u64; fl_r = (r8) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 29; continue; } bb = 17; continue; }
            17 => { rcx = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0x19u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 23; continue; } bb = 18; continue; }
            18 => { rcx = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0x19u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { fl_k = 2; bb = 22; continue; } bb = 19; continue; }
            19 => { bb = 20; continue; }
            20 => { fl_l = (rax) as u64 as u64; fl_r = (crate::simd_ops::ru64(rcx.wrapping_add(0x10u64))) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { fl_k = 1; bb = 22; continue; } bb = 21; continue; }
            21 => { rax = (rcx) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0x19u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 20; continue; } fl_k = 2; bb = 22; continue; }
            22 => { rax = (rcx) as u64; bb = 26; continue; }
            23 => { rdx = (crate::simd_ops::ru64(rcx)) as u64; rax = (rcx) as u64; fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(0x19u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { fl_k = 2; bb = 26; continue; } bb = 24; continue; }
            24 => { bb = 25; continue; }
            25 => { rcx = (crate::simd_ops::ru64(rdx)) as u64; rax = (rdx) as u64; rdx = (rcx) as u64; fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0x19u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 25; continue; } fl_k = 2; bb = 26; continue; }
            26 => { fl_l = (rax) as u64 as u64; fl_r = (r10) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { fl_k = 1; bb = 16; continue; } bb = 27; continue; }
            27 => { crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), (r8) as u64); bb = 28; continue; }
            28 => { r9 = ((rsp.wrapping_add(0x40u64)) as u64) as u64; r8 = (r11) as u64; rdx = ((rsp.wrapping_add(0x20u64)) as u64) as u64; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rbx = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            29 => { fl_l = ((r8).wrapping_add(1)) as u64 as u64; r8 = ((fl_l as u64)) as u64; rax = (r9) as u64; fl_k = 5; bb = 16; continue; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141aa42f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x140466950;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut fl_k: u32 = 0;
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; r10 = (crate::simd_ops::ru64(rcx.wrapping_add(0x48u64))) as u64; rbx = ((rcx.wrapping_add(0x48u64)) as u64) as u64; r11 = (rdx) as u64; r8 = (r10) as u64; r9 = (0x3fffu32) as u32 as u64; rcx = (crate::simd_ops::ru64(r10.wrapping_add(0x8u64))) as u64; fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0x19u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 8; continue; } bb = 1; continue; }
            1 => { rdx = ((crate::simd_ops::ru16(rdx)) as u32) as u32 as u64; fl_l = ((((rdx as u16)) & ((r9 as u16)))) as u16 as u64; fl_r = fl_l; rdx = (rdx & !0xffffu64) | ((((fl_l as u16)) as u16 as u64)); fl_k = 3; bb = 2; continue; }
            2 => { rax = ((crate::simd_ops::ru16(rcx.wrapping_add(0x20u64))) as u32) as u32 as u64; rax = (rax & !0xffffu64) | ((((((rax as u16)) & ((r9 as u16)))) as u16 as u64)); fl_l = ((rax as u16)) as u16 as u64; fl_r = ((rdx as u16)) as u16 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { fl_k = 0; bb = 4; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(r11)) as u64; fl_l = (crate::simd_ops::ru64(rcx.wrapping_add(0x20u64))) as u64 as u64; fl_r = (rax) as u64 as u64; fl_k = 1; bb = 4; continue; }
            4 => { rax = (rax & !0xffu64) | (((if (match fl_k { 0 => (((fl_l as u16)) < ((fl_r as u16))), 1 => (((fl_l as u64)) < ((fl_r as u64))), _ => false }) { 1u8 } else { 0u8 }) as u8 as u64)); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { rcx = (crate::simd_ops::ru64(rcx.wrapping_add(0x10u64))) as u64; bb = 7; continue; }
            6 => { r8 = (rcx) as u64; rcx = (crate::simd_ops::ru64(rcx)) as u64; bb = 7; continue; }
            7 => { fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0x19u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { fl_k = 2; bb = 2; continue; } bb = 8; continue; }
            8 => { fl_l = (crate::simd_ops::ru8(r8.wrapping_add(0x19u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { fl_k = 2; bb = 14; continue; } bb = 9; continue; }
            9 => { rax = ((crate::simd_ops::ru16(r11)) as u32) as u32 as u64; rcx = ((crate::simd_ops::ru16(r8.wrapping_add(0x20u64))) as u32) as u32 as u64; rax = (rax & !0xffffu64) | ((((((rax as u16)) & ((r9 as u16)))) as u16 as u64)); rcx = (rcx & !0xffffu64) | ((((((rcx as u16)) & ((r9 as u16)))) as u16 as u64)); fl_l = ((rax as u16)) as u16 as u64; fl_r = ((rcx as u16)) as u16 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { fl_k = 0; bb = 11; continue; } bb = 10; continue; }
            10 => { rax = (crate::simd_ops::ru64(r8.wrapping_add(0x20u64))) as u64; fl_l = (crate::simd_ops::ru64(r11)) as u64 as u64; fl_r = (rax) as u64 as u64; fl_k = 1; bb = 11; continue; }
            11 => { rax = (rax & !0xffu64) | (((if (match fl_k { 0 => (((fl_l as u16)) < ((fl_r as u16))), 1 => (((fl_l as u64)) < ((fl_r as u64))), _ => false }) { 1u8 } else { 0u8 }) as u8 as u64)); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { fl_k = 4; bb = 14; continue; } bb = 12; continue; }
            12 => { fl_l = (r8) as u64 as u64; fl_r = (r10) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { fl_k = 1; bb = 14; continue; } bb = 13; continue; }
            13 => { rax = (crate::simd_ops::ru64(r8.wrapping_add(0x28u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rbx = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            14 => { r9 = (crate::simd_ops::ru64(r10)) as u64; r8 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), (r8) as u64); rax = (r9) as u64; fl_l = (r9) as u64 as u64; fl_r = (r10) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 28; continue; } bb = 15; continue; }
            15 => { fl_k = 1; bb = 16; continue; }
            16 => { fl_l = (crate::simd_ops::ru64(rax.wrapping_add(0x28u64))) as u64 as u64; fl_r = (r8) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 29; continue; } bb = 17; continue; }
            17 => { rcx = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0x19u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 23; continue; } bb = 18; continue; }
            18 => { rcx = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0x19u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { fl_k = 2; bb = 22; continue; } bb = 19; continue; }
            19 => { bb = 20; continue; }
            20 => { fl_l = (rax) as u64 as u64; fl_r = (crate::simd_ops::ru64(rcx.wrapping_add(0x10u64))) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { fl_k = 1; bb = 22; continue; } bb = 21; continue; }
            21 => { rax = (rcx) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0x19u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 20; continue; } fl_k = 2; bb = 22; continue; }
            22 => { rax = (rcx) as u64; bb = 26; continue; }
            23 => { rdx = (crate::simd_ops::ru64(rcx)) as u64; rax = (rcx) as u64; fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(0x19u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { fl_k = 2; bb = 26; continue; } bb = 24; continue; }
            24 => { bb = 25; continue; }
            25 => { rcx = (crate::simd_ops::ru64(rdx)) as u64; rax = (rdx) as u64; rdx = (rcx) as u64; fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0x19u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 25; continue; } fl_k = 2; bb = 26; continue; }
            26 => { fl_l = (rax) as u64 as u64; fl_r = (r10) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { fl_k = 1; bb = 16; continue; } bb = 27; continue; }
            27 => { crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), (r8) as u64); bb = 28; continue; }
            28 => { r9 = ((rsp.wrapping_add(0x40u64)) as u64) as u64; r8 = (r11) as u64; rdx = ((rsp.wrapping_add(0x20u64)) as u64) as u64; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rbx = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            29 => { fl_l = ((r8).wrapping_add(1)) as u64 as u64; r8 = ((fl_l as u64)) as u64; rax = (r9) as u64; fl_k = 5; bb = 16; continue; }
            _ => { return rax; }
        }
    }
}
