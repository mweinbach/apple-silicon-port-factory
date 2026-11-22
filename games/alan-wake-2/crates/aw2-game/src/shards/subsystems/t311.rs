#![allow(unused_imports, unused_variables, unused_mut, unused_assignments, non_snake_case, dead_code, clippy::all, unreachable_code, unused_parens)]

#[inline(never)]
pub unsafe extern "win64" fn fn_141d87f60(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14240aa00; const P1: u64 = 0x140dfd9e0; const P2: u64 = 0x140dfd740;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = r8; rdi = rdx; rax = ((crate::simd_ops::ru8(rdx.wrapping_add(0x8u64))) as u32) as u32 as u64; crate::simd_ops::wu8(r8.wrapping_add(0x8u64), ((rax as u8)) as u8); rax = ((crate::simd_ops::ru8(rdx.wrapping_add(0x9u64))) as u32) as u32 as u64; crate::simd_ops::wu8(r8.wrapping_add(0x9u64), ((rax as u8)) as u8); rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x10u64))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(0x10u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x14u64))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(0x14u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x18u64))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(0x18u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x1cu64))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(0x1cu64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x20u64))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(0x20u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x24u64))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(0x24u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x28u64))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(0x28u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x2cu64))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(0x2cu64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x30u64))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(0x30u64), ((rax as u32)) as u32); rax = ((crate::simd_ops::ru8(rdx.wrapping_add(0x34u64))) as u32) as u32 as u64; crate::simd_ops::wu8(r8.wrapping_add(0x34u64), ((rax as u8)) as u8); rax = ((crate::simd_ops::ru8(rdx.wrapping_add(0x35u64))) as u32) as u32 as u64; crate::simd_ops::wu8(r8.wrapping_add(0x35u64), ((rax as u8)) as u8); rax = ((crate::simd_ops::ru8(rdx.wrapping_add(0x36u64))) as u32) as u32 as u64; crate::simd_ops::wu8(r8.wrapping_add(0x36u64), ((rax as u8)) as u8); rax = ((crate::simd_ops::ru8(rdx.wrapping_add(0x37u64))) as u32) as u32 as u64; crate::simd_ops::wu8(r8.wrapping_add(0x37u64), ((rax as u8)) as u8); rax = ((crate::simd_ops::ru8(rdx.wrapping_add(0x38u64))) as u32) as u32 as u64; crate::simd_ops::wu8(r8.wrapping_add(0x38u64), ((rax as u8)) as u8); rdx = ((rdx).wrapping_add(0x3cu64)) as u64; rcx = r8.wrapping_add(0x3c); rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rax = rdi.wrapping_add(0x68); rcx = rbx.wrapping_add(0x68); fl_l = rcx; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rdx = (crate::simd_ops::ru64(rax)) as u64; rax = (crate::simd_ops::ru32(rax.wrapping_add(0x8u64))) as u32 as u64; r8 = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; r8 = ((r8).wrapping_shl(4u32)) as u64; r8 = ((r8).wrapping_add(rdx)) as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x78u64))) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x78u64), ((rax as u32)) as u32); rax = ((crate::simd_ops::ru8(rdi.wrapping_add(0x7cu64))) as u32) as u32 as u64; crate::simd_ops::wu8(rbx.wrapping_add(0x7cu64), ((rax as u8)) as u8); rax = ((crate::simd_ops::ru8(rdi.wrapping_add(0x7du64))) as u32) as u32 as u64; crate::simd_ops::wu8(rbx.wrapping_add(0x7du64), ((rax as u8)) as u8); rax = ((crate::simd_ops::ru8(rdi.wrapping_add(0x7eu64))) as u32) as u32 as u64; crate::simd_ops::wu8(rbx.wrapping_add(0x7eu64), ((rax as u8)) as u8); rdx = rdi.wrapping_add(0x80); rcx = rbx.wrapping_add(0x80); rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rdx = rdi.wrapping_add(0xa8); rcx = rbx.wrapping_add(0xa8); rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rdx = rdi.wrapping_add(0xd0); rcx = rbx.wrapping_add(0xd0); rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rdi.wrapping_add(0xf8u64))) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0xf8u64), rax); rdi = ((rdi).wrapping_add(0x100u64)) as u64; rcx = rbx.wrapping_add(0x100); fl_l = rcx; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rdx = (crate::simd_ops::ru64(rdi)) as u64; rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x8u64))) as u32 as u64; r8 = (((rax) as i64).wrapping_mul((0x34u64) as i64) as u64) as u64; r8 = ((r8).wrapping_add(rdx)) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 4; continue; }
            4 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141d88250(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1432f9258; const P1: u64 = 0x14; const P2: u64 = 0x143877b88; const P3: u64 = 0x1; const P4: u64 = 0x1401de410;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x48u64)) as u64;
    rax = rdx;
    r9 = P0;
    rdx = ((rcx.wrapping_add(P1)) as u64) as u64;
    rcx = P2;
    crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rcx);
    rcx = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rcx);
    crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rcx);
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rcx);
    r8 = ((rcx.wrapping_add(P3)) as u32) as u32 as u64;
    rcx = rax;
    rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x48u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141d88420(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143877b88; const P1: u64 = 0x1432f9258; const P2: u64 = 0x1; const P3: u64 = 0x1401de410;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x48u64)) as u64;
    rcx = P0;
    rax = r8;
    crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rcx);
    r9 = P1;
    rcx = 0;
    fl_l = 0; fl_r = 0;
    rdx = ((rdx).wrapping_add(0x14u64)) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rcx);
    crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rcx);
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rcx);
    r8 = ((rcx.wrapping_add(P2)) as u32) as u32 as u64;
    rcx = rax;
    rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x48u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141d88490(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143877a38; const P1: u64 = 0x1432f9258; const P2: u64 = 0x1; const P3: u64 = 0x1401de410; const P4: u64 = 0x40; const P5: u64 = 0x140728030;
    let (mut rax, mut r10, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x48u64)) as u64;
    rcx = P0;
    r10 = r8;
    crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rcx);
    r9 = P1;
    rcx = 0;
    fl_l = 0; fl_r = 0;
    rax = rdx;
    crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rcx);
    rdx = ((rdx).wrapping_add(0x14u64)) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rcx);
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rcx);
    r8 = ((rcx.wrapping_add(P2)) as u32) as u32 as u64;
    rcx = r10;
    rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp);
    rdx = ((rax.wrapping_add(P4)) as u64) as u64;
    rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x48u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141d888e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x18; const P2: u64 = 0x9; const P3: u64 = 0x10; const P4: u64 = 0x14; const P5: u64 = 0x14240aa00; const P6: u64 = 0x40;
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P0))) as u32) as u32 as u64;
    rcx = ((r8.wrapping_add(P1)) as u64) as u64;
    crate::simd_ops::wu8(r8.wrapping_add(P0), ((rax as u8)) as u8);
    rbx = rdx;
    rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P2))) as u32) as u32 as u64;
    rdi = r8;
    crate::simd_ops::wu8(r8.wrapping_add(P2), ((rax as u8)) as u8);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P3))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P3), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P4))) as u32 as u64;
    rdx = ((rdx).wrapping_add(0x18u64)) as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P4), ((rax as u32)) as u32);
    rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp);
    rax = (crate::simd_ops::ru64(rbx.wrapping_add(P6))) as u64;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    crate::simd_ops::wu64(rdi.wrapping_add(P6), rax);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141d88970(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x1432efff8; const P1: u64 = 0x14; const P2: u64 = 0x18; const P3: u64 = 0x1c; const P4: u64 = 0x20; const P5: u64 = 0x1b; const P6: u64 = 0x40;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = P0;
    crate::simd_ops::wu64(rdx, rax);
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu32(rdx.wrapping_add(P1), ((rax as u32)) as u32);
    crate::simd_ops::lk_and_u32(rdx.wrapping_add(P2), (0xff000004u32) as u32);
    fl_l = (crate::simd_ops::lk_or_u32(rdx.wrapping_add(P2), (0x4u32) as u32)) as u32 as u64; fl_r = fl_l;
    crate::simd_ops::wu32(rdx.wrapping_add(P3), ((rax as u32)) as u32);
    crate::simd_ops::wu8(rdx.wrapping_add(P4), ((rax as u8)) as u8);
    crate::simd_ops::wu8(rdx.wrapping_add(P5), (0x4u8) as u8);
    crate::simd_ops::wu64(rdx.wrapping_add(P6), rax);
    rax = rdx;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141d88cd0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143877b20; const P1: u64 = 0x1432f9490; const P2: u64 = 0x9; const P3: u64 = 0x1401de410; const P4: u64 = 0xd8; const P5: u64 = 0x140728030;
    let (mut rax, mut r10, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x48u64)) as u64;
    rcx = P0;
    r10 = r8;
    crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rcx);
    r9 = P1;
    rcx = 0;
    fl_l = 0; fl_r = 0;
    rax = rdx;
    crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rcx);
    rdx = ((rdx).wrapping_add(0x74u64)) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rcx);
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rcx);
    r8 = ((rcx.wrapping_add(P2)) as u32) as u32 as u64;
    rcx = r10;
    rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp);
    rdx = ((rax.wrapping_add(P4)) as u64) as u64;
    rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x48u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141d89120(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x24; const P2: u64 = 0x9; const P3: u64 = 0x10; const P4: u64 = 0x14; const P5: u64 = 0x18; const P6: u64 = 0x1c; const P7: u64 = 0x20; const P8: u64 = 0x14240aa00; const P9: u64 = 0x4c; const P10: u64 = 0x74; const P11: u64 = 0x78;
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P0))) as u32) as u32 as u64; rcx = ((r8.wrapping_add(P1)) as u64) as u64; crate::simd_ops::wu8(r8.wrapping_add(P0), ((rax as u8)) as u8); rbx = rdx; rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P2))) as u32) as u32 as u64; rdi = r8; crate::simd_ops::wu8(r8.wrapping_add(P2), ((rax as u8)) as u8); rax = (crate::simd_ops::ru32(rdx.wrapping_add(P3))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(P3), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdx.wrapping_add(P4))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(P4), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdx.wrapping_add(P5))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(P5), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdx.wrapping_add(P6))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(P6), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdx.wrapping_add(P7))) as u32 as u64; rdx = ((rdx).wrapping_add(0x24u64)) as u64; crate::simd_ops::wu32(r8.wrapping_add(P7), ((rax as u32)) as u32); rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); rdx = ((rbx.wrapping_add(P9)) as u64) as u64; rcx = ((rdi.wrapping_add(P9)) as u64) as u64; rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru32(rbx.wrapping_add(P10))) as u32 as u64; rdx = ((rbx.wrapping_add(P11)) as u64) as u64; rcx = rdi.wrapping_add(0x78); crate::simd_ops::wu32(rdi.wrapping_add(0x74u64), ((rax as u32)) as u32); rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); rdx = rbx.wrapping_add(0xa0); rcx = rdi.wrapping_add(0xa0); rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru32(rbx.wrapping_add(0xc8u64))) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0xc8u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rbx.wrapping_add(0xccu64))) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0xccu64), ((rax as u32)) as u32); rax = ((crate::simd_ops::ru8(rbx.wrapping_add(0xd0u64))) as u32) as u32 as u64; crate::simd_ops::wu8(rdi.wrapping_add(0xd0u64), ((rax as u8)) as u8); rax = ((crate::simd_ops::ru8(rbx.wrapping_add(0xd1u64))) as u32) as u32 as u64; crate::simd_ops::wu8(rdi.wrapping_add(0xd1u64), ((rax as u8)) as u8); rax = ((crate::simd_ops::ru8(rbx.wrapping_add(0xd2u64))) as u32) as u32 as u64; crate::simd_ops::wu8(rdi.wrapping_add(0xd2u64), ((rax as u8)) as u8); rax = ((crate::simd_ops::ru8(rbx.wrapping_add(0xd3u64))) as u32) as u32 as u64; crate::simd_ops::wu8(rdi.wrapping_add(0xd3u64), ((rax as u8)) as u8); rax = ((crate::simd_ops::ru8(rbx.wrapping_add(0xd4u64))) as u32) as u32 as u64; crate::simd_ops::wu8(rdi.wrapping_add(0xd4u64), ((rax as u8)) as u8); rax = ((crate::simd_ops::ru8(rbx.wrapping_add(0xd5u64))) as u32) as u32 as u64; crate::simd_ops::wu8(rdi.wrapping_add(0xd5u64), ((rax as u8)) as u8); rax = (crate::simd_ops::ru64(rbx.wrapping_add(0xd8u64))) as u64; crate::simd_ops::wu64(rdi.wrapping_add(0xd8u64), rax); rax = ((crate::simd_ops::ru8(rbx.wrapping_add(0xe0u64))) as u32) as u32 as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu8(rdi.wrapping_add(0xe0u64), ((rax as u8)) as u8); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141d89270(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x1432f6358; const P1: u64 = 0x24; const P2: u64 = 0x28; const P3: u64 = 0x2c; const P4: u64 = 0x27; const P5: u64 = 0x4c; const P6: u64 = 0x50; const P7: u64 = 0x54; const P8: u64 = 0x4f; const P9: u64 = 0x74; const P10: u64 = 0x78; const P11: u64 = 0x7c;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = P0;
    crate::simd_ops::wu64(rdx, rax);
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::lk_and_u32(rdx.wrapping_add(P1), (0xff000004u32) as u32);
    crate::simd_ops::lk_or_u32(rdx.wrapping_add(P1), (0x4u32) as u32);
    crate::simd_ops::wu32(rdx.wrapping_add(P2), ((rax as u32)) as u32);
    crate::simd_ops::wu8(rdx.wrapping_add(P3), ((rax as u8)) as u8);
    crate::simd_ops::wu8(rdx.wrapping_add(P4), (0x4u8) as u8);
    crate::simd_ops::lk_and_u32(rdx.wrapping_add(P5), (0xff000004u32) as u32);
    crate::simd_ops::lk_or_u32(rdx.wrapping_add(P5), (0x4u32) as u32);
    crate::simd_ops::wu32(rdx.wrapping_add(P6), ((rax as u32)) as u32);
    crate::simd_ops::wu8(rdx.wrapping_add(P7), ((rax as u8)) as u8);
    crate::simd_ops::wu8(rdx.wrapping_add(P8), (0x4u8) as u8);
    crate::simd_ops::wu32(rdx.wrapping_add(P9), ((rax as u32)) as u32);
    crate::simd_ops::lk_and_u32(rdx.wrapping_add(P10), (0xff000004u32) as u32);
    crate::simd_ops::lk_or_u32(rdx.wrapping_add(P10), (0x4u32) as u32);
    crate::simd_ops::wu32(rdx.wrapping_add(P11), ((rax as u32)) as u32);
    crate::simd_ops::wu8(rdx.wrapping_add(0x80u64), ((rax as u8)) as u8);
    crate::simd_ops::wu8(rdx.wrapping_add(0x7bu64), (0x4u8) as u8);
    crate::simd_ops::lk_and_u32(rdx.wrapping_add(0xa0u64), (0xff000004u32) as u32);
    fl_l = (crate::simd_ops::lk_or_u32(rdx.wrapping_add(0xa0u64), (0x4u32) as u32)) as u32 as u64; fl_r = fl_l;
    crate::simd_ops::wu32(rdx.wrapping_add(0xa4u64), ((rax as u32)) as u32);
    crate::simd_ops::wu8(rdx.wrapping_add(0xa8u64), ((rax as u8)) as u8);
    crate::simd_ops::wu8(rdx.wrapping_add(0xa3u64), (0x4u8) as u8);
    crate::simd_ops::wu64(rdx.wrapping_add(0xd8u64), rax);
    rax = rdx;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141d8a960(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x9; const P2: u64 = 0x10; const P3: u64 = 0x140dfd1a0; const P4: u64 = 0x20; const P5: u64 = 0x140dfd470; const P6: u64 = 0x30; const P7: u64 = 0x140dfd740; const P8: u64 = 0x40; const P9: u64 = 0x14240aa00; const P10: u64 = 0x68; const P11: u64 = 0x6c;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = r8; rdi = rdx; rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P0))) as u32) as u32 as u64; crate::simd_ops::wu8(r8.wrapping_add(P0), ((rax as u8)) as u8); rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P1))) as u32) as u32 as u64; crate::simd_ops::wu8(r8.wrapping_add(P1), ((rax as u8)) as u8); rax = ((rdx.wrapping_add(P2)) as u64) as u64; rcx = ((r8.wrapping_add(P2)) as u64) as u64; fl_l = rcx; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rdx = (crate::simd_ops::ru64(rax)) as u64; r8 = (crate::simd_ops::ru32(rax.wrapping_add(P0))) as u32 as u64; r8 = ((r8).wrapping_shl(6u32)) as u64; r8 = ((r8).wrapping_add(rdx)) as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 2; continue; }
            2 => { rax = ((rdi.wrapping_add(P4)) as u64) as u64; rcx = ((rbx.wrapping_add(P4)) as u64) as u64; fl_l = rcx; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rdx = (crate::simd_ops::ru64(rax)) as u64; rax = (crate::simd_ops::ru32(rax.wrapping_add(P0))) as u32 as u64; r8 = (((rax) as i64).wrapping_mul((0x54u64) as i64) as u64) as u64; r8 = ((r8).wrapping_add(rdx)) as u64; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); bb = 4; continue; }
            4 => { rax = ((rdi.wrapping_add(P6)) as u64) as u64; rcx = ((rbx.wrapping_add(P6)) as u64) as u64; fl_l = rcx; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rdx = (crate::simd_ops::ru64(rax)) as u64; rax = (crate::simd_ops::ru32(rax.wrapping_add(P0))) as u32 as u64; r8 = (((rax) as i64).wrapping_mul((0x34u64) as i64) as u64) as u64; r8 = ((r8).wrapping_add(rdx)) as u64; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); bb = 6; continue; }
            6 => { rdx = ((rdi.wrapping_add(P8)) as u64) as u64; rcx = ((rbx.wrapping_add(P8)) as u64) as u64; rax = crate::simd_ops::call_stk(P9, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru32(rdi.wrapping_add(P10))) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(P10), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdi.wrapping_add(P11))) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x6cu64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x70u64))) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x70u64), ((rax as u32)) as u32); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141d8aa50(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x1432f0268; const P1: u64 = 0x10; const P2: u64 = 0x18; const P3: u64 = 0x20; const P4: u64 = 0x28; const P5: u64 = 0x30; const P6: u64 = 0x38; const P7: u64 = 0x40; const P8: u64 = 0x44; const P9: u64 = 0x48; const P10: u64 = 0x43;
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
    crate::simd_ops::lk_and_u32(rdx.wrapping_add(P7), (0xff000004u32) as u32);
    fl_l = (crate::simd_ops::lk_or_u32(rdx.wrapping_add(P7), (0x4u32) as u32)) as u32 as u64; fl_r = fl_l;
    crate::simd_ops::wu32(rdx.wrapping_add(P8), ((rax as u32)) as u32);
    crate::simd_ops::wu8(rdx.wrapping_add(P9), ((rax as u8)) as u8);
    rax = rdx;
    crate::simd_ops::wu8(rdx.wrapping_add(P10), (0x4u8) as u8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141d8ad50(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1432f9620; const P1: u64 = 0x14020f270;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x38u64)) as u64;
    rcx = 0;
    fl_l = 0; fl_r = 0;
    r9 = P0;
    crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rcx);
    rax = r8;
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rcx);
    rdx = ((rdx).wrapping_add(0xa0u64)) as u64;
    rcx = rax;
    r8 = 0;
    fl_l = 0; fl_r = 0;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x38u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141d8b1b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0x9; const P3: u64 = 0x14240aa00; const P4: u64 = 0x58; const P5: u64 = 0xa0; const P6: u64 = 0xa4; const P7: u64 = 0xa8; const P8: u64 = 0xac;
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P0))) as u32) as u32 as u64;
    rcx = ((r8.wrapping_add(P1)) as u64) as u64;
    crate::simd_ops::wu8(r8.wrapping_add(P0), ((rax as u8)) as u8);
    rbx = rdx;
    rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P2))) as u32) as u32 as u64;
    rdi = r8;
    rdx = ((rdx).wrapping_add(0x10u64)) as u64;
    crate::simd_ops::wu8(r8.wrapping_add(P2), ((rax as u8)) as u8);
    rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp);
    rdx = ((rbx.wrapping_add(P4)) as u64) as u64;
    rcx = ((rdi.wrapping_add(P4)) as u64) as u64;
    rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp);
    rax = (crate::simd_ops::ru32(rbx.wrapping_add(P5))) as u32 as u64;
    crate::simd_ops::wu32(rdi.wrapping_add(P5), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rbx.wrapping_add(P6))) as u32 as u64;
    crate::simd_ops::wu32(rdi.wrapping_add(P6), ((rax as u32)) as u32);
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
pub unsafe extern "win64" fn fn_141d8b960(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0x9; const P3: u64 = 0x14240aa00; const P4: u64 = 0x58; const P5: u64 = 0xa0; const P6: u64 = 0xa4; const P7: u64 = 0xa8; const P8: u64 = 0xac; const P9: u64 = 0xb0; const P10: u64 = 0xb4; const P11: u64 = 0xb8;
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P0))) as u32) as u32 as u64; rcx = ((r8.wrapping_add(P1)) as u64) as u64; crate::simd_ops::wu8(r8.wrapping_add(P0), ((rax as u8)) as u8); rbx = rdx; rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P2))) as u32) as u32 as u64; rdi = r8; rdx = ((rdx).wrapping_add(0x10u64)) as u64; crate::simd_ops::wu8(r8.wrapping_add(P2), ((rax as u8)) as u8); rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rdx = ((rbx.wrapping_add(P4)) as u64) as u64; rcx = ((rdi.wrapping_add(P4)) as u64) as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru32(rbx.wrapping_add(P5))) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(P5), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rbx.wrapping_add(P6))) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(P6), ((rax as u32)) as u32); rax = ((crate::simd_ops::ru8(rbx.wrapping_add(P7))) as u32) as u32 as u64; crate::simd_ops::wu8(rdi.wrapping_add(P7), ((rax as u8)) as u8); rax = (crate::simd_ops::ru32(rbx.wrapping_add(P8))) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(P8), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rbx.wrapping_add(P9))) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(P9), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rbx.wrapping_add(P10))) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(P10), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rbx.wrapping_add(P11))) as u32 as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu32(rdi.wrapping_add(0xb8u64), ((rax as u32)) as u32); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141d8bca0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x9; const P2: u64 = 0x10; const P3: u64 = 0x14; const P4: u64 = 0x18; const P5: u64 = 0x20;
    let (mut rax, mut r10, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    r8 = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    r10 = rdx;
    rax = ((crate::simd_ops::ru8(rcx.wrapping_add(P0))) as u32) as u32 as u64;
    r9 = rcx;
    crate::simd_ops::wu8(r8.wrapping_add(P0), ((rax as u8)) as u8);
    rax = ((crate::simd_ops::ru8(rcx.wrapping_add(P1))) as u32) as u32 as u64;
    crate::simd_ops::wu8(r8.wrapping_add(P1), ((rax as u8)) as u8);
    r8 = ((rcx.wrapping_add(P2)) as u64) as u64;
    r8 = ((r8).wrapping_sub(crate::simd_ops::ru64(rdx))) as u64;
    rdx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    rax = (crate::simd_ops::ru32(rcx.wrapping_add(P2))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(rdx), ((rax as u32)) as u32);
    rdx = ((rcx.wrapping_add(P3)) as u64) as u64;
    rdx = ((rdx).wrapping_sub(crate::simd_ops::ru64(r10))) as u64;
    rax = (crate::simd_ops::ru32(r9.wrapping_add(P3))) as u32 as u64;
    rcx = (crate::simd_ops::ru64(r10.wrapping_add(P0))) as u64;
    crate::simd_ops::wu32(rdx.wrapping_add(rcx), ((rax as u32)) as u32);
    rdx = ((r9.wrapping_add(P4)) as u64) as u64;
    rdx = ((rdx).wrapping_sub(crate::simd_ops::ru64(r10))) as u64;
    rcx = (crate::simd_ops::ru64(r10.wrapping_add(P0))) as u64;
    rax = (crate::simd_ops::ru32(r9.wrapping_add(P4))) as u32 as u64;
    crate::simd_ops::wu32(rdx.wrapping_add(rcx), ((rax as u32)) as u32);
    rdx = ((r9.wrapping_add(P5)) as u64) as u64;
    fl_l = rdx; fl_r = (crate::simd_ops::ru64(r10)) as u64;
    rdx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64;
    rcx = (crate::simd_ops::ru64(r10.wrapping_add(P0))) as u64;
    rax = (crate::simd_ops::ru64(r9.wrapping_add(P5))) as u64;
    crate::simd_ops::wu64(rdx.wrapping_add(rcx), rax);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141d8c030(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0x9; const P3: u64 = 0x14; const P4: u64 = 0x18; const P5: u64 = 0x20;
    let (mut rax, mut r10, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P0))) as u32) as u32 as u64;
    r9 = ((rdx.wrapping_add(P1)) as u64) as u64;
    rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    r10 = rdx;
    crate::simd_ops::wu8(rcx.wrapping_add(P0), ((rax as u8)) as u8);
    rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P2))) as u32) as u32 as u64;
    crate::simd_ops::wu8(rcx.wrapping_add(P2), ((rax as u8)) as u8);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P1))) as u32 as u64;
    rdx = ((rdx).wrapping_add(0x14u64)) as u64;
    rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    r9 = ((r9).wrapping_sub(crate::simd_ops::ru64(r8))) as u64;
    crate::simd_ops::wu32(r9.wrapping_add(rcx), ((rax as u32)) as u32);
    rdx = ((rdx).wrapping_sub(crate::simd_ops::ru64(r8))) as u64;
    rax = (crate::simd_ops::ru32(r10.wrapping_add(P3))) as u32 as u64;
    rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    crate::simd_ops::wu32(rdx.wrapping_add(rcx), ((rax as u32)) as u32);
    rdx = ((r10.wrapping_add(P4)) as u64) as u64;
    rdx = ((rdx).wrapping_sub(crate::simd_ops::ru64(r8))) as u64;
    rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    rax = (crate::simd_ops::ru32(r10.wrapping_add(P4))) as u32 as u64;
    crate::simd_ops::wu32(rdx.wrapping_add(rcx), ((rax as u32)) as u32);
    rdx = ((r10.wrapping_add(P5)) as u64) as u64;
    fl_l = rdx; fl_r = (crate::simd_ops::ru64(r8)) as u64;
    rdx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64;
    rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    rax = (crate::simd_ops::ru64(r10.wrapping_add(P5))) as u64;
    crate::simd_ops::wu64(rdx.wrapping_add(rcx), rax);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141d8c250(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x9; const P2: u64 = 0x10; const P3: u64 = 0x14; const P4: u64 = 0x18; const P5: u64 = 0x20;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P0))) as u32) as u32 as u64;
    crate::simd_ops::wu8(r8.wrapping_add(P0), ((rax as u8)) as u8);
    rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P1))) as u32) as u32 as u64;
    crate::simd_ops::wu8(r8.wrapping_add(P1), ((rax as u8)) as u8);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P2))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P2), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P3))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P3), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P4))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P4), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(P5))) as u64;
    crate::simd_ops::wu64(r8.wrapping_add(P5), rax);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141d8c9c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0x9; const P3: u64 = 0x1408b03b0; const P4: u64 = 0x20; const P5: u64 = 0x24; const P6: u64 = 0x28; const P7: u64 = 0x2c; const P8: u64 = 0x30; const P9: u64 = 0x34; const P10: u64 = 0x38; const P11: u64 = 0x3c;
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P0))) as u32) as u32 as u64; rcx = ((r8.wrapping_add(P1)) as u64) as u64; crate::simd_ops::wu8(r8.wrapping_add(P0), ((rax as u8)) as u8); rbx = rdx; rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P2))) as u32) as u32 as u64; rdi = r8; rdx = ((rdx).wrapping_add(0x10u64)) as u64; crate::simd_ops::wu8(r8.wrapping_add(P2), ((rax as u8)) as u8); rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru32(rbx.wrapping_add(P4))) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(P4), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rbx.wrapping_add(P5))) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(P5), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rbx.wrapping_add(P6))) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(P6), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rbx.wrapping_add(P7))) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(P7), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rbx.wrapping_add(P8))) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(P8), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rbx.wrapping_add(P9))) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(P9), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rbx.wrapping_add(P10))) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(P10), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rbx.wrapping_add(P11))) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x3cu64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x40u64))) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x40u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x44u64))) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x44u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x48u64))) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x48u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x4cu64))) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x4cu64), ((rax as u32)) as u32); rax = ((crate::simd_ops::ru8(rbx.wrapping_add(0x50u64))) as u32) as u32 as u64; crate::simd_ops::wu8(rdi.wrapping_add(0x50u64), ((rax as u8)) as u8); rax = ((crate::simd_ops::ru8(rbx.wrapping_add(0x51u64))) as u32) as u32 as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu8(rdi.wrapping_add(0x51u64), ((rax as u8)) as u8); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141d8d190(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x9; const P2: u64 = 0x10; const P3: u64 = 0x14240aa00; const P4: u64 = 0x98; const P5: u64 = 0x140dfcb10; const P6: u64 = 0xa8; const P7: u64 = 0xb8; const P8: u64 = 0xc8; const P9: u64 = 0x140dfcdd0; const P10: u64 = 0xd8; const P11: u64 = 0xdc;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = r8; rdi = rdx; rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P0))) as u32) as u32 as u64; crate::simd_ops::wu8(r8.wrapping_add(P0), ((rax as u8)) as u8); rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P1))) as u32) as u32 as u64; crate::simd_ops::wu8(r8.wrapping_add(P1), ((rax as u8)) as u8); rdx = ((rdx).wrapping_add(0x10u64)) as u64; rcx = ((r8.wrapping_add(P2)) as u64) as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rax = ((rdi.wrapping_add(P4)) as u64) as u64; rcx = ((rbx.wrapping_add(P4)) as u64) as u64; fl_l = rcx; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rdx = (crate::simd_ops::ru64(rax)) as u64; rax = (crate::simd_ops::ru32(rax.wrapping_add(P0))) as u32 as u64; r8 = ((rax.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; r8 = ((r8).wrapping_shl(4u32)) as u64; r8 = ((r8).wrapping_add(rdx)) as u64; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); bb = 2; continue; }
            2 => { rax = ((rdi.wrapping_add(P6)) as u64) as u64; rcx = ((rbx.wrapping_add(P6)) as u64) as u64; fl_l = rcx; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rdx = (crate::simd_ops::ru64(rax)) as u64; rax = (crate::simd_ops::ru32(rax.wrapping_add(P0))) as u32 as u64; r8 = ((rax.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; r8 = ((r8).wrapping_shl(4u32)) as u64; r8 = ((r8).wrapping_add(rdx)) as u64; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); bb = 4; continue; }
            4 => { rax = ((rdi.wrapping_add(P7)) as u64) as u64; rcx = ((rbx.wrapping_add(P7)) as u64) as u64; fl_l = rcx; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rdx = (crate::simd_ops::ru64(rax)) as u64; rax = (crate::simd_ops::ru32(rax.wrapping_add(P0))) as u32 as u64; r8 = ((rax.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; r8 = ((r8).wrapping_shl(4u32)) as u64; r8 = ((r8).wrapping_add(rdx)) as u64; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); bb = 6; continue; }
            6 => { rax = ((rdi.wrapping_add(P8)) as u64) as u64; rcx = ((rbx.wrapping_add(P8)) as u64) as u64; fl_l = rcx; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 8; continue; } bb = 7; continue; }
            7 => { rdx = (crate::simd_ops::ru64(rax)) as u64; rax = (crate::simd_ops::ru32(rax.wrapping_add(P0))) as u32 as u64; r8 = (((rax) as i64).wrapping_mul((0x19cu64) as i64) as u64) as u64; r8 = ((r8).wrapping_add(rdx)) as u64; rax = crate::simd_ops::call_stk(P9, rax, rcx, rdx, r8, r9, rsp); bb = 8; continue; }
            8 => { rax = (crate::simd_ops::ru32(rdi.wrapping_add(P10))) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(P10), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdi.wrapping_add(P11))) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0xdcu64), ((rax as u32)) as u32); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141d8d2c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x1432f0de8; const P1: u64 = 0x10; const P2: u64 = 0x14; const P3: u64 = 0x18; const P4: u64 = 0x13; const P5: u64 = 0x98; const P6: u64 = 0xa0; const P7: u64 = 0xa8; const P8: u64 = 0xb0; const P9: u64 = 0xb8; const P10: u64 = 0xc0; const P11: u64 = 0xc8;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = P0;
    crate::simd_ops::wu64(rdx, rax);
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::lk_and_u32(rdx.wrapping_add(P1), (0xff000010u32) as u32);
    fl_l = (crate::simd_ops::lk_or_u32(rdx.wrapping_add(P1), (0x10u32) as u32)) as u32 as u64; fl_r = fl_l;
    crate::simd_ops::wu32(rdx.wrapping_add(P2), ((rax as u32)) as u32);
    crate::simd_ops::wu8(rdx.wrapping_add(P3), ((rax as u8)) as u8);
    crate::simd_ops::wu8(rdx.wrapping_add(P4), (0x10u8) as u8);
    crate::simd_ops::wu64(rdx.wrapping_add(P5), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(P6), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(P7), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(P8), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(P9), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(P10), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(P11), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(0xd0u64), rax);
    rax = rdx;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141d8d9e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x9; const P2: u64 = 0x10; const P3: u64 = 0x14; const P4: u64 = 0x18; const P5: u64 = 0x140dfc840; const P6: u64 = 0x28;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = r8; rdi = rdx; rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P0))) as u32) as u32 as u64; crate::simd_ops::wu8(r8.wrapping_add(P0), ((rax as u8)) as u8); rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P1))) as u32) as u32 as u64; crate::simd_ops::wu8(r8.wrapping_add(P1), ((rax as u8)) as u8); rax = (crate::simd_ops::ru32(rdx.wrapping_add(P2))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(P2), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdx.wrapping_add(P3))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(P3), ((rax as u32)) as u32); rax = ((rdx.wrapping_add(P4)) as u64) as u64; rcx = ((r8.wrapping_add(P4)) as u64) as u64; fl_l = rcx; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rdx = (crate::simd_ops::ru64(rax)) as u64; rax = (crate::simd_ops::ru32(rax.wrapping_add(P0))) as u32 as u64; r8 = (((rax) as i64).wrapping_mul((0x94u64) as i64) as u64) as u64; r8 = ((r8).wrapping_add(rdx)) as u64; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); bb = 2; continue; }
            2 => { rdi = ((rdi).wrapping_add(0x28u64)) as u64; rcx = ((rbx.wrapping_add(P6)) as u64) as u64; fl_l = rcx; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rdx = (crate::simd_ops::ru64(rdi)) as u64; rax = (crate::simd_ops::ru32(rdi.wrapping_add(P0))) as u32 as u64; r8 = (((rax) as i64).wrapping_mul((0x94u64) as i64) as u64) as u64; r8 = ((r8).wrapping_add(rdx)) as u64; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); bb = 4; continue; }
            4 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141d8f090(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x2c; const P2: u64 = 0x9; const P3: u64 = 0x10; const P4: u64 = 0x18; const P5: u64 = 0x20; const P6: u64 = 0x21; const P7: u64 = 0x22; const P8: u64 = 0x23; const P9: u64 = 0x24; const P10: u64 = 0x28; const P11: u64 = 0x14240aa00;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P0))) as u32) as u32 as u64;
    rcx = ((r8.wrapping_add(P1)) as u64) as u64;
    crate::simd_ops::wu8(r8.wrapping_add(P0), ((rax as u8)) as u8);
    rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P2))) as u32) as u32 as u64;
    crate::simd_ops::wu8(r8.wrapping_add(P2), ((rax as u8)) as u8);
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(P3))) as u64;
    crate::simd_ops::wu64(r8.wrapping_add(P3), rax);
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(P4))) as u64;
    crate::simd_ops::wu64(r8.wrapping_add(P4), rax);
    rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P5))) as u32) as u32 as u64;
    crate::simd_ops::wu8(r8.wrapping_add(P5), ((rax as u8)) as u8);
    rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P6))) as u32) as u32 as u64;
    crate::simd_ops::wu8(r8.wrapping_add(P6), ((rax as u8)) as u8);
    rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P7))) as u32) as u32 as u64;
    crate::simd_ops::wu8(r8.wrapping_add(P7), ((rax as u8)) as u8);
    rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P8))) as u32) as u32 as u64;
    crate::simd_ops::wu8(r8.wrapping_add(P8), ((rax as u8)) as u8);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P9))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P9), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P10))) as u32 as u64;
    fl_l = ((rdx).wrapping_add(0x2cu64)) as u64; fl_r = fl_l;
    rdx = ((fl_l as u64)) as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P10), ((rax as u32)) as u32);
    return crate::simd_ops::call_fwd(P11, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141d8f820(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0x9; const P3: u64 = 0x14240aa00; const P4: u64 = 0x38; const P5: u64 = 0x3c;
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P0))) as u32) as u32 as u64;
    rcx = ((r8.wrapping_add(P1)) as u64) as u64;
    crate::simd_ops::wu8(r8.wrapping_add(P0), ((rax as u8)) as u8);
    rbx = rdx;
    rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P2))) as u32) as u32 as u64;
    rdi = r8;
    rdx = ((rdx).wrapping_add(0x10u64)) as u64;
    crate::simd_ops::wu8(r8.wrapping_add(P2), ((rax as u8)) as u8);
    rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp);
    rax = (crate::simd_ops::ru32(rbx.wrapping_add(P4))) as u32 as u64;
    crate::simd_ops::wu32(rdi.wrapping_add(P4), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rbx.wrapping_add(P5))) as u32 as u64;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    crate::simd_ops::wu32(rdi.wrapping_add(P5), ((rax as u32)) as u32);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141d8fb00(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x9; const P2: u64 = 0x10; const P3: u64 = 0x14; const P4: u64 = 0x18; const P5: u64 = 0x1c; const P6: u64 = 0x20;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = ((crate::simd_ops::ru8(rcx.wrapping_add(P0))) as u32) as u32 as u64;
    r9 = rcx;
    r8 = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    crate::simd_ops::wu8(r8.wrapping_add(P0), ((rax as u8)) as u8);
    rax = ((crate::simd_ops::ru8(rcx.wrapping_add(P1))) as u32) as u32 as u64;
    crate::simd_ops::wu8(r8.wrapping_add(P1), ((rax as u8)) as u8);
    rax = (crate::simd_ops::ru32(rcx.wrapping_add(P2))) as u32 as u64;
    r8 = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    r8 = ((r8).wrapping_sub(crate::simd_ops::ru64(rdx))) as u64;
    crate::simd_ops::wu32(r8.wrapping_add(rcx).wrapping_add(P2), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(r9.wrapping_add(P3))) as u32 as u64;
    rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(rdx))) as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(r9).wrapping_add(P3), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(r9.wrapping_add(P4))) as u32 as u64;
    rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(rdx))) as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(r9).wrapping_add(P4), ((rax as u32)) as u32);
    rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(rdx))) as u64;
    rax = ((crate::simd_ops::ru8(r9.wrapping_add(P5))) as u32) as u32 as u64;
    crate::simd_ops::wu8(rcx.wrapping_add(r9).wrapping_add(P5), ((rax as u8)) as u8);
    rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    fl_l = rcx; fl_r = (crate::simd_ops::ru64(rdx)) as u64;
    rcx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64;
    rax = (crate::simd_ops::ru64(r9.wrapping_add(P6))) as u64;
    crate::simd_ops::wu64(rcx.wrapping_add(r9).wrapping_add(P6), rax);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141d8fe90(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x9; const P2: u64 = 0x10; const P3: u64 = 0x14; const P4: u64 = 0x18; const P5: u64 = 0x1c; const P6: u64 = 0x20;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P0))) as u32) as u32 as u64;
    rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    crate::simd_ops::wu8(rcx.wrapping_add(P0), ((rax as u8)) as u8);
    rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P1))) as u32) as u32 as u64;
    crate::simd_ops::wu8(rcx.wrapping_add(P1), ((rax as u8)) as u8);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P2))) as u32 as u64;
    rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(r8))) as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(rdx).wrapping_add(P2), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P3))) as u32 as u64;
    rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(r8))) as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(rdx).wrapping_add(P3), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P4))) as u32 as u64;
    rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(r8))) as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(rdx).wrapping_add(P4), ((rax as u32)) as u32);
    rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(r8))) as u64;
    rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P5))) as u32) as u32 as u64;
    crate::simd_ops::wu8(rcx.wrapping_add(rdx).wrapping_add(P5), ((rax as u8)) as u8);
    rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    fl_l = rcx; fl_r = (crate::simd_ops::ru64(r8)) as u64;
    rcx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64;
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(P6))) as u64;
    crate::simd_ops::wu64(rcx.wrapping_add(rdx).wrapping_add(P6), rax);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141d900b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x9; const P2: u64 = 0x10; const P3: u64 = 0x14; const P4: u64 = 0x18; const P5: u64 = 0x1c; const P6: u64 = 0x20;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P0))) as u32) as u32 as u64;
    crate::simd_ops::wu8(r8.wrapping_add(P0), ((rax as u8)) as u8);
    rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P1))) as u32) as u32 as u64;
    crate::simd_ops::wu8(r8.wrapping_add(P1), ((rax as u8)) as u8);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P2))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P2), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P3))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P3), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P4))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P4), ((rax as u32)) as u32);
    rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P5))) as u32) as u32 as u64;
    crate::simd_ops::wu8(r8.wrapping_add(P5), ((rax as u8)) as u8);
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(P6))) as u64;
    crate::simd_ops::wu64(r8.wrapping_add(P6), rax);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141d90940(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0x9; const P3: u64 = 0x1408b03b0; const P4: u64 = 0x20; const P5: u64 = 0x24; const P6: u64 = 0x28; const P7: u64 = 0x2c; const P8: u64 = 0x30;
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P0))) as u32) as u32 as u64;
    rcx = ((r8.wrapping_add(P1)) as u64) as u64;
    crate::simd_ops::wu8(r8.wrapping_add(P0), ((rax as u8)) as u8);
    rbx = rdx;
    rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P2))) as u32) as u32 as u64;
    rdi = r8;
    rdx = ((rdx).wrapping_add(0x10u64)) as u64;
    crate::simd_ops::wu8(r8.wrapping_add(P2), ((rax as u8)) as u8);
    rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp);
    rax = (crate::simd_ops::ru32(rbx.wrapping_add(P4))) as u32 as u64;
    crate::simd_ops::wu32(rdi.wrapping_add(P4), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rbx.wrapping_add(P5))) as u32 as u64;
    crate::simd_ops::wu32(rdi.wrapping_add(P5), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rbx.wrapping_add(P6))) as u32 as u64;
    crate::simd_ops::wu32(rdi.wrapping_add(P6), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rbx.wrapping_add(P7))) as u32 as u64;
    crate::simd_ops::wu32(rdi.wrapping_add(P7), ((rax as u32)) as u32);
    rax = ((crate::simd_ops::ru8(rbx.wrapping_add(P8))) as u32) as u32 as u64;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    crate::simd_ops::wu8(rdi.wrapping_add(P8), ((rax as u8)) as u8);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141d90c90(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x9; const P2: u64 = 0x10; const P3: u64 = 0x14; const P4: u64 = 0x18; const P5: u64 = 0x1c; const P6: u64 = 0x20; const P7: u64 = 0x28; const P8: u64 = 0x24;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = ((crate::simd_ops::ru8(rcx.wrapping_add(P0))) as u32) as u32 as u64; r9 = rcx; r8 = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; crate::simd_ops::wu8(r8.wrapping_add(P0), ((rax as u8)) as u8); rax = ((crate::simd_ops::ru8(rcx.wrapping_add(P1))) as u32) as u32 as u64; crate::simd_ops::wu8(r8.wrapping_add(P1), ((rax as u8)) as u8); rax = (crate::simd_ops::ru32(rcx.wrapping_add(P2))) as u32 as u64; r8 = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; r8 = ((r8).wrapping_sub(crate::simd_ops::ru64(rdx))) as u64; crate::simd_ops::wu32(r8.wrapping_add(rcx).wrapping_add(P2), ((rax as u32)) as u32); rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(rdx))) as u64; rax = (crate::simd_ops::ru32(r9.wrapping_add(P3))) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(r9).wrapping_add(P3), ((rax as u32)) as u32); rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(rdx))) as u64; rax = (crate::simd_ops::ru32(r9.wrapping_add(P4))) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(r9).wrapping_add(P4), ((rax as u32)) as u32); rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(rdx))) as u64; rax = (crate::simd_ops::ru32(r9.wrapping_add(P5))) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(r9).wrapping_add(P5), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(r9.wrapping_add(P6))) as u32 as u64; rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(rdx))) as u64; crate::simd_ops::wu32(rcx.wrapping_add(r9).wrapping_add(P6), ((rax as u32)) as u32); rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(rdx))) as u64; rax = ((crate::simd_ops::ru8(r9.wrapping_add(P7))) as u32) as u32 as u64; crate::simd_ops::wu8(rcx.wrapping_add(r9).wrapping_add(P7), ((rax as u8)) as u8); rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; fl_l = rcx; fl_r = (crate::simd_ops::ru64(rdx)) as u64; rcx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; rax = (crate::simd_ops::ru32(r9.wrapping_add(P8))) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(r9).wrapping_add(P8), ((rax as u32)) as u32); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141d910b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x9; const P2: u64 = 0x10; const P3: u64 = 0x14; const P4: u64 = 0x18; const P5: u64 = 0x1c; const P6: u64 = 0x20; const P7: u64 = 0x28; const P8: u64 = 0x24;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P0))) as u32) as u32 as u64; rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64; crate::simd_ops::wu8(rcx.wrapping_add(P0), ((rax as u8)) as u8); rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P1))) as u32) as u32 as u64; crate::simd_ops::wu8(rcx.wrapping_add(P1), ((rax as u8)) as u8); rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64; rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(r8))) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(P2))) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(rdx).wrapping_add(P2), ((rax as u32)) as u32); rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64; rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(r8))) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(P3))) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(rdx).wrapping_add(P3), ((rax as u32)) as u32); rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64; rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(r8))) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(P4))) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(rdx).wrapping_add(P4), ((rax as u32)) as u32); rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64; rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(r8))) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(P5))) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(rdx).wrapping_add(P5), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdx.wrapping_add(P6))) as u32 as u64; rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64; rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(r8))) as u64; crate::simd_ops::wu32(rcx.wrapping_add(rdx).wrapping_add(P6), ((rax as u32)) as u32); rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64; rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(r8))) as u64; rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P7))) as u32) as u32 as u64; crate::simd_ops::wu8(rcx.wrapping_add(rdx).wrapping_add(P7), ((rax as u8)) as u8); rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64; fl_l = rcx; fl_r = (crate::simd_ops::ru64(r8)) as u64; rcx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(P8))) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(rdx).wrapping_add(P8), ((rax as u32)) as u32); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141d912e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x9; const P2: u64 = 0x10; const P3: u64 = 0x14; const P4: u64 = 0x18; const P5: u64 = 0x1c; const P6: u64 = 0x20; const P7: u64 = 0x24; const P8: u64 = 0x28;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P0))) as u32) as u32 as u64;
    crate::simd_ops::wu8(r8.wrapping_add(P0), ((rax as u8)) as u8);
    rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P1))) as u32) as u32 as u64;
    crate::simd_ops::wu8(r8.wrapping_add(P1), ((rax as u8)) as u8);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P2))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P2), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P3))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P3), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P4))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P4), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P5))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P5), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P6))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P6), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P7))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P7), ((rax as u32)) as u32);
    rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P8))) as u32) as u32 as u64;
    crate::simd_ops::wu8(r8.wrapping_add(P8), ((rax as u8)) as u8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141d91dc0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x9; const P2: u64 = 0x10; const P3: u64 = 0x14; const P4: u64 = 0x18; const P5: u64 = 0x1c; const P6: u64 = 0x20; const P7: u64 = 0x24; const P8: u64 = 0x28;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = ((crate::simd_ops::ru8(rcx.wrapping_add(P0))) as u32) as u32 as u64; r9 = rcx; r8 = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; crate::simd_ops::wu8(r8.wrapping_add(P0), ((rax as u8)) as u8); rax = ((crate::simd_ops::ru8(rcx.wrapping_add(P1))) as u32) as u32 as u64; crate::simd_ops::wu8(r8.wrapping_add(P1), ((rax as u8)) as u8); rax = (crate::simd_ops::ru32(rcx.wrapping_add(P2))) as u32 as u64; r8 = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; r8 = ((r8).wrapping_sub(crate::simd_ops::ru64(rdx))) as u64; crate::simd_ops::wu32(r8.wrapping_add(rcx).wrapping_add(P2), ((rax as u32)) as u32); rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(rdx))) as u64; rax = (crate::simd_ops::ru32(r9.wrapping_add(P3))) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(r9).wrapping_add(P3), ((rax as u32)) as u32); rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(rdx))) as u64; rax = (crate::simd_ops::ru32(r9.wrapping_add(P4))) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(r9).wrapping_add(P4), ((rax as u32)) as u32); rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(rdx))) as u64; rax = (crate::simd_ops::ru32(r9.wrapping_add(P5))) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(r9).wrapping_add(P5), ((rax as u32)) as u32); rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(rdx))) as u64; rax = (crate::simd_ops::ru32(r9.wrapping_add(P6))) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(r9).wrapping_add(P6), ((rax as u32)) as u32); rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(rdx))) as u64; rax = (crate::simd_ops::ru32(r9.wrapping_add(P7))) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(r9).wrapping_add(P7), ((rax as u32)) as u32); rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; fl_l = rcx; fl_r = (crate::simd_ops::ru64(rdx)) as u64; rcx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; rax = (crate::simd_ops::ru32(r9.wrapping_add(P8))) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(r9).wrapping_add(P8), ((rax as u32)) as u32); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141d92240(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x9; const P2: u64 = 0x10; const P3: u64 = 0x14; const P4: u64 = 0x18; const P5: u64 = 0x1c; const P6: u64 = 0x20; const P7: u64 = 0x24; const P8: u64 = 0x28;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P0))) as u32) as u32 as u64; rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64; crate::simd_ops::wu8(rcx.wrapping_add(P0), ((rax as u8)) as u8); rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P1))) as u32) as u32 as u64; crate::simd_ops::wu8(rcx.wrapping_add(P1), ((rax as u8)) as u8); rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64; rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(r8))) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(P2))) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(rdx).wrapping_add(P2), ((rax as u32)) as u32); rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64; rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(r8))) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(P3))) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(rdx).wrapping_add(P3), ((rax as u32)) as u32); rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64; rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(r8))) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(P4))) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(rdx).wrapping_add(P4), ((rax as u32)) as u32); rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64; rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(r8))) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(P5))) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(rdx).wrapping_add(P5), ((rax as u32)) as u32); rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64; rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(r8))) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(P6))) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(rdx).wrapping_add(P6), ((rax as u32)) as u32); rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64; rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(r8))) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(P7))) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(rdx).wrapping_add(P7), ((rax as u32)) as u32); rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64; fl_l = rcx; fl_r = (crate::simd_ops::ru64(r8)) as u64; rcx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(P8))) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(rdx).wrapping_add(P8), ((rax as u32)) as u32); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141d92470(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x9; const P2: u64 = 0x10; const P3: u64 = 0x14; const P4: u64 = 0x18; const P5: u64 = 0x1c; const P6: u64 = 0x20; const P7: u64 = 0x24; const P8: u64 = 0x28;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P0))) as u32) as u32 as u64;
    crate::simd_ops::wu8(r8.wrapping_add(P0), ((rax as u8)) as u8);
    rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P1))) as u32) as u32 as u64;
    crate::simd_ops::wu8(r8.wrapping_add(P1), ((rax as u8)) as u8);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P2))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P2), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P3))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P3), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P4))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P4), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P5))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P5), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P6))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P6), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P7))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P7), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P8))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P8), ((rax as u32)) as u32);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141d92c40(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x1432ee258; const P1: u64 = 0x10; const P2: u64 = 0x13; const P3: u64 = 0x14; const P4: u64 = 0x18;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = P0;
    crate::simd_ops::wu64(rdx, rax);
    rax = rdx;
    crate::simd_ops::lk_and_u32(rdx.wrapping_add(P1), (0xff000008u32) as u32);
    fl_l = (crate::simd_ops::lk_or_u32(rdx.wrapping_add(P1), (0x8u32) as u32)) as u32 as u64; fl_r = fl_l;
    crate::simd_ops::wu8(rdx.wrapping_add(P2), (0x8u8) as u8);
    crate::simd_ops::wu32(rdx.wrapping_add(P3), (0x0u32) as u32);
    crate::simd_ops::wu8(rdx.wrapping_add(P4), (0x0u8) as u8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141d93330(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0x9; const P3: u64 = 0x14240aa00; const P4: u64 = 0x58; const P5: u64 = 0xa0; const P6: u64 = 0xe8; const P7: u64 = 0xf0; const P8: u64 = 0xf4; const P9: u64 = 0xf8; const P10: u64 = 0xfc; const P11: u64 = 0x100;
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P0))) as u32) as u32 as u64; rcx = ((r8.wrapping_add(P1)) as u64) as u64; crate::simd_ops::wu8(r8.wrapping_add(P0), ((rax as u8)) as u8); rbx = rdx; rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P2))) as u32) as u32 as u64; rdi = r8; rdx = ((rdx).wrapping_add(0x10u64)) as u64; crate::simd_ops::wu8(r8.wrapping_add(P2), ((rax as u8)) as u8); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = ((rbx.wrapping_add(P4)) as u64) as u64; rcx = ((rdi.wrapping_add(P4)) as u64) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = ((rbx.wrapping_add(P5)) as u64) as u64; rcx = ((rdi.wrapping_add(P5)) as u64) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm0 = ((crate::simd_ops::ru64(rbx.wrapping_add(P6))) as u64) as u128; crate::simd_ops::wu64(rdi.wrapping_add(P6), ((xmm0 as u64)) as u64); rax = (crate::simd_ops::ru32(rbx.wrapping_add(P7))) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(P7), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rbx.wrapping_add(P8))) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(P8), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rbx.wrapping_add(P9))) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(P9), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rbx.wrapping_add(P10))) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(P10), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rbx.wrapping_add(P11))) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x100u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x104u64))) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x104u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x108u64))) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x108u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x10cu64))) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x10cu64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x110u64))) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x110u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x114u64))) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x114u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x118u64))) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x118u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x11cu64))) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x11cu64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x120u64))) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x120u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x124u64))) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x124u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x128u64))) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x128u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x12cu64))) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x12cu64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x130u64))) as u32 as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x130u64), ((rax as u32)) as u32); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141d934a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x1432f6408; const P1: u64 = 0x10; const P2: u64 = 0x14; const P3: u64 = 0x18; const P4: u64 = 0x13; const P5: u64 = 0x58; const P6: u64 = 0x5c; const P7: u64 = 0x60; const P8: u64 = 0x5b; const P9: u64 = 0xa0; const P10: u64 = 0xa4; const P11: u64 = 0xa8;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = P0;
    crate::simd_ops::wu64(rdx, rax);
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::lk_and_u32(rdx.wrapping_add(P1), (0xff000008u32) as u32);
    crate::simd_ops::lk_or_u32(rdx.wrapping_add(P1), (0x8u32) as u32);
    crate::simd_ops::wu32(rdx.wrapping_add(P2), ((rax as u32)) as u32);
    crate::simd_ops::wu8(rdx.wrapping_add(P3), ((rax as u8)) as u8);
    crate::simd_ops::wu8(rdx.wrapping_add(P4), (0x8u8) as u8);
    crate::simd_ops::lk_and_u32(rdx.wrapping_add(P5), (0xff000008u32) as u32);
    crate::simd_ops::lk_or_u32(rdx.wrapping_add(P5), (0x8u32) as u32);
    crate::simd_ops::wu32(rdx.wrapping_add(P6), ((rax as u32)) as u32);
    crate::simd_ops::wu8(rdx.wrapping_add(P7), ((rax as u8)) as u8);
    crate::simd_ops::wu8(rdx.wrapping_add(P8), (0x8u8) as u8);
    crate::simd_ops::lk_and_u32(rdx.wrapping_add(P9), (0xff000008u32) as u32);
    fl_l = (crate::simd_ops::lk_or_u32(rdx.wrapping_add(P9), (0x8u32) as u32)) as u32 as u64; fl_r = fl_l;
    crate::simd_ops::wu32(rdx.wrapping_add(P10), ((rax as u32)) as u32);
    crate::simd_ops::wu8(rdx.wrapping_add(P11), ((rax as u8)) as u8);
    crate::simd_ops::wu8(rdx.wrapping_add(0xa3u64), (0x8u8) as u8);
    crate::simd_ops::wu64(rdx.wrapping_add(0xe8u64), rax);
    crate::simd_ops::wu32(rdx.wrapping_add(0xf0u64), ((rax as u32)) as u32);
    rax = rdx;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141d94530(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0x18; const P3: u64 = 0x1c; const P4: u64 = 0x20;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    r9 = rcx;
    r8 = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    r8 = ((r8).wrapping_sub(crate::simd_ops::ru64(rdx))) as u64;
    crate::simd_ops::wu64(r8.wrapping_add(rcx).wrapping_add(P0), rax);
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64;
    r8 = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    r8 = ((r8).wrapping_sub(crate::simd_ops::ru64(rdx))) as u64;
    crate::simd_ops::wu64(r8.wrapping_add(rcx).wrapping_add(P1), rax);
    rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(rdx))) as u64;
    rax = (crate::simd_ops::ru32(r9.wrapping_add(P2))) as u32 as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(r9).wrapping_add(P2), ((rax as u32)) as u32);
    rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(rdx))) as u64;
    rax = (crate::simd_ops::ru32(r9.wrapping_add(P3))) as u32 as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(r9).wrapping_add(P3), ((rax as u32)) as u32);
    rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    fl_l = rcx; fl_r = (crate::simd_ops::ru64(rdx)) as u64;
    rcx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64;
    rax = (crate::simd_ops::ru32(r9.wrapping_add(P4))) as u32 as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(r9).wrapping_add(P4), ((rax as u32)) as u32);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141d948a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0x18; const P3: u64 = 0x1c; const P4: u64 = 0x20;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(r8))) as u64;
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    crate::simd_ops::wu64(rcx.wrapping_add(rdx).wrapping_add(P0), rax);
    rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(r8))) as u64;
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(P1))) as u64;
    crate::simd_ops::wu64(rcx.wrapping_add(rdx).wrapping_add(P1), rax);
    rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(r8))) as u64;
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P2))) as u32 as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(rdx).wrapping_add(P2), ((rax as u32)) as u32);
    rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(r8))) as u64;
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P3))) as u32 as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(rdx).wrapping_add(P3), ((rax as u32)) as u32);
    rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    fl_l = rcx; fl_r = (crate::simd_ops::ru64(r8)) as u64;
    rcx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64;
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P4))) as u32 as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(rdx).wrapping_add(P4), ((rax as u32)) as u32);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141d94ab0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0x18; const P3: u64 = 0x1c; const P4: u64 = 0x20;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    crate::simd_ops::wu64(r8.wrapping_add(P0), rax);
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(P1))) as u64;
    crate::simd_ops::wu64(r8.wrapping_add(P1), rax);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P2))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P2), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P3))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P3), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P4))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P4), ((rax as u32)) as u32);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141d952f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0xc; const P2: u64 = 0x10; const P3: u64 = 0x140dfbf30;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P0), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P1))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P1), ((rax as u32)) as u32);
    rax = ((rdx.wrapping_add(P2)) as u64) as u64;
    rcx = ((r8.wrapping_add(P2)) as u64) as u64;
    fl_l = rcx; fl_r = rax;
    if !((((fl_l as u64)) == ((fl_r as u64)))) { rdx = (crate::simd_ops::ru64(rax)) as u64; rax = (crate::simd_ops::ru32(rax.wrapping_add(P0))) as u32 as u64; r8 = ((rax.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; r8 = ((rdx.wrapping_add(r8.wrapping_mul(8))) as u64) as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); }
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141d95b70(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x1432f26a8; const P1: u64 = 0x8; const P2: u64 = 0x10; const P3: u64 = 0x18; const P4: u64 = 0x20; const P5: u64 = 0x28; const P6: u64 = 0x30; const P7: u64 = 0x40; const P8: u64 = 0x48; const P9: u64 = 0x58; const P10: u64 = 0x60; const P11: u64 = 0x68;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rcx = 0;
    fl_l = 0; fl_r = 0;
    rax = P0;
    crate::simd_ops::wu64(rdx.wrapping_add(P1), rcx);
    crate::simd_ops::wu32(rdx.wrapping_add(P2), ((rcx as u32)) as u32);
    crate::simd_ops::wu64(rdx.wrapping_add(P3), rcx);
    crate::simd_ops::wu8(rdx.wrapping_add(P4), ((rcx as u8)) as u8);
    crate::simd_ops::wu64(rdx.wrapping_add(P5), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(P6), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(P7), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(P8), rcx);
    crate::simd_ops::wu64(rdx, rax);
    rax = rdx;
    crate::simd_ops::wu64(rdx.wrapping_add(P9), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(P10), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(P11), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(0x70u64), rcx);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141d962f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x14240aa00; const P2: u64 = 0x38; const P3: u64 = 0x48; const P4: u64 = 0x4c; const P5: u64 = 0x50; const P6: u64 = 0x54; const P7: u64 = 0x58; const P8: u64 = 0x60; const P9: u64 = 0x68; const P10: u64 = 0x70; const P11: u64 = 0x74;
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rdx; rcx = ((r8.wrapping_add(P0)) as u64) as u64; rdx = ((rdx).wrapping_add(0x8u64)) as u64; rdi = r8; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = ((rbx.wrapping_add(P2)) as u64) as u64; rcx = ((rdi.wrapping_add(P2)) as u64) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru32(rbx.wrapping_add(P3))) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(P3), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rbx.wrapping_add(P4))) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(P4), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rbx.wrapping_add(P5))) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(P5), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rbx.wrapping_add(P6))) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(P6), ((rax as u32)) as u32); rax = ((crate::simd_ops::ru8(rbx.wrapping_add(P7))) as u32) as u32 as u64; crate::simd_ops::wu8(rdi.wrapping_add(P7), ((rax as u8)) as u8); rax = (crate::simd_ops::ru64(rbx.wrapping_add(P8))) as u64; crate::simd_ops::wu64(rdi.wrapping_add(P8), rax); xmm0 = ((crate::simd_ops::ru64(rbx.wrapping_add(P9))) as u64) as u128; crate::simd_ops::wu64(rdi.wrapping_add(P9), ((xmm0 as u64)) as u64); rax = (crate::simd_ops::ru32(rbx.wrapping_add(P10))) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(P10), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rbx.wrapping_add(P11))) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x74u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x78u64))) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x78u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x7cu64))) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x7cu64), ((rax as u32)) as u32); rax = ((crate::simd_ops::ru8(rbx.wrapping_add(0x80u64))) as u32) as u32 as u64; crate::simd_ops::wu8(rdi.wrapping_add(0x80u64), ((rax as u8)) as u8); rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x84u64))) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x84u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x88u64))) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x88u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x8cu64))) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x8cu64), ((rax as u32)) as u32); rax = ((crate::simd_ops::ru8(rbx.wrapping_add(0x90u64))) as u32) as u32 as u64; crate::simd_ops::wu8(rdi.wrapping_add(0x90u64), ((rax as u8)) as u8); rax = ((crate::simd_ops::ru8(rbx.wrapping_add(0x91u64))) as u32) as u32 as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu8(rdi.wrapping_add(0x91u64), ((rax as u8)) as u8); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141d963f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x1432f3178; const P2: u64 = 0xb; const P3: u64 = 0xc; const P4: u64 = 0x10; const P5: u64 = 0x38; const P6: u64 = 0x3b; const P7: u64 = 0x3c; const P8: u64 = 0x40; const P9: u64 = 0x48; const P10: u64 = 0x60; const P11: u64 = 0x68;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    crate::simd_ops::lk_and_u32(rdx.wrapping_add(P0), (0xff000005u32) as u32);
    rax = P1;
    crate::simd_ops::lk_or_u32(rdx.wrapping_add(P0), (0x5u32) as u32);
    rcx = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu8(rdx.wrapping_add(P2), (0x5u8) as u8);
    crate::simd_ops::wu32(rdx.wrapping_add(P3), ((rcx as u32)) as u32);
    crate::simd_ops::wu8(rdx.wrapping_add(P4), ((rcx as u8)) as u8);
    crate::simd_ops::lk_and_u32(rdx.wrapping_add(P5), (0xff000001u32) as u32);
    fl_l = (crate::simd_ops::lk_or_u32(rdx.wrapping_add(P5), (0x1u32) as u32)) as u32 as u64; fl_r = fl_l;
    crate::simd_ops::wu8(rdx.wrapping_add(P6), (0x1u8) as u8);
    crate::simd_ops::wu32(rdx.wrapping_add(P7), ((rcx as u32)) as u32);
    crate::simd_ops::wu8(rdx.wrapping_add(P8), ((rcx as u8)) as u8);
    crate::simd_ops::wu64(rdx, rax);
    rax = rdx;
    crate::simd_ops::wu32(rdx.wrapping_add(P9), ((rcx as u32)) as u32);
    crate::simd_ops::wu64(rdx.wrapping_add(P10), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(P11), rcx);
    crate::simd_ops::wu32(rdx.wrapping_add(0x70u64), ((rcx as u32)) as u32);
    crate::simd_ops::wu64(rdx.wrapping_add(0x84u64), rcx);
    crate::simd_ops::wu32(rdx.wrapping_add(0x8cu64), ((rcx as u32)) as u32);
    crate::simd_ops::wu16(rdx.wrapping_add(0x90u64), ((rcx as u16)) as u16);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141d96b10(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x14240aa00; const P2: u64 = 0x38; const P3: u64 = 0x48; const P4: u64 = 0x60; const P5: u64 = 0x4c; const P6: u64 = 0x50; const P7: u64 = 0x54; const P8: u64 = 0x58;
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rdx;
    rcx = ((r8.wrapping_add(P0)) as u64) as u64;
    rdx = ((rdx).wrapping_add(0x8u64)) as u64;
    rdi = r8;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rdx = ((rbx.wrapping_add(P2)) as u64) as u64;
    rcx = ((rdi.wrapping_add(P2)) as u64) as u64;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rax = (crate::simd_ops::ru32(rbx.wrapping_add(P3))) as u32 as u64;
    rdx = ((rbx.wrapping_add(P4)) as u64) as u64;
    crate::simd_ops::wu32(rdi.wrapping_add(P3), ((rax as u32)) as u32);
    rcx = ((rdi.wrapping_add(P4)) as u64) as u64;
    rax = (crate::simd_ops::ru32(rbx.wrapping_add(P5))) as u32 as u64;
    crate::simd_ops::wu32(rdi.wrapping_add(P5), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rbx.wrapping_add(P6))) as u32 as u64;
    crate::simd_ops::wu32(rdi.wrapping_add(P6), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rbx.wrapping_add(P7))) as u32 as u64;
    crate::simd_ops::wu32(rdi.wrapping_add(P7), ((rax as u32)) as u32);
    rax = ((crate::simd_ops::ru8(rbx.wrapping_add(P8))) as u32) as u32 as u64;
    crate::simd_ops::wu8(rdi.wrapping_add(P8), ((rax as u8)) as u8);
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return crate::simd_ops::tail_stk(P1, rax, rcx, rdx, r8, r9, rsp);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141d96bb0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x1432f5e28; const P2: u64 = 0xb; const P3: u64 = 0xc; const P4: u64 = 0x10; const P5: u64 = 0x38; const P6: u64 = 0x3b; const P7: u64 = 0x3c; const P8: u64 = 0x40; const P9: u64 = 0x48; const P10: u64 = 0x60; const P11: u64 = 0x63;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    crate::simd_ops::lk_and_u32(rdx.wrapping_add(P0), (0xff000005u32) as u32);
    rax = P1;
    crate::simd_ops::lk_or_u32(rdx.wrapping_add(P0), (0x5u32) as u32);
    rcx = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu8(rdx.wrapping_add(P2), (0x5u8) as u8);
    crate::simd_ops::wu32(rdx.wrapping_add(P3), ((rcx as u32)) as u32);
    crate::simd_ops::wu8(rdx.wrapping_add(P4), ((rcx as u8)) as u8);
    crate::simd_ops::lk_and_u32(rdx.wrapping_add(P5), (0xff000001u32) as u32);
    crate::simd_ops::lk_or_u32(rdx.wrapping_add(P5), (0x1u32) as u32);
    crate::simd_ops::wu8(rdx.wrapping_add(P6), (0x1u8) as u8);
    crate::simd_ops::wu32(rdx.wrapping_add(P7), ((rcx as u32)) as u32);
    crate::simd_ops::wu8(rdx.wrapping_add(P8), ((rcx as u8)) as u8);
    crate::simd_ops::wu64(rdx, rax);
    rax = rdx;
    crate::simd_ops::wu32(rdx.wrapping_add(P9), ((rcx as u32)) as u32);
    crate::simd_ops::lk_and_u32(rdx.wrapping_add(P10), (0xff000004u32) as u32);
    fl_l = (crate::simd_ops::lk_or_u32(rdx.wrapping_add(P10), (0x4u32) as u32)) as u32 as u64; fl_r = fl_l;
    crate::simd_ops::wu8(rdx.wrapping_add(P11), (0x4u8) as u8);
    crate::simd_ops::wu32(rdx.wrapping_add(0x64u64), ((rcx as u32)) as u32);
    crate::simd_ops::wu8(rdx.wrapping_add(0x68u64), ((rcx as u8)) as u8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141d96ec0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143251128; const P1: u64 = 0x1; const P2: u64 = 0x14020f270;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x38u64)) as u64;
    rcx = 0;
    fl_l = 0; fl_r = 0;
    r9 = P0;
    rax = r8;
    crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rcx);
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rcx);
    rdx = ((rdx).wrapping_add(0x64u64)) as u64;
    r8 = ((rcx.wrapping_add(P1)) as u32) as u32 as u64;
    rcx = rax;
    rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x38u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141d97320(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x14240aa00; const P2: u64 = 0x38; const P3: u64 = 0x48; const P4: u64 = 0x68; const P5: u64 = 0x4c; const P6: u64 = 0x50; const P7: u64 = 0x54; const P8: u64 = 0x58; const P9: u64 = 0x60; const P10: u64 = 0x64; const P11: u64 = 0x1408265b0;
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rdx;
    rcx = ((r8.wrapping_add(P0)) as u64) as u64;
    rdx = ((rdx).wrapping_add(0x8u64)) as u64;
    rdi = r8;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rdx = ((rbx.wrapping_add(P2)) as u64) as u64;
    rcx = ((rdi.wrapping_add(P2)) as u64) as u64;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rax = (crate::simd_ops::ru32(rbx.wrapping_add(P3))) as u32 as u64;
    rdx = ((rbx.wrapping_add(P4)) as u64) as u64;
    crate::simd_ops::wu32(rdi.wrapping_add(P3), ((rax as u32)) as u32);
    rcx = ((rdi.wrapping_add(P4)) as u64) as u64;
    rax = (crate::simd_ops::ru32(rbx.wrapping_add(P5))) as u32 as u64;
    crate::simd_ops::wu32(rdi.wrapping_add(P5), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rbx.wrapping_add(P6))) as u32 as u64;
    crate::simd_ops::wu32(rdi.wrapping_add(P6), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rbx.wrapping_add(P7))) as u32 as u64;
    crate::simd_ops::wu32(rdi.wrapping_add(P7), ((rax as u32)) as u32);
    rax = ((crate::simd_ops::ru8(rbx.wrapping_add(P8))) as u32) as u32 as u64;
    crate::simd_ops::wu8(rdi.wrapping_add(P8), ((rax as u8)) as u8);
    rax = (crate::simd_ops::ru32(rbx.wrapping_add(P9))) as u32 as u64;
    crate::simd_ops::wu32(rdi.wrapping_add(P9), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rbx.wrapping_add(P10))) as u32 as u64;
    crate::simd_ops::wu32(rdi.wrapping_add(P10), ((rax as u32)) as u32);
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return crate::simd_ops::tail_stk(P11, rax, rcx, rdx, r8, r9, rsp);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141d973d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x1432efd88; const P2: u64 = 0xb; const P3: u64 = 0xc; const P4: u64 = 0x10; const P5: u64 = 0x38; const P6: u64 = 0x3b; const P7: u64 = 0x3c; const P8: u64 = 0x40; const P9: u64 = 0x48; const P10: u64 = 0x68; const P11: u64 = 0x70;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    crate::simd_ops::lk_and_u32(rdx.wrapping_add(P0), (0xff000005u32) as u32);
    rax = P1;
    crate::simd_ops::lk_or_u32(rdx.wrapping_add(P0), (0x5u32) as u32);
    rcx = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu8(rdx.wrapping_add(P2), (0x5u8) as u8);
    crate::simd_ops::wu32(rdx.wrapping_add(P3), ((rcx as u32)) as u32);
    crate::simd_ops::wu8(rdx.wrapping_add(P4), ((rcx as u8)) as u8);
    crate::simd_ops::lk_and_u32(rdx.wrapping_add(P5), (0xff000001u32) as u32);
    fl_l = (crate::simd_ops::lk_or_u32(rdx.wrapping_add(P5), (0x1u32) as u32)) as u32 as u64; fl_r = fl_l;
    crate::simd_ops::wu8(rdx.wrapping_add(P6), (0x1u8) as u8);
    crate::simd_ops::wu32(rdx.wrapping_add(P7), ((rcx as u32)) as u32);
    crate::simd_ops::wu8(rdx.wrapping_add(P8), ((rcx as u8)) as u8);
    crate::simd_ops::wu32(rdx.wrapping_add(P9), ((rcx as u32)) as u32);
    crate::simd_ops::wu64(rdx, rax);
    rax = rdx;
    crate::simd_ops::wu64(rdx.wrapping_add(P10), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(P11), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(0x78u64), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(0x80u64), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(0x88u64), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(0x90u64), rcx);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141d97590(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1432f89d0; const P1: u64 = 0x84; const P2: u64 = 0x1426cdcb0; const P3: u64 = 0x14020f270;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rsp = ((rsp).wrapping_sub(0x48u64)) as u64;
    rax = rdx;
    r9 = P0;
    rdx = ((rcx.wrapping_add(P1)) as u64) as u64;
    r8 = 0x4;
    rcx = P2;
    xmm0 = 0u128;
    crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), ((xmm0 as u64)) as u64);
    crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rcx);
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rcx);
    rcx = rax;
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    rsp = ((rsp).wrapping_add(0x48u64)) as u64;
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141d97710(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1426cdcb0; const P1: u64 = 0x1432f89d0; const P2: u64 = 0x14020f270;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rsp = ((rsp).wrapping_sub(0x48u64)) as u64;
    rcx = P0;
    rax = r8;
    xmm0 = 0u128;
    crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), ((xmm0 as u64)) as u64);
    crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rcx);
    r9 = P1;
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rcx);
    rdx = ((rdx).wrapping_add(0x84u64)) as u64;
    rcx = rax;
    r8 = 0x4;
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    rsp = ((rsp).wrapping_add(0x48u64)) as u64;
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141d97b90(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x14240aa00; const P2: u64 = 0x38; const P3: u64 = 0x48; const P4: u64 = 0x4c; const P5: u64 = 0x50; const P6: u64 = 0x54; const P7: u64 = 0x58; const P8: u64 = 0x60; const P9: u64 = 0x68; const P10: u64 = 0x70; const P11: u64 = 0x74;
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rdx; rcx = ((r8.wrapping_add(P0)) as u64) as u64; rdx = ((rdx).wrapping_add(0x8u64)) as u64; rdi = r8; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rdx = ((rbx.wrapping_add(P2)) as u64) as u64; rcx = ((rdi.wrapping_add(P2)) as u64) as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru32(rbx.wrapping_add(P3))) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(P3), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rbx.wrapping_add(P4))) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(P4), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rbx.wrapping_add(P5))) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(P5), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rbx.wrapping_add(P6))) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(P6), ((rax as u32)) as u32); rax = ((crate::simd_ops::ru8(rbx.wrapping_add(P7))) as u32) as u32 as u64; crate::simd_ops::wu8(rdi.wrapping_add(P7), ((rax as u8)) as u8); rax = (crate::simd_ops::ru64(rbx.wrapping_add(P8))) as u64; crate::simd_ops::wu64(rdi.wrapping_add(P8), rax); rax = (crate::simd_ops::ru64(rbx.wrapping_add(P9))) as u64; crate::simd_ops::wu64(rdi.wrapping_add(P9), rax); rax = (crate::simd_ops::ru32(rbx.wrapping_add(P10))) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(P10), ((rax as u32)) as u32); rax = ((crate::simd_ops::ru8(rbx.wrapping_add(P11))) as u32) as u32 as u64; crate::simd_ops::wu8(rdi.wrapping_add(0x74u64), ((rax as u8)) as u8); rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x78u64))) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x78u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x7cu64))) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x7cu64), ((rax as u32)) as u32); rax = ((crate::simd_ops::ru8(rbx.wrapping_add(0x80u64))) as u32) as u32 as u64; crate::simd_ops::wu8(rdi.wrapping_add(0x80u64), ((rax as u8)) as u8); rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x84u64))) as u32 as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x84u64), ((rax as u32)) as u32); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141d97c60(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x1432ef748; const P2: u64 = 0xb; const P3: u64 = 0xc; const P4: u64 = 0x10; const P5: u64 = 0x38; const P6: u64 = 0x3b; const P7: u64 = 0x3c; const P8: u64 = 0x40; const P9: u64 = 0x48; const P10: u64 = 0x60; const P11: u64 = 0x68;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    crate::simd_ops::lk_and_u32(rdx.wrapping_add(P0), (0xff000005u32) as u32);
    rax = P1;
    crate::simd_ops::lk_or_u32(rdx.wrapping_add(P0), (0x5u32) as u32);
    rcx = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu8(rdx.wrapping_add(P2), (0x5u8) as u8);
    crate::simd_ops::wu32(rdx.wrapping_add(P3), ((rcx as u32)) as u32);
    crate::simd_ops::wu8(rdx.wrapping_add(P4), ((rcx as u8)) as u8);
    crate::simd_ops::lk_and_u32(rdx.wrapping_add(P5), (0xff000001u32) as u32);
    fl_l = (crate::simd_ops::lk_or_u32(rdx.wrapping_add(P5), (0x1u32) as u32)) as u32 as u64; fl_r = fl_l;
    crate::simd_ops::wu8(rdx.wrapping_add(P6), (0x1u8) as u8);
    crate::simd_ops::wu32(rdx.wrapping_add(P7), ((rcx as u32)) as u32);
    crate::simd_ops::wu8(rdx.wrapping_add(P8), ((rcx as u8)) as u8);
    crate::simd_ops::wu64(rdx, rax);
    rax = rdx;
    crate::simd_ops::wu32(rdx.wrapping_add(P9), ((rcx as u32)) as u32);
    crate::simd_ops::wu64(rdx.wrapping_add(P10), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(P11), rcx);
    crate::simd_ops::wu32(rdx.wrapping_add(0x7cu64), ((rcx as u32)) as u32);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141d97df0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143a082e8; const P1: u64 = 0x140e48750; const P2: u64 = 0x1420eaf00; const P3: u64 = 0x1420ea8d0; const P4: u64 = 0x141d97f40; const P5: u64 = 0x140e48670; const P6: u64 = 0x1433df968; const P7: u64 = 0x1423ef2b0; const P8: u64 = 0x1426469d0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (crate::simd_ops::ru64(P0)) as u64; rdi = rcx.wrapping_add(0xa0); rbx = rcx; rsi = (crate::simd_ops::ru32(rax.wrapping_add(0x10u64))) as u32 as u64; fl_l = ((rsi as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rcx.wrapping_add(0x9cu64))) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x8u64))) as u32 as u64; rdx = (crate::simd_ops::ru64(rdi)) as u64; r8 = (((rax) as i64).wrapping_mul((0x128u64) as i64) as u64) as u64; r8 = ((r8).wrapping_add(rdx)) as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu32(rdi.wrapping_add(0x8u64), (0x0u32) as u32); crate::simd_ops::wu32(rbx.wrapping_add(0x9cu64), ((rsi as u32)) as u32); bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x8u64))) as u32 as u64; rcx = (((rax) as i64).wrapping_mul((0x128u64) as i64) as u64) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rdi))) as u64; fl_l = (crate::simd_ops::ru64(rdi)) as u64; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 12; continue; } bb = 3; continue; }
            3 => { rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rcx = ((crate::simd_ops::ru8(rbx.wrapping_add(0x3u64))) as u32) as u32 as u64; rdx = rbx.wrapping_add(0x8); r8 = rax; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x80; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 5; continue; } bb = 4; continue; }
            4 => { rcx = ((rcx.wrapping_mul(8)) as u32) as u32 as u64; bb = 6; continue; }
            5 => { rcx = (((rcx as u32)).wrapping_shl(8u32)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x7c00; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru32(rbx)) as u32 as u64; rax = ((((rax as u32)) & (0xffffffu32))) as u32 as u64; rax = (((rax as u32)).wrapping_shl(3u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 8; continue; } bb = 7; continue; }
            7 => { rdx = (crate::simd_ops::ru64(rdx)) as u64; bb = 8; continue; }
            8 => { rcx = r8; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rdx = rdi; rcx = rax; rbx = rax; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 12; continue; } bb = 9; continue; }
            9 => { rcx = rdi; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), r14); rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); fl_l = rbx; fl_r = rbx; rsi = P6; r14 = rax; rsi = (if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { rbx } else { rsi }) as u64; rbx = (0xffffffffffffffffu64) as u64; bb = 10; continue; }
            10 => { rbx = ((rbx).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru16(rsi.wrapping_add(rbx.wrapping_mul(2)))) as u16 as u64; fl_r = (0x0u16) as u16 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { bb = 10; continue; } bb = 11; continue; }
            11 => { rdx = (crate::simd_ops::ru32(r14.wrapping_add(0x10u64))) as u32 as u64; rcx = r14; rax = ((rbx as u32)) as u32 as u64; fl_l = rax; fl_r = rdx; rdx = (if (((fl_l as u64)) >= ((fl_r as u64))) { (rax as u32) } else { (rdx as u32) }) as u32 as u64; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); rdx = (crate::simd_ops::ru32(r14.wrapping_add(0xcu64))) as u32 as u64; r8 = rsi; rcx = (crate::simd_ops::ru64(r14)) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P8), rax), rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu32(r14.wrapping_add(0x10u64), ((rbx as u32)) as u32); crate::simd_ops::wu32(r14.wrapping_add(0x120u64), (0x0u32) as u32); crate::simd_ops::wu32(r14.wrapping_add(0x124u64), (0x7f7fffffu32) as u32); r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; bb = 12; continue; }
            12 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rax = rdi; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141d97f40(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x140e485b0; const P1: u64 = 0x1432f9bd8; const P2: u64 = 0x1426469d8; const P3: u64 = 0x1432f9bdc; const P4: u64 = 0x142442de0; const P5: u64 = 0x1423ef3d0; const P6: u64 = 0x1426469c8; const P7: u64 = 0x1425371a0; const P8: u64 = 0x142533200; const P9: u64 = 0x1423ef2b0; const P10: u64 = 0x1426469d0; const P11: u64 = 0x1426467c0; const P12: u64 = 0x1426464e0; const P13: u64 = 0x1432f9bd0; const P14: u64 = 0x1423f0110; const P15: u64 = 0x1423f0370; const P16: u64 = 0x140e48670; const P17: u64 = 0x1432f9bd4; const P18: u64 = 0x1423f01c0; const P19: u64 = 0x1423ef7d0;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut xmm6: u128 = 0; let mut xmm7: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<512, 229>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = (rsp) as u64; crate::simd_ops::wu64(rax.wrapping_add(0x8u64), (rbx) as u64); crate::simd_ops::wu64(rax.wrapping_add(0x10u64), (rdx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r12) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r13) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r15) as u64); rbp = ((rax.wrapping_sub(0x628u64)) as u64) as u64; rsp = ((rsp).wrapping_sub(0x6f0u64)) as u64; crate::simd_ops::wu128(rax.wrapping_sub(0x48u64), (xmm6) as u128); crate::simd_ops::wu128(rax.wrapping_sub(0x58u64), (xmm7) as u128); rdi = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rbp.wrapping_add(0x648u64), ((rdi as u32)) as u32); rdx = (rcx) as u64; rcx = ((rbp.wrapping_add(0x4b0u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; r13 = ((rdi as u32)) as u32 as u64; bb = 1; continue; }
            1 => { r15 = ((P1) as u64) as u64; rcx = (crate::simd_ops::ru32(rbp.wrapping_add(0x4c0u64))) as u32 as u64; fl_l = (r13) as u64 as u64; fl_r = (rcx) as u64 as u64; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 20; continue; } bb = 2; continue; }
            2 => { rax = ((rcx.wrapping_sub(0x1u64)) as u32) as u32 as u64; fl_l = ((r13 as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) > (((fl_r as u32)) as i32)) { bb = 20; continue; } bb = 3; continue; }
            3 => { rax = ((((r13 as u32)) as i32 as i64) as u64) as u64; r14 = ((rax.wrapping_add(rax)) as u64) as u64; fl_l = (r14) as u64 as u64; fl_r = (r15) as u64 as u64; r14 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; bb = 4; continue; }
            4 => { rsi = ((r14.wrapping_add(r15)) as u64) as u64; bb = 5; continue; }
            5 => { rax = ((rsi.wrapping_add(rdi.wrapping_mul(2))) as u64) as u64; rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x4b0u64))) as u64; rcx = ((crate::simd_ops::ru16(rax.wrapping_add(rcx))) as u32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P2)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rbx = (((rax as u16)) as u32) as u32 as u64; rcx = ((crate::simd_ops::ru16(r15.wrapping_add(rdi.wrapping_mul(2)))) as u32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P2)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; fl_l = ((rbx as u16)) as u16 as u64; fl_r = ((rax as u16)) as u16 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { bb = 8; continue; } bb = 6; continue; }
            6 => { fl_l = (rdi) as u64 as u64; fl_r = (rdi) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 7; continue; }
            7 => { rdi = ((rdi).wrapping_add(1)) as u64; fl_l = (rdi) as u64 as u64; fl_r = (0x1u64) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 5; continue; } bb = 8; continue; }
            8 => { r13 = (((r13 as u32)).wrapping_add(1)) as u32 as u64; r14 = ((r14).wrapping_add(0x2u64)) as u64; rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x4c0u64))) as u32 as u64; rax = (((rax as u32)).wrapping_sub(1)) as u32 as u64; rdi = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; fl_l = ((r13 as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) > (((fl_r as u32)) as i32)) { bb = 20; continue; } bb = 9; continue; }
            9 => { bb = 4; continue; }
            10 => { r12 = ((((r13 as u32)) as i32 as i64) as u64) as u64; fl_l = (r12) as u64 as u64; fl_r = (0xffffffffffffffffu64) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 19; continue; } bb = 11; continue; }
            11 => { rcx = (crate::simd_ops::ru32(rbp.wrapping_add(0x4c0u64))) as u32 as u64; fl_l = (r12) as u64 as u64; fl_r = (rcx) as u64 as u64; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 19; continue; } bb = 12; continue; }
            12 => { rax = ((rcx.wrapping_sub(0x1u64)) as u32) as u32 as u64; fl_l = ((r13 as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) > (((fl_r as u32)) as i32)) { bb = 19; continue; } bb = 13; continue; }
            13 => { rax = ((((r13 as u32)) as i32 as i64) as u64) as u64; r14 = ((rax.wrapping_add(rax)) as u64) as u64; r15 = ((P3) as u64) as u64; fl_l = (r14) as u64 as u64; fl_r = (r15) as u64 as u64; r14 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; bb = 14; continue; }
            14 => { rax = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rdi = ((rax as u32)) as u32 as u64; rsi = ((r14.wrapping_add(r15)) as u64) as u64; bb = 15; continue; }
            15 => { rax = ((rsi.wrapping_add(rdi.wrapping_mul(2))) as u64) as u64; rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x4b0u64))) as u64; rcx = ((crate::simd_ops::ru16(rax.wrapping_add(rcx))) as u32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P2)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rbx = (((rax as u16)) as u32) as u32 as u64; rcx = ((crate::simd_ops::ru16(r15.wrapping_add(rdi.wrapping_mul(2)))) as u32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P2)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; fl_l = ((rbx as u16)) as u16 as u64; fl_r = ((rax as u16)) as u16 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { bb = 18; continue; } bb = 16; continue; }
            16 => { fl_l = (rdi) as u64 as u64; fl_r = (rdi) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 27; continue; } bb = 17; continue; }
            17 => { rdi = ((rdi).wrapping_add(1)) as u64; fl_l = (rdi) as u64 as u64; fl_r = (0x1u64) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 15; continue; } bb = 18; continue; }
            18 => { r13 = (((r13 as u32)).wrapping_add(1)) as u32 as u64; r14 = ((r14).wrapping_add(0x2u64)) as u64; rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x4c0u64))) as u32 as u64; rax = (((rax as u32)).wrapping_sub(1)) as u32 as u64; fl_l = ((r13 as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) <= (((fl_r as u32)) as i32)) { bb = 14; continue; } bb = 19; continue; }
            19 => { rdi = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; bb = 20; continue; }
            20 => { rbx = (rbx & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 21; continue; }
            21 => { rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x4b0u64))) as u64; rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x4c8u64))) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 24; continue; } bb = 22; continue; }
            22 => { fl_l = (rcx) as u64 as u64; fl_r = (rax) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 24; continue; } bb = 23; continue; }
            23 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x4c8u64))) as u64; bb = 24; continue; }
            24 => { fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 26; continue; } bb = 25; continue; }
            25 => { crate::simd_ops::wu16(rax, ((rdi as u16)) as u16); bb = 26; continue; }
            26 => { rax = (((rbx as u8)) as u32) as u32 as u64; r11 = ((rsp.wrapping_add(0x6f0u64)) as u64) as u64; rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x40u64))) as u64; xmm6 = (crate::simd_ops::ru128(r11.wrapping_sub(0x10u64))) as u128; xmm7 = (crate::simd_ops::ru128(r11.wrapping_sub(0x20u64))) as u128; rsp = (r11) as u64; r15 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r13 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r12 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rsi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rbp = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            27 => { rbx = ((((r13 as u32)) as i32 as i64) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x640u64), (rbx) as u64); fl_l = (rbx) as u64 as u64; fl_r = (0xffffffffffffffffu64) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 19; continue; } bb = 28; continue; }
            28 => { r15 = ((r12.wrapping_add(0x1u64)) as u64) as u64; rsi = ((rsp.wrapping_add(0x50u64)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (rsi) as u64); rdx = (0x100u32) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x38u64), ((rdx as u32)) as u32); rdi = ((rdx as u32)) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x3cu64), (rdx) as u64); r9 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; r14 = ((r9 as u32)) as u32 as u64; rcx = ((rsp.wrapping_add(0x50u64)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), (rcx) as u64); crate::simd_ops::wu16(rsp.wrapping_add(0x50u64), ((r9 as u16)) as u16); crate::simd_ops::lk_or_u32(rbp.wrapping_add(0x648u64), (0x1u32) as u32); r8 = (rbx) as u64; r8 = ((r8).wrapping_sub(r12)) as u64; r8 = ((r8).wrapping_sub(1)) as u64; rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x4c0u64))) as u32 as u64; fl_l = (r15) as u64 as u64; fl_r = (rax) as u64 as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 42; continue; } bb = 29; continue; }
            29 => { rax = ((rax).wrapping_sub(r15)) as u64; fl_l = (r8) as u64 as u64; fl_r = (rax) as u64 as u64; rax = (if (((fl_l as u64)) <= ((fl_r as u64))) { r8 } else { rax }) as u64; r14 = (rax) as u64; rcx = ((rax.wrapping_add(0x1u64)) as u64) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rdx) as u64 as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 41; continue; } bb = 30; continue; }
            30 => { rax = ((rcx as u32)) as u32 as u64; rax = ((((rax as u32)) & (0xfffffff0u32))) as u32 as u64; fl_l = ((rcx as u8)) as u8 as u64; fl_r = (0xfu8) as u8 as u64; rdi = ((rax.wrapping_add(0x10u64)) as u32) as u32 as u64; rdi = (if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { (rax as u32) } else { (rdi as u32) }) as u32 as u64; rax = (0x180u32) as u32 as u64; fl_l = ((rdi as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; rdi = (if (((fl_l as u32)) < ((fl_r as u32))) { (rax as u32) } else { (rdi as u32) }) as u32 as u64; r9 = ((rdi as u32)) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rsi = (rax) as u64; r12 = ((rdi.wrapping_sub(0x1u64)) as u32) as u32 as u64; r9 = ((r12 as u32)) as u32 as u64; r8 = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rdx = ((rdi as u32)) as u32 as u64; rcx = (rax) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P6)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; r10 = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; fl_l = (r10) as u64 as u64; fl_r = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 39; continue; } bb = 31; continue; }
            31 => { fl_l = (r10) as u64 as u64; fl_r = (r10) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 39; continue; } bb = 32; continue; }
            32 => { r10 = ((r10).wrapping_sub(1)) as u64; r10 = (((r10) & (0xfffffffffe000000u64))) as u64; rax = ((crate::exe::teb_self() as u64)) as u64; fl_l = (rax) as u64 as u64; fl_r = (crate::simd_ops::ru64(r10.wrapping_add(0x100u64))) as u64 as u64; rcx = (rcx & !0xffu64) | (((if (((fl_l as u64)) == ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); r9 = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = (r9) as u64; rax = ((rax).wrapping_sub(r10)) as u64; rax = ((rax).wrapping_shr(16u32)) as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rdx = ((r10.wrapping_add(0x108u64)) as u64) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x4u64))) as u32 as u64; rdx = ((rdx).wrapping_sub(rax)) as u64; fl_l = ((rcx as u8)) as u8 as u64; fl_r = ((rcx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 37; continue; } bb = 33; continue; }
            33 => { fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(0xeu64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 36; continue; } bb = 34; continue; }
            34 => { rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(r9, (rax) as u64); crate::simd_ops::wu64(rdx.wrapping_add(0x20u64), (r9) as u64); fl_l = (crate::simd_ops::lk_add_u32(rdx.wrapping_add(0x18u64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; if (((fl_l as u32)) != 0) { bb = 39; continue; } bb = 35; continue; }
            35 => { rcx = (rdx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P7) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 39; continue; }
            36 => { r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 38; continue; }
            37 => { r8 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; bb = 38; continue; }
            38 => { rcx = (r10) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P8) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 39; continue; }
            39 => { crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (rsi) as u64); crate::simd_ops::wu32(rsp.wrapping_add(0x3cu64), ((rdi as u32)) as u32); rbx = (crate::simd_ops::ru64(rbp.wrapping_add(0x640u64))) as u64; fl_l = ((rdi as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rsp.wrapping_add(0x40u64))) as u32 as u64; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 41; continue; } bb = 40; continue; }
            40 => { crate::simd_ops::wu32(rsp.wrapping_add(0x40u64), ((r12 as u32)) as u32); rcx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; crate::simd_ops::wu16(rsi.wrapping_add(r12.wrapping_mul(2)), ((rcx as u16)) as u16); rdi = (crate::simd_ops::ru32(rsp.wrapping_add(0x3cu64))) as u32 as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; bb = 41; continue; }
            41 => { rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x4b0u64))) as u64; r8 = ((rax.wrapping_add(r15.wrapping_mul(2))) as u64) as u64; rdx = ((rdi as u32)) as u32 as u64; r9 = (r14) as u64; rcx = (rsi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P6)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu32(rsp.wrapping_add(0x40u64), ((r14 as u32)) as u32); rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rdi = (crate::simd_ops::ru32(rsp.wrapping_add(0x3cu64))) as u32 as u64; rdx = (crate::simd_ops::ru32(rsp.wrapping_add(0x38u64))) as u32 as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; r9 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; bb = 42; continue; }
            42 => { rax = ((rbp.wrapping_add(0x3b0u64)) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x390u64), (rax) as u64); crate::simd_ops::wu32(rbp.wrapping_add(0x398u64), (0x80u32) as u32); crate::simd_ops::wu64(rbp.wrapping_add(0x39cu64), (0x80u64) as u64); rax = ((rbp.wrapping_add(0x3b0u64)) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x3a8u64), (rax) as u64); crate::simd_ops::wu16(rbp.wrapping_add(0x3b0u64), ((r9 as u16)) as u16); fl_l = ((r14 as u32)) as u32 as u64; fl_r = (0x80u32) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 45; continue; } bb = 43; continue; }
            43 => { fl_l = (rcx) as u64 as u64; fl_r = (rsi) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 45; continue; } bb = 44; continue; }
            44 => { crate::simd_ops::wu64(rbp.wrapping_add(0x390u64), (rsi) as u64); crate::simd_ops::wu32(rbp.wrapping_add(0x39cu64), ((rdi as u32)) as u32); crate::simd_ops::wu32(rbp.wrapping_add(0x3a0u64), ((r14 as u32)) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (rcx) as u64); crate::simd_ops::wu32(rsp.wrapping_add(0x3cu64), ((rdx as u32)) as u32); rdi = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rsp.wrapping_add(0x40u64), ((rdi as u32)) as u32); crate::simd_ops::wu16(rcx, ((rdi as u16)) as u16); bb = 46; continue; }
            45 => { rdx = ((r14 as u32)) as u32 as u64; rcx = ((rbp.wrapping_add(0x390u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P9) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdx = (crate::simd_ops::ru32(rbp.wrapping_add(0x39cu64))) as u32 as u64; r8 = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x390u64))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P10)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x40u64))) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x3a0u64), ((rax as u32)) as u32); rdi = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; bb = 46; continue; }
            46 => { crate::simd_ops::lk_and_u32(rbp.wrapping_add(0x648u64), (0xfffffffeu32) as u32); r10 = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; fl_l = (r10) as u64 as u64; fl_r = (r10) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 55; continue; } bb = 47; continue; }
            47 => { fl_l = (r10) as u64 as u64; fl_r = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 55; continue; } bb = 48; continue; }
            48 => { r10 = ((r10).wrapping_sub(1)) as u64; r10 = (((r10) & (0xfffffffffe000000u64))) as u64; rax = ((crate::exe::teb_self() as u64)) as u64; fl_l = (rax) as u64 as u64; fl_r = (crate::simd_ops::ru64(r10.wrapping_add(0x100u64))) as u64 as u64; rcx = (rcx & !0xffu64) | (((if (((fl_l as u64)) == ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); r9 = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = (r9) as u64; rax = ((rax).wrapping_sub(r10)) as u64; rax = ((rax).wrapping_shr(16u32)) as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rdx = ((r10.wrapping_add(0x108u64)) as u64) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x4u64))) as u32 as u64; rdx = ((rdx).wrapping_sub(rax)) as u64; fl_l = ((rcx as u8)) as u8 as u64; fl_r = ((rcx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 53; continue; } bb = 49; continue; }
            49 => { fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(0xeu64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 52; continue; } bb = 50; continue; }
            50 => { rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(r9, (rax) as u64); crate::simd_ops::wu64(rdx.wrapping_add(0x20u64), (r9) as u64); fl_l = (crate::simd_ops::lk_add_u32(rdx.wrapping_add(0x18u64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; if (((fl_l as u32)) != 0) { bb = 55; continue; } bb = 51; continue; }
            51 => { rcx = (rdx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P7) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 55; continue; }
            52 => { r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 54; continue; }
            53 => { r8 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; bb = 54; continue; }
            54 => { rcx = (r10) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P8) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 55; continue; }
            55 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 57; continue; } bb = 56; continue; }
            56 => { crate::simd_ops::wu16(rax, ((rdi as u16)) as u16); bb = 57; continue; }
            57 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P11)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu32(rax, ((rdi as u32)) as u32); rdx = ((rsp.wrapping_add(0x20u64)) as u64) as u64; rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x390u64))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P12)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; xmm7 = (xmm0 & !0xffff_ffffu128) | (((f64::from_bits(xmm0 as u64)) as f32).to_bits() as u128); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P11)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; fl_l = (crate::simd_ops::ru32(rax)) as u32 as u64; fl_r = (0x0u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 191; continue; } bb = 58; continue; }
            58 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; fl_l = (crate::simd_ops::ru16(rax)) as u16 as u64; fl_r = (0x0u16) as u16 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { bb = 191; continue; } bb = 59; continue; }
            59 => { rcx = (crate::simd_ops::ru32(rbp.wrapping_add(0x4c0u64))) as u32 as u64; fl_l = (rbx) as u64 as u64; fl_r = (rcx) as u64 as u64; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 191; continue; } bb = 60; continue; }
            60 => { rax = ((rcx.wrapping_sub(0x1u64)) as u32) as u32 as u64; fl_l = ((r13 as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) > (((fl_r as u32)) as i32)) { bb = 191; continue; } bb = 61; continue; }
            61 => { rax = ((((r13 as u32)) as i32 as i64) as u64) as u64; r14 = ((rax.wrapping_add(rax)) as u64) as u64; r15 = ((P13) as u64) as u64; fl_l = (r14) as u64 as u64; fl_r = (r15) as u64 as u64; r14 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; bb = 62; continue; }
            62 => { rsi = ((r14.wrapping_add(r15)) as u64) as u64; bb = 63; continue; }
            63 => { rax = ((rsi.wrapping_add(rdi.wrapping_mul(2))) as u64) as u64; rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x4b0u64))) as u64; rcx = ((crate::simd_ops::ru16(rax.wrapping_add(rcx))) as u32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P2)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rbx = (((rax as u16)) as u32) as u32 as u64; rcx = ((crate::simd_ops::ru16(r15.wrapping_add(rdi.wrapping_mul(2)))) as u32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P2)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; fl_l = ((rbx as u16)) as u16 as u64; fl_r = ((rax as u16)) as u16 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { bb = 66; continue; } bb = 64; continue; }
            64 => { fl_l = (rdi) as u64 as u64; fl_r = (rdi) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 68; continue; } bb = 65; continue; }
            65 => { rdi = ((rdi).wrapping_add(1)) as u64; fl_l = (rdi) as u64 as u64; fl_r = (0x1u64) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 63; continue; } bb = 66; continue; }
            66 => { r13 = (((r13 as u32)).wrapping_add(1)) as u32 as u64; r14 = ((r14).wrapping_add(0x2u64)) as u64; rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x4c0u64))) as u32 as u64; rax = (((rax as u32)).wrapping_sub(1)) as u32 as u64; rdi = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; fl_l = ((r13 as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) > (((fl_r as u32)) as i32)) { bb = 191; continue; } bb = 67; continue; }
            67 => { bb = 62; continue; }
            68 => { r12 = ((((r13 as u32)) as i32 as i64) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (r12) as u64); fl_l = (r12) as u64 as u64; fl_r = (0xffffffffffffffffu64) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 190; continue; } bb = 69; continue; }
            69 => { r8 = (crate::simd_ops::ru64(rbp.wrapping_add(0x640u64))) as u64; r15 = ((r8.wrapping_add(0x1u64)) as u64) as u64; rdi = ((rsp.wrapping_add(0x50u64)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (rdi) as u64); rdx = (0x100u32) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x38u64), ((rdx as u32)) as u32); rsi = ((rdx as u32)) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x3cu64), (rdx) as u64); rbx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; r14 = ((rbx as u32)) as u32 as u64; rcx = ((rsp.wrapping_add(0x50u64)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), (rcx) as u64); crate::simd_ops::wu16(rsp.wrapping_add(0x50u64), ((rbx as u16)) as u16); crate::simd_ops::lk_or_u32(rbp.wrapping_add(0x648u64), (0x2u32) as u32); rax = (r12) as u64; rax = ((rax).wrapping_sub(r8)) as u64; r8 = ((rax.wrapping_sub(0x1u64)) as u64) as u64; rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x4c0u64))) as u32 as u64; fl_l = (r15) as u64 as u64; fl_r = (rax) as u64 as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 84; continue; } bb = 70; continue; }
            70 => { rax = ((rax).wrapping_sub(r15)) as u64; fl_l = (r8) as u64 as u64; fl_r = (rax) as u64 as u64; rax = (if (((fl_l as u64)) <= ((fl_r as u64))) { r8 } else { rax }) as u64; r14 = (rax) as u64; rcx = ((rax.wrapping_add(0x1u64)) as u64) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rdx) as u64 as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 83; continue; } bb = 71; continue; }
            71 => { rax = ((rcx as u32)) as u32 as u64; rax = ((((rax as u32)) & (0xfffffff0u32))) as u32 as u64; fl_l = ((rcx as u8)) as u8 as u64; fl_r = (0xfu8) as u8 as u64; rsi = ((rax.wrapping_add(0x10u64)) as u32) as u32 as u64; rsi = (if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { (rax as u32) } else { (rsi as u32) }) as u32 as u64; rax = (0x180u32) as u32 as u64; fl_l = ((rsi as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; rsi = (if (((fl_l as u32)) < ((fl_r as u32))) { (rax as u32) } else { (rsi as u32) }) as u32 as u64; r9 = ((rsi as u32)) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdi = (rax) as u64; r12 = ((rsi.wrapping_sub(0x1u64)) as u32) as u32 as u64; r9 = ((r12 as u32)) as u32 as u64; r8 = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rdx = ((rsi as u32)) as u32 as u64; rcx = (rax) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P6)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; r10 = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; fl_l = (r10) as u64 as u64; fl_r = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 80; continue; } bb = 72; continue; }
            72 => { fl_l = (r10) as u64 as u64; fl_r = (r10) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 80; continue; } bb = 73; continue; }
            73 => { r10 = ((r10).wrapping_sub(1)) as u64; r10 = (((r10) & (0xfffffffffe000000u64))) as u64; rax = ((crate::exe::teb_self() as u64)) as u64; fl_l = (rax) as u64 as u64; fl_r = (crate::simd_ops::ru64(r10.wrapping_add(0x100u64))) as u64 as u64; rcx = (rcx & !0xffu64) | (((if (((fl_l as u64)) == ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); r9 = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = (r9) as u64; rax = ((rax).wrapping_sub(r10)) as u64; rax = ((rax).wrapping_shr(16u32)) as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rdx = ((r10.wrapping_add(0x108u64)) as u64) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x4u64))) as u32 as u64; rdx = ((rdx).wrapping_sub(rax)) as u64; fl_l = ((rcx as u8)) as u8 as u64; fl_r = ((rcx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 78; continue; } bb = 74; continue; }
            74 => { fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(0xeu64))) as u8 as u64; fl_r = ((rbx as u8)) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 77; continue; } bb = 75; continue; }
            75 => { rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(r9, (rax) as u64); crate::simd_ops::wu64(rdx.wrapping_add(0x20u64), (r9) as u64); fl_l = (crate::simd_ops::lk_add_u32(rdx.wrapping_add(0x18u64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; if (((fl_l as u32)) != 0) { bb = 80; continue; } bb = 76; continue; }
            76 => { rcx = (rdx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P7) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 80; continue; }
            77 => { r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 79; continue; }
            78 => { r8 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; bb = 79; continue; }
            79 => { rcx = (r10) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P8) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 80; continue; }
            80 => { crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (rdi) as u64); crate::simd_ops::wu32(rsp.wrapping_add(0x3cu64), ((rsi as u32)) as u32); fl_l = ((rsi as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rsp.wrapping_add(0x40u64))) as u32 as u64; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 82; continue; } bb = 81; continue; }
            81 => { crate::simd_ops::wu32(rsp.wrapping_add(0x40u64), ((r12 as u32)) as u32); crate::simd_ops::wu16(rdi.wrapping_add(r12.wrapping_mul(2)), ((rbx as u16)) as u16); rsi = (crate::simd_ops::ru32(rsp.wrapping_add(0x3cu64))) as u32 as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; bb = 82; continue; }
            82 => { r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; bb = 83; continue; }
            83 => { rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x4b0u64))) as u64; r8 = ((rax.wrapping_add(r15.wrapping_mul(2))) as u64) as u64; rdx = ((rsi as u32)) as u32 as u64; r9 = (r14) as u64; rcx = (rdi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P6)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu32(rsp.wrapping_add(0x40u64), ((r14 as u32)) as u32); rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsi = (crate::simd_ops::ru32(rsp.wrapping_add(0x3cu64))) as u32 as u64; rdx = (crate::simd_ops::ru32(rsp.wrapping_add(0x38u64))) as u32 as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; bb = 84; continue; }
            84 => { rax = ((rbp.wrapping_add(0x170u64)) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x150u64), (rax) as u64); crate::simd_ops::wu32(rbp.wrapping_add(0x158u64), (0x80u32) as u32); crate::simd_ops::wu64(rbp.wrapping_add(0x15cu64), (0x80u64) as u64); rax = ((rbp.wrapping_add(0x170u64)) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x168u64), (rax) as u64); crate::simd_ops::wu16(rbp.wrapping_add(0x170u64), ((rbx as u16)) as u16); fl_l = ((r14 as u32)) as u32 as u64; fl_r = (0x80u32) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 87; continue; } bb = 85; continue; }
            85 => { fl_l = (rcx) as u64 as u64; fl_r = (rdi) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 87; continue; } bb = 86; continue; }
            86 => { crate::simd_ops::wu64(rbp.wrapping_add(0x150u64), (rdi) as u64); crate::simd_ops::wu32(rbp.wrapping_add(0x15cu64), ((rsi as u32)) as u32); crate::simd_ops::wu32(rbp.wrapping_add(0x160u64), ((r14 as u32)) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (rcx) as u64); crate::simd_ops::wu32(rsp.wrapping_add(0x3cu64), ((rdx as u32)) as u32); rdi = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rsp.wrapping_add(0x40u64), ((rdi as u32)) as u32); crate::simd_ops::wu16(rcx, ((rdi as u16)) as u16); bb = 88; continue; }
            87 => { rdx = ((r14 as u32)) as u32 as u64; rcx = ((rbp.wrapping_add(0x150u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P9) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdx = (crate::simd_ops::ru32(rbp.wrapping_add(0x15cu64))) as u32 as u64; r8 = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x150u64))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P10)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x40u64))) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x160u64), ((rax as u32)) as u32); rdi = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; bb = 88; continue; }
            88 => { crate::simd_ops::lk_and_u32(rbp.wrapping_add(0x648u64), (0xfffffffdu32) as u32); rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 91; continue; } bb = 89; continue; }
            89 => { fl_l = (rcx) as u64 as u64; fl_r = (rax) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 91; continue; } bb = 90; continue; }
            90 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; bb = 91; continue; }
            91 => { fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 93; continue; } bb = 92; continue; }
            92 => { crate::simd_ops::wu16(rax, ((rdi as u16)) as u16); bb = 93; continue; }
            93 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P11)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu32(rax, ((rdi as u32)) as u32); rdx = ((rsp.wrapping_add(0x20u64)) as u64) as u64; rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x150u64))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P12)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; xmm6 = (xmm0 & !0xffff_ffffu128) | (((f64::from_bits(xmm0 as u64)) as f32).to_bits() as u128); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P11)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; fl_l = (crate::simd_ops::ru32(rax)) as u32 as u64; fl_r = (0x0u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 177; continue; } bb = 94; continue; }
            94 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; fl_l = (crate::simd_ops::ru16(rax)) as u16 as u64; fl_r = (0x0u16) as u16 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { bb = 177; continue; } bb = 95; continue; }
            95 => { rcx = (crate::simd_ops::ru32(rbp.wrapping_add(0x4c0u64))) as u32 as u64; r15 = (r12) as u64; fl_l = (r12) as u64 as u64; fl_r = (rcx) as u64 as u64; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 105; continue; } bb = 96; continue; }
            96 => { rax = ((rcx.wrapping_sub(0x1u64)) as u32) as u32 as u64; fl_l = ((r13 as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) > (((fl_r as u32)) as i32)) { bb = 105; continue; } bb = 97; continue; }
            97 => { rax = ((((r13 as u32)) as i32 as i64) as u64) as u64; r14 = ((rax.wrapping_add(rax)) as u64) as u64; rax = ((P1) as u64) as u64; fl_l = (r14) as u64 as u64; fl_r = (rax) as u64 as u64; r14 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; bb = 98; continue; }
            98 => { rsi = ((r14.wrapping_add(rax)) as u64) as u64; bb = 99; continue; }
            99 => { rax = ((rsi.wrapping_add(rdi.wrapping_mul(2))) as u64) as u64; rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x4b0u64))) as u64; rcx = ((crate::simd_ops::ru16(rax.wrapping_add(rcx))) as u32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P2)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rbx = (((rax as u16)) as u32) as u32 as u64; rax = ((P1) as u64) as u64; rcx = ((crate::simd_ops::ru16(rax.wrapping_add(rdi.wrapping_mul(2)))) as u32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P2)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; fl_l = ((rbx as u16)) as u16 as u64; fl_r = ((rax as u16)) as u16 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { bb = 102; continue; } bb = 100; continue; }
            100 => { fl_l = (rdi) as u64 as u64; fl_r = (rdi) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 104; continue; } bb = 101; continue; }
            101 => { rdi = ((rdi).wrapping_add(1)) as u64; fl_l = (rdi) as u64 as u64; fl_r = (0x1u64) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 99; continue; } bb = 102; continue; }
            102 => { r13 = (((r13 as u32)).wrapping_add(1)) as u32 as u64; r14 = ((r14).wrapping_add(0x2u64)) as u64; rcx = (crate::simd_ops::ru32(rbp.wrapping_add(0x4c0u64))) as u32 as u64; rax = ((rcx.wrapping_sub(0x1u64)) as u32) as u32 as u64; rdi = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; fl_l = ((r13 as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) > (((fl_r as u32)) as i32)) { bb = 105; continue; } bb = 103; continue; }
            103 => { rax = ((P1) as u64) as u64; bb = 98; continue; }
            104 => { r13 = ((((r13 as u32)) as i32 as i64) as u64) as u64; rcx = (crate::simd_ops::ru32(rbp.wrapping_add(0x4c0u64))) as u32 as u64; rdi = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; bb = 106; continue; }
            105 => { r13 = (0xffffffffffffffffu64) as u64; bb = 106; continue; }
            106 => { rax = ((rbp.wrapping_add(0x290u64)) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x270u64), (rax) as u64); crate::simd_ops::wu32(rbp.wrapping_add(0x278u64), (0x80u32) as u32); crate::simd_ops::wu64(rbp.wrapping_add(0x27cu64), (0x80u64) as u64); rax = ((rbp.wrapping_add(0x290u64)) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x288u64), (rax) as u64); crate::simd_ops::wu16(rbp.wrapping_add(0x290u64), ((rdi as u16)) as u16); crate::simd_ops::wu8(rbp.wrapping_add(0x640u64), (0x0u8) as u8); r8 = ((r12.wrapping_add(0x1u64)) as u64) as u64; rdx = ((rsp.wrapping_add(0x30u64)) as u64) as u64; fl_l = (r13) as u64 as u64; fl_r = (0xffffffffffffffffu64) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 113; continue; } bb = 107; continue; }
            107 => { r9 = ((rcx as u32)) as u32 as u64; r9 = ((r9).wrapping_sub(r15)) as u64; r9 = ((r9).wrapping_sub(1)) as u64; rcx = ((rbp.wrapping_add(0x4b0u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P14) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdx = (rax) as u64; rcx = ((rbp.wrapping_add(0x270u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P15) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 110; continue; } bb = 108; continue; }
            108 => { fl_l = (rcx) as u64 as u64; fl_r = (rax) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 110; continue; } bb = 109; continue; }
            109 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; bb = 110; continue; }
            110 => { fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 112; continue; } bb = 111; continue; }
            111 => { crate::simd_ops::wu16(rax, ((rdi as u16)) as u16); bb = 112; continue; }
            112 => { crate::simd_ops::wu8(rbp.wrapping_add(0x640u64), (0x1u8) as u8); bb = 118; continue; }
            113 => { r9 = (r13) as u64; r9 = ((r9).wrapping_sub(r15)) as u64; r9 = ((r9).wrapping_sub(1)) as u64; rcx = ((rbp.wrapping_add(0x4b0u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P14) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdx = (rax) as u64; rcx = ((rbp.wrapping_add(0x270u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P15) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 116; continue; } bb = 114; continue; }
            114 => { fl_l = (rcx) as u64 as u64; fl_r = (rax) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 116; continue; } bb = 115; continue; }
            115 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; bb = 116; continue; }
            116 => { fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 118; continue; } bb = 117; continue; }
            117 => { crate::simd_ops::wu16(rax, ((rdi as u16)) as u16); bb = 118; continue; }
            118 => { rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x638u64))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P16) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; r14 = (rax) as u64; crate::simd_ops::wu32(rax.wrapping_add(0x120u64), ((xmm7 as u32)) as u32); crate::simd_ops::wu32(rax.wrapping_add(0x124u64), ((xmm6 as u32)) as u32); rcx = (crate::simd_ops::ru32(rbp.wrapping_add(0x280u64))) as u32 as u64; rdx = (crate::simd_ops::ru32(rax.wrapping_add(0x10u64))) as u32 as u64; fl_l = (rcx) as u64 as u64; fl_r = (rdx) as u64 as u64; rdx = (if (((fl_l as u64)) >= ((fl_r as u64))) { (rcx as u32) } else { (rdx as u32) }) as u32 as u64; rcx = (rax) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P9) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdx = (crate::simd_ops::ru32(r14.wrapping_add(0xcu64))) as u32 as u64; r8 = (crate::simd_ops::ru64(rbp.wrapping_add(0x270u64))) as u64; rcx = (crate::simd_ops::ru64(r14)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P10)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x280u64))) as u32 as u64; crate::simd_ops::wu32(r14.wrapping_add(0x10u64), ((rax as u32)) as u32); r8 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rbx = ((P17) as u64) as u64; rdx = (rbx) as u64; rcx = (r14) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P18) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rsi = (rax) as u64; fl_l = (rax) as u64 as u64; fl_r = (0xffffffffffffffffu64) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 130; continue; } bb = 119; continue; }
            119 => { bb = 120; continue; }
            120 => { rdx = (rsi) as u64; rcx = (r14) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P19) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = (crate::simd_ops::ru32(r14.wrapping_add(0x10u64))) as u32 as u64; fl_l = (rsi) as u64 as u64; fl_r = (rax) as u64 as u64; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 130; continue; } bb = 121; continue; }
            121 => { rax = (((rax as u32)).wrapping_sub(1)) as u32 as u64; fl_l = ((rsi as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) > (((fl_r as u32)) as i32)) { bb = 130; continue; } bb = 122; continue; }
            122 => { rax = ((((rsi as u32)) as i32 as i64) as u64) as u64; r12 = ((rax.wrapping_add(rax)) as u64) as u64; fl_l = (r12) as u64 as u64; fl_r = (rbx) as u64 as u64; r12 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; bb = 123; continue; }
            123 => { r15 = ((r12.wrapping_add(rbx)) as u64) as u64; bb = 124; continue; }
            124 => { rax = ((r15.wrapping_add(rdi.wrapping_mul(2))) as u64) as u64; rcx = (crate::simd_ops::ru64(r14)) as u64; rcx = ((crate::simd_ops::ru16(rax.wrapping_add(rcx))) as u32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P2)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rbx = (((rax as u16)) as u32) as u32 as u64; rax = ((P17) as u64) as u64; rcx = ((crate::simd_ops::ru16(rax.wrapping_add(rdi.wrapping_mul(2)))) as u32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P2)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; fl_l = ((rbx as u16)) as u16 as u64; fl_r = ((rax as u16)) as u16 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { bb = 127; continue; } bb = 125; continue; }
            125 => { fl_l = (rdi) as u64 as u64; fl_r = (rdi) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 129; continue; } bb = 126; continue; }
            126 => { rdi = ((rdi).wrapping_add(1)) as u64; fl_l = (rdi) as u64 as u64; fl_r = (0x1u64) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 124; continue; } bb = 127; continue; }
            127 => { rsi = (((rsi as u32)).wrapping_add(1)) as u32 as u64; r12 = ((r12).wrapping_add(0x2u64)) as u64; rax = (crate::simd_ops::ru32(r14.wrapping_add(0x10u64))) as u32 as u64; rax = (((rax as u32)).wrapping_sub(1)) as u32 as u64; rdi = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; fl_l = ((rsi as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) > (((fl_r as u32)) as i32)) { bb = 130; continue; } bb = 128; continue; }
            128 => { rbx = ((P17) as u64) as u64; bb = 123; continue; }
            129 => { rsi = ((((rsi as u32)) as i32 as i64) as u64) as u64; fl_l = (rsi) as u64 as u64; fl_r = (0xffffffffffffffffu64) as u64 as u64; rbx = ((P17) as u64) as u64; rdi = (0x0u32) as u32 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 120; continue; } bb = 130; continue; }
            130 => { fl_l = (crate::simd_ops::ru8(rbp.wrapping_add(0x640u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 165; continue; } bb = 131; continue; }
            131 => { r10 = (crate::simd_ops::ru64(rbp.wrapping_add(0x270u64))) as u64; fl_l = (r10) as u64 as u64; fl_r = (r10) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 140; continue; } bb = 132; continue; }
            132 => { fl_l = (r10) as u64 as u64; fl_r = (crate::simd_ops::ru64(rbp.wrapping_add(0x288u64))) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 140; continue; } bb = 133; continue; }
            133 => { r10 = ((r10).wrapping_sub(1)) as u64; r10 = (((r10) & (0xfffffffffe000000u64))) as u64; rax = ((crate::exe::teb_self() as u64)) as u64; fl_l = (rax) as u64 as u64; fl_r = (crate::simd_ops::ru64(r10.wrapping_add(0x100u64))) as u64 as u64; rcx = (rcx & !0xffu64) | (((if (((fl_l as u64)) == ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); r9 = (crate::simd_ops::ru64(rbp.wrapping_add(0x270u64))) as u64; rax = (r9) as u64; rax = ((rax).wrapping_sub(r10)) as u64; rax = ((rax).wrapping_shr(16u32)) as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rdx = ((r10.wrapping_add(0x108u64)) as u64) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x4u64))) as u32 as u64; rdx = ((rdx).wrapping_sub(rax)) as u64; fl_l = ((rcx as u8)) as u8 as u64; fl_r = ((rcx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 138; continue; } bb = 134; continue; }
            134 => { fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(0xeu64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 137; continue; } bb = 135; continue; }
            135 => { rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(r9, (rax) as u64); crate::simd_ops::wu64(rdx.wrapping_add(0x20u64), (r9) as u64); fl_l = (crate::simd_ops::lk_add_u32(rdx.wrapping_add(0x18u64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; if (((fl_l as u32)) != 0) { bb = 140; continue; } bb = 136; continue; }
            136 => { rcx = (rdx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P7) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 140; continue; }
            137 => { r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 139; continue; }
            138 => { r8 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; bb = 139; continue; }
            139 => { rcx = (r10) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P8) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 140; continue; }
            140 => { rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x288u64))) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x270u64), (rcx) as u64); rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x278u64))) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x27cu64), ((rax as u32)) as u32); fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 142; continue; } bb = 141; continue; }
            141 => { crate::simd_ops::wu16(rcx, ((rdi as u16)) as u16); bb = 142; continue; }
            142 => { crate::simd_ops::wu32(rbp.wrapping_add(0x280u64), ((rdi as u32)) as u32); r10 = (crate::simd_ops::ru64(rbp.wrapping_add(0x150u64))) as u64; fl_l = (r10) as u64 as u64; fl_r = (r10) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 151; continue; } bb = 143; continue; }
            143 => { fl_l = (r10) as u64 as u64; fl_r = (crate::simd_ops::ru64(rbp.wrapping_add(0x168u64))) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 151; continue; } bb = 144; continue; }
            144 => { r10 = ((r10).wrapping_sub(1)) as u64; r10 = (((r10) & (0xfffffffffe000000u64))) as u64; rax = ((crate::exe::teb_self() as u64)) as u64; fl_l = (rax) as u64 as u64; fl_r = (crate::simd_ops::ru64(r10.wrapping_add(0x100u64))) as u64 as u64; rcx = (rcx & !0xffu64) | (((if (((fl_l as u64)) == ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); r9 = (crate::simd_ops::ru64(rbp.wrapping_add(0x150u64))) as u64; rax = (r9) as u64; rax = ((rax).wrapping_sub(r10)) as u64; rax = ((rax).wrapping_shr(16u32)) as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rdx = ((r10.wrapping_add(0x108u64)) as u64) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x4u64))) as u32 as u64; rdx = ((rdx).wrapping_sub(rax)) as u64; fl_l = ((rcx as u8)) as u8 as u64; fl_r = ((rcx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 149; continue; } bb = 145; continue; }
            145 => { fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(0xeu64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 148; continue; } bb = 146; continue; }
            146 => { rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(r9, (rax) as u64); crate::simd_ops::wu64(rdx.wrapping_add(0x20u64), (r9) as u64); fl_l = (crate::simd_ops::lk_add_u32(rdx.wrapping_add(0x18u64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; if (((fl_l as u32)) != 0) { bb = 151; continue; } bb = 147; continue; }
            147 => { rcx = (rdx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P7) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 151; continue; }
            148 => { r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 150; continue; }
            149 => { r8 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; bb = 150; continue; }
            150 => { rcx = (r10) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P8) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 151; continue; }
            151 => { rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x168u64))) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x150u64), (rcx) as u64); rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x158u64))) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x15cu64), ((rax as u32)) as u32); fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 153; continue; } bb = 152; continue; }
            152 => { crate::simd_ops::wu16(rcx, ((rdi as u16)) as u16); bb = 153; continue; }
            153 => { crate::simd_ops::wu32(rbp.wrapping_add(0x160u64), ((rdi as u32)) as u32); r10 = (crate::simd_ops::ru64(rbp.wrapping_add(0x390u64))) as u64; fl_l = (r10) as u64 as u64; fl_r = (r10) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 162; continue; } bb = 154; continue; }
            154 => { fl_l = (r10) as u64 as u64; fl_r = (crate::simd_ops::ru64(rbp.wrapping_add(0x3a8u64))) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 162; continue; } bb = 155; continue; }
            155 => { r10 = ((r10).wrapping_sub(1)) as u64; r10 = (((r10) & (0xfffffffffe000000u64))) as u64; rax = ((crate::exe::teb_self() as u64)) as u64; fl_l = (rax) as u64 as u64; fl_r = (crate::simd_ops::ru64(r10.wrapping_add(0x100u64))) as u64 as u64; rcx = (rcx & !0xffu64) | (((if (((fl_l as u64)) == ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); r9 = (crate::simd_ops::ru64(rbp.wrapping_add(0x390u64))) as u64; rax = (r9) as u64; rax = ((rax).wrapping_sub(r10)) as u64; rax = ((rax).wrapping_shr(16u32)) as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rdx = ((r10.wrapping_add(0x108u64)) as u64) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x4u64))) as u32 as u64; rdx = ((rdx).wrapping_sub(rax)) as u64; fl_l = ((rcx as u8)) as u8 as u64; fl_r = ((rcx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 160; continue; } bb = 156; continue; }
            156 => { fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(0xeu64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 159; continue; } bb = 157; continue; }
            157 => { rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(r9, (rax) as u64); crate::simd_ops::wu64(rdx.wrapping_add(0x20u64), (r9) as u64); fl_l = (crate::simd_ops::lk_add_u32(rdx.wrapping_add(0x18u64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; if (((fl_l as u32)) != 0) { bb = 162; continue; } bb = 158; continue; }
            158 => { rcx = (rdx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P7) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 162; continue; }
            159 => { r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 161; continue; }
            160 => { r8 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; bb = 161; continue; }
            161 => { rcx = (r10) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P8) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 162; continue; }
            162 => { rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x3a8u64))) as u64; fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 164; continue; } bb = 163; continue; }
            163 => { crate::simd_ops::wu16(rax, ((rdi as u16)) as u16); bb = 164; continue; }
            164 => { bb = 1; continue; }
            165 => { rbx = (rbx & !0xffu64) | (((0x1u8) as u8 as u64)); r10 = (crate::simd_ops::ru64(rbp.wrapping_add(0x270u64))) as u64; fl_l = (r10) as u64 as u64; fl_r = (r10) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 174; continue; } bb = 166; continue; }
            166 => { fl_l = (r10) as u64 as u64; fl_r = (crate::simd_ops::ru64(rbp.wrapping_add(0x288u64))) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 174; continue; } bb = 167; continue; }
            167 => { r10 = ((r10).wrapping_sub(1)) as u64; r10 = (((r10) & (0xfffffffffe000000u64))) as u64; rax = ((crate::exe::teb_self() as u64)) as u64; fl_l = (rax) as u64 as u64; fl_r = (crate::simd_ops::ru64(r10.wrapping_add(0x100u64))) as u64 as u64; rcx = (rcx & !0xffu64) | (((if (((fl_l as u64)) == ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); r9 = (crate::simd_ops::ru64(rbp.wrapping_add(0x270u64))) as u64; rax = (r9) as u64; rax = ((rax).wrapping_sub(r10)) as u64; rax = ((rax).wrapping_shr(16u32)) as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rdx = ((r10.wrapping_add(0x108u64)) as u64) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x4u64))) as u32 as u64; rdx = ((rdx).wrapping_sub(rax)) as u64; fl_l = ((rcx as u8)) as u8 as u64; fl_r = ((rcx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 172; continue; } bb = 168; continue; }
            168 => { fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(0xeu64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 171; continue; } bb = 169; continue; }
            169 => { rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(r9, (rax) as u64); crate::simd_ops::wu64(rdx.wrapping_add(0x20u64), (r9) as u64); fl_l = (crate::simd_ops::lk_add_u32(rdx.wrapping_add(0x18u64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; if (((fl_l as u32)) != 0) { bb = 174; continue; } bb = 170; continue; }
            170 => { rcx = (rdx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P7) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 174; continue; }
            171 => { r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 173; continue; }
            172 => { r8 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; bb = 173; continue; }
            173 => { rcx = (r10) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P8) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 174; continue; }
            174 => { rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x288u64))) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x270u64), (rcx) as u64); rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x278u64))) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x27cu64), ((rax as u32)) as u32); fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 176; continue; } bb = 175; continue; }
            175 => { crate::simd_ops::wu16(rcx, ((rdi as u16)) as u16); bb = 176; continue; }
            176 => { crate::simd_ops::wu32(rbp.wrapping_add(0x280u64), ((rdi as u32)) as u32); bb = 178; continue; }
            177 => { rbx = (rbx & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 178; continue; }
            178 => { r10 = (crate::simd_ops::ru64(rbp.wrapping_add(0x150u64))) as u64; fl_l = (r10) as u64 as u64; fl_r = (r10) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 187; continue; } bb = 179; continue; }
            179 => { fl_l = (r10) as u64 as u64; fl_r = (crate::simd_ops::ru64(rbp.wrapping_add(0x168u64))) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 187; continue; } bb = 180; continue; }
            180 => { r10 = ((r10).wrapping_sub(1)) as u64; r10 = (((r10) & (0xfffffffffe000000u64))) as u64; rax = ((crate::exe::teb_self() as u64)) as u64; fl_l = (rax) as u64 as u64; fl_r = (crate::simd_ops::ru64(r10.wrapping_add(0x100u64))) as u64 as u64; rcx = (rcx & !0xffu64) | (((if (((fl_l as u64)) == ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); r9 = (crate::simd_ops::ru64(rbp.wrapping_add(0x150u64))) as u64; rax = (r9) as u64; rax = ((rax).wrapping_sub(r10)) as u64; rax = ((rax).wrapping_shr(16u32)) as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rdx = ((r10.wrapping_add(0x108u64)) as u64) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x4u64))) as u32 as u64; rdx = ((rdx).wrapping_sub(rax)) as u64; fl_l = ((rcx as u8)) as u8 as u64; fl_r = ((rcx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 185; continue; } bb = 181; continue; }
            181 => { fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(0xeu64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 184; continue; } bb = 182; continue; }
            182 => { rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(r9, (rax) as u64); crate::simd_ops::wu64(rdx.wrapping_add(0x20u64), (r9) as u64); fl_l = (crate::simd_ops::lk_add_u32(rdx.wrapping_add(0x18u64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; if (((fl_l as u32)) != 0) { bb = 187; continue; } bb = 183; continue; }
            183 => { rcx = (rdx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P7) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 187; continue; }
            184 => { r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 186; continue; }
            185 => { r8 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; bb = 186; continue; }
            186 => { rcx = (r10) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P8) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 187; continue; }
            187 => { rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x168u64))) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x150u64), (rcx) as u64); rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x158u64))) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x15cu64), ((rax as u32)) as u32); fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 189; continue; } bb = 188; continue; }
            188 => { crate::simd_ops::wu16(rcx, ((rdi as u16)) as u16); bb = 189; continue; }
            189 => { crate::simd_ops::wu32(rbp.wrapping_add(0x160u64), ((rdi as u32)) as u32); bb = 192; continue; }
            190 => { rdi = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; bb = 191; continue; }
            191 => { rbx = (rbx & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 192; continue; }
            192 => { r10 = (crate::simd_ops::ru64(rbp.wrapping_add(0x390u64))) as u64; fl_l = (r10) as u64 as u64; fl_r = (r10) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 201; continue; } bb = 193; continue; }
            193 => { fl_l = (r10) as u64 as u64; fl_r = (crate::simd_ops::ru64(rbp.wrapping_add(0x3a8u64))) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 201; continue; } bb = 194; continue; }
            194 => { r10 = ((r10).wrapping_sub(1)) as u64; r10 = (((r10) & (0xfffffffffe000000u64))) as u64; rax = ((crate::exe::teb_self() as u64)) as u64; fl_l = (rax) as u64 as u64; fl_r = (crate::simd_ops::ru64(r10.wrapping_add(0x100u64))) as u64 as u64; rcx = (rcx & !0xffu64) | (((if (((fl_l as u64)) == ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); r9 = (crate::simd_ops::ru64(rbp.wrapping_add(0x390u64))) as u64; rax = (r9) as u64; rax = ((rax).wrapping_sub(r10)) as u64; rax = ((rax).wrapping_shr(16u32)) as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rdx = ((r10.wrapping_add(0x108u64)) as u64) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x4u64))) as u32 as u64; rdx = ((rdx).wrapping_sub(rax)) as u64; fl_l = ((rcx as u8)) as u8 as u64; fl_r = ((rcx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 199; continue; } bb = 195; continue; }
            195 => { fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(0xeu64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 198; continue; } bb = 196; continue; }
            196 => { rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(r9, (rax) as u64); crate::simd_ops::wu64(rdx.wrapping_add(0x20u64), (r9) as u64); fl_l = (crate::simd_ops::lk_add_u32(rdx.wrapping_add(0x18u64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; if (((fl_l as u32)) != 0) { bb = 201; continue; } bb = 197; continue; }
            197 => { rcx = (rdx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P7) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 201; continue; }
            198 => { r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 200; continue; }
            199 => { r8 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; bb = 200; continue; }
            200 => { rcx = (r10) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P8) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 201; continue; }
            201 => { rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x3a8u64))) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x390u64), (rcx) as u64); rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x398u64))) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x39cu64), ((rax as u32)) as u32); fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 203; continue; } bb = 202; continue; }
            202 => { crate::simd_ops::wu16(rcx, ((rdi as u16)) as u16); bb = 203; continue; }
            203 => { crate::simd_ops::wu32(rbp.wrapping_add(0x3a0u64), ((rdi as u32)) as u32); bb = 21; continue; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141d993f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0x18; const P3: u64 = 0x1c; const P4: u64 = 0x20; const P5: u64 = 0x28;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu64(rdx.wrapping_add(P0), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(P1), rax);
    crate::simd_ops::wu32(rdx.wrapping_add(P2), ((rax as u32)) as u32);
    crate::simd_ops::wu8(rdx.wrapping_add(P3), ((rax as u8)) as u8);
    crate::simd_ops::wu64(rdx.wrapping_add(P4), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(P5), rax);
    rax = rdx;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141d99a50(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x4; const P1: u64 = 0x8; const P2: u64 = 0x3; const P3: u64 = 0x4c;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    crate::simd_ops::lk_and_u32(rdx, (0xff000008u32) as u32);
    rax = 0;
    fl_l = 0; fl_r = 0;
    fl_l = (crate::simd_ops::lk_or_u32(rdx, (0x8u32) as u32)) as u32 as u64; fl_r = fl_l;
    crate::simd_ops::wu32(rdx.wrapping_add(P0), ((rax as u32)) as u32);
    crate::simd_ops::wu8(rdx.wrapping_add(P1), ((rax as u8)) as u8);
    crate::simd_ops::wu8(rdx.wrapping_add(P2), (0x8u8) as u8);
    crate::simd_ops::wu32(rdx.wrapping_add(P3), ((rax as u32)) as u32);
    rax = rdx;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141d9a080(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x4; const P2: u64 = 0x14240aa00; const P3: u64 = 0x50; const P4: u64 = 0x98; const P5: u64 = 0xe0; const P6: u64 = 0xec; const P7: u64 = 0xe4; const P8: u64 = 0xe5; const P9: u64 = 0xe8; const P10: u64 = 0x114;
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (crate::simd_ops::ru32(rdx)) as u32 as u64; rcx = ((r8.wrapping_add(P0)) as u64) as u64; crate::simd_ops::wu32(r8, ((rax as u32)) as u32); rbx = rdx; rax = (crate::simd_ops::ru32(rdx.wrapping_add(P1))) as u32 as u64; rdi = r8; rdx = ((rdx).wrapping_add(0x8u64)) as u64; crate::simd_ops::wu32(r8.wrapping_add(P1), ((rax as u32)) as u32); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rdx = ((rbx.wrapping_add(P3)) as u64) as u64; rcx = ((rdi.wrapping_add(P3)) as u64) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rdx = ((rbx.wrapping_add(P4)) as u64) as u64; rcx = ((rdi.wrapping_add(P4)) as u64) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru32(rbx.wrapping_add(P5))) as u32 as u64; rdx = ((rbx.wrapping_add(P6)) as u64) as u64; crate::simd_ops::wu32(rdi.wrapping_add(P5), ((rax as u32)) as u32); rcx = ((rdi.wrapping_add(P6)) as u64) as u64; rax = ((crate::simd_ops::ru8(rbx.wrapping_add(P7))) as u32) as u32 as u64; crate::simd_ops::wu8(rdi.wrapping_add(P7), ((rax as u8)) as u8); rax = ((crate::simd_ops::ru8(rbx.wrapping_add(P8))) as u32) as u32 as u64; crate::simd_ops::wu8(rdi.wrapping_add(P8), ((rax as u8)) as u8); rax = (crate::simd_ops::ru32(rbx.wrapping_add(P9))) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(P9), ((rax as u32)) as u32); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rdx = ((rbx.wrapping_add(P10)) as u64) as u64; rcx = ((rdi.wrapping_add(P10)) as u64) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(P2, rax, rcx, rdx, r8, r9, rsp); }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141d9a150(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x114; const P1: u64 = 0x14240ab10; const P2: u64 = 0xec; const P3: u64 = 0x98; const P4: u64 = 0x50; const P5: u64 = 0x8;
    let (mut rax, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rcx = ((rdx.wrapping_add(P0)) as u64) as u64;
    rbx = rdx;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rcx = ((rbx.wrapping_add(P2)) as u64) as u64;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rcx = ((rbx.wrapping_add(P3)) as u64) as u64;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rcx = ((rbx.wrapping_add(P4)) as u64) as u64;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rcx = ((rbx.wrapping_add(P5)) as u64) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return crate::simd_ops::tail_stk(P1, rax, rcx, rdx, r8, r9, rsp);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141d9a1b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xfffffffffffffff8; const P1: u64 = 0xfffffffffffffff0; const P2: u64 = 0x114; const P3: u64 = 0x14240ab10; const P4: u64 = 0xec; const P5: u64 = 0x98; const P6: u64 = 0x50; const P7: u64 = 0x8; const P8: u64 = 0x142442c80;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 1; continue; }
            1 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rsi); rbx = rdx; rsi = 0; fl_l = 0; fl_r = 0; fl_l = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64; fl_r = ((rsi as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 5; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rdi); bb = 3; continue; }
            3 => { rdi = rsi; rdi = (((rdi) as i64).wrapping_mul((crate::simd_ops::ru64(rbx.wrapping_add(P1))) as i64) as u64) as u64; rdi = ((rdi).wrapping_add(rbx)) as u64; rcx = ((rdi.wrapping_add(P2)) as u64) as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rcx = ((rdi.wrapping_add(P4)) as u64) as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rcx = ((rdi.wrapping_add(P5)) as u64) as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rcx = ((rdi.wrapping_add(P6)) as u64) as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rcx = ((rdi.wrapping_add(P7)) as u64) as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru32(rbx.wrapping_add(P0))) as u32 as u64; rsi = ((rsi).wrapping_add(1)) as u64; fl_l = rsi; fl_r = rax; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 3; continue; } bb = 4; continue; }
            4 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; bb = 5; continue; }
            5 => { rcx = rbx; rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); bb = 6; continue; }
            6 => { return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141d9a240(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x114; const P1: u64 = 0x14240ab10; const P2: u64 = 0xec; const P3: u64 = 0x98; const P4: u64 = 0x50; const P5: u64 = 0x8; const P6: u64 = 0x142442de0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    fl_l = rdx; fl_r = rdx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rcx = ((rdx.wrapping_add(P0)) as u64) as u64; rbx = rdx; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rcx = ((rbx.wrapping_add(P2)) as u64) as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rcx = ((rbx.wrapping_add(P3)) as u64) as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rcx = ((rbx.wrapping_add(P4)) as u64) as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rcx = ((rbx.wrapping_add(P5)) as u64) as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rcx = rbx; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); }
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141d9a8b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x4; const P1: u64 = 0x8; const P2: u64 = 0x3; const P3: u64 = 0x108; const P4: u64 = 0x10c; const P5: u64 = 0x110; const P6: u64 = 0x10b;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    crate::simd_ops::lk_and_u32(rdx, (0xff000020u32) as u32);
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::lk_or_u32(rdx, (0x20u32) as u32);
    crate::simd_ops::wu32(rdx.wrapping_add(P0), ((rax as u32)) as u32);
    crate::simd_ops::wu8(rdx.wrapping_add(P1), ((rax as u8)) as u8);
    crate::simd_ops::wu8(rdx.wrapping_add(P2), (0x20u8) as u8);
    crate::simd_ops::lk_and_u32(rdx.wrapping_add(P3), (0xff000008u32) as u32);
    fl_l = (crate::simd_ops::lk_or_u32(rdx.wrapping_add(P3), (0x8u32) as u32)) as u32 as u64; fl_r = fl_l;
    crate::simd_ops::wu32(rdx.wrapping_add(P4), ((rax as u32)) as u32);
    crate::simd_ops::wu8(rdx.wrapping_add(P5), ((rax as u8)) as u8);
    rax = rdx;
    crate::simd_ops::wu8(rdx.wrapping_add(P6), (0x8u8) as u8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141d9bcf0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0x18; const P3: u64 = 0x1c; const P4: u64 = 0x20; const P5: u64 = 0x28;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu32(rdx, ((rax as u32)) as u32);
    crate::simd_ops::wu64(rdx.wrapping_add(P0), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(P1), rax);
    crate::simd_ops::wu32(rdx.wrapping_add(P2), ((rax as u32)) as u32);
    crate::simd_ops::wu8(rdx.wrapping_add(P3), ((rax as u8)) as u8);
    crate::simd_ops::wu64(rdx.wrapping_add(P4), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(P5), rax);
    rax = rdx;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141d9c300(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14240aa00; const P1: u64 = 0x48; const P2: u64 = 0x1401e84a0; const P3: u64 = 0x60; const P4: u64 = 0x78; const P5: u64 = 0x7c; const P6: u64 = 0x7d; const P7: u64 = 0x7e; const P8: u64 = 0x7f; const P9: u64 = 0x80; const P10: u64 = 0x84; const P11: u64 = 0x88; const P12: u64 = 0x140e5d880;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = r8; rbx = rdx; rcx = r8; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rdx = ((rbx.wrapping_add(P1)) as u64) as u64; rcx = ((rdi.wrapping_add(P1)) as u64) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rdx = ((rbx.wrapping_add(P3)) as u64) as u64; rcx = ((rdi.wrapping_add(P3)) as u64) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru32(rbx.wrapping_add(P4))) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(P4), ((rax as u32)) as u32); rax = ((crate::simd_ops::ru8(rbx.wrapping_add(P5))) as u32) as u32 as u64; crate::simd_ops::wu8(rdi.wrapping_add(P5), ((rax as u8)) as u8); rax = ((crate::simd_ops::ru8(rbx.wrapping_add(P6))) as u32) as u32 as u64; crate::simd_ops::wu8(rdi.wrapping_add(P6), ((rax as u8)) as u8); rax = ((crate::simd_ops::ru8(rbx.wrapping_add(P7))) as u32) as u32 as u64; crate::simd_ops::wu8(rdi.wrapping_add(P7), ((rax as u8)) as u8); rax = ((crate::simd_ops::ru8(rbx.wrapping_add(P8))) as u32) as u32 as u64; crate::simd_ops::wu8(rdi.wrapping_add(P8), ((rax as u8)) as u8); rax = (crate::simd_ops::ru32(rbx.wrapping_add(P9))) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(P9), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rbx.wrapping_add(P10))) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(P10), ((rax as u32)) as u32); rax = (crate::simd_ops::ru64(rbx.wrapping_add(P11))) as u64; crate::simd_ops::wu64(rdi.wrapping_add(0x88u64), rax); rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x90u64))) as u64; crate::simd_ops::wu64(rdi.wrapping_add(0x90u64), rax); rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x98u64))) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x98u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x9cu64))) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x9cu64), ((rax as u32)) as u32); rbx = ((rbx).wrapping_add(0xa0u64)) as u64; rcx = rdi.wrapping_add(0xa0); fl_l = rcx; fl_r = rbx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rdx = (crate::simd_ops::ru64(rbx)) as u64; rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x8u64))) as u32 as u64; r8 = (((rax) as i64).wrapping_mul((0x128u64) as i64) as u64) as u64; r8 = ((r8).wrapping_add(rdx)) as u64; rax = crate::simd_ops::call_stk(P12, rax, rcx, rdx, r8, r9, rsp); bb = 2; continue; }
            2 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141d9c3f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xa0; const P1: u64 = 0x140e48990; const P2: u64 = 0x68; const P3: u64 = 0x50; const P4: u64 = 0x14240ab10;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rdx;
    rcx = ((rdx.wrapping_add(P0)) as u64) as u64;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rcx = 0;
    fl_l = 0; fl_r = 0;
    rax = ((rcx as u32)) as u32 as u64;
    let tmp_xchg: u64 = crate::simd_ops::atomic_xchg_u64(rbx.wrapping_add(P2), rax);
    rax = (tmp_xchg) as u64;
    let tmp_xchg: u64 = crate::simd_ops::atomic_xchg_u64(rbx.wrapping_add(P3), rcx);
    rcx = (tmp_xchg) as u64;
    rcx = rbx;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return crate::simd_ops::tail_stk(P4, rax, rcx, rdx, r8, r9, rsp);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141d9c430(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x4; const P1: u64 = 0x8; const P2: u64 = 0x3; const P3: u64 = 0x48; const P4: u64 = 0x50; const P5: u64 = 0x58; const P6: u64 = 0x5c; const P7: u64 = 0x60; const P8: u64 = 0x68; const P9: u64 = 0x70; const P10: u64 = 0x74; const P11: u64 = 0x88;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    crate::simd_ops::lk_and_u32(rdx, (0xff000008u32) as u32);
    rax = 0;
    fl_l = 0; fl_r = 0;
    fl_l = (crate::simd_ops::lk_or_u32(rdx, (0x8u32) as u32)) as u32 as u64; fl_r = fl_l;
    crate::simd_ops::wu32(rdx.wrapping_add(P0), ((rax as u32)) as u32);
    crate::simd_ops::wu8(rdx.wrapping_add(P1), ((rax as u8)) as u8);
    crate::simd_ops::wu8(rdx.wrapping_add(P2), (0x8u8) as u8);
    crate::simd_ops::wu64(rdx.wrapping_add(P3), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(P4), rax);
    crate::simd_ops::wu32(rdx.wrapping_add(P5), ((rax as u32)) as u32);
    crate::simd_ops::wu8(rdx.wrapping_add(P6), ((rax as u8)) as u8);
    crate::simd_ops::wu64(rdx.wrapping_add(P7), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(P8), rax);
    crate::simd_ops::wu32(rdx.wrapping_add(P9), ((rax as u32)) as u32);
    crate::simd_ops::wu8(rdx.wrapping_add(P10), ((rax as u8)) as u8);
    crate::simd_ops::wu64(rdx.wrapping_add(P11), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(0x90u64), rax);
    crate::simd_ops::wu32(rdx.wrapping_add(0x9cu64), (0xffffu32) as u32);
    crate::simd_ops::wu64(rdx.wrapping_add(0xa0u64), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(0xa8u64), rax);
    rax = rdx;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141d9c4a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xfffffffffffffff8; const P1: u64 = 0xfffffffffffffff0; const P2: u64 = 0xa0; const P3: u64 = 0x140e48990; const P4: u64 = 0x68; const P5: u64 = 0x50; const P6: u64 = 0x14240ab10; const P7: u64 = 0x142442c80;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = rdx; rsi = 0; fl_l = 0; fl_r = 0; fl_l = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64; fl_r = ((rsi as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 4; continue; } bb = 2; continue; }
            2 => { bb = 3; continue; }
            3 => { rbx = rsi; rbx = (((rbx) as i64).wrapping_mul((crate::simd_ops::ru64(rdi.wrapping_add(P1))) as i64) as u64) as u64; rbx = ((rbx).wrapping_add(rdi)) as u64; rcx = ((rbx.wrapping_add(P2)) as u64) as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rax = 0; fl_l = 0; fl_r = 0; let tmp_xchg: u64 = crate::simd_ops::atomic_xchg_u64(rbx.wrapping_add(P4), rax); rax = (tmp_xchg) as u64; rax = 0; fl_l = 0; fl_r = 0; let tmp_xchg: u64 = crate::simd_ops::atomic_xchg_u64(rbx.wrapping_add(P5), rax); rax = (tmp_xchg) as u64; rcx = rbx; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); rsi = ((rsi).wrapping_add(1)) as u64; rax = (crate::simd_ops::ru32(rdi.wrapping_add(P0))) as u32 as u64; fl_l = rsi; fl_r = rax; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 3; continue; } bb = 4; continue; }
            4 => { rcx = rdi; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); bb = 5; continue; }
            5 => { return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141d9c520(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xa0; const P1: u64 = 0x140e48990; const P2: u64 = 0x68; const P3: u64 = 0x50; const P4: u64 = 0x14240ab10; const P5: u64 = 0x142442de0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    fl_l = rdx; fl_r = rdx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rdx; rcx = ((rdx.wrapping_add(P0)) as u64) as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rcx = 0; fl_l = 0; fl_r = 0; rax = ((rcx as u32)) as u32 as u64; let tmp_xchg: u64 = crate::simd_ops::atomic_xchg_u64(rbx.wrapping_add(P2), rax); rax = (tmp_xchg) as u64; let tmp_xchg: u64 = crate::simd_ops::atomic_xchg_u64(rbx.wrapping_add(P3), rcx); rcx = (tmp_xchg) as u64; rcx = rbx; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rcx = rbx; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); }
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141d9cae0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x140948700; const P2: u64 = 0x10;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rdi = r8;
    rbx = rdx;
    fl_l = r8; fl_r = rdx;
    if !((((fl_l as u64)) == ((fl_r as u64)))) { rdx = (crate::simd_ops::ru64(rdx)) as u64; rax = (crate::simd_ops::ru32(rbx.wrapping_add(P0))) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; r8 = ((rdx.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; rcx = rdi; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); }
    rbx = ((rbx).wrapping_add(0x10u64)) as u64;
    rcx = ((rdi.wrapping_add(P2)) as u64) as u64;
    fl_l = rcx; fl_r = rbx;
    if !((((fl_l as u64)) == ((fl_r as u64)))) { rdx = (crate::simd_ops::ru64(rbx)) as u64; rax = (crate::simd_ops::ru32(rbx.wrapping_add(P0))) as u32 as u64; r8 = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; r8 = ((rdx.wrapping_add(r8.wrapping_mul(8))) as u64) as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); }
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141d9d1f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0x14; const P3: u64 = 0x18; const P4: u64 = 0x20; const P5: u64 = 0x28; const P6: u64 = 0x30; const P7: u64 = 0x38; const P8: u64 = 0x40; const P9: u64 = 0x48; const P10: u64 = 0x50; const P11: u64 = 0x58;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu64(rdx, rax);
    crate::simd_ops::wu64(rdx.wrapping_add(P0), rax);
    crate::simd_ops::wu32(rdx.wrapping_add(P1), ((rax as u32)) as u32);
    crate::simd_ops::wu8(rdx.wrapping_add(P2), ((rax as u8)) as u8);
    crate::simd_ops::wu64(rdx.wrapping_add(P3), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(P4), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(P5), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(P6), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(P7), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(P8), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(P9), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(P10), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(P11), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(0x60u64), rax);
    crate::simd_ops::wu32(rdx.wrapping_add(0x68u64), ((rax as u32)) as u32);
    crate::simd_ops::wu8(rdx.wrapping_add(0x6cu64), ((rax as u8)) as u8);
    crate::simd_ops::wu64(rdx.wrapping_add(0x70u64), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(0x78u64), rax);
    crate::simd_ops::wu32(rdx.wrapping_add(0x80u64), ((rax as u32)) as u32);
    crate::simd_ops::wu8(rdx.wrapping_add(0x84u64), ((rax as u8)) as u8);
    crate::simd_ops::wu64(rdx.wrapping_add(0x88u64), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(0x90u64), rax);
    crate::simd_ops::wu32(rdx.wrapping_add(0x98u64), ((rax as u32)) as u32);
    crate::simd_ops::wu8(rdx.wrapping_add(0x9cu64), ((rax as u8)) as u8);
    crate::simd_ops::wu64(rdx.wrapping_add(0xa0u64), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(0xa8u64), rax);
    crate::simd_ops::wu32(rdx.wrapping_add(0xb0u64), ((rax as u32)) as u32);
    crate::simd_ops::wu8(rdx.wrapping_add(0xb4u64), ((rax as u8)) as u8);
    rax = rdx;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141d9d280(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xfffffffffffffff8; const P1: u64 = 0xfffffffffffffff0; const P2: u64 = 0xa8; const P3: u64 = 0x90; const P4: u64 = 0x78; const P5: u64 = 0x60; const P6: u64 = 0x48; const P7: u64 = 0x140236fa0; const P8: u64 = 0x38; const P9: u64 = 0x28; const P10: u64 = 0x18; const P11: u64 = 0x8; const P12: u64 = 0x142442c80;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = rdx; rsi = 0; fl_l = 0; fl_r = 0; fl_l = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64; fl_r = ((rsi as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 3; continue; } bb = 2; continue; }
            2 => { rbx = rsi; rbx = (((rbx) as i64).wrapping_mul((crate::simd_ops::ru64(rdi.wrapping_add(P1))) as i64) as u64) as u64; rbx = ((rbx).wrapping_add(rdi)) as u64; rax = 0; fl_l = 0; fl_r = 0; let tmp_xchg: u64 = crate::simd_ops::atomic_xchg_u64(rbx.wrapping_add(P2), rax); rax = (tmp_xchg) as u64; rax = 0; fl_l = 0; fl_r = 0; let tmp_xchg: u64 = crate::simd_ops::atomic_xchg_u64(rbx.wrapping_add(P3), rax); rax = (tmp_xchg) as u64; rax = 0; fl_l = 0; fl_r = 0; let tmp_xchg: u64 = crate::simd_ops::atomic_xchg_u64(rbx.wrapping_add(P4), rax); rax = (tmp_xchg) as u64; rax = 0; fl_l = 0; fl_r = 0; let tmp_xchg: u64 = crate::simd_ops::atomic_xchg_u64(rbx.wrapping_add(P5), rax); rax = (tmp_xchg) as u64; rcx = ((rbx.wrapping_add(P6)) as u64) as u64; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); rcx = ((rbx.wrapping_add(P8)) as u64) as u64; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); rcx = ((rbx.wrapping_add(P9)) as u64) as u64; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); rcx = ((rbx.wrapping_add(P10)) as u64) as u64; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); rax = 0; fl_l = 0; fl_r = 0; let tmp_xchg: u64 = crate::simd_ops::atomic_xchg_u64(rbx.wrapping_add(P11), rax); rax = (tmp_xchg) as u64; rsi = ((rsi).wrapping_add(1)) as u64; rax = (crate::simd_ops::ru32(rdi.wrapping_sub(0x8u64))) as u32 as u64; fl_l = rsi; fl_r = rax; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 2; continue; } bb = 3; continue; }
            3 => { rcx = rdi; rax = crate::simd_ops::call_stk(P12, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); bb = 4; continue; }
            4 => { return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141d9d930(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x140948700; const P2: u64 = 0x18; const P3: u64 = 0x1401e84a0; const P4: u64 = 0x30;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = r8;
    rdi = rdx;
    rax = (crate::simd_ops::ru32(rdx)) as u32 as u64;
    crate::simd_ops::wu32(r8, ((rax as u32)) as u32);
    rax = ((rdx.wrapping_add(P0)) as u64) as u64;
    rcx = ((r8.wrapping_add(P0)) as u64) as u64;
    fl_l = rcx; fl_r = rax;
    if !((((fl_l as u64)) == ((fl_r as u64)))) { rdx = (crate::simd_ops::ru64(rax)) as u64; rax = (crate::simd_ops::ru32(rax.wrapping_add(P0))) as u32 as u64; r9 = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; r8 = ((rdx.wrapping_add(r9.wrapping_mul(8))) as u64) as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); }
    rdx = ((rdi.wrapping_add(P2)) as u64) as u64;
    rcx = ((rbx.wrapping_add(P2)) as u64) as u64;
    rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp);
    rax = (crate::simd_ops::ru32(rdi.wrapping_add(P4))) as u32 as u64;
    crate::simd_ops::wu32(rbx.wrapping_add(P4), ((rax as u32)) as u32);
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141d9d9c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0x18; const P3: u64 = 0x20; const P4: u64 = 0x28; const P5: u64 = 0x2c;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu64(rdx.wrapping_add(P0), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(P1), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(P2), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(P3), rax);
    crate::simd_ops::wu32(rdx.wrapping_add(P4), ((rax as u32)) as u32);
    crate::simd_ops::wu8(rdx.wrapping_add(P5), ((rax as u8)) as u8);
    rax = rdx;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141d9d9f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xfffffffffffffff8; const P1: u64 = 0xfffffffffffffff0; const P2: u64 = 0x20; const P3: u64 = 0x140236fa0; const P4: u64 = 0x142442c80;
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
    if (((fl_l as u32)) <= ((fl_r as u32))) { rcx = rbx; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    loop { rcx = rdi; rcx = (((rcx) as i64).wrapping_mul((crate::simd_ops::ru64(rbx.wrapping_add(P1))) as i64) as u64) as u64; rcx = ((rcx).wrapping_add(rbx)) as u64; rax = 0; fl_l = 0; fl_r = 0; let tmp_xchg: u64 = crate::simd_ops::atomic_xchg_u64(rcx.wrapping_add(P2), rax); rax = (tmp_xchg) as u64; rcx = ((rcx).wrapping_add(0x8u64)) as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rdi = ((rdi).wrapping_add(1)) as u64; rax = (crate::simd_ops::ru32(rbx.wrapping_add(P0))) as u32 as u64; fl_l = rdi; fl_r = rax; if !((((fl_l as u64)) < ((fl_r as u64)))) { break; } }
    rcx = rbx;
    rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp);
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141d9e6c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1401e84a0; const P1: u64 = 0x18; const P2: u64 = 0x14240aa00; const P3: u64 = 0x60; const P4: u64 = 0x64; const P5: u64 = 0x68; const P6: u64 = 0x6c;
    let (mut rax, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp);
    crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rcx = r8;
    rbp = r8;
    rsi = rdx;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rdx = ((rsi.wrapping_add(P1)) as u64) as u64;
    rcx = rbp.wrapping_add(0x18);
    rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp);
    rax = (crate::simd_ops::ru32(rsi.wrapping_add(P3))) as u32 as u64;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    crate::simd_ops::wu32(rbp.wrapping_add(0x60u64), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rsi.wrapping_add(P4))) as u32 as u64;
    crate::simd_ops::wu32(rbp.wrapping_add(0x64u64), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rsi.wrapping_add(P5))) as u32 as u64;
    crate::simd_ops::wu32(rbp.wrapping_add(0x68u64), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rsi.wrapping_add(P6))) as u32 as u64;
    rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64;
    crate::simd_ops::wu32(rbp.wrapping_add(0x6cu64), ((rax as u32)) as u32);
    rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141d9e770(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0x14; const P3: u64 = 0x18; const P4: u64 = 0x1c; const P5: u64 = 0x20; const P6: u64 = 0x1b; const P7: u64 = 0x64;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu64(rdx, rax);
    crate::simd_ops::wu64(rdx.wrapping_add(P0), rax);
    crate::simd_ops::wu32(rdx.wrapping_add(P1), ((rax as u32)) as u32);
    crate::simd_ops::wu8(rdx.wrapping_add(P2), ((rax as u8)) as u8);
    crate::simd_ops::lk_and_u32(rdx.wrapping_add(P3), (0xff000008u32) as u32);
    fl_l = (crate::simd_ops::lk_or_u32(rdx.wrapping_add(P3), (0x8u32) as u32)) as u32 as u64; fl_r = fl_l;
    crate::simd_ops::wu32(rdx.wrapping_add(P4), ((rax as u32)) as u32);
    crate::simd_ops::wu8(rdx.wrapping_add(P5), ((rax as u8)) as u8);
    crate::simd_ops::wu8(rdx.wrapping_add(P6), (0x8u8) as u8);
    crate::simd_ops::wu32(rdx.wrapping_add(P7), ((rax as u32)) as u32);
    rax = rdx;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141d9eed0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x14240aa00; const P2: u64 = 0x38; const P3: u64 = 0x48; const P4: u64 = 0x4c; const P5: u64 = 0x50; const P6: u64 = 0x54; const P7: u64 = 0x58; const P8: u64 = 0x60; const P9: u64 = 0x68; const P10: u64 = 0x69; const P11: u64 = 0x6a;
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rdx; rcx = ((r8.wrapping_add(P0)) as u64) as u64; rdx = ((rdx).wrapping_add(0x8u64)) as u64; rdi = r8; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rdx = ((rbx.wrapping_add(P2)) as u64) as u64; rcx = ((rdi.wrapping_add(P2)) as u64) as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru32(rbx.wrapping_add(P3))) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(P3), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rbx.wrapping_add(P4))) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(P4), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rbx.wrapping_add(P5))) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(P5), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rbx.wrapping_add(P6))) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(P6), ((rax as u32)) as u32); rax = ((crate::simd_ops::ru8(rbx.wrapping_add(P7))) as u32) as u32 as u64; crate::simd_ops::wu8(rdi.wrapping_add(P7), ((rax as u8)) as u8); rax = (crate::simd_ops::ru64(rbx.wrapping_add(P8))) as u64; crate::simd_ops::wu64(rdi.wrapping_add(P8), rax); rax = ((crate::simd_ops::ru8(rbx.wrapping_add(P9))) as u32) as u32 as u64; crate::simd_ops::wu8(rdi.wrapping_add(P9), ((rax as u8)) as u8); rax = ((crate::simd_ops::ru8(rbx.wrapping_add(P10))) as u32) as u32 as u64; crate::simd_ops::wu8(rdi.wrapping_add(P10), ((rax as u8)) as u8); rax = ((crate::simd_ops::ru8(rbx.wrapping_add(P11))) as u32) as u32 as u64; crate::simd_ops::wu8(rdi.wrapping_add(0x6au64), ((rax as u8)) as u8); rax = ((crate::simd_ops::ru8(rbx.wrapping_add(0x6bu64))) as u32) as u32 as u64; crate::simd_ops::wu8(rdi.wrapping_add(0x6bu64), ((rax as u8)) as u8); rax = ((crate::simd_ops::ru8(rbx.wrapping_add(0x6cu64))) as u32) as u32 as u64; crate::simd_ops::wu8(rdi.wrapping_add(0x6cu64), ((rax as u8)) as u8); rax = ((crate::simd_ops::ru8(rbx.wrapping_add(0x6du64))) as u32) as u32 as u64; crate::simd_ops::wu8(rdi.wrapping_add(0x6du64), ((rax as u8)) as u8); rax = ((crate::simd_ops::ru8(rbx.wrapping_add(0x6eu64))) as u32) as u32 as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu8(rdi.wrapping_add(0x6eu64), ((rax as u8)) as u8); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141d9f820(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0x18; const P3: u64 = 0x20; const P4: u64 = 0x28; const P5: u64 = 0x30; const P6: u64 = 0x40; const P7: u64 = 0x48; const P8: u64 = 0x1432fbf88;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu64(rdx.wrapping_add(P0), rax);
    crate::simd_ops::wu32(rdx.wrapping_add(P1), ((rax as u32)) as u32);
    crate::simd_ops::wu64(rdx.wrapping_add(P2), rax);
    crate::simd_ops::wu8(rdx.wrapping_add(P3), ((rax as u8)) as u8);
    crate::simd_ops::wu64(rdx.wrapping_add(P4), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(P5), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(P6), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(P7), rax);
    rax = P8;
    crate::simd_ops::wu64(rdx, rax);
    rax = rdx;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141d9ff40(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x14240aa00; const P2: u64 = 0x38; const P3: u64 = 0x48; const P4: u64 = 0x4c; const P5: u64 = 0x50; const P6: u64 = 0x54; const P7: u64 = 0x58; const P8: u64 = 0x60; const P9: u64 = 0x68; const P10: u64 = 0x70; const P11: u64 = 0x78;
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rdx; rcx = ((r8.wrapping_add(P0)) as u64) as u64; rdx = ((rdx).wrapping_add(0x8u64)) as u64; rdi = r8; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rdx = ((rbx.wrapping_add(P2)) as u64) as u64; rcx = ((rdi.wrapping_add(P2)) as u64) as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru32(rbx.wrapping_add(P3))) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(P3), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rbx.wrapping_add(P4))) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(P4), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rbx.wrapping_add(P5))) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(P5), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rbx.wrapping_add(P6))) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(P6), ((rax as u32)) as u32); rax = ((crate::simd_ops::ru8(rbx.wrapping_add(P7))) as u32) as u32 as u64; crate::simd_ops::wu8(rdi.wrapping_add(P7), ((rax as u8)) as u8); rax = (crate::simd_ops::ru64(rbx.wrapping_add(P8))) as u64; crate::simd_ops::wu64(rdi.wrapping_add(P8), rax); rax = (crate::simd_ops::ru64(rbx.wrapping_add(P9))) as u64; crate::simd_ops::wu64(rdi.wrapping_add(P9), rax); rax = (crate::simd_ops::ru64(rbx.wrapping_add(P10))) as u64; crate::simd_ops::wu64(rdi.wrapping_add(P10), rax); rax = (crate::simd_ops::ru64(rbx.wrapping_add(P11))) as u64; crate::simd_ops::wu64(rdi.wrapping_add(0x78u64), rax); rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x80u64))) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x80u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x84u64))) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x84u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x88u64))) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x88u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x8cu64))) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x8cu64), ((rax as u32)) as u32); rax = ((crate::simd_ops::ru8(rbx.wrapping_add(0x90u64))) as u32) as u32 as u64; crate::simd_ops::wu8(rdi.wrapping_add(0x90u64), ((rax as u8)) as u8); rax = ((crate::simd_ops::ru8(rbx.wrapping_add(0x91u64))) as u32) as u32 as u64; crate::simd_ops::wu8(rdi.wrapping_add(0x91u64), ((rax as u8)) as u8); rax = ((crate::simd_ops::ru8(rbx.wrapping_add(0x92u64))) as u32) as u32 as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu8(rdi.wrapping_add(0x92u64), ((rax as u8)) as u8); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141da0050(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x1432fc038; const P2: u64 = 0xb; const P3: u64 = 0xc; const P4: u64 = 0x10; const P5: u64 = 0x38; const P6: u64 = 0x3b; const P7: u64 = 0x3c; const P8: u64 = 0x40; const P9: u64 = 0x48; const P10: u64 = 0x60; const P11: u64 = 0x68;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    crate::simd_ops::lk_and_u32(rdx.wrapping_add(P0), (0xff000005u32) as u32);
    rax = P1;
    crate::simd_ops::lk_or_u32(rdx.wrapping_add(P0), (0x5u32) as u32);
    rcx = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu8(rdx.wrapping_add(P2), (0x5u8) as u8);
    crate::simd_ops::wu32(rdx.wrapping_add(P3), ((rcx as u32)) as u32);
    crate::simd_ops::wu8(rdx.wrapping_add(P4), ((rcx as u8)) as u8);
    crate::simd_ops::lk_and_u32(rdx.wrapping_add(P5), (0xff000001u32) as u32);
    fl_l = (crate::simd_ops::lk_or_u32(rdx.wrapping_add(P5), (0x1u32) as u32)) as u32 as u64; fl_r = fl_l;
    crate::simd_ops::wu8(rdx.wrapping_add(P6), (0x1u8) as u8);
    crate::simd_ops::wu32(rdx.wrapping_add(P7), ((rcx as u32)) as u32);
    crate::simd_ops::wu8(rdx.wrapping_add(P8), ((rcx as u8)) as u8);
    crate::simd_ops::wu64(rdx, rax);
    rax = rdx;
    crate::simd_ops::wu32(rdx.wrapping_add(P9), ((rcx as u32)) as u32);
    crate::simd_ops::wu64(rdx.wrapping_add(P10), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(P11), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(0x70u64), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(0x78u64), rcx);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141da06a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x140948050; const P2: u64 = 0x10; const P3: u64 = 0x140e65ce0; const P4: u64 = 0x20;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = r8; rbx = rdx; fl_l = r8; fl_r = rdx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rdx = (crate::simd_ops::ru64(rdx)) as u64; rax = (crate::simd_ops::ru32(rbx.wrapping_add(P0))) as u32 as u64; r8 = (((rax) as i64).wrapping_mul((0x88u64) as i64) as u64) as u64; r8 = ((r8).wrapping_add(rdx)) as u64; rcx = rdi; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 2; continue; }
            2 => { rax = ((rbx.wrapping_add(P2)) as u64) as u64; rcx = ((rdi.wrapping_add(P2)) as u64) as u64; fl_l = rcx; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rdx = (crate::simd_ops::ru64(rax)) as u64; r8 = (crate::simd_ops::ru32(rax.wrapping_add(P0))) as u32 as u64; r8 = ((r8).wrapping_shl(4u32)) as u64; r8 = ((r8).wrapping_add(rdx)) as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 4; continue; }
            4 => { rbx = ((rbx).wrapping_add(0x20u64)) as u64; rcx = ((rdi.wrapping_add(P4)) as u64) as u64; fl_l = rcx; fl_r = rbx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rdx = (crate::simd_ops::ru64(rbx)) as u64; rax = (crate::simd_ops::ru32(rbx.wrapping_add(P0))) as u32 as u64; r8 = (((rax) as i64).wrapping_mul((0x88u64) as i64) as u64) as u64; r8 = ((r8).wrapping_add(rdx)) as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 6; continue; }
            6 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141da15b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1401e1560; const P1: u64 = 0x1434268d0; const P2: u64 = 0x142645878;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut xmm6: u128 = 0; let mut xmm7: u128 = 0; let mut xmm8: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 15>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = rsp; crate::simd_ops::wu64(rax.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rax.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rax.wrapping_add(0x20u64), r9); crate::simd_ops::wu64(rax.wrapping_add(0x18u64), r8); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x50u64)) as u64; crate::simd_ops::wu128(rax.wrapping_sub(0x38u64), (xmm6) as u128); crate::simd_ops::wu128(rax.wrapping_sub(0x48u64), (xmm7) as u128); crate::simd_ops::wu128(rax.wrapping_sub(0x58u64), (xmm8) as u128); rsi = r9; rdi = r8; xmm6 = (xmm0) as u128; r15 = (crate::simd_ops::ru64(rdx)) as u64; fl_l = r15; fl_r = r15; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 7; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xa0u64))) as u64; xmm6 = ((crate::simd_ops::ru32(rax)) as u32) as u128; rcx = (crate::simd_ops::ru64(r8.wrapping_add(0x38u64))) as u64; bb = 2; continue; }
            2 => { rbx = 0; fl_l = 0; fl_r = 0; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { fl_l = rcx; fl_r = rdi; rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = (rdx & !0xffu64) | (((if (((fl_l as u64)) != ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x20u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rdi.wrapping_add(0x38u64), rbx); bb = 4; continue; }
            4 => { rcx = (crate::simd_ops::ru64(rsi.wrapping_add(0x38u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { fl_l = rcx; fl_r = rsi; rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = (rdx & !0xffu64) | (((if (((fl_l as u64)) != ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x20u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rsi.wrapping_add(0x38u64), rbx); bb = 6; continue; }
            6 => { xmm0 = (xmm6) as u128; r11 = rsp.wrapping_add(0x50); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(r11.wrapping_add(0x38u64))) as u64; xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x40u64))) as u128; xmm7 = (crate::simd_ops::ru128(rsp.wrapping_add(0x30u64))) as u128; xmm8 = (crate::simd_ops::ru128(r11.wrapping_sub(0x30u64))) as u128; rsp = r11; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            7 => { r13 = (crate::simd_ops::ru64(rdx.wrapping_add(0x8u64))) as u64; fl_l = r15; fl_r = (0x1u64) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 11; continue; } bb = 8; continue; }
            8 => { rcx = (crate::simd_ops::ru64(r9.wrapping_add(0x38u64))) as u64; bb = 9; continue; }
            9 => { fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 33; continue; } bb = 10; continue; }
            10 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = r13; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm6 = (xmm0) as u128; rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x38u64))) as u64; bb = 2; continue; }
            11 => { rbx = r15; rbx = ((rbx).wrapping_shl(5u32)) as u64; rcx = (crate::simd_ops::ru64(r8.wrapping_add(0x38u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 33; continue; } bb = 12; continue; }
            12 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = r13; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = ((xmm6 as u32)) as u32 as u64; if !(f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { bb = 14; continue; } bb = 13; continue; }
            13 => { rcx = (crate::simd_ops::ru64(rsi.wrapping_add(0x38u64))) as u64; bb = 9; continue; }
            14 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x38u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 33; continue; } bb = 15; continue; }
            15 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = r13.wrapping_sub(0x20); rdx = ((rdx).wrapping_add(rbx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((xmm6 as u32)) as u32 as u64; fl_r = ((xmm0 as u32)) as u32 as u64; if !(f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { bb = 19; continue; } bb = 16; continue; }
            16 => { rcx = (crate::simd_ops::ru64(rsi.wrapping_add(0x38u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 33; continue; } bb = 17; continue; }
            17 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = r13.wrapping_sub(0x20); rdx = ((rdx).wrapping_add(rbx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm6 = (xmm0) as u128; bb = 18; continue; }
            18 => { rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 6; continue; }
            19 => { rbx = 0; fl_l = 0; fl_r = 0; fl_l = r15; fl_r = (0x1u64) as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 24; continue; } bb = 20; continue; }
            20 => { rbp = ((rbx.wrapping_add(0x1u64)) as u32) as u32 as u64; r14 = r13.wrapping_add(0x20); bb = 21; continue; }
            21 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x38u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 33; continue; } bb = 22; continue; }
            22 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = r14; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((xmm6 as u32)) as u32 as u64; fl_r = ((xmm0 as u32)) as u32 as u64; if !(f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 24; continue; } bb = 23; continue; }
            23 => { rbx = ((rbx).wrapping_add(1)) as u64; r14 = ((r14).wrapping_add(0x20u64)) as u64; rbp = ((rbp).wrapping_add(1)) as u64; fl_l = rbp; fl_r = r15; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 21; continue; } bb = 24; continue; }
            24 => { rbx = ((rbx).wrapping_shl(5u32)) as u64; rbx = ((rbx).wrapping_add(r13)) as u64; rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x38u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 33; continue; } bb = 25; continue; }
            25 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm8 = (xmm0) as u128; rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x38u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 33; continue; } bb = 26; continue; }
            26 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = rbx.wrapping_add(0x20); (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm7 = (xmm0) as u128; rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x38u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 33; continue; } bb = 27; continue; }
            27 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm2 = (xmm7 & !0xffff_ffffu128) | ((f32::from_bits(xmm7 as u32) - f32::from_bits(xmm0 as u32)).to_bits() as u128); xmm1 = 0u128; xmm7 = ((crate::simd_ops::ru32(P1)) as u32) as u128; fl_l = ((xmm2 as u32)) as u32 as u64; fl_r = ((xmm1 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)).is_nan() || f32::from_bits((fl_r as u32)).is_nan()) { bb = 29; continue; } bb = 28; continue; }
            28 => { if (f32::from_bits((fl_l as u32)) == f32::from_bits((fl_r as u32)) || f32::from_bits((fl_l as u32)).is_nan() || f32::from_bits((fl_r as u32)).is_nan()) { bb = 30; continue; } bb = 29; continue; }
            29 => { xmm1 = (xmm7 & !0xffff_ffffu128) | ((f32::from_bits(xmm7 as u32) / f32::from_bits(xmm2 as u32)).to_bits() as u128); bb = 30; continue; }
            30 => { xmm0 = (xmm6 & !0xffff_ffffu128) | ((f32::from_bits(xmm6 as u32) - f32::from_bits(xmm8 as u32)).to_bits() as u128); xmm6 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm1 as u32)).to_bits() as u128); rcx = (crate::simd_ops::ru64(rsi.wrapping_add(0x38u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 33; continue; } bb = 31; continue; }
            31 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = rbx.wrapping_add(0x20); (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm8 = (xmm0) as u128; rcx = (crate::simd_ops::ru64(rsi.wrapping_add(0x38u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 33; continue; } bb = 32; continue; }
            32 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm1 = (xmm7 & !0xffff_ffffu128) | ((f32::from_bits(xmm7 as u32) - f32::from_bits(xmm6 as u32)).to_bits() as u128); xmm2 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(xmm0 as u32)).to_bits() as u128); xmm0 = (xmm8 & !0xffff_ffffu128) | ((f32::from_bits(xmm8 as u32) * f32::from_bits(xmm6 as u32)).to_bits() as u128); xmm6 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) + f32::from_bits(xmm0 as u32)).to_bits() as u128); bb = 18; continue; }
            33 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::lift_ops::trap(0x80000003, 0x141da1802, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141da1d90(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x2e4; const P1: u64 = 0x2d8; const P2: u64 = 0x2dc;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 3>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rsp = ((rsp).wrapping_sub(0x18u64)) as u64;
    fl_l = ((xmm2 as u32)) as u32 as u64; fl_r = ((xmm1 as u32)) as u32 as u64;
    if !(!(f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32)))) { crate::simd_ops::wu32(rsp, ((xmm1 as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x4u64), ((xmm2 as u32)) as u32); rax = (crate::simd_ops::ru64(rsp)) as u64; crate::simd_ops::wu64(rcx.wrapping_add(P0), rax); rax = (((r9 as u8)) as u32) as u32 as u64; rax = (((rax as u32)).wrapping_add(1)) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(P1), ((rax as u32)) as u32); rsp = ((rsp).wrapping_add(0x18u64)) as u64; return rax; }
    fl_l = ((xmm1 as u32)) as u32 as u64; fl_r = ((xmm2 as u32)) as u32 as u64;
    if !(!(f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32)))) { crate::simd_ops::wu32(rsp, ((xmm2 as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x4u64), ((xmm1 as u32)) as u32); rax = (crate::simd_ops::ru64(rsp)) as u64; crate::simd_ops::wu64(rcx.wrapping_add(P2), rax); rax = (((r9 as u8)) as u32) as u32 as u64; fl_l = (((rax as u32)).wrapping_add(1)) as u32 as u64; rax = ((fl_l as u32)) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(P1), ((rax as u32)) as u32); }
    rsp = ((rsp).wrapping_add(0x18u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141da1e00(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xc8; const P1: u64 = 0xd8; const P2: u64 = 0xdc; const P3: u64 = 0xe0; const P4: u64 = 0xd0; const P5: u64 = 0xe4; const P6: u64 = 0x120; const P7: u64 = 0x194; const P8: u64 = 0x143426abc;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 3>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x18u64)) as u64; fl_l = ((rdx as u8)) as u8 as u64; fl_r = ((rdx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rcx.wrapping_add(P0), (0x4u64) as u64); rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rcx.wrapping_add(P1), (0x3f800000u32) as u32); crate::simd_ops::wu32(rcx.wrapping_add(P2), (0x3f800000u32) as u32); crate::simd_ops::wu32(rcx.wrapping_add(P3), (0x3f800000u32) as u32); crate::simd_ops::wu64(rcx.wrapping_add(P4), rax); crate::simd_ops::wu8(rcx.wrapping_add(P5), (0x1u8) as u8); crate::simd_ops::wu8(rcx.wrapping_add(P6), (0x1u8) as u8); rsp = ((rsp).wrapping_add(0x18u64)) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            2 => { xmm0 = ((crate::simd_ops::ru32(rcx.wrapping_add(P1))) as u32) as u128; xmm1 = 0u128; fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = ((xmm1 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)).is_nan() || f32::from_bits((fl_r as u32)).is_nan()) { bb = 9; continue; } bb = 3; continue; }
            3 => { if (f32::from_bits((fl_l as u32)) != f32::from_bits((fl_r as u32)) && !f32::from_bits((fl_l as u32)).is_nan() && !f32::from_bits((fl_r as u32)).is_nan()) { bb = 9; continue; } bb = 4; continue; }
            4 => { xmm0 = ((crate::simd_ops::ru32(rcx.wrapping_add(P2))) as u32) as u128; fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = ((xmm1 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)).is_nan() || f32::from_bits((fl_r as u32)).is_nan()) { bb = 9; continue; } bb = 5; continue; }
            5 => { if (f32::from_bits((fl_l as u32)) != f32::from_bits((fl_r as u32)) && !f32::from_bits((fl_l as u32)).is_nan() && !f32::from_bits((fl_r as u32)).is_nan()) { bb = 9; continue; } bb = 6; continue; }
            6 => { xmm0 = ((crate::simd_ops::ru32(rcx.wrapping_add(P3))) as u32) as u128; fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = ((xmm1 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)).is_nan() || f32::from_bits((fl_r as u32)).is_nan()) { bb = 9; continue; } bb = 7; continue; }
            7 => { if (f32::from_bits((fl_l as u32)) != f32::from_bits((fl_r as u32)) && !f32::from_bits((fl_l as u32)).is_nan() && !f32::from_bits((fl_r as u32)).is_nan()) { bb = 9; continue; } bb = 8; continue; }
            8 => { rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rcx.wrapping_add(P0), ((rax as u32)) as u32); crate::simd_ops::wu8(rcx.wrapping_add(P7), ((rax as u8)) as u8); rsp = ((rsp).wrapping_add(0x18u64)) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            9 => { xmm3 = ((crate::simd_ops::ru32(P8)) as u32) as u128; xmm0 = ((crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64) as u128; crate::simd_ops::wu64(rsp, ((xmm0 as u64)) as u64); xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm3 as u32)).to_bits() as u128); crate::simd_ops::wu32(rcx.wrapping_add(P1), ((xmm0 as u32)) as u32); xmm2 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) * f32::from_bits(crate::simd_ops::ru32(rsp.wrapping_add(0x4u64)))).to_bits() as u128); crate::simd_ops::wu32(rcx.wrapping_add(P2), ((xmm2 as u32)) as u32); xmm1 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) * f32::from_bits(crate::simd_ops::ru32(rcx.wrapping_add(P3)))).to_bits() as u128); crate::simd_ops::wu32(rcx.wrapping_add(P3), ((xmm1 as u32)) as u32); rax = 0x1; crate::simd_ops::wu32(rcx.wrapping_add(P0), ((rax as u32)) as u32); crate::simd_ops::wu8(rcx.wrapping_add(P7), (0x0u8) as u8); rsp = ((rsp).wrapping_add(0x18u64)) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141da1ef0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1401f2bc0; const P1: u64 = 0xb0; const P2: u64 = 0x110; const P3: u64 = 0x1401de830; const P4: u64 = 0x180; const P5: u64 = 0x220; const P6: u64 = 0x260; const P7: u64 = 0x2a8; const P8: u64 = 0x298;
    let (mut rax, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rcx;
    rcx = ((rcx).wrapping_add(0xa0u64)) as u64;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rcx = ((rbx.wrapping_add(P1)) as u64) as u64;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rcx = ((rbx.wrapping_add(P2)) as u64) as u64;
    rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp);
    rcx = ((rbx.wrapping_add(P4)) as u64) as u64;
    rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp);
    rcx = ((rbx.wrapping_add(P5)) as u64) as u64;
    rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp);
    rcx = ((rbx.wrapping_add(P6)) as u64) as u64;
    rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp);
    rcx = ((rbx.wrapping_add(P7)) as u64) as u64;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rcx = ((rbx.wrapping_add(P8)) as u64) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return crate::simd_ops::tail_stk(P0, rax, rcx, rdx, r8, r9, rsp);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141da1fa0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14241c0a0; const P1: u64 = 0x140208500; const P2: u64 = 0x140200130;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x50u64)) as u64; r15 = rcx; rbx = (crate::simd_ops::ru64(rdx.wrapping_add(0x20u64))) as u64; rdi = (crate::simd_ops::ru32(rdx.wrapping_add(0x28u64))) as u32 as u64; rdi = ((rdi).wrapping_shl(5u32)) as u64; rdi = ((rdi).wrapping_add(rbx)) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 13; continue; } bb = 1; continue; }
            1 => { rbp = 0; fl_l = 0; fl_r = 0; r12 = (0x7ffffffffffffffu64) as u64; bb = 2; continue; }
            2 => { r14 = (crate::simd_ops::ru64(r15)) as u64; rdx = (crate::simd_ops::ru64(r14)) as u64; rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rcx); crate::simd_ops::wu32(rsp.wrapping_add(0x38u64), ((rbp as u32)) as u32); fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0x19u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 8; continue; } bb = 3; continue; }
            3 => { xmm0 = ((crate::simd_ops::ru32(rbx)) as u32) as u128; bb = 4; continue; }
            4 => { crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rcx); fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rcx.wrapping_add(0x1cu64))) as u32 as u64; rax = (rax & !0xffu64) | (((if (f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { 1u8 } else { 0u8 }) as u8 as u64)); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { crate::simd_ops::wu32(rsp.wrapping_add(0x38u64), ((rbp as u32)) as u32); rcx = (crate::simd_ops::ru64(rcx.wrapping_add(0x10u64))) as u64; bb = 7; continue; }
            6 => { crate::simd_ops::wu32(rsp.wrapping_add(0x38u64), (0x1u32) as u32); rdx = rcx; rcx = (crate::simd_ops::ru64(rcx)) as u64; bb = 7; continue; }
            7 => { fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0x19u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 4; continue; } bb = 8; continue; }
            8 => { fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(0x19u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 10; continue; } bb = 9; continue; }
            9 => { xmm0 = ((crate::simd_ops::ru32(rdx.wrapping_add(0x1cu64))) as u32) as u128; fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rbx)) as u32 as u64; rax = (rax & !0xffu64) | (((if (f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { 1u8 } else { 0u8 }) as u8 as u64)); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 12; continue; } bb = 10; continue; }
            10 => { fl_l = (crate::simd_ops::ru64(r14.wrapping_add(0x8u64))) as u64; fl_r = r12; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 14; continue; } bb = 11; continue; }
            11 => { rsi = (crate::simd_ops::ru64(r14)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r14); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rbp); rdx = 0x8; rcx = ((rdx.wrapping_add(0x18u64)) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r8 = rax; rax = (crate::simd_ops::ru32(rbx)) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(0x1cu64), ((rax as u32)) as u32); crate::simd_ops::wu64(r8, rsi); crate::simd_ops::wu64(r8.wrapping_add(0x8u64), rsi); crate::simd_ops::wu64(r8.wrapping_add(0x10u64), rsi); crate::simd_ops::wu16(r8.wrapping_add(0x18u64), (0x0u16) as u16); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rbp); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x30u64))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm0) as u128); rdx = rsp.wrapping_add(0x30); rcx = r14; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 12; continue; }
            12 => { rbx = ((rbx).wrapping_add(0x20u64)) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 13; continue; }
            13 => { r11 = rsp.wrapping_add(0x50); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x20u64))) as u64; rbp = (crate::simd_ops::ru64(r11.wrapping_add(0x28u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x30u64))) as u64; rdi = (crate::simd_ops::ru64(r11.wrapping_add(0x38u64))) as u64; rsp = r11; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            14 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::lift_ops::trap(0x80000003, 0x141da20df, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141da20f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14241c0a0; const P1: u64 = 0x141da1fa0; const P2: u64 = 0x1432fc590; const P3: u64 = 0x1432fc558; const P4: u64 = 0x1434268d0; const P5: u64 = 0x141da15b0; const P6: u64 = 0x140268f50; const P7: u64 = 0x140e69270; const P8: u64 = 0x1403e7e10; const P9: u64 = 0x14241c190;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut xmm6: u128 = 0; let mut xmm7: u128 = 0; let mut xmm8: u128 = 0; let mut xmm9: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 51>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = rsp; crate::simd_ops::wu64(rax.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rax.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rax.wrapping_add(0x20u64), rdi); crate::simd_ops::wu64(rax.wrapping_add(0x8u64), rcx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rbp = rsp.wrapping_sub(0x70); rsp = ((rsp).wrapping_sub(0x170u64)) as u64; crate::simd_ops::wu128(rax.wrapping_sub(0x38u64), (xmm6) as u128); crate::simd_ops::wu128(rax.wrapping_sub(0x48u64), (xmm7) as u128); crate::simd_ops::wu128(rax.wrapping_sub(0x58u64), (xmm8) as u128); crate::simd_ops::wu128(rax.wrapping_sub(0x68u64), (xmm9) as u128); rsi = r9; r14 = r8; r15 = rdx; rdi = rcx; rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rsp.wrapping_add(0x60u64), ((rcx as u32)) as u32); crate::simd_ops::wu64(rdi, rcx); crate::simd_ops::wu32(rdi.wrapping_add(0x8u64), ((rcx as u32)) as u32); crate::simd_ops::wu32(rdi.wrapping_add(0xcu64), ((rcx as u32)) as u32); crate::simd_ops::wu64(rdi.wrapping_add(0x10u64), rcx); crate::simd_ops::wu64(rdi.wrapping_add(0x18u64), rcx); crate::simd_ops::wu32(rsp.wrapping_add(0x60u64), (0x1u32) as u32); xmm0 = 0u128; crate::simd_ops::wu128(rsp.wrapping_add(0x50u64), (xmm0) as u128); rdx = ((rcx.wrapping_add(0x8u64)) as u32) as u32 as u64; rcx = ((rdx.wrapping_add(0x18u64)) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rax, rax); crate::simd_ops::wu64(rax.wrapping_add(0x8u64), rax); crate::simd_ops::wu64(rax.wrapping_add(0x10u64), rax); crate::simd_ops::wu16(rax.wrapping_add(0x18u64), (0x101u16) as u16); crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rax); rax = rsp.wrapping_add(0x50); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); rdx = r15; rcx = rsp.wrapping_add(0x30); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = r14; rcx = rsp.wrapping_add(0x30); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = rsi; rcx = rsp.wrapping_add(0x30); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rbx = (crate::simd_ops::ru64(rcx)) as u64; fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(0x19u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 15; continue; } bb = 1; continue; }
            1 => { r12 = P2; r13 = P3; xmm9 = ((crate::simd_ops::ru32(P4)) as u32) as u128; bb = 2; continue; }
            2 => { xmm6 = ((crate::simd_ops::ru32(rbx.wrapping_add(0x1cu64))) as u32) as u128; crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((xmm9 as u32)) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x78u64), r12); rax = rsp.wrapping_add(0x78); crate::simd_ops::wu64(rbp.wrapping_sub(0x50u64), rax); crate::simd_ops::wu64(rbp.wrapping_sub(0x48u64), r13); rax = rbp.wrapping_sub(0x48); crate::simd_ops::wu64(rbp.wrapping_sub(0x10u64), rax); rax = (crate::simd_ops::ru32(r15.wrapping_add(0x28u64))) as u32 as u64; rcx = (crate::simd_ops::ru64(r15.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(rbp, rax); crate::simd_ops::wu64(rbp.wrapping_add(0x8u64), rcx); xmm0 = (crate::simd_ops::ru128(rbp)) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm0) as u128); rax = rsp.wrapping_add(0x30); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); r9 = rsp.wrapping_add(0x78); r8 = rbp.wrapping_sub(0x48); rdx = rsp.wrapping_add(0x40); xmm0 = (xmm6) as u128; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm7 = (xmm0) as u128; crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((xmm9 as u32)) as u32); crate::simd_ops::wu64(rbp.wrapping_sub(0x48u64), r12); rax = rbp.wrapping_sub(0x48); crate::simd_ops::wu64(rbp.wrapping_sub(0x10u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x78u64), r13); rax = rsp.wrapping_add(0x78); crate::simd_ops::wu64(rbp.wrapping_sub(0x50u64), rax); rax = (crate::simd_ops::ru32(r14.wrapping_add(0x28u64))) as u32 as u64; rcx = (crate::simd_ops::ru64(r14.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x10u64), rax); crate::simd_ops::wu64(rbp.wrapping_add(0x18u64), rcx); xmm0 = (crate::simd_ops::ru128(rbp.wrapping_add(0x10u64))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm0) as u128); rax = rsp.wrapping_add(0x30); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); r9 = rbp.wrapping_sub(0x48); r8 = rsp.wrapping_add(0x78); rdx = rsp.wrapping_add(0x40); xmm0 = (xmm6) as u128; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm8 = (xmm0) as u128; crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((xmm9 as u32)) as u32); crate::simd_ops::wu64(rbp.wrapping_sub(0x48u64), r12); rax = rbp.wrapping_sub(0x48); crate::simd_ops::wu64(rbp.wrapping_sub(0x10u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x78u64), r13); rax = rsp.wrapping_add(0x78); crate::simd_ops::wu64(rbp.wrapping_sub(0x50u64), rax); rax = (crate::simd_ops::ru32(rsi.wrapping_add(0x28u64))) as u32 as u64; rcx = (crate::simd_ops::ru64(rsi.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x20u64), rax); crate::simd_ops::wu64(rbp.wrapping_add(0x28u64), rcx); xmm0 = (crate::simd_ops::ru128(rbp.wrapping_add(0x20u64))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm0) as u128); rax = rsp.wrapping_add(0x30); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); r9 = rbp.wrapping_sub(0x48); r8 = rsp.wrapping_add(0x78); rdx = rsp.wrapping_add(0x40); xmm0 = (xmm6) as u128; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu32(rsp.wrapping_add(0x74u64), ((xmm6 as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x68u64), ((xmm7 as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x6cu64), ((xmm8 as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x70u64), ((xmm0 as u32)) as u32); r8 = (crate::simd_ops::ru32(rdi.wrapping_add(0x8u64))) as u32 as u64; rdx = ((r8 as u32)) as u32 as u64; rdx = ((rdx).wrapping_shl(4u32)) as u64; rax = (crate::simd_ops::ru32(rdi.wrapping_add(0xcu64))) as u32 as u64; rax = ((((rax as u32)) & (0x3fffffffu32))) as u32 as u64; rcx = (crate::simd_ops::ru64(rdi)) as u64; fl_l = ((r8 as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rcx = ((rcx).wrapping_add(rdx)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rcx); r8 = rsp.wrapping_add(0x68); rdx = rsp.wrapping_add(0x30); rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 5; continue; }
            4 => { rax = ((r8.wrapping_add(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x8u64), ((rax as u32)) as u32); crate::simd_ops::wu32(rdx.wrapping_add(rcx), ((xmm7 as u32)) as u32); crate::simd_ops::wu32(rdx.wrapping_add(rcx).wrapping_add(0x4u64), ((xmm8 as u32)) as u32); crate::simd_ops::wu32(rdx.wrapping_add(rcx).wrapping_add(0x8u64), ((xmm0 as u32)) as u32); crate::simd_ops::wu32(rdx.wrapping_add(rcx).wrapping_add(0xcu64), ((xmm6 as u32)) as u32); bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; fl_l = (crate::simd_ops::ru8(rax.wrapping_add(0x19u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 10; continue; } bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; fl_l = (crate::simd_ops::ru8(rax.wrapping_add(0x19u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 9; continue; } bb = 7; continue; }
            7 => { fl_l = rbx; fl_r = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 9; continue; } bb = 8; continue; }
            8 => { rbx = rax; rax = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; fl_l = (crate::simd_ops::ru8(rax.wrapping_add(0x19u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 7; continue; } bb = 9; continue; }
            9 => { rbx = rax; bb = 13; continue; }
            10 => { rbx = rax; rcx = (crate::simd_ops::ru64(rax)) as u64; fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0x19u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 13; continue; } bb = 11; continue; }
            11 => { bb = 12; continue; }
            12 => { rbx = rcx; rax = (crate::simd_ops::ru64(rcx)) as u64; rcx = rax; fl_l = (crate::simd_ops::ru8(rax.wrapping_add(0x19u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 12; continue; } bb = 13; continue; }
            13 => { fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(0x19u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 2; continue; } bb = 14; continue; }
            14 => { rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; bb = 15; continue; }
            15 => { rax = (crate::simd_ops::ru64(rbp.wrapping_add(0xc0u64))) as u64; rbx = (crate::simd_ops::ru64(rax.wrapping_add(0x20u64))) as u64; r14 = (crate::simd_ops::ru32(rax.wrapping_add(0x28u64))) as u32 as u64; r14 = ((r14).wrapping_shl(5u32)) as u64; r14 = ((r14).wrapping_add(rbx)) as u64; fl_l = rbx; fl_r = r14; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 22; continue; } bb = 16; continue; }
            16 => { bb = 17; continue; }
            17 => { xmm0 = ((crate::simd_ops::ru32(rbx)) as u32) as u128; crate::simd_ops::wu32(rbp.wrapping_add(0xc4u64), ((xmm0 as u32)) as u32); xmm1 = ((crate::simd_ops::ru32(rbx.wrapping_add(0x4u64))) as u32) as u128; crate::simd_ops::wu32(rbp.wrapping_add(0xc0u64), ((xmm1 as u32)) as u32); rcx = (crate::simd_ops::ru32(rdi.wrapping_add(0x18u64))) as u32 as u64; rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x10u64))) as u64; rdx = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x1cu64))) as u32 as u64; rax = ((((rax as u32)) & (0x3fffffffu32))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 19; continue; } bb = 18; continue; }
            18 => { crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rdx); r8 = rbp.wrapping_add(0xc0); rdx = rsp.wrapping_add(0x30); rcx = rdi.wrapping_add(0x10); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 20; continue; }
            19 => { rax = ((rcx.wrapping_add(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x18u64), ((rax as u32)) as u32); xmm0 = crate::simd_ops::vunpcklps(xmm1, xmm0); crate::simd_ops::wu64(rbp.wrapping_add(0xc0u64), ((xmm0 as u64)) as u64); crate::simd_ops::wu64(rdx, ((xmm0 as u64)) as u64); bb = 20; continue; }
            20 => { rbx = ((rbx).wrapping_add(0x20u64)) as u64; fl_l = rbx; fl_r = r14; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 17; continue; } bb = 21; continue; }
            21 => { rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; bb = 22; continue; }
            22 => { rbx = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(0x19u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 28; continue; } bb = 23; continue; }
            23 => { bb = 24; continue; }
            24 => { r8 = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; rdx = rsp.wrapping_add(0x50); rcx = rsp.wrapping_add(0x50); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P8, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rbx; rbx = (crate::simd_ops::ru64(rbx)) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 26; continue; } bb = 25; continue; }
            25 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P9, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 26; continue; }
            26 => { fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(0x19u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 24; continue; } bb = 27; continue; }
            27 => { rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; bb = 28; continue; }
            28 => { fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 30; continue; } bb = 29; continue; }
            29 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P9, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 30; continue; }
            30 => { rax = rdi; r11 = rsp.wrapping_add(0x170); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x40u64))) as u64; rdi = (crate::simd_ops::ru64(r11.wrapping_add(0x48u64))) as u64; xmm6 = (crate::simd_ops::ru128(r11.wrapping_sub(0x10u64))) as u128; xmm7 = (crate::simd_ops::ru128(r11.wrapping_sub(0x20u64))) as u128; xmm8 = (crate::simd_ops::ru128(r11.wrapping_sub(0x30u64))) as u128; xmm9 = (crate::simd_ops::ru128(r11.wrapping_sub(0x40u64))) as u128; rsp = r11; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141da2510(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1434268d0; const P1: u64 = 0x1432fc5c8; const P2: u64 = 0x1432fc600; const P3: u64 = 0x18; const P4: u64 = 0x10; const P5: u64 = 0x141da1810;
    let (mut rax, mut r11) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 25>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    r11 = rsp;
    rsp = ((rsp).wrapping_sub(0xc8u64)) as u64;
    xmm0 = ((crate::simd_ops::ru32(P0)) as u32) as u128;
    crate::simd_ops::wu32(r11.wrapping_add(0x8u64), ((xmm0 as u32)) as u32);
    rax = P1;
    crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rax);
    r9 = rsp.wrapping_add(0x40);
    rax = rsp.wrapping_add(0x40);
    crate::simd_ops::wu64(r11.wrapping_sub(0x50u64), rax);
    r8 = r11.wrapping_sub(0x48);
    rax = P2;
    crate::simd_ops::wu64(r11.wrapping_sub(0x48u64), rax);
    rdx = rsp.wrapping_add(0x30);
    rax = r11.wrapping_sub(0x48);
    crate::simd_ops::wu64(r11.wrapping_sub(0x10u64), rax);
    rax = (crate::simd_ops::ru32(rcx.wrapping_add(P3))) as u32 as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax);
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P4))) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rax);
    rax = r11.wrapping_add(0x8);
    xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x30u64))) as u128;
    crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm0) as u128);
    xmm0 = (xmm1) as u128;
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax);
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    rsp = ((rsp).wrapping_add(0xc8u64)) as u64;
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141da25d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1434268d0; const P1: u64 = 0x1432fc4e8; const P2: u64 = 0x1432fc520; const P3: u64 = 0x8; const P4: u64 = 0x141da1a60;
    let (mut rax, mut r11, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 27>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { r11 = rsp; rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0xd0u64)) as u64; xmm0 = ((crate::simd_ops::ru32(P0)) as u32) as u128; rax = P1; rbx = rcx; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rax); r9 = r11.wrapping_sub(0x48); rax = rsp.wrapping_add(0x50); crate::simd_ops::wu64(r11.wrapping_sub(0x50u64), rax); r8 = rsp.wrapping_add(0x40); rax = P2; crate::simd_ops::wu64(r11.wrapping_sub(0x48u64), rax); rax = r11.wrapping_sub(0x48); crate::simd_ops::wu64(r11.wrapping_sub(0x10u64), rax); rax = (crate::simd_ops::ru32(rdx.wrapping_add(P3))) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rax); rax = (crate::simd_ops::ru64(rdx)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rax); rax = rsp.wrapping_add(0x30); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax); rax = rsp.wrapping_add(0x50); crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((xmm0 as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x34u64), ((xmm0 as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x38u64), ((xmm0 as u32)) as u32); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x40u64))) as u128; xmm1 = (xmm2) as u128; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm0) as u128); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = rbx; rsp = ((rsp).wrapping_add(0xd0u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141da26c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x140e68b90; const P2: u64 = 0x10; const P3: u64 = 0x18; const P4: u64 = 0x140e68f00;
    let (mut rax, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    rbx = rcx;
    r8 = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64;
    rcx = rsp.wrapping_add(0x38);
    rdx = ((r8 as u32)) as u32 as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rax);
    rdx = ((rdx).wrapping_shl(4u32)) as u64;
    rdx = ((rdx).wrapping_add(rax)) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rdx);
    rdx = rsp.wrapping_add(0x30);
    r9 = ((crate::simd_ops::ru8(rsp.wrapping_add(0x30u64))) as u32) as u32 as u64;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rax = (crate::simd_ops::ru64(rbx.wrapping_add(P2))) as u64;
    rdx = rsp.wrapping_add(0x30);
    r8 = (crate::simd_ops::ru32(rbx.wrapping_add(P3))) as u32 as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rax);
    rcx = ((rax.wrapping_add(r8.wrapping_mul(8))) as u64) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rcx);
    rcx = rsp.wrapping_add(0x38);
    r9 = ((crate::simd_ops::ru8(rsp.wrapping_add(0x30u64))) as u32) as u32 as u64;
    rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141da2d20(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x28; const P1: u64 = 0x20; const P2: u64 = 0x4; const P3: u64 = 0x141c6a020;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rax = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64;
    fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64;
    if !(((((fl_l as u32)) & ((fl_r as u32))) != 0)) { xmm0 = (xmm2) as u128; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
    fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1;
    if !((((fl_l as u32)) != ((fl_r as u32)))) { rax = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; xmm0 = ((crate::simd_ops::ru32(rax.wrapping_add(P2))) as u32) as u128; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
    return crate::simd_ops::tail_fwd_xmm(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8, [xmm0, xmm1, xmm2, xmm3]);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141da2d50(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x1ec;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; fl_r = 0x3;
    rax = (rax & !0xffu64) | (((if (((fl_l as u32)) == ((fl_r as u32))) { 1u8 } else { 0u8 }) as u8 as u64));
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141da2d70(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x280; const P1: u64 = 0x1ec;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(P0))) as u8 as u64; fl_r = (0x0u8) as u8 as u64;
    if (((fl_l as u8)) == ((fl_r as u8))) { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; return rax; }
    rax = (crate::simd_ops::ru32(rcx.wrapping_add(P1))) as u32 as u64;
    rax = (((rax as u32)).wrapping_sub(1)) as u32 as u64;
    fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1;
    if !((((fl_l as u32)) > ((fl_r as u32)))) { rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); return rax; }
    rax = (rax & !0xffu64) | (((0u8) as u8 as u64));
    fl_l = 0; fl_r = 0;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141da3320(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0x14240aa00; const P3: u64 = 0x48; const P4: u64 = 0x70; const P5: u64 = 0x98; const P6: u64 = 0x140e70cd0; const P7: u64 = 0xa8; const P8: u64 = 0x140225870; const P9: u64 = 0xb8; const P10: u64 = 0xc0;
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
    rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp);
    rdx = ((rbx.wrapping_add(P4)) as u64) as u64;
    rcx = ((rdi.wrapping_add(P4)) as u64) as u64;
    rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp);
    rdx = ((rbx.wrapping_add(P5)) as u64) as u64;
    rcx = ((rdi.wrapping_add(P5)) as u64) as u64;
    rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp);
    rdx = ((rbx.wrapping_add(P7)) as u64) as u64;
    rcx = ((rdi.wrapping_add(P7)) as u64) as u64;
    rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp);
    rax = ((crate::simd_ops::ru8(rbx.wrapping_add(P9))) as u32) as u32 as u64;
    crate::simd_ops::wu8(rdi.wrapping_add(P9), ((rax as u8)) as u8);
    rax = (crate::simd_ops::ru64(rbx.wrapping_add(P10))) as u64;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    crate::simd_ops::wu64(rdi.wrapping_add(P10), rax);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141da33f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x1432fcab0; const P2: u64 = 0x10; const P3: u64 = 0x14; const P4: u64 = 0x18; const P5: u64 = 0x13; const P6: u64 = 0x48; const P7: u64 = 0x4c; const P8: u64 = 0x50; const P9: u64 = 0x4b; const P10: u64 = 0x70; const P11: u64 = 0x74;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    crate::simd_ops::wu8(rdx.wrapping_add(P0), (0x1u8) as u8);
    rax = P1;
    crate::simd_ops::wu64(rdx, rax);
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::lk_and_u32(rdx.wrapping_add(P2), (0xff000006u32) as u32);
    crate::simd_ops::lk_or_u32(rdx.wrapping_add(P2), (0x6u32) as u32);
    crate::simd_ops::wu32(rdx.wrapping_add(P3), ((rax as u32)) as u32);
    crate::simd_ops::wu8(rdx.wrapping_add(P4), ((rax as u8)) as u8);
    crate::simd_ops::wu8(rdx.wrapping_add(P5), (0x6u8) as u8);
    crate::simd_ops::lk_and_u32(rdx.wrapping_add(P6), (0xff000004u32) as u32);
    crate::simd_ops::lk_or_u32(rdx.wrapping_add(P6), (0x4u32) as u32);
    crate::simd_ops::wu32(rdx.wrapping_add(P7), ((rax as u32)) as u32);
    crate::simd_ops::wu8(rdx.wrapping_add(P8), ((rax as u8)) as u8);
    crate::simd_ops::wu8(rdx.wrapping_add(P9), (0x4u8) as u8);
    crate::simd_ops::lk_and_u32(rdx.wrapping_add(P10), (0xff000004u32) as u32);
    fl_l = (crate::simd_ops::lk_or_u32(rdx.wrapping_add(P10), (0x4u32) as u32)) as u32 as u64; fl_r = fl_l;
    crate::simd_ops::wu32(rdx.wrapping_add(P11), ((rax as u32)) as u32);
    crate::simd_ops::wu8(rdx.wrapping_add(0x78u64), ((rax as u8)) as u8);
    crate::simd_ops::wu8(rdx.wrapping_add(0x73u64), (0x4u8) as u8);
    crate::simd_ops::wu64(rdx.wrapping_add(0x98u64), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(0xa0u64), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(0xa8u64), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(0xb0u64), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(0xc0u64), rax);
    crate::simd_ops::wu8(rdx.wrapping_add(0xb8u64), ((rax as u8)) as u8);
    rax = rdx;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141da3b30(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x18; const P2: u64 = 0x10; const P3: u64 = 0x14240aa00; const P4: u64 = 0x60; const P5: u64 = 0x140e70de0; const P6: u64 = 0x70; const P7: u64 = 0x80; const P8: u64 = 0x90; const P9: u64 = 0xa0;
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P0))) as u32) as u32 as u64;
    rcx = ((r8.wrapping_add(P1)) as u64) as u64;
    crate::simd_ops::wu8(r8.wrapping_add(P0), ((rax as u8)) as u8);
    rbx = rdx;
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(P2))) as u64;
    rdi = r8;
    rdx = ((rdx).wrapping_add(0x18u64)) as u64;
    crate::simd_ops::wu64(r8.wrapping_add(P2), rax);
    rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp);
    rdx = ((rbx.wrapping_add(P4)) as u64) as u64;
    rcx = ((rdi.wrapping_add(P4)) as u64) as u64;
    rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp);
    rdx = ((rbx.wrapping_add(P6)) as u64) as u64;
    rcx = ((rdi.wrapping_add(P6)) as u64) as u64;
    rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp);
    rdx = ((rbx.wrapping_add(P7)) as u64) as u64;
    rcx = ((rdi.wrapping_add(P7)) as u64) as u64;
    rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp);
    rdx = ((rbx.wrapping_add(P8)) as u64) as u64;
    rcx = ((rdi.wrapping_add(P8)) as u64) as u64;
    rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp);
    rdx = ((rbx.wrapping_add(P9)) as u64) as u64;
    rcx = ((rdi.wrapping_add(P9)) as u64) as u64;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return crate::simd_ops::tail_stk(P3, rax, rcx, rdx, r8, r9, rsp);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141da3bf0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x1432fd0a0; const P2: u64 = 0x10; const P3: u64 = 0x18; const P4: u64 = 0x1c; const P5: u64 = 0x20; const P6: u64 = 0x1b; const P7: u64 = 0x60; const P8: u64 = 0x68; const P9: u64 = 0x70; const P10: u64 = 0x78; const P11: u64 = 0x80;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    crate::simd_ops::wu8(rdx.wrapping_add(P0), (0x1u8) as u8);
    rax = P1;
    crate::simd_ops::wu64(rdx, rax);
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu64(rdx.wrapping_add(P2), rax);
    crate::simd_ops::lk_and_u32(rdx.wrapping_add(P3), (0xff000008u32) as u32);
    crate::simd_ops::lk_or_u32(rdx.wrapping_add(P3), (0x8u32) as u32);
    crate::simd_ops::wu32(rdx.wrapping_add(P4), ((rax as u32)) as u32);
    crate::simd_ops::wu8(rdx.wrapping_add(P5), ((rax as u8)) as u8);
    crate::simd_ops::wu8(rdx.wrapping_add(P6), (0x8u8) as u8);
    crate::simd_ops::wu64(rdx.wrapping_add(P7), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(P8), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(P9), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(P10), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(P11), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(0x88u64), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(0x90u64), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(0x98u64), rax);
    crate::simd_ops::lk_and_u32(rdx.wrapping_add(0xa0u64), (0xff000020u32) as u32);
    fl_l = (crate::simd_ops::lk_or_u32(rdx.wrapping_add(0xa0u64), (0x20u32) as u32)) as u32 as u64; fl_r = fl_l;
    crate::simd_ops::wu32(rdx.wrapping_add(0xa4u64), ((rax as u32)) as u32);
    crate::simd_ops::wu8(rdx.wrapping_add(0xa8u64), ((rax as u8)) as u8);
    rax = rdx;
    crate::simd_ops::wu8(rdx.wrapping_add(0xa3u64), (0x20u8) as u8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141da3ec0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14387fc38; const P1: u64 = 0x14243b0b0; const P2: u64 = 0x8; const P3: u64 = 0xfffffffffffffff8; const P4: u64 = 0xffffffffffffffff; const P5: u64 = 0x328;
    let (mut rax, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x40u64)) as u64;
    r8 = (crate::simd_ops::ru64(P0)) as u64;
    rbx = rdx;
    rdx = rcx;
    crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), (0x0u32) as u32);
    rcx = rbx;
    crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), (0x3u32) as u32);
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    r8 = (crate::simd_ops::ru32(rbx.wrapping_add(P2))) as u32 as u64;
    rax = (crate::simd_ops::ru64(rbx)) as u64;
    rcx = ((r8.wrapping_add(r8.wrapping_mul(2))) as u64) as u64;
    rcx = ((rcx).wrapping_add(rcx)) as u64;
    rcx = (crate::simd_ops::ru32(rax.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(P3))) as u32 as u64;
    rax = ((r8.wrapping_add(P4)) as u32) as u32 as u64;
    crate::simd_ops::wu32(rbx.wrapping_add(P2), ((rax as u32)) as u32);
    crate::simd_ops::wu32(rbx.wrapping_add(P5), ((rcx as u32)) as u32);
    rsp = ((rsp).wrapping_add(0x40u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141da43c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0x140e70ec0; const P3: u64 = 0x20; const P4: u64 = 0x140e70de0; const P5: u64 = 0x30; const P6: u64 = 0x140225870;
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
    rdx = ((rbx.wrapping_add(P5)) as u64) as u64;
    rcx = ((rdi.wrapping_add(P5)) as u64) as u64;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return crate::simd_ops::tail_stk(P6, rax, rcx, rdx, r8, r9, rsp);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141da4b30(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x4; const P1: u64 = 0x8; const P2: u64 = 0x3; const P3: u64 = 0x88; const P4: u64 = 0x8c; const P5: u64 = 0x90; const P6: u64 = 0x8b;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    crate::simd_ops::lk_and_u32(rdx, (0xff000010u32) as u32);
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::lk_or_u32(rdx, (0x10u32) as u32);
    crate::simd_ops::wu32(rdx.wrapping_add(P0), ((rax as u32)) as u32);
    crate::simd_ops::wu8(rdx.wrapping_add(P1), ((rax as u8)) as u8);
    crate::simd_ops::wu8(rdx.wrapping_add(P2), (0x10u8) as u8);
    crate::simd_ops::lk_and_u32(rdx.wrapping_add(P3), (0xff000004u32) as u32);
    fl_l = (crate::simd_ops::lk_or_u32(rdx.wrapping_add(P3), (0x4u32) as u32)) as u32 as u64; fl_r = fl_l;
    crate::simd_ops::wu32(rdx.wrapping_add(P4), ((rax as u32)) as u32);
    crate::simd_ops::wu8(rdx.wrapping_add(P5), ((rax as u8)) as u8);
    rax = rdx;
    crate::simd_ops::wu8(rdx.wrapping_add(P6), (0x4u8) as u8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141da4cd0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141d67860; const P1: u64 = 0x141d67760; const P2: u64 = 0x142645878;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 69>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (r9) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (r8) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rdx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rcx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r12) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r15) as u64); rbp = ((rsp.wrapping_sub(0xf0u64)) as u64) as u64; rsp = ((rsp).wrapping_sub(0x1f0u64)) as u64; rsi = (r9) as u64; r14 = (r8) as u64; r15 = (rdx) as u64; rbx = (rcx) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), (0x101u64) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), (0x0u64) as u64); crate::simd_ops::wu32(rsp.wrapping_add(0x78u64), (0x0u32) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x7cu64), (0x1000000u32) as u32); crate::simd_ops::wu64(rbp.wrapping_sub(0x80u64), (0x10001u64) as u64); crate::simd_ops::wu32(rbp.wrapping_sub(0x78u64), (0x0u32) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), (0x24u64) as u64); rax = ((rsp.wrapping_add(0x68u64)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), (rax) as u64); rax = ((rbp.wrapping_add(0xb0u64)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), (rax) as u64); r12 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rbp.wrapping_add(0xe8u64), (r12) as u64); rcx = (crate::simd_ops::ru64(rcx.wrapping_add(0x38u64))) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = ((rbp.wrapping_add(0xb0u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu64(rbp.wrapping_add(0xe8u64), (rax) as u64); bb = 2; continue; }
            2 => { xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x40u64))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm0) as u128); r9 = (0x2du32) as u32 as u64; r8 = ((r9.wrapping_sub(0x7u64)) as u32) as u32 as u64; rdx = ((rsp.wrapping_add(0x40u64)) as u64) as u64; rcx = ((rbp.wrapping_add(0xb0u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu64(rbp.wrapping_sub(0x10u64), (0x0u64) as u64); crate::simd_ops::wu64(rbp.wrapping_sub(0x8u64), (0x0u64) as u64); crate::simd_ops::wu32(rbp, (0x1000000u32) as u32); crate::simd_ops::wu32(rbp.wrapping_add(0x4u64), (0x1010001u32) as u32); crate::simd_ops::wu32(rbp.wrapping_add(0x8u64), (0x10101u32) as u32); crate::simd_ops::wu64(rbp.wrapping_add(0xcu64), (0x1000000u64) as u64); crate::simd_ops::wu64(rbp.wrapping_add(0x14u64), (0x0u64) as u64); crate::simd_ops::wu64(rbp.wrapping_add(0x1cu64), (0x0u64) as u64); crate::simd_ops::wu32(rbp.wrapping_add(0x24u64), (0x1010000u32) as u32); crate::simd_ops::wu64(rbp.wrapping_add(0x28u64), (0x1u64) as u64); crate::simd_ops::wu64(rbp.wrapping_add(0x30u64), (0x0u64) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), (0x48u64) as u64); rax = ((rbp.wrapping_sub(0x10u64)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), (rax) as u64); rax = ((rbp.wrapping_add(0xb0u64)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), (rax) as u64); crate::simd_ops::wu64(rbp.wrapping_add(0xe8u64), (r12) as u64); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x38u64))) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = ((rbp.wrapping_add(0xb0u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu64(rbp.wrapping_add(0xe8u64), (rax) as u64); bb = 4; continue; }
            4 => { xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x40u64))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm0) as u128); crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), (0x9u32) as u32); rdi = (0x1u32) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rdi as u32)) as u32); r9 = ((rdi.wrapping_add(0x2cu64)) as u32) as u32 as u64; r8 = ((rdi.wrapping_add(0x25u64)) as u32) as u32 as u64; rdx = ((rsp.wrapping_add(0x40u64)) as u64) as u64; rcx = ((rbp.wrapping_add(0xb0u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), (0x10101u64) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), (0x0u64) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), (0x0u64) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), (0x18u64) as u64); rax = ((rsp.wrapping_add(0x50u64)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), (rax) as u64); rax = ((rbp.wrapping_add(0xb0u64)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), (rax) as u64); crate::simd_ops::wu64(rbp.wrapping_add(0xe8u64), (r12) as u64); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x38u64))) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = ((rbp.wrapping_add(0xb0u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu64(rbp.wrapping_add(0xe8u64), (rax) as u64); bb = 6; continue; }
            6 => { xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x40u64))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm0) as u128); crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), (0x2du32) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), (0x26u32) as u32); r9 = (0x14u32) as u32 as u64; r8 = ((r9.wrapping_sub(0x2u64)) as u32) as u32 as u64; rdx = ((rsp.wrapping_add(0x40u64)) as u64) as u64; rcx = ((rbp.wrapping_add(0xb0u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu64(rbp.wrapping_sub(0x48u64), (0x0u64) as u64); crate::simd_ops::wu32(rbp.wrapping_sub(0x40u64), (0x0u32) as u32); crate::simd_ops::wu64(rbp.wrapping_sub(0x3cu64), (0x100u64) as u64); crate::simd_ops::wu64(rbp.wrapping_sub(0x34u64), (0x0u64) as u64); crate::simd_ops::wu32(rbp.wrapping_sub(0x2cu64), (0x0u32) as u32); crate::simd_ops::wu32(rbp.wrapping_sub(0x28u64), (0x100u32) as u32); crate::simd_ops::wu64(rbp.wrapping_sub(0x24u64), (0x101u64) as u64); crate::simd_ops::wu8(rbp.wrapping_sub(0x1cu64), (0x0u8) as u8); crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), (0x2du64) as u64); rax = ((rbp.wrapping_sub(0x48u64)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), (rax) as u64); rax = ((rbp.wrapping_add(0xb0u64)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), (rax) as u64); crate::simd_ops::wu64(rbp.wrapping_add(0xe8u64), (r12) as u64); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x38u64))) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 7; continue; }
            7 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = ((rbp.wrapping_add(0xb0u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu64(rbp.wrapping_add(0xe8u64), (rax) as u64); bb = 8; continue; }
            8 => { xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x40u64))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm0) as u128); crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), (0x9u32) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rdi as u32)) as u32); r9 = (0x32u32) as u32 as u64; r8 = ((r9.wrapping_sub(0x4u64)) as u32) as u32 as u64; rdx = ((rsp.wrapping_add(0x40u64)) as u64) as u64; rcx = ((rbp.wrapping_add(0xb0u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu32(rbp.wrapping_add(0x40u64), (0x10000u32) as u32); crate::simd_ops::wu32(rbp.wrapping_add(0x44u64), (0x10000u32) as u32); crate::simd_ops::wu32(rbp.wrapping_add(0x48u64), (0x10000u32) as u32); crate::simd_ops::wu32(rbp.wrapping_add(0x4cu64), (0x1u32) as u32); crate::simd_ops::wu32(rbp.wrapping_add(0x50u64), (0x1010100u32) as u32); crate::simd_ops::wu32(rbp.wrapping_add(0x54u64), (0x10001u32) as u32); crate::simd_ops::wu32(rbp.wrapping_add(0x58u64), (0x1010100u32) as u32); crate::simd_ops::wu32(rbp.wrapping_add(0x5cu64), (0x10001u32) as u32); crate::simd_ops::wu32(rbp.wrapping_add(0x60u64), (0x1010000u32) as u32); crate::simd_ops::wu32(rbp.wrapping_add(0x64u64), (0x101u32) as u32); crate::simd_ops::wu64(rbp.wrapping_add(0x68u64), (0x10000u64) as u64); crate::simd_ops::wu64(rbp.wrapping_add(0x70u64), (0x0u64) as u64); crate::simd_ops::wu32(rbp.wrapping_add(0x78u64), (0x10000u32) as u32); crate::simd_ops::wu64(rbp.wrapping_add(0x7cu64), (0x10000u64) as u64); crate::simd_ops::wu32(rbp.wrapping_add(0x84u64), (0x0u32) as u32); crate::simd_ops::wu32(rbp.wrapping_add(0x88u64), (0x10000u32) as u32); crate::simd_ops::wu32(rbp.wrapping_add(0x8cu64), (0x10000u32) as u32); crate::simd_ops::wu64(rbp.wrapping_add(0x90u64), (0x10000u64) as u64); crate::simd_ops::wu64(rbp.wrapping_add(0x98u64), (0x0u64) as u64); crate::simd_ops::wu32(rbp.wrapping_add(0xa0u64), (0x10000u32) as u32); crate::simd_ops::wu32(rbp.wrapping_add(0xa4u64), (0x1u32) as u32); crate::simd_ops::wu64(rbp.wrapping_add(0xa8u64), (0x10000u64) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), (0x70u64) as u64); rax = ((rbp.wrapping_add(0x40u64)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), (rax) as u64); rax = ((rbp.wrapping_add(0xb0u64)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), (rax) as u64); crate::simd_ops::wu64(rbp.wrapping_add(0xe8u64), (r12) as u64); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x38u64))) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 9; continue; }
            9 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = ((rbp.wrapping_add(0xb0u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu64(rbp.wrapping_add(0xe8u64), (rax) as u64); bb = 10; continue; }
            10 => { xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x40u64))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm0) as u128); crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), (0x2du32) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), (0x26u32) as u32); r9 = (0x22u32) as u32 as u64; r8 = ((r9.wrapping_sub(0xdu64)) as u32) as u32 as u64; rdx = ((rsp.wrapping_add(0x40u64)) as u64) as u64; rcx = ((rbp.wrapping_add(0xb0u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu64(rbp.wrapping_sub(0x70u64), (0x10000u64) as u64); crate::simd_ops::wu32(rbp.wrapping_sub(0x68u64), (0x0u32) as u32); crate::simd_ops::wu64(rbp.wrapping_sub(0x64u64), (0x10000u64) as u64); crate::simd_ops::wu64(rbp.wrapping_sub(0x5cu64), (0x1u64) as u64); crate::simd_ops::wu32(rbp.wrapping_sub(0x54u64), (0x0u32) as u32); crate::simd_ops::wu32(rbp.wrapping_sub(0x50u64), (0x10000u32) as u32); crate::simd_ops::wu32(rbp.wrapping_sub(0x4cu64), (0x1000000u32) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), (0x28u64) as u64); rax = ((rbp.wrapping_sub(0x70u64)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), (rax) as u64); rax = ((rbp.wrapping_add(0xb0u64)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), (rax) as u64); crate::simd_ops::wu64(rbp.wrapping_add(0xe8u64), (r12) as u64); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x38u64))) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 11; continue; }
            11 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = ((rbp.wrapping_add(0xb0u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu64(rbp.wrapping_add(0xe8u64), (rax) as u64); bb = 12; continue; }
            12 => { xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x40u64))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm0) as u128); crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), (0x2du32) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), (0x26u32) as u32); r9 = (0x32u32) as u32 as u64; r8 = ((r9.wrapping_sub(0x4u64)) as u32) as u32 as u64; rdx = ((rsp.wrapping_add(0x40u64)) as u64) as u64; rcx = ((rbp.wrapping_add(0xb0u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), (0x1du32) as u32); rcx = (crate::simd_ops::ru64(r14.wrapping_add(0x38u64))) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 14; continue; } bb = 13; continue; }
            13 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P2)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::lift_ops::trap(0x80000003, 0x141da51a5, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            14 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = ((rsp.wrapping_add(0x30u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), (0x2fu32) as u32); rcx = (crate::simd_ops::ru64(r14.wrapping_add(0x38u64))) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 16; continue; } bb = 15; continue; }
            15 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P2)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::lift_ops::trap(0x80000003, 0x141da51c8, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            16 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = ((rsp.wrapping_add(0x30u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), (0x5u32) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x38u64), (0x33u32) as u32); rcx = (crate::simd_ops::ru64(rsi.wrapping_add(0x38u64))) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 18; continue; } bb = 17; continue; }
            17 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P2)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::lift_ops::trap(0x80000003, 0x141da51f3, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            18 => { rax = (crate::simd_ops::ru64(rcx)) as u64; r8 = ((rsp.wrapping_add(0x30u64)) as u64) as u64; rdx = ((rsp.wrapping_add(0x38u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu32(rsp.wrapping_add(0x38u64), (0x2au32) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), (0x33u32) as u32); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x38u64))) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 20; continue; } bb = 19; continue; }
            19 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P2)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::lift_ops::trap(0x80000003, 0x141da5223, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            20 => { rax = (crate::simd_ops::ru64(rcx)) as u64; r8 = ((rsp.wrapping_add(0x38u64)) as u64) as u64; rdx = ((rsp.wrapping_add(0x30u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu32(rsp.wrapping_add(0x38u64), (0x33u32) as u32); rcx = (crate::simd_ops::ru64(r14.wrapping_add(0x38u64))) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 22; continue; } bb = 21; continue; }
            21 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P2)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::lift_ops::trap(0x80000003, 0x141da524b, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            22 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = ((rsp.wrapping_add(0x38u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu32(rsp.wrapping_add(0x38u64), (0x33u32) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), (0x34u32) as u32); rcx = (crate::simd_ops::ru64(rsi.wrapping_add(0x38u64))) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 24; continue; } bb = 23; continue; }
            23 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P2)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::lift_ops::trap(0x80000003, 0x141da5276, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            24 => { rax = (crate::simd_ops::ru64(rcx)) as u64; r8 = ((rsp.wrapping_add(0x38u64)) as u64) as u64; rdx = ((rsp.wrapping_add(0x30u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu32(rsp.wrapping_add(0x38u64), (0x2bu32) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), (0x34u32) as u32); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x38u64))) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 26; continue; } bb = 25; continue; }
            25 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P2)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::lift_ops::trap(0x80000003, 0x141da52a6, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            26 => { rax = (crate::simd_ops::ru64(rcx)) as u64; r8 = ((rsp.wrapping_add(0x38u64)) as u64) as u64; rdx = ((rsp.wrapping_add(0x30u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu32(rsp.wrapping_add(0x38u64), (0x19u32) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), (0x35u32) as u32); rcx = (crate::simd_ops::ru64(rsi.wrapping_add(0x38u64))) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 28; continue; } bb = 27; continue; }
            27 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P2)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::lift_ops::trap(0x80000003, 0x141da52d6, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            28 => { rax = (crate::simd_ops::ru64(rcx)) as u64; r8 = ((rsp.wrapping_add(0x38u64)) as u64) as u64; rdx = ((rsp.wrapping_add(0x30u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu32(rsp.wrapping_add(0x38u64), (0x35u32) as u32); rcx = (crate::simd_ops::ru64(r14.wrapping_add(0x38u64))) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 30; continue; } bb = 29; continue; }
            29 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P2)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::lift_ops::trap(0x80000003, 0x141da52fe, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            30 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = ((rsp.wrapping_add(0x38u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu32(rsp.wrapping_add(0x38u64), (0xbu32) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), (0x12u32) as u32); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x38u64))) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 32; continue; } bb = 31; continue; }
            31 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P2)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::lift_ops::trap(0x80000003, 0x141da5329, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            32 => { rax = (crate::simd_ops::ru64(rcx)) as u64; r8 = ((rsp.wrapping_add(0x38u64)) as u64) as u64; rdx = ((rsp.wrapping_add(0x30u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu32(rsp.wrapping_add(0x38u64), (0x5u32) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), (0x12u32) as u32); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x38u64))) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 34; continue; } bb = 33; continue; }
            33 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P2)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::lift_ops::trap(0x80000003, 0x141da5359, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            34 => { rax = (crate::simd_ops::ru64(rcx)) as u64; r8 = ((rsp.wrapping_add(0x38u64)) as u64) as u64; rdx = ((rsp.wrapping_add(0x30u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu32(rsp.wrapping_add(0x38u64), (0x33u32) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), (0xbu32) as u32); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x38u64))) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 36; continue; } bb = 35; continue; }
            35 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P2)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::lift_ops::trap(0x80000003, 0x141da5389, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            36 => { rax = (crate::simd_ops::ru64(rcx)) as u64; r8 = ((rsp.wrapping_add(0x38u64)) as u64) as u64; rdx = ((rsp.wrapping_add(0x30u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu32(rsp.wrapping_add(0x38u64), (0x1bu32) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), (0x36u32) as u32); rcx = (crate::simd_ops::ru64(rsi.wrapping_add(0x38u64))) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 38; continue; } bb = 37; continue; }
            37 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P2)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::lift_ops::trap(0x80000003, 0x141da53b9, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            38 => { rax = (crate::simd_ops::ru64(rcx)) as u64; r8 = ((rsp.wrapping_add(0x38u64)) as u64) as u64; rdx = ((rsp.wrapping_add(0x30u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu32(rsp.wrapping_add(0x38u64), (0x36u32) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), (0x28u32) as u32); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x38u64))) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 40; continue; } bb = 39; continue; }
            39 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P2)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::lift_ops::trap(0x80000003, 0x141da53e9, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            40 => { rax = (crate::simd_ops::ru64(rcx)) as u64; r8 = ((rsp.wrapping_add(0x38u64)) as u64) as u64; rdx = ((rsp.wrapping_add(0x30u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 41; continue; }
            41 => { crate::simd_ops::wu32(rsp.wrapping_add(0x38u64), ((rdi as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), (0x23u32) as u32); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x38u64))) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 52; continue; } bb = 42; continue; }
            42 => { rax = (crate::simd_ops::ru64(rcx)) as u64; r8 = ((rsp.wrapping_add(0x38u64)) as u64) as u64; rdx = ((rsp.wrapping_add(0x30u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdi = (((rdi as u32)).wrapping_add(1)) as u32 as u64; fl_l = ((rdi as u32)) as u32 as u64; fl_r = (0x37u32) as u32 as u64; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 41; continue; } bb = 43; continue; }
            43 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x38u64))) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 45; continue; } bb = 44; continue; }
            44 => { rax = (crate::simd_ops::ru64(rcx)) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rbx) as u64 as u64; rdx = (rdx & !0xffu64) | (((if (((fl_l as u64)) != ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x20u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu64(rbx.wrapping_add(0x38u64), (r12) as u64); bb = 45; continue; }
            45 => { rcx = (crate::simd_ops::ru64(r15.wrapping_add(0x38u64))) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 47; continue; } bb = 46; continue; }
            46 => { rax = (crate::simd_ops::ru64(rcx)) as u64; fl_l = (rcx) as u64 as u64; fl_r = (r15) as u64 as u64; rdx = (rdx & !0xffu64) | (((if (((fl_l as u64)) != ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x20u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu64(r15.wrapping_add(0x38u64), (r12) as u64); bb = 47; continue; }
            47 => { rcx = (crate::simd_ops::ru64(r14.wrapping_add(0x38u64))) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 49; continue; } bb = 48; continue; }
            48 => { rax = (crate::simd_ops::ru64(rcx)) as u64; fl_l = (rcx) as u64 as u64; fl_r = (r14) as u64 as u64; rdx = (rdx & !0xffu64) | (((if (((fl_l as u64)) != ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x20u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu64(r14.wrapping_add(0x38u64), (r12) as u64); bb = 49; continue; }
            49 => { rcx = (crate::simd_ops::ru64(rsi.wrapping_add(0x38u64))) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 51; continue; } bb = 50; continue; }
            50 => { rax = (crate::simd_ops::ru64(rcx)) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rsi) as u64 as u64; rdx = (rdx & !0xffu64) | (((if (((fl_l as u64)) != ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x20u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu64(rsi.wrapping_add(0x38u64), (r12) as u64); bb = 51; continue; }
            51 => { rsp = ((rsp).wrapping_add(0x1f0u64)) as u64; r15 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r12 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rsi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rbx = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rbp = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            52 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P2)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141da6cc0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0xc; const P2: u64 = 0x10; const P3: u64 = 0x14; const P4: u64 = 0x18; const P5: u64 = 0x1c; const P6: u64 = 0x20; const P7: u64 = 0x24; const P8: u64 = 0x28; const P9: u64 = 0x2c; const P10: u64 = 0x30;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P0), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P1))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P1), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P2))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P2), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P3))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P3), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P4))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P4), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P5))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P5), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P6))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P6), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P7))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P7), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P8))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P8), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P9))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P9), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(P10))) as u64;
    crate::simd_ops::wu64(r8.wrapping_add(P10), rax);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141da6d50(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x143300f60; const P1: u64 = 0x9; const P2: u64 = 0x2c; const P3: u64 = 0x30; const P4: u64 = 0x8; const P5: u64 = 0xb;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = P0;
    crate::simd_ops::wu16(rdx.wrapping_add(P1), (0x0u16) as u16);
    crate::simd_ops::wu64(rdx, rax);
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu32(rdx.wrapping_add(P2), ((rax as u32)) as u32);
    crate::simd_ops::wu64(rdx.wrapping_add(P3), rax);
    rax = rdx;
    crate::simd_ops::wu8(rdx.wrapping_add(P4), (0x0u8) as u8);
    crate::simd_ops::wu8(rdx.wrapping_add(P5), (0xffu8) as u8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141da6f70(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x9; const P2: u64 = 0xa;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = ((crate::simd_ops::ru8(rcx.wrapping_add(P0))) as u32) as u32 as u64;
    r9 = rcx;
    r8 = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    r8 = ((r8).wrapping_sub(crate::simd_ops::ru64(rdx))) as u64;
    crate::simd_ops::wu8(r8.wrapping_add(rcx).wrapping_add(P0), ((rax as u8)) as u8);
    rax = ((crate::simd_ops::ru8(rcx.wrapping_add(P1))) as u32) as u32 as u64;
    r8 = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    r8 = ((r8).wrapping_sub(crate::simd_ops::ru64(rdx))) as u64;
    crate::simd_ops::wu8(r8.wrapping_add(rcx).wrapping_add(P1), ((rax as u8)) as u8);
    rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    fl_l = rcx; fl_r = (crate::simd_ops::ru64(rdx)) as u64;
    rcx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64;
    rax = ((crate::simd_ops::ru8(r9.wrapping_add(P2))) as u32) as u32 as u64;
    crate::simd_ops::wu8(rcx.wrapping_add(r9).wrapping_add(P2), ((rax as u8)) as u8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141da7290(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x9; const P2: u64 = 0xa;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(r8))) as u64;
    rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P0))) as u32) as u32 as u64;
    crate::simd_ops::wu8(rcx.wrapping_add(rdx).wrapping_add(P0), ((rax as u8)) as u8);
    rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(r8))) as u64;
    rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P1))) as u32) as u32 as u64;
    crate::simd_ops::wu8(rcx.wrapping_add(rdx).wrapping_add(P1), ((rax as u8)) as u8);
    rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    fl_l = rcx; fl_r = (crate::simd_ops::ru64(r8)) as u64;
    rcx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64;
    rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P2))) as u32) as u32 as u64;
    crate::simd_ops::wu8(rcx.wrapping_add(rdx).wrapping_add(P2), ((rax as u8)) as u8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141da74d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x143301a68; const P1: u64 = 0x8; const P2: u64 = 0xa;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = P0;
    crate::simd_ops::wu16(rdx.wrapping_add(P1), (0x1u16) as u16);
    crate::simd_ops::wu64(rdx, rax);
    rax = rdx;
    crate::simd_ops::wu8(rdx.wrapping_add(P2), (0x0u8) as u8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141da77d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0xc; const P2: u64 = 0xd; const P3: u64 = 0xe; const P4: u64 = 0xf; const P5: u64 = 0x10;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64;
    r9 = rcx;
    r8 = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    r8 = ((r8).wrapping_sub(crate::simd_ops::ru64(rdx))) as u64;
    crate::simd_ops::wu32(r8.wrapping_add(rcx).wrapping_add(P0), ((rax as u32)) as u32);
    rax = ((crate::simd_ops::ru8(rcx.wrapping_add(P1))) as u32) as u32 as u64;
    r8 = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    r8 = ((r8).wrapping_sub(crate::simd_ops::ru64(rdx))) as u64;
    crate::simd_ops::wu8(r8.wrapping_add(rcx).wrapping_add(P1), ((rax as u8)) as u8);
    rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(rdx))) as u64;
    rax = ((crate::simd_ops::ru8(r9.wrapping_add(P2))) as u32) as u32 as u64;
    crate::simd_ops::wu8(rcx.wrapping_add(r9).wrapping_add(P2), ((rax as u8)) as u8);
    rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(rdx))) as u64;
    rax = ((crate::simd_ops::ru8(r9.wrapping_add(P3))) as u32) as u32 as u64;
    crate::simd_ops::wu8(rcx.wrapping_add(r9).wrapping_add(P3), ((rax as u8)) as u8);
    rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(rdx))) as u64;
    rax = ((crate::simd_ops::ru8(r9.wrapping_add(P4))) as u32) as u32 as u64;
    crate::simd_ops::wu8(rcx.wrapping_add(r9).wrapping_add(P4), ((rax as u8)) as u8);
    rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    fl_l = rcx; fl_r = (crate::simd_ops::ru64(rdx)) as u64;
    rcx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64;
    rax = ((crate::simd_ops::ru8(r9.wrapping_add(P5))) as u32) as u32 as u64;
    crate::simd_ops::wu8(rcx.wrapping_add(r9).wrapping_add(P5), ((rax as u8)) as u8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141da7c10(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0xc; const P2: u64 = 0xd; const P3: u64 = 0xe; const P4: u64 = 0xf; const P5: u64 = 0x10;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(r8))) as u64;
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(rdx).wrapping_add(P0), ((rax as u32)) as u32);
    rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(r8))) as u64;
    rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P1))) as u32) as u32 as u64;
    crate::simd_ops::wu8(rcx.wrapping_add(rdx).wrapping_add(P1), ((rax as u8)) as u8);
    rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(r8))) as u64;
    rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P2))) as u32) as u32 as u64;
    crate::simd_ops::wu8(rcx.wrapping_add(rdx).wrapping_add(P2), ((rax as u8)) as u8);
    rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(r8))) as u64;
    rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P3))) as u32) as u32 as u64;
    crate::simd_ops::wu8(rcx.wrapping_add(rdx).wrapping_add(P3), ((rax as u8)) as u8);
    rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(r8))) as u64;
    rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P4))) as u32) as u32 as u64;
    crate::simd_ops::wu8(rcx.wrapping_add(rdx).wrapping_add(P4), ((rax as u8)) as u8);
    rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    fl_l = rcx; fl_r = (crate::simd_ops::ru64(r8)) as u64;
    rcx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64;
    rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P5))) as u32) as u32 as u64;
    crate::simd_ops::wu8(rcx.wrapping_add(rdx).wrapping_add(P5), ((rax as u8)) as u8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141da7e20(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0xc; const P2: u64 = 0xd; const P3: u64 = 0xe; const P4: u64 = 0xf; const P5: u64 = 0x10;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P0), ((rax as u32)) as u32);
    rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P1))) as u32) as u32 as u64;
    crate::simd_ops::wu8(r8.wrapping_add(P1), ((rax as u8)) as u8);
    rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P2))) as u32) as u32 as u64;
    crate::simd_ops::wu8(r8.wrapping_add(P2), ((rax as u8)) as u8);
    rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P3))) as u32) as u32 as u64;
    crate::simd_ops::wu8(r8.wrapping_add(P3), ((rax as u8)) as u8);
    rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P4))) as u32) as u32 as u64;
    crate::simd_ops::wu8(r8.wrapping_add(P4), ((rax as u8)) as u8);
    rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P5))) as u32) as u32 as u64;
    crate::simd_ops::wu8(r8.wrapping_add(P5), ((rax as u8)) as u8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141da9040(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x20; const P2: u64 = 0xc; const P3: u64 = 0x10; const P4: u64 = 0x14; const P5: u64 = 0x18; const P6: u64 = 0x28; const P7: u64 = 0x140dd2530; const P8: u64 = 0x4; const P9: u64 = 0x38; const P10: u64 = 0x30; const P11: u64 = 0x34;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64; rdi = ((r8.wrapping_add(P1)) as u64) as u64; crate::simd_ops::wu32(r8.wrapping_add(P0), ((rax as u32)) as u32); rbx = rdx; rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P2))) as u32) as u32 as u64; rcx = rdi; crate::simd_ops::wu8(r8.wrapping_add(P2), ((rax as u8)) as u8); rax = (crate::simd_ops::ru32(rdx.wrapping_add(P3))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(P3), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdx.wrapping_add(P4))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(P4), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdx.wrapping_add(P5))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(P5), ((rax as u32)) as u32); rdx = (crate::simd_ops::ru32(rdx.wrapping_add(P6))) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = 0; fl_l = 0; fl_r = 0; fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(P6))) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 3; continue; } bb = 1; continue; }
            1 => { rcx = ((rdx as u32)) as u32 as u64; bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(P1))) as u64; rdx = (((rdx as u32)).wrapping_add(1)) as u32 as u64; xmm0 = ((crate::simd_ops::ru32(rax.wrapping_add(rcx.wrapping_mul(8)))) as u32) as u128; xmm1 = ((crate::simd_ops::ru32(rax.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(P8))) as u32) as u128; rax = (crate::simd_ops::ru64(rdi)) as u64; crate::simd_ops::wu32(rax.wrapping_add(rcx.wrapping_mul(8)), ((xmm0 as u32)) as u32); crate::simd_ops::wu32(rax.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(P8), ((xmm1 as u32)) as u32); rax = (crate::simd_ops::ru32(rbx.wrapping_add(P6))) as u32 as u64; rcx = ((((rdx as u32)) as i32 as i64) as u64) as u64; fl_l = rcx; fl_r = rax; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 2; continue; } bb = 3; continue; }
            3 => { fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(P9))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(P10))) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(P3), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rbx.wrapping_add(P11))) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x14u64), ((rax as u32)) as u32); crate::simd_ops::wu8(rdi.wrapping_add(0x18u64), (0x1u8) as u8); bb = 5; continue; }
            5 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141da9120(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x143302978; const P1: u64 = 0x18; const P2: u64 = 0x20; const P3: u64 = 0x28; const P4: u64 = 0x38; const P5: u64 = 0x3c;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = P0;
    crate::simd_ops::wu64(rdx, rax);
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu32(rdx.wrapping_add(P1), ((rax as u32)) as u32);
    crate::simd_ops::wu64(rdx.wrapping_add(P2), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(P3), rax);
    crate::simd_ops::wu8(rdx.wrapping_add(P4), ((rax as u8)) as u8);
    crate::simd_ops::wu32(rdx.wrapping_add(P5), ((rax as u32)) as u32);
    rax = rdx;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141daa0c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x1401e84a0; const P2: u64 = 0x20; const P3: u64 = 0x24; const P4: u64 = 0x28; const P5: u64 = 0x140868e20; const P6: u64 = 0x38; const P7: u64 = 0x140a34aa0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rdi = r8;
    rbx = rdx;
    rcx = ((r8.wrapping_add(P0)) as u64) as u64;
    rdx = ((rdx).wrapping_add(0x8u64)) as u64;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rax = (crate::simd_ops::ru32(rbx.wrapping_add(P2))) as u32 as u64;
    crate::simd_ops::wu32(rdi.wrapping_add(P2), ((rax as u32)) as u32);
    rax = ((crate::simd_ops::ru8(rbx.wrapping_add(P3))) as u32) as u32 as u64;
    crate::simd_ops::wu8(rdi.wrapping_add(P3), ((rax as u8)) as u8);
    rdx = ((rbx.wrapping_add(P4)) as u64) as u64;
    rcx = ((rdi.wrapping_add(P4)) as u64) as u64;
    rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp);
    rbx = ((rbx).wrapping_add(0x38u64)) as u64;
    rcx = ((rdi.wrapping_add(P6)) as u64) as u64;
    fl_l = rcx; fl_r = rbx;
    if !((((fl_l as u64)) == ((fl_r as u64)))) { rdx = (crate::simd_ops::ru64(rbx)) as u64; rax = (crate::simd_ops::ru32(rbx.wrapping_add(P0))) as u32 as u64; r8 = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; r8 = ((r8).wrapping_shl(4u32)) as u64; r8 = ((r8).wrapping_add(rdx)) as u64; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); }
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141daa160(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x143301d80; const P1: u64 = 0x8; const P2: u64 = 0x10; const P3: u64 = 0x18; const P4: u64 = 0x1c; const P5: u64 = 0x28; const P6: u64 = 0x30; const P7: u64 = 0x38; const P8: u64 = 0x40;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = P0;
    crate::simd_ops::wu64(rdx, rax);
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu64(rdx.wrapping_add(P1), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(P2), rax);
    crate::simd_ops::wu32(rdx.wrapping_add(P3), ((rax as u32)) as u32);
    crate::simd_ops::wu8(rdx.wrapping_add(P4), ((rax as u8)) as u8);
    crate::simd_ops::wu64(rdx.wrapping_add(P5), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(P6), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(P7), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(P8), rax);
    rax = rdx;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141dab940(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0xc; const P2: u64 = 0x9; const P3: u64 = 0xa; const P4: u64 = 0x14240aa00; const P5: u64 = 0x58; const P6: u64 = 0x1401e84a0;
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P0))) as u32) as u32 as u64;
    rcx = ((r8.wrapping_add(P1)) as u64) as u64;
    crate::simd_ops::wu8(r8.wrapping_add(P0), ((rax as u8)) as u8);
    rbx = rdx;
    rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P2))) as u32) as u32 as u64;
    rdi = r8;
    crate::simd_ops::wu8(r8.wrapping_add(P2), ((rax as u8)) as u8);
    rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P3))) as u32) as u32 as u64;
    rdx = ((rdx).wrapping_add(0xcu64)) as u64;
    crate::simd_ops::wu8(r8.wrapping_add(P3), ((rax as u8)) as u8);
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
pub unsafe extern "win64" fn fn_141dab9d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x1432ff638; const P1: u64 = 0xc; const P2: u64 = 0x10; const P3: u64 = 0x14; const P4: u64 = 0xf; const P5: u64 = 0x58; const P6: u64 = 0x60; const P7: u64 = 0x68; const P8: u64 = 0x6c;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = P0;
    crate::simd_ops::wu64(rdx, rax);
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::lk_and_u32(rdx.wrapping_add(P1), (0xff000008u32) as u32);
    fl_l = (crate::simd_ops::lk_or_u32(rdx.wrapping_add(P1), (0x8u32) as u32)) as u32 as u64; fl_r = fl_l;
    crate::simd_ops::wu32(rdx.wrapping_add(P2), ((rax as u32)) as u32);
    crate::simd_ops::wu8(rdx.wrapping_add(P3), ((rax as u8)) as u8);
    crate::simd_ops::wu8(rdx.wrapping_add(P4), (0x8u8) as u8);
    crate::simd_ops::wu64(rdx.wrapping_add(P5), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(P6), rax);
    crate::simd_ops::wu32(rdx.wrapping_add(P7), ((rax as u32)) as u32);
    crate::simd_ops::wu8(rdx.wrapping_add(P8), ((rax as u8)) as u8);
    rax = rdx;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141dac0c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x1409f9a40; const P2: u64 = 0x18; const P3: u64 = 0x1404b9990; const P4: u64 = 0x38;
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rdx;
    rcx = ((r8.wrapping_add(P0)) as u64) as u64;
    rdx = ((rdx).wrapping_add(0x8u64)) as u64;
    rdi = r8;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rdx = ((rbx.wrapping_add(P2)) as u64) as u64;
    rcx = ((rdi.wrapping_add(P2)) as u64) as u64;
    rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp);
    rdx = ((rbx.wrapping_add(P4)) as u64) as u64;
    rcx = ((rdi.wrapping_add(P4)) as u64) as u64;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return crate::simd_ops::tail_stk(P3, rax, rcx, rdx, r8, r9, rsp);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141dac140(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x1433026c8; const P1: u64 = 0x8; const P2: u64 = 0x10; const P3: u64 = 0x18; const P4: u64 = 0x20; const P5: u64 = 0x28; const P6: u64 = 0x30; const P7: u64 = 0x38; const P8: u64 = 0x40; const P9: u64 = 0x48; const P10: u64 = 0x50;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = P0;
    crate::simd_ops::wu64(rdx, rax);
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu64(rdx.wrapping_add(P1), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(P2), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(P3), rax);
    crate::simd_ops::wu32(rdx.wrapping_add(P4), ((rax as u32)) as u32);
    crate::simd_ops::wu64(rdx.wrapping_add(P5), rax);
    crate::simd_ops::wu8(rdx.wrapping_add(P6), ((rax as u8)) as u8);
    crate::simd_ops::wu64(rdx.wrapping_add(P7), rax);
    crate::simd_ops::wu32(rdx.wrapping_add(P8), ((rax as u32)) as u32);
    crate::simd_ops::wu64(rdx.wrapping_add(P9), rax);
    crate::simd_ops::wu8(rdx.wrapping_add(P10), ((rax as u8)) as u8);
    rax = rdx;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141dac720(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x140461550; const P2: u64 = 0x18; const P3: u64 = 0x28; const P4: u64 = 0x38; const P5: u64 = 0x48; const P6: u64 = 0x58; const P7: u64 = 0x68; const P8: u64 = 0x78;
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rdx;
    rcx = ((r8.wrapping_add(P0)) as u64) as u64;
    rdx = ((rdx).wrapping_add(0x8u64)) as u64;
    rdi = r8;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rdx = ((rbx.wrapping_add(P2)) as u64) as u64;
    rcx = ((rdi.wrapping_add(P2)) as u64) as u64;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rdx = ((rbx.wrapping_add(P3)) as u64) as u64;
    rcx = ((rdi.wrapping_add(P3)) as u64) as u64;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rdx = ((rbx.wrapping_add(P4)) as u64) as u64;
    rcx = ((rdi.wrapping_add(P4)) as u64) as u64;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rdx = ((rbx.wrapping_add(P5)) as u64) as u64;
    rcx = ((rdi.wrapping_add(P5)) as u64) as u64;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rdx = ((rbx.wrapping_add(P6)) as u64) as u64;
    rcx = ((rdi.wrapping_add(P6)) as u64) as u64;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rdx = ((rbx.wrapping_add(P7)) as u64) as u64;
    rcx = ((rdi.wrapping_add(P7)) as u64) as u64;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rdx = ((rbx.wrapping_add(P8)) as u64) as u64;
    rcx = ((rdi.wrapping_add(P8)) as u64) as u64;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return crate::simd_ops::tail_stk(P1, rax, rcx, rdx, r8, r9, rsp);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141dac7e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x1431778f0; const P1: u64 = 0x8; const P2: u64 = 0x10; const P3: u64 = 0x18; const P4: u64 = 0x20; const P5: u64 = 0x28; const P6: u64 = 0x30; const P7: u64 = 0x38; const P8: u64 = 0x40; const P9: u64 = 0x48; const P10: u64 = 0x50; const P11: u64 = 0x58;
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
    crate::simd_ops::wu64(rdx.wrapping_add(P8), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(P9), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(P10), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(P11), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(0x60u64), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(0x68u64), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(0x70u64), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(0x78u64), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(0x80u64), rax);
    rax = rdx;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141dad830(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x14240aa00; const P2: u64 = 0x90; const P3: u64 = 0x118; const P4: u64 = 0x1a0;
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rdx;
    rcx = ((r8.wrapping_add(P0)) as u64) as u64;
    rdx = ((rdx).wrapping_add(0x8u64)) as u64;
    rdi = r8;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rdx = ((rbx.wrapping_add(P2)) as u64) as u64;
    rcx = ((rdi.wrapping_add(P2)) as u64) as u64;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rdx = ((rbx.wrapping_add(P3)) as u64) as u64;
    rcx = ((rdi.wrapping_add(P3)) as u64) as u64;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rax = (crate::simd_ops::ru32(rbx.wrapping_add(P4))) as u32 as u64;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    crate::simd_ops::wu32(rdi.wrapping_add(P4), ((rax as u32)) as u32);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141daeb80(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x143302570; const P1: u64 = 0x8; const P2: u64 = 0x10; const P3: u64 = 0x18; const P4: u64 = 0x20; const P5: u64 = 0x28;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = P0;
    crate::simd_ops::wu64(rdx, rax);
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu32(rdx.wrapping_add(P1), ((rax as u32)) as u32);
    crate::simd_ops::wu64(rdx.wrapping_add(P2), rax);
    crate::simd_ops::wu32(rdx.wrapping_add(P3), ((rax as u32)) as u32);
    crate::simd_ops::wu64(rdx.wrapping_add(P4), rax);
    crate::simd_ops::wu8(rdx.wrapping_add(P5), ((rax as u8)) as u8);
    rax = rdx;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141daf8c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x140e97760; const P2: u64 = 0xb8; const P3: u64 = 0xc0; const P4: u64 = 0x1401e84a0; const P5: u64 = 0xd8; const P6: u64 = 0xdc; const P7: u64 = 0x14240aa00;
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rdx;
    rcx = ((r8.wrapping_add(P0)) as u64) as u64;
    rdx = ((rdx).wrapping_add(0x8u64)) as u64;
    rdi = r8;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rax = (crate::simd_ops::ru32(rbx.wrapping_add(P2))) as u32 as u64;
    rdx = ((rbx.wrapping_add(P3)) as u64) as u64;
    rcx = ((rdi.wrapping_add(P3)) as u64) as u64;
    crate::simd_ops::wu32(rdi.wrapping_add(P2), ((rax as u32)) as u32);
    rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp);
    rax = (crate::simd_ops::ru32(rbx.wrapping_add(P5))) as u32 as u64;
    rdx = ((rbx.wrapping_add(P6)) as u64) as u64;
    rcx = ((rdi.wrapping_add(P6)) as u64) as u64;
    crate::simd_ops::wu32(rdi.wrapping_add(P5), ((rax as u32)) as u32);
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return crate::simd_ops::tail_stk(P7, rax, rcx, rdx, r8, r9, rsp);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141daf960(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1431bf580; const P1: u64 = 0x10; const P2: u64 = 0x14183acc2; const P3: u64 = 0x14; const P4: u64 = 0x18; const P5: u64 = 0x8; const P6: u64 = 0xc0; const P7: u64 = 0xc8; const P8: u64 = 0xd0; const P9: u64 = 0xd4; const P10: u64 = 0xdc; const P11: u64 = 0xe0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rax = P0;
    rbx = rdx;
    crate::simd_ops::wu64(rdx, rax);
    rcx = ((rdx.wrapping_add(P1)) as u64) as u64;
    rdx = 0;
    fl_l = 0; fl_r = 0;
    r8 = 0xa8;
    rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp);
    crate::simd_ops::wu32(rbx.wrapping_add(P3), (0x3u32) as u32);
    rax = ((rbx.wrapping_add(P4)) as u64) as u64;
    crate::simd_ops::wu64(rbx.wrapping_add(P5), rax);
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu32(rbx.wrapping_add(P1), ((rax as u32)) as u32);
    crate::simd_ops::wu64(rbx.wrapping_add(P6), rax);
    crate::simd_ops::wu64(rbx.wrapping_add(P7), rax);
    crate::simd_ops::wu32(rbx.wrapping_add(P8), ((rax as u32)) as u32);
    crate::simd_ops::wu8(rbx.wrapping_add(P9), ((rax as u8)) as u8);
    crate::simd_ops::lk_and_u32(rbx.wrapping_add(P10), (0xff000002u32) as u32);
    crate::simd_ops::lk_or_u32(rbx.wrapping_add(P10), (0x2u32) as u32);
    crate::simd_ops::wu32(rbx.wrapping_add(P11), ((rax as u32)) as u32);
    crate::simd_ops::wu8(rbx.wrapping_add(0xe4u64), ((rax as u8)) as u8);
    rax = rbx;
    crate::simd_ops::wu8(rbx.wrapping_add(0xdfu64), (0x2u8) as u8);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141db02f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x1432ffcd0; const P1: u64 = 0x8; const P2: u64 = 0x10; const P3: u64 = 0x14; const P4: u64 = 0x18; const P5: u64 = 0x13;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = P0;
    crate::simd_ops::wu64(rdx, rax);
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu32(rdx.wrapping_add(P1), ((rax as u32)) as u32);
    crate::simd_ops::lk_and_u32(rdx.wrapping_add(P2), (0xff000004u32) as u32);
    fl_l = (crate::simd_ops::lk_or_u32(rdx.wrapping_add(P2), (0x4u32) as u32)) as u32 as u64; fl_r = fl_l;
    crate::simd_ops::wu32(rdx.wrapping_add(P3), ((rax as u32)) as u32);
    crate::simd_ops::wu8(rdx.wrapping_add(P4), ((rax as u8)) as u8);
    rax = rdx;
    crate::simd_ops::wu8(rdx.wrapping_add(P5), (0x4u8) as u8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141db0510(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x14240aa00; const P2: u64 = 0x50;
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rdi = rdx;
    rbx = rcx;
    rdx = ((rcx.wrapping_add(P0)) as u64) as u64;
    rcx = (crate::simd_ops::ru64(rdi.wrapping_add(P0))) as u64;
    rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(rdi))) as u64;
    rcx = ((rcx).wrapping_add(rdx)) as u64;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rcx = (crate::simd_ops::ru64(rdi.wrapping_add(P0))) as u64;
    rax = ((crate::simd_ops::ru8(rbx.wrapping_add(P2))) as u32) as u32 as u64;
    rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(rdi))) as u64;
    crate::simd_ops::wu8(rcx.wrapping_add(rbx).wrapping_add(P2), ((rax as u8)) as u8);
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141db0840(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x14240aa00; const P2: u64 = 0x50;
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    rbx = rdx;
    rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(r8))) as u64;
    rdx = ((rdx).wrapping_add(0x8u64)) as u64;
    rcx = ((rcx).wrapping_add(rdx)) as u64;
    rdi = r8;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rcx = (crate::simd_ops::ru64(rdi.wrapping_add(P0))) as u64;
    rax = ((crate::simd_ops::ru8(rbx.wrapping_add(P2))) as u32) as u32 as u64;
    rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(rdi))) as u64;
    crate::simd_ops::wu8(rcx.wrapping_add(rbx).wrapping_add(P2), ((rax as u8)) as u8);
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141db1390(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x4; const P1: u64 = 0x8; const P2: u64 = 0x3; const P3: u64 = 0x108; const P4: u64 = 0x110;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    crate::simd_ops::lk_and_u32(rdx, (0xff000020u32) as u32);
    rax = 0;
    fl_l = 0; fl_r = 0;
    fl_l = (crate::simd_ops::lk_or_u32(rdx, (0x20u32) as u32)) as u32 as u64; fl_r = fl_l;
    crate::simd_ops::wu32(rdx.wrapping_add(P0), ((rax as u32)) as u32);
    crate::simd_ops::wu8(rdx.wrapping_add(P1), ((rax as u8)) as u8);
    crate::simd_ops::wu8(rdx.wrapping_add(P2), (0x20u8) as u8);
    crate::simd_ops::wu64(rdx.wrapping_add(P3), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(P4), rax);
    rax = rdx;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141db1ae0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x143304a88; const P1: u64 = 0x8; const P2: u64 = 0xb; const P3: u64 = 0xc; const P4: u64 = 0x10;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = P0;
    crate::simd_ops::wu64(rdx, rax);
    rax = rdx;
    crate::simd_ops::lk_and_u32(rdx.wrapping_add(P1), (0xff000020u32) as u32);
    fl_l = (crate::simd_ops::lk_or_u32(rdx.wrapping_add(P1), (0x20u32) as u32)) as u32 as u64; fl_r = fl_l;
    crate::simd_ops::wu8(rdx.wrapping_add(P2), (0x20u8) as u8);
    crate::simd_ops::wu32(rdx.wrapping_add(P3), (0x0u32) as u32);
    crate::simd_ops::wu8(rdx.wrapping_add(P4), (0x0u8) as u8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141db1c50(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x430; const P1: u64 = 0x10; const P2: u64 = 0x438; const P3: u64 = 0x14307ead0; const P4: u64 = 0x140ec2d10;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x30u64)) as u64;
    rbx = rcx;
    rdi = rdx;
    rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    fl_l = rcx; fl_r = rcx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rax = (crate::simd_ops::ru64(rcx)) as u64; r8 = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); }
    rdx = ((rbx.wrapping_add(P2)) as u64) as u64;
    rcx = rdi;
    r9 = P3;
    rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp);
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64;
    rsp = ((rsp).wrapping_add(0x30u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141db1da0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x430; const P1: u64 = 0x10; const P2: u64 = 0x438; const P3: u64 = 0x14307ead0; const P4: u64 = 0x140ec2d10;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x30u64)) as u64;
    rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    rbx = r8;
    rdi = rdx;
    fl_l = rcx; fl_r = rcx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rax = (crate::simd_ops::ru64(rcx)) as u64; r8 = 0; fl_l = 0; fl_r = 0; rdx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); }
    rdx = ((rdi.wrapping_add(P2)) as u64) as u64;
    rcx = rbx;
    r9 = P3;
    rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp);
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64;
    rsp = ((rsp).wrapping_add(0x30u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141db2250(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x14240aa00; const P2: u64 = 0x110; const P3: u64 = 0x218; const P4: u64 = 0x320; const P5: u64 = 0x428; const P6: u64 = 0x430; const P7: u64 = 0x429; const P8: u64 = 0x140ec1730; const P9: u64 = 0x438; const P10: u64 = 0x140ec1680; const P11: u64 = 0x448; const P12: u64 = 0x140ec5c40; const P13: u64 = 0x140ec5b90;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rdx; rcx = ((r8.wrapping_add(P0)) as u64) as u64; rdx = ((rdx).wrapping_add(0x8u64)) as u64; rdi = r8; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rdx = ((rbx.wrapping_add(P2)) as u64) as u64; rcx = ((rdi.wrapping_add(P2)) as u64) as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rdx = ((rbx.wrapping_add(P3)) as u64) as u64; rcx = ((rdi.wrapping_add(P3)) as u64) as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rdx = ((rbx.wrapping_add(P4)) as u64) as u64; rcx = ((rdi.wrapping_add(P4)) as u64) as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rax = ((crate::simd_ops::ru8(rbx.wrapping_add(P5))) as u32) as u32 as u64; rdx = ((rbx.wrapping_add(P6)) as u64) as u64; crate::simd_ops::wu8(rdi.wrapping_add(P5), ((rax as u8)) as u8); rcx = ((rdi.wrapping_add(P6)) as u64) as u64; rax = ((crate::simd_ops::ru8(rbx.wrapping_add(P7))) as u32) as u32 as u64; crate::simd_ops::wu8(rdi.wrapping_add(P7), ((rax as u8)) as u8); rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); rdx = ((rbx.wrapping_add(P9)) as u64) as u64; rcx = ((rdi.wrapping_add(P9)) as u64) as u64; rax = crate::simd_ops::call_stk(P10, rax, rcx, rdx, r8, r9, rsp); rsi = ((rbx.wrapping_add(P11)) as u64) as u64; rdi = ((rdi).wrapping_add(0x448u64)) as u64; fl_l = rdi; fl_r = rsi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rbx = (crate::simd_ops::ru64(rdi)) as u64; rdx = rdi; rcx = rdi; r8 = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; rax = crate::simd_ops::call_stk(P12, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx.wrapping_add(0x8u64), rbx); rdx = rsi; crate::simd_ops::wu64(rbx, rbx); rcx = rdi; crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rdi.wrapping_add(0x8u64), (0x0u64) as u64); rax = crate::simd_ops::call_stk(P13, rax, rcx, rdx, r8, r9, rsp); bb = 2; continue; }
            2 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141db3430(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14240aa00; const P1: u64 = 0x108; const P2: u64 = 0x210; const P3: u64 = 0x140ec1680;
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
    rdx = ((rbx.wrapping_add(P1)) as u64) as u64;
    rcx = ((rdi.wrapping_add(P1)) as u64) as u64;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rdx = ((rbx.wrapping_add(P2)) as u64) as u64;
    rcx = ((rdi.wrapping_add(P2)) as u64) as u64;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return crate::simd_ops::tail_stk(P3, rax, rcx, rdx, r8, r9, rsp);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141db34d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x4; const P1: u64 = 0x8; const P2: u64 = 0x3; const P3: u64 = 0x108; const P4: u64 = 0x10c; const P5: u64 = 0x110; const P6: u64 = 0x10b; const P7: u64 = 0x210; const P8: u64 = 0x218;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    crate::simd_ops::lk_and_u32(rdx, (0xff000020u32) as u32);
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::lk_or_u32(rdx, (0x20u32) as u32);
    crate::simd_ops::wu32(rdx.wrapping_add(P0), ((rax as u32)) as u32);
    crate::simd_ops::wu8(rdx.wrapping_add(P1), ((rax as u8)) as u8);
    crate::simd_ops::wu8(rdx.wrapping_add(P2), (0x20u8) as u8);
    crate::simd_ops::lk_and_u32(rdx.wrapping_add(P3), (0xff000020u32) as u32);
    fl_l = (crate::simd_ops::lk_or_u32(rdx.wrapping_add(P3), (0x20u32) as u32)) as u32 as u64; fl_r = fl_l;
    crate::simd_ops::wu32(rdx.wrapping_add(P4), ((rax as u32)) as u32);
    crate::simd_ops::wu8(rdx.wrapping_add(P5), ((rax as u8)) as u8);
    crate::simd_ops::wu8(rdx.wrapping_add(P6), (0x20u8) as u8);
    crate::simd_ops::wu64(rdx.wrapping_add(P7), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(P8), rax);
    rax = rdx;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141db3b10(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x140ec1810; const P1: u64 = 0x10;
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rcx = r8;
    rdi = r8;
    rbx = rdx;
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    xmm0 = (crate::simd_ops::ru128(rbx.wrapping_add(P1))) as u128;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    crate::simd_ops::wu128(rdi.wrapping_add(P1), (xmm0) as u128);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141db4730(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x140ee4720; const P1: u64 = 0x10; const P2: u64 = 0x20; const P3: u64 = 0x28; const P4: u64 = 0x140225740;
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
    rdx = ((rbx.wrapping_add(P1)) as u64) as u64;
    rcx = ((rdi.wrapping_add(P1)) as u64) as u64;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rax = (crate::simd_ops::ru64(rbx.wrapping_add(P2))) as u64;
    rdx = ((rbx.wrapping_add(P3)) as u64) as u64;
    rcx = ((rdi.wrapping_add(P3)) as u64) as u64;
    crate::simd_ops::wu64(rdi.wrapping_add(P2), rax);
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return crate::simd_ops::tail_stk(P4, rax, rcx, rdx, r8, r9, rsp);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141db47c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0x18; const P3: u64 = 0x28; const P4: u64 = 0x30;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu64(rdx, rax);
    crate::simd_ops::wu64(rdx.wrapping_add(P0), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(P1), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(P2), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(P3), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(P4), rax);
    rax = rdx;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141db6630(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14240aa00; const P1: u64 = 0x108; const P2: u64 = 0x110; const P3: u64 = 0x8; const P4: u64 = 0x14083fed0; const P5: u64 = 0x10; const P6: u64 = 0x140eefdd0; const P7: u64 = 0x120; const P8: u64 = 0x1a8;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rcx = r8; rdi = r8; rsi = rdx; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rsi.wrapping_add(P1))) as u64; rbp = ((rsi.wrapping_add(P2)) as u64) as u64; r14 = ((rdi.wrapping_add(P2)) as u64) as u64; crate::simd_ops::wu64(rdi.wrapping_add(P1), rax); fl_l = r14; fl_r = rbp; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbx); rdx = r14; rbx = (crate::simd_ops::ru64(r14)) as u64; rcx = r14; r8 = (crate::simd_ops::ru64(rbx.wrapping_add(P3))) as u64; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx.wrapping_add(P3), rbx); rdx = rbp; crate::simd_ops::wu64(rbx, rbx); rcx = r14; crate::simd_ops::wu64(rbx.wrapping_add(P5), rbx); crate::simd_ops::wu64(r14.wrapping_add(P3), (0x0u64) as u64); rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; bb = 2; continue; }
            2 => { rdx = ((rsi.wrapping_add(P7)) as u64) as u64; rcx = ((rdi.wrapping_add(P7)) as u64) as u64; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru32(rsi.wrapping_add(P8))) as u32 as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; crate::simd_ops::wu32(rdi.wrapping_add(P8), ((rax as u32)) as u32); rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141db6720(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xfffffffffffffff8; const P1: u64 = 0xfffffffffffffff0; const P2: u64 = 0x120; const P3: u64 = 0x14240ab10; const P4: u64 = 0x110; const P5: u64 = 0x8; const P6: u64 = 0x14083fed0; const P7: u64 = 0x1401e66f0; const P8: u64 = 0x142442c80;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 1; continue; }
            1 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbp); rbx = rdx; rbp = 0; fl_l = 0; fl_r = 0; fl_l = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 5; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rdi); bb = 3; continue; }
            3 => { rsi = rbp; rsi = (((rsi) as i64).wrapping_mul((crate::simd_ops::ru64(rbx.wrapping_add(P1))) as i64) as u64) as u64; rsi = ((rsi).wrapping_add(rbx)) as u64; rcx = ((rsi.wrapping_add(P2)) as u64) as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); r8 = (crate::simd_ops::ru64(rsi.wrapping_add(P4))) as u64; rdx = ((rsi.wrapping_add(P4)) as u64) as u64; rcx = ((rsi.wrapping_add(P4)) as u64) as u64; r8 = (crate::simd_ops::ru64(r8.wrapping_add(P5))) as u64; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); rdx = (crate::simd_ops::ru64(rsi.wrapping_add(P4))) as u64; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); rcx = rsi; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru32(rbx.wrapping_add(P0))) as u32 as u64; rbp = ((rbp).wrapping_add(1)) as u64; fl_l = rbp; fl_r = rax; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 3; continue; } bb = 4; continue; }
            4 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; bb = 5; continue; }
            5 => { rcx = rbx; rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); bb = 6; continue; }
            6 => { return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141db6e10(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x4; const P1: u64 = 0x8; const P2: u64 = 0x3; const P3: u64 = 0x108; const P4: u64 = 0x10c; const P5: u64 = 0x110; const P6: u64 = 0x10b;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    crate::simd_ops::lk_and_u32(rdx, (0xff000020u32) as u32);
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::lk_or_u32(rdx, (0x20u32) as u32);
    crate::simd_ops::wu32(rdx.wrapping_add(P0), ((rax as u32)) as u32);
    crate::simd_ops::wu8(rdx.wrapping_add(P1), ((rax as u8)) as u8);
    crate::simd_ops::wu8(rdx.wrapping_add(P2), (0x20u8) as u8);
    crate::simd_ops::lk_and_u32(rdx.wrapping_add(P3), (0xff000004u32) as u32);
    fl_l = (crate::simd_ops::lk_or_u32(rdx.wrapping_add(P3), (0x4u32) as u32)) as u32 as u64; fl_r = fl_l;
    crate::simd_ops::wu32(rdx.wrapping_add(P4), ((rax as u32)) as u32);
    crate::simd_ops::wu8(rdx.wrapping_add(P5), ((rax as u8)) as u8);
    rax = rdx;
    crate::simd_ops::wu8(rdx.wrapping_add(P6), (0x4u8) as u8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141db74e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x140ef43c0;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    r9 = r8;
    rax = rdx;
    fl_l = r8; fl_r = rdx;
    if !((((fl_l as u64)) == ((fl_r as u64)))) { rdx = (crate::simd_ops::ru64(rdx)) as u64; rax = (crate::simd_ops::ru32(rax.wrapping_add(P0))) as u32 as u64; r8 = (((rax) as i64).wrapping_mul((0x270u64) as i64) as u64) as u64; r8 = ((r8).wrapping_add(rdx)) as u64; rcx = r9; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); }
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141db81c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0xc; const P2: u64 = 0x10; const P3: u64 = 0x18;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    xmm0 = ((crate::simd_ops::ru64(rdx)) as u64) as u128;
    crate::simd_ops::wu64(r8, ((xmm0 as u64)) as u64);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P0), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P1))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P1), ((rax as u32)) as u32);
    xmm0 = ((crate::simd_ops::ru64(rdx.wrapping_add(P2))) as u64) as u128;
    crate::simd_ops::wu64(r8.wrapping_add(P2), ((xmm0 as u64)) as u64);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P3))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P3), ((rax as u32)) as u32);
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141db8d90(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x20; const P1: u64 = 0x40;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut ymm0_hi: u128 = 0; let mut ymm1_hi: u128 = 0;
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let y_addr: u64 = rdx; xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16));
    let y_addr: u64 = r8; crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi);
    let y_addr: u64 = rdx.wrapping_add(P0); xmm1 = crate::simd_ops::ru128(y_addr); ymm1_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16));
    let y_addr: u64 = r8.wrapping_add(P0); crate::simd_ops::wu128(y_addr, xmm1); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm1_hi);
    xmm0 = (crate::simd_ops::ru128(rdx.wrapping_add(P1))) as u128; ymm0_hi = 0;
    crate::simd_ops::wu128(r8.wrapping_add(P1), (xmm0) as u128);
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141db9f00(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14240aa00; const P1: u64 = 0x30; const P2: u64 = 0xb8; const P3: u64 = 0x140; const P4: u64 = 0x1402c11a0;
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
    rdx = ((rbx.wrapping_add(P1)) as u64) as u64;
    rcx = ((rdi.wrapping_add(P1)) as u64) as u64;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rdx = ((rbx.wrapping_add(P2)) as u64) as u64;
    rcx = ((rdi.wrapping_add(P2)) as u64) as u64;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rdx = ((rbx.wrapping_add(P3)) as u64) as u64;
    rcx = ((rdi.wrapping_add(P3)) as u64) as u64;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return crate::simd_ops::tail_stk(P4, rax, rcx, rdx, r8, r9, rsp);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141db9f70(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x140; const P1: u64 = 0x140260f30; const P2: u64 = 0xb8; const P3: u64 = 0x14240ab10; const P4: u64 = 0x30;
    let (mut rax, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rcx = ((rdx.wrapping_add(P0)) as u64) as u64;
    rbx = rdx;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rcx = ((rbx.wrapping_add(P2)) as u64) as u64;
    rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp);
    rcx = ((rbx.wrapping_add(P4)) as u64) as u64;
    rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp);
    rcx = rbx;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return crate::simd_ops::tail_stk(P3, rax, rcx, rdx, r8, r9, rsp);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141db9fb0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x4; const P1: u64 = 0x8; const P2: u64 = 0x3; const P3: u64 = 0x30; const P4: u64 = 0x34; const P5: u64 = 0x38; const P6: u64 = 0x33; const P7: u64 = 0xb8; const P8: u64 = 0xbc; const P9: u64 = 0xc0; const P10: u64 = 0xbb; const P11: u64 = 0x140;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    crate::simd_ops::lk_and_u32(rdx, (0xff000005u32) as u32);
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::lk_or_u32(rdx, (0x5u32) as u32);
    crate::simd_ops::wu32(rdx.wrapping_add(P0), ((rax as u32)) as u32);
    crate::simd_ops::wu8(rdx.wrapping_add(P1), ((rax as u8)) as u8);
    crate::simd_ops::wu8(rdx.wrapping_add(P2), (0x5u8) as u8);
    crate::simd_ops::lk_and_u32(rdx.wrapping_add(P3), (0xff000010u32) as u32);
    crate::simd_ops::lk_or_u32(rdx.wrapping_add(P3), (0x10u32) as u32);
    crate::simd_ops::wu32(rdx.wrapping_add(P4), ((rax as u32)) as u32);
    crate::simd_ops::wu8(rdx.wrapping_add(P5), ((rax as u8)) as u8);
    crate::simd_ops::wu8(rdx.wrapping_add(P6), (0x10u8) as u8);
    crate::simd_ops::lk_and_u32(rdx.wrapping_add(P7), (0xff000010u32) as u32);
    fl_l = (crate::simd_ops::lk_or_u32(rdx.wrapping_add(P7), (0x10u32) as u32)) as u32 as u64; fl_r = fl_l;
    crate::simd_ops::wu32(rdx.wrapping_add(P8), ((rax as u32)) as u32);
    crate::simd_ops::wu8(rdx.wrapping_add(P9), ((rax as u8)) as u8);
    crate::simd_ops::wu8(rdx.wrapping_add(P10), (0x10u8) as u8);
    crate::simd_ops::wu64(rdx.wrapping_add(P11), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(0x148u64), rax);
    rax = rdx;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141dba020(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xfffffffffffffff8; const P1: u64 = 0xfffffffffffffff0; const P2: u64 = 0x140; const P3: u64 = 0x140260f30; const P4: u64 = 0xb8; const P5: u64 = 0x14240ab10; const P6: u64 = 0x30; const P7: u64 = 0x142442c80;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 1; continue; }
            1 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rsi); rbx = rdx; rsi = 0; fl_l = 0; fl_r = 0; fl_l = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64; fl_r = ((rsi as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 5; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rdi); bb = 3; continue; }
            3 => { rdi = rsi; rdi = (((rdi) as i64).wrapping_mul((crate::simd_ops::ru64(rbx.wrapping_add(P1))) as i64) as u64) as u64; rdi = ((rdi).wrapping_add(rbx)) as u64; rcx = ((rdi.wrapping_add(P2)) as u64) as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rcx = ((rdi.wrapping_add(P4)) as u64) as u64; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rcx = ((rdi.wrapping_add(P6)) as u64) as u64; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rcx = rdi; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru32(rbx.wrapping_add(P0))) as u32 as u64; rsi = ((rsi).wrapping_add(1)) as u64; fl_l = rsi; fl_r = rax; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 3; continue; } bb = 4; continue; }
            4 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; bb = 5; continue; }
            5 => { rcx = rbx; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); bb = 6; continue; }
            6 => { return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141dba0a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x140; const P1: u64 = 0x140260f30; const P2: u64 = 0xb8; const P3: u64 = 0x14240ab10; const P4: u64 = 0x30; const P5: u64 = 0x142442de0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    fl_l = rdx; fl_r = rdx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rcx = ((rdx.wrapping_add(P0)) as u64) as u64; rbx = rdx; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rcx = ((rbx.wrapping_add(P2)) as u64) as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rcx = ((rbx.wrapping_add(P4)) as u64) as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rcx = rbx; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rcx = rbx; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); }
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141dbb2a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14240aa00; const P1: u64 = 0x88; const P2: u64 = 0xc8; const P3: u64 = 0x90; const P4: u64 = 0x98; const P5: u64 = 0x9c; const P6: u64 = 0xa4; const P7: u64 = 0xa8; const P8: u64 = 0xb0; const P9: u64 = 0xb4; const P10: u64 = 0xbc; const P11: u64 = 0xc0;
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rcx = r8; rdi = r8; rbx = rdx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = ((crate::simd_ops::ru8(rbx.wrapping_add(P1))) as u32) as u32 as u64; rdx = ((rbx.wrapping_add(P2)) as u64) as u64; crate::simd_ops::wu8(rdi.wrapping_add(P1), ((rax as u8)) as u8); rcx = ((rdi.wrapping_add(P2)) as u64) as u64; xmm0 = ((crate::simd_ops::ru64(rbx.wrapping_add(P3))) as u64) as u128; crate::simd_ops::wu64(rdi.wrapping_add(P3), ((xmm0 as u64)) as u64); rax = (crate::simd_ops::ru32(rbx.wrapping_add(P4))) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(P4), ((rax as u32)) as u32); xmm0 = ((crate::simd_ops::ru64(rbx.wrapping_add(P5))) as u64) as u128; crate::simd_ops::wu64(rdi.wrapping_add(P5), ((xmm0 as u64)) as u64); rax = (crate::simd_ops::ru32(rbx.wrapping_add(P6))) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(P6), ((rax as u32)) as u32); xmm0 = ((crate::simd_ops::ru64(rbx.wrapping_add(P7))) as u64) as u128; crate::simd_ops::wu64(rdi.wrapping_add(P7), ((xmm0 as u64)) as u64); rax = (crate::simd_ops::ru32(rbx.wrapping_add(P8))) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(P8), ((rax as u32)) as u32); xmm0 = ((crate::simd_ops::ru64(rbx.wrapping_add(P9))) as u64) as u128; crate::simd_ops::wu64(rdi.wrapping_add(P9), ((xmm0 as u64)) as u64); rax = (crate::simd_ops::ru32(rbx.wrapping_add(P10))) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(P10), ((rax as u32)) as u32); rax = ((crate::simd_ops::ru8(rbx.wrapping_add(P11))) as u32) as u32 as u64; crate::simd_ops::wu8(rdi.wrapping_add(0xc0u64), ((rax as u8)) as u8); rax = (crate::simd_ops::ru32(rbx.wrapping_add(0xc4u64))) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0xc4u64), ((rax as u32)) as u32); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = ((crate::simd_ops::ru8(rbx.wrapping_add(0x150u64))) as u32) as u32 as u64; crate::simd_ops::wu8(rdi.wrapping_add(0x150u64), ((rax as u8)) as u8); rax = ((crate::simd_ops::ru8(rbx.wrapping_add(0x151u64))) as u32) as u32 as u64; crate::simd_ops::wu8(rdi.wrapping_add(0x151u64), ((rax as u8)) as u8); rax = ((crate::simd_ops::ru8(rbx.wrapping_add(0x152u64))) as u32) as u32 as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu8(rdi.wrapping_add(0x152u64), ((rax as u8)) as u8); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141dbbf30(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x1432673f8; const P1: u64 = 0x2f; const P2: u64 = 0x8; const P3: u64 = 0x10; const P4: u64 = 0x18; const P5: u64 = 0x20; const P6: u64 = 0x28; const P7: u64 = 0x2d; const P8: u64 = 0x2c; const P9: u64 = 0x80; const P10: u64 = 0x98; const P11: u64 = 0xac;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = P0;
    crate::simd_ops::wu8(rdx.wrapping_add(P1), (0xffu8) as u8);
    crate::simd_ops::wu64(rdx, rax);
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu64(rdx.wrapping_add(P2), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(P3), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(P4), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(P5), rax);
    crate::simd_ops::wu32(rdx.wrapping_add(P6), ((rax as u32)) as u32);
    crate::simd_ops::wu16(rdx.wrapping_add(P7), ((rax as u16)) as u16);
    crate::simd_ops::wu8(rdx.wrapping_add(P8), ((rax as u8)) as u8);
    crate::simd_ops::wu64(rdx.wrapping_add(P9), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(P10), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(P11), rax);
    crate::simd_ops::wu32(rdx.wrapping_add(0xb4u64), ((rax as u32)) as u32);
    crate::simd_ops::wu64(rdx.wrapping_add(0xbcu64), rax);
    rax = rdx;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141dbc510(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x140f0aa40; const P2: u64 = 0x18; const P3: u64 = 0x140225740;
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rax = (crate::simd_ops::ru64(rdx)) as u64;
    rcx = ((r8.wrapping_add(P0)) as u64) as u64;
    rbx = rdx;
    crate::simd_ops::wu64(r8, rax);
    rdx = ((rdx).wrapping_add(0x8u64)) as u64;
    rdi = r8;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rdx = ((rbx.wrapping_add(P2)) as u64) as u64;
    rcx = ((rdi.wrapping_add(P2)) as u64) as u64;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return crate::simd_ops::tail_stk(P3, rax, rcx, rdx, r8, r9, rsp);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141dbc560(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x18; const P1: u64 = 0x1401f2bc0; const P2: u64 = 0x8; const P3: u64 = 0x140f03410;
    let (mut rax, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rdx;
    rcx = ((rdx.wrapping_add(P0)) as u64) as u64;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rcx = ((rbx.wrapping_add(P2)) as u64) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return crate::simd_ops::tail_stk(P3, rax, rcx, rdx, r8, r9, rsp);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141dbc5b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xfffffffffffffff8; const P1: u64 = 0xfffffffffffffff0; const P2: u64 = 0x18; const P3: u64 = 0x1401f2bc0; const P4: u64 = 0x8; const P5: u64 = 0x140f03410; const P6: u64 = 0x142442c80;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    fl_l = rdx; fl_r = rdx;
    if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { return rax; }
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rdx;
    rsi = 0;
    fl_l = 0; fl_r = 0;
    fl_l = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64; fl_r = ((rsi as u32)) as u32 as u64;
    if (((fl_l as u32)) <= ((fl_r as u32))) { rcx = rbx; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    loop { rdi = rsi; rdi = (((rdi) as i64).wrapping_mul((crate::simd_ops::ru64(rbx.wrapping_add(P1))) as i64) as u64) as u64; rcx = ((rbx.wrapping_add(P2)) as u64) as u64; rcx = ((rcx).wrapping_add(rdi)) as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rcx = ((rbx.wrapping_add(P4)) as u64) as u64; rcx = ((rcx).wrapping_add(rdi)) as u64; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rsi = ((rsi).wrapping_add(1)) as u64; rax = (crate::simd_ops::ru32(rbx.wrapping_add(P0))) as u32 as u64; fl_l = rsi; fl_r = rax; if !((((fl_l as u64)) < ((fl_r as u64)))) { break; } }
    rcx = rbx;
    rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp);
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141dbc620(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x18; const P1: u64 = 0x1401f2bc0; const P2: u64 = 0x8; const P3: u64 = 0x140f03410; const P4: u64 = 0x142442de0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    fl_l = rdx; fl_r = rdx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rdx; rcx = ((rdx.wrapping_add(P0)) as u64) as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rcx = ((rbx.wrapping_add(P2)) as u64) as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rcx = rbx; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); }
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141dbcb90(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14240aa00; const P1: u64 = 0x48; const P2: u64 = 0x1401e84a0; const P3: u64 = 0x60; const P4: u64 = 0x64; const P5: u64 = 0x65;
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
    rdx = ((rbx.wrapping_add(P1)) as u64) as u64;
    rcx = ((rdi.wrapping_add(P1)) as u64) as u64;
    rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp);
    rax = (crate::simd_ops::ru32(rbx.wrapping_add(P3))) as u32 as u64;
    crate::simd_ops::wu32(rdi.wrapping_add(P3), ((rax as u32)) as u32);
    rax = ((crate::simd_ops::ru8(rbx.wrapping_add(P4))) as u32) as u32 as u64;
    crate::simd_ops::wu8(rdi.wrapping_add(P4), ((rax as u8)) as u8);
    rax = ((crate::simd_ops::ru8(rbx.wrapping_add(P5))) as u32) as u32 as u64;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    crate::simd_ops::wu8(rdi.wrapping_add(P5), ((rax as u8)) as u8);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141dbcc10(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x4; const P1: u64 = 0x8; const P2: u64 = 0x3; const P3: u64 = 0x48; const P4: u64 = 0x50; const P5: u64 = 0x58; const P6: u64 = 0x5c; const P7: u64 = 0x60;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    crate::simd_ops::lk_and_u32(rdx, (0xff000008u32) as u32);
    rax = 0;
    fl_l = 0; fl_r = 0;
    fl_l = (crate::simd_ops::lk_or_u32(rdx, (0x8u32) as u32)) as u32 as u64; fl_r = fl_l;
    crate::simd_ops::wu32(rdx.wrapping_add(P0), ((rax as u32)) as u32);
    crate::simd_ops::wu8(rdx.wrapping_add(P1), ((rax as u8)) as u8);
    crate::simd_ops::wu8(rdx.wrapping_add(P2), (0x8u8) as u8);
    crate::simd_ops::wu64(rdx.wrapping_add(P3), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(P4), rax);
    crate::simd_ops::wu32(rdx.wrapping_add(P5), ((rax as u32)) as u32);
    crate::simd_ops::wu8(rdx.wrapping_add(P6), ((rax as u8)) as u8);
    crate::simd_ops::wu32(rdx.wrapping_add(P7), ((rax as u32)) as u32);
    rax = rdx;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141dbd280(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14240aa00; const P1: u64 = 0x48; const P2: u64 = 0x90; const P3: u64 = 0x98; const P4: u64 = 0x1401fdd10; const P5: u64 = 0xa8; const P6: u64 = 0xb0;
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rcx = r8;
    rdi = r8;
    rbx = rdx;
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    rdx = ((rbx.wrapping_add(P1)) as u64) as u64;
    rcx = ((rdi.wrapping_add(P1)) as u64) as u64;
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    rax = ((crate::simd_ops::ru8(rbx.wrapping_add(P2))) as u32) as u32 as u64;
    rdx = ((rbx.wrapping_add(P3)) as u64) as u64;
    rcx = ((rdi.wrapping_add(P3)) as u64) as u64;
    crate::simd_ops::wu8(rdi.wrapping_add(P2), ((rax as u8)) as u8);
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    xmm0 = ((crate::simd_ops::ru64(rbx.wrapping_add(P5))) as u64) as u128;
    crate::simd_ops::wu64(rdi.wrapping_add(P5), ((xmm0 as u64)) as u64);
    rax = (crate::simd_ops::ru32(rbx.wrapping_add(P6))) as u32 as u64;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    crate::simd_ops::wu32(rdi.wrapping_add(P6), ((rax as u32)) as u32);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141dbd350(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x4; const P1: u64 = 0x8; const P2: u64 = 0x3; const P3: u64 = 0x48; const P4: u64 = 0x4c; const P5: u64 = 0x50; const P6: u64 = 0x4b; const P7: u64 = 0x90; const P8: u64 = 0x98; const P9: u64 = 0xa0; const P10: u64 = 0xa8; const P11: u64 = 0xb0;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    crate::simd_ops::lk_and_u32(rdx, (0xff000008u32) as u32);
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::lk_or_u32(rdx, (0x8u32) as u32);
    crate::simd_ops::wu32(rdx.wrapping_add(P0), ((rax as u32)) as u32);
    crate::simd_ops::wu8(rdx.wrapping_add(P1), ((rax as u8)) as u8);
    crate::simd_ops::wu8(rdx.wrapping_add(P2), (0x8u8) as u8);
    crate::simd_ops::lk_and_u32(rdx.wrapping_add(P3), (0xff000008u32) as u32);
    fl_l = (crate::simd_ops::lk_or_u32(rdx.wrapping_add(P3), (0x8u32) as u32)) as u32 as u64; fl_r = fl_l;
    crate::simd_ops::wu32(rdx.wrapping_add(P4), ((rax as u32)) as u32);
    crate::simd_ops::wu8(rdx.wrapping_add(P5), ((rax as u8)) as u8);
    crate::simd_ops::wu8(rdx.wrapping_add(P6), (0x8u8) as u8);
    crate::simd_ops::wu8(rdx.wrapping_add(P7), ((rax as u8)) as u8);
    crate::simd_ops::wu64(rdx.wrapping_add(P8), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(P9), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(P10), rax);
    crate::simd_ops::wu32(rdx.wrapping_add(P11), ((rax as u32)) as u32);
    rax = rdx;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141dbdb40(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x4; const P1: u64 = 0x8;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    crate::simd_ops::wu32(rdx, (0x1u32) as u32);
    rax = rdx;
    crate::simd_ops::wu32(rdx.wrapping_add(P0), (0x3f800000u32) as u32);
    crate::simd_ops::wu32(rdx.wrapping_add(P1), (0x3f800000u32) as u32);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141dbe130(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x20; const P1: u64 = 0x40; const P2: u64 = 0x60;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut ymm0_hi: u128 = 0; let mut ymm1_hi: u128 = 0;
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let y_addr: u64 = rdx; xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16));
    let y_addr: u64 = r8; crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi);
    let y_addr: u64 = rdx.wrapping_add(P0); xmm1 = crate::simd_ops::ru128(y_addr); ymm1_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16));
    let y_addr: u64 = r8.wrapping_add(P0); crate::simd_ops::wu128(y_addr, xmm1); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm1_hi);
    let y_addr: u64 = rdx.wrapping_add(P1); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16));
    let y_addr: u64 = r8.wrapping_add(P1); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi);
    let y_addr: u64 = rdx.wrapping_add(P2); xmm1 = crate::simd_ops::ru128(y_addr); ymm1_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16));
    let y_addr: u64 = r8.wrapping_add(P2); crate::simd_ops::wu128(y_addr, xmm1); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm1_hi);
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141dbe190(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x1; const P1: u64 = 0x24; const P2: u64 = 0x2c; const P3: u64 = 0x78; const P4: u64 = 0x3;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu16(rdx.wrapping_add(P0), (0x0u16) as u16);
    crate::simd_ops::wu64(rdx.wrapping_add(P1), rax);
    crate::simd_ops::wu32(rdx.wrapping_add(P2), ((rax as u32)) as u32);
    crate::simd_ops::wu64(rdx.wrapping_add(P3), rax);
    rax = rdx;
    crate::simd_ops::wu8(rdx, (0x0u8) as u8);
    crate::simd_ops::wu8(rdx.wrapping_add(P4), (0xffu8) as u8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141dbe6f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14240aa00; const P1: u64 = 0x28; const P2: u64 = 0x50; const P3: u64 = 0x2c; const P4: u64 = 0x30; const P5: u64 = 0x34; const P6: u64 = 0x38; const P7: u64 = 0x3c; const P8: u64 = 0x40; const P9: u64 = 0x44; const P10: u64 = 0x48; const P11: u64 = 0x1401e84a0;
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rcx = r8; rdi = r8; rbx = rdx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru32(rbx.wrapping_add(P1))) as u32 as u64; rdx = ((rbx.wrapping_add(P2)) as u64) as u64; crate::simd_ops::wu32(rdi.wrapping_add(P1), ((rax as u32)) as u32); rcx = ((rdi.wrapping_add(P2)) as u64) as u64; rax = (crate::simd_ops::ru32(rbx.wrapping_add(P3))) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(P3), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rbx.wrapping_add(P4))) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(P4), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rbx.wrapping_add(P5))) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(P5), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rbx.wrapping_add(P6))) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(P6), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rbx.wrapping_add(P7))) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(P7), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rbx.wrapping_add(P8))) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(P8), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rbx.wrapping_add(P9))) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(P9), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rbx.wrapping_add(P10))) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(P10), ((rax as u32)) as u32); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P11, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = rbx.wrapping_add(0x68); rcx = rdi.wrapping_add(0x68); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P11, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm0 = ((crate::simd_ops::ru64(rbx.wrapping_add(0x80u64))) as u64) as u128; crate::simd_ops::wu64(rdi.wrapping_add(0x80u64), ((xmm0 as u64)) as u64); rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x88u64))) as u32 as u64; rdx = rbx.wrapping_add(0xc0); crate::simd_ops::wu32(rdi.wrapping_add(0x88u64), ((rax as u32)) as u32); rcx = rdi.wrapping_add(0xc0); xmm0 = ((crate::simd_ops::ru64(rbx.wrapping_add(0x8cu64))) as u64) as u128; crate::simd_ops::wu64(rdi.wrapping_add(0x8cu64), ((xmm0 as u64)) as u64); rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x94u64))) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x94u64), ((rax as u32)) as u32); xmm0 = ((crate::simd_ops::ru64(rbx.wrapping_add(0x98u64))) as u64) as u128; crate::simd_ops::wu64(rdi.wrapping_add(0x98u64), ((xmm0 as u64)) as u64); rax = (crate::simd_ops::ru32(rbx.wrapping_add(0xa0u64))) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0xa0u64), ((rax as u32)) as u32); xmm0 = ((crate::simd_ops::ru64(rbx.wrapping_add(0xa4u64))) as u64) as u128; crate::simd_ops::wu64(rdi.wrapping_add(0xa4u64), ((xmm0 as u64)) as u64); rax = (crate::simd_ops::ru32(rbx.wrapping_add(0xacu64))) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0xacu64), ((rax as u32)) as u32); xmm0 = (crate::simd_ops::ru128(rbx.wrapping_add(0xb0u64))) as u128; crate::simd_ops::wu128(rdi.wrapping_add(0xb0u64), (xmm0) as u128); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P11, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = rbx.wrapping_add(0xd8); rcx = rdi.wrapping_add(0xd8); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P11, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = ((crate::simd_ops::ru8(rbx.wrapping_add(0xf0u64))) as u32) as u32 as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu8(rdi.wrapping_add(0xf0u64), ((rax as u8)) as u8); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141dbe830(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xe0; const P1: u64 = 0xc8; const P2: u64 = 0x70; const P3: u64 = 0x58; const P4: u64 = 0x14240ab10;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rcx = 0;
    fl_l = 0; fl_r = 0;
    rax = ((rcx as u32)) as u32 as u64;
    let tmp_xchg: u64 = crate::simd_ops::atomic_xchg_u64(rdx.wrapping_add(P0), rax);
    rax = (tmp_xchg) as u64;
    rax = ((rcx as u32)) as u32 as u64;
    let tmp_xchg: u64 = crate::simd_ops::atomic_xchg_u64(rdx.wrapping_add(P1), rax);
    rax = (tmp_xchg) as u64;
    rax = ((rcx as u32)) as u32 as u64;
    let tmp_xchg: u64 = crate::simd_ops::atomic_xchg_u64(rdx.wrapping_add(P2), rax);
    rax = (tmp_xchg) as u64;
    let tmp_xchg: u64 = crate::simd_ops::atomic_xchg_u64(rdx.wrapping_add(P3), rcx);
    rcx = (tmp_xchg) as u64;
    rcx = rdx;
    return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141dbe870(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xfffffffffffffff8; const P1: u64 = 0xfffffffffffffff0; const P2: u64 = 0xe0; const P3: u64 = 0xc8; const P4: u64 = 0x70; const P5: u64 = 0x58; const P6: u64 = 0x14240ab10; const P7: u64 = 0x142442c80;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    fl_l = rdx; fl_r = rdx;
    if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { return rax; }
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rdi);
    rbx = rdx;
    rdi = 0;
    fl_l = 0; fl_r = 0;
    fl_l = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64;
    if (((fl_l as u32)) <= ((fl_r as u32))) { rcx = rbx; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    loop { rax = 0; fl_l = 0; fl_r = 0; rcx = rdi; rcx = (((rcx) as i64).wrapping_mul((crate::simd_ops::ru64(rbx.wrapping_add(P1))) as i64) as u64) as u64; rcx = ((rcx).wrapping_add(rbx)) as u64; let tmp_xchg: u64 = crate::simd_ops::atomic_xchg_u64(rcx.wrapping_add(P2), rax); rax = (tmp_xchg) as u64; rax = 0; fl_l = 0; fl_r = 0; let tmp_xchg: u64 = crate::simd_ops::atomic_xchg_u64(rcx.wrapping_add(P3), rax); rax = (tmp_xchg) as u64; rax = 0; fl_l = 0; fl_r = 0; let tmp_xchg: u64 = crate::simd_ops::atomic_xchg_u64(rcx.wrapping_add(P4), rax); rax = (tmp_xchg) as u64; rax = 0; fl_l = 0; fl_r = 0; let tmp_xchg: u64 = crate::simd_ops::atomic_xchg_u64(rcx.wrapping_add(P5), rax); rax = (tmp_xchg) as u64; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru32(rbx.wrapping_add(P0))) as u32 as u64; rdi = ((rdi).wrapping_add(1)) as u64; fl_l = rdi; fl_r = rax; if !((((fl_l as u64)) < ((fl_r as u64)))) { break; } }
    rcx = rbx;
    rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp);
    rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141dbe8f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xe0; const P1: u64 = 0xc8; const P2: u64 = 0x70; const P3: u64 = 0x58; const P4: u64 = 0x14240ab10; const P5: u64 = 0x142442de0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    fl_l = rdx; fl_r = rdx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rcx = 0; fl_l = 0; fl_r = 0; rbx = rdx; rax = ((rcx as u32)) as u32 as u64; let tmp_xchg: u64 = crate::simd_ops::atomic_xchg_u64(rdx.wrapping_add(P0), rax); rax = (tmp_xchg) as u64; rax = ((rcx as u32)) as u32 as u64; let tmp_xchg: u64 = crate::simd_ops::atomic_xchg_u64(rdx.wrapping_add(P1), rax); rax = (tmp_xchg) as u64; rax = ((rcx as u32)) as u32 as u64; let tmp_xchg: u64 = crate::simd_ops::atomic_xchg_u64(rdx.wrapping_add(P2), rax); rax = (tmp_xchg) as u64; let tmp_xchg: u64 = crate::simd_ops::atomic_xchg_u64(rdx.wrapping_add(P3), rcx); rcx = (tmp_xchg) as u64; rcx = rdx; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rcx = rbx; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); }
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141dbfc10(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x4; const P2: u64 = 0x140244c00; const P3: u64 = 0x18; const P4: u64 = 0x140f1e940; const P5: u64 = 0x28; const P6: u64 = 0x140f1e870; const P7: u64 = 0x38; const P8: u64 = 0x140f1e7a0;
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rax = (crate::simd_ops::ru32(rdx)) as u32 as u64;
    rcx = ((r8.wrapping_add(P0)) as u64) as u64;
    crate::simd_ops::wu32(r8, ((rax as u32)) as u32);
    rbx = rdx;
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P1))) as u32 as u64;
    rdi = r8;
    rdx = ((rdx).wrapping_add(0x8u64)) as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P1), ((rax as u32)) as u32);
    rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp);
    rdx = ((rbx.wrapping_add(P3)) as u64) as u64;
    rcx = ((rdi.wrapping_add(P3)) as u64) as u64;
    rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp);
    rdx = ((rbx.wrapping_add(P5)) as u64) as u64;
    rcx = ((rdi.wrapping_add(P5)) as u64) as u64;
    rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp);
    rdx = ((rbx.wrapping_add(P7)) as u64) as u64;
    rcx = ((rdi.wrapping_add(P7)) as u64) as u64;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return crate::simd_ops::tail_stk(P8, rax, rcx, rdx, r8, r9, rsp);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141dbfcc0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0x18; const P3: u64 = 0x20; const P4: u64 = 0x28; const P5: u64 = 0x30; const P6: u64 = 0x38; const P7: u64 = 0x40;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu64(rdx.wrapping_add(P0), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(P1), rax);
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
pub unsafe extern "win64" fn fn_141dc0550(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143cc5058; const P1: u64 = 0x143cc5060; const P2: u64 = 0x141834a78; const P3: u64 = 0x141834a0c;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x28u64)) as u64; rax = ((crate::exe::tls_pointer() as u64)) as u64; rcx = 0x74; rax = (crate::simd_ops::ru64(rax)) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(rax))) as u32 as u64; fl_l = (crate::simd_ops::ru32(P0)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) > (((fl_r as u32)) as i32)) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru32(P1)) as u32 as u64; rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
            2 => { rcx = P0; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru32(P0)) as u32 as u64; fl_r = 0xffffffff; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 1; continue; } bb = 3; continue; }
            3 => { rcx = P0; crate::simd_ops::wu32(P1, (0x98ad5ce3u32) as u32); rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru32(P1)) as u32 as u64; rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}
