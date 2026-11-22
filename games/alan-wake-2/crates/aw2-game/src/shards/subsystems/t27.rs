#![allow(unused_imports, unused_variables, unused_mut, unused_assignments, non_snake_case, dead_code, clippy::all, unreachable_code, unused_parens)]

#[inline(never)]
pub unsafe extern "win64" fn tpl_41965<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    xmm0 = 0u128;
    xmm0 = (xmm0 & !0xffff_ffffu128) | ((((crate::simd_ops::ru32(rcx.wrapping_add(P0))) as i32) as f32).to_bits() as u128);
    xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) / f32::from_bits(xmm0 as u32)).to_bits() as u128);
    xmm2 = 0u128;
    xmm2 = crate::simd_ops::vroundss(xmm2, (xmm1 as u32), 0x1u8);
    rax = (crate::simd_ops::cvt_f32_i32((f32::from_bits(xmm2 as u32)))) as u32 as u64;
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_42053<const P0: u64, const P1: u64, const P2: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64;
    crate::simd_ops::wu32(rdx, ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rcx.wrapping_add(P1))) as u32 as u64;
    crate::simd_ops::wu32(rdx.wrapping_add(P2), ((rax as u32)) as u32);
    rax = rdx;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_42081<const P0: u64, const P1: u64, const P2: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 75>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x258u64)) as u64;
    rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    rcx = ((rcx).wrapping_add(0x2a0u64)) as u64;
    rax = (crate::simd_ops::ru32(P1)) as u32 as u64;
    crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rax as u32)) as u32);
    crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (0xffffffffffffffffu64) as u64);
    rdx = rsp.wrapping_add(0x20);
    rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp);
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64;
    fl_l = rax; fl_r = rax;
    if (((((fl_l as u64)) & ((fl_r as u64))) as i64) < 0) { rsp = ((rsp).wrapping_add(0x258u64)) as u64; return rax; }
    rcx = rsp.wrapping_add(0x30);
    fl_l = ((rcx).wrapping_add(rax)) as u64; fl_r = fl_l;
    rcx = ((fl_l as u64)) as u64;
    if !((((fl_l as u64)) == 0)) { rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(rax)) as u64, rax, rcx, rdx, r8, r9, rsp); }
    rsp = ((rsp).wrapping_add(0x258u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_42117<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rdi = rax; rbx = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64; rsi = (crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64; rsi = ((rsi).wrapping_shl(4u32)) as u64; rsi = ((rsi).wrapping_add(rbx)) as u64; fl_l = rbx; fl_r = rsi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 8; continue; } rcx = (crate::simd_ops::ru32(rax.wrapping_add(P3))) as u32 as u64; bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru32(rbx)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 2; continue; } fl_l = (crate::simd_ops::ru64(rdi.wrapping_add(P4))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 3; continue; } fl_l = (crate::simd_ops::ru32(rdi.wrapping_add(P5))) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 3; continue; } bb = 2; continue; }
            2 => { rbx = ((rbx).wrapping_add(0x10u64)) as u64; fl_l = rbx; fl_r = rsi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } bb = 3; continue; }
            3 => { fl_l = rbx; fl_r = rsi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 8; continue; } bb = 4; continue; }
            4 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P6))) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(rax.wrapping_add(P7))) as u64, rax, rcx, rdx, r8, r9, rsp); rbx = ((rbx).wrapping_add(0x10u64)) as u64; fl_l = rbx; fl_r = rsi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 8; continue; } rcx = (crate::simd_ops::ru32(rdi.wrapping_add(P3))) as u32 as u64; bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru32(rbx)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 6; continue; } fl_l = (crate::simd_ops::ru64(rdi.wrapping_add(P4))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 7; continue; } fl_l = (crate::simd_ops::ru32(rdi.wrapping_add(P5))) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 7; continue; } bb = 6; continue; }
            6 => { rbx = ((rbx).wrapping_add(0x10u64)) as u64; fl_l = rbx; fl_r = rsi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 5; continue; } bb = 7; continue; }
            7 => { fl_l = rbx; fl_r = rsi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 4; continue; } bb = 8; continue; }
            8 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_42137<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    crate::simd_ops::forward_incoming_to_outgoing();
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = rdx;
    rdx = rcx;
    rcx = (crate::simd_ops::ru64(rax)) as u64;
    crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8(P0, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_42155<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rcx;
    rcx = ((rcx).wrapping_sub(0xffffffffffffff80u64)) as u64;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rcx = ((rbx.wrapping_add(P1)) as u64) as u64;
    rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp);
    rcx = ((rbx.wrapping_add(P3)) as u64) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return crate::simd_ops::tail_stk(P4, rax, rcx, rdx, r8, r9, rsp);
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_42201<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rcx;
    rcx = (crate::simd_ops::ru32(rcx)) as u32 as u64;
    rax = ((rcx.wrapping_add(P0)) as u32) as u32 as u64;
    fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x2;
    if !((((fl_l as u32)) > ((fl_r as u32)))) { crate::simd_ops::wu32(rbx, (0x2u32) as u32); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x4;
    if (((fl_l as u32)) != ((fl_r as u32))) { rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    rcx = rbx;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P2))) as u64;
    crate::simd_ops::wu32(rbx, (0x0u32) as u32);
    fl_l = rcx; fl_r = rcx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rax = ((crate::simd_ops::ru8(rbx.wrapping_add(P3))) as u32) as u32 as u64; crate::simd_ops::wu8(rcx.wrapping_add(P4), ((rax as u8)) as u8); }
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_42203<const P0: u64, const P1: u64, const P2: u64, const P3: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    crate::simd_ops::forward_incoming_to_outgoing();
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    fl_l = (crate::simd_ops::ru32(rcx)) as u32 as u64; fl_r = 0x3;
    if !((((fl_l as u32)) != ((fl_r as u32)))) { rdx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; crate::simd_ops::wu32(rcx, (0x1u32) as u32); rcx = (crate::simd_ops::ru64(P1)) as u64; fl_l = (crate::simd_ops::lk_and_u8(rdx.wrapping_add(P2), (0xfdu8) as u8)) as u8 as u64; fl_r = fl_l; crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8(P3, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_42204<const P0: u64, const P1: u64, const P2: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    crate::simd_ops::forward_incoming_to_outgoing();
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    fl_l = (crate::simd_ops::ru32(rcx)) as u32 as u64; fl_r = 0x1;
    if !((((fl_l as u32)) != ((fl_r as u32)))) { rdx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; crate::simd_ops::wu32(rcx, (0x3u32) as u32); rcx = (crate::simd_ops::ru64(P1)) as u64; crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8(P2, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_42238<const P0: u64, const P1: u64, const P2: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    fl_l = (crate::simd_ops::ru8(P0)) as u8 as u64; fl_r = (0x0u8) as u8 as u64;
    rax = P1;
    rcx = P2;
    rax = (if (((fl_l as u8)) == ((fl_r as u8))) { rcx } else { rax }) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_42244<const P0: u64, const P1: u64, const P2: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = (crate::simd_ops::ru64(P0)) as u64;
    fl_l = rbx; fl_r = rbx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rcx = rbx; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rcx = rbx; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); }
    crate::simd_ops::wu64(P0, (0x0u64) as u64);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_42260<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rdi = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    rbx = (crate::simd_ops::ru64(rcx)) as u64;
    rdx = rbx;
    r8 = (crate::simd_ops::ru64(rdi.wrapping_add(P0))) as u64;
    rcx = ((rbx.wrapping_add(P1)) as u64) as u64;
    rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp);
    r8 = (crate::simd_ops::ru64(rdi.wrapping_add(P0))) as u64;
    rcx = ((rbx.wrapping_add(P3)) as u64) as u64;
    r9 = 0;
    fl_l = 0; fl_r = 0;
    rdx = rbx;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return crate::simd_ops::tail_stk(P4, rax, rcx, rdx, r8, r9, rsp);
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_42275<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rdx;
    rdi = rcx;
    crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rcx);
    crate::simd_ops::lk_and_u32(rcx, (0xff000020u32) as u32);
    crate::simd_ops::lk_or_u32(rcx, (0x20u32) as u32);
    crate::simd_ops::wu8(rcx.wrapping_add(P0), (0x20u8) as u8);
    crate::simd_ops::wu32(rcx.wrapping_add(P1), (0x0u32) as u32);
    crate::simd_ops::wu8(rcx.wrapping_add(P2), (0x0u8) as u8);
    rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp);
    rax = (crate::simd_ops::ru64(rbx.wrapping_add(P4))) as u64;
    crate::simd_ops::wu64(rdi.wrapping_add(P4), rax);
    rax = rdi;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_42276<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    crate::simd_ops::forward_incoming_to_outgoing();
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rcx = (crate::simd_ops::ru64(rcx)) as u64;
    fl_l = rcx; fl_r = rcx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rdx = 0x118; crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8(P0, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_42316<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r) = (0u64, 0u64, 0u64, 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { r8 = ((crate::simd_ops::ru8(rdx.wrapping_add(P0))) as u32) as u32 as u64; r10 = ((rdx.wrapping_add(P1)) as u64) as u64; r9 = rcx; fl_l = ((r8 as u32)) as u32 as u64; fl_r = 0x80; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 1; continue; } r8 = ((r8.wrapping_mul(8)) as u32) as u32 as u64; bb = 2; continue; }
            1 => { r8 = (((r8 as u32)).wrapping_shl(8u32)) as u32 as u64; fl_l = ((r8 as u32)) as u32 as u64; fl_r = 0x7c00; r8 = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru32(rdx.wrapping_add(P2))) as u32 as u64; rax = ((((rax as u32)) & (0xffffffu32))) as u32 as u64; rax = (((rax as u32)).wrapping_shl(3u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 3; continue; } r10 = (crate::simd_ops::ru64(r10)) as u64; bb = 3; continue; }
            3 => { rdx = (crate::simd_ops::ru32(rdx.wrapping_add(P3))) as u32 as u64; rcx = (crate::simd_ops::ru64(rcx)) as u64; fl_l = rdx; fl_r = (crate::simd_ops::ru64(r9.wrapping_add(P4))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 6; continue; } fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } fl_l = r10; fl_r = rcx; r10 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; r11 = P5; bb = 4; continue; }
            4 => { rax = ((crate::simd_ops::ru8(r10.wrapping_add(rcx))) as u32) as u32 as u64; rdx = ((rdx).wrapping_sub(1)) as u64; r8 = ((crate::simd_ops::ru8(rax.wrapping_add(r11))) as u32) as u32 as u64; rax = ((crate::simd_ops::ru8(rcx)) as u32) as u32 as u64; fl_l = ((r8 as u8)) as u8 as u64; fl_r = (crate::simd_ops::ru8(rax.wrapping_add(r11))) as u8 as u64; if ((((fl_l as u8)) as i8) < (((fl_r as u8)) as i8)) { bb = 6; continue; } if ((((fl_l as u8)) as i8) > (((fl_r as u8)) as i8)) { bb = 6; continue; } rcx = ((rcx).wrapping_add(1)) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 4; continue; } bb = 5; continue; }
            5 => { rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); return rax; }
            6 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_42379<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rdx = 0;
    fl_l = 0; fl_r = 0;
    rbx = rcx;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rax = rbx;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_42387<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x30u64)) as u64;
    rax = r8;
    rbx = rdx;
    r8 = (crate::simd_ops::ru64(rcx)) as u64;
    rdx = rax;
    rcx = rbx;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rax = rbx;
    rsp = ((rsp).wrapping_add(0x30u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_42410<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 0>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64;
    crate::simd_ops::wu64(rcx.wrapping_add(P0), rax);
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    crate::simd_ops::wu64(rcx.wrapping_add(P1), rax);
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64;
    crate::simd_ops::wu64(rcx.wrapping_add(P2), rax);
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64;
    crate::simd_ops::wu64(rcx.wrapping_add(P3), rax);
    rax = rcx;
    crate::simd_ops::wu64(rcx, rdx);
    crate::simd_ops::wu64(rcx.wrapping_add(P4), r8);
    crate::simd_ops::wu64(rcx.wrapping_add(P5), r9);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_42418<const P0: u64, const P1: u64, const P2: u64, const P3: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r10, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 0>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    r9 = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    r10 = ((r9.wrapping_add(rax.wrapping_mul(8))) as u64) as u64;
    fl_l = r9; fl_r = r10;
    if (((fl_l as u64)) == ((fl_r as u64))) { return rax; }
    loop { rax = (crate::simd_ops::ru64(rdx.wrapping_add(P1))) as u64; r9 = ((r9).wrapping_add(0x8u64)) as u64; rcx = ((crate::simd_ops::ru16(P2)) as u32) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (0xffffffffffffffffu64) as u64); r8 = (crate::simd_ops::ru64(rax.wrapping_add(rcx.wrapping_mul(8)))) as u64; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(r8.wrapping_add(P3), rax); fl_l = r9; fl_r = r10; if !((((fl_l as u64)) != ((fl_r as u64)))) { break; } }
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_42432<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    fl_l = (crate::simd_ops::lk_add_u64(rcx.wrapping_add(P0), (0x10u64) as u64)) as u64; fl_r = fl_l;
    rax = rcx;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_42434<const P0: u64, const P1: u64, const P2: u64, const P3: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x28u64)) as u64; fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(P0))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 1; continue; } fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(P0))) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 1; continue; } rax = (crate::simd_ops::ru64(rdx)) as u64; fl_l = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; fl_r = rax; rax = (rax & !0xffu64) | (((if (((fl_l as u64)) == ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
            1 => { rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, P3, rcx, rsp); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_42437<const P0: u64, const P1: u64, const P2: u64, const P3: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x28u64)) as u64; fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(P0))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 1; continue; } fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(P0))) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 1; continue; } rax = (crate::simd_ops::ru64(rdx)) as u64; fl_l = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; fl_r = rax; rax = (rax & !0xffu64) | (((if (((fl_l as u64)) != ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
            1 => { rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, P3, rcx, rsp); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_42439<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = (crate::simd_ops::ru64(rdx)) as u64;
    crate::simd_ops::wu64(rcx, rax);
    rax = rcx;
    crate::simd_ops::wu8(rcx.wrapping_add(P0), (0x1u8) as u8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_42440<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    xmm0 = (crate::simd_ops::ru128(r8)) as u128;
    crate::simd_ops::wu128(rcx, (xmm0) as u128);
    xmm1 = ((crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64) as u128;
    crate::simd_ops::wu64(rcx.wrapping_add(P0), ((xmm1 as u64)) as u64);
    rax = rcx;
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_42443<const P0: u64, const P1: u64, const P2: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    xmm0 = (crate::simd_ops::ru128(r8)) as u128;
    crate::simd_ops::wu128(rcx.wrapping_add(P0), (xmm0) as u128);
    xmm1 = ((crate::simd_ops::ru64(r8.wrapping_add(P1))) as u64) as u128;
    crate::simd_ops::wu64(rcx.wrapping_add(P2), ((xmm1 as u64)) as u64);
    rax = rcx;
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_42449(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    xmm0 = (crate::simd_ops::ru128(r8)) as u128;
    crate::simd_ops::wu128(rcx, (xmm0) as u128);
    rax = rcx;
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_42450<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (0u64, 0u64, 0u64);
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    rdx = (crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64;
    fl_l = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; fl_r = rdx;
    rax = (rax & !0xffu64) | (((if (((fl_l as u64)) == ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64));
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_42452<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    xmm0 = (crate::simd_ops::ru128(r8)) as u128;
    crate::simd_ops::wu128(rcx.wrapping_add(P0), (xmm0) as u128);
    rax = rcx;
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_42455<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (0u64, 0u64, 0u64);
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    rdx = (crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64;
    fl_l = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; fl_r = rdx;
    rax = (rax & !0xffu64) | (((if (((fl_l as u64)) != ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64));
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_42480<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu64(rcx, (0x0u64) as u64);
    crate::simd_ops::wu64(rcx.wrapping_add(P0), rax);
    rbx = r8;
    crate::simd_ops::wu32(rcx.wrapping_add(P1), ((rax as u32)) as u32);
    rdi = rcx;
    crate::simd_ops::wu8(rcx.wrapping_add(P2), ((rax as u8)) as u8);
    rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp);
    rax = (crate::simd_ops::ru64(rbx)) as u64;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    crate::simd_ops::wu64(rdi.wrapping_add(P4), rax);
    rax = rdi;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_42481<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (0u64, 0u64, 0u64);
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    fl_l = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; fl_r = rax;
    if (((fl_l as u64)) != ((fl_r as u64))) { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; return rax; }
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64;
    fl_l = (crate::simd_ops::ru64(rdx)) as u64; fl_r = rax;
    if !((((fl_l as u64)) != ((fl_r as u64)))) { rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); return rax; }
    rax = (rax & !0xffu64) | (((0u8) as u8 as u64));
    fl_l = 0; fl_r = 0;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_42518<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    crate::simd_ops::wu32(P0, ((xmm1 as u32)) as u32);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_42543(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    crate::simd_ops::wu64(rcx, rdx);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_42554<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    crate::simd_ops::wu8(P0, ((rcx as u8)) as u8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_42593(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (0u64, 0u64, 0u64);
    rax = (crate::simd_ops::ru32(rdx)) as u32 as u64;
    fl_l = (crate::simd_ops::ru32(rcx)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64;
    rax = (rax & !0xffu64) | (((if (((fl_l as u32)) == ((fl_r as u32))) { 1u8 } else { 0u8 }) as u8 as u64));
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_42617<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rsp = ((rsp).wrapping_sub(0x58u64)) as u64;
    xmm0 = (crate::simd_ops::ru128(rcx.wrapping_add(P0))) as u128;
    xmm1 = (crate::simd_ops::ru128(rcx.wrapping_add(P1))) as u128;
    rdx = ((crate::simd_ops::ru16(rcx.wrapping_add(P2))) as u32) as u32 as u64;
    rax = ((rcx.wrapping_add(P3)) as u64) as u64;
    r9 = ((rcx.wrapping_add(P4)) as u64) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (0x0u64) as u64);
    rcx = rsp.wrapping_add(0x40);
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax);
    r8 = rsp.wrapping_add(0x30);
    crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm0) as u128);
    crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm1) as u128);
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    rsp = ((rsp).wrapping_add(0x58u64)) as u64;
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_42621<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rsp = ((rsp).wrapping_sub(0x58u64)) as u64;
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    rdx = ((rcx.wrapping_add(P1)) as u64) as u64;
    xmm0 = (crate::simd_ops::ru128(rcx.wrapping_add(P2))) as u128;
    xmm1 = (crate::simd_ops::ru128(rcx)) as u128;
    crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax);
    r9 = ((rcx.wrapping_add(P3)) as u64) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdx);
    r8 = rsp.wrapping_add(0x30);
    rdx = ((crate::simd_ops::ru16(rcx.wrapping_add(P4))) as u32) as u32 as u64;
    rcx = rsp.wrapping_add(0x40);
    crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm0) as u128);
    crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm1) as u128);
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    rsp = ((rsp).wrapping_add(0x58u64)) as u64;
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_42624<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rsp = ((rsp).wrapping_sub(0x58u64)) as u64;
    xmm0 = (crate::simd_ops::ru128(rcx.wrapping_add(P0))) as u128;
    xmm1 = (crate::simd_ops::ru128(rcx)) as u128;
    rdx = ((crate::simd_ops::ru16(rcx.wrapping_add(P1))) as u32) as u32 as u64;
    rax = ((rcx.wrapping_add(P2)) as u64) as u64;
    r9 = ((rcx.wrapping_add(P3)) as u64) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (0x0u64) as u64);
    rcx = rsp.wrapping_add(0x40);
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax);
    r8 = rsp.wrapping_add(0x30);
    crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm0) as u128);
    crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm1) as u128);
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    rsp = ((rsp).wrapping_add(0x58u64)) as u64;
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_42643(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    fl_l = (crate::simd_ops::lk_add_u64(rcx, (0x100u64) as u64)) as u64; fl_r = fl_l;
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    crate::simd_ops::wu64(rdx, rax);
    rax = rdx;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_42651<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64;
    rax = ((rax).wrapping_shl(8u32)) as u64;
    fl_l = ((rax).wrapping_add(crate::simd_ops::ru64(rcx))) as u64; fl_r = fl_l;
    rax = ((fl_l as u64)) as u64;
    crate::simd_ops::wu64(rdx, rax);
    rax = rdx;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_42653<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = P0;
    crate::simd_ops::wu64(rcx.wrapping_add(P1), rax);
    rax = rcx;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_42657<const P0: u64, const P1: u64, const P2: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        core::sync::atomic::fence(core::sync::atomic::Ordering::SeqCst);
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = rcx; rbx = (crate::simd_ops::ru64(rcx)) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x8u64))) as u32 as u64; rdx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rsi = ((rbx.wrapping_add(rdx.wrapping_mul(8))) as u64) as u64; fl_l = rbx; fl_r = rsi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 3; continue; } bb = 1; continue; }
            1 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } rax = 0xffffffff; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rdx.wrapping_add(0x24u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 2; continue; } rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x28u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); bb = 2; continue; }
            2 => { rbx = ((rbx).wrapping_add(0x18u64)) as u64; fl_l = rbx; fl_r = rsi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } bb = 3; continue; }
            3 => { rbx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rdi.wrapping_add(0x8u64), ((rbx as u32)) as u32); fl_l = (crate::simd_ops::ru32(rdi.wrapping_add(0xcu64))) as u32 as u64; fl_r = 0x40000000; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 8; continue; } r9 = (crate::simd_ops::ru64(rdi)) as u64; fl_l = r9; fl_r = r9; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } rcx = r9.wrapping_sub(0x1); rcx = (((rcx) & (0xfffffffffe000000u64))) as u64; rax = r9; rax = ((rax).wrapping_sub(rcx)) as u64; rax = ((rax).wrapping_shr(16u32)) as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rdx = rcx.wrapping_add(0x108); rdx = ((rdx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x4u64))) as u32 as u64; rdx = ((rdx).wrapping_sub(rax)) as u64; rax = ((crate::exe::teb_self() as u64)) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(rcx.wrapping_add(0x100u64))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 5; continue; } fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(0xeu64))) as u8 as u64; fl_r = ((rbx as u8)) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 4; continue; } rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(r9, rax); crate::simd_ops::wu64(rdx.wrapping_add(0x20u64), r9); fl_l = (crate::simd_ops::lk_add_u32(rdx.wrapping_add(0x18u64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; if (((fl_l as u32)) != 0) { bb = 7; continue; } rcx = rdx; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 7; continue; }
            4 => { r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 6; continue; }
            5 => { r8 = 0; fl_l = 0; fl_r = 0; bb = 6; continue; }
            6 => { rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 7; continue; }
            7 => { crate::simd_ops::wu64(rdi, rbx); fl_l = (crate::simd_ops::lk_and_u32(rdi.wrapping_add(0xcu64), (0xc0000000u32) as u32)) as u32 as u64; fl_r = fl_l; bb = 8; continue; }
            8 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_42711<const P0: u64, const P1: u64, const P2: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    rcx = P0;
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    fl_l = rax; fl_r = rax;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { xmm0 = ((crate::simd_ops::ru32(rax)) as u32) as u128; rsp = ((rsp).wrapping_add(0x28u64)) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
    xmm0 = ((crate::simd_ops::ru32(P2)) as u32) as u128;
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_42712<const P0: u64, const P1: u64, const P2: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    fl_l = (crate::simd_ops::ru32(P0)) as u32 as u64; fl_r = 0xfffffffd;
    if !(((((fl_l as u32)) & ((fl_r as u32))) == 0)) { rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::lk_or_u32(rax.wrapping_add(P2), (0x8000u32) as u32); rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    crate::simd_ops::lk_and_u32(rax.wrapping_add(P2), (0xffff7fffu32) as u32);
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_42721<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    crate::simd_ops::wu32(P0, ((xmm0 as u32)) as u32);
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_42742<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 0>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rax = ((r8 as u32)) as u32 as u64;
    r9 = P0;
    r8 = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64;
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    crate::simd_ops::wu64(rcx, r9);
    r8 = ((r8).wrapping_add(r8)) as u64;
    r9 = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu64(rdx.wrapping_add(r8.wrapping_mul(8)), rax);
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64;
    crate::simd_ops::wu64(rcx.wrapping_add(P1), r9);
    crate::simd_ops::wu64(rdx.wrapping_add(r8.wrapping_mul(8)).wrapping_add(P1), rax);
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P2))) as u64;
    crate::simd_ops::wu64(rcx.wrapping_add(P2), r9);
    crate::simd_ops::wu64(rdx.wrapping_add(r8.wrapping_mul(8)).wrapping_add(P2), rax);
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P3))) as u64;
    crate::simd_ops::wu64(rcx.wrapping_add(P3), r9);
    crate::simd_ops::wu64(rdx.wrapping_add(r8.wrapping_mul(8)).wrapping_add(P3), rax);
    rax = ((crate::simd_ops::ru8(rsp.wrapping_add(0x18u64))) as u32) as u32 as u64;
    crate::simd_ops::wu8(rdx.wrapping_add(r8.wrapping_mul(8)).wrapping_add(P4), ((rax as u8)) as u8);
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P5))) as u64;
    crate::simd_ops::wu64(rdx.wrapping_add(r8.wrapping_mul(8)).wrapping_add(P5), rax);
    crate::simd_ops::wu64(rcx.wrapping_add(P5), r9);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_42777<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64, const P9: u64, const P10: u64, const P11: u64, const P12: u64, const P13: u64, const P14: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        core::sync::atomic::fence(core::sync::atomic::Ordering::SeqCst);
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rbx = (crate::simd_ops::ru64(P0)) as u64; rdi = P1; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rcx); rcx = rsp.wrapping_add(0x20); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), (0x64u64) as u64); bb = 1; continue; }
            1 => { rax = ((crate::simd_ops::ru8(rbx.wrapping_add(P3))) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { rax = ((crate::simd_ops::ru8(rbx.wrapping_add(P3))) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 2; continue; } bb = 3; continue; }
            3 => { rax = 0x1; let tmp_xchg: u8 = crate::simd_ops::atomic_xchg_u8(rbx.wrapping_add(P3), ((rax as u8)) as u8); rax = (rax & !0xffu64) | (((tmp_xchg) as u8 as u64)); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 4; continue; } rcx = rsp.wrapping_add(0x40); rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); bb = 1; continue; }
            4 => { rcx = ((rbx.wrapping_add(P5)) as u64) as u64; rdx = 0x10; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rax, rdi); rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; crate::simd_ops::wu64(rax.wrapping_add(P7), rcx); rax = ((rax).wrapping_add(crate::simd_ops::ru64(rax.wrapping_add(P8)))) as u64; let tmp_xchg: u64 = crate::simd_ops::atomic_xchg_u64(rbx.wrapping_add(P9), rax); rax = (tmp_xchg) as u64; let _ = crate::simd_ops::atomic_add_u32(rbx.wrapping_add(P10), (1) as u32); crate::simd_ops::wu8(rbx.wrapping_add(P3), (0x0u8) as u8); rcx = ((rbx.wrapping_add(P11)) as u64) as u64; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P12)) as u64, rax, rcx, rdx, r8, r9, rsp); let _ = crate::simd_ops::atomic_add_u32(rbx.wrapping_add(0x258u64), (1) as u32); rcx = rbx.wrapping_add(0x250); rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P13)) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = rbx.wrapping_add(0x248); rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P14)) as u64, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_42793<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64, const P9: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rdi = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    r8 = P1;
    rbx = rdx;
    rcx = rbx;
    r9 = ((rdi.wrapping_add(P2)) as u64) as u64;
    rdx = ((rdi.wrapping_add(P3)) as u64) as u64;
    rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp);
    rdx = ((rdi.wrapping_add(P5)) as u64) as u64;
    crate::simd_ops::wu64(rdi.wrapping_add(P6), rax);
    r8 = P7;
    rcx = rbx;
    rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp);
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    crate::simd_ops::wu64(rdi.wrapping_add(P9), rax);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_42799<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rdi = (crate::simd_ops::ru64(rcx)) as u64;
    r8 = P0;
    rbx = rdx;
    rcx = rbx;
    r9 = ((rdi.wrapping_add(P1)) as u64) as u64;
    rdx = ((rdi.wrapping_add(P2)) as u64) as u64;
    rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp);
    rdx = ((rdi.wrapping_add(P4)) as u64) as u64;
    crate::simd_ops::wu64(rdi.wrapping_add(P5), rax);
    r8 = P6;
    rcx = rbx;
    rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp);
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    crate::simd_ops::wu64(rdi.wrapping_add(P8), rax);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_42863<const P0: u64, const P1: u64, const P2: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    crate::simd_ops::forward_incoming_to_outgoing();
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rdx = (crate::simd_ops::ru32(P0)) as u32 as u64;
    r9 = 0;
    fl_l = 0; fl_r = 0;
    rcx = (crate::simd_ops::ru64(P1)) as u64;
    r8 = (0x100000001u64) as u64;
    crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8(P2, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_42875<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l) = (crate::simd_ops::get_rax_in(), 0u64);
    rax = 0x1;
    let xa_src: u64 = rax; let xa_old: u64 = crate::simd_ops::atomic_xadd_u64(P0, xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64;
    rax = (xa_old) as u64;
    fl_l = ((rax).wrapping_add(1)) as u64;
    rax = ((fl_l as u64)) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_42913<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    crate::simd_ops::wu32(rax.wrapping_add(P0), (0x0u32) as u32);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_42952<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    xmm0 = (crate::simd_ops::ru128(rcx)) as u128;
    xmm1 = ((crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64) as u128;
    crate::simd_ops::wu128(rsp, (xmm0) as u128);
    rcx = (crate::simd_ops::ru64(rsp)) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), ((xmm1 as u64)) as u64);
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x10u64))) as u64;
    crate::simd_ops::wu8(rax.wrapping_add(rcx), (0x0u8) as u8);
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_42972<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    rcx = rsp.wrapping_add(0x30);
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    fl_l = rax; fl_r = rax;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { xmm0 = ((crate::simd_ops::ru32(rax.wrapping_add(P1))) as u32) as u128; rsp = ((rsp).wrapping_add(0x28u64)) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
    xmm0 = 0u128;
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_43032<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rcx;
    rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P0)) as u64, rax, rcx, rdx, r8, r9, rsp);
    rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P1))) as u64;
    crate::simd_ops::wu64(rbx.wrapping_add(P1), (0x0u64) as u64);
    fl_l = rdx; fl_r = rdx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); }
    rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp);
    rcx = rbx;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return crate::simd_ops::tail_stk((crate::simd_ops::ru64(P4)) as u64, rax, rcx, rdx, r8, r9, rsp);
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_43052<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(P0))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 1; continue; } fl_l = (crate::simd_ops::ru16(rcx.wrapping_add(P1))) as u16 as u64; fl_r = (0x0u16) as u16 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { bb = 2; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu16(rcx.wrapping_add(P1), ((rdx as u16)) as u16); bb = 2; continue; }
            2 => { return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_43053<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rbx = rcx; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 8; continue; } rcx = ((crate::simd_ops::ru8(rbx.wrapping_add(P1))) as u32) as u32 as u64; rdi = ((rbx.wrapping_add(P2)) as u64) as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x80; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 1; continue; } rcx = ((rcx.wrapping_mul(8)) as u32) as u32 as u64; bb = 2; continue; }
            1 => { rcx = (((rcx as u32)).wrapping_shl(8u32)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x7c00; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(P3))) as u32 as u64; rax = ((((rax as u32)) & (0xffffffu32))) as u32 as u64; rax = (((rax as u32)).wrapping_shl(3u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; rax = rdi; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 3; continue; } rax = (crate::simd_ops::ru64(rdi)) as u64; bb = 3; continue; }
            3 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); rcx = rsp.wrapping_add(0x20); rax = (crate::simd_ops::ru32(rbx.wrapping_add(P4))) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax); rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 7; continue; } rcx = ((crate::simd_ops::ru8(rbx.wrapping_add(P1))) as u32) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x80; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 4; continue; } rcx = ((rcx.wrapping_mul(8)) as u32) as u32 as u64; bb = 5; continue; }
            4 => { rcx = (((rcx as u32)).wrapping_shl(8u32)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x7c00; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(P3))) as u32 as u64; rax = ((((rax as u32)) & (0xffffffu32))) as u32 as u64; rax = (((rax as u32)).wrapping_shl(3u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 6; continue; } rdi = (crate::simd_ops::ru64(rdi)) as u64; bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(P4))) as u32 as u64; rcx = rsp.wrapping_add(0x20); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); rcx = rax; rdx = (crate::simd_ops::ru64(rax)) as u64; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(rdx.wrapping_add(P7))) as u64, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 8; continue; } bb = 7; continue; }
            7 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            8 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_43055<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(P0))) as u8 as u64; fl_r = (0x0u8) as u8 as u64;
    rdi = rdx;
    rbx = rcx;
    if (((fl_l as u8)) == ((fl_r as u8))) { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    rdx = P1;
    rcx = rdi;
    rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp);
    fl_l = rax; fl_r = rax;
    if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = rbx; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(rax.wrapping_add(P7))) as u64, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    fl_l = (crate::simd_ops::ru16(rax.wrapping_add(P3))) as u16 as u64; fl_r = (0x0u16) as u16 as u64;
    if (((fl_l as u16)) == ((fl_r as u16))) { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = rbx; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(rax.wrapping_add(P7))) as u64, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    rcx = ((rax.wrapping_add(P4)) as u64) as u64;
    fl_l = rcx; fl_r = rcx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { r9 = (crate::simd_ops::ru64(rdi)) as u64; rdx = ((rbx.wrapping_add(P5)) as u64) as u64; r8 = P1; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); }
    rax = (crate::simd_ops::ru64(rbx)) as u64;
    rcx = rbx;
    rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(rax.wrapping_add(P7))) as u64, rax, rcx, rdx, r8, r9, rsp);
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_43083<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let mut rax: u64 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rax = (crate::simd_ops::ru32(rax.wrapping_add(P1))) as u32 as u64;
    rax = (((rax as u32)).wrapping_shr(4u32)) as u32 as u64;
    rax = (rax & !0xffu64) | ((((((rax as u8)) & (0x1u8))) as u8 as u64));
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_43090<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 21>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x68u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (0x4000004u64) as u64); crate::simd_ops::wu8(rsp.wrapping_add(0x38u64), (0x0u8) as u8); rax = P0; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (0x20u64) as u64); r9 = 0x20; r8 = 0; fl_l = 0; fl_r = 0; rdx = rsp.wrapping_add(0x20); rcx = rsp.wrapping_add(0x30); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru32(rsp.wrapping_add(0x34u64))) as u32 as u64; rcx = (crate::simd_ops::ru32(rsp.wrapping_add(0x30u64))) as u32 as u64; rax = ((rcx as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_shr(24u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x80; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 1; continue; } rax = ((rax.wrapping_mul(8)) as u32) as u32 as u64; bb = 2; continue; }
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
pub unsafe extern "win64" fn tpl_43091<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 21>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x68u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (0x4000004u64) as u64); crate::simd_ops::wu8(rsp.wrapping_add(0x38u64), (0x0u8) as u8); rax = P0; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (0x21u64) as u64); r9 = 0x21; r8 = 0; fl_l = 0; fl_r = 0; rdx = rsp.wrapping_add(0x20); rcx = rsp.wrapping_add(0x30); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru32(rsp.wrapping_add(0x34u64))) as u32 as u64; rcx = (crate::simd_ops::ru32(rsp.wrapping_add(0x30u64))) as u32 as u64; rax = ((rcx as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_shr(24u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x80; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 1; continue; } rax = ((rax.wrapping_mul(8)) as u32) as u32 as u64; bb = 2; continue; }
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
pub unsafe extern "win64" fn tpl_43092<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 21>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x68u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (0x4000004u64) as u64); crate::simd_ops::wu8(rsp.wrapping_add(0x38u64), (0x0u8) as u8); rax = P0; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (0x23u64) as u64); r9 = 0x23; r8 = 0; fl_l = 0; fl_r = 0; rdx = rsp.wrapping_add(0x20); rcx = rsp.wrapping_add(0x30); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru32(rsp.wrapping_add(0x34u64))) as u32 as u64; rcx = (crate::simd_ops::ru32(rsp.wrapping_add(0x30u64))) as u32 as u64; rax = ((rcx as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_shr(24u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x80; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 1; continue; } rax = ((rax.wrapping_mul(8)) as u32) as u32 as u64; bb = 2; continue; }
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
pub unsafe extern "win64" fn tpl_43093<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 21>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x68u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (0x4000004u64) as u64); crate::simd_ops::wu8(rsp.wrapping_add(0x38u64), (0x0u8) as u8); rax = P0; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (0x1fu64) as u64); r9 = 0x1f; r8 = 0; fl_l = 0; fl_r = 0; rdx = rsp.wrapping_add(0x20); rcx = rsp.wrapping_add(0x30); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru32(rsp.wrapping_add(0x34u64))) as u32 as u64; rcx = (crate::simd_ops::ru32(rsp.wrapping_add(0x30u64))) as u32 as u64; rax = ((rcx as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_shr(24u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x80; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 1; continue; } rax = ((rax.wrapping_mul(8)) as u32) as u32 as u64; bb = 2; continue; }
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
pub unsafe extern "win64" fn tpl_43094<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 21>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x68u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (0x4000004u64) as u64); crate::simd_ops::wu8(rsp.wrapping_add(0x38u64), (0x0u8) as u8); rax = P0; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (0x28u64) as u64); r9 = 0x28; r8 = 0; fl_l = 0; fl_r = 0; rdx = rsp.wrapping_add(0x20); rcx = rsp.wrapping_add(0x30); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru32(rsp.wrapping_add(0x34u64))) as u32 as u64; rcx = (crate::simd_ops::ru32(rsp.wrapping_add(0x30u64))) as u32 as u64; rax = ((rcx as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_shr(24u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x80; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 1; continue; } rax = ((rax.wrapping_mul(8)) as u32) as u32 as u64; bb = 2; continue; }
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
pub unsafe extern "win64" fn tpl_43097<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 21>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x68u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (0x4000004u64) as u64); crate::simd_ops::wu8(rsp.wrapping_add(0x38u64), (0x0u8) as u8); rax = P0; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (0x26u64) as u64); r9 = 0x26; r8 = 0; fl_l = 0; fl_r = 0; rdx = rsp.wrapping_add(0x20); rcx = rsp.wrapping_add(0x30); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru32(rsp.wrapping_add(0x34u64))) as u32 as u64; rcx = (crate::simd_ops::ru32(rsp.wrapping_add(0x30u64))) as u32 as u64; rax = ((rcx as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_shr(24u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x80; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 1; continue; } rax = ((rax.wrapping_mul(8)) as u32) as u32 as u64; bb = 2; continue; }
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
pub unsafe extern "win64" fn tpl_43127<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; fl_r = 0xffffffff;
    if !((((fl_l as u32)) == ((fl_r as u32)))) { rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rcx.wrapping_add(P0), (0xffffffffu32) as u32); crate::simd_ops::wu32(rcx.wrapping_add(P1), ((rax as u32)) as u32); crate::simd_ops::wu32(rcx.wrapping_add(P2), ((rax as u32)) as u32); crate::simd_ops::wu8(rcx.wrapping_add(P3), ((rax as u8)) as u8); crate::simd_ops::wu32(rcx.wrapping_add(P4), ((rax as u32)) as u32); }
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_43130<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (0u64, 0u64, 0u64);
    fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; fl_r = 0xf;
    rax = (rax & !0xffu64) | (((if (((fl_l as u32)) > ((fl_r as u32))) { 1u8 } else { 0u8 }) as u8 as u64));
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_43145<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu64(rdx, rax);
    crate::simd_ops::wu32(rdx.wrapping_add(P0), ((rax as u32)) as u32);
    rax = rdx;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_43181<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64, const P9: u64, const P10: u64, const P11: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x50u64)) as u64; xmm1 = (crate::simd_ops::ru128(P0)) as u128; xmm0 = 0u128; crate::simd_ops::wu32(rsp.wrapping_add(0x2cu64), ((xmm0 as u32)) as u32); crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm1) as u128); crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm1) as u128); crate::simd_ops::wu64(rdx, (0x3f800000u64) as u64); rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rdx.wrapping_add(P1), ((rax as u32)) as u32); rdi = rdx; rbx = rcx; xmm0 = ((crate::simd_ops::ru64(rsp.wrapping_add(0x2cu64))) as u64) as u128; crate::simd_ops::wu64(rdx.wrapping_add(P2), ((xmm0 as u64)) as u64); xmm0 = ((crate::simd_ops::ru64(rsp.wrapping_add(0x44u64))) as u64) as u128; xmm2 = (xmm1) as u128; crate::simd_ops::wu64(rdx.wrapping_add(P3), ((xmm0 as u64)) as u64); crate::simd_ops::wu64(rdx.wrapping_add(P4), (((xmm1 >> 64) as u64)) as u64); crate::simd_ops::wu32(rdx.wrapping_add(P5), (((xmm1 >> 96) as u32) as u32) as u32); crate::simd_ops::wu32(rdx.wrapping_add(P6), (((xmm1 >> 32) as u32) as u32) as u32); crate::simd_ops::wu32(rdx.wrapping_add(P7), ((xmm2 as u32)) as u32); fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(P8))) as u64; fl_l = (crate::simd_ops::ru8(rax.wrapping_add(P9))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 2; continue; } rdx = ((rbx.wrapping_add(P10)) as u64) as u64; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P11, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = (crate::simd_ops::ru64(rbx.wrapping_add(0x78u64))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 1; continue; } bb = 2; continue; }
            2 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rax = rdi; rsp = ((rsp).wrapping_add(0x50u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_43182<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64, const P9: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 19>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { r11 = rsp; crate::simd_ops::wu64(r11.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x90u64)) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rdi = rdx; rbx = rcx; fl_l = (crate::simd_ops::ru8(rax.wrapping_add(P1))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 3; continue; } xmm2 = (crate::simd_ops::ru128(P2)) as u128; xmm0 = ((crate::simd_ops::ru32(P3)) as u32) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x70u64), (xmm2) as u128); crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((xmm0 as u32)) as u32); xmm1 = 0u128; crate::simd_ops::wu128(r11.wrapping_sub(0x18u64), (xmm2) as u128); xmm0 = ((crate::simd_ops::ru64(rsp.wrapping_add(0x6cu64))) as u64) as u128; xmm0 = (xmm0 & !0xffff_ffffu128) | (xmm1 & 0xffff_ffffu128); crate::simd_ops::wu64(rsp.wrapping_add(0x3cu64), ((xmm0 as u64)) as u64); xmm0 = ((crate::simd_ops::ru64(r11.wrapping_sub(0x14u64))) as u64) as u128; xmm3 = (xmm2) as u128; crate::simd_ops::wu64(rsp.wrapping_add(0x54u64), ((xmm0 as u64)) as u64); crate::simd_ops::wu32(rsp.wrapping_add(0x34u64), ((xmm1 as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x38u64), ((xmm1 as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x44u64), (((xmm2 >> 32) as u32) as u32) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), (((xmm2 >> 64) as u64)) as u64); crate::simd_ops::wu32(rsp.wrapping_add(0x50u64), ((xmm3 as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x5cu64), (((xmm2 >> 96) as u32) as u32) as u32); bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(P0))) as u64; fl_l = (crate::simd_ops::ru8(rax.wrapping_add(P1))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 2; continue; } rdx = ((rbx.wrapping_add(P4)) as u64) as u64; rcx = rsp.wrapping_add(0x30); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = (crate::simd_ops::ru64(rbx.wrapping_add(P6))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 1; continue; } bb = 2; continue; }
            2 => { xmm0 = ((crate::simd_ops::ru64(rsp.wrapping_add(0x54u64))) as u64) as u128; rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x5cu64))) as u32 as u64; bb = 4; continue; }
            3 => { xmm0 = ((crate::simd_ops::ru64(rcx.wrapping_add(P7))) as u64) as u128; rax = (crate::simd_ops::ru32(rcx.wrapping_add(P8))) as u32 as u64; bb = 4; continue; }
            4 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0xa0u64))) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((rax as u32)) as u32); rax = rsp.wrapping_add(0x20); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), ((xmm0 as u64)) as u64); xmm0 = ((crate::simd_ops::ru64(rax)) as u64) as u128; rax = (crate::simd_ops::ru32(rax.wrapping_add(P9))) as u32 as u64; crate::simd_ops::wu64(rdi, ((xmm0 as u64)) as u64); crate::simd_ops::wu32(rdi.wrapping_add(P9), ((rax as u32)) as u32); rax = rdi; rsp = ((rsp).wrapping_add(0x90u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_43216<const P0: u64, const P1: u64, const P2: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    xmm0 = ((crate::simd_ops::ru64(rdx)) as u64) as u128;
    crate::simd_ops::wu64(rcx.wrapping_add(P0), ((xmm0 as u64)) as u64);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P1))) as u32 as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(P2), ((rax as u32)) as u32);
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_43236<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    crate::simd_ops::forward_incoming_to_outgoing();
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    fl_l = rcx; fl_r = rcx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rax = (crate::simd_ops::ru64(rcx)) as u64; r8 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_43242<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x60u64)) as u64;
    rbx = (crate::simd_ops::ru32(P0)) as u32 as u64;
    rdi = rcx;
    rcx = (crate::simd_ops::ru64(P1)) as u64;
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64, rax, rcx, rdx, r8, r9, rsp);
    rcx = P3;
    crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rdi);
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rcx);
    rdx = rsp.wrapping_add(0x20);
    rcx = rsp.wrapping_add(0x20);
    r9 = ((rbx as u32)) as u32 as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rcx);
    r8 = rax;
    rcx = rdi;
    rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp);
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64;
    rsp = ((rsp).wrapping_add(0x60u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_43244<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    rdi = ((rdx.wrapping_add(P1)) as u64) as u64;
    rbx = ((rbx).wrapping_sub(crate::simd_ops::ru64(r8))) as u64;
    rax = (crate::simd_ops::ru32(rdi)) as u32 as u64;
    rbx = ((rbx).wrapping_add(rdi)) as u64;
    crate::simd_ops::wu32(rbx, ((rax as u32)) as u32);
    rcx = ((rbx.wrapping_add(P2)) as u64) as u64;
    rax = (crate::simd_ops::ru32(rdi.wrapping_add(P3))) as u32 as u64;
    crate::simd_ops::wu32(rbx.wrapping_add(P3), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rdi.wrapping_add(P0))) as u32 as u64;
    crate::simd_ops::wu32(rbx.wrapping_add(P0), ((rax as u32)) as u32);
    rdx = (crate::simd_ops::ru32(rdi.wrapping_add(P4))) as u32 as u64;
    rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp);
    r8 = (crate::simd_ops::ru32(rdi.wrapping_add(P4))) as u32 as u64;
    rdx = (crate::simd_ops::ru64(rdi.wrapping_add(P2))) as u64;
    rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P2))) as u64;
    r8 = ((r8).wrapping_shl(3u32)) as u64;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return crate::simd_ops::tail_stk(P6, rax, rcx, rdx, r8, r9, rsp);
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_43248<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rdx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    rax = 0x4;
    let tmp_xchg: u32 = crate::simd_ops::atomic_xchg_u32(rdx.wrapping_add(P1), ((rax as u32)) as u32);
    rax = (tmp_xchg) as u32 as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_43249<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = 0x4;
    let tmp_xchg: u32 = crate::simd_ops::atomic_xchg_u32(rcx.wrapping_add(P0), ((rax as u32)) as u32);
    rax = (tmp_xchg) as u32 as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_43250<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rdx = (crate::simd_ops::ru64(rcx)) as u64;
    rax = 0x4;
    let tmp_xchg: u32 = crate::simd_ops::atomic_xchg_u32(rdx.wrapping_add(P0), ((rax as u32)) as u32);
    rax = (tmp_xchg) as u32 as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_43251<const P0: u64, const P1: u64, const P2: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    r8 = ((rcx.wrapping_add(P0)) as u64) as u64;
    rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P1))) as u64;
    rax = (crate::simd_ops::ru32(r8)) as u32 as u64;
    fl_l = rcx; fl_r = (crate::simd_ops::ru64(rdx)) as u64;
    rcx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(r8), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(r8.wrapping_add(P2))) as u32 as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(r8).wrapping_add(P2), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru64(r8.wrapping_add(P1))) as u64;
    crate::simd_ops::wu64(rcx.wrapping_add(r8).wrapping_add(P1), rax);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_43252<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    rcx = rdx;
    r10 = rdx;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rax = 0;
    fl_l = 0; fl_r = 0;
    r11 = P1;
    crate::simd_ops::wu64(r10, r11);
    crate::simd_ops::wu64(r10.wrapping_add(P2), rax);
    crate::simd_ops::wu64(r10.wrapping_add(P3), rax);
    crate::simd_ops::wu64(r10.wrapping_add(P4), rax);
    crate::simd_ops::wu64(r10.wrapping_add(P5), rax);
    rax = r10;
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_43259<const P0: u64, const P1: u64, const P2: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rcx;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rcx = ((rbx.wrapping_add(P1)) as u64) as u64;
    rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp);
    rcx = rbx;
    rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_43260<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    r8 = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu64(rcx.wrapping_add(P0), r8);
    crate::simd_ops::wu64(rcx, r8);
    rax = (crate::simd_ops::ru64(rdx)) as u64;
    crate::simd_ops::wu64(rcx, rax);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(P0), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P1))) as u32 as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(P1), ((rax as u32)) as u32);
    crate::simd_ops::wu64(rdx, r8);
    crate::simd_ops::wu64(rdx.wrapping_add(P0), r8);
    crate::simd_ops::wu64(rcx.wrapping_add(P2), r8);
    crate::simd_ops::wu64(rcx.wrapping_add(P3), r8);
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(P2))) as u64;
    crate::simd_ops::wu64(rcx.wrapping_add(P2), rax);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P3))) as u32 as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(P3), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P4))) as u32 as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(P4), ((rax as u32)) as u32);
    rax = rcx;
    crate::simd_ops::wu64(rdx.wrapping_add(P2), r8);
    crate::simd_ops::wu64(rdx.wrapping_add(P3), r8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_43270<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r11, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 0>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { r11 = rsp; r8 = (((rdx as u8)) as u32) as u32 as u64; r9 = rcx; rax = (0xcbf29ce484222325u64) as u64; r8 = (((r8) ^ (rax))) as u64; rcx = (0x100000001b3u64) as u64; r8 = (((r8) as i64).wrapping_mul((rcx) as i64) as u64) as u64; crate::simd_ops::wu64(r11.wrapping_add(0x10u64), rdx); rax = ((crate::simd_ops::ru8(r11.wrapping_add(0x11u64))) as u32) as u32 as u64; r8 = (((r8) ^ (rax))) as u64; rax = ((crate::simd_ops::ru8(r11.wrapping_add(0x12u64))) as u32) as u32 as u64; r8 = (((r8) as i64).wrapping_mul((rcx) as i64) as u64) as u64; r8 = (((r8) ^ (rax))) as u64; rax = ((crate::simd_ops::ru8(r11.wrapping_add(0x13u64))) as u32) as u32 as u64; r8 = (((r8) as i64).wrapping_mul((rcx) as i64) as u64) as u64; r8 = (((r8) ^ (rax))) as u64; rax = ((crate::simd_ops::ru8(r11.wrapping_add(0x14u64))) as u32) as u32 as u64; r8 = (((r8) as i64).wrapping_mul((rcx) as i64) as u64) as u64; r8 = (((r8) ^ (rax))) as u64; rax = ((crate::simd_ops::ru8(r11.wrapping_add(0x15u64))) as u32) as u32 as u64; r8 = (((r8) as i64).wrapping_mul((rcx) as i64) as u64) as u64; r8 = (((r8) ^ (rax))) as u64; rax = ((crate::simd_ops::ru8(r11.wrapping_add(0x16u64))) as u32) as u32 as u64; r8 = (((r8) as i64).wrapping_mul((rcx) as i64) as u64) as u64; r8 = (((r8) ^ (rax))) as u64; rax = ((crate::simd_ops::ru8(r11.wrapping_add(0x17u64))) as u32) as u32 as u64; r8 = (((r8) as i64).wrapping_mul((rcx) as i64) as u64) as u64; r8 = (((r8) ^ (rax))) as u64; r8 = (((r8) as i64).wrapping_mul((rcx) as i64) as u64) as u64; rcx = (crate::simd_ops::ru64(r9.wrapping_add(P0))) as u64; rcx = (((rcx) & (r8))) as u64; r8 = (crate::simd_ops::ru64(r9.wrapping_add(P1))) as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(r9.wrapping_add(P2)))) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(P3))) as u64; fl_l = rax; fl_r = r8; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 2; continue; } rcx = (crate::simd_ops::ru64(rcx)) as u64; fl_l = rdx; fl_r = (crate::simd_ops::ru64(rax.wrapping_add(P4))) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 3; continue; } bb = 1; continue; }
            1 => { fl_l = rax; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 2; continue; } rax = (crate::simd_ops::ru64(rax.wrapping_add(P3))) as u64; fl_l = rdx; fl_r = (crate::simd_ops::ru64(rax.wrapping_add(P4))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } bb = 3; continue; }
            2 => { rax = 0; fl_l = 0; fl_r = 0; bb = 3; continue; }
            3 => { fl_l = rax; fl_r = rax; rax = (if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { r8 } else { rax }) as u64; fl_l = rax; fl_r = r8; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 4; continue; } rax = 0; fl_l = 0; fl_r = 0; return rax; }
            4 => { rax = (crate::simd_ops::ru64(rax.wrapping_add(P5))) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(P4))) as u64; return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_43289<const P0: u64, const P1: u64, const P2: u64, const P3: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    rcx = rdx;
    r10 = rdx;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rax = 0;
    fl_l = 0; fl_r = 0;
    r11 = P1;
    crate::simd_ops::wu64(r10, r11);
    crate::simd_ops::wu64(r10.wrapping_add(P2), rax);
    crate::simd_ops::wu64(r10.wrapping_add(P3), rax);
    rax = r10;
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_43302<const P0: u64, const P1: u64, const P2: u64, const P3: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), r8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x30u64)) as u64;
    rdi = rdx;
    r8 = rsp.wrapping_add(0x50);
    rdx = rsp.wrapping_add(0x20);
    rbx = rcx;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rax = (crate::simd_ops::ru64(rbx.wrapping_add(P1))) as u64;
    rax = ((rax).wrapping_add(crate::simd_ops::ru64(rbx))) as u64;
    fl_l = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; fl_r = rax;
    if !((((fl_l as u64)) == ((fl_r as u64)))) { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64; rax = (crate::simd_ops::ru32(rax.wrapping_add(P3))) as u32 as u64; crate::simd_ops::wu64(rdi.wrapping_add(P2), rcx); crate::simd_ops::wu64(rdi, rax); rax = rdi; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64;
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu64(rdi.wrapping_add(P2), rax);
    crate::simd_ops::wu64(rdi, rax);
    rax = rdi;
    rsp = ((rsp).wrapping_add(0x30u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_43316<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rbx = (0xcbf29ce484222325u64) as u64; rcx = P0; rsi = r8; rdi = rdx; r8 = rbx; rdx = P1; rbp = (0x100000001b3u64) as u64; bb = 1; continue; }
            1 => { rax = (((crate::simd_ops::ru8(rcx)) as i8 as i64) as u64) as u64; rcx = ((rcx).wrapping_add(1)) as u64; r8 = (((r8) ^ (rax))) as u64; r8 = (((r8) as i64).wrapping_mul((rbp) as i64) as u64) as u64; fl_l = rcx; fl_r = rdx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } rdx = rsi; rcx = rdi; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); r8 = rax; rcx = rsp.wrapping_add(0x20); rdx = rdi; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rdx = P4; rcx = P5; bb = 2; continue; }
            2 => { rax = (((crate::simd_ops::ru8(rcx)) as i8 as i64) as u64) as u64; rcx = ((rcx).wrapping_add(1)) as u64; rbx = (((rbx) ^ (rax))) as u64; rbx = (((rbx) as i64).wrapping_mul((rbp) as i64) as u64) as u64; fl_l = rcx; fl_r = rdx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } r8 = rbx; rdx = rsi; rcx = rdi; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; rdx = ((rcx.wrapping_add(rdx.wrapping_mul(4))) as u64) as u64; fl_l = rcx; fl_r = rdx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 4; continue; } bb = 3; continue; }
            3 => { fl_l = (crate::simd_ops::ru32(rcx)) as u32 as u64; fl_r = 0xffffffff; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 6; continue; } rcx = ((rcx).wrapping_add(0x4u64)) as u64; fl_l = rcx; fl_r = rdx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } bb = 4; continue; }
            4 => { crate::simd_ops::wu8(rax, (0x0u8) as u8); bb = 5; continue; }
            5 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            6 => { crate::simd_ops::wu8(rax, (0x1u8) as u8); bb = 5; continue; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_43319<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64;
    r9 = r8;
    rax = (crate::simd_ops::ru32(rcx.wrapping_add(P1))) as u32 as u64;
    r10 = rdx;
    rbx = ((rbx).wrapping_shl(32u32)) as u64;
    rdx = P2;
    fl_l = (((rbx) | (rax))) as u64; fl_r = fl_l;
    rbx = ((fl_l as u64)) as u64;
    rcx = P3;
    r8 = (0xcbf29ce484222325u64) as u64;
    r11 = (0x100000001b3u64) as u64;
    loop { rax = (((crate::simd_ops::ru8(rcx)) as i8 as i64) as u64) as u64; rcx = ((rcx).wrapping_add(1)) as u64; r8 = (((r8) ^ (rax))) as u64; r8 = (((r8) as i64).wrapping_mul((r11) as i64) as u64) as u64; fl_l = rcx; fl_r = rdx; if !((((fl_l as u64)) != ((fl_r as u64)))) { break; } }
    rdx = r9;
    rcx = r10;
    rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp);
    crate::simd_ops::wu64(rax, rbx);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_43320<const P0: u64, const P1: u64, const P2: u64, const P3: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64;
    r9 = r8;
    r10 = rdx;
    rcx = P1;
    r8 = (0xcbf29ce484222325u64) as u64;
    rdx = P2;
    r11 = (0x100000001b3u64) as u64;
    loop { rax = (((crate::simd_ops::ru8(rcx)) as i8 as i64) as u64) as u64; rcx = ((rcx).wrapping_add(1)) as u64; r8 = (((r8) ^ (rax))) as u64; r8 = (((r8) as i64).wrapping_mul((r11) as i64) as u64) as u64; fl_l = rcx; fl_r = rdx; if !((((fl_l as u64)) != ((fl_r as u64)))) { break; } }
    rdx = r9;
    rcx = r10;
    rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp);
    crate::simd_ops::wu32(rax, ((rbx as u32)) as u32);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_43356<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64, const P9: u64, const P10: u64, const P11: u64, const P12: u64, const P13: u64, const P14: u64, const P15: u64, const P16: u64, const P17: u64, const P18: u64, const P19: u64, const P20: u64, const P21: u64, const P22: u64, const P23: u64, const P24: u64, const P25: u64, const P26: u64, const P27: u64, const P28: u64, const P29: u64, const P30: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut xmm6: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 23>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        core::sync::atomic::fence(core::sync::atomic::Ordering::SeqCst);
        match bb {
            0 => { rax = rsp; crate::simd_ops::wu64(rax.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rax.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rax.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x90u64)) as u64; crate::simd_ops::wu128(rax.wrapping_sub(0x38u64), (xmm6) as u128); rsi = r8; rdi = rdx; r12 = rcx; rbx = (0xcbf29ce484222325u64) as u64; r8 = rbx; rcx = P0; r9 = P1; r14 = (0x100000001b3u64) as u64; bb = 1; continue; }
            1 => { rax = (((crate::simd_ops::ru8(r9)) as i8 as i64) as u64) as u64; r8 = (((r8) ^ (rax))) as u64; r8 = (((r8) as i64).wrapping_mul((r14) as i64) as u64) as u64; r9 = ((r9).wrapping_add(1)) as u64; fl_l = r9; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } rdx = rsi; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbp = rax; r8 = rbx; rdx = P3; rcx = P4; bb = 2; continue; }
            2 => { rax = (((crate::simd_ops::ru8(rcx)) as i8 as i64) as u64) as u64; r8 = (((r8) ^ (rax))) as u64; r8 = (((r8) as i64).wrapping_mul((r14) as i64) as u64) as u64; rcx = ((rcx).wrapping_add(1)) as u64; fl_l = rcx; fl_r = rdx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } rdx = rsi; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru32(rax)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0xd8u64), ((rax as u32)) as u32); r8 = rbx; rdx = P6; rcx = P7; bb = 3; continue; }
            3 => { rax = (((crate::simd_ops::ru8(rcx)) as i8 as i64) as u64) as u64; r8 = (((r8) ^ (rax))) as u64; r8 = (((r8) as i64).wrapping_mul((r14) as i64) as u64) as u64; rcx = ((rcx).wrapping_add(1)) as u64; fl_l = rcx; fl_r = rdx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } rdx = rsi; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru32(rax)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((rax as u32)) as u32); r8 = rbx; rdx = P8; rcx = P9; bb = 4; continue; }
            4 => { rax = (((crate::simd_ops::ru8(rcx)) as i8 as i64) as u64) as u64; r8 = (((r8) ^ (rax))) as u64; r8 = (((r8) as i64).wrapping_mul((r14) as i64) as u64) as u64; rcx = ((rcx).wrapping_add(1)) as u64; fl_l = rcx; fl_r = rdx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 4; continue; } rdx = rsi; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru32(rax)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x34u64), ((rax as u32)) as u32); r8 = rbx; rdx = P10; rcx = P11; bb = 5; continue; }
            5 => { rax = (((crate::simd_ops::ru8(rcx)) as i8 as i64) as u64) as u64; r8 = (((r8) ^ (rax))) as u64; r8 = (((r8) as i64).wrapping_mul((r14) as i64) as u64) as u64; rcx = ((rcx).wrapping_add(1)) as u64; fl_l = rcx; fl_r = rdx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 5; continue; } rdx = rsi; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r13 = (crate::simd_ops::ru32(rax)) as u32 as u64; r9 = rbx; rdx = P12; rcx = P13; bb = 6; continue; }
            6 => { rax = (((crate::simd_ops::ru8(rcx)) as i8 as i64) as u64) as u64; r9 = (((r9) ^ (rax))) as u64; r9 = (((r9) as i64).wrapping_mul((r14) as i64) as u64) as u64; rcx = ((rcx).wrapping_add(1)) as u64; fl_l = rcx; fl_r = rdx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 6; continue; } r8 = rsi; rdx = rdi; rcx = rsp.wrapping_add(0x40); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P14, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = P15; rcx = P16; bb = 7; continue; }
            7 => { rax = (((crate::simd_ops::ru8(rcx)) as i8 as i64) as u64) as u64; rbx = (((rbx) ^ (rax))) as u64; rbx = (((rbx) as i64).wrapping_mul((r14) as i64) as u64) as u64; rcx = ((rcx).wrapping_add(1)) as u64; fl_l = rcx; fl_r = rdx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 7; continue; } r8 = rbx; rdx = rsi; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P17, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r14 = rax; fl_l = (crate::simd_ops::ru64(rbp)) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 8; continue; } fl_l = (crate::simd_ops::ru64(rbp.wrapping_add(0x8u64))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 15; continue; } bb = 8; continue; }
            8 => { fl_l = ((r13 as u32)) as u32 as u64; fl_r = ((r13 as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 15; continue; } r8 = rbp; rdx = rdi; rcx = rsp.wrapping_add(0x60); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P18, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r15 = rax; xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x40u64))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x50u64), (xmm0) as u128); rax = rsp.wrapping_add(0x50); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); r8 = rdi; rdx = r12; rcx = rsp.wrapping_add(0x70); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P19, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm6 = (crate::simd_ops::ru128(rax)) as u128; r12 = (crate::simd_ops::ru64(r12.wrapping_add(0x330u64))) as u64; rbp = (crate::simd_ops::ru64(P20)) as u64; rsi = 0xffffffff; fl_l = rbp; fl_r = rbp; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } rbx = rbp.wrapping_add(0x18); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rbx); (rax, xmm0) = crate::simd_ops::call_stk_xmm((crate::simd_ops::ru64(P21)) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdi = ((rax as u32)) as u32 as u64; r8 = (crate::simd_ops::ru32(rbx.wrapping_add(0x8u64))) as u32 as u64; fl_l = ((r8 as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 9; continue; } rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm((crate::simd_ops::ru64(P22)) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu32(rbx.wrapping_add(0x8u64), ((rdi as u32)) as u32); bb = 9; continue; }
            9 => { crate::simd_ops::lk_add_u32(rbx.wrapping_add(0xcu64), (1) as u32); rdx = r12; rcx = rbp; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P23, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdi = rax; rdx = 0x40; rcx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P24, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = rax; rax = P25; crate::simd_ops::wu64(rbx, rax); crate::simd_ops::wu128(rbx.wrapping_add(0x8u64), (xmm6) as u128); rdx = (crate::simd_ops::ru64(r15)) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x18u64), rdx); fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } rax = 0x1; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rdx.wrapping_add(0x24u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; rax = (((rax as u32)).wrapping_add(1)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 10; continue; } rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x28u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } (rax, xmm0) = crate::simd_ops::call_stk_xmm(P26, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 10; continue; }
            10 => { rax = (crate::simd_ops::ru32(r15.wrapping_add(0x8u64))) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x20u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(r15.wrapping_add(0xcu64))) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x24u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rsp.wrapping_add(0xd8u64))) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x2cu64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x30u64))) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x30u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x34u64))) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x34u64), ((rax as u32)) as u32); rax = P27; crate::simd_ops::wu64(rbx, rax); crate::simd_ops::wu32(rbx.wrapping_add(0x38u64), ((r13 as u32)) as u32); rdx = (crate::simd_ops::ru32(rdi.wrapping_add(0x30u64))) as u32 as u64; rcx = (crate::simd_ops::ru32(rdi.wrapping_add(0x34u64))) as u32 as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rax = ((rdx as u32)) as u32 as u64; rax = ((rax).wrapping_shl(4u32)) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 11; continue; } rdx = ((rdx).wrapping_add(1)) as u64; rcx = rdi.wrapping_add(0x28); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P28, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 11; continue; }
            11 => { rcx = (crate::simd_ops::ru32(rdi.wrapping_add(0x30u64))) as u32 as u64; rcx = ((rcx).wrapping_add(rcx)) as u64; rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x28u64))) as u64; crate::simd_ops::wu64(rax.wrapping_add(rcx.wrapping_mul(8)), rbx); rdx = P29; crate::simd_ops::wu64(rax.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x8u64), rdx); crate::simd_ops::lk_add_u32(rdi.wrapping_add(0x30u64), (1) as u32); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; fl_l = (crate::simd_ops::lk_add_u32(rax.wrapping_add(0xcu64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; if (((fl_l as u32)) != 0) { bb = 13; continue; } rcx = ((rsi as u32)) as u32 as u64; let tmp_xchg: u32 = crate::simd_ops::atomic_xchg_u32(rax.wrapping_add(0x8u64), ((rcx as u32)) as u32); rcx = (tmp_xchg) as u32 as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm((crate::simd_ops::ru64(P30)) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 13; continue; }
            12 => { rbx = 0; fl_l = 0; fl_r = 0; bb = 13; continue; }
            13 => { rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 14; continue; } let xa_src: u32 = (rsi as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rdx.wrapping_add(0x24u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rsi = (xa_old) as u32 as u64; fl_l = ((rsi as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 14; continue; } rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x28u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 14; continue; } (rax, xmm0) = crate::simd_ops::call_stk_xmm(P26, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 14; continue; }
            14 => { crate::simd_ops::wu64(r14, rbx); bb = 16; continue; }
            15 => { rbx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rax, rbx); bb = 16; continue; }
            16 => { r11 = rsp.wrapping_add(0x90); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(r11.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x40u64))) as u64; xmm6 = (crate::simd_ops::ru128(r11.wrapping_sub(0x10u64))) as u128; rsp = r11; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_43363<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = (0xcbf29ce484222325u64) as u64; rcx = P0; rbp = r8; rsi = rdx; r8 = rbx; rdx = P1; r14 = (0x100000001b3u64) as u64; bb = 1; continue; }
            1 => { rax = (((crate::simd_ops::ru8(rcx)) as i8 as i64) as u64) as u64; rcx = ((rcx).wrapping_add(1)) as u64; r8 = (((r8) ^ (rax))) as u64; r8 = (((r8) as i64).wrapping_mul((r14) as i64) as u64) as u64; fl_l = rcx; fl_r = rdx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } rdx = rbp; rcx = rsi; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rdx = P3; rcx = P4; rdi = (crate::simd_ops::ru64(rax)) as u64; bb = 2; continue; }
            2 => { rax = (((crate::simd_ops::ru8(rcx)) as i8 as i64) as u64) as u64; rcx = ((rcx).wrapping_add(1)) as u64; rbx = (((rbx) ^ (rax))) as u64; rbx = (((rbx) as i64).wrapping_mul((r14) as i64) as u64) as u64; fl_l = rcx; fl_r = rdx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } r8 = rbx; rdx = rbp; rcx = rsi; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rdx = (crate::simd_ops::ru64(rsi)) as u64; r9 = rax; rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xb0u64))) as u64; rdx = (crate::simd_ops::ru32(rdx.wrapping_add(0xb8u64))) as u32 as u64; rdx = ((rdx).wrapping_shl(4u32)) as u64; rdx = ((rdx).wrapping_add(rcx)) as u64; fl_l = rcx; fl_r = rdx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 4; continue; } bb = 3; continue; }
            3 => { fl_l = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 7; continue; } rcx = ((rcx).wrapping_add(0x10u64)) as u64; fl_l = rcx; fl_r = rdx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } bb = 4; continue; }
            4 => { rax = 0; fl_l = 0; fl_r = 0; bb = 5; continue; }
            5 => { fl_l = r9; fl_r = r9; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } r8 = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; r8 = ((r8).wrapping_shl(4u32)) as u64; rax = ((r9.wrapping_add(r9.wrapping_mul(4))) as u64) as u64; r8 = ((r8).wrapping_add(crate::simd_ops::ru64(rsi.wrapping_add(0x10u64)))) as u64; rdx = rsp.wrapping_add(0x48); rax = ((rax).wrapping_shl(4u32)) as u64; rax = ((rax).wrapping_add(crate::simd_ops::ru64(rsi.wrapping_add(0x10u64)))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rax); rcx = (((crate::simd_ops::ru8(r8.wrapping_add(0x40u64))) as i8 as i64) as u64) as u64; rcx = ((rcx).wrapping_add(1)) as u64; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); bb = 6; continue; }
            6 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            7 => { rax = (crate::simd_ops::ru64(rcx)) as u64; bb = 5; continue; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_43481<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64, const P9: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        core::sync::atomic::fence(core::sync::atomic::Ordering::SeqCst);
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rbx = rdx; rdi = rcx; rdx = (crate::simd_ops::ru64(rdx)) as u64; rax = crate::simd_ops::call_stk_caught(P0, rax, rcx, rdx, r8, r9, rsp); r8 = (crate::simd_ops::ru64(rax)) as u64; rax = ((r8.wrapping_add(P1)) as u64) as u64; fl_l = rax; fl_r = (0xfffffffffffffffdu64) as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 1; continue; } crate::simd_ops::wu8(rsp.wrapping_add(0x20u64), (0x0u8) as u8); rdx = P2; rcx = (crate::simd_ops::ru64(P3)) as u64; rax = crate::simd_ops::call_stk_caught(P4, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rax); rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rdi.wrapping_add(P5)))) as u64; rdx = rsp.wrapping_add(0x58); rax = crate::simd_ops::call_stk_caught(P6, rax, rcx, rdx, r8, r9, rsp); bb = 2; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), (0x0u64) as u64); rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rdi.wrapping_add(P5)))) as u64; rdx = rsp.wrapping_add(0x58); rax = crate::simd_ops::call_stk_caught(P6, rax, rcx, rdx, r8, r9, rsp); bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } rcx = 0xffffffff; let xa_src: u32 = (rcx as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rax.wrapping_add(P7), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rcx = (xa_old) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 3; continue; } rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P8))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } rax = crate::simd_ops::call_stk_caught(P9, rax, rcx, rdx, r8, r9, rsp); bb = 3; continue; }
            3 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_43509<const P0: u64, const P1: u64, const P2: u64, const P3: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P1))) as u32 as u64;
    fl_l = rcx; fl_r = (crate::simd_ops::ru64(r8)) as u64;
    rcx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(rdx).wrapping_add(P1), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P2))) as u32 as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(rdx).wrapping_add(P2), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P3))) as u32 as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(rdx).wrapping_add(P3), ((rax as u32)) as u32);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_43520<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64, const P9: u64, const P10: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = rdx; rbx = (crate::simd_ops::ru64(P0)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rbx); rcx = rbx; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P1)) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = ((rbx.wrapping_add(P2)) as u64) as u64; r9 = (crate::simd_ops::ru64(rdi.wrapping_add(P2))) as u64; rax = ((crate::simd_ops::ru16(r9.wrapping_add(P3))) as u32) as u32 as u64; crate::simd_ops::lk_sub_u64(rcx.wrapping_add(P4), (rax) as u64); crate::simd_ops::lk_sub_u64(rcx.wrapping_add(P5), (1) as u64); rax = ((crate::simd_ops::ru16(r9.wrapping_add(P3))) as u32) as u32 as u64; r8 = 0; fl_l = 0; fl_r = 0; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 1; continue; } rax = (((rax as u32)).wrapping_sub(1)) as u32 as u64; let bit_in: u32 = (rax as u32); if bit_in != 0 { rdx = (((31u32.wrapping_sub(bit_in.leading_zeros())) as u32)) as u32 as u64; } fl_l = bit_in as u64; if (((fl_l as u32)) == 0) { bb = 1; continue; } r8 = ((rdx.wrapping_add(P6)) as u32) as u32 as u64; bb = 1; continue; }
            1 => { rdx = r9; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); rcx = ((rbx.wrapping_add(P8)) as u64) as u64; rdx = rdi; rax = crate::simd_ops::call_stk(P9, rax, rcx, rdx, r8, r9, rsp); rcx = rbx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk((crate::simd_ops::ru64(P10)) as u64, rax, rcx, rdx, r8, r9, rsp); }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_43569<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (0u64, 0u64, 0u64);
    rcx = (crate::simd_ops::ru64(rdx)) as u64;
    fl_l = rcx; fl_r = rcx;
    if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); return rax; }
    rax = ((crate::simd_ops::ru16(rcx.wrapping_add(P0))) as u32) as u32 as u64;
    rax = (rax & !0xffffu64) | (((((rax as u16)).wrapping_shr(3u32)) as u16 as u64));
    fl_l = (crate::simd_ops::ru16(rdx.wrapping_add(P1))) as u16 as u64; fl_r = ((rax as u16)) as u16 as u64;
    if (((fl_l as u16)) != ((fl_r as u16))) { rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); return rax; }
    rax = (crate::simd_ops::ru32(rcx)) as u32 as u64;
    fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64;
    if !(((((fl_l as u32)) & ((fl_r as u32))) == 0)) { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; return rax; }
    rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64));
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_43573<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (0u64, 0u64, 0u64);
    rax = (crate::simd_ops::ru64(P0)) as u64;
    fl_l = rdx; fl_r = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64;
    rax = (rax & !0xffu64) | (((if (((fl_l as u64)) <= ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64));
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_43598<const P0: u64, const P1: u64, const P2: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rax = rcx;
    crate::simd_ops::wu32(rcx, (0x1u32) as u32);
    crate::simd_ops::wu64(rcx.wrapping_add(P1), rdx);
    crate::simd_ops::wu64(rcx.wrapping_add(P2), r8);
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_43605<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64, const P9: u64, const P10: u64, const P11: u64, const P12: u64, const P13: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = (crate::simd_ops::ru64(P0)) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 1; continue; } rcx = rbx; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rcx = rbx; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 1; continue; }
            1 => { rbx = (crate::simd_ops::ru64(P3)) as u64; rdi = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(P0, rdi); fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } rcx = rbx; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rcx = rbx; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 2; continue; }
            2 => { rbx = (crate::simd_ops::ru64(P4)) as u64; crate::simd_ops::wu64(P3, rdi); fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } rcx = rbx; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rcx = rbx; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 3; continue; }
            3 => { rbx = (crate::simd_ops::ru64(P5)) as u64; crate::simd_ops::wu64(P4, rdi); fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } rcx = rbx; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rcx = rbx; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 4; continue; }
            4 => { rbx = (crate::simd_ops::ru64(P6)) as u64; crate::simd_ops::wu64(P5, rdi); fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } rcx = rbx; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rcx = rbx; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 5; continue; }
            5 => { rbx = (crate::simd_ops::ru64(P7)) as u64; crate::simd_ops::wu64(P6, rdi); fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } rcx = rbx; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rcx = rbx; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 6; continue; }
            6 => { rbx = (crate::simd_ops::ru64(P8)) as u64; crate::simd_ops::wu64(P7, rdi); fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } rcx = rbx; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rcx = rbx; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 7; continue; }
            7 => { rbx = (crate::simd_ops::ru64(P9)) as u64; crate::simd_ops::wu64(P8, rdi); fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } rcx = rbx; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rcx = rbx; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 8; continue; }
            8 => { rbx = (crate::simd_ops::ru64(P10)) as u64; crate::simd_ops::wu64(P9, rdi); fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } rcx = rbx; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rcx = rbx; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 9; continue; }
            9 => { rbx = (crate::simd_ops::ru64(P11)) as u64; crate::simd_ops::wu64(P10, rdi); fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } rcx = rbx; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rcx = rbx; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 10; continue; }
            10 => { rbx = (crate::simd_ops::ru64(P12)) as u64; crate::simd_ops::wu64(P11, rdi); fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } rcx = rbx; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rcx = rbx; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 11; continue; }
            11 => { rbx = (crate::simd_ops::ru64(P13)) as u64; crate::simd_ops::wu64(P12, rdi); fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } rcx = rbx; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rcx = rbx; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 12; continue; }
            12 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu64(P13, rdi); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_43614<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    rcx = P0;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_43617<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64, const P9: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        core::sync::atomic::fence(core::sync::atomic::Ordering::SeqCst);
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = P0; rdi = rcx; crate::simd_ops::wu64(rcx, rax); rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P1)) as u64, rax, rcx, rdx, r8, r9, rsp); rdx = (crate::simd_ops::ru32(P2)) as u32 as u64; rbx = ((rax as u32)) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 1; continue; } rcx = P3; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P4)) as u64, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu32(P2, ((rbx as u32)) as u32); bb = 1; continue; }
            1 => { rdx = (crate::simd_ops::ru64(P5)) as u64; r8 = (crate::simd_ops::ru32(P6)) as u32 as u64; rbx = rdx; crate::simd_ops::lk_add_u32(P7, (1) as u32); rcx = ((rdx.wrapping_add(r8.wrapping_mul(8))) as u64) as u64; fl_l = rdx; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 3; continue; } rax = rdx; bb = 2; continue; }
            2 => { rbx = rax; fl_l = (crate::simd_ops::ru64(rax)) as u64; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 3; continue; } rbx = rax.wrapping_add(0x8); rax = rbx; fl_l = rbx; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 3; continue; }
            3 => { fl_l = rbx; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 6; continue; } rax = rbx.wrapping_add(0x8); fl_l = rax; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 6; continue; } bb = 4; continue; }
            4 => { rdx = (crate::simd_ops::ru64(rax)) as u64; fl_l = rdx; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 5; continue; } crate::simd_ops::wu64(rbx, rdx); fl_l = ((rbx).wrapping_add(0x8u64)) as u64; fl_r = fl_l; rbx = ((fl_l as u64)) as u64; bb = 5; continue; }
            5 => { rax = ((rax).wrapping_add(0x8u64)) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 4; continue; } r8 = (crate::simd_ops::ru32(P6)) as u32 as u64; rdx = (crate::simd_ops::ru64(P5)) as u64; bb = 6; continue; }
            6 => { rax = ((r8 as u32)) as u32 as u64; r8 = ((rax.wrapping_mul(8)) as u64) as u64; rdi = ((r8.wrapping_add(rdx)) as u64) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 8; continue; } fl_l = rbx; fl_r = rdx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 7; continue; } crate::simd_ops::wu32(P6, (0x0u32) as u32); bb = 8; continue; }
            7 => { r8 = ((r8).wrapping_sub(rdi)) as u64; rcx = rbx; r8 = ((r8).wrapping_add(rdx)) as u64; rdx = rdi; rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); rdi = ((rdi).wrapping_sub(rbx)) as u64; rdi = ((((rdi) as i64).wrapping_shr(3u32) as u64)) as u64; fl_l = (crate::simd_ops::ru32(P6)) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; crate::simd_ops::wu32(P6, ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32); bb = 8; continue; }
            8 => { fl_l = (crate::simd_ops::ru32(P7)) as u32 as u64; fl_r = 0x1; crate::simd_ops::wu32(P7, ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32); if (((fl_l as u32)) != ((fl_r as u32))) { bb = 9; continue; } rax = 0xffffffff; rcx = P3; let tmp_xchg: u32 = crate::simd_ops::atomic_xchg_u32(P2, ((rax as u32)) as u32); rax = (tmp_xchg) as u32 as u64; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P9)) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 9; continue; }
            9 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_43619<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64, const P9: u64, const P10: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        core::sync::atomic::fence(core::sync::atomic::Ordering::SeqCst);
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rcx; rax = P0; crate::simd_ops::wu64(rcx, rax); rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P1)) as u64, rax, rcx, rdx, r8, r9, rsp); rdi = ((rax as u32)) as u32 as u64; rdx = (crate::simd_ops::ru32(P2)) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 1; continue; } rcx = P3; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P4)) as u64, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu32(P2, ((rdi as u32)) as u32); bb = 1; continue; }
            1 => { crate::simd_ops::lk_add_u32(P5, (1) as u32); r9 = (crate::simd_ops::ru32(P6)) as u32 as u64; r8 = (crate::simd_ops::ru64(P7)) as u64; rdx = ((r8.wrapping_add(r9.wrapping_mul(8))) as u64) as u64; rax = (crate::simd_ops::ru32(P8)) as u32 as u64; rcx = ((r8.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; fl_l = rdx; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } rdx = ((rdx).wrapping_sub(r8)) as u64; rdx = ((((rdx) as i64).wrapping_shr(3u32) as u64)) as u64; rdx = ((rdx).wrapping_add(1)) as u64; rax = crate::simd_ops::call_stk(P9, rax, rcx, rdx, r8, r9, rsp); r9 = (crate::simd_ops::ru32(P6)) as u32 as u64; r8 = (crate::simd_ops::ru64(P7)) as u64; bb = 2; continue; }
            2 => { rax = ((r9 as u32)) as u32 as u64; crate::simd_ops::wu64(r8.wrapping_add(rax.wrapping_mul(8)), rbx); crate::simd_ops::lk_add_u32(P6, (1) as u32); fl_l = (crate::simd_ops::ru32(P5)) as u32 as u64; fl_r = 0x1; crate::simd_ops::wu32(P5, ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32); if (((fl_l as u32)) != ((fl_r as u32))) { bb = 3; continue; } rax = 0xffffffff; let tmp_xchg: u32 = crate::simd_ops::atomic_xchg_u32(P2, ((rax as u32)) as u32); rax = (tmp_xchg) as u32 as u64; rcx = P3; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P10)) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 3; continue; }
            3 => { rax = rbx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_43621<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64, const P9: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        core::sync::atomic::fence(core::sync::atomic::Ordering::SeqCst);
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbp = P0; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbp); rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P1)) as u64, rax, rcx, rdx, r8, r9, rsp); rbx = ((rax as u32)) as u32 as u64; rcx = (crate::simd_ops::ru32(P2)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 1; continue; } rcx = rbp; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P3)) as u64, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu32(P2, ((rbx as u32)) as u32); bb = 1; continue; }
            1 => { crate::simd_ops::lk_add_u32(P4, (1) as u32); rbx = (crate::simd_ops::ru64(P5)) as u64; rax = (crate::simd_ops::ru32(P6)) as u32 as u64; rdi = ((rbx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rsi = (0xffffffffffffffffu64) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 3; continue; } bb = 2; continue; }
            2 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; crate::simd_ops::wu64(rcx.wrapping_add(P7), rsi); rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(rax.wrapping_add(P7))) as u64, rax, rcx, rdx, r8, r9, rsp); rbx = ((rbx).wrapping_add(0x8u64)) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 3; continue; }
            3 => { rcx = P5; rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru32(P4)) as u32 as u64; fl_r = 0x1; crate::simd_ops::wu32(P4, ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32); if (((fl_l as u32)) != ((fl_r as u32))) { bb = 4; continue; } let tmp_xchg: u32 = crate::simd_ops::atomic_xchg_u32(P2, ((rsi as u32)) as u32); rsi = (tmp_xchg) as u32 as u64; rcx = rbp; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P9)) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 4; continue; }
            4 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_43622<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64, const P9: u64, const P10: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        core::sync::atomic::fence(core::sync::atomic::Ordering::SeqCst);
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = rcx; rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; crate::simd_ops::wu64(rcx.wrapping_add(P0), rdx); fl_l = rax; fl_r = (0xffffffffffffffffu64) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P1)) as u64, rax, rcx, rdx, r8, r9, rsp); rbx = ((rax as u32)) as u32 as u64; rdx = (crate::simd_ops::ru32(P2)) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 1; continue; } rcx = P3; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P4)) as u64, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu32(P2, ((rbx as u32)) as u32); bb = 1; continue; }
            1 => { crate::simd_ops::lk_add_u32(P5, (1) as u32); r9 = (crate::simd_ops::ru32(P6)) as u32 as u64; r8 = (crate::simd_ops::ru64(P7)) as u64; rdx = ((r8.wrapping_add(r9.wrapping_mul(8))) as u64) as u64; rax = (crate::simd_ops::ru32(P8)) as u32 as u64; rcx = ((r8.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; fl_l = rdx; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } rdx = ((rdx).wrapping_sub(r8)) as u64; rdx = ((((rdx) as i64).wrapping_shr(3u32) as u64)) as u64; rdx = ((rdx).wrapping_add(1)) as u64; rax = crate::simd_ops::call_stk(P9, rax, rcx, rdx, r8, r9, rsp); r9 = (crate::simd_ops::ru32(P6)) as u32 as u64; r8 = (crate::simd_ops::ru64(P7)) as u64; bb = 2; continue; }
            2 => { rax = ((r9 as u32)) as u32 as u64; crate::simd_ops::wu64(r8.wrapping_add(rax.wrapping_mul(8)), rdi); crate::simd_ops::lk_add_u32(P6, (1) as u32); fl_l = (crate::simd_ops::ru32(P5)) as u32 as u64; fl_r = 0x1; crate::simd_ops::wu32(P5, ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32); if (((fl_l as u32)) != ((fl_r as u32))) { bb = 3; continue; } rax = 0xffffffff; let tmp_xchg: u32 = crate::simd_ops::atomic_xchg_u32(P2, ((rax as u32)) as u32); rax = (tmp_xchg) as u32 as u64; rcx = P3; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P10)) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 3; continue; }
            3 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_43623<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        core::sync::atomic::fence(core::sync::atomic::Ordering::SeqCst);
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbp = rcx; r14 = P0; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), r14); rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P1)) as u64, rax, rcx, rdx, r8, r9, rsp); rbx = ((rax as u32)) as u32 as u64; rdx = (crate::simd_ops::ru32(P2)) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 1; continue; } rcx = r14; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P3)) as u64, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu32(P2, ((rbx as u32)) as u32); bb = 1; continue; }
            1 => { r8 = (crate::simd_ops::ru32(P4)) as u32 as u64; r8 = (((r8 as u32)).wrapping_add(1)) as u32 as u64; crate::simd_ops::wu32(P4, ((r8 as u32)) as u32); rbx = 0; fl_l = 0; fl_r = 0; r9 = (crate::simd_ops::ru32(P5)) as u32 as u64; rdx = (crate::simd_ops::ru64(P6)) as u64; rcx = ((rdx.wrapping_add(r9.wrapping_mul(8))) as u64) as u64; rcx = ((rcx).wrapping_sub(rdx)) as u64; rcx = ((((rcx) as i64).wrapping_shr(3u32) as u64)) as u64; rsi = rbx.wrapping_sub(0x1); fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 4; continue; } bb = 2; continue; }
            2 => { rax = ((((rbx as u32)) as i32 as i64) as u64) as u64; rdi = (crate::simd_ops::ru64(rdx.wrapping_add(rax.wrapping_mul(8)))) as u64; fl_l = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; fl_r = rbp; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 3; continue; } rcx = ((rdx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rax = ((r9 as u32)) as u32 as u64; r8 = ((rdx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rdx = rcx.wrapping_add(0x8); r8 = ((r8).wrapping_sub(rdx)) as u64; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::lk_sub_u32(P5, (1) as u32); rbx = (((rbx as u32)).wrapping_sub(1)) as u32 as u64; crate::simd_ops::wu64(rdi.wrapping_add(0x8u64), rsi); rax = (crate::simd_ops::ru64(rdi)) as u64; rcx = rdi; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64, rax, rcx, rdx, r8, r9, rsp); r9 = (crate::simd_ops::ru32(P5)) as u32 as u64; rdx = (crate::simd_ops::ru64(P6)) as u64; bb = 3; continue; }
            3 => { rbx = (((rbx as u32)).wrapping_add(1)) as u32 as u64; rax = ((r9 as u32)) as u32 as u64; rcx = ((rdx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rcx = ((rcx).wrapping_sub(rdx)) as u64; rcx = ((((rcx) as i64).wrapping_shr(3u32) as u64)) as u64; fl_l = ((rbx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 2; continue; } r8 = (crate::simd_ops::ru32(P4)) as u32 as u64; bb = 4; continue; }
            4 => { rax = ((r8.wrapping_sub(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu32(P4, ((rax as u32)) as u32); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 5; continue; } let tmp_xchg: u32 = crate::simd_ops::atomic_xchg_u32(P2, ((rsi as u32)) as u32); rsi = (tmp_xchg) as u32 as u64; rcx = r14; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P8)) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 5; continue; }
            5 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_43654<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64, const P9: u64, const P10: u64, const P11: u64, const P12: u64, const P13: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 21>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0xa0u64)) as u64; rdi = rcx; bb = 1; continue; }
            1 => { xmm0 = (crate::simd_ops::ru128(P0)) as u128; ymm0_hi = 0; rcx = (((xmm0 >> 64) as u64)) as u64; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128); fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } rax = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm((crate::simd_ops::ru64(rax.wrapping_add(0x40u64))) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = rax; fl_l = rax; fl_r = (0xffffffffffffffffu64) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } rdx = 0x887a0005; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 2; continue; }
            2 => { fl_l = rbx; fl_r = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 3; continue; } (rax, xmm0) = crate::simd_ops::call_stk_xmm((crate::simd_ops::ru64(P2)) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 1; continue; }
            3 => { rax = (crate::simd_ops::ru64(rdi)) as u64; rbx = 0; fl_l = 0; fl_r = 0; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } rcx = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } rdx = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } fl_l = (crate::simd_ops::ru64(P3)) as u64; fl_r = rbx; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rcx); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rdx); crate::simd_ops::wu8(rsp.wrapping_add(0x40u64), (0x1u8) as u8); if (((fl_l as u64)) == ((fl_r as u64))) { bb = 5; continue; } xmm0 = 0u128; ymm0_hi = 0; crate::simd_ops::wu64(P4, rcx); rdx = P5; let y_addr: u64 = P6; crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); crate::simd_ops::wu128(P7, (xmm0) as u128); crate::simd_ops::wu8(P8, ((rbx as u8)) as u8); rcx = P3; crate::simd_ops::wu64(P9, rbx); crate::simd_ops::wu64(P10, rbx); (rax, xmm0) = crate::simd_ops::call_stk_xmm((crate::simd_ops::ru64(P11)) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rsp.wrapping_add(0x90u64), rbx); rdx = rsp.wrapping_add(0x50); xmm0 = 0u128; ymm0_hi = 0; let y_addr: u64 = rsp.wrapping_add(0x60u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); crate::simd_ops::wu128(rsp.wrapping_add(0x80u64), (xmm0) as u128); crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), (0x30002u64) as u64); rcx = rsp.wrapping_add(0x38); crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rbx); (rax, xmm0) = crate::simd_ops::call_stk_xmm((crate::simd_ops::ru64(P11)) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 5; continue; }
            4 => { crate::simd_ops::wu8(rsp.wrapping_add(0x40u64), ((rbx as u8)) as u8); bb = 5; continue; }
            5 => { fl_l = (crate::simd_ops::ru64(P12)) as u64; fl_r = rbx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 6; continue; } rdx = 0; fl_l = 0; fl_r = 0; rcx = P12; (rax, xmm0) = crate::simd_ops::call_stk_xmm((crate::simd_ops::ru64(P13)) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(P12, rbx); bb = 6; continue; }
            6 => { fl_l = (crate::simd_ops::ru64(P3)) as u64; fl_r = rbx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 7; continue; } rdx = 0; fl_l = 0; fl_r = 0; rcx = P3; (rax, xmm0) = crate::simd_ops::call_stk_xmm((crate::simd_ops::ru64(P13)) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(P3, rbx); bb = 7; continue; }
            7 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0xb0u64))) as u64; rsp = ((rsp).wrapping_add(0xa0u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_43661(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = ((rcx as u32)) as u32 as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_43666<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64, const P9: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        core::sync::atomic::fence(core::sync::atomic::Ordering::SeqCst);
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rcx = (crate::simd_ops::ru64(P0)) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 1; continue; } rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 1; continue; }
            1 => { rbp = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(P0, rbp); r14 = (crate::simd_ops::ru64(P2)) as u64; fl_l = r14; fl_r = r14; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } rbx = (crate::simd_ops::ru64(r14)) as u64; rax = (crate::simd_ops::ru32(r14.wrapping_add(0x8u64))) as u32 as u64; rdi = ((rbx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 4; continue; } bb = 2; continue; }
            2 => { rsi = (crate::simd_ops::ru64(rbx)) as u64; fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } rax = (crate::simd_ops::ru64(rsi)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = rsi; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(rax)) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = rsi; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 3; continue; }
            3 => { rbx = ((rbx).wrapping_add(0x8u64)) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 4; continue; }
            4 => { crate::simd_ops::wu32(r14.wrapping_add(0x8u64), ((rbp as u32)) as u32); rcx = r14; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rcx = r14; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 5; continue; }
            5 => { crate::simd_ops::wu64(P2, rbp); rbx = (crate::simd_ops::ru64(P4)) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 13; continue; } rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P5)) as u64, rax, rcx, rdx, r8, r9, rsp); rdi = ((rax as u32)) as u32 as u64; rcx = (crate::simd_ops::ru32(rbx.wrapping_add(0x8u64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 6; continue; } rcx = rbx; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P6)) as u64, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu32(rbx.wrapping_add(0x8u64), ((rdi as u32)) as u32); bb = 6; continue; }
            6 => { crate::simd_ops::lk_add_u32(rbx.wrapping_add(0xcu64), (1) as u32); fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x20u64))) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 11; continue; } bb = 7; continue; }
            7 => { r8 = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x18u64))) as u32 as u64; rcx = (crate::simd_ops::ru32(rbx.wrapping_add(0x24u64))) as u32 as u64; rdx = 0; fl_l = 0; fl_r = 0; let d = (rcx) as u128; let n = ((rdx as u128) << 64) | (rax as u128); if d != 0 { rax = (n / d) as u64; rdx = (n % d) as u64; } rdi = (crate::simd_ops::ru64(r8.wrapping_add(rdx.wrapping_mul(8)))) as u64; rcx = (crate::simd_ops::ru64(rdi)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; r8 = 0; fl_l = 0; fl_r = 0; rdx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(rax.wrapping_add(0x48u64))) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rdi)) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } rdx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); bb = 8; continue; }
            8 => { crate::simd_ops::wu64(rdi, rbp); crate::simd_ops::wu64(rdi.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rdi.wrapping_add(0x8u64), rbp); rcx = rdi; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru32(rbx.wrapping_add(0x20u64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 10; continue; } rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x18u64))) as u32 as u64; rax = (((rax as u32)).wrapping_add(1)) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x18u64), ((rax as u32)) as u32); fl_l = ((rax as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rbx.wrapping_add(0x24u64))) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 9; continue; } crate::simd_ops::wu32(rbx.wrapping_add(0x18u64), ((rbp as u32)) as u32); bb = 9; continue; }
            9 => { fl_l = (((rcx as u32)).wrapping_sub(1)) as u32 as u64; rcx = ((fl_l as u32)) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x20u64), ((rcx as u32)) as u32); bb = 10; continue; }
            10 => { crate::simd_ops::lk_sub_u64(rbx.wrapping_add(0x28u64), (1) as u64); fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 7; continue; } bb = 11; continue; }
            11 => { fl_l = (crate::simd_ops::lk_add_u32(rbx.wrapping_add(0xcu64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; if (((fl_l as u32)) != 0) { bb = 12; continue; } rax = 0xffffffff; let tmp_xchg: u32 = crate::simd_ops::atomic_xchg_u32(rbx.wrapping_add(0x8u64), ((rax as u32)) as u32); rax = (tmp_xchg) as u32 as u64; rcx = rbx; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P8)) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 12; continue; }
            12 => { rcx = rbx.wrapping_add(0x10); rax = crate::simd_ops::call_stk(P9, rax, rcx, rdx, r8, r9, rsp); rcx = rbx; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 13; continue; }
            13 => { crate::simd_ops::wu64(P4, rbp); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_43667<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    crate::simd_ops::wu64(P1, rax);
    rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp);
    crate::simd_ops::wu64(P3, rax);
    rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp);
    crate::simd_ops::wu64(P5, rax);
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    let _ = &stack; return crate::simd_ops::tail_stk(P6, rax, rcx, rdx, r8, r9, rsp);
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_43686<const P0: u64, const P1: u64, const P2: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    crate::simd_ops::wu64(P1, rax);
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    let _ = &stack; return crate::simd_ops::tail_stk(P2, rax, rcx, rdx, r8, r9, rsp);
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_43695<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; r13 = rdx; r12 = rcx; rax = (crate::simd_ops::ru64(rdx)) as u64; r15 = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; rcx = r15.wrapping_add(0x38); rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P0)) as u64, rax, rcx, rdx, r8, r9, rsp); rbp = 0; fl_l = 0; fl_r = 0; bb = 1; continue; }
            1 => { r14 = (crate::simd_ops::ru64(r13.wrapping_add(rbp.wrapping_mul(8)))) as u64; rdx = (crate::simd_ops::ru32(r15.wrapping_add(0x20u64))) as u32 as u64; r8 = (crate::simd_ops::ru32(r15.wrapping_add(0x24u64))) as u32 as u64; rcx = ((r8.wrapping_mul(8)) as u64) as u64; rax = ((rdx.wrapping_mul(8)) as u64) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } rax = rdx.wrapping_add(0x1); rdx = ((r8 as u32)) as u32 as u64; rdx = ((rdx).wrapping_shr(1u32)) as u64; rdx = ((rdx).wrapping_add(r8)) as u64; fl_l = rdx; fl_r = rax; rdx = (if (((fl_l as u64)) < ((fl_r as u64))) { rax } else { rdx }) as u64; rcx = r15.wrapping_add(0x18); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 2; continue; }
            2 => { rcx = (crate::simd_ops::ru32(r15.wrapping_add(0x20u64))) as u32 as u64; rax = (crate::simd_ops::ru64(r15.wrapping_add(0x18u64))) as u64; crate::simd_ops::wu64(rax.wrapping_add(rcx.wrapping_mul(8)), r14); crate::simd_ops::lk_add_u32(r15.wrapping_add(0x20u64), (1) as u32); fl_l = (crate::simd_ops::ru64(r14.wrapping_add(0x18u64))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 4; continue; } rdx = (crate::simd_ops::ru32(r15.wrapping_add(0x30u64))) as u32 as u64; rcx = (crate::simd_ops::ru32(r15.wrapping_add(0x34u64))) as u32 as u64; rcx = ((rcx).wrapping_shl(3u32)) as u64; rax = ((rdx.wrapping_mul(8)) as u64) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } rdx = ((rdx).wrapping_add(1)) as u64; rcx = r15.wrapping_add(0x28); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru32(r15.wrapping_add(0x30u64))) as u32 as u64; rax = (crate::simd_ops::ru64(r15.wrapping_add(0x28u64))) as u64; rdx = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; rax = (crate::simd_ops::ru64(r14.wrapping_add(0x18u64))) as u64; crate::simd_ops::wu64(rdx, rax); fl_l = (crate::simd_ops::lk_add_u32(r15.wrapping_add(0x30u64), (1) as u32)) as u32 as u64; bb = 4; continue; }
            4 => { rbp = ((rbp).wrapping_add(1)) as u64; fl_l = rbp; fl_r = r12; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 1; continue; } rcx = r15.wrapping_add(0x38); rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P3)) as u64, rax, rcx, rdx, r8, r9, rsp); rdx = (crate::simd_ops::ru32(r15.wrapping_add(0x30u64))) as u32 as u64; rax = (((crate::simd_ops::ru32(P4)) as i32 as i64) as u64) as u64; fl_l = rdx; fl_r = rax; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 5; continue; } rcx = r15; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); bb = 5; continue; }
            5 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); bb = 6; continue; }
            6 => { return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_43721<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rcx;
    rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    fl_l = rcx; fl_r = rcx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64, rax, rcx, rdx, r8, r9, rsp); }
    crate::simd_ops::wu64(rbx.wrapping_add(P0), (0x0u64) as u64);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_43755<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = (crate::simd_ops::ru64(rdx)) as u64;
    crate::simd_ops::wu64(rcx, rax);
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    crate::simd_ops::wu64(rcx.wrapping_add(P0), rax);
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(P1))) as u64;
    crate::simd_ops::wu64(rcx.wrapping_add(P1), rax);
    rax = rcx;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_43892<const P0: u64, const P1: u64, const P2: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    crate::simd_ops::forward_incoming_to_outgoing();
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rcx = rdx;
    rdx = (crate::simd_ops::ru64(P0)) as u64;
    rdx = (crate::simd_ops::ru64(rdx.wrapping_add(P1))) as u64;
    crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8(P2, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_43901<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = ((((rdx as u32)) as i32 as i64) as u64) as u64;
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(rax.wrapping_mul(8)).wrapping_add(P0))) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_43910<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = (crate::simd_ops::ru64(P0)) as u64;
    rax = ((crate::simd_ops::ru8(rax.wrapping_add(P1))) as u32) as u32 as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_43959<const P0: u64, const P1: u64, const P2: u64, const P3: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x28u64)) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x8007000e; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 1; continue; } rcx = ((rax.wrapping_add(P2)) as u32) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x2; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rdx = ((rax as u32)) as u32 as u64; rsp = ((rsp).wrapping_add(0x28u64)) as u64; return crate::simd_ops::tail_stk(P3, rax, rcx, rdx, r8, r9, rsp); }
            2 => { rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_43987<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = (crate::simd_ops::ru64(rdx)) as u64;
    r8 = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu64(rcx, rax);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(P0), ((rax as u32)) as u32);
    crate::simd_ops::wu64(rcx.wrapping_add(P1), r8);
    crate::simd_ops::wu64(rcx.wrapping_add(P2), r8);
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(P1))) as u64;
    crate::simd_ops::wu64(rcx.wrapping_add(P1), rax);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P2))) as u32 as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(P2), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P3))) as u32 as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(P3), ((rax as u32)) as u32);
    crate::simd_ops::wu64(rdx.wrapping_add(P1), r8);
    crate::simd_ops::wu64(rdx.wrapping_add(P2), r8);
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(P4))) as u64;
    crate::simd_ops::wu64(rcx.wrapping_add(P4), rax);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P5))) as u32 as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(P5), ((rax as u32)) as u32);
    rax = rcx;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_43990<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (0u64, 0u64, 0u64);
    fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(P0))) as u8 as u64; fl_r = (0x0u8) as u8 as u64;
    rax = (rax & !0xffu64) | (((if (((fl_l as u8)) != ((fl_r as u8))) { 1u8 } else { 0u8 }) as u8 as u64));
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_43992<const P0: u64, const P1: u64, const P2: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (0u64, 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(P0))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 1; continue; } rax = (crate::simd_ops::ru32(P1)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 2; continue; } fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(P2))) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 3; continue; } bb = 1; continue; }
            1 => { rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); return rax; }
            2 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x2; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 1; continue; } bb = 3; continue; }
            3 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_43994<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    rax = ((crate::simd_ops::ru8(rax.wrapping_add(P0))) as u32) as u32 as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_44028<const P0: u64, const P1: u64, const P2: u64, const P3: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = (crate::simd_ops::ru64(rdx)) as u64;
    r8 = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu64(rcx, rax);
    crate::simd_ops::wu64(rcx.wrapping_add(P0), r8);
    crate::simd_ops::wu64(rcx.wrapping_add(P1), r8);
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(P1))) as u64;
    crate::simd_ops::wu64(rcx.wrapping_add(P1), rax);
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    crate::simd_ops::wu64(rcx.wrapping_add(P0), rax);
    crate::simd_ops::wu64(rcx.wrapping_add(P2), r8);
    crate::simd_ops::wu64(rcx.wrapping_add(P3), r8);
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(P3))) as u64;
    crate::simd_ops::wu64(rcx.wrapping_add(P3), rax);
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(P2))) as u64;
    crate::simd_ops::wu64(rcx.wrapping_add(P2), rax);
    rax = rcx;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_44029<const P0: u64, const P1: u64, const P2: u64, const P3: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = (crate::simd_ops::ru64(rdx)) as u64;
    crate::simd_ops::wu64(rcx, rax);
    rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P0))) as u32) as u32 as u64;
    crate::simd_ops::wu8(rcx.wrapping_add(P0), ((rax as u8)) as u8);
    rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P1))) as u32) as u32 as u64;
    crate::simd_ops::wu8(rcx.wrapping_add(P1), ((rax as u8)) as u8);
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu64(rcx.wrapping_add(P2), rax);
    crate::simd_ops::wu64(rcx.wrapping_add(P3), rax);
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(P3))) as u64;
    crate::simd_ops::wu64(rcx.wrapping_add(P3), rax);
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(P2))) as u64;
    crate::simd_ops::wu64(rcx.wrapping_add(P2), rax);
    rax = rcx;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_44031<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    crate::simd_ops::forward_incoming_to_outgoing();
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rcx = (crate::simd_ops::ru64(rcx)) as u64;
    fl_l = rcx; fl_r = rcx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rdx = 0x30; crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8(P0, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_44032<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    crate::simd_ops::forward_incoming_to_outgoing();
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rcx = (crate::simd_ops::ru64(rcx)) as u64;
    fl_l = rcx; fl_r = rcx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rdx = 0x28; crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8(P0, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_44037<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    crate::simd_ops::forward_incoming_to_outgoing();
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    fl_l = ((rdx as u8)) as u8 as u64; fl_r = ((rdx as u8)) as u8 as u64;
    if !(((((fl_l as u8)) & ((fl_r as u8))) == 0)) { rdx = 0x28; crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8(P0, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_44064<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { r10 = (crate::simd_ops::ru64(rdx)) as u64; r11 = 0; fl_l = 0; fl_r = 0; r9 = ((r11 as u32)) as u32 as u64; fl_l = r10; fl_r = r10; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } rax = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru8(rax.wrapping_add(P1))) as u8 as u64; fl_r = ((r8 as u8)) as u8 as u64; if (((fl_l as u8)) >= ((fl_r as u8))) { bb = 3; continue; } r9 = ((r9).wrapping_add(1)) as u64; rax = ((rax).wrapping_add(0x20u64)) as u64; fl_l = r9; fl_r = r10; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 1; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::wu64(rcx, r11); rax = rcx; crate::simd_ops::wu64(rcx.wrapping_add(P0), r11); return rax; }
            3 => { fl_l = r10; fl_r = r9; r10 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; crate::simd_ops::wu64(rcx.wrapping_add(P0), rax); crate::simd_ops::wu64(rcx, r10); rax = rcx; return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_44065<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rax = (crate::simd_ops::ru64(rdx)) as u64;
    crate::simd_ops::wu64(rdx, (0x0u64) as u64);
    crate::simd_ops::wu64(rcx, rax);
    rax = rcx;
    xmm0 = (crate::simd_ops::ru128(rdx.wrapping_add(P0))) as u128;
    crate::simd_ops::wu128(rcx.wrapping_add(P0), (xmm0) as u128);
    xmm0 = (crate::simd_ops::ru128(rdx.wrapping_add(P1))) as u128;
    crate::simd_ops::wu128(rcx.wrapping_add(P1), (xmm0) as u128);
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_44066<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rcx;
    crate::simd_ops::wu64(rcx.wrapping_add(P0), (0x0u64) as u64);
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rax = rbx;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_44072<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rcx;
    rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    fl_l = rcx; fl_r = rcx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64, rax, rcx, rdx, r8, r9, rsp); }
    crate::simd_ops::wu64(rbx.wrapping_add(P0), (0x0u64) as u64);
    rax = rbx;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_44078<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = (crate::simd_ops::ru64(rdx)) as u64;
    crate::simd_ops::wu64(rdx, (0x0u64) as u64);
    crate::simd_ops::wu64(rcx, rax);
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    crate::simd_ops::wu64(rcx.wrapping_add(P0), rax);
    rax = rcx;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_44079<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rcx;
    rcx = (crate::simd_ops::ru64(rcx)) as u64;
    fl_l = rcx; fl_r = rcx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64, rax, rcx, rdx, r8, r9, rsp); }
    crate::simd_ops::wu64(rbx, (0x0u64) as u64);
    rax = rbx;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_44086<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    crate::simd_ops::forward_incoming_to_outgoing();
    let (mut rax, mut fl_l) = (crate::simd_ops::get_rax_in(), 0u64);
    rcx = rdx;
    rdx = 0x8;
    fl_l = ((rcx).wrapping_shl(5u32)) as u64;
    rcx = ((fl_l as u64)) as u64;
    crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8(P0, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_44093(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    fl_l = (crate::simd_ops::lk_add_u64(rcx, (0xffffffffffffffe0u64) as u64)) as u64; fl_r = fl_l;
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    crate::simd_ops::wu64(rdx, rax);
    rax = rdx;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_44095(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    fl_l = (crate::simd_ops::lk_add_u64(rcx, (0xffffffffffffffd0u64) as u64)) as u64; fl_r = fl_l;
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    crate::simd_ops::wu64(rdx, rax);
    rax = rdx;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_44097<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64;
    rax = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64;
    rax = ((rax).wrapping_shl(5u32)) as u64;
    fl_l = ((rax).wrapping_add(crate::simd_ops::ru64(rcx))) as u64; fl_r = fl_l;
    rax = ((fl_l as u64)) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_44098<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64;
    rax = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64;
    rax = ((rax).wrapping_shl(4u32)) as u64;
    fl_l = ((rax).wrapping_add(crate::simd_ops::ru64(rcx))) as u64; fl_r = fl_l;
    rax = ((fl_l as u64)) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_44099<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64;
    rax = (((rax) as i64).wrapping_mul((0x38u64) as i64) as u64) as u64;
    fl_l = ((rax).wrapping_add(crate::simd_ops::ru64(rcx))) as u64; fl_r = fl_l;
    rax = ((fl_l as u64)) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_44100(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    fl_l = rax; fl_r = (0x20u64) as u64;
    rax = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_44102(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    fl_l = rax; fl_r = (0x30u64) as u64;
    rax = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_44106<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    crate::simd_ops::wu8(rcx.wrapping_add(P0), (0xffu8) as u8);
    rax = rcx;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_44123<const P0: u64, const P1: u64, const P2: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rcx; fl_l = (crate::simd_ops::ru64(rcx.wrapping_add(0x18u64))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 8; continue; } r15 = 0; fl_l = 0; fl_r = 0; rdi = ((r15 as u32)) as u32 as u64; r14 = ((r15 as u32)) as u32 as u64; bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rsi = rbx.wrapping_add(0x8); fl_l = (crate::simd_ops::ru8(rdi.wrapping_add(rax))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if ((((fl_l as u8)) as i8) < (((fl_r as u8)) as i8)) { bb = 3; continue; } rbp = (crate::simd_ops::ru64(rsi)) as u64; rcx = (crate::simd_ops::ru64(rbp.wrapping_add(r14).wrapping_add(0x18u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 2; continue; }
            2 => { crate::simd_ops::wu64(rbp.wrapping_add(r14).wrapping_add(0x18u64), r15); bb = 3; continue; }
            3 => { rdi = ((rdi).wrapping_add(1)) as u64; r14 = ((r14).wrapping_add(0x40u64)) as u64; fl_l = rdi; fl_r = (crate::simd_ops::ru64(rbx.wrapping_add(0x18u64))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } r9 = (crate::simd_ops::ru64(rbx)) as u64; fl_l = r9; fl_r = r9; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } rcx = r9.wrapping_sub(0x1); rcx = (((rcx) & (0xfffffffffe000000u64))) as u64; rax = r9; rax = ((rax).wrapping_sub(rcx)) as u64; rax = ((rax).wrapping_shr(16u32)) as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rdx = rcx.wrapping_add(0x108); rdx = ((rdx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x4u64))) as u32 as u64; rdx = ((rdx).wrapping_sub(rax)) as u64; rax = ((crate::exe::teb_self() as u64)) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(rcx.wrapping_add(0x100u64))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 6; continue; } fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(0xeu64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 4; continue; } rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(r9, rax); crate::simd_ops::wu64(rdx.wrapping_add(0x20u64), r9); fl_l = (crate::simd_ops::lk_add_u32(rdx.wrapping_add(0x18u64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; if (((fl_l as u32)) != 0) { bb = 5; continue; } rcx = rdx; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rsi = rbx.wrapping_add(0x8); bb = 7; continue; }
            4 => { r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 5; continue; }
            5 => { rsi = rbx.wrapping_add(0x8); bb = 7; continue; }
            6 => { r8 = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 7; continue; }
            7 => { rax = P2; crate::simd_ops::wu64(rbx, rax); crate::simd_ops::wu64(rsi, r15); crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), r15); crate::simd_ops::wu64(rbx.wrapping_add(0x18u64), r15); crate::simd_ops::wu64(rbx.wrapping_add(0x28u64), r15); bb = 8; continue; }
            8 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_44130<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64;
    crate::simd_ops::wu32(rdx.wrapping_add(P1), ((rax as u32)) as u32);
    rax = rdx;
    crate::simd_ops::wu64(rdx, rcx);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_44133<const P0: u64, const P1: u64, const P2: u64, const P3: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut r10) = (crate::simd_ops::get_rax_in(), 0u64);
    r8 = (crate::simd_ops::ru64(rcx)) as u64;
    r10 = rdx;
    rax = (crate::simd_ops::ru64(rdx)) as u64;
    r9 = rcx;
    crate::simd_ops::wu64(rcx, rax);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64;
    crate::simd_ops::wu64(rdx, r8);
    r8 = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(P0), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(r10.wrapping_add(P1))) as u32 as u64;
    crate::simd_ops::wu32(rdx.wrapping_add(P0), ((r8 as u32)) as u32);
    rdx = (crate::simd_ops::ru32(rcx.wrapping_add(P1))) as u32 as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(P1), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(r10.wrapping_add(P2))) as u32 as u64;
    crate::simd_ops::wu32(r10.wrapping_add(P1), ((rdx as u32)) as u32);
    rcx = (crate::simd_ops::ru32(rcx.wrapping_add(P2))) as u32 as u64;
    crate::simd_ops::wu32(r9.wrapping_add(P2), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(r10.wrapping_add(P3))) as u32 as u64;
    crate::simd_ops::wu32(r10.wrapping_add(P2), ((rcx as u32)) as u32);
    rcx = (crate::simd_ops::ru32(r9.wrapping_add(P3))) as u32 as u64;
    crate::simd_ops::wu32(r9.wrapping_add(P3), ((rax as u32)) as u32);
    crate::simd_ops::wu32(r10.wrapping_add(P3), ((rcx as u32)) as u32);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_44142<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l) = (crate::simd_ops::get_rax_in(), 0u64);
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    fl_l = (crate::simd_ops::lk_add_u32(rcx.wrapping_add(P0), (1) as u32)) as u32 as u64;
    r8 = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64;
    crate::simd_ops::wu64(rdx, rax);
    rax = rdx;
    crate::simd_ops::wu32(rdx.wrapping_add(P0), ((r8 as u32)) as u32);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_44143<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (0u64, 0u64, 0u64);
    rax = (crate::simd_ops::ru64(rdx)) as u64;
    r8 = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64;
    fl_l = (crate::simd_ops::ru64(rcx)) as u64; fl_r = rax;
    if (((fl_l as u64)) != ((fl_r as u64))) { rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); return rax; }
    fl_l = ((r8 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64;
    if !((((fl_l as u32)) != ((fl_r as u32)))) { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; return rax; }
    rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64));
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_44159<const P0: u64, const P1: u64, const P2: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    rdx = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64;
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    rax = (crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64;
    rax = ((rax).wrapping_shl(16u32)) as u64;
    fl_l = ((rax).wrapping_add(rdx)) as u64; fl_r = fl_l;
    rax = ((fl_l as u64)) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_44160<const P0: u64, const P1: u64, const P2: u64, const P3: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rcx;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rax = ((rax as u32)) as u32 as u64;
    rdx = P1;
    rdx = (crate::simd_ops::ru64(rdx.wrapping_add(rax.wrapping_mul(8)))) as u64;
    fl_l = rdx; fl_r = rdx;
    if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    fl_l = (crate::simd_ops::ru32(rdx)) as u32 as u64; fl_r = 0x3;
    if !((((fl_l as u32)) != ((fl_r as u32)))) { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P2))) as u64; rcx = ((rcx).wrapping_add(0x30u64)) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(P3, rax, rcx, rdx, r8, r9, rsp); }
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_44161<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64, const P9: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rsi = rcx; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(0x18u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rcx); rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P0)) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = rsi; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rcx = rsi; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rdi = (crate::simd_ops::ru64(rsi.wrapping_add(0x18u64))) as u64; rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x210u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rax); rbp = (crate::simd_ops::ru64(rsi)) as u64; rcx = rsp.wrapping_add(0x40); fl_l = rax; fl_r = rbp; rcx = (if (((fl_l as u64)) >= ((fl_r as u64))) { rsi } else { rcx }) as u64; rbp = ((rbp).wrapping_sub(crate::simd_ops::ru64(rcx))) as u64; rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x100u64))) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(rax.wrapping_add(0x40u64))) as u64, rax, rcx, rdx, r8, r9, rsp); rbx = rax; fl_l = rax; fl_r = (0xffffffffffffffffu64) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } rdx = 0x887a0005; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 1; continue; }
            1 => { rbp = ((rbp).wrapping_shr(25u32)) as u64; r8 = rbx; rdx = rbp; rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x110u64))) as u64; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rbp = rdi.wrapping_add(0x58); rcx = rbp; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P5)) as u64, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 5; continue; } rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0x98u64))) as u64; rcx = (crate::simd_ops::ru32(rdi.wrapping_add(0xa0u64))) as u32 as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rcx = ((rcx).wrapping_add(rdx)) as u64; rax = rdx; fl_l = rdx; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 4; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; fl_r = rbx; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 3; continue; } rax = ((rax).wrapping_add(0x10u64)) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 3; continue; }
            3 => { fl_l = rax; fl_r = rdx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 4; continue; } crate::simd_ops::wu8(rdi.wrapping_add(0xa8u64), (0x1u8) as u8); rdi = (crate::simd_ops::ru64(P6)) as u64; rcx = rdi.wrapping_add(0x250); rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P0)) as u64, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::lk_add_u32(rdi.wrapping_add(0x258u64), (1) as u32); rcx = rdi.wrapping_add(0x250); rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P7)) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = rdi.wrapping_add(0x248); rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P8)) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 4; continue; }
            4 => { rcx = rbp; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P7)) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 5; continue; }
            5 => { rcx = rsi; rax = crate::simd_ops::call_stk(P9, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x18u64))) as u64; crate::simd_ops::wu8(rax.wrapping_add(0x218u64), (0x0u8) as u8); rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk((crate::simd_ops::ru64(P7)) as u64, rax, rcx, rdx, r8, r9, rsp); }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_44165<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 37>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rbp = rsp.wrapping_sub(0x10); rsp = ((rsp).wrapping_sub(0x110u64)) as u64; rbx = rcx; crate::simd_ops::wu64(rcx, (0x40000000u64) as u64); crate::simd_ops::wu64(rcx.wrapping_add(0x8u64), (0x6400000u64) as u64); rax = 0xc0000000; crate::simd_ops::wu64(rcx.wrapping_add(0x10u64), rax); rax = P0; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rcx); rax = P1; crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rcx); rax = 0x100; crate::simd_ops::wu16(rsp.wrapping_add(0x60u64), ((rax as u16)) as u16); crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), (0x400u64) as u64); r8 = rsp.wrapping_add(0x40); rcx = ((rcx).wrapping_add(0x18u64)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu16(rbx.wrapping_add(0x20u64), (0x0u16) as u16); rcx = (crate::simd_ops::ru64(P3)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; r9 = 0x3c; r8 = rbp.wrapping_sub(0x38); rdx = 0; fl_l = 0; fl_r = 0; (rax, xmm0) = crate::simd_ops::call_stk_xmm((crate::simd_ops::ru64(rax.wrapping_add(0x68u64))) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x8007000e; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 1; continue; } rcx = ((rax.wrapping_add(0x7785fffbu64)) as u32) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x2; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rdx = ((rax as u32)) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru32(rbp.wrapping_sub(0x2cu64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 3; continue; } crate::simd_ops::wu8(rbx.wrapping_add(0x20u64), (0x1u8) as u8); bb = 3; continue; }
            3 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x3; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 4; continue; } crate::simd_ops::wu8(rbx.wrapping_add(0x21u64), (0x1u8) as u8); bb = 4; continue; }
            4 => { rax = 0x1; crate::simd_ops::wu64(rsp.wrapping_add(0x78u64), rax); r14 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rbp.wrapping_sub(0x80u64), ((r14 as u32)) as u32); crate::simd_ops::wu32(rbp.wrapping_sub(0x7cu64), ((rax as u32)) as u32); crate::simd_ops::wu32(rbp.wrapping_sub(0x78u64), ((rax as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x40u64), ((rax as u32)) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), r14); crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), (0x1000000u64) as u64); crate::simd_ops::wu32(rsp.wrapping_add(0x58u64), ((rax as u32)) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x5cu64), (0x10001u64) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x64u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x6cu64), rax); let y_addr: u64 = rsp.wrapping_add(0x40u64); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rbp.wrapping_sub(0x70u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); xmm1 = (crate::simd_ops::ru128(rsp.wrapping_add(0x60u64))) as u128; crate::simd_ops::wu128(rbp.wrapping_sub(0x50u64), (xmm1) as u128); xmm0 = ((crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64) as u128; ymm0_hi = 0; crate::simd_ops::wu64(rbp.wrapping_sub(0x40u64), ((xmm0 as u64)) as u64); r10 = (crate::simd_ops::ru64(P3)) as u64; rax = (crate::simd_ops::ru64(r10)) as u64; rsi = (crate::simd_ops::ru64(rax.wrapping_add(0xd8u64))) as u64; rdi = (crate::simd_ops::ru64(rbx.wrapping_add(0x18u64))) as u64; rdi = ((rdi).wrapping_add(0x1a8u64)) as u64; rcx = (crate::simd_ops::ru64(rdi)) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } rax = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm((crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r10 = (crate::simd_ops::ru64(P3)) as u64; bb = 5; continue; }
            5 => { crate::simd_ops::wu64(rdi, r14); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rdi); rax = P5; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), r14); crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), (0x400u32) as u32); r9 = rbp.wrapping_sub(0x70); r8 = 0; fl_l = 0; fl_r = 0; rdx = rsp.wrapping_add(0x78); rcx = r10; (rax, xmm0) = crate::simd_ops::call_stk_xmm(rsi, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x8007000e; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 6; continue; } rcx = ((rax.wrapping_add(0x7785fffbu64)) as u32) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x2; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 7; continue; } bb = 6; continue; }
            6 => { rdx = ((rax as u32)) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 7; continue; }
            7 => { rax = rbx; r11 = rsp.wrapping_add(0x110); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x28u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x30u64))) as u64; rsp = r11; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_44182<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (0u64, 0u64, 0u64);
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    fl_l = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; fl_r = rax;
    rax = (rax & !0xffu64) | (((if (((fl_l as u64)) > ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64));
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_44199<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    r8 = (crate::simd_ops::ru64(rcx)) as u64;
    rax = (crate::simd_ops::ru32(r8.wrapping_add(P0))) as u32 as u64;
    rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64;
    rax = (crate::simd_ops::ru64(r8)) as u64;
    rcx = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64;
    rax = rdx;
    crate::simd_ops::wu64(rdx, rcx);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_44233<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu64(P0, rax);
    crate::simd_ops::wu64(P1, rax);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_44276<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64, const P9: u64, const P10: u64, const P11: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        core::sync::atomic::fence(core::sync::atomic::Ordering::SeqCst);
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = rcx; rsi = ((rcx.wrapping_add(P0)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rsi); rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P1)) as u64, rax, rcx, rdx, r8, r9, rsp); rbp = ((rax as u32)) as u32 as u64; rdx = (crate::simd_ops::ru32(rsi.wrapping_add(P2))) as u32 as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 1; continue; } rcx = rbx; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P3)) as u64, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu32(rsi.wrapping_add(P2), ((rbp as u32)) as u32); bb = 1; continue; }
            1 => { crate::simd_ops::lk_add_u32(rbx.wrapping_add(P4), (1) as u32); rcx = ((rdi.wrapping_add(P5)) as u64) as u64; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); rcx = ((rdi.wrapping_add(P7)) as u64) as u64; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); rcx = ((rdi.wrapping_add(P8)) as u64) as u64; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); rcx = ((rdi.wrapping_add(P9)) as u64) as u64; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); rcx = ((rdi.wrapping_add(P10)) as u64) as u64; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::lk_add_u32(rbx.wrapping_add(P4), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; if (((fl_l as u32)) != 0) { bb = 2; continue; } rax = 0xffffffff; let tmp_xchg: u32 = crate::simd_ops::atomic_xchg_u32(rbx.wrapping_add(P2), ((rax as u32)) as u32); rax = (tmp_xchg) as u32 as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P11)) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 2; continue; }
            2 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_44278<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64, const P9: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        core::sync::atomic::fence(core::sync::atomic::Ordering::SeqCst);
        match bb {
            0 => { fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = rdx; r14 = rcx; rsi = ((rcx.wrapping_add(P0)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rsi); rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P1)) as u64, rax, rcx, rdx, r8, r9, rsp); rbp = ((rax as u32)) as u32 as u64; rdx = (crate::simd_ops::ru32(rsi.wrapping_add(P2))) as u32 as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 1; continue; } rcx = rbx; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P3)) as u64, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu32(rsi.wrapping_add(P2), ((rbp as u32)) as u32); bb = 1; continue; }
            1 => { crate::simd_ops::lk_add_u32(rbx.wrapping_add(P4), (1) as u32); rsi = ((r14.wrapping_add(P5)) as u64) as u64; rdx = (crate::simd_ops::ru32(rsi.wrapping_add(P6))) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rsi.wrapping_add(P7))) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 2; continue; } rdx = (((rdx as u32)).wrapping_add(1)) as u32 as u64; rcx = rsi; rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); bb = 2; continue; }
            2 => { rcx = (crate::simd_ops::ru32(rsi.wrapping_add(P4))) as u32 as u64; rax = (crate::simd_ops::ru64(rsi)) as u64; crate::simd_ops::wu64(rax.wrapping_add(rcx.wrapping_mul(8)), rdi); crate::simd_ops::lk_add_u32(rsi.wrapping_add(P4), (1) as u32); rax = (crate::simd_ops::ru32(rsi.wrapping_add(P4))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rsi.wrapping_add(P7))) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 3; continue; } crate::simd_ops::wu32(rsi.wrapping_add(P4), (0x0u32) as u32); bb = 3; continue; }
            3 => { crate::simd_ops::lk_add_u32(rsi.wrapping_add(P6), (1) as u32); fl_l = (crate::simd_ops::lk_add_u32(rbx.wrapping_add(P4), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; if (((fl_l as u32)) != 0) { bb = 4; continue; } rax = 0xffffffff; let tmp_xchg: u32 = crate::simd_ops::atomic_xchg_u32(rbx.wrapping_add(P2), ((rax as u32)) as u32); rax = (tmp_xchg) as u32 as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P9)) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 4; continue; }
            4 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); bb = 5; continue; }
            5 => { return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_44286<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    crate::simd_ops::forward_incoming_to_outgoing();
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rcx = (crate::simd_ops::ru64(P0)) as u64;
    crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8(P1, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_44293<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = (crate::simd_ops::ru64(P0)) as u64;
    rcx = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64;
    rax = rdx;
    crate::simd_ops::wu64(rdx, rcx);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_44315<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64, const P9: u64, const P10: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        core::sync::atomic::fence(core::sync::atomic::Ordering::SeqCst);
        match bb {
            0 => { fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = rdx; r14 = (crate::simd_ops::ru64(P0)) as u64; rsi = ((r14.wrapping_add(P1)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rsi); rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P2)) as u64, rax, rcx, rdx, r8, r9, rsp); rbp = ((rax as u32)) as u32 as u64; rcx = (crate::simd_ops::ru32(rsi.wrapping_add(P3))) as u32 as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 1; continue; } rcx = rbx; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P4)) as u64, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu32(rsi.wrapping_add(P3), ((rbp as u32)) as u32); bb = 1; continue; }
            1 => { crate::simd_ops::lk_add_u32(rbx.wrapping_add(P5), (1) as u32); rsi = ((r14.wrapping_add(P6)) as u64) as u64; rdx = (crate::simd_ops::ru32(rsi.wrapping_add(P7))) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rsi.wrapping_add(P8))) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 2; continue; } rdx = (((rdx as u32)).wrapping_add(1)) as u32 as u64; rcx = rsi; rax = crate::simd_ops::call_stk(P9, rax, rcx, rdx, r8, r9, rsp); bb = 2; continue; }
            2 => { rcx = (crate::simd_ops::ru32(rsi.wrapping_add(P5))) as u32 as u64; rax = (crate::simd_ops::ru64(rsi)) as u64; crate::simd_ops::wu64(rax.wrapping_add(rcx.wrapping_mul(8)), rdi); crate::simd_ops::lk_add_u32(rsi.wrapping_add(P5), (1) as u32); rax = (crate::simd_ops::ru32(rsi.wrapping_add(P5))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rsi.wrapping_add(P8))) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 3; continue; } crate::simd_ops::wu32(rsi.wrapping_add(P5), (0x0u32) as u32); bb = 3; continue; }
            3 => { crate::simd_ops::lk_add_u32(rsi.wrapping_add(P7), (1) as u32); fl_l = (crate::simd_ops::lk_add_u32(rbx.wrapping_add(P5), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; if (((fl_l as u32)) != 0) { bb = 4; continue; } rax = 0xffffffff; let tmp_xchg: u32 = crate::simd_ops::atomic_xchg_u32(rbx.wrapping_add(P3), ((rax as u32)) as u32); rax = (tmp_xchg) as u32 as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P10)) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 4; continue; }
            4 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); bb = 5; continue; }
            5 => { return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_44316<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        core::sync::atomic::fence(core::sync::atomic::Ordering::SeqCst);
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = rdx; r14 = rcx; rsi = ((rcx.wrapping_add(P0)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rsi); rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P1)) as u64, rax, rcx, rdx, r8, r9, rsp); rbp = ((rax as u32)) as u32 as u64; r8 = (crate::simd_ops::ru32(rsi.wrapping_add(P2))) as u32 as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; fl_l = ((r8 as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 1; continue; } rcx = rbx; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P3)) as u64, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu32(rsi.wrapping_add(P2), ((rbp as u32)) as u32); bb = 1; continue; }
            1 => { crate::simd_ops::lk_add_u32(rbx.wrapping_add(P4), (1) as u32); rcx = ((r14.wrapping_add(P5)) as u64) as u64; rdx = rdi; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::lk_add_u32(rbx.wrapping_add(P4), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; if (((fl_l as u32)) != 0) { bb = 2; continue; } rax = 0xffffffff; let tmp_xchg: u32 = crate::simd_ops::atomic_xchg_u32(rbx.wrapping_add(P2), ((rax as u32)) as u32); rax = (tmp_xchg) as u32 as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P7)) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 2; continue; }
            2 => { rax = rdi; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_44317<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        core::sync::atomic::fence(core::sync::atomic::Ordering::SeqCst);
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = rdx; r14 = (crate::simd_ops::ru64(P0)) as u64; rsi = ((r14.wrapping_add(P1)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rsi); rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P2)) as u64, rax, rcx, rdx, r8, r9, rsp); rbp = ((rax as u32)) as u32 as u64; rcx = (crate::simd_ops::ru32(rsi.wrapping_add(P3))) as u32 as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 1; continue; } rcx = rbx; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P4)) as u64, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu32(rsi.wrapping_add(P3), ((rbp as u32)) as u32); bb = 1; continue; }
            1 => { crate::simd_ops::lk_add_u32(rbx.wrapping_add(P5), (1) as u32); rcx = ((r14.wrapping_add(P6)) as u64) as u64; rdx = rdi; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::lk_add_u32(rbx.wrapping_add(P5), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; if (((fl_l as u32)) != 0) { bb = 2; continue; } rax = 0xffffffff; let tmp_xchg: u32 = crate::simd_ops::atomic_xchg_u32(rbx.wrapping_add(P3), ((rax as u32)) as u32); rax = (tmp_xchg) as u32 as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P8)) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 2; continue; }
            2 => { rax = rdi; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_44330(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = (crate::simd_ops::ru64(r8)) as u64;
    crate::simd_ops::wu64(rdx, rax);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_44332<const P0: u64, const P1: u64, const P2: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64;
    rdx = (crate::simd_ops::ru32(rcx.wrapping_add(P1))) as u32 as u64;
    rax = (((rax as u32)).wrapping_sub(1)) as u32 as u64;
    r8 = (crate::simd_ops::ru32(rcx.wrapping_add(P2))) as u32 as u64;
    rax = ((rax).wrapping_add(rdx)) as u64;
    rdx = 0;
    fl_l = 0; fl_r = 0;
    let d = (r8) as u128; let n = ((rdx as u128) << 64) | (rax as u128);
    if d != 0 { rax = (n / d) as u64; rdx = (n % d) as u64; }
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    rax = ((rax.wrapping_add(rdx.wrapping_mul(8))) as u64) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_44333<const P0: u64, const P1: u64, const P2: u64, const P3: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    r8 = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu64(rcx, r8);
    crate::simd_ops::wu64(rcx.wrapping_add(P0), r8);
    crate::simd_ops::wu64(rcx.wrapping_add(P1), r8);
    fl_l = rcx; fl_r = rdx;
    if !((((fl_l as u64)) == ((fl_r as u64)))) { rax = (crate::simd_ops::ru64(rdx)) as u64; crate::simd_ops::wu64(rcx, rax); rax = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(P0), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdx.wrapping_add(P2))) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(P2), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdx.wrapping_add(P1))) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(P1), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdx.wrapping_add(P3))) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(P3), ((rax as u32)) as u32); crate::simd_ops::wu64(rdx, r8); crate::simd_ops::wu64(rdx.wrapping_add(P0), r8); crate::simd_ops::wu64(rdx.wrapping_add(P1), r8); }
    rax = rcx;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_44339<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64;
    r8 = (crate::simd_ops::ru32(rcx.wrapping_add(P1))) as u32 as u64;
    rax = ((rax).wrapping_add(rdx)) as u64;
    rdx = 0;
    fl_l = 0; fl_r = 0;
    let d = (r8) as u128; let n = ((rdx as u128) << 64) | (rax as u128);
    if d != 0 { rax = (n / d) as u64; rdx = (n % d) as u64; }
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    rax = ((rax.wrapping_add(rdx.wrapping_mul(8))) as u64) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_44340<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    r8 = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64;
    rdx = 0;
    fl_l = 0; fl_r = 0;
    rax = (crate::simd_ops::ru32(rcx.wrapping_add(P1))) as u32 as u64;
    let d = (r8) as u128; let n = ((rdx as u128) << 64) | (rax as u128);
    if d != 0 { rax = (n / d) as u64; rdx = (n % d) as u64; }
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    rax = ((rax.wrapping_add(rdx.wrapping_mul(8))) as u64) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_44341<const P0: u64, const P1: u64, const P2: u64, const P3: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rdi = rdx;
    rbx = rcx;
    rdx = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64;
    fl_l = ((rdx as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rcx.wrapping_add(P1))) as u32 as u64;
    if !((((fl_l as u32)) != ((fl_r as u32)))) { rdx = (((rdx as u32)).wrapping_add(1)) as u32 as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); }
    rax = (crate::simd_ops::ru64(rdi)) as u64;
    rdx = (crate::simd_ops::ru32(rbx.wrapping_add(P3))) as u32 as u64;
    rcx = (crate::simd_ops::ru64(rbx)) as u64;
    crate::simd_ops::wu64(rcx.wrapping_add(rdx.wrapping_mul(8)), rax);
    crate::simd_ops::lk_add_u32(rbx.wrapping_add(P3), (1) as u32);
    crate::simd_ops::lk_add_u32(rbx.wrapping_add(P0), (1) as u32);
    rax = (crate::simd_ops::ru32(rbx.wrapping_add(P3))) as u32 as u64;
    fl_l = ((rax as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rbx.wrapping_add(P1))) as u32 as u64;
    if !((((fl_l as u32)) != ((fl_r as u32)))) { crate::simd_ops::wu32(rbx.wrapping_add(P3), (0x0u32) as u32); }
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_44342<const P0: u64, const P1: u64, const P2: u64, const P3: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rdx = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64;
    fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64;
    if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { return rax; }
    rax = (crate::simd_ops::ru32(rcx.wrapping_add(P1))) as u32 as u64;
    rax = (((rax as u32)).wrapping_add(1)) as u32 as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(P1), ((rax as u32)) as u32);
    fl_l = ((rax as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rcx.wrapping_add(P2))) as u32 as u64;
    if !((((fl_l as u32)) != ((fl_r as u32)))) { crate::simd_ops::wu32(rcx.wrapping_add(P1), (0x0u32) as u32); }
    rax = ((rdx.wrapping_add(P3)) as u32) as u32 as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(P0), ((rax as u32)) as u32);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_44343<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu64(rcx, rax);
    crate::simd_ops::wu64(rcx.wrapping_add(P0), rax);
    crate::simd_ops::wu64(rcx.wrapping_add(P1), rax);
    rax = rcx;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_44344<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    crate::simd_ops::wu64(rdx, rcx);
    rax = rdx;
    crate::simd_ops::wu32(rdx.wrapping_add(P0), (0x0u32) as u32);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_44345<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (0u64, 0u64, 0u64);
    fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; fl_r = 0x0;
    rax = (rax & !0xffu64) | (((if (((fl_l as u32)) == ((fl_r as u32))) { 1u8 } else { 0u8 }) as u8 as u64));
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_44347<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64, const P9: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        core::sync::atomic::fence(core::sync::atomic::Ordering::SeqCst);
        match bb {
            0 => { fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rsi = rdx; rbx = rcx; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rcx); rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P0)) as u64, rax, rcx, rdx, r8, r9, rsp); rdi = ((rax as u32)) as u32 as u64; r8 = (crate::simd_ops::ru32(rbx.wrapping_add(P1))) as u32 as u64; fl_l = ((r8 as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 1; continue; } rcx = rbx; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P2)) as u64, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu32(rbx.wrapping_add(P1), ((rdi as u32)) as u32); bb = 1; continue; }
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
pub unsafe extern "win64" fn tpl_44349<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    r8 = (crate::simd_ops::ru64(rcx)) as u64;
    rdx = 0;
    fl_l = 0; fl_r = 0;
    rcx = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64;
    rax = (crate::simd_ops::ru32(r8.wrapping_add(P0))) as u32 as u64;
    rax = ((rax).wrapping_add(rcx)) as u64;
    rcx = (crate::simd_ops::ru32(r8.wrapping_add(P1))) as u32 as u64;
    let d = (rcx) as u128; let n = ((rdx as u128) << 64) | (rax as u128);
    if d != 0 { rax = (n / d) as u64; rdx = (n % d) as u64; }
    rdx = ((rdx).wrapping_shl(4u32)) as u64;
    fl_l = ((rdx).wrapping_add(crate::simd_ops::ru64(r8))) as u64; fl_r = fl_l;
    rdx = ((fl_l as u64)) as u64;
    rax = rdx;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_44355<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu32(rcx.wrapping_add(P0), (0xffffffffu32) as u32);
    crate::simd_ops::wu32(rcx.wrapping_add(P1), ((rax as u32)) as u32);
    crate::simd_ops::wu64(rcx, rax);
    crate::simd_ops::wu64(rcx.wrapping_add(P2), rax);
    crate::simd_ops::wu64(rcx.wrapping_add(P3), rax);
    crate::simd_ops::wu64(rcx.wrapping_add(P4), rax);
    crate::simd_ops::wu64(rcx.wrapping_add(P5), rax);
    rax = rcx;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_44366<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = ((((rcx as u32)) as i32 as i64) as u64) as u64;
    rcx = P0;
    rax = (crate::simd_ops::ru32(rcx.wrapping_add(rax.wrapping_mul(4)))) as u32 as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_44389<const P0: u64, const P1: u64, const P2: u64, const P3: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = P0;
    crate::simd_ops::wu64(P1, rax);
    rax = P2;
    crate::simd_ops::wu64(P3, rax);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_44468<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (0u64, 0u64, 0u64);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    xmm0 = (crate::simd_ops::ru128(rax)) as u128;
    rax = (crate::simd_ops::ru64(rax)) as u64;
    rdx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64;
    rax = (((xmm0 >> 64) as u64)) as u64;
    rdx = ((rax.wrapping_add(rdx.wrapping_mul(4))) as u64) as u64;
    fl_l = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; fl_r = rdx;
    rax = (rax & !0xffu64) | (((if (((fl_l as u64)) == ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64));
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_44470<const P0: u64, const P1: u64, const P2: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r10) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    crate::simd_ops::wu64(rcx, rdx);
    r10 = rcx;
    crate::simd_ops::wu64(rcx.wrapping_add(P0), r8);
    crate::simd_ops::wu8(rcx.wrapping_add(P1), (0x0u8) as u8);
    rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp);
    rax = r10;
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_44471<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    crate::simd_ops::forward_incoming_to_outgoing();
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    fl_l = (crate::simd_ops::lk_add_u64(rcx.wrapping_add(P0), (0xcu64) as u64)) as u64; fl_r = fl_l;
    crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8(P1, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_44472<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (0u64, 0u64, 0u64);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    xmm0 = (crate::simd_ops::ru128(rax)) as u128;
    rax = (crate::simd_ops::ru64(rax)) as u64;
    rdx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64;
    rax = (((xmm0 >> 64) as u64)) as u64;
    rdx = ((rax.wrapping_add(rdx.wrapping_mul(4))) as u64) as u64;
    fl_l = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; fl_r = rdx;
    rax = (rax & !0xffu64) | (((if (((fl_l as u64)) != ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64));
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_44474<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x8u64))) as u64; rbp = r8; rsi = (crate::simd_ops::ru64(rdx)) as u64; rbx = rdx; rdi = rcx; r9 = 0; fl_l = 0; fl_r = 0; rcx = rax; rdx = ((rax.wrapping_add(rsi.wrapping_mul(2))) as u64) as u64; r8 = rdx; r8 = ((r8).wrapping_sub(rax)) as u64; r8 = ((((r8) as i64).wrapping_shr(1u32) as u64)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; rcx = rax; r9 = ((rax.wrapping_add(rsi.wrapping_mul(2))) as u64) as u64; fl_l = rax; fl_r = r9; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 6; continue; } rax = ((rax).wrapping_add(0x2u64)) as u64; fl_l = rax; fl_r = r9; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 6; continue; } r8 = ((crate::simd_ops::ru16(rcx)) as u32) as u32 as u64; r10 = rax; r11 = rax; bb = 1; continue; }
            1 => { rdx = (((r8 as u16)) as u32) as u32 as u64; r8 = ((crate::simd_ops::ru16(rax)) as u32) as u32 as u64; fl_l = ((rdx as u16)) as u16 as u64; fl_r = ((r8 as u16)) as u16 as u64; if (((fl_l as u16)) == ((fl_r as u16))) { bb = 2; continue; } rcx = rax; rax = r10.wrapping_add(0x2); r10 = rax; r11 = rax; fl_l = rax; fl_r = r9; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } bb = 6; continue; }
            2 => { rax = r11.wrapping_add(0x2); fl_l = rax; fl_r = r9; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 5; continue; } r10 = rax; r11 = rax; bb = 3; continue; }
            3 => { r8 = ((crate::simd_ops::ru16(rax)) as u32) as u32 as u64; fl_l = ((rdx as u16)) as u16 as u64; fl_r = ((r8 as u16)) as u16 as u64; if (((fl_l as u16)) == ((fl_r as u16))) { bb = 4; continue; } fl_l = ((rcx).wrapping_add(0x2u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; rdx = (((r8 as u16)) as u32) as u32 as u64; r10 = r11; crate::simd_ops::wu16(rcx, ((r8 as u16)) as u16); bb = 4; continue; }
            4 => { rax = r10.wrapping_add(0x2); r10 = rax; r11 = rax; fl_l = rax; fl_r = r9; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } bb = 5; continue; }
            5 => { r9 = rcx.wrapping_add(0x2); bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; r8 = rsp.wrapping_add(0x20); r9 = ((r9).wrapping_sub(rax)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax); r9 = ((((r9) as i64).wrapping_shr(1u32) as u64)) as u64; rdx = rsp.wrapping_add(0x30); fl_l = r9; fl_r = (0xffffffffffffffffu64) as u64; rcx = rbp; r9 = (if (((fl_l as u64)) == ((fl_r as u64))) { rsi } else { r9 }) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r9); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; rax = (crate::simd_ops::ru64(rax)) as u64; crate::simd_ops::wu64(rdi, rax); rax = rdi; crate::simd_ops::wu64(rdi.wrapping_add(0x8u64), rcx); rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_44476<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rax = (crate::simd_ops::ru64(rdx)) as u64;
    crate::simd_ops::wu64(rcx, rax);
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    crate::simd_ops::wu64(rcx.wrapping_add(P0), rax);
    rax = rcx;
    xmm0 = (crate::simd_ops::ru128(rdx.wrapping_add(P1))) as u128;
    crate::simd_ops::wu128(rcx.wrapping_add(P1), (xmm0) as u128);
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_44477<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    crate::simd_ops::forward_incoming_to_outgoing();
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rcx = ((rdx.wrapping_mul(4)) as u64) as u64;
    rdx = 0x2;
    crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8(P0, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_44479<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    crate::simd_ops::forward_incoming_to_outgoing();
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    fl_l = (crate::simd_ops::lk_add_u64(rcx.wrapping_add(P0), (0x2u64) as u64)) as u64; fl_r = fl_l;
    crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8(P1, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_44481<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rdx = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64;
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    rax = ((rax.wrapping_add(rdx.wrapping_mul(4))) as u64) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_44488<const P0: u64, const P1: u64, const P2: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    crate::simd_ops::wu8(rcx.wrapping_add(P0), (0xffu8) as u8);
    rax = (((crate::simd_ops::ru8(rdx.wrapping_add(P0))) as i8 as i64) as u64) as u64;
    fl_l = ((rax).wrapping_add(0x1u64)) as u64; fl_r = fl_l;
    rax = ((fl_l as u64)) as u64;
    if (((fl_l as u64)) == 0) { rax = rcx; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
    fl_l = rax; fl_r = (0x1u64) as u64;
    if !((((fl_l as u64)) == ((fl_r as u64)))) { crate::simd_ops::wu8(rcx.wrapping_add(P0), (0x1u8) as u8); rax = rcx; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
    rax = (crate::simd_ops::ru64(rdx)) as u64;
    crate::simd_ops::wu64(rcx, rax);
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(P1))) as u64;
    crate::simd_ops::wu64(rcx.wrapping_add(P1), rax);
    xmm0 = (crate::simd_ops::ru128(rdx.wrapping_add(P2))) as u128;
    crate::simd_ops::wu128(rcx.wrapping_add(P2), (xmm0) as u128);
    crate::simd_ops::wu8(rcx.wrapping_add(P0), (0x0u8) as u8);
    rax = rcx;
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_44489(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    fl_l = (crate::simd_ops::lk_add_u64(rcx, (0xfffffffffffffffcu64) as u64)) as u64; fl_r = fl_l;
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    crate::simd_ops::wu64(rdx, rax);
    rax = rdx;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_44492(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    fl_l = rax; fl_r = (0x4u64) as u64;
    rax = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_44494<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    crate::simd_ops::wu64(rdx.wrapping_add(P0), rax);
    rax = rdx;
    crate::simd_ops::wu64(rdx, rcx);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_44495(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = (crate::simd_ops::ru64(r8)) as u64;
    crate::simd_ops::wu64(rcx, rax);
    rax = rcx;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_44501<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (0u64, 0u64, 0u64);
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    rax = (crate::simd_ops::ru64(rax)) as u64;
    rdx = (crate::simd_ops::ru32(rax.wrapping_add(P0))) as u32 as u64;
    rax = (crate::simd_ops::ru64(rax)) as u64;
    rax = ((rax.wrapping_add(rdx.wrapping_mul(8))) as u64) as u64;
    fl_l = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; fl_r = rax;
    rax = (rax & !0xffu64) | (((if (((fl_l as u64)) == ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64));
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_44503<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = (crate::simd_ops::ru64(r8)) as u64;
    crate::simd_ops::wu64(rcx.wrapping_add(P0), rax);
    rax = rcx;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_44511<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = (crate::simd_ops::ru64(rdx)) as u64;
    crate::simd_ops::wu64(rcx, rax);
    rax = (crate::simd_ops::ru64(r8)) as u64;
    crate::simd_ops::wu64(rcx.wrapping_add(P0), rax);
    rax = rcx;
    crate::simd_ops::wu8(rcx.wrapping_add(P1), (0x1u8) as u8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_44517<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (0u64, 0u64, 0u64);
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    rax = (crate::simd_ops::ru64(rax)) as u64;
    rdx = (crate::simd_ops::ru32(rax.wrapping_add(P0))) as u32 as u64;
    rax = (crate::simd_ops::ru64(rax)) as u64;
    rax = ((rax.wrapping_add(rdx.wrapping_mul(8))) as u64) as u64;
    fl_l = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; fl_r = rax;
    rax = (rax & !0xffu64) | (((if (((fl_l as u64)) != ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64));
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_44520(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = ((crate::simd_ops::ru16(rdx)) as u32) as u32 as u64;
    crate::simd_ops::wu16(rcx, ((rax as u16)) as u16);
    rax = rcx;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_44529<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    crate::simd_ops::wu64(rcx, rdx);
    rax = ((crate::simd_ops::ru16(r8)) as u32) as u32 as u64;
    crate::simd_ops::wu16(rcx.wrapping_add(P0), ((rax as u16)) as u16);
    rax = rcx;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_44556<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = (((rdx as u16)) as u32) as u32 as u64;
    rax = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64;
    rax = ((rax).wrapping_shl(4u32)) as u64;
    fl_l = ((rax).wrapping_add(crate::simd_ops::ru64(rcx.wrapping_add(P0)))) as u64; fl_r = fl_l;
    rax = ((fl_l as u64)) as u64;
    rax = ((crate::simd_ops::ru8(rax.wrapping_add(P1))) as u32) as u32 as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_44566<const P0: u64, const P1: u64, const P2: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rax = (crate::simd_ops::ru64(rdx)) as u64;
    crate::simd_ops::wu64(rcx, rax);
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    crate::simd_ops::wu64(rcx.wrapping_add(P0), rax);
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(P1))) as u64;
    crate::simd_ops::wu64(rcx.wrapping_add(P1), rax);
    rax = rcx;
    xmm0 = (crate::simd_ops::ru128(rdx.wrapping_add(P2))) as u128;
    crate::simd_ops::wu128(rcx.wrapping_add(P2), (xmm0) as u128);
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_44567<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    crate::simd_ops::forward_incoming_to_outgoing();
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    fl_l = (crate::simd_ops::lk_add_u64(rcx.wrapping_add(P0), (0x8u64) as u64)) as u64; fl_r = fl_l;
    crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8(P1, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_44575<const P0: u64, const P1: u64, const P2: u64, const P3: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rax = (((crate::simd_ops::ru8(rdx.wrapping_add(P0))) as i8 as i64) as u64) as u64;
    fl_l = ((rax).wrapping_add(0x1u64)) as u64; fl_r = fl_l;
    rax = ((fl_l as u64)) as u64;
    if (((fl_l as u64)) == 0) { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
    fl_l = rax; fl_r = (0x1u64) as u64;
    if !((((fl_l as u64)) == ((fl_r as u64)))) { rax = ((crate::simd_ops::ru8(rdx)) as u32) as u32 as u64; crate::simd_ops::wu8(rcx, ((rax as u8)) as u8); crate::simd_ops::wu8(rcx.wrapping_add(P0), (0x1u8) as u8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
    rax = (crate::simd_ops::ru64(rdx)) as u64;
    crate::simd_ops::wu64(rcx, rax);
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(P1))) as u64;
    crate::simd_ops::wu64(rcx.wrapping_add(P1), rax);
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(P2))) as u64;
    crate::simd_ops::wu64(rcx.wrapping_add(P2), rax);
    xmm0 = (crate::simd_ops::ru128(rdx.wrapping_add(P3))) as u128;
    crate::simd_ops::wu128(rcx.wrapping_add(P3), (xmm0) as u128);
    crate::simd_ops::wu8(rcx.wrapping_add(P0), (0x0u8) as u8);
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_44576<const P0: u64, const P1: u64, const P2: u64, const P3: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    crate::simd_ops::wu8(rcx.wrapping_add(P0), (0xffu8) as u8);
    rax = (((crate::simd_ops::ru8(rdx.wrapping_add(P0))) as i8 as i64) as u64) as u64;
    fl_l = ((rax).wrapping_add(0x1u64)) as u64; fl_r = fl_l;
    rax = ((fl_l as u64)) as u64;
    if (((fl_l as u64)) == 0) { rax = rcx; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
    fl_l = rax; fl_r = (0x1u64) as u64;
    if !((((fl_l as u64)) == ((fl_r as u64)))) { rax = ((crate::simd_ops::ru8(rdx)) as u32) as u32 as u64; crate::simd_ops::wu8(rcx, ((rax as u8)) as u8); rax = rcx; crate::simd_ops::wu8(rcx.wrapping_add(P0), (0x1u8) as u8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
    rax = (crate::simd_ops::ru64(rdx)) as u64;
    crate::simd_ops::wu64(rcx, rax);
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(P1))) as u64;
    crate::simd_ops::wu64(rcx.wrapping_add(P1), rax);
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(P2))) as u64;
    crate::simd_ops::wu64(rcx.wrapping_add(P2), rax);
    xmm0 = (crate::simd_ops::ru128(rdx.wrapping_add(P3))) as u128;
    crate::simd_ops::wu128(rcx.wrapping_add(P3), (xmm0) as u128);
    crate::simd_ops::wu8(rcx.wrapping_add(P0), (0x0u8) as u8);
    rax = rcx;
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_44577<const P0: u64, const P1: u64, const P2: u64, const P3: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rax = (crate::simd_ops::ru64(rdx)) as u64;
    crate::simd_ops::wu64(rcx, rax);
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    crate::simd_ops::wu64(rcx.wrapping_add(P0), rax);
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(P1))) as u64;
    crate::simd_ops::wu64(rcx.wrapping_add(P1), rax);
    rax = rcx;
    xmm0 = (crate::simd_ops::ru128(rdx.wrapping_add(P2))) as u128;
    crate::simd_ops::wu128(rcx.wrapping_add(P2), (xmm0) as u128);
    crate::simd_ops::wu8(rcx.wrapping_add(P3), (0x0u8) as u8);
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_44578<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = ((crate::simd_ops::ru8(rdx)) as u32) as u32 as u64;
    crate::simd_ops::wu8(rcx, ((rax as u8)) as u8);
    rax = rcx;
    crate::simd_ops::wu8(rcx.wrapping_add(P0), (0x1u8) as u8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_44585<const P0: u64, const P1: u64, const P2: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    crate::simd_ops::wu64(rcx, rdx);
    crate::simd_ops::wu64(rcx.wrapping_add(P0), r8);
    crate::simd_ops::wu8(rcx.wrapping_add(P1), (0x0u8) as u8);
    rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp);
    rax = rcx;
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_44590<const P0: u64, const P1: u64, const P2: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    crate::simd_ops::wu64(rcx, rdx);
    rax = (crate::simd_ops::ru64(r8)) as u64;
    crate::simd_ops::wu64(rcx.wrapping_add(P0), rax);
    rax = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    crate::simd_ops::wu64(rcx.wrapping_add(P1), rax);
    rax = rcx;
    xmm0 = (crate::simd_ops::ru128(r8.wrapping_add(P1))) as u128;
    crate::simd_ops::wu128(rcx.wrapping_add(P2), (xmm0) as u128);
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_44591<const P0: u64, const P1: u64, const P2: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    xmm0 = (crate::simd_ops::ru128(rdx)) as u128;
    xmm1 = ((crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64) as u128;
    crate::simd_ops::wu128(rcx, (xmm0) as u128);
    crate::simd_ops::wu64(rcx.wrapping_add(P0), ((xmm1 as u64)) as u64);
    crate::simd_ops::wu64(rcx.wrapping_add(P1), r8);
    rax = rcx;
    crate::simd_ops::wu8(rcx.wrapping_add(P2), (0x1u8) as u8);
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_44592(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    crate::simd_ops::wu8(rdx, (0x0u8) as u8);
    rax = rdx;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_44601<const P0: u64, const P1: u64, const P2: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rcx;
    rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    fl_l = rcx; fl_r = rcx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); }
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu64(rbx.wrapping_add(P0), rax);
    crate::simd_ops::wu64(rbx.wrapping_add(P2), rax);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_44670<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    crate::simd_ops::forward_incoming_to_outgoing();
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rcx = ((rdx.wrapping_mul(8)) as u64) as u64;
    rdx = 0x4;
    crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8(P0, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_44717<const P0: u64, const P1: u64, const P2: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 0>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    xmm0 = (crate::simd_ops::ru128(rdx)) as u128;
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64;
    crate::simd_ops::wu128(rcx, (xmm0) as u128);
    xmm0 = (crate::simd_ops::ru128(r8)) as u128;
    crate::simd_ops::wu128(rcx.wrapping_add(P0), (xmm0) as u128);
    crate::simd_ops::wu64(rcx.wrapping_add(P1), r9);
    xmm0 = (crate::simd_ops::ru128(rax)) as u128;
    crate::simd_ops::wu128(rcx.wrapping_add(P2), (xmm0) as u128);
    rax = rcx;
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_44755<const P0: u64, const P1: u64, const P2: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = ((rcx.wrapping_add(P0)) as u64) as u64;
    rcx = rbx;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rcx = rbx;
    rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_44774<const P0: u64, const P1: u64, const P2: u64, const P3: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rbx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rsi = rcx; rdi = (crate::simd_ops::ru64(rcx)) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rbx = ((rbx).wrapping_sub(1)) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } rax = (crate::simd_ops::ru64(rdi.wrapping_add(P0))) as u64; rdx = rsp.wrapping_add(0x30); xmm0 = (crate::simd_ops::ru128(rsi.wrapping_add(P1))) as u128; r8 = (crate::simd_ops::ru64(rsi.wrapping_add(P2))) as u64; rcx = rsp.wrapping_add(0x20); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); rax = ((rdi.wrapping_add(P3)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax); crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm0) as u128); (rax, xmm0) = crate::simd_ops::call_stk_xmm((crate::simd_ops::ru64(rsi.wrapping_add(P3))) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru64(rdi.wrapping_add(P0))) as u64; crate::simd_ops::lk_add_u64(rsi.wrapping_add(P2), (rax) as u64); rdi = (crate::simd_ops::ru64(rdi)) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 1; continue; } bb = 2; continue; }
            2 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_44787(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    xmm0 = (crate::simd_ops::ru128(rcx)) as u128;
    rcx = (crate::simd_ops::ru64(rcx)) as u64;
    rax = (((xmm0 >> 64) as u64)) as u64;
    rcx = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64;
    rax = rdx;
    crate::simd_ops::wu64(rdx, rcx);
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_44788<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = ((rdx.wrapping_add(rdx.wrapping_mul(2))) as u64) as u64;
    rax = ((rax).wrapping_shl(4u32)) as u64;
    fl_l = ((rax).wrapping_add(crate::simd_ops::ru64(rcx.wrapping_add(P0)))) as u64; fl_r = fl_l;
    rax = ((fl_l as u64)) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_44828<const P0: u64, const P1: u64, const P2: u64, const P3: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 0>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rax = (crate::simd_ops::ru64(rdx)) as u64;
    crate::simd_ops::wu64(rcx, rax);
    rax = (crate::simd_ops::ru64(r8)) as u64;
    crate::simd_ops::wu64(rcx.wrapping_add(P0), rax);
    xmm0 = (crate::simd_ops::ru128(r9)) as u128;
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64;
    crate::simd_ops::wu128(rcx.wrapping_add(P1), (xmm0) as u128);
    xmm0 = (crate::simd_ops::ru128(rax)) as u128;
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    crate::simd_ops::wu128(rcx.wrapping_add(P2), (xmm0) as u128);
    xmm0 = (crate::simd_ops::ru128(rax)) as u128;
    crate::simd_ops::wu128(rcx.wrapping_add(P3), (xmm0) as u128);
    rax = rcx;
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_44840<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rsi = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rbx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; rbx = ((rbx).wrapping_shl(4u32)) as u64; rdi = rax; rbx = ((rbx).wrapping_add(rax)) as u64; fl_l = rax; fl_r = rbx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 2; continue; } fl_l = ((rdi).wrapping_add(0x8u64)) as u64; fl_r = fl_l; rdi = ((fl_l as u64)) as u64; bb = 1; continue; }
            1 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(P2))) as u64; r11 = (crate::simd_ops::ru64(rdi.wrapping_add(P3))) as u64; rax = rcx; r9 = (crate::simd_ops::ru64(rdi.wrapping_add(P1))) as u64; r8 = (crate::simd_ops::ru64(rdi)) as u64; rdx = (crate::simd_ops::ru32(rdi.wrapping_add(P0))) as u32 as u64; rax = ((rax).wrapping_shr(32u32)) as u64; rax = ((rax).wrapping_sub(rcx)) as u64; r10 = ((rcx as u32)) as u32 as u64; rax = ((rax as u32)) as u32 as u64; rcx = rsp.wrapping_add(0x20); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); rax = ((rsi.wrapping_add(r10.wrapping_mul(8))) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax); rax = crate::simd_ops::call_stk(r11, rax, rcx, rdx, r8, r9, rsp); rdi = ((rdi.wrapping_add(P4)) as u64) as u64; rax = ((rdi.wrapping_add(P3)) as u64) as u64; fl_l = rax; fl_r = rbx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } bb = 2; continue; }
            2 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_44841<const P0: u64, const P1: u64, const P2: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rsp = ((rsp).wrapping_sub(0x48u64)) as u64;
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    rdx = rsp.wrapping_add(0x20);
    xmm1 = (crate::simd_ops::ru128(rcx)) as u128;
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax);
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64;
    rcx = rsp.wrapping_add(0x30);
    crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax);
    xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128;
    crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128);
    crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm1) as u128);
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    rsp = ((rsp).wrapping_add(0x48u64)) as u64;
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_44856<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    xmm1 = (crate::simd_ops::ru128(rcx)) as u128;
    rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    xmm3 = (crate::simd_ops::ru128(rdx)) as u128;
    rax = ((xmm1 as u64)) as u64;
    rcx = ((rcx.wrapping_add(rax.wrapping_mul(2))) as u64) as u64;
    rax = (((xmm1 >> 64) as u64)) as u64;
    fl_l = rax; fl_r = rcx;
    if (((fl_l as u64)) == ((fl_r as u64))) { return rax; }
    rdx = (((xmm3 >> 64) as u64)) as u64;
    rdx = ((rdx.wrapping_add(r8.wrapping_mul(8))) as u64) as u64;
    loop { r8 = ((crate::simd_ops::ru16(rax)) as u32) as u32 as u64; rax = ((rax).wrapping_add(0x2u64)) as u64; crate::simd_ops::wu16(rdx, ((r8 as u16)) as u16); rdx = ((rdx.wrapping_add(P0)) as u64) as u64; fl_l = rax; fl_r = rcx; if !((((fl_l as u64)) != ((fl_r as u64)))) { break; } }
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_44857<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    rcx = (crate::simd_ops::ru64(rdx)) as u64;
    rcx = ((rax.wrapping_add(rcx.wrapping_mul(2))) as u64) as u64;
    fl_l = rax; fl_r = rcx;
    if (((fl_l as u64)) == ((fl_r as u64))) { return rax; }
    rdx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    rdx = ((rdx.wrapping_add(r9.wrapping_mul(8))) as u64) as u64;
    loop { r8 = ((crate::simd_ops::ru16(rax)) as u32) as u32 as u64; rax = ((rax).wrapping_add(0x2u64)) as u64; crate::simd_ops::wu16(rdx, ((r8 as u16)) as u16); rdx = ((rdx.wrapping_add(P0)) as u64) as u64; fl_l = rax; fl_r = rcx; if !((((fl_l as u64)) != ((fl_r as u64)))) { break; } }
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_44858<const P0: u64, const P1: u64, const P2: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut r10, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    xmm1 = (crate::simd_ops::ru128(rcx)) as u128;
    rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    xmm3 = (crate::simd_ops::ru128(rdx)) as u128;
    rax = ((xmm1 as u64)) as u64;
    rcx = ((rcx.wrapping_add(rax.wrapping_mul(4))) as u64) as u64;
    rax = (((xmm1 >> 64) as u64)) as u64;
    fl_l = rax; fl_r = rcx;
    if (((fl_l as u64)) == ((fl_r as u64))) { return rax; }
    rdx = (((xmm3 >> 64) as u64)) as u64;
    r10 = 0;
    fl_l = 0; fl_r = 0;
    rdx = ((rdx.wrapping_add(r8.wrapping_mul(8))) as u64) as u64;
    loop { r8 = (crate::simd_ops::ru32(rax)) as u32 as u64; r9 = rdx; rdx = ((rdx).wrapping_add(0x8u64)) as u64; rax = ((rax).wrapping_add(0x4u64)) as u64; crate::simd_ops::wu32(r9.wrapping_add(P1), ((r8 as u32)) as u32); crate::simd_ops::wu16(r9.wrapping_add(P2), ((r10 as u16)) as u16); fl_l = rax; fl_r = rcx; if !((((fl_l as u64)) != ((fl_r as u64)))) { break; } }
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_44859<const P0: u64, const P1: u64, const P2: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut r10, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    rcx = (crate::simd_ops::ru64(rdx)) as u64;
    rcx = ((rax.wrapping_add(rcx.wrapping_mul(4))) as u64) as u64;
    fl_l = rax; fl_r = rcx;
    if (((fl_l as u64)) == ((fl_r as u64))) { return rax; }
    rdx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    r10 = 0;
    fl_l = 0; fl_r = 0;
    rdx = ((rdx.wrapping_add(r9.wrapping_mul(8))) as u64) as u64;
    loop { r8 = (crate::simd_ops::ru32(rax)) as u32 as u64; r9 = rdx; rdx = ((rdx).wrapping_add(0x8u64)) as u64; rax = ((rax).wrapping_add(0x4u64)) as u64; crate::simd_ops::wu32(r9.wrapping_add(P1), ((r8 as u32)) as u32); crate::simd_ops::wu16(r9.wrapping_add(P2), ((r10 as u16)) as u16); fl_l = rax; fl_r = rcx; if !((((fl_l as u64)) != ((fl_r as u64)))) { break; } }
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_44879<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rax = ((crate::simd_ops::ru16(rcx.wrapping_add(P0))) as u32) as u32 as u64;
    xmm1 = (crate::simd_ops::ru128(P1)) as u128;
    rax = ((rax).wrapping_add(rax)) as u64;
    xmm2 = (xmm1) ^ (crate::simd_ops::ru128(r8.wrapping_add(rax.wrapping_mul(8))));
    rax = ((crate::simd_ops::ru16(rcx)) as u32) as u32 as u64;
    fl_l = ((rax).wrapping_add(rax)) as u64; fl_r = fl_l;
    rax = ((fl_l as u64)) as u64;
    crate::simd_ops::wu128(r8.wrapping_add(rax.wrapping_mul(8)), (xmm2) as u128);
    rax = 0x1;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_44894<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rax = ((crate::simd_ops::ru16(rcx.wrapping_add(P0))) as u32) as u32 as u64;
    xmm0 = (crate::simd_ops::ru128(P1)) as u128;
    rax = ((rax).wrapping_add(rax)) as u64;
    xmm1 = crate::simd_ops::vmulps(xmm0, crate::simd_ops::ru128(r8.wrapping_add(rax.wrapping_mul(8))));
    rax = ((crate::simd_ops::ru16(rcx)) as u32) as u32 as u64;
    fl_l = ((rax).wrapping_add(rax)) as u64; fl_r = fl_l;
    rax = ((fl_l as u64)) as u64;
    crate::simd_ops::wu128(r8.wrapping_add(rax.wrapping_mul(8)), (xmm1) as u128);
    rax = 0x1;
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_44943<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rax = ((crate::simd_ops::ru16(rcx.wrapping_add(P0))) as u32) as u32 as u64;
    rax = ((rax).wrapping_add(rax)) as u64;
    xmm0 = crate::simd_ops::vbroadcastss(crate::simd_ops::ru32(r8.wrapping_add(rax.wrapping_mul(8)).wrapping_add(P1)));
    rax = ((crate::simd_ops::ru16(rcx)) as u32) as u32 as u64;
    fl_l = ((rax).wrapping_add(rax)) as u64; fl_r = fl_l;
    rax = ((fl_l as u64)) as u64;
    crate::simd_ops::wu128(r8.wrapping_add(rax.wrapping_mul(8)), (xmm0) as u128);
    rax = 0x1;
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_44947<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = ((crate::simd_ops::ru16(rcx.wrapping_add(P0))) as u32) as u32 as u64;
    rcx = ((crate::simd_ops::ru16(rcx)) as u32) as u32 as u64;
    rax = ((rax).wrapping_add(rax)) as u64;
    fl_l = ((rcx).wrapping_add(rcx)) as u64; fl_r = fl_l;
    rcx = ((fl_l as u64)) as u64;
    rax = (crate::simd_ops::ru32(r8.wrapping_add(rax.wrapping_mul(8)))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(P1), ((rax as u32)) as u32);
    rax = 0x1;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_44954<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rdx = ((crate::simd_ops::ru16(rcx.wrapping_add(P0))) as u32) as u32 as u64;
    rax = ((crate::simd_ops::ru16(rcx)) as u32) as u32 as u64;
    rdx = ((rdx).wrapping_add(rdx)) as u64;
    fl_l = ((rax).wrapping_add(rax)) as u64; fl_r = fl_l;
    rax = ((fl_l as u64)) as u64;
    xmm0 = (crate::simd_ops::ru128(r8.wrapping_add(rdx.wrapping_mul(8)))) as u128;
    crate::simd_ops::wu128(r8.wrapping_add(rax.wrapping_mul(8)), (xmm0) as u128);
    rax = 0x1;
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_44955<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rax = ((crate::simd_ops::ru16(rcx)) as u32) as u32 as u64;
    rdx = ((crate::simd_ops::ru16(rcx.wrapping_add(P0))) as u32) as u32 as u64;
    fl_l = ((rdx).wrapping_add(rdx)) as u64; fl_r = fl_l;
    rdx = ((fl_l as u64)) as u64;
    rcx = (crate::simd_ops::ru64(r9.wrapping_add(rax.wrapping_mul(8)))) as u64;
    rax = 0x1;
    xmm0 = (crate::simd_ops::ru128(r8.wrapping_add(rdx.wrapping_mul(8)))) as u128;
    crate::simd_ops::wu128(rcx, (xmm0) as u128);
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_44957<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rax = ((crate::simd_ops::ru16(rcx)) as u32) as u32 as u64;
    rdx = ((crate::simd_ops::ru16(rcx.wrapping_add(P0))) as u32) as u32 as u64;
    fl_l = ((rdx).wrapping_add(rdx)) as u64; fl_r = fl_l;
    rdx = ((fl_l as u64)) as u64;
    rax = (crate::simd_ops::ru64(r9.wrapping_add(rax.wrapping_mul(8)))) as u64;
    xmm0 = (crate::simd_ops::ru128(r8.wrapping_add(rdx.wrapping_mul(8)))) as u128;
    crate::simd_ops::wu64(rax, ((xmm0 as u64)) as u64);
    rax = 0x1;
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_44958<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = ((crate::simd_ops::ru16(rcx)) as u32) as u32 as u64;
    rdx = ((crate::simd_ops::ru16(rcx.wrapping_add(P0))) as u32) as u32 as u64;
    fl_l = ((rdx).wrapping_add(rdx)) as u64; fl_r = fl_l;
    rdx = ((fl_l as u64)) as u64;
    rcx = (crate::simd_ops::ru64(r9.wrapping_add(rax.wrapping_mul(8)))) as u64;
    rax = (crate::simd_ops::ru32(r8.wrapping_add(rdx.wrapping_mul(8)))) as u32 as u64;
    crate::simd_ops::wu32(rcx, ((rax as u32)) as u32);
    rax = 0x1;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_44961<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rax = ((crate::simd_ops::ru16(rcx.wrapping_add(P0))) as u32) as u32 as u64;
    rdx = (crate::simd_ops::ru64(r9.wrapping_add(rax.wrapping_mul(8)))) as u64;
    rax = ((crate::simd_ops::ru16(rcx)) as u32) as u32 as u64;
    fl_l = ((rax).wrapping_add(rax)) as u64; fl_r = fl_l;
    rax = ((fl_l as u64)) as u64;
    xmm0 = (crate::simd_ops::ru128(rdx)) as u128;
    crate::simd_ops::wu128(r8.wrapping_add(rax.wrapping_mul(8)), (xmm0) as u128);
    rax = 0x1;
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_44963<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rax = ((crate::simd_ops::ru16(rcx.wrapping_add(P0))) as u32) as u32 as u64;
    rdx = (crate::simd_ops::ru64(r9.wrapping_add(rax.wrapping_mul(8)))) as u64;
    rax = ((crate::simd_ops::ru16(rcx)) as u32) as u32 as u64;
    fl_l = ((rax).wrapping_add(rax)) as u64; fl_r = fl_l;
    rax = ((fl_l as u64)) as u64;
    xmm0 = ((crate::simd_ops::ru64(rdx)) as u64) as u128;
    crate::simd_ops::wu128(r8.wrapping_add(rax.wrapping_mul(8)), (xmm0) as u128);
    rax = 0x1;
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_44970<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 137>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r9); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x430u64)) as u64; rdi = rcx; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((r8 as u32)) as u32); r9 = ((rdx as u32)) as u32 as u64; r8 = P0; rbx = 0x400; rcx = rsp.wrapping_add(0x30); rdx = ((rbx as u32)) as u32 as u64; rsi = rsp.wrapping_add(0x470); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); r8 = (crate::simd_ops::ru64(rsp.wrapping_add(0x468u64))) as u64; rcx = rsp.wrapping_add(0x30); rbx = (((rbx as u32)).wrapping_sub((rax as u32))) as u32 as u64; r9 = rsi; rax = (rax as u32 as i32 as i64) as u64; rcx = ((rcx).wrapping_add(rax)) as u64; rdx = ((((rbx as u32)) as i32 as i64) as u64) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rax = rsp.wrapping_add(0x30); r8 = (0xffffffffffffffffu64) as u64; bb = 1; continue; }
            1 => { r8 = ((r8).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru8(rax.wrapping_add(r8))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 1; continue; } rcx = ((rdi.wrapping_add(P3)) as u64) as u64; rdx = rsp.wrapping_add(0x30); rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rsp = ((rsp).wrapping_add(0x430u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_44977<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (0u64, 0u64, 0u64);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    xmm0 = ((crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32) as u128;
    xmm2 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) - f32::from_bits(crate::simd_ops::ru32(r8.wrapping_add(P0)))).to_bits() as u128);
    xmm1 = ((crate::simd_ops::ru32(P1)) as u32) as u128;
    fl_l = ((xmm1 as u32)) as u32 as u64; fl_r = ((xmm2 as u32)) as u32 as u64;
    rax = (rax & !0xffu64) | (((if (f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { 1u8 } else { 0u8 }) as u8 as u64));
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_44978<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (0u64, 0u64, 0u64);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    xmm0 = ((crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32) as u128;
    xmm1 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) - f32::from_bits(crate::simd_ops::ru32(r8.wrapping_add(P0)))).to_bits() as u128);
    fl_l = ((xmm1 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(P1)) as u32 as u64;
    rax = (rax & !0xffu64) | (((if (f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { 1u8 } else { 0u8 }) as u8 as u64));
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_44979<const P0: u64, const P1: u64, const P2: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (0u64, 0u64, 0u64);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    xmm0 = ((crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32) as u128;
    xmm1 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) - f32::from_bits(crate::simd_ops::ru32(r8.wrapping_add(P0)))).to_bits() as u128);
    xmm2 = ((crate::simd_ops::ru32(P1)) as u32) as u128;
    xmm0 = ((crate::simd_ops::ru32(P2)) as u32) as u128;
    xmm3 = (!(xmm2)) & (xmm1);
    fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = ((xmm3 as u32)) as u32 as u64;
    rax = (rax & !0xffu64) | (((if (f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { 1u8 } else { 0u8 }) as u8 as u64));
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_44980<const P0: u64, const P1: u64, const P2: u64, const P3: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (0u64, 0u64, 0u64);
    let mut xmm6: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rsp = ((rsp).wrapping_sub(0x38u64)) as u64;
    xmm2 = ((crate::simd_ops::ru32(P0)) as u32) as u128;
    rcx = rdx;
    crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm6) as u128);
    xmm6 = ((crate::simd_ops::ru32(P1)) as u32) as u128;
    rdx = r8;
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    xmm1 = ((crate::simd_ops::ru32(P3)) as u32) as u128;
    xmm2 = (!(xmm6)) & (xmm0);
    xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128;
    fl_l = ((xmm1 as u32)) as u32 as u64; fl_r = ((xmm2 as u32)) as u32 as u64;
    rax = (rax & !0xffu64) | (((if (f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { 1u8 } else { 0u8 }) as u8 as u64));
    rsp = ((rsp).wrapping_add(0x38u64)) as u64;
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_44989<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    xmm0 = ((crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32) as u128;
    xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) - f32::from_bits(crate::simd_ops::ru32(rdx.wrapping_add(P0)))).to_bits() as u128);
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_45027<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = 0;
    rax = 0x1;
    let tmp_xchg: u8 = crate::simd_ops::atomic_xchg_u8(rcx.wrapping_add(P0), ((rax as u8)) as u8);
    rax = (rax & !0xffu64) | (((tmp_xchg) as u8 as u64));
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_45060<const P0: u64, const P1: u64, const P2: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rax = ((crate::simd_ops::ru16(rcx.wrapping_add(P0))) as u32) as u32 as u64;
    xmm0 = 0u128;
    xmm1 = 0u128;
    xmm1 = (xmm1 & !0xffff_ffffu128) | (((((rax as u32)) as i32) as f32).to_bits() as u128);
    rax = ((crate::simd_ops::ru16(rcx.wrapping_add(P1))) as u32) as u32 as u64;
    xmm0 = (xmm0 & !0xffff_ffffu128) | (((((rax as u32)) as i32) as f32).to_bits() as u128);
    xmm2 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) / f32::from_bits(xmm0 as u32)).to_bits() as u128);
    xmm3 = 0u128;
    xmm0 = (xmm2 & !0xffff_ffffu128) | ((if f32::from_bits(xmm2 as u32) > f32::from_bits(xmm3 as u32) { f32::from_bits(xmm2 as u32) } else { f32::from_bits(xmm3 as u32) }).to_bits() as u128);
    xmm0 = (xmm0 & !0xffff_ffffu128) | ((if f32::from_bits(xmm0 as u32) < f32::from_bits(crate::simd_ops::ru32(P2)) { f32::from_bits(xmm0 as u32) } else { f32::from_bits(crate::simd_ops::ru32(P2)) }).to_bits() as u128);
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_45061<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rax = ((crate::simd_ops::ru16(rcx.wrapping_add(P0))) as u32) as u32 as u64;
    xmm0 = 0u128;
    xmm0 = (xmm0 & !0xffff_ffffu128) | (((((rax as u32)) as i32) as f32).to_bits() as u128);
    xmm1 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm1 as u32)).to_bits() as u128);
    rax = (((crate::simd_ops::cvt_f32_i32(f32::from_bits(xmm1 as u32)) as i32)) as u32) as u32 as u64;
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_45081<const P0: u64, const P1: u64, const P2: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rdi = rdx;
    rsi = rcx;
    rdx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    rcx = rsp.wrapping_add(0x40);
    rbx = (((r9 as u8)) as u32) as u32 as u64;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    r8 = ((crate::simd_ops::ru16(rsp.wrapping_add(0x40u64))) as u32) as u32 as u64;
    r9 = (((rbx as u8)) as u32) as u32 as u64;
    rdx = rdi;
    rcx = rsi;
    rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp);
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rax = rsi;
    rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_45084<const P0: u64, const P1: u64, const P2: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    r8 = (((rdx as u16)) as u32) as u32 as u64;
    rbx = rcx;
    rdx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    rcx = rsp.wrapping_add(0x38);
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rcx = rbx;
    rdx = ((crate::simd_ops::ru16(rax)) as u32) as u32 as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return crate::simd_ops::tail_stk(P2, rax, rcx, rdx, r8, r9, rsp);
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_45108<const P0: u64, const P1: u64, const P2: u64, const P3: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    rbx = rdx;
    r8 = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64;
    rcx = (crate::simd_ops::ru64(rcx)) as u64;
    r9 = (crate::simd_ops::ru64(rax)) as u64;
    r9 = ((crate::simd_ops::ru16(r9.wrapping_add(P2))) as u32) as u32 as u64;
    rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp);
    rax = rbx;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_45139(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (0u64, 0u64, 0u64);
    fl_l = rcx; fl_r = rcx;
    if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; return rax; }
    rax = ((crate::simd_ops::ru16(rcx)) as u32) as u32 as u64;
    fl_l = ((rdx as u16)) as u16 as u64; fl_r = ((rax as u16)) as u16 as u64;
    if !(((((fl_l as u16)) & ((fl_r as u16))) == 0)) { rdx = (rdx & !0xffffu64) | (((!((rdx as u16))) as u16 as u64)); fl_l = ((((rdx as u16)) & ((rax as u16)))) as u16 as u64; fl_r = fl_l; rdx = (rdx & !0xffffu64) | ((((fl_l as u16)) as u16 as u64)); rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); crate::simd_ops::wu16(rcx, ((rdx as u16)) as u16); return rax; }
    rax = (rax & !0xffu64) | (((0u8) as u8 as u64));
    fl_l = 0; fl_r = 0;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_45161<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (0u64, 0u64, 0u64);
    rax = 0;
    fl_l = 0; fl_r = 0;
    let tmp_xchg: u8 = crate::simd_ops::atomic_xchg_u8(rcx.wrapping_add(P0), ((rax as u8)) as u8);
    rax = (rax & !0xffu64) | (((tmp_xchg) as u8 as u64));
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_45178<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64, const P9: u64, const P10: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rbx = (((rdx as u16)) as u32) as u32 as u64; rdi = rcx; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } rax = (crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64; fl_l = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 1; continue; } rax = P2; fl_l = (crate::simd_ops::ru64(rcx.wrapping_add(P3))) as u64; fl_r = rax; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 4; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(P3))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P4))) as u64; rax = crate::simd_ops::call_stk(rax, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 4; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru64(rdi.wrapping_add(P5))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } fl_l = (crate::simd_ops::ru64(rdi.wrapping_add(P6))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } rax = (crate::simd_ops::ru64(rdi.wrapping_add(P7))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } rax = ((crate::simd_ops::ru8(rax.wrapping_add(P8))) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { rdx = (crate::simd_ops::ru64(rdi)) as u64; rcx = rsp.wrapping_add(0x40); r8 = (((rbx as u16)) as u32) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rdx); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rax = crate::simd_ops::call_stk(P9, rax, rcx, rdx, r8, r9, rsp); rcx = rsp.wrapping_add(0x20); rdx = ((crate::simd_ops::ru16(rax)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P10, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            4 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_45203<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64, const P9: u64, const P10: u64, const P11: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rdx; rdi = rcx; rax = (crate::simd_ops::ru64(rdx)) as u64; crate::simd_ops::wu64(rcx, rax); rcx = ((rcx).wrapping_add(0x8u64)) as u64; rdx = ((rdx).wrapping_add(0x8u64)) as u64; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rcx = ((rdi.wrapping_add(P1)) as u64) as u64; rdx = ((rbx.wrapping_add(P1)) as u64) as u64; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rcx = ((rdi.wrapping_add(P2)) as u64) as u64; rdx = ((rbx.wrapping_add(P2)) as u64) as u64; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rcx = ((rdi.wrapping_add(P3)) as u64) as u64; rdx = ((rbx.wrapping_add(P3)) as u64) as u64; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rcx = ((rdi.wrapping_add(P4)) as u64) as u64; rdx = ((rbx.wrapping_add(P4)) as u64) as u64; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rcx = ((rdi.wrapping_add(P5)) as u64) as u64; rdx = ((rbx.wrapping_add(P5)) as u64) as u64; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rcx = ((rdi.wrapping_add(P6)) as u64) as u64; rdx = ((rbx.wrapping_add(P6)) as u64) as u64; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rcx = ((rdi.wrapping_add(P7)) as u64) as u64; rdx = ((rbx.wrapping_add(P7)) as u64) as u64; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rcx = ((rdi.wrapping_add(P8)) as u64) as u64; rdx = ((rbx.wrapping_add(P8)) as u64) as u64; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rcx = ((rdi.wrapping_add(P9)) as u64) as u64; rdx = ((rbx.wrapping_add(P9)) as u64) as u64; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rcx = ((rdi.wrapping_add(P10)) as u64) as u64; rdx = ((rbx.wrapping_add(P10)) as u64) as u64; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rcx = ((rdi.wrapping_add(P11)) as u64) as u64; rdx = rbx.wrapping_add(0x530); rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rax = rdi; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_45206<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64;
    rax = (((rax) as i64).wrapping_mul((0x5a8u64) as i64) as u64) as u64;
    fl_l = ((rax).wrapping_add(crate::simd_ops::ru64(rcx))) as u64; fl_r = fl_l;
    rax = ((fl_l as u64)) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_45210(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    fl_l = (crate::simd_ops::lk_add_u64(rcx, (0x5a8u64) as u64)) as u64; fl_r = fl_l;
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    crate::simd_ops::wu64(rdx, rax);
    rax = rdx;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_45294(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = rcx;
    rcx = (crate::simd_ops::ru64(r8)) as u64;
    rax = (crate::simd_ops::ru32(rcx.wrapping_add(rax))) as u32 as u64;
    fl_l = ((rcx).wrapping_add(0x4u64)) as u64; fl_r = fl_l;
    rcx = ((fl_l as u64)) as u64;
    crate::simd_ops::wu64(r8, rcx);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_45393<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64, const P9: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 73>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x240u64)) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x10u64))) as u64; rdi = P0; fl_l = rax; fl_r = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; rdx = rdi; rbx = rcx; rdx = (if (((fl_l as u64)) < ((fl_r as u64))) { rax } else { rdx }) as u64; fl_l = (crate::simd_ops::ru32(rdx.wrapping_add(0x10u64))) as u32 as u64; fl_r = 0x5; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 3; continue; } rax = ((crate::simd_ops::ru8(rcx.wrapping_add(0x1u64))) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x4u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 1; continue; } rcx = (crate::simd_ops::ru64(rcx.wrapping_add(0x18u64))) as u64; fl_l = ((((rax as u8)) & (0xfbu8))) as u8 as u64; fl_r = fl_l; rax = (rax & !0xffu64) | ((((fl_l as u8)) as u8 as u64)); crate::simd_ops::wu8(rbx.wrapping_add(0x1u64), ((rax as u8)) as u8); rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x68u64), rax); crate::simd_ops::wu64(rcx.wrapping_add(0x30u64), rbx); bb = 1; continue; }
            1 => { rcx = rbx; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 11; continue; } rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x18u64))) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x40u64))) as u64; fl_l = (crate::simd_ops::ru64(rcx.wrapping_add(0x48u64))) as u64; fl_r = rax; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 2; continue; } rcx = rbx; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; rdx = rdi; fl_l = rax; fl_r = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; rdx = (if (((fl_l as u64)) < ((fl_r as u64))) { rax } else { rdx }) as u64; bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rdx)) as u64; rbp = (crate::simd_ops::ru32(rax.wrapping_add(0x14u64))) as u32 as u64; r14 = rax.wrapping_add(0x18); fl_l = r14; fl_r = r14; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } rdi = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rdi); rsi = rsp.wrapping_add(0x40); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); rax = rsp.wrapping_add(0x240); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax); fl_l = rbp; fl_r = (0x200u64) as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 4; continue; } rdx = rbp.wrapping_sub(0x200); r8 = ((rdi.wrapping_sub(0x1u64)) as u32) as u32 as u64; rcx = rsp.wrapping_add(0x20); rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; bb = 4; continue; }
            4 => { fl_l = rbp; fl_r = rbp; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { rcx = ((crate::simd_ops::ru8(r14.wrapping_add(rdi))) as u32) as u32 as u64; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P4)) as u64, rax, rcx, rdx, r8, r9, rsp); rdi = ((rdi).wrapping_add(1)) as u64; crate::simd_ops::wu8(rsi, ((rax as u8)) as u8); rsi = rsi.wrapping_add(0x1); fl_l = rdi; fl_r = rbp; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 5; continue; } rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; bb = 6; continue; }
            6 => { r8 = ((rsi.wrapping_add(rbp)) as u64) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r8); fl_l = rbp; fl_r = rbp; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x18u64))) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x40u64))) as u64; fl_l = (crate::simd_ops::ru64(rcx.wrapping_add(0x48u64))) as u64; fl_r = rax; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 7; continue; } rcx = rbx; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); r8 = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; bb = 7; continue; }
            7 => { rsi = 0x5; rdi = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; rcx = rbx; fl_l = r8; fl_r = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 8; continue; } rdx = rbp; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rdi.wrapping_sub(0x18u64), rax); crate::simd_ops::wu32(rdi.wrapping_sub(0x8u64), ((rsi as u32)) as u32); bb = 10; continue; }
            8 => { r8 = ((r8).wrapping_sub(rbp)) as u64; rdx = rbp.wrapping_add(0x18); r8 = ((r8).wrapping_sub(0x18u64)) as u64; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rdi.wrapping_sub(0x18u64), rax); crate::simd_ops::wu32(rdi.wrapping_sub(0x8u64), ((rsi as u32)) as u32); bb = 10; continue; }
            9 => { rax = rsp.wrapping_add(0x20); rcx = rbx; r8 = ((r8).wrapping_sub(rax)) as u64; rdx = rsp.wrapping_add(0x40); r8 = ((r8).wrapping_sub(0x20u64)) as u64; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); bb = 10; continue; }
            10 => { r11 = rsp.wrapping_add(0x240); rax = 0x1; rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x10u64))) as u64; rbp = (crate::simd_ops::ru64(r11.wrapping_add(0x18u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x20u64))) as u64; rdi = (crate::simd_ops::ru64(r11.wrapping_add(0x28u64))) as u64; rsp = r11; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            11 => { rdx = 0x1; rcx = rbx; r8 = ((rdx.wrapping_add(0x4u64)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, P9, rcx, rsp); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_45419<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rdx;
    rdx = rcx;
    rcx = rbx;
    rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P0)) as u64, rax, rcx, rdx, r8, r9, rsp);
    rcx = 0;
    fl_l = 0; fl_r = 0;
    fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64;
    rcx = (if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { rbx } else { rcx }) as u64;
    rax = rcx;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_45420<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    crate::simd_ops::forward_incoming_to_outgoing();
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rcx = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((crate::simd_ops::ru64(P0)) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_45421<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdx);
    crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), r8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r9);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    r8 = rsp.wrapping_add(0x40);
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_45448(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) - f32::from_bits(xmm1 as u32)).to_bits() as u128);
    xmm0 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) * f32::from_bits(xmm0 as u32)).to_bits() as u128);
    xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) + f32::from_bits(xmm1 as u32)).to_bits() as u128);
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_45449<const P0: u64, const P1: u64, const P2: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    xmm1 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(crate::simd_ops::ru32(P0))).to_bits() as u128);
    xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) - f32::from_bits(crate::simd_ops::ru32(P1))).to_bits() as u128);
    xmm2 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(xmm0 as u32)).to_bits() as u128);
    xmm3 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) + f32::from_bits(crate::simd_ops::ru32(P2))).to_bits() as u128);
    xmm1 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm0 as u32)).to_bits() as u128);
    xmm0 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(xmm0 as u32)).to_bits() as u128);
    xmm0 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) * f32::from_bits(xmm0 as u32)).to_bits() as u128);
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_45463<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    rax = rdx;
    rcx = rdx;
    fl_l = r9; fl_r = r9;
    if !(((((fl_l as u64)) & ((fl_r as u64))) != 0)) { rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P0)) as u64, rax, rcx, rdx, r8, r9, rsp); rax = 0; fl_l = 0; fl_r = 0; rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
    rdx = r9;
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    let _ = &stack; return crate::simd_ops::tail_stk((crate::simd_ops::ru64(P1)) as u64, rax, rcx, rdx, r8, r9, rsp);
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_45540<const P0: u64, const P1: u64, const P2: u64, const P3: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = r8; rdi = rcx; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); r9 = rax; rdx = P1; r8 = rbx; rcx = rdi; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, P3, rcx, rsp); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_45546<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64, const P9: u64, const P10: u64, const P11: u64, const P12: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 15>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x50u64)) as u64; r14 = P0; r13 = 0; fl_l = 0; fl_r = 0; rax = r14; rbp = ((r13 as u32)) as u32 as u64; rsi = rcx; bb = 1; continue; }
            1 => { rbp = (((rbp as u32)).wrapping_add(1)) as u32 as u64; rax = rax.wrapping_add(0x10); fl_l = (crate::simd_ops::ru64(rax)) as u64; fl_r = r13; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } r9 = 0x1; crate::simd_ops::wu64(rsp.wrapping_add(0x80u64), r12); r8 = P1; rdx = 0xffffd8f0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = ((crate::simd_ops::ru8(rsi.wrapping_add(0x1u64))) as u32) as u32 as u64; r12 = rsi.wrapping_add(0x1); fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x4u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 2; continue; } rcx = (crate::simd_ops::ru64(rsi.wrapping_add(0x18u64))) as u64; fl_l = ((((rax as u8)) & (0xfbu8))) as u8 as u64; fl_r = fl_l; rax = (rax & !0xffu64) | ((((fl_l as u8)) as u8 as u64)); crate::simd_ops::wu8(rsi.wrapping_add(0x1u64), ((rax as u8)) as u8); rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu64(rsi.wrapping_add(0x68u64), rax); crate::simd_ops::wu64(rcx.wrapping_add(0x30u64), rsi); bb = 2; continue; }
            2 => { rbx = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; rdi = rsi.wrapping_add(0x8); r8 = 0x5; rdx = P3; rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r9 = (crate::simd_ops::ru64(rdi)) as u64; r8 = rsp.wrapping_add(0x30); rdx = rbx.wrapping_sub(0x18); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); rcx = rsi; crate::simd_ops::wu32(rsp.wrapping_add(0x40u64), (0x5u32) as u32); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru64(rdi)) as u64; rcx = P6; rdx = rax.wrapping_add(0x18); crate::simd_ops::wu64(rdi, rdx); fl_l = rax; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 3; continue; } fl_l = (crate::simd_ops::ru32(rax.wrapping_add(0x10u64))) as u32 as u64; fl_r = 0x6; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 5; continue; } bb = 3; continue; }
            3 => { r9 = ((rbp as u32)) as u32 as u64; crate::simd_ops::wu64(rdi, rax); r8 = P3; rdx = 0xffffd8ee; rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 10; continue; } rax = ((crate::simd_ops::ru8(rsi.wrapping_add(0x1u64))) as u32) as u32 as u64; rbp = rdi; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x4u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 4; continue; } rcx = (crate::simd_ops::ru64(rsi.wrapping_add(0x18u64))) as u64; rbp = rsi.wrapping_add(0x8); fl_l = ((((rax as u8)) & (0xfbu8))) as u8 as u64; fl_r = fl_l; rax = (rax & !0xffu64) | ((((fl_l as u8)) as u8 as u64)); crate::simd_ops::wu8(r12, ((rax as u8)) as u8); rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu64(rsi.wrapping_add(0x68u64), rax); crate::simd_ops::wu64(rcx.wrapping_add(0x30u64), rsi); bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(rdi)) as u64; rdx = P3; r8 = 0x5; rcx = rsi; xmm0 = (crate::simd_ops::ru128(rax.wrapping_sub(0x18u64))) as u128; crate::simd_ops::wu128(rax, (xmm0) as u128); xmm1 = ((crate::simd_ops::ru64(rax.wrapping_sub(0x8u64))) as u64) as u128; crate::simd_ops::wu64(rax.wrapping_add(0x10u64), ((xmm1 as u64)) as u64); rbx = (crate::simd_ops::ru64(rdi)) as u64; rbx = ((rbx).wrapping_add(0x18u64)) as u64; crate::simd_ops::wu64(rdi, rbx); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r9 = (crate::simd_ops::ru64(rdi)) as u64; rdx = rbx.wrapping_sub(0x48); r9 = ((r9).wrapping_sub(0x18u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); r8 = rsp.wrapping_add(0x30); crate::simd_ops::wu32(rsp.wrapping_add(0x40u64), (0x5u32) as u32); rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = (crate::simd_ops::lk_add_u64(rdi, (0xffffffffffffffe8u64) as u64)) as u64; fl_r = fl_l; rdx = (crate::simd_ops::ru64(rdi)) as u64; rdi = rbp; bb = 5; continue; }
            5 => { rax = rdx.wrapping_sub(0x18); fl_l = rax; fl_r = rdx; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 7; continue; } rcx = rax.wrapping_sub(0x18); bb = 6; continue; }
            6 => { xmm0 = (crate::simd_ops::ru128(rax)) as u128; crate::simd_ops::wu128(rcx, (xmm0) as u128); xmm1 = ((crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64) as u128; crate::simd_ops::wu64(rcx.wrapping_add(0x10u64), ((xmm1 as u64)) as u64); rdx = (crate::simd_ops::ru64(rdi)) as u64; rcx = rcx.wrapping_add(0x18); rax = ((rax).wrapping_add(0x18u64)) as u64; fl_l = rax; fl_r = rdx; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 6; continue; } rdi = rsi.wrapping_add(0x8); bb = 7; continue; }
            7 => { rax = rdx.wrapping_sub(0x18); crate::simd_ops::wu64(rdi, rax); rax = P8; bb = 8; continue; }
            8 => { rdx = (crate::simd_ops::ru64(r14.wrapping_add(0x8u64))) as u64; r9 = 0; fl_l = 0; fl_r = 0; r8 = rax; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r13); rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P9, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = (crate::simd_ops::ru64(r14)) as u64; r8 = (0xffffffffffffffffu64) as u64; rbx = (crate::simd_ops::ru64(rdi)) as u64; bb = 9; continue; }
            9 => { r8 = ((r8).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(r8))) as u8 as u64; fl_r = ((r13 as u8)) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 9; continue; } rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r9 = (crate::simd_ops::ru64(rdi)) as u64; r8 = rsp.wrapping_add(0x30); r9 = ((r9).wrapping_sub(0x18u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); rdx = rbx.wrapping_sub(0x30); crate::simd_ops::wu32(rsp.wrapping_add(0x40u64), (0x5u32) as u32); rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::lk_add_u64(rdi, (0xffffffffffffffe8u64) as u64); r14 = ((r14).wrapping_add(0x10u64)) as u64; rax = (crate::simd_ops::ru64(r14)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 8; continue; } r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; r11 = rsp.wrapping_add(0x50); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x38u64))) as u64; rax = 0x1; rbp = (crate::simd_ops::ru64(r11.wrapping_add(0x40u64))) as u64; rsp = r11; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            10 => { r8 = P3; rcx = rsi; rdx = P10; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P11, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::lift_ops::trap(0x80000003, P12, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_45595<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    fl_l = (crate::simd_ops::ru32(rsp.wrapping_add(0x58u64))) as u32 as u64; fl_r = 0x1;
    rbx = rdx;
    if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { rax = 0xffffffff; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
    fl_l = ((r9 as u32)) as u32 as u64; fl_r = 0x1;
    if ((((fl_l as u32)) as i32) > (((fl_r as u32)) as i32)) { rax = 0xffffffff; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
    fl_l = (crate::simd_ops::ru32(r8.wrapping_add(P0))) as u32 as u64; fl_r = 0x3;
    if !((((fl_l as u32)) != ((fl_r as u32)))) { xmm0 = ((crate::simd_ops::ru64(r8)) as u64) as u128; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rbx, ((xmm0 as u64)) as u64); rax = 0x1; crate::simd_ops::wu32(rbx.wrapping_add(P0), (0x3u32) as u32); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
    rax = 0xffffffff;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_45598<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    fl_l = (crate::simd_ops::ru32(rsp.wrapping_add(0x58u64))) as u32 as u64; fl_r = 0x2;
    rbx = rdx;
    if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { rax = 0xffffffff; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
    fl_l = ((r9 as u32)) as u32 as u64; fl_r = 0x1;
    if ((((fl_l as u32)) as i32) > (((fl_r as u32)) as i32)) { rax = 0xffffffff; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
    fl_l = (crate::simd_ops::ru32(r8.wrapping_add(P0))) as u32 as u64; fl_r = 0x3;
    if (((fl_l as u32)) != ((fl_r as u32))) { rax = 0xffffffff; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64;
    fl_l = (crate::simd_ops::ru32(rax.wrapping_add(P0))) as u32 as u64; fl_r = 0x3;
    if !((((fl_l as u32)) != ((fl_r as u32)))) { xmm1 = ((crate::simd_ops::ru64(rax)) as u64) as u128; xmm0 = ((crate::simd_ops::ru64(r8)) as u64) as u128; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rbx, ((xmm0 as u64)) as u64); rax = 0x1; crate::simd_ops::wu32(rbx.wrapping_add(P0), (0x3u32) as u32); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
    rax = 0xffffffff;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_45747<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    rax = P0;
    crate::simd_ops::wu64(rcx, rax);
    rcx = ((rcx).wrapping_add(0x8u64)) as u64;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_45748<const P0: u64, const P1: u64, const P2: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(P1))) as u32 as u64; rdi = ((rbx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 3; continue; } bb = 1; continue; }
            1 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } rdx = rsp.wrapping_add(0x30); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 2; continue; }
            2 => { rbx = ((rbx).wrapping_add(0x8u64)) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } bb = 3; continue; }
            3 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_45765<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (0u64, 0u64, 0u64);
    fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; fl_r = 0x19;
    rax = (rax & !0xffu64) | (((if (((fl_l as u32)) == ((fl_r as u32))) { 1u8 } else { 0u8 }) as u8 as u64));
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_45766<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (0u64, 0u64, 0u64);
    fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; fl_r = 0xd;
    rax = (rax & !0xffu64) | (((if ((((fl_l as u32)) as i32) >= (((fl_r as u32)) as i32)) { 1u8 } else { 0u8 }) as u8 as u64));
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_45769<const P0: u64, const P1: u64, const P2: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rdx;
    rdi = rcx;
    rdx = 0;
    fl_l = 0; fl_r = 0;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    fl_l = (crate::simd_ops::ru32(rdi.wrapping_add(P1))) as u32 as u64; fl_r = 0x19;
    if !((((fl_l as u32)) != ((fl_r as u32)))) { fl_l = rbx; fl_r = rbx; if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rdx = rbx; rcx = rdi; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); } rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rax = (rax & !0xffu64) | (((0u8) as u8 as u64));
    fl_l = 0; fl_r = 0;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_45782<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    crate::simd_ops::wu32(rcx.wrapping_add(P0), ((xmm1 as u32)) as u32);
    crate::simd_ops::wu8(rcx.wrapping_add(P1), (0x1u8) as u8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_45805<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    fl_l = rax; fl_r = rax;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { crate::simd_ops::wu8(rax.wrapping_add(P0), ((rdx as u8)) as u8); }
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_45806<const P0: u64, const P1: u64, const P2: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rcx; rcx = (crate::simd_ops::ru64(rcx)) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 1; continue; } fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; fl_r = 0xffffffff; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 1; continue; } rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 1; continue; }
            1 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx, (0x0u64) as u64); bb = 2; continue; }
            2 => { rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_45807<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    r8 = (crate::simd_ops::ru64(rcx)) as u64;
    fl_l = r8; fl_r = r8;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { xmm0 = (crate::simd_ops::ru128(rdx)) as u128; xmm1 = crate::simd_ops::vshufps(xmm0, xmm0, 0xd8u8); crate::simd_ops::wu64(r8.wrapping_add(P0), ((xmm1 as u64)) as u64); crate::simd_ops::wu32(r8.wrapping_add(P1), (((xmm1 >> 64) as u32) as u32) as u32); }
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_45848<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu32(rcx.wrapping_add(P0), ((rax as u32)) as u32);
    crate::simd_ops::wu32(rcx.wrapping_add(P1), ((rax as u32)) as u32);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_45886(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    fl_l = (crate::simd_ops::lk_add_u64(rcx, (0xffffffffffffffe8u64) as u64)) as u64; fl_r = fl_l;
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    crate::simd_ops::wu64(rdx, rax);
    rax = rdx;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_45887(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    fl_l = (crate::simd_ops::lk_add_u64(rcx, (0xffffffffffffffecu64) as u64)) as u64; fl_r = fl_l;
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    crate::simd_ops::wu64(rdx, rax);
    rax = rdx;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_45889<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64;
    rdx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64;
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    rax = ((rax.wrapping_add(rdx.wrapping_mul(8))) as u64) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_45890(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    fl_l = rax; fl_r = (0x14u64) as u64;
    rax = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_45891(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    fl_l = rax; fl_r = (0x18u64) as u64;
    rax = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64;
    return rax;
}
