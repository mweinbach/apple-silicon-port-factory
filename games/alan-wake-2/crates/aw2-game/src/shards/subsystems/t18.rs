#![allow(unused_imports, unused_variables, unused_mut, unused_assignments, non_snake_case, dead_code, clippy::all, unreachable_code, unused_parens)]

#[inline(never)]
pub unsafe extern "win64" fn tpl_22540<const P0: u64, const P1: u64, const P2: u64, const P3: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
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
    rdx = 0xc8;
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
pub unsafe extern "win64" fn tpl_22563<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = rdx; rbx = rcx; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 1; continue; } fl_l = (crate::simd_ops::lk_add_u32(rdx.wrapping_add(P0), (1) as u32)) as u32 as u64; bb = 1; continue; }
            1 => { rsi = (crate::simd_ops::ru64(rcx)) as u64; fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } rcx = (crate::simd_ops::ru32(rsi.wrapping_add(P0))) as u32 as u64; rax = ((rcx.wrapping_add(P1)) as u32) as u32 as u64; crate::simd_ops::wu32(rsi.wrapping_add(P0), ((rax as u32)) as u32); fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 2; continue; } rcx = rsi; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(P3)) as u64; rdx = rsi; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(rax.wrapping_add(P4))) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 2; continue; }
            2 => { crate::simd_ops::wu64(rbx, rdi); rax = rbx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            3 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rax = rbx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu64(rcx, rdi); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_22564<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rax = (crate::simd_ops::ru64(rdx)) as u64;
    rsi = rdx;
    rdi = rcx;
    fl_l = rax; fl_r = rax;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { fl_l = (crate::simd_ops::lk_add_u32(rax.wrapping_add(P0), (1) as u32)) as u32 as u64; }
    rbx = (crate::simd_ops::ru64(rcx)) as u64;
    fl_l = rbx; fl_r = rbx;
    if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { rax = (crate::simd_ops::ru64(rsi)) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; crate::simd_ops::wu64(rdi, rax); rax = rdi; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    rcx = (crate::simd_ops::ru32(rbx.wrapping_add(P0))) as u32 as u64;
    rax = ((rcx.wrapping_add(P1)) as u32) as u32 as u64;
    crate::simd_ops::wu32(rbx.wrapping_add(P0), ((rax as u32)) as u32);
    fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x1;
    if !((((fl_l as u32)) != ((fl_r as u32)))) { rcx = rbx; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(P3)) as u64; rdx = rbx; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(rax.wrapping_add(P4))) as u64, rax, rcx, rdx, r8, r9, rsp); }
    rax = (crate::simd_ops::ru64(rsi)) as u64;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64;
    crate::simd_ops::wu64(rdi, rax);
    rax = rdi;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_22569<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
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
    rdx = 0x220;
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
pub unsafe extern "win64" fn tpl_22570<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
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
    rdx = 0x118;
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
pub unsafe extern "win64" fn tpl_22574<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rcx;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P1))) as u64;
    fl_l = rcx; fl_r = rcx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rax = (crate::simd_ops::ru32(rbx.wrapping_add(P2))) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(P3), ((rax as u32)) as u32); }
    rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P4))) as u64;
    fl_l = rcx; fl_r = rcx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rax = (crate::simd_ops::ru32(rbx.wrapping_add(P2))) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(P3), ((rax as u32)) as u32); }
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_22575<const P0: u64, const P1: u64, const P2: u64, const P3: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    crate::simd_ops::forward_incoming_to_outgoing();
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    fl_l = rdx; fl_r = rdx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) != 0)) { rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rdx = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64; }
    crate::simd_ops::wu32(rcx.wrapping_add(P2), (0x17u32) as u32);
    crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8(P3, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_22604<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 19>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x80u64)) as u64; fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(0x20u64))) as u32 as u64; fl_r = 0x0; rax = 0xffff; r12 = (crate::simd_ops::ru64(rcx.wrapping_add(0x68u64))) as u64; rbx = rcx; xmm0 = 0u128; crate::simd_ops::wu32(rsp.wrapping_add(0x70u64), ((xmm0 as u32)) as u32); crate::simd_ops::wu16(rsp.wrapping_add(0x68u64), ((rax as u16)) as u16); crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), (0x0u64) as u64); if (((fl_l as u32)) == ((fl_r as u32))) { bb = 8; continue; } bb = 1; continue; }
            1 => { rdx = rsp.wrapping_add(0x60); rcx = rbx.wrapping_add(0x10); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm2 = ((crate::simd_ops::ru32(rsp.wrapping_add(0x70u64))) as u32) as u128; rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; r9 = rbx.wrapping_add(0x30); rdx = rsp.wrapping_add(0x60); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = r12; fl_r = r12; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } rdi = (crate::simd_ops::ru64(rbx.wrapping_add(0x68u64))) as u64; rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x14u64))) as u32 as u64; fl_l = (crate::simd_ops::ru32(rdi.wrapping_add(0x10u64))) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 3; continue; } rdx = (crate::simd_ops::ru32(rdi.wrapping_add(0x8u64))) as u32 as u64; r8 = 0; fl_l = 0; fl_r = 0; rcx = (crate::simd_ops::ru64(rdi)) as u64; rax = ((rdx as u32)) as u32 as u64; rax = ((rax).wrapping_add(rax)) as u64; rbp = (crate::simd_ops::ru64(rcx.wrapping_add(rax.wrapping_mul(8)).wrapping_add(0x18u64))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } fl_l = rbp; fl_r = rbp; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } r8 = (crate::simd_ops::ru32(rdi.wrapping_add(0x10u64))) as u32 as u64; rdx = rbp; r8 = ((r8).wrapping_shl(4u32)) as u64; rcx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru64(P4)) as u64; rdx = rbp; rax = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm((crate::simd_ops::ru64(rax.wrapping_add(0x60u64))) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 2; continue; }
            2 => { rcx = (crate::simd_ops::ru32(rdi.wrapping_add(0x8u64))) as u32 as u64; rax = (crate::simd_ops::ru64(rdi)) as u64; rcx = ((rcx).wrapping_add(0x2u64)) as u64; rcx = ((rcx).wrapping_add(rcx)) as u64; rcx = (crate::simd_ops::ru32(rax.wrapping_add(rcx.wrapping_mul(8)))) as u32 as u64; fl_l = (((rcx as u32)).wrapping_shr(4u32)) as u32 as u64; rcx = ((fl_l as u32)) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x14u64), ((rcx as u32)) as u32); bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru32(rdi.wrapping_add(0x8u64))) as u32 as u64; rax = (crate::simd_ops::ru64(rdi)) as u64; rcx = ((rcx).wrapping_add(rcx)) as u64; rdx = (crate::simd_ops::ru32(rdi.wrapping_add(0x10u64))) as u32 as u64; xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x60u64))) as u128; rdx = ((rdx).wrapping_add(rdx)) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x18u64))) as u64; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128); crate::simd_ops::wu128(rcx.wrapping_add(rdx.wrapping_mul(8)), (xmm0) as u128); fl_l = (crate::simd_ops::lk_add_u32(rdi.wrapping_add(0x10u64), (1) as u32)) as u32 as u64; bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; xmm0 = ((crate::simd_ops::ru32(rax.wrapping_add(0x50u64))) as u32) as u128; fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rax.wrapping_add(0x14u64))) as u32 as u64; if (f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 8; continue; } fl_l = (crate::simd_ops::ru8(rax.wrapping_add(0x54u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 5; continue; } xmm0 = (crate::simd_ops::ru128(rax.wrapping_add(0x58u64))) as u128; rdx = rsp.wrapping_add(0x30); rcx = rbx; crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm0) as u128); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 9; continue; } bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; fl_l = (crate::simd_ops::ru8(rax.wrapping_add(0x55u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 6; continue; } xmm0 = (crate::simd_ops::ru128(rax.wrapping_add(0x68u64))) as u128; rdx = rsp.wrapping_add(0x40); rcx = rbx; crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm0) as u128); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 9; continue; } bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; fl_l = (crate::simd_ops::ru8(rax.wrapping_add(0x56u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 7; continue; } xmm0 = (crate::simd_ops::ru128(rax.wrapping_add(0x78u64))) as u128; rdx = rsp.wrapping_add(0x50); rcx = rbx; crate::simd_ops::wu128(rsp.wrapping_add(0x50u64), (xmm0) as u128); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 9; continue; } bb = 7; continue; }
            7 => { fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x20u64))) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 1; continue; } bb = 8; continue; }
            8 => { rax = 0; fl_l = 0; fl_r = 0; bb = 9; continue; }
            9 => { r11 = rsp.wrapping_add(0x80); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x20u64))) as u64; rbp = (crate::simd_ops::ru64(r11.wrapping_add(0x28u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x30u64))) as u64; rdi = (crate::simd_ops::ru64(r11.wrapping_add(0x38u64))) as u64; rsp = r11; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            10 => { rax = 0x3; bb = 9; continue; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_22608<const P0: u64, const P1: u64, const P2: u64, const P3: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rcx;
    rdx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    rcx = (crate::simd_ops::ru64(rcx)) as u64;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rdx = (crate::simd_ops::ru64(rbx)) as u64;
    fl_l = rdx; fl_r = rdx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rcx = (crate::simd_ops::ru64(P2)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(rax.wrapping_add(P3))) as u64, rax, rcx, rdx, r8, r9, rsp); }
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_22621<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rbx = rdx; r10 = rcx; fl_l = ((r9 as u32)) as u32 as u64; fl_r = 0xffffffff; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 2; continue; } xmm0 = ((crate::simd_ops::ru32(rsp.wrapping_add(0x70u64))) as u32) as u128; xmm1 = ((crate::simd_ops::ru32(rsp.wrapping_add(0x68u64))) as u32) as u128; r9 = (crate::simd_ops::ru32(rsp.wrapping_add(0x60u64))) as u32 as u64; rdx = rsp.wrapping_add(0x58); crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((xmm0 as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((xmm1 as u32)) as u32); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = (crate::simd_ops::ru8(rsp.wrapping_add(0x58u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 3; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu8(rbx, (0x1u8) as u8); rax = rbx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            2 => { rcx = (crate::simd_ops::ru32(rax.wrapping_add(P2))) as u32 as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(P3))) as u64; rcx = ((rcx).wrapping_add(rcx)) as u64; rdi = (crate::simd_ops::ru32(rsp.wrapping_add(0x60u64))) as u32 as u64; rdx = (crate::simd_ops::ru64(rax.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(P4))) as u64; r11 = ((rdi.wrapping_add(rdi.wrapping_mul(8))) as u64) as u64; fl_l = ((r9 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rdx.wrapping_add(r11.wrapping_mul(4)).wrapping_add(0x1cu64))) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 1; continue; } xmm0 = ((crate::simd_ops::ru32(rsp.wrapping_add(0x68u64))) as u32) as u128; xmm1 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) + f32::from_bits(crate::simd_ops::ru32(rdx.wrapping_add(r11.wrapping_mul(4)).wrapping_add(0xcu64)))).to_bits() as u128); rax = ((r9 as u32)) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; fl_l = ((xmm1 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rdx.wrapping_add(rcx.wrapping_mul(4)).wrapping_add(P5))) as u32 as u64; r8 = ((rdx.wrapping_add(rcx.wrapping_mul(4))) as u64) as u64; if (f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { bb = 1; continue; } xmm0 = ((crate::simd_ops::ru32(rsp.wrapping_add(0x70u64))) as u32) as u128; crate::simd_ops::wu32(r8.wrapping_add(P6), ((xmm0 as u32)) as u32); crate::simd_ops::wu32(r8.wrapping_add(P5), ((xmm1 as u32)) as u32); crate::simd_ops::wu32(r8.wrapping_add(P7), ((rdi as u32)) as u32); rdx = rsp.wrapping_add(0x58); rcx = (crate::simd_ops::ru64(r10.wrapping_add(P0))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P8, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = (crate::simd_ops::ru8(rsp.wrapping_add(0x58u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 1; continue; } bb = 3; continue; }
            3 => { crate::simd_ops::wu8(rbx, (0x0u8) as u8); rax = rbx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_22631<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x50u64)) as u64; rbx = rdx; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), (0x0u64) as u64); rsi = rcx; crate::simd_ops::wu32(rsp.wrapping_add(0x48u64), (0xffffu32) as u32); rdx = rsp.wrapping_add(0x30); rdi = ((r9 as u32)) as u32 as u64; rcx = rsp.wrapping_add(0x40); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = (crate::simd_ops::ru8(rax)) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 2; continue; } rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x48u64))) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rdx = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64; fl_l = (crate::simd_ops::ru32(rdx.wrapping_add(rcx.wrapping_mul(4)))) as u32 as u64; fl_r = 0xffffffff; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 2; continue; } fl_l = (crate::simd_ops::ru32(rdx.wrapping_add(rcx.wrapping_mul(4)).wrapping_add(P2))) as u32 as u64; fl_r = 0xffff; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 2; continue; } rcx = (crate::simd_ops::ru64(rsi.wrapping_add(P3))) as u64; r9 = rsp.wrapping_add(0x38); r8 = rsp.wrapping_add(0x40); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), (0x0u64) as u64); rdx = rsp.wrapping_add(0x30); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = (crate::simd_ops::ru8(rax)) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 1; continue; } rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rdx = (crate::simd_ops::ru32(rsp.wrapping_add(0x48u64))) as u32 as u64; rax = (crate::simd_ops::ru32(rcx)) as u32 as u64; rcx = ((rcx).wrapping_add(rax)) as u64; r9 = (crate::simd_ops::ru32(rcx.wrapping_add(rdx.wrapping_mul(4)))) as u32 as u64; fl_l = ((r9 as u32)) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 2; continue; } xmm0 = 0u128; crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((xmm0 as u32)) as u32); r8 = rsp.wrapping_add(0x40); crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rdi as u32)) as u32); rdx = rsp.wrapping_add(0x30); rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = (crate::simd_ops::ru8(rsp.wrapping_add(0x30u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 2; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu8(rbx, (0x0u8) as u8); rax = rbx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rsp = ((rsp).wrapping_add(0x50u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            2 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rax = rbx; crate::simd_ops::wu8(rbx, (0x1u8) as u8); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rsp = ((rsp).wrapping_add(0x50u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_22645<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
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
    rdx = 0xf0;
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
pub unsafe extern "win64" fn tpl_22649<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 61>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x1e0u64)) as u64; rbx = r9; rbp = r8; rsi = rdx; rdi = rcx; rcx = rsp.wrapping_add(0x40); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = rsp.wrapping_add(0x40); rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm0 = ((crate::simd_ops::ru32(rsp.wrapping_add(0x220u64))) as u32) as u128; crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((xmm0 as u32)) as u32); rax = rsp.wrapping_add(0x40); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x210u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); r9 = rbx; r8 = (crate::simd_ops::ru64(rdi.wrapping_add(P2))) as u64; rdx = rbp; rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = (crate::simd_ops::ru8(rsi)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 1; continue; } rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x218u64))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rax as u32)) as u32); r9 = rsp.wrapping_add(0x40); r8 = rbp; rdx = rsp.wrapping_add(0x1f0); rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = ((crate::simd_ops::ru8(rax)) as u32) as u32 as u64; crate::simd_ops::wu8(rsi, ((rcx as u8)) as u8); bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru32(rdi.wrapping_add(P5))) as u32 as u64; rax = (((rax as u32)).wrapping_shr(13u32)) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 2; continue; } rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 2; continue; }
            2 => { rcx = rsp.wrapping_add(0x40); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = rsi; r11 = rsp.wrapping_add(0x1e0); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x18u64))) as u64; rbp = (crate::simd_ops::ru64(r11.wrapping_add(0x20u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x28u64))) as u64; rsp = r11; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_22671<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; rbx = rcx; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 2; continue; } fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 1; continue; } rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = (crate::simd_ops::ru32(rcx.wrapping_add(P1))) as u32 as u64; rdx = (((rdx as u32)).wrapping_add(1)) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rax.wrapping_add(P2))) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rcx = ((rcx).wrapping_add(0x8u64)) as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rcx = rbx; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(P4, rax, rcx, rdx, r8, r9, rsp); }
            2 => { rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_22683<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
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
    rdx = 0x20;
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
pub unsafe extern "win64" fn tpl_22707<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64, const P9: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 39>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        core::sync::atomic::fence(core::sync::atomic::Ordering::SeqCst);
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rbp = rsp.wrapping_sub(0x1f); rsp = ((rsp).wrapping_sub(0x100u64)) as u64; r13 = r9; rbx = r8; r12 = rdx; r15 = rcx; r8 = (crate::simd_ops::ru64(r8.wrapping_add(0x38u64))) as u64; rdx = rsp.wrapping_add(0x20); rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdi = (crate::simd_ops::ru64(rbx.wrapping_add(0x38u64))) as u64; r14 = rbx.wrapping_add(0x214); rax = (crate::simd_ops::ru32(r13.wrapping_add(0x3cu64))) as u32 as u64; fl_l = 0; fl_r = ((rax as u32)) as u32 as u64; rax = (0u32.wrapping_sub(fl_r as u32)) as u32 as u64; rbx = (if (((fl_l as u32)) < ((fl_r as u32))) { 0xffffffffu32 } else { 0u32 }) as u32 as u64; rbx = ((((rbx as u32)) & (0x5u32))) as u32 as u64; rcx = rsp.wrapping_add(0x30); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = P2; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); rdx = rdi; rcx = rsp.wrapping_add(0x30); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = r15.wrapping_add(0x20); crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rax); crate::simd_ops::wu32(rbp.wrapping_add(0x3u64), ((rbx as u32)) as u32); crate::simd_ops::wu32(rbp.wrapping_add(0x7u64), (0x1u32) as u32); rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r8 = r13.wrapping_add(0x10); rdx = r14; rcx = rsp.wrapping_add(0x30); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 1; continue; } fl_l = (crate::simd_ops::atomic_add_u32(rax, 1)) as u32 as u64; bb = 1; continue; }
            1 => { rsi = 0xffffffff; rdx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x59u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } rax = ((rsi as u32)) as u32 as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rdx, xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 2; continue; } rcx = (crate::simd_ops::ru64(P6)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm((crate::simd_ops::ru64(rax.wrapping_add(0x60u64))) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x59u64), rax); rax = ((crate::simd_ops::ru16(rsp.wrapping_add(0x28u64))) as u32) as u32 as u64; crate::simd_ops::wu16(rbp.wrapping_sub(0x51u64), ((rax as u16)) as u16); crate::simd_ops::wu32(rbp.wrapping_sub(0x71u64), (0x6u32) as u32); rcx = (crate::simd_ops::ru32(rbp.wrapping_add(0xbu64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 7; continue; } rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x68u64))) as u32 as u64; fl_r = 0x7fffffff; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 6; continue; } crate::simd_ops::wu32(rbp.wrapping_add(0xbu64), (0x2u32) as u32); rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x7fu64))) as u64; xmm0 = ((crate::simd_ops::ru32(rbp.wrapping_add(0xfu64))) as u32) as u128; crate::simd_ops::wu32(rax, ((xmm0 as u32)) as u32); bb = 3; continue; }
            3 => { crate::simd_ops::wu8(r12, (0x0u8) as u8); rcx = rsp.wrapping_add(0x30); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 4; continue; }
            4 => { rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } let xa_src: u32 = (rsi as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rdx, xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rsi = (xa_old) as u32 as u64; rax = ((rsi.wrapping_sub(0x1u64)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 5; continue; } rcx = (crate::simd_ops::ru64(P6)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm((crate::simd_ops::ru64(rax.wrapping_add(0x60u64))) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 5; continue; }
            5 => { rax = r12; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x148u64))) as u64; rsp = ((rsp).wrapping_add(0x100u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            6 => { rdi = (crate::simd_ops::ru64(rbx.wrapping_add(0xa0u64))) as u64; xmm0 = ((crate::simd_ops::ru32(rbp.wrapping_sub(0x69u64))) as u32) as u128; crate::simd_ops::wu32(rbp.wrapping_add(0x5fu64), ((xmm0 as u32)) as u32); xmm1 = ((crate::simd_ops::ru32(rbp.wrapping_sub(0x65u64))) as u32) as u128; crate::simd_ops::wu32(rbp.wrapping_add(0x63u64), ((xmm1 as u32)) as u32); r8 = rbp.wrapping_sub(0x49); rdx = rbp.wrapping_add(0x5f); rcx = rbx.wrapping_add(0x58); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P8, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm0 = ((crate::simd_ops::ru32(rbp.wrapping_sub(0x39u64))) as u32) as u128; crate::simd_ops::wu32(rbp.wrapping_add(0x6fu64), ((xmm0 as u32)) as u32); xmm1 = ((crate::simd_ops::ru32(rbp.wrapping_sub(0x35u64))) as u32) as u128; crate::simd_ops::wu32(rbp.wrapping_add(0x73u64), ((xmm1 as u32)) as u32); r8 = rbp.wrapping_sub(0x19); rdx = rbp.wrapping_add(0x6f); rcx = rbx.wrapping_add(0x58); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P8, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = rdi; rcx = rsp.wrapping_add(0x30); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P9, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru32(rbp.wrapping_add(0xbu64))) as u32 as u64; bb = 7; continue; }
            7 => { rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x7fu64))) as u64; xmm0 = ((crate::simd_ops::ru32(rbp.wrapping_add(0xfu64))) as u32) as u128; crate::simd_ops::wu32(rax, ((xmm0 as u32)) as u32); fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x9; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 3; continue; } crate::simd_ops::wu32(r13.wrapping_add(0x38u64), (0x4u32) as u32); crate::simd_ops::wu8(r12, (0x1u8) as u8); rcx = rsp.wrapping_add(0x30); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 4; continue; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_22714<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
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
    rdx = 0x100;
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
pub unsafe extern "win64" fn tpl_22727<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 39>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rbp = rsp.wrapping_sub(0x20); rsp = ((rsp).wrapping_sub(0x120u64)) as u64; rbx = r9; r14 = r8; rdi = rdx; rdx = (crate::simd_ops::ru64(r9)) as u64; rsi = (crate::simd_ops::ru64(rdx.wrapping_add(0xa0u64))) as u64; fl_l = (crate::simd_ops::ru64(rsi.wrapping_add(0x120u64))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } crate::simd_ops::wu8(rdi, (0x0u8) as u8); bb = 15; continue; }
            1 => { r9 = r14; r8 = rsi; rcx = rbp; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x60u64))) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x60u64), rax); crate::simd_ops::wu64(rbp.wrapping_sub(0x58u64), r14); crate::simd_ops::wu8(rbp.wrapping_sub(0x50u64), (0x0u8) as u8); xmm0 = (crate::simd_ops::ru128(rbx.wrapping_add(0x2cu64))) as u128; crate::simd_ops::wu128(rbp.wrapping_sub(0x4cu64), (xmm0) as u128); xmm1 = ((crate::simd_ops::ru64(rbx.wrapping_add(0x3cu64))) as u64) as u128; crate::simd_ops::wu64(rbp.wrapping_sub(0x3cu64), ((xmm1 as u64)) as u64); r14 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rbp.wrapping_sub(0x30u64), r14); rax = 0xffff; crate::simd_ops::wu16(rbp.wrapping_sub(0x28u64), ((rax as u16)) as u16); crate::simd_ops::wu64(rbp.wrapping_sub(0x20u64), r14); crate::simd_ops::wu16(rbp.wrapping_sub(0x18u64), ((rax as u16)) as u16); crate::simd_ops::wu64(rbp.wrapping_sub(0x10u64), r14); crate::simd_ops::wu16(rbp.wrapping_sub(0x8u64), ((rax as u16)) as u16); crate::simd_ops::wu16(rbp.wrapping_sub(0x34u64), (0x101u16) as u16); crate::simd_ops::wu8(rbp.wrapping_sub(0x32u64), (0x1u8) as u8); rax = (crate::simd_ops::ru64(rbp)) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(0x78u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rcx); rax = rbp.wrapping_sub(0x60); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax); rdx = (crate::simd_ops::ru64(rbp.wrapping_add(0x8u64))) as u64; rcx = rsp.wrapping_add(0x30); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r8 = rbx.wrapping_add(0x2c); rdx = (crate::simd_ops::ru64(rbp.wrapping_add(0x8u64))) as u64; rcx = rsp.wrapping_add(0x50); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rbp.wrapping_sub(0x78u64), r14); rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; fl_l = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; fl_r = r14; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 12; continue; } fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } rbx = ((r14 as u32)) as u32 as u64; rcx = (crate::simd_ops::ru64(rsi.wrapping_add(0x120u64))) as u64; fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(0x18u64))) as u32 as u64; fl_r = ((r14 as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 10; continue; } bb = 2; continue; }
            2 => { rax = ((rbx as u32)) as u32 as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rax = ((rax).wrapping_add(crate::simd_ops::ru64(rcx.wrapping_add(0x10u64)))) as u64; rcx = (crate::simd_ops::ru64(rax)) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x70u64), rcx); rax = ((crate::simd_ops::ru16(rax.wrapping_add(0x8u64))) as u32) as u32 as u64; crate::simd_ops::wu16(rbp.wrapping_sub(0x68u64), ((rax as u16)) as u16); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; rdx = (crate::simd_ops::ru32(rcx.wrapping_add(0x38u64))) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rax.wrapping_add(0x14u64))) as u32 as u64; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 9; continue; } fl_l = ((rdx as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rax.wrapping_add(0x1cu64))) as u32 as u64; if ((((fl_l as u32)) as i32) > (((fl_r as u32)) as i32)) { bb = 9; continue; } rdx = (crate::simd_ops::ru32(rcx.wrapping_add(0x3cu64))) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rax.wrapping_add(0x18u64))) as u32 as u64; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 9; continue; } fl_l = ((rdx as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rax.wrapping_add(0x20u64))) as u32 as u64; if ((((fl_l as u32)) as i32) > (((fl_r as u32)) as i32)) { bb = 9; continue; } r8 = (crate::simd_ops::ru32(rsp.wrapping_add(0x44u64))) as u32 as u64; fl_l = ((r8 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rsp.wrapping_add(0x48u64))) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 3; continue; } rax = ((r8.wrapping_add(0x1u64)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rsp.wrapping_add(0x40u64))) as u32 as u64; bb = 4; continue; }
            3 => { fl_l = (crate::simd_ops::ru32(rsp.wrapping_add(0x40u64))) as u32 as u64; fl_r = 0x0; bb = 4; continue; }
            4 => { rax = (rax & !0xffu64) | (((if (((fl_l as u32)) == ((fl_r as u32))) { 1u8 } else { 0u8 }) as u8 as u64)); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 5; continue; } crate::simd_ops::wu8(rbp.wrapping_add(0x40u64), (0x1u8) as u8); bb = 6; continue; }
            5 => { rdx = rbp.wrapping_add(0x40); rcx = rsp.wrapping_add(0x30); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = (crate::simd_ops::ru8(rbp.wrapping_add(0x40u64))) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 11; continue; } r8 = (crate::simd_ops::ru32(rsp.wrapping_add(0x44u64))) as u32 as u64; bb = 6; continue; }
            6 => { xmm0 = (crate::simd_ops::ru128(rbp.wrapping_sub(0x70u64))) as u128; rcx = (crate::simd_ops::ru32(rsp.wrapping_add(0x38u64))) as u32 as u64; rcx = ((rcx).wrapping_add(rcx)) as u64; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rdx = (crate::simd_ops::ru64(rax.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x18u64))) as u64; rax = ((r8 as u32)) as u32 as u64; rax = ((rax).wrapping_add(rax)) as u64; crate::simd_ops::wu128(rdx.wrapping_add(rax.wrapping_mul(8)), (xmm0) as u128); rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x44u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rsp.wrapping_add(0x48u64))) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 7; continue; } fl_l = (((rax as u32)).wrapping_add(1)) as u32 as u64; rax = ((fl_l as u32)) as u32 as u64; bb = 8; continue; }
            7 => { rax = ((r14 as u32)) as u32 as u64; bb = 8; continue; }
            8 => { crate::simd_ops::wu32(rsp.wrapping_add(0x44u64), ((rax as u32)) as u32); rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } r9 = rbp.wrapping_add(0x58); r8 = rbp.wrapping_sub(0x70); rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; rcx = rsp.wrapping_add(0x50); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 11; continue; } bb = 9; continue; }
            9 => { rbx = (((rbx as u32)).wrapping_add(1)) as u32 as u64; rcx = (crate::simd_ops::ru64(rsi.wrapping_add(0x120u64))) as u64; fl_l = ((rbx as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rcx.wrapping_add(0x18u64))) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 2; continue; } bb = 10; continue; }
            10 => { rcx = rsp.wrapping_add(0x20); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 16; continue; } crate::simd_ops::wu8(rdi, (0x0u8) as u8); rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; bb = 13; continue; }
            11 => { rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; bb = 12; continue; }
            12 => { crate::simd_ops::wu8(rdi, (0x0u8) as u8); bb = 13; continue; }
            13 => { fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 14; continue; } rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x58u64))) as u32 as u64; fl_l = ((rax).wrapping_add(rax)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; crate::simd_ops::wu8(rcx.wrapping_add(rax.wrapping_mul(8)).wrapping_add(0x24u64), (0x0u8) as u8); bb = 14; continue; }
            14 => { rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 15; continue; } rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x38u64))) as u32 as u64; fl_l = ((rax).wrapping_add(rax)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; crate::simd_ops::wu8(rcx.wrapping_add(rax.wrapping_mul(8)).wrapping_add(0x24u64), (0x0u8) as u8); bb = 15; continue; }
            15 => { rax = rdi; r11 = rsp.wrapping_add(0x120); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x28u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x30u64))) as u64; rsp = r11; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            16 => { fl_l = (crate::simd_ops::ru8(rbp.wrapping_sub(0x50u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 17; continue; } crate::simd_ops::wu8(rdi, (0x0u8) as u8); rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; bb = 13; continue; }
            17 => { crate::simd_ops::wu8(rdi, (0x1u8) as u8); rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; bb = 13; continue; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_22731<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rcx;
    rdx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    fl_l = rdx; fl_r = rdx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rcx = (crate::simd_ops::ru64(P1)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64, rax, rcx, rdx, r8, r9, rsp); }
    rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P3))) as u64;
    fl_l = rcx; fl_r = rcx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); }
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_22735<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
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
pub unsafe extern "win64" fn tpl_22748<const P0: u64, const P1: u64, const P2: u64, const P3: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
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
    rdx = 0x210;
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
pub unsafe extern "win64" fn tpl_22805<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rsi = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rbx = rdx; rax = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; rdi = rcx; fl_l = rdx; fl_r = rsi; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 2; continue; } rax = ((rax).wrapping_shr(1u32)) as u64; fl_l = rdx; fl_r = rax; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 1; continue; } r8 = rdx; rdx = rcx; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rdi.wrapping_add(P0), rbx); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            2 => { fl_l = rbx; fl_r = rax; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 3; continue; } r8 = rbx; rdx = rdi; r8 = ((r8).wrapping_shr(2u32)) as u64; r8 = ((r8).wrapping_add(rbx)) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 3; continue; }
            3 => { crate::simd_ops::wu64(rdi.wrapping_add(P0), rbx); fl_l = rbx; fl_r = rsi; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 5; continue; } rax = (crate::simd_ops::ru64(rdi)) as u64; rcx = ((rsi.wrapping_add(rsi.wrapping_mul(2))) as u64) as u64; rdx = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = rbx; fl_r = rsi; rbx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 5; continue; } rax = ((rdx.wrapping_add(P3)) as u64) as u64; bb = 4; continue; }
            4 => { crate::simd_ops::wu32(rdx, (0x7fffffffu32) as u32); rdx = ((rdx.wrapping_add(P4)) as u64) as u64; crate::simd_ops::wu32(rax.wrapping_add(P5), (0x7fffffffu32) as u32); crate::simd_ops::wu32(rax.wrapping_add(P6), (0x80000000u32) as u32); crate::simd_ops::wu64(rax, (0xffffffff80000000u64) as u64); crate::simd_ops::wu32(rax.wrapping_add(P0), (0xffffffffu32) as u32); rax = ((rax.wrapping_add(P4)) as u64) as u64; fl_l = rbx; fl_r = (0x1u64) as u64; rbx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 4; continue; } bb = 5; continue; }
            5 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_22829<const P0: u64, const P1: u64, const P2: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rdi = rdx;
    rbx = rcx;
    rdx = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64;
    rax = (crate::simd_ops::ru32(rcx.wrapping_add(P1))) as u32 as u64;
    r8 = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64;
    r8 = ((r8).wrapping_shl(2u32)) as u64;
    rax = ((rdx.wrapping_add(rdx.wrapping_mul(2))) as u64) as u64;
    rax = ((rax).wrapping_shl(2u32)) as u64;
    fl_l = rax; fl_r = r8;
    if !((((fl_l as u64)) != ((fl_r as u64)))) { rdx = ((rdx).wrapping_add(1)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); }
    rax = (crate::simd_ops::ru32(rbx.wrapping_add(P0))) as u32 as u64;
    rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64;
    rax = (crate::simd_ops::ru64(rbx)) as u64;
    rdx = ((rax.wrapping_add(rcx.wrapping_mul(4))) as u64) as u64;
    xmm0 = ((crate::simd_ops::ru64(rdi)) as u64) as u128;
    crate::simd_ops::wu64(rdx, ((xmm0 as u64)) as u64);
    rax = (crate::simd_ops::ru32(rdi.wrapping_add(P0))) as u32 as u64;
    crate::simd_ops::wu32(rdx.wrapping_add(P0), ((rax as u32)) as u32);
    crate::simd_ops::lk_add_u32(rbx.wrapping_add(P0), (1) as u32);
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_22851<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64, const P9: u64, const P10: u64, const P11: u64, const P12: u64, const P13: u64, const P14: u64, const P15: u64, const P16: u64, const P17: u64, const P18: u64, const P19: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x60u64)) as u64; rdi = rdx; rbx = rcx; crate::simd_ops::wu8(rsp.wrapping_add(0x38u64), ((r8 as u8)) as u8); rax = P0; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), (0x5u32) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), (0x5u32) as u32); r8 = (crate::simd_ops::ru64(P1)) as u64; rcx = rdx; rax = crate::simd_ops::call_stk_caught(P2, rax, rcx, rdx, r8, r9, rsp); r9 = P3; r8 = 0x1; rdx = rbx; rcx = rdi; rax = crate::simd_ops::call_stk_caught(P4, rax, rcx, rdx, r8, r9, rsp); rdx = rbx.wrapping_add(0x10); r9 = P5; r8 = 0x1; rcx = rdi; rax = crate::simd_ops::call_stk_caught(P6, rax, rcx, rdx, r8, r9, rsp); rdx = rbx.wrapping_add(0x20); r9 = P7; r8 = 0x1; rcx = rdi; rax = crate::simd_ops::call_stk_caught(P6, rax, rcx, rdx, r8, r9, rsp); rdx = rbx.wrapping_add(0x30); r9 = P8; r8 = 0x3; rcx = rdi; rax = crate::simd_ops::call_stk_caught(P6, rax, rcx, rdx, r8, r9, rsp); rdx = rbx.wrapping_add(0x40); r9 = P9; r8 = 0x3; rcx = rdi; rax = crate::simd_ops::call_stk_caught(P6, rax, rcx, rdx, r8, r9, rsp); rdx = rbx.wrapping_add(0x50); r9 = P10; r8 = 0x4; rcx = rdi; rax = crate::simd_ops::call_stk_caught(P6, rax, rcx, rdx, r8, r9, rsp); rdx = rbx.wrapping_add(0x60); r9 = P11; r8 = 0x2; rcx = rdi; rax = crate::simd_ops::call_stk_caught(P12, rax, rcx, rdx, r8, r9, rsp); rdx = rbx.wrapping_add(0x70); r9 = P13; r8 = 0x2; rcx = rdi; rax = crate::simd_ops::call_stk_caught(P12, rax, rcx, rdx, r8, r9, rsp); rdx = rbx.wrapping_add(0x80); r9 = P14; r8 = 0x5; rcx = rdi; rax = crate::simd_ops::call_stk_caught(P6, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x28u64))) as u32 as u64; rcx = (((rax) as i64).wrapping_mul((0x108u64) as i64) as u64) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rdi.wrapping_add(0x20u64)))) as u64; fl_l = (crate::simd_ops::ru64(rdi.wrapping_add(0x20u64))) as u64; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } rax = 0; fl_l = 0; fl_r = 0; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rsp = ((rsp).wrapping_add(0x60u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            1 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x40u64))) as u64; rax = crate::simd_ops::call_stk_caught(P15, rax, rcx, rdx, r8, r9, rsp); rcx = rax; rax = crate::simd_ops::call_stk_caught(P16, rax, rcx, rdx, r8, r9, rsp); r8 = rax; rdx = rdi.wrapping_add(0x20); rcx = rsp.wrapping_add(0x40); rax = crate::simd_ops::call_stk_caught(P17, rax, rcx, rdx, r8, r9, rsp); rcx = rax; rax = crate::simd_ops::call_stk_caught(P18, rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, P19, rcx, rsp); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_22865<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rax = P0;
    rbx = rcx;
    crate::simd_ops::wu64(rcx, rax);
    fl_l = ((rdx as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64;
    if !(((((fl_l as u8)) & ((fl_r as u8))) == 0)) { rdx = 0x18; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); }
    rax = rbx;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_22866<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    crate::simd_ops::forward_incoming_to_outgoing();
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rdx = rcx;
    rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_22867<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    crate::simd_ops::forward_incoming_to_outgoing();
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    fl_l = rcx; fl_r = rcx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rax = (crate::simd_ops::ru64(rcx)) as u64; crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_22884<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64, const P9: u64, const P10: u64, const P11: u64, const P12: u64, const P13: u64, const P14: u64, const P15: u64, const P16: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x60u64)) as u64; rdi = rdx; rbx = rcx; crate::simd_ops::wu8(rsp.wrapping_add(0x38u64), ((r8 as u8)) as u8); rax = P0; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), (0x2u32) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), (0x2u32) as u32); r8 = (crate::simd_ops::ru64(P1)) as u64; rcx = rdx; rax = crate::simd_ops::call_stk_caught(P2, rax, rcx, rdx, r8, r9, rsp); rcx = rdi; rax = crate::simd_ops::call_stk_caught(P3, rax, rcx, rdx, r8, r9, rsp); rdx = ((rbx.wrapping_add(P4)) as u64) as u64; r9 = P5; r8 = 0x2; rcx = rdi; rax = crate::simd_ops::call_stk_caught(P6, rax, rcx, rdx, r8, r9, rsp); rdx = ((rbx.wrapping_add(P7)) as u64) as u64; r9 = P8; r8 = 0x2; rcx = rdi; rax = crate::simd_ops::call_stk_caught(P9, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru32(rdi.wrapping_add(P10))) as u32 as u64; rcx = (((rax) as i64).wrapping_mul((0x108u64) as i64) as u64) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rdi.wrapping_add(P7)))) as u64; fl_l = (crate::simd_ops::ru64(rdi.wrapping_add(P7))) as u64; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } rax = 0; fl_l = 0; fl_r = 0; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rsp = ((rsp).wrapping_add(0x60u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            1 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(P11))) as u64; rax = crate::simd_ops::call_stk_caught(P12, rax, rcx, rdx, r8, r9, rsp); rcx = rax; rax = crate::simd_ops::call_stk_caught(P13, rax, rcx, rdx, r8, r9, rsp); r8 = rax; rdx = rdi.wrapping_add(0x20); rcx = rsp.wrapping_add(0x40); rax = crate::simd_ops::call_stk_caught(P14, rax, rcx, rdx, r8, r9, rsp); rcx = rax; rax = crate::simd_ops::call_stk_caught(P15, rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, P16, rcx, rsp); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_22892<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 113>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { fl_l = ((r8 as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 8; continue; } crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rbp = rsp.wrapping_sub(0x260); rsp = ((rsp).wrapping_sub(0x360u64)) as u64; rdi = rdx; rsi = rcx; r14 = ((((r8 as u32)) as i32 as i64) as u64) as u64; r12 = 0; fl_l = 0; fl_r = 0; r15 = P0; r13 = (0x2aaaaaaaaaaaaaabu64) as u64; bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru32(rsi.wrapping_add(0x328u64))) as u32 as u64; fl_r = 0x2; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 2; continue; } rdx = rsi; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 7; continue; }
            2 => { fl_l = (crate::simd_ops::ru8(rsi.wrapping_add(0x369u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 7; continue; } rdx = 0; fl_l = 0; fl_r = 0; r8 = 0x310; rcx = rsp.wrapping_add(0x20); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rsp.wrapping_add(0x30); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rcx); crate::simd_ops::wu32(rsp.wrapping_add(0x2cu64), (0x10u32) as u32); crate::simd_ops::wu32(rbp.wrapping_add(0x238u64), (0x6u32) as u32); crate::simd_ops::wu64(rbp.wrapping_add(0x240u64), r12); crate::simd_ops::wu32(rbp.wrapping_add(0x248u64), (0xffffffffu32) as u32); crate::simd_ops::wu64(rbp.wrapping_add(0x250u64), r12); crate::simd_ops::wu64(rbp.wrapping_add(0x230u64), r12); xmm0 = 0u128; ymm0_hi = 0; let y_addr: u64 = rsp.wrapping_add(0x30u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); crate::simd_ops::wu128(rsp.wrapping_add(0x50u64), (xmm0) as u128); rax = ((r12.wrapping_add(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((rax as u32)) as u32); rbx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rbx = ((rbx).wrapping_shl(4u32)) as u64; rbx = ((rbx).wrapping_add(rcx)) as u64; crate::simd_ops::wu64(rbx.wrapping_sub(0x30u64), r12); crate::simd_ops::wu64(rbx.wrapping_sub(0x28u64), r15); crate::simd_ops::wu64(rbx.wrapping_sub(0x20u64), r12); crate::simd_ops::wu64(rbx.wrapping_sub(0x18u64), r12); crate::simd_ops::wu32(rbx.wrapping_sub(0x8u64), (0xffffffffu32) as u32); crate::simd_ops::wu64(rbx.wrapping_sub(0x10u64), r12); rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x240u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } rcx = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } rax = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm((crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rbx.wrapping_sub(0x18u64), rax); bb = 3; continue; }
            3 => { crate::simd_ops::wu32(rbp.wrapping_add(0x258u64), ((r12 as u32)) as u32); r15 = (crate::simd_ops::ru64(P3)) as u64; rdx = (crate::simd_ops::ru32(rsp.wrapping_add(0x28u64))) as u32 as u64; r8 = ((rdx.wrapping_add(rdx.wrapping_mul(2))) as u64) as u64; r8 = ((r8).wrapping_shl(4u32)) as u64; r9 = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; r8 = ((r8).wrapping_add(r9)) as u64; rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x2cu64))) as u32 as u64; r10 = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; r10 = ((r10).wrapping_shl(4u32)) as u64; r10 = ((r10).wrapping_add(r9)) as u64; fl_l = r8; fl_r = r10; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 4; continue; } r8 = ((r8).wrapping_sub(r9)) as u64; rax = r13; let prod = (rax as i64 as i128).wrapping_mul((r8) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; rdx = ((((rdx) as i64).wrapping_shr(3u32) as u64)) as u64; r8 = rdx; r8 = ((r8).wrapping_shr(63u32)) as u64; rdx = ((rdx).wrapping_add(1)) as u64; r8 = ((r8).wrapping_add(rdx)) as u64; r10 = ((r10).wrapping_sub(r9)) as u64; rax = r13; let prod = (rax as i64 as i128).wrapping_mul((r10) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; rcx = rdx; rcx = ((((rcx) as i64).wrapping_shr(3u32) as u64)) as u64; rax = rcx; rax = ((rax).wrapping_shr(63u32)) as u64; rcx = ((rcx).wrapping_add(rax)) as u64; rdx = rcx; rdx = ((rdx).wrapping_shr(1u32)) as u64; rdx = ((rdx).wrapping_add(rcx)) as u64; fl_l = rdx; fl_r = r8; rdx = (if (((fl_l as u64)) < ((fl_r as u64))) { r8 } else { rdx }) as u64; rcx = rsp.wrapping_add(0x20); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = (crate::simd_ops::ru32(rsp.wrapping_add(0x28u64))) as u32 as u64; r9 = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; bb = 4; continue; }
            4 => { rax = ((rdx as u32)) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rcx = ((rcx).wrapping_add(rcx)) as u64; xmm0 = 0u128; ymm0_hi = 0; let y_addr: u64 = r9.wrapping_add(rcx.wrapping_mul(8)); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); crate::simd_ops::wu128(r9.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x20u64), (xmm0) as u128); rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x28u64))) as u32 as u64; rax = (((rax as u32)).wrapping_add(1)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((rax as u32)) as u32); rbx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rbx = ((rbx).wrapping_shl(4u32)) as u64; rbx = ((rbx).wrapping_add(crate::simd_ops::ru64(rsp.wrapping_add(0x20u64)))) as u64; crate::simd_ops::wu64(rbx.wrapping_sub(0x30u64), rdi); crate::simd_ops::wu64(rbx.wrapping_sub(0x28u64), r15); crate::simd_ops::wu32(rbx.wrapping_sub(0x20u64), (0x1u32) as u32); crate::simd_ops::wu32(rbx.wrapping_sub(0x1cu64), (0x1u32) as u32); crate::simd_ops::wu64(rbx.wrapping_sub(0x18u64), r12); crate::simd_ops::wu32(rbx.wrapping_sub(0x8u64), (0xffffffffu32) as u32); crate::simd_ops::wu64(rbx.wrapping_sub(0x10u64), r12); rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x240u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } rcx = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } rax = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm((crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rbx.wrapping_sub(0x18u64), rax); bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x258u64))) as u32 as u64; rax = (((rax as u32)).wrapping_shr(1u32)) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 6; continue; } crate::simd_ops::wu64(rdi, r12); crate::simd_ops::wu32(rdi.wrapping_add(0x8u64), ((r12 as u32)) as u32); bb = 6; continue; }
            6 => { r8 = (crate::simd_ops::ru32(rsp.wrapping_add(0x28u64))) as u32 as u64; rcx = ((r8.wrapping_add(r8.wrapping_mul(2))) as u64) as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rax = r13; let prod = (rax as i64 as i128).wrapping_mul((rcx) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; rdx = ((((rdx) as i64).wrapping_shr(3u32) as u64)) as u64; rax = rdx; rax = ((rax).wrapping_shr(63u32)) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; rcx = ((rdx.wrapping_add(rdx.wrapping_mul(2))) as u64) as u64; rcx = ((rcx).wrapping_add(rcx)) as u64; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; rcx = (crate::simd_ops::ru32(rax.wrapping_add(rcx.wrapping_mul(8)).wrapping_sub(0x8u64))) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x248u64), ((rcx as u32)) as u32); r8 = (((r8 as u32)).wrapping_sub(1)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((r8 as u32)) as u32); rcx = rsp.wrapping_add(0x20); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r15 = P0; bb = 7; continue; }
            7 => { rdi = ((rdi).wrapping_add(0xcu64)) as u64; fl_l = r14; fl_r = (0x1u64) as u64; r14 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } r11 = rsp.wrapping_add(0x360); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x38u64))) as u64; rdi = (crate::simd_ops::ru64(r11.wrapping_add(0x40u64))) as u64; rsp = r11; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); bb = 8; continue; }
            8 => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_22893<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 115>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { fl_l = ((r8 as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 8; continue; } crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rdi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x380u64)) as u64; rdi = rdx; rsi = rcx; r14 = ((((r8 as u32)) as i32 as i64) as u64) as u64; r12 = 0; fl_l = 0; fl_r = 0; r15 = P0; r13 = (0x2aaaaaaaaaaaaaabu64) as u64; bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru32(rsi.wrapping_add(0x328u64))) as u32 as u64; fl_r = 0x1; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 2; continue; } rdx = rsi; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 7; continue; }
            2 => { fl_l = (crate::simd_ops::ru8(rsi.wrapping_add(0x369u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 7; continue; } rdx = 0; fl_l = 0; fl_r = 0; r8 = 0x310; rcx = rsp.wrapping_add(0x40); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rsp.wrapping_add(0x50); crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rcx); crate::simd_ops::wu32(rsp.wrapping_add(0x4cu64), (0x10u32) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x358u64), (0x6u32) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x360u64), r12); crate::simd_ops::wu32(rsp.wrapping_add(0x368u64), (0xffffffffu32) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x370u64), r12); crate::simd_ops::wu64(rsp.wrapping_add(0x350u64), r12); xmm0 = 0u128; ymm0_hi = 0; let y_addr: u64 = rsp.wrapping_add(0x50u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); crate::simd_ops::wu128(rsp.wrapping_add(0x70u64), (xmm0) as u128); rax = ((r12.wrapping_add(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x48u64), ((rax as u32)) as u32); rbx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rbx = ((rbx).wrapping_shl(4u32)) as u64; rbx = ((rbx).wrapping_add(rcx)) as u64; crate::simd_ops::wu64(rbx.wrapping_sub(0x30u64), r12); crate::simd_ops::wu64(rbx.wrapping_sub(0x28u64), r15); crate::simd_ops::wu64(rbx.wrapping_sub(0x20u64), r12); crate::simd_ops::wu64(rbx.wrapping_sub(0x18u64), r12); crate::simd_ops::wu32(rbx.wrapping_sub(0x8u64), (0xffffffffu32) as u32); crate::simd_ops::wu64(rbx.wrapping_sub(0x10u64), r12); rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x360u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } rcx = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } rax = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm((crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rbx.wrapping_sub(0x18u64), rax); bb = 3; continue; }
            3 => { crate::simd_ops::wu32(rsp.wrapping_add(0x378u64), ((r12 as u32)) as u32); r15 = (crate::simd_ops::ru64(P3)) as u64; rdx = (crate::simd_ops::ru32(rsp.wrapping_add(0x48u64))) as u32 as u64; r8 = ((rdx.wrapping_add(rdx.wrapping_mul(2))) as u64) as u64; r8 = ((r8).wrapping_shl(4u32)) as u64; r9 = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; r8 = ((r8).wrapping_add(r9)) as u64; rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x4cu64))) as u32 as u64; r10 = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; r10 = ((r10).wrapping_shl(4u32)) as u64; r10 = ((r10).wrapping_add(r9)) as u64; fl_l = r8; fl_r = r10; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 4; continue; } r8 = ((r8).wrapping_sub(r9)) as u64; rax = r13; let prod = (rax as i64 as i128).wrapping_mul((r8) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; rdx = ((((rdx) as i64).wrapping_shr(3u32) as u64)) as u64; r8 = rdx; r8 = ((r8).wrapping_shr(63u32)) as u64; rdx = ((rdx).wrapping_add(1)) as u64; r8 = ((r8).wrapping_add(rdx)) as u64; r10 = ((r10).wrapping_sub(r9)) as u64; rax = r13; let prod = (rax as i64 as i128).wrapping_mul((r10) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; rcx = rdx; rcx = ((((rcx) as i64).wrapping_shr(3u32) as u64)) as u64; rax = rcx; rax = ((rax).wrapping_shr(63u32)) as u64; rcx = ((rcx).wrapping_add(rax)) as u64; rdx = rcx; rdx = ((rdx).wrapping_shr(1u32)) as u64; rdx = ((rdx).wrapping_add(rcx)) as u64; fl_l = rdx; fl_r = r8; rdx = (if (((fl_l as u64)) < ((fl_r as u64))) { r8 } else { rdx }) as u64; rcx = rsp.wrapping_add(0x40); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = (crate::simd_ops::ru32(rsp.wrapping_add(0x48u64))) as u32 as u64; r9 = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; bb = 4; continue; }
            4 => { rax = ((rdx as u32)) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rcx = ((rcx).wrapping_add(rcx)) as u64; xmm0 = 0u128; ymm0_hi = 0; let y_addr: u64 = r9.wrapping_add(rcx.wrapping_mul(8)); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); crate::simd_ops::wu128(r9.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x20u64), (xmm0) as u128); rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x48u64))) as u32 as u64; rax = (((rax as u32)).wrapping_add(1)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x48u64), ((rax as u32)) as u32); rbx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rbx = ((rbx).wrapping_shl(4u32)) as u64; rbx = ((rbx).wrapping_add(crate::simd_ops::ru64(rsp.wrapping_add(0x40u64)))) as u64; crate::simd_ops::wu64(rbx.wrapping_sub(0x30u64), rdi); crate::simd_ops::wu64(rbx.wrapping_sub(0x28u64), r15); crate::simd_ops::wu32(rbx.wrapping_sub(0x20u64), (0x1u32) as u32); crate::simd_ops::wu32(rbx.wrapping_sub(0x1cu64), (0x1u32) as u32); crate::simd_ops::wu64(rbx.wrapping_sub(0x18u64), r12); crate::simd_ops::wu32(rbx.wrapping_sub(0x8u64), (0xffffffffu32) as u32); crate::simd_ops::wu64(rbx.wrapping_sub(0x10u64), r12); rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x360u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } rcx = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } rax = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm((crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rbx.wrapping_sub(0x18u64), rax); bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x378u64))) as u32 as u64; rax = (((rax as u32)).wrapping_shr(1u32)) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 6; continue; } crate::simd_ops::wu16(rdi, ((r12 as u16)) as u16); crate::simd_ops::wu64(rdi.wrapping_add(0x4u64), r12); xmm1 = 0u128; xmm0 = (crate::simd_ops::ru128(P5)) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128); crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm1) as u128); let y_addr: u64 = rsp.wrapping_add(0x20u64); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rdi.wrapping_add(0x10u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); bb = 6; continue; }
            6 => { r8 = (crate::simd_ops::ru32(rsp.wrapping_add(0x48u64))) as u32 as u64; rcx = ((r8.wrapping_add(r8.wrapping_mul(2))) as u64) as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rax = r13; let prod = (rax as i64 as i128).wrapping_mul((rcx) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; rdx = ((((rdx) as i64).wrapping_shr(3u32) as u64)) as u64; rax = rdx; rax = ((rax).wrapping_shr(63u32)) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; rcx = ((rdx.wrapping_add(rdx.wrapping_mul(2))) as u64) as u64; rcx = ((rcx).wrapping_add(rcx)) as u64; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rcx = (crate::simd_ops::ru32(rax.wrapping_add(rcx.wrapping_mul(8)).wrapping_sub(0x8u64))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x368u64), ((rcx as u32)) as u32); r8 = (((r8 as u32)).wrapping_sub(1)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x48u64), ((r8 as u32)) as u32); rcx = rsp.wrapping_add(0x40); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r15 = P0; bb = 7; continue; }
            7 => { rdi = ((rdi).wrapping_add(0x30u64)) as u64; fl_l = r14; fl_r = (0x1u64) as u64; r14 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } r11 = rsp.wrapping_add(0x380); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x20u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x28u64))) as u64; rdi = (crate::simd_ops::ru64(r11.wrapping_add(0x30u64))) as u64; r12 = (crate::simd_ops::ru64(r11.wrapping_add(0x38u64))) as u64; rsp = r11; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); bb = 8; continue; }
            8 => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_22894<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 113>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { fl_l = ((r8 as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 8; continue; } crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rbp = rsp.wrapping_sub(0x260); rsp = ((rsp).wrapping_sub(0x360u64)) as u64; rdi = rdx; rsi = rcx; r14 = ((((r8 as u32)) as i32 as i64) as u64) as u64; r12 = 0; fl_l = 0; fl_r = 0; r15 = P0; r13 = (0x2aaaaaaaaaaaaaabu64) as u64; bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru32(rsi.wrapping_add(0x328u64))) as u32 as u64; fl_r = 0x3; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 2; continue; } rdx = rsi; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 7; continue; }
            2 => { fl_l = (crate::simd_ops::ru8(rsi.wrapping_add(0x369u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 7; continue; } rdx = 0; fl_l = 0; fl_r = 0; r8 = 0x310; rcx = rsp.wrapping_add(0x20); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rsp.wrapping_add(0x30); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rcx); crate::simd_ops::wu32(rsp.wrapping_add(0x2cu64), (0x10u32) as u32); crate::simd_ops::wu32(rbp.wrapping_add(0x238u64), (0x6u32) as u32); crate::simd_ops::wu64(rbp.wrapping_add(0x240u64), r12); crate::simd_ops::wu32(rbp.wrapping_add(0x248u64), (0xffffffffu32) as u32); crate::simd_ops::wu64(rbp.wrapping_add(0x250u64), r12); crate::simd_ops::wu64(rbp.wrapping_add(0x230u64), r12); xmm0 = 0u128; ymm0_hi = 0; let y_addr: u64 = rsp.wrapping_add(0x30u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); crate::simd_ops::wu128(rsp.wrapping_add(0x50u64), (xmm0) as u128); rax = ((r12.wrapping_add(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((rax as u32)) as u32); rbx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rbx = ((rbx).wrapping_shl(4u32)) as u64; rbx = ((rbx).wrapping_add(rcx)) as u64; crate::simd_ops::wu64(rbx.wrapping_sub(0x30u64), r12); crate::simd_ops::wu64(rbx.wrapping_sub(0x28u64), r15); crate::simd_ops::wu64(rbx.wrapping_sub(0x20u64), r12); crate::simd_ops::wu64(rbx.wrapping_sub(0x18u64), r12); crate::simd_ops::wu32(rbx.wrapping_sub(0x8u64), (0xffffffffu32) as u32); crate::simd_ops::wu64(rbx.wrapping_sub(0x10u64), r12); rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x240u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } rcx = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } rax = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm((crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rbx.wrapping_sub(0x18u64), rax); bb = 3; continue; }
            3 => { crate::simd_ops::wu32(rbp.wrapping_add(0x258u64), ((r12 as u32)) as u32); r15 = (crate::simd_ops::ru64(P3)) as u64; rdx = (crate::simd_ops::ru32(rsp.wrapping_add(0x28u64))) as u32 as u64; r8 = ((rdx.wrapping_add(rdx.wrapping_mul(2))) as u64) as u64; r8 = ((r8).wrapping_shl(4u32)) as u64; r9 = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; r8 = ((r8).wrapping_add(r9)) as u64; rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x2cu64))) as u32 as u64; r10 = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; r10 = ((r10).wrapping_shl(4u32)) as u64; r10 = ((r10).wrapping_add(r9)) as u64; fl_l = r8; fl_r = r10; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 4; continue; } r8 = ((r8).wrapping_sub(r9)) as u64; rax = r13; let prod = (rax as i64 as i128).wrapping_mul((r8) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; rdx = ((((rdx) as i64).wrapping_shr(3u32) as u64)) as u64; r8 = rdx; r8 = ((r8).wrapping_shr(63u32)) as u64; rdx = ((rdx).wrapping_add(1)) as u64; r8 = ((r8).wrapping_add(rdx)) as u64; r10 = ((r10).wrapping_sub(r9)) as u64; rax = r13; let prod = (rax as i64 as i128).wrapping_mul((r10) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; rcx = rdx; rcx = ((((rcx) as i64).wrapping_shr(3u32) as u64)) as u64; rax = rcx; rax = ((rax).wrapping_shr(63u32)) as u64; rcx = ((rcx).wrapping_add(rax)) as u64; rdx = rcx; rdx = ((rdx).wrapping_shr(1u32)) as u64; rdx = ((rdx).wrapping_add(rcx)) as u64; fl_l = rdx; fl_r = r8; rdx = (if (((fl_l as u64)) < ((fl_r as u64))) { r8 } else { rdx }) as u64; rcx = rsp.wrapping_add(0x20); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = (crate::simd_ops::ru32(rsp.wrapping_add(0x28u64))) as u32 as u64; r9 = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; bb = 4; continue; }
            4 => { rax = ((rdx as u32)) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rcx = ((rcx).wrapping_add(rcx)) as u64; xmm0 = 0u128; ymm0_hi = 0; let y_addr: u64 = r9.wrapping_add(rcx.wrapping_mul(8)); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); crate::simd_ops::wu128(r9.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x20u64), (xmm0) as u128); rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x28u64))) as u32 as u64; rax = (((rax as u32)).wrapping_add(1)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((rax as u32)) as u32); rbx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rbx = ((rbx).wrapping_shl(4u32)) as u64; rbx = ((rbx).wrapping_add(crate::simd_ops::ru64(rsp.wrapping_add(0x20u64)))) as u64; crate::simd_ops::wu64(rbx.wrapping_sub(0x30u64), rdi); crate::simd_ops::wu64(rbx.wrapping_sub(0x28u64), r15); crate::simd_ops::wu32(rbx.wrapping_sub(0x20u64), (0x1u32) as u32); crate::simd_ops::wu32(rbx.wrapping_sub(0x1cu64), (0x1u32) as u32); crate::simd_ops::wu64(rbx.wrapping_sub(0x18u64), r12); crate::simd_ops::wu32(rbx.wrapping_sub(0x8u64), (0xffffffffu32) as u32); crate::simd_ops::wu64(rbx.wrapping_sub(0x10u64), r12); rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x240u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } rcx = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } rax = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm((crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rbx.wrapping_sub(0x18u64), rax); bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x258u64))) as u32 as u64; rax = (((rax as u32)).wrapping_shr(1u32)) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 6; continue; } crate::simd_ops::wu64(rdi, r12); crate::simd_ops::wu64(rdi.wrapping_add(0x8u64), r12); bb = 6; continue; }
            6 => { r8 = (crate::simd_ops::ru32(rsp.wrapping_add(0x28u64))) as u32 as u64; rcx = ((r8.wrapping_add(r8.wrapping_mul(2))) as u64) as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rax = r13; let prod = (rax as i64 as i128).wrapping_mul((rcx) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; rdx = ((((rdx) as i64).wrapping_shr(3u32) as u64)) as u64; rax = rdx; rax = ((rax).wrapping_shr(63u32)) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; rcx = ((rdx.wrapping_add(rdx.wrapping_mul(2))) as u64) as u64; rcx = ((rcx).wrapping_add(rcx)) as u64; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; rcx = (crate::simd_ops::ru32(rax.wrapping_add(rcx.wrapping_mul(8)).wrapping_sub(0x8u64))) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x248u64), ((rcx as u32)) as u32); r8 = (((r8 as u32)).wrapping_sub(1)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((r8 as u32)) as u32); rcx = rsp.wrapping_add(0x20); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r15 = P0; bb = 7; continue; }
            7 => { rdi = ((rdi).wrapping_add(0x10u64)) as u64; fl_l = r14; fl_r = (0x1u64) as u64; r14 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } r11 = rsp.wrapping_add(0x360); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x38u64))) as u64; rdi = (crate::simd_ops::ru64(r11.wrapping_add(0x40u64))) as u64; rsp = r11; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); bb = 8; continue; }
            8 => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_22904<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut ymm0_hi: u128 = 0;
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let y_addr: u64 = rcx.wrapping_add(P0); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16));
    let y_addr: u64 = rdx; crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi);
    rax = rdx;
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_22914<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rax = P0;
    rbx = rcx;
    crate::simd_ops::wu64(rcx, rax);
    fl_l = ((rdx as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64;
    if !(((((fl_l as u8)) & ((fl_r as u8))) == 0)) { rdx = 0x60; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); }
    rax = rbx;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_22928<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64, const P9: u64, const P10: u64, const P11: u64, const P12: u64, const P13: u64, const P14: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x60u64)) as u64; rdi = rdx; rbx = rcx; crate::simd_ops::wu8(rsp.wrapping_add(0x38u64), ((r8 as u8)) as u8); rax = P0; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), (0x2u32) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), (0x2u32) as u32); r8 = (crate::simd_ops::ru64(P1)) as u64; rcx = rdx; rax = crate::simd_ops::call_stk_caught(P2, rax, rcx, rdx, r8, r9, rsp); rdx = rbx.wrapping_add(0x40); r9 = P3; r8 = 0x2; rcx = rdi; rax = crate::simd_ops::call_stk_caught(P4, rax, rcx, rdx, r8, r9, rsp); rsi = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rsp.wrapping_add(0x70u64), ((rsi as u32)) as u32); rdx = rbx.wrapping_add(0x138); rax = rsp.wrapping_add(0x70); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); r9 = P5; r8 = ((rsi.wrapping_add(0x2u64)) as u32) as u32 as u64; rcx = rdi; rax = crate::simd_ops::call_stk_caught(P6, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu32(rsp.wrapping_add(0x70u64), ((rsi as u32)) as u32); rdx = rbx.wrapping_add(0x13c); rax = rsp.wrapping_add(0x70); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); r9 = P7; r8 = ((rsi.wrapping_add(0x2u64)) as u32) as u32 as u64; rcx = rdi; rax = crate::simd_ops::call_stk_caught(P6, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu32(rsp.wrapping_add(0x70u64), ((rsi as u32)) as u32); rdx = rbx.wrapping_add(0x140); rax = rsp.wrapping_add(0x70); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); r9 = P8; r8 = ((rsi.wrapping_add(0x2u64)) as u32) as u32 as u64; rcx = rdi; rax = crate::simd_ops::call_stk_caught(P6, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu32(rsp.wrapping_add(0x70u64), ((rsi as u32)) as u32); rdx = rbx.wrapping_add(0x144); rax = rsp.wrapping_add(0x70); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); r9 = P9; r8 = ((rsi.wrapping_add(0x2u64)) as u32) as u32 as u64; rcx = rdi; rax = crate::simd_ops::call_stk_caught(P6, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x28u64))) as u32 as u64; rcx = (((rax) as i64).wrapping_mul((0x108u64) as i64) as u64) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rdi.wrapping_add(0x20u64)))) as u64; fl_l = (crate::simd_ops::ru64(rdi.wrapping_add(0x20u64))) as u64; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } rax = 0; fl_l = 0; fl_r = 0; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; rsp = ((rsp).wrapping_add(0x60u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            1 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x40u64))) as u64; rax = crate::simd_ops::call_stk_caught(P10, rax, rcx, rdx, r8, r9, rsp); rcx = rax; rax = crate::simd_ops::call_stk_caught(P11, rax, rcx, rdx, r8, r9, rsp); r8 = rax; rdx = rdi.wrapping_add(0x20); rcx = rsp.wrapping_add(0x40); rax = crate::simd_ops::call_stk_caught(P12, rax, rcx, rdx, r8, r9, rsp); rcx = rax; rax = crate::simd_ops::call_stk_caught(P13, rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, P14, rcx, rsp); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_22950<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 115>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { fl_l = ((r8 as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 8; continue; } crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rdi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x380u64)) as u64; r12 = ((r9 as u32)) as u32 as u64; rdi = rdx; rsi = rcx; r14 = ((((r8 as u32)) as i32 as i64) as u64) as u64; r13 = 0; fl_l = 0; fl_r = 0; r15 = P0; bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru32(rsi.wrapping_add(0x328u64))) as u32 as u64; fl_r = ((r12 as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 2; continue; } rdx = rsi; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 7; continue; }
            2 => { fl_l = (crate::simd_ops::ru8(rsi.wrapping_add(0x369u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 7; continue; } rdx = 0; fl_l = 0; fl_r = 0; r8 = 0x310; rcx = rsp.wrapping_add(0x40); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rsp.wrapping_add(0x50); crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rcx); crate::simd_ops::wu32(rsp.wrapping_add(0x4cu64), (0x10u32) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x358u64), (0x6u32) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x360u64), r13); crate::simd_ops::wu32(rsp.wrapping_add(0x368u64), (0xffffffffu32) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x370u64), r13); crate::simd_ops::wu64(rsp.wrapping_add(0x350u64), r13); xmm0 = 0u128; ymm0_hi = 0; let y_addr: u64 = rsp.wrapping_add(0x50u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); crate::simd_ops::wu128(rsp.wrapping_add(0x70u64), (xmm0) as u128); rax = ((r13.wrapping_add(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x48u64), ((rax as u32)) as u32); rbx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rbx = ((rbx).wrapping_shl(4u32)) as u64; rbx = ((rbx).wrapping_add(rcx)) as u64; crate::simd_ops::wu64(rbx.wrapping_sub(0x30u64), r13); crate::simd_ops::wu64(rbx.wrapping_sub(0x28u64), r15); crate::simd_ops::wu64(rbx.wrapping_sub(0x20u64), r13); crate::simd_ops::wu64(rbx.wrapping_sub(0x18u64), r13); crate::simd_ops::wu32(rbx.wrapping_sub(0x8u64), (0xffffffffu32) as u32); crate::simd_ops::wu64(rbx.wrapping_sub(0x10u64), r13); rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x360u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } rcx = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } rax = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm((crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rbx.wrapping_sub(0x18u64), rax); bb = 3; continue; }
            3 => { crate::simd_ops::wu32(rsp.wrapping_add(0x378u64), ((r13 as u32)) as u32); r15 = (crate::simd_ops::ru64(P3)) as u64; rdx = (crate::simd_ops::ru32(rsp.wrapping_add(0x48u64))) as u32 as u64; r8 = ((rdx.wrapping_add(rdx.wrapping_mul(2))) as u64) as u64; r8 = ((r8).wrapping_shl(4u32)) as u64; r9 = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; r8 = ((r8).wrapping_add(r9)) as u64; rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x4cu64))) as u32 as u64; r10 = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; r10 = ((r10).wrapping_shl(4u32)) as u64; r10 = ((r10).wrapping_add(r9)) as u64; fl_l = r8; fl_r = r10; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 4; continue; } r8 = ((r8).wrapping_sub(r9)) as u64; rcx = (0x2aaaaaaaaaaaaaabu64) as u64; rax = rcx; let prod = (rax as i64 as i128).wrapping_mul((r8) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; rdx = ((((rdx) as i64).wrapping_shr(3u32) as u64)) as u64; r8 = rdx; r8 = ((r8).wrapping_shr(63u32)) as u64; rdx = ((rdx).wrapping_add(1)) as u64; r8 = ((r8).wrapping_add(rdx)) as u64; r10 = ((r10).wrapping_sub(r9)) as u64; rax = rcx; let prod = (rax as i64 as i128).wrapping_mul((r10) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; rcx = rdx; rcx = ((((rcx) as i64).wrapping_shr(3u32) as u64)) as u64; rax = rcx; rax = ((rax).wrapping_shr(63u32)) as u64; rcx = ((rcx).wrapping_add(rax)) as u64; rdx = rcx; rdx = ((rdx).wrapping_shr(1u32)) as u64; rdx = ((rdx).wrapping_add(rcx)) as u64; fl_l = rdx; fl_r = r8; rdx = (if (((fl_l as u64)) < ((fl_r as u64))) { r8 } else { rdx }) as u64; rcx = rsp.wrapping_add(0x40); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = (crate::simd_ops::ru32(rsp.wrapping_add(0x48u64))) as u32 as u64; r9 = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; bb = 4; continue; }
            4 => { rax = ((rdx as u32)) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rcx = ((rcx).wrapping_add(rcx)) as u64; xmm0 = 0u128; ymm0_hi = 0; let y_addr: u64 = r9.wrapping_add(rcx.wrapping_mul(8)); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); crate::simd_ops::wu128(r9.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x20u64), (xmm0) as u128); rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x48u64))) as u32 as u64; rax = (((rax as u32)).wrapping_add(1)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x48u64), ((rax as u32)) as u32); rbx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rbx = ((rbx).wrapping_shl(4u32)) as u64; rbx = ((rbx).wrapping_add(crate::simd_ops::ru64(rsp.wrapping_add(0x40u64)))) as u64; crate::simd_ops::wu64(rbx.wrapping_sub(0x30u64), rdi); crate::simd_ops::wu64(rbx.wrapping_sub(0x28u64), r15); crate::simd_ops::wu32(rbx.wrapping_sub(0x20u64), (0x1u32) as u32); crate::simd_ops::wu32(rbx.wrapping_sub(0x1cu64), (0x1u32) as u32); crate::simd_ops::wu64(rbx.wrapping_sub(0x18u64), r13); crate::simd_ops::wu32(rbx.wrapping_sub(0x8u64), (0xffffffffu32) as u32); crate::simd_ops::wu64(rbx.wrapping_sub(0x10u64), r13); rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x360u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } rcx = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } rax = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm((crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rbx.wrapping_sub(0x18u64), rax); bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x378u64))) as u32 as u64; rax = (((rax as u32)).wrapping_shr(1u32)) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 6; continue; } xmm1 = 0u128; xmm0 = (crate::simd_ops::ru128(P5)) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128); crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm1) as u128); let y_addr: u64 = rsp.wrapping_add(0x20u64); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rdi; crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); bb = 6; continue; }
            6 => { r8 = (crate::simd_ops::ru32(rsp.wrapping_add(0x48u64))) as u32 as u64; rcx = ((r8.wrapping_add(r8.wrapping_mul(2))) as u64) as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rax = (0x2aaaaaaaaaaaaaabu64) as u64; let prod = (rax as i64 as i128).wrapping_mul((rcx) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; rdx = ((((rdx) as i64).wrapping_shr(3u32) as u64)) as u64; rax = rdx; rax = ((rax).wrapping_shr(63u32)) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; rcx = ((rdx.wrapping_add(rdx.wrapping_mul(2))) as u64) as u64; rcx = ((rcx).wrapping_add(rcx)) as u64; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rcx = (crate::simd_ops::ru32(rax.wrapping_add(rcx.wrapping_mul(8)).wrapping_sub(0x8u64))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x368u64), ((rcx as u32)) as u32); r8 = (((r8 as u32)).wrapping_sub(1)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x48u64), ((r8 as u32)) as u32); rcx = rsp.wrapping_add(0x40); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r15 = P0; bb = 7; continue; }
            7 => { rdi = ((rdi).wrapping_add(0x20u64)) as u64; fl_l = r14; fl_r = (0x1u64) as u64; r14 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } r11 = rsp.wrapping_add(0x380); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x20u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x28u64))) as u64; rdi = (crate::simd_ops::ru64(r11.wrapping_add(0x30u64))) as u64; r12 = (crate::simd_ops::ru64(r11.wrapping_add(0x38u64))) as u64; rsp = r11; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); bb = 8; continue; }
            8 => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_22961<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64, const P9: u64, const P10: u64, const P11: u64, const P12: u64, const P13: u64, const P14: u64, const P15: u64, const P16: u64, const P17: u64, const P18: u64, const P19: u64, const P20: u64, const P21: u64, const P22: u64, const P23: u64, const P24: u64, const P25: u64, const P26: u64, const P27: u64, const P28: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x60u64)) as u64; rdi = rdx; rbx = rcx; crate::simd_ops::wu8(rsp.wrapping_add(0x38u64), ((r8 as u8)) as u8); rax = P0; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), (0x2u32) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), (0x4u32) as u32); r8 = (crate::simd_ops::ru64(P1)) as u64; rcx = rdx; rax = crate::simd_ops::call_stk_caught(P2, rax, rcx, rdx, r8, r9, rsp); r9 = P3; r8 = 0x1; rdx = rbx; rcx = rdi; rax = crate::simd_ops::call_stk_caught(P4, rax, rcx, rdx, r8, r9, rsp); rdx = rbx.wrapping_add(0x10); r9 = P5; r8 = 0x1; rcx = rdi; rax = crate::simd_ops::call_stk_caught(P6, rax, rcx, rdx, r8, r9, rsp); rdx = rbx.wrapping_add(0x20); r9 = P7; r8 = 0x1; rcx = rdi; rax = crate::simd_ops::call_stk_caught(P6, rax, rcx, rdx, r8, r9, rsp); rdx = rbx.wrapping_add(0x30); r9 = P8; r8 = 0x1; rcx = rdi; rax = crate::simd_ops::call_stk_caught(P9, rax, rcx, rdx, r8, r9, rsp); rdx = rbx.wrapping_add(0x50); r9 = P10; r8 = 0x1; rcx = rdi; rax = crate::simd_ops::call_stk_caught(P11, rax, rcx, rdx, r8, r9, rsp); rdx = rbx.wrapping_add(0x40); r9 = P12; r8 = 0x1; rcx = rdi; rax = crate::simd_ops::call_stk_caught(P13, rax, rcx, rdx, r8, r9, rsp); rdx = rbx.wrapping_add(0x60); r9 = P14; r8 = 0x2; rcx = rdi; rax = crate::simd_ops::call_stk_caught(P15, rax, rcx, rdx, r8, r9, rsp); rdx = rbx.wrapping_add(0x70); r9 = P16; r8 = 0x2; rcx = rdi; rax = crate::simd_ops::call_stk_caught(P6, rax, rcx, rdx, r8, r9, rsp); rdx = rbx.wrapping_add(0x80); r9 = P17; r8 = 0x2; rcx = rdi; rax = crate::simd_ops::call_stk_caught(P18, rax, rcx, rdx, r8, r9, rsp); rdx = rbx.wrapping_add(0x90); r9 = P19; r8 = 0x2; rcx = rdi; rax = crate::simd_ops::call_stk_caught(P18, rax, rcx, rdx, r8, r9, rsp); rdx = rbx.wrapping_add(0xa0); r9 = P20; r8 = 0x2; rcx = rdi; rax = crate::simd_ops::call_stk_caught(P21, rax, rcx, rdx, r8, r9, rsp); rdx = rbx.wrapping_add(0xb0); r9 = P22; r8 = 0x3; rcx = rdi; rax = crate::simd_ops::call_stk_caught(P11, rax, rcx, rdx, r8, r9, rsp); rdx = rbx.wrapping_add(0xe8); r9 = P23; r8 = 0x4; rcx = rdi; rax = crate::simd_ops::call_stk_caught(P21, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x28u64))) as u32 as u64; rcx = (((rax) as i64).wrapping_mul((0x108u64) as i64) as u64) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rdi.wrapping_add(0x20u64)))) as u64; fl_l = (crate::simd_ops::ru64(rdi.wrapping_add(0x20u64))) as u64; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } rax = 0; fl_l = 0; fl_r = 0; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rsp = ((rsp).wrapping_add(0x60u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            1 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x40u64))) as u64; rax = crate::simd_ops::call_stk_caught(P24, rax, rcx, rdx, r8, r9, rsp); rcx = rax; rax = crate::simd_ops::call_stk_caught(P25, rax, rcx, rdx, r8, r9, rsp); r8 = rax; rdx = rdi.wrapping_add(0x20); rcx = rsp.wrapping_add(0x40); rax = crate::simd_ops::call_stk_caught(P26, rax, rcx, rdx, r8, r9, rsp); rcx = rax; rax = crate::simd_ops::call_stk_caught(P27, rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, P28, rcx, rsp); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_22963<const P0: u64, const P1: u64, const P2: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x48u64)) as u64; rbx = ((crate::simd_ops::ru8(rcx.wrapping_add(P0))) as u32) as u32 as u64; rdi = r9; fl_l = ((rbx as u8)) as u8 as u64; fl_r = ((rbx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 1; continue; } rax = (crate::simd_ops::ru32(rdx.wrapping_add(P1))) as u32 as u64; r8 = (((rax) as i64).wrapping_mul((0xb0u64) as i64) as u64) as u64; r8 = ((r8).wrapping_add(crate::simd_ops::ru64(rdx))) as u64; fl_l = (crate::simd_ops::ru64(rdx)) as u64; fl_r = r8; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru32(rdx.wrapping_add(P1))) as u32 as u64; r8 = rdi; r9 = (crate::simd_ops::ru64(rdx)) as u64; rcx = (crate::simd_ops::ru64(rcx)) as u64; crate::simd_ops::wu8(rsp.wrapping_add(0x30u64), ((rbx as u8)) as u8); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 2; continue; }
            2 => { rsp = ((rsp).wrapping_add(0x48u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_22965<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64, const P9: u64, const P10: u64, const P11: u64, const P12: u64, const P13: u64, const P14: u64, const P15: u64, const P16: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 121>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r9); crate::simd_ops::wu32(rsp.wrapping_add(0x18u64), ((r8 as u32)) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rbp = rsp.wrapping_sub(0x290); rsp = ((rsp).wrapping_sub(0x390u64)) as u64; r14 = r9; r12 = rdx; rdi = rcx; fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(0x54u64))) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 10; continue; } rbx = (crate::simd_ops::ru32(rdx.wrapping_add(0x8u64))) as u32 as u64; r8 = rsp.wrapping_add(0x40); rdx = r9; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 4; continue; } rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rdx = rsi; rcx = r12; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = rbx; fl_r = rsi; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 3; continue; } r15 = (((rbx) as i64).wrapping_mul((0xb0u64) as i64) as u64) as u64; r13 = rsi; r13 = ((r13).wrapping_sub(rbx)) as u64; rsi = 0; fl_l = 0; fl_r = 0; rdi = rsi.wrapping_sub(0x1); r14 = P2; bb = 1; continue; }
            1 => { rdx = 0; fl_l = 0; fl_r = 0; r8 = 0x310; rcx = rsp.wrapping_add(0x50); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rsp.wrapping_add(0x60); crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rcx); crate::simd_ops::wu32(rsp.wrapping_add(0x5cu64), (0x10u32) as u32); crate::simd_ops::wu32(rbp.wrapping_add(0x268u64), (0x6u32) as u32); crate::simd_ops::wu64(rbp.wrapping_add(0x270u64), rsi); crate::simd_ops::wu32(rbp.wrapping_add(0x278u64), ((rdi as u32)) as u32); crate::simd_ops::wu64(rbp.wrapping_add(0x280u64), rsi); crate::simd_ops::wu64(rbp.wrapping_add(0x260u64), rsi); xmm0 = 0u128; ymm0_hi = 0; let y_addr: u64 = rsp.wrapping_add(0x60u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); crate::simd_ops::wu128(rbp.wrapping_sub(0x80u64), (xmm0) as u128); rax = ((rsi.wrapping_add(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x58u64), ((rax as u32)) as u32); rbx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rbx = ((rbx).wrapping_shl(4u32)) as u64; rbx = ((rbx).wrapping_add(rcx)) as u64; crate::simd_ops::wu64(rbx.wrapping_sub(0x30u64), rsi); crate::simd_ops::wu64(rbx.wrapping_sub(0x28u64), r14); crate::simd_ops::wu64(rbx.wrapping_sub(0x20u64), rsi); crate::simd_ops::wu64(rbx.wrapping_sub(0x18u64), rsi); crate::simd_ops::wu32(rbx.wrapping_sub(0x8u64), ((rdi as u32)) as u32); crate::simd_ops::wu64(rbx.wrapping_sub(0x10u64), rsi); rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x270u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } rcx = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } rax = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm((crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rbx.wrapping_sub(0x18u64), rax); bb = 2; continue; }
            2 => { crate::simd_ops::wu32(rbp.wrapping_add(0x288u64), ((rsi as u32)) as u32); rcx = (crate::simd_ops::ru64(r12)) as u64; rcx = ((rcx).wrapping_add(r15)) as u64; rdx = rsp.wrapping_add(0x50); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rsp.wrapping_add(0x50); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r15 = ((r15).wrapping_add(0xb0u64)) as u64; fl_l = r13; fl_r = (0x1u64) as u64; r13 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } r14 = (crate::simd_ops::ru64(rbp.wrapping_add(0x2e8u64))) as u64; rdi = (crate::simd_ops::ru64(rbp.wrapping_add(0x2d0u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x2e0u64))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rax as u32)) as u32); r9 = r14; r8 = rsi; rdx = (crate::simd_ops::ru64(r12)) as u64; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 10; continue; }
            4 => { rsi = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rsi); rax = rsp.wrapping_add(0x30); crate::simd_ops::wu64(rbp.wrapping_add(0x2d0u64), rax); r9 = r14; r8 = P7; rdx = ((crate::simd_ops::ru8(P8)) as u32) as u32 as u64; rcx = rsp.wrapping_add(0x50); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P9, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = rax; rcx = rdi.wrapping_add(0x20); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P10, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rsp.wrapping_add(0x50); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P11, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = rsp.wrapping_add(0x30); fl_l = r12; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 5; continue; } rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x38u64))) as u32 as u64; r8 = (((rax) as i64).wrapping_mul((0xb0u64) as i64) as u64) as u64; rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; r8 = ((r8).wrapping_add(rdx)) as u64; rcx = r12; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P12, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 5; continue; }
            5 => { rbx = rdi.wrapping_add(0x30); crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), (0x10000010u64) as u64); crate::simd_ops::wu8(rsp.wrapping_add(0x58u64), (0x0u8) as u8); fl_l = r14; fl_r = r14; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 6; continue; } crate::simd_ops::wu32(rsp.wrapping_add(0x54u64), ((rsi as u32)) as u32); crate::simd_ops::wu8(rsp.wrapping_add(0x58u64), ((r14 as u8)) as u8); bb = 8; continue; }
            6 => { rdi = (0xffffffffffffffffu64) as u64; bb = 7; continue; }
            7 => { rdi = ((rdi).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru8(r14.wrapping_add(rdi))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 7; continue; } crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), r14); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rdi); r9 = rdi; r8 = 0; fl_l = 0; fl_r = 0; rdx = rsp.wrapping_add(0x40); rcx = rsp.wrapping_add(0x50); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P13, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 8; continue; }
            8 => { rdx = (crate::simd_ops::ru32(rbx.wrapping_add(0x8u64))) as u32 as u64; r8 = (crate::simd_ops::ru32(rbx.wrapping_add(0xcu64))) as u32 as u64; rcx = (((r8) as i64).wrapping_mul((0x88u64) as i64) as u64) as u64; rax = (((rdx) as i64).wrapping_mul((0x88u64) as i64) as u64) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 9; continue; } rax = rdx.wrapping_add(0x1); rdx = ((r8 as u32)) as u32 as u64; rdx = ((rdx).wrapping_shr(1u32)) as u64; rdx = ((rdx).wrapping_add(r8)) as u64; fl_l = rdx; fl_r = rax; rdx = (if (((fl_l as u64)) < ((fl_r as u64))) { rax } else { rdx }) as u64; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P14, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 9; continue; }
            9 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x8u64))) as u32 as u64; rcx = (((rax) as i64).wrapping_mul((0x88u64) as i64) as u64) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rbx))) as u64; crate::simd_ops::lk_and_u32(rcx, (0xff000010u32) as u32); crate::simd_ops::lk_or_u32(rcx, (0x10u32) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x3u64), (0x10u8) as u8); crate::simd_ops::wu32(rcx.wrapping_add(0x4u64), ((rsi as u32)) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x8u64), (0x0u8) as u8); rdx = rsp.wrapping_add(0x50); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P15, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::lk_add_u32(rbx.wrapping_add(0x8u64), (1) as u32); rcx = rsp.wrapping_add(0x50); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P11, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rsp.wrapping_add(0x30); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P16, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 10; continue; }
            10 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x3d8u64))) as u64; rsp = ((rsp).wrapping_add(0x390u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_22968<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64, const P9: u64, const P10: u64, const P11: u64, const P12: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 121>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r9); crate::simd_ops::wu32(rsp.wrapping_add(0x18u64), ((r8 as u32)) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rbp = rsp.wrapping_sub(0x290); rsp = ((rsp).wrapping_sub(0x390u64)) as u64; r14 = r9; r15 = rdx; rdi = rcx; fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(0x54u64))) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 10; continue; } rbx = (crate::simd_ops::ru32(rdx.wrapping_add(0x8u64))) as u32 as u64; r8 = rsp.wrapping_add(0x40); rdx = r9; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 4; continue; } rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rdx = rsi; rcx = r15; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = rbx; fl_r = rsi; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 3; continue; } r12 = (((rbx) as i64).wrapping_mul((0xb0u64) as i64) as u64) as u64; r13 = rsi; r13 = ((r13).wrapping_sub(rbx)) as u64; rsi = 0; fl_l = 0; fl_r = 0; rdi = rsi.wrapping_sub(0x1); r14 = P2; bb = 1; continue; }
            1 => { rdx = 0; fl_l = 0; fl_r = 0; r8 = 0x310; rcx = rsp.wrapping_add(0x50); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rsp.wrapping_add(0x60); crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rcx); crate::simd_ops::wu32(rsp.wrapping_add(0x5cu64), (0x10u32) as u32); crate::simd_ops::wu32(rbp.wrapping_add(0x268u64), (0x6u32) as u32); crate::simd_ops::wu64(rbp.wrapping_add(0x270u64), rsi); crate::simd_ops::wu32(rbp.wrapping_add(0x278u64), ((rdi as u32)) as u32); crate::simd_ops::wu64(rbp.wrapping_add(0x280u64), rsi); crate::simd_ops::wu64(rbp.wrapping_add(0x260u64), rsi); xmm0 = 0u128; ymm0_hi = 0; let y_addr: u64 = rsp.wrapping_add(0x60u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); crate::simd_ops::wu128(rbp.wrapping_sub(0x80u64), (xmm0) as u128); rax = ((rsi.wrapping_add(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x58u64), ((rax as u32)) as u32); rbx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rbx = ((rbx).wrapping_shl(4u32)) as u64; rbx = ((rbx).wrapping_add(rcx)) as u64; crate::simd_ops::wu64(rbx.wrapping_sub(0x30u64), rsi); crate::simd_ops::wu64(rbx.wrapping_sub(0x28u64), r14); crate::simd_ops::wu64(rbx.wrapping_sub(0x20u64), rsi); crate::simd_ops::wu64(rbx.wrapping_sub(0x18u64), rsi); crate::simd_ops::wu32(rbx.wrapping_sub(0x8u64), ((rdi as u32)) as u32); crate::simd_ops::wu64(rbx.wrapping_sub(0x10u64), rsi); rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x270u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } rcx = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } rax = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm((crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rbx.wrapping_sub(0x18u64), rax); bb = 2; continue; }
            2 => { crate::simd_ops::wu32(rbp.wrapping_add(0x288u64), ((rsi as u32)) as u32); rcx = (crate::simd_ops::ru64(r15)) as u64; rcx = ((rcx).wrapping_add(r12)) as u64; rdx = rsp.wrapping_add(0x50); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rsp.wrapping_add(0x50); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r12 = ((r12).wrapping_add(0xb0u64)) as u64; fl_l = r13; fl_r = (0x1u64) as u64; r13 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } r14 = (crate::simd_ops::ru64(rbp.wrapping_add(0x2e8u64))) as u64; rdi = (crate::simd_ops::ru64(rbp.wrapping_add(0x2d0u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x2e0u64))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rax as u32)) as u32); r9 = r14; r8 = rsi; rdx = (crate::simd_ops::ru64(r15)) as u64; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 10; continue; }
            4 => { rsi = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rsi); rax = rsp.wrapping_add(0x30); crate::simd_ops::wu64(rbp.wrapping_add(0x2d0u64), rax); rax = rsp.wrapping_add(0x30); fl_l = r15; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 5; continue; } r8 = 0; fl_l = 0; fl_r = 0; rdx = 0; fl_l = 0; fl_r = 0; rcx = r15; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 5; continue; }
            5 => { rbx = rdi.wrapping_add(0x30); crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), (0x10000010u64) as u64); crate::simd_ops::wu8(rsp.wrapping_add(0x58u64), (0x0u8) as u8); fl_l = r14; fl_r = r14; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 6; continue; } crate::simd_ops::wu32(rsp.wrapping_add(0x54u64), ((rsi as u32)) as u32); crate::simd_ops::wu8(rsp.wrapping_add(0x58u64), ((r14 as u8)) as u8); bb = 8; continue; }
            6 => { rdi = (0xffffffffffffffffu64) as u64; bb = 7; continue; }
            7 => { rdi = ((rdi).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru8(r14.wrapping_add(rdi))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 7; continue; } crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), r14); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rdi); r9 = rdi; r8 = 0; fl_l = 0; fl_r = 0; rdx = rsp.wrapping_add(0x40); rcx = rsp.wrapping_add(0x50); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P8, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 8; continue; }
            8 => { rdx = (crate::simd_ops::ru32(rbx.wrapping_add(0x8u64))) as u32 as u64; r8 = (crate::simd_ops::ru32(rbx.wrapping_add(0xcu64))) as u32 as u64; rcx = (((r8) as i64).wrapping_mul((0x88u64) as i64) as u64) as u64; rax = (((rdx) as i64).wrapping_mul((0x88u64) as i64) as u64) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 9; continue; } rax = rdx.wrapping_add(0x1); rdx = ((r8 as u32)) as u32 as u64; rdx = ((rdx).wrapping_shr(1u32)) as u64; rdx = ((rdx).wrapping_add(r8)) as u64; fl_l = rdx; fl_r = rax; rdx = (if (((fl_l as u64)) < ((fl_r as u64))) { rax } else { rdx }) as u64; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P9, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 9; continue; }
            9 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x8u64))) as u32 as u64; rcx = (((rax) as i64).wrapping_mul((0x88u64) as i64) as u64) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rbx))) as u64; crate::simd_ops::lk_and_u32(rcx, (0xff000010u32) as u32); crate::simd_ops::lk_or_u32(rcx, (0x10u32) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x3u64), (0x10u8) as u8); crate::simd_ops::wu32(rcx.wrapping_add(0x4u64), ((rsi as u32)) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x8u64), (0x0u8) as u8); rdx = rsp.wrapping_add(0x50); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P10, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::lk_add_u32(rbx.wrapping_add(0x8u64), (1) as u32); rcx = rsp.wrapping_add(0x50); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P11, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rsp.wrapping_add(0x30); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P12, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 10; continue; }
            10 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x3d8u64))) as u64; rsp = ((rsp).wrapping_add(0x390u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_22971<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64, const P9: u64, const P10: u64, const P11: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 121>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r9); crate::simd_ops::wu32(rsp.wrapping_add(0x18u64), ((r8 as u32)) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rbp = rsp.wrapping_sub(0x290); rsp = ((rsp).wrapping_sub(0x390u64)) as u64; r14 = r9; r13 = rdx; rdi = rcx; fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(0x54u64))) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 9; continue; } rbx = (crate::simd_ops::ru32(rdx.wrapping_add(0x8u64))) as u32 as u64; r8 = rsp.wrapping_add(0x30); rdx = r9; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 4; continue; } rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rdx = rsi; rcx = r13; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = rbx; fl_r = rsi; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 3; continue; } r15 = (((rbx) as i64).wrapping_mul((0xb0u64) as i64) as u64) as u64; r12 = rsi; r12 = ((r12).wrapping_sub(rbx)) as u64; rsi = 0; fl_l = 0; fl_r = 0; rdi = rsi.wrapping_sub(0x1); r14 = P2; bb = 1; continue; }
            1 => { rdx = 0; fl_l = 0; fl_r = 0; r8 = 0x310; rcx = rsp.wrapping_add(0x50); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rsp.wrapping_add(0x60); crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rcx); crate::simd_ops::wu32(rsp.wrapping_add(0x5cu64), (0x10u32) as u32); crate::simd_ops::wu32(rbp.wrapping_add(0x268u64), (0x6u32) as u32); crate::simd_ops::wu64(rbp.wrapping_add(0x270u64), rsi); crate::simd_ops::wu32(rbp.wrapping_add(0x278u64), ((rdi as u32)) as u32); crate::simd_ops::wu64(rbp.wrapping_add(0x280u64), rsi); crate::simd_ops::wu64(rbp.wrapping_add(0x260u64), rsi); xmm0 = 0u128; ymm0_hi = 0; let y_addr: u64 = rsp.wrapping_add(0x60u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); crate::simd_ops::wu128(rbp.wrapping_sub(0x80u64), (xmm0) as u128); rax = ((rsi.wrapping_add(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x58u64), ((rax as u32)) as u32); rbx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rbx = ((rbx).wrapping_shl(4u32)) as u64; rbx = ((rbx).wrapping_add(rcx)) as u64; crate::simd_ops::wu64(rbx.wrapping_sub(0x30u64), rsi); crate::simd_ops::wu64(rbx.wrapping_sub(0x28u64), r14); crate::simd_ops::wu64(rbx.wrapping_sub(0x20u64), rsi); crate::simd_ops::wu64(rbx.wrapping_sub(0x18u64), rsi); crate::simd_ops::wu32(rbx.wrapping_sub(0x8u64), ((rdi as u32)) as u32); crate::simd_ops::wu64(rbx.wrapping_sub(0x10u64), rsi); rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x270u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } rcx = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } rax = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm((crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rbx.wrapping_sub(0x18u64), rax); bb = 2; continue; }
            2 => { crate::simd_ops::wu32(rbp.wrapping_add(0x288u64), ((rsi as u32)) as u32); rcx = (crate::simd_ops::ru64(r13)) as u64; rcx = ((rcx).wrapping_add(r15)) as u64; rdx = rsp.wrapping_add(0x50); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rsp.wrapping_add(0x50); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r15 = ((r15).wrapping_add(0xb0u64)) as u64; fl_l = r12; fl_r = (0x1u64) as u64; r12 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } r14 = (crate::simd_ops::ru64(rbp.wrapping_add(0x2e8u64))) as u64; rdi = (crate::simd_ops::ru64(rbp.wrapping_add(0x2d0u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x2e0u64))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rax as u32)) as u32); r9 = r14; r8 = rsi; rdx = (crate::simd_ops::ru64(r13)) as u64; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 9; continue; }
            4 => { rsi = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rsi); rax = rsp.wrapping_add(0x30); crate::simd_ops::wu64(rbp.wrapping_add(0x2d0u64), rax); rbx = rdi.wrapping_add(0x30); crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), (0x10000010u64) as u64); crate::simd_ops::wu8(rsp.wrapping_add(0x58u64), ((rsi as u8)) as u8); fl_l = r14; fl_r = r14; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 5; continue; } crate::simd_ops::wu32(rsp.wrapping_add(0x54u64), ((rsi as u32)) as u32); crate::simd_ops::wu8(rsp.wrapping_add(0x58u64), ((r14 as u8)) as u8); bb = 7; continue; }
            5 => { rdi = (0xffffffffffffffffu64) as u64; bb = 6; continue; }
            6 => { rdi = ((rdi).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru8(r14.wrapping_add(rdi))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 6; continue; } crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), r14); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rdi); r9 = rdi; r8 = 0; fl_l = 0; fl_r = 0; rdx = rsp.wrapping_add(0x40); rcx = rsp.wrapping_add(0x50); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 7; continue; }
            7 => { rdx = (crate::simd_ops::ru32(rbx.wrapping_add(0x8u64))) as u32 as u64; r8 = (crate::simd_ops::ru32(rbx.wrapping_add(0xcu64))) as u32 as u64; rcx = (((r8) as i64).wrapping_mul((0x88u64) as i64) as u64) as u64; rax = (((rdx) as i64).wrapping_mul((0x88u64) as i64) as u64) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 8; continue; } rax = rdx.wrapping_add(0x1); rdx = ((r8 as u32)) as u32 as u64; rdx = ((rdx).wrapping_shr(1u32)) as u64; rdx = ((rdx).wrapping_add(r8)) as u64; fl_l = rdx; fl_r = rax; rdx = (if (((fl_l as u64)) < ((fl_r as u64))) { rax } else { rdx }) as u64; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P8, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 8; continue; }
            8 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x8u64))) as u32 as u64; rcx = (((rax) as i64).wrapping_mul((0x88u64) as i64) as u64) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rbx))) as u64; crate::simd_ops::lk_and_u32(rcx, (0xff000010u32) as u32); crate::simd_ops::lk_or_u32(rcx, (0x10u32) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x3u64), (0x10u8) as u8); crate::simd_ops::wu32(rcx.wrapping_add(0x4u64), ((rsi as u32)) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x8u64), (0x0u8) as u8); rdx = rsp.wrapping_add(0x50); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P9, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::lk_add_u32(rbx.wrapping_add(0x8u64), (1) as u32); rcx = rsp.wrapping_add(0x50); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P10, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rsp.wrapping_add(0x30); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P11, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 9; continue; }
            9 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x3d8u64))) as u64; rsp = ((rsp).wrapping_add(0x390u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_22976<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x40u64)) as u64;
    r9 = (crate::simd_ops::ru32(P0)) as u32 as u64;
    rbx = rdx;
    r8 = (crate::simd_ops::ru64(P1)) as u64;
    rdx = rcx;
    rdi = rcx;
    crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), (0x1u32) as u32);
    rcx = rbx;
    crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), (0x1u32) as u32);
    rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp);
    fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(P3))) as u32 as u64; fl_r = 0x1;
    if !(((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32))) { r9 = P4; r8 = 0x1; rdx = rdi; rcx = rbx; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); }
    rcx = rbx;
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
pub unsafe extern "win64" fn tpl_22981<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
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
    rdx = rbx;
    rcx = rdi;
    rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp);
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
pub unsafe extern "win64" fn tpl_22989<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 121>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x3a0u64)) as u64; rdi = rdx; rdx = rcx; fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(0x54u64))) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) >= (((fl_r as u32)) as i32)) { bb = 5; continue; } rdx = 0; fl_l = 0; fl_r = 0; r8 = 0x310; rcx = rsp.wrapping_add(0x60); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rsi = rsp.wrapping_add(0x70); crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), rsi); r15 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rsp.wrapping_add(0x6cu64), (0x10u32) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x378u64), (0x6u32) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x380u64), r15); crate::simd_ops::wu32(rsp.wrapping_add(0x388u64), (0xffffffffu32) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x390u64), r15); crate::simd_ops::wu64(rsp.wrapping_add(0x370u64), r15); xmm0 = 0u128; ymm0_hi = 0; let y_addr: u64 = rsp.wrapping_add(0x70u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); crate::simd_ops::wu128(rsp.wrapping_add(0x90u64), (xmm0) as u128); rax = ((r15.wrapping_add(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x68u64), ((rax as u32)) as u32); rbx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rbx = ((rbx).wrapping_add(rbx)) as u64; crate::simd_ops::wu64(rsi.wrapping_add(rbx.wrapping_mul(8)).wrapping_sub(0x30u64), r15); rax = P1; crate::simd_ops::wu64(rsi.wrapping_add(rbx.wrapping_mul(8)).wrapping_sub(0x28u64), rax); crate::simd_ops::wu64(rsi.wrapping_add(rbx.wrapping_mul(8)).wrapping_sub(0x20u64), r15); crate::simd_ops::wu64(rsi.wrapping_add(rbx.wrapping_mul(8)).wrapping_sub(0x18u64), r15); crate::simd_ops::wu32(rsi.wrapping_add(rbx.wrapping_mul(8)).wrapping_sub(0x8u64), (0xffffffffu32) as u32); crate::simd_ops::wu64(rsi.wrapping_add(rbx.wrapping_mul(8)).wrapping_sub(0x10u64), r15); rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x380u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 1; continue; } rcx = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 1; continue; } rax = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm((crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rsi.wrapping_add(rbx.wrapping_mul(8)).wrapping_sub(0x18u64), rax); bb = 1; continue; }
            1 => { crate::simd_ops::wu32(rsp.wrapping_add(0x398u64), ((r15 as u32)) as u32); r14 = (crate::simd_ops::ru64(P2)) as u64; r9 = (crate::simd_ops::ru32(rsp.wrapping_add(0x68u64))) as u32 as u64; r8 = (crate::simd_ops::ru32(rsp.wrapping_add(0x6cu64))) as u32 as u64; rcx = ((r8.wrapping_add(r8.wrapping_mul(2))) as u64) as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rax = ((r9.wrapping_add(r9.wrapping_mul(2))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } rax = r9.wrapping_add(0x1); rdx = ((r8 as u32)) as u32 as u64; rdx = ((rdx).wrapping_shr(1u32)) as u64; rdx = ((rdx).wrapping_add(r8)) as u64; fl_l = rdx; fl_r = rax; rdx = (if (((fl_l as u64)) < ((fl_r as u64))) { rax } else { rdx }) as u64; rcx = rsp.wrapping_add(0x60); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r9 = (crate::simd_ops::ru32(rsp.wrapping_add(0x68u64))) as u32 as u64; bb = 2; continue; }
            2 => { rax = ((r9 as u32)) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rsp.wrapping_add(0x60u64)))) as u64; xmm0 = 0u128; ymm0_hi = 0; let y_addr: u64 = rcx; crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); crate::simd_ops::wu128(rcx.wrapping_add(0x20u64), (xmm0) as u128); rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x68u64))) as u32 as u64; rax = (((rax as u32)).wrapping_add(1)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x68u64), ((rax as u32)) as u32); rbx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rbx = ((rbx).wrapping_add(rbx)) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; crate::simd_ops::wu64(rsi.wrapping_add(rbx.wrapping_mul(8)).wrapping_sub(0x30u64), rdi); crate::simd_ops::wu64(rsi.wrapping_add(rbx.wrapping_mul(8)).wrapping_sub(0x28u64), r14); crate::simd_ops::wu32(rsi.wrapping_add(rbx.wrapping_mul(8)).wrapping_sub(0x20u64), (0x1u32) as u32); crate::simd_ops::wu32(rsi.wrapping_add(rbx.wrapping_mul(8)).wrapping_sub(0x1cu64), (0x1u32) as u32); crate::simd_ops::wu64(rsi.wrapping_add(rbx.wrapping_mul(8)).wrapping_sub(0x18u64), r15); crate::simd_ops::wu32(rsi.wrapping_add(rbx.wrapping_mul(8)).wrapping_sub(0x8u64), (0xffffffffu32) as u32); crate::simd_ops::wu64(rsi.wrapping_add(rbx.wrapping_mul(8)).wrapping_sub(0x10u64), r15); rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x380u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } rcx = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } rax = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm((crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rsi.wrapping_add(rbx.wrapping_mul(8)).wrapping_sub(0x18u64), rax); bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x398u64))) as u32 as u64; rax = (((rax as u32)).wrapping_shr(1u32)) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 4; continue; } xmm1 = 0u128; xmm0 = (crate::simd_ops::ru128(P4)) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm0) as u128); crate::simd_ops::wu128(rsp.wrapping_add(0x50u64), (xmm1) as u128); let y_addr: u64 = rsp.wrapping_add(0x40u64); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rdi; crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); bb = 4; continue; }
            4 => { rdx = (crate::simd_ops::ru32(rsp.wrapping_add(0x68u64))) as u32 as u64; rcx = ((rdx.wrapping_add(rdx.wrapping_mul(2))) as u64) as u64; rcx = ((rcx).wrapping_add(rcx)) as u64; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rcx = (crate::simd_ops::ru32(rax.wrapping_add(rcx.wrapping_mul(8)).wrapping_sub(0x8u64))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x388u64), ((rcx as u32)) as u32); rdx = (((rdx as u32)).wrapping_sub(1)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x68u64), ((rdx as u32)) as u32); rcx = rsp.wrapping_add(0x60); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rsp = ((rsp).wrapping_add(0x3a0u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            5 => { rcx = ((rcx).wrapping_add(0x20u64)) as u64; rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x18u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rcx); rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x50u64))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rax as u32)) as u32); r9 = rdi; rdx = (crate::simd_ops::ru64(rdx.wrapping_add(0x48u64))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rsp = ((rsp).wrapping_add(0x3a0u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_22999<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedStack::<1>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { fl_l = ((r8 as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 9; continue; } crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rbp = rsp.wrapping_sub(0x1260); rax = 0x1360; rsp = ((rsp).wrapping_sub(rax)) as u64; rdi = rdx; rsi = rcx; r14 = ((((r8 as u32)) as i32 as i64) as u64) as u64; r12 = 0; fl_l = 0; fl_r = 0; r15 = P0; r13 = (0x2aaaaaaaaaaaaaabu64) as u64; bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru32(rsi.wrapping_add(0x328u64))) as u32 as u64; fl_r = 0x0; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 2; continue; } rdx = rsi; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 8; continue; }
            2 => { fl_l = (crate::simd_ops::ru8(rsi.wrapping_add(0x369u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 8; continue; } rdx = 0; fl_l = 0; fl_r = 0; r8 = 0x310; rcx = rsp.wrapping_add(0x20); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rsp.wrapping_add(0x30); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rcx); crate::simd_ops::wu32(rsp.wrapping_add(0x2cu64), (0x10u32) as u32); crate::simd_ops::wu32(rbp.wrapping_add(0x238u64), (0x6u32) as u32); crate::simd_ops::wu64(rbp.wrapping_add(0x240u64), r12); crate::simd_ops::wu32(rbp.wrapping_add(0x248u64), (0xffffffffu32) as u32); crate::simd_ops::wu64(rbp.wrapping_add(0x250u64), r12); crate::simd_ops::wu64(rbp.wrapping_add(0x230u64), r12); xmm0 = 0u128; ymm0_hi = 0; let y_addr: u64 = rsp.wrapping_add(0x30u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); crate::simd_ops::wu128(rsp.wrapping_add(0x50u64), (xmm0) as u128); rax = ((r12.wrapping_add(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((rax as u32)) as u32); rbx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rbx = ((rbx).wrapping_shl(4u32)) as u64; rbx = ((rbx).wrapping_add(rcx)) as u64; crate::simd_ops::wu64(rbx.wrapping_sub(0x30u64), r12); crate::simd_ops::wu64(rbx.wrapping_sub(0x28u64), r15); crate::simd_ops::wu64(rbx.wrapping_sub(0x20u64), r12); crate::simd_ops::wu64(rbx.wrapping_sub(0x18u64), r12); crate::simd_ops::wu32(rbx.wrapping_sub(0x8u64), (0xffffffffu32) as u32); crate::simd_ops::wu64(rbx.wrapping_sub(0x10u64), r12); rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x240u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } rcx = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } rax = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm((crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rbx.wrapping_sub(0x18u64), rax); bb = 3; continue; }
            3 => { crate::simd_ops::wu32(rbp.wrapping_add(0x258u64), ((r12 as u32)) as u32); r15 = (crate::simd_ops::ru64(P3)) as u64; rdx = (crate::simd_ops::ru32(rsp.wrapping_add(0x28u64))) as u32 as u64; r8 = ((rdx.wrapping_add(rdx.wrapping_mul(2))) as u64) as u64; r8 = ((r8).wrapping_shl(4u32)) as u64; r9 = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; r8 = ((r8).wrapping_add(r9)) as u64; rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x2cu64))) as u32 as u64; r10 = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; r10 = ((r10).wrapping_shl(4u32)) as u64; r10 = ((r10).wrapping_add(r9)) as u64; fl_l = r8; fl_r = r10; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 4; continue; } r8 = ((r8).wrapping_sub(r9)) as u64; rax = r13; let prod = (rax as i64 as i128).wrapping_mul((r8) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; rdx = ((((rdx) as i64).wrapping_shr(3u32) as u64)) as u64; r8 = rdx; r8 = ((r8).wrapping_shr(63u32)) as u64; rdx = ((rdx).wrapping_add(1)) as u64; r8 = ((r8).wrapping_add(rdx)) as u64; r10 = ((r10).wrapping_sub(r9)) as u64; rax = r13; let prod = (rax as i64 as i128).wrapping_mul((r10) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; rcx = rdx; rcx = ((((rcx) as i64).wrapping_shr(3u32) as u64)) as u64; rax = rcx; rax = ((rax).wrapping_shr(63u32)) as u64; rcx = ((rcx).wrapping_add(rax)) as u64; rdx = rcx; rdx = ((rdx).wrapping_shr(1u32)) as u64; rdx = ((rdx).wrapping_add(rcx)) as u64; fl_l = rdx; fl_r = r8; rdx = (if (((fl_l as u64)) < ((fl_r as u64))) { r8 } else { rdx }) as u64; rcx = rsp.wrapping_add(0x20); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = (crate::simd_ops::ru32(rsp.wrapping_add(0x28u64))) as u32 as u64; r9 = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; bb = 4; continue; }
            4 => { rax = ((rdx as u32)) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rcx = ((rcx).wrapping_add(rcx)) as u64; xmm0 = 0u128; ymm0_hi = 0; let y_addr: u64 = r9.wrapping_add(rcx.wrapping_mul(8)); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); crate::simd_ops::wu128(r9.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x20u64), (xmm0) as u128); rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x28u64))) as u32 as u64; rax = (((rax as u32)).wrapping_add(1)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((rax as u32)) as u32); rbx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rbx = ((rbx).wrapping_shl(4u32)) as u64; rbx = ((rbx).wrapping_add(crate::simd_ops::ru64(rsp.wrapping_add(0x20u64)))) as u64; crate::simd_ops::wu64(rbx.wrapping_sub(0x30u64), rdi); crate::simd_ops::wu64(rbx.wrapping_sub(0x28u64), r15); crate::simd_ops::wu32(rbx.wrapping_sub(0x20u64), (0x4u32) as u32); crate::simd_ops::wu32(rbx.wrapping_sub(0x1cu64), (0x4u32) as u32); crate::simd_ops::wu64(rbx.wrapping_sub(0x18u64), r12); crate::simd_ops::wu32(rbx.wrapping_sub(0x8u64), (0xffffffffu32) as u32); crate::simd_ops::wu64(rbx.wrapping_sub(0x10u64), r12); rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x240u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } rdx = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } rax = (crate::simd_ops::ru64(rdx)) as u64; rcx = rdx; (rax, xmm0) = crate::simd_ops::call_stk_xmm((crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rbx.wrapping_sub(0x18u64), rax); rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x240u64))) as u64; bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x238u64))) as u32 as u64; rax = (((rax as u32)).wrapping_sub(1)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x3; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 6; continue; } fl_l = (crate::simd_ops::ru32(rbp.wrapping_add(0x248u64))) as u32 as u64; fl_r = 0x2; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 6; continue; } fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } rcx = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; r8 = 0x28; rdx = rbp.wrapping_add(0x260); (rax, xmm0) = crate::simd_ops::call_stk_xmm((crate::simd_ops::ru64(rax.wrapping_add(0x28u64))) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x258u64))) as u32 as u64; rax = (((rax as u32)).wrapping_shr(1u32)) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 7; continue; } crate::simd_ops::wu32(rdi, ((r12 as u32)) as u32); crate::simd_ops::wu32(rdi.wrapping_add(0x4u64), (0x3f800000u32) as u32); bb = 7; continue; }
            7 => { r8 = (crate::simd_ops::ru32(rsp.wrapping_add(0x28u64))) as u32 as u64; rcx = ((r8.wrapping_add(r8.wrapping_mul(2))) as u64) as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rax = r13; let prod = (rax as i64 as i128).wrapping_mul((rcx) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; rdx = ((((rdx) as i64).wrapping_shr(3u32) as u64)) as u64; rax = rdx; rax = ((rax).wrapping_shr(63u32)) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; rcx = ((rdx.wrapping_add(rdx.wrapping_mul(2))) as u64) as u64; rcx = ((rcx).wrapping_add(rcx)) as u64; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; rcx = (crate::simd_ops::ru32(rax.wrapping_add(rcx.wrapping_mul(8)).wrapping_sub(0x8u64))) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x248u64), ((rcx as u32)) as u32); r8 = (((r8 as u32)).wrapping_sub(1)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((r8 as u32)) as u32); rcx = rsp.wrapping_add(0x20); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r15 = P0; bb = 8; continue; }
            8 => { rdi = ((rdi).wrapping_add(0x8u64)) as u64; fl_l = r14; fl_r = (0x1u64) as u64; r14 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } r11 = rsp.wrapping_add(0x1360); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x38u64))) as u64; rdi = (crate::simd_ops::ru64(r11.wrapping_add(0x40u64))) as u64; rsp = r11; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); bb = 9; continue; }
            9 => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_23002<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64, const P9: u64, const P10: u64, const P11: u64, const P12: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 53>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { r11 = rsp; crate::simd_ops::wu64(r11.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(r11.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x1a0u64)) as u64; rbx = r9; rsi = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rsi); rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x8u64))) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; rax = (0xffffffffffffu64) as u64; rcx = (((rcx) & (rax))) as u64; rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x1d0u64))) as u32 as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rcx = ((rcx).wrapping_add(rax)) as u64; fl_l = (crate::simd_ops::ru16(rcx.wrapping_add(0xeu64))) as u16 as u64; fl_r = (0x3u16) as u16 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { bb = 18; continue; } crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rcx); rax = (crate::simd_ops::ru64(rdx)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rax); xmm0 = 0u128; ymm0_hi = 0; crate::simd_ops::wu128(rsp.wrapping_add(0x60u64), (xmm0) as u128); crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), rsi); crate::simd_ops::wu8(rsp.wrapping_add(0x70u64), ((rsi as u8)) as u8); let y_addr: u64 = rsp.wrapping_add(0x80u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); crate::simd_ops::wu64(r11.wrapping_sub(0x128u64), rsi); crate::simd_ops::wu64(r11.wrapping_sub(0x120u64), rsi); crate::simd_ops::wu64(r11.wrapping_sub(0x118u64), rsi); crate::simd_ops::wu64(r11.wrapping_sub(0x110u64), rsi); rax = rsp.wrapping_add(0x40); crate::simd_ops::wu64(r11.wrapping_sub(0x108u64), rax); crate::simd_ops::wu64(r11.wrapping_sub(0x100u64), rsi); crate::simd_ops::wu32(rsp.wrapping_add(0xb0u64), ((rsi as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0xb4u64), (0xffffffffu32) as u32); rax = (crate::simd_ops::ru32(rsp.wrapping_add(0xb8u64))) as u32 as u64; rax = ((((rax as u32)) & (0xff000010u32))) as u32 as u64; rax = ((((rax as u32)) | (0x10u32))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0xb8u64), ((rax as u32)) as u32); crate::simd_ops::wu8(rsp.wrapping_add(0xbbu64), (0x10u8) as u8); crate::simd_ops::wu32(rsp.wrapping_add(0xbcu64), ((rsi as u32)) as u32); crate::simd_ops::wu8(rsp.wrapping_add(0xc0u64), ((rsi as u8)) as u8); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x1e8u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x78u64), rax); crate::simd_ops::wu8(rsp.wrapping_add(0x38u64), ((rsi as u8)) as u8); rax = P0; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), (0x1u32) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), (0x1u32) as u32); r8 = (crate::simd_ops::ru64(P1)) as u64; rcx = rsp.wrapping_add(0x60); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r9 = P3; r8 = ((rsi.wrapping_add(0x1u64)) as u32) as u32 as u64; rdx = rbx; rcx = rsp.wrapping_add(0x60); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x88u64))) as u32 as u64; rax = (((rax) as i64).wrapping_mul((0x108u64) as i64) as u64) as u64; rax = ((rax).wrapping_add(crate::simd_ops::ru64(rsp.wrapping_add(0x80u64)))) as u64; fl_l = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; fl_r = rax; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 19; continue; } rcx = rsp.wrapping_add(0xb8); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x98u64))) as u32 as u64; rdi = (((rax) as i64).wrapping_mul((0x88u64) as i64) as u64) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x90u64))) as u64; rdi = ((rdi).wrapping_add(rbx)) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = ((rbx).wrapping_add(0x88u64)) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x90u64))) as u64; bb = 2; continue; }
            2 => { fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } rbx = ((rbx).wrapping_sub(1)) as u64; rbx = (((rbx) & (0xfffffffffe000000u64))) as u64; rax = ((crate::exe::teb_self() as u64)) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(rbx.wrapping_add(0x100u64))) as u64; rcx = (rcx & !0xffu64) | (((if (((fl_l as u64)) == ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); r9 = (crate::simd_ops::ru64(rsp.wrapping_add(0x90u64))) as u64; rax = r9; rax = ((rax).wrapping_sub(rbx)) as u64; rax = ((rax).wrapping_shr(16u32)) as u64; rdx = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rdx = ((rdx).wrapping_shl(4u32)) as u64; rdx = ((rdx).wrapping_add(0x108u64)) as u64; rdx = ((rdx).wrapping_add(rbx)) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x4u64))) as u32 as u64; rdx = ((rdx).wrapping_sub(rax)) as u64; fl_l = ((rcx as u8)) as u8 as u64; fl_r = ((rcx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 4; continue; } fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(0xeu64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 3; continue; } rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(r9, rax); crate::simd_ops::wu64(rdx.wrapping_add(0x20u64), r9); fl_l = (crate::simd_ops::lk_add_u32(rdx.wrapping_add(0x18u64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; if (((fl_l as u32)) != 0) { bb = 6; continue; } rcx = rdx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 6; continue; }
            3 => { r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 5; continue; }
            4 => { r8 = 0; fl_l = 0; fl_r = 0; bb = 5; continue; }
            5 => { rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 6; continue; }
            6 => { crate::simd_ops::wu64(rsp.wrapping_add(0x90u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x98u64), rsi); rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x88u64))) as u32 as u64; rdi = (((rax) as i64).wrapping_mul((0x108u64) as i64) as u64) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; rdi = ((rdi).wrapping_add(rbx)) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 8; continue; } bb = 7; continue; }
            7 => { rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = ((rbx).wrapping_add(0x108u64)) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 7; continue; } rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; bb = 8; continue; }
            8 => { fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } rbx = ((rbx).wrapping_sub(1)) as u64; rbx = (((rbx) & (0xfffffffffe000000u64))) as u64; rax = ((crate::exe::teb_self() as u64)) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(rbx.wrapping_add(0x100u64))) as u64; rcx = (rcx & !0xffu64) | (((if (((fl_l as u64)) == ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); r9 = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; rax = r9; rax = ((rax).wrapping_sub(rbx)) as u64; rax = ((rax).wrapping_shr(16u32)) as u64; rdx = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rdx = ((rdx).wrapping_shl(4u32)) as u64; rdx = ((rdx).wrapping_add(0x108u64)) as u64; rdx = ((rdx).wrapping_add(rbx)) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x4u64))) as u32 as u64; rdx = ((rdx).wrapping_sub(rax)) as u64; fl_l = ((rcx as u8)) as u8 as u64; fl_r = ((rcx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 10; continue; } fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(0xeu64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 9; continue; } rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(r9, rax); crate::simd_ops::wu64(rdx.wrapping_add(0x20u64), r9); fl_l = (crate::simd_ops::lk_add_u32(rdx.wrapping_add(0x18u64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; if (((fl_l as u32)) != 0) { bb = 12; continue; } rcx = rdx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 12; continue; }
            9 => { r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 11; continue; }
            10 => { r8 = 0; fl_l = 0; fl_r = 0; bb = 11; continue; }
            11 => { rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 12; continue; }
            12 => { crate::simd_ops::wu64(rsp.wrapping_add(0x80u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x88u64), rsi); rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x68u64))) as u32 as u64; rdi = (((rax) as i64).wrapping_mul((0x108u64) as i64) as u64) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rdi = ((rdi).wrapping_add(rbx)) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 14; continue; } bb = 13; continue; }
            13 => { rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = ((rbx).wrapping_add(0x108u64)) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 13; continue; } rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; bb = 14; continue; }
            14 => { fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 18; continue; } rbx = ((rbx).wrapping_sub(1)) as u64; rbx = (((rbx) & (0xfffffffffe000000u64))) as u64; rax = ((crate::exe::teb_self() as u64)) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(rbx.wrapping_add(0x100u64))) as u64; rcx = (rcx & !0xffu64) | (((if (((fl_l as u64)) == ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); r9 = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rax = r9; rax = ((rax).wrapping_sub(rbx)) as u64; rax = ((rax).wrapping_shr(16u32)) as u64; rdx = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rdx = ((rdx).wrapping_shl(4u32)) as u64; rdx = ((rdx).wrapping_add(0x108u64)) as u64; rdx = ((rdx).wrapping_add(rbx)) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x4u64))) as u32 as u64; rdx = ((rdx).wrapping_sub(rax)) as u64; fl_l = ((rcx as u8)) as u8 as u64; fl_r = ((rcx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 16; continue; } fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(0xeu64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 15; continue; } rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(r9, rax); crate::simd_ops::wu64(rdx.wrapping_add(0x20u64), r9); fl_l = (crate::simd_ops::lk_add_u32(rdx.wrapping_add(0x18u64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; if (((fl_l as u32)) != 0) { bb = 18; continue; } rcx = rdx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 18; continue; }
            15 => { r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 17; continue; }
            16 => { r8 = 0; fl_l = 0; fl_r = 0; bb = 17; continue; }
            17 => { rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 18; continue; }
            18 => { r11 = rsp.wrapping_add(0x1a0); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x10u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x18u64))) as u64; rsp = r11; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            19 => { rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0xa0u64))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P8, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P9, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r8 = rax; rdx = rsp.wrapping_add(0x80); rcx = rsp.wrapping_add(0x178); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P10, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P11, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::lift_ops::trap(0x80000003, P12, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_23016<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64, const P9: u64, const P10: u64, const P11: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedStack::<1>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rax = 0x1040; rsp = ((rsp).wrapping_sub(rax)) as u64; rbx = rdx; rdi = rcx; crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), (0x4u32) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), (0x4u32) as u32); r8 = (crate::simd_ops::ru64(P0)) as u64; rdx = rcx; rcx = rbx; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru32(rbx.wrapping_add(P2))) as u32 as u64; rax = (((rax as u32)).wrapping_sub(1)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x3; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 1; continue; } fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(P3))) as u32 as u64; fl_r = 0x2; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 1; continue; } rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P4))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 1; continue; } rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P5))) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; r8 = 0x28; rdx = rsp.wrapping_add(0x40); rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(rax.wrapping_add(P6))) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 1; continue; }
            1 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P7))) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = rdi; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(rax.wrapping_add(P8))) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P7))) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = ((rdi.wrapping_add(P9)) as u64) as u64; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(rax.wrapping_add(P10))) as u64, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = (crate::simd_ops::ru32(rbx.wrapping_add(P5))) as u32 as u64; rcx = ((rdx.wrapping_add(rdx.wrapping_mul(2))) as u64) as u64; rcx = ((rcx).wrapping_add(rcx)) as u64; rcx = (crate::simd_ops::ru32(rax.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(P11))) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x328u64), ((rcx as u32)) as u32); rax = ((rdx.wrapping_sub(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x8u64), ((rax as u32)) as u32); rax = 0; fl_l = 0; fl_r = 0; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x1050u64))) as u64; rsp = ((rsp).wrapping_add(0x1040u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_23024<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedStack::<1>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rbp = rsp.wrapping_sub(0x1280); rax = 0x1380; rsp = ((rsp).wrapping_sub(rax)) as u64; rdi = rdx; rdx = rcx; fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(0x54u64))) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) >= (((fl_r as u32)) as i32)) { bb = 6; continue; } rdx = 0; fl_l = 0; fl_r = 0; r8 = 0x310; rcx = rsp.wrapping_add(0x40); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rsp.wrapping_add(0x50); crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rcx); r14 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rsp.wrapping_add(0x4cu64), (0x10u32) as u32); crate::simd_ops::wu32(rbp.wrapping_add(0x258u64), (0x6u32) as u32); crate::simd_ops::wu64(rbp.wrapping_add(0x260u64), r14); crate::simd_ops::wu32(rbp.wrapping_add(0x268u64), (0xffffffffu32) as u32); crate::simd_ops::wu64(rbp.wrapping_add(0x270u64), r14); crate::simd_ops::wu64(rbp.wrapping_add(0x250u64), r14); xmm0 = 0u128; ymm0_hi = 0; let y_addr: u64 = rsp.wrapping_add(0x50u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); crate::simd_ops::wu128(rsp.wrapping_add(0x70u64), (xmm0) as u128); rax = ((r14.wrapping_add(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x48u64), ((rax as u32)) as u32); rbx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rbx = ((rbx).wrapping_shl(4u32)) as u64; rbx = ((rbx).wrapping_add(rcx)) as u64; crate::simd_ops::wu64(rbx.wrapping_sub(0x30u64), r14); rax = P1; crate::simd_ops::wu64(rbx.wrapping_sub(0x28u64), rax); crate::simd_ops::wu64(rbx.wrapping_sub(0x20u64), r14); crate::simd_ops::wu64(rbx.wrapping_sub(0x18u64), r14); crate::simd_ops::wu32(rbx.wrapping_sub(0x8u64), (0xffffffffu32) as u32); crate::simd_ops::wu64(rbx.wrapping_sub(0x10u64), r14); rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x260u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 1; continue; } rcx = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 1; continue; } rax = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm((crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rbx.wrapping_sub(0x18u64), rax); bb = 1; continue; }
            1 => { crate::simd_ops::wu32(rbp.wrapping_add(0x278u64), ((r14 as u32)) as u32); rsi = (crate::simd_ops::ru64(P2)) as u64; r9 = (crate::simd_ops::ru32(rsp.wrapping_add(0x48u64))) as u32 as u64; r8 = (crate::simd_ops::ru32(rsp.wrapping_add(0x4cu64))) as u32 as u64; rcx = ((r8.wrapping_add(r8.wrapping_mul(2))) as u64) as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rax = ((r9.wrapping_add(r9.wrapping_mul(2))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } rax = r9.wrapping_add(0x1); rdx = ((r8 as u32)) as u32 as u64; rdx = ((rdx).wrapping_shr(1u32)) as u64; rdx = ((rdx).wrapping_add(r8)) as u64; fl_l = rdx; fl_r = rax; rdx = (if (((fl_l as u64)) < ((fl_r as u64))) { rax } else { rdx }) as u64; rcx = rsp.wrapping_add(0x40); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r9 = (crate::simd_ops::ru32(rsp.wrapping_add(0x48u64))) as u32 as u64; bb = 2; continue; }
            2 => { rax = ((r9 as u32)) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rsp.wrapping_add(0x40u64)))) as u64; xmm0 = 0u128; ymm0_hi = 0; let y_addr: u64 = rcx; crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); crate::simd_ops::wu128(rcx.wrapping_add(0x20u64), (xmm0) as u128); rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x48u64))) as u32 as u64; rax = (((rax as u32)).wrapping_add(1)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x48u64), ((rax as u32)) as u32); rbx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rbx = ((rbx).wrapping_shl(4u32)) as u64; rbx = ((rbx).wrapping_add(crate::simd_ops::ru64(rsp.wrapping_add(0x40u64)))) as u64; crate::simd_ops::wu64(rbx.wrapping_sub(0x30u64), rdi); crate::simd_ops::wu64(rbx.wrapping_sub(0x28u64), rsi); crate::simd_ops::wu32(rbx.wrapping_sub(0x20u64), (0x4u32) as u32); crate::simd_ops::wu32(rbx.wrapping_sub(0x1cu64), (0x4u32) as u32); crate::simd_ops::wu64(rbx.wrapping_sub(0x18u64), r14); crate::simd_ops::wu32(rbx.wrapping_sub(0x8u64), (0xffffffffu32) as u32); crate::simd_ops::wu64(rbx.wrapping_sub(0x10u64), r14); rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x260u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } rdx = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } rax = (crate::simd_ops::ru64(rdx)) as u64; rcx = rdx; (rax, xmm0) = crate::simd_ops::call_stk_xmm((crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rbx.wrapping_sub(0x18u64), rax); rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x260u64))) as u64; bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x258u64))) as u32 as u64; rax = (((rax as u32)).wrapping_sub(1)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x3; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 4; continue; } fl_l = (crate::simd_ops::ru32(rbp.wrapping_add(0x268u64))) as u32 as u64; fl_r = 0x2; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 4; continue; } fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } rcx = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; r8 = 0x28; rdx = rbp.wrapping_add(0x280); (rax, xmm0) = crate::simd_ops::call_stk_xmm((crate::simd_ops::ru64(rax.wrapping_add(0x28u64))) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x278u64))) as u32 as u64; rax = (((rax as u32)).wrapping_shr(1u32)) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 5; continue; } crate::simd_ops::wu32(rdi, ((r14 as u32)) as u32); crate::simd_ops::wu32(rdi.wrapping_add(0x4u64), (0x3f800000u32) as u32); bb = 5; continue; }
            5 => { r8 = (crate::simd_ops::ru32(rsp.wrapping_add(0x48u64))) as u32 as u64; rcx = ((r8.wrapping_add(r8.wrapping_mul(2))) as u64) as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rax = (0x2aaaaaaaaaaaaaabu64) as u64; let prod = (rax as i64 as i128).wrapping_mul((rcx) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; rdx = ((((rdx) as i64).wrapping_shr(3u32) as u64)) as u64; rax = rdx; rax = ((rax).wrapping_shr(63u32)) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; rcx = ((rdx.wrapping_add(rdx.wrapping_mul(2))) as u64) as u64; rcx = ((rcx).wrapping_add(rcx)) as u64; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rcx = (crate::simd_ops::ru32(rax.wrapping_add(rcx.wrapping_mul(8)).wrapping_sub(0x8u64))) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x268u64), ((rcx as u32)) as u32); r8 = (((r8 as u32)).wrapping_sub(1)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x48u64), ((r8 as u32)) as u32); rcx = rsp.wrapping_add(0x40); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rsp = ((rsp).wrapping_add(0x1380u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            6 => { rcx = ((rcx).wrapping_add(0x20u64)) as u64; rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x18u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rcx); rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x50u64))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rax as u32)) as u32); r9 = rdi; rdx = (crate::simd_ops::ru64(rdx.wrapping_add(0x48u64))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rsp = ((rsp).wrapping_add(0x1380u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_23028<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64, const P9: u64, const P10: u64, const P11: u64, const P12: u64, const P13: u64, const P14: u64, const P15: u64, const P16: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x60u64)) as u64; rdi = rdx; rbx = rcx; crate::simd_ops::wu8(rsp.wrapping_add(0x38u64), ((r8 as u8)) as u8); rax = P0; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), (0x4u32) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), (0x4u32) as u32); r8 = (crate::simd_ops::ru64(P1)) as u64; rcx = rdx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rsi = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rsp.wrapping_add(0x70u64), ((rsi as u32)) as u32); rax = rsp.wrapping_add(0x70); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); r9 = P3; r8 = ((rsi.wrapping_add(P4)) as u32) as u32 as u64; rdx = rbx; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm0 = ((crate::simd_ops::ru32(P6)) as u32) as u128; crate::simd_ops::wu32(rsp.wrapping_add(0x70u64), ((xmm0 as u32)) as u32); rdx = ((rbx.wrapping_add(P4)) as u64) as u64; rax = rsp.wrapping_add(0x70); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); r9 = P7; r8 = ((rsi.wrapping_add(P8)) as u32) as u32 as u64; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P9, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru32(rdi.wrapping_add(P10))) as u32 as u64; rcx = (((rax) as i64).wrapping_mul((0x108u64) as i64) as u64) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rdi.wrapping_add(P11)))) as u64; fl_l = (crate::simd_ops::ru64(rdi.wrapping_add(0x20u64))) as u64; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } rax = 0; fl_l = 0; fl_r = 0; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; rsp = ((rsp).wrapping_add(0x60u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            1 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x40u64))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P12, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P13, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r8 = rax; rdx = rdi.wrapping_add(0x20); rcx = rsp.wrapping_add(0x40); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P14, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P15, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::lift_ops::trap(0x80000003, P16, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_23029<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x38u64)) as u64; rax = ((crate::exe::tls_pointer() as u64)) as u64; rdx = 0x6348; rcx = (crate::simd_ops::ru64(rax)) as u64; rax = (crate::simd_ops::ru64(rdx.wrapping_add(rcx))) as u64; rdx = (crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64; rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P1))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 1; continue; } fl_l = ((rcx as u8)) as u8 as u64; fl_r = (0xfu8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 1; continue; } fl_l = (crate::simd_ops::lk_add_u32(rdx.wrapping_add(P2), (1) as u32)) as u32 as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; crate::simd_ops::wu64(rdx.wrapping_add(P1), rax); bb = 2; continue; }
            1 => { rdx = 0xf8; r8 = 0x10; rcx = rax; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rcx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 2; continue; } rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P4)) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = rax; rdx = 0xdeadd00d; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P5)) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = 0; fl_l = 0; fl_r = 0; bb = 2; continue; }
            2 => { rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); rsp = ((rsp).wrapping_add(0x38u64)) as u64; return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_23039<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; r8 = (crate::simd_ops::ru64(P0)) as u64; rdi = rdx; rdx = rcx; crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), (0x4u32) as u32); rbx = rcx; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), (0x4u32) as u32); rcx = rdi; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); r8 = (crate::simd_ops::ru64(P2)) as u64; rdx = rbx; rcx = rdi; crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), (0x1u32) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), (0x1u32) as u32); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rdx = (crate::simd_ops::ru32(rdi.wrapping_add(P3))) as u32 as u64; rax = (crate::simd_ops::ru64(rdi)) as u64; rcx = ((rdx.wrapping_add(rdx.wrapping_mul(2))) as u64) as u64; rcx = ((rcx).wrapping_add(rcx)) as u64; rcx = (crate::simd_ops::ru32(rax.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(P4))) as u32 as u64; rax = ((rdx.wrapping_add(P5)) as u32) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(P6), ((rcx as u32)) as u32); rdx = rdi; rcx = ((rbx.wrapping_add(P7)) as u64) as u64; crate::simd_ops::wu32(rdi.wrapping_add(P3), ((rax as u32)) as u32); rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); rdx = (crate::simd_ops::ru32(rdi.wrapping_add(P3))) as u32 as u64; rax = (crate::simd_ops::ru64(rdi)) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rcx = ((rdx.wrapping_add(rdx.wrapping_mul(2))) as u64) as u64; rcx = ((rcx).wrapping_add(rcx)) as u64; rcx = (crate::simd_ops::ru32(rax.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(P4))) as u32 as u64; rax = ((rdx.wrapping_add(P5)) as u32) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(P3), ((rax as u32)) as u32); rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rdi.wrapping_add(P6), ((rcx as u32)) as u32); rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_23040<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; r8 = (crate::simd_ops::ru64(P0)) as u64; rdi = rdx; rdx = rcx; crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), (0x4u32) as u32); rbx = rcx; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), (0x4u32) as u32); rcx = rdi; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); r8 = (crate::simd_ops::ru64(P2)) as u64; rdx = rbx; rcx = rdi; crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), (0x1u32) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), (0x1u32) as u32); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rdx = (crate::simd_ops::ru32(rdi.wrapping_add(P3))) as u32 as u64; rax = (crate::simd_ops::ru64(rdi)) as u64; rcx = ((rdx.wrapping_add(rdx.wrapping_mul(2))) as u64) as u64; rcx = ((rcx).wrapping_add(rcx)) as u64; rcx = (crate::simd_ops::ru32(rax.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(P4))) as u32 as u64; rax = ((rdx.wrapping_add(P5)) as u32) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(P6), ((rcx as u32)) as u32); rdx = rdi; rcx = ((rbx.wrapping_add(P7)) as u64) as u64; crate::simd_ops::wu32(rdi.wrapping_add(P3), ((rax as u32)) as u32); rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); rdx = (crate::simd_ops::ru32(rdi.wrapping_add(P3))) as u32 as u64; rax = (crate::simd_ops::ru64(rdi)) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rcx = ((rdx.wrapping_add(rdx.wrapping_mul(2))) as u64) as u64; rcx = ((rcx).wrapping_add(rcx)) as u64; rcx = (crate::simd_ops::ru32(rax.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(P4))) as u32 as u64; rax = ((rdx.wrapping_add(P5)) as u32) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(P3), ((rax as u32)) as u32); rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rdi.wrapping_add(P6), ((rcx as u32)) as u32); rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_23043<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64, const P9: u64, const P10: u64, const P11: u64, const P12: u64, const P13: u64, const P14: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x60u64)) as u64; rdi = rdx; rbx = rcx; crate::simd_ops::wu8(rsp.wrapping_add(0x38u64), ((r8 as u8)) as u8); rax = P0; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), (0x4u32) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), (0x4u32) as u32); r8 = (crate::simd_ops::ru64(P1)) as u64; rcx = rdx; rax = crate::simd_ops::call_stk_caught(P2, rax, rcx, rdx, r8, r9, rsp); rcx = rdi; rax = crate::simd_ops::call_stk_caught(P3, rax, rcx, rdx, r8, r9, rsp); rdx = ((rbx.wrapping_add(P4)) as u64) as u64; r9 = P5; r8 = 0x4; rcx = rdi; rax = crate::simd_ops::call_stk_caught(P6, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru32(rdi.wrapping_add(P7))) as u32 as u64; rcx = (((rax) as i64).wrapping_mul((0x108u64) as i64) as u64) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rdi.wrapping_add(P8)))) as u64; fl_l = (crate::simd_ops::ru64(rdi.wrapping_add(P8))) as u64; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } rax = 0; fl_l = 0; fl_r = 0; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rsp = ((rsp).wrapping_add(0x60u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            1 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(P9))) as u64; rax = crate::simd_ops::call_stk_caught(P10, rax, rcx, rdx, r8, r9, rsp); rcx = rax; rax = crate::simd_ops::call_stk_caught(P11, rax, rcx, rdx, r8, r9, rsp); r8 = rax; rdx = rdi.wrapping_add(0x20); rcx = rsp.wrapping_add(0x40); rax = crate::simd_ops::call_stk_caught(P12, rax, rcx, rdx, r8, r9, rsp); rcx = rax; rax = crate::simd_ops::call_stk_caught(P13, rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, P14, rcx, rsp); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_23048<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 109>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rdi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rbp = rsp.wrapping_sub(0x260); rsp = ((rsp).wrapping_sub(0x360u64)) as u64; rdi = rdx; fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(0x328u64))) as u32 as u64; fl_r = 0x4; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 1; continue; } rdx = rcx; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 7; continue; }
            1 => { fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0x369u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 7; continue; } rdx = 0; fl_l = 0; fl_r = 0; r8 = 0x310; rcx = rsp.wrapping_add(0x20); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rsp.wrapping_add(0x30); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rcx); r14 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rsp.wrapping_add(0x2cu64), (0x10u32) as u32); crate::simd_ops::wu32(rbp.wrapping_add(0x238u64), (0x6u32) as u32); crate::simd_ops::wu64(rbp.wrapping_add(0x240u64), r14); crate::simd_ops::wu32(rbp.wrapping_add(0x248u64), (0xffffffffu32) as u32); crate::simd_ops::wu64(rbp.wrapping_add(0x250u64), r14); crate::simd_ops::wu64(rbp.wrapping_add(0x230u64), r14); xmm0 = 0u128; ymm0_hi = 0; let y_addr: u64 = rsp.wrapping_add(0x30u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); crate::simd_ops::wu128(rsp.wrapping_add(0x50u64), (xmm0) as u128); rax = ((r14.wrapping_add(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((rax as u32)) as u32); rbx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rbx = ((rbx).wrapping_shl(4u32)) as u64; rbx = ((rbx).wrapping_add(rcx)) as u64; crate::simd_ops::wu64(rbx.wrapping_sub(0x30u64), r14); rax = P2; crate::simd_ops::wu64(rbx.wrapping_sub(0x28u64), rax); crate::simd_ops::wu64(rbx.wrapping_sub(0x20u64), r14); crate::simd_ops::wu64(rbx.wrapping_sub(0x18u64), r14); crate::simd_ops::wu32(rbx.wrapping_sub(0x8u64), (0xffffffffu32) as u32); crate::simd_ops::wu64(rbx.wrapping_sub(0x10u64), r14); rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x240u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } rcx = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } rax = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm((crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rbx.wrapping_sub(0x18u64), rax); bb = 2; continue; }
            2 => { crate::simd_ops::wu32(rbp.wrapping_add(0x258u64), ((r14 as u32)) as u32); rsi = (crate::simd_ops::ru64(P3)) as u64; r9 = (crate::simd_ops::ru32(rsp.wrapping_add(0x28u64))) as u32 as u64; r8 = (crate::simd_ops::ru32(rsp.wrapping_add(0x2cu64))) as u32 as u64; rcx = ((r8.wrapping_add(r8.wrapping_mul(2))) as u64) as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rax = ((r9.wrapping_add(r9.wrapping_mul(2))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } rax = r9.wrapping_add(0x1); rdx = ((r8 as u32)) as u32 as u64; rdx = ((rdx).wrapping_shr(1u32)) as u64; rdx = ((rdx).wrapping_add(r8)) as u64; fl_l = rdx; fl_r = rax; rdx = (if (((fl_l as u64)) < ((fl_r as u64))) { rax } else { rdx }) as u64; rcx = rsp.wrapping_add(0x20); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r9 = (crate::simd_ops::ru32(rsp.wrapping_add(0x28u64))) as u32 as u64; bb = 3; continue; }
            3 => { rax = ((r9 as u32)) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rsp.wrapping_add(0x20u64)))) as u64; xmm0 = 0u128; ymm0_hi = 0; let y_addr: u64 = rcx; crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); crate::simd_ops::wu128(rcx.wrapping_add(0x20u64), (xmm0) as u128); rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x28u64))) as u32 as u64; rax = (((rax as u32)).wrapping_add(1)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((rax as u32)) as u32); rbx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rbx = ((rbx).wrapping_shl(4u32)) as u64; rbx = ((rbx).wrapping_add(crate::simd_ops::ru64(rsp.wrapping_add(0x20u64)))) as u64; crate::simd_ops::wu64(rbx.wrapping_sub(0x30u64), rdi); crate::simd_ops::wu64(rbx.wrapping_sub(0x28u64), rsi); crate::simd_ops::wu32(rbx.wrapping_sub(0x20u64), (0x2u32) as u32); crate::simd_ops::wu32(rbx.wrapping_sub(0x1cu64), (0x2u32) as u32); crate::simd_ops::wu64(rbx.wrapping_sub(0x18u64), r14); crate::simd_ops::wu32(rbx.wrapping_sub(0x8u64), (0xffffffffu32) as u32); crate::simd_ops::wu64(rbx.wrapping_sub(0x10u64), r14); rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x240u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } rcx = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } rax = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm((crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rbx.wrapping_sub(0x18u64), rax); bb = 4; continue; }
            4 => { rcx = (crate::simd_ops::ru32(rbp.wrapping_add(0x258u64))) as u32 as u64; rax = ((rcx as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_shr(1u32)) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 5; continue; } rcx = rdi.wrapping_add(0x40); rdx = rsp.wrapping_add(0x20); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru32(rbp.wrapping_add(0x258u64))) as u32 as u64; bb = 5; continue; }
            5 => { rcx = (((rcx as u32)).wrapping_shr(1u32)) as u32 as u64; fl_l = ((rcx as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 6; continue; } crate::simd_ops::wu64(rdi.wrapping_add(0x138u64), r14); crate::simd_ops::wu64(rdi.wrapping_add(0x140u64), r14); bb = 6; continue; }
            6 => { r8 = (crate::simd_ops::ru32(rsp.wrapping_add(0x28u64))) as u32 as u64; rcx = ((r8.wrapping_add(r8.wrapping_mul(2))) as u64) as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rax = (0x2aaaaaaaaaaaaaabu64) as u64; let prod = (rax as i64 as i128).wrapping_mul((rcx) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; rdx = ((((rdx) as i64).wrapping_shr(3u32) as u64)) as u64; rax = rdx; rax = ((rax).wrapping_shr(63u32)) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; rcx = ((rdx.wrapping_add(rdx.wrapping_mul(2))) as u64) as u64; rcx = ((rcx).wrapping_add(rcx)) as u64; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; rcx = (crate::simd_ops::ru32(rax.wrapping_add(rcx.wrapping_mul(8)).wrapping_sub(0x8u64))) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x248u64), ((rcx as u32)) as u32); r8 = (((r8 as u32)).wrapping_sub(1)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((r8 as u32)) as u32); rcx = rsp.wrapping_add(0x20); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 7; continue; }
            7 => { r11 = rsp.wrapping_add(0x360); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x10u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x18u64))) as u64; rdi = (crate::simd_ops::ru64(r11.wrapping_add(0x20u64))) as u64; r14 = (crate::simd_ops::ru64(r11.wrapping_add(0x28u64))) as u64; rsp = r11; rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_23049<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 117>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rbp = rsp.wrapping_sub(0x280); rsp = ((rsp).wrapping_sub(0x380u64)) as u64; r11 = r9; rdi = rdx; r10 = rcx; fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(0x54u64))) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) >= (((fl_r as u32)) as i32)) { bb = 6; continue; } rdx = 0; fl_l = 0; fl_r = 0; r8 = 0x310; rcx = rsp.wrapping_add(0x40); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rsp.wrapping_add(0x50); crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rcx); r14 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rsp.wrapping_add(0x4cu64), (0x10u32) as u32); crate::simd_ops::wu32(rbp.wrapping_add(0x258u64), (0x6u32) as u32); crate::simd_ops::wu64(rbp.wrapping_add(0x260u64), r14); crate::simd_ops::wu32(rbp.wrapping_add(0x268u64), (0xffffffffu32) as u32); crate::simd_ops::wu64(rbp.wrapping_add(0x270u64), r14); crate::simd_ops::wu64(rbp.wrapping_add(0x250u64), r14); xmm0 = 0u128; ymm0_hi = 0; let y_addr: u64 = rsp.wrapping_add(0x50u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); crate::simd_ops::wu128(rsp.wrapping_add(0x70u64), (xmm0) as u128); rax = ((r14.wrapping_add(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x48u64), ((rax as u32)) as u32); rbx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rbx = ((rbx).wrapping_shl(4u32)) as u64; rbx = ((rbx).wrapping_add(rcx)) as u64; crate::simd_ops::wu64(rbx.wrapping_sub(0x30u64), r14); rax = P1; crate::simd_ops::wu64(rbx.wrapping_sub(0x28u64), rax); crate::simd_ops::wu64(rbx.wrapping_sub(0x20u64), r14); crate::simd_ops::wu64(rbx.wrapping_sub(0x18u64), r14); crate::simd_ops::wu32(rbx.wrapping_sub(0x8u64), (0xffffffffu32) as u32); crate::simd_ops::wu64(rbx.wrapping_sub(0x10u64), r14); rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x260u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 1; continue; } rcx = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 1; continue; } rax = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm((crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rbx.wrapping_sub(0x18u64), rax); bb = 1; continue; }
            1 => { crate::simd_ops::wu32(rbp.wrapping_add(0x278u64), ((r14 as u32)) as u32); rsi = (crate::simd_ops::ru64(P2)) as u64; r9 = (crate::simd_ops::ru32(rsp.wrapping_add(0x48u64))) as u32 as u64; r8 = (crate::simd_ops::ru32(rsp.wrapping_add(0x4cu64))) as u32 as u64; rcx = ((r8.wrapping_add(r8.wrapping_mul(2))) as u64) as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rax = ((r9.wrapping_add(r9.wrapping_mul(2))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } rax = r9.wrapping_add(0x1); rdx = ((r8 as u32)) as u32 as u64; rdx = ((rdx).wrapping_shr(1u32)) as u64; rdx = ((rdx).wrapping_add(r8)) as u64; fl_l = rdx; fl_r = rax; rdx = (if (((fl_l as u64)) < ((fl_r as u64))) { rax } else { rdx }) as u64; rcx = rsp.wrapping_add(0x40); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r9 = (crate::simd_ops::ru32(rsp.wrapping_add(0x48u64))) as u32 as u64; bb = 2; continue; }
            2 => { rax = ((r9 as u32)) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rsp.wrapping_add(0x40u64)))) as u64; xmm0 = 0u128; ymm0_hi = 0; let y_addr: u64 = rcx; crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); crate::simd_ops::wu128(rcx.wrapping_add(0x20u64), (xmm0) as u128); rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x48u64))) as u32 as u64; rax = (((rax as u32)).wrapping_add(1)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x48u64), ((rax as u32)) as u32); rbx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rbx = ((rbx).wrapping_shl(4u32)) as u64; rbx = ((rbx).wrapping_add(crate::simd_ops::ru64(rsp.wrapping_add(0x40u64)))) as u64; crate::simd_ops::wu64(rbx.wrapping_sub(0x30u64), rdi); crate::simd_ops::wu64(rbx.wrapping_sub(0x28u64), rsi); crate::simd_ops::wu32(rbx.wrapping_sub(0x20u64), (0x2u32) as u32); crate::simd_ops::wu32(rbx.wrapping_sub(0x1cu64), (0x2u32) as u32); crate::simd_ops::wu64(rbx.wrapping_sub(0x18u64), r14); crate::simd_ops::wu32(rbx.wrapping_sub(0x8u64), (0xffffffffu32) as u32); crate::simd_ops::wu64(rbx.wrapping_sub(0x10u64), r14); rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x260u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } rcx = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } rax = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm((crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rbx.wrapping_sub(0x18u64), rax); bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru32(rbp.wrapping_add(0x278u64))) as u32 as u64; rax = ((rcx as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_shr(1u32)) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 4; continue; } rcx = rdi.wrapping_add(0x40); rdx = rsp.wrapping_add(0x40); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru32(rbp.wrapping_add(0x278u64))) as u32 as u64; bb = 4; continue; }
            4 => { rcx = (((rcx as u32)).wrapping_shr(1u32)) as u32 as u64; fl_l = ((rcx as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 5; continue; } crate::simd_ops::wu64(rdi.wrapping_add(0x138u64), r14); crate::simd_ops::wu64(rdi.wrapping_add(0x140u64), r14); bb = 5; continue; }
            5 => { r8 = (crate::simd_ops::ru32(rsp.wrapping_add(0x48u64))) as u32 as u64; rcx = ((r8.wrapping_add(r8.wrapping_mul(2))) as u64) as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rax = (0x2aaaaaaaaaaaaaabu64) as u64; let prod = (rax as i64 as i128).wrapping_mul((rcx) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; rdx = ((((rdx) as i64).wrapping_shr(3u32) as u64)) as u64; rax = rdx; rax = ((rax).wrapping_shr(63u32)) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; rcx = ((rdx.wrapping_add(rdx.wrapping_mul(2))) as u64) as u64; rcx = ((rcx).wrapping_add(rcx)) as u64; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rcx = (crate::simd_ops::ru32(rax.wrapping_add(rcx.wrapping_mul(8)).wrapping_sub(0x8u64))) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x268u64), ((rcx as u32)) as u32); r8 = (((r8 as u32)).wrapping_sub(1)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x48u64), ((r8 as u32)) as u32); rcx = rsp.wrapping_add(0x40); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rsp = ((rsp).wrapping_add(0x380u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            6 => { rcx = ((rcx).wrapping_add(0x30u64)) as u64; rdx = r10.wrapping_add(0x20); rax = (crate::simd_ops::ru64(r10.wrapping_add(0x18u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rcx); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rdx); r9 = rdi; r8 = r11; rcx = (crate::simd_ops::ru64(r10.wrapping_add(0x40u64))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rsp = ((rsp).wrapping_add(0x380u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_23061<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64, const P9: u64, const P10: u64, const P11: u64, const P12: u64, const P13: u64, const P14: u64, const P15: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x60u64)) as u64; rdi = rdx; rbx = rcx; crate::simd_ops::wu8(rsp.wrapping_add(0x38u64), ((r8 as u8)) as u8); rax = P0; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), (0xau32) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), (0xcu32) as u32); r8 = (crate::simd_ops::ru64(P1)) as u64; rcx = rdx; rax = crate::simd_ops::call_stk_caught(P2, rax, rcx, rdx, r8, r9, rsp); rcx = rdi; rax = crate::simd_ops::call_stk_caught(P3, rax, rcx, rdx, r8, r9, rsp); rdx = rbx.wrapping_add(0x10); rcx = rdi; rax = crate::simd_ops::call_stk_caught(P4, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu8(rsp.wrapping_add(0x70u64), (0x0u8) as u8); rdx = rbx.wrapping_add(0x34); rax = rsp.wrapping_add(0x70); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); rsi = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); r9 = P5; r8 = ((rsi.wrapping_add(0xau64)) as u32) as u32 as u64; rcx = rdi; rax = crate::simd_ops::call_stk_caught(P6, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu8(rsp.wrapping_add(0x70u64), ((rsi as u8)) as u8); rdx = rbx.wrapping_add(0x35); rax = rsp.wrapping_add(0x70); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); r9 = P7; r8 = ((rsi.wrapping_add(0xau64)) as u32) as u32 as u64; rcx = rdi; rax = crate::simd_ops::call_stk_caught(P6, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu8(rsp.wrapping_add(0x70u64), ((rsi as u8)) as u8); rdx = rbx.wrapping_add(0x36); rax = rsp.wrapping_add(0x70); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); r9 = P8; r8 = ((rsi.wrapping_add(0xau64)) as u32) as u32 as u64; rcx = rdi; rax = crate::simd_ops::call_stk_caught(P6, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu32(rsp.wrapping_add(0x70u64), ((rsi as u32)) as u32); rdx = rbx.wrapping_add(0x30); rax = rsp.wrapping_add(0x70); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); r9 = P9; r8 = ((rsi.wrapping_add(0xbu64)) as u32) as u32 as u64; rcx = rdi; rax = crate::simd_ops::call_stk_caught(P10, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x28u64))) as u32 as u64; rcx = (((rax) as i64).wrapping_mul((0x108u64) as i64) as u64) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rdi.wrapping_add(0x20u64)))) as u64; fl_l = (crate::simd_ops::ru64(rdi.wrapping_add(0x20u64))) as u64; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } rax = 0; fl_l = 0; fl_r = 0; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; rsp = ((rsp).wrapping_add(0x60u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            1 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x40u64))) as u64; rax = crate::simd_ops::call_stk_caught(P11, rax, rcx, rdx, r8, r9, rsp); rcx = rax; rax = crate::simd_ops::call_stk_caught(P12, rax, rcx, rdx, r8, r9, rsp); r8 = rax; rdx = rdi.wrapping_add(0x20); rcx = rsp.wrapping_add(0x40); rax = crate::simd_ops::call_stk_caught(P13, rax, rcx, rdx, r8, r9, rsp); rcx = rax; rax = crate::simd_ops::call_stk_caught(P14, rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, P15, rcx, rsp); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_23074<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64, const P9: u64, const P10: u64, const P11: u64, const P12: u64, const P13: u64, const P14: u64, const P15: u64, const P16: u64, const P17: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi, mut r12, mut r14, mut r15) = (0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 111>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x360u64)) as u64; r12 = rdx; rsi = rcx; fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(0x54u64))) as u32 as u64; fl_r = 0xa; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 19; continue; } rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; rdi = 0; fl_l = 0; fl_r = 0; r8 = ((rdi as u32)) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rdi); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rdi); rbx = (crate::simd_ops::ru64(rcx.wrapping_add(0x40u64))) as u64; r15 = P0; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } rdx = r15; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 1; continue; } fl_l = (crate::simd_ops::ru16(rax.wrapping_add(0x1eu64))) as u16 as u64; fl_r = ((rdi as u16)) as u16 as u64; if (((fl_l as u16)) == ((fl_r as u16))) { bb = 1; continue; } r8 = rax.wrapping_add(0x10); fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 1; continue; } fl_l = (crate::simd_ops::ru16(r8.wrapping_add(0xeu64))) as u16 as u64; fl_r = (0x4u16) as u16 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { bb = 20; continue; } crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), r8); rax = (crate::simd_ops::ru64(rbx)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 2; continue; }
            1 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; r8 = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; bb = 2; continue; }
            2 => { rdx = (crate::simd_ops::ru64(rsi.wrapping_add(0x48u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x8u64))) as u64; r8 = (crate::simd_ops::ru32(rsi.wrapping_add(0x50u64))) as u32 as u64; r8 = ((r8).wrapping_shl(4u32)) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; rax = (0xffffffffffffu64) as u64; rcx = (((rcx) & (rax))) as u64; r8 = ((r8).wrapping_add(rcx)) as u64; fl_l = (crate::simd_ops::ru16(r8.wrapping_add(0xeu64))) as u16 as u64; fl_r = (0x4u16) as u16 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { bb = 5; continue; } crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), r8); rax = (crate::simd_ops::ru64(rdx)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); bb = 3; continue; }
            3 => { r14 = (crate::simd_ops::ru32(r8)) as u32 as u64; fl_l = r14; fl_r = (0x2u64) as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 7; continue; } crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (0x2u64) as u64); r9 = r15; r8 = P2; rdx = ((crate::simd_ops::ru8(P3)) as u32) as u32 as u64; rcx = rsp.wrapping_add(0x150); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rsi.wrapping_add(0x20); rdx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rsp.wrapping_add(0x150); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 19; continue; }
            4 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 3; continue; } bb = 5; continue; }
            5 => { crate::simd_ops::wu64(rsp.wrapping_add(0x150u64), (0x10000010u64) as u64); crate::simd_ops::wu8(rsp.wrapping_add(0x158u64), (0x0u8) as u8); crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), r15); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), (0x11u64) as u64); r9 = 0x11; r8 = 0; fl_l = 0; fl_r = 0; rdx = rsp.wrapping_add(0x40); rcx = rsp.wrapping_add(0x150); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = (crate::simd_ops::ru32(rsi.wrapping_add(0x38u64))) as u32 as u64; r8 = (crate::simd_ops::ru32(rsi.wrapping_add(0x3cu64))) as u32 as u64; rcx = (((r8) as i64).wrapping_mul((0x88u64) as i64) as u64) as u64; rax = (((rdx) as i64).wrapping_mul((0x88u64) as i64) as u64) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 6; continue; } rax = rdx.wrapping_add(0x1); rdx = ((r8 as u32)) as u32 as u64; rdx = ((rdx).wrapping_shr(1u32)) as u64; rdx = ((rdx).wrapping_add(r8)) as u64; fl_l = rdx; fl_r = rax; rdx = (if (((fl_l as u64)) < ((fl_r as u64))) { rax } else { rdx }) as u64; rcx = rsi.wrapping_add(0x30); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P8, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru32(rsi.wrapping_add(0x38u64))) as u32 as u64; rcx = (((rax) as i64).wrapping_mul((0x88u64) as i64) as u64) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rsi.wrapping_add(0x30u64)))) as u64; crate::simd_ops::lk_and_u32(rcx, (0xff000010u32) as u32); crate::simd_ops::lk_or_u32(rcx, (0x10u32) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x3u64), (0x10u8) as u8); crate::simd_ops::wu32(rcx.wrapping_add(0x4u64), ((rdi as u32)) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x8u64), (0x0u8) as u8); rdx = rsp.wrapping_add(0x150); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P9, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::lk_add_u32(rsi.wrapping_add(0x38u64), (1) as u32); rcx = rsp.wrapping_add(0x150); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 19; continue; }
            7 => { rax = (crate::simd_ops::ru32(rsi.wrapping_add(0x54u64))) as u32 as u64; xmm0 = 0u128; ymm0_hi = 0; crate::simd_ops::wu128(rsp.wrapping_add(0x50u64), (xmm0) as u128); crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rdi); crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rdi); crate::simd_ops::wu8(rsp.wrapping_add(0x60u64), ((rdi as u8)) as u8); let y_addr: u64 = rsp.wrapping_add(0x70u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), rdi); crate::simd_ops::wu64(rsp.wrapping_add(0x78u64), rdi); crate::simd_ops::wu64(rsp.wrapping_add(0x80u64), rdi); crate::simd_ops::wu64(rsp.wrapping_add(0x88u64), rdi); crate::simd_ops::wu64(rsp.wrapping_add(0x90u64), rdi); rcx = rsp.wrapping_add(0x30); crate::simd_ops::wu64(rsp.wrapping_add(0x98u64), rcx); crate::simd_ops::wu32(rsp.wrapping_add(0xa0u64), ((rdi as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0xa4u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rsp.wrapping_add(0xa8u64))) as u32 as u64; rax = ((((rax as u32)) & (0xff000010u32))) as u32 as u64; fl_l = ((((rax as u32)) | (0x10u32))) as u32 as u64; fl_r = fl_l; rax = ((fl_l as u32)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0xa8u64), ((rax as u32)) as u32); crate::simd_ops::wu8(rsp.wrapping_add(0xabu64), (0x10u8) as u8); crate::simd_ops::wu32(rsp.wrapping_add(0xacu64), ((rdi as u32)) as u32); crate::simd_ops::wu8(rsp.wrapping_add(0xb0u64), ((rdi as u8)) as u8); rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x18u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), rax); rbx = rdi; bb = 8; continue; }
            8 => { fl_l = rbx; fl_r = r14; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 9; continue; } crate::simd_ops::wu32(rsp.wrapping_add(0xa0u64), ((rbx as u32)) as u32); rdx = rbx; rdx = ((rdx).wrapping_shl(4u32)) as u64; rdx = ((rdx).wrapping_add(r12)) as u64; r9 = P10; r8 = 0xa; rcx = rsp.wrapping_add(0x50); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P11, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rbx).wrapping_add(1)) as u64; rbx = ((fl_l as u64)) as u64; bb = 8; continue; }
            9 => { bb = 11; continue; }
            10 => { rdi = 0; fl_l = 0; fl_r = 0; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x380u64))) as u64; bb = 11; continue; }
            11 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x78u64))) as u32 as u64; r14 = (((rax) as i64).wrapping_mul((0x108u64) as i64) as u64) as u64; r14 = ((r14).wrapping_add(rbx)) as u64; fl_l = rbx; fl_r = r14; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 18; continue; } bb = 12; continue; }
            12 => { crate::simd_ops::wu64(rsp.wrapping_add(0x258u64), (0x20000020u64) as u64); crate::simd_ops::wu8(rsp.wrapping_add(0x260u64), (0x0u8) as u8); rax = rsp.wrapping_add(0x258); fl_l = rax; fl_r = rbx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 16; continue; } r9 = (crate::simd_ops::ru32(rbx.wrapping_add(0x4u64))) as u32 as u64; rdx = rbx.wrapping_add(0x8); rcx = ((crate::simd_ops::ru8(rbx.wrapping_add(0x3u64))) as u32) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x80; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 13; continue; } rcx = ((rcx.wrapping_mul(8)) as u32) as u32 as u64; bb = 14; continue; }
            13 => { rcx = (((rcx as u32)).wrapping_shl(8u32)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x7c00; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 14; continue; }
            14 => { rax = (crate::simd_ops::ru32(rbx)) as u32 as u64; rax = ((((rax as u32)) & (0xffffffu32))) as u32 as u64; rax = (((rax as u32)).wrapping_shl(3u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 15; continue; } rdx = (crate::simd_ops::ru64(rdx)) as u64; bb = 15; continue; }
            15 => { crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rdx); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), r9); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x40u64))) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rsp.wrapping_add(0x130u64), (xmm0) as u128); r8 = 0; fl_l = 0; fl_r = 0; rdx = rsp.wrapping_add(0x130); rcx = rsp.wrapping_add(0x258); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 16; continue; }
            16 => { rdx = (crate::simd_ops::ru32(rsi.wrapping_add(0x28u64))) as u32 as u64; r8 = (crate::simd_ops::ru32(rsi.wrapping_add(0x2cu64))) as u32 as u64; rcx = (((r8) as i64).wrapping_mul((0x108u64) as i64) as u64) as u64; rax = (((rdx) as i64).wrapping_mul((0x108u64) as i64) as u64) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 17; continue; } rax = rdx.wrapping_add(0x1); rdx = ((r8 as u32)) as u32 as u64; rdx = ((rdx).wrapping_shr(1u32)) as u64; rdx = ((rdx).wrapping_add(r8)) as u64; fl_l = rdx; fl_r = rax; rdx = (if (((fl_l as u64)) < ((fl_r as u64))) { rax } else { rdx }) as u64; rcx = rsi.wrapping_add(0x20); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P12, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 17; continue; }
            17 => { rax = (crate::simd_ops::ru32(rsi.wrapping_add(0x28u64))) as u32 as u64; rcx = (((rax) as i64).wrapping_mul((0x108u64) as i64) as u64) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rsi.wrapping_add(0x20u64)))) as u64; crate::simd_ops::lk_and_u32(rcx, (0xff000020u32) as u32); crate::simd_ops::lk_or_u32(rcx, (0x20u32) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x3u64), (0x20u8) as u8); crate::simd_ops::wu32(rcx.wrapping_add(0x4u64), ((rdi as u32)) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x8u64), (0x0u8) as u8); rdx = rsp.wrapping_add(0x258); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P9, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::lk_add_u32(rsi.wrapping_add(0x28u64), (1) as u32); rcx = rsp.wrapping_add(0x258); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = ((rbx).wrapping_add(0x108u64)) as u64; fl_l = rbx; fl_r = r14; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 12; continue; } bb = 18; continue; }
            18 => { rcx = rsp.wrapping_add(0xa8); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rsp.wrapping_add(0x80); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P13, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rsp.wrapping_add(0x70); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P14, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rsp.wrapping_add(0x50); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P14, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 19; continue; }
            19 => { r11 = rsp.wrapping_add(0x360); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x28u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x30u64))) as u64; rdi = (crate::simd_ops::ru64(r11.wrapping_add(0x38u64))) as u64; rsp = r11; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            20 => { r8 = r15; rdx = (crate::simd_ops::ru64(rbx)) as u64; rcx = rsp.wrapping_add(0x130); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P15, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P16, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::lift_ops::trap(0x80000003, P17, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_23077<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 117>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rbp = rsp.wrapping_sub(0x280); rsp = ((rsp).wrapping_sub(0x380u64)) as u64; rdi = rdx; rdx = rcx; fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(0x54u64))) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) >= (((fl_r as u32)) as i32)) { bb = 7; continue; } rdx = 0; fl_l = 0; fl_r = 0; r8 = 0x310; rcx = rsp.wrapping_add(0x40); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rsp.wrapping_add(0x50); crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rcx); r14 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rsp.wrapping_add(0x4cu64), (0x10u32) as u32); crate::simd_ops::wu32(rbp.wrapping_add(0x258u64), (0x6u32) as u32); crate::simd_ops::wu64(rbp.wrapping_add(0x260u64), r14); crate::simd_ops::wu32(rbp.wrapping_add(0x268u64), (0xffffffffu32) as u32); crate::simd_ops::wu64(rbp.wrapping_add(0x270u64), r14); crate::simd_ops::wu64(rbp.wrapping_add(0x250u64), r14); xmm0 = 0u128; ymm0_hi = 0; let y_addr: u64 = rsp.wrapping_add(0x50u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); crate::simd_ops::wu128(rsp.wrapping_add(0x70u64), (xmm0) as u128); rax = ((r14.wrapping_add(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x48u64), ((rax as u32)) as u32); rbx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rbx = ((rbx).wrapping_shl(4u32)) as u64; rbx = ((rbx).wrapping_add(rcx)) as u64; crate::simd_ops::wu64(rbx.wrapping_sub(0x30u64), r14); rax = P1; crate::simd_ops::wu64(rbx.wrapping_sub(0x28u64), rax); crate::simd_ops::wu64(rbx.wrapping_sub(0x20u64), r14); crate::simd_ops::wu64(rbx.wrapping_sub(0x18u64), r14); crate::simd_ops::wu32(rbx.wrapping_sub(0x8u64), (0xffffffffu32) as u32); crate::simd_ops::wu64(rbx.wrapping_sub(0x10u64), r14); rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x260u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 1; continue; } rcx = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 1; continue; } rax = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm((crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rbx.wrapping_sub(0x18u64), rax); bb = 1; continue; }
            1 => { crate::simd_ops::wu32(rbp.wrapping_add(0x278u64), ((r14 as u32)) as u32); rsi = (crate::simd_ops::ru64(P2)) as u64; r9 = (crate::simd_ops::ru32(rsp.wrapping_add(0x48u64))) as u32 as u64; r8 = (crate::simd_ops::ru32(rsp.wrapping_add(0x4cu64))) as u32 as u64; rcx = ((r8.wrapping_add(r8.wrapping_mul(2))) as u64) as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rax = ((r9.wrapping_add(r9.wrapping_mul(2))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } rax = r9.wrapping_add(0x1); rdx = ((r8 as u32)) as u32 as u64; rdx = ((rdx).wrapping_shr(1u32)) as u64; rdx = ((rdx).wrapping_add(r8)) as u64; fl_l = rdx; fl_r = rax; rdx = (if (((fl_l as u64)) < ((fl_r as u64))) { rax } else { rdx }) as u64; rcx = rsp.wrapping_add(0x40); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r9 = (crate::simd_ops::ru32(rsp.wrapping_add(0x48u64))) as u32 as u64; bb = 2; continue; }
            2 => { rax = ((r9 as u32)) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rsp.wrapping_add(0x40u64)))) as u64; xmm0 = 0u128; ymm0_hi = 0; let y_addr: u64 = rcx; crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); crate::simd_ops::wu128(rcx.wrapping_add(0x20u64), (xmm0) as u128); rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x48u64))) as u32 as u64; rax = (((rax as u32)).wrapping_add(1)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x48u64), ((rax as u32)) as u32); rbx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rbx = ((rbx).wrapping_shl(4u32)) as u64; rbx = ((rbx).wrapping_add(crate::simd_ops::ru64(rsp.wrapping_add(0x40u64)))) as u64; crate::simd_ops::wu64(rbx.wrapping_sub(0x30u64), rdi); crate::simd_ops::wu64(rbx.wrapping_sub(0x28u64), rsi); crate::simd_ops::wu32(rbx.wrapping_sub(0x20u64), (0x1u32) as u32); crate::simd_ops::wu32(rbx.wrapping_sub(0x1cu64), (0x1u32) as u32); crate::simd_ops::wu64(rbx.wrapping_sub(0x18u64), r14); crate::simd_ops::wu32(rbx.wrapping_sub(0x8u64), (0xffffffffu32) as u32); crate::simd_ops::wu64(rbx.wrapping_sub(0x10u64), r14); rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x260u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } rcx = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } rax = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm((crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rbx.wrapping_sub(0x18u64), rax); bb = 3; continue; }
            3 => { r8 = (crate::simd_ops::ru32(rbp.wrapping_add(0x278u64))) as u32 as u64; rax = ((r8 as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_shr(1u32)) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 6; continue; } rdx = (crate::simd_ops::ru64(rdi)) as u64; rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x8u64))) as u32 as u64; rax = ((rdx.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; fl_l = ((r8 as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 6; continue; } fl_l = rdx; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 6; continue; } bb = 4; continue; }
            4 => { rcx = ((r8 as u32)) as u32 as u64; rcx = (((rcx as u32)).wrapping_shr(1u32)) as u32 as u64; fl_l = ((rcx as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 5; continue; } crate::simd_ops::wu16(rdx, ((r14 as u16)) as u16); r8 = (crate::simd_ops::ru32(rbp.wrapping_add(0x278u64))) as u32 as u64; bb = 5; continue; }
            5 => { rdx = ((rdx).wrapping_add(0x2u64)) as u64; fl_l = rdx; fl_r = rax; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 4; continue; } bb = 6; continue; }
            6 => { rdx = (crate::simd_ops::ru32(rsp.wrapping_add(0x48u64))) as u32 as u64; rcx = ((rdx.wrapping_add(rdx.wrapping_mul(2))) as u64) as u64; rcx = ((rcx).wrapping_add(rcx)) as u64; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rcx = (crate::simd_ops::ru32(rax.wrapping_add(rcx.wrapping_mul(8)).wrapping_sub(0x8u64))) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x268u64), ((rcx as u32)) as u32); rdx = (((rdx as u32)).wrapping_sub(1)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x48u64), ((rdx as u32)) as u32); rcx = rsp.wrapping_add(0x40); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rsp = ((rsp).wrapping_add(0x380u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            7 => { rcx = ((rcx).wrapping_add(0x20u64)) as u64; rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x18u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rcx); rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x50u64))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rax as u32)) as u32); r9 = rdi; rdx = (crate::simd_ops::ru64(rdx.wrapping_add(0x48u64))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rsp = ((rsp).wrapping_add(0x380u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_23111<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rdx;
    rdi = rcx;
    rdx = (crate::simd_ops::ru64(rcx)) as u64;
    rsi = r8;
    rcx = (crate::simd_ops::ru64(rbx)) as u64;
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    fl_l = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; fl_r = rax;
    if !((((fl_l as u64)) >= ((fl_r as u64)))) { rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); }
    rdx = (crate::simd_ops::ru64(rbx)) as u64;
    rcx = (crate::simd_ops::ru64(rsi)) as u64;
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    fl_l = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; fl_r = rax;
    if (((fl_l as u64)) >= ((fl_r as u64))) { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rdx = (crate::simd_ops::ru64(rdi)) as u64;
    rcx = (crate::simd_ops::ru64(rbx)) as u64;
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    fl_l = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; fl_r = rax;
    if !((((fl_l as u64)) >= ((fl_r as u64)))) { rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); }
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_23116<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64, const P9: u64, const P10: u64, const P11: u64, const P12: u64, const P13: u64, const P14: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut xmm6: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 15>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x70u64)) as u64; crate::simd_ops::wu128(rsp.wrapping_add(0x60u64), (xmm6) as u128); rdi = rdx; rbx = rcx; crate::simd_ops::wu8(rsp.wrapping_add(0x38u64), ((r8 as u8)) as u8); rax = P0; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), (0x1u32) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), (0x1u32) as u32); r8 = (crate::simd_ops::ru64(P1)) as u64; rcx = rdx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rsi = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu16(rsp.wrapping_add(0x80u64), ((rsi as u16)) as u16); rax = rsp.wrapping_add(0x80); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); r9 = P3; r8 = ((rsi.wrapping_add(0x1u64)) as u32) as u32 as u64; rdx = rbx; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm6 = 0u128; crate::simd_ops::wu32(rsp.wrapping_add(0x80u64), ((xmm6 as u32)) as u32); rdx = rbx.wrapping_add(0x4); rax = rsp.wrapping_add(0x80); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); r9 = P5; r8 = ((rsi.wrapping_add(0x1u64)) as u32) as u32 as u64; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu32(rsp.wrapping_add(0x80u64), ((xmm6 as u32)) as u32); rdx = rbx.wrapping_add(0x8); rax = rsp.wrapping_add(0x80); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); r9 = P7; r8 = ((rsi.wrapping_add(0x1u64)) as u32) as u32 as u64; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = rbx.wrapping_add(0x10); r9 = P8; r8 = ((rsi.wrapping_add(0x1u64)) as u32) as u32 as u64; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P9, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x28u64))) as u32 as u64; rcx = (((rax) as i64).wrapping_mul((0x108u64) as i64) as u64) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rdi.wrapping_add(0x20u64)))) as u64; fl_l = (crate::simd_ops::ru64(rdi.wrapping_add(0x20u64))) as u64; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } rax = 0; fl_l = 0; fl_r = 0; r11 = rsp.wrapping_add(0x70); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x18u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x20u64))) as u64; xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x60u64))) as u128; rsp = r11; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            1 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x40u64))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P10, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P11, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r8 = rax; rdx = rdi.wrapping_add(0x20); rcx = rsp.wrapping_add(0x40); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P12, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P13, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::lift_ops::trap(0x80000003, P14, rcx, rsp); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_23123<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64, const P9: u64, const P10: u64, const P11: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut xmm6: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 15>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x70u64)) as u64; crate::simd_ops::wu128(rsp.wrapping_add(0x60u64), (xmm6) as u128); rdi = rdx; rbx = rcx; crate::simd_ops::wu8(rsp.wrapping_add(0x38u64), ((r8 as u8)) as u8); rax = P0; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), (0x1u32) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), (0x1u32) as u32); r8 = (crate::simd_ops::ru64(P1)) as u64; rcx = rdx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm6 = 0u128; crate::simd_ops::wu32(rsp.wrapping_add(0x80u64), ((xmm6 as u32)) as u32); rax = rsp.wrapping_add(0x80); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); rsi = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); r9 = P3; r8 = ((rsi.wrapping_add(0x1u64)) as u32) as u32 as u64; rdx = rbx; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu32(rsp.wrapping_add(0x80u64), ((xmm6 as u32)) as u32); rdx = rbx.wrapping_add(0x4); rax = rsp.wrapping_add(0x80); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); r9 = P5; r8 = ((rsi.wrapping_add(0x1u64)) as u32) as u32 as u64; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu32(rsp.wrapping_add(0x80u64), ((xmm6 as u32)) as u32); rdx = rbx.wrapping_add(0x8); rax = rsp.wrapping_add(0x80); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); r9 = P6; r8 = ((rsi.wrapping_add(0x1u64)) as u32) as u32 as u64; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x28u64))) as u32 as u64; rcx = (((rax) as i64).wrapping_mul((0x108u64) as i64) as u64) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rdi.wrapping_add(0x20u64)))) as u64; fl_l = (crate::simd_ops::ru64(rdi.wrapping_add(0x20u64))) as u64; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } rax = 0; fl_l = 0; fl_r = 0; r11 = rsp.wrapping_add(0x70); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x18u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x20u64))) as u64; xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x60u64))) as u128; rsp = r11; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            1 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x40u64))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P8, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r8 = rax; rdx = rdi.wrapping_add(0x20); rcx = rsp.wrapping_add(0x40); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P9, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P10, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::lift_ops::trap(0x80000003, P11, rcx, rsp); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_23130<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64, const P9: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; r8 = (crate::simd_ops::ru64(P0)) as u64; rdi = rdx; rdx = rcx; crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), (0x1u32) as u32); rbx = rcx; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), (0x1u32) as u32); rcx = rdi; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rdi.wrapping_add(P2))) as u64; rdx = rbx; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(rax.wrapping_add(P3))) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rdi.wrapping_add(P2))) as u64; rdx = ((rbx.wrapping_add(P4)) as u64) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(rax.wrapping_add(P3))) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rdi.wrapping_add(P2))) as u64; rdx = ((rbx.wrapping_add(P5)) as u64) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(rax.wrapping_add(P3))) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rdi.wrapping_add(P2))) as u64; rdx = ((rbx.wrapping_add(P6)) as u64) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(rax.wrapping_add(P3))) as u64, rax, rcx, rdx, r8, r9, rsp); rdx = (crate::simd_ops::ru32(rdi.wrapping_add(P5))) as u32 as u64; rax = (crate::simd_ops::ru64(rdi)) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rcx = ((rdx.wrapping_add(rdx.wrapping_mul(2))) as u64) as u64; rcx = ((rcx).wrapping_add(rcx)) as u64; rcx = (crate::simd_ops::ru32(rax.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(P7))) as u32 as u64; rax = ((rdx.wrapping_add(P8)) as u32) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(P5), ((rax as u32)) as u32); rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rdi.wrapping_add(P9), ((rcx as u32)) as u32); rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_23132<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64, const P9: u64, const P10: u64, const P11: u64, const P12: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut xmm6: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 15>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x70u64)) as u64; crate::simd_ops::wu128(rsp.wrapping_add(0x60u64), (xmm6) as u128); rdi = rdx; rbx = rcx; crate::simd_ops::wu8(rsp.wrapping_add(0x38u64), ((r8 as u8)) as u8); rax = P0; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), (0x1u32) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), (0x1u32) as u32); r8 = (crate::simd_ops::ru64(P1)) as u64; rcx = rdx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm6 = 0u128; crate::simd_ops::wu32(rsp.wrapping_add(0x80u64), ((xmm6 as u32)) as u32); rax = rsp.wrapping_add(0x80); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); rsi = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); r9 = P3; r8 = ((rsi.wrapping_add(0x1u64)) as u32) as u32 as u64; rdx = rbx; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu32(rsp.wrapping_add(0x80u64), ((xmm6 as u32)) as u32); rdx = rbx.wrapping_add(0x4); rax = rsp.wrapping_add(0x80); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); r9 = P5; r8 = ((rsi.wrapping_add(0x1u64)) as u32) as u32 as u64; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu32(rsp.wrapping_add(0x80u64), ((xmm6 as u32)) as u32); rdx = rbx.wrapping_add(0x8); rax = rsp.wrapping_add(0x80); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); r9 = P6; r8 = ((rsi.wrapping_add(0x1u64)) as u32) as u32 as u64; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu32(rsp.wrapping_add(0x80u64), ((xmm6 as u32)) as u32); rdx = rbx.wrapping_add(0xc); rax = rsp.wrapping_add(0x80); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); r9 = P7; r8 = ((rsi.wrapping_add(0x1u64)) as u32) as u32 as u64; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x28u64))) as u32 as u64; rcx = (((rax) as i64).wrapping_mul((0x108u64) as i64) as u64) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rdi.wrapping_add(0x20u64)))) as u64; fl_l = (crate::simd_ops::ru64(rdi.wrapping_add(0x20u64))) as u64; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } rax = 0; fl_l = 0; fl_r = 0; r11 = rsp.wrapping_add(0x70); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x18u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x20u64))) as u64; xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x60u64))) as u128; rsp = r11; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            1 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x40u64))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P8, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P9, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r8 = rax; rdx = rdi.wrapping_add(0x20); rcx = rsp.wrapping_add(0x40); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P10, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P11, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::lift_ops::trap(0x80000003, P12, rcx, rsp); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_23141<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64, const P9: u64, const P10: u64, const P11: u64, const P12: u64, const P13: u64, const P14: u64, const P15: u64, const P16: u64, const P17: u64, const P18: u64, const P19: u64, const P20: u64, const P21: u64, const P22: u64, const P23: u64, const P24: u64, const P25: u64, const P26: u64, const P27: u64, const P28: u64, const P29: u64, const P30: u64, const P31: u64, const P32: u64, const P33: u64, const P34: u64, const P35: u64, const P36: u64, const P37: u64, const P38: u64, const P39: u64, const P40: u64, const P41: u64, const P42: u64, const P43: u64, const P44: u64, const P45: u64, const P46: u64, const P47: u64, const P48: u64, const P49: u64, const P50: u64, const P51: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut xmm6: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 15>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rbp = rsp; rsp = ((rsp).wrapping_sub(0x70u64)) as u64; crate::simd_ops::wu128(rsp.wrapping_add(0x60u64), (xmm6) as u128); rdi = rdx; rbx = rcx; crate::simd_ops::wu8(rsp.wrapping_add(0x38u64), ((r8 as u8)) as u8); rax = P0; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), (0x6u32) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), (0x7u32) as u32); r8 = (crate::simd_ops::ru64(P1)) as u64; rcx = rdx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r9 = P3; r8 = 0x1; rdx = rbx; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = rbx.wrapping_add(0x10); r9 = P5; r8 = 0x1; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = rbx.wrapping_add(0x20); r9 = P6; r8 = 0x2; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = rbx.wrapping_add(0x40); r9 = P8; r8 = 0x1; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P9, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = rbx.wrapping_add(0x50); r9 = P10; r8 = 0x1; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P11, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = rbx.wrapping_add(0x30); r9 = P12; r8 = 0x1; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P13, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = rbx.wrapping_add(0x60); r9 = P14; r8 = 0x1; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P15, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = rbx.wrapping_add(0x70); r9 = P16; r8 = 0x1; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P17, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = rbx.wrapping_add(0x80); r9 = P18; r8 = 0x1; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P19, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm6 = 0u128; crate::simd_ops::wu32(rbp.wrapping_add(0x10u64), ((xmm6 as u32)) as u32); rdx = rbx.wrapping_add(0x90); rax = rbp.wrapping_add(0x10); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); rsi = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); r9 = P20; r8 = ((rsi.wrapping_add(0x1u64)) as u32) as u32 as u64; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P21, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu8(rbp.wrapping_add(0x10u64), ((rsi as u8)) as u8); rdx = rbx.wrapping_add(0x94); rax = rbp.wrapping_add(0x10); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); r9 = P22; r8 = ((rsi.wrapping_add(0x3u64)) as u32) as u32 as u64; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P23, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu32(rbp.wrapping_add(0x10u64), ((xmm6 as u32)) as u32); rdx = rbx.wrapping_add(0x98); rax = rbp.wrapping_add(0x10); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); r9 = P24; r8 = ((rsi.wrapping_add(0x3u64)) as u32) as u32 as u64; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P21, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu32(rbp.wrapping_add(0x10u64), ((xmm6 as u32)) as u32); rdx = rbx.wrapping_add(0x9c); rax = rbp.wrapping_add(0x10); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); r9 = P25; r8 = ((rsi.wrapping_add(0x3u64)) as u32) as u32 as u64; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P21, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu32(rbp.wrapping_add(0x10u64), ((xmm6 as u32)) as u32); rdx = rbx.wrapping_add(0xa0); rax = rbp.wrapping_add(0x10); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); r9 = P26; r8 = ((rsi.wrapping_add(0x3u64)) as u32) as u32 as u64; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P21, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu32(rbp.wrapping_add(0x10u64), ((xmm6 as u32)) as u32); rdx = rbx.wrapping_add(0xa4); rax = rbp.wrapping_add(0x10); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); r9 = P27; r8 = ((rsi.wrapping_add(0x3u64)) as u32) as u32 as u64; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P21, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu32(rbp.wrapping_add(0x10u64), ((xmm6 as u32)) as u32); rdx = rbx.wrapping_add(0xa8); rax = rbp.wrapping_add(0x10); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); r9 = P28; r8 = ((rsi.wrapping_add(0x4u64)) as u32) as u32 as u64; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P21, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm0 = ((crate::simd_ops::ru32(P29)) as u32) as u128; crate::simd_ops::wu32(rbp.wrapping_add(0x10u64), ((xmm0 as u32)) as u32); rdx = rbx.wrapping_add(0xac); rax = rbp.wrapping_add(0x10); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); rax = P30; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); r9 = P31; r8 = ((rsi.wrapping_add(0x7u64)) as u32) as u32 as u64; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P21, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu32(rbp.wrapping_add(0x10u64), ((xmm6 as u32)) as u32); rdx = rbx.wrapping_add(0xb0); rax = rbp.wrapping_add(0x10); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); r9 = P32; r8 = ((rsi.wrapping_add(0x5u64)) as u32) as u32 as u64; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P21, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu32(rbp.wrapping_add(0x10u64), ((xmm6 as u32)) as u32); rdx = rbx.wrapping_add(0xb4); rax = rbp.wrapping_add(0x10); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); r9 = P33; r8 = ((rsi.wrapping_add(0x6u64)) as u32) as u32 as u64; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P21, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu32(rbp.wrapping_add(0x10u64), ((xmm6 as u32)) as u32); rdx = rbx.wrapping_add(0xb8); rax = rbp.wrapping_add(0x10); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); r9 = P34; r8 = ((rsi.wrapping_add(0x6u64)) as u32) as u32 as u64; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P21, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu32(rbp.wrapping_add(0x10u64), ((xmm6 as u32)) as u32); rdx = rbx.wrapping_add(0xbc); rax = rbp.wrapping_add(0x10); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); r9 = P35; r8 = ((rsi.wrapping_add(0x6u64)) as u32) as u32 as u64; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P21, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = rbx.wrapping_add(0xc0); r9 = P36; r8 = ((rsi.wrapping_add(0x3u64)) as u32) as u32 as u64; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P37, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = rbx.wrapping_add(0xd0); r9 = P38; r8 = ((rsi.wrapping_add(0x3u64)) as u32) as u32 as u64; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P13, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = rbx.wrapping_add(0xe0); r9 = P39; r8 = ((rsi.wrapping_add(0x3u64)) as u32) as u32 as u64; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P13, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = rbx.wrapping_add(0xf0); r9 = P40; r8 = ((rsi.wrapping_add(0x3u64)) as u32) as u32 as u64; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P13, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = rbx.wrapping_add(0x100); r9 = P41; r8 = ((rsi.wrapping_add(0x3u64)) as u32) as u32 as u64; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P11, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = rbx.wrapping_add(0x110); r9 = P42; r8 = ((rsi.wrapping_add(0x3u64)) as u32) as u32 as u64; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P11, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = rbx.wrapping_add(0x120); r9 = P43; r8 = ((rsi.wrapping_add(0x3u64)) as u32) as u32 as u64; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P13, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = rbx.wrapping_add(0x130); r9 = P44; r8 = ((rsi.wrapping_add(0x3u64)) as u32) as u32 as u64; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P13, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = rbx.wrapping_add(0x140); r9 = P45; r8 = ((rsi.wrapping_add(0x3u64)) as u32) as u32 as u64; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P11, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = rbx.wrapping_add(0x150); r9 = P46; r8 = ((rsi.wrapping_add(0x5u64)) as u32) as u32 as u64; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P13, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x28u64))) as u32 as u64; rcx = (((rax) as i64).wrapping_mul((0x108u64) as i64) as u64) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rdi.wrapping_add(0x20u64)))) as u64; fl_l = (crate::simd_ops::ru64(rdi.wrapping_add(0x20u64))) as u64; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } rax = 0; fl_l = 0; fl_r = 0; r11 = rsp.wrapping_add(0x70); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x18u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x20u64))) as u64; rdi = (crate::simd_ops::ru64(r11.wrapping_add(0x28u64))) as u64; xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x60u64))) as u128; rsp = r11; rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            1 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x40u64))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P47, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P48, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r8 = rax; rdx = rdi.wrapping_add(0x20); rcx = rbp.wrapping_sub(0x30); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P49, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P50, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::lift_ops::trap(0x80000003, P51, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_23165<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 117>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rbp = rsp.wrapping_sub(0x280); rsp = ((rsp).wrapping_sub(0x380u64)) as u64; rdi = rdx; rdx = rcx; fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(0x54u64))) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) >= (((fl_r as u32)) as i32)) { bb = 5; continue; } rdx = 0; fl_l = 0; fl_r = 0; r8 = 0x310; rcx = rsp.wrapping_add(0x40); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rsp.wrapping_add(0x50); crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rcx); r14 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rsp.wrapping_add(0x4cu64), (0x10u32) as u32); crate::simd_ops::wu32(rbp.wrapping_add(0x258u64), (0x6u32) as u32); crate::simd_ops::wu64(rbp.wrapping_add(0x260u64), r14); crate::simd_ops::wu32(rbp.wrapping_add(0x268u64), (0xffffffffu32) as u32); crate::simd_ops::wu64(rbp.wrapping_add(0x270u64), r14); crate::simd_ops::wu64(rbp.wrapping_add(0x250u64), r14); xmm0 = 0u128; ymm0_hi = 0; let y_addr: u64 = rsp.wrapping_add(0x50u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); crate::simd_ops::wu128(rsp.wrapping_add(0x70u64), (xmm0) as u128); rax = ((r14.wrapping_add(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x48u64), ((rax as u32)) as u32); rbx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rbx = ((rbx).wrapping_shl(4u32)) as u64; rbx = ((rbx).wrapping_add(rcx)) as u64; crate::simd_ops::wu64(rbx.wrapping_sub(0x30u64), r14); rax = P1; crate::simd_ops::wu64(rbx.wrapping_sub(0x28u64), rax); crate::simd_ops::wu64(rbx.wrapping_sub(0x20u64), r14); crate::simd_ops::wu64(rbx.wrapping_sub(0x18u64), r14); crate::simd_ops::wu32(rbx.wrapping_sub(0x8u64), (0xffffffffu32) as u32); crate::simd_ops::wu64(rbx.wrapping_sub(0x10u64), r14); rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x260u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 1; continue; } rcx = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 1; continue; } rax = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm((crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rbx.wrapping_sub(0x18u64), rax); bb = 1; continue; }
            1 => { crate::simd_ops::wu32(rbp.wrapping_add(0x278u64), ((r14 as u32)) as u32); rsi = (crate::simd_ops::ru64(P2)) as u64; r9 = (crate::simd_ops::ru32(rsp.wrapping_add(0x48u64))) as u32 as u64; r8 = (crate::simd_ops::ru32(rsp.wrapping_add(0x4cu64))) as u32 as u64; rcx = ((r8.wrapping_add(r8.wrapping_mul(2))) as u64) as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rax = ((r9.wrapping_add(r9.wrapping_mul(2))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } rax = r9.wrapping_add(0x1); rdx = ((r8 as u32)) as u32 as u64; rdx = ((rdx).wrapping_shr(1u32)) as u64; rdx = ((rdx).wrapping_add(r8)) as u64; fl_l = rdx; fl_r = rax; rdx = (if (((fl_l as u64)) < ((fl_r as u64))) { rax } else { rdx }) as u64; rcx = rsp.wrapping_add(0x40); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r9 = (crate::simd_ops::ru32(rsp.wrapping_add(0x48u64))) as u32 as u64; bb = 2; continue; }
            2 => { rax = ((r9 as u32)) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rsp.wrapping_add(0x40u64)))) as u64; xmm0 = 0u128; ymm0_hi = 0; let y_addr: u64 = rcx; crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); crate::simd_ops::wu128(rcx.wrapping_add(0x20u64), (xmm0) as u128); rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x48u64))) as u32 as u64; rax = (((rax as u32)).wrapping_add(1)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x48u64), ((rax as u32)) as u32); rbx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rbx = ((rbx).wrapping_shl(4u32)) as u64; rbx = ((rbx).wrapping_add(crate::simd_ops::ru64(rsp.wrapping_add(0x40u64)))) as u64; crate::simd_ops::wu64(rbx.wrapping_sub(0x30u64), rdi); crate::simd_ops::wu64(rbx.wrapping_sub(0x28u64), rsi); crate::simd_ops::wu32(rbx.wrapping_sub(0x20u64), (0x1u32) as u32); crate::simd_ops::wu32(rbx.wrapping_sub(0x1cu64), (0x1u32) as u32); crate::simd_ops::wu64(rbx.wrapping_sub(0x18u64), r14); crate::simd_ops::wu32(rbx.wrapping_sub(0x8u64), (0xffffffffu32) as u32); crate::simd_ops::wu64(rbx.wrapping_sub(0x10u64), r14); rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x260u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } rcx = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } rax = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm((crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rbx.wrapping_sub(0x18u64), rax); bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x278u64))) as u32 as u64; rax = (((rax as u32)).wrapping_shr(1u32)) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 4; continue; } crate::simd_ops::wu64(rdi, r14); crate::simd_ops::wu32(rdi.wrapping_add(0x8u64), ((r14 as u32)) as u32); bb = 4; continue; }
            4 => { r8 = (crate::simd_ops::ru32(rsp.wrapping_add(0x48u64))) as u32 as u64; rcx = ((r8.wrapping_add(r8.wrapping_mul(2))) as u64) as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rax = (0x2aaaaaaaaaaaaaabu64) as u64; let prod = (rax as i64 as i128).wrapping_mul((rcx) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; rdx = ((((rdx) as i64).wrapping_shr(3u32) as u64)) as u64; rax = rdx; rax = ((rax).wrapping_shr(63u32)) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; rcx = ((rdx.wrapping_add(rdx.wrapping_mul(2))) as u64) as u64; rcx = ((rcx).wrapping_add(rcx)) as u64; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rcx = (crate::simd_ops::ru32(rax.wrapping_add(rcx.wrapping_mul(8)).wrapping_sub(0x8u64))) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x268u64), ((rcx as u32)) as u32); r8 = (((r8 as u32)).wrapping_sub(1)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x48u64), ((r8 as u32)) as u32); rcx = rsp.wrapping_add(0x40); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rsp = ((rsp).wrapping_add(0x380u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            5 => { rcx = ((rcx).wrapping_add(0x20u64)) as u64; rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x18u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rcx); rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x50u64))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rax as u32)) as u32); r9 = rdi; rdx = (crate::simd_ops::ru64(rdx.wrapping_add(0x48u64))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rsp = ((rsp).wrapping_add(0x380u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_23166<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 121>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x3a0u64)) as u64; rbx = rdx; rdx = rcx; fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(0x54u64))) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) >= (((fl_r as u32)) as i32)) { bb = 5; continue; } rdx = 0; fl_l = 0; fl_r = 0; r8 = 0x310; rcx = rsp.wrapping_add(0x60); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rsi = rsp.wrapping_add(0x70); crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), rsi); r15 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rsp.wrapping_add(0x6cu64), (0x10u32) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x378u64), (0x6u32) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x380u64), r15); crate::simd_ops::wu32(rsp.wrapping_add(0x388u64), (0xffffffffu32) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x390u64), r15); crate::simd_ops::wu64(rsp.wrapping_add(0x370u64), r15); xmm0 = 0u128; ymm0_hi = 0; let y_addr: u64 = rsp.wrapping_add(0x70u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); crate::simd_ops::wu128(rsp.wrapping_add(0x90u64), (xmm0) as u128); rax = ((r15.wrapping_add(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x68u64), ((rax as u32)) as u32); rdi = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rdi = ((rdi).wrapping_add(rdi)) as u64; crate::simd_ops::wu64(rsi.wrapping_add(rdi.wrapping_mul(8)).wrapping_sub(0x30u64), r15); rax = P1; crate::simd_ops::wu64(rsi.wrapping_add(rdi.wrapping_mul(8)).wrapping_sub(0x28u64), rax); crate::simd_ops::wu64(rsi.wrapping_add(rdi.wrapping_mul(8)).wrapping_sub(0x20u64), r15); crate::simd_ops::wu64(rsi.wrapping_add(rdi.wrapping_mul(8)).wrapping_sub(0x18u64), r15); crate::simd_ops::wu32(rsi.wrapping_add(rdi.wrapping_mul(8)).wrapping_sub(0x8u64), (0xffffffffu32) as u32); crate::simd_ops::wu64(rsi.wrapping_add(rdi.wrapping_mul(8)).wrapping_sub(0x10u64), r15); rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x380u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 1; continue; } rcx = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 1; continue; } rax = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm((crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rsi.wrapping_add(rdi.wrapping_mul(8)).wrapping_sub(0x18u64), rax); bb = 1; continue; }
            1 => { crate::simd_ops::wu32(rsp.wrapping_add(0x398u64), ((r15 as u32)) as u32); r14 = (crate::simd_ops::ru64(P2)) as u64; r9 = (crate::simd_ops::ru32(rsp.wrapping_add(0x68u64))) as u32 as u64; r8 = (crate::simd_ops::ru32(rsp.wrapping_add(0x6cu64))) as u32 as u64; rcx = ((r8.wrapping_add(r8.wrapping_mul(2))) as u64) as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rax = ((r9.wrapping_add(r9.wrapping_mul(2))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } rax = r9.wrapping_add(0x1); rdx = ((r8 as u32)) as u32 as u64; rdx = ((rdx).wrapping_shr(1u32)) as u64; rdx = ((rdx).wrapping_add(r8)) as u64; fl_l = rdx; fl_r = rax; rdx = (if (((fl_l as u64)) < ((fl_r as u64))) { rax } else { rdx }) as u64; rcx = rsp.wrapping_add(0x60); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r9 = (crate::simd_ops::ru32(rsp.wrapping_add(0x68u64))) as u32 as u64; bb = 2; continue; }
            2 => { rax = ((r9 as u32)) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rsp.wrapping_add(0x60u64)))) as u64; xmm0 = 0u128; ymm0_hi = 0; let y_addr: u64 = rcx; crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); crate::simd_ops::wu128(rcx.wrapping_add(0x20u64), (xmm0) as u128); rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x68u64))) as u32 as u64; rax = (((rax as u32)).wrapping_add(1)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x68u64), ((rax as u32)) as u32); rdi = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rdi = ((rdi).wrapping_add(rdi)) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; crate::simd_ops::wu64(rsi.wrapping_add(rdi.wrapping_mul(8)).wrapping_sub(0x30u64), rbx); crate::simd_ops::wu64(rsi.wrapping_add(rdi.wrapping_mul(8)).wrapping_sub(0x28u64), r14); crate::simd_ops::wu32(rsi.wrapping_add(rdi.wrapping_mul(8)).wrapping_sub(0x20u64), (0x1u32) as u32); crate::simd_ops::wu32(rsi.wrapping_add(rdi.wrapping_mul(8)).wrapping_sub(0x1cu64), (0x1u32) as u32); crate::simd_ops::wu64(rsi.wrapping_add(rdi.wrapping_mul(8)).wrapping_sub(0x18u64), r15); crate::simd_ops::wu32(rsi.wrapping_add(rdi.wrapping_mul(8)).wrapping_sub(0x8u64), (0xffffffffu32) as u32); crate::simd_ops::wu64(rsi.wrapping_add(rdi.wrapping_mul(8)).wrapping_sub(0x10u64), r15); rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x380u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } rcx = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } rax = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm((crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rsi.wrapping_add(rdi.wrapping_mul(8)).wrapping_sub(0x18u64), rax); bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x398u64))) as u32 as u64; rax = (((rax as u32)).wrapping_shr(1u32)) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 4; continue; } crate::simd_ops::wu16(rbx, ((r15 as u16)) as u16); crate::simd_ops::wu64(rbx.wrapping_add(0x4u64), r15); xmm1 = 0u128; xmm0 = (crate::simd_ops::ru128(P4)) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm0) as u128); crate::simd_ops::wu128(rsp.wrapping_add(0x50u64), (xmm1) as u128); let y_addr: u64 = rsp.wrapping_add(0x40u64); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rbx.wrapping_add(0x10u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); bb = 4; continue; }
            4 => { rdx = (crate::simd_ops::ru32(rsp.wrapping_add(0x68u64))) as u32 as u64; rcx = ((rdx.wrapping_add(rdx.wrapping_mul(2))) as u64) as u64; rcx = ((rcx).wrapping_add(rcx)) as u64; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rcx = (crate::simd_ops::ru32(rax.wrapping_add(rcx.wrapping_mul(8)).wrapping_sub(0x8u64))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x388u64), ((rcx as u32)) as u32); rdx = (((rdx as u32)).wrapping_sub(1)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x68u64), ((rdx as u32)) as u32); rcx = rsp.wrapping_add(0x60); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rsp = ((rsp).wrapping_add(0x3a0u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            5 => { rcx = ((rcx).wrapping_add(0x20u64)) as u64; rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x18u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rcx); rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x50u64))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rax as u32)) as u32); r9 = rbx; rdx = (crate::simd_ops::ru64(rdx.wrapping_add(0x48u64))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rsp = ((rsp).wrapping_add(0x3a0u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_23196<const P0: u64, const P1: u64, const P2: u64, const P3: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rcx;
    rcx = (crate::simd_ops::ru64(rcx)) as u64;
    fl_l = rcx; fl_r = rcx;
    if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { rax = ((rbx.wrapping_add(P0)) as u64) as u64; crate::simd_ops::wu64(rbx, rax); crate::simd_ops::wu32(rbx.wrapping_add(P2), (0x0u32) as u32); crate::simd_ops::wu32(rbx.wrapping_add(P3), (0x32u32) as u32); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    rax = ((rbx.wrapping_add(P0)) as u64) as u64;
    fl_l = rcx; fl_r = rax;
    if !((((fl_l as u64)) == ((fl_r as u64)))) { rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); }
    rax = ((rbx.wrapping_add(P0)) as u64) as u64;
    crate::simd_ops::wu64(rbx, rax);
    crate::simd_ops::wu32(rbx.wrapping_add(P2), (0x0u32) as u32);
    crate::simd_ops::wu32(rbx.wrapping_add(P3), (0x32u32) as u32);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_23234<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64, const P9: u64, const P10: u64, const P11: u64, const P12: u64, const P13: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x60u64)) as u64; rdi = rdx; rbx = rcx; crate::simd_ops::wu8(rsp.wrapping_add(0x38u64), ((r8 as u8)) as u8); rax = P0; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); rsi = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((rsi as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rsi as u32)) as u32); r8 = (crate::simd_ops::ru64(P1)) as u64; rcx = rdx; rax = crate::simd_ops::call_stk_caught(P2, rax, rcx, rdx, r8, r9, rsp); r9 = P3; r8 = 0; fl_l = 0; fl_r = 0; rdx = rbx; rcx = rdi; rax = crate::simd_ops::call_stk_caught(P4, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu8(rsp.wrapping_add(0x70u64), ((rsi as u8)) as u8); rdx = rbx.wrapping_add(0x30); rax = rsp.wrapping_add(0x70); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); r9 = P5; r8 = 0; fl_l = 0; fl_r = 0; rcx = rdi; rax = crate::simd_ops::call_stk_caught(P6, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu8(rsp.wrapping_add(0x70u64), ((rsi as u8)) as u8); rdx = rbx.wrapping_add(0x31); rax = rsp.wrapping_add(0x70); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); r9 = P7; r8 = 0; fl_l = 0; fl_r = 0; rcx = rdi; rax = crate::simd_ops::call_stk_caught(P6, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu8(rsp.wrapping_add(0x70u64), ((rsi as u8)) as u8); rdx = rbx.wrapping_add(0x32); rax = rsp.wrapping_add(0x70); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); r9 = P8; r8 = 0; fl_l = 0; fl_r = 0; rcx = rdi; rax = crate::simd_ops::call_stk_caught(P6, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x28u64))) as u32 as u64; rcx = (((rax) as i64).wrapping_mul((0x108u64) as i64) as u64) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rdi.wrapping_add(0x20u64)))) as u64; fl_l = (crate::simd_ops::ru64(rdi.wrapping_add(0x20u64))) as u64; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } rax = 0; fl_l = 0; fl_r = 0; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; rsp = ((rsp).wrapping_add(0x60u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            1 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x40u64))) as u64; rax = crate::simd_ops::call_stk_caught(P9, rax, rcx, rdx, r8, r9, rsp); rcx = rax; rax = crate::simd_ops::call_stk_caught(P10, rax, rcx, rdx, r8, r9, rsp); r8 = rax; rdx = rdi.wrapping_add(0x20); rcx = rsp.wrapping_add(0x40); rax = crate::simd_ops::call_stk_caught(P11, rax, rcx, rdx, r8, r9, rsp); rcx = rax; rax = crate::simd_ops::call_stk_caught(P12, rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, P13, rcx, rsp); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_23249<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64, const P9: u64, const P10: u64, const P11: u64, const P12: u64, const P13: u64, const P14: u64, const P15: u64, const P16: u64, const P17: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x60u64)) as u64; rdi = rdx; rbx = rcx; crate::simd_ops::wu8(rsp.wrapping_add(0x38u64), ((r8 as u8)) as u8); rax = P0; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((rax as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rax as u32)) as u32); r8 = (crate::simd_ops::ru64(P1)) as u64; rcx = rdx; rax = crate::simd_ops::call_stk_caught(P2, rax, rcx, rdx, r8, r9, rsp); rcx = rdi; rax = crate::simd_ops::call_stk_caught(P3, rax, rcx, rdx, r8, r9, rsp); rdx = ((rbx.wrapping_add(P4)) as u64) as u64; r9 = P5; r8 = 0; fl_l = 0; fl_r = 0; rcx = rdi; rax = crate::simd_ops::call_stk_caught(P6, rax, rcx, rdx, r8, r9, rsp); rdx = ((rbx.wrapping_add(P7)) as u64) as u64; r9 = P8; r8 = 0; fl_l = 0; fl_r = 0; rcx = rdi; rax = crate::simd_ops::call_stk_caught(P9, rax, rcx, rdx, r8, r9, rsp); rdx = ((rbx.wrapping_add(P10)) as u64) as u64; r9 = P11; r8 = 0; fl_l = 0; fl_r = 0; rcx = rdi; rax = crate::simd_ops::call_stk_caught(P12, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x28u64))) as u32 as u64; rcx = (((rax) as i64).wrapping_mul((0x108u64) as i64) as u64) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rdi.wrapping_add(0x20u64)))) as u64; fl_l = (crate::simd_ops::ru64(rdi.wrapping_add(0x20u64))) as u64; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } rax = 0; fl_l = 0; fl_r = 0; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rsp = ((rsp).wrapping_add(0x60u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            1 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x40u64))) as u64; rax = crate::simd_ops::call_stk_caught(P13, rax, rcx, rdx, r8, r9, rsp); rcx = rax; rax = crate::simd_ops::call_stk_caught(P14, rax, rcx, rdx, r8, r9, rsp); r8 = rax; rdx = rdi.wrapping_add(0x20); rcx = rsp.wrapping_add(0x40); rax = crate::simd_ops::call_stk_caught(P15, rax, rcx, rdx, r8, r9, rsp); rcx = rax; rax = crate::simd_ops::call_stk_caught(P16, rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, P17, rcx, rsp); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_23253<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64, const P9: u64, const P10: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = (crate::simd_ops::ru64(rcx)) as u64; rdi = rcx; rax = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; rsi = (((rax) as i64).wrapping_mul((0x110u64) as i64) as u64) as u64; rsi = ((rsi).wrapping_add(rbx)) as u64; fl_l = rbx; fl_r = rsi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rcx = rbx; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rbx = ((rbx).wrapping_add(0x110u64)) as u64; fl_l = rbx; fl_r = rsi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } bb = 2; continue; }
            2 => { r9 = (crate::simd_ops::ru64(rdi)) as u64; fl_l = r9; fl_r = r9; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } rcx = ((r9.wrapping_add(P2)) as u64) as u64; rax = r9; rcx = (((rcx) & (0xfffffffffe000000u64))) as u64; rax = ((rax).wrapping_sub(rcx)) as u64; rax = ((rax).wrapping_shr(16u32)) as u64; rdx = ((rcx.wrapping_add(P3)) as u64) as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(P4))) as u32 as u64; rdx = ((rdx).wrapping_sub(rax)) as u64; rax = ((crate::exe::teb_self() as u64)) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(rcx.wrapping_add(P5))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 4; continue; } fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(P6))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 3; continue; } rax = (crate::simd_ops::ru64(rdx.wrapping_add(P7))) as u64; crate::simd_ops::wu64(r9, rax); fl_l = (crate::simd_ops::lk_add_u32(rdx.wrapping_add(P8), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; crate::simd_ops::wu64(rdx.wrapping_add(P7), r9); if (((fl_l as u32)) != 0) { bb = 6; continue; } rcx = rdx; rax = crate::simd_ops::call_stk(P9, rax, rcx, rdx, r8, r9, rsp); bb = 6; continue; }
            3 => { r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 5; continue; }
            4 => { r8 = 0; fl_l = 0; fl_r = 0; bb = 5; continue; }
            5 => { rax = crate::simd_ops::call_stk(P10, rax, rcx, rdx, r8, r9, rsp); bb = 6; continue; }
            6 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = 0; fl_l = 0; fl_r = 0; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; crate::simd_ops::wu64(rdi, rax); crate::simd_ops::wu64(rdi.wrapping_add(P0), rax); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_23299<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; r15 = rdx; r14 = rcx; r12 = (crate::simd_ops::ru32(rcx.wrapping_add(0x8u64))) as u32 as u64; rdx = (((rdx) as i64).wrapping_mul((0x78u64) as i64) as u64) as u64; r10 = 0x6348; rax = ((crate::exe::tls_pointer() as u64)) as u64; r8 = (crate::simd_ops::ru64(rax)) as u64; r10 = (crate::simd_ops::ru64(r10.wrapping_add(r8))) as u64; rbp = 0; fl_l = 0; fl_r = 0; fl_l = ((rdx) & ((rdx).wrapping_sub(1))) as u64; rax = ((fl_l as u64)) as u64; if (((fl_l as u64)) != 0) { bb = 2; continue; } rax = rdx.wrapping_sub(0x10); fl_l = rax; fl_r = (0x3f0u64) as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 2; continue; } rax = rdx.wrapping_add(0x7); rax = ((rax).wrapping_shr(3u32)) as u64; r9 = (crate::simd_ops::ru64(r10.wrapping_add(rax.wrapping_mul(8)).wrapping_add(0x8u64))) as u64; r8 = (crate::simd_ops::ru64(r9.wrapping_add(0x10u64))) as u64; fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 1; continue; } r9 = 0; fl_l = 0; fl_r = 0; rcx = r10; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); bb = 5; continue; }
            1 => { fl_l = (crate::simd_ops::lk_add_u32(r9.wrapping_add(0x18u64), (1) as u32)) as u32 as u64; rax = (crate::simd_ops::ru64(r8)) as u64; crate::simd_ops::wu64(r9.wrapping_add(0x10u64), rax); bb = 6; continue; }
            2 => { rax = (0x7fffffffffffffffu64) as u64; fl_l = rdx; fl_r = rax; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 3; continue; } r8 = rbp; bb = 6; continue; }
            3 => { rax = rdx.wrapping_sub(0x10); fl_l = rax; fl_r = (0x3f0u64) as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 4; continue; } rax = rdx.wrapping_add(0x7); rax = ((rax).wrapping_shr(3u32)) as u64; rcx = (crate::simd_ops::ru64(r10.wrapping_add(rax.wrapping_mul(8)).wrapping_add(0x8u64))) as u64; r8 = (crate::simd_ops::ru64(rcx.wrapping_add(0x10u64))) as u64; rax = r8; rax = ((((rax as u32)) & (0xfu32))) as u32 as u64; fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 4; continue; } fl_l = (crate::simd_ops::lk_add_u32(rcx.wrapping_add(0x18u64), (1) as u32)) as u32 as u64; rax = (crate::simd_ops::ru64(r8)) as u64; crate::simd_ops::wu64(rcx.wrapping_add(0x10u64), rax); bb = 6; continue; }
            4 => { r8 = 0x10; rcx = r10; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 5; continue; }
            5 => { r8 = rax; bb = 6; continue; }
            6 => { fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 7; continue; } rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P2)) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = rax; rdx = 0xdeadd00d; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P3)) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 8; continue; }
            7 => { rbp = r8; bb = 8; continue; }
            8 => { rbx = (crate::simd_ops::ru64(r14)) as u64; rax = (crate::simd_ops::ru32(r14.wrapping_add(0x8u64))) as u32 as u64; rsi = (((rax) as i64).wrapping_mul((0x78u64) as i64) as u64) as u64; rsi = ((rsi).wrapping_add(rbx)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbp); rdi = rbp; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rbp); fl_l = rbx; fl_r = rsi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 10; continue; } bb = 9; continue; }
            9 => { rdx = rbx; rcx = rdi; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rdi = ((rdi).wrapping_add(0x78u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rdi); rbx = ((rbx).wrapping_add(0x78u64)) as u64; fl_l = rbx; fl_r = rsi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 9; continue; } bb = 10; continue; }
            10 => { rcx = r14; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(r14, rbp); crate::simd_ops::wu32(r14.wrapping_add(0x8u64), ((r12 as u32)) as u32); crate::simd_ops::wu32(r14.wrapping_add(0xcu64), ((r15 as u32)) as u32); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_23315<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        core::sync::atomic::fence(core::sync::atomic::Ordering::SeqCst);
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; crate::simd_ops::wu8(rcx.wrapping_add(0x9u64), (0x1u8) as u8); rbp = (crate::simd_ops::ru64(rcx)) as u64; rax = (crate::simd_ops::ru32(rdx)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rax as u32)) as u32); crate::simd_ops::wu8(rsp.wrapping_add(0x30u64), (0x0u8) as u8); rdx = rsp.wrapping_add(0x20); rcx = rbp; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rdi = 0x1; let tmp_xchg: u64 = crate::simd_ops::atomic_xchg_u64(rbp.wrapping_add(0x28u64), rdi); rdi = (tmp_xchg) as u64; rax = (((rdi as u8)) as u32) as u32 as u64; rax = (rax & !0xffu64) | (((((rax as u8)).wrapping_shr(1u32)) as u8 as u64)); fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 1; continue; } rcx = rbp.wrapping_add(0x38); rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P1)) as u64, rax, rcx, rdx, r8, r9, rsp); let _ = crate::simd_ops::atomic_add_u32(rbp.wrapping_add(0x40u64), (1) as u32); rcx = rbp.wrapping_add(0x38); rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P2)) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = rbp.wrapping_add(0x30); rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P3)) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 1; continue; }
            1 => { fl_l = (((rdi) & (0xfffffffffffffffcu64))) as u64; fl_r = fl_l; rdi = ((fl_l as u64)) as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } let _ = crate::simd_ops::atomic_add_u32(rbp.wrapping_add(0x20u64), (1) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), rdi); rax = (crate::simd_ops::ru64(rdi)) as u64; rdx = rbp; rcx = rdi; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(rax)) as u64, rax, rcx, rdx, r8, r9, rsp); rax = 0xffffffff; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rbp.wrapping_add(0x20u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 4; continue; } fl_l = rbp; fl_r = rbp; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } rbx = (crate::simd_ops::ru64(rbp.wrapping_add(0x28u64))) as u64; fl_l = (((rbx) & (0xfffffffffffffffcu64))) as u64; fl_r = fl_l; rbx = ((fl_l as u64)) as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = rbx; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = rbx; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru8(rbp.wrapping_add(0x18u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 3; continue; } fl_l = (crate::simd_ops::ru8(rbp.wrapping_add(0x10u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 3; continue; } rcx = rbp; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); bb = 3; continue; }
            3 => { rcx = rbp; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); bb = 4; continue; }
            4 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = rbx; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = rbx; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); bb = 5; continue; }
            5 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_23319<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
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
    if !(((((fl_l as u8)) & ((fl_r as u8))) == 0)) { rdx = 0x138; rcx = rdi; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); }
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rax = rdi;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_23327<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        core::sync::atomic::fence(core::sync::atomic::Ordering::SeqCst);
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x28u64)) as u64; rax = ((crate::exe::tls_pointer() as u64)) as u64; rcx = 0x74; rax = (crate::simd_ops::ru64(rax)) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(rax))) as u32 as u64; fl_l = (crate::simd_ops::ru32(P0)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) > (((fl_r as u32)) as i32)) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru32(P1)) as u32 as u64; rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
            2 => { rcx = P0; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru32(P0)) as u32 as u64; fl_r = 0xffffffff; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 1; continue; } rax = 0x1; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(P3, xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; rcx = P0; crate::simd_ops::wu32(P1, ((rax as u32)) as u32); rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru32(P1)) as u32 as u64; rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_23328<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64, const P9: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 17>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        core::sync::atomic::fence(core::sync::atomic::Ordering::SeqCst);
        match bb {
            0 => { rax = rsp; crate::simd_ops::wu64(rax.wrapping_add(0x8u64), rcx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x50u64)) as u64; rsi = r8; rbx = rdx; rdi = rcx; crate::simd_ops::wu32(rax.wrapping_sub(0x68u64), (0x0u32) as u32); fl_l = (crate::simd_ops::ru64(rdx.wrapping_add(0x238u64))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } crate::simd_ops::wu32(rax.wrapping_add(0x10u64), (0x2u32) as u32); crate::simd_ops::wu8(rax.wrapping_add(0x14u64), (0x0u8) as u8); rdx = rax.wrapping_add(0x10); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 8; continue; }
            1 => { rcx = rdx.wrapping_add(0xa8); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbp = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax); rdx = rsi; rcx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rbp.wrapping_add(0x78u64), rax); crate::simd_ops::wu16(rbp.wrapping_add(0x80u64), (0x0u16) as u16); r12 = (crate::simd_ops::ru32(rbx.wrapping_add(0x220u64))) as u32 as u64; crate::simd_ops::wu8(rbp.wrapping_add(0x80u64), (0x1u8) as u8); rsi = rax; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rax); crate::simd_ops::wu32(rsp.wrapping_add(0x40u64), ((r12 as u32)) as u32); let _ = crate::simd_ops::atomic_add_u32(rax.wrapping_add(0xcu64), (1) as u32); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((rax as u32)) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbp); bb = 2; continue; }
            2 => { r10 = (crate::simd_ops::ru32(rbx.wrapping_add(0x5cu64))) as u32 as u64; bb = 3; continue; }
            3 => { r9 = ((r10 as u32)) as u32 as u64; rcx = (crate::simd_ops::ru32(rbx.wrapping_add(0x18u64))) as u32 as u64; rax = ((r10 as u32)) as u32 as u64; rcx = (((rcx) & (rax))) as u64; rdx = ((rcx.wrapping_add(rcx.wrapping_mul(2))) as u64) as u64; rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(rdx.wrapping_mul(8)))) as u32 as u64; fl_l = rax; fl_r = r9; rax = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 4; continue; } r8 = ((r10.wrapping_add(0x1u64)) as u32) as u32 as u64; rax = ((r10 as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; let cx_src: u32 = (r8 as u32); let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u32(rbx.wrapping_add(0x5cu64), fl_l as u32, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u32)) as u32 as u64; } r10 = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 3; continue; } xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x28u64))) as u128; crate::simd_ops::wu128(rcx.wrapping_add(rdx.wrapping_mul(8)).wrapping_add(0x8u64), (xmm0) as u128); crate::simd_ops::wu32(rcx.wrapping_add(rdx.wrapping_mul(8)), ((r8 as u32)) as u32); let _ = crate::simd_ops::atomic_add_u32(rbx.wrapping_add(0xf8u64), (1) as u32); rcx = rbx.wrapping_add(0x110); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rdi, rsi); crate::simd_ops::wu32(rdi.wrapping_add(0x8u64), ((r12 as u32)) as u32); bb = 8; continue; }
            4 => { fl_l = rax; fl_r = rax; if (((((fl_l as u64)) & ((fl_r as u64))) as i64) >= 0) { bb = 2; continue; } rcx = rbp.wrapping_add(0x78); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rbp.wrapping_add(0x30); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rbp.wrapping_add(0x8); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = rbp; rcx = rbx.wrapping_add(0xa8); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P8, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu32(rsp.wrapping_add(0xa8u64), (0x8u32) as u32); crate::simd_ops::wu8(rsp.wrapping_add(0xacu64), (0x0u8) as u8); rdx = rsp.wrapping_add(0xa8); rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), (0x1u32) as u32); fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } rax = 0xffffffff; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rsi.wrapping_add(0xcu64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 6; continue; } rbx = (crate::simd_ops::ru64(rsi.wrapping_add(0x10u64))) as u64; fl_l = (((rbx) & (0xfffffffffffffffcu64))) as u64; fl_r = fl_l; rbx = ((fl_l as u64)) as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm((crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P9, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 5; continue; }
            5 => { rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P9, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 6; continue; }
            6 => { bb = 8; continue; }
            7 => { crate::simd_ops::wu32(rsp.wrapping_add(0x98u64), (0x8u32) as u32); crate::simd_ops::wu8(rsp.wrapping_add(0x9cu64), (0x0u8) as u8); rdx = rsp.wrapping_add(0x98); rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 8; continue; }
            8 => { rax = rdi; rsp = ((rsp).wrapping_add(0x50u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_23340<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        core::sync::atomic::fence(core::sync::atomic::Ordering::SeqCst);
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rbp = rcx; rax = ((crate::simd_ops::ru8(rdx.wrapping_add(0x10u64))) as u32) as u32 as u64; crate::simd_ops::wu8(rsp.wrapping_add(0x30u64), ((rax as u8)) as u8); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 1; continue; } rax = (crate::simd_ops::ru64(rdx)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x8u64))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdx.wrapping_add(0xcu64))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x2cu64), ((rax as u32)) as u32); rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rdx, rax); crate::simd_ops::wu64(rdx.wrapping_add(0x8u64), rax); bb = 2; continue; }
            1 => { rax = (crate::simd_ops::ru32(rdx)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rax as u32)) as u32); bb = 2; continue; }
            2 => { rdx = rsp.wrapping_add(0x20); rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rdi = 0x1; let tmp_xchg: u64 = crate::simd_ops::atomic_xchg_u64(rbp.wrapping_add(0x28u64), rdi); rdi = (tmp_xchg) as u64; rax = (((rdi as u8)) as u32) as u32 as u64; rax = (rax & !0xffu64) | (((((rax as u8)).wrapping_shr(1u32)) as u8 as u64)); fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 3; continue; } rcx = rbp.wrapping_add(0x38); rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P1)) as u64, rax, rcx, rdx, r8, r9, rsp); let _ = crate::simd_ops::atomic_add_u32(rbp.wrapping_add(0x40u64), (1) as u32); rcx = rbp.wrapping_add(0x38); rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P2)) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = rbp.wrapping_add(0x30); rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P3)) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 3; continue; }
            3 => { fl_l = (((rdi) & (0xfffffffffffffffcu64))) as u64; fl_r = fl_l; rdi = ((fl_l as u64)) as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } let _ = crate::simd_ops::atomic_add_u32(rbp.wrapping_add(0x20u64), (1) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rdi); rax = (crate::simd_ops::ru64(rdi)) as u64; rdx = rbp; rcx = rdi; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(rax)) as u64, rax, rcx, rdx, r8, r9, rsp); rax = 0xffffffff; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rbp.wrapping_add(0x20u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 6; continue; } fl_l = rbp; fl_r = rbp; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } rbx = (crate::simd_ops::ru64(rbp.wrapping_add(0x28u64))) as u64; fl_l = (((rbx) & (0xfffffffffffffffcu64))) as u64; fl_r = fl_l; rbx = ((fl_l as u64)) as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = rbx; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = rbx; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); bb = 4; continue; }
            4 => { fl_l = (crate::simd_ops::ru8(rbp.wrapping_add(0x18u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 5; continue; } fl_l = (crate::simd_ops::ru8(rbp.wrapping_add(0x10u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 5; continue; } rcx = rbp; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); bb = 5; continue; }
            5 => { rcx = rbp; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); bb = 6; continue; }
            6 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = rbx; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = rbx; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); bb = 7; continue; }
            7 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_23342<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rax = P0;
    crate::simd_ops::wu64(rdx, rax);
    rax = rdx;
    xmm0 = (crate::simd_ops::ru128(rcx.wrapping_add(P1))) as u128;
    crate::simd_ops::wu128(rdx.wrapping_add(P1), (xmm0) as u128);
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_23343<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rax = (crate::simd_ops::ru64(r8)) as u64;
    r9 = rdx;
    rbx = rcx;
    rdi = (crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64;
    rcx = (crate::simd_ops::ru64(r9)) as u64;
    rdx = rdi;
    rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(rbx.wrapping_add(P1))) as u64, rax, rcx, rdx, r8, r9, rsp);
    rbx = (crate::simd_ops::ru64(rbx.wrapping_add(P0))) as u64;
    fl_l = rdi; fl_r = rdi;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rcx = ((rdi.wrapping_add(P2)) as u64) as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rcx = ((rdi.wrapping_add(P4)) as u64) as u64; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rcx = ((rdi.wrapping_add(P0)) as u64) as u64; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rdx = rdi; rcx = ((rbx.wrapping_add(P6)) as u64) as u64; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); }
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_23344<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rax = (crate::simd_ops::ru64(r8)) as u64;
    r9 = rdx;
    rbx = rcx;
    rdi = (crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64;
    rcx = (crate::simd_ops::ru64(r9)) as u64;
    rdx = rdi;
    rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(rbx.wrapping_add(P1))) as u64, rax, rcx, rdx, r8, r9, rsp);
    rbx = (crate::simd_ops::ru64(rbx.wrapping_add(P0))) as u64;
    fl_l = rdi; fl_r = rdi;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rcx = ((rdi.wrapping_add(P2)) as u64) as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rcx = ((rdi.wrapping_add(P4)) as u64) as u64; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rcx = ((rdi.wrapping_add(P6)) as u64) as u64; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rcx = ((rdi.wrapping_add(P0)) as u64) as u64; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rdx = rdi; rcx = ((rbx.wrapping_add(P7)) as u64) as u64; rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); }
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_23348<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rax = (crate::simd_ops::ru64(r8)) as u64;
    r9 = rdx;
    rbx = rcx;
    rdi = (crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64;
    rcx = (crate::simd_ops::ru64(r9)) as u64;
    rdx = rdi;
    rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(rbx.wrapping_add(P1))) as u64, rax, rcx, rdx, r8, r9, rsp);
    rbx = (crate::simd_ops::ru64(rbx.wrapping_add(P0))) as u64;
    fl_l = rdi; fl_r = rdi;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rcx = ((rdi.wrapping_add(P0)) as u64) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rdx = rdi; rcx = ((rbx.wrapping_add(P3)) as u64) as u64; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); }
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_23352<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    fl_l = rcx; fl_r = rcx;
    if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64, rax, rcx, rdx, r8, r9, rsp);
    fl_l = rax; fl_r = rax;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { r8 = (crate::simd_ops::ru64(rax)) as u64; rdx = 0x1; rcx = rax; rsp = ((rsp).wrapping_add(0x28u64)) as u64; return crate::simd_ops::tail_stk((crate::simd_ops::ru64(r8)) as u64, rax, rcx, rdx, r8, r9, rsp); }
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_23356<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rax = P0;
    rbx = rcx;
    crate::simd_ops::wu64(rcx, rax);
    fl_l = ((rdx as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64;
    if !(((((fl_l as u8)) & ((fl_r as u8))) == 0)) { rdx = 0x30; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); }
    rax = rbx;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_23357<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rax = P0;
    rbx = rcx;
    crate::simd_ops::wu64(rcx, rax);
    fl_l = ((rdx as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64;
    if !(((((fl_l as u8)) & ((fl_r as u8))) == 0)) { rdx = 0x28; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); }
    rax = rbx;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_23358<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rax = P0;
    rbx = rcx;
    crate::simd_ops::wu64(rcx, rax);
    fl_l = ((rdx as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64;
    if !(((((fl_l as u8)) & ((fl_r as u8))) == 0)) { rdx = 0x38; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); }
    rax = rbx;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_23439<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        core::sync::atomic::fence(core::sync::atomic::Ordering::SeqCst);
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rsi = rcx; rbx = (crate::simd_ops::ru64(rcx)) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } rax = 0xffffffff; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rbx.wrapping_add(P0), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 2; continue; } rdi = (crate::simd_ops::ru64(rbx.wrapping_add(P1))) as u64; fl_l = (((rdi) & (0xfffffffffffffffcu64))) as u64; fl_r = fl_l; rdi = ((fl_l as u64)) as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 1; continue; } rax = (crate::simd_ops::ru64(rdi)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = rdi; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = rdi; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 1; continue; }
            1 => { rcx = rbx; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rcx = rbx; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 2; continue; }
            2 => { crate::simd_ops::wu64(rsi, (0x0u64) as u64); bb = 3; continue; }
            3 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_23456<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rax = (crate::simd_ops::ru64(r8)) as u64;
    r9 = rdx;
    rbx = rcx;
    rdi = (crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64;
    rcx = (crate::simd_ops::ru64(r9)) as u64;
    rdx = rdi;
    rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(rbx.wrapping_add(P1))) as u64, rax, rcx, rdx, r8, r9, rsp);
    rbx = (crate::simd_ops::ru64(rbx.wrapping_add(P0))) as u64;
    fl_l = rdi; fl_r = rdi;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rcx = rdi; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rdx = rdi; rcx = ((rbx.wrapping_add(P3)) as u64) as u64; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); }
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_23461<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(P0))) as u8 as u64; fl_r = (0x0u8) as u8 as u64;
    if !((((fl_l as u8)) == ((fl_r as u8)))) { rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); }
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_23476<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rax = (crate::simd_ops::ru64(r8)) as u64;
    r9 = rdx;
    rbx = rcx;
    rdi = (crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64;
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64;
    rdx = rdi;
    rcx = (crate::simd_ops::ru64(r9)) as u64;
    rax = crate::simd_ops::call_stk(rax, rax, rcx, rdx, r8, r9, rsp);
    rbx = (crate::simd_ops::ru64(rbx.wrapping_add(P0))) as u64;
    fl_l = rdi; fl_r = rdi;
    if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    rcx = ((rdi.wrapping_add(P2)) as u64) as u64;
    rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp);
    rax = (((crate::simd_ops::ru8(rdi.wrapping_add(P4))) as i8 as i64) as u64) as u64;
    fl_l = ((rax).wrapping_add(0x1u64)) as u64; fl_r = fl_l;
    rax = ((fl_l as u64)) as u64;
    if (((fl_l as u64)) == 0) { rdx = rdi; rcx = ((rbx.wrapping_add(P6)) as u64) as u64; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    fl_l = rax; fl_r = (0x1u64) as u64;
    if !((((fl_l as u64)) == ((fl_r as u64)))) { rcx = rdi; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); }
    rdx = rdi;
    rcx = ((rbx.wrapping_add(P6)) as u64) as u64;
    rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp);
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_23484<const P0: u64, const P1: u64, const P2: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x30u64)) as u64;
    rdi = rdx;
    rbx = rcx;
    crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), (0x0u32) as u32);
    r9 = r8;
    r8 = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64;
    rcx = (crate::simd_ops::ru64(rcx)) as u64;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), (0x1u32) as u32);
    rcx = rbx;
    rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp);
    rax = rdi;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64;
    rsp = ((rsp).wrapping_add(0x30u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_23505<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rdx; rdi = rcx; rax = (crate::simd_ops::ru64(rdx)) as u64; crate::simd_ops::wu64(rcx, rax); rcx = ((rcx).wrapping_add(0x8u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rcx); crate::simd_ops::lk_and_u32(rcx, (0xff000008u32) as u32); crate::simd_ops::lk_or_u32(rcx, (0x8u32) as u32); crate::simd_ops::wu8(rcx.wrapping_add(P0), (0x8u8) as u8); crate::simd_ops::wu32(rcx.wrapping_add(P1), (0x0u32) as u32); crate::simd_ops::wu8(rcx.wrapping_add(P2), (0x0u8) as u8); rdx = ((rdx).wrapping_add(0x8u64)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = ((crate::simd_ops::ru8(rbx.wrapping_add(P4))) as u32) as u32 as u64; crate::simd_ops::wu8(rdi.wrapping_add(P4), ((rax as u8)) as u8); rax = (crate::simd_ops::ru32(rbx.wrapping_add(P5))) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(P5), ((rax as u32)) as u32); xmm0 = (crate::simd_ops::ru128(rbx.wrapping_add(P6))) as u128; crate::simd_ops::wu128(rdi.wrapping_add(P6), (xmm0) as u128); rcx = ((rdi.wrapping_add(P7)) as u64) as u64; rdx = ((rbx.wrapping_add(P7)) as u64) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P8, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = rdi; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_23552<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64, const P9: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 27>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        core::sync::atomic::fence(core::sync::atomic::Ordering::SeqCst);
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rbp = rsp.wrapping_sub(0x27); rsp = ((rsp).wrapping_sub(0xa0u64)) as u64; rsi = r8; rbx = rdx; rdi = rcx; r12 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rbp.wrapping_sub(0x59u64), ((r12 as u32)) as u32); fl_l = (crate::simd_ops::ru64(rdx.wrapping_add(0x238u64))) as u64; fl_r = r12; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } crate::simd_ops::wu32(rbp.wrapping_add(0x7fu64), (0x2u32) as u32); crate::simd_ops::wu8(rbp.wrapping_add(0x83u64), ((r12 as u8)) as u8); rdx = rbp.wrapping_add(0x7f); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 8; continue; }
            1 => { rcx = rdx.wrapping_add(0xa8); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r14 = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } crate::simd_ops::wu64(rbp.wrapping_add(0x6fu64), rax); rcx = (crate::simd_ops::ru64(rsi)) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x29u64), rcx); rax = rbp.wrapping_sub(0x21); crate::simd_ops::wu64(rbp.wrapping_sub(0x49u64), rax); rcx = (crate::simd_ops::ru32(rbp.wrapping_sub(0x21u64))) as u32 as u64; rcx = ((((rcx as u32)) & (0xff000008u32))) as u32 as u64; rcx = ((((rcx as u32)) | (0x8u32))) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_sub(0x21u64), ((rcx as u32)) as u32); crate::simd_ops::wu8(rbp.wrapping_sub(0x1eu64), (0x8u8) as u8); crate::simd_ops::wu32(rbp.wrapping_sub(0x1du64), ((r12 as u32)) as u32); crate::simd_ops::wu8(rbp.wrapping_sub(0x19u64), (0x0u8) as u8); rdx = rsi.wrapping_add(0x8); rcx = rbp.wrapping_sub(0x21); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = rbp.wrapping_sub(0x29); crate::simd_ops::wu64(rbp.wrapping_sub(0x49u64), rax); rax = (crate::simd_ops::ru64(rbp.wrapping_sub(0x29u64))) as u64; crate::simd_ops::wu64(r14, rax); r13 = r14.wrapping_add(0x8); crate::simd_ops::wu64(rbp.wrapping_sub(0x39u64), r13); crate::simd_ops::lk_and_u32(r13, (0xff000008u32) as u32); crate::simd_ops::lk_or_u32(r13, (0x8u32) as u32); crate::simd_ops::wu8(r13.wrapping_add(0x3u64), (0x8u8) as u8); crate::simd_ops::wu32(r13.wrapping_add(0x4u64), ((r12 as u32)) as u32); crate::simd_ops::wu8(r13.wrapping_add(0x8u64), (0x0u8) as u8); rdx = rbp.wrapping_sub(0x21); rcx = r13; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(r14.wrapping_add(0x50u64), rax); crate::simd_ops::wu16(r14.wrapping_add(0x58u64), (0x0u16) as u16); rcx = rbp.wrapping_sub(0x21); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x220u64))) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x6fu64), ((rax as u32)) as u32); crate::simd_ops::wu8(r14.wrapping_add(0x58u64), (0x1u8) as u8); rsi = (crate::simd_ops::ru64(r14.wrapping_add(0x50u64))) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x49u64), rsi); crate::simd_ops::wu32(rbp.wrapping_sub(0x41u64), ((rax as u32)) as u32); let _ = crate::simd_ops::atomic_add_u32(rsi.wrapping_add(0xcu64), (1) as u32); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu32(rbp.wrapping_sub(0x39u64), ((rax as u32)) as u32); crate::simd_ops::wu64(rbp.wrapping_sub(0x31u64), r14); bb = 2; continue; }
            2 => { r10 = (crate::simd_ops::ru32(rbx.wrapping_add(0x5cu64))) as u32 as u64; bb = 3; continue; }
            3 => { r9 = ((r10 as u32)) as u32 as u64; rcx = (crate::simd_ops::ru32(rbx.wrapping_add(0x18u64))) as u32 as u64; rax = ((r10 as u32)) as u32 as u64; rcx = (((rcx) & (rax))) as u64; rdx = ((rcx.wrapping_add(rcx.wrapping_mul(2))) as u64) as u64; rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(rdx.wrapping_mul(8)))) as u32 as u64; fl_l = rax; fl_r = r9; rax = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 4; continue; } r8 = ((r10.wrapping_add(0x1u64)) as u32) as u32 as u64; rax = ((r10 as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; let cx_src: u32 = (r8 as u32); let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u32(rbx.wrapping_add(0x5cu64), fl_l as u32, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u32)) as u32 as u64; } r10 = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 3; continue; } xmm0 = (crate::simd_ops::ru128(rbp.wrapping_sub(0x39u64))) as u128; crate::simd_ops::wu128(rcx.wrapping_add(rdx.wrapping_mul(8)).wrapping_add(0x8u64), (xmm0) as u128); crate::simd_ops::wu32(rcx.wrapping_add(rdx.wrapping_mul(8)), ((r8 as u32)) as u32); let _ = crate::simd_ops::atomic_add_u32(rbx.wrapping_add(0xf8u64), (1) as u32); rcx = rbx.wrapping_add(0x110); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rdi, rsi); rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x6fu64))) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x8u64), ((rax as u32)) as u32); bb = 8; continue; }
            4 => { fl_l = rax; fl_r = rax; if (((((fl_l as u64)) & ((fl_r as u64))) as i64) >= 0) { bb = 2; continue; } rcx = r14.wrapping_add(0x50); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = r13; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = r14; rcx = rbx.wrapping_add(0xa8); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P8, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu32(rbp.wrapping_sub(0x55u64), (0x8u32) as u32); crate::simd_ops::wu8(rbp.wrapping_sub(0x51u64), (0x0u8) as u8); rdx = rbp.wrapping_sub(0x55); rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu32(rbp.wrapping_sub(0x59u64), (0x1u32) as u32); fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } rax = 0xffffffff; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rsi.wrapping_add(0xcu64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 6; continue; } rbx = (crate::simd_ops::ru64(rsi.wrapping_add(0x10u64))) as u64; fl_l = (((rbx) & (0xfffffffffffffffcu64))) as u64; fl_r = fl_l; rbx = ((fl_l as u64)) as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm((crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P9, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 5; continue; }
            5 => { rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P9, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 6; continue; }
            6 => { bb = 8; continue; }
            7 => { crate::simd_ops::wu32(rbp.wrapping_add(0x7fu64), (0x8u32) as u32); crate::simd_ops::wu8(rbp.wrapping_add(0x83u64), ((r12 as u8)) as u8); rdx = rbp.wrapping_add(0x7f); rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 8; continue; }
            8 => { rax = rdi; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0xf0u64))) as u64; rsp = ((rsp).wrapping_add(0xa0u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_23553<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64, const P9: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 67>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        core::sync::atomic::fence(core::sync::atomic::Ordering::SeqCst);
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rbp = rsp.wrapping_sub(0xe0); rsp = ((rsp).wrapping_sub(0x1e0u64)) as u64; rdi = r8; rbx = rdx; rsi = rcx; r12 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((r12 as u32)) as u32); fl_l = (crate::simd_ops::ru64(rdx.wrapping_add(0x238u64))) as u64; fl_r = r12; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } crate::simd_ops::wu32(rbp.wrapping_sub(0x68u64), (0x2u32) as u32); crate::simd_ops::wu8(rbp.wrapping_add(0x30u64), ((r12 as u8)) as u8); rdx = rbp.wrapping_sub(0x68); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 10; continue; }
            1 => { rcx = rdx.wrapping_add(0xa8); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r14 = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } crate::simd_ops::wu64(rbp.wrapping_add(0x128u64), rax); rcx = (crate::simd_ops::ru64(rdi)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rcx); rax = rsp.wrapping_add(0x40); crate::simd_ops::wu64(rbp.wrapping_add(0x138u64), rax); rcx = (crate::simd_ops::ru32(rsp.wrapping_add(0x40u64))) as u32 as u64; rcx = ((((rcx as u32)) & (0xff000008u32))) as u32 as u64; rcx = ((((rcx as u32)) | (0x8u32))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x40u64), ((rcx as u32)) as u32); crate::simd_ops::wu8(rsp.wrapping_add(0x43u64), (0x8u8) as u8); crate::simd_ops::wu32(rsp.wrapping_add(0x44u64), ((r12 as u32)) as u32); crate::simd_ops::wu8(rsp.wrapping_add(0x48u64), (0x0u8) as u8); rdx = rdi.wrapping_add(0x8); rcx = rsp.wrapping_add(0x40); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = rsp.wrapping_add(0x38); crate::simd_ops::wu64(rbp.wrapping_add(0x138u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; crate::simd_ops::wu64(r14, rax); r13 = r14.wrapping_add(0x8); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), r13); crate::simd_ops::lk_and_u32(r13, (0xff000008u32) as u32); crate::simd_ops::lk_or_u32(r13, (0x8u32) as u32); crate::simd_ops::wu8(r13.wrapping_add(0x3u64), (0x8u8) as u8); crate::simd_ops::wu32(r13.wrapping_add(0x4u64), ((r12 as u32)) as u32); crate::simd_ops::wu8(r13.wrapping_add(0x8u64), (0x0u8) as u8); rdx = rsp.wrapping_add(0x40); rcx = r13; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(r14.wrapping_add(0x50u64), rax); crate::simd_ops::wu16(r14.wrapping_add(0x58u64), (0x0u16) as u16); rcx = rsp.wrapping_add(0x40); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x220u64))) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x128u64), ((rax as u32)) as u32); crate::simd_ops::wu8(r14.wrapping_add(0x58u64), (0x1u8) as u8); rdi = (crate::simd_ops::ru64(r14.wrapping_add(0x50u64))) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x78u64), rdi); crate::simd_ops::wu32(rbp.wrapping_sub(0x70u64), ((rax as u32)) as u32); let _ = crate::simd_ops::atomic_add_u32(rdi.wrapping_add(0xa8u64), (1) as u32); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((rax as u32)) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), r14); bb = 2; continue; }
            2 => { r10 = (crate::simd_ops::ru32(rbx.wrapping_add(0x5cu64))) as u32 as u64; bb = 3; continue; }
            3 => { r9 = ((r10 as u32)) as u32 as u64; rcx = (crate::simd_ops::ru32(rbx.wrapping_add(0x18u64))) as u32 as u64; rax = ((r10 as u32)) as u32 as u64; rcx = (((rcx) & (rax))) as u64; rdx = ((rcx.wrapping_add(rcx.wrapping_mul(2))) as u64) as u64; rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(rdx.wrapping_mul(8)))) as u32 as u64; fl_l = rax; fl_r = r9; rax = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 4; continue; } r8 = ((r10.wrapping_add(0x1u64)) as u32) as u32 as u64; rax = ((r10 as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; let cx_src: u32 = (r8 as u32); let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u32(rbx.wrapping_add(0x5cu64), fl_l as u32, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u32)) as u32 as u64; } r10 = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 3; continue; } xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x28u64))) as u128; crate::simd_ops::wu128(rcx.wrapping_add(rdx.wrapping_mul(8)).wrapping_add(0x8u64), (xmm0) as u128); crate::simd_ops::wu32(rcx.wrapping_add(rdx.wrapping_mul(8)), ((r8 as u32)) as u32); let _ = crate::simd_ops::atomic_add_u32(rbx.wrapping_add(0xf8u64), (1) as u32); rcx = rbx.wrapping_add(0x110); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rsi, rdi); rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x128u64))) as u32 as u64; crate::simd_ops::wu32(rsi.wrapping_add(0x8u64), ((rax as u32)) as u32); bb = 11; continue; }
            4 => { fl_l = rax; fl_r = rax; if (((((fl_l as u64)) & ((fl_r as u64))) as i64) >= 0) { bb = 2; continue; } rcx = r14.wrapping_add(0x50); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = r13; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = r14; rcx = rbx.wrapping_add(0xa8); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P8, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu32(rbp.wrapping_add(0x38u64), (0x8u32) as u32); crate::simd_ops::wu8(rbp.wrapping_add(0xd0u64), (0x0u8) as u8); rdx = rbp.wrapping_add(0x38); rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), (0x1u32) as u32); fl_l = (crate::simd_ops::ru8(rbp.wrapping_add(0xd0u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 5; continue; } rcx = rbp.wrapping_add(0x88); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rbp.wrapping_add(0x40); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 5; continue; }
            5 => { fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } rax = 0xffffffff; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rdi.wrapping_add(0xa8u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 8; continue; } rbx = (crate::simd_ops::ru64(rdi.wrapping_add(0xb0u64))) as u64; fl_l = (((rbx) & (0xfffffffffffffffcu64))) as u64; fl_r = fl_l; rbx = ((fl_l as u64)) as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm((crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P9, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 6; continue; }
            6 => { fl_l = (crate::simd_ops::ru8(rdi.wrapping_add(0xa0u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 7; continue; } fl_l = (crate::simd_ops::ru8(rdi.wrapping_add(0x98u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 7; continue; } rcx = rdi.wrapping_add(0x50); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rdi.wrapping_add(0x8); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 7; continue; }
            7 => { rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P9, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 8; continue; }
            8 => { bb = 11; continue; }
            9 => { crate::simd_ops::wu32(rbp.wrapping_sub(0x68u64), (0x8u32) as u32); crate::simd_ops::wu8(rbp.wrapping_add(0x30u64), ((r12 as u8)) as u8); rdx = rbp.wrapping_sub(0x68); rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 10; continue; }
            10 => { fl_l = (crate::simd_ops::ru8(rbp.wrapping_add(0x30u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 11; continue; } rcx = rbp.wrapping_sub(0x18); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rbp.wrapping_sub(0x60); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 11; continue; }
            11 => { rax = rsi; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x230u64))) as u64; rsp = ((rsp).wrapping_add(0x1e0u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_23561<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x40u64)) as u64;
    rbx = rdx;
    rsi = rcx;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rdi = rax;
    crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax);
    crate::simd_ops::wu16(rsp.wrapping_add(0x30u64), (0x100u16) as u16);
    rdx = rbx;
    rcx = rax;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    crate::simd_ops::wu64(rsi, rdi);
    crate::simd_ops::wu32(rsi.wrapping_add(P2), (0x0u32) as u32);
    let _ = crate::simd_ops::atomic_add_u32(rdi.wrapping_add(P3), (1) as u32);
    fl_l = rdi; fl_r = rdi;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); }
    rax = rsi;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64;
    rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64;
    rsp = ((rsp).wrapping_add(0x40u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_23566<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rax = (crate::simd_ops::ru64(r8)) as u64;
    r9 = rdx;
    rbx = rcx;
    rdi = (crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64;
    rcx = (crate::simd_ops::ru64(r9)) as u64;
    rdx = rdi;
    rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(rbx.wrapping_add(P1))) as u64, rax, rcx, rdx, r8, r9, rsp);
    rbx = (crate::simd_ops::ru64(rbx.wrapping_add(P0))) as u64;
    fl_l = rdi; fl_r = rdi;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rcx = ((rdi.wrapping_add(P2)) as u64) as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rcx = ((rdi.wrapping_add(P0)) as u64) as u64; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rdx = rdi; rcx = ((rbx.wrapping_add(P5)) as u64) as u64; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); }
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_23573<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; r8 = (crate::simd_ops::ru64(P0)) as u64; rdi = rdx; rdx = rcx; crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), (0x1u32) as u32); rbx = rcx; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), (0x2u32) as u32); rcx = rdi; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rdi.wrapping_add(P2))) as u64; rdx = rbx; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(rax.wrapping_add(P3))) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rdi.wrapping_add(P2))) as u64; rdx = ((rbx.wrapping_add(P4)) as u64) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(rax.wrapping_add(P3))) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rdi.wrapping_add(P2))) as u64; rdx = ((rbx.wrapping_add(P5)) as u64) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(rax.wrapping_add(P3))) as u64, rax, rcx, rdx, r8, r9, rsp); rdx = (crate::simd_ops::ru32(rdi.wrapping_add(P5))) as u32 as u64; rax = (crate::simd_ops::ru64(rdi)) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rcx = ((rdx.wrapping_add(rdx.wrapping_mul(2))) as u64) as u64; rcx = ((rcx).wrapping_add(rcx)) as u64; rcx = (crate::simd_ops::ru32(rax.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(P6))) as u32 as u64; rax = ((rdx.wrapping_add(P7)) as u32) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(P5), ((rax as u32)) as u32); rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rdi.wrapping_add(P8), ((rcx as u32)) as u32); rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_23575<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64, const P9: u64, const P10: u64, const P11: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x60u64)) as u64; rdi = rdx; rbx = rcx; crate::simd_ops::wu8(rsp.wrapping_add(0x38u64), ((r8 as u8)) as u8); rax = P0; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), (0x1u32) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), (0x2u32) as u32); r8 = (crate::simd_ops::ru64(P1)) as u64; rcx = rdx; rax = crate::simd_ops::call_stk_caught(P2, rax, rcx, rdx, r8, r9, rsp); rsi = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rsp.wrapping_add(0x70u64), ((rsi as u32)) as u32); rax = rsp.wrapping_add(0x70); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); r9 = P3; r8 = ((rsi.wrapping_add(0x1u64)) as u32) as u32 as u64; rdx = rbx; rcx = rdi; rax = crate::simd_ops::call_stk_caught(P4, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu32(rsp.wrapping_add(0x70u64), ((rsi as u32)) as u32); rdx = rbx.wrapping_add(0x4); rax = rsp.wrapping_add(0x70); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); r9 = P5; r8 = ((rsi.wrapping_add(0x1u64)) as u32) as u32 as u64; rcx = rdi; rax = crate::simd_ops::call_stk_caught(P4, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu32(rsp.wrapping_add(0x70u64), ((rsi as u32)) as u32); rdx = rbx.wrapping_add(0x8); rax = rsp.wrapping_add(0x70); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); r9 = P6; r8 = ((rsi.wrapping_add(0x2u64)) as u32) as u32 as u64; rcx = rdi; rax = crate::simd_ops::call_stk_caught(P4, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x28u64))) as u32 as u64; rcx = (((rax) as i64).wrapping_mul((0x108u64) as i64) as u64) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rdi.wrapping_add(0x20u64)))) as u64; fl_l = (crate::simd_ops::ru64(rdi.wrapping_add(0x20u64))) as u64; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } rax = 0; fl_l = 0; fl_r = 0; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; rsp = ((rsp).wrapping_add(0x60u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            1 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x40u64))) as u64; rax = crate::simd_ops::call_stk_caught(P7, rax, rcx, rdx, r8, r9, rsp); rcx = rax; rax = crate::simd_ops::call_stk_caught(P8, rax, rcx, rdx, r8, r9, rsp); r8 = rax; rdx = rdi.wrapping_add(0x20); rcx = rsp.wrapping_add(0x40); rax = crate::simd_ops::call_stk_caught(P9, rax, rcx, rdx, r8, r9, rsp); rcx = rax; rax = crate::simd_ops::call_stk_caught(P10, rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, P11, rcx, rsp); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_23610<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 113>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { fl_l = ((r8 as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 8; continue; } crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rbp = rsp.wrapping_sub(0x260); rsp = ((rsp).wrapping_sub(0x360u64)) as u64; rdi = rdx; rsi = rcx; r14 = ((((r8 as u32)) as i32 as i64) as u64) as u64; r12 = 0; fl_l = 0; fl_r = 0; r15 = P0; r13 = (0x2aaaaaaaaaaaaaabu64) as u64; bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru32(rsi.wrapping_add(0x328u64))) as u32 as u64; fl_r = 0x1; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 2; continue; } rdx = rsi; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 7; continue; }
            2 => { fl_l = (crate::simd_ops::ru8(rsi.wrapping_add(0x369u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 7; continue; } rdx = 0; fl_l = 0; fl_r = 0; r8 = 0x310; rcx = rsp.wrapping_add(0x20); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rsp.wrapping_add(0x30); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rcx); crate::simd_ops::wu32(rsp.wrapping_add(0x2cu64), (0x10u32) as u32); crate::simd_ops::wu32(rbp.wrapping_add(0x238u64), (0x6u32) as u32); crate::simd_ops::wu64(rbp.wrapping_add(0x240u64), r12); crate::simd_ops::wu32(rbp.wrapping_add(0x248u64), (0xffffffffu32) as u32); crate::simd_ops::wu64(rbp.wrapping_add(0x250u64), r12); crate::simd_ops::wu64(rbp.wrapping_add(0x230u64), r12); xmm0 = 0u128; ymm0_hi = 0; let y_addr: u64 = rsp.wrapping_add(0x30u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); crate::simd_ops::wu128(rsp.wrapping_add(0x50u64), (xmm0) as u128); rax = ((r12.wrapping_add(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((rax as u32)) as u32); rbx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rbx = ((rbx).wrapping_shl(4u32)) as u64; rbx = ((rbx).wrapping_add(rcx)) as u64; crate::simd_ops::wu64(rbx.wrapping_sub(0x30u64), r12); crate::simd_ops::wu64(rbx.wrapping_sub(0x28u64), r15); crate::simd_ops::wu64(rbx.wrapping_sub(0x20u64), r12); crate::simd_ops::wu64(rbx.wrapping_sub(0x18u64), r12); crate::simd_ops::wu32(rbx.wrapping_sub(0x8u64), (0xffffffffu32) as u32); crate::simd_ops::wu64(rbx.wrapping_sub(0x10u64), r12); rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x240u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } rcx = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } rax = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm((crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rbx.wrapping_sub(0x18u64), rax); bb = 3; continue; }
            3 => { crate::simd_ops::wu32(rbp.wrapping_add(0x258u64), ((r12 as u32)) as u32); r15 = (crate::simd_ops::ru64(P3)) as u64; rdx = (crate::simd_ops::ru32(rsp.wrapping_add(0x28u64))) as u32 as u64; r8 = ((rdx.wrapping_add(rdx.wrapping_mul(2))) as u64) as u64; r8 = ((r8).wrapping_shl(4u32)) as u64; r9 = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; r8 = ((r8).wrapping_add(r9)) as u64; rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x2cu64))) as u32 as u64; r10 = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; r10 = ((r10).wrapping_shl(4u32)) as u64; r10 = ((r10).wrapping_add(r9)) as u64; fl_l = r8; fl_r = r10; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 4; continue; } r8 = ((r8).wrapping_sub(r9)) as u64; rax = r13; let prod = (rax as i64 as i128).wrapping_mul((r8) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; rdx = ((((rdx) as i64).wrapping_shr(3u32) as u64)) as u64; r8 = rdx; r8 = ((r8).wrapping_shr(63u32)) as u64; rdx = ((rdx).wrapping_add(1)) as u64; r8 = ((r8).wrapping_add(rdx)) as u64; r10 = ((r10).wrapping_sub(r9)) as u64; rax = r13; let prod = (rax as i64 as i128).wrapping_mul((r10) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; rcx = rdx; rcx = ((((rcx) as i64).wrapping_shr(3u32) as u64)) as u64; rax = rcx; rax = ((rax).wrapping_shr(63u32)) as u64; rcx = ((rcx).wrapping_add(rax)) as u64; rdx = rcx; rdx = ((rdx).wrapping_shr(1u32)) as u64; rdx = ((rdx).wrapping_add(rcx)) as u64; fl_l = rdx; fl_r = r8; rdx = (if (((fl_l as u64)) < ((fl_r as u64))) { r8 } else { rdx }) as u64; rcx = rsp.wrapping_add(0x20); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = (crate::simd_ops::ru32(rsp.wrapping_add(0x28u64))) as u32 as u64; r9 = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; bb = 4; continue; }
            4 => { rax = ((rdx as u32)) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rcx = ((rcx).wrapping_add(rcx)) as u64; xmm0 = 0u128; ymm0_hi = 0; let y_addr: u64 = r9.wrapping_add(rcx.wrapping_mul(8)); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); crate::simd_ops::wu128(r9.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x20u64), (xmm0) as u128); rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x28u64))) as u32 as u64; rax = (((rax as u32)).wrapping_add(1)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((rax as u32)) as u32); rbx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rbx = ((rbx).wrapping_shl(4u32)) as u64; rbx = ((rbx).wrapping_add(crate::simd_ops::ru64(rsp.wrapping_add(0x20u64)))) as u64; crate::simd_ops::wu64(rbx.wrapping_sub(0x30u64), rdi); crate::simd_ops::wu64(rbx.wrapping_sub(0x28u64), r15); crate::simd_ops::wu32(rbx.wrapping_sub(0x20u64), (0x2u32) as u32); crate::simd_ops::wu32(rbx.wrapping_sub(0x1cu64), (0x1u32) as u32); crate::simd_ops::wu64(rbx.wrapping_sub(0x18u64), r12); crate::simd_ops::wu32(rbx.wrapping_sub(0x8u64), (0xffffffffu32) as u32); crate::simd_ops::wu64(rbx.wrapping_sub(0x10u64), r12); rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x240u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } rcx = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } rax = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm((crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rbx.wrapping_sub(0x18u64), rax); bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x258u64))) as u32 as u64; rax = (((rax as u32)).wrapping_shr(1u32)) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 6; continue; } crate::simd_ops::wu64(rdi, r12); crate::simd_ops::wu32(rdi.wrapping_add(0x8u64), ((r12 as u32)) as u32); bb = 6; continue; }
            6 => { r8 = (crate::simd_ops::ru32(rsp.wrapping_add(0x28u64))) as u32 as u64; rcx = ((r8.wrapping_add(r8.wrapping_mul(2))) as u64) as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rax = r13; let prod = (rax as i64 as i128).wrapping_mul((rcx) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; rdx = ((((rdx) as i64).wrapping_shr(3u32) as u64)) as u64; rax = rdx; rax = ((rax).wrapping_shr(63u32)) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; rcx = ((rdx.wrapping_add(rdx.wrapping_mul(2))) as u64) as u64; rcx = ((rcx).wrapping_add(rcx)) as u64; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; rcx = (crate::simd_ops::ru32(rax.wrapping_add(rcx.wrapping_mul(8)).wrapping_sub(0x8u64))) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x248u64), ((rcx as u32)) as u32); r8 = (((r8 as u32)).wrapping_sub(1)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((r8 as u32)) as u32); rcx = rsp.wrapping_add(0x20); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r15 = P0; bb = 7; continue; }
            7 => { rdi = ((rdi).wrapping_add(0xcu64)) as u64; fl_l = r14; fl_r = (0x1u64) as u64; r14 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } r11 = rsp.wrapping_add(0x360); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x38u64))) as u64; rdi = (crate::simd_ops::ru64(r11.wrapping_add(0x40u64))) as u64; rsp = r11; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); bb = 8; continue; }
            8 => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_23617<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64, const P9: u64, const P10: u64, const P11: u64, const P12: u64, const P13: u64, const P14: u64, const P15: u64, const P16: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 113>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x360u64)) as u64; rbx = r9; r13 = r8; r12 = rdx; rsi = rcx; r15 = (crate::simd_ops::ru32(rsp.wrapping_add(0x3b0u64))) as u32 as u64; fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(0x54u64))) as u32 as u64; fl_r = ((r15 as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 22; continue; } rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; rdi = 0; fl_l = 0; fl_r = 0; r10 = ((rdi as u32)) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rdi); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rdi); r14 = (crate::simd_ops::ru64(rcx.wrapping_add(0x40u64))) as u64; fl_l = r14; fl_r = r14; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } rdx = rbx; rcx = r14; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 1; continue; } fl_l = (crate::simd_ops::ru16(rax.wrapping_add(0x1eu64))) as u16 as u64; fl_r = ((rdi as u16)) as u16 as u64; if (((fl_l as u16)) == ((fl_r as u16))) { bb = 1; continue; } r10 = rax.wrapping_add(0x10); fl_l = r10; fl_r = r10; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 1; continue; } fl_l = (crate::simd_ops::ru16(r10.wrapping_add(0xeu64))) as u16 as u64; fl_r = (0x4u16) as u16 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { bb = 23; continue; } crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), r10); rax = (crate::simd_ops::ru64(r14)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 2; continue; }
            1 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; r10 = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; bb = 2; continue; }
            2 => { rdx = (crate::simd_ops::ru64(rsi.wrapping_add(0x48u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x8u64))) as u64; r10 = (crate::simd_ops::ru32(rsi.wrapping_add(0x50u64))) as u32 as u64; r10 = ((r10).wrapping_shl(4u32)) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; rax = (0xffffffffffffu64) as u64; rcx = (((rcx) & (rax))) as u64; r10 = ((r10).wrapping_add(rcx)) as u64; fl_l = (crate::simd_ops::ru16(r10.wrapping_add(0xeu64))) as u16 as u64; fl_r = (0x4u16) as u16 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { bb = 5; continue; } crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), r10); rax = (crate::simd_ops::ru64(rdx)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); bb = 3; continue; }
            3 => { r14 = (crate::simd_ops::ru32(r10)) as u32 as u64; fl_l = r14; fl_r = r13; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 10; continue; } crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r13); r9 = rbx; r8 = P1; rdx = ((crate::simd_ops::ru8(P2)) as u32) as u32 as u64; rcx = rsp.wrapping_add(0x150); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rsi.wrapping_add(0x20); rdx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rsp.wrapping_add(0x150); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 22; continue; }
            4 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 3; continue; } bb = 5; continue; }
            5 => { crate::simd_ops::wu64(rsp.wrapping_add(0x150u64), (0x10000010u64) as u64); crate::simd_ops::wu8(rsp.wrapping_add(0x158u64), (0x0u8) as u8); fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 6; continue; } crate::simd_ops::wu32(rsp.wrapping_add(0x154u64), ((rdi as u32)) as u32); crate::simd_ops::wu8(rsp.wrapping_add(0x158u64), ((rbx as u8)) as u8); bb = 8; continue; }
            6 => { r9 = (0xffffffffffffffffu64) as u64; bb = 7; continue; }
            7 => { r9 = ((r9).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(r9))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 7; continue; } crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), r9); r8 = 0; fl_l = 0; fl_r = 0; rdx = rsp.wrapping_add(0x40); rcx = rsp.wrapping_add(0x150); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 8; continue; }
            8 => { rdx = (crate::simd_ops::ru32(rsi.wrapping_add(0x38u64))) as u32 as u64; r8 = (crate::simd_ops::ru32(rsi.wrapping_add(0x3cu64))) as u32 as u64; rcx = (((r8) as i64).wrapping_mul((0x88u64) as i64) as u64) as u64; rax = (((rdx) as i64).wrapping_mul((0x88u64) as i64) as u64) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 9; continue; } rax = rdx.wrapping_add(0x1); rdx = ((r8 as u32)) as u32 as u64; rdx = ((rdx).wrapping_shr(1u32)) as u64; rdx = ((rdx).wrapping_add(r8)) as u64; fl_l = rdx; fl_r = rax; rdx = (if (((fl_l as u64)) < ((fl_r as u64))) { rax } else { rdx }) as u64; rcx = rsi.wrapping_add(0x30); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 9; continue; }
            9 => { rax = (crate::simd_ops::ru32(rsi.wrapping_add(0x38u64))) as u32 as u64; rcx = (((rax) as i64).wrapping_mul((0x88u64) as i64) as u64) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rsi.wrapping_add(0x30u64)))) as u64; crate::simd_ops::lk_and_u32(rcx, (0xff000010u32) as u32); crate::simd_ops::lk_or_u32(rcx, (0x10u32) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x3u64), (0x10u8) as u8); crate::simd_ops::wu32(rcx.wrapping_add(0x4u64), ((rdi as u32)) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x8u64), (0x0u8) as u8); rdx = rsp.wrapping_add(0x150); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P8, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::lk_add_u32(rsi.wrapping_add(0x38u64), (1) as u32); rcx = rsp.wrapping_add(0x150); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 22; continue; }
            10 => { rax = (crate::simd_ops::ru32(rsi.wrapping_add(0x54u64))) as u32 as u64; xmm0 = 0u128; ymm0_hi = 0; crate::simd_ops::wu128(rsp.wrapping_add(0x50u64), (xmm0) as u128); crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rdi); crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rdi); crate::simd_ops::wu8(rsp.wrapping_add(0x60u64), ((rdi as u8)) as u8); let y_addr: u64 = rsp.wrapping_add(0x70u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), rdi); crate::simd_ops::wu64(rsp.wrapping_add(0x78u64), rdi); crate::simd_ops::wu64(rsp.wrapping_add(0x80u64), rdi); crate::simd_ops::wu64(rsp.wrapping_add(0x88u64), rdi); crate::simd_ops::wu64(rsp.wrapping_add(0x90u64), rdi); rcx = rsp.wrapping_add(0x30); crate::simd_ops::wu64(rsp.wrapping_add(0x98u64), rcx); crate::simd_ops::wu32(rsp.wrapping_add(0xa0u64), ((rdi as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0xa4u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rsp.wrapping_add(0xa8u64))) as u32 as u64; rax = ((((rax as u32)) & (0xff000010u32))) as u32 as u64; fl_l = ((((rax as u32)) | (0x10u32))) as u32 as u64; fl_r = fl_l; rax = ((fl_l as u32)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0xa8u64), ((rax as u32)) as u32); crate::simd_ops::wu8(rsp.wrapping_add(0xabu64), (0x10u8) as u8); crate::simd_ops::wu32(rsp.wrapping_add(0xacu64), ((rdi as u32)) as u32); crate::simd_ops::wu8(rsp.wrapping_add(0xb0u64), ((rdi as u8)) as u8); rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x18u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), rax); rbx = rdi; bb = 11; continue; }
            11 => { fl_l = rbx; fl_r = r14; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 12; continue; } crate::simd_ops::wu32(rsp.wrapping_add(0xa0u64), ((rbx as u32)) as u32); rdx = (((rbx) as i64).wrapping_mul((0x2a8u64) as i64) as u64) as u64; rdx = ((rdx).wrapping_add(r12)) as u64; r9 = P9; r8 = ((r15 as u32)) as u32 as u64; rcx = rsp.wrapping_add(0x50); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P10, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rbx).wrapping_add(1)) as u64; rbx = ((fl_l as u64)) as u64; bb = 11; continue; }
            12 => { bb = 14; continue; }
            13 => { rdi = 0; fl_l = 0; fl_r = 0; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x390u64))) as u64; bb = 14; continue; }
            14 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x78u64))) as u32 as u64; r14 = (((rax) as i64).wrapping_mul((0x108u64) as i64) as u64) as u64; r14 = ((r14).wrapping_add(rbx)) as u64; fl_l = rbx; fl_r = r14; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 21; continue; } bb = 15; continue; }
            15 => { crate::simd_ops::wu64(rsp.wrapping_add(0x258u64), (0x20000020u64) as u64); crate::simd_ops::wu8(rsp.wrapping_add(0x260u64), (0x0u8) as u8); rax = rsp.wrapping_add(0x258); fl_l = rax; fl_r = rbx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 19; continue; } r9 = (crate::simd_ops::ru32(rbx.wrapping_add(0x4u64))) as u32 as u64; rdx = rbx.wrapping_add(0x8); rcx = ((crate::simd_ops::ru8(rbx.wrapping_add(0x3u64))) as u32) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x80; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 16; continue; } rcx = ((rcx.wrapping_mul(8)) as u32) as u32 as u64; bb = 17; continue; }
            16 => { rcx = (((rcx as u32)).wrapping_shl(8u32)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x7c00; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 17; continue; }
            17 => { rax = (crate::simd_ops::ru32(rbx)) as u32 as u64; rax = ((((rax as u32)) & (0xffffffu32))) as u32 as u64; rax = (((rax as u32)).wrapping_shl(3u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 18; continue; } rdx = (crate::simd_ops::ru64(rdx)) as u64; bb = 18; continue; }
            18 => { crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rdx); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), r9); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x40u64))) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rsp.wrapping_add(0x130u64), (xmm0) as u128); r8 = 0; fl_l = 0; fl_r = 0; rdx = rsp.wrapping_add(0x130); rcx = rsp.wrapping_add(0x258); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 19; continue; }
            19 => { rdx = (crate::simd_ops::ru32(rsi.wrapping_add(0x28u64))) as u32 as u64; r8 = (crate::simd_ops::ru32(rsi.wrapping_add(0x2cu64))) as u32 as u64; rcx = (((r8) as i64).wrapping_mul((0x108u64) as i64) as u64) as u64; rax = (((rdx) as i64).wrapping_mul((0x108u64) as i64) as u64) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 20; continue; } rax = rdx.wrapping_add(0x1); rdx = ((r8 as u32)) as u32 as u64; rdx = ((rdx).wrapping_shr(1u32)) as u64; rdx = ((rdx).wrapping_add(r8)) as u64; fl_l = rdx; fl_r = rax; rdx = (if (((fl_l as u64)) < ((fl_r as u64))) { rax } else { rdx }) as u64; rcx = rsi.wrapping_add(0x20); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P11, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 20; continue; }
            20 => { rax = (crate::simd_ops::ru32(rsi.wrapping_add(0x28u64))) as u32 as u64; rcx = (((rax) as i64).wrapping_mul((0x108u64) as i64) as u64) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rsi.wrapping_add(0x20u64)))) as u64; crate::simd_ops::lk_and_u32(rcx, (0xff000020u32) as u32); crate::simd_ops::lk_or_u32(rcx, (0x20u32) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x3u64), (0x20u8) as u8); crate::simd_ops::wu32(rcx.wrapping_add(0x4u64), ((rdi as u32)) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x8u64), (0x0u8) as u8); rdx = rsp.wrapping_add(0x258); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P8, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::lk_add_u32(rsi.wrapping_add(0x28u64), (1) as u32); rcx = rsp.wrapping_add(0x258); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = ((rbx).wrapping_add(0x108u64)) as u64; fl_l = rbx; fl_r = r14; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 15; continue; } bb = 21; continue; }
            21 => { rcx = rsp.wrapping_add(0xa8); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rsp.wrapping_add(0x80); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P12, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rsp.wrapping_add(0x70); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P13, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rsp.wrapping_add(0x50); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P13, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 22; continue; }
            22 => { r11 = rsp.wrapping_add(0x360); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x40u64))) as u64; rsp = r11; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            23 => { r8 = rbx; rdx = (crate::simd_ops::ru64(r14)) as u64; rcx = rsp.wrapping_add(0x130); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P14, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P15, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::lift_ops::trap(0x80000003, P16, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_23619<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 117>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rbp = rsp.wrapping_sub(0x280); rsp = ((rsp).wrapping_sub(0x380u64)) as u64; rdi = rdx; rdx = rcx; fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(0x54u64))) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) >= (((fl_r as u32)) as i32)) { bb = 5; continue; } rdx = 0; fl_l = 0; fl_r = 0; r8 = 0x310; rcx = rsp.wrapping_add(0x40); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rsp.wrapping_add(0x50); crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rcx); r14 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rsp.wrapping_add(0x4cu64), (0x10u32) as u32); crate::simd_ops::wu32(rbp.wrapping_add(0x258u64), (0x6u32) as u32); crate::simd_ops::wu64(rbp.wrapping_add(0x260u64), r14); crate::simd_ops::wu32(rbp.wrapping_add(0x268u64), (0xffffffffu32) as u32); crate::simd_ops::wu64(rbp.wrapping_add(0x270u64), r14); crate::simd_ops::wu64(rbp.wrapping_add(0x250u64), r14); xmm0 = 0u128; ymm0_hi = 0; let y_addr: u64 = rsp.wrapping_add(0x50u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); crate::simd_ops::wu128(rsp.wrapping_add(0x70u64), (xmm0) as u128); rax = ((r14.wrapping_add(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x48u64), ((rax as u32)) as u32); rbx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rbx = ((rbx).wrapping_shl(4u32)) as u64; rbx = ((rbx).wrapping_add(rcx)) as u64; crate::simd_ops::wu64(rbx.wrapping_sub(0x30u64), r14); rax = P1; crate::simd_ops::wu64(rbx.wrapping_sub(0x28u64), rax); crate::simd_ops::wu64(rbx.wrapping_sub(0x20u64), r14); crate::simd_ops::wu64(rbx.wrapping_sub(0x18u64), r14); crate::simd_ops::wu32(rbx.wrapping_sub(0x8u64), (0xffffffffu32) as u32); crate::simd_ops::wu64(rbx.wrapping_sub(0x10u64), r14); rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x260u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 1; continue; } rcx = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 1; continue; } rax = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm((crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rbx.wrapping_sub(0x18u64), rax); bb = 1; continue; }
            1 => { crate::simd_ops::wu32(rbp.wrapping_add(0x278u64), ((r14 as u32)) as u32); rsi = (crate::simd_ops::ru64(P2)) as u64; r9 = (crate::simd_ops::ru32(rsp.wrapping_add(0x48u64))) as u32 as u64; r8 = (crate::simd_ops::ru32(rsp.wrapping_add(0x4cu64))) as u32 as u64; rcx = ((r8.wrapping_add(r8.wrapping_mul(2))) as u64) as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rax = ((r9.wrapping_add(r9.wrapping_mul(2))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } rax = r9.wrapping_add(0x1); rdx = ((r8 as u32)) as u32 as u64; rdx = ((rdx).wrapping_shr(1u32)) as u64; rdx = ((rdx).wrapping_add(r8)) as u64; fl_l = rdx; fl_r = rax; rdx = (if (((fl_l as u64)) < ((fl_r as u64))) { rax } else { rdx }) as u64; rcx = rsp.wrapping_add(0x40); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r9 = (crate::simd_ops::ru32(rsp.wrapping_add(0x48u64))) as u32 as u64; bb = 2; continue; }
            2 => { rax = ((r9 as u32)) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rsp.wrapping_add(0x40u64)))) as u64; xmm0 = 0u128; ymm0_hi = 0; let y_addr: u64 = rcx; crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); crate::simd_ops::wu128(rcx.wrapping_add(0x20u64), (xmm0) as u128); rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x48u64))) as u32 as u64; rax = (((rax as u32)).wrapping_add(1)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x48u64), ((rax as u32)) as u32); rbx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rbx = ((rbx).wrapping_shl(4u32)) as u64; rbx = ((rbx).wrapping_add(crate::simd_ops::ru64(rsp.wrapping_add(0x40u64)))) as u64; crate::simd_ops::wu64(rbx.wrapping_sub(0x30u64), rdi); crate::simd_ops::wu64(rbx.wrapping_sub(0x28u64), rsi); crate::simd_ops::wu32(rbx.wrapping_sub(0x20u64), (0x2u32) as u32); crate::simd_ops::wu32(rbx.wrapping_sub(0x1cu64), (0x1u32) as u32); crate::simd_ops::wu64(rbx.wrapping_sub(0x18u64), r14); crate::simd_ops::wu32(rbx.wrapping_sub(0x8u64), (0xffffffffu32) as u32); crate::simd_ops::wu64(rbx.wrapping_sub(0x10u64), r14); rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x260u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } rcx = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } rax = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm((crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rbx.wrapping_sub(0x18u64), rax); bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x278u64))) as u32 as u64; rax = (((rax as u32)).wrapping_shr(1u32)) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 4; continue; } crate::simd_ops::wu64(rdi, r14); crate::simd_ops::wu32(rdi.wrapping_add(0x8u64), ((r14 as u32)) as u32); bb = 4; continue; }
            4 => { r8 = (crate::simd_ops::ru32(rsp.wrapping_add(0x48u64))) as u32 as u64; rcx = ((r8.wrapping_add(r8.wrapping_mul(2))) as u64) as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rax = (0x2aaaaaaaaaaaaaabu64) as u64; let prod = (rax as i64 as i128).wrapping_mul((rcx) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; rdx = ((((rdx) as i64).wrapping_shr(3u32) as u64)) as u64; rax = rdx; rax = ((rax).wrapping_shr(63u32)) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; rcx = ((rdx.wrapping_add(rdx.wrapping_mul(2))) as u64) as u64; rcx = ((rcx).wrapping_add(rcx)) as u64; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rcx = (crate::simd_ops::ru32(rax.wrapping_add(rcx.wrapping_mul(8)).wrapping_sub(0x8u64))) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x268u64), ((rcx as u32)) as u32); r8 = (((r8 as u32)).wrapping_sub(1)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x48u64), ((r8 as u32)) as u32); rcx = rsp.wrapping_add(0x40); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rsp = ((rsp).wrapping_add(0x380u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            5 => { rcx = ((rcx).wrapping_add(0x20u64)) as u64; rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x18u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rcx); rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x50u64))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rax as u32)) as u32); r9 = rdi; rdx = (crate::simd_ops::ru64(rdx.wrapping_add(0x48u64))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rsp = ((rsp).wrapping_add(0x380u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_23623<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut ymm0_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x28u64)) as u64; rax = ((crate::exe::tls_pointer() as u64)) as u64; rdx = 0x6348; rcx = (crate::simd_ops::ru64(rax)) as u64; rcx = (crate::simd_ops::ru64(rdx.wrapping_add(rcx))) as u64; r8 = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rdx = (crate::simd_ops::ru64(r8.wrapping_add(P1))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 1; continue; } r9 = 0; fl_l = 0; fl_r = 0; r8 = 0; fl_l = 0; fl_r = 0; rdx = ((r9.wrapping_add(P2)) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 2; continue; } (rax, xmm0) = crate::simd_ops::call_stk_xmm((crate::simd_ops::ru64(P4)) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rax; rdx = 0xdeadd00d; (rax, xmm0) = crate::simd_ops::call_stk_xmm((crate::simd_ops::ru64(P5)) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = 0; fl_l = 0; fl_r = 0; bb = 2; continue; }
            1 => { fl_l = (crate::simd_ops::lk_add_u32(r8.wrapping_add(P6), (1) as u32)) as u32 as u64; rax = (crate::simd_ops::ru64(rdx)) as u64; crate::simd_ops::wu64(r8.wrapping_add(P1), rax); bb = 2; continue; }
            2 => { rax = P7; xmm0 = 0u128; ymm0_hi = 0; let y_addr: u64 = rdx; crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); crate::simd_ops::wu64(rdx, rax); rax = rdx; crate::simd_ops::wu8(rdx.wrapping_add(P8), (0x1u8) as u8); crate::simd_ops::wu64(rdx.wrapping_add(P1), (0x0u64) as u64); crate::simd_ops::wu64(rdx.wrapping_add(P6), (0x0u64) as u64); rsp = ((rsp).wrapping_add(0x28u64)) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_23624<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
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
    crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), (0x2u32) as u32);
    rcx = rdi;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rdx = ((rbx.wrapping_add(P2)) as u64) as u64;
    rcx = rdi;
    rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp);
    r8 = (crate::simd_ops::ru32(rdi.wrapping_add(P4))) as u32 as u64;
    rax = (crate::simd_ops::ru64(rdi)) as u64;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64;
    rcx = ((r8.wrapping_add(r8.wrapping_mul(2))) as u64) as u64;
    rcx = ((rcx).wrapping_add(rcx)) as u64;
    rcx = (crate::simd_ops::ru32(rax.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(P5))) as u32 as u64;
    rax = ((r8.wrapping_add(P6)) as u32) as u32 as u64;
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
pub unsafe extern "win64" fn tpl_23625<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
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
    rdx = ((rbx.wrapping_add(P3)) as u64) as u64;
    r8 = 0x2;
    rcx = rdi;
    rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp);
    rcx = rdi;
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
pub unsafe extern "win64" fn tpl_23631<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64, const P9: u64, const P10: u64, const P11: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rax = ((crate::exe::tls_pointer() as u64)) as u64; rbx = 0; fl_l = 0; fl_r = 0; rdx = 0x6348; rcx = (crate::simd_ops::ru64(rax)) as u64; rdi = ((rbx.wrapping_add(P0)) as u32) as u32 as u64; rax = (crate::simd_ops::ru64(rdx.wrapping_add(rcx))) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64; r10 = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; fl_l = r10; fl_r = r10; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 1; continue; } fl_l = ((r10 as u8)) as u8 as u64; fl_r = (0xfu8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 1; continue; } fl_l = (crate::simd_ops::lk_add_u32(rcx.wrapping_add(P2), (1) as u32)) as u32 as u64; rax = (crate::simd_ops::ru64(r10)) as u64; crate::simd_ops::wu64(rcx.wrapping_add(P0), rax); bb = 2; continue; }
            1 => { r8 = rdi; rdx = 0xf0; rcx = rax; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); r10 = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 2; continue; } rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P4)) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = rax; rdx = 0xdeadd00d; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P5)) as u64, rax, rcx, rdx, r8, r9, rsp); r10 = rbx; bb = 2; continue; }
            2 => { rcx = r10; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); r11 = P7; crate::simd_ops::wu64(r10.wrapping_add(P8), rbx); crate::simd_ops::wu64(r10, r11); rax = r10; crate::simd_ops::wu64(r10.wrapping_add(P9), rbx); crate::simd_ops::wu64(r10.wrapping_add(P10), rbx); crate::simd_ops::wu64(r10.wrapping_add(P11), rbx); crate::simd_ops::wu64(r10.wrapping_add(0xd8u64), rbx); crate::simd_ops::wu16(r10.wrapping_add(0xe0u64), ((rdi as u16)) as u16); crate::simd_ops::wu64(r10.wrapping_add(0xe8u64), rbx); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_23632<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64, const P9: u64, const P10: u64, const P11: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = r9; rax = (((r9) as i64).wrapping_mul((0xf0u64) as i64) as u64) as u64; rdx = ((rax.wrapping_add(P0)) as u64) as u64; rcx = ((rdx.wrapping_add(P1)) as u64) as u64; fl_l = rax; fl_r = rax; rcx = (if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { rdx } else { rcx }) as u64; r14 = 0x10; rdx = ((r14 as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rsi = 0; fl_l = 0; fl_r = 0; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 1; continue; } rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P3)) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = rax; rdx = 0xdeadd00d; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P4)) as u64, rax, rcx, rdx, r8, r9, rsp); rax = ((rsi as u32)) as u32 as u64; bb = 1; continue; }
            1 => { rdi = ((rax.wrapping_add(P0)) as u64) as u64; crate::simd_ops::wu64(rax, (0xf0u64) as u64); crate::simd_ops::wu32(rax.wrapping_add(P5), ((rbx as u32)) as u32); crate::simd_ops::wu32(rax.wrapping_add(P6), (0x8u32) as u32); fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } r10 = ((rdi.wrapping_add(P7)) as u64) as u64; rbp = P8; bb = 2; continue; }
            2 => { rcx = ((r10.wrapping_add(P9)) as u64) as u64; rax = crate::simd_ops::call_stk(P10, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(r10.wrapping_add(P9), rbp); crate::simd_ops::wu64(r10.wrapping_add(P11), rsi); crate::simd_ops::wu64(r10.wrapping_sub(0x10u64), rsi); crate::simd_ops::wu64(r10.wrapping_sub(0x8u64), rsi); crate::simd_ops::wu64(r10, rsi); crate::simd_ops::wu64(r10.wrapping_add(0x8u64), rsi); crate::simd_ops::wu16(r10.wrapping_add(0x10u64), ((r14 as u16)) as u16); crate::simd_ops::wu64(r10.wrapping_add(0x18u64), rsi); r10 = r10.wrapping_add(0xf0); fl_l = rbx; fl_r = (0x1u64) as u64; rbx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 3; continue; }
            3 => { rax = rdi; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_23634<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = r9; rax = (((r9) as i64).wrapping_mul((0xf8u64) as i64) as u64) as u64; rdx = ((rax.wrapping_add(P0)) as u64) as u64; rcx = ((rdx.wrapping_add(P1)) as u64) as u64; fl_l = rax; fl_r = rax; rcx = (if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { rdx } else { rcx }) as u64; rdx = 0x10; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 1; continue; } rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P3)) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = rax; rdx = 0xdeadd00d; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P4)) as u64, rax, rcx, rdx, r8, r9, rsp); rax = 0; fl_l = 0; fl_r = 0; bb = 1; continue; }
            1 => { rsi = ((rax.wrapping_add(P0)) as u64) as u64; crate::simd_ops::wu64(rax, (0xf8u64) as u64); crate::simd_ops::wu32(rax.wrapping_add(P5), ((rdi as u32)) as u32); crate::simd_ops::wu32(rax.wrapping_add(P6), (0x8u32) as u32); fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } rbx = rsi; bb = 2; continue; }
            2 => { rcx = rbx; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); rbx = ((rbx).wrapping_add(0xf8u64)) as u64; fl_l = rdi; fl_r = (0x1u64) as u64; rdi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 3; continue; }
            3 => { rax = rsi; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_23670<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    rdx = (crate::simd_ops::ru64(rcx)) as u64;
    fl_l = rdx; fl_r = rdx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rcx = (crate::simd_ops::ru64(P0)) as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); }
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_23675<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r9); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rsi = r8; rdi = rcx; rbx = 0; fl_l = 0; fl_r = 0; rcx = 0x6348; rax = ((crate::exe::tls_pointer() as u64)) as u64; rdx = (crate::simd_ops::ru64(rax)) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(rdx))) as u64; r9 = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rdx = (crate::simd_ops::ru64(r9.wrapping_add(P1))) as u64; rax = rdx; rax = ((((rax as u32)) & (0xfu32))) as u32 as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 3; continue; } fl_l = (crate::simd_ops::lk_add_u32(r9.wrapping_add(P2), (1) as u32)) as u32 as u64; rax = (crate::simd_ops::ru64(rdx)) as u64; crate::simd_ops::wu64(r9.wrapping_add(P1), rax); bb = 1; continue; }
            1 => { rbx = rdx; bb = 2; continue; }
            2 => { crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), rbx); r8 = P3; rdx = rsi; rcx = rbx; rax = crate::simd_ops::call_stk_caught(P4, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rdi, rax); rax = rdi; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            3 => { rdx = 0x220; r8 = 0x10; rax = crate::simd_ops::call_stk_caught(P5, rax, rcx, rdx, r8, r9, rsp); rdx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 1; continue; } rax = crate::simd_ops::call_stk_caught((crate::simd_ops::ru64(P6)) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = rax; rdx = 0xdeadd00d; rax = crate::simd_ops::call_stk_caught((crate::simd_ops::ru64(P7)) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 2; continue; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_23678<const P0: u64, const P1: u64, const P2: u64, const P3: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0; let mut xmm6: u128 = 0; let mut xmm7: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 25>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = rsp; crate::simd_ops::wu64(rax.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rbp = rax.wrapping_sub(0x5f); rsp = ((rsp).wrapping_sub(0xc0u64)) as u64; crate::simd_ops::wu128(rax.wrapping_sub(0x18u64), (xmm6) as u128); rbx = rcx; crate::simd_ops::wu128(rax.wrapping_sub(0x28u64), (xmm7) as u128); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = rbx.wrapping_add(0x10); rcx = rbp.wrapping_sub(0x39); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm0 = ((crate::simd_ops::ru32(rbp.wrapping_sub(0x2du64))) as u32) as u128; ymm0_hi = 0; xmm1 = ((crate::simd_ops::ru32(rbp.wrapping_sub(0x29u64))) as u32) as u128; xmm6 = ((crate::simd_ops::ru32(P2)) as u32) as u128; xmm2 = ((crate::simd_ops::ru32(rbp.wrapping_sub(0x35u64))) as u32) as u128; xmm3 = ((crate::simd_ops::ru32(rbp.wrapping_sub(0x31u64))) as u32) as u128; crate::simd_ops::wu32(rbp.wrapping_add(0x7u64), ((xmm0 as u32)) as u32); xmm0 = ((crate::simd_ops::ru32(rbp.wrapping_sub(0x25u64))) as u32) as u128; ymm0_hi = 0; crate::simd_ops::wu32(rbp.wrapping_add(0xfu64), ((xmm0 as u32)) as u32); xmm0 = ((crate::simd_ops::ru32(rbp.wrapping_sub(0x21u64))) as u32) as u128; ymm0_hi = 0; crate::simd_ops::wu32(rbp.wrapping_add(0x17u64), ((xmm0 as u32)) as u32); xmm0 = ((crate::simd_ops::ru32(rbp.wrapping_sub(0x19u64))) as u32) as u128; ymm0_hi = 0; crate::simd_ops::wu32(rbp.wrapping_add(0xbu64), ((xmm1 as u32)) as u32); xmm1 = ((crate::simd_ops::ru32(rbp.wrapping_sub(0x1du64))) as u32) as u128; crate::simd_ops::wu32(rbp.wrapping_add(0x1fu64), ((xmm0 as u32)) as u32); xmm0 = ((crate::simd_ops::ru32(rbp.wrapping_sub(0x11u64))) as u32) as u128; ymm0_hi = 0; crate::simd_ops::wu32(rbp.wrapping_add(0x1bu64), ((xmm1 as u32)) as u32); xmm1 = ((crate::simd_ops::ru32(rbp.wrapping_sub(0x15u64))) as u32) as u128; crate::simd_ops::wu32(rbp.wrapping_add(0x2bu64), ((xmm0 as u32)) as u32); xmm0 = ((crate::simd_ops::ru32(rbp.wrapping_sub(0x39u64))) as u32) as u128; ymm0_hi = 0; crate::simd_ops::wu32(rbp.wrapping_add(0x27u64), ((xmm1 as u32)) as u32); xmm1 = ((crate::simd_ops::ru32(rbp.wrapping_sub(0xdu64))) as u32) as u128; crate::simd_ops::wu32(rbp.wrapping_add(0x2fu64), ((xmm1 as u32)) as u32); crate::simd_ops::wu32(rbp.wrapping_add(0x33u64), ((xmm6 as u32)) as u32); xmm7 = 0u128; crate::simd_ops::wu32(rbp.wrapping_add(0x13u64), ((xmm7 as u32)) as u32); crate::simd_ops::wu32(rbp.wrapping_add(0x23u64), ((xmm7 as u32)) as u32); crate::simd_ops::wu32(rbx.wrapping_add(0x4b0u64), ((xmm0 as u32)) as u32); crate::simd_ops::wu32(rbx.wrapping_add(0x4b4u64), ((xmm2 as u32)) as u32); crate::simd_ops::wu32(rbx.wrapping_add(0x4b8u64), ((xmm3 as u32)) as u32); r8 = 0; fl_l = 0; fl_r = 0; rdx = rbx.wrapping_add(0x70); crate::simd_ops::wu32(rbx.wrapping_add(0x4bcu64), ((r8 as u32)) as u32); rcx = rbp.wrapping_sub(0x39); let y_addr: u64 = rbp.wrapping_add(0x7u64); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rbx.wrapping_add(0x4c0u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); xmm0 = (crate::simd_ops::ru128(rbp.wrapping_add(0x27u64))) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rbx.wrapping_add(0x4e0u64), (xmm0) as u128); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm0 = ((crate::simd_ops::ru32(rbp.wrapping_sub(0x2du64))) as u32) as u128; ymm0_hi = 0; xmm1 = ((crate::simd_ops::ru32(rbp.wrapping_sub(0x29u64))) as u32) as u128; xmm2 = ((crate::simd_ops::ru32(rbp.wrapping_sub(0x35u64))) as u32) as u128; xmm3 = ((crate::simd_ops::ru32(rbp.wrapping_sub(0x31u64))) as u32) as u128; crate::simd_ops::wu32(rbp.wrapping_add(0x7u64), ((xmm0 as u32)) as u32); xmm0 = ((crate::simd_ops::ru32(rbp.wrapping_sub(0x25u64))) as u32) as u128; ymm0_hi = 0; crate::simd_ops::wu32(rbp.wrapping_add(0xfu64), ((xmm0 as u32)) as u32); xmm0 = ((crate::simd_ops::ru32(rbp.wrapping_sub(0x1du64))) as u32) as u128; ymm0_hi = 0; crate::simd_ops::wu32(rbp.wrapping_add(0xbu64), ((xmm1 as u32)) as u32); xmm1 = ((crate::simd_ops::ru32(rbp.wrapping_sub(0x21u64))) as u32) as u128; crate::simd_ops::wu32(rbp.wrapping_add(0x1bu64), ((xmm0 as u32)) as u32); xmm0 = ((crate::simd_ops::ru32(rbp.wrapping_sub(0x15u64))) as u32) as u128; ymm0_hi = 0; crate::simd_ops::wu32(rbp.wrapping_add(0x17u64), ((xmm1 as u32)) as u32); xmm1 = ((crate::simd_ops::ru32(rbp.wrapping_sub(0x19u64))) as u32) as u128; crate::simd_ops::wu32(rbp.wrapping_add(0x27u64), ((xmm0 as u32)) as u32); xmm0 = ((crate::simd_ops::ru32(rbp.wrapping_sub(0xdu64))) as u32) as u128; ymm0_hi = 0; crate::simd_ops::wu32(rbp.wrapping_add(0x1fu64), ((xmm1 as u32)) as u32); xmm1 = ((crate::simd_ops::ru32(rbp.wrapping_sub(0x11u64))) as u32) as u128; crate::simd_ops::wu32(rbp.wrapping_add(0x2fu64), ((xmm0 as u32)) as u32); crate::simd_ops::wu32(rbp.wrapping_add(0x2bu64), ((xmm1 as u32)) as u32); xmm1 = ((crate::simd_ops::ru32(rbp.wrapping_sub(0x39u64))) as u32) as u128; crate::simd_ops::wu32(rbp.wrapping_add(0x13u64), ((xmm7 as u32)) as u32); crate::simd_ops::wu32(rbp.wrapping_add(0x23u64), ((xmm7 as u32)) as u32); crate::simd_ops::wu32(rbp.wrapping_add(0x33u64), ((xmm6 as u32)) as u32); crate::simd_ops::wu32(rbx.wrapping_add(0x4f0u64), ((xmm1 as u32)) as u32); crate::simd_ops::wu32(rbx.wrapping_add(0x4f4u64), ((xmm2 as u32)) as u32); crate::simd_ops::wu32(rbx.wrapping_add(0x4f8u64), ((xmm3 as u32)) as u32); crate::simd_ops::wu32(rbx.wrapping_add(0x4fcu64), ((r8 as u32)) as u32); let y_addr: u64 = rbp.wrapping_add(0x7u64); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rbx.wrapping_add(0x500u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); xmm0 = (crate::simd_ops::ru128(rbp.wrapping_add(0x27u64))) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rbx.wrapping_add(0x520u64), (xmm0) as u128); xmm2 = ((crate::simd_ops::ru64(rbx.wrapping_add(0xb8u64))) as u64) as u128; xmm0 = ((crate::simd_ops::ru64(rbx.wrapping_add(0xc0u64))) as u64) as u128; ymm0_hi = 0; xmm2 = crate::simd_ops::vcvtpd2ps(xmm2); xmm1 = ((crate::simd_ops::ru64(rbx.wrapping_add(0xc8u64))) as u64) as u128; xmm0 = crate::simd_ops::vcvtpd2ps(xmm0); ymm0_hi = 0; xmm2 = crate::simd_ops::vinsertps(xmm2, xmm0, 0x10u8); xmm1 = crate::simd_ops::vcvtpd2ps(xmm1); xmm2 = crate::simd_ops::vinsertps(xmm2, xmm1, 0x20u8); xmm2 = crate::simd_ops::vinsertps(xmm2, xmm7, 0x30u8); crate::simd_ops::wu128(rbx.wrapping_add(0x7b0u64), (xmm2) as u128); xmm3 = ((crate::simd_ops::ru64(rbx.wrapping_add(0xa0u64))) as u64) as u128; xmm0 = ((crate::simd_ops::ru64(rbx.wrapping_add(0xa8u64))) as u64) as u128; ymm0_hi = 0; xmm1 = ((crate::simd_ops::ru64(rbx.wrapping_add(0xb0u64))) as u64) as u128; xmm3 = crate::simd_ops::vcvtpd2ps(xmm3); xmm0 = crate::simd_ops::vcvtpd2ps(xmm0); ymm0_hi = 0; xmm3 = crate::simd_ops::vinsertps(xmm3, xmm0, 0x10u8); xmm1 = crate::simd_ops::vcvtpd2ps(xmm1); xmm3 = crate::simd_ops::vinsertps(xmm3, xmm1, 0x20u8); xmm3 = crate::simd_ops::vinsertps(xmm3, xmm7, 0x30u8); crate::simd_ops::wu128(rbx.wrapping_add(0x7c0u64), (xmm3) as u128); rcx = rbx; r11 = rsp.wrapping_add(0xc0); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x10u64))) as u64; xmm6 = (crate::simd_ops::ru128(r11.wrapping_sub(0x10u64))) as u128; xmm7 = (crate::simd_ops::ru128(r11.wrapping_sub(0x20u64))) as u128; rsp = r11; rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_23688<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        core::sync::atomic::fence(core::sync::atomic::Ordering::SeqCst);
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbp = rcx; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rax = ((rax as u32)) as u32 as u64; rdx = P1; rdx = (crate::simd_ops::ru64(rdx.wrapping_add(rax.wrapping_mul(8)))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 1; continue; } fl_l = (crate::simd_ops::ru32(rdx)) as u32 as u64; fl_r = 0x2; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 1; continue; } rcx = rsp.wrapping_add(0x38); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rdi); rdi = (crate::simd_ops::ru64(rbp.wrapping_add(0x168u64))) as u64; rcx = rdi.wrapping_add(0x18); rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P3)) as u64, rax, rcx, rdx, r8, r9, rsp); rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0x20u64))) as u64; rsi = (crate::simd_ops::ru64(rdi.wrapping_add(0x28u64))) as u64; r8 = rsi.wrapping_add(0x20); fl_l = r8; fl_r = (crate::simd_ops::ru64(rdi.wrapping_add(0x10u64))) as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 5; continue; } fl_l = rsi; fl_r = rdx; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 3; continue; } bb = 2; continue; }
            2 => { fl_l = r8; fl_r = rdx; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 3; continue; } rcx = (crate::simd_ops::ru32(rdi.wrapping_add(0x30u64))) as u32 as u64; rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x34u64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 3; continue; } rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0x20u64))) as u64; fl_l = rsi; fl_r = rdx; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 2; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x10u64))) as u64; rax = ((rax).wrapping_sub(0x8u64)) as u64; fl_l = rsi; fl_r = rax; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 4; continue; } rax = (0xffffffffffu64) as u64; crate::simd_ops::wu64(rsi, rax); bb = 4; continue; }
            4 => { rsi = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; r8 = rsi.wrapping_add(0x20); bb = 5; continue; }
            5 => { fl_l = rsi; fl_r = rdx; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 7; continue; } bb = 6; continue; }
            6 => { fl_l = r8; fl_r = rdx; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 7; continue; } rcx = (crate::simd_ops::ru32(rdi.wrapping_add(0x30u64))) as u32 as u64; rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x34u64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 7; continue; } rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0x20u64))) as u64; fl_l = rsi; fl_r = rdx; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 6; continue; } bb = 7; continue; }
            7 => { rax = (0x69fafafafaffu64) as u64; rcx = rdi.wrapping_add(0x18); crate::simd_ops::wu64(rsi, rax); let tmp_xchg: u64 = crate::simd_ops::atomic_xchg_u64(rdi.wrapping_add(0x28u64), r8); r8 = (tmp_xchg) as u64; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P4)) as u64, rax, rcx, rdx, r8, r9, rsp); rax = P5; crate::simd_ops::wu32(rsi.wrapping_add(0x8u64), (0x0u32) as u32); crate::simd_ops::wu64(rsi.wrapping_add(0xcu64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; crate::simd_ops::wu64(rsi.wrapping_add(0x14u64), rax); rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x168u64))) as u64; crate::simd_ops::wu64(rsi, (0x14u64) as u64); let _ = crate::simd_ops::atomic_add_u32(rax.wrapping_add(0x30u64), (1) as u32); fl_l = (crate::simd_ops::ru8(rbp.wrapping_add(0x198u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 8; continue; } rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x1a0u64))) as u64; r8 = 0; fl_l = 0; fl_r = 0; rdx = ((r8.wrapping_add(0x1u64)) as u32) as u32 as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk((crate::simd_ops::ru64(P6)) as u64, rax, rcx, rdx, r8, r9, rsp); }
            8 => { rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            9 => { crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rdi); rdi = (crate::simd_ops::ru64(rbp.wrapping_add(0x168u64))) as u64; rcx = rdi.wrapping_add(0x18); rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P3)) as u64, rax, rcx, rdx, r8, r9, rsp); rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0x20u64))) as u64; rsi = (crate::simd_ops::ru64(rdi.wrapping_add(0x28u64))) as u64; r8 = rsi.wrapping_add(0x20); fl_l = r8; fl_r = (crate::simd_ops::ru64(rdi.wrapping_add(0x10u64))) as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 5; continue; } fl_l = rsi; fl_r = rdx; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 3; continue; } bb = 10; continue; }
            10 => { fl_l = r8; fl_r = rdx; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 3; continue; } rcx = (crate::simd_ops::ru32(rdi.wrapping_add(0x30u64))) as u32 as u64; rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x34u64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 3; continue; } rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0x20u64))) as u64; fl_l = rsi; fl_r = rdx; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 2; continue; } bb = 11; continue; }
            11 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x10u64))) as u64; rax = ((rax).wrapping_sub(0x8u64)) as u64; fl_l = rsi; fl_r = rax; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 4; continue; } rax = (0xffffffffffu64) as u64; crate::simd_ops::wu64(rsi, rax); bb = 12; continue; }
            12 => { rsi = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; r8 = rsi.wrapping_add(0x20); bb = 13; continue; }
            13 => { fl_l = rsi; fl_r = rdx; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 7; continue; } bb = 14; continue; }
            14 => { fl_l = r8; fl_r = rdx; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 7; continue; } rcx = (crate::simd_ops::ru32(rdi.wrapping_add(0x30u64))) as u32 as u64; rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x34u64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 7; continue; } rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0x20u64))) as u64; fl_l = rsi; fl_r = rdx; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 6; continue; } bb = 15; continue; }
            15 => { rax = (0x69fafafafaffu64) as u64; rcx = rdi.wrapping_add(0x18); crate::simd_ops::wu64(rsi, rax); let tmp_xchg: u64 = crate::simd_ops::atomic_xchg_u64(rdi.wrapping_add(0x28u64), r8); r8 = (tmp_xchg) as u64; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P4)) as u64, rax, rcx, rdx, r8, r9, rsp); rax = P5; crate::simd_ops::wu32(rsi.wrapping_add(0x8u64), (0x0u32) as u32); crate::simd_ops::wu64(rsi.wrapping_add(0xcu64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; crate::simd_ops::wu64(rsi.wrapping_add(0x14u64), rax); rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x168u64))) as u64; crate::simd_ops::wu64(rsi, (0x14u64) as u64); let _ = crate::simd_ops::atomic_add_u32(rax.wrapping_add(0x30u64), (1) as u32); fl_l = (crate::simd_ops::ru8(rbp.wrapping_add(0x198u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 8; continue; } rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x1a0u64))) as u64; r8 = 0; fl_l = 0; fl_r = 0; rdx = ((r8.wrapping_add(0x1u64)) as u32) as u32 as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk((crate::simd_ops::ru64(P6)) as u64, rax, rcx, rdx, r8, r9, rsp); }
            16 => { rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_23741<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
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
    if !(((((fl_l as u8)) & ((fl_r as u8))) == 0)) { rdx = 0x78; rcx = rdi; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); }
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rax = rdi;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_23749<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    crate::simd_ops::forward_incoming_to_outgoing();
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = (((crate::simd_ops::ru32(rcx.wrapping_add(P0))) as i32 as i64) as u64) as u64;
    fl_l = rcx; fl_r = rax;
    rcx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64;
    crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8(P1, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_23750<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    crate::simd_ops::wu64(rcx.wrapping_add(P0), (0x0u64) as u64);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_23751<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rcx;
    rsi = ((rcx.wrapping_add(P0)) as u64) as u64;
    rax = (crate::simd_ops::ru64(rsi)) as u64;
    rcx = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64;
    rdi = ((rbx.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64;
    fl_l = rbx; fl_r = rdi;
    if (((fl_l as u64)) == ((fl_r as u64))) { crate::simd_ops::wu64(rsi, (0x0u64) as u64); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    rcx = rbx;
    loop { rcx = ((rcx).wrapping_add(0x18u64)) as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rbx = ((rbx).wrapping_add(0x28u64)) as u64; rcx = rbx; fl_l = rbx; fl_r = rdi; if !((((fl_l as u64)) != ((fl_r as u64)))) { break; } }
    crate::simd_ops::wu64(rsi, (0x0u64) as u64);
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_23777<const P0: u64, const P1: u64, const P2: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = (crate::simd_ops::ru64(rcx)) as u64;
    fl_l = rbx; fl_r = rbx;
    if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    rdi = (crate::simd_ops::ru64(rbx.wrapping_add(P0))) as u64;
    fl_l = rdi; fl_r = rdi;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rcx = rdi; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rcx = rdi; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); }
    rdi = (crate::simd_ops::ru64(rbx)) as u64;
    fl_l = rdi; fl_r = rdi;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rcx = rdi; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rcx = rdi; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); }
    rcx = rbx;
    rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp);
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_23782<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64, const P9: u64, const P10: u64, const P11: u64, const P12: u64, const P13: u64, const P14: u64, const P15: u64, const P16: u64, const P17: u64, const P18: u64, const P19: u64, const P20: u64, const P21: u64, const P22: u64, const P23: u64, const P24: u64, const P25: u64, const P26: u64, const P27: u64, const P28: u64, const P29: u64, const P30: u64, const P31: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut xmm6: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 15>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rbp = rsp; rsp = ((rsp).wrapping_sub(0x70u64)) as u64; crate::simd_ops::wu128(rsp.wrapping_add(0x60u64), (xmm6) as u128); rdi = rdx; rbx = rcx; crate::simd_ops::wu8(rsp.wrapping_add(0x38u64), ((r8 as u8)) as u8); rax = P0; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), (0x8u32) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), (0x8u32) as u32); r8 = (crate::simd_ops::ru64(P1)) as u64; rcx = rdx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu32(rbp.wrapping_add(0x10u64), (0xffffffffu32) as u32); rdx = rbx.wrapping_add(0x10); rax = rbp.wrapping_add(0x10); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); rsi = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); r9 = P4; r8 = ((rsi.wrapping_add(0x8u64)) as u32) as u32 as u64; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu16(rbp.wrapping_add(0x10u64), ((rsi as u16)) as u16); rdx = rbx.wrapping_add(0x14); rax = rbp.wrapping_add(0x10); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); r9 = P6; r8 = ((rsi.wrapping_add(0x8u64)) as u32) as u32 as u64; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu16(rbp.wrapping_add(0x10u64), ((rsi as u16)) as u16); rdx = rbx.wrapping_add(0x16); rax = rbp.wrapping_add(0x10); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); r9 = P8; r8 = ((rsi.wrapping_add(0x8u64)) as u32) as u32 as u64; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu16(rbp.wrapping_add(0x10u64), ((rsi as u16)) as u16); rdx = rbx.wrapping_add(0x18); rax = rbp.wrapping_add(0x10); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); r9 = P9; r8 = ((rsi.wrapping_add(0x8u64)) as u32) as u32 as u64; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu16(rbp.wrapping_add(0x10u64), ((rsi as u16)) as u16); rdx = rbx.wrapping_add(0x1a); rax = rbp.wrapping_add(0x10); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); r9 = P10; r8 = ((rsi.wrapping_add(0x8u64)) as u32) as u32 as u64; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu8(rbp.wrapping_add(0x10u64), (0x1u8) as u8); rdx = rbx.wrapping_add(0x1c); rax = rbp.wrapping_add(0x10); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); r9 = P11; r8 = ((rsi.wrapping_add(0x8u64)) as u32) as u32 as u64; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P12, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu8(rbp.wrapping_add(0x10u64), (0xffu8) as u8); rdx = rbx.wrapping_add(0x1d); rax = rbp.wrapping_add(0x10); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); r9 = P13; r8 = ((rsi.wrapping_add(0x8u64)) as u32) as u32 as u64; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P14, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu8(rbp.wrapping_add(0x10u64), ((rsi as u8)) as u8); rdx = rbx.wrapping_add(0x1e); rax = rbp.wrapping_add(0x10); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); r9 = P15; r8 = ((rsi.wrapping_add(0x8u64)) as u32) as u32 as u64; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P14, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu8(rbp.wrapping_add(0x10u64), ((rsi as u8)) as u8); rdx = rbx.wrapping_add(0x1f); rax = rbp.wrapping_add(0x10); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); r9 = P16; r8 = ((rsi.wrapping_add(0x8u64)) as u32) as u32 as u64; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P14, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = rbx.wrapping_add(0x20); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); r9 = P17; r8 = ((rsi.wrapping_add(0x8u64)) as u32) as u32 as u64; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P18, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu32(rbp.wrapping_add(0x10u64), (0x1u32) as u32); rdx = rbx.wrapping_add(0x68); rax = rbp.wrapping_add(0x10); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); r9 = P19; r8 = ((rsi.wrapping_add(0x2u64)) as u32) as u32 as u64; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P20, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu32(rbp.wrapping_add(0x10u64), (0x1u32) as u32); rdx = rbx.wrapping_add(0x6c); rax = rbp.wrapping_add(0x10); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); r9 = P21; r8 = ((rsi.wrapping_add(0x2u64)) as u32) as u32 as u64; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P20, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm6 = 0u128; crate::simd_ops::wu32(rbp.wrapping_add(0x10u64), ((xmm6 as u32)) as u32); rdx = rbx.wrapping_add(0x70); rax = rbp.wrapping_add(0x10); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); r9 = P22; r8 = ((rsi.wrapping_add(0x5u64)) as u32) as u32 as u64; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P23, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm0 = ((crate::simd_ops::ru32(P24)) as u32) as u128; crate::simd_ops::wu32(rbp.wrapping_add(0x10u64), ((xmm0 as u32)) as u32); rdx = rbx.wrapping_add(0x74); rax = rbp.wrapping_add(0x10); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); r9 = P25; r8 = ((rsi.wrapping_add(0x5u64)) as u32) as u32 as u64; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P23, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu32(rbp.wrapping_add(0x10u64), ((xmm6 as u32)) as u32); rdx = rbx.wrapping_add(0x78); rax = rbp.wrapping_add(0x10); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); r9 = P26; r8 = ((rsi.wrapping_add(0x7u64)) as u32) as u32 as u64; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P23, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x28u64))) as u32 as u64; rcx = (((rax) as i64).wrapping_mul((0x108u64) as i64) as u64) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rdi.wrapping_add(0x20u64)))) as u64; fl_l = (crate::simd_ops::ru64(rdi.wrapping_add(0x20u64))) as u64; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } rax = 0; fl_l = 0; fl_r = 0; r11 = rsp.wrapping_add(0x70); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x18u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x20u64))) as u64; rdi = (crate::simd_ops::ru64(r11.wrapping_add(0x28u64))) as u64; xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x60u64))) as u128; rsp = r11; rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            1 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x40u64))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P27, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P28, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r8 = rax; rdx = rdi.wrapping_add(0x20); rcx = rbp.wrapping_sub(0x30); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P29, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P30, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::lift_ops::trap(0x80000003, P31, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_23791<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        core::sync::atomic::fence(core::sync::atomic::Ordering::SeqCst);
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbp = rdx; r15 = rcx; rax = crate::simd_ops::call_stk_caught(P0, rax, rcx, rdx, r8, r9, rsp); rax = ((rax as u32)) as u32 as u64; r8 = P1; r8 = (crate::simd_ops::ru64(r8.wrapping_add(rax.wrapping_mul(8)))) as u64; fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 1; continue; } fl_l = (crate::simd_ops::ru32(r8)) as u32 as u64; fl_r = 0x2; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 8; continue; } bb = 1; continue; }
            1 => { rdi = (crate::simd_ops::ru64(r15.wrapping_add(0x168u64))) as u64; rcx = rdi.wrapping_add(0x18); rax = crate::simd_ops::call_stk_caught((crate::simd_ops::ru64(P2)) as u64, rax, rcx, rdx, r8, r9, rsp); rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0x20u64))) as u64; rsi = (crate::simd_ops::ru64(rdi.wrapping_add(0x28u64))) as u64; r8 = rsi.wrapping_add(0xb0); fl_l = r8; fl_r = (crate::simd_ops::ru64(rdi.wrapping_add(0x10u64))) as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 5; continue; } fl_l = rsi; fl_r = rdx; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 3; continue; } bb = 2; continue; }
            2 => { fl_l = r8; fl_r = rdx; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 3; continue; } rcx = (crate::simd_ops::ru32(rdi.wrapping_add(0x30u64))) as u32 as u64; rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x34u64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 3; continue; } rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0x20u64))) as u64; fl_l = rsi; fl_r = rdx; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 2; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x10u64))) as u64; rax = ((rax).wrapping_sub(0x8u64)) as u64; fl_l = rsi; fl_r = rax; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 4; continue; } rax = (0xffffffffffu64) as u64; crate::simd_ops::wu64(rsi, rax); bb = 4; continue; }
            4 => { rsi = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; r8 = rsi.wrapping_add(0xb0); bb = 5; continue; }
            5 => { fl_l = rsi; fl_r = rdx; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 7; continue; } bb = 6; continue; }
            6 => { fl_l = r8; fl_r = rdx; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 7; continue; } rcx = (crate::simd_ops::ru32(rdi.wrapping_add(0x30u64))) as u32 as u64; rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x34u64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 7; continue; } rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0x20u64))) as u64; fl_l = rsi; fl_r = rdx; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 6; continue; } bb = 7; continue; }
            7 => { rax = (0x69fafafafaffu64) as u64; crate::simd_ops::wu64(rsi, rax); let tmp_xchg: u64 = crate::simd_ops::atomic_xchg_u64(rdi.wrapping_add(0x28u64), r8); r8 = (tmp_xchg) as u64; rcx = rdi.wrapping_add(0x18); rax = crate::simd_ops::call_stk_caught((crate::simd_ops::ru64(P3)) as u64, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu32(rsi.wrapping_add(0x8u64), (0x0u32) as u32); rax = P4; crate::simd_ops::wu64(rsi.wrapping_add(0xcu64), rax); rcx = rsi.wrapping_add(0x14); rdx = rbp; rax = crate::simd_ops::call_stk_caught(P5, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(r15.wrapping_add(0x168u64))) as u64; crate::simd_ops::wu64(rsi, (0xa4u64) as u64); let _ = crate::simd_ops::atomic_add_u32(rax.wrapping_add(0x30u64), (1) as u32); fl_l = (crate::simd_ops::ru8(r15.wrapping_add(0x198u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 8; continue; } r8 = 0; fl_l = 0; fl_r = 0; rdx = ((r8.wrapping_add(0x1u64)) as u32) as u32 as u64; rcx = (crate::simd_ops::ru64(r15.wrapping_add(0x1a0u64))) as u64; rax = crate::simd_ops::call_stk_caught((crate::simd_ops::ru64(P6)) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 8; continue; }
            8 => { rcx = rbp; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk_caught(P7, rax, rcx, rdx, r8, r9, rsp); }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_23792<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 127>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu32(rsp.wrapping_add(0x18u64), ((r8 as u32)) as u32); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rbp = rsp.wrapping_sub(0x2c0); rsp = ((rsp).wrapping_sub(0x3c0u64)) as u64; r13 = r9; rdi = rdx; r12 = rcx; fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(0x54u64))) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 7; continue; } rbx = (crate::simd_ops::ru64(rdx.wrapping_add(0x40u64))) as u64; r8 = rbp.wrapping_add(0x300); rdx = r9; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rsi = 0; fl_l = 0; fl_r = 0; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 6; continue; } rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x40u64))) as u64; r15 = (crate::simd_ops::ru64(rbp.wrapping_add(0x300u64))) as u64; fl_l = r15; fl_r = rcx; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 2; continue; } rax = ((rdi.wrapping_add(r15.wrapping_mul(8))) as u64) as u64; rcx = ((rdi.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = rcx; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rcx, rsi); rcx = ((rcx).wrapping_add(0x8u64)) as u64; fl_l = rcx; fl_r = rax; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::wu64(rdi.wrapping_add(0x40u64), r15); fl_l = rbx; fl_r = r15; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 5; continue; } r14 = r15; fl_l = r14; fl_r = rbx; r14 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; r15 = P1; bb = 3; continue; }
            3 => { rdx = 0; fl_l = 0; fl_r = 0; r8 = 0x310; rcx = rbp.wrapping_sub(0x80); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rbp.wrapping_sub(0x70); crate::simd_ops::wu64(rbp.wrapping_sub(0x80u64), rcx); crate::simd_ops::wu32(rbp.wrapping_sub(0x74u64), (0x10u32) as u32); crate::simd_ops::wu32(rbp.wrapping_add(0x298u64), (0x6u32) as u32); crate::simd_ops::wu64(rbp.wrapping_add(0x2a0u64), rsi); crate::simd_ops::wu32(rbp.wrapping_add(0x2a8u64), (0xffffffffu32) as u32); crate::simd_ops::wu64(rbp.wrapping_add(0x2b0u64), rsi); crate::simd_ops::wu64(rbp.wrapping_add(0x290u64), rsi); xmm0 = 0u128; ymm0_hi = 0; let y_addr: u64 = rbp.wrapping_sub(0x70u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); crate::simd_ops::wu128(rbp.wrapping_sub(0x50u64), (xmm0) as u128); rax = ((rsi.wrapping_add(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_sub(0x78u64), ((rax as u32)) as u32); rbx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rbx = ((rbx).wrapping_shl(4u32)) as u64; rbx = ((rbx).wrapping_add(rcx)) as u64; crate::simd_ops::wu64(rbx.wrapping_sub(0x30u64), rsi); crate::simd_ops::wu64(rbx.wrapping_sub(0x28u64), r15); crate::simd_ops::wu64(rbx.wrapping_sub(0x20u64), rsi); crate::simd_ops::wu64(rbx.wrapping_sub(0x18u64), rsi); crate::simd_ops::wu32(rbx.wrapping_sub(0x8u64), (0xffffffffu32) as u32); crate::simd_ops::wu64(rbx.wrapping_sub(0x10u64), rsi); rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x2a0u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } rcx = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } rax = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm((crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rbx.wrapping_sub(0x18u64), rax); bb = 4; continue; }
            4 => { crate::simd_ops::wu32(rbp.wrapping_add(0x2b8u64), ((rsi as u32)) as u32); rcx = rbp.wrapping_sub(0x80); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = r14; fl_r = (0x1u64) as u64; r14 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } r15 = (crate::simd_ops::ru64(rbp.wrapping_add(0x300u64))) as u64; bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x310u64))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rax as u32)) as u32); r9 = r13; r8 = r15; rdx = rdi; rcx = r12; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 7; continue; }
            6 => { crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rsi); xmm0 = 0u128; ymm0_hi = 0; let y_addr: u64 = rsp.wrapping_add(0x38u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); xmm1 = 0u128; crate::simd_ops::wu128(rsp.wrapping_add(0x58u64), (xmm1) as u128); crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), rsi); r9 = rsp.wrapping_add(0x30); r8 = rdi; rdx = r13; rcx = r12; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 7; continue; }
            7 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x408u64))) as u64; rsp = ((rsp).wrapping_add(0x3c0u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_23797<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64, const P9: u64, const P10: u64, const P11: u64, const P12: u64, const P13: u64, const P14: u64, const P15: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 113>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x360u64)) as u64; rbx = r9; r13 = r8; r12 = rdx; rsi = rcx; r15 = (crate::simd_ops::ru32(rsp.wrapping_add(0x3b0u64))) as u32 as u64; fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(0x54u64))) as u32 as u64; fl_r = ((r15 as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 22; continue; } rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; rdi = 0; fl_l = 0; fl_r = 0; r10 = ((rdi as u32)) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rdi); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rdi); r14 = (crate::simd_ops::ru64(rcx.wrapping_add(0x40u64))) as u64; fl_l = r14; fl_r = r14; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } rdx = rbx; rcx = r14; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 1; continue; } fl_l = (crate::simd_ops::ru16(rax.wrapping_add(0x1eu64))) as u16 as u64; fl_r = ((rdi as u16)) as u16 as u64; if (((fl_l as u16)) == ((fl_r as u16))) { bb = 1; continue; } r10 = rax.wrapping_add(0x10); fl_l = r10; fl_r = r10; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 1; continue; } fl_l = (crate::simd_ops::ru16(r10.wrapping_add(0xeu64))) as u16 as u64; fl_r = (0x4u16) as u16 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { bb = 23; continue; } crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), r10); rax = (crate::simd_ops::ru64(r14)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 2; continue; }
            1 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; r10 = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; bb = 2; continue; }
            2 => { rdx = (crate::simd_ops::ru64(rsi.wrapping_add(0x48u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x8u64))) as u64; r10 = (crate::simd_ops::ru32(rsi.wrapping_add(0x50u64))) as u32 as u64; r10 = ((r10).wrapping_shl(4u32)) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; rax = (0xffffffffffffu64) as u64; rcx = (((rcx) & (rax))) as u64; r10 = ((r10).wrapping_add(rcx)) as u64; fl_l = (crate::simd_ops::ru16(r10.wrapping_add(0xeu64))) as u16 as u64; fl_r = (0x4u16) as u16 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { bb = 5; continue; } crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), r10); rax = (crate::simd_ops::ru64(rdx)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); bb = 3; continue; }
            3 => { r14 = (crate::simd_ops::ru32(r10)) as u32 as u64; fl_l = r14; fl_r = r13; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 10; continue; } crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r13); r9 = rbx; r8 = P1; rdx = ((crate::simd_ops::ru8(P2)) as u32) as u32 as u64; rcx = rsp.wrapping_add(0x150); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rsi.wrapping_add(0x20); rdx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rsp.wrapping_add(0x150); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 22; continue; }
            4 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 3; continue; } bb = 5; continue; }
            5 => { crate::simd_ops::wu64(rsp.wrapping_add(0x150u64), (0x10000010u64) as u64); crate::simd_ops::wu8(rsp.wrapping_add(0x158u64), (0x0u8) as u8); fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 6; continue; } crate::simd_ops::wu32(rsp.wrapping_add(0x154u64), ((rdi as u32)) as u32); crate::simd_ops::wu8(rsp.wrapping_add(0x158u64), ((rbx as u8)) as u8); bb = 8; continue; }
            6 => { r9 = (0xffffffffffffffffu64) as u64; bb = 7; continue; }
            7 => { r9 = ((r9).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(r9))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 7; continue; } crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), r9); r8 = 0; fl_l = 0; fl_r = 0; rdx = rsp.wrapping_add(0x40); rcx = rsp.wrapping_add(0x150); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 8; continue; }
            8 => { rdx = (crate::simd_ops::ru32(rsi.wrapping_add(0x38u64))) as u32 as u64; r8 = (crate::simd_ops::ru32(rsi.wrapping_add(0x3cu64))) as u32 as u64; rcx = (((r8) as i64).wrapping_mul((0x88u64) as i64) as u64) as u64; rax = (((rdx) as i64).wrapping_mul((0x88u64) as i64) as u64) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 9; continue; } rax = rdx.wrapping_add(0x1); rdx = ((r8 as u32)) as u32 as u64; rdx = ((rdx).wrapping_shr(1u32)) as u64; rdx = ((rdx).wrapping_add(r8)) as u64; fl_l = rdx; fl_r = rax; rdx = (if (((fl_l as u64)) < ((fl_r as u64))) { rax } else { rdx }) as u64; rcx = rsi.wrapping_add(0x30); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 9; continue; }
            9 => { rax = (crate::simd_ops::ru32(rsi.wrapping_add(0x38u64))) as u32 as u64; rcx = (((rax) as i64).wrapping_mul((0x88u64) as i64) as u64) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rsi.wrapping_add(0x30u64)))) as u64; crate::simd_ops::lk_and_u32(rcx, (0xff000010u32) as u32); crate::simd_ops::lk_or_u32(rcx, (0x10u32) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x3u64), (0x10u8) as u8); crate::simd_ops::wu32(rcx.wrapping_add(0x4u64), ((rdi as u32)) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x8u64), (0x0u8) as u8); rdx = rsp.wrapping_add(0x150); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P8, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::lk_add_u32(rsi.wrapping_add(0x38u64), (1) as u32); rcx = rsp.wrapping_add(0x150); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 22; continue; }
            10 => { rax = (crate::simd_ops::ru32(rsi.wrapping_add(0x54u64))) as u32 as u64; xmm0 = 0u128; ymm0_hi = 0; crate::simd_ops::wu128(rsp.wrapping_add(0x70u64), (xmm0) as u128); crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), rdi); crate::simd_ops::wu64(rsp.wrapping_add(0x78u64), rdi); crate::simd_ops::wu8(rsp.wrapping_add(0x80u64), ((rdi as u8)) as u8); let y_addr: u64 = rsp.wrapping_add(0x90u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); crate::simd_ops::wu64(rsp.wrapping_add(0x90u64), rdi); crate::simd_ops::wu64(rsp.wrapping_add(0x98u64), rdi); crate::simd_ops::wu64(rsp.wrapping_add(0xa0u64), rdi); crate::simd_ops::wu64(rsp.wrapping_add(0xa8u64), rdi); crate::simd_ops::wu64(rsp.wrapping_add(0xb0u64), rdi); rcx = rsp.wrapping_add(0x30); crate::simd_ops::wu64(rsp.wrapping_add(0xb8u64), rcx); crate::simd_ops::wu32(rsp.wrapping_add(0xc0u64), ((rdi as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0xc4u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rsp.wrapping_add(0xc8u64))) as u32 as u64; rax = ((((rax as u32)) & (0xff000010u32))) as u32 as u64; fl_l = ((((rax as u32)) | (0x10u32))) as u32 as u64; fl_r = fl_l; rax = ((fl_l as u32)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0xc8u64), ((rax as u32)) as u32); crate::simd_ops::wu8(rsp.wrapping_add(0xcbu64), (0x10u8) as u8); crate::simd_ops::wu32(rsp.wrapping_add(0xccu64), ((rdi as u32)) as u32); crate::simd_ops::wu8(rsp.wrapping_add(0xd0u64), ((rdi as u8)) as u8); rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x18u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x88u64), rax); rbx = rdi; bb = 11; continue; }
            11 => { fl_l = rbx; fl_r = r14; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 12; continue; } crate::simd_ops::wu32(rsp.wrapping_add(0xc0u64), ((rbx as u32)) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), rdi); crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((r15 as u32)) as u32); rdx = rsp.wrapping_add(0x60); rcx = rsp.wrapping_add(0x70); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P9, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x40u64), ((rax as u32)) as u32); rdx = (crate::simd_ops::ru32(rsp.wrapping_add(0x64u64))) as u32 as u64; rdx = (((rdx as u32)).wrapping_add((rax as u32))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x44u64), ((rdx as u32)) as u32); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; crate::simd_ops::wu64(r12.wrapping_add(rbx.wrapping_mul(8)), rax); fl_l = ((rbx).wrapping_add(1)) as u64; rbx = ((fl_l as u64)) as u64; bb = 11; continue; }
            12 => { bb = 14; continue; }
            13 => { rdi = 0; fl_l = 0; fl_r = 0; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x390u64))) as u64; bb = 14; continue; }
            14 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x90u64))) as u64; rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x98u64))) as u32 as u64; r14 = (((rax) as i64).wrapping_mul((0x108u64) as i64) as u64) as u64; r14 = ((r14).wrapping_add(rbx)) as u64; fl_l = rbx; fl_r = r14; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 21; continue; } bb = 15; continue; }
            15 => { crate::simd_ops::wu64(rsp.wrapping_add(0x258u64), (0x20000020u64) as u64); crate::simd_ops::wu8(rsp.wrapping_add(0x260u64), (0x0u8) as u8); rax = rsp.wrapping_add(0x258); fl_l = rax; fl_r = rbx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 19; continue; } r9 = (crate::simd_ops::ru32(rbx.wrapping_add(0x4u64))) as u32 as u64; rdx = rbx.wrapping_add(0x8); rcx = ((crate::simd_ops::ru8(rbx.wrapping_add(0x3u64))) as u32) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x80; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 16; continue; } rcx = ((rcx.wrapping_mul(8)) as u32) as u32 as u64; bb = 17; continue; }
            16 => { rcx = (((rcx as u32)).wrapping_shl(8u32)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x7c00; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 17; continue; }
            17 => { rax = (crate::simd_ops::ru32(rbx)) as u32 as u64; rax = ((((rax as u32)) & (0xffffffu32))) as u32 as u64; rax = (((rax as u32)).wrapping_shl(3u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 18; continue; } rdx = (crate::simd_ops::ru64(rdx)) as u64; bb = 18; continue; }
            18 => { crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rdx); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), r9); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x40u64))) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rsp.wrapping_add(0x60u64), (xmm0) as u128); r8 = 0; fl_l = 0; fl_r = 0; rdx = rsp.wrapping_add(0x60); rcx = rsp.wrapping_add(0x258); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 19; continue; }
            19 => { rdx = (crate::simd_ops::ru32(rsi.wrapping_add(0x28u64))) as u32 as u64; r8 = (crate::simd_ops::ru32(rsi.wrapping_add(0x2cu64))) as u32 as u64; rcx = (((r8) as i64).wrapping_mul((0x108u64) as i64) as u64) as u64; rax = (((rdx) as i64).wrapping_mul((0x108u64) as i64) as u64) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 20; continue; } rax = rdx.wrapping_add(0x1); rdx = ((r8 as u32)) as u32 as u64; rdx = ((rdx).wrapping_shr(1u32)) as u64; rdx = ((rdx).wrapping_add(r8)) as u64; fl_l = rdx; fl_r = rax; rdx = (if (((fl_l as u64)) < ((fl_r as u64))) { rax } else { rdx }) as u64; rcx = rsi.wrapping_add(0x20); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P10, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 20; continue; }
            20 => { rax = (crate::simd_ops::ru32(rsi.wrapping_add(0x28u64))) as u32 as u64; rcx = (((rax) as i64).wrapping_mul((0x108u64) as i64) as u64) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rsi.wrapping_add(0x20u64)))) as u64; crate::simd_ops::lk_and_u32(rcx, (0xff000020u32) as u32); crate::simd_ops::lk_or_u32(rcx, (0x20u32) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x3u64), (0x20u8) as u8); crate::simd_ops::wu32(rcx.wrapping_add(0x4u64), ((rdi as u32)) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x8u64), (0x0u8) as u8); rdx = rsp.wrapping_add(0x258); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P8, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::lk_add_u32(rsi.wrapping_add(0x28u64), (1) as u32); rcx = rsp.wrapping_add(0x258); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = ((rbx).wrapping_add(0x108u64)) as u64; fl_l = rbx; fl_r = r14; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 15; continue; } bb = 21; continue; }
            21 => { rcx = rsp.wrapping_add(0xc8); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rsp.wrapping_add(0xa0); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P11, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rsp.wrapping_add(0x90); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P12, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rsp.wrapping_add(0x70); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P12, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 22; continue; }
            22 => { r11 = rsp.wrapping_add(0x360); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x40u64))) as u64; rsp = r11; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            23 => { r8 = rbx; rdx = (crate::simd_ops::ru64(r14)) as u64; rcx = rsp.wrapping_add(0x40); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P13, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P14, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::lift_ops::trap(0x80000003, P15, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_23800<const P0: u64, const P1: u64, const P2: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x48u64)) as u64;
    rax = ((crate::simd_ops::ru8(rcx.wrapping_add(P0))) as u32) as u32 as u64;
    r9 = rdx;
    rdx = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64;
    crate::simd_ops::wu8(rsp.wrapping_add(0x30u64), ((rax as u8)) as u8);
    rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x48u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_23803<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64, const P9: u64, const P10: u64, const P11: u64, const P12: u64, const P13: u64, const P14: u64, const P15: u64, const P16: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 115>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x370u64)) as u64; r13 = rdx; rsi = rcx; r12 = (crate::simd_ops::ru32(rsp.wrapping_add(0x3c0u64))) as u32 as u64; fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(0x54u64))) as u32 as u64; fl_r = ((r12 as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 19; continue; } rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; rdi = 0; fl_l = 0; fl_r = 0; r8 = ((rdi as u32)) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rdi); crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rdi); rbx = (crate::simd_ops::ru64(rcx.wrapping_add(0x40u64))) as u64; r14 = P0; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } rdx = r14; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 1; continue; } fl_l = (crate::simd_ops::ru16(rax.wrapping_add(0x1eu64))) as u16 as u64; fl_r = ((rdi as u16)) as u16 as u64; if (((fl_l as u16)) == ((fl_r as u16))) { bb = 1; continue; } r8 = rax.wrapping_add(0x10); fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 1; continue; } fl_l = (crate::simd_ops::ru16(r8.wrapping_add(0xeu64))) as u16 as u64; fl_r = (0x4u16) as u16 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { bb = 20; continue; } crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), r8); rax = (crate::simd_ops::ru64(rbx)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rax); rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 2; continue; }
            1 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; r8 = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; bb = 2; continue; }
            2 => { rdx = (crate::simd_ops::ru64(rsi.wrapping_add(0x48u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x8u64))) as u64; r8 = (crate::simd_ops::ru32(rsi.wrapping_add(0x50u64))) as u32 as u64; r8 = ((r8).wrapping_shl(4u32)) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; rax = (0xffffffffffffu64) as u64; rcx = (((rcx) & (rax))) as u64; r8 = ((r8).wrapping_add(rcx)) as u64; fl_l = (crate::simd_ops::ru16(r8.wrapping_add(0xeu64))) as u16 as u64; fl_r = (0x4u16) as u16 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { bb = 5; continue; } crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), r8); rax = (crate::simd_ops::ru64(rdx)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rax); bb = 3; continue; }
            3 => { r15 = (crate::simd_ops::ru32(r8)) as u32 as u64; fl_l = r15; fl_r = (0x2u64) as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 7; continue; } crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (0x2u64) as u64); r9 = r14; r8 = P2; rdx = ((crate::simd_ops::ru8(P3)) as u32) as u32 as u64; rcx = rsp.wrapping_add(0x160); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rsi.wrapping_add(0x20); rdx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rsp.wrapping_add(0x160); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 19; continue; }
            4 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 3; continue; } bb = 5; continue; }
            5 => { crate::simd_ops::wu64(rsp.wrapping_add(0x160u64), (0x10000010u64) as u64); crate::simd_ops::wu8(rsp.wrapping_add(0x168u64), (0x0u8) as u8); crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), r14); crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rdi); r9 = 0; fl_l = 0; fl_r = 0; r8 = 0; fl_l = 0; fl_r = 0; rdx = rsp.wrapping_add(0x50); rcx = rsp.wrapping_add(0x160); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = (crate::simd_ops::ru32(rsi.wrapping_add(0x38u64))) as u32 as u64; r8 = (crate::simd_ops::ru32(rsi.wrapping_add(0x3cu64))) as u32 as u64; rcx = (((r8) as i64).wrapping_mul((0x88u64) as i64) as u64) as u64; rax = (((rdx) as i64).wrapping_mul((0x88u64) as i64) as u64) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 6; continue; } rax = rdx.wrapping_add(0x1); rdx = ((r8 as u32)) as u32 as u64; rdx = ((rdx).wrapping_shr(1u32)) as u64; rdx = ((rdx).wrapping_add(r8)) as u64; fl_l = rdx; fl_r = rax; rdx = (if (((fl_l as u64)) < ((fl_r as u64))) { rax } else { rdx }) as u64; rcx = rsi.wrapping_add(0x30); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P8, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru32(rsi.wrapping_add(0x38u64))) as u32 as u64; rcx = (((rax) as i64).wrapping_mul((0x88u64) as i64) as u64) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rsi.wrapping_add(0x30u64)))) as u64; crate::simd_ops::lk_and_u32(rcx, (0xff000010u32) as u32); crate::simd_ops::lk_or_u32(rcx, (0x10u32) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x3u64), (0x10u8) as u8); crate::simd_ops::wu32(rcx.wrapping_add(0x4u64), ((rdi as u32)) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x8u64), (0x0u8) as u8); rdx = rsp.wrapping_add(0x160); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P9, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::lk_add_u32(rsi.wrapping_add(0x38u64), (1) as u32); rcx = rsp.wrapping_add(0x160); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 19; continue; }
            7 => { rax = (crate::simd_ops::ru32(rsi.wrapping_add(0x54u64))) as u32 as u64; xmm0 = 0u128; ymm0_hi = 0; crate::simd_ops::wu128(rsp.wrapping_add(0x60u64), (xmm0) as u128); crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), rdi); crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), rdi); crate::simd_ops::wu8(rsp.wrapping_add(0x70u64), ((rdi as u8)) as u8); let y_addr: u64 = rsp.wrapping_add(0x80u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); crate::simd_ops::wu64(rsp.wrapping_add(0x80u64), rdi); crate::simd_ops::wu64(rsp.wrapping_add(0x88u64), rdi); crate::simd_ops::wu64(rsp.wrapping_add(0x90u64), rdi); crate::simd_ops::wu64(rsp.wrapping_add(0x98u64), rdi); crate::simd_ops::wu64(rsp.wrapping_add(0xa0u64), rdi); rcx = rsp.wrapping_add(0x40); crate::simd_ops::wu64(rsp.wrapping_add(0xa8u64), rcx); crate::simd_ops::wu32(rsp.wrapping_add(0xb0u64), ((rdi as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0xb4u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rsp.wrapping_add(0xb8u64))) as u32 as u64; rax = ((((rax as u32)) & (0xff000010u32))) as u32 as u64; fl_l = ((((rax as u32)) | (0x10u32))) as u32 as u64; fl_r = fl_l; rax = ((fl_l as u32)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0xb8u64), ((rax as u32)) as u32); crate::simd_ops::wu8(rsp.wrapping_add(0xbbu64), (0x10u8) as u8); crate::simd_ops::wu32(rsp.wrapping_add(0xbcu64), ((rdi as u32)) as u32); crate::simd_ops::wu8(rsp.wrapping_add(0xc0u64), ((rdi as u8)) as u8); rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x18u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x78u64), rax); rbx = rdi; bb = 8; continue; }
            8 => { fl_l = rbx; fl_r = r15; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 9; continue; } crate::simd_ops::wu32(rsp.wrapping_add(0xb0u64), ((rbx as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x3c0u64), ((rdi as u32)) as u32); rdx = ((rbx.wrapping_mul(4)) as u64) as u64; rdx = ((rdx).wrapping_add(r13)) as u64; rax = rsp.wrapping_add(0x3c0); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rdi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); r9 = r14; r8 = ((r12 as u32)) as u32 as u64; rcx = rsp.wrapping_add(0x60); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P10, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rbx).wrapping_add(1)) as u64; rbx = ((fl_l as u64)) as u64; bb = 8; continue; }
            9 => { bb = 11; continue; }
            10 => { rdi = 0; fl_l = 0; fl_r = 0; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x3a0u64))) as u64; bb = 11; continue; }
            11 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x88u64))) as u32 as u64; r14 = (((rax) as i64).wrapping_mul((0x108u64) as i64) as u64) as u64; r14 = ((r14).wrapping_add(rbx)) as u64; fl_l = rbx; fl_r = r14; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 18; continue; } bb = 12; continue; }
            12 => { crate::simd_ops::wu64(rsp.wrapping_add(0x268u64), (0x20000020u64) as u64); crate::simd_ops::wu8(rsp.wrapping_add(0x270u64), (0x0u8) as u8); rax = rsp.wrapping_add(0x268); fl_l = rax; fl_r = rbx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 16; continue; } r9 = (crate::simd_ops::ru32(rbx.wrapping_add(0x4u64))) as u32 as u64; rdx = rbx.wrapping_add(0x8); rcx = ((crate::simd_ops::ru8(rbx.wrapping_add(0x3u64))) as u32) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x80; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 13; continue; } rcx = ((rcx.wrapping_mul(8)) as u32) as u32 as u64; bb = 14; continue; }
            13 => { rcx = (((rcx as u32)).wrapping_shl(8u32)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x7c00; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 14; continue; }
            14 => { rax = (crate::simd_ops::ru32(rbx)) as u32 as u64; rax = ((((rax as u32)) & (0xffffffu32))) as u32 as u64; rax = (((rax as u32)).wrapping_shl(3u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 15; continue; } rdx = (crate::simd_ops::ru64(rdx)) as u64; bb = 15; continue; }
            15 => { crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rdx); crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), r9); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x50u64))) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rsp.wrapping_add(0x140u64), (xmm0) as u128); r8 = 0; fl_l = 0; fl_r = 0; rdx = rsp.wrapping_add(0x140); rcx = rsp.wrapping_add(0x268); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 16; continue; }
            16 => { rdx = (crate::simd_ops::ru32(rsi.wrapping_add(0x28u64))) as u32 as u64; r8 = (crate::simd_ops::ru32(rsi.wrapping_add(0x2cu64))) as u32 as u64; rcx = (((r8) as i64).wrapping_mul((0x108u64) as i64) as u64) as u64; rax = (((rdx) as i64).wrapping_mul((0x108u64) as i64) as u64) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 17; continue; } rax = rdx.wrapping_add(0x1); rdx = ((r8 as u32)) as u32 as u64; rdx = ((rdx).wrapping_shr(1u32)) as u64; rdx = ((rdx).wrapping_add(r8)) as u64; fl_l = rdx; fl_r = rax; rdx = (if (((fl_l as u64)) < ((fl_r as u64))) { rax } else { rdx }) as u64; rcx = rsi.wrapping_add(0x20); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P11, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 17; continue; }
            17 => { rax = (crate::simd_ops::ru32(rsi.wrapping_add(0x28u64))) as u32 as u64; rcx = (((rax) as i64).wrapping_mul((0x108u64) as i64) as u64) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rsi.wrapping_add(0x20u64)))) as u64; crate::simd_ops::lk_and_u32(rcx, (0xff000020u32) as u32); crate::simd_ops::lk_or_u32(rcx, (0x20u32) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x3u64), (0x20u8) as u8); crate::simd_ops::wu32(rcx.wrapping_add(0x4u64), ((rdi as u32)) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x8u64), (0x0u8) as u8); rdx = rsp.wrapping_add(0x268); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P9, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::lk_add_u32(rsi.wrapping_add(0x28u64), (1) as u32); rcx = rsp.wrapping_add(0x268); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = ((rbx).wrapping_add(0x108u64)) as u64; fl_l = rbx; fl_r = r14; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 12; continue; } bb = 18; continue; }
            18 => { rcx = rsp.wrapping_add(0xb8); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rsp.wrapping_add(0x90); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P12, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rsp.wrapping_add(0x80); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P13, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rsp.wrapping_add(0x60); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P13, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 19; continue; }
            19 => { r11 = rsp.wrapping_add(0x370); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x40u64))) as u64; rsp = r11; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            20 => { r8 = r14; rdx = (crate::simd_ops::ru64(rbx)) as u64; rcx = rsp.wrapping_add(0x140); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P14, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P15, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::lift_ops::trap(0x80000003, P16, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_23818<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64, const P9: u64, const P10: u64, const P11: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rsi = rcx; rbx = (crate::simd_ops::ru64(rcx)) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; rbp = ((rbx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; fl_l = rbx; fl_r = rbp; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 3; continue; } bb = 1; continue; }
            1 => { rdi = (crate::simd_ops::ru64(rbx)) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } rcx = rdi; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rcx = rdi; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 2; continue; }
            2 => { rbx = ((rbx).wrapping_add(0x8u64)) as u64; fl_l = rbx; fl_r = rbp; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } bb = 3; continue; }
            3 => { r9 = (crate::simd_ops::ru64(rsi)) as u64; fl_l = r9; fl_r = r9; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } rcx = ((r9.wrapping_add(P3)) as u64) as u64; rcx = (((rcx) & (0xfffffffffe000000u64))) as u64; rax = r9; rax = ((rax).wrapping_sub(rcx)) as u64; rax = ((rax).wrapping_shr(16u32)) as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rdx = ((rcx.wrapping_add(P4)) as u64) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(P5))) as u32 as u64; rdx = ((rdx).wrapping_sub(rax)) as u64; rax = ((crate::exe::teb_self() as u64)) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(rcx.wrapping_add(P6))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 5; continue; } fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(P7))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 4; continue; } rax = (crate::simd_ops::ru64(rdx.wrapping_add(P8))) as u64; crate::simd_ops::wu64(r9, rax); crate::simd_ops::wu64(rdx.wrapping_add(P8), r9); fl_l = (crate::simd_ops::lk_add_u32(rdx.wrapping_add(P9), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; if (((fl_l as u32)) != 0) { bb = 7; continue; } rcx = rdx; rax = crate::simd_ops::call_stk(P10, rax, rcx, rdx, r8, r9, rsp); bb = 7; continue; }
            4 => { r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 6; continue; }
            5 => { r8 = 0; fl_l = 0; fl_r = 0; bb = 6; continue; }
            6 => { rax = crate::simd_ops::call_stk(P11, rax, rcx, rdx, r8, r9, rsp); bb = 7; continue; }
            7 => { rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsi, rax); crate::simd_ops::wu64(rsi.wrapping_add(0x8u64), rax); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_23827<const P0: u64, const P1: u64, const P2: u64, const P3: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rcx;
    rax = P0;
    crate::simd_ops::wu64(rcx.wrapping_add(P1), rax);
    rax = P2;
    crate::simd_ops::wu64(rcx, rax);
    rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp);
    rax = rbx;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_23850<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rcx;
    rax = (crate::simd_ops::ru64(rdx)) as u64;
    crate::simd_ops::wu64(rdx, (0x0u64) as u64);
    rdi = (crate::simd_ops::ru64(rcx)) as u64;
    crate::simd_ops::wu64(rcx, rax);
    fl_l = rdi; fl_r = rdi;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rcx = rdi; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rcx = rdi; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); }
    rax = rbx;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_23854<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rcx;
    rcx = ((rcx).wrapping_add(0xc0u64)) as u64;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rcx = ((rbx.wrapping_add(P1)) as u64) as u64;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rcx = ((rbx.wrapping_add(P2)) as u64) as u64;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rax = P3;
    crate::simd_ops::wu64(rbx, rax);
    rcx = rbx;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return crate::simd_ops::tail_stk(P4, rax, rcx, rdx, r8, r9, rsp);
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_23857<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = ((rdx as u32)) as u32 as u64;
    rdi = rcx;
    rcx = ((rcx).wrapping_add(0xc0u64)) as u64;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rcx = ((rdi.wrapping_add(P1)) as u64) as u64;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rcx = ((rdi.wrapping_add(P2)) as u64) as u64;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rax = P3;
    crate::simd_ops::wu64(rdi, rax);
    rcx = rdi;
    rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp);
    fl_l = ((rbx as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64;
    if !(((((fl_l as u8)) & ((fl_r as u8))) == 0)) { rdx = 0xd0; rcx = rdi; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); }
    rax = rdi;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_23860<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu64(rcx.wrapping_add(P0), rax);
    crate::simd_ops::wu32(rcx.wrapping_add(P1), ((rax as u32)) as u32);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_23861<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rdi = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    r8 = rdx;
    rsi = rcx;
    fl_l = rdx; fl_r = rdi;
    if !((((fl_l as u64)) < ((fl_r as u64)))) { rcx = ((rdi.wrapping_add(rdx)) as u64) as u64; rdx = 0; fl_l = 0; fl_r = 0; rax = ((rcx.wrapping_add(P1)) as u64) as u64; let d = (rdi) as u128; let n = ((rdx as u128) << 64) | (rax as u128); if d != 0 { rax = (n / d) as u64; rdx = (n % d) as u64; } rdi = ((rcx.wrapping_add(P1)) as u64) as u64; fl_l = rdi; fl_r = rdx; rdi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; }
    rdx = rdi;
    crate::simd_ops::wu64(rsi.wrapping_add(P2), r8);
    rcx = ((rsi.wrapping_add(P3)) as u64) as u64;
    rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp);
    rcx = ((rsi.wrapping_add(P3)) as u64) as u64;
    rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp);
    rcx = ((rsi.wrapping_add(P6)) as u64) as u64;
    rdx = rdi;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return crate::simd_ops::tail_stk(P7, rax, rcx, rdx, r8, r9, rsp);
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_23862<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    r8 = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    rsi = rcx;
    rdi = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64;
    r8 = ((r8).wrapping_add(rdx)) as u64;
    fl_l = r8; fl_r = rdi;
    if !((((fl_l as u64)) < ((fl_r as u64)))) { rcx = ((rdi.wrapping_add(r8)) as u64) as u64; rdx = 0; fl_l = 0; fl_r = 0; rax = ((rcx.wrapping_add(P2)) as u64) as u64; let d = (rdi) as u128; let n = ((rdx as u128) << 64) | (rax as u128); if d != 0 { rax = (n / d) as u64; rdx = (n % d) as u64; } rdi = ((rcx.wrapping_add(P2)) as u64) as u64; fl_l = rdi; fl_r = rdx; rdi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; }
    rdx = rdi;
    crate::simd_ops::wu64(rsi.wrapping_add(P0), r8);
    rcx = ((rsi.wrapping_add(P3)) as u64) as u64;
    rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp);
    rcx = ((rsi.wrapping_add(P3)) as u64) as u64;
    rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp);
    rcx = ((rsi.wrapping_add(P6)) as u64) as u64;
    rdx = rdi;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return crate::simd_ops::tail_stk(P7, rax, rcx, rdx, r8, r9, rsp);
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_23865<const P0: u64, const P1: u64, const P2: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
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
    if !(((((fl_l as u8)) & ((fl_r as u8))) == 0)) { rdx = 0x90; rcx = rdi; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); }
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rax = rdi;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_23888<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0; let mut ymm1_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 35>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rbp = rsp.wrapping_sub(0x27); rsp = ((rsp).wrapping_sub(0xe0u64)) as u64; rbx = (((r9 as u8)) as u32) as u32 as u64; r14 = r8; r9 = (crate::simd_ops::ru64(rdx)) as u64; r15 = rdx; r8 = (crate::simd_ops::ru64(rcx)) as u64; rax = r9; rax = ((rax).wrapping_sub(r8)) as u64; r12 = rcx; fl_l = rax; fl_r = (0x14a0u64) as u64; if ((((fl_l as u64)) as i64) < (((fl_r as u64)) as i64)) { bb = 4; continue; } r13 = (0x6666666666666667u64) as u64; bb = 1; continue; }
            1 => { fl_l = r14; fl_r = r14; if (((((fl_l as u64)) & ((fl_r as u64))) as i64) <= 0) { bb = 6; continue; } crate::simd_ops::wu64(rbp.wrapping_add(0x67u64), r9); rdx = rbp.wrapping_add(0x6f); crate::simd_ops::wu64(rbp.wrapping_add(0x6fu64), r8); rcx = rsp.wrapping_add(0x30); r9 = (((rbx as u8)) as u32) as u32 as u64; r8 = rbp.wrapping_add(0x67); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r10 = (crate::simd_ops::ru64(r15)) as u64; rcx = r14; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rax = r13; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; r9 = (crate::simd_ops::ru64(r12)) as u64; rcx = ((((rcx) as i64).wrapping_shr(2u32) as u64)) as u64; r14 = ((((r14) as i64).wrapping_shr(1u32) as u64)) as u64; r14 = ((r14).wrapping_add(rcx)) as u64; rcx = r10; rcx = ((rcx).wrapping_sub(rdi)) as u64; let prod = (rax as i64 as i128).wrapping_mul((rcx) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; rcx = rsi; r8 = rdx; rcx = ((rcx).wrapping_sub(r9)) as u64; r8 = ((((r8) as i64).wrapping_shr(6u32) as u64)) as u64; rax = r8; rax = ((rax).wrapping_shr(63u32)) as u64; r8 = ((r8).wrapping_add(rax)) as u64; rax = r13; let prod = (rax as i64 as i128).wrapping_mul((rcx) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; rcx = rbp.wrapping_add(0x6f); rdx = ((((rdx) as i64).wrapping_shr(6u32) as u64)) as u64; rax = rdx; rax = ((rax).wrapping_shr(63u32)) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; fl_l = rdx; fl_r = r8; r8 = r14; rdx = rbp.wrapping_add(0x67); if ((((fl_l as u64)) as i64) >= (((fl_r as u64)) as i64)) { bb = 2; continue; } crate::simd_ops::wu64(rbp.wrapping_add(0x6fu64), r9); r9 = (((rbx as u8)) as u32) as u32 as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x67u64), rsi); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(r12, rdi); bb = 3; continue; }
            2 => { r9 = (((rbx as u8)) as u32) as u32 as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x67u64), r10); crate::simd_ops::wu64(rbp.wrapping_add(0x6fu64), rdi); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(r15, rsi); bb = 3; continue; }
            3 => { r9 = (crate::simd_ops::ru64(r15)) as u64; r8 = (crate::simd_ops::ru64(r12)) as u64; rax = r9; rax = ((rax).wrapping_sub(r8)) as u64; fl_l = rax; fl_r = (0x14a0u64) as u64; if ((((fl_l as u64)) as i64) >= (((fl_r as u64)) as i64)) { bb = 1; continue; } bb = 4; continue; }
            4 => { crate::simd_ops::wu64(rbp.wrapping_add(0x6fu64), r8); rdx = rbp.wrapping_add(0x6f); r8 = rbp.wrapping_add(0x67); crate::simd_ops::wu64(rbp.wrapping_add(0x67u64), r9); rcx = rsp.wrapping_add(0x30); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 5; continue; }
            5 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x130u64))) as u64; rsp = ((rsp).wrapping_add(0xe0u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            6 => { r9 = ((r9).wrapping_sub(r8)) as u64; rax = r13; let prod = (rax as i64 as i128).wrapping_mul((r9) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; r14 = rdx; r14 = ((((r14) as i64).wrapping_shr(6u32) as u64)) as u64; rax = r14; rax = ((rax).wrapping_shr(63u32)) as u64; r14 = ((r14).wrapping_add(rax)) as u64; rsi = r14; rsi = ((((rsi) as i64).wrapping_shr(1u32) as u64)) as u64; fl_l = rsi; fl_r = rsi; if (((((fl_l as u64)) & ((fl_r as u64))) as i64) <= 0) { bb = 8; continue; } rdi = ((rsi.wrapping_add(rsi.wrapping_mul(4))) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x67u64), r8); rdi = ((rdi).wrapping_shl(5u32)) as u64; rdi = ((rdi).wrapping_add(0x38u64)) as u64; fl_l = ((rdi).wrapping_add(r8)) as u64; fl_r = fl_l; rdi = ((fl_l as u64)) as u64; bb = 7; continue; }
            7 => { rdi = rdi.wrapping_sub(0xa0); crate::simd_ops::wu8(rsp.wrapping_add(0x20u64), ((rbx as u8)) as u8); xmm0 = ((crate::simd_ops::ru32(rdi.wrapping_sub(0x2cu64))) as u32) as u128; ymm0_hi = 0; rax = (crate::simd_ops::ru32(rdi.wrapping_sub(0x38u64))) as u32 as u64; r9 = rbp.wrapping_sub(0x79); xmm1 = (crate::simd_ops::ru128(rdi.wrapping_sub(0x28u64))) as u128; ymm1_hi = 0; crate::simd_ops::wu32(rbp.wrapping_sub(0x79u64), ((rax as u32)) as u32); rcx = rbp.wrapping_add(0x67); rax = (crate::simd_ops::ru32(rdi.wrapping_sub(0x34u64))) as u32 as u64; rsi = ((rsi).wrapping_sub(1)) as u64; crate::simd_ops::wu32(rbp.wrapping_sub(0x6du64), ((xmm0 as u32)) as u32); xmm0 = ((crate::simd_ops::ru64(rdi.wrapping_sub(0x18u64))) as u64) as u128; ymm0_hi = 0; crate::simd_ops::wu32(rbp.wrapping_sub(0x75u64), ((rax as u32)) as u32); r8 = r14; rax = (crate::simd_ops::ru32(rdi.wrapping_sub(0x30u64))) as u32 as u64; rdx = rsi; crate::simd_ops::wu64(rbp.wrapping_sub(0x59u64), ((xmm0 as u64)) as u64); xmm0 = ((crate::simd_ops::ru64(rdi.wrapping_sub(0xcu64))) as u64) as u128; ymm0_hi = 0; crate::simd_ops::wu32(rbp.wrapping_sub(0x71u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdi.wrapping_sub(0x10u64))) as u32 as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x4du64), ((xmm0 as u64)) as u64); xmm0 = ((crate::simd_ops::ru64(rdi)) as u64) as u128; ymm0_hi = 0; crate::simd_ops::wu32(rbp.wrapping_sub(0x51u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdi.wrapping_sub(0x4u64))) as u32 as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x41u64), ((xmm0 as u64)) as u64); xmm0 = ((crate::simd_ops::ru64(rdi.wrapping_add(0xcu64))) as u64) as u128; ymm0_hi = 0; crate::simd_ops::wu32(rbp.wrapping_sub(0x45u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x8u64))) as u32 as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x35u64), ((xmm0 as u64)) as u64); let y_addr: u64 = rdi.wrapping_add(0x20u64); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); crate::simd_ops::wu32(rbp.wrapping_sub(0x39u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x14u64))) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_sub(0x2du64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x18u64))) as u32 as u64; let y_addr: u64 = rbp.wrapping_sub(0x21u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); xmm0 = ((crate::simd_ops::ru64(rdi.wrapping_add(0x60u64))) as u64) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rbp.wrapping_sub(0x69u64), (xmm1) as u128); let y_addr: u64 = rdi.wrapping_add(0x40u64); xmm1 = crate::simd_ops::ru128(y_addr); ymm1_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); crate::simd_ops::wu32(rbp.wrapping_sub(0x29u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x1cu64))) as u32 as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x1fu64), ((xmm0 as u64)) as u64); crate::simd_ops::wu32(rbp.wrapping_sub(0x25u64), ((rax as u32)) as u32); let y_addr: u64 = rbp.wrapping_sub(0x1u64); crate::simd_ops::wu128(y_addr, xmm1); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm1_hi); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = rsi; fl_r = rsi; if (((((fl_l as u64)) & ((fl_r as u64))) as i64) > 0) { bb = 7; continue; } bb = 8; continue; }
            8 => { rcx = (crate::simd_ops::ru64(r15)) as u64; rsi = (crate::simd_ops::ru64(r12)) as u64; rax = rcx; rax = ((rax).wrapping_sub(rsi)) as u64; fl_l = rax; fl_r = (0x140u64) as u64; if ((((fl_l as u64)) as i64) < (((fl_r as u64)) as i64)) { bb = 5; continue; } bb = 9; continue; }
            9 => { rdi = rcx.wrapping_sub(0xa0); rax = r13; rcx = ((rcx).wrapping_sub(rsi)) as u64; let prod = (rax as i64 as i128).wrapping_mul((rcx) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; rdx = ((((rdx) as i64).wrapping_shr(6u32) as u64)) as u64; rax = rdx; rax = ((rax).wrapping_shr(63u32)) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; fl_l = rdx; fl_r = (0x2u64) as u64; if ((((fl_l as u64)) as i64) < (((fl_r as u64)) as i64)) { bb = 10; continue; } xmm0 = ((crate::simd_ops::ru32(rdi.wrapping_add(0xcu64))) as u32) as u128; ymm0_hi = 0; rax = (crate::simd_ops::ru32(rdi)) as u32 as u64; rdx = rsi; xmm1 = (crate::simd_ops::ru128(rdi.wrapping_add(0x10u64))) as u128; ymm1_hi = 0; crate::simd_ops::wu32(rbp.wrapping_sub(0x79u64), ((rax as u32)) as u32); rcx = rdi; rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x4u64))) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_sub(0x6du64), ((xmm0 as u32)) as u32); xmm0 = ((crate::simd_ops::ru64(rdi.wrapping_add(0x20u64))) as u64) as u128; ymm0_hi = 0; crate::simd_ops::wu32(rbp.wrapping_sub(0x75u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x8u64))) as u32 as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x59u64), ((xmm0 as u64)) as u64); xmm0 = ((crate::simd_ops::ru64(rdi.wrapping_add(0x2cu64))) as u64) as u128; ymm0_hi = 0; crate::simd_ops::wu32(rbp.wrapping_sub(0x71u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x28u64))) as u32 as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x4du64), ((xmm0 as u64)) as u64); xmm0 = ((crate::simd_ops::ru64(rdi.wrapping_add(0x38u64))) as u64) as u128; ymm0_hi = 0; crate::simd_ops::wu32(rbp.wrapping_sub(0x51u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x34u64))) as u32 as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x41u64), ((xmm0 as u64)) as u64); xmm0 = ((crate::simd_ops::ru64(rdi.wrapping_add(0x44u64))) as u64) as u128; ymm0_hi = 0; crate::simd_ops::wu32(rbp.wrapping_sub(0x45u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x40u64))) as u32 as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x35u64), ((xmm0 as u64)) as u64); let y_addr: u64 = rdi.wrapping_add(0x58u64); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); crate::simd_ops::wu32(rbp.wrapping_sub(0x39u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x4cu64))) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_sub(0x2du64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x50u64))) as u32 as u64; let y_addr: u64 = rbp.wrapping_sub(0x21u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); xmm0 = ((crate::simd_ops::ru64(rdi.wrapping_add(0x98u64))) as u64) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rbp.wrapping_sub(0x69u64), (xmm1) as u128); let y_addr: u64 = rdi.wrapping_add(0x78u64); xmm1 = crate::simd_ops::ru128(y_addr); ymm1_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); crate::simd_ops::wu32(rbp.wrapping_sub(0x29u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x54u64))) as u32 as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x1fu64), ((xmm0 as u64)) as u64); crate::simd_ops::wu32(rbp.wrapping_sub(0x25u64), ((rax as u32)) as u32); let y_addr: u64 = rbp.wrapping_sub(0x1u64); crate::simd_ops::wu128(y_addr, xmm1); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm1_hi); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = ((rcx).wrapping_sub(rsi)) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x67u64), rsi); rax = r13; crate::simd_ops::wu8(rsp.wrapping_add(0x20u64), ((rbx as u8)) as u8); let prod = (rax as i64 as i128).wrapping_mul((rcx) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; r9 = rbp.wrapping_sub(0x79); rdx = ((((rdx) as i64).wrapping_shr(6u32) as u64)) as u64; rcx = rbp.wrapping_add(0x67); r8 = rdx; r8 = ((r8).wrapping_shr(63u32)) as u64; r8 = ((r8).wrapping_add(rdx)) as u64; rdx = 0; fl_l = 0; fl_r = 0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 10; continue; }
            10 => { rcx = rdi; rax = rcx; rax = ((rax).wrapping_sub(rsi)) as u64; fl_l = rax; fl_r = (0x140u64) as u64; if ((((fl_l as u64)) as i64) >= (((fl_r as u64)) as i64)) { bb = 9; continue; } bb = 5; continue; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_23904<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r9); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rsi = r8; rdi = rcx; rbx = 0; fl_l = 0; fl_r = 0; rcx = 0x6348; rax = ((crate::exe::tls_pointer() as u64)) as u64; rdx = (crate::simd_ops::ru64(rax)) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(rdx))) as u64; r9 = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rdx = (crate::simd_ops::ru64(r9.wrapping_add(P1))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 1; continue; } r9 = 0; fl_l = 0; fl_r = 0; r8 = 0; fl_l = 0; fl_r = 0; rdx = ((rbx.wrapping_add(P1)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk_caught(P2, rax, rcx, rdx, r8, r9, rsp); rdx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 2; continue; } rax = crate::simd_ops::call_stk_caught((crate::simd_ops::ru64(P3)) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = rax; rdx = 0xdeadd00d; rax = crate::simd_ops::call_stk_caught((crate::simd_ops::ru64(P4)) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 3; continue; }
            1 => { fl_l = (crate::simd_ops::lk_add_u32(r9.wrapping_add(P0), (1) as u32)) as u32 as u64; rax = (crate::simd_ops::ru64(rdx)) as u64; crate::simd_ops::wu64(r9.wrapping_add(P1), rax); bb = 2; continue; }
            2 => { rbx = rdx; bb = 3; continue; }
            3 => { crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rbx); r8 = P5; rdx = rsi; rcx = rbx; rax = crate::simd_ops::call_stk_caught(P6, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rdi, rax); rax = rdi; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_23908<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r9); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rsi = r8; rdi = rcx; rbx = 0; fl_l = 0; fl_r = 0; rcx = 0x6348; rax = ((crate::exe::tls_pointer() as u64)) as u64; rdx = (crate::simd_ops::ru64(rax)) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(rdx))) as u64; r9 = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rdx = (crate::simd_ops::ru64(r9.wrapping_add(P1))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 1; continue; } r9 = 0; fl_l = 0; fl_r = 0; r8 = 0; fl_l = 0; fl_r = 0; rdx = ((rbx.wrapping_add(P2)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk_caught(P3, rax, rcx, rdx, r8, r9, rsp); rdx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 2; continue; } rax = crate::simd_ops::call_stk_caught((crate::simd_ops::ru64(P4)) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = rax; rdx = 0xdeadd00d; rax = crate::simd_ops::call_stk_caught((crate::simd_ops::ru64(P5)) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 3; continue; }
            1 => { fl_l = (crate::simd_ops::lk_add_u32(r9.wrapping_add(P6), (1) as u32)) as u32 as u64; rax = (crate::simd_ops::ru64(rdx)) as u64; crate::simd_ops::wu64(r9.wrapping_add(P1), rax); bb = 2; continue; }
            2 => { rbx = rdx; bb = 3; continue; }
            3 => { crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rbx); r8 = P7; rdx = rsi; rcx = rbx; rax = crate::simd_ops::call_stk_caught(P8, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rdi, rax); rax = rdi; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_23920<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64, const P9: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rax = ((crate::exe::tls_pointer() as u64)) as u64; rdx = 0x6348; rcx = (crate::simd_ops::ru64(rax)) as u64; rax = (crate::simd_ops::ru64(rdx.wrapping_add(rcx))) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64; rbx = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 1; continue; } fl_l = ((rbx as u8)) as u8 as u64; fl_r = (0xfu8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 1; continue; } fl_l = (crate::simd_ops::lk_add_u32(rcx.wrapping_add(P2), (1) as u32)) as u32 as u64; rax = (crate::simd_ops::ru64(rbx)) as u64; crate::simd_ops::wu64(rcx.wrapping_add(P1), rax); bb = 2; continue; }
            1 => { rdx = 0xc0; r8 = 0x10; rcx = rax; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rbx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 2; continue; } rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P4)) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = rax; rdx = 0xdeadd00d; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P5)) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 2; continue; }
            2 => { rdx = 0; fl_l = 0; fl_r = 0; r8 = 0xc0; rcx = rbx; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); rcx = rbx; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); r10 = P8; rax = rbx; crate::simd_ops::wu64(rbx, r10); crate::simd_ops::wu64(rbx.wrapping_add(P9), (0x0u64) as u64); rsp = ((rsp).wrapping_add(0x30u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_23921<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64, const P9: u64, const P10: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = r9; rdx = ((r9.wrapping_add(r9.wrapping_mul(2))) as u64) as u64; rdx = ((rdx).wrapping_shl(6u32)) as u64; rax = ((rdx.wrapping_add(P0)) as u64) as u64; rcx = ((rax.wrapping_add(P1)) as u64) as u64; fl_l = rdx; fl_r = rdx; rcx = (if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { rax } else { rcx }) as u64; rdx = 0x10; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rbp = 0; fl_l = 0; fl_r = 0; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 1; continue; } rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P3)) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = rax; rdx = 0xdeadd00d; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P4)) as u64, rax, rcx, rdx, r8, r9, rsp); rax = ((rbp as u32)) as u32 as u64; bb = 1; continue; }
            1 => { rsi = ((rax.wrapping_add(P0)) as u64) as u64; crate::simd_ops::wu64(rax, (0xc0u64) as u64); crate::simd_ops::wu32(rax.wrapping_add(P5), ((rdi as u32)) as u32); crate::simd_ops::wu32(rax.wrapping_add(P6), (0x8u32) as u32); fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } rbx = rsi; r14 = P7; bb = 2; continue; }
            2 => { rdx = 0; fl_l = 0; fl_r = 0; r8 = 0xc0; rcx = rbx; rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); rcx = rbx; rax = crate::simd_ops::call_stk(P9, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx, r14); crate::simd_ops::wu64(rbx.wrapping_add(P10), rbp); rbx = ((rbx).wrapping_add(0xc0u64)) as u64; fl_l = rdi; fl_r = (0x1u64) as u64; rdi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 3; continue; }
            3 => { rax = rsi; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_23922<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
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
    r9 = (crate::simd_ops::ru32(P3)) as u32 as u64;
    rdx = rbx;
    r8 = (crate::simd_ops::ru64(P4)) as u64;
    rcx = rdi;
    crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), (0x1u32) as u32);
    crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), (0x1u32) as u32);
    rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp);
    rcx = rdi;
    rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp);
    rcx = rdi;
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
pub unsafe extern "win64" fn tpl_23923<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
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
    r9 = (crate::simd_ops::ru32(P3)) as u32 as u64;
    rdx = rbx;
    r8 = (crate::simd_ops::ru64(P4)) as u64;
    rcx = rdi;
    crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), (0x1u32) as u32);
    crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), (0x1u32) as u32);
    rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp);
    rcx = rdi;
    rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp);
    rcx = rdi;
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
pub unsafe extern "win64" fn tpl_23924<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = (crate::simd_ops::ru64(rdx)) as u64; rbx = rdx; crate::simd_ops::wu32(rsp.wrapping_add(0x40u64), (0x0u32) as u32); fl_l = ((r8 as u8)) as u8 as u64; fl_r = ((r8 as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 1; continue; } rax = (crate::simd_ops::ru64(P0)) as u64; r8 = rsp.wrapping_add(0x30); rdx = P1; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); rcx = rdi; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rdi = (crate::simd_ops::ru64(rbx)) as u64; rax = P3; bb = 2; continue; }
            1 => { rdx = (crate::simd_ops::ru64(P0)) as u64; rcx = P4; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); bb = 2; continue; }
            2 => { r8 = rsp.wrapping_add(0x40); rdx = rax; rcx = rdi; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); rdx = (crate::simd_ops::ru64(P7)) as u64; rcx = P4; rbx = (crate::simd_ops::ru64(rbx)) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x40u64), (0x1u32) as u32); rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); r8 = rsp.wrapping_add(0x40); rdx = rax; rcx = rbx; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rax = 0; fl_l = 0; fl_r = 0; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_23926<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64, const P9: u64, const P10: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdx = (crate::simd_ops::ru64(rcx)) as u64; rbx = rcx; rax = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; r8 = (((rax) as i64).wrapping_mul((0x78u64) as i64) as u64) as u64; r8 = ((r8).wrapping_add(rdx)) as u64; fl_l = rdx; fl_r = r8; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 1; continue; } rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 1; continue; }
            1 => { r9 = (crate::simd_ops::ru64(rbx)) as u64; fl_l = r9; fl_r = r9; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } rcx = ((r9.wrapping_add(P2)) as u64) as u64; rax = r9; rcx = (((rcx) & (0xfffffffffe000000u64))) as u64; rax = ((rax).wrapping_sub(rcx)) as u64; rax = ((rax).wrapping_shr(16u32)) as u64; rdx = ((rcx.wrapping_add(P3)) as u64) as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(P4))) as u32 as u64; rdx = ((rdx).wrapping_sub(rax)) as u64; rax = ((crate::exe::teb_self() as u64)) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(rcx.wrapping_add(P5))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(P6))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 2; continue; } rax = (crate::simd_ops::ru64(rdx.wrapping_add(P7))) as u64; crate::simd_ops::wu64(r9, rax); fl_l = (crate::simd_ops::lk_add_u32(rdx.wrapping_add(P8), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; crate::simd_ops::wu64(rdx.wrapping_add(P7), r9); if (((fl_l as u32)) != 0) { bb = 5; continue; } rcx = rdx; rax = crate::simd_ops::call_stk(P9, rax, rcx, rdx, r8, r9, rsp); rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rbx, rax); crate::simd_ops::wu64(rbx.wrapping_add(P0), rax); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            2 => { r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 4; continue; }
            3 => { r8 = 0; fl_l = 0; fl_r = 0; bb = 4; continue; }
            4 => { rax = crate::simd_ops::call_stk(P10, rax, rcx, rdx, r8, r9, rsp); bb = 5; continue; }
            5 => { rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rbx, rax); crate::simd_ops::wu64(rbx.wrapping_add(P0), rax); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_23929<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        core::sync::atomic::fence(core::sync::atomic::Ordering::SeqCst);
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rdx; rbp = rcx; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rax = ((rax as u32)) as u32 as u64; rdx = P1; rdx = (crate::simd_ops::ru64(rdx.wrapping_add(rax.wrapping_mul(8)))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 1; continue; } fl_l = (crate::simd_ops::ru32(rdx)) as u32 as u64; fl_r = 0x2; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 1; continue; } fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } rcx = rbx; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rcx = rbx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(P3, rax, rcx, rdx, r8, r9, rsp); }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rsi); rsi = (crate::simd_ops::ru64(rbp.wrapping_add(0x168u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rdi); crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), r14); rcx = rsi.wrapping_add(0x18); rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P4)) as u64, rax, rcx, rdx, r8, r9, rsp); rdx = (crate::simd_ops::ru64(rsi.wrapping_add(0x20u64))) as u64; r14 = (crate::simd_ops::ru64(rsi.wrapping_add(0x28u64))) as u64; r8 = r14.wrapping_add(0x20); fl_l = r8; fl_r = (crate::simd_ops::ru64(rsi.wrapping_add(0x10u64))) as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 5; continue; } fl_l = r14; fl_r = rdx; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 3; continue; } bb = 2; continue; }
            2 => { fl_l = r8; fl_r = rdx; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 3; continue; } rcx = (crate::simd_ops::ru32(rsi.wrapping_add(0x30u64))) as u32 as u64; rax = (crate::simd_ops::ru32(rsi.wrapping_add(0x34u64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 3; continue; } rdx = (crate::simd_ops::ru64(rsi.wrapping_add(0x20u64))) as u64; fl_l = r14; fl_r = rdx; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 2; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x10u64))) as u64; rax = ((rax).wrapping_sub(0x8u64)) as u64; fl_l = r14; fl_r = rax; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 4; continue; } rax = (0xffffffffffu64) as u64; crate::simd_ops::wu64(r14, rax); bb = 4; continue; }
            4 => { r14 = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; r8 = r14.wrapping_add(0x20); bb = 5; continue; }
            5 => { fl_l = r14; fl_r = rdx; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 7; continue; } bb = 6; continue; }
            6 => { fl_l = r8; fl_r = rdx; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 7; continue; } rcx = (crate::simd_ops::ru32(rsi.wrapping_add(0x30u64))) as u32 as u64; rax = (crate::simd_ops::ru32(rsi.wrapping_add(0x34u64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 7; continue; } rdx = (crate::simd_ops::ru64(rsi.wrapping_add(0x20u64))) as u64; fl_l = r14; fl_r = rdx; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 6; continue; } bb = 7; continue; }
            7 => { rax = (0x69fafafafaffu64) as u64; rcx = rsi.wrapping_add(0x18); crate::simd_ops::wu64(r14, rax); let tmp_xchg: u64 = crate::simd_ops::atomic_xchg_u64(rsi.wrapping_add(0x28u64), r8); r8 = (tmp_xchg) as u64; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P5)) as u64, rax, rcx, rdx, r8, r9, rsp); rax = P6; crate::simd_ops::wu32(r14.wrapping_add(0x8u64), (0x0u32) as u32); crate::simd_ops::wu64(r14.wrapping_add(0xcu64), rax); crate::simd_ops::wu64(r14.wrapping_add(0x14u64), rbx); rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x168u64))) as u64; crate::simd_ops::wu64(r14, (0x14u64) as u64); let _ = crate::simd_ops::atomic_add_u32(rax.wrapping_add(0x30u64), (1) as u32); fl_l = (crate::simd_ops::ru8(rbp.wrapping_add(0x198u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 8; continue; } rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x1a0u64))) as u64; r8 = 0; fl_l = 0; fl_r = 0; rdx = ((r8.wrapping_add(0x1u64)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P7)) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 8; continue; }
            8 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            9 => { crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rsi); rsi = (crate::simd_ops::ru64(rbp.wrapping_add(0x168u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rdi); crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), r14); rcx = rsi.wrapping_add(0x18); rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P4)) as u64, rax, rcx, rdx, r8, r9, rsp); rdx = (crate::simd_ops::ru64(rsi.wrapping_add(0x20u64))) as u64; r14 = (crate::simd_ops::ru64(rsi.wrapping_add(0x28u64))) as u64; r8 = r14.wrapping_add(0x20); fl_l = r8; fl_r = (crate::simd_ops::ru64(rsi.wrapping_add(0x10u64))) as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 5; continue; } fl_l = r14; fl_r = rdx; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 3; continue; } bb = 10; continue; }
            10 => { fl_l = r8; fl_r = rdx; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 3; continue; } rcx = (crate::simd_ops::ru32(rsi.wrapping_add(0x30u64))) as u32 as u64; rax = (crate::simd_ops::ru32(rsi.wrapping_add(0x34u64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 3; continue; } rdx = (crate::simd_ops::ru64(rsi.wrapping_add(0x20u64))) as u64; fl_l = r14; fl_r = rdx; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 2; continue; } bb = 11; continue; }
            11 => { rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x10u64))) as u64; rax = ((rax).wrapping_sub(0x8u64)) as u64; fl_l = r14; fl_r = rax; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 4; continue; } rax = (0xffffffffffu64) as u64; crate::simd_ops::wu64(r14, rax); bb = 12; continue; }
            12 => { r14 = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; r8 = r14.wrapping_add(0x20); bb = 13; continue; }
            13 => { fl_l = r14; fl_r = rdx; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 7; continue; } bb = 14; continue; }
            14 => { fl_l = r8; fl_r = rdx; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 7; continue; } rcx = (crate::simd_ops::ru32(rsi.wrapping_add(0x30u64))) as u32 as u64; rax = (crate::simd_ops::ru32(rsi.wrapping_add(0x34u64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 7; continue; } rdx = (crate::simd_ops::ru64(rsi.wrapping_add(0x20u64))) as u64; fl_l = r14; fl_r = rdx; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 6; continue; } bb = 15; continue; }
            15 => { rax = (0x69fafafafaffu64) as u64; rcx = rsi.wrapping_add(0x18); crate::simd_ops::wu64(r14, rax); let tmp_xchg: u64 = crate::simd_ops::atomic_xchg_u64(rsi.wrapping_add(0x28u64), r8); r8 = (tmp_xchg) as u64; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P5)) as u64, rax, rcx, rdx, r8, r9, rsp); rax = P6; crate::simd_ops::wu32(r14.wrapping_add(0x8u64), (0x0u32) as u32); crate::simd_ops::wu64(r14.wrapping_add(0xcu64), rax); crate::simd_ops::wu64(r14.wrapping_add(0x14u64), rbx); rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x168u64))) as u64; crate::simd_ops::wu64(r14, (0x14u64) as u64); let _ = crate::simd_ops::atomic_add_u32(rax.wrapping_add(0x30u64), (1) as u32); fl_l = (crate::simd_ops::ru8(rbp.wrapping_add(0x198u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 8; continue; } rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x1a0u64))) as u64; r8 = 0; fl_l = 0; fl_r = 0; rdx = ((r8.wrapping_add(0x1u64)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P7)) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 16; continue; }
            16 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_23930<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = (crate::simd_ops::ru64(rcx)) as u64;
    fl_l = rbx; fl_r = rbx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rcx = rbx; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rcx = rbx; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(P1, rax, rcx, rdx, r8, r9, rsp); }
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_23933<const P0: u64, const P1: u64, const P2: u64, const P3: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        core::sync::atomic::fence(core::sync::atomic::Ordering::SeqCst);
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rsi = rcx; rdi = 0xffffffff; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 1; continue; } rax = ((rdi as u32)) as u32 as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rbx.wrapping_add(P1), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 1; continue; } rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = rbx; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(rax)) as u64, rax, rcx, rdx, r8, r9, rsp); rax = ((rdi as u32)) as u32 as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rbx.wrapping_add(P2), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 1; continue; } rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = rbx; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 1; continue; }
            1 => { rbx = (crate::simd_ops::ru64(rsi.wrapping_add(P3))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } rax = ((rdi as u32)) as u32 as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rbx.wrapping_add(P1), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 2; continue; } rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = rbx; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(rax)) as u64, rax, rcx, rdx, r8, r9, rsp); let xa_src: u32 = (rdi as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rbx.wrapping_add(P2), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rdi = (xa_old) as u32 as u64; fl_l = ((rdi as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 2; continue; } rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = rbx; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 2; continue; }
            2 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_23943<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rax = P0;
    rbx = rcx;
    crate::simd_ops::wu64(rcx, rax);
    fl_l = ((rdx as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64;
    if !(((((fl_l as u8)) & ((fl_r as u8))) == 0)) { rdx = 0x118; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); }
    rax = rbx;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_23945<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64, const P9: u64, const P10: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; r8 = (crate::simd_ops::ru64(P0)) as u64; rdi = rdx; rdx = rcx; crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), (0x1u32) as u32); rbx = rcx; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), (0x3u32) as u32); rcx = rdi; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rdi.wrapping_add(P2))) as u64; rdx = rbx; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(rax.wrapping_add(P3))) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rdi.wrapping_add(P2))) as u64; rdx = ((rbx.wrapping_add(P4)) as u64) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(rax.wrapping_add(P3))) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rdi.wrapping_add(P2))) as u64; rdx = ((rbx.wrapping_add(P5)) as u64) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(rax.wrapping_add(P3))) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rdi.wrapping_add(P2))) as u64; rdx = ((rbx.wrapping_add(P6)) as u64) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(rax.wrapping_add(P7))) as u64, rax, rcx, rdx, r8, r9, rsp); rdx = (crate::simd_ops::ru32(rdi.wrapping_add(P5))) as u32 as u64; rax = (crate::simd_ops::ru64(rdi)) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rcx = ((rdx.wrapping_add(rdx.wrapping_mul(2))) as u64) as u64; rcx = ((rcx).wrapping_add(rcx)) as u64; rcx = (crate::simd_ops::ru32(rax.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(P8))) as u32 as u64; rax = ((rdx.wrapping_add(P9)) as u32) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(P5), ((rax as u32)) as u32); rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rdi.wrapping_add(P10), ((rcx as u32)) as u32); rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_23957<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r9); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = r8; rdx = 0x6348; rax = ((crate::exe::tls_pointer() as u64)) as u64; rcx = (crate::simd_ops::ru64(rax)) as u64; rax = (crate::simd_ops::ru64(rdx.wrapping_add(rcx))) as u64; rdx = (crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64; rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P1))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 1; continue; } r9 = 0; fl_l = 0; fl_r = 0; r8 = 0; fl_l = 0; fl_r = 0; rdx = ((rcx.wrapping_add(P1)) as u32) as u32 as u64; rcx = rax; rax = crate::simd_ops::call_stk_caught(P2, rax, rcx, rdx, r8, r9, rsp); rcx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 2; continue; } rax = crate::simd_ops::call_stk_caught((crate::simd_ops::ru64(P3)) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = rax; rdx = 0xdeadd00d; rax = crate::simd_ops::call_stk_caught((crate::simd_ops::ru64(P4)) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = 0; fl_l = 0; fl_r = 0; bb = 2; continue; }
            1 => { fl_l = (crate::simd_ops::lk_add_u32(rdx.wrapping_add(P0), (1) as u32)) as u32 as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; crate::simd_ops::wu64(rdx.wrapping_add(P1), rax); bb = 2; continue; }
            2 => { crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rcx); r8 = P5; rdx = rbx; rax = crate::simd_ops::call_stk_caught(P6, rax, rcx, rdx, r8, r9, rsp); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_23964<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rdx;
    rdi = rcx;
    rax = ((crate::simd_ops::ru8(rdx)) as u32) as u32 as u64;
    crate::simd_ops::wu8(rcx, ((rax as u8)) as u8);
    rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P0))) as u32) as u32 as u64;
    crate::simd_ops::wu8(rcx.wrapping_add(P0), ((rax as u8)) as u8);
    rcx = ((rcx).wrapping_add(0x4u64)) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rcx);
    crate::simd_ops::lk_and_u32(rcx, (0xff000004u32) as u32);
    crate::simd_ops::lk_or_u32(rcx, (0x4u32) as u32);
    crate::simd_ops::wu8(rcx.wrapping_add(P1), (0x4u8) as u8);
    crate::simd_ops::wu32(rcx.wrapping_add(P2), (0x0u32) as u32);
    crate::simd_ops::wu8(rcx.wrapping_add(P3), (0x0u8) as u8);
    rdx = ((rdx).wrapping_add(0x4u64)) as u64;
    rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp);
    rax = (crate::simd_ops::ru32(rbx.wrapping_add(P5))) as u32 as u64;
    crate::simd_ops::wu32(rdi.wrapping_add(P5), ((rax as u32)) as u32);
    rax = rdi;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_23966<const P0: u64, const P1: u64, const P2: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rdi = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    rbx = (crate::simd_ops::ru64(rcx)) as u64;
    fl_l = rbx; fl_r = rdi;
    if (((fl_l as u64)) == ((fl_r as u64))) { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    loop { rcx = ((rbx.wrapping_add(P1)) as u64) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rbx = ((rbx).wrapping_add(0x68u64)) as u64; fl_l = rbx; fl_r = rdi; if !((((fl_l as u64)) != ((fl_r as u64)))) { break; } }
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_23970<const P0: u64, const P1: u64, const P2: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rcx;
    rcx = ((rcx).wrapping_add(0x98u64)) as u64;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rax = P1;
    crate::simd_ops::wu64(rbx, rax);
    rcx = rbx;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return crate::simd_ops::tail_stk(P2, rax, rcx, rdx, r8, r9, rsp);
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_23977<const P0: u64, const P1: u64, const P2: u64, const P3: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = ((rdx as u32)) as u32 as u64;
    rdi = rcx;
    rcx = ((rcx).wrapping_add(0x98u64)) as u64;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rax = P1;
    crate::simd_ops::wu64(rdi, rax);
    rcx = rdi;
    rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp);
    fl_l = ((rbx as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64;
    if !(((((fl_l as u8)) & ((fl_r as u8))) == 0)) { rdx = 0xa8; rcx = rdi; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); }
    rax = rdi;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_23984<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64, const P9: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; r8 = rdx; rdi = rcx; rbx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; fl_l = rdx; fl_r = rbx; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 1; continue; } rcx = ((rbx.wrapping_add(rdx)) as u64) as u64; rdx = 0; fl_l = 0; fl_r = 0; rax = ((rcx.wrapping_add(P1)) as u64) as u64; let d = (rbx) as u128; let n = ((rdx as u128) << 64) | (rax as u128); if d != 0 { rax = (n / d) as u64; rdx = (n % d) as u64; } rbx = ((rcx.wrapping_add(P1)) as u64) as u64; fl_l = rbx; fl_r = rdx; rbx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rdi.wrapping_add(P2), r8); rdx = rbx; rcx = ((rdi.wrapping_add(P3)) as u64) as u64; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rdx = (crate::simd_ops::ru32(rdi.wrapping_add(P5))) as u32 as u64; r8 = (crate::simd_ops::ru32(rdi.wrapping_add(P6))) as u32 as u64; r8 = ((r8).wrapping_shl(4u32)) as u64; rax = ((rdx as u32)) as u32 as u64; rax = ((rax).wrapping_shl(4u32)) as u64; fl_l = rax; fl_r = r8; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 2; continue; } rcx = ((rdi.wrapping_add(P3)) as u64) as u64; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); bb = 2; continue; }
            2 => { rcx = ((rdi.wrapping_add(P8)) as u64) as u64; rdx = rbx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(P9, rax, rcx, rdx, r8, r9, rsp); }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_23985<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64, const P9: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = rcx; r8 = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; r8 = ((r8).wrapping_add(rdx)) as u64; rbx = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; fl_l = r8; fl_r = rbx; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 1; continue; } rcx = ((rbx.wrapping_add(r8)) as u64) as u64; rdx = 0; fl_l = 0; fl_r = 0; rax = ((rcx.wrapping_add(P2)) as u64) as u64; let d = (rbx) as u128; let n = ((rdx as u128) << 64) | (rax as u128); if d != 0 { rax = (n / d) as u64; rdx = (n % d) as u64; } rbx = ((rcx.wrapping_add(P2)) as u64) as u64; fl_l = rbx; fl_r = rdx; rbx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rdi.wrapping_add(P0), r8); rdx = rbx; rcx = ((rdi.wrapping_add(P3)) as u64) as u64; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rdx = (crate::simd_ops::ru32(rdi.wrapping_add(P5))) as u32 as u64; r8 = (crate::simd_ops::ru32(rdi.wrapping_add(P6))) as u32 as u64; r8 = ((r8).wrapping_shl(4u32)) as u64; rax = ((rdx as u32)) as u32 as u64; rax = ((rax).wrapping_shl(4u32)) as u64; fl_l = rax; fl_r = r8; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 2; continue; } rcx = ((rdi.wrapping_add(P3)) as u64) as u64; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); bb = 2; continue; }
            2 => { rcx = ((rdi.wrapping_add(P8)) as u64) as u64; rdx = rbx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(P9, rax, rcx, rdx, r8, r9, rsp); }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_23986<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x30u64)) as u64;
    r8 = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64;
    rbx = rcx;
    fl_l = ((r8 as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64;
    if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { rsp = ((rsp).wrapping_add(0x30u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(P1))) as u8 as u64; fl_r = (0x0u8) as u8 as u64;
    if !((((fl_l as u8)) != ((fl_r as u8)))) { rdx = (crate::simd_ops::ru64(rcx.wrapping_add(P2))) as u64; r9 = 0x10; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P3))) as u64; crate::simd_ops::wu8(rsp.wrapping_add(0x20u64), (0x0u8) as u8); rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu32(rbx.wrapping_add(P0), (0x0u32) as u32); }
    rsp = ((rsp).wrapping_add(0x30u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_23987<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x30u64)) as u64;
    rax = ((crate::simd_ops::ru8(rcx.wrapping_add(P0))) as u32) as u32 as u64;
    rbx = rcx;
    r8 = (crate::simd_ops::ru32(rcx.wrapping_add(P1))) as u32 as u64;
    r9 = 0x10;
    rdx = (crate::simd_ops::ru64(rcx.wrapping_add(P2))) as u64;
    rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P3))) as u64;
    crate::simd_ops::wu8(rsp.wrapping_add(0x20u64), ((rax as u8)) as u8);
    rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp);
    crate::simd_ops::wu32(rbx.wrapping_add(P1), (0x0u32) as u32);
    rsp = ((rsp).wrapping_add(0x30u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_23991<const P0: u64, const P1: u64, const P2: u64, const P3: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 1>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rbx = ((r8.wrapping_add(P0)) as u64) as u64; rbp = r8; rbx = ((((rbx) as i64).wrapping_shr(1u32) as u64)) as u64; r10 = rdx; rsi = rcx; rdi = rdx; r11 = rdx; fl_l = rdx; fl_r = rbx; if ((((fl_l as u64)) as i64) >= (((fl_r as u64)) as i64)) { bb = 2; continue; } bb = 1; continue; }
            1 => { rdx = (crate::simd_ops::ru64(rcx)) as u64; r8 = ((r11.wrapping_add(r11)) as u64) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(r8.wrapping_mul(4)).wrapping_add(P1))) as u32 as u64; fl_l = (crate::simd_ops::ru32(rdx.wrapping_add(r8.wrapping_mul(4)).wrapping_add(P2))) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; rax = (if (((fl_l as u32)) < ((fl_r as u32))) { 0xffffffffffffffffu64 } else { 0u64 }) as u64; r11 = ((rax.wrapping_add(P3)) as u64) as u64; r11 = ((r11).wrapping_add(r8)) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(r11.wrapping_mul(4)))) as u32 as u64; crate::simd_ops::wu32(rdx.wrapping_add(r10.wrapping_mul(4)), ((rax as u32)) as u32); r10 = r11; fl_l = r11; fl_r = rbx; if ((((fl_l as u64)) as i64) < (((fl_r as u64)) as i64)) { bb = 1; continue; } bb = 2; continue; }
            2 => { fl_l = r11; fl_r = rbx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } fl_l = ((rbp as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 3; continue; } rcx = (crate::simd_ops::ru64(rcx)) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(rbp.wrapping_mul(4)).wrapping_sub(0x4u64))) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(r10.wrapping_mul(4)), ((rax as u32)) as u32); r10 = rbp.wrapping_sub(0x1); bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rsi)) as u64; rcx = ((r10.wrapping_add(P0)) as u64) as u64; rcx = ((((rcx) as i64).wrapping_shr(1u32) as u64)) as u64; fl_l = rdi; fl_r = r10; if ((((fl_l as u64)) as i64) >= (((fl_r as u64)) as i64)) { bb = 5; continue; } r8 = (crate::simd_ops::ru32(r9)) as u32 as u64; bb = 4; continue; }
            4 => { rdx = (crate::simd_ops::ru32(rax.wrapping_add(rcx.wrapping_mul(4)))) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 5; continue; } crate::simd_ops::wu32(rax.wrapping_add(r10.wrapping_mul(4)), ((rdx as u32)) as u32); r10 = rcx; rcx = ((rcx).wrapping_sub(1)) as u64; rcx = ((((rcx) as i64).wrapping_shr(1u32) as u64)) as u64; fl_l = rdi; fl_r = r10; if ((((fl_l as u64)) as i64) < (((fl_r as u64)) as i64)) { bb = 4; continue; } bb = 5; continue; }
            5 => { rcx = (crate::simd_ops::ru32(r9)) as u32 as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x10u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x18u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; crate::simd_ops::wu32(rax.wrapping_add(r10.wrapping_mul(4)), ((rcx as u32)) as u32); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_23992(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 0>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdi); r9 = (crate::simd_ops::ru64(r8)) as u64; rbx = r8; rax = (crate::simd_ops::ru64(rcx)) as u64; r8 = r9; r8 = ((r8).wrapping_sub(rax)) as u64; r11 = rdx; r8 = ((((r8) as i64).wrapping_shr(2u32) as u64)) as u64; rdi = rcx; fl_l = r8; fl_r = (0x28u64) as u64; if ((((fl_l as u64)) as i64) <= (((fl_r as u64)) as i64)) { bb = 8; continue; } rdx = (crate::simd_ops::ru32(rax)) as u32 as u64; r8 = ((r8).wrapping_add(1)) as u64; r8 = ((((r8) as i64).wrapping_shr(3u32) as u64)) as u64; r10 = ((r8.wrapping_mul(4)) as u64) as u64; rcx = (crate::simd_ops::ru32(r10.wrapping_add(rax))) as u32 as u64; r9 = ((r8.wrapping_mul(8)) as u64) as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 1; continue; } crate::simd_ops::wu32(r10.wrapping_add(rax), ((rdx as u32)) as u32); crate::simd_ops::wu32(rax, ((rcx as u32)) as u32); bb = 1; continue; }
            1 => { rcx = (crate::simd_ops::ru32(r9.wrapping_add(rax))) as u32 as u64; rdx = (crate::simd_ops::ru32(r10.wrapping_add(rax))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 2; continue; } crate::simd_ops::wu32(r9.wrapping_add(rax), ((rdx as u32)) as u32); crate::simd_ops::wu32(r10.wrapping_add(rax), ((rcx as u32)) as u32); rdx = (crate::simd_ops::ru32(rax)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 2; continue; } crate::simd_ops::wu32(r10.wrapping_add(rax), ((rdx as u32)) as u32); crate::simd_ops::wu32(rax, ((rcx as u32)) as u32); bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(r11)) as u64; rcx = rax; rcx = ((rcx).wrapping_sub(r10)) as u64; rdx = (crate::simd_ops::ru32(rax)) as u32 as u64; r8 = (crate::simd_ops::ru32(rcx)) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 3; continue; } crate::simd_ops::wu32(rax, ((r8 as u32)) as u32); crate::simd_ops::wu32(rcx, ((rdx as u32)) as u32); bb = 3; continue; }
            3 => { rdx = (crate::simd_ops::ru32(rax.wrapping_add(r10))) as u32 as u64; r8 = (crate::simd_ops::ru32(rax)) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 4; continue; } crate::simd_ops::wu32(rax.wrapping_add(r10), ((r8 as u32)) as u32); crate::simd_ops::wu32(rax, ((rdx as u32)) as u32); r8 = (crate::simd_ops::ru32(rcx)) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 4; continue; } crate::simd_ops::wu32(rax, ((r8 as u32)) as u32); crate::simd_ops::wu32(rcx, ((rdx as u32)) as u32); bb = 4; continue; }
            4 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; rax = rcx; rdx = rcx; rax = ((rax).wrapping_sub(r10)) as u64; rdx = ((rdx).wrapping_sub(r9)) as u64; r8 = (crate::simd_ops::ru32(rax)) as u32 as u64; r9 = (crate::simd_ops::ru32(rdx)) as u32 as u64; fl_l = ((r8 as u32)) as u32 as u64; fl_r = ((r9 as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 5; continue; } crate::simd_ops::wu32(rax, ((r9 as u32)) as u32); crate::simd_ops::wu32(rdx, ((r8 as u32)) as u32); bb = 5; continue; }
            5 => { r8 = (crate::simd_ops::ru32(rcx)) as u32 as u64; r9 = (crate::simd_ops::ru32(rax)) as u32 as u64; fl_l = ((r8 as u32)) as u32 as u64; fl_r = ((r9 as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 6; continue; } crate::simd_ops::wu32(rcx, ((r9 as u32)) as u32); crate::simd_ops::wu32(rax, ((r8 as u32)) as u32); rcx = (crate::simd_ops::ru32(rdx)) as u32 as u64; fl_l = ((r8 as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 6; continue; } crate::simd_ops::wu32(rax, ((rcx as u32)) as u32); crate::simd_ops::wu32(rdx, ((r8 as u32)) as u32); bb = 6; continue; }
            6 => { r8 = (crate::simd_ops::ru64(rdi)) as u64; rax = (crate::simd_ops::ru64(r11)) as u64; r8 = ((r8).wrapping_add(r10)) as u64; rcx = (crate::simd_ops::ru64(rbx)) as u64; rcx = ((rcx).wrapping_sub(r10)) as u64; r9 = (crate::simd_ops::ru32(r8)) as u32 as u64; rdx = (crate::simd_ops::ru32(rax)) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((r9 as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 7; continue; } crate::simd_ops::wu32(rax, ((r9 as u32)) as u32); crate::simd_ops::wu32(r8, ((rdx as u32)) as u32); bb = 7; continue; }
            7 => { rdx = (crate::simd_ops::ru32(rcx)) as u32 as u64; r9 = (crate::simd_ops::ru32(rax)) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((r9 as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 10; continue; } crate::simd_ops::wu32(rcx, ((r9 as u32)) as u32); crate::simd_ops::wu32(rax, ((rdx as u32)) as u32); rcx = (crate::simd_ops::ru32(r8)) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 10; continue; } crate::simd_ops::wu32(rax, ((rcx as u32)) as u32); crate::simd_ops::wu32(r8, ((rdx as u32)) as u32); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x8u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x10u64))) as u64; return rax; }
            8 => { rcx = (crate::simd_ops::ru64(rdx)) as u64; r8 = (crate::simd_ops::ru32(rax)) as u32 as u64; rdx = (crate::simd_ops::ru32(rcx)) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 9; continue; } crate::simd_ops::wu32(rcx, ((r8 as u32)) as u32); crate::simd_ops::wu32(rax, ((rdx as u32)) as u32); bb = 9; continue; }
            9 => { rdx = (crate::simd_ops::ru32(r9)) as u32 as u64; r8 = (crate::simd_ops::ru32(rcx)) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 10; continue; } crate::simd_ops::wu32(r9, ((r8 as u32)) as u32); crate::simd_ops::wu32(rcx, ((rdx as u32)) as u32); r8 = (crate::simd_ops::ru32(rax)) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 10; continue; } crate::simd_ops::wu32(rcx, ((r8 as u32)) as u32); crate::simd_ops::wu32(rax, ((rdx as u32)) as u32); bb = 10; continue; }
            10 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x8u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x10u64))) as u64; return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_23993<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = (((crate::simd_ops::ru32(rcx.wrapping_add(P0))) as i32 as i64) as u64) as u64;
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(rax.wrapping_mul(8)).wrapping_add(P1))) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_23994<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64, const P9: u64, const P10: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        core::sync::atomic::fence(core::sync::atomic::Ordering::SeqCst);
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = (crate::simd_ops::ru64(P0)) as u64; r12 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(P0, r12); rbp = ((r12.wrapping_sub(0x1u64)) as u32) as u32 as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } r13 = (crate::simd_ops::ru64(P1)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm((crate::simd_ops::ru64(P2)) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = ((rax as u32)) as u32 as u64; rcx = (crate::simd_ops::ru32(r13.wrapping_add(0x10u64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 1; continue; } rcx = r13.wrapping_add(0x8); (rax, xmm0) = crate::simd_ops::call_stk_xmm((crate::simd_ops::ru64(P3)) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu32(r13.wrapping_add(0x10u64), ((rbx as u32)) as u32); bb = 1; continue; }
            1 => { crate::simd_ops::lk_add_u32(r13.wrapping_add(0x14u64), (1) as u32); rax = ((r12 as u32)) as u32 as u64; rdx = (crate::simd_ops::ru32(r13.wrapping_add(0x20u64))) as u32 as u64; r15 = (crate::simd_ops::ru64(r13.wrapping_add(0x18u64))) as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 5; continue; } rcx = r12; bb = 2; continue; }
            2 => { r14 = (((rcx) as i64).wrapping_mul((0x68u64) as i64) as u64) as u64; rbx = (crate::simd_ops::ru64(r14.wrapping_add(r15))) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 3; continue; } rax = (((rax as u32)).wrapping_add(1)) as u32 as u64; rcx = ((((rax as u32)) as i32 as i64) as u64) as u64; fl_l = rcx; fl_r = rdx; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 2; continue; } bb = 5; continue; }
            3 => { crate::simd_ops::wu8(r14.wrapping_add(r15).wrapping_add(0x38u64), (0x0u8) as u8); rax = (crate::simd_ops::ru32(P4)) as u32 as u64; crate::simd_ops::wu32(r14.wrapping_add(r15).wrapping_add(0x64u64), ((rax as u32)) as u32); fl_l = (crate::simd_ops::ru8(r14.wrapping_add(r15).wrapping_add(0x39u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 5; continue; } fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 4; continue; }
            4 => { crate::simd_ops::wu64(r14.wrapping_add(r15), r12); rdx = (crate::simd_ops::ru64(r13.wrapping_add(0x18u64))) as u64; rax = (crate::simd_ops::ru32(r13.wrapping_add(0x20u64))) as u32 as u64; rdi = (((rax) as i64).wrapping_mul((0x68u64) as i64) as u64) as u64; rdi = ((rdi).wrapping_add(rdx)) as u64; rbx = ((rdx.wrapping_add(r14)) as u64) as u64; rax = (crate::simd_ops::ru64(rdi.wrapping_sub(0x68u64))) as u64; crate::simd_ops::wu64(rbx, rax); let y_addr: u64 = rdi.wrapping_sub(0x60u64); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rbx.wrapping_add(0x8u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); xmm1 = (crate::simd_ops::ru128(rdi.wrapping_sub(0x40u64))) as u128; crate::simd_ops::wu128(rbx.wrapping_add(0x28u64), (xmm1) as u128); rax = ((crate::simd_ops::ru8(rdi.wrapping_sub(0x30u64))) as u32) as u32 as u64; crate::simd_ops::wu8(rbx.wrapping_add(0x38u64), ((rax as u8)) as u8); rax = ((crate::simd_ops::ru8(rdi.wrapping_sub(0x2fu64))) as u32) as u32 as u64; crate::simd_ops::wu8(rbx.wrapping_add(0x39u64), ((rax as u8)) as u8); rdx = rdi.wrapping_sub(0x2c); rcx = rbx.wrapping_add(0x3c); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru32(rdi.wrapping_sub(0x4u64))) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x64u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(r13.wrapping_add(0x20u64))) as u32 as u64; rdx = (((rax) as i64).wrapping_mul((0x68u64) as i64) as u64) as u64; rcx = (crate::simd_ops::ru64(r13.wrapping_add(0x18u64))) as u64; rcx = ((rcx).wrapping_add(0xffffffffffffffd4u64)) as u64; rcx = ((rcx).wrapping_add(rdx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P8, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = (crate::simd_ops::lk_sub_u32(r13.wrapping_add(0x20u64), (1) as u32)) as u32 as u64; bb = 5; continue; }
            5 => { fl_l = (crate::simd_ops::lk_add_u32(r13.wrapping_add(0x14u64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; if (((fl_l as u32)) != 0) { bb = 6; continue; } rax = ((rbp as u32)) as u32 as u64; let tmp_xchg: u32 = crate::simd_ops::atomic_xchg_u32(r13.wrapping_add(0x10u64), ((rax as u32)) as u32); rax = (tmp_xchg) as u32 as u64; rcx = r13.wrapping_add(0x8); (rax, xmm0) = crate::simd_ops::call_stk_xmm((crate::simd_ops::ru64(P9)) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 6; continue; }
            6 => { rdi = (crate::simd_ops::ru64(P10)) as u64; crate::simd_ops::wu64(P10, r12); fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } r13 = (crate::simd_ops::ru64(P1)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm((crate::simd_ops::ru64(P2)) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = ((rax as u32)) as u32 as u64; rcx = (crate::simd_ops::ru32(r13.wrapping_add(0x10u64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 7; continue; } rcx = r13.wrapping_add(0x8); (rax, xmm0) = crate::simd_ops::call_stk_xmm((crate::simd_ops::ru64(P3)) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu32(r13.wrapping_add(0x10u64), ((rbx as u32)) as u32); bb = 7; continue; }
            7 => { crate::simd_ops::lk_add_u32(r13.wrapping_add(0x14u64), (1) as u32); rax = ((r12 as u32)) as u32 as u64; rdx = (crate::simd_ops::ru32(r13.wrapping_add(0x20u64))) as u32 as u64; r15 = (crate::simd_ops::ru64(r13.wrapping_add(0x18u64))) as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 11; continue; } rcx = r12; bb = 8; continue; }
            8 => { r14 = (((rcx) as i64).wrapping_mul((0x68u64) as i64) as u64) as u64; rbx = (crate::simd_ops::ru64(r14.wrapping_add(r15))) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 9; continue; } rax = (((rax as u32)).wrapping_add(1)) as u32 as u64; rcx = ((((rax as u32)) as i32 as i64) as u64) as u64; fl_l = rcx; fl_r = rdx; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 8; continue; } bb = 11; continue; }
            9 => { crate::simd_ops::wu8(r14.wrapping_add(r15).wrapping_add(0x38u64), (0x0u8) as u8); rax = (crate::simd_ops::ru32(P4)) as u32 as u64; crate::simd_ops::wu32(r14.wrapping_add(r15).wrapping_add(0x64u64), ((rax as u32)) as u32); fl_l = (crate::simd_ops::ru8(r14.wrapping_add(r15).wrapping_add(0x39u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 11; continue; } fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 10; continue; }
            10 => { crate::simd_ops::wu64(r14.wrapping_add(r15), r12); rdx = (crate::simd_ops::ru64(r13.wrapping_add(0x18u64))) as u64; rax = (crate::simd_ops::ru32(r13.wrapping_add(0x20u64))) as u32 as u64; rdi = (((rax) as i64).wrapping_mul((0x68u64) as i64) as u64) as u64; rdi = ((rdi).wrapping_add(rdx)) as u64; rbx = ((rdx.wrapping_add(r14)) as u64) as u64; rax = (crate::simd_ops::ru64(rdi.wrapping_sub(0x68u64))) as u64; crate::simd_ops::wu64(rbx, rax); let y_addr: u64 = rdi.wrapping_sub(0x60u64); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rbx.wrapping_add(0x8u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); xmm1 = (crate::simd_ops::ru128(rdi.wrapping_sub(0x40u64))) as u128; crate::simd_ops::wu128(rbx.wrapping_add(0x28u64), (xmm1) as u128); rax = ((crate::simd_ops::ru8(rdi.wrapping_sub(0x30u64))) as u32) as u32 as u64; crate::simd_ops::wu8(rbx.wrapping_add(0x38u64), ((rax as u8)) as u8); rax = ((crate::simd_ops::ru8(rdi.wrapping_sub(0x2fu64))) as u32) as u32 as u64; crate::simd_ops::wu8(rbx.wrapping_add(0x39u64), ((rax as u8)) as u8); rdx = rdi.wrapping_sub(0x2c); rcx = rbx.wrapping_add(0x3c); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru32(rdi.wrapping_sub(0x4u64))) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x64u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(r13.wrapping_add(0x20u64))) as u32 as u64; rdx = (((rax) as i64).wrapping_mul((0x68u64) as i64) as u64) as u64; rcx = (crate::simd_ops::ru64(r13.wrapping_add(0x18u64))) as u64; rcx = ((rcx).wrapping_add(0xffffffffffffffd4u64)) as u64; rcx = ((rcx).wrapping_add(rdx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P8, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = (crate::simd_ops::lk_sub_u32(r13.wrapping_add(0x20u64), (1) as u32)) as u32 as u64; bb = 11; continue; }
            11 => { fl_l = (crate::simd_ops::lk_add_u32(r13.wrapping_add(0x14u64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; if (((fl_l as u32)) != 0) { bb = 12; continue; } let tmp_xchg: u32 = crate::simd_ops::atomic_xchg_u32(r13.wrapping_add(0x10u64), ((rbp as u32)) as u32); rbp = (tmp_xchg) as u32 as u64; rcx = r13.wrapping_add(0x8); (rax, xmm0) = crate::simd_ops::call_stk_xmm((crate::simd_ops::ru64(P9)) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 12; continue; }
            12 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_23995<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64, const P9: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        core::sync::atomic::fence(core::sync::atomic::Ordering::SeqCst);
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rsi = (crate::simd_ops::ru64(P0)) as u64; crate::simd_ops::wu64(P0, (0x0u64) as u64); fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } r15 = (crate::simd_ops::ru64(P1)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm((crate::simd_ops::ru64(P2)) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdi = ((rax as u32)) as u32 as u64; rcx = (crate::simd_ops::ru32(r15.wrapping_add(0x10u64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 1; continue; } rcx = r15.wrapping_add(0x8); (rax, xmm0) = crate::simd_ops::call_stk_xmm((crate::simd_ops::ru64(P3)) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu32(r15.wrapping_add(0x10u64), ((rdi as u32)) as u32); bb = 1; continue; }
            1 => { crate::simd_ops::lk_add_u32(r15.wrapping_add(0x14u64), (1) as u32); rax = 0; fl_l = 0; fl_r = 0; rdx = (crate::simd_ops::ru32(r15.wrapping_add(0x20u64))) as u32 as u64; rbp = (crate::simd_ops::ru64(r15.wrapping_add(0x18u64))) as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 5; continue; } rcx = 0; fl_l = 0; fl_r = 0; bb = 2; continue; }
            2 => { r14 = (((rcx) as i64).wrapping_mul((0x68u64) as i64) as u64) as u64; rdi = (crate::simd_ops::ru64(r14.wrapping_add(rbp))) as u64; fl_l = rdi; fl_r = rsi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 3; continue; } rax = (((rax as u32)).wrapping_add(1)) as u32 as u64; rcx = ((((rax as u32)) as i32 as i64) as u64) as u64; fl_l = rcx; fl_r = rdx; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 2; continue; } bb = 5; continue; }
            3 => { crate::simd_ops::wu8(r14.wrapping_add(rbp).wrapping_add(0x38u64), (0x0u8) as u8); rax = (crate::simd_ops::ru32(P4)) as u32 as u64; crate::simd_ops::wu32(r14.wrapping_add(rbp).wrapping_add(0x64u64), ((rax as u32)) as u32); fl_l = (crate::simd_ops::ru8(r14.wrapping_add(rbp).wrapping_add(0x39u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 5; continue; } fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 4; continue; }
            4 => { crate::simd_ops::wu64(r14.wrapping_add(rbp), (0x0u64) as u64); rdx = (crate::simd_ops::ru64(r15.wrapping_add(0x18u64))) as u64; rax = (crate::simd_ops::ru32(r15.wrapping_add(0x20u64))) as u32 as u64; rsi = (((rax) as i64).wrapping_mul((0x68u64) as i64) as u64) as u64; rsi = ((rsi).wrapping_add(rdx)) as u64; rdi = ((rdx.wrapping_add(r14)) as u64) as u64; rax = (crate::simd_ops::ru64(rsi.wrapping_sub(0x68u64))) as u64; crate::simd_ops::wu64(rdi, rax); let y_addr: u64 = rsi.wrapping_sub(0x60u64); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rdi.wrapping_add(0x8u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); xmm1 = (crate::simd_ops::ru128(rsi.wrapping_sub(0x40u64))) as u128; crate::simd_ops::wu128(rdi.wrapping_add(0x28u64), (xmm1) as u128); rax = ((crate::simd_ops::ru8(rsi.wrapping_sub(0x30u64))) as u32) as u32 as u64; crate::simd_ops::wu8(rdi.wrapping_add(0x38u64), ((rax as u8)) as u8); rax = ((crate::simd_ops::ru8(rsi.wrapping_sub(0x2fu64))) as u32) as u32 as u64; crate::simd_ops::wu8(rdi.wrapping_add(0x39u64), ((rax as u8)) as u8); rdx = rsi.wrapping_sub(0x2c); rcx = rdi.wrapping_add(0x3c); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru32(rsi.wrapping_sub(0x4u64))) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x64u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(r15.wrapping_add(0x20u64))) as u32 as u64; rdx = (((rax) as i64).wrapping_mul((0x68u64) as i64) as u64) as u64; rcx = (crate::simd_ops::ru64(r15.wrapping_add(0x18u64))) as u64; rcx = ((rcx).wrapping_add(0xffffffffffffffd4u64)) as u64; rcx = ((rcx).wrapping_add(rdx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P8, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = (crate::simd_ops::lk_sub_u32(r15.wrapping_add(0x20u64), (1) as u32)) as u32 as u64; bb = 5; continue; }
            5 => { fl_l = (crate::simd_ops::lk_add_u32(r15.wrapping_add(0x14u64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; if (((fl_l as u32)) != 0) { bb = 6; continue; } rax = 0xffffffff; let tmp_xchg: u32 = crate::simd_ops::atomic_xchg_u32(r15.wrapping_add(0x10u64), ((rax as u32)) as u32); rax = (tmp_xchg) as u32 as u64; rcx = r15.wrapping_add(0x8); (rax, xmm0) = crate::simd_ops::call_stk_xmm((crate::simd_ops::ru64(P9)) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 6; continue; }
            6 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_23999(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (0u64, 0u64, 0u64);
    rax = (crate::simd_ops::ru64(rdx)) as u64;
    fl_l = (crate::simd_ops::ru64(rcx)) as u64; fl_r = rax;
    rax = (rax & !0xffu64) | (((if (((fl_l as u64)) != ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64));
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_24000(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    fl_l = (crate::simd_ops::lk_add_u64(rcx, (0x8u64) as u64)) as u64; fl_r = fl_l;
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    crate::simd_ops::wu64(rdx, rax);
    rax = rdx;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_24001<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64, const P9: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        core::sync::atomic::fence(core::sync::atomic::Ordering::SeqCst);
        match bb {
            0 => { fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rsi = r8; rbx = rdx; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rdx); rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P0)) as u64, rax, rcx, rdx, r8, r9, rsp); rdi = ((rax as u32)) as u32 as u64; rcx = (crate::simd_ops::ru32(rbx.wrapping_add(P1))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 1; continue; } rcx = rbx; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P2)) as u64, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu32(rbx.wrapping_add(P1), ((rdi as u32)) as u32); bb = 1; continue; }
            1 => { crate::simd_ops::lk_add_u32(rbx.wrapping_add(P3), (1) as u32); rdx = (crate::simd_ops::ru32(rbx.wrapping_add(P4))) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rbx.wrapping_add(P5))) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 2; continue; } rdx = (((rdx as u32)).wrapping_add(1)) as u32 as u64; rcx = ((rbx.wrapping_add(P6)) as u64) as u64; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); bb = 2; continue; }
            2 => { rcx = (crate::simd_ops::ru32(rbx.wrapping_add(P8))) as u32 as u64; rax = (crate::simd_ops::ru64(rbx.wrapping_add(P6))) as u64; crate::simd_ops::wu64(rax.wrapping_add(rcx.wrapping_mul(8)), rsi); crate::simd_ops::lk_add_u32(rbx.wrapping_add(P8), (1) as u32); rax = (crate::simd_ops::ru32(rbx.wrapping_add(P8))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rbx.wrapping_add(P5))) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 3; continue; } crate::simd_ops::wu32(rbx.wrapping_add(P8), (0x0u32) as u32); bb = 3; continue; }
            3 => { crate::simd_ops::lk_add_u32(rbx.wrapping_add(P4), (1) as u32); fl_l = (crate::simd_ops::lk_add_u32(rbx.wrapping_add(P3), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; if (((fl_l as u32)) != 0) { bb = 4; continue; } rax = 0xffffffff; let tmp_xchg: u32 = crate::simd_ops::atomic_xchg_u32(rbx.wrapping_add(P1), ((rax as u32)) as u32); rax = (tmp_xchg) as u32 as u64; rcx = rbx; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P9)) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 4; continue; }
            4 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); bb = 5; continue; }
            5 => { return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_24002<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        core::sync::atomic::fence(core::sync::atomic::Ordering::SeqCst);
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rcx; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rcx); rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P0)) as u64, rax, rcx, rdx, r8, r9, rsp); rdi = ((rax as u32)) as u32 as u64; rdx = (crate::simd_ops::ru32(rbx.wrapping_add(0x8u64))) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 1; continue; } rcx = rbx; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P1)) as u64, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu32(rbx.wrapping_add(0x8u64), ((rdi as u32)) as u32); bb = 1; continue; }
            1 => { rsi = rbx.wrapping_add(0xc); crate::simd_ops::lk_add_u32(rsi, (1) as u32); r14 = rbx.wrapping_add(0x20); fl_l = (crate::simd_ops::ru32(r14)) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 4; continue; } rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rbp = rax; rdi = rbx.wrapping_add(0x10); rcx = (crate::simd_ops::ru32(rdi.wrapping_add(0x10u64))) as u32 as u64; rdx = r14; fl_l = ((rcx as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rdi.wrapping_add(0x14u64))) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 2; continue; } rdx = ((rcx.wrapping_add(0x1u64)) as u32) as u32 as u64; rcx = rdi; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rdx = rbx.wrapping_add(0x20); rsi = rbx.wrapping_add(0xc); bb = 2; continue; }
            2 => { rcx = (crate::simd_ops::ru32(rdi.wrapping_add(0xcu64))) as u32 as u64; rax = (crate::simd_ops::ru64(rdi)) as u64; crate::simd_ops::wu64(rax.wrapping_add(rcx.wrapping_mul(8)), rbp); crate::simd_ops::lk_add_u32(rdi.wrapping_add(0xcu64), (1) as u32); rax = (crate::simd_ops::ru32(rdi.wrapping_add(0xcu64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rdi.wrapping_add(0x14u64))) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 3; continue; } crate::simd_ops::wu32(rdi.wrapping_add(0xcu64), (0x0u32) as u32); r14 = rdx; bb = 3; continue; }
            3 => { crate::simd_ops::lk_add_u32(rdi.wrapping_add(0x10u64), (1) as u32); fl_l = (crate::simd_ops::lk_add_u64(rbx.wrapping_add(0x28u64), (1) as u64)) as u64; bb = 5; continue; }
            4 => { rdi = rbx.wrapping_add(0x10); bb = 5; continue; }
            5 => { rcx = (crate::simd_ops::ru64(rdi)) as u64; r9 = (crate::simd_ops::ru32(rdi.wrapping_add(0x14u64))) as u32 as u64; r8 = (crate::simd_ops::ru32(rdi.wrapping_add(0x8u64))) as u32 as u64; rax = ((r8 as u32)) as u32 as u64; rdx = 0; fl_l = 0; fl_r = 0; let d = (r9) as u128; let n = ((rdx as u128) << 64) | (rax as u128); if d != 0 { rax = (n / d) as u64; rdx = (n % d) as u64; } rdi = (crate::simd_ops::ru64(rcx.wrapping_add(rdx.wrapping_mul(8)))) as u64; rcx = (crate::simd_ops::ru32(r14)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 7; continue; } rax = ((r8.wrapping_add(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x18u64), ((rax as u32)) as u32); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((r9 as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 6; continue; } crate::simd_ops::wu32(rbx.wrapping_add(0x18u64), (0x0u32) as u32); bb = 6; continue; }
            6 => { fl_l = (((rcx as u32)).wrapping_sub(1)) as u32 as u64; rcx = ((fl_l as u32)) as u32 as u64; crate::simd_ops::wu32(r14, ((rcx as u32)) as u32); bb = 7; continue; }
            7 => { fl_l = (crate::simd_ops::lk_add_u32(rsi, (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; if (((fl_l as u32)) != 0) { bb = 8; continue; } rcx = 0xffffffff; let tmp_xchg: u32 = crate::simd_ops::atomic_xchg_u32(rbx.wrapping_add(0x8u64), ((rcx as u32)) as u32); rcx = (tmp_xchg) as u32 as u64; rcx = rbx; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P4)) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 8; continue; }
            8 => { rax = rdi; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_24028<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (0u64, 0u64, 0u64);
    rdx = (crate::simd_ops::ru64(rcx)) as u64;
    rax = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64;
    rcx = ((rdx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64;
    fl_l = rdx; fl_r = rcx;
    rax = (rax & !0xffu64) | (((if (((fl_l as u64)) == ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64));
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_24071<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rdi = rcx; rbx = 0; fl_l = 0; fl_r = 0; rcx = 0x6348; rax = ((crate::exe::tls_pointer() as u64)) as u64; rdx = (crate::simd_ops::ru64(rax)) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(rdx))) as u64; r8 = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rdx = (crate::simd_ops::ru64(r8.wrapping_add(P1))) as u64; rax = rdx; rax = ((((rax as u32)) & (0xfu32))) as u32 as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 3; continue; } fl_l = (crate::simd_ops::lk_add_u32(r8.wrapping_add(P2), (1) as u32)) as u32 as u64; rax = (crate::simd_ops::ru64(rdx)) as u64; crate::simd_ops::wu64(r8.wrapping_add(P1), rax); bb = 1; continue; }
            1 => { rbx = rdx; bb = 2; continue; }
            2 => { crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rbx); rcx = rbx; rax = crate::simd_ops::call_stk_caught(P3, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rdi, rax); rax = rdi; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            3 => { rdx = 0xe0; r8 = 0x10; rax = crate::simd_ops::call_stk_caught(P4, rax, rcx, rdx, r8, r9, rsp); rdx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 1; continue; } rax = crate::simd_ops::call_stk_caught((crate::simd_ops::ru64(P5)) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = rax; rdx = 0xdeadd00d; rax = crate::simd_ops::call_stk_caught((crate::simd_ops::ru64(P6)) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 2; continue; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_24112<const P0: u64, const P1: u64, const P2: u64, const P3: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut r10, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { r9 = rcx; rdx = P0; rcx = P1; r8 = (0xcbf29ce484222325u64) as u64; r10 = (0x100000001b3u64) as u64; bb = 1; continue; }
            1 => { rax = (((crate::simd_ops::ru8(rdx)) as i8 as i64) as u64) as u64; rdx = ((rdx).wrapping_add(1)) as u64; r8 = (((r8) ^ (rax))) as u64; r8 = (((r8) as i64).wrapping_mul((r10) as i64) as u64) as u64; fl_l = rdx; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } rax = 0; fl_l = 0; fl_r = 0; rcx = ((r9.wrapping_add(P2)) as u64) as u64; bb = 2; continue; }
            2 => { rdx = (crate::simd_ops::ru64(rcx)) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } fl_l = rdx; fl_r = r8; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rax = (((rax as u32)).wrapping_add(1)) as u32 as u64; rcx = ((rcx).wrapping_add(0x8u64)) as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x40; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 2; continue; } rax = 0; fl_l = 0; fl_r = 0; return rax; }
            4 => { rax = (rax as u32 as i32 as i64) as u64; rax = (crate::simd_ops::ru64(r9.wrapping_add(rax.wrapping_mul(8)).wrapping_add(P3))) as u64; return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_24113<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64, const P9: u64, const P10: u64, const P11: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rsi = (0xcbf29ce484222325u64) as u64; r8 = P0; rbx = rsi; rcx = P1; rdx = (0x100000001b3u64) as u64; bb = 1; continue; }
            1 => { rax = (((crate::simd_ops::ru8(rcx)) as i8 as i64) as u64) as u64; rcx = ((rcx).wrapping_add(1)) as u64; rbx = (((rbx) ^ (rax))) as u64; rbx = (((rbx) as i64).wrapping_mul((rdx) as i64) as u64) as u64; fl_l = rcx; fl_r = r8; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } rdi = rsi; r8 = P2; rcx = P3; bb = 2; continue; }
            2 => { rax = (((crate::simd_ops::ru8(rcx)) as i8 as i64) as u64) as u64; rcx = ((rcx).wrapping_add(1)) as u64; rdi = (((rdi) ^ (rax))) as u64; rdi = (((rdi) as i64).wrapping_mul((rdx) as i64) as u64) as u64; fl_l = rcx; fl_r = r8; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } r8 = P4; rcx = P5; bb = 3; continue; }
            3 => { rax = (((crate::simd_ops::ru8(rcx)) as i8 as i64) as u64) as u64; rcx = ((rcx).wrapping_add(1)) as u64; rsi = (((rsi) ^ (rax))) as u64; rsi = (((rsi) as i64).wrapping_mul((rdx) as i64) as u64) as u64; fl_l = rcx; fl_r = r8; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } rcx = P6; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru32(P8)) as u32 as u64; crate::simd_ops::wu64(rax.wrapping_add(P9), rsi); rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; crate::simd_ops::wu64(rax.wrapping_add(P10), rbx); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu32(rax, ((rcx as u32)) as u32); crate::simd_ops::wu32(rax.wrapping_add(P11), (0x2u32) as u32); crate::simd_ops::wu64(rax.wrapping_add(0x18u64), rdi); crate::simd_ops::wu32(rax.wrapping_add(0x28u64), (0x0u32) as u32); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_24116<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r11, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    r11 = rsp;
    crate::simd_ops::wu64(r11.wrapping_add(0x8u64), rcx);
    rsp = ((rsp).wrapping_sub(0x68u64)) as u64;
    rax = P0;
    crate::simd_ops::wu64(r11.wrapping_sub(0x48u64), rax);
    xmm0 = (crate::simd_ops::ru128(rdx)) as u128;
    crate::simd_ops::wu128(rsp.wrapping_add(0x28u64), (xmm0) as u128);
    xmm1 = ((crate::simd_ops::ru64(rdx.wrapping_add(P1))) as u64) as u128;
    crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), ((xmm1 as u64)) as u64);
    rax = r11.wrapping_sub(0x48);
    crate::simd_ops::wu64(r11.wrapping_sub(0x10u64), rax);
    rax = r11.wrapping_sub(0x48);
    crate::simd_ops::wu64(r11.wrapping_add(0x8u64), rax);
    rax = (((crate::simd_ops::ru32(P2)) as i32 as i64) as u64) as u64;
    rcx = (((rax) as i64).wrapping_mul((0x2e0u64) as i64) as u64) as u64;
    rax = (crate::simd_ops::ru64(P3)) as u64;
    rax = ((rax).wrapping_add(0x10u64)) as u64;
    rcx = ((rcx).wrapping_add(rax)) as u64;
    rdx = r11.wrapping_sub(0x48);
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64;
    fl_l = rcx; fl_r = rcx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = rsp.wrapping_add(0x20); fl_l = rcx; fl_r = rdx; rdx = (rdx & !0xffu64) | (((if (((fl_l as u64)) != ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); (rax, xmm0) = crate::simd_ops::call_stk_xmm((crate::simd_ops::ru64(rax.wrapping_add(P5))) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); }
    rsp = ((rsp).wrapping_add(0x68u64)) as u64;
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_24117<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r11, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    r11 = rsp;
    crate::simd_ops::wu64(r11.wrapping_add(0x8u64), rcx);
    rsp = ((rsp).wrapping_sub(0x68u64)) as u64;
    rax = P0;
    crate::simd_ops::wu64(r11.wrapping_sub(0x48u64), rax);
    crate::simd_ops::wu64(r11.wrapping_sub(0x40u64), rdx);
    rax = r11.wrapping_sub(0x48);
    crate::simd_ops::wu64(r11.wrapping_sub(0x10u64), rax);
    rax = r11.wrapping_sub(0x48);
    crate::simd_ops::wu64(r11.wrapping_add(0x8u64), rax);
    rax = (((crate::simd_ops::ru32(P1)) as i32 as i64) as u64) as u64;
    rcx = (((rax) as i64).wrapping_mul((0x2e0u64) as i64) as u64) as u64;
    rcx = ((rcx).wrapping_add(0x10u64)) as u64;
    rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(P2))) as u64;
    rdx = r11.wrapping_sub(0x48);
    rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp);
    rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64;
    fl_l = rcx; fl_r = rcx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = rsp.wrapping_add(0x20); fl_l = rcx; fl_r = rdx; rdx = (rdx & !0xffu64) | (((if (((fl_l as u64)) != ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(rax.wrapping_add(P4))) as u64, rax, rcx, rdx, r8, r9, rsp); }
    rsp = ((rsp).wrapping_add(0x68u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_24118<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r11, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    r11 = rsp;
    crate::simd_ops::wu64(r11.wrapping_add(0x8u64), rcx);
    rsp = ((rsp).wrapping_sub(0x68u64)) as u64;
    rax = P0;
    crate::simd_ops::wu64(r11.wrapping_sub(0x48u64), rax);
    crate::simd_ops::wu64(r11.wrapping_sub(0x40u64), rdx);
    rax = r11.wrapping_sub(0x48);
    crate::simd_ops::wu64(r11.wrapping_sub(0x10u64), rax);
    rax = r11.wrapping_sub(0x48);
    crate::simd_ops::wu64(r11.wrapping_add(0x8u64), rax);
    rax = (((crate::simd_ops::ru32(P1)) as i32 as i64) as u64) as u64;
    rcx = (((rax) as i64).wrapping_mul((0x2e0u64) as i64) as u64) as u64;
    rcx = ((rcx).wrapping_add(0x90u64)) as u64;
    rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(P2))) as u64;
    rdx = r11.wrapping_sub(0x48);
    rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp);
    rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64;
    fl_l = rcx; fl_r = rcx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = rsp.wrapping_add(0x20); fl_l = rcx; fl_r = rdx; rdx = (rdx & !0xffu64) | (((if (((fl_l as u64)) != ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(rax.wrapping_add(P4))) as u64, rax, rcx, rdx, r8, r9, rsp); }
    rsp = ((rsp).wrapping_add(0x68u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_24119<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r11, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    r11 = rsp;
    crate::simd_ops::wu64(r11.wrapping_add(0x8u64), rcx);
    rsp = ((rsp).wrapping_sub(0x68u64)) as u64;
    rax = P0;
    crate::simd_ops::wu64(r11.wrapping_sub(0x48u64), rax);
    rax = r11.wrapping_sub(0x48);
    crate::simd_ops::wu64(r11.wrapping_sub(0x10u64), rax);
    rax = r11.wrapping_sub(0x48);
    crate::simd_ops::wu64(r11.wrapping_add(0x8u64), rax);
    rax = (((crate::simd_ops::ru32(P1)) as i32 as i64) as u64) as u64;
    rcx = (((rax) as i64).wrapping_mul((0x2e0u64) as i64) as u64) as u64;
    rcx = ((rcx).wrapping_add(0x10u64)) as u64;
    rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(P2))) as u64;
    rdx = r11.wrapping_sub(0x48);
    rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp);
    rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64;
    fl_l = rcx; fl_r = rcx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = rsp.wrapping_add(0x20); fl_l = rcx; fl_r = rdx; rdx = (rdx & !0xffu64) | (((if (((fl_l as u64)) != ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(rax.wrapping_add(P4))) as u64, rax, rcx, rdx, r8, r9, rsp); }
    rsp = ((rsp).wrapping_add(0x68u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_24121<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r11, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    r11 = rsp;
    crate::simd_ops::wu64(r11.wrapping_add(0x8u64), rcx);
    rsp = ((rsp).wrapping_sub(0x68u64)) as u64;
    rax = P0;
    crate::simd_ops::wu64(r11.wrapping_sub(0x48u64), rax);
    xmm0 = (crate::simd_ops::ru128(rdx)) as u128;
    crate::simd_ops::wu128(rsp.wrapping_add(0x28u64), (xmm0) as u128);
    rax = r11.wrapping_sub(0x48);
    crate::simd_ops::wu64(r11.wrapping_sub(0x10u64), rax);
    rax = r11.wrapping_sub(0x48);
    crate::simd_ops::wu64(r11.wrapping_add(0x8u64), rax);
    rax = (((crate::simd_ops::ru32(P1)) as i32 as i64) as u64) as u64;
    rcx = (((rax) as i64).wrapping_mul((0x2e0u64) as i64) as u64) as u64;
    rax = (crate::simd_ops::ru64(P2)) as u64;
    rax = ((rax).wrapping_add(0x10u64)) as u64;
    rcx = ((rcx).wrapping_add(rax)) as u64;
    rdx = r11.wrapping_sub(0x48);
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64;
    fl_l = rcx; fl_r = rcx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = rsp.wrapping_add(0x20); fl_l = rcx; fl_r = rdx; rdx = (rdx & !0xffu64) | (((if (((fl_l as u64)) != ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); (rax, xmm0) = crate::simd_ops::call_stk_xmm((crate::simd_ops::ru64(rax.wrapping_add(P4))) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); }
    rsp = ((rsp).wrapping_add(0x68u64)) as u64;
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_24122<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r11, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    r11 = rsp;
    crate::simd_ops::wu64(r11.wrapping_add(0x8u64), rcx);
    rsp = ((rsp).wrapping_sub(0x68u64)) as u64;
    rax = P0;
    crate::simd_ops::wu64(r11.wrapping_sub(0x48u64), rax);
    let y_addr: u64 = rdx; xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16));
    let y_addr: u64 = rsp.wrapping_add(0x28u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi);
    rax = r11.wrapping_sub(0x48);
    crate::simd_ops::wu64(r11.wrapping_sub(0x10u64), rax);
    rax = r11.wrapping_sub(0x48);
    crate::simd_ops::wu64(r11.wrapping_add(0x8u64), rax);
    rax = (((crate::simd_ops::ru32(P1)) as i32 as i64) as u64) as u64;
    rcx = (((rax) as i64).wrapping_mul((0x2e0u64) as i64) as u64) as u64;
    rax = (crate::simd_ops::ru64(P2)) as u64;
    rax = ((rax).wrapping_add(0x10u64)) as u64;
    rcx = ((rcx).wrapping_add(rax)) as u64;
    rdx = r11.wrapping_sub(0x48);
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64;
    fl_l = rcx; fl_r = rcx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = rsp.wrapping_add(0x20); fl_l = rcx; fl_r = rdx; rdx = (rdx & !0xffu64) | (((if (((fl_l as u64)) != ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); (rax, xmm0) = crate::simd_ops::call_stk_xmm((crate::simd_ops::ru64(rax.wrapping_add(P4))) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); }
    rsp = ((rsp).wrapping_add(0x68u64)) as u64;
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_24123<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64, const P9: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = (0xcbf29ce484222325u64) as u64; r8 = P0; rbx = rdi; rcx = P1; rdx = (0x100000001b3u64) as u64; bb = 1; continue; }
            1 => { rax = (((crate::simd_ops::ru8(rcx)) as i8 as i64) as u64) as u64; rcx = ((rcx).wrapping_add(1)) as u64; rbx = (((rbx) ^ (rax))) as u64; rbx = (((rbx) as i64).wrapping_mul((rdx) as i64) as u64) as u64; fl_l = rcx; fl_r = r8; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } r8 = P2; rcx = P3; bb = 2; continue; }
            2 => { rax = (((crate::simd_ops::ru8(rcx)) as i8 as i64) as u64) as u64; rcx = ((rcx).wrapping_add(1)) as u64; rdi = (((rdi) ^ (rax))) as u64; rdi = (((rdi) as i64).wrapping_mul((rdx) as i64) as u64) as u64; fl_l = rcx; fl_r = r8; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } rcx = P4; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rax.wrapping_add(P6), rbx); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu64(rax, rdi); crate::simd_ops::wu64(rax.wrapping_add(P7), (0x2u64) as u64); crate::simd_ops::wu64(rax.wrapping_add(P8), rcx); crate::simd_ops::wu64(rax.wrapping_add(P9), (0xffffffffffffffffu64) as u64); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_24130<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64, const P9: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = (0xcbf29ce484222325u64) as u64; rcx = P0; rdi = rbx; r8 = P1; rdx = rcx; r9 = (0x100000001b3u64) as u64; bb = 1; continue; }
            1 => { rax = (((crate::simd_ops::ru8(rdx)) as i8 as i64) as u64) as u64; rdx = ((rdx).wrapping_add(1)) as u64; rdi = (((rdi) ^ (rax))) as u64; rdi = (((rdi) as i64).wrapping_mul((r9) as i64) as u64) as u64; fl_l = rdx; fl_r = r8; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } rsi = rbx; bb = 2; continue; }
            2 => { rax = (((crate::simd_ops::ru8(rcx)) as i8 as i64) as u64) as u64; rcx = ((rcx).wrapping_add(1)) as u64; rbx = (((rbx) ^ (rax))) as u64; rbx = (((rbx) as i64).wrapping_mul((r9) as i64) as u64) as u64; fl_l = rcx; fl_r = r8; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } rcx = P2; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru32(P4)) as u32 as u64; crate::simd_ops::wu64(rax.wrapping_add(P5), rbx); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu64(rax.wrapping_add(P6), rsi); rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; crate::simd_ops::wu32(rax, ((rcx as u32)) as u32); crate::simd_ops::wu32(rax.wrapping_add(P7), (0x2u32) as u32); crate::simd_ops::wu64(rax.wrapping_add(P8), rdi); crate::simd_ops::wu32(rax.wrapping_add(P9), (0x0u32) as u32); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_24139<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64, const P9: u64, const P10: u64, const P11: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = (0xcbf29ce484222325u64) as u64; rdx = P0; rdi = rbx; r9 = P1; rcx = rdx; r8 = (0x100000001b3u64) as u64; bb = 1; continue; }
            1 => { rax = (((crate::simd_ops::ru8(rcx)) as i8 as i64) as u64) as u64; rcx = ((rcx).wrapping_add(1)) as u64; rdi = (((rdi) ^ (rax))) as u64; rdi = (((rdi) as i64).wrapping_mul((r8) as i64) as u64) as u64; fl_l = rcx; fl_r = r9; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } rsi = rbx; r10 = P2; rcx = P3; bb = 2; continue; }
            2 => { rax = (((crate::simd_ops::ru8(rcx)) as i8 as i64) as u64) as u64; rcx = ((rcx).wrapping_add(1)) as u64; rsi = (((rsi) ^ (rax))) as u64; rsi = (((rsi) as i64).wrapping_mul((r8) as i64) as u64) as u64; fl_l = rcx; fl_r = r10; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 3; continue; }
            3 => { rax = (((crate::simd_ops::ru8(rdx)) as i8 as i64) as u64) as u64; rdx = ((rdx).wrapping_add(1)) as u64; rbx = (((rbx) ^ (rax))) as u64; rbx = (((rbx) as i64).wrapping_mul((r8) as i64) as u64) as u64; fl_l = rdx; fl_r = r9; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } rcx = P4; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru32(P6)) as u32 as u64; crate::simd_ops::wu64(rax.wrapping_add(P7), rbx); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu64(rax.wrapping_add(P8), rsi); rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; crate::simd_ops::wu32(rax, ((rcx as u32)) as u32); crate::simd_ops::wu32(rax.wrapping_add(P9), (0x2u32) as u32); crate::simd_ops::wu64(rax.wrapping_add(P10), rdi); crate::simd_ops::wu32(rax.wrapping_add(P11), (0x0u32) as u32); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_24148<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rdx; rdi = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 1; continue; } rcx = rdi; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rcx = ((rdi.wrapping_add(P2)) as u64) as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rcx = rdi; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); bb = 1; continue; }
            1 => { rdi = (crate::simd_ops::ru64(rbx.wrapping_add(P5))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } rcx = rdi; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rcx = ((rdi.wrapping_add(P2)) as u64) as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rcx = rdi; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); bb = 2; continue; }
            2 => { rdi = (crate::simd_ops::ru64(rbx.wrapping_add(P6))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } rcx = rdi; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rcx = ((rdi.wrapping_add(P2)) as u64) as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rcx = rdi; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); bb = 3; continue; }
            3 => { rcx = rbx; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); bb = 4; continue; }
            4 => { return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_24173<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    crate::simd_ops::wu64(rcx.wrapping_add(P0), (0x0u64) as u64);
    rax = P1;
    crate::simd_ops::wu64(rcx, rax);
    rax = rcx;
    xmm0 = (crate::simd_ops::ru128(rdx)) as u128;
    crate::simd_ops::wu128(rcx.wrapping_add(P2), (xmm0) as u128);
    xmm1 = ((crate::simd_ops::ru64(rdx.wrapping_add(P3))) as u64) as u128;
    crate::simd_ops::wu64(rcx.wrapping_add(P4), ((xmm1 as u64)) as u64);
    crate::simd_ops::wu64(rcx.wrapping_add(P0), rcx);
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_24188<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rcx;
    rcx = ((rcx).wrapping_add(0xd8u64)) as u64;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rcx = ((rbx.wrapping_add(P1)) as u64) as u64;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rcx = ((rbx.wrapping_add(P2)) as u64) as u64;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rcx = ((rbx.wrapping_add(P3)) as u64) as u64;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rax = P4;
    crate::simd_ops::wu64(rbx, rax);
    rcx = rbx;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return crate::simd_ops::tail_stk(P5, rax, rcx, rdx, r8, r9, rsp);
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_24189<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = ((rdx as u32)) as u32 as u64;
    rdi = rcx;
    rcx = ((rcx).wrapping_add(0xd8u64)) as u64;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rcx = ((rdi.wrapping_add(P1)) as u64) as u64;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rcx = ((rdi.wrapping_add(P2)) as u64) as u64;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rcx = ((rdi.wrapping_add(P3)) as u64) as u64;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rax = P4;
    crate::simd_ops::wu64(rdi, rax);
    rcx = rdi;
    rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp);
    fl_l = ((rbx as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64;
    if !(((((fl_l as u8)) & ((fl_r as u8))) == 0)) { rdx = 0xe8; rcx = rdi; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); }
    rax = rdi;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_24197<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64, const P9: u64, const P10: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rsi = rdx; rdi = rcx; rcx = ((rcx).wrapping_add(0xd0u64)) as u64; rbx = (crate::simd_ops::ru64(rdi.wrapping_add(P0))) as u64; fl_l = rdx; fl_r = rbx; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 1; continue; } r8 = ((rbx.wrapping_add(rdx)) as u64) as u64; rdx = 0; fl_l = 0; fl_r = 0; rax = ((r8.wrapping_add(P1)) as u64) as u64; let d = (rbx) as u128; let n = ((rdx as u128) << 64) | (rax as u128); if d != 0 { rax = (n / d) as u64; rdx = (n % d) as u64; } rbx = ((r8.wrapping_add(P1)) as u64) as u64; fl_l = rbx; fl_r = rdx; rbx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; bb = 1; continue; }
            1 => { rdx = ((rbx as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rdi.wrapping_add(P3), rsi); rdx = rbx; rcx = ((rdi.wrapping_add(P4)) as u64) as u64; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rdx = (crate::simd_ops::ru32(rdi.wrapping_add(P6))) as u32 as u64; r8 = (crate::simd_ops::ru32(rdi.wrapping_add(P7))) as u32 as u64; r8 = ((r8).wrapping_shl(4u32)) as u64; rax = ((rdx as u32)) as u32 as u64; rax = ((rax).wrapping_shl(4u32)) as u64; fl_l = rax; fl_r = r8; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 2; continue; } rcx = ((rdi.wrapping_add(P4)) as u64) as u64; rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); bb = 2; continue; }
            2 => { rcx = ((rdi.wrapping_add(P9)) as u64) as u64; rdx = rbx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(P10, rax, rcx, rdx, r8, r9, rsp); }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_24198<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64, const P9: u64, const P10: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbp = rdx; rsi = rcx; rcx = ((rcx).wrapping_add(0xd0u64)) as u64; rbx = (crate::simd_ops::ru32(rcx)) as u32 as u64; rax = ((rbx.wrapping_add(rdx)) as u64) as u64; rdi = (crate::simd_ops::ru64(rsi.wrapping_add(P0))) as u64; fl_l = rax; fl_r = rdi; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 1; continue; } r8 = ((rdi.wrapping_add(rax)) as u64) as u64; rdx = 0; fl_l = 0; fl_r = 0; rax = ((r8.wrapping_add(P1)) as u64) as u64; let d = (rdi) as u128; let n = ((rdx as u128) << 64) | (rax as u128); if d != 0 { rax = (n / d) as u64; rdx = (n % d) as u64; } rdi = ((r8.wrapping_add(P1)) as u64) as u64; fl_l = rdi; fl_r = rdx; rdi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; bb = 1; continue; }
            1 => { rdx = rdi; rdx = ((rdx).wrapping_sub(rbx)) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::lk_add_u64(rsi.wrapping_add(P3), (rbp) as u64); rdx = rdi; rcx = ((rsi.wrapping_add(P4)) as u64) as u64; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rdx = (crate::simd_ops::ru32(rsi.wrapping_add(P6))) as u32 as u64; r8 = (crate::simd_ops::ru32(rsi.wrapping_add(P7))) as u32 as u64; r8 = ((r8).wrapping_shl(4u32)) as u64; rax = ((rdx as u32)) as u32 as u64; rax = ((rax).wrapping_shl(4u32)) as u64; fl_l = rax; fl_r = r8; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 2; continue; } rcx = ((rsi.wrapping_add(P4)) as u64) as u64; rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); bb = 2; continue; }
            2 => { rdx = rdi; rcx = ((rsi.wrapping_add(P9)) as u64) as u64; rax = crate::simd_ops::call_stk(P10, rax, rcx, rdx, r8, r9, rsp); fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 4; continue; } bb = 3; continue; }
            3 => { r8 = (crate::simd_ops::ru64(rsi.wrapping_add(P9))) as u64; rcx = rbx; rcx = ((rcx).wrapping_shr(6u32)) as u64; rdx = rbx; rdx = ((((rdx as u32)) & (0x3fu32))) as u32 as u64; rax = (crate::simd_ops::ru64(r8.wrapping_add(rcx.wrapping_mul(8)))) as u64; fl_l = (((rax) >> (((rdx) as u32 & 63))) & 1) as u64; rax = ((rax) | ((1u64) << (((rdx) as u32 & 63)))) as u64; crate::simd_ops::wu64(r8.wrapping_add(rcx.wrapping_mul(8)), rax); rbx = ((rbx).wrapping_add(1)) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 3; continue; } bb = 4; continue; }
            4 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_24199<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; r13 = ((rdx as u32)) as u32 as u64; r14 = rcx; rcx = ((rcx).wrapping_add(0xd0u64)) as u64; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0xffffffff; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 7; continue; } rsi = (crate::simd_ops::ru32(r14.wrapping_add(0xd0u64))) as u32 as u64; rbx = ((rsi.wrapping_add(r13)) as u32) as u32 as u64; rax = ((rbx as u32)) as u32 as u64; rdi = (crate::simd_ops::ru64(r14.wrapping_add(0x58u64))) as u64; fl_l = rax; fl_r = rdi; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 1; continue; } rbp = rdi; bb = 2; continue; }
            1 => { rbp = rdi.wrapping_sub(0x1); rbp = ((rbp).wrapping_add(rax)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rax = rbp; let d = (rdi) as u128; let n = ((rdx as u128) << 64) | (rax as u128); if d != 0 { rax = (n / d) as u64; rdx = (n % d) as u64; } fl_l = rbp; fl_r = rdx; rbp = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; bb = 2; continue; }
            2 => { r12 = ((rbp as u32)) as u32 as u64; r12 = (((r12 as u32)).wrapping_sub((rsi as u32))) as u32 as u64; r8 = ((r12 as u32)) as u32 as u64; r8 = ((r8).wrapping_add(crate::simd_ops::ru64(r14.wrapping_add(0x90u64)))) as u64; fl_l = r8; fl_r = rdi; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 3; continue; } rcx = ((rdi.wrapping_add(r8)) as u64) as u64; rdx = 0; fl_l = 0; fl_r = 0; rax = rcx.wrapping_sub(0x1); let d = (rdi) as u128; let n = ((rdx as u128) << 64) | (rax as u128); if d != 0 { rax = (n / d) as u64; rdx = (n % d) as u64; } rdi = rcx.wrapping_sub(0x1); fl_l = rdi; fl_r = rdx; rdi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; bb = 3; continue; }
            3 => { crate::simd_ops::wu64(r14.wrapping_add(0x90u64), r8); rdx = rdi; rcx = r14.wrapping_add(0x98); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rdx = (crate::simd_ops::ru32(r14.wrapping_add(0xa0u64))) as u32 as u64; r8 = (crate::simd_ops::ru32(r14.wrapping_add(0xa4u64))) as u32 as u64; r8 = ((r8).wrapping_shl(4u32)) as u64; rax = ((rdx as u32)) as u32 as u64; rax = ((rax).wrapping_shl(4u32)) as u64; fl_l = rax; fl_r = r8; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 4; continue; } rcx = r14.wrapping_add(0x98); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 4; continue; }
            4 => { rdx = rdi; rcx = r14.wrapping_add(0xa8); rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu32(r14.wrapping_add(0xd0u64), ((rbp as u32)) as u32); fl_l = ((r12 as u32)) as u32 as u64; fl_r = ((r13 as u32)) as u32 as u64; r12 = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 6; continue; } rcx = r14.wrapping_add(0xd8); rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu32(rax, ((rbx as u32)) as u32); crate::simd_ops::wu32(rax.wrapping_add(0x4u64), ((r12 as u32)) as u32); fl_l = ((rbx as u32)) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 6; continue; } bb = 5; continue; }
            5 => { r9 = ((rbx as u32)) as u32 as u64; r8 = (crate::simd_ops::ru64(r14.wrapping_add(0xa8u64))) as u64; rdx = ((rbx as u32)) as u32 as u64; rdx = ((rdx).wrapping_shr(6u32)) as u64; r9 = ((((r9 as u32)) & (0x3fu32))) as u32 as u64; rcx = (crate::simd_ops::ru64(r8.wrapping_add(rdx.wrapping_mul(8)))) as u64; fl_l = (((rcx) >> (((r9) as u32 & 63))) & 1) as u64; rcx = ((rcx) | ((1u64) << (((r9) as u32 & 63)))) as u64; crate::simd_ops::wu64(r8.wrapping_add(rdx.wrapping_mul(8)), rcx); rbx = (((rbx as u32)).wrapping_add(1)) as u32 as u64; rcx = (crate::simd_ops::ru32(rax)) as u32 as u64; rcx = (((rcx as u32)).wrapping_add(crate::simd_ops::ru32(rax.wrapping_add(0x4u64)))) as u32 as u64; fl_l = ((rbx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 5; continue; } bb = 6; continue; }
            6 => { rax = ((rsi as u32)) as u32 as u64; bb = 7; continue; }
            7 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_24212<const P0: u64, const P1: u64, const P2: u64, const P3: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut xmm6: u128 = 0; let mut xmm7: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 17>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), r8); crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x48u64)) as u64; r12 = (crate::simd_ops::ru64(r8)) as u64; r13 = (crate::simd_ops::ru64(rdx)) as u64; rcx = r12; crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm6) as u128); crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm7) as u128); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r14 = (crate::simd_ops::ru64(r13.wrapping_add(0x18u64))) as u64; rdx = rax; rcx = (crate::simd_ops::ru64(r13)) as u64; rax = (0xde5fb9d2630458e9u64) as u64; let prod = (rax as u128).wrapping_mul((rdx) as u128); rax = prod as u64; rdx = (prod >> 64) as u64; r15 = 0; fl_l = 0; fl_r = 0; rax = ((rax).wrapping_add(rdx)) as u64; rsi = rax; crate::simd_ops::wu64(rsp.wrapping_add(0xa8u64), rax); rsi = ((rsi).wrapping_shr(7u32)) as u64; rsi = (((rsi) & (r14))) as u64; fl_l = ((((rax as u8)) & (0x7fu8))) as u8 as u64; fl_r = fl_l; rax = (rax & !0xffu64) | ((((fl_l as u8)) as u8 as u64)); rax = ((((rax as u8)) as i8 as i32) as u32) as u32 as u64; xmm7 = (((rax as u32)) as u32) as u128; xmm7 = crate::simd_ops::vpbroadcastb((xmm7 as u8)); bb = 1; continue; }
            1 => { xmm6 = (crate::simd_ops::ru128(rcx.wrapping_add(rsi))) as u128; xmm0 = crate::simd_ops::vpcmpeqb(xmm6, xmm7); rbx = (crate::simd_ops::vpmovmskb(xmm0) as u32) as u32 as u64; fl_l = ((rbx as u32)) as u32 as u64; fl_r = ((rbx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 3; continue; } rbp = (crate::simd_ops::ru64(r13.wrapping_add(0x8u64))) as u64; bb = 2; continue; }
            2 => { let bit_in: u32 = (rbx as u32); if bit_in != 0 { rdi = ((bit_in.trailing_zeros() as u32)) as u32 as u64; } fl_l = bit_in as u64; r8 = r12; crate::simd_ops::wu32(rsp.wrapping_add(0x98u64), (0x0u32) as u32); rdi = ((rdi).wrapping_add(rsi)) as u64; rdi = (((rdi) & (r14))) as u64; rdx = rdi; rdx = ((rdx).wrapping_shl(5u32)) as u64; rdx = ((rdx).wrapping_add(rbp)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 4; continue; } fl_l = (((rbx as u32)) & (((rbx as u32)).wrapping_sub(1))) as u32 as u64; rbx = ((fl_l as u32)) as u32 as u64; if (((fl_l as u32)) != 0) { bb = 2; continue; } rcx = (crate::simd_ops::ru64(r13)) as u64; bb = 3; continue; }
            3 => { xmm0 = crate::simd_ops::vpsignb(xmm6, xmm6); rax = (crate::simd_ops::vpmovmskb(xmm0) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 5; continue; } r15 = ((r15).wrapping_add(0x10u64)) as u64; rsi = ((rsi).wrapping_add(r15)) as u64; fl_l = (((rsi) & (r14))) as u64; fl_r = fl_l; rsi = ((fl_l as u64)) as u64; bb = 1; continue; }
            4 => { rbp = (rbp & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 6; continue; }
            5 => { rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0xa8u64))) as u64; rcx = r13; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0xa0u64))) as u64; rsi = rax; rsi = ((rsi).wrapping_shl(5u32)) as u64; rdi = rax; rsi = ((rsi).wrapping_add(crate::simd_ops::ru64(r13.wrapping_add(0x8u64)))) as u64; rbp = (rbp & !0xffu64) | (((0x1u8) as u8 as u64)); rcx = rsi; rbx = (crate::simd_ops::ru64(rdx.wrapping_add(0x8u64))) as u64; rdx = (crate::simd_ops::ru64(rdx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru64(rbx)) as u64; crate::simd_ops::wu64(rbx, (0x0u64) as u64); crate::simd_ops::wu64(rsi.wrapping_add(0x18u64), rax); bb = 6; continue; }
            6 => { rcx = (crate::simd_ops::ru64(r13)) as u64; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x90u64))) as u64; rcx = ((rcx).wrapping_add(rdi)) as u64; rdx = (crate::simd_ops::ru64(r13.wrapping_add(0x8u64))) as u64; rdi = ((rdi).wrapping_shl(5u32)) as u64; rdi = ((rdi).wrapping_add(rdx)) as u64; crate::simd_ops::wu64(rax, rcx); crate::simd_ops::wu64(rax.wrapping_add(0x8u64), rdi); xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x30u64))) as u128; xmm7 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; crate::simd_ops::wu8(rax.wrapping_add(0x10u64), ((rbp as u8)) as u8); rsp = ((rsp).wrapping_add(0x48u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_24214<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rax = ((crate::exe::tls_pointer() as u64)) as u64; rbp = rdx.wrapping_add(0x18); rdi = (crate::simd_ops::ru64(rcx)) as u64; rbx = rdx; rsi = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; rbp = (((rbp) & (0xfffffffffffffff8u64))) as u64; r15 = (crate::simd_ops::ru64(rcx.wrapping_add(0x18u64))) as u64; r14 = rcx; rcx = (crate::simd_ops::ru64(rax)) as u64; r12 = 0; fl_l = 0; fl_r = 0; rdx = ((rdx).wrapping_shl(5u32)) as u64; r10 = 0x6348; rdx = ((rdx).wrapping_add(rbp)) as u64; fl_l = ((rdx) & ((rdx).wrapping_sub(1))) as u64; rax = ((fl_l as u64)) as u64; r10 = (crate::simd_ops::ru64(r10.wrapping_add(rcx))) as u64; if (((fl_l as u64)) != 0) { bb = 1; continue; } rax = rdx.wrapping_sub(0x10); fl_l = rax; fl_r = (0x3f0u64) as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 1; continue; } rax = rdx; rax = ((rax).wrapping_shr(3u32)) as u64; r8 = (crate::simd_ops::ru64(r10.wrapping_add(rax.wrapping_mul(8)).wrapping_add(0x8u64))) as u64; rcx = (crate::simd_ops::ru64(r8.wrapping_add(0x10u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 3; continue; } r9 = 0; fl_l = 0; fl_r = 0; r8 = 0; fl_l = 0; fl_r = 0; rcx = r10; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 5; continue; }
            1 => { rax = (0x7fffffffffffffffu64) as u64; fl_l = rdx; fl_r = rax; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 2; continue; } rcx = r12; bb = 6; continue; }
            2 => { rax = rdx.wrapping_sub(0x10); fl_l = rax; fl_r = (0x3f0u64) as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 4; continue; } rax = rdx; rax = ((rax).wrapping_shr(3u32)) as u64; r8 = (crate::simd_ops::ru64(r10.wrapping_add(rax.wrapping_mul(8)).wrapping_add(0x8u64))) as u64; rcx = (crate::simd_ops::ru64(r8.wrapping_add(0x10u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } fl_l = ((rcx as u8)) as u8 as u64; fl_r = (0xfu8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { fl_l = (crate::simd_ops::lk_add_u32(r8.wrapping_add(0x18u64), (1) as u32)) as u32 as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; crate::simd_ops::wu64(r8.wrapping_add(0x10u64), rax); bb = 6; continue; }
            4 => { r8 = 0x10; rcx = r10; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 5; continue; }
            5 => { rcx = rax; bb = 6; continue; }
            6 => { fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 7; continue; } (rax, xmm0) = crate::simd_ops::call_stk_xmm((crate::simd_ops::ru64(P2)) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rax; rdx = 0xdeadd00d; (rax, xmm0) = crate::simd_ops::call_stk_xmm((crate::simd_ops::ru64(P3)) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = r12; bb = 7; continue; }
            7 => { rax = ((rcx.wrapping_add(rbp)) as u64) as u64; crate::simd_ops::wu64(r14, rcx); r8 = rbx.wrapping_add(0x10); crate::simd_ops::wu64(r14.wrapping_add(0x8u64), rax); rdx = 0xffffff80; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru64(r14)) as u64; rcx = rbx; crate::simd_ops::wu8(rbx.wrapping_add(rax), (0xffu8) as u8); rax = rbx; rax = ((rax).wrapping_shr(3u32)) as u64; rcx = ((rcx).wrapping_sub(rax)) as u64; crate::simd_ops::wu64(r14.wrapping_add(0x18u64), rbx); rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(r14.wrapping_add(0x10u64)))) as u64; rbx = r12; crate::simd_ops::wu64(r14.wrapping_add(0x28u64), rcx); fl_l = r15; fl_r = r15; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 15; continue; } rbp = (0xde5fb9d2630458e9u64) as u64; bb = 8; continue; }
            8 => { fl_l = (crate::simd_ops::ru8(rdi.wrapping_add(rbx))) as u8 as u64; fl_r = ((r12 as u8)) as u8 as u64; if ((((fl_l as u8)) as i8) < (((fl_r as u8)) as i8)) { bb = 11; continue; } rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r8 = (crate::simd_ops::ru64(r14.wrapping_add(0x18u64))) as u64; rcx = rax; r9 = (crate::simd_ops::ru64(r14)) as u64; rax = rbp; let prod = (rax as u128).wrapping_mul((rcx) as u128); rax = prod as u64; rdx = (prod >> 64) as u64; xmm1 = crate::simd_ops::vpcmpeqd(xmm1, xmm1); r10 = ((rdx.wrapping_add(rax)) as u64) as u64; rdx = r12; rcx = r10; rcx = ((rcx).wrapping_shr(7u32)) as u64; rcx = (((rcx) & (r8))) as u64; xmm0 = crate::simd_ops::vpcmpgtb(xmm1, crate::simd_ops::ru128(r9.wrapping_add(rcx))); rax = (crate::simd_ops::vpmovmskb(xmm0) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 10; continue; } bb = 9; continue; }
            9 => { rdx = ((rdx).wrapping_add(0x10u64)) as u64; rcx = ((rcx).wrapping_add(rdx)) as u64; rcx = (((rcx) & (r8))) as u64; xmm0 = crate::simd_ops::vpcmpgtb(xmm1, crate::simd_ops::ru128(r9.wrapping_add(rcx))); rax = (crate::simd_ops::vpmovmskb(xmm0) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 9; continue; } bb = 10; continue; }
            10 => { let bit_in: u32 = (rax as u32); if bit_in != 0 { rdx = ((bit_in.trailing_zeros() as u32)) as u32 as u64; } fl_l = bit_in as u64; r10 = (r10 & !0xffu64) | ((((((r10 as u8)) & (0x7fu8))) as u8 as u64)); rdx = ((rdx).wrapping_add(rcx)) as u64; rdx = (((rdx) & (r8))) as u64; r8 = rsi; crate::simd_ops::wu8(r9.wrapping_add(rdx), ((r10 as u8)) as u8); rcx = rdx.wrapping_sub(0x10); rax = (crate::simd_ops::ru64(r14.wrapping_add(0x18u64))) as u64; rcx = (((rcx) & (rax))) as u64; rdx = ((rdx).wrapping_shl(5u32)) as u64; rax = ((((rax as u32)) & (0xfu32))) as u32 as u64; rcx = ((rcx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru64(r14)) as u64; crate::simd_ops::wu8(rcx.wrapping_add(rax).wrapping_add(0x1u64), ((r10 as u8)) as u8); rdx = ((rdx).wrapping_add(crate::simd_ops::ru64(r14.wrapping_add(0x8u64)))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 11; continue; }
            11 => { rbx = ((rbx).wrapping_add(1)) as u64; rsi = ((rsi).wrapping_add(0x20u64)) as u64; fl_l = rbx; fl_r = r15; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 8; continue; } fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 15; continue; } rcx = rdi.wrapping_sub(0x1); rax = rdi; rcx = (((rcx) & (0xfffffffffe000000u64))) as u64; rax = ((rax).wrapping_sub(rcx)) as u64; rax = ((rax).wrapping_shr(16u32)) as u64; rdx = rcx.wrapping_add(0x108); rax = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x4u64))) as u32 as u64; rdx = ((rdx).wrapping_sub(rax)) as u64; rax = ((crate::exe::teb_self() as u64)) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(rcx.wrapping_add(0x100u64))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 13; continue; } fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(0xeu64))) as u8 as u64; fl_r = ((r12 as u8)) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 12; continue; } rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(rdi, rax); fl_l = (crate::simd_ops::lk_add_u32(rdx.wrapping_add(0x18u64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; crate::simd_ops::wu64(rdx.wrapping_add(0x20u64), rdi); if (((fl_l as u32)) != 0) { bb = 15; continue; } rcx = rdx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 15; continue; }
            12 => { r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 14; continue; }
            13 => { r8 = 0; fl_l = 0; fl_r = 0; bb = 14; continue; }
            14 => { r9 = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P8, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 15; continue; }
            15 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_24226<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbp = rcx; r14 = r8; rcx = ((rcx).wrapping_add(0xd0u64)) as u64; rdi = rdx; rsi = (crate::simd_ops::ru64(rbp.wrapping_add(0x58u64))) as u64; rbx = (crate::simd_ops::ru32(rcx)) as u32 as u64; fl_l = rdx; fl_r = rsi; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 1; continue; } r8 = ((rsi.wrapping_add(rdx)) as u64) as u64; rdx = 0; fl_l = 0; fl_r = 0; rax = ((r8.wrapping_add(P0)) as u64) as u64; let d = (rsi) as u128; let n = ((rdx as u128) << 64) | (rax as u128); if d != 0 { rax = (n / d) as u64; rdx = (n % d) as u64; } rsi = ((r8.wrapping_add(P0)) as u64) as u64; fl_l = rsi; fl_r = rdx; rsi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; bb = 1; continue; }
            1 => { rdx = ((rsi as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); r8 = r14; crate::simd_ops::wu64(rbp.wrapping_add(0x90u64), rdi); rdx = rsi; rcx = rbp.wrapping_add(0x98); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rcx = rbp.wrapping_add(0x98); rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rdx = rsi; rcx = rbp.wrapping_add(0xa8); rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); fl_l = rbx; fl_r = rsi; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 3; continue; } bb = 2; continue; }
            2 => { r8 = (crate::simd_ops::ru64(rbp.wrapping_add(0xa8u64))) as u64; rdx = rbx; rcx = rbx; rdx = ((((rdx as u32)) & (0x3fu32))) as u32 as u64; rcx = ((rcx).wrapping_shr(6u32)) as u64; rbx = ((rbx).wrapping_add(1)) as u64; rax = (crate::simd_ops::ru64(r8.wrapping_add(rcx.wrapping_mul(8)))) as u64; fl_l = (((rax) >> (((rdx) as u32 & 63))) & 1) as u64; rax = ((rax) | ((1u64) << (((rdx) as u32 & 63)))) as u64; crate::simd_ops::wu64(r8.wrapping_add(rcx.wrapping_mul(8)), rax); fl_l = rbx; fl_r = rsi; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 2; continue; } bb = 3; continue; }
            3 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_24227<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rsi = rcx;
    rbx = rdx;
    rcx = ((rcx).wrapping_add(0xd0u64)) as u64;
    rdi = (crate::simd_ops::ru64(rsi.wrapping_add(P0))) as u64;
    fl_l = rdx; fl_r = rdi;
    if !((((fl_l as u64)) < ((fl_r as u64)))) { r8 = ((rdi.wrapping_add(rdx)) as u64) as u64; rdx = 0; fl_l = 0; fl_r = 0; rax = ((r8.wrapping_add(P1)) as u64) as u64; let d = (rdi) as u128; let n = ((rdx as u128) << 64) | (rax as u128); if d != 0 { rax = (n / d) as u64; rdx = (n % d) as u64; } rdi = ((r8.wrapping_add(P1)) as u64) as u64; fl_l = rdi; fl_r = rdx; rdi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; }
    rdx = ((rdi as u32)) as u32 as u64;
    rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp);
    rdx = rdi;
    crate::simd_ops::wu64(rsi.wrapping_add(P3), rbx);
    rcx = ((rsi.wrapping_add(P4)) as u64) as u64;
    rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp);
    rcx = ((rsi.wrapping_add(P4)) as u64) as u64;
    rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp);
    rcx = ((rsi.wrapping_add(P7)) as u64) as u64;
    rdx = rdi;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return crate::simd_ops::tail_stk(P8, rax, rcx, rdx, r8, r9, rsp);
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_24228<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbp = rcx; rdi = rdx; rcx = ((rcx).wrapping_add(0xd0u64)) as u64; rsi = (crate::simd_ops::ru64(rbp.wrapping_add(0x58u64))) as u64; rbx = (crate::simd_ops::ru32(rcx)) as u32 as u64; rax = ((rbx.wrapping_add(rdx)) as u64) as u64; fl_l = rax; fl_r = rsi; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 1; continue; } r8 = ((rsi.wrapping_add(rax)) as u64) as u64; rdx = 0; fl_l = 0; fl_r = 0; rax = ((r8.wrapping_add(P0)) as u64) as u64; let d = (rsi) as u128; let n = ((rdx as u128) << 64) | (rax as u128); if d != 0 { rax = (n / d) as u64; rdx = (n % d) as u64; } rsi = ((r8.wrapping_add(P0)) as u64) as u64; fl_l = rsi; fl_r = rdx; rsi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; bb = 1; continue; }
            1 => { rdx = ((rsi as u32)) as u32 as u64; rdx = (((rdx as u32)).wrapping_sub((rbx as u32))) as u32 as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::lk_add_u64(rbp.wrapping_add(0x90u64), (rdi) as u64); rcx = rbp.wrapping_add(0x98); rdx = rsi; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rcx = rbp.wrapping_add(0x98); rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rdx = rsi; rcx = rbp.wrapping_add(0xa8); rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); fl_l = rbx; fl_r = rsi; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 3; continue; } bb = 2; continue; }
            2 => { r8 = (crate::simd_ops::ru64(rbp.wrapping_add(0xa8u64))) as u64; rdx = rbx; rcx = rbx; rdx = ((((rdx as u32)) & (0x3fu32))) as u32 as u64; rcx = ((rcx).wrapping_shr(6u32)) as u64; rbx = ((rbx).wrapping_add(1)) as u64; rax = (crate::simd_ops::ru64(r8.wrapping_add(rcx.wrapping_mul(8)))) as u64; fl_l = (((rax) >> (((rdx) as u32 & 63))) & 1) as u64; rax = ((rax) | ((1u64) << (((rdx) as u32 & 63)))) as u64; crate::simd_ops::wu64(r8.wrapping_add(rcx.wrapping_mul(8)), rax); fl_l = rbx; fl_r = rsi; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 2; continue; } bb = 3; continue; }
            3 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_24234<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x30u64)) as u64;
    r8 = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64;
    rbx = rcx;
    fl_l = ((r8 as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64;
    if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { rsp = ((rsp).wrapping_add(0x30u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(P1))) as u8 as u64; fl_r = (0x0u8) as u8 as u64;
    if !((((fl_l as u8)) != ((fl_r as u8)))) { rdx = (crate::simd_ops::ru64(rcx.wrapping_add(P2))) as u64; r9 = 0x8; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P3))) as u64; crate::simd_ops::wu8(rsp.wrapping_add(0x20u64), (0x0u8) as u8); rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu32(rbx.wrapping_add(P0), (0x0u32) as u32); }
    rsp = ((rsp).wrapping_add(0x30u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_24235<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x30u64)) as u64;
    rax = ((crate::simd_ops::ru8(rcx.wrapping_add(P0))) as u32) as u32 as u64;
    rbx = rcx;
    r8 = (crate::simd_ops::ru32(rcx.wrapping_add(P1))) as u32 as u64;
    r9 = 0x8;
    rdx = (crate::simd_ops::ru64(rcx.wrapping_add(P2))) as u64;
    rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P3))) as u64;
    crate::simd_ops::wu8(rsp.wrapping_add(0x20u64), ((rax as u8)) as u8);
    rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp);
    crate::simd_ops::wu32(rbx.wrapping_add(P1), (0x0u32) as u32);
    rsp = ((rsp).wrapping_add(0x30u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_24240<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64, const P9: u64, const P10: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = (crate::simd_ops::ru64(rcx)) as u64; rdi = rcx; rax = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; rsi = ((rax.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rsi = ((rsi).wrapping_shl(5u32)) as u64; rsi = ((rsi).wrapping_add(rbx)) as u64; fl_l = rbx; fl_r = rsi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rcx = rbx; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rbx = ((rbx).wrapping_add(0x120u64)) as u64; fl_l = rbx; fl_r = rsi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } bb = 2; continue; }
            2 => { r9 = (crate::simd_ops::ru64(rdi)) as u64; fl_l = r9; fl_r = r9; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } rcx = ((r9.wrapping_add(P2)) as u64) as u64; rax = r9; rcx = (((rcx) & (0xfffffffffe000000u64))) as u64; rax = ((rax).wrapping_sub(rcx)) as u64; rax = ((rax).wrapping_shr(16u32)) as u64; rdx = ((rcx.wrapping_add(P3)) as u64) as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(P4))) as u32 as u64; rdx = ((rdx).wrapping_sub(rax)) as u64; rax = ((crate::exe::teb_self() as u64)) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(rcx.wrapping_add(P5))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 4; continue; } fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(P6))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 3; continue; } rax = (crate::simd_ops::ru64(rdx.wrapping_add(P7))) as u64; crate::simd_ops::wu64(r9, rax); fl_l = (crate::simd_ops::lk_add_u32(rdx.wrapping_add(P8), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; crate::simd_ops::wu64(rdx.wrapping_add(P7), r9); if (((fl_l as u32)) != 0) { bb = 6; continue; } rcx = rdx; rax = crate::simd_ops::call_stk(P9, rax, rcx, rdx, r8, r9, rsp); bb = 6; continue; }
            3 => { r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 5; continue; }
            4 => { r8 = 0; fl_l = 0; fl_r = 0; bb = 5; continue; }
            5 => { rax = crate::simd_ops::call_stk(P10, rax, rcx, rdx, r8, r9, rsp); bb = 6; continue; }
            6 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = 0; fl_l = 0; fl_r = 0; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; crate::simd_ops::wu64(rdi, rax); crate::simd_ops::wu64(rdi.wrapping_add(P0), rax); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_24250<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rdi = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    rbx = (crate::simd_ops::ru64(rcx)) as u64;
    fl_l = rbx; fl_r = rdi;
    if (((fl_l as u64)) == ((fl_r as u64))) { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    loop { rcx = rbx; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rbx = ((rbx).wrapping_add(0x120u64)) as u64; fl_l = rbx; fl_r = rdi; if !((((fl_l as u64)) != ((fl_r as u64)))) { break; } }
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_24256<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx); rsp = ((rsp).wrapping_sub(0x38u64)) as u64; r9 = 0x6348; rax = ((crate::exe::tls_pointer() as u64)) as u64; rcx = (crate::simd_ops::ru64(rax)) as u64; r9 = (crate::simd_ops::ru64(r9.wrapping_add(rcx))) as u64; rdx = (crate::simd_ops::ru64(r9.wrapping_add(P0))) as u64; rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P1))) as u64; rax = rcx; rax = ((((rax as u32)) & (0xfu32))) as u32 as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 1; continue; } fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 1; continue; } fl_l = (crate::simd_ops::lk_add_u32(rdx.wrapping_add(P2), (1) as u32)) as u32 as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; crate::simd_ops::wu64(rdx.wrapping_add(P1), rax); bb = 2; continue; }
            1 => { rdx = 0x268; r8 = 0x10; rcx = r9; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rcx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 2; continue; } rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P4)) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = rax; rdx = 0xdeadd00d; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P5)) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = 0; fl_l = 0; fl_r = 0; bb = 2; continue; }
            2 => { crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rcx); rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); rsp = ((rsp).wrapping_add(0x38u64)) as u64; return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_24263<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64, const P9: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; r9 = (crate::simd_ops::ru32(P0)) as u32 as u64; rbx = rdx; r8 = (crate::simd_ops::ru64(P1)) as u64; rdx = rcx; rcx = rbx; crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), (0x6u32) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), (0x7u32) as u32); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru32(rbx.wrapping_add(P3))) as u32 as u64; rax = (((rax as u32)).wrapping_sub(1)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x3; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 1; continue; } fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(P4))) as u32 as u64; fl_r = 0x6; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 1; continue; } rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P5))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 1; continue; } rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P6))) as u64; r8 = 0; fl_l = 0; fl_r = 0; rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = ((r8.wrapping_add(P7)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(rax.wrapping_add(P8))) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(P3))) as u32 as u64; rax = (((rax as u32)).wrapping_sub(1)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x3; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 2; continue; } fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(P4))) as u32 as u64; fl_r = 0x6; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 2; continue; } rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P5))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P6))) as u64; r8 = 0; fl_l = 0; fl_r = 0; rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = ((r8.wrapping_add(P7)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(rax.wrapping_add(P8))) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(P3))) as u32 as u64; rax = (((rax as u32)).wrapping_sub(1)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x3; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 3; continue; } rax = (crate::simd_ops::ru32(rbx.wrapping_add(P4))) as u32 as u64; rax = (((rax as u32)).wrapping_sub(0x2u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x4; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 3; continue; } rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P5))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P6))) as u64; r8 = 0; fl_l = 0; fl_r = 0; rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = ((r8.wrapping_add(P7)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(rax.wrapping_add(P8))) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 3; continue; }
            3 => { rcx = rbx; rax = crate::simd_ops::call_stk(P9, rax, rcx, rdx, r8, r9, rsp); rax = 0; fl_l = 0; fl_r = 0; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_24318<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64, const P9: u64, const P10: u64, const P11: u64, const P12: u64, const P13: u64, const P14: u64, const P15: u64, const P16: u64, const P17: u64, const P18: u64, const P19: u64, const P20: u64, const P21: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rbp = rsp; rsp = ((rsp).wrapping_sub(0x60u64)) as u64; rdi = rdx; rbx = rcx; crate::simd_ops::wu8(rsp.wrapping_add(0x38u64), ((r8 as u8)) as u8); rax = P0; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), (0x5u32) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), (0x5u32) as u32); r8 = (crate::simd_ops::ru64(P1)) as u64; rcx = rdx; rax = crate::simd_ops::call_stk_caught(P2, rax, rcx, rdx, r8, r9, rsp); rsi = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rbp.wrapping_add(0x10u64), ((rsi as u32)) as u32); rax = rbp.wrapping_add(0x10); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); r9 = P3; r8 = 0; fl_l = 0; fl_r = 0; rdx = rbx; rcx = rdi; rax = crate::simd_ops::call_stk_caught(P4, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu32(rbp.wrapping_add(0x10u64), ((rsi as u32)) as u32); rdx = rbx.wrapping_add(0x4); rax = rbp.wrapping_add(0x10); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); r9 = P5; r8 = 0; fl_l = 0; fl_r = 0; rcx = rdi; rax = crate::simd_ops::call_stk_caught(P4, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu32(rbp.wrapping_add(0x10u64), ((rsi as u32)) as u32); rdx = rbx.wrapping_add(0x8); rax = rbp.wrapping_add(0x10); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); r9 = P6; r8 = 0; fl_l = 0; fl_r = 0; rcx = rdi; rax = crate::simd_ops::call_stk_caught(P4, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu32(rbp.wrapping_add(0x10u64), ((rsi as u32)) as u32); rdx = rbx.wrapping_add(0xc); rax = rbp.wrapping_add(0x10); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); r9 = P7; r8 = 0; fl_l = 0; fl_r = 0; rcx = rdi; rax = crate::simd_ops::call_stk_caught(P4, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu32(rbp.wrapping_add(0x10u64), ((rsi as u32)) as u32); rdx = rbx.wrapping_add(0x10); rax = rbp.wrapping_add(0x10); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); r9 = P8; r8 = 0; fl_l = 0; fl_r = 0; rcx = rdi; rax = crate::simd_ops::call_stk_caught(P4, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu32(rbp.wrapping_add(0x10u64), ((rsi as u32)) as u32); rdx = rbx.wrapping_add(0x14); rax = rbp.wrapping_add(0x10); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); r9 = P9; r8 = 0; fl_l = 0; fl_r = 0; rcx = rdi; rax = crate::simd_ops::call_stk_caught(P4, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu32(rbp.wrapping_add(0x10u64), ((rsi as u32)) as u32); rdx = rbx.wrapping_add(0x18); rax = rbp.wrapping_add(0x10); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); r9 = P10; r8 = ((rsi.wrapping_add(0x4u64)) as u32) as u32 as u64; rcx = rdi; rax = crate::simd_ops::call_stk_caught(P4, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu32(rbp.wrapping_add(0x10u64), ((rsi as u32)) as u32); rdx = rbx.wrapping_add(0x1c); rax = rbp.wrapping_add(0x10); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); r9 = P11; r8 = ((rsi.wrapping_add(0x5u64)) as u32) as u32 as u64; rcx = rdi; rax = crate::simd_ops::call_stk_caught(P4, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu32(rbp.wrapping_add(0x10u64), ((rsi as u32)) as u32); rdx = rbx.wrapping_add(0x20); rax = rbp.wrapping_add(0x10); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); r9 = P12; r8 = ((rsi.wrapping_add(0x3u64)) as u32) as u32 as u64; rcx = rdi; rax = crate::simd_ops::call_stk_caught(P4, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu32(rbp.wrapping_add(0x10u64), ((rsi as u32)) as u32); rdx = rbx.wrapping_add(0x24); rax = rbp.wrapping_add(0x10); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); r9 = P13; r8 = ((rsi.wrapping_add(0x3u64)) as u32) as u32 as u64; rcx = rdi; rax = crate::simd_ops::call_stk_caught(P4, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu32(rbp.wrapping_add(0x10u64), ((rsi as u32)) as u32); rdx = rbx.wrapping_add(0x28); rax = rbp.wrapping_add(0x10); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); r9 = P14; r8 = ((rsi.wrapping_add(0x3u64)) as u32) as u32 as u64; rcx = rdi; rax = crate::simd_ops::call_stk_caught(P4, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu32(rbp.wrapping_add(0x10u64), ((rsi as u32)) as u32); rdx = rbx.wrapping_add(0x2c); rax = rbp.wrapping_add(0x10); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); r9 = P15; r8 = ((rsi.wrapping_add(0x1u64)) as u32) as u32 as u64; rcx = rdi; rax = crate::simd_ops::call_stk_caught(P4, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu32(rbp.wrapping_add(0x10u64), ((rsi as u32)) as u32); rdx = rbx.wrapping_add(0x30); rax = rbp.wrapping_add(0x10); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); r9 = P16; r8 = ((rsi.wrapping_add(0x2u64)) as u32) as u32 as u64; rcx = rdi; rax = crate::simd_ops::call_stk_caught(P4, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x28u64))) as u32 as u64; rcx = (((rax) as i64).wrapping_mul((0x108u64) as i64) as u64) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rdi.wrapping_add(0x20u64)))) as u64; fl_l = (crate::simd_ops::ru64(rdi.wrapping_add(0x20u64))) as u64; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } rax = 0; fl_l = 0; fl_r = 0; r11 = rsp.wrapping_add(0x60); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x18u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x20u64))) as u64; rdi = (crate::simd_ops::ru64(r11.wrapping_add(0x28u64))) as u64; rsp = r11; rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            1 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x40u64))) as u64; rax = crate::simd_ops::call_stk_caught(P17, rax, rcx, rdx, r8, r9, rsp); rcx = rax; rax = crate::simd_ops::call_stk_caught(P18, rax, rcx, rdx, r8, r9, rsp); r8 = rax; rdx = rdi.wrapping_add(0x20); rcx = rbp.wrapping_sub(0x20); rax = crate::simd_ops::call_stk_caught(P19, rax, rcx, rdx, r8, r9, rsp); rcx = rax; rax = crate::simd_ops::call_stk_caught(P20, rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, P21, rcx, rsp); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_24331<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64, const P9: u64, const P10: u64, const P11: u64, const P12: u64, const P13: u64, const P14: u64, const P15: u64, const P16: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x60u64)) as u64; rdi = rdx; rbx = rcx; crate::simd_ops::wu8(rsp.wrapping_add(0x38u64), ((r8 as u8)) as u8); rax = P0; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), (0x2u32) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), (0x3u32) as u32); r8 = (crate::simd_ops::ru64(P1)) as u64; rcx = rdx; rax = crate::simd_ops::call_stk_caught(P2, rax, rcx, rdx, r8, r9, rsp); rsi = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rsp.wrapping_add(0x70u64), ((rsi as u32)) as u32); rax = rsp.wrapping_add(0x70); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); r9 = P3; r8 = 0; fl_l = 0; fl_r = 0; rdx = rbx; rcx = rdi; rax = crate::simd_ops::call_stk_caught(P4, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu32(rsp.wrapping_add(0x70u64), ((rsi as u32)) as u32); rdx = rbx.wrapping_add(0x4); rax = rsp.wrapping_add(0x70); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); r9 = P5; r8 = ((rsi.wrapping_add(0x3u64)) as u32) as u32 as u64; rcx = rdi; rax = crate::simd_ops::call_stk_caught(P4, rax, rcx, rdx, r8, r9, rsp); rdx = rbx.wrapping_add(0x8); r9 = P6; r8 = 0; fl_l = 0; fl_r = 0; rcx = rdi; rax = crate::simd_ops::call_stk_caught(P7, rax, rcx, rdx, r8, r9, rsp); rdx = rbx.wrapping_add(0x18); r9 = P8; r8 = 0; fl_l = 0; fl_r = 0; rcx = rdi; rax = crate::simd_ops::call_stk_caught(P9, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu8(rsp.wrapping_add(0x70u64), ((rsi as u8)) as u8); rdx = rbx.wrapping_add(0x4c); rax = rsp.wrapping_add(0x70); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); r9 = P10; r8 = 0; fl_l = 0; fl_r = 0; rcx = rdi; rax = crate::simd_ops::call_stk_caught(P11, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x28u64))) as u32 as u64; rcx = (((rax) as i64).wrapping_mul((0x108u64) as i64) as u64) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rdi.wrapping_add(0x20u64)))) as u64; fl_l = (crate::simd_ops::ru64(rdi.wrapping_add(0x20u64))) as u64; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } rax = 0; fl_l = 0; fl_r = 0; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; rsp = ((rsp).wrapping_add(0x60u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            1 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x40u64))) as u64; rax = crate::simd_ops::call_stk_caught(P12, rax, rcx, rdx, r8, r9, rsp); rcx = rax; rax = crate::simd_ops::call_stk_caught(P13, rax, rcx, rdx, r8, r9, rsp); r8 = rax; rdx = rdi.wrapping_add(0x20); rcx = rsp.wrapping_add(0x40); rax = crate::simd_ops::call_stk_caught(P14, rax, rcx, rdx, r8, r9, rsp); rcx = rax; rax = crate::simd_ops::call_stk_caught(P15, rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, P16, rcx, rsp); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_24366<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64, const P9: u64, const P10: u64, const P11: u64, const P12: u64, const P13: u64, const P14: u64, const P15: u64, const P16: u64, const P17: u64, const P18: u64, const P19: u64, const P20: u64, const P21: u64, const P22: u64, const P23: u64, const P24: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x60u64)) as u64; rdi = rdx; rbx = rcx; crate::simd_ops::wu8(rsp.wrapping_add(0x38u64), ((r8 as u8)) as u8); rax = P0; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); rsi = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((rsi as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), (0x8u32) as u32); r8 = (crate::simd_ops::ru64(P1)) as u64; rcx = rdx; rax = crate::simd_ops::call_stk_caught(P2, rax, rcx, rdx, r8, r9, rsp); rcx = rdi; rax = crate::simd_ops::call_stk_caught(P3, rax, rcx, rdx, r8, r9, rsp); rdx = rbx.wrapping_add(0x10); r9 = P4; r8 = 0; fl_l = 0; fl_r = 0; rcx = rdi; rax = crate::simd_ops::call_stk_caught(P5, rax, rcx, rdx, r8, r9, rsp); rdx = rbx.wrapping_add(0x20); r9 = P6; r8 = 0; fl_l = 0; fl_r = 0; rcx = rdi; rax = crate::simd_ops::call_stk_caught(P7, rax, rcx, rdx, r8, r9, rsp); rdx = rbx.wrapping_add(0x30); r9 = P8; r8 = ((rsi.wrapping_add(0x1u64)) as u32) as u32 as u64; rcx = rdi; rax = crate::simd_ops::call_stk_caught(P9, rax, rcx, rdx, r8, r9, rsp); rdx = rbx.wrapping_add(0x40); r9 = P10; r8 = ((rsi.wrapping_add(0x2u64)) as u32) as u32 as u64; rcx = rdi; rax = crate::simd_ops::call_stk_caught(P5, rax, rcx, rdx, r8, r9, rsp); rdx = rbx.wrapping_add(0x50); r9 = P11; r8 = ((rsi.wrapping_add(0x2u64)) as u32) as u32 as u64; rcx = rdi; rax = crate::simd_ops::call_stk_caught(P9, rax, rcx, rdx, r8, r9, rsp); rdx = rbx.wrapping_add(0x60); r9 = P12; r8 = ((rsi.wrapping_add(0x3u64)) as u32) as u32 as u64; rcx = rdi; rax = crate::simd_ops::call_stk_caught(P9, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu8(rsp.wrapping_add(0x70u64), ((rsi as u8)) as u8); rdx = rbx.wrapping_add(0x72); rax = rsp.wrapping_add(0x70); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); r9 = P13; r8 = ((rsi.wrapping_add(0x5u64)) as u32) as u32 as u64; rcx = rdi; rax = crate::simd_ops::call_stk_caught(P14, rax, rcx, rdx, r8, r9, rsp); rdx = rbx.wrapping_add(0x74); r9 = P15; r8 = ((rsi.wrapping_add(0x5u64)) as u32) as u32 as u64; rcx = rdi; rax = crate::simd_ops::call_stk_caught(P16, rax, rcx, rdx, r8, r9, rsp); rdx = rbx.wrapping_add(0x180); r9 = P17; r8 = ((rsi.wrapping_add(0x5u64)) as u32) as u32 as u64; rcx = rdi; rax = crate::simd_ops::call_stk_caught(P5, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu8(rsp.wrapping_add(0x70u64), ((rsi as u8)) as u8); rdx = rbx.wrapping_add(0x70); rax = rsp.wrapping_add(0x70); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); r9 = P18; r8 = ((rsi.wrapping_add(0x6u64)) as u32) as u32 as u64; rcx = rdi; rax = crate::simd_ops::call_stk_caught(P14, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu8(rsp.wrapping_add(0x70u64), ((rsi as u8)) as u8); rdx = rbx.wrapping_add(0x71); rax = rsp.wrapping_add(0x70); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); r9 = P19; r8 = ((rsi.wrapping_add(0x7u64)) as u32) as u32 as u64; rcx = rdi; rax = crate::simd_ops::call_stk_caught(P14, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x28u64))) as u32 as u64; rcx = (((rax) as i64).wrapping_mul((0x108u64) as i64) as u64) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rdi.wrapping_add(0x20u64)))) as u64; fl_l = (crate::simd_ops::ru64(rdi.wrapping_add(0x20u64))) as u64; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } rax = 0; fl_l = 0; fl_r = 0; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; rsp = ((rsp).wrapping_add(0x60u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            1 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x40u64))) as u64; rax = crate::simd_ops::call_stk_caught(P20, rax, rcx, rdx, r8, r9, rsp); rcx = rax; rax = crate::simd_ops::call_stk_caught(P21, rax, rcx, rdx, r8, r9, rsp); r8 = rax; rdx = rdi.wrapping_add(0x20); rcx = rsp.wrapping_add(0x40); rax = crate::simd_ops::call_stk_caught(P22, rax, rcx, rdx, r8, r9, rsp); rcx = rax; rax = crate::simd_ops::call_stk_caught(P23, rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, P24, rcx, rsp); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_24379<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 111>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rbp = rsp.wrapping_sub(0x260); rsp = ((rsp).wrapping_sub(0x360u64)) as u64; rbx = rdx; fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(0x328u64))) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 1; continue; } rdx = rcx; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 6; continue; }
            1 => { fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0x369u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 6; continue; } rdx = 0; fl_l = 0; fl_r = 0; r8 = 0x310; rcx = rsp.wrapping_add(0x20); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rsi = rsp.wrapping_add(0x30); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); r15 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rsp.wrapping_add(0x2cu64), (0x10u32) as u32); crate::simd_ops::wu32(rbp.wrapping_add(0x238u64), (0x6u32) as u32); crate::simd_ops::wu64(rbp.wrapping_add(0x240u64), r15); crate::simd_ops::wu32(rbp.wrapping_add(0x248u64), (0xffffffffu32) as u32); crate::simd_ops::wu64(rbp.wrapping_add(0x250u64), r15); crate::simd_ops::wu64(rbp.wrapping_add(0x230u64), r15); xmm0 = 0u128; ymm0_hi = 0; let y_addr: u64 = rsp.wrapping_add(0x30u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); crate::simd_ops::wu128(rsp.wrapping_add(0x50u64), (xmm0) as u128); rax = ((r15.wrapping_add(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((rax as u32)) as u32); rdi = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rdi = ((rdi).wrapping_add(rdi)) as u64; crate::simd_ops::wu64(rsi.wrapping_add(rdi.wrapping_mul(8)).wrapping_sub(0x30u64), r15); rax = P2; crate::simd_ops::wu64(rsi.wrapping_add(rdi.wrapping_mul(8)).wrapping_sub(0x28u64), rax); crate::simd_ops::wu64(rsi.wrapping_add(rdi.wrapping_mul(8)).wrapping_sub(0x20u64), r15); crate::simd_ops::wu64(rsi.wrapping_add(rdi.wrapping_mul(8)).wrapping_sub(0x18u64), r15); crate::simd_ops::wu32(rsi.wrapping_add(rdi.wrapping_mul(8)).wrapping_sub(0x8u64), (0xffffffffu32) as u32); crate::simd_ops::wu64(rsi.wrapping_add(rdi.wrapping_mul(8)).wrapping_sub(0x10u64), r15); rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x240u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } rcx = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } rax = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm((crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rsi.wrapping_add(rdi.wrapping_mul(8)).wrapping_sub(0x18u64), rax); bb = 2; continue; }
            2 => { crate::simd_ops::wu32(rbp.wrapping_add(0x258u64), ((r15 as u32)) as u32); r14 = (crate::simd_ops::ru64(P3)) as u64; r9 = (crate::simd_ops::ru32(rsp.wrapping_add(0x28u64))) as u32 as u64; r8 = (crate::simd_ops::ru32(rsp.wrapping_add(0x2cu64))) as u32 as u64; rcx = ((r8.wrapping_add(r8.wrapping_mul(2))) as u64) as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rax = ((r9.wrapping_add(r9.wrapping_mul(2))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } rax = r9.wrapping_add(0x1); rdx = ((r8 as u32)) as u32 as u64; rdx = ((rdx).wrapping_shr(1u32)) as u64; rdx = ((rdx).wrapping_add(r8)) as u64; fl_l = rdx; fl_r = rax; rdx = (if (((fl_l as u64)) < ((fl_r as u64))) { rax } else { rdx }) as u64; rcx = rsp.wrapping_add(0x20); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r9 = (crate::simd_ops::ru32(rsp.wrapping_add(0x28u64))) as u32 as u64; bb = 3; continue; }
            3 => { rax = ((r9 as u32)) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rsp.wrapping_add(0x20u64)))) as u64; xmm0 = 0u128; ymm0_hi = 0; let y_addr: u64 = rcx; crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); crate::simd_ops::wu128(rcx.wrapping_add(0x20u64), (xmm0) as u128); rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x28u64))) as u32 as u64; rax = (((rax as u32)).wrapping_add(1)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((rax as u32)) as u32); rdi = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rdi = ((rdi).wrapping_add(rdi)) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(rsi.wrapping_add(rdi.wrapping_mul(8)).wrapping_sub(0x30u64), rbx); crate::simd_ops::wu64(rsi.wrapping_add(rdi.wrapping_mul(8)).wrapping_sub(0x28u64), r14); crate::simd_ops::wu64(rsi.wrapping_add(rdi.wrapping_mul(8)).wrapping_sub(0x20u64), r15); crate::simd_ops::wu64(rsi.wrapping_add(rdi.wrapping_mul(8)).wrapping_sub(0x18u64), r15); crate::simd_ops::wu32(rsi.wrapping_add(rdi.wrapping_mul(8)).wrapping_sub(0x8u64), (0xffffffffu32) as u32); crate::simd_ops::wu64(rsi.wrapping_add(rdi.wrapping_mul(8)).wrapping_sub(0x10u64), r15); rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x240u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } rcx = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } rax = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm((crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rsi.wrapping_add(rdi.wrapping_mul(8)).wrapping_sub(0x18u64), rax); bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x258u64))) as u32 as u64; rax = (((rax as u32)).wrapping_shr(1u32)) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 5; continue; } crate::simd_ops::wu64(rbx, r15); crate::simd_ops::wu64(rbx.wrapping_add(0x8u64), r15); bb = 5; continue; }
            5 => { rdx = (crate::simd_ops::ru32(rsp.wrapping_add(0x28u64))) as u32 as u64; rcx = ((rdx.wrapping_add(rdx.wrapping_mul(2))) as u64) as u64; rcx = ((rcx).wrapping_add(rcx)) as u64; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; rcx = (crate::simd_ops::ru32(rax.wrapping_add(rcx.wrapping_mul(8)).wrapping_sub(0x8u64))) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x248u64), ((rcx as u32)) as u32); rdx = (((rdx as u32)).wrapping_sub(1)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((rdx as u32)) as u32); rcx = rsp.wrapping_add(0x20); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 6; continue; }
            6 => { r11 = rsp.wrapping_add(0x360); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x20u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x28u64))) as u64; rdi = (crate::simd_ops::ru64(r11.wrapping_add(0x30u64))) as u64; rsp = r11; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_24380<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 119>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rbp = rsp.wrapping_sub(0x288); rsp = ((rsp).wrapping_sub(0x388u64)) as u64; r11 = r9; rbx = rdx; r10 = rcx; fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(0x54u64))) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) >= (((fl_r as u32)) as i32)) { bb = 5; continue; } rdx = 0; fl_l = 0; fl_r = 0; r8 = 0x310; rcx = rsp.wrapping_add(0x40); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rsi = rsp.wrapping_add(0x50); crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rsi); r15 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rsp.wrapping_add(0x4cu64), (0x10u32) as u32); crate::simd_ops::wu32(rbp.wrapping_add(0x258u64), (0x6u32) as u32); crate::simd_ops::wu64(rbp.wrapping_add(0x260u64), r15); crate::simd_ops::wu32(rbp.wrapping_add(0x268u64), (0xffffffffu32) as u32); crate::simd_ops::wu64(rbp.wrapping_add(0x270u64), r15); crate::simd_ops::wu64(rbp.wrapping_add(0x250u64), r15); xmm0 = 0u128; ymm0_hi = 0; let y_addr: u64 = rsp.wrapping_add(0x50u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); crate::simd_ops::wu128(rsp.wrapping_add(0x70u64), (xmm0) as u128); rax = ((r15.wrapping_add(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x48u64), ((rax as u32)) as u32); rdi = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rdi = ((rdi).wrapping_add(rdi)) as u64; crate::simd_ops::wu64(rsi.wrapping_add(rdi.wrapping_mul(8)).wrapping_sub(0x30u64), r15); rax = P1; crate::simd_ops::wu64(rsi.wrapping_add(rdi.wrapping_mul(8)).wrapping_sub(0x28u64), rax); crate::simd_ops::wu64(rsi.wrapping_add(rdi.wrapping_mul(8)).wrapping_sub(0x20u64), r15); crate::simd_ops::wu64(rsi.wrapping_add(rdi.wrapping_mul(8)).wrapping_sub(0x18u64), r15); crate::simd_ops::wu32(rsi.wrapping_add(rdi.wrapping_mul(8)).wrapping_sub(0x8u64), (0xffffffffu32) as u32); crate::simd_ops::wu64(rsi.wrapping_add(rdi.wrapping_mul(8)).wrapping_sub(0x10u64), r15); rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x260u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 1; continue; } rcx = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 1; continue; } rax = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm((crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rsi.wrapping_add(rdi.wrapping_mul(8)).wrapping_sub(0x18u64), rax); bb = 1; continue; }
            1 => { crate::simd_ops::wu32(rbp.wrapping_add(0x278u64), ((r15 as u32)) as u32); r14 = (crate::simd_ops::ru64(P2)) as u64; r9 = (crate::simd_ops::ru32(rsp.wrapping_add(0x48u64))) as u32 as u64; r8 = (crate::simd_ops::ru32(rsp.wrapping_add(0x4cu64))) as u32 as u64; rcx = ((r8.wrapping_add(r8.wrapping_mul(2))) as u64) as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rax = ((r9.wrapping_add(r9.wrapping_mul(2))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } rax = r9.wrapping_add(0x1); rdx = ((r8 as u32)) as u32 as u64; rdx = ((rdx).wrapping_shr(1u32)) as u64; rdx = ((rdx).wrapping_add(r8)) as u64; fl_l = rdx; fl_r = rax; rdx = (if (((fl_l as u64)) < ((fl_r as u64))) { rax } else { rdx }) as u64; rcx = rsp.wrapping_add(0x40); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r9 = (crate::simd_ops::ru32(rsp.wrapping_add(0x48u64))) as u32 as u64; bb = 2; continue; }
            2 => { rax = ((r9 as u32)) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rsp.wrapping_add(0x40u64)))) as u64; xmm0 = 0u128; ymm0_hi = 0; let y_addr: u64 = rcx; crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); crate::simd_ops::wu128(rcx.wrapping_add(0x20u64), (xmm0) as u128); rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x48u64))) as u32 as u64; rax = (((rax as u32)).wrapping_add(1)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x48u64), ((rax as u32)) as u32); rdi = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rdi = ((rdi).wrapping_add(rdi)) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; crate::simd_ops::wu64(rsi.wrapping_add(rdi.wrapping_mul(8)).wrapping_sub(0x30u64), rbx); crate::simd_ops::wu64(rsi.wrapping_add(rdi.wrapping_mul(8)).wrapping_sub(0x28u64), r14); crate::simd_ops::wu64(rsi.wrapping_add(rdi.wrapping_mul(8)).wrapping_sub(0x20u64), r15); crate::simd_ops::wu64(rsi.wrapping_add(rdi.wrapping_mul(8)).wrapping_sub(0x18u64), r15); crate::simd_ops::wu32(rsi.wrapping_add(rdi.wrapping_mul(8)).wrapping_sub(0x8u64), (0xffffffffu32) as u32); crate::simd_ops::wu64(rsi.wrapping_add(rdi.wrapping_mul(8)).wrapping_sub(0x10u64), r15); rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x260u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } rcx = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } rax = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm((crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rsi.wrapping_add(rdi.wrapping_mul(8)).wrapping_sub(0x18u64), rax); bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x278u64))) as u32 as u64; rax = (((rax as u32)).wrapping_shr(1u32)) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 4; continue; } crate::simd_ops::wu64(rbx, r15); crate::simd_ops::wu64(rbx.wrapping_add(0x8u64), r15); bb = 4; continue; }
            4 => { rdx = (crate::simd_ops::ru32(rsp.wrapping_add(0x48u64))) as u32 as u64; rcx = ((rdx.wrapping_add(rdx.wrapping_mul(2))) as u64) as u64; rcx = ((rcx).wrapping_add(rcx)) as u64; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rcx = (crate::simd_ops::ru32(rax.wrapping_add(rcx.wrapping_mul(8)).wrapping_sub(0x8u64))) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x268u64), ((rcx as u32)) as u32); rdx = (((rdx as u32)).wrapping_sub(1)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x48u64), ((rdx as u32)) as u32); rcx = rsp.wrapping_add(0x40); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rsp = ((rsp).wrapping_add(0x388u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            5 => { rcx = ((rcx).wrapping_add(0x30u64)) as u64; rdx = r10.wrapping_add(0x20); rax = (crate::simd_ops::ru64(r10.wrapping_add(0x18u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rcx); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rdx); r9 = rbx; r8 = r11; rcx = (crate::simd_ops::ru64(r10.wrapping_add(0x40u64))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rsp = ((rsp).wrapping_add(0x388u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_24432<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x38u64)) as u64; rax = ((crate::exe::tls_pointer() as u64)) as u64; rdx = 0x6348; rcx = (crate::simd_ops::ru64(rax)) as u64; rax = (crate::simd_ops::ru64(rdx.wrapping_add(rcx))) as u64; rdx = (crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64; rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P1))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 1; continue; } fl_l = ((rcx as u8)) as u8 as u64; fl_r = (0xfu8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 1; continue; } fl_l = (crate::simd_ops::lk_add_u32(rdx.wrapping_add(P2), (1) as u32)) as u32 as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; crate::simd_ops::wu64(rdx.wrapping_add(P1), rax); bb = 2; continue; }
            1 => { rdx = 0x90; rcx = rax; r8 = ((rdx.wrapping_add(P3)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rcx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 2; continue; } rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P5)) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = rax; rdx = 0xdeadd00d; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P6)) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = 0; fl_l = 0; fl_r = 0; bb = 2; continue; }
            2 => { rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); rsp = ((rsp).wrapping_add(0x38u64)) as u64; return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_24438<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64, const P9: u64, const P10: u64, const P11: u64, const P12: u64, const P13: u64, const P14: u64, const P15: u64, const P16: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 15>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x70u64)) as u64; rdi = rdx; rbx = rcx; crate::simd_ops::wu8(rsp.wrapping_add(0x38u64), ((r8 as u8)) as u8); rax = P0; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); rsi = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((rsi as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rsi as u32)) as u32); r8 = (crate::simd_ops::ru64(P1)) as u64; rcx = rdx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = rbx; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); let y_addr: u64 = P4; xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rsp.wrapping_add(0x40u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); xmm0 = (crate::simd_ops::ru128(P5)) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rsp.wrapping_add(0x60u64), (xmm0) as u128); rdx = rbx.wrapping_add(0x40); rax = rsp.wrapping_add(0x40); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); r9 = P6; r8 = 0; fl_l = 0; fl_r = 0; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = rbx.wrapping_add(0x70); r9 = P8; r8 = 0; fl_l = 0; fl_r = 0; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P9, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu32(rsp.wrapping_add(0x80u64), ((rsi as u32)) as u32); rdx = rbx.wrapping_add(0x80); rax = rsp.wrapping_add(0x80); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); r9 = P10; r8 = 0; fl_l = 0; fl_r = 0; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P11, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x28u64))) as u32 as u64; rcx = (((rax) as i64).wrapping_mul((0x108u64) as i64) as u64) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rdi.wrapping_add(0x20u64)))) as u64; fl_l = (crate::simd_ops::ru64(rdi.wrapping_add(0x20u64))) as u64; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } rax = 0; fl_l = 0; fl_r = 0; r11 = rsp.wrapping_add(0x70); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x18u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x20u64))) as u64; rsp = r11; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            1 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x40u64))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P12, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P13, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r8 = rax; rdx = rdi.wrapping_add(0x20); rcx = rsp.wrapping_add(0x40); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P14, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P15, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::lift_ops::trap(0x80000003, P16, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_24447<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64, const P9: u64, const P10: u64, const P11: u64, const P12: u64, const P13: u64, const P14: u64, const P15: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x60u64)) as u64; rdi = rdx; rbx = rcx; crate::simd_ops::wu8(rsp.wrapping_add(0x38u64), ((r8 as u8)) as u8); rax = P0; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); rsi = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((rsi as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rsi as u32)) as u32); r8 = (crate::simd_ops::ru64(P1)) as u64; rcx = rdx; rax = crate::simd_ops::call_stk_caught(P2, rax, rcx, rdx, r8, r9, rsp); r9 = P3; r8 = 0; fl_l = 0; fl_r = 0; rdx = rbx; rcx = rdi; rax = crate::simd_ops::call_stk_caught(P4, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu32(rsp.wrapping_add(0x70u64), ((rsi as u32)) as u32); rdx = ((rbx.wrapping_add(P5)) as u64) as u64; rax = rsp.wrapping_add(0x70); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); r9 = P6; r8 = 0; fl_l = 0; fl_r = 0; rcx = rdi; rax = crate::simd_ops::call_stk_caught(P7, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru32(rdi.wrapping_add(P8))) as u32 as u64; rcx = (((rax) as i64).wrapping_mul((0x108u64) as i64) as u64) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rdi.wrapping_add(P9)))) as u64; fl_l = (crate::simd_ops::ru64(rdi.wrapping_add(P9))) as u64; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } rax = 0; fl_l = 0; fl_r = 0; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; rsp = ((rsp).wrapping_add(0x60u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            1 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(P10))) as u64; rax = crate::simd_ops::call_stk_caught(P11, rax, rcx, rdx, r8, r9, rsp); rcx = rax; rax = crate::simd_ops::call_stk_caught(P12, rax, rcx, rdx, r8, r9, rsp); r8 = rax; rdx = rdi.wrapping_add(0x20); rcx = rsp.wrapping_add(0x40); rax = crate::simd_ops::call_stk_caught(P13, rax, rcx, rdx, r8, r9, rsp); rcx = rax; rax = crate::simd_ops::call_stk_caught(P14, rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, P15, rcx, rsp); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}
