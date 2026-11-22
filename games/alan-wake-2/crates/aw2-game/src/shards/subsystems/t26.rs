#![allow(unused_imports, unused_variables, unused_mut, unused_assignments, non_snake_case, dead_code, clippy::all, unreachable_code, unused_parens)]

#[inline(never)]
pub unsafe extern "win64" fn tpl_39816<const P0: u64, const P1: u64, const P2: u64, const P3: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x48u64)) as u64;
    rcx = P0;
    rax = r8;
    crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rcx);
    r9 = P1;
    rcx = P2;
    crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (0x0u64) as u64);
    crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rcx);
    rdx = ((rdx).wrapping_add(0x24u64)) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rcx);
    r8 = 0x4;
    rcx = rax;
    rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x48u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_39819<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x40u64)) as u64;
    r10 = rdx;
    rbx = P0;
    rdx = ((rcx.wrapping_add(P1)) as u64) as u64;
    rax = rcx;
    r11 = 0;
    fl_l = 0; fl_r = 0;
    rcx = P2;
    crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rcx);
    r9 = P3;
    crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), r11);
    rcx = r10;
    crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rbx);
    r8 = ((r11.wrapping_add(0x4u64)) as u32) as u32 as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbx);
    rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp);
    rdx = ((rax.wrapping_add(P5)) as u64) as u64;
    rax = P6;
    crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rax);
    r9 = P7;
    crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), r11);
    r8 = ((r11.wrapping_add(0x5u64)) as u32) as u32 as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rbx);
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbx);
    rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x40u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_39821<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x40u64)) as u64;
    r11 = 0;
    fl_l = 0; fl_r = 0;
    rcx = P0;
    crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rcx);
    rbx = P1;
    r10 = r8;
    crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), r11);
    rax = rdx;
    crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rbx);
    r8 = ((r11.wrapping_add(0x4u64)) as u32) as u32 as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbx);
    rdx = ((rdx).wrapping_add(0x24u64)) as u64;
    r9 = P2;
    rcx = r10;
    rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp);
    rdx = ((rax.wrapping_add(P4)) as u64) as u64;
    rax = P5;
    crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rax);
    r9 = P6;
    crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), r11);
    r8 = ((r11.wrapping_add(0x5u64)) as u32) as u32 as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rbx);
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbx);
    rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x40u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_39830<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x48u64)) as u64;
    rax = rdx;
    r9 = P0;
    rdx = ((rcx.wrapping_add(P1)) as u64) as u64;
    r8 = 0x2;
    rcx = P2;
    crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rcx);
    rcx = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rcx);
    crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rcx);
    rcx = P3;
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rcx);
    rcx = rax;
    rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x48u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_39831<const P0: u64, const P1: u64, const P2: u64, const P3: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x48u64)) as u64;
    rcx = P0;
    rax = r8;
    crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rcx);
    r9 = P1;
    rcx = 0;
    fl_l = 0; fl_r = 0;
    rdx = ((rdx).wrapping_add(0x8u64)) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rcx);
    r8 = 0x2;
    crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rcx);
    rcx = P2;
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rcx);
    rcx = rax;
    rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x48u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_39845<const P0: u64, const P1: u64, const P2: u64, const P3: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rdi = r8;
    rbx = rdx;
    rax = (crate::simd_ops::ru32(rdx)) as u32 as u64;
    crate::simd_ops::wu32(r8, ((rax as u32)) as u32);
    rcx = ((r8.wrapping_add(P0)) as u64) as u64;
    rdx = ((rdx).wrapping_add(0x8u64)) as u64;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rbx = ((rbx).wrapping_add(0x20u64)) as u64;
    rcx = ((rdi.wrapping_add(P2)) as u64) as u64;
    fl_l = rcx; fl_r = rbx;
    if !((((fl_l as u64)) == ((fl_r as u64)))) { rdx = (crate::simd_ops::ru64(rbx)) as u64; rax = (crate::simd_ops::ru32(rbx.wrapping_add(P0))) as u32 as u64; r8 = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; r8 = ((rdx.wrapping_add(r8.wrapping_mul(8))) as u64) as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); }
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_39846<const P0: u64, const P1: u64, const P2: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rdx;
    rcx = ((rdx.wrapping_add(P0)) as u64) as u64;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rax = 0;
    fl_l = 0; fl_r = 0;
    let tmp_xchg: u64 = crate::simd_ops::atomic_xchg_u64(rbx.wrapping_add(P2), rax);
    rax = (tmp_xchg) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_39848<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    fl_l = rdx; fl_r = rdx;
    if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { return rax; }
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rdi = rdx;
    rsi = 0;
    fl_l = 0; fl_r = 0;
    fl_l = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64; fl_r = ((rsi as u32)) as u32 as u64;
    if (((fl_l as u32)) <= ((fl_r as u32))) { rcx = rdi; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    loop { rbx = rsi; rbx = (((rbx) as i64).wrapping_mul((crate::simd_ops::ru64(rdi.wrapping_add(P1))) as i64) as u64) as u64; rcx = ((rdi.wrapping_add(P2)) as u64) as u64; rcx = ((rcx).wrapping_add(rbx)) as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rax = 0; fl_l = 0; fl_r = 0; let tmp_xchg: u64 = crate::simd_ops::atomic_xchg_u64(rbx.wrapping_add(rdi).wrapping_add(P4), rax); rax = (tmp_xchg) as u64; rsi = ((rsi).wrapping_add(1)) as u64; rax = (crate::simd_ops::ru32(rdi.wrapping_add(P0))) as u32 as u64; fl_l = rsi; fl_r = rax; if !((((fl_l as u64)) < ((fl_r as u64)))) { break; } }
    rcx = rdi;
    rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp);
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_39849<const P0: u64, const P1: u64, const P2: u64, const P3: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    fl_l = rdx; fl_r = rdx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rdx; rcx = ((rdx.wrapping_add(P0)) as u64) as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rax = 0; fl_l = 0; fl_r = 0; let tmp_xchg: u64 = crate::simd_ops::atomic_xchg_u64(rbx.wrapping_add(P2), rax); rax = (tmp_xchg) as u64; rcx = rbx; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); }
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_39850<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    crate::simd_ops::forward_incoming_to_outgoing();
    let (mut rax, mut r10, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64;
    r9 = ((rdx.wrapping_add(P1)) as u64) as u64;
    rcx = (crate::simd_ops::ru64(r8.wrapping_add(P2))) as u64;
    r10 = rdx;
    rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(r8))) as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(rdx).wrapping_add(P0), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P1))) as u32 as u64;
    rdx = ((rdx).wrapping_add(0x50u64)) as u64;
    rcx = (crate::simd_ops::ru64(r8.wrapping_add(P2))) as u64;
    r9 = ((r9).wrapping_sub(crate::simd_ops::ru64(r8))) as u64;
    crate::simd_ops::wu32(r9.wrapping_add(rcx), ((rax as u32)) as u32);
    rdx = ((rdx).wrapping_sub(crate::simd_ops::ru64(r8))) as u64;
    rcx = (crate::simd_ops::ru64(r8.wrapping_add(P2))) as u64;
    rax = (crate::simd_ops::ru32(r10.wrapping_add(P3))) as u32 as u64;
    crate::simd_ops::wu32(rdx.wrapping_add(rcx), ((rax as u32)) as u32);
    rcx = r10;
    rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(r8))) as u64;
    rdx = r10;
    fl_l = ((rcx).wrapping_add(crate::simd_ops::ru64(r8.wrapping_add(P2)))) as u64; fl_r = fl_l;
    rcx = ((fl_l as u64)) as u64;
    crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8(P4, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_39851<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    crate::simd_ops::forward_incoming_to_outgoing();
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(r8))) as u64;
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P1))) as u32 as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(rdx).wrapping_add(P1), ((rax as u32)) as u32);
    rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(r8))) as u64;
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P2))) as u32 as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(rdx).wrapping_add(P2), ((rax as u32)) as u32);
    rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(r8))) as u64;
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P3))) as u32 as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(rdx).wrapping_add(P3), ((rax as u32)) as u32);
    rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(r8))) as u64;
    fl_l = ((rcx).wrapping_add(rdx)) as u64; fl_r = fl_l;
    rcx = ((fl_l as u64)) as u64;
    crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8(P4, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_39857<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x38u64)) as u64;
    rcx = 0;
    fl_l = 0; fl_r = 0;
    r9 = P0;
    crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rcx);
    rax = r8;
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rcx);
    rdx = ((rdx).wrapping_add(0x18u64)) as u64;
    rcx = rax;
    r8 = 0;
    fl_l = 0; fl_r = 0;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x38u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_39870<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x38u64)) as u64;
    rcx = 0;
    fl_l = 0; fl_r = 0;
    r9 = P0;
    crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rcx);
    rax = r8;
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rcx);
    rdx = ((rdx).wrapping_add(0x6cu64)) as u64;
    rcx = rax;
    r8 = 0;
    fl_l = 0; fl_r = 0;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x38u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_39878<const P0: u64, const P1: u64, const P2: u64, const P3: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x40u64)) as u64;
    r9 = (crate::simd_ops::ru32(P0)) as u32 as u64;
    rbx = r8;
    r8 = (crate::simd_ops::ru64(P1)) as u64;
    rcx = rbx;
    crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), (0x4u32) as u32);
    crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), (0x4u32) as u32);
    rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp);
    rcx = rbx;
    rsp = ((rsp).wrapping_add(0x40u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return crate::simd_ops::tail_stk(P3, rax, rcx, rdx, r8, r9, rsp);
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_39879<const P0: u64, const P1: u64, const P2: u64, const P3: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x40u64)) as u64;
    r9 = (crate::simd_ops::ru32(P0)) as u32 as u64;
    rbx = r8;
    r8 = (crate::simd_ops::ru64(P1)) as u64;
    rcx = rbx;
    crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), (0x4u32) as u32);
    crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), (0x4u32) as u32);
    rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp);
    rcx = rbx;
    rsp = ((rsp).wrapping_add(0x40u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return crate::simd_ops::tail_stk(P3, rax, rcx, rdx, r8, r9, rsp);
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_39880<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    xmm0 = ((crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32) as u128;
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_39881<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rax = (crate::simd_ops::ru32(r8.wrapping_add(P0))) as u32 as u64;
    xmm0 = ((crate::simd_ops::ru64(r8)) as u64) as u128;
    crate::simd_ops::wu64(rdx, ((xmm0 as u64)) as u64);
    crate::simd_ops::wu32(rdx.wrapping_add(P0), ((rax as u32)) as u32);
    rax = rdx;
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_39892<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (0u64, 0u64, 0u64);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    xmm0 = ((crate::simd_ops::ru32(r8.wrapping_add(P0))) as u32) as u128;
    fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64;
    rax = (rax & !0xffu64) | (((if (f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { 1u8 } else { 0u8 }) as u8 as u64));
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_39898<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (0u64, 0u64, 0u64);
    rax = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64;
    rax = (((rax as u32)).wrapping_sub(1)) as u32 as u64;
    fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1;
    rax = (rax & !0xffu64) | (((if (((fl_l as u32)) <= ((fl_r as u32))) { 1u8 } else { 0u8 }) as u8 as u64));
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_39905<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
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
    crate::simd_ops::wu64(rdx.wrapping_add(P7), rax);
    rax = rdx;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_39908<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64, const P9: u64, const P10: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x48u64)) as u64;
    rax = rcx;
    r9 = P0;
    r10 = rdx;
    r11 = 0;
    fl_l = 0; fl_r = 0;
    rdx = ((rcx.wrapping_add(P1)) as u64) as u64;
    r8 = 0;
    fl_l = 0; fl_r = 0;
    rcx = P2;
    crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rcx);
    rcx = P3;
    crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), r11);
    crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rcx);
    rcx = P4;
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rcx);
    rcx = r10;
    rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp);
    rdx = ((rax.wrapping_add(P6)) as u64) as u64;
    rax = P7;
    crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rax);
    r9 = P8;
    rax = P9;
    crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), r11);
    crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax);
    r8 = ((r11.wrapping_add(0x1u64)) as u32) as u32 as u64;
    rax = P10;
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax);
    rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x48u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_39909<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64, const P9: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x48u64)) as u64;
    rcx = P0;
    r10 = r8;
    crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rcx);
    r9 = P1;
    rcx = P2;
    r11 = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), r11);
    rax = rdx;
    crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rcx);
    rdx = ((rdx).wrapping_add(0x8u64)) as u64;
    rcx = P3;
    r8 = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rcx);
    rcx = r10;
    rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp);
    rdx = ((rax.wrapping_add(P5)) as u64) as u64;
    rax = P6;
    crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rax);
    r9 = P7;
    rax = P8;
    crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), r11);
    crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax);
    r8 = ((r11.wrapping_add(0x1u64)) as u32) as u32 as u64;
    rax = P9;
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax);
    rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x48u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_39918<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x48u64)) as u64;
    rcx = P0;
    rax = r8;
    crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rcx);
    r9 = P1;
    rcx = P2;
    crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (0x0u64) as u64);
    crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rcx);
    rdx = ((rdx).wrapping_add(0x18u64)) as u64;
    rcx = P3;
    r8 = 0x2;
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rcx);
    rcx = rax;
    rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x48u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_39930<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    crate::simd_ops::forward_incoming_to_outgoing();
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64;
    rcx = ((r8.wrapping_add(P1)) as u64) as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P0), ((rax as u32)) as u32);
    rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P2))) as u32) as u32 as u64;
    crate::simd_ops::wu8(r8.wrapping_add(P2), ((rax as u8)) as u8);
    rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P3))) as u32) as u32 as u64;
    crate::simd_ops::wu8(r8.wrapping_add(P3), ((rax as u8)) as u8);
    rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P4))) as u32) as u32 as u64;
    crate::simd_ops::wu8(r8.wrapping_add(P4), ((rax as u8)) as u8);
    rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P5))) as u32) as u32 as u64;
    fl_l = ((rdx).wrapping_add(0x10u64)) as u64; fl_r = fl_l;
    rdx = ((fl_l as u64)) as u64;
    crate::simd_ops::wu8(r8.wrapping_add(P5), ((rax as u8)) as u8);
    crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8(P6, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_39942<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x38u64)) as u64;
    rdx = ((rdx).wrapping_add(0x210u64)) as u64;
    r9 = P0;
    rcx = r8;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x38u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_39943<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rcx = rdx;
    rdi = r8;
    rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(r8))) as u64;
    rbx = rdx;
    rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(r8.wrapping_add(P0)))) as u64;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rdx = ((rbx.wrapping_add(P2)) as u64) as u64;
    rcx = rdx;
    rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(rdi))) as u64;
    rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rdi.wrapping_add(P0)))) as u64;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rdx = ((rbx.wrapping_add(P3)) as u64) as u64;
    rcx = rdi;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return crate::simd_ops::tail_stk(P4, rax, rcx, rdx, r8, r9, rsp);
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_39947<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x38u64)) as u64;
    rdx = ((rdx).wrapping_add(0x28u64)) as u64;
    rcx = r8;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x38u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_39950<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x40u64)) as u64;
    r9 = (crate::simd_ops::ru32(P0)) as u32 as u64;
    rdi = r8;
    r8 = (crate::simd_ops::ru64(P1)) as u64;
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((rax as u32)) as u32);
    rcx = rdi;
    crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rax as u32)) as u32);
    rbx = rdx;
    rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp);
    rdx = rbx;
    rcx = rdi;
    rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp);
    rcx = rdi;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64;
    rsp = ((rsp).wrapping_add(0x40u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return crate::simd_ops::tail_stk(P4, rax, rcx, rdx, r8, r9, rsp);
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_39953<const P0: u64, const P1: u64, const P2: u64, const P3: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rax = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    rax = ((rax).wrapping_sub(crate::simd_ops::ru64(r8))) as u64;
    xmm0 = ((crate::simd_ops::ru64(rdx)) as u64) as u128;
    rcx = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64;
    crate::simd_ops::wu64(rax.wrapping_add(rdx), ((xmm0 as u64)) as u64);
    crate::simd_ops::wu32(rax.wrapping_add(rdx).wrapping_add(P0), ((rcx as u32)) as u32);
    rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P1))) as u32 as u64;
    fl_l = rcx; fl_r = (crate::simd_ops::ru64(r8)) as u64;
    rcx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(rdx).wrapping_add(P1), ((rax as u32)) as u32);
    xmm0 = ((crate::simd_ops::ru64(rdx.wrapping_add(P2))) as u64) as u128;
    crate::simd_ops::wu64(rcx.wrapping_add(rdx).wrapping_add(P2), ((xmm0 as u64)) as u64);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P3))) as u32 as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(rdx).wrapping_add(P3), ((rax as u32)) as u32);
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_39955<const P0: u64, const P1: u64, const P2: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut r10, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    rax = (crate::simd_ops::ru32(rdx)) as u32 as u64;
    r10 = rdx;
    rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    r9 = rdx;
    r9 = ((r9).wrapping_sub(crate::simd_ops::ru64(r8))) as u64;
    crate::simd_ops::wu32(r9.wrapping_add(rcx), ((rax as u32)) as u32);
    r9 = ((rdx.wrapping_add(P1)) as u64) as u64;
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P1))) as u32 as u64;
    rdx = ((rdx).wrapping_add(0x8u64)) as u64;
    rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    r9 = ((r9).wrapping_sub(crate::simd_ops::ru64(r8))) as u64;
    crate::simd_ops::wu32(r9.wrapping_add(rcx), ((rax as u32)) as u32);
    rdx = ((rdx).wrapping_sub(crate::simd_ops::ru64(r8))) as u64;
    rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    rax = (crate::simd_ops::ru32(r10.wrapping_add(P0))) as u32 as u64;
    crate::simd_ops::wu32(rdx.wrapping_add(rcx), ((rax as u32)) as u32);
    rdx = ((r10.wrapping_add(P2)) as u64) as u64;
    fl_l = rdx; fl_r = (crate::simd_ops::ru64(r8)) as u64;
    rdx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64;
    rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    rax = (crate::simd_ops::ru32(r10.wrapping_add(P2))) as u32 as u64;
    crate::simd_ops::wu32(rdx.wrapping_add(rcx), ((rax as u32)) as u32);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_39957<const P0: u64, const P1: u64, const P2: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x38u64)) as u64;
    rcx = 0;
    fl_l = 0; fl_r = 0;
    r9 = P0;
    rax = r8;
    crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rcx);
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rcx);
    rdx = ((rdx).wrapping_add(0x88u64)) as u64;
    r8 = ((rcx.wrapping_add(P1)) as u32) as u32 as u64;
    rcx = rax;
    rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x38u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_39962<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = (crate::simd_ops::ru64(rdx)) as u64;
    r9 = rdx;
    rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    r9 = ((r9).wrapping_sub(crate::simd_ops::ru64(r8))) as u64;
    crate::simd_ops::wu64(r9.wrapping_add(rcx), rax);
    rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(r8))) as u64;
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(rdx).wrapping_add(P0), ((rax as u32)) as u32);
    rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    fl_l = rcx; fl_r = (crate::simd_ops::ru64(r8)) as u64;
    rcx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64;
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(P1))) as u64;
    crate::simd_ops::wu64(rcx.wrapping_add(rdx).wrapping_add(P1), rax);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_39963<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x40u64)) as u64;
    rbx = r8;
    crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), (0x4cu32) as u32);
    r8 = (crate::simd_ops::ru64(P0)) as u64;
    rcx = rbx;
    crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), (0x4cu32) as u32);
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
pub unsafe extern "win64" fn tpl_39964<const P0: u64, const P1: u64, const P2: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x38u64)) as u64;
    rcx = 0;
    fl_l = 0; fl_r = 0;
    r9 = P0;
    rax = r8;
    crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rcx);
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rcx);
    rdx = ((rdx).wrapping_add(0x90u64)) as u64;
    r8 = ((rcx.wrapping_add(P1)) as u32) as u32 as u64;
    rcx = rax;
    rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x38u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_39965<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x40u64)) as u64;
    rbx = r8;
    crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), (0x7u32) as u32);
    r8 = (crate::simd_ops::ru64(P0)) as u64;
    rcx = rbx;
    crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), (0x7u32) as u32);
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
pub unsafe extern "win64" fn tpl_39967<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x28u64)) as u64; rax = r8; rcx = rdx; rdx = rax; r8 = (((r9 as u8)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, P1, rcx, rsp); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_39968<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x28u64)) as u64; rdx = r8; r8 = (((r9 as u8)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, P1, rcx, rsp); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_39969<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = P0;
    crate::simd_ops::wu64(rdx, rax);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_39971<const P0: u64, const P1: u64, const P2: u64, const P3: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    crate::simd_ops::forward_incoming_to_outgoing();
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    fl_l = rdx; fl_r = rdx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rcx = 0; fl_l = 0; fl_r = 0; fl_l = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { rcx = rdx; crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8(P3, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); } r8 = P1; loop { rax = rcx; rcx = ((rcx).wrapping_add(1)) as u64; rax = (((rax) as i64).wrapping_mul((crate::simd_ops::ru64(rdx.wrapping_add(P2))) as i64) as u64) as u64; crate::simd_ops::wu64(rax.wrapping_add(rdx), r8); rax = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64; fl_l = rcx; fl_r = rax; if !((((fl_l as u64)) < ((fl_r as u64)))) { break; } } rcx = rdx; crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8(P3, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_39972<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    crate::simd_ops::forward_incoming_to_outgoing();
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    fl_l = rdx; fl_r = rdx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rax = P0; rcx = rdx; crate::simd_ops::wu64(rdx, rax); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8(P1, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_39973<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x38u64)) as u64;
    rdx = ((rdx).wrapping_add(0x18u64)) as u64;
    rcx = r8;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x38u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_39980<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x30u64)) as u64;
    rdi = r8;
    rbx = rdx;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rax = 0;
    fl_l = 0; fl_r = 0;
    rdx = ((rbx.wrapping_add(P2)) as u64) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax);
    r9 = P3;
    rcx = rdi;
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax);
    r8 = ((rax.wrapping_add(P4)) as u32) as u32 as u64;
    rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp);
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64;
    rsp = ((rsp).wrapping_add(0x30u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_39983<const P0: u64, const P1: u64, const P2: u64, const P3: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
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
    r8 = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rcx);
    rcx = rax;
    rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x48u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_39985<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x48u64)) as u64;
    rax = r8;
    rcx = P0;
    crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rcx);
    r9 = P1;
    rcx = P2;
    rdx = ((rdx).wrapping_add(0x78u64)) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rcx);
    r8 = 0xd;
    rcx = P3;
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rcx);
    rcx = rax;
    rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x48u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_39991<const P0: u64, const P1: u64, const P2: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x38u64)) as u64;
    rax = r8;
    crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (0x0u64) as u64);
    rcx = P0;
    rdx = ((rdx).wrapping_add(0x4u64)) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rcx);
    r9 = P1;
    rcx = rax;
    r8 = 0;
    fl_l = 0; fl_r = 0;
    rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x38u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_39992<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x38u64)) as u64;
    rcx = 0;
    fl_l = 0; fl_r = 0;
    r9 = P0;
    crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rcx);
    rax = r8;
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rcx);
    rdx = ((rdx).wrapping_add(0x4u64)) as u64;
    rcx = rax;
    r8 = 0;
    fl_l = 0; fl_r = 0;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x38u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_39994<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
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
    rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp);
    rcx = ((rbx.wrapping_add(P6)) as u64) as u64;
    rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_39996<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64, const P9: u64, const P10: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = rdx; rsi = 0; fl_l = 0; fl_r = 0; fl_l = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64; fl_r = ((rsi as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rbx = rsi; rbx = (((rbx) as i64).wrapping_mul((crate::simd_ops::ru64(rdi.wrapping_add(P1))) as i64) as u64) as u64; rbx = ((rbx).wrapping_add(rdi)) as u64; rcx = ((rbx.wrapping_add(P2)) as u64) as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rcx = ((rbx.wrapping_add(P4)) as u64) as u64; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rcx = ((rbx.wrapping_add(P6)) as u64) as u64; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); rcx = ((rbx.wrapping_add(P8)) as u64) as u64; rax = crate::simd_ops::call_stk(P9, rax, rcx, rdx, r8, r9, rsp); rsi = ((rsi).wrapping_add(1)) as u64; rax = (crate::simd_ops::ru32(rdi.wrapping_add(P0))) as u32 as u64; fl_l = rsi; fl_r = rax; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 1; continue; } bb = 2; continue; }
            2 => { rcx = rdi; rax = crate::simd_ops::call_stk(P10, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); bb = 3; continue; }
            3 => { return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_39997<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    fl_l = rdx; fl_r = rdx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rdx; rcx = ((rdx.wrapping_add(P0)) as u64) as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rcx = ((rbx.wrapping_add(P2)) as u64) as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rcx = ((rbx.wrapping_add(P4)) as u64) as u64; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rcx = ((rbx.wrapping_add(P6)) as u64) as u64; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); rcx = rbx; rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); }
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_39998<const P0: u64, const P1: u64, const P2: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    crate::simd_ops::forward_incoming_to_outgoing();
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = (crate::simd_ops::ru32(rdx)) as u32 as u64;
    r9 = rdx;
    r9 = ((r9).wrapping_sub(crate::simd_ops::ru64(r8))) as u64;
    rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    crate::simd_ops::wu32(r9.wrapping_add(rcx), ((rax as u32)) as u32);
    r9 = ((rdx.wrapping_add(P1)) as u64) as u64;
    rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P1))) as u32 as u64;
    rdx = ((rdx).wrapping_add(0x8u64)) as u64;
    fl_l = r9; fl_r = (crate::simd_ops::ru64(r8)) as u64;
    r9 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64;
    crate::simd_ops::wu32(r9.wrapping_add(rcx), ((rax as u32)) as u32);
    rcx = r8;
    crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8(P2, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_40031<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    rsi = rdx;
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    rdi = ((rbx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64;
    fl_l = rbx; fl_r = rdi;
    if (((fl_l as u64)) == ((fl_r as u64))) { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    loop { rdx = (crate::simd_ops::ru64(rbx)) as u64; rcx = rsi; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rbx = ((rbx).wrapping_add(0x8u64)) as u64; fl_l = rbx; fl_r = rdi; if !((((fl_l as u64)) != ((fl_r as u64)))) { break; } }
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_40041<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    fl_l = rdx; fl_r = rdx;
    if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { return rax; }
    rax = P0;
    loop { crate::simd_ops::wu64(rcx, rax); rcx = ((rcx.wrapping_add(P1)) as u64) as u64; fl_l = rdx; fl_r = (0x1u64) as u64; rdx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if !((((fl_l as u64)) != ((fl_r as u64)))) { break; } }
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_40049<const P0: u64, const P1: u64, const P2: u64, const P3: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rcx;
    crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rcx);
    crate::simd_ops::lk_and_u32(rcx, (0xff000002u32) as u32);
    crate::simd_ops::lk_or_u32(rcx, (0x2u32) as u32);
    crate::simd_ops::wu8(rcx.wrapping_add(P0), (0x2u8) as u8);
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
pub unsafe extern "win64" fn tpl_40050<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    crate::simd_ops::forward_incoming_to_outgoing();
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rcx = (crate::simd_ops::ru64(rcx)) as u64;
    fl_l = rcx; fl_r = rcx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rdx = 0x20; crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8(P0, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_40051<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rcx;
    rcx = ((rcx).wrapping_add(0x8u64)) as u64;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rax = rbx;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_40056<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    crate::simd_ops::forward_incoming_to_outgoing();
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    crate::simd_ops::wu64(rcx, rdx);
    fl_l = rax; fl_r = rax;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rcx = rax; crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((crate::simd_ops::ru64(P0)) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_40063<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    r8 = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu64(rcx, r8);
    crate::simd_ops::wu64(rcx.wrapping_add(P0), r8);
    rax = (crate::simd_ops::ru64(rdx)) as u64;
    crate::simd_ops::wu64(rcx, rax);
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    crate::simd_ops::wu64(rcx.wrapping_add(P0), rax);
    crate::simd_ops::wu64(rdx, r8);
    crate::simd_ops::wu64(rdx.wrapping_add(P0), r8);
    crate::simd_ops::wu64(rcx.wrapping_add(P1), r8);
    crate::simd_ops::wu64(rcx.wrapping_add(P2), r8);
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(P1))) as u64;
    crate::simd_ops::wu64(rcx.wrapping_add(P1), rax);
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(P2))) as u64;
    crate::simd_ops::wu64(rcx.wrapping_add(P2), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(P1), r8);
    crate::simd_ops::wu64(rdx.wrapping_add(P2), r8);
    crate::simd_ops::wu64(rcx.wrapping_add(P3), r8);
    crate::simd_ops::wu64(rcx.wrapping_add(P4), r8);
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(P3))) as u64;
    crate::simd_ops::wu64(rcx.wrapping_add(P3), rax);
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(P4))) as u64;
    crate::simd_ops::wu64(rcx.wrapping_add(P4), rax);
    rax = rcx;
    crate::simd_ops::wu64(rdx.wrapping_add(P3), r8);
    crate::simd_ops::wu64(rdx.wrapping_add(P4), r8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_40069<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
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
    rax = rcx;
    crate::simd_ops::wu64(rdx, r8);
    crate::simd_ops::wu64(rdx.wrapping_add(P0), r8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_40071<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rdi = rdx;
    r8 = ((rdx.wrapping_add(P0)) as u64) as u64;
    rbx = rcx;
    rdx = 0xffffff80;
    rcx = (crate::simd_ops::ru64(rcx)) as u64;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rax = (crate::simd_ops::ru64(rbx)) as u64;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    crate::simd_ops::wu8(rdi.wrapping_add(rax), (0xffu8) as u8);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_40074<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 0>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    r10 = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    r9 = 0;
    fl_l = 0; fl_r = 0;
    r11 = (crate::simd_ops::ru64(rcx)) as u64;
    r8 = ((r8).wrapping_shr(7u32)) as u64;
    r8 = (((r8) & (r10))) as u64;
    xmm1 = crate::simd_ops::vpcmpeqd(xmm1, xmm1);
    xmm0 = crate::simd_ops::vpcmpgtb(xmm1, crate::simd_ops::ru128(r11.wrapping_add(r8)));
    rax = (crate::simd_ops::vpmovmskb(xmm0) as u32) as u32 as u64;
    fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64;
    if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { let bit_in: u32 = (rax as u32); if bit_in != 0 { rax = ((bit_in.trailing_zeros() as u32)) as u32 as u64; } fl_l = bit_in as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x8u64), (0x0u32) as u32); rax = ((rax).wrapping_add(r8)) as u64; crate::simd_ops::wu64(rdx.wrapping_add(P1), r9); fl_l = (((rax) & (r10))) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; crate::simd_ops::wu64(rdx, rax); rax = rdx; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
    loop { r9 = ((r9).wrapping_add(0x10u64)) as u64; r8 = ((r8).wrapping_add(r9)) as u64; r8 = (((r8) & (r10))) as u64; xmm0 = crate::simd_ops::vpcmpgtb(xmm1, crate::simd_ops::ru128(r11.wrapping_add(r8))); rax = (crate::simd_ops::vpmovmskb(xmm0) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if !(((((fl_l as u32)) & ((fl_r as u32))) == 0)) { break; } }
    let bit_in: u32 = (rax as u32);
    if bit_in != 0 { rax = ((bit_in.trailing_zeros() as u32)) as u32 as u64; }
    fl_l = bit_in as u64;
    crate::simd_ops::wu32(rsp.wrapping_add(0x8u64), (0x0u32) as u32);
    rax = ((rax).wrapping_add(r8)) as u64;
    crate::simd_ops::wu64(rdx.wrapping_add(P1), r9);
    fl_l = (((rax) & (r10))) as u64; fl_r = fl_l;
    rax = ((fl_l as u64)) as u64;
    crate::simd_ops::wu64(rdx, rax);
    rax = rdx;
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_40075<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rcx;
    rcx = ((rcx).wrapping_add(0x38u64)) as u64;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rcx = ((rbx.wrapping_add(P1)) as u64) as u64;
    rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp);
    rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P3))) as u64;
    fl_l = rdx; fl_r = rdx;
    if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    rax = 0xffffffff;
    let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rdx.wrapping_add(P4), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64;
    rax = (xa_old) as u32 as u64;
    fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1;
    if (((fl_l as u32)) != ((fl_r as u32))) { rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P5))) as u64;
    fl_l = rcx; fl_r = rcx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); }
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_40076<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rcx;
    rcx = ((rcx).wrapping_add(0x38u64)) as u64;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rcx = ((rbx.wrapping_add(P1)) as u64) as u64;
    rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp);
    rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P3))) as u64;
    fl_l = rdx; fl_r = rdx;
    if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { rax = rbx; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    rax = 0xffffffff;
    let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rdx.wrapping_add(P4), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64;
    rax = (xa_old) as u32 as u64;
    fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1;
    if (((fl_l as u32)) != ((fl_r as u32))) { rax = rbx; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P5))) as u64;
    fl_l = rcx; fl_r = rcx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); }
    rax = rbx;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_40077<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    crate::simd_ops::forward_incoming_to_outgoing();
    let (mut rax, mut fl_l) = (crate::simd_ops::get_rax_in(), 0u64);
    rcx = ((rdx.wrapping_add(rdx.wrapping_mul(2))) as u64) as u64;
    rdx = 0x4;
    fl_l = ((rcx).wrapping_shl(2u32)) as u64;
    rcx = ((fl_l as u64)) as u64;
    crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8(P0, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_40081(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    fl_l = (crate::simd_ops::lk_add_u64(rcx, (0xfffffffffffffff4u64) as u64)) as u64; fl_r = fl_l;
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    crate::simd_ops::wu64(rdx, rax);
    rax = rdx;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_40082(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    fl_l = rax; fl_r = (0xcu64) as u64;
    rax = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_40083<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = (((crate::simd_ops::ru8(rcx.wrapping_add(P0))) as i8 as i64) as u64) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_40084<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    crate::simd_ops::wu8(rdx.wrapping_add(rax), ((r8 as u8)) as u8);
    rdx = ((rdx).wrapping_add(0xfffffffffffffff0u64)) as u64;
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    rdx = (((rdx) & (rax))) as u64;
    rax = ((((rax as u32)) & (0xfu32))) as u32 as u64;
    fl_l = ((rdx).wrapping_add(rax)) as u64; fl_r = fl_l;
    rdx = ((fl_l as u64)) as u64;
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    crate::simd_ops::wu8(rdx.wrapping_add(rax).wrapping_add(P1), ((r8 as u8)) as u8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_40085<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = ((rdx.wrapping_add(P0)) as u64) as u64;
    crate::simd_ops::wu64(rcx.wrapping_add(P1), rdx);
    crate::simd_ops::wu64(rcx, rax);
    rax = rcx;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_40092<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    fl_l = ((rax).wrapping_add(crate::simd_ops::ru64(rcx))) as u64; fl_r = fl_l;
    rax = ((fl_l as u64)) as u64;
    crate::simd_ops::wu64(rdx, rax);
    rax = rdx;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_40096(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    fl_l = (crate::simd_ops::lk_add_u64(rcx, (0xcu64) as u64)) as u64; fl_r = fl_l;
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    crate::simd_ops::wu64(rdx, rax);
    rax = rdx;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_40098<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    crate::simd_ops::wu64(rcx, rdx);
    rax = rcx;
    crate::simd_ops::wu64(rcx.wrapping_add(P0), r8);
    xmm0 = (crate::simd_ops::ru128(r9)) as u128;
    crate::simd_ops::wu128(rcx.wrapping_add(P1), (xmm0) as u128);
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_40099(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = 0;
    rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64));
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_40107<const P0: u64, const P1: u64, const P2: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    crate::simd_ops::forward_incoming_to_outgoing();
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    fl_l = (crate::simd_ops::lk_and_u8(rcx.wrapping_add(P0), (0xfdu8) as u8)) as u8 as u64; fl_r = fl_l;
    rdx = rcx;
    rcx = (crate::simd_ops::ru64(P1)) as u64;
    crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8(P2, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_40124<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    r8 = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu64(rcx, r8);
    crate::simd_ops::wu64(rcx.wrapping_add(P0), r8);
    rax = (crate::simd_ops::ru64(rdx)) as u64;
    crate::simd_ops::wu64(rcx, rax);
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    crate::simd_ops::wu64(rcx.wrapping_add(P0), rax);
    rax = rcx;
    crate::simd_ops::wu64(rdx, r8);
    crate::simd_ops::wu64(rdx.wrapping_add(P0), r8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_40126<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    fl_l = rax; fl_r = rax;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { fl_l = (crate::simd_ops::atomic_add_u32(rax.wrapping_add(P0), 1)) as u32 as u64; }
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_40131<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64, const P9: u64, const P10: u64, const P11: u64, const P12: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = rdx; rbx = rcx; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rdx = (crate::simd_ops::ru64(rbx)) as u64; rsi = rax; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } rcx = (crate::simd_ops::ru32(rbx.wrapping_add(P1))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 1; continue; } r8 = ((rcx as u32)) as u32 as u64; rcx = rax; r8 = ((r8).wrapping_shl(4u32)) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(P3))) as u32 as u64; fl_r = 0x40000000; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 5; continue; } r9 = (crate::simd_ops::ru64(rbx)) as u64; fl_l = r9; fl_r = r9; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } rcx = ((r9.wrapping_add(P4)) as u64) as u64; rax = r9; rcx = (((rcx) & (0xfffffffffe000000u64))) as u64; rax = ((rax).wrapping_sub(rcx)) as u64; rax = ((rax).wrapping_shr(16u32)) as u64; rdx = ((rcx.wrapping_add(P5)) as u64) as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(P6))) as u32 as u64; rdx = ((rdx).wrapping_sub(rax)) as u64; rax = ((crate::exe::teb_self() as u64)) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(rcx.wrapping_add(P7))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(P8))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 2; continue; } rax = (crate::simd_ops::ru64(rdx.wrapping_add(P9))) as u64; crate::simd_ops::wu64(r9, rax); fl_l = (crate::simd_ops::lk_add_u32(rdx.wrapping_add(P10), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; crate::simd_ops::wu64(rdx.wrapping_add(P9), r9); if (((fl_l as u32)) != 0) { bb = 5; continue; } rcx = rdx; rax = crate::simd_ops::call_stk(P11, rax, rcx, rdx, r8, r9, rsp); bb = 5; continue; }
            2 => { r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 4; continue; }
            3 => { r8 = 0; fl_l = 0; fl_r = 0; bb = 4; continue; }
            4 => { rax = crate::simd_ops::call_stk(P12, rax, rcx, rdx, r8, r9, rsp); bb = 5; continue; }
            5 => { crate::simd_ops::wu64(rbx, rsi); rdi = ((((rdi as u32)) & (0x3fffffffu32))) as u32 as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; crate::simd_ops::wu32(rbx.wrapping_add(0xcu64), ((rdi as u32)) as u32); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_40132<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64, const P9: u64, const P10: u64, const P11: u64, const P12: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = rdx; rbx = rcx; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rdx = (crate::simd_ops::ru64(rbx)) as u64; rsi = rax; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } rcx = (crate::simd_ops::ru32(rbx.wrapping_add(P1))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 1; continue; } r8 = ((rcx as u32)) as u32 as u64; rcx = rax; r8 = ((r8).wrapping_shl(3u32)) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(P3))) as u32 as u64; fl_r = 0x40000000; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 5; continue; } r9 = (crate::simd_ops::ru64(rbx)) as u64; fl_l = r9; fl_r = r9; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } rcx = ((r9.wrapping_add(P4)) as u64) as u64; rax = r9; rcx = (((rcx) & (0xfffffffffe000000u64))) as u64; rax = ((rax).wrapping_sub(rcx)) as u64; rax = ((rax).wrapping_shr(16u32)) as u64; rdx = ((rcx.wrapping_add(P5)) as u64) as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(P6))) as u32 as u64; rdx = ((rdx).wrapping_sub(rax)) as u64; rax = ((crate::exe::teb_self() as u64)) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(rcx.wrapping_add(P7))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(P8))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 2; continue; } rax = (crate::simd_ops::ru64(rdx.wrapping_add(P9))) as u64; crate::simd_ops::wu64(r9, rax); fl_l = (crate::simd_ops::lk_add_u32(rdx.wrapping_add(P10), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; crate::simd_ops::wu64(rdx.wrapping_add(P9), r9); if (((fl_l as u32)) != 0) { bb = 5; continue; } rcx = rdx; rax = crate::simd_ops::call_stk(P11, rax, rcx, rdx, r8, r9, rsp); bb = 5; continue; }
            2 => { r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 4; continue; }
            3 => { r8 = 0; fl_l = 0; fl_r = 0; bb = 4; continue; }
            4 => { rax = crate::simd_ops::call_stk(P12, rax, rcx, rdx, r8, r9, rsp); bb = 5; continue; }
            5 => { crate::simd_ops::wu64(rbx, rsi); rdi = ((((rdi as u32)) & (0x3fffffffu32))) as u32 as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; crate::simd_ops::wu32(rbx.wrapping_add(0xcu64), ((rdi as u32)) as u32); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_40133(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l) = (crate::simd_ops::get_rax_in(), 0u64);
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    rax = ((rax).wrapping_sub(crate::simd_ops::ru64(rdx))) as u64;
    fl_l = ((((rax) as i64).wrapping_shr(3u32) as u64)) as u64;
    rax = ((fl_l as u64)) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_40135<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rax = ((crate::exe::tls_pointer() as u64)) as u64; rbx = rcx; crate::simd_ops::wu64(rcx.wrapping_add(P0), (0x0u64) as u64); rcx = 0x6348; rdx = (crate::simd_ops::ru64(rax)) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(rdx))) as u64; r8 = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; rdx = (crate::simd_ops::ru64(r8.wrapping_add(P2))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } fl_l = ((rdx as u8)) as u8 as u64; fl_r = (0xfu8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 2; continue; } fl_l = (crate::simd_ops::lk_add_u32(r8.wrapping_add(P3), (1) as u32)) as u32 as u64; rax = (crate::simd_ops::ru64(rdx)) as u64; crate::simd_ops::wu64(r8.wrapping_add(P2), rax); bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rbx.wrapping_add(P0), rdx); rsp = ((rsp).wrapping_add(0x30u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            2 => { rdx = 0x38; r8 = ((rdx.wrapping_add(P4)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rdx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 1; continue; } rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P6)) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = rax; rdx = 0xdeadd00d; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P7)) as u64, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx.wrapping_add(P0), (0x0u64) as u64); rsp = ((rsp).wrapping_add(0x30u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_40136<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    crate::simd_ops::forward_incoming_to_outgoing();
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    rcx = ((rcx).wrapping_add(0x10u64)) as u64;
    rdx = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((crate::simd_ops::ru64(rax)) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_40138(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = (0x276276276276276u64) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_40139<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l) = (crate::simd_ops::get_rax_in(), 0u64);
    rdx = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64;
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    fl_l = (((rdx as u32)).wrapping_sub(1)) as u32 as u64;
    rdx = ((fl_l as u32)) as u32 as u64;
    rax = ((rax.wrapping_add(rdx.wrapping_mul(8))) as u64) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_40140<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rdx = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64;
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    rax = ((rax.wrapping_add(rdx.wrapping_mul(8))) as u64) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_40143<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    crate::simd_ops::wu64(rcx, rdx);
    rax = rcx;
    crate::simd_ops::wu32(rcx.wrapping_add(P0), ((r8 as u32)) as u32);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_40144<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    crate::simd_ops::wu8(rcx, ((rdx as u8)) as u8);
    rax = rcx;
    crate::simd_ops::wu32(rcx.wrapping_add(P0), ((r8 as u32)) as u32);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_40147<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P0))) as u32) as u32 as u64;
    crate::simd_ops::wu8(rcx.wrapping_add(P0), ((rax as u8)) as u8);
    rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P1))) as u32) as u32 as u64;
    crate::simd_ops::wu8(rcx.wrapping_add(P1), ((rax as u8)) as u8);
    rax = rcx;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_40148<const P0: u64, const P1: u64, const P2: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu8(rcx.wrapping_add(P0), (0x0u8) as u8);
    crate::simd_ops::wu64(rcx.wrapping_add(P1), rax);
    rax = P2;
    crate::simd_ops::wu64(rcx, rax);
    rax = rcx;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_40150<const P0: u64, const P1: u64, const P2: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = P0;
    crate::simd_ops::wu8(rcx.wrapping_add(P1), (0x0u8) as u8);
    crate::simd_ops::wu64(rcx, rax);
    rax = rcx;
    crate::simd_ops::wu8(rcx.wrapping_add(P2), (0x0u8) as u8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_40151<const P0: u64, const P1: u64, const P2: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(P0))) as u8 as u64; fl_r = (0x0u8) as u8 as u64;
    rbx = rcx;
    if !((((fl_l as u8)) == ((fl_r as u8)))) { rax = (crate::simd_ops::ru64(rcx)) as u64; r8 = rdx; rdx = rsp.wrapping_add(0x30); rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64, rax, rcx, rdx, r8, r9, rsp); rdx = (crate::simd_ops::ru32(rax)) as u32 as u64; fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(P2))) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; rax = (rax & !0xffu64) | (((if (((fl_l as u32)) == ((fl_r as u32))) { 1u8 } else { 0u8 }) as u8 as u64)); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64));
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_40152<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rdi = rdx; rbx = rcx; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64, rax, rcx, rdx, r8, r9, rsp); rdx = rax; rcx = rdi; rsi = rax; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } fl_l = (crate::simd_ops::ru16(rax.wrapping_add(P2))) as u16 as u64; fl_r = (0x0u16) as u16 as u64; if (((fl_l as u16)) == ((fl_r as u16))) { bb = 3; continue; } rcx = ((rax.wrapping_add(P3)) as u64) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } rdx = rsi; rcx = rdi; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P5))) as u64; rdx = (crate::simd_ops::ru32(rbx.wrapping_add(P6))) as u32 as u64; rdx = ((rcx.wrapping_add(rdx.wrapping_mul(8))) as u64) as u64; fl_l = rcx; fl_r = rdx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 3; continue; } bb = 1; continue; }
            1 => { r8 = (crate::simd_ops::ru64(rcx)) as u64; fl_l = ((r8 as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 2; continue; } rcx = ((rcx).wrapping_add(0x8u64)) as u64; fl_l = rcx; fl_r = rdx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            2 => { fl_l = ((r8).wrapping_shr(32u32)) as u64; r8 = ((fl_l as u64)) as u64; crate::simd_ops::wu32(rbx.wrapping_add(P3), ((r8 as u32)) as u32); crate::simd_ops::wu8(rbx.wrapping_add(P7), (0x1u8) as u8); bb = 3; continue; }
            3 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_40153<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64, const P9: u64, const P10: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x50u64)) as u64; r14 = rdx; rsi = rcx; rdi = (crate::simd_ops::ru64(rcx)) as u64; rax = (crate::simd_ops::ru64(rdi.wrapping_add(P0))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rsp.wrapping_add(0x38u64), ((rcx as u32)) as u32); r8 = rdi; fl_l = (crate::simd_ops::ru8(rax.wrapping_add(P1))) as u8 as u64; fl_r = ((rcx as u8)) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 4; continue; } r9 = (crate::simd_ops::ru32(rdx)) as u32 as u64; bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); fl_l = (crate::simd_ops::ru32(rax.wrapping_add(P2))) as u32 as u64; fl_r = ((r9 as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) >= (((fl_r as u32)) as i32)) { bb = 2; continue; } crate::simd_ops::wu32(rsp.wrapping_add(0x38u64), ((rcx as u32)) as u32); rax = (crate::simd_ops::ru64(rax.wrapping_add(P3))) as u64; bb = 3; continue; }
            2 => { crate::simd_ops::wu32(rsp.wrapping_add(0x38u64), (0x1u32) as u32); r8 = rax; rax = (crate::simd_ops::ru64(rax)) as u64; bb = 3; continue; }
            3 => { fl_l = (crate::simd_ops::ru8(rax.wrapping_add(P1))) as u8 as u64; fl_r = ((rcx as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 1; continue; } bb = 4; continue; }
            4 => { fl_l = (crate::simd_ops::ru8(r8.wrapping_add(P1))) as u8 as u64; fl_r = ((rcx as u8)) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 5; continue; } rax = (crate::simd_ops::ru32(r8.wrapping_add(P2))) as u32 as u64; fl_l = (crate::simd_ops::ru32(rdx)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) >= (((fl_r as u32)) as i32)) { bb = 6; continue; } bb = 5; continue; }
            5 => { rax = (0x492492492492492u64) as u64; fl_l = (crate::simd_ops::ru64(rsi.wrapping_add(P0))) as u64; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 7; continue; } crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rcx); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = rax; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax); rax = (crate::simd_ops::ru32(r14)) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(P2), ((rax as u32)) as u32); rcx = ((rbx.wrapping_add(P5)) as u64) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rbx, rdi); crate::simd_ops::wu64(rbx.wrapping_add(P0), rdi); crate::simd_ops::wu64(rbx.wrapping_add(P3), rdi); crate::simd_ops::wu16(rbx.wrapping_add(P7), (0x0u16) as u16); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x30u64))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128); r8 = rbx; rdx = rsp.wrapping_add(0x20); rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P8, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r8 = rax; bb = 6; continue; }
            6 => { rax = ((r8.wrapping_add(P5)) as u64) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rsp = ((rsp).wrapping_add(0x50u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            7 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P9, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::lift_ops::trap(0x80000003, P10, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_40188<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = (crate::simd_ops::ru32(r8.wrapping_add(P0))) as u32 as u64;
    crate::simd_ops::wu32(rdx, ((rax as u32)) as u32);
    rax = rdx;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_40189<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = ((rdx.wrapping_add(P0)) as u64) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_40192<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64, const P9: u64, const P10: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = rcx; crate::simd_ops::wu8(rcx.wrapping_add(P0), (0x0u8) as u8); r8 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rcx.wrapping_add(P1), ((r8 as u32)) as u32); crate::simd_ops::wu64(rcx.wrapping_add(P2), r8); crate::simd_ops::wu64(rcx.wrapping_add(P3), r8); rax = P4; crate::simd_ops::wu64(rcx, rax); crate::simd_ops::wu32(rsp.wrapping_add(0x38u64), ((r8 as u32)) as u32); rdx = (crate::simd_ops::ru32(rcx.wrapping_add(P3))) as u32 as u64; rcx = ((rdx as u32)) as u32 as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rdi.wrapping_add(P2)))) as u64; rax = (crate::simd_ops::ru32(rdi.wrapping_add(P5))) as u32 as u64; rax = ((((rax as u32)) & (0x3fffffffu32))) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 1; continue; } crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rcx); r9 = rsp.wrapping_add(0x38); rdx = rsp.wrapping_add(0x40); rcx = ((rdi.wrapping_add(P2)) as u64) as u64; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); bb = 2; continue; }
            1 => { rax = ((rdx.wrapping_add(P7)) as u32) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(P3), ((rax as u32)) as u32); rax = P8; crate::simd_ops::wu64(rcx, rax); crate::simd_ops::wu32(rcx.wrapping_add(P0), ((r8 as u32)) as u32); bb = 2; continue; }
            2 => { crate::simd_ops::wu32(rsp.wrapping_add(0x38u64), (0x1u32) as u32); rdx = (crate::simd_ops::ru32(rdi.wrapping_add(P3))) as u32 as u64; rcx = ((rdx as u32)) as u32 as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rdi.wrapping_add(P2)))) as u64; rax = (crate::simd_ops::ru32(rdi.wrapping_add(P5))) as u32 as u64; rax = ((((rax as u32)) & (0x3fffffffu32))) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 3; continue; } crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rcx); r9 = rsp.wrapping_add(0x38); rdx = rsp.wrapping_add(0x40); rcx = ((rdi.wrapping_add(P2)) as u64) as u64; rax = crate::simd_ops::call_stk(P9, rax, rcx, rdx, r8, r9, rsp); bb = 4; continue; }
            3 => { rax = ((rdx.wrapping_add(P7)) as u32) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(P3), ((rax as u32)) as u32); rax = P10; crate::simd_ops::wu64(rcx, rax); crate::simd_ops::wu32(rcx.wrapping_add(P0), (0x1u32) as u32); bb = 4; continue; }
            4 => { rax = rdi; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_40193<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = rcx; crate::simd_ops::wu8(rcx.wrapping_add(P0), (0x0u8) as u8); rsi = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rcx.wrapping_add(P1), ((rsi as u32)) as u32); crate::simd_ops::wu64(rcx.wrapping_add(P2), rsi); crate::simd_ops::wu64(rcx.wrapping_add(P3), rsi); rax = P4; crate::simd_ops::wu64(rcx, rax); crate::simd_ops::wu32(rsp.wrapping_add(0x50u64), (0x1u32) as u32); crate::simd_ops::wu8(rsp.wrapping_add(0x48u64), (0x1u8) as u8); rdx = (crate::simd_ops::ru32(rcx.wrapping_add(P3))) as u32 as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(P2))) as u64; rcx = ((rax.wrapping_add(rdx.wrapping_mul(8))) as u64) as u64; rax = (crate::simd_ops::ru32(rdi.wrapping_add(P5))) as u32 as u64; rax = ((((rax as u32)) & (0x3fffffffu32))) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 1; continue; } crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rcx); r9 = rsp.wrapping_add(0x50); r8 = rsp.wrapping_add(0x48); rdx = rsp.wrapping_add(0x58); rcx = ((rdi.wrapping_add(P2)) as u64) as u64; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); bb = 2; continue; }
            1 => { rax = ((rdx.wrapping_add(P7)) as u32) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(P3), ((rax as u32)) as u32); crate::simd_ops::wu8(rcx, (0x1u8) as u8); crate::simd_ops::wu32(rcx.wrapping_add(P8), (0x1u32) as u32); bb = 2; continue; }
            2 => { crate::simd_ops::wu32(rsp.wrapping_add(0x50u64), ((rsi as u32)) as u32); crate::simd_ops::wu8(rsp.wrapping_add(0x48u64), (0x0u8) as u8); rdx = (crate::simd_ops::ru32(rdi.wrapping_add(P3))) as u32 as u64; rax = (crate::simd_ops::ru64(rdi.wrapping_add(P2))) as u64; rcx = ((rax.wrapping_add(rdx.wrapping_mul(8))) as u64) as u64; rax = (crate::simd_ops::ru32(rdi.wrapping_add(P5))) as u32 as u64; rax = ((((rax as u32)) & (0x3fffffffu32))) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 3; continue; } crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rcx); r9 = rsp.wrapping_add(0x50); r8 = rsp.wrapping_add(0x48); rdx = rsp.wrapping_add(0x58); rcx = ((rdi.wrapping_add(P2)) as u64) as u64; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); bb = 4; continue; }
            3 => { rax = ((rdx.wrapping_add(P7)) as u32) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(P3), ((rax as u32)) as u32); crate::simd_ops::wu8(rcx, (0x0u8) as u8); crate::simd_ops::wu32(rcx.wrapping_add(P8), ((rsi as u32)) as u32); bb = 4; continue; }
            4 => { rax = rdi; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_40194<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = rcx; crate::simd_ops::wu8(rcx.wrapping_add(0x8u64), (0x0u8) as u8); r8 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rcx.wrapping_add(0x10u64), ((r8 as u32)) as u32); crate::simd_ops::wu64(rcx.wrapping_add(0x18u64), r8); crate::simd_ops::wu64(rcx.wrapping_add(0x20u64), r8); rax = P0; crate::simd_ops::wu64(rcx, rax); crate::simd_ops::wu32(rsp.wrapping_add(0x38u64), ((r8 as u32)) as u32); rdx = (crate::simd_ops::ru32(rcx.wrapping_add(0x20u64))) as u32 as u64; rcx = ((rdx as u32)) as u32 as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rdi.wrapping_add(0x18u64)))) as u64; rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x24u64))) as u32 as u64; rax = ((((rax as u32)) & (0x3fffffffu32))) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 1; continue; } crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rcx); r9 = rsp.wrapping_add(0x38); r8 = P1; rdx = rsp.wrapping_add(0x40); rcx = rdi.wrapping_add(0x18); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 2; continue; }
            1 => { rax = ((rdx.wrapping_add(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x20u64), ((rax as u32)) as u32); rax = P1; crate::simd_ops::wu64(rcx, rax); crate::simd_ops::wu32(rcx.wrapping_add(0x8u64), ((r8 as u32)) as u32); bb = 2; continue; }
            2 => { crate::simd_ops::wu32(rsp.wrapping_add(0x38u64), (0x1u32) as u32); rdx = (crate::simd_ops::ru32(rdi.wrapping_add(0x20u64))) as u32 as u64; rcx = ((rdx as u32)) as u32 as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rdi.wrapping_add(0x18u64)))) as u64; rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x24u64))) as u32 as u64; rax = ((((rax as u32)) & (0x3fffffffu32))) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 3; continue; } crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rcx); r9 = rsp.wrapping_add(0x38); r8 = P3; rdx = rsp.wrapping_add(0x40); rcx = rdi.wrapping_add(0x18); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 4; continue; }
            3 => { rax = ((rdx.wrapping_add(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x20u64), ((rax as u32)) as u32); rax = P3; crate::simd_ops::wu64(rcx, rax); crate::simd_ops::wu32(rcx.wrapping_add(0x8u64), (0x1u32) as u32); bb = 4; continue; }
            4 => { crate::simd_ops::wu32(rsp.wrapping_add(0x38u64), (0x2u32) as u32); rdx = (crate::simd_ops::ru32(rdi.wrapping_add(0x20u64))) as u32 as u64; rcx = ((rdx as u32)) as u32 as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rdi.wrapping_add(0x18u64)))) as u64; rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x24u64))) as u32 as u64; rax = ((((rax as u32)) & (0x3fffffffu32))) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 5; continue; } crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rcx); r9 = rsp.wrapping_add(0x38); rdx = rsp.wrapping_add(0x40); rcx = rdi.wrapping_add(0x18); rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); bb = 6; continue; }
            5 => { rax = ((rdx.wrapping_add(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x20u64), ((rax as u32)) as u32); rax = P5; crate::simd_ops::wu64(rcx, rax); crate::simd_ops::wu32(rcx.wrapping_add(0x8u64), (0x2u32) as u32); bb = 6; continue; }
            6 => { rax = rdi; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_40196<const P0: u64, const P1: u64, const P2: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = ((crate::simd_ops::ru8(rcx.wrapping_add(P0))) as u32) as u32 as u64;
    crate::simd_ops::wu8(rdx.wrapping_add(P0), ((rax as u8)) as u8);
    rax = (crate::simd_ops::ru32(rcx.wrapping_add(P1))) as u32 as u64;
    crate::simd_ops::wu32(rdx.wrapping_add(P1), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rcx.wrapping_add(P2))) as u32 as u64;
    crate::simd_ops::wu32(rdx.wrapping_add(P2), ((rax as u32)) as u32);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_40201<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = rcx; crate::simd_ops::wu8(rcx.wrapping_add(0x8u64), (0x0u8) as u8); r8 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rcx.wrapping_add(0x10u64), ((r8 as u32)) as u32); crate::simd_ops::wu64(rcx.wrapping_add(0x18u64), r8); crate::simd_ops::wu64(rcx.wrapping_add(0x20u64), r8); rax = P0; crate::simd_ops::wu64(rcx, rax); crate::simd_ops::wu32(rsp.wrapping_add(0x38u64), ((r8 as u32)) as u32); rdx = (crate::simd_ops::ru32(rcx.wrapping_add(0x20u64))) as u32 as u64; rcx = ((rdx as u32)) as u32 as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rdi.wrapping_add(0x18u64)))) as u64; rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x24u64))) as u32 as u64; rax = ((((rax as u32)) & (0x3fffffffu32))) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 1; continue; } crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rcx); r9 = rsp.wrapping_add(0x38); rdx = rsp.wrapping_add(0x40); rcx = rdi.wrapping_add(0x18); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 2; continue; }
            1 => { rax = ((rdx.wrapping_add(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x20u64), ((rax as u32)) as u32); rax = P2; crate::simd_ops::wu64(rcx, rax); crate::simd_ops::wu32(rcx.wrapping_add(0x8u64), ((r8 as u32)) as u32); bb = 2; continue; }
            2 => { crate::simd_ops::wu32(rsp.wrapping_add(0x38u64), (0x1u32) as u32); rdx = (crate::simd_ops::ru32(rdi.wrapping_add(0x20u64))) as u32 as u64; rcx = ((rdx as u32)) as u32 as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rdi.wrapping_add(0x18u64)))) as u64; rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x24u64))) as u32 as u64; rax = ((((rax as u32)) & (0x3fffffffu32))) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 3; continue; } crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rcx); r9 = rsp.wrapping_add(0x38); rdx = rsp.wrapping_add(0x40); rcx = rdi.wrapping_add(0x18); rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 4; continue; }
            3 => { rax = ((rdx.wrapping_add(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x20u64), ((rax as u32)) as u32); rax = P4; crate::simd_ops::wu64(rcx, rax); crate::simd_ops::wu32(rcx.wrapping_add(0x8u64), (0x1u32) as u32); bb = 4; continue; }
            4 => { crate::simd_ops::wu32(rsp.wrapping_add(0x38u64), (0x2u32) as u32); rdx = (crate::simd_ops::ru32(rdi.wrapping_add(0x20u64))) as u32 as u64; rcx = ((rdx as u32)) as u32 as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rdi.wrapping_add(0x18u64)))) as u64; rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x24u64))) as u32 as u64; rax = ((((rax as u32)) & (0x3fffffffu32))) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 5; continue; } crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rcx); r9 = rsp.wrapping_add(0x38); rdx = rsp.wrapping_add(0x40); rcx = rdi.wrapping_add(0x18); rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); bb = 6; continue; }
            5 => { rax = ((rdx.wrapping_add(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x20u64), ((rax as u32)) as u32); rax = P6; crate::simd_ops::wu64(rcx, rax); crate::simd_ops::wu32(rcx.wrapping_add(0x8u64), (0x2u32) as u32); bb = 6; continue; }
            6 => { rax = rdi; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_40202<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = rcx; crate::simd_ops::wu8(rcx.wrapping_add(0x8u64), (0x0u8) as u8); r8 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rcx.wrapping_add(0x10u64), ((r8 as u32)) as u32); crate::simd_ops::wu64(rcx.wrapping_add(0x18u64), r8); crate::simd_ops::wu64(rcx.wrapping_add(0x20u64), r8); rax = P0; crate::simd_ops::wu64(rcx, rax); crate::simd_ops::wu32(rsp.wrapping_add(0x38u64), ((r8 as u32)) as u32); rdx = (crate::simd_ops::ru32(rcx.wrapping_add(0x20u64))) as u32 as u64; rcx = ((rdx as u32)) as u32 as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rdi.wrapping_add(0x18u64)))) as u64; rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x24u64))) as u32 as u64; rax = ((((rax as u32)) & (0x3fffffffu32))) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 1; continue; } crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rcx); r9 = rsp.wrapping_add(0x38); rdx = rsp.wrapping_add(0x40); rcx = rdi.wrapping_add(0x18); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 2; continue; }
            1 => { rax = ((rdx.wrapping_add(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x20u64), ((rax as u32)) as u32); rax = P2; crate::simd_ops::wu64(rcx, rax); crate::simd_ops::wu32(rcx.wrapping_add(0x8u64), ((r8 as u32)) as u32); bb = 2; continue; }
            2 => { crate::simd_ops::wu32(rsp.wrapping_add(0x38u64), (0x1u32) as u32); rdx = (crate::simd_ops::ru32(rdi.wrapping_add(0x20u64))) as u32 as u64; rcx = ((rdx as u32)) as u32 as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rdi.wrapping_add(0x18u64)))) as u64; rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x24u64))) as u32 as u64; rax = ((((rax as u32)) & (0x3fffffffu32))) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 3; continue; } crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rcx); r9 = rsp.wrapping_add(0x38); rdx = rsp.wrapping_add(0x40); rcx = rdi.wrapping_add(0x18); rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 4; continue; }
            3 => { rax = ((rdx.wrapping_add(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x20u64), ((rax as u32)) as u32); rax = P4; crate::simd_ops::wu64(rcx, rax); crate::simd_ops::wu32(rcx.wrapping_add(0x8u64), (0x1u32) as u32); bb = 4; continue; }
            4 => { crate::simd_ops::wu32(rsp.wrapping_add(0x38u64), (0x2u32) as u32); rdx = (crate::simd_ops::ru32(rdi.wrapping_add(0x20u64))) as u32 as u64; rcx = ((rdx as u32)) as u32 as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rdi.wrapping_add(0x18u64)))) as u64; rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x24u64))) as u32 as u64; rax = ((((rax as u32)) & (0x3fffffffu32))) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 5; continue; } crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rcx); r9 = rsp.wrapping_add(0x38); rdx = rsp.wrapping_add(0x40); rcx = rdi.wrapping_add(0x18); rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); bb = 6; continue; }
            5 => { rax = ((rdx.wrapping_add(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x20u64), ((rax as u32)) as u32); rax = P6; crate::simd_ops::wu64(rcx, rax); crate::simd_ops::wu32(rcx.wrapping_add(0x8u64), (0x2u32) as u32); bb = 6; continue; }
            6 => { crate::simd_ops::wu32(rsp.wrapping_add(0x38u64), (0x3u32) as u32); rdx = (crate::simd_ops::ru32(rdi.wrapping_add(0x20u64))) as u32 as u64; rcx = ((rdx as u32)) as u32 as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rdi.wrapping_add(0x18u64)))) as u64; rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x24u64))) as u32 as u64; rax = ((((rax as u32)) & (0x3fffffffu32))) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 7; continue; } crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rcx); r9 = rsp.wrapping_add(0x38); rdx = rsp.wrapping_add(0x40); rcx = rdi.wrapping_add(0x18); rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); bb = 8; continue; }
            7 => { rax = ((rdx.wrapping_add(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x20u64), ((rax as u32)) as u32); rax = P8; crate::simd_ops::wu64(rcx, rax); crate::simd_ops::wu32(rcx.wrapping_add(0x8u64), (0x3u32) as u32); bb = 8; continue; }
            8 => { rax = rdi; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_40233<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    crate::simd_ops::forward_incoming_to_outgoing();
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    r8 = (crate::simd_ops::ru64(r8)) as u64;
    crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8(P0, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_40240(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (0u64, 0u64, 0u64);
    rax = (crate::simd_ops::ru32(rcx)) as u32 as u64;
    fl_l = (crate::simd_ops::ru32(rdx)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64;
    rax = (rax & !0xffu64) | (((if (((fl_l as u32)) == ((fl_r as u32))) { 1u8 } else { 0u8 }) as u8 as u64));
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_40267<const P0: u64, const P1: u64, const P2: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rbx = rcx; r8 = rdx; rdx = rcx; rcx = rsp.wrapping_add(0x20); rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); r8 = (((crate::simd_ops::ru32(rbx.wrapping_add(P1))) as i32 as i64) as u64) as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; fl_l = ((r8 as u32)) as u32 as u64; fl_r = 0xffffffff; if ((((fl_l as u32)) as i32) <= (((fl_r as u32)) as i32)) { bb = 2; continue; } rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x28u64))) as u32 as u64; rdx = ((rcx.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rdx = ((rdx).wrapping_sub(rcx)) as u64; rdx = ((((rdx) as i64).wrapping_shr(2u32) as u64)) as u64; fl_l = r8; fl_r = rdx; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 2; continue; } rbx = (crate::simd_ops::ru32(rcx.wrapping_add(r8.wrapping_mul(4)))) as u32 as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 1; continue; } rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 1; continue; }
            1 => { rax = ((rbx as u32)) as u32 as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            2 => { fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 3; continue; }
            3 => { rax = 0xffffffff; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_40268<const P0: u64, const P1: u64, const P2: u64, const P3: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rdi) = (0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rbx = ((r8 as u32)) as u32 as u64; rdi = rcx; r8 = rdx; rdx = rcx; rcx = rsp.wrapping_add(0x20); rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rax = 0; fl_l = 0; fl_r = 0; rdx = (crate::simd_ops::ru32(rsp.wrapping_add(0x28u64))) as u32 as u64; r9 = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; r10 = ((r9.wrapping_add(rdx.wrapping_mul(4))) as u64) as u64; r10 = ((r10).wrapping_sub(r9)) as u64; r10 = ((((r10) as i64).wrapping_shr(2u32) as u64)) as u64; fl_l = r10; fl_r = r10; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } rcx = ((rax as u32)) as u32 as u64; bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru32(r9.wrapping_add(rcx.wrapping_mul(4)))) as u32 as u64; fl_r = ((rbx as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 4; continue; } rax = (((rax as u32)).wrapping_add(1)) as u32 as u64; rcx = ((((rax as u32)) as i32 as i64) as u64) as u64; fl_l = rcx; fl_r = r10; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 1; continue; } bb = 2; continue; }
            2 => { fl_l = r9; fl_r = r9; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } rcx = r9; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 3; continue; }
            3 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            4 => { crate::simd_ops::wu32(rdi.wrapping_add(P2), ((rax as u32)) as u32); crate::simd_ops::wu16(rdi.wrapping_add(P3), (0x101u16) as u16); fl_l = r9; fl_r = r9; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } rcx = r9; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 5; continue; }
            5 => { rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_40270<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    crate::simd_ops::forward_incoming_to_outgoing();
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    r9 = rcx;
    rax = ((r8 as u32)) as u32 as u64;
    rcx = (((rax) as i64).wrapping_mul((0x58u64) as i64) as u64) as u64;
    fl_l = ((rcx).wrapping_add(rdx)) as u64; fl_r = fl_l;
    rcx = ((fl_l as u64)) as u64;
    rdx = r9;
    crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8(P0, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_40271<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rcx;
    rax = (crate::simd_ops::ru32(rdx)) as u32 as u64;
    crate::simd_ops::wu32(rcx, ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    crate::simd_ops::wu64(rcx.wrapping_add(P0), rax);
    xmm0 = ((crate::simd_ops::ru64(rdx.wrapping_add(P1))) as u64) as u128;
    crate::simd_ops::wu64(rcx.wrapping_add(P1), ((xmm0 as u64)) as u64);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P2))) as u32 as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(P2), ((rax as u32)) as u32);
    rcx = ((rcx).wrapping_add(0x18u64)) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rcx);
    crate::simd_ops::lk_and_u32(rcx, (0xff000004u32) as u32);
    crate::simd_ops::lk_or_u32(rcx, (0x4u32) as u32);
    crate::simd_ops::wu8(rcx.wrapping_add(P3), (0x4u8) as u8);
    crate::simd_ops::wu32(rcx.wrapping_add(P0), (0x0u32) as u32);
    crate::simd_ops::wu8(rcx.wrapping_add(P4), (0x0u8) as u8);
    rdx = ((rdx).wrapping_add(0x18u64)) as u64;
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    rax = rbx;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_40273<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rcx;
    crate::simd_ops::wu8(rcx.wrapping_add(P0), (0x0u8) as u8);
    fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(P0))) as u8 as u64; fl_r = (0x0u8) as u8 as u64;
    if !((((fl_l as u8)) == ((fl_r as u8)))) { rax = (crate::simd_ops::ru32(rdx)) as u32 as u64; crate::simd_ops::wu32(rcx, ((rax as u32)) as u32); rax = (crate::simd_ops::ru64(rdx.wrapping_add(P1))) as u64; crate::simd_ops::wu64(rcx.wrapping_add(P1), rax); xmm0 = ((crate::simd_ops::ru64(rdx.wrapping_add(P2))) as u64) as u128; crate::simd_ops::wu64(rcx.wrapping_add(P2), ((xmm0 as u64)) as u64); rax = (crate::simd_ops::ru32(rdx.wrapping_add(P3))) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(P3), ((rax as u32)) as u32); rcx = ((rcx).wrapping_add(0x18u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rcx); crate::simd_ops::lk_and_u32(rcx, (0xff000004u32) as u32); crate::simd_ops::lk_or_u32(rcx, (0x4u32) as u32); crate::simd_ops::wu8(rcx.wrapping_add(P4), (0x4u8) as u8); crate::simd_ops::wu32(rcx.wrapping_add(P1), (0x0u32) as u32); crate::simd_ops::wu8(rcx.wrapping_add(P5), (0x0u8) as u8); rdx = ((rdx).wrapping_add(0x18u64)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu8(rbx.wrapping_add(P0), (0x1u8) as u8); }
    rax = rbx;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_40274<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut ymm0_hi: u128 = 0; let mut ymm1_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 29>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rsp = ((rsp).wrapping_sub(0xe8u64)) as u64;
    xmm0 = (crate::simd_ops::ru128(rcx)) as u128; ymm0_hi = 0;
    crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm0) as u128);
    xmm0 = ((crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64) as u128; ymm0_hi = 0;
    rcx = rsp.wrapping_add(0xb8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), ((xmm0 as u64)) as u64);
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    xmm1 = (crate::simd_ops::ru128(rsp.wrapping_add(0x30u64))) as u128; ymm1_hi = 0;
    rcx = ((crate::simd_ops::ru16(P2)) as u32) as u32 as u64;
    r8 = rsp.wrapping_add(0x30);
    rdx = ((xmm1 as u64)) as u64;
    let y_addr: u64 = rax; xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16));
    let y_addr: u64 = rsp.wrapping_add(0x30u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi);
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (((xmm1 >> 64) as u64)) as u64);
    r9 = (crate::simd_ops::ru64(rdx.wrapping_add(P3))) as u64;
    r9 = ((crate::simd_ops::ru16(r9.wrapping_add(rcx.wrapping_mul(2)))) as u32) as u32 as u64;
    rcx = rsp.wrapping_add(0x70);
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64;
    let y_addr: u64 = rsp.wrapping_add(0x70u64); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16));
    let y_addr: u64 = rsp.wrapping_add(0x90u64); xmm1 = crate::simd_ops::ru128(y_addr); ymm1_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16));
    let y_addr: u64 = rax.wrapping_add(P5); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi);
    xmm0 = ((crate::simd_ops::ru64(rsp.wrapping_add(0xb0u64))) as u64) as u128; ymm0_hi = 0;
    let y_addr: u64 = rax.wrapping_add(P6); crate::simd_ops::wu128(y_addr, xmm1); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm1_hi);
    crate::simd_ops::wu64(rax.wrapping_add(P7), ((xmm0 as u64)) as u64);
    rsp = ((rsp).wrapping_add(0xe8u64)) as u64;
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_40275<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64, const P9: u64, const P10: u64, const P11: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rdx; rdi = rcx; rax = (crate::simd_ops::ru32(rdx)) as u32 as u64; crate::simd_ops::wu32(rcx, ((rax as u32)) as u32); rsi = ((rcx.wrapping_add(P0)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rsi); crate::simd_ops::wu8(rsi.wrapping_add(P1), (0x0u8) as u8); fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(P2))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 1; continue; } rax = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64; crate::simd_ops::wu32(rsi, ((rax as u32)) as u32); rax = (crate::simd_ops::ru64(rdx.wrapping_add(P3))) as u64; crate::simd_ops::wu64(rsi.wrapping_add(P0), rax); xmm0 = ((crate::simd_ops::ru64(rdx.wrapping_add(P4))) as u64) as u128; crate::simd_ops::wu64(rsi.wrapping_add(P5), ((xmm0 as u64)) as u64); rax = (crate::simd_ops::ru32(rdx.wrapping_add(P6))) as u32 as u64; crate::simd_ops::wu32(rsi.wrapping_add(P7), ((rax as u32)) as u32); rcx = ((rsi.wrapping_add(P6)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rcx); crate::simd_ops::lk_and_u32(rcx, (0xff000004u32) as u32); crate::simd_ops::lk_or_u32(rcx, (0x4u32) as u32); crate::simd_ops::wu8(rcx.wrapping_add(P8), (0x4u8) as u8); crate::simd_ops::wu32(rcx.wrapping_add(P0), (0x0u32) as u32); crate::simd_ops::wu8(rcx.wrapping_add(P3), (0x0u8) as u8); rdx = ((rdx).wrapping_add(0x1cu64)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P9, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu8(rsi.wrapping_add(P1), (0x1u8) as u8); bb = 1; continue; }
            1 => { xmm0 = ((crate::simd_ops::ru64(rbx.wrapping_add(P10))) as u64) as u128; crate::simd_ops::wu64(rdi.wrapping_add(P10), ((xmm0 as u64)) as u64); rax = (crate::simd_ops::ru32(rbx.wrapping_add(P11))) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x50u64), ((rax as u32)) as u32); rax = ((crate::simd_ops::ru16(rbx.wrapping_add(0x54u64))) as u32) as u32 as u64; crate::simd_ops::wu16(rdi.wrapping_add(0x54u64), ((rax as u16)) as u16); rax = rdi; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_40276<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rcx;
    rcx = ((rcx).wrapping_add(0x18u64)) as u64;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rax = rbx;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_40277<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    crate::simd_ops::forward_incoming_to_outgoing();
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(P0))) as u8 as u64; fl_r = (0x0u8) as u8 as u64;
    if !((((fl_l as u8)) == ((fl_r as u8)))) { fl_l = ((rcx).wrapping_add(0x18u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8(P1, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_40321<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    crate::simd_ops::forward_incoming_to_outgoing();
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rdx = (crate::simd_ops::ru64(rdx)) as u64;
    crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8(P0, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_40344<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rcx = ((crate::simd_ops::ru16(P0)) as u32) as u32 as u64;
    rax = (crate::simd_ops::ru64(P1)) as u64;
    rax = (crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64;
    rax = (crate::simd_ops::ru64(rax.wrapping_add(P3))) as u64;
    rdx = (crate::simd_ops::ru64(rax.wrapping_add(rcx.wrapping_mul(8)))) as u64;
    crate::simd_ops::wu8(rdx.wrapping_add(P4), (0x1u8) as u8);
    crate::simd_ops::wu32(rdx.wrapping_add(P5), (0x0u32) as u32);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_40348<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (crate::simd_ops::ru64(rdx)) as u64; rdi = rcx; rcx = rdx; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64, rax, rcx, rdx, r8, r9, rsp); rbx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } rdx = (crate::simd_ops::ru64(rax)) as u64; rcx = rax; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(rdx.wrapping_add(P1))) as u64, rax, rcx, rdx, r8, r9, rsp); rdx = (crate::simd_ops::ru64(rbx)) as u64; rcx = rbx; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(rdx.wrapping_add(P2))) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = rbx; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 1; continue; } rdx = (crate::simd_ops::ru64(rbx)) as u64; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(rdx.wrapping_add(P2))) as u64, rax, rcx, rdx, r8, r9, rsp); rdx = rdi; rcx = rbx; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = rdi; rcx = rbx; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(rax.wrapping_add(P4))) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 2; continue; }
            1 => { rdx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = rdi; rcx = rbx; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(rax.wrapping_add(P4))) as u64, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = P5; r8 = rdi; rcx = rbx; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(rax.wrapping_add(P6))) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 3; continue; }
            3 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_40393<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = (crate::simd_ops::ru32(r8)) as u32 as u64;
    crate::simd_ops::wu32(rdx, ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(r8.wrapping_add(P0))) as u32 as u64;
    crate::simd_ops::wu32(rdx.wrapping_add(P0), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(r8.wrapping_add(P1))) as u32 as u64;
    crate::simd_ops::wu32(rdx.wrapping_add(P1), ((rax as u32)) as u32);
    rax = rdx;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_40394<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = (crate::simd_ops::ru32(r8)) as u32 as u64;
    crate::simd_ops::wu32(rdx, ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(r8.wrapping_add(P0))) as u32 as u64;
    crate::simd_ops::wu32(rdx.wrapping_add(P0), ((rax as u32)) as u32);
    rax = rdx;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_40424<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
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
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = P2; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rdx); crate::simd_ops::wu8(rsp.wrapping_add(0x40u64), (0x1u8) as u8); rdx = P3; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rdx); crate::simd_ops::wu32(rsp.wrapping_add(0x48u64), (0x70u32) as u32); r8 = rsp.wrapping_add(0x30); rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(rax.wrapping_add(P4))) as u64, rax, rcx, rdx, r8, r9, rsp); }
    rcx = rsp.wrapping_add(0x20);
    rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P5)) as u64, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x58u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_40439<const P0: u64, const P1: u64, const P2: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    r9 = 0;
    fl_l = 0; fl_r = 0;
    rax = ((r8 as u32)) as u32 as u64;
    r8 = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64;
    crate::simd_ops::wu64(rdx.wrapping_add(r8.wrapping_mul(8)), r9);
    crate::simd_ops::wu64(rdx.wrapping_add(r8.wrapping_mul(8)).wrapping_add(P0), r9);
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    crate::simd_ops::wu64(rdx.wrapping_add(r8.wrapping_mul(8)), rax);
    rax = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64;
    crate::simd_ops::wu32(rdx.wrapping_add(r8.wrapping_mul(8)).wrapping_add(P0), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rcx.wrapping_add(P1))) as u32 as u64;
    crate::simd_ops::wu32(rdx.wrapping_add(r8.wrapping_mul(8)).wrapping_add(P1), ((rax as u32)) as u32);
    crate::simd_ops::wu64(rcx, r9);
    crate::simd_ops::wu64(rcx.wrapping_add(P0), r9);
    rax = ((crate::simd_ops::ru8(rcx.wrapping_add(P2))) as u32) as u32 as u64;
    crate::simd_ops::wu8(rdx.wrapping_add(r8.wrapping_mul(8)).wrapping_add(P2), ((rax as u8)) as u8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_40465<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    r8 = ((r8 as u32)) as u32 as u64;
    r8 = ((r8).wrapping_shl(4u32)) as u64;
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
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_40475<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64, const P9: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rcx; rcx = P0; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P1)) as u64, rax, rcx, rdx, r8, r9, rsp); rbp = (crate::simd_ops::ru32(P2)) as u32 as u64; rsi = (crate::simd_ops::ru64(P3)) as u64; rdx = ((rsi.wrapping_add(rbp.wrapping_mul(8))) as u64) as u64; rax = (crate::simd_ops::ru32(P4)) as u32 as u64; rcx = ((rsi.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; fl_l = rdx; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } rdx = ((rdx).wrapping_sub(rsi)) as u64; rdx = ((((rdx) as i64).wrapping_shr(3u32) as u64)) as u64; rbp = rdx; rax = ((rdx.wrapping_add(P5)) as u64) as u64; rcx = ((rcx).wrapping_sub(rsi)) as u64; rcx = ((((rcx) as i64).wrapping_shr(3u32) as u64)) as u64; rdi = rcx; rdi = ((rdi).wrapping_shr(1u32)) as u64; rdi = ((rdi).wrapping_add(rcx)) as u64; fl_l = rdi; fl_r = rax; rdi = (if (((fl_l as u64)) < ((fl_r as u64))) { rax } else { rdi }) as u64; rdx = rdi; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); rsi = rax; rcx = (crate::simd_ops::ru32(P2)) as u32 as u64; rdx = (crate::simd_ops::ru64(P3)) as u64; r8 = ((rdx.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; r8 = ((r8).wrapping_sub(rdx)) as u64; rcx = rax; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); rcx = P3; rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(P3, rsi); crate::simd_ops::wu32(P2, ((rbp as u32)) as u32); crate::simd_ops::wu32(P4, ((rdi as u32)) as u32); bb = 1; continue; }
            1 => { rax = ((rbp as u32)) as u32 as u64; crate::simd_ops::wu64(rsi.wrapping_add(rax.wrapping_mul(8)), rbx); crate::simd_ops::lk_add_u32(P2, (1) as u32); rcx = P0; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk((crate::simd_ops::ru64(P9)) as u64, rax, rcx, rdx, r8, r9, rsp); }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_40477<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r10) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut ymm0_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 25>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0xc8u64)) as u64; r10 = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; let y_addr: u64 = r8; xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rcx); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rcx); r9 = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64; rax = ((crate::simd_ops::ru16(P2)) as u32) as u32 as u64; let y_addr: u64 = rsp.wrapping_add(0x70u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); r8 = ((crate::simd_ops::ru16(r9.wrapping_add(rax.wrapping_mul(2)))) as u32) as u32 as u64; r8 = ((r8).wrapping_add(r8)) as u64; rax = (crate::simd_ops::ru32(r10.wrapping_add(r8.wrapping_mul(8)).wrapping_add(P0))) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rax); rax = (crate::simd_ops::ru64(r10.wrapping_add(r8.wrapping_mul(8)))) as u64; r8 = rdx; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rax); rdx = rsp.wrapping_add(0x90); rax = ((crate::simd_ops::ru16(P3)) as u32) as u32 as u64; rcx = ((crate::simd_ops::ru16(r9.wrapping_add(rax.wrapping_mul(2)))) as u32) as u32 as u64; rcx = ((rcx).wrapping_add(rcx)) as u64; rax = (crate::simd_ops::ru32(r10.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(P0))) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax); rax = (crate::simd_ops::ru64(r10.wrapping_add(rcx.wrapping_mul(8)))) as u64; rcx = rsp.wrapping_add(0x20); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); let y_addr: u64 = rsp.wrapping_add(0x70u64); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); xmm1 = ((crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64) as u128; let y_addr: u64 = rsp.wrapping_add(0x70u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x40u64))) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128); let y_addr: u64 = rsp.wrapping_add(0x90u64); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), ((xmm1 as u64)) as u64); xmm1 = ((crate::simd_ops::ru64(rsp.wrapping_add(0xb0u64))) as u64) as u128; let y_addr: u64 = rsp.wrapping_add(0x40u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), ((xmm1 as u64)) as u64); r8 = rsp.wrapping_add(0x70); rdx = rsp.wrapping_add(0x20); rcx = rsp.wrapping_add(0x40); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rsp = ((rsp).wrapping_add(0xc8u64)) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_40479<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r10, mut r11, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0; let mut ymm1_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 25>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0xc0u64)) as u64; r11 = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rbx = r8; rax = (crate::simd_ops::ru64(rcx)) as u64; r8 = rdx; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rcx); rdx = rsp.wrapping_add(0x70); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rcx); r10 = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64; rax = ((crate::simd_ops::ru16(P2)) as u32) as u32 as u64; r9 = ((crate::simd_ops::ru16(r10.wrapping_add(rax.wrapping_mul(2)))) as u32) as u32 as u64; r9 = ((r9).wrapping_add(r9)) as u64; rax = (crate::simd_ops::ru32(r11.wrapping_add(r9.wrapping_mul(8)).wrapping_add(0x8u64))) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rax); rax = (crate::simd_ops::ru64(r11.wrapping_add(r9.wrapping_mul(8)))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rax); rax = ((crate::simd_ops::ru16(P3)) as u32) as u32 as u64; rcx = ((crate::simd_ops::ru16(r10.wrapping_add(rax.wrapping_mul(2)))) as u32) as u32 as u64; rcx = ((rcx).wrapping_add(rcx)) as u64; rax = (crate::simd_ops::ru32(r11.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x8u64))) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax); rax = (crate::simd_ops::ru64(r11.wrapping_add(rcx.wrapping_mul(8)))) as u64; rcx = rsp.wrapping_add(0x20); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); let y_addr: u64 = rbx; xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); xmm1 = (crate::simd_ops::ru128(rsp.wrapping_add(0x40u64))) as u128; ymm1_hi = 0; let y_addr: u64 = rsp.wrapping_add(0xa0u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); xmm0 = ((crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64) as u128; ymm0_hi = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), ((xmm0 as u64)) as u64); xmm0 = ((crate::simd_ops::ru64(rsp.wrapping_add(0x90u64))) as u64) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm1) as u128); let y_addr: u64 = rsp.wrapping_add(0x70u64); xmm1 = crate::simd_ops::ru128(y_addr); ymm1_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), ((xmm0 as u64)) as u64); let y_addr: u64 = rsp.wrapping_add(0x40u64); crate::simd_ops::wu128(y_addr, xmm1); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm1_hi); r8 = rsp.wrapping_add(0xa0); rdx = rsp.wrapping_add(0x20); rcx = rsp.wrapping_add(0x40); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rsp = ((rsp).wrapping_add(0xc0u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_40496(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    fl_l = (crate::simd_ops::lk_add_u64(rcx, (0xffffffffffffffb8u64) as u64)) as u64; fl_r = fl_l;
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    crate::simd_ops::wu64(rdx, rax);
    rax = rdx;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_40497<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64;
    rdx = ((rax.wrapping_add(rax.wrapping_mul(8))) as u64) as u64;
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    rax = ((rax.wrapping_add(rdx.wrapping_mul(8))) as u64) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_40498(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    fl_l = rax; fl_r = (0x48u64) as u64;
    rax = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_40500(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    fl_l = (crate::simd_ops::lk_add_u64(rcx, (0x48u64) as u64)) as u64; fl_r = fl_l;
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    crate::simd_ops::wu64(rdx, rax);
    rax = rdx;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_40523<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64;
    r8 = (((rax) as i64).wrapping_mul((0x38u64) as i64) as u64) as u64;
    rax = rdx;
    fl_l = ((r8).wrapping_add(crate::simd_ops::ru64(rcx))) as u64; fl_r = fl_l;
    r8 = ((fl_l as u64)) as u64;
    crate::simd_ops::wu64(rdx, r8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_40526<const P0: u64, const P1: u64, const P2: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = ((rcx.wrapping_add(P0)) as u64) as u64;
    rcx = rbx;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rcx = ((rbx.wrapping_add(P2)) as u64) as u64;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rcx = rbx;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_40527<const P0: u64, const P1: u64, const P2: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rdi = rcx;
    rcx = ((rcx).wrapping_add(0x8u64)) as u64;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rcx = ((rdi.wrapping_add(P1)) as u64) as u64;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rcx = ((rdi.wrapping_add(P2)) as u64) as u64;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rax = rdi;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_40540<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64, const P9: u64, const P10: u64, const P11: u64, const P12: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = rdx; rbx = rcx; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rdx = (crate::simd_ops::ru64(rbx)) as u64; rsi = rax; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } rcx = (crate::simd_ops::ru32(rbx.wrapping_add(P1))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 1; continue; } r8 = ((rcx.wrapping_add(rcx.wrapping_mul(2))) as u64) as u64; rcx = rax; r8 = ((r8).wrapping_shl(3u32)) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(P3))) as u32 as u64; fl_r = 0x40000000; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 5; continue; } r9 = (crate::simd_ops::ru64(rbx)) as u64; fl_l = r9; fl_r = r9; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } rcx = ((r9.wrapping_add(P4)) as u64) as u64; rax = r9; rcx = (((rcx) & (0xfffffffffe000000u64))) as u64; rax = ((rax).wrapping_sub(rcx)) as u64; rax = ((rax).wrapping_shr(16u32)) as u64; rdx = ((rcx.wrapping_add(P5)) as u64) as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(P6))) as u32 as u64; rdx = ((rdx).wrapping_sub(rax)) as u64; rax = ((crate::exe::teb_self() as u64)) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(rcx.wrapping_add(P7))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(P8))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 2; continue; } rax = (crate::simd_ops::ru64(rdx.wrapping_add(P9))) as u64; crate::simd_ops::wu64(r9, rax); fl_l = (crate::simd_ops::lk_add_u32(rdx.wrapping_add(P10), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; crate::simd_ops::wu64(rdx.wrapping_add(P9), r9); if (((fl_l as u32)) != 0) { bb = 5; continue; } rcx = rdx; rax = crate::simd_ops::call_stk(P11, rax, rcx, rdx, r8, r9, rsp); bb = 5; continue; }
            2 => { r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 4; continue; }
            3 => { r8 = 0; fl_l = 0; fl_r = 0; bb = 4; continue; }
            4 => { rax = crate::simd_ops::call_stk(P12, rax, rcx, rdx, r8, r9, rsp); bb = 5; continue; }
            5 => { crate::simd_ops::wu64(rbx, rsi); rdi = ((((rdi as u32)) & (0x3fffffffu32))) as u32 as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; crate::simd_ops::wu32(rbx.wrapping_add(0xcu64), ((rdi as u32)) as u32); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_40541<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    xmm0 = (crate::simd_ops::ru128(r8)) as u128;
    crate::simd_ops::wu128(rdx, (xmm0) as u128);
    xmm1 = ((crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64) as u128;
    crate::simd_ops::wu64(rdx.wrapping_add(P0), ((xmm1 as u64)) as u64);
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_40543<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 0>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rdi); r11 = r9; rdi = r9; rbx = r8; rsi = rdx; r10 = rcx; fl_l = r11; fl_r = r8; r11 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 3; continue; } r9 = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; rdx = r11; r8 = ((r9 as u32)) as u32 as u64; rax = ((r9 as u32)) as u32 as u64; r8 = ((r8).wrapping_sub(rdi)) as u64; rax = ((rax).wrapping_sub(r11)) as u64; fl_l = rax; fl_r = rdi; rcx = rax; rcx = (if (((fl_l as u64)) < ((fl_r as u64))) { rdi } else { rcx }) as u64; rdx = (if (((fl_l as u64)) >= ((fl_r as u64))) { r8 } else { rdx }) as u64; fl_l = rcx; fl_r = r9; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 2; continue; } rax = ((rcx.wrapping_add(rcx.wrapping_mul(2))) as u64) as u64; rax = ((rax).wrapping_shl(3u32)) as u64; r8 = (((rdx) as i64).wrapping_mul((0xffffffffffffffe8u64) as i64) as u64) as u64; fl_l = r9; fl_r = rcx; r9 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; bb = 1; continue; }
            1 => { rdx = (crate::simd_ops::ru64(r10)) as u64; rcx = ((r8.wrapping_add(rax)) as u64) as u64; rax = ((rax.wrapping_add(P1)) as u64) as u64; xmm0 = (crate::simd_ops::ru128(rax.wrapping_add(rdx).wrapping_add(P2))) as u128; crate::simd_ops::wu128(rcx.wrapping_add(rdx), (xmm0) as u128); xmm1 = ((crate::simd_ops::ru64(rax.wrapping_add(rdx).wrapping_add(P3))) as u64) as u128; crate::simd_ops::wu64(rcx.wrapping_add(rdx).wrapping_add(P4), ((xmm1 as u64)) as u64); fl_l = r9; fl_r = (0x1u64) as u64; r9 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru32(r10.wrapping_add(P0))) as u32 as u64; fl_r = ((r11 as u32)) as u32 as u64; crate::simd_ops::wu32(r10.wrapping_add(P0), ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32); bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(r10)) as u64; rcx = ((rbx.wrapping_add(rbx.wrapping_mul(2))) as u64) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x8u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x18u64))) as u64; rcx = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; rax = rsi; crate::simd_ops::wu64(rsi, rcx); rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x10u64))) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_40544<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 0>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rdi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r14); r11 = (crate::simd_ops::ru64(rcx)) as u64; r14 = rdx; r9 = ((r9).wrapping_sub(r11)) as u64; r8 = ((r8).wrapping_sub(r11)) as u64; r10 = rcx; rcx = (0x2aaaaaaaaaaaaaabu64) as u64; rax = rcx; let prod = (rax as i64 as i128).wrapping_mul((r9) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; rax = rcx; rbx = rdx; let prod = (rax as i64 as i128).wrapping_mul((r8) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; rbx = ((((rbx) as i64).wrapping_shr(2u32) as u64)) as u64; r9 = rbx; rdi = rdx; rdi = ((((rdi) as i64).wrapping_shr(2u32) as u64)) as u64; rax = rdi; r9 = ((r9).wrapping_shr(63u32)) as u64; rax = ((rax).wrapping_shr(63u32)) as u64; rbx = ((rbx).wrapping_add(r9)) as u64; rdi = ((rdi).wrapping_add(rax)) as u64; rsi = rbx; fl_l = rsi; fl_r = rdi; rsi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 3; continue; } r8 = (crate::simd_ops::ru32(r10.wrapping_add(P0))) as u32 as u64; r9 = rsi; rcx = ((r8 as u32)) as u32 as u64; rax = ((r8 as u32)) as u32 as u64; rcx = ((rcx).wrapping_sub(rbx)) as u64; rax = ((rax).wrapping_sub(rsi)) as u64; fl_l = rax; fl_r = rbx; rdx = rax; rdx = (if (((fl_l as u64)) < ((fl_r as u64))) { rbx } else { rdx }) as u64; r9 = (if (((fl_l as u64)) >= ((fl_r as u64))) { rcx } else { r9 }) as u64; fl_l = rdx; fl_r = r8; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 2; continue; } rax = ((rdx.wrapping_add(rdx.wrapping_mul(2))) as u64) as u64; rax = ((rax).wrapping_shl(3u32)) as u64; r9 = (((r9) as i64).wrapping_mul((0xffffffffffffffe8u64) as i64) as u64) as u64; fl_l = r8; fl_r = rdx; r8 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; bb = 1; continue; }
            1 => { rdx = (crate::simd_ops::ru64(r10)) as u64; rcx = ((r9.wrapping_add(rax)) as u64) as u64; rax = ((rax.wrapping_add(P1)) as u64) as u64; xmm0 = (crate::simd_ops::ru128(rax.wrapping_add(rdx).wrapping_add(P2))) as u128; crate::simd_ops::wu128(rcx.wrapping_add(rdx), (xmm0) as u128); xmm1 = ((crate::simd_ops::ru64(rax.wrapping_add(rdx).wrapping_add(P3))) as u64) as u128; crate::simd_ops::wu64(rcx.wrapping_add(rdx).wrapping_add(P4), ((xmm1 as u64)) as u64); fl_l = r8; fl_r = (0x1u64) as u64; r8 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } r11 = (crate::simd_ops::ru64(r10)) as u64; bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru32(r10.wrapping_add(P0))) as u32 as u64; fl_r = ((rsi as u32)) as u32 as u64; crate::simd_ops::wu32(r10.wrapping_add(P0), ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32); bb = 3; continue; }
            3 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x8u64))) as u64; rax = ((rdi.wrapping_add(rdi.wrapping_mul(2))) as u64) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x10u64))) as u64; rcx = ((r11.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x18u64))) as u64; rax = r14; crate::simd_ops::wu64(r14, rcx); r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_40545(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    fl_l = ((rax).wrapping_add(0x18u64)) as u64; fl_r = fl_l;
    rax = ((fl_l as u64)) as u64;
    crate::simd_ops::wu64(rdx, rax);
    rax = rdx;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_40546(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 0>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rdi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r14); r11 = (crate::simd_ops::ru64(rcx)) as u64; r9 = r8; r9 = ((r9).wrapping_sub(r11)) as u64; r14 = rdx; r9 = ((r9).wrapping_add(0x18u64)) as u64; r8 = ((r8).wrapping_sub(r11)) as u64; r10 = rcx; rcx = (0x2aaaaaaaaaaaaaabu64) as u64; rax = rcx; let prod = (rax as i64 as i128).wrapping_mul((r9) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; rax = rcx; rbx = rdx; let prod = (rax as i64 as i128).wrapping_mul((r8) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; rbx = ((((rbx) as i64).wrapping_shr(2u32) as u64)) as u64; r9 = rbx; rdi = rdx; rdi = ((((rdi) as i64).wrapping_shr(2u32) as u64)) as u64; rax = rdi; r9 = ((r9).wrapping_shr(63u32)) as u64; rax = ((rax).wrapping_shr(63u32)) as u64; rbx = ((rbx).wrapping_add(r9)) as u64; rdi = ((rdi).wrapping_add(rax)) as u64; rsi = rbx; fl_l = rsi; fl_r = rdi; rsi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 3; continue; } r8 = (crate::simd_ops::ru32(r10.wrapping_add(0x8u64))) as u32 as u64; r9 = rsi; rcx = ((r8 as u32)) as u32 as u64; rax = ((r8 as u32)) as u32 as u64; rcx = ((rcx).wrapping_sub(rbx)) as u64; rax = ((rax).wrapping_sub(rsi)) as u64; fl_l = rax; fl_r = rbx; rdx = rax; rdx = (if (((fl_l as u64)) < ((fl_r as u64))) { rbx } else { rdx }) as u64; r9 = (if (((fl_l as u64)) >= ((fl_r as u64))) { rcx } else { r9 }) as u64; fl_l = rdx; fl_r = r8; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 2; continue; } rax = ((rdx.wrapping_add(rdx.wrapping_mul(2))) as u64) as u64; rax = ((rax).wrapping_shl(3u32)) as u64; r9 = (((r9) as i64).wrapping_mul((0xffffffffffffffe8u64) as i64) as u64) as u64; fl_l = r8; fl_r = rdx; r8 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; bb = 1; continue; }
            1 => { rdx = (crate::simd_ops::ru64(r10)) as u64; rcx = ((r9.wrapping_add(rax)) as u64) as u64; rax = rax.wrapping_add(0x18); xmm0 = (crate::simd_ops::ru128(rax.wrapping_add(rdx).wrapping_sub(0x18u64))) as u128; crate::simd_ops::wu128(rcx.wrapping_add(rdx), (xmm0) as u128); xmm1 = ((crate::simd_ops::ru64(rax.wrapping_add(rdx).wrapping_sub(0x8u64))) as u64) as u128; crate::simd_ops::wu64(rcx.wrapping_add(rdx).wrapping_add(0x10u64), ((xmm1 as u64)) as u64); fl_l = r8; fl_r = (0x1u64) as u64; r8 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } r11 = (crate::simd_ops::ru64(r10)) as u64; bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru32(r10.wrapping_add(0x8u64))) as u32 as u64; fl_r = ((rsi as u32)) as u32 as u64; crate::simd_ops::wu32(r10.wrapping_add(0x8u64), ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32); bb = 3; continue; }
            3 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x8u64))) as u64; rax = ((rdi.wrapping_add(rdi.wrapping_mul(2))) as u64) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x10u64))) as u64; rcx = ((r11.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x18u64))) as u64; rax = r14; crate::simd_ops::wu64(r14, rcx); r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_40547<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = 0;
    rax = 0x1;
    let tmp_xchg: u8 = crate::simd_ops::atomic_xchg_u8(P0, ((rax as u8)) as u8);
    rax = (rax & !0xffu64) | (((tmp_xchg) as u8 as u64));
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_40551<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = (crate::simd_ops::ru64(rcx)) as u64; r8 = 0; fl_l = 0; fl_r = 0; rcx = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; r9 = ((rcx.wrapping_add(rcx.wrapping_mul(2))) as u64) as u64; rcx = ((rax.wrapping_add(r9.wrapping_mul(8))) as u64) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru64(rax)) as u64; fl_r = rdx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 3; continue; } r8 = (((r8 as u32)).wrapping_add(1)) as u32 as u64; rax = ((rax).wrapping_add(0x18u64)) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } bb = 2; continue; }
            2 => { rax = 0xffffffff; return rax; }
            3 => { rax = ((r8 as u32)) as u32 as u64; return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_40559<const P0: u64, const P1: u64, const P2: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 0>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rdi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r14); rbx = (crate::simd_ops::ru64(rcx)) as u64; rdi = r9; rdi = ((rdi).wrapping_sub(rbx)) as u64; r11 = r8; rdi = ((((rdi) as i64).wrapping_shr(4u32) as u64)) as u64; r11 = ((r11).wrapping_sub(rbx)) as u64; rsi = rdi; r11 = ((((r11) as i64).wrapping_shr(4u32) as u64)) as u64; r14 = rdx; r10 = rcx; fl_l = rsi; fl_r = r11; rsi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 3; continue; } r8 = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; r9 = rsi; rcx = ((r8 as u32)) as u32 as u64; rax = ((r8 as u32)) as u32 as u64; rcx = ((rcx).wrapping_sub(rdi)) as u64; rax = ((rax).wrapping_sub(rsi)) as u64; fl_l = rax; fl_r = rdi; rdx = rax; rdx = (if (((fl_l as u64)) < ((fl_r as u64))) { rdi } else { rdx }) as u64; r9 = (if (((fl_l as u64)) >= ((fl_r as u64))) { rcx } else { r9 }) as u64; fl_l = rdx; fl_r = r8; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 2; continue; } r9 = (0u64.wrapping_sub(r9)) as u64; rax = rdx; rax = ((rax).wrapping_shl(4u32)) as u64; r9 = ((r9).wrapping_add(r9)) as u64; fl_l = r8; fl_r = rdx; r8 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; bb = 1; continue; }
            1 => { rdx = (crate::simd_ops::ru64(r10)) as u64; rcx = ((rax.wrapping_add(r9.wrapping_mul(8))) as u64) as u64; rax = ((rax.wrapping_add(P1)) as u64) as u64; xmm0 = (crate::simd_ops::ru128(rax.wrapping_add(rdx).wrapping_add(P2))) as u128; crate::simd_ops::wu128(rcx.wrapping_add(rdx), (xmm0) as u128); fl_l = r8; fl_r = (0x1u64) as u64; r8 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } rbx = (crate::simd_ops::ru64(r10)) as u64; bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru32(r10.wrapping_add(P0))) as u32 as u64; fl_r = ((rsi as u32)) as u32 as u64; crate::simd_ops::wu32(r10.wrapping_add(P0), ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32); bb = 3; continue; }
            3 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x10u64))) as u64; rax = r14; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x18u64))) as u64; r11 = ((r11).wrapping_shl(4u32)) as u64; fl_l = ((r11).wrapping_add(rbx)) as u64; fl_r = fl_l; r11 = ((fl_l as u64)) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(r14, r11); r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_40560<const P0: u64, const P1: u64, const P2: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 0>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rdi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r14); rbx = (crate::simd_ops::ru64(rcx)) as u64; rdi = r8; rdi = ((rdi).wrapping_sub(rbx)) as u64; r9 = r8; rdi = ((rdi).wrapping_add(0x10u64)) as u64; r9 = ((r9).wrapping_sub(rbx)) as u64; rdi = ((((rdi) as i64).wrapping_shr(4u32) as u64)) as u64; r14 = rdx; rsi = rdi; r9 = ((((r9) as i64).wrapping_shr(4u32) as u64)) as u64; r10 = rcx; fl_l = rsi; fl_r = r9; rsi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 3; continue; } r8 = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; r11 = rsi; rcx = ((r8 as u32)) as u32 as u64; rax = ((r8 as u32)) as u32 as u64; rcx = ((rcx).wrapping_sub(rdi)) as u64; rax = ((rax).wrapping_sub(rsi)) as u64; fl_l = rax; fl_r = rdi; rdx = rax; rdx = (if (((fl_l as u64)) < ((fl_r as u64))) { rdi } else { rdx }) as u64; r11 = (if (((fl_l as u64)) >= ((fl_r as u64))) { rcx } else { r11 }) as u64; fl_l = rdx; fl_r = r8; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 2; continue; } r11 = (0u64.wrapping_sub(r11)) as u64; rax = rdx; rax = ((rax).wrapping_shl(4u32)) as u64; r11 = ((r11).wrapping_add(r11)) as u64; fl_l = r8; fl_r = rdx; r8 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; bb = 1; continue; }
            1 => { rdx = (crate::simd_ops::ru64(r10)) as u64; rcx = ((rax.wrapping_add(r11.wrapping_mul(8))) as u64) as u64; rax = ((rax.wrapping_add(P1)) as u64) as u64; xmm0 = (crate::simd_ops::ru128(rax.wrapping_add(rdx).wrapping_add(P2))) as u128; crate::simd_ops::wu128(rcx.wrapping_add(rdx), (xmm0) as u128); fl_l = r8; fl_r = (0x1u64) as u64; r8 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } rbx = (crate::simd_ops::ru64(r10)) as u64; bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru32(r10.wrapping_add(P0))) as u32 as u64; fl_r = ((rsi as u32)) as u32 as u64; crate::simd_ops::wu32(r10.wrapping_add(P0), ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32); bb = 3; continue; }
            3 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x10u64))) as u64; rax = r14; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x18u64))) as u64; r9 = ((r9).wrapping_shl(4u32)) as u64; fl_l = ((r9).wrapping_add(rbx)) as u64; fl_r = fl_l; r9 = ((fl_l as u64)) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(r14, r9); r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_40562<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64, const P9: u64, const P10: u64, const P11: u64, const P12: u64, const P13: u64, const P14: u64, const P15: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 19>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x80u64)) as u64; rbp = rcx; rbx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rbx); rsi = ((crate::simd_ops::ru16(P0)) as u32) as u32 as u64; rdi = ((rsi as u32)) as u32 as u64; rdi = ((rdi).wrapping_shl(6u32)) as u64; rdi = ((rdi).wrapping_add(crate::simd_ops::ru64(rcx.wrapping_add(0x90u64)))) as u64; rax = (crate::simd_ops::ru32(P1)) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x3cu64), ((rax as u32)) as u32); rax = P2; crate::simd_ops::wu64(rdi.wrapping_add(0x30u64), rax); crate::simd_ops::wu8(rdi.wrapping_add(0x38u64), ((rbx as u8)) as u8); rdx = 0x6348; rax = ((crate::exe::tls_pointer() as u64)) as u64; rcx = (crate::simd_ops::ru64(rax)) as u64; rcx = (crate::simd_ops::ru64(rdx.wrapping_add(rcx))) as u64; r8 = (crate::simd_ops::ru64(rcx.wrapping_add(0x18u64))) as u64; rdx = (crate::simd_ops::ru64(r8.wrapping_add(0x10u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 1; continue; } r9 = 0; fl_l = 0; fl_r = 0; r8 = 0; fl_l = 0; fl_r = 0; rdx = ((rbx.wrapping_add(0x10u64)) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 2; continue; } (rax, xmm0) = crate::simd_ops::call_stk_xmm((crate::simd_ops::ru64(P4)) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rax; rdx = 0xdeadd00d; (rax, xmm0) = crate::simd_ops::call_stk_xmm((crate::simd_ops::ru64(P5)) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 3; continue; }
            1 => { fl_l = (crate::simd_ops::lk_add_u32(r8.wrapping_add(0x18u64), (1) as u32)) as u32 as u64; rax = (crate::simd_ops::ru64(rdx)) as u64; crate::simd_ops::wu64(r8.wrapping_add(0x10u64), rax); bb = 2; continue; }
            2 => { rbx = rdx; bb = 3; continue; }
            3 => { rdx = rsp.wrapping_add(0x20); rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rdi, rbx); rax = (crate::simd_ops::ru64(rbp.wrapping_add(0xa0u64))) as u64; crate::simd_ops::wu64(rax.wrapping_add(rsi.wrapping_mul(8)), rbx); fl_l = (crate::simd_ops::ru32(rsp.wrapping_add(0x2cu64))) as u32 as u64; fl_r = 0x40000000; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 7; continue; } r10 = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; fl_l = r10; fl_r = r10; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } r10 = ((r10).wrapping_sub(1)) as u64; r10 = (((r10) & (0xfffffffffe000000u64))) as u64; rax = ((crate::exe::teb_self() as u64)) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(r10.wrapping_add(0x100u64))) as u64; rdx = (rdx & !0xffu64) | (((if (((fl_l as u64)) == ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); r9 = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; rax = r9; rax = ((rax).wrapping_sub(r10)) as u64; rax = ((rax).wrapping_shr(16u32)) as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rcx = r10.wrapping_add(0x108); rcx = ((rcx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x4u64))) as u32 as u64; rcx = ((rcx).wrapping_sub(rax)) as u64; fl_l = ((rdx as u8)) as u8 as u64; fl_r = ((rdx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 5; continue; } fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0xeu64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 4; continue; } rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(r9, rax); crate::simd_ops::wu64(rcx.wrapping_add(0x20u64), r9); fl_l = (crate::simd_ops::lk_add_u32(rcx.wrapping_add(0x18u64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; if (((fl_l as u32)) != 0) { bb = 7; continue; } (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 7; continue; }
            4 => { r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 6; continue; }
            5 => { r8 = 0; fl_l = 0; fl_r = 0; bb = 6; continue; }
            6 => { rdx = rcx; rcx = r10; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P8, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 7; continue; }
            7 => { rbx = rbp.wrapping_add(0x8); xmm0 = (crate::simd_ops::ru128(P9)) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128); rcx = rbx.wrapping_add(0x18); r8 = rsp.wrapping_add(0x20); rdx = rsp.wrapping_add(0x30); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P10, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rbx); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; xmm0 = (crate::simd_ops::ru128(rax.wrapping_add(0x10u64))) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rsp.wrapping_add(0x48u64), (xmm0) as u128); rdx = rsp.wrapping_add(0x60); rcx = rsp.wrapping_add(0x40); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P11, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = (crate::simd_ops::ru64(rax)) as u64; rbp = ((crate::simd_ops::ru16(rax.wrapping_add(0xau64))) as u32) as u32 as u64; xmm0 = (crate::simd_ops::ru128(rax.wrapping_add(0x10u64))) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm0) as u128); rcx = rbx.wrapping_add(0x48); xmm1 = (crate::simd_ops::ru128(P12)) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm1) as u128); rdx = rsp.wrapping_add(0x30); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P13, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r14 = rax; rdi = ((crate::simd_ops::ru16(rax)) as u32) as u32 as u64; r15 = 0xffff; fl_l = ((rdi as u16)) as u16 as u64; fl_r = ((r15 as u16)) as u16 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { bb = 9; continue; } rsi = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; xmm0 = (crate::simd_ops::ru128(P12)) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128); rax = (crate::simd_ops::ru32(rsi.wrapping_add(0x18u64))) as u32 as u64; r8 = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; r8 = ((r8).wrapping_shl(4u32)) as u64; rax = (0x2aaaaaaaaaaaaaabu64) as u64; let prod = (rax as i64 as i128).wrapping_mul((r8) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; rdi = rdx; rdi = ((((rdi) as i64).wrapping_shr(3u32) as u64)) as u64; rax = rdi; rax = ((rax).wrapping_shr(63u32)) as u64; rdi = ((rdi).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru32(rsi.wrapping_add(0x1cu64))) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; fl_l = r8; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 8; continue; } rdx = rdi.wrapping_add(0x1); rcx = rsi.wrapping_add(0x10); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P14, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 8; continue; }
            8 => { rax = (crate::simd_ops::ru32(rsi.wrapping_add(0x18u64))) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rsi.wrapping_add(0x10u64)))) as u64; xmm0 = 0u128; ymm0_hi = 0; let y_addr: u64 = rcx; crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); crate::simd_ops::wu128(rcx.wrapping_add(0x20u64), (xmm0) as u128); crate::simd_ops::lk_add_u32(rsi.wrapping_add(0x18u64), (1) as u32); rax = (crate::simd_ops::ru32(rsi.wrapping_add(0x18u64))) as u32 as u64; rdx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rdx = ((rdx).wrapping_shl(4u32)) as u64; fl_l = ((rdx).wrapping_add(crate::simd_ops::ru64(rsi.wrapping_add(0x10u64)))) as u64; fl_r = fl_l; rdx = ((fl_l as u64)) as u64; xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rdx.wrapping_sub(0x30u64), (xmm0) as u128); crate::simd_ops::wu32(rdx.wrapping_sub(0xeu64), (0xffffffffu32) as u32); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; crate::simd_ops::wu64(rdx.wrapping_sub(0x20u64), rax); crate::simd_ops::wu64(rdx.wrapping_sub(0x18u64), rdi); crate::simd_ops::wu16(r14, ((rdi as u16)) as u16); bb = 9; continue; }
            9 => { r9 = 0x1; r8 = (((rdi as u16)) as u32) as u32 as u64; rdx = (((rbp as u16)) as u32) as u32 as u64; rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P15, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r11 = rsp.wrapping_add(0x80); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x20u64))) as u64; rbp = (crate::simd_ops::ru64(r11.wrapping_add(0x28u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x30u64))) as u64; rsp = r11; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_40563<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbx);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rdx;
    rdi = rcx;
    rax = (crate::simd_ops::ru64(rdx)) as u64;
    crate::simd_ops::wu64(rcx, rax);
    rcx = ((rcx).wrapping_add(0x8u64)) as u64;
    rdx = ((rdx).wrapping_add(0x8u64)) as u64;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rax = (crate::simd_ops::ru64(rbx.wrapping_add(P1))) as u64;
    crate::simd_ops::wu64(rdi.wrapping_add(P1), rax);
    rax = (crate::simd_ops::ru32(rbx.wrapping_add(P2))) as u32 as u64;
    crate::simd_ops::wu32(rdi.wrapping_add(P2), ((rax as u32)) as u32);
    rcx = ((rdi.wrapping_add(P3)) as u64) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rcx);
    crate::simd_ops::lk_and_u32(rcx, (0xff000008u32) as u32);
    crate::simd_ops::lk_or_u32(rcx, (0x8u32) as u32);
    crate::simd_ops::wu8(rcx.wrapping_add(P4), (0x8u8) as u8);
    crate::simd_ops::wu32(rcx.wrapping_add(P5), (0x0u32) as u32);
    crate::simd_ops::wu8(rcx.wrapping_add(P6), (0x0u8) as u8);
    rdx = ((rbx.wrapping_add(P3)) as u64) as u64;
    rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp);
    rax = rdi;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_40569(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    fl_l = (crate::simd_ops::lk_add_u64(rcx, (0x90u64) as u64)) as u64; fl_r = fl_l;
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    crate::simd_ops::wu64(rdx, rax);
    rax = rdx;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_40572<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64;
    r8 = ((rax.wrapping_add(rax.wrapping_mul(8))) as u64) as u64;
    rax = rdx;
    r8 = ((r8).wrapping_shl(4u32)) as u64;
    fl_l = ((r8).wrapping_add(crate::simd_ops::ru64(rcx))) as u64; fl_r = fl_l;
    r8 = ((fl_l as u64)) as u64;
    crate::simd_ops::wu64(rdx, r8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_40609<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rax = (crate::simd_ops::ru64(rdx)) as u64;
    rbx = rdx;
    crate::simd_ops::wu64(rcx, rax);
    rdi = rcx;
    rcx = ((rcx).wrapping_add(0x8u64)) as u64;
    rdx = ((rdx).wrapping_add(0x8u64)) as u64;
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
pub unsafe extern "win64" fn tpl_40617<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64;
    rax = ((rax).wrapping_shl(6u32)) as u64;
    fl_l = ((rax).wrapping_add(crate::simd_ops::ru64(rcx))) as u64; fl_r = fl_l;
    rax = ((fl_l as u64)) as u64;
    crate::simd_ops::wu64(rdx, rax);
    rax = rdx;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_40626(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    r8 = ((r8).wrapping_shl(6u32)) as u64;
    rax = rdx;
    fl_l = ((r8).wrapping_add(crate::simd_ops::ru64(rcx))) as u64; fl_r = fl_l;
    r8 = ((fl_l as u64)) as u64;
    crate::simd_ops::wu64(rdx, r8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_40672<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = (crate::simd_ops::ru64(rcx)) as u64; r8 = 0; fl_l = 0; fl_r = 0; rcx = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rcx = ((rcx).wrapping_add(rax)) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru64(rax)) as u64; fl_r = rdx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 3; continue; } r8 = ((r8).wrapping_add(1)) as u64; rax = ((rax).wrapping_add(0x10u64)) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } bb = 2; continue; }
            2 => { rax = 0; fl_l = 0; fl_r = 0; return rax; }
            3 => { rax = r8; return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_40693<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    fl_l = rcx; fl_r = (crate::simd_ops::ru64(r8)) as u64;
    rcx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64;
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P1))) as u32 as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(rdx).wrapping_add(P1), ((rax as u32)) as u32);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_40767<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (0u64, 0u64, 0u64);
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    fl_l = (crate::simd_ops::ru64(rdx)) as u64; fl_r = rax;
    if (((fl_l as u64)) != ((fl_r as u64))) { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; return rax; }
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    fl_l = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; fl_r = rax;
    if !((((fl_l as u64)) != ((fl_r as u64)))) { rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); return rax; }
    rax = (rax & !0xffu64) | (((0u8) as u8 as u64));
    fl_l = 0; fl_r = 0;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_40792(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    crate::simd_ops::wu64(rcx, (0xffffffffffffffffu64) as u64);
    rax = rcx;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_40793<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    crate::simd_ops::forward_incoming_to_outgoing();
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rcx = ((rdx.wrapping_mul(8)) as u64) as u64;
    rdx = 0x8;
    crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8(P0, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_40795<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (0u64, 0u64, 0u64);
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    rcx = (crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64;
    fl_l = (crate::simd_ops::ru64(rdx)) as u64; fl_r = rcx;
    rax = (rax & !0xffu64) | (((if (((fl_l as u64)) == ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64));
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_40810<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    crate::simd_ops::forward_incoming_to_outgoing();
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    xmm1 = ((crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32) as u128;
    rcx = (crate::simd_ops::ru64(rcx)) as u64;
    crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(P1, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::set_xmm_ret(c_x0); return c_rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_40817<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    r8 = ((r8 as u32)) as u32 as u64;
    r8 = ((r8).wrapping_shl(4u32)) as u64;
    r8 = ((r8).wrapping_add(rdx)) as u64;
    rdx = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu64(r8, rdx);
    crate::simd_ops::wu64(r8.wrapping_add(P0), rdx);
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    crate::simd_ops::wu64(r8, rax);
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    crate::simd_ops::wu64(r8.wrapping_add(P0), rax);
    crate::simd_ops::wu64(rcx, rdx);
    crate::simd_ops::wu64(rcx.wrapping_add(P0), rdx);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_40837<const P0: u64, const P1: u64, const P2: u64, const P3: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = ((rcx.wrapping_add(P0)) as u64) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rcx);
    rcx = rbx;
    rdx = rsp.wrapping_add(0x30);
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rax = ((rbx.wrapping_add(P2)) as u64) as u64;
    fl_l = rbx; fl_r = rax;
    if (((fl_l as u64)) == ((fl_r as u64))) { rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    rcx = 0;
    fl_l = 0; fl_r = 0;
    loop { crate::simd_ops::wu32(rbx.wrapping_add(P3), ((rcx as u32)) as u32); rbx = ((rbx).wrapping_add(0x40u64)) as u64; fl_l = rbx; fl_r = rax; if !((((fl_l as u64)) != ((fl_r as u64)))) { break; } }
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_40847<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    crate::simd_ops::forward_incoming_to_outgoing();
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    r9 = rcx;
    rax = ((r8 as u32)) as u32 as u64;
    rcx = (((rax) as i64).wrapping_mul((0x70u64) as i64) as u64) as u64;
    fl_l = ((rcx).wrapping_add(rdx)) as u64; fl_r = fl_l;
    rcx = ((fl_l as u64)) as u64;
    rdx = r9;
    crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8(P0, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_40865<const P0: u64, const P1: u64, const P2: u64, const P3: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    fl_l = (crate::simd_ops::ru8(P0)) as u8 as u64; fl_r = (0x0u8) as u8 as u64;
    if !((((fl_l as u8)) == ((fl_r as u8)))) { rcx = (rcx & !0xffu64) | (((0x1u8) as u8 as u64)); crate::simd_ops::wu8(P0, (0x0u8) as u8); rsp = ((rsp).wrapping_add(0x28u64)) as u64; return crate::simd_ops::tail_stk(P1, rax, rcx, rdx, r8, r9, rsp); }
    rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp);
    rcx = (0xffffffffffffffffu64) as u64;
    rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp);
    rcx = (rcx & !0xffu64) | (((0x1u8) as u8 as u64));
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    let _ = &stack; return crate::simd_ops::tail_stk(P1, rax, rcx, rdx, r8, r9, rsp);
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_40881<const P0: u64, const P1: u64, const P2: u64, const P3: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = (crate::simd_ops::ru64(P0)) as u64;
    rcx = P1;
    rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp);
    r9 = 0;
    fl_l = 0; fl_r = 0;
    r8 = (0x100000001u64) as u64;
    rdx = ((rax as u32)) as u32 as u64;
    rcx = rbx;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return crate::simd_ops::tail_stk(P3, rax, rcx, rdx, r8, r9, rsp);
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_40887<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = (crate::simd_ops::ru64(P0)) as u64;
    fl_l = rbx; fl_r = rbx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = rbx; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(rax)) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = rbx; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); }
    crate::simd_ops::wu64(P0, (0x0u64) as u64);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_40903<const P0: u64, const P1: u64, const P2: u64, const P3: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 23>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rdi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rbp = rsp.wrapping_sub(0x47); rsp = ((rsp).wrapping_sub(0xa0u64)) as u64; r15 = rdx; fl_l = (crate::simd_ops::ru64(rdx.wrapping_add(0x8u64))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 19; continue; } r11 = (crate::simd_ops::ru64(rcx)) as u64; r11 = ((r11).wrapping_add(0x40u64)) as u64; xmm0 = (crate::simd_ops::ru128(rcx.wrapping_add(0x8u64))) as u128; crate::simd_ops::wu128(rbp.wrapping_sub(0x39u64), (xmm0) as u128); r8 = rbp.wrapping_sub(0x39); rdx = r11; rcx = rbp.wrapping_sub(0x29); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rbp.wrapping_sub(0x9u64), r11); xmm1 = (crate::simd_ops::ru128(rbp.wrapping_sub(0x29u64))) as u128; crate::simd_ops::wu128(rbp.wrapping_sub(0x1u64), (xmm1) as u128); xmm0 = ((crate::simd_ops::ru64(rbp.wrapping_sub(0x19u64))) as u64) as u128; crate::simd_ops::wu64(rbp.wrapping_add(0xfu64), ((xmm0 as u64)) as u64); r12 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rbp.wrapping_add(0x1fu64), r12); crate::simd_ops::wu64(rbp.wrapping_add(0x27u64), r12); rax = (((xmm1 >> 64) as u64)) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(rbp.wrapping_sub(0x19u64))) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 3; continue; } rcx = ((crate::simd_ops::ru16(rax)) as u32) as u32 as u64; rax = (crate::simd_ops::ru64(r11.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x48u64))) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x17u64), rax); fl_l = rax; fl_r = r12; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 1; continue; } rax = (crate::simd_ops::ru32(r11.wrapping_add(rcx.wrapping_mul(4)).wrapping_add(0x10048u64))) as u32 as u64; bb = 2; continue; }
            1 => { rax = ((r12 as u32)) as u32 as u64; bb = 2; continue; }
            2 => { crate::simd_ops::wu64(rbp.wrapping_add(0x1fu64), rax); bb = 3; continue; }
            3 => { rcx = rbp.wrapping_sub(0x9); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdi = (crate::simd_ops::ru64(rbp.wrapping_add(0x27u64))) as u64; rsi = (crate::simd_ops::ru64(rbp.wrapping_add(0x1fu64))) as u64; fl_l = rdi; fl_r = rsi; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 19; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x2fu64))) as u64; rdx = ((rax.wrapping_add(rdi.wrapping_mul(8))) as u64) as u64; rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x37u64))) as u64; r8 = ((rax.wrapping_add(rdi.wrapping_mul(8))) as u64) as u64; rcx = (crate::simd_ops::ru64(rdx)) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } rbx = (crate::simd_ops::ru64(r15)) as u64; rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; fl_l = (crate::simd_ops::ru8(rax.wrapping_add(0x19u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 8; continue; } bb = 5; continue; }
            5 => { fl_l = (crate::simd_ops::ru64(rax.wrapping_add(0x20u64))) as u64; fl_r = rcx; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 6; continue; } rax = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; bb = 7; continue; }
            6 => { rbx = rax; rax = (crate::simd_ops::ru64(rax)) as u64; bb = 7; continue; }
            7 => { fl_l = (crate::simd_ops::ru8(rax.wrapping_add(0x19u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 5; continue; } bb = 8; continue; }
            8 => { fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(0x19u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 9; continue; } fl_l = rcx; fl_r = (crate::simd_ops::ru64(rbx.wrapping_add(0x20u64))) as u64; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 10; continue; } bb = 9; continue; }
            9 => { rbx = (crate::simd_ops::ru64(r15)) as u64; bb = 10; continue; }
            10 => { rdx = r8; rcx = rbx.wrapping_add(0x28); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x17u64))) as u64; r14 = (crate::simd_ops::ru64(rax.wrapping_add(rdi.wrapping_mul(8)).wrapping_add(0x10u64))) as u64; rdx = (crate::simd_ops::ru32(rbx.wrapping_add(0x40u64))) as u32 as u64; rcx = (crate::simd_ops::ru32(rbx.wrapping_add(0x44u64))) as u32 as u64; rcx = ((rcx).wrapping_shl(3u32)) as u64; rax = ((rdx.wrapping_mul(8)) as u64) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 11; continue; } rdx = ((rdx).wrapping_add(1)) as u64; rcx = rbx.wrapping_add(0x38); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 11; continue; }
            11 => { rcx = (crate::simd_ops::ru32(rbx.wrapping_add(0x40u64))) as u32 as u64; rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x38u64))) as u64; crate::simd_ops::wu64(rax.wrapping_add(rcx.wrapping_mul(8)), r14); fl_l = (crate::simd_ops::lk_add_u32(rbx.wrapping_add(0x40u64), (1) as u32)) as u32 as u64; bb = 12; continue; }
            12 => { rdi = ((rdi).wrapping_add(1)) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x27u64), rdi); fl_l = rdi; fl_r = rsi; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 4; continue; } rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x7u64))) as u64; rax = ((rax).wrapping_add(0x2u64)) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x7u64), rax); r8 = (crate::simd_ops::ru64(rbp.wrapping_add(0xfu64))) as u64; fl_l = rax; fl_r = r8; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 18; continue; } rdx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x1u64))) as u64; bb = 13; continue; }
            13 => { rcx = ((crate::simd_ops::ru16(rax)) as u32) as u32 as u64; fl_l = (crate::simd_ops::ru64(rdx.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x48u64))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 14; continue; } fl_l = (crate::simd_ops::ru32(rdx.wrapping_add(rcx.wrapping_mul(4)).wrapping_add(0x10048u64))) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 15; continue; } bb = 14; continue; }
            14 => { rax = ((rax).wrapping_add(0x2u64)) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x7u64), rax); fl_l = rax; fl_r = r8; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 13; continue; } bb = 18; continue; }
            15 => { fl_l = rax; fl_r = r8; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 18; continue; } rcx = ((crate::simd_ops::ru16(rax)) as u32) as u32 as u64; rdx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x9u64))) as u64; rax = (crate::simd_ops::ru64(rdx.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x48u64))) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x17u64), rax); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 16; continue; } rax = (crate::simd_ops::ru32(rdx.wrapping_add(rcx.wrapping_mul(4)).wrapping_add(0x10048u64))) as u32 as u64; bb = 17; continue; }
            16 => { rax = ((r12 as u32)) as u32 as u64; bb = 17; continue; }
            17 => { crate::simd_ops::wu64(rbp.wrapping_add(0x1fu64), rax); crate::simd_ops::wu64(rbp.wrapping_add(0x27u64), r12); bb = 18; continue; }
            18 => { rcx = rbp.wrapping_sub(0x9); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdi = (crate::simd_ops::ru64(rbp.wrapping_add(0x27u64))) as u64; rsi = (crate::simd_ops::ru64(rbp.wrapping_add(0x1fu64))) as u64; fl_l = rdi; fl_r = rsi; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 4; continue; } bb = 19; continue; }
            19 => { r11 = rsp.wrapping_add(0xa0); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x20u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x28u64))) as u64; rdi = (crate::simd_ops::ru64(r11.wrapping_add(0x30u64))) as u64; r12 = (crate::simd_ops::ru64(r11.wrapping_add(0x38u64))) as u64; rsp = r11; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_40910<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 75>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x258u64)) as u64;
    rcx = (crate::simd_ops::ru64(rcx)) as u64;
    rcx = ((rcx).wrapping_add(0x2a0u64)) as u64;
    rax = (crate::simd_ops::ru32(P0)) as u32 as u64;
    crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rax as u32)) as u32);
    crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (0xffffffffffffffffu64) as u64);
    rdx = rsp.wrapping_add(0x20);
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
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
pub unsafe extern "win64" fn tpl_40933<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
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
    loop { rcx = rbx; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rbx = ((rbx).wrapping_add(0x20u64)) as u64; fl_l = rdi; fl_r = (0x1u64) as u64; rdi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if !((((fl_l as u64)) != ((fl_r as u64)))) { break; } }
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_40934(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    r8 = ((r8.wrapping_add(r8.wrapping_mul(2))) as u64) as u64;
    r8 = ((r8).wrapping_shl(3u32)) as u64;
    fl_l = rax; fl_r = r8;
    rax = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64;
    crate::simd_ops::wu64(rdx, rax);
    rax = rdx;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_40935<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    crate::simd_ops::forward_incoming_to_outgoing();
    let (mut rax, mut fl_l) = (crate::simd_ops::get_rax_in(), 0u64);
    rcx = ((rdx.wrapping_add(rdx.wrapping_mul(2))) as u64) as u64;
    rdx = 0x8;
    fl_l = ((rcx).wrapping_shl(3u32)) as u64;
    rcx = ((fl_l as u64)) as u64;
    crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8(P0, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_40940(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (0u64, 0u64, 0u64);
    rax = ((crate::simd_ops::ru16(rdx)) as u32) as u32 as u64;
    r9 = 0x3fff;
    rcx = ((crate::simd_ops::ru16(r8)) as u32) as u32 as u64;
    rax = (rax & !0xffffu64) | ((((((rax as u16)) & ((r9 as u16)))) as u16 as u64));
    rcx = (rcx & !0xffffu64) | ((((((rcx as u16)) & ((r9 as u16)))) as u16 as u64));
    fl_l = ((rax as u16)) as u16 as u64; fl_r = ((rcx as u16)) as u16 as u64;
    if (((fl_l as u16)) != ((fl_r as u16))) { rax = (rax & !0xffu64) | (((if (((fl_l as u16)) < ((fl_r as u16))) { 1u8 } else { 0u8 }) as u8 as u64)); return rax; }
    rax = (crate::simd_ops::ru64(r8)) as u64;
    fl_l = (crate::simd_ops::ru64(rdx)) as u64; fl_r = rax;
    rax = (rax & !0xffu64) | (((if (((fl_l as u64)) < ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64));
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_40948<const P0: u64, const P1: u64, const P2: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64;
    r8 = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64;
    crate::simd_ops::wu64(rdx, rax);
    rax = rdx;
    crate::simd_ops::wu64(rdx.wrapping_add(P2), r8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_40976<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64, const P9: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = (crate::simd_ops::ru64(P0)) as u64;
    r8 = rcx;
    rdx = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64;
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    crate::simd_ops::wu64(rax.wrapping_add(P2), rdx);
    rax = (crate::simd_ops::ru64(P0)) as u64;
    rdx = (crate::simd_ops::ru32(rax.wrapping_add(P3))) as u32 as u64;
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    crate::simd_ops::wu32(rax.wrapping_add(P4), ((rdx as u32)) as u32);
    rax = (crate::simd_ops::ru64(P5)) as u64;
    rcx = (crate::simd_ops::ru64(rax.wrapping_add(P6))) as u64;
    rax = (crate::simd_ops::ru64(r8)) as u64;
    crate::simd_ops::wu64(rax.wrapping_add(P7), rcx);
    rax = (crate::simd_ops::ru64(P5)) as u64;
    rcx = (crate::simd_ops::ru64(rax.wrapping_add(P8))) as u64;
    rax = (crate::simd_ops::ru64(r8)) as u64;
    crate::simd_ops::wu64(rax.wrapping_add(P9), rcx);
    rcx = 0;
    fl_l = 0; fl_r = 0;
    rdx = (crate::simd_ops::ru64(P0)) as u64;
    rax = ((rcx as u32)) as u32 as u64;
    let tmp_xchg: u64 = crate::simd_ops::atomic_xchg_u64(rdx.wrapping_add(P1), rax);
    rax = (tmp_xchg) as u64;
    let tmp_xchg: u32 = crate::simd_ops::atomic_xchg_u32(rdx.wrapping_add(P3), ((rcx as u32)) as u32);
    rcx = (tmp_xchg) as u32 as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_40979<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu32(rsp.wrapping_add(0x10u64), ((rdx as u32)) as u32);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    rcx = ((rcx).wrapping_add(0xcu64)) as u64;
    rdx = rsp.wrapping_add(0x38);
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_40988<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
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
    loop { rcx = ((rbx.wrapping_add(P0)) as u64) as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rcx = rbx; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rbx = ((rbx).wrapping_add(0x30u64)) as u64; fl_l = rdi; fl_r = (0x1u64) as u64; rdi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if !((((fl_l as u64)) != ((fl_r as u64)))) { break; } }
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_40992<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
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
    loop { rcx = rbx; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rbx = ((rbx).wrapping_add(0x98u64)) as u64; fl_l = rdi; fl_r = (0x1u64) as u64; rdi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if !((((fl_l as u64)) != ((fl_r as u64)))) { break; } }
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_40998<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; r15 = r9; r12 = r8; r14 = ((rdx as u32)) as u32 as u64; rbx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; r10 = (crate::simd_ops::ru64(rcx)) as u64; rdi = ((rbx.wrapping_add(r10.wrapping_mul(8))) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), rdi); fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 4; continue; } r13 = 0; fl_l = 0; fl_r = 0; bb = 1; continue; }
            1 => { rsi = (crate::simd_ops::ru64(rbx)) as u64; rbp = rsi; rbp = ((rbp).wrapping_shr(32u32)) as u64; fl_l = ((rsi as u32)) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 3; continue; } rdi = P1; bb = 2; continue; }
            2 => { rax = ((rsi as u32)) as u32 as u64; r8 = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; r8 = ((r8).wrapping_shl(4u32)) as u64; r8 = ((r8).wrapping_add(0x8u64)) as u64; rax = ((r14 as u32)) as u32 as u64; rdx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rdx = ((rdx).wrapping_shl(4u32)) as u64; rdx = ((rdx).wrapping_add(r15)) as u64; r14 = (((r14 as u32)).wrapping_add(1)) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdx); crate::simd_ops::wu64(rdx, rdi); rcx = ((rdx.wrapping_add(P0)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rcx); crate::simd_ops::lk_and_u32(rcx, (0xff000004u32) as u32); crate::simd_ops::lk_or_u32(rcx, (0x4u32) as u32); crate::simd_ops::wu8(rdx.wrapping_add(P2), (0x4u8) as u8); crate::simd_ops::wu32(rdx.wrapping_add(P3), ((r13 as u32)) as u32); crate::simd_ops::wu8(rdx.wrapping_add(P4), (0x0u8) as u8); rdx = ((r8.wrapping_add(r12)) as u64) as u64; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rsi = (((rsi as u32)).wrapping_add(1)) as u32 as u64; fl_l = ((rsi as u32)) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 2; continue; } rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; bb = 3; continue; }
            3 => { rbx = ((rbx).wrapping_add(0x8u64)) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } bb = 4; continue; }
            4 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_40999<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    r9 = rcx;
    rax = ((r8 as u32)) as u32 as u64;
    rax = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64;
    rax = ((rax).wrapping_shl(4u32)) as u64;
    rax = ((rax).wrapping_add(rdx)) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax);
    rcx = P0;
    crate::simd_ops::wu64(rax, rcx);
    rcx = ((rax.wrapping_add(P1)) as u64) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rcx);
    crate::simd_ops::lk_and_u32(rcx, (0xff000004u32) as u32);
    crate::simd_ops::lk_or_u32(rcx, (0x4u32) as u32);
    crate::simd_ops::wu8(rcx.wrapping_add(P2), (0x4u8) as u8);
    crate::simd_ops::wu32(rcx.wrapping_add(P3), (0x0u32) as u32);
    crate::simd_ops::wu8(rcx.wrapping_add(P1), (0x0u8) as u8);
    rdx = ((r9.wrapping_add(P1)) as u64) as u64;
    rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_41000(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
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
    loop { rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = rbx; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(rax)) as u64, rax, rcx, rdx, r8, r9, rsp); rbx = ((rbx).wrapping_add(0x30u64)) as u64; fl_l = rdi; fl_r = (0x1u64) as u64; rdi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if !((((fl_l as u64)) != ((fl_r as u64)))) { break; } }
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_41056<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    rcx = (crate::simd_ops::ru64(rax)) as u64;
    rax = rdx;
    crate::simd_ops::wu64(rdx, rcx);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_41088<const P0: u64, const P1: u64, const P2: u64, const P3: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rcx; r15 = rdx.wrapping_add(0x580f0); rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rax = ((rax as u32)) as u32 as u64; rsi = (((rax) as i64).wrapping_mul((0x188u64) as i64) as u64) as u64; rsi = ((rsi).wrapping_add(r15)) as u64; rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; r12 = ((rax.wrapping_add(rdx.wrapping_mul(8))) as u64) as u64; rbp = (crate::simd_ops::ru64(r12.wrapping_add(0x10u64))) as u64; rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x58u64))) as u64; rcx = rax; rcx = ((rcx).wrapping_shr(32u32)) as u64; r14 = 0; fl_l = 0; fl_r = 0; fl_l = ((rbp as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 1; continue; } rax = rbp; rax = ((rax).wrapping_shr(32u32)) as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; rcx = ((r14 as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rcx = 0x1; bb = 2; continue; }
            2 => { crate::simd_ops::lk_add_u64(rsi.wrapping_add(0x60u64), (rcx) as u64); crate::simd_ops::wu64(rsi.wrapping_add(0x58u64), rbp); rbx = ((crate::simd_ops::ru16(P1)) as u32) as u32 as u64; rdi = rsi.wrapping_add(0x158); rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } rax = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; rax = ((rax).wrapping_add(1)) as u64; fl_l = rax; fl_r = (0x4000u64) as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rcx = rdi; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 4; continue; }
            4 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu32(rcx.wrapping_add(rax.wrapping_mul(4)).wrapping_add(0x10u64), ((rbp as u32)) as u32); rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; crate::simd_ops::lk_add_u64(rax.wrapping_add(0x8u64), (1) as u64); crate::simd_ops::lk_add_u64(rdi.wrapping_add(0x10u64), (1) as u64); rsi = ((rsi).wrapping_add(0x170u64)) as u64; rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } rax = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; rax = ((rax).wrapping_add(1)) as u64; fl_l = rax; fl_r = (0x8000u64) as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rcx = rsi; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 6; continue; }
            6 => { rcx = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu16(rcx.wrapping_add(rax.wrapping_mul(2)).wrapping_add(0x10u64), ((rbx as u16)) as u16); rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; crate::simd_ops::lk_add_u64(rax.wrapping_add(0x8u64), (1) as u64); crate::simd_ops::lk_add_u64(rsi.wrapping_add(0x10u64), (1) as u64); rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rax = ((rax as u32)) as u32 as u64; rsi = (((rax) as i64).wrapping_mul((0x188u64) as i64) as u64) as u64; rsi = ((rsi).wrapping_add(r15)) as u64; rbp = (crate::simd_ops::ru64(r12.wrapping_add(0x10u64))) as u64; rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x58u64))) as u64; rcx = rax; rcx = ((rcx).wrapping_shr(32u32)) as u64; fl_l = ((rbp as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 7; continue; } rax = rbp; rax = ((rax).wrapping_shr(32u32)) as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 8; continue; } bb = 7; continue; }
            7 => { r14 = 0x1; bb = 8; continue; }
            8 => { crate::simd_ops::lk_add_u64(rsi.wrapping_add(0x60u64), (r14) as u64); crate::simd_ops::wu64(rsi.wrapping_add(0x58u64), rbp); rbx = ((crate::simd_ops::ru16(P3)) as u32) as u32 as u64; rdi = rsi.wrapping_add(0x158); rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } rax = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; rax = ((rax).wrapping_add(1)) as u64; fl_l = rax; fl_r = (0x4000u64) as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 10; continue; } bb = 9; continue; }
            9 => { rcx = rdi; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 10; continue; }
            10 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu32(rcx.wrapping_add(rax.wrapping_mul(4)).wrapping_add(0x10u64), ((rbp as u32)) as u32); rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; crate::simd_ops::lk_add_u64(rax.wrapping_add(0x8u64), (1) as u64); crate::simd_ops::lk_add_u64(rdi.wrapping_add(0x10u64), (1) as u64); rdi = rsi.wrapping_add(0x170); rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } rax = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; rax = ((rax).wrapping_add(1)) as u64; fl_l = rax; fl_r = (0x8000u64) as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 12; continue; } bb = 11; continue; }
            11 => { rcx = rdi; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 12; continue; }
            12 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu16(rcx.wrapping_add(rax.wrapping_mul(2)).wrapping_add(0x10u64), ((rbx as u16)) as u16); rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; crate::simd_ops::lk_add_u64(rax.wrapping_add(0x8u64), (1) as u64); crate::simd_ops::lk_add_u64(rdi.wrapping_add(0x10u64), (1) as u64); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_41094<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    crate::simd_ops::forward_incoming_to_outgoing();
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rcx = (crate::simd_ops::ru64(rcx)) as u64;
    fl_l = rcx; fl_r = rcx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rdx = 0x58; crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8(P0, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_41095<const P0: u64, const P1: u64, const P2: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rdi = ((rcx.wrapping_add(P0)) as u64) as u64;
    rbx = rcx;
    rcx = (crate::simd_ops::ru64(rdi.wrapping_add(P1))) as u64;
    fl_l = rcx; fl_r = rcx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rax = (crate::simd_ops::ru64(rcx)) as u64; fl_l = rcx; fl_r = rdi; rdx = (rdx & !0xffu64) | (((if (((fl_l as u64)) != ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rdi.wrapping_add(P1), (0x0u64) as u64); }
    rax = rbx;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_41132<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64, const P9: u64, const P10: u64, const P11: u64, const P12: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut rbp) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut ymm0_hi: u128 = 0; let mut ymm1_hi: u128 = 0; let mut ymm2_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 45>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rbp = rsp.wrapping_sub(0x60); rsp = ((rsp).wrapping_sub(0x160u64)) as u64; xmm0 = (crate::simd_ops::ru128(r8)) as u128; ymm0_hi = 0; xmm1 = ((crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64) as u128; ymm1_hi = 0; rax = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; r8 = rsp.wrapping_add(0x20); let y_addr: u64 = rcx; xmm2 = crate::simd_ops::ru128(y_addr); ymm2_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); crate::simd_ops::wu64(rbp.wrapping_sub(0x80u64), rax); rax = (crate::simd_ops::ru64(rcx.wrapping_add(P2))) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x78u64), rax); rax = (crate::simd_ops::ru64(rcx.wrapping_add(P3))) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x70u64), rax); rax = (crate::simd_ops::ru64(rcx.wrapping_add(P4))) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x68u64), rax); rax = (crate::simd_ops::ru64(rcx.wrapping_add(P5))) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x60u64), rax); rax = (crate::simd_ops::ru64(rcx.wrapping_add(P6))) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x58u64), rax); rax = (crate::simd_ops::ru64(rcx.wrapping_add(P7))) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x50u64), rax); rax = (crate::simd_ops::ru64(rcx.wrapping_add(P8))) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x48u64), rax); rax = (crate::simd_ops::ru64(rcx.wrapping_add(P9))) as u64; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128); xmm0 = (crate::simd_ops::ru128(rdx)) as u128; ymm0_hi = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), ((xmm1 as u64)) as u64); xmm1 = ((crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64) as u128; ymm1_hi = 0; crate::simd_ops::wu64(rbp.wrapping_sub(0x40u64), rax); rdx = rsp.wrapping_add(0x40); rax = (crate::simd_ops::ru64(rcx.wrapping_add(P10))) as u64; crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm0) as u128); let y_addr: u64 = rbp.wrapping_sub(0x80u64); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), ((xmm1 as u64)) as u64); let y_addr: u64 = rbp.wrapping_sub(0x60u64); xmm1 = crate::simd_ops::ru128(y_addr); ymm1_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); crate::simd_ops::wu64(rbp.wrapping_sub(0x38u64), rax); rax = (crate::simd_ops::ru64(rcx.wrapping_add(P11))) as u64; rcx = rbp.wrapping_sub(0x20); let y_addr: u64 = rbp; crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); xmm0 = (crate::simd_ops::ru128(rbp.wrapping_sub(0x40u64))) as u128; ymm0_hi = 0; let y_addr: u64 = rbp.wrapping_add(0x20u64); crate::simd_ops::wu128(y_addr, xmm1); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm1_hi); crate::simd_ops::wu64(rbp.wrapping_sub(0x30u64), rax); xmm1 = ((crate::simd_ops::ru64(rbp.wrapping_sub(0x30u64))) as u64) as u128; ymm1_hi = 0; crate::simd_ops::wu64(rbp.wrapping_add(0x50u64), ((xmm1 as u64)) as u64); let y_addr: u64 = rbp.wrapping_sub(0x20u64); crate::simd_ops::wu128(y_addr, xmm2); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm2_hi); crate::simd_ops::wu128(rbp.wrapping_add(0x40u64), (xmm0) as u128); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P12, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rsp = ((rsp).wrapping_add(0x160u64)) as u64; rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_41155<const P0: u64, const P1: u64, const P2: u64, const P3: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 49>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        core::sync::atomic::fence(core::sync::atomic::Ordering::SeqCst);
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x170u64)) as u64; rbx = rdx; rdi = rcx; rsi = (crate::simd_ops::ru64(rdx.wrapping_add(0x28u64))) as u64; rbp = (crate::simd_ops::ru64(rdx.wrapping_add(0x20u64))) as u64; r8 = rdx.wrapping_add(0x8); rdx = ((crate::simd_ops::ru8(rdx.wrapping_add(0x3u64))) as u32) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0x80; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 1; continue; } rdx = ((rdx.wrapping_mul(8)) as u32) as u32 as u64; bb = 2; continue; }
            1 => { rdx = (((rdx as u32)).wrapping_shl(8u32)) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0x7c00; rdx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru32(rbx)) as u32 as u64; rax = ((((rax as u32)) & (0xffffffu32))) as u32 as u64; rax = (((rax as u32)).wrapping_shl(3u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 3; continue; } r8 = (crate::simd_ops::ru64(r8)) as u64; bb = 3; continue; }
            3 => { r9 = (crate::simd_ops::ru32(rbx.wrapping_add(0x4u64))) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r8); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), r9); rax = P0; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rax); crate::simd_ops::wu32(rsp.wrapping_add(0x48u64), (0x0u32) as u32); rax = rsp.wrapping_add(0x4c); crate::simd_ops::wu64(rsp.wrapping_add(0x190u64), rax); rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x4cu64))) as u32 as u64; rax = ((((rax as u32)) & (0xff000020u32))) as u32 as u64; rax = ((((rax as u32)) | (0x20u32))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x4cu64), ((rax as u32)) as u32); crate::simd_ops::wu8(rsp.wrapping_add(0x4fu64), (0x20u8) as u8); crate::simd_ops::wu32(rsp.wrapping_add(0x50u64), (0x0u32) as u32); crate::simd_ops::wu8(rsp.wrapping_add(0x54u64), (0x0u8) as u8); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm0) as u128); r8 = 0; fl_l = 0; fl_r = 0; rdx = rsp.wrapping_add(0x30); rcx = rsp.wrapping_add(0x4c); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rsp.wrapping_add(0x158u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x160u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x168u64), (0x0u64) as u64); rdx = rsp.wrapping_add(0x40); rcx = (crate::simd_ops::ru64(rdi)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru64(rdi)) as u64; fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0x2a0u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 4; continue; } fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0x2a1u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 4; continue; } rax = 0x3; let tmp_xchg: u32 = crate::simd_ops::atomic_xchg_u32(rcx.wrapping_add(0x4u64), ((rax as u32)) as u32); rax = (tmp_xchg) as u32 as u64; bb = 4; continue; }
            4 => { rcx = rsp.wrapping_add(0x4c); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x30u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x48u64))) as u64; r9 = ((crate::simd_ops::ru16(rbx.wrapping_add(0x40u64))) as u32) as u32 as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; r8 = (crate::simd_ops::ru64(rbx.wrapping_add(0x38u64))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm((crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 5; continue; }
            5 => { rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x1a0u64))) as u64; rsp = ((rsp).wrapping_add(0x170u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_41168<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r11, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x60u64)) as u64; rdi = (crate::simd_ops::ru64(P0)) as u64; rsi = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; rbx = (crate::simd_ops::ru64(rdi.wrapping_add(P2))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = ((rax as u32)) as u32 as u64; rcx = (((rax) as i64).wrapping_mul((0x188u64) as i64) as u64) as u64; r8 = ((rbx.wrapping_add(P4)) as u64) as u64; r8 = ((r8).wrapping_add(rcx)) as u64; r11 = (crate::simd_ops::ru64(rdi.wrapping_add(P2))) as u64; rdx = ((crate::simd_ops::ru16(P5)) as u32) as u32 as u64; rax = (crate::simd_ops::ru64(r11)) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(P6))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r11); rcx = ((crate::simd_ops::ru16(rcx.wrapping_add(rdx.wrapping_mul(2)))) as u32) as u32 as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(r11.wrapping_add(0x8u64)))) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(P2))) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax); rax = (crate::simd_ops::ru64(rcx)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm0) as u128); xmm1 = ((crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64) as u128; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), ((xmm1 as u64)) as u64); r9 = rsi; rdx = rsp.wrapping_add(0x40); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; rsp = ((rsp).wrapping_add(0x60u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_41177<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut ymm0_hi: u128 = 0; let mut ymm1_hi: u128 = 0;
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let y_addr: u64 = rdx; xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16));
    let y_addr: u64 = rdx.wrapping_add(P0); xmm1 = crate::simd_ops::ru128(y_addr); ymm1_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16));
    let y_addr: u64 = rcx.wrapping_add(P1); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi);
    xmm0 = (crate::simd_ops::ru128(rdx.wrapping_add(P2))) as u128; ymm0_hi = 0;
    rax = P3;
    let y_addr: u64 = rcx.wrapping_add(P4); crate::simd_ops::wu128(y_addr, xmm1); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm1_hi);
    crate::simd_ops::wu64(rcx, rax);
    rax = rcx;
    crate::simd_ops::wu128(rcx.wrapping_add(P5), (xmm0) as u128);
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_41194<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rax = (crate::simd_ops::ru32(rdx)) as u32 as u64;
    rbx = rcx;
    crate::simd_ops::wu32(rcx, ((rax as u32)) as u32);
    rdx = ((rdx).wrapping_add(0x4u64)) as u64;
    rcx = ((rcx).wrapping_add(0x4u64)) as u64;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rax = rbx;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_41210<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rcx;
    rax = P0;
    crate::simd_ops::wu64(rcx, rax);
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    crate::simd_ops::wu64(rbx.wrapping_add(P2), rax);
    crate::simd_ops::wu16(rbx.wrapping_add(P3), (0x0u16) as u16);
    rax = P4;
    crate::simd_ops::wu64(rbx, rax);
    rax = rbx;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_41215<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        core::sync::atomic::fence(core::sync::atomic::Ordering::SeqCst);
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rsi = rcx; rbx = (crate::simd_ops::ru64(rcx)) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } rax = 0xffffffff; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rbx.wrapping_add(P0), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 3; continue; } rdi = (crate::simd_ops::ru64(rbx.wrapping_add(P1))) as u64; fl_l = (((rdi) & (0xfffffffffffffffcu64))) as u64; fl_r = fl_l; rdi = ((fl_l as u64)) as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 1; continue; } rax = (crate::simd_ops::ru64(rdi)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = rdi; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = rdi; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(P4))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 2; continue; } fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(P5))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 2; continue; } rcx = ((rbx.wrapping_add(P6)) as u64) as u64; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); bb = 2; continue; }
            2 => { rcx = rbx; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 3; continue; }
            3 => { crate::simd_ops::wu64(rsi, (0x0u64) as u64); bb = 4; continue; }
            4 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_41222<const P0: u64, const P1: u64, const P2: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    crate::simd_ops::forward_incoming_to_outgoing();
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(P0))) as u8 as u64; fl_r = (0x0u8) as u8 as u64;
    if (((fl_l as u8)) == ((fl_r as u8))) { return rax; }
    fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(P1))) as u8 as u64; fl_r = (0x0u8) as u8 as u64;
    if !((((fl_l as u8)) == ((fl_r as u8)))) { fl_l = ((rcx).wrapping_add(0x4u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8(P2, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_41230<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    crate::simd_ops::wu8(rcx, (0x1u8) as u8);
    rax = (crate::simd_ops::ru64(rdx)) as u64;
    crate::simd_ops::wu64(rcx.wrapping_add(P0), rax);
    rax = rcx;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_41231<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    crate::simd_ops::forward_incoming_to_outgoing();
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    fl_l = (crate::simd_ops::ru8(rcx)) as u8 as u64; fl_r = (0x0u8) as u8 as u64;
    if !((((fl_l as u8)) == ((fl_r as u8)))) { rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8(P1, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_41250<const P0: u64, const P1: u64, const P2: u64, const P3: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx);
    crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp);
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbp = rcx;
    rsi = ((rcx.wrapping_add(P0)) as u64) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rsi);
    rcx = rsi;
    rax = crate::simd_ops::call_stk_caught((crate::simd_ops::ru64(P1)) as u64, rax, rcx, rdx, r8, r9, rsp);
    rbx = (crate::simd_ops::ru64(rbp.wrapping_add(0xc0u64))) as u64;
    rax = (crate::simd_ops::ru32(rbp.wrapping_add(0xc8u64))) as u32 as u64;
    rdi = (((rax) as i64).wrapping_mul((0xc8u64) as i64) as u64) as u64;
    rdi = ((rdi).wrapping_add(rbx)) as u64;
    fl_l = rbx; fl_r = rdi;
    if (((fl_l as u64)) == ((fl_r as u64))) { rcx = rsi; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk_caught((crate::simd_ops::ru64(P3)) as u64, rax, rcx, rdx, r8, r9, rsp); }
    loop { rdx = rbx; rcx = rbp; rax = crate::simd_ops::call_stk_caught(P2, rax, rcx, rdx, r8, r9, rsp); rbx = ((rbx).wrapping_add(0xc8u64)) as u64; fl_l = rbx; fl_r = rdi; if !((((fl_l as u64)) != ((fl_r as u64)))) { break; } }
    rcx = rsi;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64;
    rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64;
    rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return crate::simd_ops::tail_stk_caught((crate::simd_ops::ru64(P3)) as u64, rax, rcx, rdx, r8, r9, rsp);
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_41253<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    crate::simd_ops::forward_incoming_to_outgoing();
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    r8 = 0x1;
    crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8(P0, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_41256<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x30u64)) as u64;
    rbx = rcx;
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rcx);
    rcx = ((rdx.wrapping_add(P0)) as u64) as u64;
    rdi = rdx;
    rax = P1;
    rdx = rsp.wrapping_add(0x20);
    crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax);
    rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp);
    rax = P3;
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbx);
    rcx = ((rdi.wrapping_add(P4)) as u64) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax);
    rdx = rsp.wrapping_add(0x20);
    rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp);
    rax = P5;
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbx);
    rcx = ((rdi.wrapping_add(P6)) as u64) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax);
    rdx = rsp.wrapping_add(0x20);
    rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp);
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64;
    rsp = ((rsp).wrapping_add(0x30u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_41275<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 19>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = rsp; crate::simd_ops::wu64(rax.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rax.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rax.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rax.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x80u64)) as u64; rbx = rcx.wrapping_add(0x8); xmm0 = (crate::simd_ops::ru128(P0)) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rax.wrapping_sub(0x78u64), (xmm0) as u128); rcx = rbx.wrapping_add(0x18); r8 = rax.wrapping_sub(0x78); rdx = rax.wrapping_sub(0x68); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rbx); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; xmm0 = (crate::simd_ops::ru128(rax.wrapping_add(0x10u64))) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rsp.wrapping_add(0x48u64), (xmm0) as u128); rdx = rsp.wrapping_add(0x58); rcx = rsp.wrapping_add(0x40); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = (crate::simd_ops::ru64(rax)) as u64; rbp = ((crate::simd_ops::ru16(rax.wrapping_add(0xau64))) as u32) as u32 as u64; xmm0 = (crate::simd_ops::ru128(rax.wrapping_add(0x10u64))) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128); rcx = rbx.wrapping_add(0x48); xmm1 = (crate::simd_ops::ru128(P3)) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm1) as u128); rdx = rsp.wrapping_add(0x30); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r14 = rax; rdi = ((crate::simd_ops::ru16(rax)) as u32) as u32 as u64; rsi = rbx.wrapping_add(0x8); r15 = 0xffff; r12 = (0x2aaaaaaaaaaaaaabu64) as u64; fl_l = ((rdi as u16)) as u16 as u64; fl_r = ((r15 as u16)) as u16 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { bb = 2; continue; } rbx = (crate::simd_ops::ru64(rsi)) as u64; xmm0 = (crate::simd_ops::ru128(P3)) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm0) as u128); rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x18u64))) as u32 as u64; r8 = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; r8 = ((r8).wrapping_shl(4u32)) as u64; rax = r12; let prod = (rax as i64 as i128).wrapping_mul((r8) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; rdi = rdx; rdi = ((((rdi) as i64).wrapping_shr(3u32) as u64)) as u64; rax = rdi; rax = ((rax).wrapping_shr(63u32)) as u64; rdi = ((rdi).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x1cu64))) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; fl_l = r8; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } rdx = rdi.wrapping_add(0x1); rcx = rbx.wrapping_add(0x10); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x18u64))) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rbx.wrapping_add(0x10u64)))) as u64; xmm0 = 0u128; ymm0_hi = 0; let y_addr: u64 = rcx; crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); crate::simd_ops::wu128(rcx.wrapping_add(0x20u64), (xmm0) as u128); crate::simd_ops::lk_add_u32(rbx.wrapping_add(0x18u64), (1) as u32); rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x18u64))) as u32 as u64; rdx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rdx = ((rdx).wrapping_shl(4u32)) as u64; fl_l = ((rdx).wrapping_add(crate::simd_ops::ru64(rbx.wrapping_add(0x10u64)))) as u64; fl_r = fl_l; rdx = ((fl_l as u64)) as u64; xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x30u64))) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rdx.wrapping_sub(0x30u64), (xmm0) as u128); crate::simd_ops::wu32(rdx.wrapping_sub(0xeu64), (0xffffffffu32) as u32); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(rdx.wrapping_sub(0x20u64), rax); crate::simd_ops::wu64(rdx.wrapping_sub(0x18u64), rdi); crate::simd_ops::wu16(r14, ((rdi as u16)) as u16); bb = 2; continue; }
            2 => { r9 = 0x2; r8 = (((rdi as u16)) as u32) as u32 as u64; rdx = (((rbp as u16)) as u32) as u32 as u64; rcx = (crate::simd_ops::ru64(rsi)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = rsp.wrapping_add(0x58); rcx = rsp.wrapping_add(0x40); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = (crate::simd_ops::ru64(rax)) as u64; rbp = ((crate::simd_ops::ru16(rax.wrapping_add(0xau64))) as u32) as u32 as u64; xmm0 = (crate::simd_ops::ru128(rax.wrapping_add(0x10u64))) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128); rcx = rbx.wrapping_add(0x48); xmm1 = (crate::simd_ops::ru128(P8)) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm1) as u128); rdx = rsp.wrapping_add(0x30); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r14 = rax; rdi = ((crate::simd_ops::ru16(rax)) as u32) as u32 as u64; fl_l = ((rdi as u16)) as u16 as u64; fl_r = ((r15 as u16)) as u16 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { bb = 4; continue; } rsi = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; xmm0 = (crate::simd_ops::ru128(P8)) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm0) as u128); rcx = (crate::simd_ops::ru32(rsi.wrapping_add(0x18u64))) as u32 as u64; r8 = ((rcx.wrapping_add(rcx.wrapping_mul(2))) as u64) as u64; r8 = ((r8).wrapping_shl(4u32)) as u64; rax = r12; let prod = (rax as i64 as i128).wrapping_mul((r8) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; rdi = rdx; rdi = ((((rdi) as i64).wrapping_shr(3u32) as u64)) as u64; rax = rdi; rax = ((rax).wrapping_shr(63u32)) as u64; rdi = ((rdi).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru32(rsi.wrapping_add(0x1cu64))) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; fl_l = r8; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } rdx = rdi.wrapping_add(0x1); rcx = rsi.wrapping_add(0x10); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru32(rsi.wrapping_add(0x18u64))) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rsi.wrapping_add(0x10u64)))) as u64; xmm0 = 0u128; ymm0_hi = 0; let y_addr: u64 = rcx; crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); crate::simd_ops::wu128(rcx.wrapping_add(0x20u64), (xmm0) as u128); crate::simd_ops::lk_add_u32(rsi.wrapping_add(0x18u64), (1) as u32); rax = (crate::simd_ops::ru32(rsi.wrapping_add(0x18u64))) as u32 as u64; rdx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rdx = ((rdx).wrapping_shl(4u32)) as u64; fl_l = ((rdx).wrapping_add(crate::simd_ops::ru64(rsi.wrapping_add(0x10u64)))) as u64; fl_r = fl_l; rdx = ((fl_l as u64)) as u64; xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x30u64))) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rdx.wrapping_sub(0x30u64), (xmm0) as u128); crate::simd_ops::wu32(rdx.wrapping_sub(0xeu64), (0xffffffffu32) as u32); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(rdx.wrapping_sub(0x20u64), rax); crate::simd_ops::wu64(rdx.wrapping_sub(0x18u64), rdi); crate::simd_ops::wu16(r14, ((rdi as u16)) as u16); bb = 4; continue; }
            4 => { r9 = 0x2; r8 = (((rdi as u16)) as u32) as u32 as u64; rdx = (((rbp as u16)) as u32) as u32 as u64; rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r11 = rsp.wrapping_add(0x80); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x20u64))) as u64; rbp = (crate::simd_ops::ru64(r11.wrapping_add(0x28u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x30u64))) as u64; rdi = (crate::simd_ops::ru64(r11.wrapping_add(0x38u64))) as u64; rsp = r11; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_41277<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    xmm0 = (crate::simd_ops::ru128(rcx)) as u128;
    rax = ((r8 as u32)) as u32 as u64;
    r8 = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64;
    crate::simd_ops::wu128(rdx.wrapping_add(r8.wrapping_mul(4)), (xmm0) as u128);
    rax = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64;
    crate::simd_ops::wu32(rdx.wrapping_add(r8.wrapping_mul(4)).wrapping_add(P0), ((rax as u32)) as u32);
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_41289<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rdx; r13 = rcx; r12 = (crate::simd_ops::ru64(rcx.wrapping_add(0x18u64))) as u64; r15 = 0; fl_l = 0; fl_r = 0; fl_l = r12; fl_r = r12; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } rbp = rdx.wrapping_add(0x158); rsi = rdx.wrapping_add(0x170); bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(r13.wrapping_add(0x8u64))) as u64; rdx = (crate::simd_ops::ru64(rax.wrapping_add(r15.wrapping_mul(8)).wrapping_add(0x10u64))) as u64; rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x58u64))) as u64; rcx = rax; rcx = ((rcx).wrapping_shr(32u32)) as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 2; continue; } rax = rdx; rax = ((rax).wrapping_shr(32u32)) as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 2; continue; } rcx = 0; fl_l = 0; fl_r = 0; bb = 3; continue; }
            2 => { rcx = 0x1; bb = 3; continue; }
            3 => { r14 = rdx; crate::simd_ops::lk_add_u64(rbx.wrapping_add(0x60u64), (rcx) as u64); crate::simd_ops::wu64(rbx.wrapping_add(0x58u64), rdx); rdi = ((crate::simd_ops::ru16(P0)) as u32) as u32 as u64; rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x8u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } rax = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; rax = ((rax).wrapping_add(1)) as u64; fl_l = rax; fl_r = (0x4000u64) as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 5; continue; } bb = 4; continue; }
            4 => { rcx = rbp; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 5; continue; }
            5 => { rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x8u64))) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu32(rcx.wrapping_add(rax.wrapping_mul(4)).wrapping_add(0x10u64), ((r14 as u32)) as u32); rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x8u64))) as u64; crate::simd_ops::lk_add_u64(rax.wrapping_add(0x8u64), (1) as u64); crate::simd_ops::lk_add_u64(rbp.wrapping_add(0x10u64), (1) as u64); rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } rax = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; rax = ((rax).wrapping_add(1)) as u64; fl_l = rax; fl_r = (0x8000u64) as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 7; continue; } bb = 6; continue; }
            6 => { rcx = rsi; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 7; continue; }
            7 => { rcx = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu16(rcx.wrapping_add(rax.wrapping_mul(2)).wrapping_add(0x10u64), ((rdi as u16)) as u16); rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; crate::simd_ops::lk_add_u64(rax.wrapping_add(0x8u64), (1) as u64); crate::simd_ops::lk_add_u64(rsi.wrapping_add(0x10u64), (1) as u64); r15 = ((r15).wrapping_add(1)) as u64; fl_l = r15; fl_r = r12; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 1; continue; } bb = 8; continue; }
            8 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_41290<const P0: u64, const P1: u64, const P2: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x60u64)) as u64; rbx = rdx; rbp = rcx; rsi = (crate::simd_ops::ru64(rcx.wrapping_add(0x18u64))) as u64; r14 = 0; fl_l = 0; fl_r = 0; rdi = ((r14 as u32)) as u32 as u64; fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), r14); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), r14); crate::simd_ops::wu32(rsp.wrapping_add(0x50u64), ((r14 as u32)) as u32); rax = (crate::simd_ops::ru64(rax.wrapping_add(rdi.wrapping_mul(8)).wrapping_add(0x10u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); crate::simd_ops::wu8(rsp.wrapping_add(0x28u64), (0x0u8) as u8); xmm0 = ((crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64) as u128; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), ((xmm0 as u64)) as u64); rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x28u64))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x38u64), ((rax as u32)) as u32); r8 = rsp.wrapping_add(0x40); rdx = rsp.wrapping_add(0x30); rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = (crate::simd_ops::ru32(rsp.wrapping_add(0x4cu64))) as u32 as u64; fl_r = 0x40000000; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 5; continue; } r10 = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; fl_l = r10; fl_r = r10; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } r10 = ((r10).wrapping_sub(1)) as u64; r10 = (((r10) & (0xfffffffffe000000u64))) as u64; rax = ((crate::exe::teb_self() as u64)) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(r10.wrapping_add(0x100u64))) as u64; rcx = (rcx & !0xffu64) | (((if (((fl_l as u64)) == ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); r9 = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rax = r9; rax = ((rax).wrapping_sub(r10)) as u64; rax = ((rax).wrapping_shr(16u32)) as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rdx = r10.wrapping_add(0x108); rdx = ((rdx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x4u64))) as u32 as u64; rdx = ((rdx).wrapping_sub(rax)) as u64; fl_l = ((rcx as u8)) as u8 as u64; fl_r = ((rcx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 3; continue; } fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(0xeu64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 2; continue; } rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(r9, rax); crate::simd_ops::wu64(rdx.wrapping_add(0x20u64), r9); fl_l = (crate::simd_ops::lk_add_u32(rdx.wrapping_add(0x18u64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; if (((fl_l as u32)) != 0) { bb = 5; continue; } rcx = rdx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 5; continue; }
            2 => { r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 4; continue; }
            3 => { r8 = 0; fl_l = 0; fl_r = 0; bb = 4; continue; }
            4 => { rcx = r10; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 5; continue; }
            5 => { rdi = ((rdi).wrapping_add(1)) as u64; fl_l = rdi; fl_r = rsi; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 1; continue; } bb = 6; continue; }
            6 => { r11 = rsp.wrapping_add(0x60); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x10u64))) as u64; rbp = (crate::simd_ops::ru64(r11.wrapping_add(0x18u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x20u64))) as u64; rdi = (crate::simd_ops::ru64(r11.wrapping_add(0x28u64))) as u64; rsp = r11; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_41310<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64, const P9: u64, const P10: u64, const P11: u64, const P12: u64, const P13: u64, const P14: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut xmm0: u128 = 0; let mut xmm1: u128 = 0; let mut xmm2: u128 = 0; let mut xmm3: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rbx = rcx; fl_l = rax; fl_r = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 2; continue; } rcx = P2; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 2; continue; } fl_l = (crate::simd_ops::ru32(rax.wrapping_add(P0))) as u32 as u64; fl_r = 0x3; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 2; continue; } r8 = (((crate::simd_ops::cvt_f64_i64(f64::from_bits(crate::simd_ops::ru64(rax))) as i64)) as u64) as u64; rax = (crate::simd_ops::ru64(rbx.wrapping_add(P3))) as u64; rcx = (crate::simd_ops::ru64(rax)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rcx = ((crate::simd_ops::ru16(P4)) as u32) as u32 as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(P5))) as u64; rdx = (crate::simd_ops::ru64(rax.wrapping_add(rcx.wrapping_mul(8)))) as u64; rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P1))) as u64; fl_l = ((r8 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rcx.wrapping_add(P6))) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 1; continue; } rax = (crate::simd_ops::ru64(rcx.wrapping_add(P7))) as u64; rdx = ((r8 as u32)) as u32 as u64; rdx = ((rdx).wrapping_shl(5u32)) as u64; fl_l = (crate::simd_ops::ru32(rdx.wrapping_add(rax))) as u32 as u64; fl_r = 0xffffffff; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 1; continue; } rdx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P8, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = 0; fl_l = 0; fl_r = 0; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            1 => { rdx = P9; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P10, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::lift_ops::trap(0x80000003, P13, rcx, rsp); return rax; }
            2 => { rdx = 0x1; rcx = rbx; r8 = ((rdx.wrapping_add(P11)) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P12, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::lift_ops::trap(0x80000003, P14, rcx, rsp); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_41316<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = rdx; rsi = rcx; rbx = 0; fl_l = 0; fl_r = 0; bb = 1; continue; }
            1 => { rcx = (crate::simd_ops::ru64(rsi.wrapping_add(rbx.wrapping_mul(8)))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); bb = 2; continue; }
            2 => { rbx = ((rbx).wrapping_add(1)) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); bb = 3; continue; }
            3 => { return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_41369<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rdx = (crate::simd_ops::ru64(rcx.wrapping_add(0x10u64))) as u64; rbx = rcx; fl_l = rdx; fl_r = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 5; continue; } rdi = P0; fl_l = rdx; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 5; continue; } fl_l = (crate::simd_ops::ru32(rdx.wrapping_add(0x10u64))) as u32 as u64; fl_r = 0x5; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 5; continue; } rdx = (crate::simd_ops::ru64(rcx.wrapping_add(0x10u64))) as u64; r8 = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; rcx = rdi; rax = rdx.wrapping_add(0x18); fl_l = rax; fl_r = r8; rcx = (if (((fl_l as u64)) < ((fl_r as u64))) { rax } else { rcx }) as u64; rax = 0; fl_l = 0; fl_r = 0; fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(0x10u64))) as u32 as u64; fl_r = 0x4; rax = (if (((fl_l as u32)) == ((fl_r as u32))) { rcx } else { rax }) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } rcx = rdx.wrapping_add(0x30); fl_l = rcx; fl_r = r8; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 1; continue; } fl_l = rcx; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 1; continue; } rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x10u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 1; continue; } fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x3; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 7; continue; } bb = 1; continue; }
            1 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; rcx = ((rcx).wrapping_add(0x48u64)) as u64; fl_l = rcx; fl_r = rax; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 2; continue; } fl_l = rcx; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 2; continue; } fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(0x10u64))) as u32 as u64; fl_r = 0x0; if ((((fl_l as u32)) as i32) <= (((fl_r as u32)) as i32)) { bb = 2; continue; } fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(0x10u64))) as u32 as u64; fl_r = 0x3; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 2; continue; } rdx = rsp.wrapping_add(0x20); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; bb = 2; continue; }
            2 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; rcx = ((rcx).wrapping_add(0x60u64)) as u64; fl_l = rcx; fl_r = rax; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 3; continue; } fl_l = rcx; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 3; continue; } fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(0x10u64))) as u32 as u64; fl_r = 0x0; if ((((fl_l as u32)) as i32) <= (((fl_r as u32)) as i32)) { bb = 3; continue; } r8 = P2; rdx = 0x5; rcx = rbx; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 3; continue; }
            3 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rax = 0; fl_l = 0; fl_r = 0; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            4 => { rdx = 0x4; rcx = rbx; r8 = ((rdx.wrapping_sub(0x1u64)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, P5, rcx, rsp); return rax; }
            5 => { rdx = 0x1; r8 = ((rdx.wrapping_add(0x4u64)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, P6, rcx, rsp); return rax; }
            6 => { rdx = 0x2; rcx = rbx; r8 = ((rdx.wrapping_add(0x2u64)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, P7, rcx, rsp); return rax; }
            7 => { rdx = 0x3; rcx = rbx; r8 = ((rdx as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, P8, rcx, rsp); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_41370<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; r8 = (crate::simd_ops::ru64(rcx.wrapping_add(0x10u64))) as u64; rdi = P0; r10 = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; rdx = rdi; fl_l = r8; fl_r = r10; rbx = rcx; rdx = (if (((fl_l as u64)) < ((fl_r as u64))) { r8 } else { rdx }) as u64; r9 = 0; fl_l = 0; fl_r = 0; rax = ((r9 as u32)) as u32 as u64; fl_l = (crate::simd_ops::ru32(rdx.wrapping_add(0x10u64))) as u32 as u64; fl_r = 0x4; rax = (if (((fl_l as u32)) == ((fl_r as u32))) { rdx } else { rax }) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } rax = r8.wrapping_add(0x18); rcx = rdi; fl_l = rax; fl_r = r10; rcx = (if (((fl_l as u64)) < ((fl_r as u64))) { rax } else { rcx }) as u64; rax = ((r9 as u32)) as u32 as u64; fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(0x10u64))) as u32 as u64; fl_r = 0x4; rax = (if (((fl_l as u32)) == ((fl_r as u32))) { rcx } else { rax }) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } rax = r8.wrapping_add(0x30); rcx = rdi; fl_l = rax; fl_r = r10; rcx = (if (((fl_l as u64)) < ((fl_r as u64))) { rax } else { rcx }) as u64; fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(0x10u64))) as u32 as u64; fl_r = 0x4; r9 = (if (((fl_l as u32)) == ((fl_r as u32))) { rcx } else { r9 }) as u64; fl_l = r9; fl_r = r9; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } rcx = r8.wrapping_add(0x48); fl_l = rcx; fl_r = r10; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 1; continue; } fl_l = rcx; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 1; continue; } rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x10u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 1; continue; } fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x3; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 6; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; rax = ((rax).wrapping_add(0x60u64)) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 2; continue; } fl_l = rax; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 2; continue; } fl_l = (crate::simd_ops::ru32(rax.wrapping_add(0x10u64))) as u32 as u64; fl_r = 0x0; if ((((fl_l as u32)) as i32) <= (((fl_r as u32)) as i32)) { bb = 2; continue; } r8 = P1; rdx = 0x5; rcx = rbx; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 2; continue; }
            2 => { rcx = rbx; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rax = 0; fl_l = 0; fl_r = 0; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            3 => { rdx = 0x1; r8 = ((rdx.wrapping_add(0x3u64)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, P5, rcx, rsp); return rax; }
            4 => { rdx = 0x2; rcx = rbx; r8 = ((rdx.wrapping_add(0x2u64)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, P6, rcx, rsp); return rax; }
            5 => { rdx = 0x3; rcx = rbx; r8 = ((rdx.wrapping_add(0x1u64)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, P7, rcx, rsp); return rax; }
            6 => { rdx = 0x4; rcx = rbx; r8 = ((rdx.wrapping_sub(0x1u64)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, P8, rcx, rsp); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_41375<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; r8 = (crate::simd_ops::ru64(rcx.wrapping_add(0x10u64))) as u64; rdi = P0; r10 = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; rdx = rdi; fl_l = r8; fl_r = r10; rbx = rcx; rdx = (if (((fl_l as u64)) < ((fl_r as u64))) { r8 } else { rdx }) as u64; r9 = 0; fl_l = 0; fl_r = 0; rax = ((r9 as u32)) as u32 as u64; fl_l = (crate::simd_ops::ru32(rdx.wrapping_add(0x10u64))) as u32 as u64; fl_r = 0x4; rax = (if (((fl_l as u32)) == ((fl_r as u32))) { rdx } else { rax }) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } rax = r8.wrapping_add(0x18); rcx = rdi; fl_l = rax; fl_r = r10; rcx = (if (((fl_l as u64)) < ((fl_r as u64))) { rax } else { rcx }) as u64; fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(0x10u64))) as u32 as u64; fl_r = 0x4; r9 = (if (((fl_l as u32)) == ((fl_r as u32))) { rcx } else { r9 }) as u64; fl_l = r9; fl_r = r9; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } rcx = r8.wrapping_add(0x30); fl_l = rcx; fl_r = r10; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 1; continue; } fl_l = rcx; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 1; continue; } rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x10u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 1; continue; } fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x3; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 5; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; rax = ((rax).wrapping_add(0x48u64)) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 2; continue; } fl_l = rax; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 2; continue; } fl_l = (crate::simd_ops::ru32(rax.wrapping_add(0x10u64))) as u32 as u64; fl_r = 0x0; if ((((fl_l as u32)) as i32) <= (((fl_r as u32)) as i32)) { bb = 2; continue; } r8 = P1; rdx = 0x4; rcx = rbx; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 2; continue; }
            2 => { rcx = rbx; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rax = 0; fl_l = 0; fl_r = 0; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            3 => { rdx = 0x1; r8 = ((rdx.wrapping_add(0x3u64)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, P5, rcx, rsp); return rax; }
            4 => { rdx = 0x2; rcx = rbx; r8 = ((rdx.wrapping_add(0x2u64)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, P6, rcx, rsp); return rax; }
            5 => { rdx = 0x3; rcx = rbx; r8 = ((rdx as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, P7, rcx, rsp); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_41376<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64, const P9: u64, const P10: u64, const P11: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; r8 = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; r11 = P1; r9 = (crate::simd_ops::ru64(rcx.wrapping_add(P2))) as u64; rdx = r11; fl_l = r8; fl_r = r9; rbx = rcx; rdx = (if (((fl_l as u64)) < ((fl_r as u64))) { r8 } else { rdx }) as u64; r10 = 0; fl_l = 0; fl_r = 0; rax = ((r10 as u32)) as u32 as u64; fl_l = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64; fl_r = 0x4; rax = (if (((fl_l as u32)) == ((fl_r as u32))) { rdx } else { rax }) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } rax = ((r8.wrapping_add(P3)) as u64) as u64; rcx = r11; fl_l = rax; fl_r = r9; rcx = (if (((fl_l as u64)) < ((fl_r as u64))) { rax } else { rcx }) as u64; fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; fl_r = 0x4; r10 = (if (((fl_l as u32)) == ((fl_r as u32))) { rcx } else { r10 }) as u64; fl_l = r10; fl_r = r10; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } rcx = ((r8.wrapping_add(P4)) as u64) as u64; fl_l = rcx; fl_r = r9; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 1; continue; } fl_l = rcx; fl_r = r11; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 1; continue; } rax = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 1; continue; } fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x3; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 4; continue; } bb = 1; continue; }
            1 => { rcx = rbx; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rax = 0; fl_l = 0; fl_r = 0; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            2 => { rdx = 0x1; r8 = ((rdx.wrapping_add(P6)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, P9, rcx, rsp); return rax; }
            3 => { rdx = 0x2; rcx = rbx; r8 = ((rdx.wrapping_add(P8)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, P10, rcx, rsp); return rax; }
            4 => { rdx = 0x3; rcx = rbx; r8 = ((rdx as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, P11, rcx, rsp); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_41396<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64, const P9: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x10u64))) as u64; rdx = P0; r15 = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; rdi = rcx; fl_l = rax; fl_r = r15; rcx = rdx; rcx = (if (((fl_l as u64)) < ((fl_r as u64))) { rax } else { rcx }) as u64; rsi = 0; fl_l = 0; fl_r = 0; fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(0x10u64))) as u32 as u64; fl_r = 0x2; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 1; continue; } rcx = ((rsi as u32)) as u32 as u64; bb = 2; continue; }
            1 => { rcx = (crate::simd_ops::ru64(rcx)) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { fl_l = rax; fl_r = r15; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 8; continue; } fl_l = rax; fl_r = rdx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 8; continue; } fl_l = (crate::simd_ops::ru32(rax.wrapping_add(0x10u64))) as u32 as u64; fl_r = 0x2; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 8; continue; } bb = 3; continue; }
            3 => { crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), rbp); rbp = (crate::simd_ops::ru64(rdi.wrapping_add(0x78u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), r14); crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rcx); rbx = (crate::simd_ops::ru64(rbp.wrapping_add(0x8u64))) as u64; rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x38u64))) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(rbx.wrapping_add(0x30u64))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 4; continue; } rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x40u64))) as u64; bb = 5; continue; }
            4 => { r8 = rsp.wrapping_add(0x50); rcx = rbx; rdx = rsp.wrapping_add(0x20); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; fl_l = rcx; fl_r = (0xffffffffffffffffu64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x40u64))) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rax = ((rax).wrapping_add(crate::simd_ops::ru64(rbx.wrapping_add(0x58u64)))) as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rbx.wrapping_add(0x58u64)))) as u64; fl_l = rcx; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 6; continue; } rdx = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0xffffffff; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 6; continue; } rax = (crate::simd_ops::ru64(rbp)) as u64; rcx = ((rdx as u32)) as u32 as u64; r9 = (crate::simd_ops::ru64(rax)) as u64; rax = (crate::simd_ops::ru64(r9.wrapping_add(0x30u64))) as u64; rdx = (crate::simd_ops::ru64(rax.wrapping_add(rcx.wrapping_mul(8)))) as u64; rax = 0xffff; fl_l = ((rdx as u16)) as u16 as u64; fl_r = ((rax as u16)) as u16 as u64; if (((fl_l as u16)) == ((fl_r as u16))) { bb = 7; continue; } r8 = ((crate::simd_ops::ru16(P2)) as u32) as u32 as u64; rax = (((rdx as u16)) as u32) as u32 as u64; rax = ((rax).wrapping_shl(5u32)) as u64; rdx = (crate::simd_ops::ru32(rax.wrapping_add(r9).wrapping_add(0x180a8u64))) as u32 as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(r9).wrapping_add(0x18090u64))) as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0xffffffff; rsi = (rsi & !0xffu64) | (((if (((fl_l as u32)) != ((fl_r as u32))) { 1u8 } else { 0u8 }) as u8 as u64)); bb = 6; continue; }
            6 => { rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rax = 0x1; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; crate::simd_ops::wu32(r15, ((rsi as u32)) as u32); crate::simd_ops::wu32(r15.wrapping_add(0x10u64), (0x1u32) as u32); crate::simd_ops::lk_add_u64(rdi.wrapping_add(0x8u64), (0x18u64) as u64); rsp = ((rsp).wrapping_add(0x30u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            7 => { rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P4)) as u64, rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, P6, rcx, rsp); return rax; }
            8 => { rdx = 0x1; rcx = rdi; r8 = ((rdx.wrapping_add(0x1u64)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, P7, rcx, rsp); return rax; }
            9 => { crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), rbp); rbp = (crate::simd_ops::ru64(rdi.wrapping_add(0x78u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), r14); crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rcx); rbx = (crate::simd_ops::ru64(rbp.wrapping_add(0x8u64))) as u64; rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x38u64))) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(rbx.wrapping_add(0x30u64))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 4; continue; } rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x40u64))) as u64; bb = 5; continue; }
            10 => { r8 = rsp.wrapping_add(0x50); rcx = rbx; rdx = rsp.wrapping_add(0x20); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; fl_l = rcx; fl_r = (0xffffffffffffffffu64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 6; continue; } bb = 11; continue; }
            11 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x40u64))) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rax = ((rax).wrapping_add(crate::simd_ops::ru64(rbx.wrapping_add(0x58u64)))) as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rbx.wrapping_add(0x58u64)))) as u64; fl_l = rcx; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 6; continue; } rdx = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0xffffffff; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 6; continue; } rax = (crate::simd_ops::ru64(rbp)) as u64; rcx = ((rdx as u32)) as u32 as u64; r9 = (crate::simd_ops::ru64(rax)) as u64; rax = (crate::simd_ops::ru64(r9.wrapping_add(0x30u64))) as u64; rdx = (crate::simd_ops::ru64(rax.wrapping_add(rcx.wrapping_mul(8)))) as u64; rax = 0xffff; fl_l = ((rdx as u16)) as u16 as u64; fl_r = ((rax as u16)) as u16 as u64; if (((fl_l as u16)) == ((fl_r as u16))) { bb = 7; continue; } r8 = ((crate::simd_ops::ru16(P2)) as u32) as u32 as u64; rax = (((rdx as u16)) as u32) as u32 as u64; rax = ((rax).wrapping_shl(5u32)) as u64; rdx = (crate::simd_ops::ru32(rax.wrapping_add(r9).wrapping_add(0x180a8u64))) as u32 as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(r9).wrapping_add(0x18090u64))) as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0xffffffff; rsi = (rsi & !0xffu64) | (((if (((fl_l as u32)) != ((fl_r as u32))) { 1u8 } else { 0u8 }) as u8 as u64)); bb = 12; continue; }
            12 => { rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rax = 0x1; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; crate::simd_ops::wu32(r15, ((rsi as u32)) as u32); crate::simd_ops::wu32(r15.wrapping_add(0x10u64), (0x1u32) as u32); crate::simd_ops::lk_add_u64(rdi.wrapping_add(0x8u64), (0x18u64) as u64); rsp = ((rsp).wrapping_add(0x30u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            13 => { rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P4)) as u64, rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, P8, rcx, rsp); return rax; }
            14 => { rdx = 0x1; rcx = rdi; r8 = ((rdx.wrapping_add(0x1u64)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, P9, rcx, rsp); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_41414<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rdi = rcx; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0x10u64))) as u64; r14 = P1; r8 = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; rsi = r14; rbp = rax; rcx = rdx.wrapping_add(0x18); fl_l = rcx; fl_r = r8; rsi = (if (((fl_l as u64)) < ((fl_r as u64))) { rcx } else { rsi }) as u64; rbx = 0; fl_l = 0; fl_r = 0; fl_l = (crate::simd_ops::ru32(rsi.wrapping_add(0x10u64))) as u32 as u64; fl_r = 0x2; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 1; continue; } rsi = ((rbx as u32)) as u32 as u64; bb = 2; continue; }
            1 => { rsi = (crate::simd_ops::ru64(rsi)) as u64; fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { fl_l = rcx; fl_r = r8; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 12; continue; } fl_l = rcx; fl_r = r14; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 12; continue; } fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(0x10u64))) as u32 as u64; fl_r = 0x2; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 12; continue; } bb = 3; continue; }
            3 => { rdx = ((rdx).wrapping_add(0x30u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), rsi); fl_l = rdx; fl_r = r8; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 13; continue; } fl_l = rdx; fl_r = r14; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 13; continue; } fl_l = (crate::simd_ops::ru32(rdx.wrapping_add(0x10u64))) as u32 as u64; fl_r = 0x5; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 13; continue; } rbx = (crate::simd_ops::ru64(rdx)) as u64; rcx = (0xcbf29ce484222325u64) as u64; fl_l = ((rbx).wrapping_add(0x18u64)) as u64; fl_r = fl_l; rbx = ((fl_l as u64)) as u64; rdx = (0xffffffffffffffffu64) as u64; bb = 4; continue; }
            4 => { rdx = ((rdx).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(rdx))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 4; continue; } rdx = ((rdx).wrapping_add(rbx)) as u64; fl_l = rbx; fl_r = rdx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 6; continue; } r8 = (0x100000001b3u64) as u64; bb = 5; continue; }
            5 => { rax = (((crate::simd_ops::ru8(rbx)) as i8 as i64) as u64) as u64; rbx = ((rbx).wrapping_add(1)) as u64; rcx = (((rcx) ^ (rax))) as u64; rcx = (((rcx) as i64).wrapping_mul((r8) as i64) as u64) as u64; fl_l = rbx; fl_r = rdx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 5; continue; } bb = 6; continue; }
            6 => { crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), rcx); r8 = rsp.wrapping_add(0x60); rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x8u64))) as u64; rdx = rsp.wrapping_add(0x20); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x8u64))) as u64; r8 = (crate::simd_ops::ru32(rdx.wrapping_add(0x10u64))) as u32 as u64; rax = ((rcx.wrapping_add(r8.wrapping_mul(8))) as u64) as u64; fl_l = rcx; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 11; continue; } r9 = rcx; bb = 7; continue; }
            7 => { fl_l = (crate::simd_ops::ru64(rcx)) as u64; fl_r = rsi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 9; continue; } rax = ((r8 as u32)) as u32 as u64; rax = ((rax).wrapping_sub(1)) as u64; rax = ((r9.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; fl_l = rcx; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 8; continue; } rax = (crate::simd_ops::ru64(rax)) as u64; crate::simd_ops::wu64(rcx, rax); bb = 8; continue; }
            8 => { fl_l = (crate::simd_ops::lk_sub_u32(rdx.wrapping_add(0x10u64), (1) as u32)) as u32 as u64; r8 = (crate::simd_ops::ru32(rdx.wrapping_add(0x10u64))) as u32 as u64; bb = 10; continue; }
            9 => { fl_l = ((rcx).wrapping_add(0x8u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; bb = 10; continue; }
            10 => { r9 = (crate::simd_ops::ru64(rdx.wrapping_add(0x8u64))) as u64; rax = ((r8 as u32)) as u32 as u64; rax = ((r9.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; fl_l = rcx; fl_r = rax; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 7; continue; } bb = 11; continue; }
            11 => { rcx = rdx.wrapping_add(0x18); rdx = rsp.wrapping_add(0x68); rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rax = 0; fl_l = 0; fl_r = 0; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            12 => { rdx = 0x2; rcx = rdi; r8 = ((rdx as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, P5, rcx, rsp); return rax; }
            13 => { rdx = 0x3; rcx = rdi; r8 = ((rdx.wrapping_add(0x2u64)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, P6, rcx, rsp); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_41415<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rbx = rcx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; rbp = P1; r8 = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; rsi = rax; rax = rbp; rcx = rdx.wrapping_add(0x18); fl_l = rcx; fl_r = r8; rax = (if (((fl_l as u64)) < ((fl_r as u64))) { rcx } else { rax }) as u64; rdi = 0; fl_l = 0; fl_r = 0; fl_l = (crate::simd_ops::ru32(rax.wrapping_add(0x10u64))) as u32 as u64; fl_r = 0x2; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 1; continue; } rax = ((rdi as u32)) as u32 as u64; bb = 2; continue; }
            1 => { rax = (crate::simd_ops::ru64(rax)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { fl_l = rcx; fl_r = r8; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 10; continue; } fl_l = rcx; fl_r = rbp; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 10; continue; } fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(0x10u64))) as u32 as u64; fl_r = 0x2; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 10; continue; } bb = 3; continue; }
            3 => { rdx = ((rdx).wrapping_add(0x30u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rax); fl_l = rdx; fl_r = r8; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 9; continue; } fl_l = rdx; fl_r = rbp; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 9; continue; } fl_l = (crate::simd_ops::ru32(rdx.wrapping_add(0x10u64))) as u32 as u64; fl_r = 0x5; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 9; continue; } rdi = (crate::simd_ops::ru64(rdx)) as u64; rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; rdi = ((rdi).wrapping_add(0x18u64)) as u64; rax = ((rax).wrapping_add(0x48u64)) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 5; continue; } fl_l = rax; fl_r = rbp; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 5; continue; } fl_l = (crate::simd_ops::ru32(rax.wrapping_add(0x10u64))) as u32 as u64; fl_r = 0x0; if ((((fl_l as u32)) as i32) <= (((fl_r as u32)) as i32)) { bb = 5; continue; } fl_l = (crate::simd_ops::ru32(rax.wrapping_add(0x10u64))) as u32 as u64; fl_r = 0x3; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 4; continue; } rdx = rsp.wrapping_add(0x20); rcx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 4; continue; }
            4 => { xmm1 = ((crate::simd_ops::ru64(rax)) as u64) as u128; bb = 6; continue; }
            5 => { xmm1 = ((crate::simd_ops::ru64(P3)) as u64) as u128; bb = 6; continue; }
            6 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rax = (0xffffffffffffffffu64) as u64; bb = 7; continue; }
            7 => { rax = ((rax).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru8(rdi.wrapping_add(rax))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 7; continue; } rcx = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; r8 = rsp.wrapping_add(0x50); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax); rdx = rsp.wrapping_add(0x20); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; xmm3 = (xmm1 & !0xffff_ffffu128) | (((f64::from_bits(xmm1 as u64)) as f32).to_bits() as u128); crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rax = 0; fl_l = 0; fl_r = 0; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            8 => { rdx = 0x4; rcx = rbx; r8 = ((rdx.wrapping_sub(0x1u64)) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::lift_ops::trap(0x80000003, P6, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            9 => { rdx = 0x3; rcx = rbx; r8 = ((rdx.wrapping_add(0x2u64)) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::lift_ops::trap(0x80000003, P7, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            10 => { rdx = 0x2; rcx = rbx; r8 = ((rdx as u32)) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::lift_ops::trap(0x80000003, P8, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_41420<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64, const P9: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rbx = rcx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r8 = (crate::simd_ops::ru64(rbx.wrapping_add(P1))) as u64; rdi = rax; r9 = (crate::simd_ops::ru64(rbx.wrapping_add(P2))) as u64; rdx = r8; rdx = ((rdx).wrapping_sub(r9)) as u64; rax = (0x2aaaaaaaaaaaaaabu64) as u64; let prod = (rax as i64 as i128).wrapping_mul((rdx) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; rdx = ((((rdx) as i64).wrapping_shr(2u32) as u64)) as u64; rcx = rdx; rcx = ((rcx).wrapping_shr(63u32)) as u64; rdx = ((rdx).wrapping_add(rcx)) as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0x1; if ((((fl_l as u32)) as i32) <= (((fl_r as u32)) as i32)) { bb = 2; continue; } rax = ((r9.wrapping_add(P3)) as u64) as u64; fl_l = rax; fl_r = r8; rcx = P4; rcx = (if (((fl_l as u64)) < ((fl_r as u64))) { rax } else { rcx }) as u64; fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(P2))) as u32 as u64; fl_r = 0x3; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 1; continue; } rdx = rsp.wrapping_add(0x20); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { xmm0 = ((crate::simd_ops::ru64(rcx)) as u64) as u128; xmm0 = crate::simd_ops::vcvtpd2ps(xmm0); crate::simd_ops::wu32(rdi.wrapping_add(P6), ((xmm0 as u32)) as u32); rax = 0; fl_l = 0; fl_r = 0; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            2 => { xmm0 = ((crate::simd_ops::ru32(rdi.wrapping_add(P6))) as u32) as u128; xmm0 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | (((f32::from_bits(xmm0 as u32)) as f64).to_bits() as u128); crate::simd_ops::wu64(r8, ((xmm0 as u64)) as u64); crate::simd_ops::wu32(r8.wrapping_add(P2), (0x3u32) as u32); rax = 0x1; crate::simd_ops::lk_add_u64(rbx.wrapping_add(P1), (0x18u64) as u64); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            3 => { rdx = 0x2; rcx = rbx; r8 = ((rdx.wrapping_add(P7)) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P8, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::lift_ops::trap(0x80000003, P9, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_41421<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rbx = rcx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r8 = (crate::simd_ops::ru64(rbx.wrapping_add(P1))) as u64; rdi = rax; r9 = (crate::simd_ops::ru64(rbx.wrapping_add(P2))) as u64; rdx = r8; rdx = ((rdx).wrapping_sub(r9)) as u64; rax = (0x2aaaaaaaaaaaaaabu64) as u64; let prod = (rax as i64 as i128).wrapping_mul((rdx) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; rdx = ((((rdx) as i64).wrapping_shr(2u32) as u64)) as u64; rcx = rdx; rcx = ((rcx).wrapping_shr(63u32)) as u64; rdx = ((rdx).wrapping_add(rcx)) as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0x1; if ((((fl_l as u32)) as i32) <= (((fl_r as u32)) as i32)) { bb = 2; continue; } rax = ((r9.wrapping_add(P3)) as u64) as u64; fl_l = rax; fl_r = r8; rcx = P4; rcx = (if (((fl_l as u64)) < ((fl_r as u64))) { rax } else { rcx }) as u64; fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(P2))) as u32 as u64; fl_r = 0x3; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 1; continue; } rdx = rsp.wrapping_add(0x20); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { xmm0 = ((crate::simd_ops::ru64(rcx)) as u64) as u128; xmm0 = crate::simd_ops::vcvtpd2ps(xmm0); crate::simd_ops::wu32(rdi, ((xmm0 as u32)) as u32); rax = 0; fl_l = 0; fl_r = 0; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            2 => { xmm0 = ((crate::simd_ops::ru32(rdi)) as u32) as u128; xmm0 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | (((f32::from_bits(xmm0 as u32)) as f64).to_bits() as u128); crate::simd_ops::wu64(r8, ((xmm0 as u64)) as u64); crate::simd_ops::wu32(r8.wrapping_add(P2), (0x3u32) as u32); rax = 0x1; crate::simd_ops::lk_add_u64(rbx.wrapping_add(P1), (0x18u64) as u64); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            3 => { rdx = 0x2; rcx = rbx; r8 = ((rdx.wrapping_add(P6)) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::lift_ops::trap(0x80000003, P8, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_41461<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rdx; rdi = rcx; rcx = rbx; rdx = ((r8 as u32)) as u32 as u64; rsi = (((r9 as u16)) as u32) as u32 as u64; r11 = ((r8 as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); r10 = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 1; continue; } rdx = ((r11 as u32)) as u32 as u64; rcx = rbx; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x2; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 3; continue; } bb = 1; continue; }
            1 => { rax = 0x3fff; crate::simd_ops::wu64(rdi, r10); r10 = (r10 & !0xffffu64) | ((((((r10 as u16)) & ((rax as u16)))) as u16 as u64)); fl_l = ((r10 as u16)) as u16 as u64; fl_r = ((rsi as u16)) as u16 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { bb = 2; continue; } rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = rdi; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            2 => { r8 = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rdx = ((r11 as u32)) as u32 as u64; rcx = rbx; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, P4, rcx, rsp); return rax; }
            3 => { r8 = 0x2; rdx = ((r11 as u32)) as u32 as u64; rcx = rbx; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, P5, rcx, rsp); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_41462<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x28u64)) as u64; rdx = 0x1; r10 = rcx; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); r11 = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 1; continue; } rdx = ((rax.wrapping_add(P1)) as u32) as u32 as u64; rcx = r10; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x2; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rdx = rsp.wrapping_add(0x40); crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), r11); rcx = r10; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
            2 => { rdx = 0x1; rcx = r10; r8 = ((rdx.wrapping_add(P1)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, P5, rcx, rsp); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_41466<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x38u64)) as u64; rdx = 0x1; r10 = rcx; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); r11 = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 1; continue; } rdx = ((rax.wrapping_add(0x1u64)) as u32) as u32 as u64; rcx = r10; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x2; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 5; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rsi); rsi = (crate::simd_ops::ru64(r10.wrapping_add(0x78u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rdi); crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), r11); rbx = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x38u64))) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(rbx.wrapping_add(0x30u64))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x40u64))) as u64; bb = 3; continue; }
            2 => { r8 = rsp.wrapping_add(0x50); rcx = rbx; rdx = rsp.wrapping_add(0x20); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; fl_l = rdx; fl_r = (0xffffffffffffffffu64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x40u64))) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rax = ((rax).wrapping_add(crate::simd_ops::ru64(rbx.wrapping_add(0x58u64)))) as u64; rdx = ((rdx).wrapping_shl(4u32)) as u64; rdx = ((rdx).wrapping_add(crate::simd_ops::ru64(rbx.wrapping_add(0x58u64)))) as u64; fl_l = rdx; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 4; continue; } rdx = (crate::simd_ops::ru64(rdx.wrapping_add(0x8u64))) as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0xffffffff; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 4; continue; } rcx = (crate::simd_ops::ru64(rsi)) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x38u64)) as u64; return crate::simd_ops::tail_stk(P3, rax, rcx, rdx, r8, r9, rsp); }
            4 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = 0; fl_l = 0; fl_r = 0; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x38u64)) as u64; return rax; }
            5 => { rdx = 0x1; rcx = r10; r8 = ((rdx.wrapping_add(0x1u64)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, P5, rcx, rsp); return rax; }
            6 => { crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rsi); rsi = (crate::simd_ops::ru64(r10.wrapping_add(0x78u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rdi); crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), r11); rbx = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x38u64))) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(rbx.wrapping_add(0x30u64))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x40u64))) as u64; bb = 3; continue; }
            7 => { r8 = rsp.wrapping_add(0x50); rcx = rbx; rdx = rsp.wrapping_add(0x20); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; fl_l = rdx; fl_r = (0xffffffffffffffffu64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 4; continue; } bb = 8; continue; }
            8 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x40u64))) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rax = ((rax).wrapping_add(crate::simd_ops::ru64(rbx.wrapping_add(0x58u64)))) as u64; rdx = ((rdx).wrapping_shl(4u32)) as u64; rdx = ((rdx).wrapping_add(crate::simd_ops::ru64(rbx.wrapping_add(0x58u64)))) as u64; fl_l = rdx; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 4; continue; } rdx = (crate::simd_ops::ru64(rdx.wrapping_add(0x8u64))) as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0xffffffff; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 4; continue; } rcx = (crate::simd_ops::ru64(rsi)) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x38u64)) as u64; return crate::simd_ops::tail_stk(P3, rax, rcx, rdx, r8, r9, rsp); }
            9 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = 0; fl_l = 0; fl_r = 0; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x38u64)) as u64; return rax; }
            10 => { rdx = 0x1; rcx = r10; r8 = ((rdx.wrapping_add(0x1u64)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, P6, rcx, rsp); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_41467<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    crate::simd_ops::forward_incoming_to_outgoing();
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    r8 = P0;
    rdx = 0x1;
    crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8(P1, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_41470<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    crate::simd_ops::wu64(rax, rdx);
    crate::simd_ops::wu32(rax.wrapping_add(P1), (0x2u32) as u32);
    fl_l = (crate::simd_ops::lk_add_u64(rcx.wrapping_add(P0), (0x18u64) as u64)) as u64; fl_r = fl_l;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_41475<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    r8 = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    rax = ((rdx as u32)) as u32 as u64;
    xmm0 = 0u128;
    xmm0 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | ((((rax) as i64) as f64).to_bits() as u128);
    crate::simd_ops::wu64(r8, ((xmm0 as u64)) as u64);
    crate::simd_ops::wu32(r8.wrapping_add(P1), (0x3u32) as u32);
    fl_l = (crate::simd_ops::lk_add_u64(rcx.wrapping_add(P0), (0x18u64) as u64)) as u64; fl_r = fl_l;
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_41476<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    xmm0 = 0u128;
    xmm0 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | (((((rdx as u32)) as i32) as f64).to_bits() as u128);
    crate::simd_ops::wu64(rax, ((xmm0 as u64)) as u64);
    crate::simd_ops::wu32(rax.wrapping_add(P1), (0x3u32) as u32);
    fl_l = (crate::simd_ops::lk_add_u64(rcx.wrapping_add(P0), (0x18u64) as u64)) as u64; fl_r = fl_l;
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_41481<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (0u64, 0u64, 0u64);
    rcx = (crate::simd_ops::ru64(P0)) as u64;
    rdx = ((crate::simd_ops::ru8(rdx.wrapping_add(P1))) as u32) as u32 as u64;
    rax = (crate::simd_ops::ru32(rcx.wrapping_add(P2))) as u32 as u64;
    fl_l = rdx; fl_r = rax;
    if (((fl_l as u64)) >= ((fl_r as u64))) { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; return rax; }
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P3))) as u64;
    rdx = (((crate::simd_ops::ru32(rax.wrapping_add(rdx.wrapping_mul(4)))) as i32 as i64) as u64) as u64;
    fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64;
    if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; return rax; }
    rax = (crate::simd_ops::ru32(rcx.wrapping_add(P4))) as u32 as u64;
    fl_l = rdx; fl_r = rax;
    if (((fl_l as u64)) >= ((fl_r as u64))) { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; return rax; }
    rax = (((rdx) as i64).wrapping_mul((0xd0u64) as i64) as u64) as u64;
    fl_l = ((rax).wrapping_add(crate::simd_ops::ru64(rcx))) as u64; fl_r = fl_l;
    rax = ((fl_l as u64)) as u64;
    if (((fl_l as u64)) == 0) { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; return rax; }
    fl_l = (crate::simd_ops::ru8(rax.wrapping_add(P5))) as u8 as u64; fl_r = (0x0u8) as u8 as u64;
    if !((((fl_l as u8)) == ((fl_r as u8)))) { rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); return rax; }
    rax = (rax & !0xffu64) | (((0u8) as u8 as u64));
    fl_l = 0; fl_r = 0;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_41504<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    crate::simd_ops::forward_incoming_to_outgoing();
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rdx = 0x1;
    crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8(P0, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_41530<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; r8 = P0; rdx = 0x1; rbx = rcx; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru8(P2)) as u8 as u64; fl_r = (0x0u8) as u8 as u64; rdi = ((rax as u32)) as u32 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 1; continue; } rdx = (((rdi as u16)) as u32) as u32 as u64; rcx = P3; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 1; continue; } rdx = (((rdi as u16)) as u32) as u32 as u64; rcx = P3; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 1; continue; } rax = 0x1; bb = 2; continue; }
            1 => { rax = 0; fl_l = 0; fl_r = 0; bb = 2; continue; }
            2 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P6))) as u64; crate::simd_ops::wu32(rcx, ((rax as u32)) as u32); rax = 0x1; crate::simd_ops::wu32(rcx.wrapping_add(P7), (0x1u32) as u32); crate::simd_ops::lk_add_u64(rbx.wrapping_add(P6), (0x18u64) as u64); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_41531<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; r8 = P0; rdx = 0x1; rbx = rcx; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru8(P2)) as u8 as u64; fl_r = (0x0u8) as u8 as u64; rdi = ((rax as u32)) as u32 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 1; continue; } rdx = (((rdi as u16)) as u32) as u32 as u64; rcx = P3; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 1; continue; } rdx = (((rdi as u16)) as u32) as u32 as u64; rcx = P3; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 1; continue; } rax = 0x1; bb = 2; continue; }
            1 => { rax = 0; fl_l = 0; fl_r = 0; bb = 2; continue; }
            2 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P6))) as u64; crate::simd_ops::wu32(rcx, ((rax as u32)) as u32); rax = 0x1; crate::simd_ops::wu32(rcx.wrapping_add(P7), (0x1u32) as u32); crate::simd_ops::lk_add_u64(rbx.wrapping_add(P6), (0x18u64) as u64); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_41532<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; r8 = P0; rdx = 0x1; rbx = rcx; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru8(P2)) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 1; continue; } rdx = (((rax as u16)) as u32) as u32 as u64; rcx = P3; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 1; continue; } rax = 0x1; bb = 2; continue; }
            1 => { rax = 0; fl_l = 0; fl_r = 0; bb = 2; continue; }
            2 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P5))) as u64; crate::simd_ops::wu32(rcx, ((rax as u32)) as u32); rax = 0x1; crate::simd_ops::wu32(rcx.wrapping_add(P6), (0x1u32) as u32); crate::simd_ops::lk_add_u64(rbx.wrapping_add(P5), (0x18u64) as u64); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_41541<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    r8 = 0;
    fl_l = 0; fl_r = 0;
    rdx = 0;
    fl_l = 0; fl_r = 0;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rax = 0x1;
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_41549<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (0u64, 0u64, 0u64);
    rax = (crate::simd_ops::ru32(r8.wrapping_add(P0))) as u32 as u64;
    fl_l = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64;
    rax = (rax & !0xffu64) | (((if (((fl_l as u32)) < ((fl_r as u32))) { 1u8 } else { 0u8 }) as u8 as u64));
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_41573<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64, const P9: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rcx; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rcx = rbx; rdi = rax; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P2))) as u64; rcx = rax; rdx = ((rdx).wrapping_add(0x18u64)) as u64; fl_l = rdx; fl_r = (crate::simd_ops::ru64(rbx.wrapping_add(P3))) as u64; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 4; continue; } rax = P4; fl_l = rdx; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 4; continue; } fl_l = (crate::simd_ops::ru32(rdx.wrapping_add(P2))) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 4; continue; } rax = (crate::simd_ops::ru32(rdx.wrapping_add(P2))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 2; continue; } fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 1; continue; } fl_l = (crate::simd_ops::ru32(rdx)) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 3; continue; }
            2 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 3; continue; }
            3 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu8(rdi.wrapping_add(P5), ((rax as u8)) as u8); rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu8(rcx.wrapping_add(P6), (0x1u8) as u8); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            4 => { rdx = 0x2; rcx = rbx; r8 = ((rdx.wrapping_add(P7)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, P9, rcx, rsp); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_41581<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    rax = (crate::simd_ops::ru64(P0)) as u64;
    fl_l = (crate::simd_ops::ru8(rax.wrapping_add(P1))) as u8 as u64; fl_r = (0x0u8) as u8 as u64;
    if !((((fl_l as u8)) == ((fl_r as u8)))) { fl_l = ((rax).wrapping_add(0xf0u64)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; } else { rax = 0; fl_l = 0; fl_r = 0; }
    rcx = (crate::simd_ops::ru64(rax)) as u64;
    fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(P2))) as u8 as u64; fl_r = (0x0u8) as u8 as u64;
    if !((((fl_l as u8)) == ((fl_r as u8)))) { rcx = ((rcx).wrapping_add(0x10u64)) as u64; rdx = P3; r8 = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); }
    rax = 0;
    fl_l = 0; fl_r = 0;
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_41584<const P0: u64, const P1: u64, const P2: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (0u64, 0u64, 0u64);
    rdx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    rax = 0;
    fl_l = 0; fl_r = 0;
    fl_l = (crate::simd_ops::ru8(P1)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64;
    rax = (rax & !0xffu64) | (((if (((fl_l as u8)) != ((fl_r as u8))) { 1u8 } else { 0u8 }) as u8 as u64));
    crate::simd_ops::wu32(rdx, ((rax as u32)) as u32);
    rax = 0x1;
    crate::simd_ops::wu32(rdx.wrapping_add(P2), (0x1u32) as u32);
    fl_l = (crate::simd_ops::lk_add_u64(rcx.wrapping_add(P0), (0x18u64) as u64)) as u64; fl_r = fl_l;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_41599<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    crate::simd_ops::wu64(rax, (0x0u64) as u64);
    crate::simd_ops::wu32(rax.wrapping_add(P1), (0x3u32) as u32);
    rax = 0x1;
    fl_l = (crate::simd_ops::lk_add_u64(rcx.wrapping_add(P0), (0x18u64) as u64)) as u64; fl_r = fl_l;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_41636<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64, const P9: u64, const P10: u64, const P11: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x60u64)) as u64; rbx = rcx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdi = rax; fl_l = (crate::simd_ops::ru32(rax.wrapping_add(P1))) as u32 as u64; fl_r = 0x4; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 3; continue; } rax = (crate::simd_ops::ru64(rbx.wrapping_add(P2))) as u64; rcx = P3; rax = ((rax).wrapping_add(0x18u64)) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(rbx.wrapping_add(P1))) as u64; rcx = (if (((fl_l as u64)) < ((fl_r as u64))) { rax } else { rcx }) as u64; fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(P2))) as u32 as u64; fl_r = 0x3; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 1; continue; } rdx = rsp.wrapping_add(0x40); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { xmm2 = ((crate::simd_ops::ru64(rcx)) as u64) as u128; rax = (crate::simd_ops::ru32(rdi.wrapping_add(P1))) as u32 as u64; rdx = rsp.wrapping_add(0x30); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); rcx = rsp.wrapping_add(0x20); rax = (crate::simd_ops::ru64(rdi)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; xmm2 = crate::simd_ops::vcvtpd2ps(xmm2); crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm0) as u128); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru64(rbx.wrapping_add(P1))) as u64; xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; crate::simd_ops::wu128(rax, (xmm0) as u128); crate::simd_ops::wu32(rax.wrapping_add(P2), (0x4u32) as u32); rax = 0x1; crate::simd_ops::lk_add_u64(rbx.wrapping_add(P1), (0x18u64) as u64); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rsp = ((rsp).wrapping_add(0x60u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            2 => { rdx = 0x2; rcx = rbx; r8 = ((rdx.wrapping_add(P6)) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::lift_ops::trap(0x80000003, P10, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            3 => { rdx = P8; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P9, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::lift_ops::trap(0x80000003, P11, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_41643<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r10, mut fl_l, mut fl_r) = (0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x28u64)) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; r9 = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; r8 = (crate::simd_ops::ru64(rcx.wrapping_add(P2))) as u64; rdx = (crate::simd_ops::ru64(rax)) as u64; rax = (crate::simd_ops::ru64(rdx)) as u64; rdx = ((crate::simd_ops::ru16(P3)) as u32) as u32 as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(P4))) as u64; r10 = (crate::simd_ops::ru64(rax.wrapping_add(rdx.wrapping_mul(8)))) as u64; rdx = r9; rdx = ((rdx).wrapping_sub(r8)) as u64; rax = (0x2aaaaaaaaaaaaaabu64) as u64; let prod = (rax as i64 as i128).wrapping_mul((rdx) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; rdx = ((((rdx) as i64).wrapping_shr(2u32) as u64)) as u64; rax = rdx; rax = ((rax).wrapping_shr(63u32)) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 3; continue; } fl_l = r8; fl_r = r9; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 4; continue; } rax = P5; fl_l = r8; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 4; continue; } fl_l = (crate::simd_ops::ru32(r8.wrapping_add(P2))) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 4; continue; } rax = (crate::simd_ops::ru32(r8.wrapping_add(P2))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 2; continue; } fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 1; continue; } fl_l = (crate::simd_ops::ru32(r8)) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); crate::simd_ops::wu8(r10.wrapping_add(P6), ((rax as u8)) as u8); rax = 0; fl_l = 0; fl_r = 0; rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
            2 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; crate::simd_ops::wu8(r10.wrapping_add(P6), ((rax as u8)) as u8); rax = 0; fl_l = 0; fl_r = 0; rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
            3 => { rax = 0; fl_l = 0; fl_r = 0; fl_l = (crate::simd_ops::ru8(r10.wrapping_add(P6))) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; crate::simd_ops::wu32(r9.wrapping_add(P2), (0x1u32) as u32); rax = (rax & !0xffu64) | (((if (((fl_l as u8)) != ((fl_r as u8))) { 1u8 } else { 0u8 }) as u8 as u64)); crate::simd_ops::wu32(r9, ((rax as u32)) as u32); rax = 0x1; crate::simd_ops::lk_add_u64(rcx.wrapping_add(P1), (0x18u64) as u64); rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
            4 => { rdx = 0x1; r8 = ((rdx as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, P8, rcx, rsp); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_41644<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x60u64)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rbx = rdx; r9 = (crate::simd_ops::ru64(rax)) as u64; rax = (0x2aaaaaaaaaaaaaabu64) as u64; r8 = (crate::simd_ops::ru64(r9.wrapping_add(0x8u64))) as u64; r10 = (crate::simd_ops::ru64(r9.wrapping_add(0x10u64))) as u64; rcx = r8; rcx = ((rcx).wrapping_sub(r10)) as u64; let prod = (rax as i64 as i128).wrapping_mul((rcx) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; rdx = ((((rdx) as i64).wrapping_shr(2u32) as u64)) as u64; rax = rdx; rax = ((rax).wrapping_shr(63u32)) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0x2; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 3; continue; } rcx = r10.wrapping_add(0x18); fl_l = rcx; fl_r = r8; rdx = P0; rax = rdx; rax = (if (((fl_l as u64)) < ((fl_r as u64))) { rcx } else { rax }) as u64; fl_l = (crate::simd_ops::ru32(rax.wrapping_add(0x10u64))) as u32 as u64; fl_r = 0x2; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 1; continue; } rax = 0; fl_l = 0; fl_r = 0; bb = 2; continue; }
            1 => { rax = (crate::simd_ops::ru64(rax)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 5; continue; } bb = 2; continue; }
            2 => { fl_l = rcx; fl_r = r8; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 6; continue; } fl_l = rcx; fl_r = rdx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 6; continue; } fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(0x10u64))) as u32 as u64; fl_r = 0x2; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 6; continue; } bb = 5; continue; }
            3 => { rax = (crate::simd_ops::ru64(r9.wrapping_add(0x78u64))) as u64; rdx = ((crate::simd_ops::ru16(P1)) as u32) as u32 as u64; rcx = (crate::simd_ops::ru64(rax)) as u64; r9 = (crate::simd_ops::ru64(rcx)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r9); rax = (crate::simd_ops::ru64(r9)) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(0x20u64))) as u64; rcx = ((crate::simd_ops::ru16(rcx.wrapping_add(rdx.wrapping_mul(2)))) as u32) as u32 as u64; rdx = rsp.wrapping_add(0x38); rcx = ((rcx).wrapping_shl(4u32)) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(r9.wrapping_add(0x8u64)))) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x8u64))) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax); rax = (crate::simd_ops::ru64(rcx)) as u64; rcx = rsp.wrapping_add(0x20); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru8(rsp.wrapping_add(0x50u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 4; continue; } rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(rcx.wrapping_mul(8)))) as u64; rax = rbx; crate::simd_ops::wu64(rbx, rcx); rsp = ((rsp).wrapping_add(0x60u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            4 => { rax = 0; fl_l = 0; fl_r = 0; bb = 5; continue; }
            5 => { crate::simd_ops::wu64(rbx, rax); rax = rbx; rsp = ((rsp).wrapping_add(0x60u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            6 => { rdx = 0x2; rcx = r9; r8 = ((rdx as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, P4, rcx, rsp); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_41680<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r10, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 0>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; r10 = rcx; r8 = ((crate::simd_ops::ru16(P1)) as u32) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (0xffffffffffffffffu64) as u64); rax = (crate::simd_ops::ru64(rax)) as u64; rax = (crate::simd_ops::ru64(rax)) as u64; rdx = (crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64; r8 = (crate::simd_ops::ru64(rdx.wrapping_add(r8.wrapping_mul(8)))) as u64; r9 = (crate::simd_ops::ru64(r8)) as u64; fl_l = ((r9 as u32)) as u32 as u64; fl_r = 0xffffffff; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 1; continue; } rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x8u64))) as u64; rdx = r9; rdx = ((rdx).wrapping_shr(32u32)) as u64; rcx = ((rcx).wrapping_shr(32u32)) as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rcx = (crate::simd_ops::ru32(rax.wrapping_add(P3))) as u32 as u64; r8 = ((r9 as u32)) as u32 as u64; fl_l = r8; fl_r = rcx; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 2; continue; } rcx = (crate::simd_ops::ru64(rax.wrapping_add(P4))) as u64; rdx = (crate::simd_ops::ru64(rcx.wrapping_add(r8.wrapping_mul(8)))) as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 2; continue; } rax = r9; rdx = ((rdx).wrapping_shr(32u32)) as u64; rax = ((rax).wrapping_shr(32u32)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), r9); fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 3; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (0xffffffffffffffffu64) as u64); bb = 3; continue; }
            3 => { rdx = rsp.wrapping_add(0x8); r8 = 0x2; rdx = (crate::simd_ops::ru64(rdx)) as u64; rcx = r10; return crate::simd_ops::tail_stk(P5, rax, rcx, rdx, r8, r9, rsp); }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_41692<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    crate::simd_ops::forward_incoming_to_outgoing();
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    r8 = P0;
    crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8(P1, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_41694<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64, const P9: u64, const P10: u64, const P11: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x38u64)) as u64; rdx = 0x1; r10 = rcx; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); r11 = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 1; continue; } rdx = ((rax.wrapping_add(0x1u64)) as u32) as u32 as u64; rcx = r10; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x2; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 6; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rsi); rsi = (crate::simd_ops::ru64(r10.wrapping_add(0x78u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rdi); crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), r11); rbx = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x38u64))) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(rbx.wrapping_add(0x30u64))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } r11 = (crate::simd_ops::ru64(rbx.wrapping_add(0x40u64))) as u64; bb = 3; continue; }
            2 => { r8 = rsp.wrapping_add(0x50); rcx = rbx; rdx = rsp.wrapping_add(0x20); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); r11 = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; fl_l = r11; fl_r = (0xffffffffffffffffu64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 5; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x40u64))) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rax = ((rax).wrapping_add(crate::simd_ops::ru64(rbx.wrapping_add(0x58u64)))) as u64; r11 = ((r11).wrapping_shl(4u32)) as u64; r11 = ((r11).wrapping_add(crate::simd_ops::ru64(rbx.wrapping_add(0x58u64)))) as u64; fl_l = r11; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 5; continue; } r11 = (crate::simd_ops::ru64(r11.wrapping_add(0x8u64))) as u64; fl_l = ((r11 as u32)) as u32 as u64; fl_r = 0xffffffff; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 5; continue; } rax = (crate::simd_ops::ru64(rsi)) as u64; rcx = ((r11 as u32)) as u32 as u64; rbx = (crate::simd_ops::ru64(rax)) as u64; rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x30u64))) as u64; rdx = (crate::simd_ops::ru64(rax.wrapping_add(rcx.wrapping_mul(8)))) as u64; rax = 0xffff; fl_l = ((rdx as u16)) as u16 as u64; fl_r = ((rax as u16)) as u16 as u64; if (((fl_l as u16)) == ((fl_r as u16))) { bb = 4; continue; } r8 = ((crate::simd_ops::ru16(P3)) as u32) as u32 as u64; rax = (((rdx as u16)) as u32) as u32 as u64; rax = ((rax).wrapping_shl(5u32)) as u64; rdx = (crate::simd_ops::ru32(rax.wrapping_add(rbx).wrapping_add(0x180a8u64))) as u32 as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(rbx).wrapping_add(0x18090u64))) as u64; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0xffffffff; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 5; continue; } rdx = r11; rcx = rbx; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x38u64)) as u64; return crate::simd_ops::tail_stk(P5, rax, rcx, rdx, r8, r9, rsp); }
            4 => { rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P6)) as u64, rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, P8, rcx, rsp); return rax; }
            5 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rax = 0; fl_l = 0; fl_r = 0; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x38u64)) as u64; return rax; }
            6 => { rdx = 0x1; rcx = r10; r8 = ((rdx.wrapping_add(0x1u64)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, P9, rcx, rsp); return rax; }
            7 => { crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rsi); rsi = (crate::simd_ops::ru64(r10.wrapping_add(0x78u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rdi); crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), r11); rbx = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x38u64))) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(rbx.wrapping_add(0x30u64))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } r11 = (crate::simd_ops::ru64(rbx.wrapping_add(0x40u64))) as u64; bb = 3; continue; }
            8 => { r8 = rsp.wrapping_add(0x50); rcx = rbx; rdx = rsp.wrapping_add(0x20); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); r11 = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; fl_l = r11; fl_r = (0xffffffffffffffffu64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 5; continue; } bb = 9; continue; }
            9 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x40u64))) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rax = ((rax).wrapping_add(crate::simd_ops::ru64(rbx.wrapping_add(0x58u64)))) as u64; r11 = ((r11).wrapping_shl(4u32)) as u64; r11 = ((r11).wrapping_add(crate::simd_ops::ru64(rbx.wrapping_add(0x58u64)))) as u64; fl_l = r11; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 5; continue; } r11 = (crate::simd_ops::ru64(r11.wrapping_add(0x8u64))) as u64; fl_l = ((r11 as u32)) as u32 as u64; fl_r = 0xffffffff; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 5; continue; } rax = (crate::simd_ops::ru64(rsi)) as u64; rcx = ((r11 as u32)) as u32 as u64; rbx = (crate::simd_ops::ru64(rax)) as u64; rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x30u64))) as u64; rdx = (crate::simd_ops::ru64(rax.wrapping_add(rcx.wrapping_mul(8)))) as u64; rax = 0xffff; fl_l = ((rdx as u16)) as u16 as u64; fl_r = ((rax as u16)) as u16 as u64; if (((fl_l as u16)) == ((fl_r as u16))) { bb = 4; continue; } r8 = ((crate::simd_ops::ru16(P3)) as u32) as u32 as u64; rax = (((rdx as u16)) as u32) as u32 as u64; rax = ((rax).wrapping_shl(5u32)) as u64; rdx = (crate::simd_ops::ru32(rax.wrapping_add(rbx).wrapping_add(0x180a8u64))) as u32 as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(rbx).wrapping_add(0x18090u64))) as u64; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0xffffffff; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 5; continue; } rdx = r11; rcx = rbx; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x38u64)) as u64; return crate::simd_ops::tail_stk(P5, rax, rcx, rdx, r8, r9, rsp); }
            10 => { rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P6)) as u64, rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, P10, rcx, rsp); return rax; }
            11 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rax = 0; fl_l = 0; fl_r = 0; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x38u64)) as u64; return rax; }
            12 => { rdx = 0x1; rcx = r10; r8 = ((rdx.wrapping_add(0x1u64)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, P11, rcx, rsp); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_41703<const P0: u64, const P1: u64, const P2: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    crate::simd_ops::forward_incoming_to_outgoing();
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rdx = P0;
    rcx = P1;
    crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8(P2, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_41711<const P0: u64, const P1: u64, const P2: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rcx;
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    rax = (crate::simd_ops::ru64(rbx.wrapping_add(P1))) as u64;
    crate::simd_ops::wu64(rax, ((xmm0 as u64)) as u64);
    crate::simd_ops::wu32(rax.wrapping_add(P2), (0x3u32) as u32);
    rax = 0x1;
    crate::simd_ops::lk_add_u64(rbx.wrapping_add(P1), (0x18u64) as u64);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_41770<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64, const P9: u64, const P10: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x38u64)) as u64; rdx = 0x1; r10 = rcx; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); r11 = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 1; continue; } rdx = ((rax.wrapping_add(0x1u64)) as u32) as u32 as u64; rcx = r10; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x2; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 6; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rsi); rsi = (crate::simd_ops::ru64(r10.wrapping_add(0x78u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rdi); crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), r11); rbx = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x38u64))) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(rbx.wrapping_add(0x30u64))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x40u64))) as u64; bb = 3; continue; }
            2 => { r8 = rsp.wrapping_add(0x50); rcx = rbx; rdx = rsp.wrapping_add(0x20); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; fl_l = rcx; fl_r = (0xffffffffffffffffu64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 5; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x40u64))) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rax = ((rax).wrapping_add(crate::simd_ops::ru64(rbx.wrapping_add(0x58u64)))) as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rbx.wrapping_add(0x58u64)))) as u64; fl_l = rcx; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 5; continue; } rdx = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0xffffffff; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 5; continue; } rax = (crate::simd_ops::ru64(rsi)) as u64; rcx = ((rdx as u32)) as u32 as u64; rsi = (crate::simd_ops::ru64(rax)) as u64; rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x30u64))) as u64; rbx = (crate::simd_ops::ru64(rax.wrapping_add(rcx.wrapping_mul(8)))) as u64; rax = 0xffff; fl_l = ((rbx as u16)) as u16 as u64; fl_r = ((rax as u16)) as u16 as u64; if (((fl_l as u16)) == ((fl_r as u16))) { bb = 4; continue; } r8 = ((crate::simd_ops::ru16(P3)) as u32) as u32 as u64; rdi = (((rbx as u16)) as u32) as u32 as u64; rdi = ((rdi).wrapping_shl(5u32)) as u64; rdx = (crate::simd_ops::ru32(rdi.wrapping_add(rsi).wrapping_add(0x180a8u64))) as u32 as u64; rcx = (crate::simd_ops::ru64(rdi.wrapping_add(rsi).wrapping_add(0x18090u64))) as u64; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0xffffffff; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 5; continue; } rdx = (crate::simd_ops::ru32(rdi.wrapping_add(rsi).wrapping_add(0x180acu64))) as u32 as u64; rcx = (crate::simd_ops::ru64(rdi.wrapping_add(rsi).wrapping_add(0x18098u64))) as u64; r11 = (((rbx as u16)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rcx = ((((rax as u32)) as i32 as i64) as u64) as u64; rax = r11.wrapping_add(0xc03); rax = ((rax).wrapping_shl(5u32)) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(rsi).wrapping_add(0x40u64))) as u64; rdx = (crate::simd_ops::ru32(rax.wrapping_add(rcx.wrapping_mul(4)))) as u32 as u64; rdx = ((rdx).wrapping_add(crate::simd_ops::ru64(rsi.wrapping_add(r11.wrapping_mul(8)).wrapping_add(0x88u64)))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rbx = ((rbx).wrapping_shr(32u32)) as u64; rax = ((rbx.wrapping_add(rbx.wrapping_mul(2))) as u64) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rax = ((rdx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rsp = ((rsp).wrapping_add(0x38u64)) as u64; return rax; }
            4 => { rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P5)) as u64, rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, P7, rcx, rsp); return rax; }
            5 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rax = 0; fl_l = 0; fl_r = 0; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x38u64)) as u64; return rax; }
            6 => { rdx = 0x1; rcx = r10; r8 = ((rdx.wrapping_add(0x1u64)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, P8, rcx, rsp); return rax; }
            7 => { crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rsi); rsi = (crate::simd_ops::ru64(r10.wrapping_add(0x78u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rdi); crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), r11); rbx = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x38u64))) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(rbx.wrapping_add(0x30u64))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x40u64))) as u64; bb = 3; continue; }
            8 => { r8 = rsp.wrapping_add(0x50); rcx = rbx; rdx = rsp.wrapping_add(0x20); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; fl_l = rcx; fl_r = (0xffffffffffffffffu64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 5; continue; } bb = 9; continue; }
            9 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x40u64))) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rax = ((rax).wrapping_add(crate::simd_ops::ru64(rbx.wrapping_add(0x58u64)))) as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rbx.wrapping_add(0x58u64)))) as u64; fl_l = rcx; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 5; continue; } rdx = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0xffffffff; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 5; continue; } rax = (crate::simd_ops::ru64(rsi)) as u64; rcx = ((rdx as u32)) as u32 as u64; rsi = (crate::simd_ops::ru64(rax)) as u64; rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x30u64))) as u64; rbx = (crate::simd_ops::ru64(rax.wrapping_add(rcx.wrapping_mul(8)))) as u64; rax = 0xffff; fl_l = ((rbx as u16)) as u16 as u64; fl_r = ((rax as u16)) as u16 as u64; if (((fl_l as u16)) == ((fl_r as u16))) { bb = 4; continue; } r8 = ((crate::simd_ops::ru16(P3)) as u32) as u32 as u64; rdi = (((rbx as u16)) as u32) as u32 as u64; rdi = ((rdi).wrapping_shl(5u32)) as u64; rdx = (crate::simd_ops::ru32(rdi.wrapping_add(rsi).wrapping_add(0x180a8u64))) as u32 as u64; rcx = (crate::simd_ops::ru64(rdi.wrapping_add(rsi).wrapping_add(0x18090u64))) as u64; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0xffffffff; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 5; continue; } rdx = (crate::simd_ops::ru32(rdi.wrapping_add(rsi).wrapping_add(0x180acu64))) as u32 as u64; rcx = (crate::simd_ops::ru64(rdi.wrapping_add(rsi).wrapping_add(0x18098u64))) as u64; r11 = (((rbx as u16)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rcx = ((((rax as u32)) as i32 as i64) as u64) as u64; rax = r11.wrapping_add(0xc03); rax = ((rax).wrapping_shl(5u32)) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(rsi).wrapping_add(0x40u64))) as u64; rdx = (crate::simd_ops::ru32(rax.wrapping_add(rcx.wrapping_mul(4)))) as u32 as u64; rdx = ((rdx).wrapping_add(crate::simd_ops::ru64(rsi.wrapping_add(r11.wrapping_mul(8)).wrapping_add(0x88u64)))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rbx = ((rbx).wrapping_shr(32u32)) as u64; rax = ((rbx.wrapping_add(rbx.wrapping_mul(2))) as u64) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rax = ((rdx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rsp = ((rsp).wrapping_add(0x38u64)) as u64; return rax; }
            10 => { rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P5)) as u64, rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, P9, rcx, rsp); return rax; }
            11 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rax = 0; fl_l = 0; fl_r = 0; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x38u64)) as u64; return rax; }
            12 => { rdx = 0x1; rcx = r10; r8 = ((rdx.wrapping_add(0x1u64)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, P10, rcx, rsp); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_41778<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rbx = rcx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P1))) as u64; rdi = rax; rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P2))) as u64; rax = P3; rdx = ((rdx).wrapping_add(0x30u64)) as u64; fl_l = rdx; fl_r = rcx; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 2; continue; } fl_l = rdx; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 1; continue; } fl_l = (crate::simd_ops::ru32(rdx.wrapping_add(P1))) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 1; continue; } xmm0 = ((crate::simd_ops::ru32(P4)) as u32) as u128; crate::simd_ops::wu32(rdi.wrapping_add(P5), ((xmm0 as u32)) as u32); rax = 0; fl_l = 0; fl_r = 0; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            1 => { fl_l = rdx; fl_r = rcx; bb = 2; continue; }
            2 => { rax = (if (((fl_l as u64)) < ((fl_r as u64))) { rdx } else { rax }) as u64; fl_l = (crate::simd_ops::ru32(rax.wrapping_add(P1))) as u32 as u64; fl_r = 0x3; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 3; continue; } rdx = rsp.wrapping_add(0x20); rcx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { xmm0 = ((crate::simd_ops::ru64(rax)) as u64) as u128; xmm0 = crate::simd_ops::vcvtpd2ps(xmm0); crate::simd_ops::wu32(rdi.wrapping_add(P5), ((xmm0 as u32)) as u32); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rax = 0; fl_l = 0; fl_r = 0; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            4 => { rdx = 0x3; rcx = rbx; r8 = ((rdx as u32)) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::lift_ops::trap(0x80000003, P8, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_41789<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), (0xffffffffffffffffu64) as u64);
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64;
    crate::simd_ops::wu64(rax, rcx);
    rax = 0;
    fl_l = 0; fl_r = 0;
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_41800<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64, const P9: u64, const P10: u64, const P11: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rdx = P1; rbx = rcx; rdi = rdx; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P2))) as u64; fl_l = rax; fl_r = rcx; rdi = (if (((fl_l as u64)) < ((fl_r as u64))) { rax } else { rdi }) as u64; fl_l = (crate::simd_ops::ru32(rdi.wrapping_add(P0))) as u32 as u64; fl_r = 0x2; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 1; continue; } rdi = 0; fl_l = 0; fl_r = 0; bb = 2; continue; }
            1 => { rdi = (crate::simd_ops::ru64(rdi)) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { fl_l = rax; fl_r = rcx; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 7; continue; } fl_l = rax; fl_r = rdx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 7; continue; } fl_l = (crate::simd_ops::ru32(rax.wrapping_add(P0))) as u32 as u64; fl_r = 0x2; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 7; continue; } bb = 3; continue; }
            3 => { rdx = rsp.wrapping_add(0x30); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rdi); rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 6; continue; } rcx = rsp.wrapping_add(0x30); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rdi); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } xmm0 = ((crate::simd_ops::ru32(rax.wrapping_add(P5))) as u32) as u128; bb = 5; continue; }
            4 => { xmm0 = 0u128; bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(P2))) as u64; xmm0 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | (((f32::from_bits(xmm0 as u32)) as f64).to_bits() as u128); crate::simd_ops::wu64(rax, ((xmm0 as u64)) as u64); crate::simd_ops::wu32(rax.wrapping_add(P0), (0x3u32) as u32); rax = 0x1; crate::simd_ops::lk_add_u64(rbx.wrapping_add(P2), (0x18u64) as u64); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            6 => { r8 = P6; rdx = 0x1; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::lift_ops::trap(0x80000003, P10, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            7 => { rdx = 0x1; rcx = rbx; r8 = ((rdx.wrapping_add(P8)) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P9, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::lift_ops::trap(0x80000003, P11, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_41844(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    fl_l = (crate::simd_ops::lk_add_u64(rcx, (0xffffffffffffffb0u64) as u64)) as u64; fl_r = fl_l;
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    crate::simd_ops::wu64(rdx, rax);
    rax = rdx;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_41845<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64;
    rax = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64;
    rax = ((rax).wrapping_shl(4u32)) as u64;
    fl_l = ((rax).wrapping_add(crate::simd_ops::ru64(rcx))) as u64; fl_r = fl_l;
    rax = ((fl_l as u64)) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_41847(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    fl_l = (crate::simd_ops::lk_add_u64(rcx, (0x50u64) as u64)) as u64; fl_r = fl_l;
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    crate::simd_ops::wu64(rdx, rax);
    rax = rdx;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_41861<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut ymm1_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    let y_addr: u64 = rcx; xmm1 = crate::simd_ops::ru128(y_addr); ymm1_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16));
    rcx = (((xmm1 >> 64) as u64)) as u64;
    xmm0 = (ymm1_hi) as u128;
    r8 = (((xmm0 >> 64) as u64)) as u64;
    r8 = ((r8).wrapping_shl(5u32)) as u64;
    let y_addr: u64 = rsp; crate::simd_ops::wu128(y_addr, xmm1); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm1_hi);
    rdx = (crate::simd_ops::ru64(rsp)) as u64;
    xmm0 = (crate::simd_ops::ru128(r8.wrapping_add(rdx))) as u128;
    crate::simd_ops::wu128(r8.wrapping_add(rcx), (xmm0) as u128);
    rax = (crate::simd_ops::ru32(r8.wrapping_add(rdx).wrapping_add(P0))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(rcx).wrapping_add(P0), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(r8.wrapping_add(rdx).wrapping_add(P1))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(rcx).wrapping_add(P1), ((rax as u32)) as u32);
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_41864(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    xmm0 = (xmm2) as u128;
    xmm0 = crate::simd_ops::vbroadcastss((xmm2 as u32));
    xmm2 = crate::simd_ops::vmulps(xmm0, crate::simd_ops::ru128(rdx));
    xmm1 = (xmm3) as u128;
    xmm1 = crate::simd_ops::vbroadcastss((xmm3 as u32));
    xmm0 = crate::simd_ops::vmulps(xmm2, xmm1);
    crate::simd_ops::wu128(rcx, (xmm0) as u128);
    rax = rcx;
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_41907<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64, const P9: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut xmm6: u128 = 0; let mut xmm8: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x50u64)) as u64; rbx = rcx; crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm6) as u128); rcx = ((rcx).wrapping_add(0x20c0u64)) as u64; crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm8) as u128); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm2 = ((crate::simd_ops::ru32(P1)) as u32) as u128; xmm6 = 0u128; fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = ((xmm6 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)).is_nan() || f32::from_bits((fl_r as u32)).is_nan()) { bb = 1; continue; } if (f32::from_bits((fl_l as u32)) != f32::from_bits((fl_r as u32)) && !f32::from_bits((fl_l as u32)).is_nan() && !f32::from_bits((fl_r as u32)).is_nan()) { bb = 1; continue; } xmm0 = 0u128; bb = 2; continue; }
            1 => { xmm0 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) / f32::from_bits(xmm0 as u32)).to_bits() as u128); bb = 2; continue; }
            2 => { xmm1 = ((crate::simd_ops::ru32(rbx.wrapping_add(0x2100u64))) as u32) as u128; fl_l = ((xmm1 as u32)) as u32 as u64; fl_r = ((xmm6 as u32)) as u32 as u64; xmm8 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(crate::simd_ops::ru32(P2))).to_bits() as u128); if (f32::from_bits((fl_l as u32)).is_nan() || f32::from_bits((fl_r as u32)).is_nan()) { bb = 3; continue; } if (f32::from_bits((fl_l as u32)) != f32::from_bits((fl_r as u32)) && !f32::from_bits((fl_l as u32)).is_nan() && !f32::from_bits((fl_r as u32)).is_nan()) { bb = 3; continue; } xmm3 = 0u128; bb = 4; continue; }
            3 => { xmm3 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) / f32::from_bits(xmm1 as u32)).to_bits() as u128); bb = 4; continue; }
            4 => { xmm0 = 0u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((((crate::simd_ops::ru32(rbx.wrapping_add(0x2104u64))) as i32) as f32).to_bits() as u128); xmm1 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm3 as u32)).to_bits() as u128); rcx = rbx.wrapping_add(0x20c0); crate::simd_ops::wu32(rbx.wrapping_add(0x20e0u64), ((xmm1 as u32)) as u32); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm1 = (xmm0) as u128; rcx = rbx.wrapping_add(0x2188); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm0 = ((crate::simd_ops::ru32(rbx.wrapping_add(0x20e4u64))) as u32) as u128; xmm1 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(crate::simd_ops::ru32(P2))).to_bits() as u128); rcx = rbx.wrapping_add(0x2330); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru32(P4)) as u32 as u64; xmm1 = (xmm8 & !0xffff_ffffu128) | ((f32::from_bits(xmm8 as u32) + f32::from_bits(crate::simd_ops::ru32(P5))).to_bits() as u128); rax = (((rax as u32)).wrapping_add(1)) as u32 as u64; fl_l = ((xmm1 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(P2)) as u32 as u64; crate::simd_ops::wu32(P5, ((xmm1 as u32)) as u32); crate::simd_ops::wu32(P4, ((rax as u32)) as u32); if !(f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 5; continue; } xmm0 = 0u128; xmm0 = (xmm0 & !0xffff_ffffu128) | (((((rax as u32)) as i32) as f32).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) / f32::from_bits(xmm0 as u32)).to_bits() as u128); crate::simd_ops::wu32(P6, ((xmm1 as u32)) as u32); crate::simd_ops::wu32(P5, ((xmm6 as u32)) as u32); crate::simd_ops::wu32(P4, (0x0u32) as u32); bb = 6; continue; }
            5 => { xmm1 = ((crate::simd_ops::ru32(P6)) as u32) as u128; bb = 6; continue; }
            6 => { xmm2 = (xmm1 & !0xffff_ffff_ffff_ffffu128) | (((f32::from_bits(xmm1 as u32)) as f64).to_bits() as u128); xmm1 = (xmm8 & !0xffff_ffff_ffff_ffffu128) | (((f32::from_bits(xmm8 as u32)) as f64).to_bits() as u128); rdx = ((xmm1 as u64)) as u64; r8 = ((xmm2 as u64)) as u64; rcx = P7; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P8, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 10; continue; } rcx = ((crate::simd_ops::ru8(rbx.wrapping_add(0x2237u64))) as u32) as u32 as u64; rdx = rbx.wrapping_add(0x223c); crate::simd_ops::wu32(rbx.wrapping_add(0x2238u64), ((rax as u32)) as u32); fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x80; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 7; continue; } rcx = ((rcx.wrapping_mul(8)) as u32) as u32 as u64; bb = 8; continue; }
            7 => { rcx = (((rcx as u32)).wrapping_shl(8u32)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x7c00; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 8; continue; }
            8 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x2234u64))) as u32 as u64; rax = ((((rax as u32)) & (0xffffffu32))) as u32 as u64; rax = (((rax as u32)).wrapping_shl(3u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 9; continue; } rdx = (crate::simd_ops::ru64(rdx)) as u64; bb = 9; continue; }
            9 => { crate::simd_ops::wu8(rdx, (0x0u8) as u8); xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x40u64))) as u128; xmm8 = (crate::simd_ops::ru128(rsp.wrapping_add(0x30u64))) as u128; rsp = ((rsp).wrapping_add(0x50u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            10 => { r9 = (0xffffffffffffffffu64) as u64; bb = 11; continue; }
            11 => { r9 = ((r9).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru8(rax.wrapping_add(r9))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 11; continue; } r8 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); rdx = rsp.wrapping_add(0x20); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), r9); rcx = rbx.wrapping_add(0x2234); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P9, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x40u64))) as u128; xmm8 = (crate::simd_ops::ru128(rsp.wrapping_add(0x30u64))) as u128; rsp = ((rsp).wrapping_add(0x50u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_41910<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let tmp_xchg: u64 = crate::simd_ops::atomic_xchg_u64(P0, rcx);
    rcx = (tmp_xchg) as u64;
    rax = rcx;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_41912<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    xmm0 = (crate::simd_ops::ru128(r8)) as u128;
    crate::simd_ops::wu128(rcx.wrapping_add(P0), (xmm0) as u128);
    crate::simd_ops::wu64(rcx, rdx);
    rax = rcx;
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_41928<const P0: u64, const P1: u64, const P2: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    r8 = rdx;
    fl_l = rdx; fl_r = rdx;
    if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { return rax; }
    rax = ((rcx.wrapping_add(P0)) as u64) as u64;
    loop { rcx = 0; fl_l = 0; fl_r = 0; rdx = 0; fl_l = 0; fl_r = 0; let tmp_xchg: u64 = crate::simd_ops::atomic_xchg_u64(rax.wrapping_add(P1), rcx); rcx = (tmp_xchg) as u64; let tmp_xchg: u64 = crate::simd_ops::atomic_xchg_u64(rax, rdx); rdx = (tmp_xchg) as u64; rax = ((rax.wrapping_add(P2)) as u64) as u64; fl_l = r8; fl_r = (0x1u64) as u64; r8 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if !((((fl_l as u64)) != ((fl_r as u64)))) { break; } }
    return rax;
}
