#![allow(unused_imports, unused_variables, unused_mut, unused_assignments, non_snake_case, dead_code, clippy::all, unreachable_code, unused_parens)]

#[inline(never)]
pub unsafe extern "win64" fn tpl_0<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    rax = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64;
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_1<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = (rcx) as u64;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    crate::simd_ops::wu64(rax.wrapping_add(P1), (rbx) as u64);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = core::ptr::read_unaligned(rsp as *const u64);
    rsp = rsp.wrapping_add(8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001ec5010(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001ec5120; const P1: u64 = 0x7001ec5150;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (rcx) as u64);
    crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), (rdx) as u64);
    crate::simd_ops::wu32(rsp.wrapping_add(0x40u64), ((r8 as u32)) as u32);
    rdx = (crate::simd_ops::ru64(rdx)) as u64;
    rax = (rcx) as u64;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((rax) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    rcx = (rax) as u64;
    rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64;
    rdx = (crate::simd_ops::ru64(rdx)) as u64;
    r8 = (0x2u32) as u32 as u64;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001ec5060(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001ec5120; const P1: u64 = 0x7001ec5150;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (rcx) as u64);
    crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), (rdx) as u64);
    crate::simd_ops::wu32(rsp.wrapping_add(0x40u64), ((r8 as u32)) as u32);
    rdx = (crate::simd_ops::ru64(rdx)) as u64;
    rax = (rcx) as u64;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((rax) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001ec5090(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001ec5120;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (rcx) as u64);
    crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), (rdx) as u64);
    rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64;
    rdx = (crate::simd_ops::ru64(rdx)) as u64;
    r8 = (0x2u32) as u32 as u64;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001ec50c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001ec5120; const P1: u64 = 0x7001ec5150;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (rcx) as u64);
    crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), (rdx) as u64);
    crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), (r8) as u64);
    crate::simd_ops::wu32(rsp.wrapping_add(0x48u64), ((r9 as u32)) as u32);
    r8 = ((r9 as u32)) as u32 as u64;
    rax = (rcx) as u64;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((rax) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    rcx = (rax) as u64;
    rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64;
    r8 = (0x2u32) as u32 as u64;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001ec5120(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001ec5140;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 0>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rcx) as u64);
    crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (rdx) as u64);
    crate::simd_ops::wu32(rsp.wrapping_add(0x10u64), ((r8 as u32)) as u32);
    r9 = (0x19930520u64) as u64;
    let s5 = crate::simd_ops::read_s5_tail(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x40) as *const u64); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001ec5154(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x7001ec9160; const P1: u64 = 0x7001ec5190; const P2: u64 = 0x98; const P3: u64 = 0x10; const P4: u64 = 0x8; const P5: u64 = 0x7001ec5188;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = (crate::simd_ops::ru64(P0)) as u64; rdx = ((P1) as u64) as u64; fl_l = (rax) as u64 as u64; fl_r = (rdx) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 4; continue; } bb = 1; continue; }
            1 => { rax = ((crate::exe::teb_self() as u64)) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P2))) as u64; fl_l = (rcx) as u64 as u64; fl_r = (crate::simd_ops::ru64(rax.wrapping_add(P3))) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 3; continue; } bb = 2; continue; }
            2 => { fl_l = (rcx) as u64 as u64; fl_r = (crate::simd_ops::ru64(rax.wrapping_add(P4))) as u64 as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rcx = (0xdu32) as u32 as u64; crate::lift_ops::trap(0xc0000409, P5, rcx, 0); return rax; bb = 4; continue; }
            4 => { return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001ec5190(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001ec51a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001ec5210; const P1: u64 = 0x7001ec9170;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&super::EH_META[0], rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu8(rsp.wrapping_add(0x10u64), ((rdx as u8)) as u8); rsp = ((rsp).wrapping_sub(0x48u64)) as u64; fl_l = (crate::simd_ops::ru32(rcx)) as u32 as u64; fl_r = (0xe06d7363u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 11; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(0x18u64))) as u32 as u64; fl_r = (0x4u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 11; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x20u64))) as u32 as u64; rax = (((rax as u32)).wrapping_sub(0x19930520u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x2u32) as u32 as u64; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 11; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x30u64))) as u64; fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 5; continue; }
            5 => { rdx = (((crate::simd_ops::ru32(rax.wrapping_add(0x4u64))) as i32 as i64) as u64) as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 8; continue; } bb = 6; continue; }
            6 => { rdx = ((rdx).wrapping_add((match eh.ru64(0x7001ec51d9, rcx.wrapping_add(0x38u64)) { Ok(v) => v, Err(l) => match l.cont_va { 0x7001ec51e8 => { rax = crate::dll_native::lift::eh::landed_rax(); bb = 7; continue; } _ => l.unknown(0x7001ec51a0) } }))) as u64; rcx = ((match eh.ru64(0x7001ec51dd, rcx.wrapping_add(0x28u64)) { Ok(v) => v, Err(l) => match l.cont_va { 0x7001ec51e8 => { rax = crate::dll_native::lift::eh::landed_rax(); bb = 7; continue; } _ => l.unknown(0x7001ec51a0) } })) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001ec51e1); crate::simd_ops::replace_rax_in(rax); rax = match eh.call(0x7001ec51e1, (P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8) { Ok(v) => v, Err(l) => match l.cont_va { 0x7001ec51e8 => { rax = crate::dll_native::lift::eh::landed_rax(); bb = 7; continue; } _ => l.unknown(0x7001ec51a0) } }; bb = 11; continue; }
            7 => { bb = 11; continue; }
            8 => { fl_l = (crate::simd_ops::ru8(rax)) as u8 as u64; fl_r = (0x10u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 11; continue; } bb = 9; continue; }
            9 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x28u64))) as u64; rcx = (crate::simd_ops::ru64(rax)) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 10; continue; }
            10 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001ec5202); crate::simd_ops::replace_rax_in(rax); rax = match eh.call(0x7001ec5202, ({ let t = (crate::simd_ops::ru64(P1)) as u64; if t == 0x7001ec87f0 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8) { Ok(v) => v, Err(l) => match l.cont_va { 0x7001ec51e8 => { rax = crate::dll_native::lift::eh::landed_rax(); bb = 7; continue; } _ => l.unknown(0x7001ec51a0) } }; bb = 11; continue; }
            11 => { rsp = ((rsp).wrapping_add(0x48u64)) as u64; bb = 12; continue; }
            12 => { return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001ec5210(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((rdx) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::set_xmm_ret(c_x0); return c_rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001ec5214(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001ec7524; const P1: u64 = 0x58; const P2: u64 = 0x8; const P3: u64 = 0x1;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = (rcx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rdx = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64; bb = 3; continue; }
            1 => { fl_l = (crate::simd_ops::ru64(rdx)) as u64 as u64; fl_r = (rbx) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 6; continue; } bb = 2; continue; }
            2 => { rdx = (crate::simd_ops::ru64(rdx.wrapping_add(P2))) as u64; bb = 3; continue; }
            3 => { fl_l = (rdx) as u64 as u64; fl_r = (rdx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 1; continue; } bb = 4; continue; }
            4 => { rax = ((rdx.wrapping_add(P3)) as u32) as u32 as u64; bb = 5; continue; }
            5 => { rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            6 => { rax = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; bb = 5; continue; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001ec5244(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x4; const P1: u64 = 0x8;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = (((crate::simd_ops::ru32(rdx)) as i32 as i64) as u64) as u64;
    rax = ((rax).wrapping_add(rcx)) as u64;
    fl_l = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64; fl_r = (0x0u32) as u32 as u64;
    if !(((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32))) { r9 = (((crate::simd_ops::ru32(rdx.wrapping_add(P0))) as i32 as i64) as u64) as u64; rdx = (((crate::simd_ops::ru32(rdx.wrapping_add(P1))) as i32 as i64) as u64) as u64; rcx = (crate::simd_ops::ru64(r9.wrapping_add(rcx))) as u64; r8 = (((crate::simd_ops::ru32(rdx.wrapping_add(rcx))) as i32 as i64) as u64) as u64; r8 = ((r8).wrapping_add(r9)) as u64; fl_l = ((rax).wrapping_add(r8)) as u64 as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; }
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001ec5268(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001ec7524; const P1: u64 = 0x20; const P2: u64 = 0x8; const P3: u64 = 0x28; const P4: u64 = 0x7001ec9130; const P5: u64 = 0x7001ec52ac; const P6: u64 = 0x30;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rbx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = (crate::simd_ops::ru64(rcx)) as u64; rbx = (rcx) as u64; fl_l = (crate::simd_ops::ru32(rdi)) as u32 as u64; fl_r = (0xe0434352u32) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 4; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru32(rdi)) as u32 as u64; fl_r = (0xe0434f4du32) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 4; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru32(rdi)) as u32 as u64; fl_r = (0xe06d7363u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 6; continue; } bb = 3; continue; }
            3 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); crate::simd_ops::wu64(rax.wrapping_add(P1), (rdi) as u64); rbx = (crate::simd_ops::ru64(rbx.wrapping_add(P2))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); crate::simd_ops::wu64(rax.wrapping_add(P3), (rbx) as u64); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P4)) as u64; if t == 0x7001ec87f0 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); crate::lift_ops::trap(0x80000003, P5, rcx, rsp); return rax; }
            4 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); fl_l = (crate::simd_ops::ru32(rax.wrapping_add(P6))) as u32 as u64; fl_r = (0x0u32) as u32 as u64; if ((((fl_l as u32)) as i32) <= (((fl_r as u32)) as i32)) { bb = 6; continue; } bb = 5; continue; }
            5 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); fl_l = (crate::simd_ops::lk_sub_u32(rax.wrapping_add(P6), (1) as u32)) as u32 as u64; bb = 6; continue; }
            6 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001ec52d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001ec8080; const P1: u64 = 0x7001ecc120; const P2: u64 = 0x7001ec9170; const P3: u64 = 0x7001ec9138; const P4: u64 = 0x7001ec87b1; const P5: u64 = 0x7001ec5244; const P6: u64 = 0x7001ec8094;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&super::EH_META[1], rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rsi) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r13) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r15) as u64); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; r14 = (r9) as u64; rbx = (r8) as u64; rsi = (rdx) as u64; r13 = (rcx) as u64; rdi = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; fl_l = (crate::simd_ops::ru32(r8.wrapping_add(0x8u64))) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 2; continue; } bb = 1; continue; }
            1 => { r15 = (((crate::simd_ops::ru32(r8.wrapping_add(0x8u64))) as i32 as i64) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001ec5301); crate::simd_ops::replace_rax_in(rax); rax = match eh.call(0x7001ec5301, (P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8) { Ok(v) => v, Err(l) => match l.cont_va { 0x7001ec5494 => { rax = crate::dll_native::lift::eh::landed_rax(); bb = 49; continue; } _ => l.unknown(0x7001ec52d0) } }; rdx = ((r15.wrapping_add(rax)) as u64) as u64; bb = 3; continue; }
            2 => { rdx = (rdi) as u64; r15 = ((rdi as u32)) as u32 as u64; bb = 3; continue; }
            3 => { fl_l = (rdx) as u64 as u64; fl_r = (rdx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 50; continue; } bb = 4; continue; }
            4 => { fl_l = ((r15 as u32)) as u32 as u64; fl_r = ((r15 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001ec5320); crate::simd_ops::replace_rax_in(rax); rax = match eh.call(0x7001ec5320, (P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8) { Ok(v) => v, Err(l) => match l.cont_va { 0x7001ec5494 => { rax = crate::dll_native::lift::eh::landed_rax(); bb = 49; continue; } _ => l.unknown(0x7001ec52d0) } }; rcx = (rax) as u64; rax = (((crate::simd_ops::ru32(rbx.wrapping_add(0x8u64))) as i32 as i64) as u64) as u64; fl_l = ((rcx).wrapping_add(rax)) as u64 as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; bb = 7; continue; }
            6 => { rcx = (rdi) as u64; bb = 7; continue; }
            7 => { fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0x10u64))) as u8 as u64; fl_r = ((rdi as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 50; continue; } bb = 8; continue; }
            8 => { fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0xcu64))) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 10; continue; } bb = 9; continue; }
            9 => { fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x4u64))) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) >= (((fl_r as u32)) as i32)) { bb = 50; continue; } bb = 10; continue; }
            10 => { fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x4u64))) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 12; continue; } bb = 11; continue; }
            11 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0xcu64))) as u32 as u64; fl_l = ((rax).wrapping_add(crate::simd_ops::ru64(rsi))) as u64 as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; rsi = (rax) as u64; bb = 12; continue; }
            12 => { fl_l = ((match eh.ru8(0x7001ec535a, rbx.wrapping_add(0x4u64)) { Ok(v) => v, Err(l) => match l.cont_va { 0x7001ec5494 => { rax = crate::dll_native::lift::eh::landed_rax(); bb = 49; continue; } _ => l.unknown(0x7001ec52d0) } })) as u8 as u64; fl_r = (0x80u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 19; continue; } bb = 13; continue; }
            13 => { fl_l = ((match eh.ru8(0x7001ec5360, r14) { Ok(v) => v, Err(l) => match l.cont_va { 0x7001ec5494 => { rax = crate::dll_native::lift::eh::landed_rax(); bb = 49; continue; } _ => l.unknown(0x7001ec52d0) } })) as u8 as u64; fl_r = (0x10u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 19; continue; } bb = 14; continue; }
            14 => { rax = (crate::simd_ops::ru64(P1)) as u64; fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 19; continue; } bb = 15; continue; }
            15 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001ec5372); crate::simd_ops::replace_rax_in(rax); rax = match eh.call(0x7001ec5372, ({ let t = (crate::simd_ops::ru64(P2)) as u64; if t == 0x7001ec87f0 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8) { Ok(v) => v, Err(l) => match l.cont_va { 0x7001ec5494 => { rax = crate::dll_native::lift::eh::landed_rax(); bb = 49; continue; } _ => l.unknown(0x7001ec52d0) } }; fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 18; continue; } bb = 16; continue; }
            16 => { fl_l = (rsi) as u64 as u64; fl_r = (rsi) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 18; continue; } bb = 17; continue; }
            17 => { if let Err(l) = eh.wu64(0x7001ec5382, rsi, (rax) as u64) { match l.cont_va { 0x7001ec5494 => { rax = crate::dll_native::lift::eh::landed_rax(); bb = 49; continue; } _ => l.unknown(0x7001ec52d0) } } rcx = (rax) as u64; bb = 30; continue; }
            18 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001ec538a); crate::simd_ops::replace_rax_in(rax); rax = match eh.call(0x7001ec538a, ({ let t = (crate::simd_ops::ru64(P3)) as u64; if t == 0x7001ec87f0 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8) { Ok(v) => v, Err(l) => match l.cont_va { 0x7001ec5494 => { rax = crate::dll_native::lift::eh::landed_rax(); bb = 49; continue; } _ => l.unknown(0x7001ec52d0) } }; bb = 19; continue; }
            19 => { fl_l = ((match eh.ru8(0x7001ec5390, rbx.wrapping_add(0x4u64)) { Ok(v) => v, Err(l) => match l.cont_va { 0x7001ec5494 => { rax = crate::dll_native::lift::eh::landed_rax(); bb = 49; continue; } _ => l.unknown(0x7001ec52d0) } })) as u8 as u64; fl_r = (0x8u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 24; continue; } bb = 20; continue; }
            20 => { rcx = ((match eh.ru64(0x7001ec5396, r13.wrapping_add(0x28u64)) { Ok(v) => v, Err(l) => match l.cont_va { 0x7001ec5494 => { rax = crate::dll_native::lift::eh::landed_rax(); bb = 49; continue; } _ => l.unknown(0x7001ec52d0) } })) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 23; continue; } bb = 21; continue; }
            21 => { fl_l = (rsi) as u64 as u64; fl_r = (rsi) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 23; continue; } bb = 22; continue; }
            22 => { if let Err(l) = eh.wu64(0x7001ec53a4, rsi, (rcx) as u64) { match l.cont_va { 0x7001ec5494 => { rax = crate::dll_native::lift::eh::landed_rax(); bb = 49; continue; } _ => l.unknown(0x7001ec52d0) } } bb = 30; continue; }
            23 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001ec53a9); crate::simd_ops::replace_rax_in(rax); rax = match eh.call(0x7001ec53a9, ({ let t = (crate::simd_ops::ru64(P3)) as u64; if t == 0x7001ec87f0 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8) { Ok(v) => v, Err(l) => match l.cont_va { 0x7001ec5494 => { rax = crate::dll_native::lift::eh::landed_rax(); bb = 49; continue; } _ => l.unknown(0x7001ec52d0) } }; bb = 24; continue; }
            24 => { fl_l = ((match eh.ru8(0x7001ec53af, r14) { Ok(v) => v, Err(l) => match l.cont_va { 0x7001ec5494 => { rax = crate::dll_native::lift::eh::landed_rax(); bb = 49; continue; } _ => l.unknown(0x7001ec52d0) } })) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 32; continue; } bb = 25; continue; }
            25 => { rdx = ((match eh.ru64(0x7001ec53b5, r13.wrapping_add(0x28u64)) { Ok(v) => v, Err(l) => match l.cont_va { 0x7001ec5494 => { rax = crate::dll_native::lift::eh::landed_rax(); bb = 49; continue; } _ => l.unknown(0x7001ec52d0) } })) as u64; fl_l = (rdx) as u64 as u64; fl_r = (rdx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 31; continue; } bb = 26; continue; }
            26 => { fl_l = (rsi) as u64 as u64; fl_r = (rsi) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 31; continue; } bb = 27; continue; }
            27 => { r8 = ((((match eh.ru32(0x7001ec53c3, r14.wrapping_add(0x14u64)) { Ok(v) => v, Err(l) => match l.cont_va { 0x7001ec5494 => { rax = crate::dll_native::lift::eh::landed_rax(); bb = 49; continue; } _ => l.unknown(0x7001ec52d0) } })) as i32 as i64) as u64) as u64; rcx = (rsi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001ec53ca); crate::simd_ops::replace_rax_in(rax); rax = match eh.call(0x7001ec53ca, (P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8) { Ok(v) => v, Err(l) => match l.cont_va { 0x7001ec5494 => { rax = crate::dll_native::lift::eh::landed_rax(); bb = 49; continue; } _ => l.unknown(0x7001ec52d0) } }; fl_l = ((match eh.ru32(0x7001ec53cf, r14.wrapping_add(0x14u64)) { Ok(v) => v, Err(l) => match l.cont_va { 0x7001ec5494 => { rax = crate::dll_native::lift::eh::landed_rax(); bb = 49; continue; } _ => l.unknown(0x7001ec52d0) } })) as u32 as u64; fl_r = (0x8u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 47; continue; } bb = 28; continue; }
            28 => { fl_l = ((match eh.ru64(0x7001ec53da, rsi) { Ok(v) => v, Err(l) => match l.cont_va { 0x7001ec5494 => { rax = crate::dll_native::lift::eh::landed_rax(); bb = 49; continue; } _ => l.unknown(0x7001ec52d0) } })) as u64 as u64; fl_r = (rdi) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 47; continue; } bb = 29; continue; }
            29 => { rcx = ((match eh.ru64(0x7001ec53e3, rsi) { Ok(v) => v, Err(l) => match l.cont_va { 0x7001ec5494 => { rax = crate::dll_native::lift::eh::landed_rax(); bb = 49; continue; } _ => l.unknown(0x7001ec52d0) } })) as u64; bb = 30; continue; }
            30 => { rdx = ((r14.wrapping_add(0x8u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001ec53ea); crate::simd_ops::replace_rax_in(rax); rax = match eh.call(0x7001ec53ea, (P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8) { Ok(v) => v, Err(l) => match l.cont_va { 0x7001ec5494 => { rax = crate::dll_native::lift::eh::landed_rax(); bb = 49; continue; } _ => l.unknown(0x7001ec52d0) } }; if let Err(l) = eh.wu64(0x7001ec53ef, rsi, (rax) as u64) { match l.cont_va { 0x7001ec5494 => { rax = crate::dll_native::lift::eh::landed_rax(); bb = 49; continue; } _ => l.unknown(0x7001ec52d0) } } bb = 47; continue; }
            31 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001ec53f7); crate::simd_ops::replace_rax_in(rax); rax = match eh.call(0x7001ec53f7, ({ let t = (crate::simd_ops::ru64(P3)) as u64; if t == 0x7001ec87f0 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8) { Ok(v) => v, Err(l) => match l.cont_va { 0x7001ec5494 => { rax = crate::dll_native::lift::eh::landed_rax(); bb = 49; continue; } _ => l.unknown(0x7001ec52d0) } }; bb = 32; continue; }
            32 => { fl_l = ((match eh.ru32(0x7001ec53fd, r14.wrapping_add(0x18u64)) { Ok(v) => v, Err(l) => match l.cont_va { 0x7001ec5494 => { rax = crate::dll_native::lift::eh::landed_rax(); bb = 49; continue; } _ => l.unknown(0x7001ec52d0) } })) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 34; continue; } bb = 33; continue; }
            33 => { rbx = ((((match eh.ru32(0x7001ec5403, r14.wrapping_add(0x18u64)) { Ok(v) => v, Err(l) => match l.cont_va { 0x7001ec5494 => { rax = crate::dll_native::lift::eh::landed_rax(); bb = 49; continue; } _ => l.unknown(0x7001ec52d0) } })) as i32 as i64) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001ec5407); crate::simd_ops::replace_rax_in(rax); rax = match eh.call(0x7001ec5407, (P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8) { Ok(v) => v, Err(l) => match l.cont_va { 0x7001ec5494 => { rax = crate::dll_native::lift::eh::landed_rax(); bb = 49; continue; } _ => l.unknown(0x7001ec52d0) } }; rcx = ((rbx.wrapping_add(rax)) as u64) as u64; bb = 35; continue; }
            34 => { rcx = (rdi) as u64; rbx = ((rdi as u32)) as u32 as u64; bb = 35; continue; }
            35 => { fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 40; continue; } bb = 36; continue; }
            36 => { fl_l = ((match eh.ru64(0x7001ec541c, r13.wrapping_add(0x28u64)) { Ok(v) => v, Err(l) => match l.cont_va { 0x7001ec5494 => { rax = crate::dll_native::lift::eh::landed_rax(); bb = 49; continue; } _ => l.unknown(0x7001ec52d0) } })) as u64 as u64; fl_r = (rdi) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 39; continue; } bb = 37; continue; }
            37 => { fl_l = (rsi) as u64 as u64; fl_r = (rsi) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 39; continue; } bb = 38; continue; }
            38 => { rbx = ((((match eh.ru32(0x7001ec5427, r14.wrapping_add(0x14u64)) { Ok(v) => v, Err(l) => match l.cont_va { 0x7001ec5494 => { rax = crate::dll_native::lift::eh::landed_rax(); bb = 49; continue; } _ => l.unknown(0x7001ec52d0) } })) as i32 as i64) as u64) as u64; rdx = ((r14.wrapping_add(0x8u64)) as u64) as u64; rcx = ((match eh.ru64(0x7001ec542f, r13.wrapping_add(0x28u64)) { Ok(v) => v, Err(l) => match l.cont_va { 0x7001ec5494 => { rax = crate::dll_native::lift::eh::landed_rax(); bb = 49; continue; } _ => l.unknown(0x7001ec52d0) } })) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001ec5433); crate::simd_ops::replace_rax_in(rax); rax = match eh.call(0x7001ec5433, (P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8) { Ok(v) => v, Err(l) => match l.cont_va { 0x7001ec5494 => { rax = crate::dll_native::lift::eh::landed_rax(); bb = 49; continue; } _ => l.unknown(0x7001ec52d0) } }; rdx = (rax) as u64; r8 = (rbx) as u64; rcx = (rsi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001ec5441); crate::simd_ops::replace_rax_in(rax); rax = match eh.call(0x7001ec5441, (P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8) { Ok(v) => v, Err(l) => match l.cont_va { 0x7001ec5494 => { rax = crate::dll_native::lift::eh::landed_rax(); bb = 49; continue; } _ => l.unknown(0x7001ec52d0) } }; bb = 47; continue; }
            39 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001ec5448); crate::simd_ops::replace_rax_in(rax); rax = match eh.call(0x7001ec5448, ({ let t = (crate::simd_ops::ru64(P3)) as u64; if t == 0x7001ec87f0 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8) { Ok(v) => v, Err(l) => match l.cont_va { 0x7001ec5494 => { rax = crate::dll_native::lift::eh::landed_rax(); bb = 49; continue; } _ => l.unknown(0x7001ec52d0) } }; bb = 40; continue; }
            40 => { fl_l = ((match eh.ru64(0x7001ec544e, r13.wrapping_add(0x28u64)) { Ok(v) => v, Err(l) => match l.cont_va { 0x7001ec5494 => { rax = crate::dll_native::lift::eh::landed_rax(); bb = 49; continue; } _ => l.unknown(0x7001ec52d0) } })) as u64 as u64; fl_r = (rdi) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 48; continue; } bb = 41; continue; }
            41 => { fl_l = (rsi) as u64 as u64; fl_r = (rsi) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 48; continue; } bb = 42; continue; }
            42 => { fl_l = ((rbx as u32)) as u32 as u64; fl_r = ((rbx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 44; continue; } bb = 43; continue; }
            43 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001ec545d); crate::simd_ops::replace_rax_in(rax); rax = match eh.call(0x7001ec545d, (P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8) { Ok(v) => v, Err(l) => match l.cont_va { 0x7001ec5494 => { rax = crate::dll_native::lift::eh::landed_rax(); bb = 49; continue; } _ => l.unknown(0x7001ec52d0) } }; rcx = (rax) as u64; rax = ((((match eh.ru32(0x7001ec5465, r14.wrapping_add(0x18u64)) { Ok(v) => v, Err(l) => match l.cont_va { 0x7001ec5494 => { rax = crate::dll_native::lift::eh::landed_rax(); bb = 49; continue; } _ => l.unknown(0x7001ec52d0) } })) as i32 as i64) as u64) as u64; fl_l = ((rcx).wrapping_add(rax)) as u64 as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; bb = 45; continue; }
            44 => { rcx = (rdi) as u64; bb = 45; continue; }
            45 => { fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 48; continue; } bb = 46; continue; }
            46 => { rax = (rax & !0xffu64) | ((((match eh.ru8(0x7001ec5476, r14) { Ok(v) => v, Err(l) => match l.cont_va { 0x7001ec5494 => { rax = crate::dll_native::lift::eh::landed_rax(); bb = 49; continue; } _ => l.unknown(0x7001ec52d0) } })) as u8 as u64)); rax = (rax & !0xffu64) | ((((((rax as u8)) & (0x4u8))) as u8 as u64)); fl_l = 0; fl_r = ((rax as u8)) as u8 as u64; rax = (rax & !0xffu64) | (((0u8.wrapping_sub(fl_r as u8)) as u8 as u64)); rcx = (if (((fl_l as u8)) < ((fl_r as u8))) { 0xffffffffu32 } else { 0u32 }) as u32 as u64; rcx = (0u32.wrapping_sub((rcx as u32))) as u32 as u64; fl_l = (((rcx as u32)).wrapping_add(1)) as u32 as u64; rcx = ((fl_l as u32)) as u32 as u64; rdi = ((rcx as u32)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rcx as u32)) as u32); bb = 47; continue; }
            47 => { rax = ((rdi as u32)) as u32 as u64; bb = 51; continue; }
            48 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001ec548d); crate::simd_ops::replace_rax_in(rax); rax = match eh.call(0x7001ec548d, ({ let t = (crate::simd_ops::ru64(P3)) as u64; if t == 0x7001ec87f0 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8) { Ok(v) => v, Err(l) => match l.cont_va { 0x7001ec5494 => { rax = crate::dll_native::lift::eh::landed_rax(); bb = 49; continue; } _ => l.unknown(0x7001ec52d0) } }; bb = 49; continue; }
            49 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001ec5494); crate::simd_ops::replace_rax_in(rax); rax = match eh.call(0x7001ec5494, ({ let t = (crate::simd_ops::ru64(P3)) as u64; if t == 0x7001ec87f0 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8) { Ok(v) => v, Err(l) => match l.cont_va { 0x7001ec5494 => { rax = crate::dll_native::lift::eh::landed_rax(); bb = 49; continue; } _ => l.unknown(0x7001ec52d0) } }; bb = 50; continue; }
            50 => { rax = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; bb = 51; continue; }
            51 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; r15 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r13 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001ec54b8(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001ec52d0; const P1: u64 = 0x7001ec5244; const P2: u64 = 0x7001ec8094; const P3: u64 = 0x7001ec6fb4; const P4: u64 = 0x7001ec6fa8; const P5: u64 = 0x7001ec9138;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&super::EH_META[2], rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rsi) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = (r9) as u64; r14 = (rcx) as u64; rbx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; fl_l = (crate::simd_ops::ru32(r8.wrapping_add(0x4u64))) as u32 as u64; fl_r = ((rbx as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) >= (((fl_r as u32)) as i32)) { bb = 2; continue; } bb = 1; continue; }
            1 => { rsi = (rdx) as u64; bb = 3; continue; }
            2 => { rsi = (crate::simd_ops::ru32(r8.wrapping_add(0xcu64))) as u32 as u64; fl_l = ((rsi).wrapping_add(crate::simd_ops::ru64(rdx))) as u64 as u64; fl_r = fl_l; rsi = ((fl_l as u64)) as u64; bb = 3; continue; }
            3 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001ec54e7); crate::simd_ops::replace_rax_in(rax); rax = match eh.call(0x7001ec54e7, (P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8) { Ok(v) => v, Err(l) => match l.cont_va { 0x7001ec555e => { rax = crate::dll_native::lift::eh::landed_rax(); bb = 12; continue; } _ => l.unknown(0x7001ec54b8) } }; fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x1u32) as u32 as u64; rax = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 8; continue; } bb = 4; continue; }
            4 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x1u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 11; continue; } bb = 5; continue; }
            5 => { rdx = ((rdi.wrapping_add(0x8u64)) as u64) as u64; rcx = ((match eh.ru64(0x7001ec54fa, r14.wrapping_add(0x28u64)) { Ok(v) => v, Err(l) => match l.cont_va { 0x7001ec555e => { rax = crate::dll_native::lift::eh::landed_rax(); bb = 12; continue; } _ => l.unknown(0x7001ec54b8) } })) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001ec54fe); crate::simd_ops::replace_rax_in(rax); rax = match eh.call(0x7001ec54fe, (P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8) { Ok(v) => v, Err(l) => match l.cont_va { 0x7001ec555e => { rax = crate::dll_native::lift::eh::landed_rax(); bb = 12; continue; } _ => l.unknown(0x7001ec54b8) } }; r14 = (rax) as u64; fl_l = ((match eh.ru32(0x7001ec5506, rdi.wrapping_add(0x18u64)) { Ok(v) => v, Err(l) => match l.cont_va { 0x7001ec555e => { rax = crate::dll_native::lift::eh::landed_rax(); bb = 12; continue; } _ => l.unknown(0x7001ec54b8) } })) as u32 as u64; fl_r = ((rbx as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 7; continue; } bb = 6; continue; }
            6 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001ec550b); crate::simd_ops::replace_rax_in(rax); rax = match eh.call(0x7001ec550b, (P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8) { Ok(v) => v, Err(l) => match l.cont_va { 0x7001ec555e => { rax = crate::dll_native::lift::eh::landed_rax(); bb = 12; continue; } _ => l.unknown(0x7001ec54b8) } }; rbx = ((((match eh.ru32(0x7001ec5510, rdi.wrapping_add(0x18u64)) { Ok(v) => v, Err(l) => match l.cont_va { 0x7001ec555e => { rax = crate::dll_native::lift::eh::landed_rax(); bb = 12; continue; } _ => l.unknown(0x7001ec54b8) } })) as i32 as i64) as u64) as u64; fl_l = ((rbx).wrapping_add(rax)) as u64 as u64; fl_r = fl_l; rbx = ((fl_l as u64)) as u64; bb = 7; continue; }
            7 => { r9 = (0x1u32) as u32 as u64; r8 = (r14) as u64; rdx = (rbx) as u64; rcx = (rsi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001ec5526); crate::simd_ops::replace_rax_in(rax); rax = match eh.call(0x7001ec5526, (P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8) { Ok(v) => v, Err(l) => match l.cont_va { 0x7001ec555e => { rax = crate::dll_native::lift::eh::landed_rax(); bb = 12; continue; } _ => l.unknown(0x7001ec54b8) } }; bb = 11; continue; }
            8 => { rdx = ((rdi.wrapping_add(0x8u64)) as u64) as u64; rcx = ((match eh.ru64(0x7001ec5531, r14.wrapping_add(0x28u64)) { Ok(v) => v, Err(l) => match l.cont_va { 0x7001ec555e => { rax = crate::dll_native::lift::eh::landed_rax(); bb = 12; continue; } _ => l.unknown(0x7001ec54b8) } })) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001ec5535); crate::simd_ops::replace_rax_in(rax); rax = match eh.call(0x7001ec5535, (P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8) { Ok(v) => v, Err(l) => match l.cont_va { 0x7001ec555e => { rax = crate::dll_native::lift::eh::landed_rax(); bb = 12; continue; } _ => l.unknown(0x7001ec54b8) } }; r14 = (rax) as u64; fl_l = ((match eh.ru32(0x7001ec553d, rdi.wrapping_add(0x18u64)) { Ok(v) => v, Err(l) => match l.cont_va { 0x7001ec555e => { rax = crate::dll_native::lift::eh::landed_rax(); bb = 12; continue; } _ => l.unknown(0x7001ec54b8) } })) as u32 as u64; fl_r = ((rbx as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 10; continue; } bb = 9; continue; }
            9 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001ec5542); crate::simd_ops::replace_rax_in(rax); rax = match eh.call(0x7001ec5542, (P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8) { Ok(v) => v, Err(l) => match l.cont_va { 0x7001ec555e => { rax = crate::dll_native::lift::eh::landed_rax(); bb = 12; continue; } _ => l.unknown(0x7001ec54b8) } }; rbx = ((((match eh.ru32(0x7001ec5547, rdi.wrapping_add(0x18u64)) { Ok(v) => v, Err(l) => match l.cont_va { 0x7001ec555e => { rax = crate::dll_native::lift::eh::landed_rax(); bb = 12; continue; } _ => l.unknown(0x7001ec54b8) } })) as i32 as i64) as u64) as u64; fl_l = ((rbx).wrapping_add(rax)) as u64 as u64; fl_r = fl_l; rbx = ((fl_l as u64)) as u64; bb = 10; continue; }
            10 => { r8 = (r14) as u64; rdx = (rbx) as u64; rcx = (rsi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001ec5557); crate::simd_ops::replace_rax_in(rax); rax = match eh.call(0x7001ec5557, (P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8) { Ok(v) => v, Err(l) => match l.cont_va { 0x7001ec555e => { rax = crate::dll_native::lift::eh::landed_rax(); bb = 12; continue; } _ => l.unknown(0x7001ec54b8) } }; bb = 11; continue; }
            11 => { bb = 13; continue; }
            12 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001ec555e); crate::simd_ops::replace_rax_in(rax); rax = match eh.call(0x7001ec555e, ({ let t = (crate::simd_ops::ru64(P5)) as u64; if t == 0x7001ec87f0 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8) { Ok(v) => v, Err(l) => match l.cont_va { 0x7001ec555e => { rax = crate::dll_native::lift::eh::landed_rax(); bb = 12; continue; } _ => l.unknown(0x7001ec54b8) } }; bb = 13; continue; }
            13 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001ec557c(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x18; const P2: u64 = 0x10; const P3: u64 = 0x7001ec7b7c; const P4: u64 = 0x7001ec54b8; const P5: u64 = 0x7001ec8080; const P6: u64 = 0x7001ec7d0c;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 19>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = (rsp) as u64; crate::simd_ops::wu64(rax.wrapping_add(P0), (rbx) as u64); crate::simd_ops::wu64(rax.wrapping_add(P1), (r8) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r12) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r13) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r15) as u64); rsp = ((rsp).wrapping_sub(0x60u64)) as u64; r13 = (crate::simd_ops::ru64(rsp.wrapping_add(0xc0u64))) as u64; r15 = (r9) as u64; r12 = (rdx) as u64; r9 = ((rax.wrapping_add(P2)) as u64) as u64; rbp = (rcx) as u64; r8 = (r13) as u64; rdx = (r15) as u64; rcx = (r12) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); r9 = (crate::simd_ops::ru64(rsp.wrapping_add(0xd0u64))) as u64; r14 = (rax) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0xc8u64))) as u64; fl_l = (r9) as u64 as u64; fl_r = (r9) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { r8 = (rsi) as u64; rdx = (rax) as u64; rcx = (rbp) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 2; continue; }
            2 => { rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0xd8u64))) as u64; rbx = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; rdi = (crate::simd_ops::ru32(rcx)) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rcx = (((crate::simd_ops::ru32(rsi.wrapping_add(P2))) as i32 as i64) as u64) as u64; r9 = (r14) as u64; r8 = (crate::simd_ops::ru64(rsp.wrapping_add(0xb0u64))) as u64; rax = ((rax).wrapping_add(rcx)) as u64; rcx = (rcx & !0xffu64) | (((crate::simd_ops::ru8(rsp.wrapping_add(0xf8u64))) as u8 as u64)); rdx = (rbp) as u64; crate::simd_ops::wu8(rsp.wrapping_add(0x50u64), ((rcx as u8)) as u8); rcx = (r12) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), (r15) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), (rsi) as u64); crate::simd_ops::wu32(rsp.wrapping_add(0x38u64), ((rbx as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((rdi as u32)) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (r13) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rax) as u64); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0xa0u64))) as u64; rsp = ((rsp).wrapping_add(0x60u64)) as u64; r15 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r13 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r12 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rsi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rbp = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001ec5650(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001ecc040; const P1: u64 = 0x7001ec78c0; const P2: u64 = 0x7001ec7448; const P3: u64 = 0x7001ec4000; const P4: u64 = 0x7001ec7524; const P5: u64 = 0x7001ec80c0; const P6: u64 = 0x7001ec6d94; const P7: u64 = 0x7001ec6e84; const P8: u64 = 0x7001ec9130; const P9: u64 = 0x7001ec57fd; const P10: u64 = 0x7001ec6424; const P11: u64 = 0x7001ec7ba0; const P12: u64 = 0x7001ec63a0; const P13: u64 = 0x7001ec8094; const P14: u64 = 0x7001ec5f7c; const P15: u64 = 0x7001ec684c; const P16: u64 = 0x7001ec557c; const P17: u64 = 0x7001ec7b40; const P18: u64 = 0x7001ec5ab1; const P19: u64 = 0x7001ec5b44; const P20: u64 = 0x7001ec9138; const P21: u64 = 0x7001ec5afc; const P22: u64 = 0x7001ec51a0; const P23: u64 = 0x7001ec6544; const P24: u64 = 0x7001ecacc0; const P25: u64 = 0x7001ec7244; const P26: u64 = 0x7001ec5b40;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 57>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r12) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r13) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r15) as u64); rbp = ((rsp.wrapping_sub(0x88u64)) as u64) as u64; rsp = ((rsp).wrapping_sub(0x188u64)) as u64; rax = (crate::simd_ops::ru64(P0)) as u64; rax = (((rax) ^ (rsp))) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x70u64), (rax) as u64); r14 = (crate::simd_ops::ru64(rbp.wrapping_add(0xf0u64))) as u64; r15 = (rdx) as u64; r12 = (crate::simd_ops::ru64(rbp.wrapping_add(0x108u64))) as u64; rbx = (rcx) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x78u64), (rdx) as u64); rcx = (r14) as u64; rdx = (r9) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x60u64), (r12) as u64); rsi = (r9) as u64; crate::simd_ops::wu8(rsp.wrapping_add(0x60u64), (0x0u8) as u8); r13 = (r8) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; fl_l = (crate::simd_ops::ru32(rsi.wrapping_add(0x48u64))) as u32 as u64; fl_r = (0x0u32) as u32 as u64; rdi = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 3; continue; } bb = 1; continue; }
            1 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; fl_l = (crate::simd_ops::ru32(rax)) as u32 as u64; fl_r = (0xfffffffeu32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 49; continue; } bb = 2; continue; }
            2 => { rdi = (crate::simd_ops::ru32(rsi.wrapping_add(0x48u64))) as u32 as u64; fl_l = ((rdi as u32)) as u32 as u64; fl_r = (0x2u32) as u32 as u64; rdi = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 5; continue; }
            3 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; fl_l = (crate::simd_ops::ru32(rax)) as u32 as u64; fl_r = (0xfffffffeu32) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 5; continue; } bb = 4; continue; }
            4 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdi = (crate::simd_ops::ru32(rax)) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu32(rax, (0xfffffffeu32) as u32); bb = 5; continue; }
            5 => { fl_l = ((rdi as u32)) as u32 as u64; fl_r = (0xffffffffu32) as u32 as u64; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 49; continue; } bb = 6; continue; }
            6 => { fl_l = (crate::simd_ops::ru32(r14.wrapping_add(0x8u64))) as u32 as u64; fl_r = (0x0u32) as u32 as u64; r8 = ((P3) as u64) as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 8; continue; } bb = 7; continue; }
            7 => { rdx = (((crate::simd_ops::ru32(r14.wrapping_add(0x8u64))) as i32 as i64) as u64) as u64; rdx = ((rdx).wrapping_add(crate::simd_ops::ru64(rsi.wrapping_add(0x8u64)))) as u64; rcx = ((crate::simd_ops::ru8(rdx)) as u32) as u32 as u64; rcx = ((((rcx as u32)) & (0xfu32))) as u32 as u64; rax = (((crate::simd_ops::ru8(rcx.wrapping_add(r8).wrapping_add(0x5208u64))) as i8 as i64) as u64) as u64; rcx = (rcx & !0xffu64) | (((crate::simd_ops::ru8(rcx.wrapping_add(r8).wrapping_add(0x5218u64))) as u8 as u64)); rdx = ((rdx).wrapping_sub(rax)) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_sub(0x4u64))) as u32 as u64; fl_l = (((rax as u32)).wrapping_shr(((rcx as u32) & 31))) as u32 as u64; rax = ((fl_l as u32)) as u32 as u64; bb = 9; continue; }
            8 => { rax = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; bb = 9; continue; }
            9 => { fl_l = ((rdi as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) >= (((fl_r as u32)) as i32)) { bb = 49; continue; } bb = 10; continue; }
            10 => { fl_l = (crate::simd_ops::ru32(rbx)) as u32 as u64; fl_r = (0xe06d7363u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 24; continue; } bb = 11; continue; }
            11 => { fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x18u64))) as u32 as u64; fl_r = (0x4u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 24; continue; } bb = 12; continue; }
            12 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x20u64))) as u32 as u64; rax = (((rax as u32)).wrapping_sub(0x19930520u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x2u32) as u32 as u64; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 24; continue; } bb = 13; continue; }
            13 => { fl_l = (crate::simd_ops::ru64(rbx.wrapping_add(0x30u64))) as u64 as u64; fl_r = (0x0u64) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 24; continue; } bb = 14; continue; }
            14 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; fl_l = (crate::simd_ops::ru64(rax.wrapping_add(0x20u64))) as u64 as u64; fl_r = (0x0u64) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 50; continue; } bb = 15; continue; }
            15 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rbx = (crate::simd_ops::ru64(rax.wrapping_add(0x20u64))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x38u64))) as u64; crate::simd_ops::wu8(rsp.wrapping_add(0x60u64), (0x1u8) as u8); r13 = (crate::simd_ops::ru64(rax.wrapping_add(0x28u64))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; fl_l = (crate::simd_ops::ru32(rbx)) as u32 as u64; fl_r = (0xe06d7363u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 19; continue; } bb = 16; continue; }
            16 => { fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x18u64))) as u32 as u64; fl_r = (0x4u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 19; continue; } bb = 17; continue; }
            17 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x20u64))) as u32 as u64; rax = (((rax as u32)).wrapping_sub(0x19930520u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x2u32) as u32 as u64; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 19; continue; } bb = 18; continue; }
            18 => { fl_l = (crate::simd_ops::ru64(rbx.wrapping_add(0x30u64))) as u64 as u64; fl_r = (0x0u64) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 49; continue; } bb = 19; continue; }
            19 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; fl_l = (crate::simd_ops::ru64(rax.wrapping_add(0x38u64))) as u64 as u64; fl_r = (0x0u64) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 24; continue; } bb = 20; continue; }
            20 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; r15 = (crate::simd_ops::ru64(rax.wrapping_add(0x38u64))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdx = (r15) as u64; rcx = (rbx) as u64; crate::simd_ops::lk_and_u64(rax.wrapping_add(0x38u64), (0x0u64) as u64); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 23; continue; } bb = 21; continue; }
            21 => { rcx = (r15) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P7) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 51; continue; } bb = 22; continue; }
            22 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P8)) as u64; if t == 0x7001ec87f0 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::lift_ops::trap(0x80000003, P9, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            23 => { r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; bb = 24; continue; }
            24 => { r8 = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; rcx = ((rbp.wrapping_sub(0x10u64)) as u64) as u64; rdx = (r14) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P10) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; fl_l = (crate::simd_ops::ru32(rbx)) as u32 as u64; fl_r = (0xe06d7363u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 45; continue; } bb = 25; continue; }
            25 => { fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x18u64))) as u32 as u64; fl_r = (0x4u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 45; continue; } bb = 26; continue; }
            26 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x20u64))) as u32 as u64; rax = (((rax as u32)).wrapping_sub(0x19930520u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x2u32) as u32 as u64; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 45; continue; } bb = 27; continue; }
            27 => { fl_l = (crate::simd_ops::ru32(rbp.wrapping_sub(0x10u64))) as u32 as u64; fl_r = (0x0u32) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 42; continue; } bb = 28; continue; }
            28 => { rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x100u64))) as u32 as u64; rdx = ((rbp.wrapping_sub(0x10u64)) as u64) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((rax as u32)) as u32); rcx = ((rbp.wrapping_sub(0x58u64)) as u64) as u64; r9 = (rsi) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (r14) as u64); r8 = ((rdi as u32)) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P11) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; xmm1 = (crate::simd_ops::ru128(rbp.wrapping_sub(0x58u64))) as u128; xmm0 = (xmm1) as u128; xmm0 = crate::simd_ops::vpsrldq(xmm0, 0x8u8); rax = ((xmm0 as u32)) as u32 as u64; crate::simd_ops::wu128(rbp.wrapping_sub(0x78u64), (xmm1) as u128); fl_l = ((rax as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rbp.wrapping_sub(0x40u64))) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 42; continue; } bb = 29; continue; }
            29 => { rax = (crate::simd_ops::ru32(rbp.wrapping_sub(0x70u64))) as u32 as u64; r15 = ((xmm1 as u64)) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x80u64), (r15) as u64); crate::simd_ops::wu32(rsp.wrapping_add(0x68u64), ((rax as u32)) as u32); bb = 30; continue; }
            30 => { xmm0 = (crate::simd_ops::ru128(r15.wrapping_add(0x18u64))) as u128; rax = ((xmm0 as u64)) as u64; crate::simd_ops::wu128(rbp.wrapping_sub(0x78u64), (xmm0) as u128); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) > (((fl_r as u32)) as i32)) { bb = 41; continue; } bb = 31; continue; }
            31 => { rax = ((rax).wrapping_shr(32u32)) as u64; fl_l = ((rdi as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) > (((fl_r as u32)) as i32)) { bb = 41; continue; } bb = 32; continue; }
            32 => { r9 = (crate::simd_ops::ru64(rsi.wrapping_add(0x10u64))) as u64; rdx = ((rbp.wrapping_sub(0x78u64)) as u64) as u64; r8 = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; rcx = ((rbp.wrapping_add(0x20u64)) as u64) as u64; r9 = (crate::simd_ops::ru32(r9)) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P12) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x20u64))) as u32 as u64; r12 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rsp.wrapping_add(0x64u64), ((r12 as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x6cu64), ((rax as u32)) as u32); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 41; continue; } bb = 33; continue; }
            33 => { xmm0 = (crate::simd_ops::ru128(rbp.wrapping_add(0x38u64))) as u128; xmm1 = (crate::simd_ops::ru128(rbp.wrapping_add(0x48u64))) as u128; crate::simd_ops::wu128(rbp.wrapping_sub(0x38u64), (xmm0) as u128); xmm0 = ((crate::simd_ops::ru64(rbp.wrapping_add(0x58u64))) as u64) as u128; crate::simd_ops::wu64(rbp.wrapping_sub(0x18u64), ((xmm0 as u64)) as u64); crate::simd_ops::wu128(rbp.wrapping_sub(0x28u64), (xmm1) as u128); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P13) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x30u64))) as u64; rax = ((rax).wrapping_add(0x4u64)) as u64; rdx = (((crate::simd_ops::ru32(rcx.wrapping_add(0xcu64))) as i32 as i64) as u64) as u64; rax = ((rax).wrapping_add(rdx)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), (rax) as u64); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P13) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x30u64))) as u64; rdx = (((crate::simd_ops::ru32(rcx.wrapping_add(0xcu64))) as i32 as i64) as u64) as u64; r15 = (crate::simd_ops::ru32(rax.wrapping_add(rdx))) as u32 as u64; fl_l = ((r15 as u32)) as u32 as u64; fl_r = ((r15 as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 37; continue; } bb = 34; continue; }
            34 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P13) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; r8 = (crate::simd_ops::ru64(rbx.wrapping_add(0x30u64))) as u64; r12 = (rax) as u64; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rcx = (((crate::simd_ops::ru32(rax)) as i32 as i64) as u64) as u64; r12 = ((r12).wrapping_add(rcx)) as u64; rcx = ((rbp.wrapping_sub(0x38u64)) as u64) as u64; rdx = (r12) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P14) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 39; continue; } bb = 35; continue; }
            35 => { crate::simd_ops::lk_add_u64(rsp.wrapping_add(0x70u64), (0x4u64) as u64); r15 = (((r15 as u32)).wrapping_sub(1)) as u32 as u64; fl_l = ((r15 as u32)) as u32 as u64; fl_r = ((r15 as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) > 0) { bb = 34; continue; } bb = 36; continue; }
            36 => { r12 = (crate::simd_ops::ru32(rsp.wrapping_add(0x64u64))) as u32 as u64; bb = 37; continue; }
            37 => { rcx = ((rbp.wrapping_add(0x20u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P15) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; r12 = (((r12 as u32)).wrapping_add(1)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x64u64), ((r12 as u32)) as u32); fl_l = ((r12 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rsp.wrapping_add(0x6cu64))) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 33; continue; } bb = 38; continue; }
            38 => { bb = 40; continue; }
            39 => { rax = (rax & !0xffu64) | (((crate::simd_ops::ru8(rbp.wrapping_add(0xf8u64))) as u8 as u64)); r9 = (rsi) as u64; rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; r8 = (r13) as u64; crate::simd_ops::wu8(rsp.wrapping_add(0x58u64), ((rax as u8)) as u8); rcx = (rbx) as u64; rax = (rax & !0xffu64) | (((crate::simd_ops::ru8(rsp.wrapping_add(0x60u64))) as u8 as u64)); crate::simd_ops::wu8(rsp.wrapping_add(0x50u64), ((rax as u8)) as u8); rax = (crate::simd_ops::ru64(rbp.wrapping_sub(0x60u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), (rax) as u64); rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x100u64))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x40u64), ((rax as u32)) as u32); rax = ((rbp.wrapping_sub(0x78u64)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), (rax) as u64); rax = ((rbp.wrapping_sub(0x38u64)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (r12) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (rax) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (r14) as u64); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P16) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 40; continue; }
            40 => { r15 = (crate::simd_ops::ru64(rbp.wrapping_sub(0x80u64))) as u64; bb = 41; continue; }
            41 => { r8 = (crate::simd_ops::ru64(r15.wrapping_add(0x8u64))) as u64; rdx = ((P3) as u64) as u64; rcx = ((crate::simd_ops::ru8(r8)) as u32) as u32 as u64; rcx = ((((rcx as u32)) & (0xfu32))) as u32 as u64; rax = (((crate::simd_ops::ru8(rcx.wrapping_add(rdx).wrapping_add(0x5208u64))) as i8 as i64) as u64) as u64; rcx = (rcx & !0xffu64) | (((crate::simd_ops::ru8(rcx.wrapping_add(rdx).wrapping_add(0x5218u64))) as u8 as u64)); r8 = ((r8).wrapping_sub(rax)) as u64; rax = (crate::simd_ops::ru32(r8.wrapping_sub(0x4u64))) as u32 as u64; rax = (((rax as u32)).wrapping_shr(((rcx as u32) & 31))) as u32 as u64; crate::simd_ops::wu64(r15.wrapping_add(0x8u64), (r8) as u64); crate::simd_ops::wu32(r15.wrapping_add(0x18u64), ((rax as u32)) as u32); rcx = ((crate::simd_ops::ru8(r8)) as u32) as u32 as u64; rcx = ((((rcx as u32)) & (0xfu32))) as u32 as u64; rax = (((crate::simd_ops::ru8(rcx.wrapping_add(rdx).wrapping_add(0x5208u64))) as i8 as i64) as u64) as u64; rcx = (rcx & !0xffu64) | (((crate::simd_ops::ru8(rcx.wrapping_add(rdx).wrapping_add(0x5218u64))) as u8 as u64)); r8 = ((r8).wrapping_sub(rax)) as u64; rax = (crate::simd_ops::ru32(r8.wrapping_sub(0x4u64))) as u32 as u64; rax = (((rax as u32)).wrapping_shr(((rcx as u32) & 31))) as u32 as u64; crate::simd_ops::wu64(r15.wrapping_add(0x8u64), (r8) as u64); crate::simd_ops::wu32(r15.wrapping_add(0x1cu64), ((rax as u32)) as u32); rcx = ((crate::simd_ops::ru8(r8)) as u32) as u32 as u64; rcx = ((((rcx as u32)) & (0xfu32))) as u32 as u64; rax = (((crate::simd_ops::ru8(rcx.wrapping_add(rdx).wrapping_add(0x5208u64))) as i8 as i64) as u64) as u64; rcx = (rcx & !0xffu64) | (((crate::simd_ops::ru8(rcx.wrapping_add(rdx).wrapping_add(0x5218u64))) as u8 as u64)); r8 = ((r8).wrapping_sub(rax)) as u64; rax = (crate::simd_ops::ru32(r8.wrapping_sub(0x4u64))) as u32 as u64; rax = (((rax as u32)).wrapping_shr(((rcx as u32) & 31))) as u32 as u64; rcx = (crate::simd_ops::ru32(rsp.wrapping_add(0x68u64))) as u32 as u64; crate::simd_ops::wu32(r15.wrapping_add(0x20u64), ((rax as u32)) as u32); rcx = (((rcx as u32)).wrapping_add(1)) as u32 as u64; crate::simd_ops::wu64(r15.wrapping_add(0x8u64), (r8) as u64); rax = ((r8.wrapping_add(0x4u64)) as u64) as u64; rdx = (crate::simd_ops::ru32(r8)) as u32 as u64; crate::simd_ops::wu64(r15.wrapping_add(0x8u64), (rax) as u64); crate::simd_ops::wu32(r15.wrapping_add(0x24u64), ((rdx as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x68u64), ((rcx as u32)) as u32); fl_l = ((rcx as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rbp.wrapping_sub(0x40u64))) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 30; continue; } bb = 42; continue; }
            42 => { fl_l = (crate::simd_ops::ru8(r14)) as u8 as u64; fl_r = (0x40u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 48; continue; } bb = 43; continue; }
            43 => { rdx = (r14) as u64; rcx = (rsi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P17) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 48; continue; } bb = 44; continue; }
            44 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu64(rax.wrapping_add(0x20u64), (rbx) as u64); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu64(rax.wrapping_add(0x28u64), (r13) as u64); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P8)) as u64; if t == 0x7001ec87f0 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::lift_ops::trap(0x80000003, P18, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            45 => { fl_l = (crate::simd_ops::ru32(rbp.wrapping_sub(0x10u64))) as u32 as u64; fl_r = (0x0u32) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 48; continue; } bb = 46; continue; }
            46 => { fl_l = (crate::simd_ops::ru8(rbp.wrapping_add(0xf8u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 49; continue; } bb = 47; continue; }
            47 => { rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x100u64))) as u32 as u64; r9 = (rsi) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), (r12) as u64); r8 = (r13) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((rax as u32)) as u32); rdx = (r15) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((rdi as u32)) as u32); rcx = (rbx) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (r14) as u64); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P19) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 48; continue; }
            48 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; fl_l = (crate::simd_ops::ru64(rax.wrapping_add(0x38u64))) as u64 as u64; fl_r = (0x0u64) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 50; continue; } bb = 49; continue; }
            49 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P20)) as u64; if t == 0x7001ec87f0 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::lift_ops::trap(0x80000003, P21, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            50 => { rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x70u64))) as u64; rcx = (((rcx) ^ (rsp))) as u64; rsp = ((rsp).wrapping_add(0x188u64)) as u64; r15 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r13 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r12 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rsi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rbx = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rbp = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            51 => { rdx = (rdx & !0xffu64) | (((0x1u8) as u8 as u64)); rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P22) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = ((rbp.wrapping_sub(0x78u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P23) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdx = ((P24) as u64) as u64; rcx = ((rbp.wrapping_sub(0x78u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P25) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::lift_ops::trap(0x80000003, P26, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001ec5b44(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001ecc040; const P1: u64 = 0x7001ec7524; const P2: u64 = 0x7001ec90a8; const P3: u64 = 0x7001ec79b8; const P4: u64 = 0x7001ec6424; const P5: u64 = 0x7001ec7ba0; const P6: u64 = 0x7001ec4000; const P7: u64 = 0x7001ec63a0; const P8: u64 = 0x7001ec684c; const P9: u64 = 0x7001ec8080; const P10: u64 = 0x7001ec557c; const P11: u64 = 0x7001ec9138; const P12: u64 = 0x7001ec5e38;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 47>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r12) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r13) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r15) as u64); rbp = ((rsp.wrapping_sub(0x38u64)) as u64) as u64; rsp = ((rsp).wrapping_sub(0x138u64)) as u64; rax = (crate::simd_ops::ru64(P0)) as u64; rax = (((rax) ^ (rsp))) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x28u64), (rax) as u64); fl_l = (crate::simd_ops::ru32(rcx)) as u32 as u64; fl_r = (0x80000003u32) as u32 as u64; rdi = (r9) as u64; rax = (crate::simd_ops::ru64(rbp.wrapping_add(0xb8u64))) as u64; r13 = (rdx) as u64; r14 = (crate::simd_ops::ru64(rbp.wrapping_add(0xa0u64))) as u64; rsi = (rcx) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), (rax) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x78u64), (r8) as u64); if (((fl_l as u32)) == ((fl_r as u32))) { bb = 23; continue; } bb = 1; continue; }
            1 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; r12 = (crate::simd_ops::ru32(rbp.wrapping_add(0xb0u64))) as u32 as u64; r15 = (crate::simd_ops::ru32(rbp.wrapping_add(0xa8u64))) as u32 as u64; fl_l = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64 as u64; fl_r = (0x0u64) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 6; continue; } bb = 2; continue; }
            2 => { rcx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P2)) as u64; if t == 0x7001ec87f0 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rbx = (rax) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; fl_l = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64 as u64; fl_r = (rbx) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 6; continue; } bb = 3; continue; }
            3 => { fl_l = (crate::simd_ops::ru32(rsi)) as u32 as u64; fl_r = (0xe0434f4du32) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 6; continue; } bb = 4; continue; }
            4 => { fl_l = (crate::simd_ops::ru32(rsi)) as u32 as u64; fl_r = (0xe0434352u32) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; r9 = (rdi) as u64; r8 = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; rdx = (r13) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x38u64), ((r15 as u32)) as u32); rcx = (rsi) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (rax) as u64); crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((r12 as u32)) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (r14) as u64); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 23; continue; } bb = 6; continue; }
            6 => { r8 = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; rcx = ((rbp) as u64) as u64; rdx = (r14) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; fl_l = (crate::simd_ops::ru32(rbp)) as u32 as u64; fl_r = (0x0u32) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 24; continue; } bb = 7; continue; }
            7 => { crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((r12 as u32)) as u32); rdx = ((rbp) as u64) as u64; r9 = (rdi) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (r14) as u64); r8 = ((r15 as u32)) as u32 as u64; rcx = ((rbp.wrapping_sub(0x70u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; xmm1 = (crate::simd_ops::ru128(rbp.wrapping_sub(0x70u64))) as u128; xmm0 = (xmm1) as u128; xmm0 = crate::simd_ops::vpsrldq(xmm0, 0x8u8); rax = ((xmm0 as u32)) as u32 as u64; crate::simd_ops::wu128(rbp.wrapping_sub(0x80u64), (xmm1) as u128); fl_l = ((rax as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rbp.wrapping_sub(0x58u64))) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 23; continue; } bb = 8; continue; }
            8 => { rax = (crate::simd_ops::ru32(rbp.wrapping_sub(0x78u64))) as u32 as u64; r9 = ((P6) as u64) as u64; r8 = ((xmm1 as u64)) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x60u64), ((rax as u32)) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), (r8) as u64); bb = 9; continue; }
            9 => { xmm0 = (crate::simd_ops::ru128(r8.wrapping_add(0x18u64))) as u128; rax = ((xmm0 as u64)) as u64; crate::simd_ops::wu128(rbp.wrapping_sub(0x80u64), (xmm0) as u128); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((r15 as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) > (((fl_r as u32)) as i32)) { bb = 22; continue; } bb = 10; continue; }
            10 => { rax = ((rax).wrapping_shr(32u32)) as u64; fl_l = ((r15 as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) > (((fl_r as u32)) as i32)) { bb = 22; continue; } bb = 11; continue; }
            11 => { r9 = (crate::simd_ops::ru64(rdi.wrapping_add(0x10u64))) as u64; rdx = ((rbp.wrapping_sub(0x80u64)) as u64) as u64; r8 = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; rcx = ((rbp.wrapping_sub(0x50u64)) as u64) as u64; r9 = (crate::simd_ops::ru32(r9)) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P7) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = (crate::simd_ops::ru64(rbp.wrapping_sub(0x40u64))) as u64; rcx = ((rbp.wrapping_sub(0x50u64)) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x48u64), (rax) as u64); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P8) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = (crate::simd_ops::ru64(rbp.wrapping_sub(0x40u64))) as u64; rcx = ((rbp.wrapping_sub(0x50u64)) as u64) as u64; rbx = (crate::simd_ops::ru32(rbp.wrapping_sub(0x50u64))) as u32 as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x48u64), (rax) as u64); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P8) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; fl_l = ((rbx as u32)) as u32 as u64; fl_r = (0x1u32) as u32 as u64; rbx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 13; continue; } bb = 12; continue; }
            12 => { rcx = ((rbp.wrapping_sub(0x50u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P8) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; fl_l = (rbx) as u64 as u64; fl_r = (0x1u64) as u64 as u64; rbx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 12; continue; } bb = 13; continue; }
            13 => { fl_l = (crate::simd_ops::ru32(rbp.wrapping_sub(0x30u64))) as u32 as u64; fl_r = (0x0u32) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 19; continue; } bb = 14; continue; }
            14 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P9) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdx = (((crate::simd_ops::ru32(rbp.wrapping_sub(0x30u64))) as i32 as i64) as u64) as u64; fl_l = ((rax).wrapping_add(rdx)) as u64 as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; if (((fl_l as u64)) == 0) { bb = 19; continue; } bb = 15; continue; }
            15 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 17; continue; } bb = 16; continue; }
            16 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P9) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = (((crate::simd_ops::ru32(rbp.wrapping_sub(0x30u64))) as i32 as i64) as u64) as u64; fl_l = ((rax).wrapping_add(rcx)) as u64 as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; bb = 18; continue; }
            17 => { rax = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; bb = 18; continue; }
            18 => { fl_l = (crate::simd_ops::ru8(rax.wrapping_add(0x10u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 21; continue; } bb = 19; continue; }
            19 => { fl_l = (crate::simd_ops::ru8(rbp.wrapping_sub(0x34u64))) as u8 as u64; fl_r = (0x40u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 21; continue; } bb = 20; continue; }
            20 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; r9 = (rdi) as u64; r8 = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; rdx = (r13) as u64; crate::simd_ops::wu8(rsp.wrapping_add(0x58u64), (0x0u8) as u8); rcx = (rsi) as u64; crate::simd_ops::wu8(rsp.wrapping_add(0x50u64), (0x1u8) as u8); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), (rax) as u64); rax = ((rbp.wrapping_sub(0x80u64)) as u64) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x40u64), ((r12 as u32)) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), (rax) as u64); rax = ((rbp.wrapping_sub(0x38u64)) as u64) as u64; crate::simd_ops::lk_and_u64(rsp.wrapping_add(0x30u64), (0x0u64) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (rax) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (r14) as u64); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P10) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 21; continue; }
            21 => { r8 = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; r9 = ((P6) as u64) as u64; bb = 22; continue; }
            22 => { rdx = (crate::simd_ops::ru64(r8.wrapping_add(0x8u64))) as u64; rcx = ((crate::simd_ops::ru8(rdx)) as u32) as u32 as u64; rcx = ((((rcx as u32)) & (0xfu32))) as u32 as u64; rax = (((crate::simd_ops::ru8(rcx.wrapping_add(r9).wrapping_add(0x5208u64))) as i8 as i64) as u64) as u64; rcx = (rcx & !0xffu64) | (((crate::simd_ops::ru8(rcx.wrapping_add(r9).wrapping_add(0x5218u64))) as u8 as u64)); rdx = ((rdx).wrapping_sub(rax)) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_sub(0x4u64))) as u32 as u64; rax = (((rax as u32)).wrapping_shr(((rcx as u32) & 31))) as u32 as u64; crate::simd_ops::wu64(r8.wrapping_add(0x8u64), (rdx) as u64); crate::simd_ops::wu32(r8.wrapping_add(0x18u64), ((rax as u32)) as u32); rcx = ((crate::simd_ops::ru8(rdx)) as u32) as u32 as u64; rcx = ((((rcx as u32)) & (0xfu32))) as u32 as u64; rax = (((crate::simd_ops::ru8(rcx.wrapping_add(r9).wrapping_add(0x5208u64))) as i8 as i64) as u64) as u64; rcx = (rcx & !0xffu64) | (((crate::simd_ops::ru8(rcx.wrapping_add(r9).wrapping_add(0x5218u64))) as u8 as u64)); rdx = ((rdx).wrapping_sub(rax)) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_sub(0x4u64))) as u32 as u64; rax = (((rax as u32)).wrapping_shr(((rcx as u32) & 31))) as u32 as u64; crate::simd_ops::wu64(r8.wrapping_add(0x8u64), (rdx) as u64); crate::simd_ops::wu32(r8.wrapping_add(0x1cu64), ((rax as u32)) as u32); rcx = ((crate::simd_ops::ru8(rdx)) as u32) as u32 as u64; rcx = ((((rcx as u32)) & (0xfu32))) as u32 as u64; rax = (((crate::simd_ops::ru8(rcx.wrapping_add(r9).wrapping_add(0x5208u64))) as i8 as i64) as u64) as u64; rcx = (rcx & !0xffu64) | (((crate::simd_ops::ru8(rcx.wrapping_add(r9).wrapping_add(0x5218u64))) as u8 as u64)); rdx = ((rdx).wrapping_sub(rax)) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_sub(0x4u64))) as u32 as u64; rax = (((rax as u32)).wrapping_shr(((rcx as u32) & 31))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(0x20u64), ((rax as u32)) as u32); rax = ((rdx.wrapping_add(0x4u64)) as u64) as u64; crate::simd_ops::wu64(r8.wrapping_add(0x8u64), (rdx) as u64); rcx = (crate::simd_ops::ru32(rdx)) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(0x24u64), ((rcx as u32)) as u32); rcx = (crate::simd_ops::ru32(rsp.wrapping_add(0x60u64))) as u32 as u64; rcx = (((rcx as u32)).wrapping_add(1)) as u32 as u64; crate::simd_ops::wu64(r8.wrapping_add(0x8u64), (rax) as u64); crate::simd_ops::wu32(rsp.wrapping_add(0x60u64), ((rcx as u32)) as u32); fl_l = ((rcx as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rbp.wrapping_sub(0x58u64))) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 9; continue; } bb = 23; continue; }
            23 => { rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x28u64))) as u64; rcx = (((rcx) ^ (rsp))) as u64; rsp = ((rsp).wrapping_add(0x138u64)) as u64; r15 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r13 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r12 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rsi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rbx = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rbp = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            24 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P11)) as u64; if t == 0x7001ec87f0 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::lift_ops::trap(0x80000003, P12, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001ec5e3c(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001ec8080; const P1: u64 = 0x7001ec8094;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = (rsp) as u64; crate::simd_ops::wu64(rax.wrapping_add(0x8u64), (rbx) as u64); crate::simd_ops::wu64(rax.wrapping_add(0x10u64), (rbp) as u64); crate::simd_ops::wu64(rax.wrapping_add(0x18u64), (rsi) as u64); crate::simd_ops::wu64(rax.wrapping_add(0x20u64), (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; r14 = (r8) as u64; rbp = (rdx) as u64; rdi = (rcx) as u64; fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(0x4u64))) as u32 as u64; fl_r = ((rbx as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 30; continue; } bb = 1; continue; }
            1 => { rsi = (((crate::simd_ops::ru32(rcx.wrapping_add(0x4u64))) as i32 as i64) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); r9 = (rax) as u64; fl_l = ((r9).wrapping_add(rsi)) as u64 as u64; fl_r = fl_l; r9 = ((fl_l as u64)) as u64; if (((fl_l as u64)) == 0) { bb = 30; continue; } bb = 2; continue; }
            2 => { fl_l = ((rsi as u32)) as u32 as u64; fl_r = ((rsi as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { rsi = (((crate::simd_ops::ru32(rdi.wrapping_add(0x4u64))) as i32 as i64) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rcx = ((rsi.wrapping_add(rax)) as u64) as u64; bb = 5; continue; }
            4 => { rcx = (rbx) as u64; rsi = ((rbx as u32)) as u32 as u64; bb = 5; continue; }
            5 => { fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0x10u64))) as u8 as u64; fl_r = ((rbx as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 30; continue; } bb = 6; continue; }
            6 => { fl_l = (crate::simd_ops::ru8(rdi)) as u8 as u64; fl_r = (0x80u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 8; continue; } bb = 7; continue; }
            7 => { fl_l = (crate::simd_ops::ru8(rbp)) as u8 as u64; fl_r = (0x10u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 30; continue; } bb = 8; continue; }
            8 => { fl_l = ((rsi as u32)) as u32 as u64; fl_r = ((rsi as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 10; continue; } bb = 9; continue; }
            9 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rsi = (rax) as u64; rax = (((crate::simd_ops::ru32(rdi.wrapping_add(0x4u64))) as i32 as i64) as u64) as u64; fl_l = ((rsi).wrapping_add(rax)) as u64 as u64; fl_r = fl_l; rsi = ((fl_l as u64)) as u64; bb = 11; continue; }
            10 => { rsi = (rbx) as u64; bb = 11; continue; }
            11 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rcx = (rax) as u64; rax = (((crate::simd_ops::ru32(rbp.wrapping_add(0x4u64))) as i32 as i64) as u64) as u64; rcx = ((rcx).wrapping_add(rax)) as u64; fl_l = (rsi) as u64 as u64; fl_r = (rcx) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 20; continue; } bb = 12; continue; }
            12 => { fl_l = (crate::simd_ops::ru32(rdi.wrapping_add(0x4u64))) as u32 as u64; fl_r = ((rbx as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 14; continue; } bb = 13; continue; }
            13 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rsi = (rax) as u64; rax = (((crate::simd_ops::ru32(rdi.wrapping_add(0x4u64))) as i32 as i64) as u64) as u64; fl_l = ((rsi).wrapping_add(rax)) as u64 as u64; fl_r = fl_l; rsi = ((fl_l as u64)) as u64; bb = 15; continue; }
            14 => { rsi = (rbx) as u64; bb = 15; continue; }
            15 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); r8 = (((crate::simd_ops::ru32(rbp.wrapping_add(0x4u64))) as i32 as i64) as u64) as u64; r8 = ((r8).wrapping_add(0x10u64)) as u64; r8 = ((r8).wrapping_add(rax)) as u64; rax = ((rsi.wrapping_add(0x10u64)) as u64) as u64; fl_l = (r8) as u64 as u64; fl_r = (rax) as u64 as u64; r8 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; bb = 16; continue; }
            16 => { rcx = ((crate::simd_ops::ru8(rax)) as u32) as u32 as u64; rdx = ((crate::simd_ops::ru8(rax.wrapping_add(r8))) as u32) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 18; continue; } bb = 17; continue; }
            17 => { rax = ((rax).wrapping_add(1)) as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 16; continue; } bb = 18; continue; }
            18 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 20; continue; } bb = 19; continue; }
            19 => { rax = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; bb = 31; continue; }
            20 => { rax = (rax & !0xffu64) | (((0x2u8) as u8 as u64)); fl_l = (crate::simd_ops::ru8(rbp)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 22; continue; } bb = 21; continue; }
            21 => { fl_l = (crate::simd_ops::ru8(rdi)) as u8 as u64; fl_r = (0x8u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 29; continue; } bb = 22; continue; }
            22 => { fl_l = (crate::simd_ops::ru8(r14)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 24; continue; } bb = 23; continue; }
            23 => { fl_l = (crate::simd_ops::ru8(rdi)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 29; continue; } bb = 24; continue; }
            24 => { fl_l = (crate::simd_ops::ru8(r14)) as u8 as u64; fl_r = (0x4u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 26; continue; } bb = 25; continue; }
            25 => { fl_l = (crate::simd_ops::ru8(rdi)) as u8 as u64; fl_r = (0x4u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 29; continue; } bb = 26; continue; }
            26 => { fl_l = (crate::simd_ops::ru8(r14)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 28; continue; } bb = 27; continue; }
            27 => { fl_l = (crate::simd_ops::ru8(rdi)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 29; continue; } bb = 28; continue; }
            28 => { rbx = (0x1u32) as u32 as u64; bb = 29; continue; }
            29 => { rax = ((rbx as u32)) as u32 as u64; bb = 31; continue; }
            30 => { rax = (0x1u32) as u32 as u64; bb = 31; continue; }
            31 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001ec5f7c(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001ec8080; const P1: u64 = 0x7001ec8094;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rbp) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r15) as u64); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; r15 = (r8) as u64; r14 = (rdx) as u64; rdi = (rcx) as u64; fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(0x8u64))) as u32 as u64; fl_r = ((rbx as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 33; continue; } bb = 1; continue; }
            1 => { rsi = (((crate::simd_ops::ru32(rcx.wrapping_add(0x8u64))) as i32 as i64) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); r8 = (rax) as u64; fl_l = ((r8).wrapping_add(rsi)) as u64 as u64; fl_r = fl_l; r8 = ((fl_l as u64)) as u64; if (((fl_l as u64)) == 0) { bb = 33; continue; } bb = 2; continue; }
            2 => { fl_l = ((rsi as u32)) as u32 as u64; fl_r = ((rsi as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { rbp = (((crate::simd_ops::ru32(rdi.wrapping_add(0x8u64))) as i32 as i64) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rcx = ((rax.wrapping_add(rbp)) as u64) as u64; bb = 5; continue; }
            4 => { rcx = (rbx) as u64; rbp = ((rbx as u32)) as u32 as u64; bb = 5; continue; }
            5 => { fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0x10u64))) as u8 as u64; fl_r = ((rbx as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 33; continue; } bb = 6; continue; }
            6 => { rsi = ((rdi.wrapping_add(0x4u64)) as u64) as u64; fl_l = (crate::simd_ops::ru8(rsi)) as u8 as u64; fl_r = (0x80u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 8; continue; } bb = 7; continue; }
            7 => { fl_l = (crate::simd_ops::ru8(r14)) as u8 as u64; fl_r = (0x10u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 33; continue; } bb = 8; continue; }
            8 => { fl_l = ((rbp as u32)) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 10; continue; } bb = 9; continue; }
            9 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rbp = (rax) as u64; rax = (((crate::simd_ops::ru32(rdi.wrapping_add(0x8u64))) as i32 as i64) as u64) as u64; fl_l = ((rbp).wrapping_add(rax)) as u64 as u64; fl_r = fl_l; rbp = ((fl_l as u64)) as u64; bb = 11; continue; }
            10 => { rbp = (rbx) as u64; bb = 11; continue; }
            11 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rcx = (rax) as u64; rax = (((crate::simd_ops::ru32(r14.wrapping_add(0x4u64))) as i32 as i64) as u64) as u64; rcx = ((rcx).wrapping_add(rax)) as u64; fl_l = (rbp) as u64 as u64; fl_r = (rcx) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 21; continue; } bb = 12; continue; }
            12 => { fl_l = (crate::simd_ops::ru32(rdi.wrapping_add(0x8u64))) as u32 as u64; fl_r = ((rbx as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 14; continue; } bb = 13; continue; }
            13 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rsi = (rax) as u64; rax = (((crate::simd_ops::ru32(rdi.wrapping_add(0x8u64))) as i32 as i64) as u64) as u64; fl_l = ((rsi).wrapping_add(rax)) as u64 as u64; fl_r = fl_l; rsi = ((fl_l as u64)) as u64; bb = 15; continue; }
            14 => { rsi = (rbx) as u64; bb = 15; continue; }
            15 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); r8 = (((crate::simd_ops::ru32(r14.wrapping_add(0x4u64))) as i32 as i64) as u64) as u64; r8 = ((r8).wrapping_add(0x10u64)) as u64; r8 = ((r8).wrapping_add(rax)) as u64; rax = ((rsi.wrapping_add(0x10u64)) as u64) as u64; fl_l = (r8) as u64 as u64; fl_r = (rax) as u64 as u64; r8 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; bb = 16; continue; }
            16 => { rcx = ((crate::simd_ops::ru8(rax)) as u32) as u32 as u64; rdx = ((crate::simd_ops::ru8(rax.wrapping_add(r8))) as u32) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 18; continue; } bb = 17; continue; }
            17 => { rax = ((rax).wrapping_add(1)) as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 16; continue; } bb = 18; continue; }
            18 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 20; continue; } bb = 19; continue; }
            19 => { rax = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; bb = 34; continue; }
            20 => { rsi = ((rdi.wrapping_add(0x4u64)) as u64) as u64; bb = 21; continue; }
            21 => { rax = (rax & !0xffu64) | (((0x2u8) as u8 as u64)); fl_l = (crate::simd_ops::ru8(r14)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 24; continue; } bb = 22; continue; }
            22 => { fl_l = (crate::simd_ops::ru8(rsi)) as u8 as u64; fl_r = (0x8u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 32; continue; } bb = 23; continue; }
            23 => { fl_l = ((rdi).wrapping_add(0x4u64)) as u64 as u64; fl_r = fl_l; rdi = ((fl_l as u64)) as u64; bb = 25; continue; }
            24 => { rdi = (rsi) as u64; bb = 25; continue; }
            25 => { fl_l = (crate::simd_ops::ru8(r15)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 27; continue; } bb = 26; continue; }
            26 => { fl_l = (crate::simd_ops::ru8(rsi)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 32; continue; } bb = 27; continue; }
            27 => { fl_l = (crate::simd_ops::ru8(r15)) as u8 as u64; fl_r = (0x4u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 29; continue; } bb = 28; continue; }
            28 => { fl_l = (crate::simd_ops::ru8(rsi)) as u8 as u64; fl_r = (0x4u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 32; continue; } bb = 29; continue; }
            29 => { fl_l = (crate::simd_ops::ru8(r15)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 31; continue; } bb = 30; continue; }
            30 => { fl_l = (crate::simd_ops::ru8(rdi)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 32; continue; } bb = 31; continue; }
            31 => { rbx = (0x1u32) as u32 as u64; bb = 32; continue; }
            32 => { rax = ((rbx as u32)) as u32 as u64; bb = 34; continue; }
            33 => { rax = (0x1u32) as u32 as u64; bb = 34; continue; }
            34 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001ec60c8(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001ec5154; const P1: u64 = 0x7001ec7524; const P2: u64 = 0x7001ec4000; const P3: u64 = 0x7001ec78c8; const P4: u64 = 0x7001ec6a94; const P5: u64 = 0x7001ec9138; const P6: u64 = 0x7001ec624d; const P7: u64 = 0x7001ec7b48; const P8: u64 = 0x7001ec6424; const P9: u64 = 0x7001ec8094; const P10: u64 = 0x7001ec9170; const P11: u64 = 0x7001ec5650;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 19>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rbp) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r15) as u64); rsp = ((rsp).wrapping_sub(0x80u64)) as u64; rbx = (rcx) as u64; rbp = (r9) as u64; rcx = (r8) as u64; r15 = (r8) as u64; r14 = (rdx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0xc0u64))) as u64; rsi = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; r8 = (0x80000029u32) as u32 as u64; r9 = (0x80000026u32) as u32 as u64; fl_l = (crate::simd_ops::ru32(rax.wrapping_add(0x40u64))) as u32 as u64; fl_r = ((rsi as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 7; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru32(rbx)) as u32 as u64; fl_r = (0xe06d7363u32) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 7; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru32(rbx)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 5; continue; } bb = 3; continue; }
            3 => { fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x18u64))) as u32 as u64; fl_r = (0xfu32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 6; continue; } bb = 4; continue; }
            4 => { fl_l = (crate::simd_ops::ru64(rbx.wrapping_add(0x60u64))) as u64 as u64; fl_r = (0x19930520u64) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 7; continue; } bb = 5; continue; }
            5 => { fl_l = (crate::simd_ops::ru32(rbx)) as u32 as u64; fl_r = ((r9 as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 7; continue; } bb = 6; continue; }
            6 => { fl_l = (crate::simd_ops::ru8(rdi)) as u8 as u64; fl_r = (0x20u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 34; continue; } bb = 7; continue; }
            7 => { fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(0x4u64))) as u8 as u64; fl_r = (0x66u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 25; continue; } bb = 8; continue; }
            8 => { fl_l = (crate::simd_ops::ru32(rdi.wrapping_add(0x8u64))) as u32 as u64; fl_r = ((rsi as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 34; continue; } bb = 9; continue; }
            9 => { rdx = (((crate::simd_ops::ru32(rdi.wrapping_add(0x8u64))) as i32 as i64) as u64) as u64; r15 = ((P2) as u64) as u64; rdx = ((rdx).wrapping_add(crate::simd_ops::ru64(rbp.wrapping_add(0x8u64)))) as u64; rcx = ((crate::simd_ops::ru8(rdx)) as u32) as u32 as u64; rcx = ((((rcx as u32)) & (0xfu32))) as u32 as u64; rax = (((crate::simd_ops::ru8(rcx.wrapping_add(r15).wrapping_add(0x5208u64))) as i8 as i64) as u64) as u64; rcx = (rcx & !0xffu64) | (((crate::simd_ops::ru8(rcx.wrapping_add(r15).wrapping_add(0x5218u64))) as u8 as u64)); rdx = ((rdx).wrapping_sub(rax)) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_sub(0x4u64))) as u32 as u64; rax = (((rax as u32)).wrapping_shr(((rcx as u32) & 31))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 34; continue; } bb = 10; continue; }
            10 => { fl_l = (crate::simd_ops::ru32(rsp.wrapping_add(0xc8u64))) as u32 as u64; fl_r = ((rsi as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 34; continue; } bb = 11; continue; }
            11 => { fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(0x4u64))) as u8 as u64; fl_r = (0x20u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 24; continue; } bb = 12; continue; }
            12 => { fl_l = (crate::simd_ops::ru32(rbx)) as u32 as u64; fl_r = ((r9 as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 19; continue; } bb = 13; continue; }
            13 => { r8 = (crate::simd_ops::ru64(rbp.wrapping_add(0x20u64))) as u64; rdx = (rbp) as u64; rcx = (rdi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); r9 = ((rax as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (0xffffffffu32) as u32 as u64; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 23; continue; } bb = 14; continue; }
            14 => { fl_l = (crate::simd_ops::ru32(rdi.wrapping_add(0x8u64))) as u32 as u64; fl_r = ((rsi as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 16; continue; } bb = 15; continue; }
            15 => { rdx = (((crate::simd_ops::ru32(rdi.wrapping_add(0x8u64))) as i32 as i64) as u64) as u64; rdx = ((rdx).wrapping_add(crate::simd_ops::ru64(rbp.wrapping_add(0x8u64)))) as u64; rcx = ((crate::simd_ops::ru8(rdx)) as u32) as u32 as u64; rcx = ((((rcx as u32)) & (0xfu32))) as u32 as u64; rax = (((crate::simd_ops::ru8(rcx.wrapping_add(r15).wrapping_add(0x5208u64))) as i8 as i64) as u64) as u64; rcx = (rcx & !0xffu64) | (((crate::simd_ops::ru8(rcx.wrapping_add(r15).wrapping_add(0x5218u64))) as u8 as u64)); rdx = ((rdx).wrapping_sub(rax)) as u64; rsi = (crate::simd_ops::ru32(rdx.wrapping_sub(0x4u64))) as u32 as u64; fl_l = (((rsi as u32)).wrapping_shr(((rcx as u32) & 31))) as u32 as u64; rsi = ((fl_l as u32)) as u32 as u64; bb = 16; continue; }
            16 => { fl_l = ((r9 as u32)) as u32 as u64; fl_r = ((rsi as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) >= (((fl_r as u32)) as i32)) { bb = 23; continue; } bb = 17; continue; }
            17 => { rcx = (r14) as u64; bb = 18; continue; }
            18 => { rdx = (rbp) as u64; r8 = (rdi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 34; continue; }
            19 => { fl_l = (crate::simd_ops::ru32(rbx)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 24; continue; } bb = 20; continue; }
            20 => { r9 = (crate::simd_ops::ru32(rbx.wrapping_add(0x38u64))) as u32 as u64; fl_l = ((r9 as u32)) as u32 as u64; fl_r = (0xffffffffu32) as u32 as u64; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 23; continue; } bb = 21; continue; }
            21 => { rdx = (((crate::simd_ops::ru32(rdi.wrapping_add(0x8u64))) as i32 as i64) as u64) as u64; rdx = ((rdx).wrapping_add(crate::simd_ops::ru64(rbp.wrapping_add(0x8u64)))) as u64; rcx = ((crate::simd_ops::ru8(rdx)) as u32) as u32 as u64; rcx = ((((rcx as u32)) & (0xfu32))) as u32 as u64; rax = (((crate::simd_ops::ru8(rcx.wrapping_add(r15).wrapping_add(0x5208u64))) as i8 as i64) as u64) as u64; rcx = (rcx & !0xffu64) | (((crate::simd_ops::ru8(rcx.wrapping_add(r15).wrapping_add(0x5218u64))) as u8 as u64)); rdx = ((rdx).wrapping_sub(rax)) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_sub(0x4u64))) as u32 as u64; rax = (((rax as u32)).wrapping_shr(((rcx as u32) & 31))) as u32 as u64; fl_l = ((r9 as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) >= (((fl_r as u32)) as i32)) { bb = 23; continue; } bb = 22; continue; }
            22 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x28u64))) as u64; bb = 18; continue; }
            23 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P5)) as u64; if t == 0x7001ec87f0 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); crate::lift_ops::trap(0x80000003, P6, rcx, rsp); return rax; }
            24 => { r8 = (rdi) as u64; rdx = (rbp) as u64; rcx = (r14) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P7) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 34; continue; }
            25 => { r8 = (crate::simd_ops::ru64(rbp.wrapping_add(0x8u64))) as u64; rcx = ((rsp.wrapping_add(0x50u64)) as u64) as u64; rdx = (rdi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P8) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); fl_l = (crate::simd_ops::ru32(rsp.wrapping_add(0x50u64))) as u32 as u64; fl_r = ((rsi as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 27; continue; } bb = 26; continue; }
            26 => { fl_l = (crate::simd_ops::ru8(rdi)) as u8 as u64; fl_r = (0x40u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 34; continue; } bb = 27; continue; }
            27 => { fl_l = (crate::simd_ops::ru32(rbx)) as u32 as u64; fl_r = (0xe06d7363u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 33; continue; } bb = 28; continue; }
            28 => { fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x18u64))) as u32 as u64; fl_r = (0x3u32) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 33; continue; } bb = 29; continue; }
            29 => { fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x20u64))) as u32 as u64; fl_r = (0x19930522u32) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 33; continue; } bb = 30; continue; }
            30 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x30u64))) as u64; fl_l = (crate::simd_ops::ru32(rax.wrapping_add(0x8u64))) as u32 as u64; fl_r = ((rsi as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 33; continue; } bb = 31; continue; }
            31 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P9) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x30u64))) as u64; r10 = (rax) as u64; rdx = (((crate::simd_ops::ru32(rcx.wrapping_add(0x8u64))) as i32 as i64) as u64) as u64; fl_l = ((r10).wrapping_add(rdx)) as u64 as u64; fl_r = fl_l; r10 = ((fl_l as u64)) as u64; if (((fl_l as u64)) == 0) { bb = 33; continue; } bb = 32; continue; }
            32 => { rcx = ((crate::simd_ops::ru8(rsp.wrapping_add(0xd8u64))) as u32) as u32 as u64; r9 = (rbp) as u64; rax = (crate::simd_ops::ru32(rsp.wrapping_add(0xc8u64))) as u32 as u64; r8 = (r15) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x38u64), ((rcx as u32)) as u32); rdx = (r14) as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0xd0u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (rcx) as u64); rcx = (rbx) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((rax as u32)) as u32); rax = (r10) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rdi) as u64); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P10)) as u64; if t == 0x7001ec87f0 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 35; continue; }
            33 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xd0u64))) as u64; r9 = (rbp) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), (rax) as u64); r8 = (r15) as u64; rax = (crate::simd_ops::ru32(rsp.wrapping_add(0xc8u64))) as u32 as u64; rdx = (r14) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((rax as u32)) as u32); rcx = (rbx) as u64; rax = (rax & !0xffu64) | (((crate::simd_ops::ru8(rsp.wrapping_add(0xd8u64))) as u8 as u64)); crate::simd_ops::wu8(rsp.wrapping_add(0x28u64), ((rax as u8)) as u8); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rdi) as u64); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P11) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 34; continue; }
            34 => { rax = (0x1u32) as u32 as u64; bb = 35; continue; }
            35 => { r11 = ((rsp.wrapping_add(0x80u64)) as u64) as u64; rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x20u64))) as u64; rbp = (crate::simd_ops::ru64(r11.wrapping_add(0x28u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x30u64))) as u64; rsp = (r11) as u64; r15 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001ec6354(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001ec60c8; const P1: u64 = 0x7001ec7448;
    let (mut rax, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&super::EH_META[3], rsp); let _eh = eh.push();
    rsp = rsp.wrapping_sub(8);
    core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64);
    rsp = ((rsp).wrapping_sub(0x40u64)) as u64;
    rax = (rax & !0xffu64) | (((crate::simd_ops::ru8(rsp.wrapping_add(0x88u64))) as u8 as u64));
    crate::simd_ops::wu8(rsp.wrapping_add(0x38u64), ((rax as u8)) as u8);
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (rax) as u64);
    rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x78u64))) as u32 as u64;
    crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rax) as u64);
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    eh.site(0x7001ec6384); crate::simd_ops::replace_rax_in(rax); rax = match eh.call(0x7001ec6384, (P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8) { Ok(v) => v, Err(l) => l.unknown(0x7001ec6354) };
    rbx = ((rax as u32)) as u32 as u64;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    eh.site(0x7001ec638b); crate::simd_ops::replace_rax_in(rax); rax = match eh.call(0x7001ec638b, (P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8) { Ok(v) => v, Err(l) => l.unknown(0x7001ec6354) };
    crate::simd_ops::wu32(rax, (0xfffffffeu32) as u32);
    rax = ((rbx as u32)) as u32 as u64;
    rsp = ((rsp).wrapping_add(0x40u64)) as u64;
    rbx = core::ptr::read_unaligned(rsp as *const u64);
    rsp = rsp.wrapping_add(8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001ec63a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0x18; const P3: u64 = 0x1c; const P4: u64 = 0x24; const P5: u64 = 0x30; const P6: u64 = 0x40; const P7: u64 = 0x48; const P8: u64 = 0xc; const P9: u64 = 0x7001ec4000; const P10: u64 = 0x5208; const P11: u64 = 0x5218; const P12: u64 = 0x7001ec684c;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; xmm0 = 0u128; crate::simd_ops::wu64(rcx.wrapping_add(P0), (rax) as u64); rbx = (rcx) as u64; crate::simd_ops::wu64(rcx.wrapping_add(P1), (rax) as u64); crate::simd_ops::wu8(rcx.wrapping_add(P2), ((rax as u8)) as u8); crate::simd_ops::wu64(rcx.wrapping_add(P3), (rax) as u64); crate::simd_ops::wu64(rcx.wrapping_add(P4), (rax) as u64); crate::simd_ops::wu128(rcx.wrapping_add(P5), (xmm0) as u128); crate::simd_ops::wu64(rcx.wrapping_add(P6), (r8) as u64); crate::simd_ops::wu32(rcx.wrapping_add(P7), ((r9 as u32)) as u32); fl_l = (crate::simd_ops::ru32(rdx.wrapping_add(P8))) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rdx = (((crate::simd_ops::ru32(rdx.wrapping_add(P8))) as i32 as i64) as u64) as u64; rdx = ((rdx).wrapping_add(r8)) as u64; r8 = ((P9) as u64) as u64; crate::simd_ops::wu64(rcx.wrapping_add(P0), (rdx) as u64); rcx = ((crate::simd_ops::ru8(rdx)) as u32) as u32 as u64; rcx = ((((rcx as u32)) & (0xfu32))) as u32 as u64; rax = (((crate::simd_ops::ru8(rcx.wrapping_add(r8).wrapping_add(P10))) as i8 as i64) as u64) as u64; rcx = (rcx & !0xffu64) | (((crate::simd_ops::ru8(rcx.wrapping_add(r8).wrapping_add(P11))) as u8 as u64)); rdx = ((rdx).wrapping_sub(rax)) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_sub(0x4u64))) as u32 as u64; rax = (((rax as u32)).wrapping_shr(((rcx as u32) & 31))) as u32 as u64; rcx = (rbx) as u64; crate::simd_ops::wu32(rbx, ((rax as u32)) as u32); crate::simd_ops::wu64(rbx.wrapping_add(0x8u64), (rdx) as u64); crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), (rdx) as u64); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P12) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 3; continue; }
            2 => { crate::simd_ops::wu32(rcx, ((rax as u32)) as u32); bb = 3; continue; }
            3 => { rax = (rbx) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001ec6424(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0x18; const P3: u64 = 0xc; const P4: u64 = 0x7001ec4000; const P5: u64 = 0x5208; const P6: u64 = 0x5218; const P7: u64 = 0xfffffffffffffffc; const P8: u64 = 0x1c; const P9: u64 = 0x20; const P10: u64 = 0x4; const P11: u64 = 0x24;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; xmm0 = 0u128; crate::simd_ops::wu64(rcx.wrapping_add(P0), (rax) as u64); r9 = (rcx) as u64; crate::simd_ops::wu64(rcx.wrapping_add(P1), (rax) as u64); crate::simd_ops::wu128(rcx.wrapping_add(P2), (xmm0) as u128); fl_l = (crate::simd_ops::ru32(rdx.wrapping_add(P3))) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rdx = (((crate::simd_ops::ru32(rdx.wrapping_add(P3))) as i32 as i64) as u64) as u64; rdx = ((rdx).wrapping_add(r8)) as u64; r8 = ((P4) as u64) as u64; crate::simd_ops::wu64(rcx.wrapping_add(P0), (rdx) as u64); rcx = ((crate::simd_ops::ru8(rdx)) as u32) as u32 as u64; rcx = ((((rcx as u32)) & (0xfu32))) as u32 as u64; rax = (((crate::simd_ops::ru8(rcx.wrapping_add(r8).wrapping_add(P5))) as i8 as i64) as u64) as u64; rcx = (rcx & !0xffu64) | (((crate::simd_ops::ru8(rcx.wrapping_add(r8).wrapping_add(P6))) as u8 as u64)); rdx = ((rdx).wrapping_sub(rax)) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(P7))) as u32 as u64; rax = (((rax as u32)).wrapping_shr(((rcx as u32) & 31))) as u32 as u64; crate::simd_ops::wu64(r9.wrapping_add(P0), (rdx) as u64); crate::simd_ops::wu32(r9, ((rax as u32)) as u32); crate::simd_ops::wu64(r9.wrapping_add(P1), (rdx) as u64); rcx = ((crate::simd_ops::ru8(rdx)) as u32) as u32 as u64; rcx = ((((rcx as u32)) & (0xfu32))) as u32 as u64; rax = (((crate::simd_ops::ru8(rcx.wrapping_add(r8).wrapping_add(P5))) as i8 as i64) as u64) as u64; rcx = (rcx & !0xffu64) | (((crate::simd_ops::ru8(rcx.wrapping_add(r8).wrapping_add(P6))) as u8 as u64)); rdx = ((rdx).wrapping_sub(rax)) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(P7))) as u32 as u64; rax = (((rax as u32)).wrapping_shr(((rcx as u32) & 31))) as u32 as u64; crate::simd_ops::wu64(r9.wrapping_add(P0), (rdx) as u64); crate::simd_ops::wu32(r9.wrapping_add(P2), ((rax as u32)) as u32); rcx = ((crate::simd_ops::ru8(rdx)) as u32) as u32 as u64; rcx = ((((rcx as u32)) & (0xfu32))) as u32 as u64; rax = (((crate::simd_ops::ru8(rcx.wrapping_add(r8).wrapping_add(P5))) as i8 as i64) as u64) as u64; rcx = (rcx & !0xffu64) | (((crate::simd_ops::ru8(rcx.wrapping_add(r8).wrapping_add(P6))) as u8 as u64)); rdx = ((rdx).wrapping_sub(rax)) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(P7))) as u32 as u64; rax = (((rax as u32)).wrapping_shr(((rcx as u32) & 31))) as u32 as u64; crate::simd_ops::wu64(r9.wrapping_add(P0), (rdx) as u64); crate::simd_ops::wu32(r9.wrapping_add(P8), ((rax as u32)) as u32); rcx = ((crate::simd_ops::ru8(rdx)) as u32) as u32 as u64; rcx = ((((rcx as u32)) & (0xfu32))) as u32 as u64; rax = (((crate::simd_ops::ru8(rcx.wrapping_add(r8).wrapping_add(P5))) as i8 as i64) as u64) as u64; rcx = (rcx & !0xffu64) | (((crate::simd_ops::ru8(rcx.wrapping_add(r8).wrapping_add(P6))) as u8 as u64)); rdx = ((rdx).wrapping_sub(rax)) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(P7))) as u32 as u64; fl_l = (((rax as u32)).wrapping_shr(((rcx as u32) & 31))) as u32 as u64; rax = ((fl_l as u32)) as u32 as u64; crate::simd_ops::wu32(r9.wrapping_add(P9), ((rax as u32)) as u32); rax = ((rdx.wrapping_add(P10)) as u64) as u64; crate::simd_ops::wu64(r9.wrapping_add(P0), (rdx) as u64); rcx = (crate::simd_ops::ru32(rdx)) as u32 as u64; crate::simd_ops::wu64(r9.wrapping_add(P0), (rax) as u64); crate::simd_ops::wu32(r9.wrapping_add(P11), ((rcx as u32)) as u32); bb = 3; continue; }
            2 => { crate::simd_ops::wu32(rcx, ((rax as u32)) as u32); bb = 3; continue; }
            3 => { rax = (r9) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001ec6508(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001ec9230; const P1: u64 = 0x8; const P2: u64 = 0x7001ec7164; const P3: u64 = 0x7001ec9260;
    let (mut rax, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rsp = rsp.wrapping_sub(8);
    core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = (rcx) as u64;
    rax = (rdx) as u64;
    rcx = ((P0) as u64) as u64;
    xmm0 = 0u128;
    crate::simd_ops::wu64(rbx, (rcx) as u64);
    rdx = ((rbx.wrapping_add(P1)) as u64) as u64;
    rcx = ((rax.wrapping_add(P1)) as u64) as u64;
    crate::simd_ops::wu128(rdx, (xmm0) as u128);
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0;
    rax = ((P3) as u64) as u64;
    crate::simd_ops::wu64(rbx, (rax) as u64);
    rax = (rbx) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = core::ptr::read_unaligned(rsp as *const u64);
    rsp = rsp.wrapping_add(8);
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001ec6544(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x7001ec9270; const P2: u64 = 0x8; const P3: u64 = 0x7001ec9260;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    fl_l = (crate::simd_ops::lk_and_u64(rcx.wrapping_add(P0), (0x0u64) as u64)) as u64; fl_r = fl_l;
    rax = ((P1) as u64) as u64;
    crate::simd_ops::wu64(rcx.wrapping_add(P2), (rax) as u64);
    rax = ((P3) as u64) as u64;
    crate::simd_ops::wu64(rcx, (rax) as u64);
    rax = (rcx) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001ec6564(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001ec9230; const P1: u64 = 0x8; const P2: u64 = 0x7001ec7164;
    let (mut rax, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rsp = rsp.wrapping_sub(8);
    core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = (rcx) as u64;
    rax = (rdx) as u64;
    rcx = ((P0) as u64) as u64;
    xmm0 = 0u128;
    crate::simd_ops::wu64(rbx, (rcx) as u64);
    rdx = ((rbx.wrapping_add(P1)) as u64) as u64;
    rcx = ((rax.wrapping_add(P1)) as u64) as u64;
    crate::simd_ops::wu128(rdx, (xmm0) as u128);
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0;
    rax = (rbx) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = core::ptr::read_unaligned(rsp as *const u64);
    rsp = rsp.wrapping_add(8);
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001ec65b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001ec9230; const P1: u64 = 0x7001ec71f4; const P2: u64 = 0x7001ec87a5;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rbx) as u64);
    rsp = rsp.wrapping_sub(8);
    core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rax = ((P0) as u64) as u64;
    rdi = (rcx) as u64;
    crate::simd_ops::wu64(rcx, (rax) as u64);
    rbx = ((rdx as u32)) as u32 as u64;
    rcx = ((rcx).wrapping_add(0x8u64)) as u64;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    fl_l = ((rbx as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64;
    if !(((((fl_l as u8)) & ((fl_r as u8))) == 0)) { rdx = (0x18u32) as u32 as u64; rcx = (rdi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); }
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rax = (rdi) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = core::ptr::read_unaligned(rsp as *const u64);
    rsp = rsp.wrapping_add(8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001ec6600(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001ec7524; const P1: u64 = 0x7001ec5154; const P2: u64 = 0x7001ec7ff0; const P3: u64 = 0x7001ec5060; const P4: u64 = 0x7001ec9138; const P5: u64 = 0x7001ec5090; const P6: u64 = 0x7001ec7448; const P7: u64 = 0x7001ec51a0; const P8: u64 = 0x7001ec90b0; const P9: u64 = 0x7001ec802c; const P10: u64 = 0x7001ec5214;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi, mut r12, mut r13, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 27>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&super::EH_META[4], rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = (rsp) as u64; rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r12) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r13) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r15) as u64); rsp = ((rsp).wrapping_sub(0xa8u64)) as u64; rdi = (rcx) as u64; r12 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((r12 as u32)) as u32); crate::simd_ops::lk_and_u32(rsp.wrapping_add(0xf0u64), ((r12 as u32)) as u32); crate::simd_ops::lk_and_u64(rsp.wrapping_add(0x28u64), (r12) as u64); crate::simd_ops::lk_and_u64(rsp.wrapping_add(0x40u64), (r12) as u64); crate::simd_ops::wu8(rax.wrapping_sub(0x80u64), ((r12 as u8)) as u8); crate::simd_ops::lk_and_u32(rax.wrapping_sub(0x7cu64), ((r12 as u32)) as u32); crate::simd_ops::lk_and_u32(rax.wrapping_sub(0x78u64), ((r12 as u32)) as u32); crate::simd_ops::lk_and_u32(rax.wrapping_sub(0x74u64), ((r12 as u32)) as u32); crate::simd_ops::lk_and_u32(rax.wrapping_sub(0x70u64), ((r12 as u32)) as u32); crate::simd_ops::lk_and_u32(rax.wrapping_sub(0x6cu64), ((r12 as u32)) as u32); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001ec6648); crate::simd_ops::replace_rax_in(rax); rax = match eh.call(0x7001ec6648, (P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8) { Ok(v) => v, Err(l) => match l.cont_va { 0x7001ec6751 => { rax = crate::dll_native::lift::eh::landed_rax(); bb = 7; continue; } _ => l.unknown(0x7001ec6600) } }; rax = (crate::simd_ops::ru64(rax.wrapping_add(0x28u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), (rax) as u64); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001ec6656); crate::simd_ops::replace_rax_in(rax); rax = match eh.call(0x7001ec6656, (P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8) { Ok(v) => v, Err(l) => match l.cont_va { 0x7001ec6751 => { rax = crate::dll_native::lift::eh::landed_rax(); bb = 7; continue; } _ => l.unknown(0x7001ec6600) } }; rax = (crate::simd_ops::ru64(rax.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (rax) as u64); rsi = (crate::simd_ops::ru64(rdi.wrapping_add(0x50u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xf8u64), (rsi) as u64); rbx = (crate::simd_ops::ru64(rdi.wrapping_add(0x40u64))) as u64; rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), (rax) as u64); r15 = (crate::simd_ops::ru64(rdi.wrapping_add(0x28u64))) as u64; rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x48u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), (rax) as u64); rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x68u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x78u64), (rax) as u64); rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x78u64))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0xe8u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x38u64))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0xe0u64), ((rax as u32)) as u32); rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001ec66aa); crate::simd_ops::replace_rax_in(rax); rax = match eh.call(0x7001ec66aa, (P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8) { Ok(v) => v, Err(l) => match l.cont_va { 0x7001ec6751 => { rax = crate::dll_native::lift::eh::landed_rax(); bb = 7; continue; } _ => l.unknown(0x7001ec6600) } }; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001ec66af); crate::simd_ops::replace_rax_in(rax); rax = match eh.call(0x7001ec66af, (P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8) { Ok(v) => v, Err(l) => match l.cont_va { 0x7001ec6751 => { rax = crate::dll_native::lift::eh::landed_rax(); bb = 7; continue; } _ => l.unknown(0x7001ec6600) } }; crate::simd_ops::wu64(rax.wrapping_add(0x20u64), (rsi) as u64); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001ec66b8); crate::simd_ops::replace_rax_in(rax); rax = match eh.call(0x7001ec66b8, (P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8) { Ok(v) => v, Err(l) => match l.cont_va { 0x7001ec6751 => { rax = crate::dll_native::lift::eh::landed_rax(); bb = 7; continue; } _ => l.unknown(0x7001ec6600) } }; crate::simd_ops::wu64(rax.wrapping_add(0x28u64), (rbx) as u64); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001ec66c1); crate::simd_ops::replace_rax_in(rax); rax = match eh.call(0x7001ec66c1, (P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8) { Ok(v) => v, Err(l) => match l.cont_va { 0x7001ec6751 => { rax = crate::dll_native::lift::eh::landed_rax(); bb = 7; continue; } _ => l.unknown(0x7001ec6600) } }; rdx = (crate::simd_ops::ru64(rax.wrapping_add(0x20u64))) as u64; rdx = (crate::simd_ops::ru64(rdx.wrapping_add(0x28u64))) as u64; rcx = ((rsp.wrapping_add(0x88u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001ec66d6); crate::simd_ops::replace_rax_in(rax); rax = match eh.call(0x7001ec66d6, (P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8) { Ok(v) => v, Err(l) => match l.cont_va { 0x7001ec6751 => { rax = crate::dll_native::lift::eh::landed_rax(); bb = 7; continue; } _ => l.unknown(0x7001ec6600) } }; r13 = (rax) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), (rax) as u64); fl_l = (crate::simd_ops::ru64(rdi.wrapping_add(0x58u64))) as u64 as u64; fl_r = (r12) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu32(rsp.wrapping_add(0xf0u64), (0x1u32) as u32); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001ec66f4); crate::simd_ops::replace_rax_in(rax); rax = match eh.call(0x7001ec66f4, (P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8) { Ok(v) => v, Err(l) => match l.cont_va { 0x7001ec6751 => { rax = crate::dll_native::lift::eh::landed_rax(); bb = 7; continue; } _ => l.unknown(0x7001ec6600) } }; rcx = (crate::simd_ops::ru64(rax.wrapping_add(0x70u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), (rcx) as u64); bb = 2; continue; }
            2 => { r8 = (0x100u32) as u32 as u64; rdx = (r15) as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001ec6710); crate::simd_ops::replace_rax_in(rax); rax = match eh.call(0x7001ec6710, (P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8) { Ok(v) => v, Err(l) => match l.cont_va { 0x7001ec6751 => { rax = crate::dll_native::lift::eh::landed_rax(); bb = 7; continue; } _ => l.unknown(0x7001ec6600) } }; rbx = (rax) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (rax) as u64); fl_l = (rax) as u64 as u64; fl_r = (0x2u64) as u64 as u64; if ((((fl_l as u64)) as i64) >= (((fl_r as u64)) as i64)) { bb = 6; continue; } bb = 3; continue; }
            3 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(rax.wrapping_mul(8)).wrapping_add(0x70u64))) as u64; fl_l = (rbx) as u64 as u64; fl_r = (rbx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (rbx) as u64); bb = 6; continue; }
            5 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001ec6734); crate::simd_ops::replace_rax_in(rax); rax = match eh.call(0x7001ec6734, ({ let t = (crate::simd_ops::ru64(P4)) as u64; if t == 0x7001ec87f0 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8) { Ok(v) => v, Err(l) => match l.cont_va { 0x7001ec6751 => { rax = crate::dll_native::lift::eh::landed_rax(); bb = 7; continue; } _ => l.unknown(0x7001ec6600) } }; bb = 6; continue; }
            6 => { rdx = (r15) as u64; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001ec6740); crate::simd_ops::replace_rax_in(rax); rax = match eh.call(0x7001ec6740, (P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8) { Ok(v) => v, Err(l) => match l.cont_va { 0x7001ec6751 => { rax = crate::dll_native::lift::eh::landed_rax(); bb = 7; continue; } _ => l.unknown(0x7001ec6600) } }; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; bb = 11; continue; }
            7 => { crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), (0x1u32) as u32); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001ec6759); crate::simd_ops::replace_rax_in(rax); rax = match eh.call(0x7001ec6759, (P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8) { Ok(v) => v, Err(l) => match l.cont_va { 0x7001ec6751 => { rax = crate::dll_native::lift::eh::landed_rax(); bb = 7; continue; } _ => l.unknown(0x7001ec6600) } }; crate::simd_ops::lk_and_u32(rax.wrapping_add(0x40u64), (0x0u32) as u32); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001ec6762); crate::simd_ops::replace_rax_in(rax); rax = match eh.call(0x7001ec6762, (P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8) { Ok(v) => v, Err(l) => match l.cont_va { 0x7001ec6751 => { rax = crate::dll_native::lift::eh::landed_rax(); bb = 7; continue; } _ => l.unknown(0x7001ec6600) } }; rcx = (crate::simd_ops::ru32(rsp.wrapping_add(0xe8u64))) as u32 as u64; crate::simd_ops::wu32(rax, ((rcx as u32)) as u32); rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0xf8u64))) as u64; fl_l = (crate::simd_ops::ru32(rsp.wrapping_add(0xf0u64))) as u32 as u64; fl_r = (0x0u32) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 9; continue; } bb = 8; continue; }
            8 => { rdx = (rdx & !0xffu64) | (((0x1u8) as u8 as u64)); rcx = (rsi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001ec6787); crate::simd_ops::replace_rax_in(rax); rax = match eh.call(0x7001ec6787, (P7) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8) { Ok(v) => v, Err(l) => match l.cont_va { 0x7001ec6751 => { rax = crate::dll_native::lift::eh::landed_rax(); bb = 7; continue; } _ => l.unknown(0x7001ec6600) } }; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; r9 = ((rcx.wrapping_add(0x20u64)) as u64) as u64; r8 = (crate::simd_ops::ru32(rcx.wrapping_add(0x18u64))) as u32 as u64; rdx = (crate::simd_ops::ru32(rcx.wrapping_add(0x4u64))) as u32 as u64; rcx = (crate::simd_ops::ru32(rcx)) as u32 as u64; bb = 10; continue; }
            9 => { r9 = ((rsi.wrapping_add(0x20u64)) as u64) as u64; r8 = (crate::simd_ops::ru32(rsi.wrapping_add(0x18u64))) as u32 as u64; rdx = (crate::simd_ops::ru32(rsi.wrapping_add(0x4u64))) as u32 as u64; rcx = (crate::simd_ops::ru32(rsi)) as u32 as u64; bb = 10; continue; }
            10 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001ec67ad); crate::simd_ops::replace_rax_in(rax); rax = match eh.call(0x7001ec67ad, ({ let t = (crate::simd_ops::ru64(P8)) as u64; if t == 0x7001ec87f0 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8) { Ok(v) => v, Err(l) => match l.cont_va { 0x7001ec6751 => { rax = crate::dll_native::lift::eh::landed_rax(); bb = 7; continue; } _ => l.unknown(0x7001ec6600) } }; r12 = (crate::simd_ops::ru32(rsp.wrapping_add(0x20u64))) as u32 as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; r13 = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; bb = 11; continue; }
            11 => { rcx = (r13) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001ec67cf); crate::simd_ops::replace_rax_in(rax); rax = match eh.call(0x7001ec67cf, (P9) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8) { Ok(v) => v, Err(l) => match l.cont_va { 0x7001ec6751 => { rax = crate::dll_native::lift::eh::landed_rax(); bb = 7; continue; } _ => l.unknown(0x7001ec6600) } }; fl_l = ((r12 as u32)) as u32 as u64; fl_r = ((r12 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 17; continue; } bb = 12; continue; }
            12 => { fl_l = (crate::simd_ops::ru32(rsi)) as u32 as u64; fl_r = (0xe06d7363u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 17; continue; } bb = 13; continue; }
            13 => { fl_l = (crate::simd_ops::ru32(rsi.wrapping_add(0x18u64))) as u32 as u64; fl_r = (0x4u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 17; continue; } bb = 14; continue; }
            14 => { rax = (crate::simd_ops::ru32(rsi.wrapping_add(0x20u64))) as u32 as u64; rax = (((rax as u32)).wrapping_sub(0x19930520u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x2u32) as u32 as u64; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 17; continue; } bb = 15; continue; }
            15 => { rcx = (crate::simd_ops::ru64(rsi.wrapping_add(0x28u64))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001ec67f8); crate::simd_ops::replace_rax_in(rax); rax = match eh.call(0x7001ec67f8, (P10) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8) { Ok(v) => v, Err(l) => match l.cont_va { 0x7001ec6751 => { rax = crate::dll_native::lift::eh::landed_rax(); bb = 7; continue; } _ => l.unknown(0x7001ec6600) } }; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 17; continue; } bb = 16; continue; }
            16 => { rdx = (rdx & !0xffu64) | (((0x1u8) as u8 as u64)); rcx = (rsi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001ec6806); crate::simd_ops::replace_rax_in(rax); rax = match eh.call(0x7001ec6806, (P7) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8) { Ok(v) => v, Err(l) => match l.cont_va { 0x7001ec6751 => { rax = crate::dll_native::lift::eh::landed_rax(); bb = 7; continue; } _ => l.unknown(0x7001ec6600) } }; bb = 17; continue; }
            17 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001ec680b); crate::simd_ops::replace_rax_in(rax); rax = match eh.call(0x7001ec680b, (P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8) { Ok(v) => v, Err(l) => match l.cont_va { 0x7001ec6751 => { rax = crate::dll_native::lift::eh::landed_rax(); bb = 7; continue; } _ => l.unknown(0x7001ec6600) } }; crate::simd_ops::wu64(rax.wrapping_add(0x20u64), (r15) as u64); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001ec6814); crate::simd_ops::replace_rax_in(rax); rax = match eh.call(0x7001ec6814, (P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8) { Ok(v) => v, Err(l) => match l.cont_va { 0x7001ec6751 => { rax = crate::dll_native::lift::eh::landed_rax(); bb = 7; continue; } _ => l.unknown(0x7001ec6600) } }; crate::simd_ops::wu64(rax.wrapping_add(0x28u64), (rdi) as u64); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001ec681d); crate::simd_ops::replace_rax_in(rax); rax = match eh.call(0x7001ec681d, (P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8) { Ok(v) => v, Err(l) => match l.cont_va { 0x7001ec6751 => { rax = crate::dll_native::lift::eh::landed_rax(); bb = 7; continue; } _ => l.unknown(0x7001ec6600) } }; rcx = (crate::simd_ops::ru32(rsp.wrapping_add(0xe0u64))) as u32 as u64; crate::simd_ops::wu32(rax, ((rcx as u32)) as u32); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001ec682b); crate::simd_ops::replace_rax_in(rax); rax = match eh.call(0x7001ec682b, (P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8) { Ok(v) => v, Err(l) => match l.cont_va { 0x7001ec6751 => { rax = crate::dll_native::lift::eh::landed_rax(); bb = 7; continue; } _ => l.unknown(0x7001ec6600) } }; crate::simd_ops::wu32(rax, (0xfffffffeu32) as u32); rax = (rbx) as u64; rsp = ((rsp).wrapping_add(0xa8u64)) as u64; r15 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r13 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r12 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rsi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rbx = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001ec684c(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x7001ec4000;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; r11 = ((P0) as u64) as u64; crate::simd_ops::wu8(rcx.wrapping_add(0x18u64), ((rax as u8)) as u8); xmm0 = 0u128; crate::simd_ops::wu64(rcx.wrapping_add(0x1cu64), (rax) as u64); r8 = (rcx) as u64; crate::simd_ops::wu64(rcx.wrapping_add(0x24u64), (rax) as u64); crate::simd_ops::wu128(rcx.wrapping_add(0x30u64), (xmm0) as u128); rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; r9 = (r9 & !0xffu64) | (((crate::simd_ops::ru8(rax)) as u8 as u64)); rdx = ((rax.wrapping_add(0x1u64)) as u64) as u64; crate::simd_ops::wu8(rcx.wrapping_add(0x18u64), ((r9 as u8)) as u8); crate::simd_ops::wu64(rcx.wrapping_add(0x8u64), (rdx) as u64); fl_l = ((r9 as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rcx = ((crate::simd_ops::ru8(rdx)) as u32) as u32 as u64; rcx = ((((rcx as u32)) & (0xfu32))) as u32 as u64; rax = (((crate::simd_ops::ru8(rcx.wrapping_add(r11).wrapping_add(0x5208u64))) as i8 as i64) as u64) as u64; rcx = (rcx & !0xffu64) | (((crate::simd_ops::ru8(rcx.wrapping_add(r11).wrapping_add(0x5218u64))) as u8 as u64)); rdx = ((rdx).wrapping_sub(rax)) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_sub(0x4u64))) as u32 as u64; fl_l = (((rax as u32)).wrapping_shr(((rcx as u32) & 31))) as u32 as u64; rax = ((fl_l as u32)) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(0x1cu64), ((rax as u32)) as u32); crate::simd_ops::wu64(r8.wrapping_add(0x8u64), (rdx) as u64); bb = 2; continue; }
            2 => { fl_l = ((r9 as u8)) as u8 as u64; fl_r = (0x2u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru32(rdx)) as u32 as u64; fl_l = ((rdx).wrapping_add(0x4u64)) as u64 as u64; fl_r = fl_l; rdx = ((fl_l as u64)) as u64; crate::simd_ops::wu64(r8.wrapping_add(0x8u64), (rdx) as u64); crate::simd_ops::wu32(r8.wrapping_add(0x20u64), ((rax as u32)) as u32); bb = 4; continue; }
            4 => { fl_l = ((r9 as u8)) as u8 as u64; fl_r = (0x4u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { rcx = ((crate::simd_ops::ru8(rdx)) as u32) as u32 as u64; rcx = ((((rcx as u32)) & (0xfu32))) as u32 as u64; rax = (((crate::simd_ops::ru8(rcx.wrapping_add(r11).wrapping_add(0x5208u64))) as i8 as i64) as u64) as u64; rcx = (rcx & !0xffu64) | (((crate::simd_ops::ru8(rcx.wrapping_add(r11).wrapping_add(0x5218u64))) as u8 as u64)); rdx = ((rdx).wrapping_sub(rax)) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_sub(0x4u64))) as u32 as u64; fl_l = (((rax as u32)).wrapping_shr(((rcx as u32) & 31))) as u32 as u64; rax = ((fl_l as u32)) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(0x24u64), ((rax as u32)) as u32); crate::simd_ops::wu64(r8.wrapping_add(0x8u64), (rdx) as u64); bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru32(rdx)) as u32 as u64; r10 = ((rdx.wrapping_add(0x4u64)) as u64) as u64; crate::simd_ops::wu32(r8.wrapping_add(0x28u64), ((rax as u32)) as u32); rcx = (rcx & !0xffu64) | (((0x30u8) as u8 as u64)); rax = (rax & !0xffu64) | ((((r9 as u8)) as u8 as u64)); crate::simd_ops::wu64(r8.wrapping_add(0x8u64), (r10) as u64); rax = (rax & !0xffu64) | ((((((rax as u8)) & ((rcx as u8)))) as u8 as u64)); fl_l = ((r9 as u8)) as u8 as u64; fl_r = (0x8u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 11; continue; } bb = 7; continue; }
            7 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x10u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 9; continue; } bb = 8; continue; }
            8 => { rcx = (((crate::simd_ops::ru32(r10)) as i32 as i64) as u64) as u64; rax = ((r10.wrapping_add(0x4u64)) as u64) as u64; crate::simd_ops::wu64(r8.wrapping_add(0x8u64), (rax) as u64); crate::simd_ops::wu64(r8.wrapping_add(0x30u64), (rcx) as u64); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            9 => { r9 = (r9 & !0xffu64) | ((((((r9 as u8)) & ((rcx as u8)))) as u8 as u64)); fl_l = ((r9 as u8)) as u8 as u64; fl_r = (0x20u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 16; continue; } bb = 10; continue; }
            10 => { rax = (((crate::simd_ops::ru32(r10)) as i32 as i64) as u64) as u64; rdx = ((r10.wrapping_add(0x4u64)) as u64) as u64; crate::simd_ops::wu64(r8.wrapping_add(0x8u64), (rdx) as u64); crate::simd_ops::wu64(r8.wrapping_add(0x30u64), (rax) as u64); rax = ((rdx.wrapping_add(0x4u64)) as u64) as u64; rcx = (((crate::simd_ops::ru32(rdx)) as i32 as i64) as u64) as u64; crate::simd_ops::wu64(r8.wrapping_add(0x8u64), (rax) as u64); bb = 15; continue; }
            11 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x10u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 13; continue; } bb = 12; continue; }
            12 => { rcx = ((crate::simd_ops::ru8(r10)) as u32) as u32 as u64; rcx = ((((rcx as u32)) & (0xfu32))) as u32 as u64; rax = (((crate::simd_ops::ru8(rcx.wrapping_add(r11).wrapping_add(0x5208u64))) as i8 as i64) as u64) as u64; rcx = (rcx & !0xffu64) | (((crate::simd_ops::ru8(rcx.wrapping_add(r11).wrapping_add(0x5218u64))) as u8 as u64)); r10 = ((r10).wrapping_sub(rax)) as u64; rax = (crate::simd_ops::ru32(r8.wrapping_add(0x48u64))) as u32 as u64; rdx = (crate::simd_ops::ru32(r10.wrapping_sub(0x4u64))) as u32 as u64; rdx = (((rdx as u32)).wrapping_shr(((rcx as u32) & 31))) as u32 as u64; fl_l = (((rax as u32)).wrapping_add((rdx as u32))) as u32 as u64; fl_r = fl_l; rax = ((fl_l as u32)) as u32 as u64; crate::simd_ops::wu64(r8.wrapping_add(0x8u64), (r10) as u64); crate::simd_ops::wu64(r8.wrapping_add(0x30u64), (rax) as u64); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            13 => { r9 = (r9 & !0xffu64) | ((((((r9 as u8)) & ((rcx as u8)))) as u8 as u64)); fl_l = ((r9 as u8)) as u8 as u64; fl_r = (0x20u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 16; continue; } bb = 14; continue; }
            14 => { rcx = ((crate::simd_ops::ru8(r10)) as u32) as u32 as u64; rdx = (crate::simd_ops::ru32(r8.wrapping_add(0x48u64))) as u32 as u64; rcx = ((((rcx as u32)) & (0xfu32))) as u32 as u64; rax = (((crate::simd_ops::ru8(rcx.wrapping_add(r11).wrapping_add(0x5208u64))) as i8 as i64) as u64) as u64; rcx = (rcx & !0xffu64) | (((crate::simd_ops::ru8(rcx.wrapping_add(r11).wrapping_add(0x5218u64))) as u8 as u64)); r10 = ((r10).wrapping_sub(rax)) as u64; rax = (crate::simd_ops::ru32(r10.wrapping_sub(0x4u64))) as u32 as u64; rax = (((rax as u32)).wrapping_shr(((rcx as u32) & 31))) as u32 as u64; crate::simd_ops::wu64(r8.wrapping_add(0x8u64), (r10) as u64); rcx = ((rdx.wrapping_add(rax)) as u32) as u32 as u64; crate::simd_ops::wu64(r8.wrapping_add(0x30u64), (rcx) as u64); rcx = ((crate::simd_ops::ru8(r10)) as u32) as u32 as u64; rcx = ((((rcx as u32)) & (0xfu32))) as u32 as u64; rax = (((crate::simd_ops::ru8(rcx.wrapping_add(r11).wrapping_add(0x5208u64))) as i8 as i64) as u64) as u64; rcx = (rcx & !0xffu64) | (((crate::simd_ops::ru8(rcx.wrapping_add(r11).wrapping_add(0x5218u64))) as u8 as u64)); r10 = ((r10).wrapping_sub(rax)) as u64; rax = (crate::simd_ops::ru32(r10.wrapping_sub(0x4u64))) as u32 as u64; fl_l = (((rax as u32)).wrapping_shr(((rcx as u32) & 31))) as u32 as u64; rax = ((fl_l as u32)) as u32 as u64; crate::simd_ops::wu64(r8.wrapping_add(0x8u64), (r10) as u64); rcx = ((rdx.wrapping_add(rax)) as u32) as u32 as u64; bb = 15; continue; }
            15 => { crate::simd_ops::wu64(r8.wrapping_add(0x38u64), (rcx) as u64); bb = 16; continue; }
            16 => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001ec69e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x18; const P1: u64 = 0x20; const P2: u64 = 0x28; const P3: u64 = 0x30; const P4: u64 = 0x7001ec7524; const P5: u64 = 0x40;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; r9 = (crate::simd_ops::ru64(rcx)) as u64; rbx = (r8) as u64; crate::simd_ops::lk_and_u32(r8, (0x0u32) as u32); rcx = (0xe06d7363u32) as u32 as u64; fl_l = (crate::simd_ops::ru32(r9)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 10; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru32(r9.wrapping_add(P0))) as u32 as u64; fl_r = (0x4u32) as u32 as u64; r8 = (0x19930520u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 5; continue; } bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru32(r9.wrapping_add(P1))) as u32 as u64; rax = (((rax as u32)).wrapping_sub((r8 as u32))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x2u32) as u32 as u64; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 5; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rdx.wrapping_add(P2))) as u64; fl_l = (crate::simd_ops::ru64(r9.wrapping_add(P2))) as u64 as u64; fl_r = (rax) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 5; continue; } bb = 4; continue; }
            4 => { crate::simd_ops::wu32(rbx, (0x1u32) as u32); bb = 5; continue; }
            5 => { fl_l = (crate::simd_ops::ru32(r9)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 10; continue; } bb = 6; continue; }
            6 => { fl_l = (crate::simd_ops::ru32(r9.wrapping_add(P0))) as u32 as u64; fl_r = (0x4u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 10; continue; } bb = 7; continue; }
            7 => { rcx = (crate::simd_ops::ru32(r9.wrapping_add(P1))) as u32 as u64; rcx = (((rcx as u32)).wrapping_sub((r8 as u32))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = (0x2u32) as u32 as u64; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 10; continue; } bb = 8; continue; }
            8 => { fl_l = (crate::simd_ops::ru64(r9.wrapping_add(P3))) as u64 as u64; fl_r = (0x0u64) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 10; continue; } bb = 9; continue; }
            9 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); crate::simd_ops::wu32(rax.wrapping_add(P5), (0x1u32) as u32); rax = (0x1u32) as u32 as u64; crate::simd_ops::wu32(rbx, (0x1u32) as u32); bb = 11; continue; }
            10 => { rax = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; bb = 11; continue; }
            11 => { rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001ec6a64(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001ec69e0; const P1: u64 = 0x7001ec7448;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rbx) as u64);
    rsp = rsp.wrapping_sub(8);
    core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rdi = ((r8 as u32)) as u32 as u64;
    r8 = (r9) as u64;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    rbx = ((rax as u32)) as u32 as u64;
    fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64;
    if !(((((fl_l as u32)) & ((fl_r as u32))) != 0)) { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); crate::simd_ops::wu32(rax, ((rdi as u32)) as u32); }
    rax = ((rbx as u32)) as u32 as u64;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = core::ptr::read_unaligned(rsp as *const u64);
    rsp = rsp.wrapping_add(8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001ec6a94(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001ecc040; const P1: u64 = 0x7001ec8080; const P2: u64 = 0x7001ec78c0; const P3: u64 = 0x7001ec7448; const P4: u64 = 0x7001ec9138; const P5: u64 = 0x7001ec6b18; const P6: u64 = 0x7001ec7524; const P7: u64 = 0x7001ec4000; const P8: u64 = 0x7001ec6fc4; const P9: u64 = 0x7001ec6f0c; const P10: u64 = 0x7001ec7094; const P11: u64 = 0x7001ec5010; const P12: u64 = 0x7001ec50c0; const P13: u64 = 0x7001ec80a8;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut xmm6: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 39>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&super::EH_META[5], rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = (rsp) as u64; rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r12) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r13) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r15) as u64); rsp = ((rsp).wrapping_sub(0x100u64)) as u64; crate::simd_ops::wu128(rax.wrapping_sub(0x48u64), (xmm6) as u128); rax = (crate::simd_ops::ru64(P0)) as u64; rax = (((rax) ^ (rsp))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xe0u64), (rax) as u64); r13 = ((r9 as u32)) as u32 as u64; rbx = (r8) as u64; rsi = (rdx) as u64; r12 = (rcx) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), (rcx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), (rcx) as u64); crate::simd_ops::wu32(rsp.wrapping_add(0x48u64), ((r9 as u32)) as u32); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001ec6ada); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = match eh.call_xmm(0x7001ec6ada, (P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]) { Ok(v) => v, Err(l) => match l.cont_va { 0x7001ec6d36 => { rax = crate::dll_native::lift::eh::landed_rax(); bb = 20; continue; } _ => l.unknown(0x7001ec6a94) } }; rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), (rax) as u64); rdx = (rsi) as u64; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001ec6aea); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = match eh.call_xmm(0x7001ec6aea, (P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]) { Ok(v) => v, Err(l) => match l.cont_va { 0x7001ec6d36 => { rax = crate::dll_native::lift::eh::landed_rax(); bb = 20; continue; } _ => l.unknown(0x7001ec6a94) } }; rax = c_rax; xmm0 = c_x0; rdi = ((rax as u32)) as u32 as u64; r14 = ((rsi.wrapping_add(0x48u64)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x78u64), (r14) as u64); fl_l = (crate::simd_ops::ru32(r14)) as u32 as u64; fl_r = (0x0u32) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 4; continue; } bb = 1; continue; }
            1 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001ec6b00); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = match eh.call_xmm(0x7001ec6b00, (P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]) { Ok(v) => v, Err(l) => match l.cont_va { 0x7001ec6d36 => { rax = crate::dll_native::lift::eh::landed_rax(); bb = 20; continue; } _ => l.unknown(0x7001ec6a94) } }; rax = c_rax; xmm0 = c_x0; fl_l = (crate::simd_ops::ru32(rax)) as u32 as u64; fl_r = (0xfffffffeu32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 3; continue; } bb = 2; continue; }
            2 => { rdi = (crate::simd_ops::ru32(r14)) as u32 as u64; fl_l = ((rdi as u32)) as u32 as u64; fl_r = (0x2u32) as u32 as u64; rdi = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 6; continue; }
            3 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001ec6b12); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = match eh.call_xmm(0x7001ec6b12, ({ let t = (crate::simd_ops::ru64(P4)) as u64; if t == 0x7001ec87f0 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]) { Ok(v) => v, Err(l) => match l.cont_va { 0x7001ec6d36 => { rax = crate::dll_native::lift::eh::landed_rax(); bb = 20; continue; } _ => l.unknown(0x7001ec6a94) } }; rax = c_rax; xmm0 = c_x0; crate::lift_ops::trap(0x80000003, P5, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            4 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001ec6b19); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = match eh.call_xmm(0x7001ec6b19, (P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]) { Ok(v) => v, Err(l) => match l.cont_va { 0x7001ec6d36 => { rax = crate::dll_native::lift::eh::landed_rax(); bb = 20; continue; } _ => l.unknown(0x7001ec6a94) } }; rax = c_rax; xmm0 = c_x0; fl_l = (crate::simd_ops::ru32(rax)) as u32 as u64; fl_r = (0xfffffffeu32) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 6; continue; } bb = 5; continue; }
            5 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001ec6b23); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = match eh.call_xmm(0x7001ec6b23, (P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]) { Ok(v) => v, Err(l) => match l.cont_va { 0x7001ec6d36 => { rax = crate::dll_native::lift::eh::landed_rax(); bb = 20; continue; } _ => l.unknown(0x7001ec6a94) } }; rax = c_rax; xmm0 = c_x0; rdi = (crate::simd_ops::ru32(rax)) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001ec6b2a); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = match eh.call_xmm(0x7001ec6b2a, (P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]) { Ok(v) => v, Err(l) => match l.cont_va { 0x7001ec6d36 => { rax = crate::dll_native::lift::eh::landed_rax(); bb = 20; continue; } _ => l.unknown(0x7001ec6a94) } }; rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu32(rax, (0xfffffffeu32) as u32); bb = 6; continue; }
            6 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001ec6b35); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = match eh.call_xmm(0x7001ec6b35, (P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]) { Ok(v) => v, Err(l) => match l.cont_va { 0x7001ec6d36 => { rax = crate::dll_native::lift::eh::landed_rax(); bb = 20; continue; } _ => l.unknown(0x7001ec6a94) } }; rax = c_rax; xmm0 = c_x0; crate::simd_ops::lk_add_u32(rax.wrapping_add(0x30u64), (1) as u32); rsi = ((rsi).wrapping_add(0x8u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x80u64), (rsi) as u64); rdx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0xc8u64), (rdx) as u64); xmm0 = 0u128; crate::simd_ops::wu128(rsp.wrapping_add(0xd0u64), (xmm0) as u128); fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x8u64))) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 8; continue; } bb = 7; continue; }
            7 => { rdx = (((crate::simd_ops::ru32(rbx.wrapping_add(0x8u64))) as i32 as i64) as u64) as u64; rdx = ((rdx).wrapping_add(crate::simd_ops::ru64(rsi))) as u64; rcx = ((crate::simd_ops::ru8(rdx)) as u32) as u32 as u64; rcx = ((((rcx as u32)) & (0xfu32))) as u32 as u64; r8 = ((P7) as u64) as u64; rax = (((crate::simd_ops::ru8(rcx.wrapping_add(r8).wrapping_add(0x5208u64))) as i8 as i64) as u64) as u64; rcx = ((crate::simd_ops::ru8(rcx.wrapping_add(r8).wrapping_add(0x5218u64))) as u32) as u32 as u64; rdx = ((rdx).wrapping_sub(rax)) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_sub(0x4u64))) as u32 as u64; fl_l = (((rax as u32)).wrapping_shr(((rcx as u32) & 31))) as u32 as u64; rax = ((fl_l as u32)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0xc0u64), ((rax as u32)) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0xc8u64), (rdx) as u64); bb = 9; continue; }
            8 => { fl_l = (crate::simd_ops::lk_and_u32(rsp.wrapping_add(0xc0u64), ((rdx as u32)) as u32)) as u32 as u64; fl_r = fl_l; bb = 9; continue; }
            9 => { rax = ((rsp.wrapping_add(0xc0u64)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (rax) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), (rdx) as u64); rax = ((rsp.wrapping_add(0xc0u64)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), (rax) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), (rdx) as u64); rax = ((rsp.wrapping_add(0x50u64)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rax) as u64); r9 = ((rsp.wrapping_add(0x30u64)) as u64) as u64; r8 = ((r13 as u32)) as u32 as u64; rdx = ((rdi as u32)) as u32 as u64; rcx = ((rsp.wrapping_add(0xc0u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001ec6be9); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = match eh.call_xmm(0x7001ec6be9, (P8) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]) { Ok(v) => v, Err(l) => match l.cont_va { 0x7001ec6d36 => { rax = crate::dll_native::lift::eh::landed_rax(); bb = 20; continue; } _ => l.unknown(0x7001ec6a94) } }; rax = c_rax; xmm0 = c_x0; bb = 10; continue; }
            10 => { rax = ((rsp.wrapping_add(0xc0u64)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x88u64), (rax) as u64); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xc8u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x90u64), (rax) as u64); r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; fl_l = (r15) as u64 as u64; fl_r = (rax) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 22; continue; } bb = 11; continue; }
            11 => { fl_l = (r15) as u64 as u64; fl_r = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64 as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 22; continue; } bb = 12; continue; }
            12 => { rdx = ((rsp.wrapping_add(0x38u64)) as u64) as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001ec6c32); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = match eh.call_xmm(0x7001ec6c32, (P9) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]) { Ok(v) => v, Err(l) => match l.cont_va { 0x7001ec6d36 => { rax = crate::dll_native::lift::eh::landed_rax(); bb = 20; continue; } _ => l.unknown(0x7001ec6a94) } }; rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), (r15) as u64); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; xmm6 = (crate::simd_ops::ru128(rbx.wrapping_add(0x10u64))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0xb0u64), (xmm6) as u128); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x30u64))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0xa0u64), (xmm0) as u128); rdx = ((rsp.wrapping_add(0x38u64)) as u64) as u64; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001ec6c63); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = match eh.call_xmm(0x7001ec6c63, (P9) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]) { Ok(v) => v, Err(l) => match l.cont_va { 0x7001ec6d36 => { rax = crate::dll_native::lift::eh::landed_rax(); bb = 20; continue; } _ => l.unknown(0x7001ec6a94) } }; rax = c_rax; xmm0 = c_x0; rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x10u64))) as u32 as u64; r15 = ((r15).wrapping_sub(rax)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), (r15) as u64); rax = ((rsp.wrapping_add(0x30u64)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rax) as u64); r9 = ((rdi as u32)) as u32 as u64; r8 = ((rsp.wrapping_add(0xa0u64)) as u64) as u64; rdx = ((r13 as u32)) as u32 as u64; rcx = ((rsp.wrapping_add(0x50u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001ec6c90); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = match eh.call_xmm(0x7001ec6c90, (P10) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]) { Ok(v) => v, Err(l) => match l.cont_va { 0x7001ec6d36 => { rax = crate::dll_native::lift::eh::landed_rax(); bb = 20; continue; } _ => l.unknown(0x7001ec6a94) } }; rax = c_rax; xmm0 = c_x0; rdi = ((rax as u32)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x44u64), ((rax as u32)) as u32); crate::simd_ops::lk_and_u32(rsp.wrapping_add(0x40u64), (0x0u32) as u32); r9 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; xmm0 = (xmm6) as u128; xmm0 = crate::simd_ops::vpsrldq(xmm0, 0x8u8); rax = ((xmm0 as u32)) as u32 as u64; xmm1 = (xmm6) as u128; xmm1 = crate::simd_ops::vpsrldq(xmm1, 0x4u8); rcx = ((xmm1 as u32)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; r9 = (if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { (rax as u32) } else { (r9 as u32) }) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x40u64), ((r9 as u32)) as u32); fl_l = ((r9 as u32)) as u32 as u64; fl_r = ((r9 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 21; continue; } bb = 13; continue; }
            13 => { rax = ((rdi.wrapping_add(0x2u64)) as u32) as u32 as u64; crate::simd_ops::wu32(r14, ((rax as u32)) as u32); rax = ((rcx.wrapping_sub(0x1u64)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x1u32) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 15; continue; } bb = 14; continue; }
            14 => { rcx = ((((r9 as u32)) as i32 as i64) as u64) as u64; rcx = ((rcx).wrapping_add((match eh.ru64(0x7001ec6ce2, rsi) { Ok(v) => v, Err(l) => match l.cont_va { 0x7001ec6d36 => { rax = crate::dll_native::lift::eh::landed_rax(); bb = 20; continue; } _ => l.unknown(0x7001ec6a94) } }))) as u64; r8 = (0x103u32) as u32 as u64; rdx = (r12) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001ec6cee); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = match eh.call_xmm(0x7001ec6cee, (P11) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]) { Ok(v) => v, Err(l) => match l.cont_va { 0x7001ec6d36 => { rax = crate::dll_native::lift::eh::landed_rax(); bb = 20; continue; } _ => l.unknown(0x7001ec6a94) } }; rax = c_rax; xmm0 = c_x0; bb = 19; continue; }
            15 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rdx = ((match eh.ru64(0x7001ec6cfa, rax) { Ok(v) => v, Err(l) => match l.cont_va { 0x7001ec6d36 => { rax = crate::dll_native::lift::eh::landed_rax(); bb = 20; continue; } _ => l.unknown(0x7001ec6a94) } })) as u64; xmm6 = crate::simd_ops::vpsrldq(xmm6, 0xcu8); fl_l = ((rcx as u32)) as u32 as u64; fl_r = (0x2u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 17; continue; } bb = 16; continue; }
            16 => { rax = ((xmm6 as u32)) as u32 as u64; r8 = ((match eh.ru64(0x7001ec6d0b, rax.wrapping_add(rdx)) { Ok(v) => v, Err(l) => match l.cont_va { 0x7001ec6d36 => { rax = crate::dll_native::lift::eh::landed_rax(); bb = 20; continue; } _ => l.unknown(0x7001ec6a94) } })) as u64; bb = 18; continue; }
            17 => { r8 = ((xmm6 as u32)) as u32 as u64; fl_l = ((r8).wrapping_add(rdx)) as u64 as u64; fl_r = fl_l; r8 = ((fl_l as u64)) as u64; bb = 18; continue; }
            18 => { rcx = ((((r9 as u32)) as i32 as i64) as u64) as u64; rcx = ((rcx).wrapping_add((match eh.ru64(0x7001ec6d1c, rsi) { Ok(v) => v, Err(l) => match l.cont_va { 0x7001ec6d36 => { rax = crate::dll_native::lift::eh::landed_rax(); bb = 20; continue; } _ => l.unknown(0x7001ec6a94) } }))) as u64; r9 = (0x103u32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001ec6d25); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = match eh.call_xmm(0x7001ec6d25, (P12) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]) { Ok(v) => v, Err(l) => match l.cont_va { 0x7001ec6d36 => { rax = crate::dll_native::lift::eh::landed_rax(); bb = 20; continue; } _ => l.unknown(0x7001ec6a94) } }; rax = c_rax; xmm0 = c_x0; bb = 19; continue; }
            19 => { rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001ec6d2f); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = match eh.call_xmm(0x7001ec6d2f, (P13) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]) { Ok(v) => v, Err(l) => match l.cont_va { 0x7001ec6d36 => { rax = crate::dll_native::lift::eh::landed_rax(); bb = 20; continue; } _ => l.unknown(0x7001ec6a94) } }; rax = c_rax; xmm0 = c_x0; bb = 21; continue; }
            20 => { rdi = (crate::simd_ops::ru32(rsp.wrapping_add(0x44u64))) as u32 as u64; r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; r13 = (crate::simd_ops::ru32(rsp.wrapping_add(0x48u64))) as u32 as u64; bb = 21; continue; }
            21 => { bb = 10; continue; }
            22 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001ec6d56); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = match eh.call_xmm(0x7001ec6d56, (P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]) { Ok(v) => v, Err(l) => match l.cont_va { 0x7001ec6d36 => { rax = crate::dll_native::lift::eh::landed_rax(); bb = 20; continue; } _ => l.unknown(0x7001ec6a94) } }; rax = c_rax; xmm0 = c_x0; fl_l = (crate::simd_ops::ru32(rax.wrapping_add(0x30u64))) as u32 as u64; fl_r = (0x0u32) as u32 as u64; if ((((fl_l as u32)) as i32) <= (((fl_r as u32)) as i32)) { bb = 24; continue; } bb = 23; continue; }
            23 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001ec6d61); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = match eh.call_xmm(0x7001ec6d61, (P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]) { Ok(v) => v, Err(l) => match l.cont_va { 0x7001ec6d36 => { rax = crate::dll_native::lift::eh::landed_rax(); bb = 20; continue; } _ => l.unknown(0x7001ec6a94) } }; rax = c_rax; xmm0 = c_x0; fl_l = (crate::simd_ops::lk_sub_u32(rax.wrapping_add(0x30u64), (1) as u32)) as u32 as u64; bb = 24; continue; }
            24 => { rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0xe0u64))) as u64; rcx = (((rcx) ^ (rsp))) as u64; xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0xf0u64))) as u128; rsp = ((rsp).wrapping_add(0x100u64)) as u64; r15 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r13 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r12 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rsi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rbx = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001ec6d94(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001ec8094; const P1: u64 = 0x7001ec8080; const P2: u64 = 0x7001ec5e3c; const P3: u64 = 0x7001ec9138; const P4: u64 = 0x7001ec6e81;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (rbp) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r12) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r13) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r15) as u64); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbp = (rdx) as u64; r13 = (rcx) as u64; fl_l = (rdx) as u64 as u64; fl_r = (rdx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 1; continue; }
            1 => { r15 = (r15 & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; rsi = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; fl_l = (crate::simd_ops::ru32(rdx)) as u32 as u64; fl_r = ((rsi as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) <= (((fl_r as u32)) as i32)) { bb = 9; continue; } bb = 2; continue; }
            2 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rdx = (rax) as u64; rax = (crate::simd_ops::ru64(r13.wrapping_add(0x30u64))) as u64; r12 = (((crate::simd_ops::ru32(rax.wrapping_add(0xcu64))) as i32 as i64) as u64) as u64; r12 = ((r12).wrapping_add(0x4u64)) as u64; r12 = ((r12).wrapping_add(rdx)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rdx = (rax) as u64; rax = (crate::simd_ops::ru64(r13.wrapping_add(0x30u64))) as u64; rcx = (((crate::simd_ops::ru32(rax.wrapping_add(0xcu64))) as i32 as i64) as u64) as u64; r14 = (crate::simd_ops::ru32(rdx.wrapping_add(rcx))) as u32 as u64; fl_l = ((r14 as u32)) as u32 as u64; fl_r = ((r14 as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 8; continue; } bb = 3; continue; }
            3 => { rax = ((((rsi as u32)) as i32 as i64) as u64) as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), (rax) as u64); bb = 4; continue; }
            4 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rbx = (crate::simd_ops::ru64(r13.wrapping_add(0x30u64))) as u64; rdi = (rax) as u64; rax = (((crate::simd_ops::ru32(r12)) as i32 as i64) as u64) as u64; rdi = ((rdi).wrapping_add(rax)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; r8 = (rbx) as u64; rcx = (((crate::simd_ops::ru32(rbp.wrapping_add(0x4u64))) as i32 as i64) as u64) as u64; rax = ((rax.wrapping_add(rdx.wrapping_mul(4))) as u64) as u64; rdx = (rdi) as u64; rcx = ((rcx).wrapping_add(rax)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 7; continue; } bb = 5; continue; }
            5 => { r14 = (((r14 as u32)).wrapping_sub(1)) as u32 as u64; r12 = ((r12).wrapping_add(0x4u64)) as u64; fl_l = ((r14 as u32)) as u32 as u64; fl_r = ((r14 as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) > 0) { bb = 4; continue; } bb = 6; continue; }
            6 => { bb = 8; continue; }
            7 => { r15 = (r15 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 8; continue; }
            8 => { rsi = (((rsi as u32)).wrapping_add(1)) as u32 as u64; fl_l = ((rsi as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rbp)) as u32 as u64; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 2; continue; } bb = 9; continue; }
            9 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rax = (rax & !0xffu64) | ((((r15 as u8)) as u8 as u64)); rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r13 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r12 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            10 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P3)) as u64; if t == 0x7001ec87f0 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); crate::lift_ops::trap(0x80000003, P4, rcx, rsp); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001ec6e84(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001ec8080; const P1: u64 = 0x4; const P2: u64 = 0x8; const P3: u64 = 0x7001ecc0b8; const P4: u64 = 0x7001ec721c;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rbp) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbp = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rdi = (rcx) as u64; fl_l = (crate::simd_ops::ru32(rcx)) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) <= (((fl_r as u32)) as i32)) { bb = 7; continue; } bb = 1; continue; }
            1 => { rsi = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; bb = 2; continue; }
            2 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rcx = (((crate::simd_ops::ru32(rdi.wrapping_add(P1))) as i32 as i64) as u64) as u64; rax = ((rax).wrapping_add(rsi)) as u64; fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(rax).wrapping_add(P1))) as u32 as u64; fl_r = (0x0u32) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 4; continue; } bb = 3; continue; }
            3 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rcx = (((crate::simd_ops::ru32(rdi.wrapping_add(P1))) as i32 as i64) as u64) as u64; rax = ((rax).wrapping_add(rsi)) as u64; rbx = (((crate::simd_ops::ru32(rcx.wrapping_add(rax).wrapping_add(P1))) as i32 as i64) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); fl_l = ((rax).wrapping_add(rbx)) as u64 as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; bb = 5; continue; }
            4 => { rax = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; bb = 5; continue; }
            5 => { rcx = ((rax.wrapping_add(P2)) as u64) as u64; rdx = ((P3) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 9; continue; } bb = 6; continue; }
            6 => { rbp = (((rbp as u32)).wrapping_add(1)) as u32 as u64; rsi = ((rsi).wrapping_add(0x14u64)) as u64; fl_l = ((rbp as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rdi)) as u32 as u64; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 2; continue; } bb = 7; continue; }
            7 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 8; continue; }
            8 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            9 => { rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 8; continue; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001ec6f0c(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x7001ec4000; const P1: u64 = 0xfffffffffffffffc; const P2: u64 = 0x10; const P3: u64 = 0x14; const P4: u64 = 0xffffffffffffffff; const P5: u64 = 0x18; const P6: u64 = 0x1c;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { r8 = (crate::simd_ops::ru64(rdx)) as u64; r11 = ((P0) as u64) as u64; r10 = (rcx) as u64; r9 = (rdx) as u64; rcx = ((crate::simd_ops::ru8(r8)) as u32) as u32 as u64; rcx = ((((rcx as u32)) & (0xfu32))) as u32 as u64; rax = (((crate::simd_ops::ru8(rcx.wrapping_add(r11).wrapping_add(0x5208u64))) as i8 as i64) as u64) as u64; rcx = (rcx & !0xffu64) | (((crate::simd_ops::ru8(rcx.wrapping_add(r11).wrapping_add(0x5218u64))) as u8 as u64)); r8 = ((r8).wrapping_sub(rax)) as u64; rax = (crate::simd_ops::ru32(r8.wrapping_add(P1))) as u32 as u64; rax = (((rax as u32)).wrapping_shr(((rcx as u32) & 31))) as u32 as u64; rcx = ((rax as u32)) as u32 as u64; crate::simd_ops::wu64(rdx, (r8) as u64); rcx = ((((rcx as u32)) & (0x3u32))) as u32 as u64; rax = (((rax as u32)).wrapping_shr(2u32)) as u32 as u64; crate::simd_ops::wu32(r10.wrapping_add(P2), ((rax as u32)) as u32); crate::simd_ops::wu32(r10.wrapping_add(P3), ((rcx as u32)) as u32); rax = ((rcx.wrapping_add(P4)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x1u32) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 3; continue; } bb = 1; continue; }
            1 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = (0x3u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 4; continue; } bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(rdx)) as u64; rcx = (crate::simd_ops::ru32(rax)) as u32 as u64; fl_l = ((rax).wrapping_add(0x4u64)) as u64 as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; crate::simd_ops::wu64(rdx, (rax) as u64); crate::simd_ops::wu32(r10.wrapping_add(P5), ((rcx as u32)) as u32); return rax; }
            3 => { rax = (crate::simd_ops::ru64(rdx)) as u64; rcx = (crate::simd_ops::ru32(rax)) as u32 as u64; rax = ((rax).wrapping_add(0x4u64)) as u64; crate::simd_ops::wu64(rdx, (rax) as u64); crate::simd_ops::wu32(r10.wrapping_add(P5), ((rcx as u32)) as u32); rdx = (crate::simd_ops::ru64(rdx)) as u64; rcx = ((crate::simd_ops::ru8(rdx)) as u32) as u32 as u64; rcx = ((((rcx as u32)) & (0xfu32))) as u32 as u64; rax = (((crate::simd_ops::ru8(rcx.wrapping_add(r11).wrapping_add(0x5208u64))) as i8 as i64) as u64) as u64; rcx = (rcx & !0xffu64) | (((crate::simd_ops::ru8(rcx.wrapping_add(r11).wrapping_add(0x5218u64))) as u8 as u64)); rdx = ((rdx).wrapping_sub(rax)) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(P1))) as u32 as u64; fl_l = (((rax as u32)).wrapping_shr(((rcx as u32) & 31))) as u32 as u64; rax = ((fl_l as u32)) as u32 as u64; crate::simd_ops::wu64(r9, (rdx) as u64); crate::simd_ops::wu32(r10.wrapping_add(P6), ((rax as u32)) as u32); bb = 4; continue; }
            4 => { return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001ec6fa8(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = (rdx) as u64; rdx = (r8) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((rax) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001ec6fb4(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r10) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = (r8) as u64; r10 = (rdx) as u64; rdx = (rax) as u64; r8 = ((r9 as u32)) as u32 as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((r10) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001ec6fc4(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001ec6f0c;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11, 32>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    // stackargs: A=32 max=5
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { r11 = (rsp) as u64; crate::simd_ops::wu64(r11.wrapping_add(0x18u64), (rbx) as u64); crate::simd_ops::wu64(r11.wrapping_add(0x20u64), (r9) as u64); crate::simd_ops::wu32(rsp.wrapping_add(0x10u64), ((rdx as u32)) as u32); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r12) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r13) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r15) as u64); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; rbp = (rbp & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; r14 = (r14 & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; crate::simd_ops::wu64(r11.wrapping_add(0x8u64), (rax) as u64); rdi = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; r12 = (r9) as u64; r13 = ((r8 as u32)) as u32 as u64; rbx = (rcx) as u64; rsi = ((rax.wrapping_sub(0x1u64)) as u64) as u64; r15 = (rsi) as u64; fl_l = (crate::simd_ops::ru32(rcx)) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) <= (((fl_r as u32)) as i32)) { bb = 11; continue; } bb = 1; continue; }
            1 => { r12 = (crate::simd_ops::ru32(r11.wrapping_add(0x10u64))) as u32 as u64; bb = 2; continue; }
            2 => { fl_l = ((rdi as u32)) as u32 as u64; fl_r = ((r12 as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rsi = (rax) as u64; rbp = (rbp & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 4; continue; }
            4 => { fl_l = ((rdi as u32)) as u32 as u64; fl_r = ((r13 as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 6; continue; } bb = 5; continue; }
            5 => { r15 = (rax) as u64; r14 = (r14 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 6; continue; }
            6 => { fl_l = ((rbp as u8)) as u8 as u64; fl_r = ((rbp as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 8; continue; } bb = 7; continue; }
            7 => { fl_l = ((r14 as u8)) as u8 as u64; fl_r = ((r14 as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 10; continue; } bb = 8; continue; }
            8 => { rdx = ((rsp.wrapping_add(0x60u64)) as u64) as u64; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdi = (((rdi as u32)).wrapping_add(1)) as u32 as u64; fl_l = ((rdi as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rbx)) as u32 as u64; if ((((fl_l as u32)) as i32) >= (((fl_r as u32)) as i32)) { bb = 10; continue; } bb = 9; continue; }
            9 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; bb = 2; continue; }
            10 => { r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; bb = 11; continue; }
            11 => { rax = (crate::simd_ops::ru64(r12)) as u64; crate::simd_ops::wu64(r12.wrapping_add(0x8u64), (rsi) as u64); xmm0 = (crate::simd_ops::ru128(rbx)) as u128; crate::simd_ops::wu128(rax, (xmm0) as u128); xmm1 = (crate::simd_ops::ru128(rbx.wrapping_add(0x10u64))) as u128; crate::simd_ops::wu128(rax.wrapping_add(0x10u64), (xmm1) as u128); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; rcx = (crate::simd_ops::ru64(rax)) as u64; crate::simd_ops::wu64(rax.wrapping_add(0x8u64), (r15) as u64); xmm0 = (crate::simd_ops::ru128(rbx)) as u128; crate::simd_ops::wu128(rcx, (xmm0) as u128); xmm1 = (crate::simd_ops::ru128(rbx.wrapping_add(0x10u64))) as u128; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; crate::simd_ops::wu128(rcx.wrapping_add(0x10u64), (xmm1) as u128); rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r13 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r12 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rsi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rbp = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001ec7094(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x7001ec6f0c;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rbx = ((rdx as u32)) as u32 as u64; rsi = (r8) as u64; r10 = (rcx) as u64; rdx = (crate::simd_ops::ru64(rdi.wrapping_add(P0))) as u64; fl_l = (rdx) as u64 as u64; fl_r = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64 as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 9; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64 as u64; fl_r = (rdx) as u64 as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 9; continue; } bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64; rcx = (rdx) as u64; rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(r10.wrapping_add(P0)))) as u64; rax = ((rax).wrapping_sub(rdx)) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) as i64) >= (((fl_r as u64)) as i64)) { bb = 6; continue; } bb = 3; continue; }
            3 => { xmm0 = (crate::simd_ops::ru128(r10)) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128); fl_l = (rdx) as u64 as u64; fl_r = (crate::simd_ops::ru64(r10.wrapping_add(P0))) as u64 as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 8; continue; } bb = 4; continue; }
            4 => { rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; rdx = ((rsp.wrapping_add(0x28u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; rbx = (((rbx as u32)).wrapping_add(1)) as u32 as u64; fl_l = (crate::simd_ops::ru64(rdi.wrapping_add(P0))) as u64 as u64; fl_r = (rax) as u64 as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 4; continue; } bb = 5; continue; }
            5 => { bb = 8; continue; }
            6 => { rbx = ((r9 as u32)) as u32 as u64; xmm0 = (crate::simd_ops::ru128(rdi)) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128); fl_l = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64 as u64; fl_r = (rdx) as u64 as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 8; continue; } bb = 7; continue; }
            7 => { rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; rdx = ((rsp.wrapping_add(0x28u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; rbx = (((rbx as u32)).wrapping_sub(1)) as u32 as u64; fl_l = (crate::simd_ops::ru64(rsi.wrapping_add(P0))) as u64 as u64; fl_r = (rcx) as u64 as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 7; continue; } bb = 8; continue; }
            8 => { rax = ((rbx as u32)) as u32 as u64; bb = 10; continue; }
            9 => { fl_l = ((((rax as u32)) | (0xffffffffu32))) as u32 as u64; fl_r = fl_l; rax = ((fl_l as u32)) as u32 as u64; bb = 10; continue; }
            10 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001ec7150(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x7001ec9240;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    fl_l = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64 as u64; fl_r = (0x0u64) as u64 as u64;
    rax = ((P1) as u64) as u64;
    rax = (if (((fl_l as u64)) != ((fl_r as u64))) { crate::simd_ops::ru64(rcx.wrapping_add(P0)) } else { rax }) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001ec7164(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x1; const P2: u64 = 0x7001ec9110; const P3: u64 = 0x7001ec9148; const P4: u64 = 0x7001ec9120;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rsi) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(P0))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; r14 = (rdx) as u64; rsi = (rcx) as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 7; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(rcx)) as u64; fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 2; continue; }
            2 => { fl_l = (((rdi) | (0xffffffffffffffffu64))) as u64 as u64; fl_r = fl_l; rdi = ((fl_l as u64)) as u64; bb = 3; continue; }
            3 => { rdi = ((rdi).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru8(rax.wrapping_add(rdi))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 3; continue; } bb = 4; continue; }
            4 => { rcx = ((rdi.wrapping_add(P1)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P2)) as u64; if t == 0x7001ec87f0 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rbx = (rax) as u64; fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { r8 = (crate::simd_ops::ru64(rsi)) as u64; rdx = ((rdi.wrapping_add(P1)) as u64) as u64; rcx = (rax) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P3)) as u64; if t == 0x7001ec87f0 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rax = (rbx) as u64; crate::simd_ops::wu8(r14.wrapping_add(P0), (0x1u8) as u8); crate::simd_ops::wu64(r14, (rax) as u64); rbx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; bb = 6; continue; }
            6 => { rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P4)) as u64; if t == 0x7001ec87f0 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 8; continue; }
            7 => { rax = (crate::simd_ops::ru64(rcx)) as u64; crate::simd_ops::wu64(rdx, (rax) as u64); crate::simd_ops::wu8(rdx.wrapping_add(P0), (0x0u8) as u8); bb = 8; continue; }
            8 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001ec71f4(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x7001ec9120;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(P0))) as u8 as u64; fl_r = (0x0u8) as u8 as u64;
    rbx = (rcx) as u64;
    if !((((fl_l as u8)) == ((fl_r as u8)))) { rcx = (crate::simd_ops::ru64(rcx)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P1)) as u64; if t == 0x7001ec87f0 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); }
    crate::simd_ops::lk_and_u64(rbx, (0x0u64) as u64);
    crate::simd_ops::wu8(rbx.wrapping_add(P0), (0x0u8) as u8);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = core::ptr::read_unaligned(rsp as *const u64);
    rsp = rsp.wrapping_add(8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001ec721c(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x9;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { fl_l = (rcx) as u64 as u64; fl_r = (rdx) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 4; continue; } bb = 1; continue; }
            1 => { rdx = ((rdx).wrapping_add(0x9u64)) as u64; rax = ((rcx.wrapping_add(P0)) as u64) as u64; fl_l = (rdx) as u64 as u64; fl_r = (rax) as u64 as u64; rdx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; bb = 2; continue; }
            2 => { rcx = (rcx & !0xffu64) | (((crate::simd_ops::ru8(rax)) as u8 as u64)); fl_l = ((rcx as u8)) as u8 as u64; fl_r = (crate::simd_ops::ru8(rax.wrapping_add(rdx))) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 5; continue; } bb = 3; continue; }
            3 => { rax = ((rax).wrapping_add(1)) as u64; fl_l = ((rcx as u8)) as u8 as u64; fl_r = ((rcx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 2; continue; } bb = 4; continue; }
            4 => { rax = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; return rax; }
            5 => { rax = (if (((fl_l as u8)) < ((fl_r as u8))) { 0xffffffffu32 } else { 0u32 }) as u32 as u64; fl_l = ((((rax as u32)) | (0x1u32))) as u32 as u64; fl_r = fl_l; rax = ((fl_l as u32)) as u32 as u64; return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001ec7244(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x30; const P1: u64 = 0x40; const P2: u64 = 0x7001ec9170; const P3: u64 = 0x7001ec90b8; const P4: u64 = 0x3; const P5: u64 = 0x7001ec90b0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = ((rsp).wrapping_sub(0x50u64)) as u64; rbx = (rdx) as u64; rsi = (rcx) as u64; rdi = (0x19930520u32) as u32 as u64; fl_l = (rdx) as u64 as u64; fl_r = (rdx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru8(rdx)) as u8 as u64; fl_r = (0x10u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { rcx = (crate::simd_ops::ru64(rcx)) as u64; rcx = ((rcx).wrapping_sub(0x8u64)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rbx = (crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P2)) as u64; if t == 0x7001ec87f0 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 3; continue; }
            3 => { rax = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rax) as u64); fl_l = (rbx) as u64 as u64; fl_r = (rbx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 4; continue; }
            4 => { rdx = ((rsp.wrapping_add(0x20u64)) as u64) as u64; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P3)) as u64; if t == 0x7001ec87f0 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rax) as u64); fl_l = (crate::simd_ops::ru8(rbx)) as u8 as u64; fl_r = (0x8u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 7; continue; } bb = 6; continue; }
            6 => { rdi = (0x1994000u32) as u32 as u64; bb = 7; continue; }
            7 => { rdx = (0x1u32) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (rdi) as u64); r9 = ((rsp.wrapping_add(0x28u64)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (rsi) as u64); rcx = (0xe06d7363u32) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), (rax) as u64); r8 = ((rdx.wrapping_add(P4)) as u32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P5)) as u64; if t == 0x7001ec87f0 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; rsp = ((rsp).wrapping_add(0x50u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001ec72ec(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001ec83c0; const P1: u64 = 0x7001ec735c; const P2: u64 = 0x7001ec7574; const P3: u64 = 0x7001ec73a4;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x28u64)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 5; continue; }
            2 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 1; continue; }
            4 => { rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 5; continue; }
            5 => { rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001ec731c(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001ec7464;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64;
    rax = (rax & !0xffu64) | (((if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { 1u8 } else { 0u8 }) as u8 as u64));
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001ec7330(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001ec7400;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64));
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001ec7340(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001ec75bc; const P1: u64 = 0x7001ec73a4;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    fl_l = ((rcx as u8)) as u8 as u64; fl_r = ((rcx as u8)) as u8 as u64;
    if !(((((fl_l as u8)) & ((fl_r as u8))) != 0)) { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); }
    rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64));
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001ec735c(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001ecc128; const P1: u64 = 0x7001ec785c; const P2: u64 = 0x7001ecc150; const P3: u64 = 0x7001ec73a4;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; bb = 1; continue; }
            1 => { rdx = ((P0) as u64) as u64; r8 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rcx = ((rbx.wrapping_add(rbx.wrapping_mul(4))) as u64) as u64; rcx = ((rdx.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; rdx = (0xfa0u32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 4; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::lk_add_u32(P2, (1) as u32); rbx = (((rbx as u32)).wrapping_add(1)) as u32 as u64; fl_l = ((rbx as u32)) as u32 as u64; fl_r = (0x1u32) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 1; continue; } bb = 3; continue; }
            3 => { rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 5; continue; }
            4 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 5; continue; }
            5 => { rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001ec73a4(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001ecc150; const P1: u64 = 0x7001ecc128; const P2: u64 = 0x7001ec90c8;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = (crate::simd_ops::ru32(P0)) as u32 as u64; bb = 2; continue; }
            1 => { rax = ((P1) as u64) as u64; rbx = (((rbx as u32)).wrapping_sub(1)) as u32 as u64; rcx = ((rbx.wrapping_add(rbx.wrapping_mul(4))) as u64) as u64; rcx = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P2)) as u64; if t == 0x7001ec87f0 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); fl_l = (crate::simd_ops::lk_sub_u32(P0, (1) as u32)) as u32 as u64; bb = 2; continue; }
            2 => { fl_l = ((rbx as u32)) as u32 as u64; fl_r = ((rbx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 1; continue; } bb = 3; continue; }
            3 => { rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001ec73e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001ecc154; const P1: u64 = 0x7001ec9120;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64;
    if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
    rax = ((P0) as u64) as u64;
    fl_l = (rcx) as u64 as u64; fl_r = (rax) as u64 as u64;
    if !((((fl_l as u64)) == ((fl_r as u64)))) { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P1)) as u64; if t == 0x7001ec87f0 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); }
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001ec7400(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001ecc000; const P1: u64 = 0x7001ec77c0; const P2: u64 = 0x7001ec7808; const P3: u64 = 0x7001ecc154; const P4: u64 = 0x7001ec9120;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rcx = (crate::simd_ops::ru32(P0)) as u32 as u64;
    fl_l = ((rcx as u32)) as u32 as u64; fl_r = (0xffffffffu32) as u32 as u64;
    if (((fl_l as u32)) == ((fl_r as u32))) { rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    rcx = (crate::simd_ops::ru32(P0)) as u32 as u64;
    rdx = (0u32) as u32 as u64;
    fl_l = 0; fl_r = 0;
    rbx = (rax) as u64;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    fl_l = (rbx) as u64 as u64; fl_r = (rbx) as u64 as u64;
    if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
    rax = ((P3) as u64) as u64;
    fl_l = (rbx) as u64 as u64; fl_r = (rax) as u64 as u64;
    if !((((fl_l as u64)) == ((fl_r as u64)))) { rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P4)) as u64; if t == 0x7001ec87f0 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); }
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = core::ptr::read_unaligned(rsp as *const u64);
    rsp = rsp.wrapping_add(8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001ec7448(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001ec7464; const P1: u64 = 0x7001ec9138; const P2: u64 = 0x7001ec745c;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64;
    if !(((((fl_l as u64)) & ((fl_r as u64))) != 0)) { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P1)) as u64; if t == 0x7001ec87f0 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); crate::lift_ops::trap(0x80000003, P2, rcx, rsp); return rax; }
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001ec7464(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001ecc000; const P1: u64 = 0x7001ec90c0; const P2: u64 = 0x7001ec77c0; const P3: u64 = 0x7001ec7808; const P4: u64 = 0xfffffffffffffffd; const P5: u64 = 0x7001ec9118; const P6: u64 = 0x7001ec9120; const P7: u64 = 0x7001ec90d0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; fl_l = (crate::simd_ops::ru32(P0)) as u32 as u64; fl_r = (0xffffffffu32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; bb = 13; continue; }
            2 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P1)) as u64; if t == 0x7001ec87f0 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rcx = (crate::simd_ops::ru32(P0)) as u32 as u64; rdi = ((rax as u32)) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rdx = (((rdx) | (0xffffffffffffffffu64))) as u64; rsi = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; fl_l = (rax) as u64 as u64; fl_r = (rdx) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 12; continue; } bb = 3; continue; }
            3 => { fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rsi = (rax) as u64; bb = 12; continue; }
            5 => { rcx = (crate::simd_ops::ru32(P0)) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 12; continue; } bb = 6; continue; }
            6 => { rdx = (0x4u32) as u32 as u64; rcx = ((rdx.wrapping_add(P4)) as u32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P5)) as u64; if t == 0x7001ec87f0 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rcx = (crate::simd_ops::ru32(P0)) as u32 as u64; rbx = (rax) as u64; fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 7; continue; }
            7 => { rdx = (rax) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 9; continue; } bb = 8; continue; }
            8 => { rax = (rbx) as u64; crate::simd_ops::wu32(rbx, (0xfffffffeu32) as u32); rbx = (rsi) as u64; rsi = (rax) as u64; bb = 11; continue; }
            9 => { rcx = (crate::simd_ops::ru32(P0)) as u32 as u64; bb = 10; continue; }
            10 => { rdx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 11; continue; }
            11 => { rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P6)) as u64; if t == 0x7001ec87f0 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 12; continue; }
            12 => { rcx = ((rdi as u32)) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P7)) as u64; if t == 0x7001ec87f0 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rax = (rsi) as u64; bb = 13; continue; }
            13 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001ec7524(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001ec87b7; const P1: u64 = 0x7001ec87bd; const P2: u64 = 0x7001ec9138;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x48u64)) as u64;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (rax) as u64);
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (rax) as u64);
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64;
    rax = ((rax).wrapping_sub(0x20u64)) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rax) as u64);
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rax = ((rax).wrapping_sub(0x30u64)) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), (rax) as u64);
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64;
    fl_l = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64 as u64; fl_r = (rax) as u64 as u64;
    if !((((fl_l as u64)) == ((fl_r as u64)))) { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P2)) as u64; if t == 0x7001ec87f0 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); }
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64;
    rsp = ((rsp).wrapping_add(0x48u64)) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001ec7574(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001ec73e0; const P1: u64 = 0x7001ec7730; const P2: u64 = 0x7001ecc000; const P3: u64 = 0x7001ecc154; const P4: u64 = 0x7001ec7808; const P5: u64 = 0x7001ec75bc;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x28u64)) as u64; rcx = ((P0) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); crate::simd_ops::wu32(P2, ((rax as u32)) as u32); fl_l = ((rax as u32)) as u32 as u64; fl_r = (0xffffffffu32) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 4; continue; } bb = 1; continue; }
            1 => { rdx = ((P3) as u64) as u64; rcx = ((rax as u32)) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::wu32(P3, (0xfffffffeu32) as u32); rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 5; continue; }
            3 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 4; continue; }
            4 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 5; continue; }
            5 => { rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001ec75bc(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001ecc000; const P1: u64 = 0x7001ec7778;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    rcx = (crate::simd_ops::ru32(P0)) as u32 as u64;
    fl_l = ((rcx as u32)) as u32 as u64; fl_r = (0xffffffffu32) as u32 as u64;
    if !((((fl_l as u32)) == ((fl_r as u32)))) { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); fl_l = (crate::simd_ops::lk_or_u32(P0, (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; }
    rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64));
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001ec75e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001ec4000; const P1: u64 = 0x7001ec9068; const P2: u64 = 0x7001ec90c0; const P3: u64 = 0x7001ec9330; const P4: u64 = 0x7001ec9150; const P5: u64 = 0x7001ec9078; const P6: u64 = 0x7001ec9070;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rbp) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r12) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r13) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r15) as u64); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = ((rcx as u32)) as u32 as u64; r15 = ((P0) as u64) as u64; r14 = (((r14) | (0xffffffffffffffffu64))) as u64; r12 = (r9) as u64; rbp = (r8) as u64; r13 = (rdx) as u64; rax = (crate::simd_ops::ru64(r15.wrapping_add(rdi.wrapping_mul(8)).wrapping_add(0x8170u64))) as u64; fl_l = (rax) as u64 as u64; fl_r = (r14) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 13; continue; } bb = 1; continue; }
            1 => { fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 14; continue; } bb = 2; continue; }
            2 => { fl_l = (r8) as u64 as u64; fl_r = (r9) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 12; continue; } bb = 3; continue; }
            3 => { rsi = (crate::simd_ops::ru32(rbp)) as u32 as u64; rbx = (crate::simd_ops::ru64(r15.wrapping_add(rsi.wrapping_mul(8)).wrapping_add(0x8158u64))) as u64; fl_l = (rbx) as u64 as u64; fl_r = (rbx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 4; continue; }
            4 => { fl_l = (rbx) as u64 as u64; fl_r = (r14) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 17; continue; } bb = 5; continue; }
            5 => { bb = 11; continue; }
            6 => { r15 = (crate::simd_ops::ru64(r15.wrapping_add(rsi.wrapping_mul(8)).wrapping_add(0x5280u64))) as u64; rdx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rcx = (r15) as u64; r8 = (0x800u32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P1)) as u64; if t == 0x7001ec87f0 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rbx = (rax) as u64; fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 15; continue; } bb = 7; continue; }
            7 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P2)) as u64; if t == 0x7001ec87f0 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x57u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 10; continue; } bb = 8; continue; }
            8 => { r8 = ((rbx.wrapping_add(0x7u64)) as u32) as u32 as u64; rcx = (r15) as u64; rdx = ((P3) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P4)) as u64; if t == 0x7001ec87f0 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 10; continue; } bb = 9; continue; }
            9 => { r8 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rdx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rcx = (r15) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P1)) as u64; if t == 0x7001ec87f0 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rbx = (rax) as u64; fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 15; continue; } bb = 10; continue; }
            10 => { rax = (r14) as u64; r15 = ((P0) as u64) as u64; let tmp_xchg: u64 = crate::simd_ops::atomic_xchg_u64(r15.wrapping_add(rsi.wrapping_mul(8)).wrapping_add(0x8158u64), (rax) as u64); rax = (tmp_xchg) as u64; bb = 11; continue; }
            11 => { rbp = ((rbp).wrapping_add(0x4u64)) as u64; fl_l = (rbp) as u64 as u64; fl_r = (r12) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } bb = 12; continue; }
            12 => { let tmp_xchg: u64 = crate::simd_ops::atomic_xchg_u64(r15.wrapping_add(rdi.wrapping_mul(8)).wrapping_add(0x8170u64), (r14) as u64); r14 = (tmp_xchg) as u64; bb = 13; continue; }
            13 => { rax = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; bb = 14; continue; }
            14 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r13 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r12 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            15 => { rax = (rbx) as u64; r15 = ((P0) as u64) as u64; let tmp_xchg: u64 = crate::simd_ops::atomic_xchg_u64(r15.wrapping_add(rsi.wrapping_mul(8)).wrapping_add(0x8158u64), (rax) as u64); rax = (tmp_xchg) as u64; fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 17; continue; } bb = 16; continue; }
            16 => { rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P5)) as u64; if t == 0x7001ec87f0 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 17; continue; }
            17 => { rdx = (r13) as u64; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P6)) as u64; if t == 0x7001ec87f0 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 18; continue; }
            18 => { rcx = (rax) as u64; let tmp_xchg: u64 = crate::simd_ops::atomic_xchg_u64(r15.wrapping_add(rdi.wrapping_mul(8)).wrapping_add(0x8170u64), (rcx) as u64); rcx = (tmp_xchg) as u64; bb = 14; continue; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001ec7730(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001ec9348; const P1: u64 = 0x7001ec9340; const P2: u64 = 0x7001ec75e0; const P3: u64 = 0x7001ec9170; const P4: u64 = 0x7001ec9098;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = (rcx) as u64;
    r9 = ((P0) as u64) as u64;
    rcx = (0u32) as u32 as u64;
    fl_l = 0; fl_r = 0;
    r8 = ((P1) as u64) as u64;
    rdx = ((P0) as u64) as u64;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rcx = (rbx) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); let s5 = crate::simd_ops::read_s5_tail(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x40) as *const u64); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P3)) as u64; if t == 0x7001ec87f0 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); }
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = core::ptr::read_unaligned(rsp as *const u64);
    rsp = rsp.wrapping_add(8);
    let s5 = crate::simd_ops::read_s5_tail(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x40) as *const u64); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P4)) as u64; if t == 0x7001ec87f0 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001ec7778(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001ec9360; const P1: u64 = 0x7001ec9358; const P2: u64 = 0x7001ec75e0; const P3: u64 = 0x7001ec9170; const P4: u64 = 0x7001ec9080;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = ((rcx as u32)) as u32 as u64;
    r9 = ((P0) as u64) as u64;
    rcx = (0x1u32) as u32 as u64;
    r8 = ((P1) as u64) as u64;
    rdx = ((P0) as u64) as u64;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    rcx = ((rbx as u32)) as u32 as u64;
    fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); let s5 = crate::simd_ops::read_s5_tail(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x40) as *const u64); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P3)) as u64; if t == 0x7001ec87f0 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); }
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = core::ptr::read_unaligned(rsp as *const u64);
    rsp = rsp.wrapping_add(8);
    let s5 = crate::simd_ops::read_s5_tail(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x40) as *const u64); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P4)) as u64; if t == 0x7001ec87f0 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001ec77c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001ec9370; const P1: u64 = 0x7001ec9368; const P2: u64 = 0x7001ec75e0; const P3: u64 = 0x7001ec9170; const P4: u64 = 0x7001ec9090;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = ((rcx as u32)) as u32 as u64;
    r9 = ((P0) as u64) as u64;
    rcx = (0x2u32) as u32 as u64;
    r8 = ((P1) as u64) as u64;
    rdx = ((P0) as u64) as u64;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    rcx = ((rbx as u32)) as u32 as u64;
    fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); let s5 = crate::simd_ops::read_s5_tail(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x40) as *const u64); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P3)) as u64; if t == 0x7001ec87f0 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); }
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = core::ptr::read_unaligned(rsp as *const u64);
    rsp = rsp.wrapping_add(8);
    let s5 = crate::simd_ops::read_s5_tail(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x40) as *const u64); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P4)) as u64; if t == 0x7001ec87f0 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001ec7808(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001ec9388; const P1: u64 = 0x7001ec9380; const P2: u64 = 0x7001ec75e0; const P3: u64 = 0x7001ec9170; const P4: u64 = 0x7001ec9088;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rbx) as u64);
    rsp = rsp.wrapping_sub(8);
    core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = (rdx) as u64;
    r9 = ((P0) as u64) as u64;
    rdi = ((rcx as u32)) as u32 as u64;
    rdx = ((P0) as u64) as u64;
    rcx = (0x3u32) as u32 as u64;
    r8 = ((P1) as u64) as u64;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    rdx = (rbx) as u64;
    rcx = ((rdi as u32)) as u32 as u64;
    fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P3)) as u64; if t == 0x7001ec87f0 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); } else { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P4)) as u64; if t == 0x7001ec87f0 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); }
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = core::ptr::read_unaligned(rsp as *const u64);
    rsp = rsp.wrapping_add(8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001ec785c(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001ec93a0; const P1: u64 = 0x7001ec9398; const P2: u64 = 0x7001ec75e0; const P3: u64 = 0x7001ec9170; const P4: u64 = 0x7001ec90a0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rbx) as u64);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rsi) as u64);
    rsp = rsp.wrapping_sub(8);
    core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rsi = ((r8 as u32)) as u32 as u64;
    r9 = ((P0) as u64) as u64;
    rbx = ((rdx as u32)) as u32 as u64;
    r8 = ((P1) as u64) as u64;
    rdi = (rcx) as u64;
    rdx = ((P0) as u64) as u64;
    rcx = (0x4u32) as u32 as u64;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    rdx = ((rbx as u32)) as u32 as u64;
    rcx = (rdi) as u64;
    fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { r8 = ((rsi as u32)) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P3)) as u64; if t == 0x7001ec87f0 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); } else { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P4)) as u64; if t == 0x7001ec87f0 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); }
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = core::ptr::read_unaligned(rsp as *const u64);
    rsp = rsp.wrapping_add(8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001ec78c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001ec78c8;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    r8 = (crate::simd_ops::ru64(rdx)) as u64;
    crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001ec78c8(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0x18; const P3: u64 = 0x20; const P4: u64 = 0x7001ec4000; const P5: u64 = 0x5208; const P6: u64 = 0x5218; const P7: u64 = 0xfffffffffffffffc;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 1>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = (rsp) as u64; crate::simd_ops::wu64(rax.wrapping_add(P0), (rbx) as u64); crate::simd_ops::wu64(rax.wrapping_add(P1), (rbp) as u64); crate::simd_ops::wu64(rax.wrapping_add(P2), (rsi) as u64); crate::simd_ops::wu64(rax.wrapping_add(P3), (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rbp = ((((rbp as u32)) | (0xffffffffu32))) as u32 as u64; rbx = (r8) as u64; fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(P1))) as u32 as u64; fl_r = (0x0u32) as u32 as u64; r10 = (rdx) as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 6; continue; } bb = 1; continue; }
            1 => { r9 = (((crate::simd_ops::ru32(rcx.wrapping_add(P1))) as i32 as i64) as u64) as u64; r14 = ((P4) as u64) as u64; rdi = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; rsi = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; r9 = ((r9).wrapping_add(rdi)) as u64; r8 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rdx = ((rbp as u32)) as u32 as u64; rcx = ((crate::simd_ops::ru8(r9)) as u32) as u32 as u64; rcx = ((((rcx as u32)) & (0xfu32))) as u32 as u64; rax = (((crate::simd_ops::ru8(rcx.wrapping_add(r14).wrapping_add(P5))) as i8 as i64) as u64) as u64; rcx = (rcx & !0xffu64) | (((crate::simd_ops::ru8(rcx.wrapping_add(r14).wrapping_add(P6))) as u8 as u64)); r9 = ((r9).wrapping_sub(rax)) as u64; r11 = (crate::simd_ops::ru32(r9.wrapping_add(P7))) as u32 as u64; r11 = (((r11 as u32)).wrapping_shr(((rcx as u32) & 31))) as u32 as u64; fl_l = ((r11 as u32)) as u32 as u64; fl_r = ((r11 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 6; continue; } bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(r10.wrapping_add(P1))) as u64; r10 = (crate::simd_ops::ru32(rax)) as u32 as u64; bb = 3; continue; }
            3 => { rcx = ((crate::simd_ops::ru8(r9)) as u32) as u32 as u64; rcx = ((((rcx as u32)) & (0xfu32))) as u32 as u64; rax = (((crate::simd_ops::ru8(rcx.wrapping_add(r14).wrapping_add(P5))) as i8 as i64) as u64) as u64; rcx = (rcx & !0xffu64) | (((crate::simd_ops::ru8(rcx.wrapping_add(r14).wrapping_add(P6))) as u8 as u64)); r9 = ((r9).wrapping_sub(rax)) as u64; rax = (crate::simd_ops::ru32(r9.wrapping_add(P7))) as u32 as u64; rax = (((rax as u32)).wrapping_shr(((rcx as u32) & 31))) as u32 as u64; rsi = (((rsi as u32)).wrapping_add((rax as u32))) as u32 as u64; rax = ((rsi as u32)) as u32 as u64; rax = ((rax).wrapping_add(r10)) as u64; rax = ((rax).wrapping_add(rdi)) as u64; fl_l = (rbx) as u64 as u64; fl_r = (rax) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 5; continue; } bb = 4; continue; }
            4 => { rcx = ((crate::simd_ops::ru8(r9)) as u32) as u32 as u64; r8 = (((r8 as u32)).wrapping_add(1)) as u32 as u64; rcx = ((((rcx as u32)) & (0xfu32))) as u32 as u64; rax = (((crate::simd_ops::ru8(rcx.wrapping_add(r14).wrapping_add(P5))) as i8 as i64) as u64) as u64; rcx = (rcx & !0xffu64) | (((crate::simd_ops::ru8(rcx.wrapping_add(r14).wrapping_add(P6))) as u8 as u64)); r9 = ((r9).wrapping_sub(rax)) as u64; rdx = (crate::simd_ops::ru32(r9.wrapping_add(P7))) as u32 as u64; rdx = (((rdx as u32)).wrapping_shr(((rcx as u32) & 31))) as u32 as u64; rdx = (((rdx as u32)).wrapping_sub(1)) as u32 as u64; fl_l = ((r8 as u32)) as u32 as u64; fl_r = ((r11 as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 3; continue; } bb = 5; continue; }
            5 => { fl_l = ((r8 as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; rdx = (if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { (rbp as u32) } else { (rdx as u32) }) as u32 as u64; rax = ((rdx as u32)) as u32 as u64; bb = 7; continue; }
            6 => { rax = ((rbp as u32)) as u32 as u64; bb = 7; continue; }
            7 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x10u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x18u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001ec7a0c(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001ec4000; const P1: u64 = 0x7001ec7aed;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 1>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = (rsp) as u64; crate::simd_ops::wu64(rax.wrapping_add(0x8u64), (rbx) as u64); crate::simd_ops::wu64(rax.wrapping_add(0x10u64), (rbp) as u64); crate::simd_ops::wu64(rax.wrapping_add(0x18u64), (rsi) as u64); crate::simd_ops::wu64(rax.wrapping_add(0x20u64), (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rbx = (rbx & !0xffu64) | (((crate::simd_ops::ru8(rcx)) as u8 as u64)); r10 = ((rcx.wrapping_add(0x1u64)) as u64) as u64; crate::simd_ops::wu8(rdx, ((rbx as u8)) as u8); rsi = ((r9 as u32)) as u32 as u64; r14 = ((P0) as u64) as u64; rbp = (r8) as u64; r11 = (rdx) as u64; rdi = (rcx) as u64; fl_l = ((rbx as u8)) as u8 as u64; fl_r = (0x4u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rcx = ((crate::simd_ops::ru8(r10)) as u32) as u32 as u64; rcx = ((((rcx as u32)) & (0xfu32))) as u32 as u64; rax = (((crate::simd_ops::ru8(rcx.wrapping_add(r14).wrapping_add(0x5208u64))) as i8 as i64) as u64) as u64; rcx = (rcx & !0xffu64) | (((crate::simd_ops::ru8(rcx.wrapping_add(r14).wrapping_add(0x5218u64))) as u8 as u64)); r10 = ((r10).wrapping_sub(rax)) as u64; rax = (crate::simd_ops::ru32(r10.wrapping_sub(0x4u64))) as u32 as u64; fl_l = (((rax as u32)).wrapping_shr(((rcx as u32) & 31))) as u32 as u64; rax = ((fl_l as u32)) as u32 as u64; crate::simd_ops::wu32(rdx.wrapping_add(0x4u64), ((rax as u32)) as u32); bb = 2; continue; }
            2 => { fl_l = ((rbx as u8)) as u8 as u64; fl_r = (0x8u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru32(r10)) as u32 as u64; fl_l = ((r10).wrapping_add(0x4u64)) as u64 as u64; fl_r = fl_l; r10 = ((fl_l as u64)) as u64; crate::simd_ops::wu32(rdx.wrapping_add(0x8u64), ((rax as u32)) as u32); bb = 4; continue; }
            4 => { fl_l = ((rbx as u8)) as u8 as u64; fl_r = (0x10u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru32(r10)) as u32 as u64; fl_l = ((r10).wrapping_add(0x4u64)) as u64 as u64; fl_r = fl_l; r10 = ((fl_l as u64)) as u64; crate::simd_ops::wu32(rdx.wrapping_add(0xcu64), ((rax as u32)) as u32); bb = 6; continue; }
            6 => { r9 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; r8 = ((r10.wrapping_add(0x4u64)) as u64) as u64; fl_l = (crate::simd_ops::ru8(rsp.wrapping_add(0x30u64))) as u8 as u64; fl_r = ((r9 as u8)) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 15; continue; } bb = 7; continue; }
            7 => { fl_l = ((rbx as u8)) as u8 as u64; fl_r = (0x2u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 15; continue; } bb = 8; continue; }
            8 => { crate::simd_ops::wu32(rdx.wrapping_add(0x10u64), ((r9 as u32)) as u32); fl_l = (crate::simd_ops::ru32(r10)) as u32 as u64; fl_r = ((r9 as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 14; continue; } bb = 9; continue; }
            9 => { rdx = (((crate::simd_ops::ru32(r10)) as i32 as i64) as u64) as u64; rdx = ((rdx).wrapping_add(rbp)) as u64; rcx = ((crate::simd_ops::ru8(rdx)) as u32) as u32 as u64; rcx = ((((rcx as u32)) & (0xfu32))) as u32 as u64; rax = (((crate::simd_ops::ru8(rcx.wrapping_add(r14).wrapping_add(0x5208u64))) as i8 as i64) as u64) as u64; rcx = (rcx & !0xffu64) | (((crate::simd_ops::ru8(rcx.wrapping_add(r14).wrapping_add(0x5218u64))) as u8 as u64)); rdx = ((rdx).wrapping_sub(rax)) as u64; r10 = (crate::simd_ops::ru32(rdx.wrapping_sub(0x4u64))) as u32 as u64; r10 = (((r10 as u32)).wrapping_shr(((rcx as u32) & 31))) as u32 as u64; fl_l = ((r10 as u32)) as u32 as u64; fl_r = ((r10 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 16; continue; } bb = 10; continue; }
            10 => { rax = (crate::simd_ops::ru32(rdx)) as u32 as u64; rcx = (crate::simd_ops::ru32(rdx.wrapping_add(0x4u64))) as u32 as u64; rdx = ((rdx.wrapping_add(0x8u64)) as u64) as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rsi as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 13; continue; } bb = 11; continue; }
            11 => { r9 = (((r9 as u32)).wrapping_add(1)) as u32 as u64; fl_l = ((r9 as u32)) as u32 as u64; fl_r = ((r10 as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 10; continue; } bb = 12; continue; }
            12 => { bb = 16; continue; }
            13 => { crate::simd_ops::wu32(r11.wrapping_add(0x10u64), ((rcx as u32)) as u32); bb = 16; continue; }
            14 => { rcx = (0x7u32) as u32 as u64; crate::lift_ops::trap(0xc0000409, P1, rcx, rsp); return rax; bb = 16; continue; }
            15 => { rax = (crate::simd_ops::ru32(r10)) as u32 as u64; crate::simd_ops::wu32(rdx.wrapping_add(0x10u64), ((rax as u32)) as u32); bb = 16; continue; }
            16 => { fl_l = ((rbx as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 18; continue; } bb = 17; continue; }
            17 => { rcx = ((crate::simd_ops::ru8(r8)) as u32) as u32 as u64; rcx = ((((rcx as u32)) & (0xfu32))) as u32 as u64; rdx = (((crate::simd_ops::ru8(rcx.wrapping_add(r14).wrapping_add(0x5208u64))) as i8 as i64) as u64) as u64; rcx = (rcx & !0xffu64) | (((crate::simd_ops::ru8(rcx.wrapping_add(r14).wrapping_add(0x5218u64))) as u8 as u64)); r8 = ((r8).wrapping_sub(rdx)) as u64; rdx = (crate::simd_ops::ru32(r8.wrapping_sub(0x4u64))) as u32 as u64; fl_l = (((rdx as u32)).wrapping_shr(((rcx as u32) & 31))) as u32 as u64; rdx = ((fl_l as u32)) as u32 as u64; crate::simd_ops::wu32(r11.wrapping_add(0x14u64), ((rdx as u32)) as u32); bb = 18; continue; }
            18 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x10u64))) as u64; fl_l = (r8) as u64 as u64; fl_r = (rdi) as u64 as u64; r8 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x18u64))) as u64; rax = (r8) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001ec7b40(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = (rax & !0xffu64) | (((crate::simd_ops::ru8(rdx)) as u8 as u64));
    fl_l = ((((rax as u8)) & (0x1u8))) as u8 as u64; fl_r = fl_l;
    rax = (rax & !0xffu64) | ((((fl_l as u8)) as u8 as u64));
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001ec7b48(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14; const P1: u64 = 0x7001ec6a94;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    fl_l = (crate::simd_ops::ru8(r8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64;
    rcx = (crate::simd_ops::ru64(rcx)) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (rcx) as u64);
    if !(((((fl_l as u8)) & ((fl_r as u8))) == 0)) { rax = (crate::simd_ops::ru32(r8.wrapping_add(P0))) as u32 as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(rcx))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (rcx) as u64); }
    r9 = ((((r9 as u32)) | (0xffffffffu32))) as u32 as u64;
    rcx = ((rsp.wrapping_add(0x30u64)) as u64) as u64;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001ec7b7c(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x14;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    rdx = (rcx) as u64;
    crate::simd_ops::wu64(r9, (rax) as u64);
    fl_l = (crate::simd_ops::ru8(r8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64;
    if !(((((fl_l as u8)) & ((fl_r as u8))) == 0)) { rcx = (crate::simd_ops::ru32(r8.wrapping_add(P0))) as u32 as u64; rax = (crate::simd_ops::ru64(rdx)) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(rax))) as u64; crate::simd_ops::wu64(r9, (rcx) as u64); }
    rax = (r9) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001ec7ba0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001ec7e2c; const P1: u64 = 0x7001ec4000;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut xmm6: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = (rsp) as u64; crate::simd_ops::wu64(rax.wrapping_add(0x8u64), (rbx) as u64); crate::simd_ops::wu64(rax.wrapping_add(0x10u64), (rbp) as u64); crate::simd_ops::wu64(rax.wrapping_add(0x18u64), (rsi) as u64); crate::simd_ops::wu64(rax.wrapping_add(0x20u64), (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = ((rsp).wrapping_sub(0x60u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rdx) as u64); rdi = (rdx) as u64; crate::simd_ops::wu128(rax.wrapping_sub(0x18u64), (xmm6) as u128); rbp = (rcx) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (rdx) as u64); rbx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((rbx as u32)) as u32); rdx = ((rax.wrapping_sub(0x28u64)) as u64) as u64; xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; rcx = (rdi) as u64; crate::simd_ops::wu128(rax.wrapping_sub(0x28u64), (xmm6) as u128); r14 = ((r8 as u32)) as u32 as u64; rsi = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; r9 = (crate::simd_ops::ru32(rdi)) as u32 as u64; rdx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; fl_l = ((r9 as u32)) as u32 as u64; fl_r = ((r9 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 6; continue; } bb = 1; continue; }
            1 => { r8 = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; r10 = ((P1) as u64) as u64; bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x18u64))) as u64; rcx = ((rbx as u32)) as u32 as u64; fl_l = ((r14 as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 5; continue; } bb = 3; continue; }
            3 => { rax = ((rax).wrapping_shr(32u32)) as u64; fl_l = ((r14 as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) > (((fl_r as u32)) as i32)) { bb = 5; continue; } bb = 4; continue; }
            4 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; rbx = ((rdx as u32)) as u32 as u64; rsi = ((rdx as u32)) as u32 as u64; rbx = (if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { (rcx as u32) } else { (rbx as u32) }) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((rbx as u32)) as u32); xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; bb = 5; continue; }
            5 => { rcx = ((crate::simd_ops::ru8(r8)) as u32) as u32 as u64; rdx = (((rdx as u32)).wrapping_add(1)) as u32 as u64; rcx = ((((rcx as u32)) & (0xfu32))) as u32 as u64; rax = (((crate::simd_ops::ru8(rcx.wrapping_add(r10).wrapping_add(0x5208u64))) as i8 as i64) as u64) as u64; rcx = (rcx & !0xffu64) | (((crate::simd_ops::ru8(rcx.wrapping_add(r10).wrapping_add(0x5218u64))) as u8 as u64)); r8 = ((r8).wrapping_sub(rax)) as u64; rax = (crate::simd_ops::ru32(r8.wrapping_sub(0x4u64))) as u32 as u64; rax = (((rax as u32)).wrapping_shr(((rcx as u32) & 31))) as u32 as u64; crate::simd_ops::wu64(rdi.wrapping_add(0x8u64), (r8) as u64); crate::simd_ops::wu32(rdi.wrapping_add(0x18u64), ((rax as u32)) as u32); rcx = ((crate::simd_ops::ru8(r8)) as u32) as u32 as u64; rcx = ((((rcx as u32)) & (0xfu32))) as u32 as u64; rax = (((crate::simd_ops::ru8(rcx.wrapping_add(r10).wrapping_add(0x5208u64))) as i8 as i64) as u64) as u64; rcx = (rcx & !0xffu64) | (((crate::simd_ops::ru8(rcx.wrapping_add(r10).wrapping_add(0x5218u64))) as u8 as u64)); r8 = ((r8).wrapping_sub(rax)) as u64; rax = (crate::simd_ops::ru32(r8.wrapping_sub(0x4u64))) as u32 as u64; rax = (((rax as u32)).wrapping_shr(((rcx as u32) & 31))) as u32 as u64; crate::simd_ops::wu64(rdi.wrapping_add(0x8u64), (r8) as u64); crate::simd_ops::wu32(rdi.wrapping_add(0x1cu64), ((rax as u32)) as u32); rcx = ((crate::simd_ops::ru8(r8)) as u32) as u32 as u64; rcx = ((((rcx as u32)) & (0xfu32))) as u32 as u64; rax = (((crate::simd_ops::ru8(rcx.wrapping_add(r10).wrapping_add(0x5208u64))) as i8 as i64) as u64) as u64; rcx = (rcx & !0xffu64) | (((crate::simd_ops::ru8(rcx.wrapping_add(r10).wrapping_add(0x5218u64))) as u8 as u64)); r8 = ((r8).wrapping_sub(rax)) as u64; rax = (crate::simd_ops::ru32(r8.wrapping_sub(0x4u64))) as u32 as u64; rax = (((rax as u32)).wrapping_shr(((rcx as u32) & 31))) as u32 as u64; crate::simd_ops::wu64(rdi.wrapping_add(0x8u64), (r8) as u64); crate::simd_ops::wu32(rdi.wrapping_add(0x20u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(r8)) as u32 as u64; r8 = ((r8).wrapping_add(0x4u64)) as u64; crate::simd_ops::wu64(rdi.wrapping_add(0x8u64), (r8) as u64); crate::simd_ops::wu32(rdi.wrapping_add(0x24u64), ((rax as u32)) as u32); fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((r9 as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 2; continue; } bb = 6; continue; }
            6 => { rsi = (((rsi as u32)).wrapping_add(1)) as u32 as u64; crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm6) as u128); rdx = ((rsp.wrapping_add(0x40u64)) as u64) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x38u64), ((rsi as u32)) as u32); rcx = (rdi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x30u64))) as u128; r11 = ((rsp.wrapping_add(0x60u64)) as u64) as u64; rax = (rbp) as u64; rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x10u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x20u64))) as u64; rdi = (crate::simd_ops::ru64(r11.wrapping_add(0x28u64))) as u64; crate::simd_ops::wu128(rbp, (xmm6) as u128); xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x50u64))) as u128; crate::simd_ops::wu128(rbp.wrapping_add(0x10u64), (xmm0) as u128); rbp = (crate::simd_ops::ru64(r11.wrapping_add(0x18u64))) as u64; rsp = (r11) as u64; r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001ec7e2c(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x7001ec4000;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { r8 = (crate::simd_ops::ru64(rcx.wrapping_add(0x10u64))) as u64; r11 = ((P0) as u64) as u64; crate::simd_ops::wu64(rcx.wrapping_add(0x8u64), (r8) as u64); r9 = (rcx) as u64; rcx = ((crate::simd_ops::ru8(r8)) as u32) as u32 as u64; rcx = ((((rcx as u32)) & (0xfu32))) as u32 as u64; rax = (((crate::simd_ops::ru8(rcx.wrapping_add(r11).wrapping_add(0x5208u64))) as i8 as i64) as u64) as u64; rcx = (rcx & !0xffu64) | (((crate::simd_ops::ru8(rcx.wrapping_add(r11).wrapping_add(0x5218u64))) as u8 as u64)); r8 = ((r8).wrapping_sub(rax)) as u64; rax = (crate::simd_ops::ru32(r8.wrapping_sub(0x4u64))) as u32 as u64; rax = (((rax as u32)).wrapping_shr(((rcx as u32) & 31))) as u32 as u64; crate::simd_ops::wu64(r9.wrapping_add(0x8u64), (r8) as u64); crate::simd_ops::wu32(r9.wrapping_add(0x18u64), ((rax as u32)) as u32); rcx = ((crate::simd_ops::ru8(r8)) as u32) as u32 as u64; rcx = ((((rcx as u32)) & (0xfu32))) as u32 as u64; rax = (((crate::simd_ops::ru8(rcx.wrapping_add(r11).wrapping_add(0x5208u64))) as i8 as i64) as u64) as u64; rcx = (rcx & !0xffu64) | (((crate::simd_ops::ru8(rcx.wrapping_add(r11).wrapping_add(0x5218u64))) as u8 as u64)); r8 = ((r8).wrapping_sub(rax)) as u64; rax = (crate::simd_ops::ru32(r8.wrapping_sub(0x4u64))) as u32 as u64; crate::simd_ops::wu64(r9.wrapping_add(0x8u64), (r8) as u64); rax = (((rax as u32)).wrapping_shr(((rcx as u32) & 31))) as u32 as u64; crate::simd_ops::wu32(r9.wrapping_add(0x1cu64), ((rax as u32)) as u32); rcx = ((crate::simd_ops::ru8(r8)) as u32) as u32 as u64; rcx = ((((rcx as u32)) & (0xfu32))) as u32 as u64; rax = (((crate::simd_ops::ru8(rcx.wrapping_add(r11).wrapping_add(0x5208u64))) as i8 as i64) as u64) as u64; rcx = (rcx & !0xffu64) | (((crate::simd_ops::ru8(rcx.wrapping_add(r11).wrapping_add(0x5218u64))) as u8 as u64)); r8 = ((r8).wrapping_sub(rax)) as u64; rax = (crate::simd_ops::ru32(r8.wrapping_sub(0x4u64))) as u32 as u64; crate::simd_ops::wu64(r9.wrapping_add(0x8u64), (r8) as u64); rax = (((rax as u32)).wrapping_shr(((rcx as u32) & 31))) as u32 as u64; crate::simd_ops::wu32(r9.wrapping_add(0x20u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(r8)) as u32 as u64; r8 = ((r8).wrapping_add(0x4u64)) as u64; fl_l = (crate::simd_ops::ru32(rdx.wrapping_add(0x8u64))) as u32 as u64; fl_r = (0x0u32) as u32 as u64; crate::simd_ops::wu64(r9.wrapping_add(0x8u64), (r8) as u64); crate::simd_ops::wu32(r9.wrapping_add(0x24u64), ((rax as u32)) as u32); if (((fl_l as u32)) == ((fl_r as u32))) { bb = 3; continue; } bb = 1; continue; }
            1 => { r10 = (crate::simd_ops::ru32(rdx.wrapping_add(0x8u64))) as u32 as u64; bb = 2; continue; }
            2 => { rcx = ((crate::simd_ops::ru8(r8)) as u32) as u32 as u64; rcx = ((((rcx as u32)) & (0xfu32))) as u32 as u64; rax = (((crate::simd_ops::ru8(rcx.wrapping_add(r11).wrapping_add(0x5208u64))) as i8 as i64) as u64) as u64; rcx = (rcx & !0xffu64) | (((crate::simd_ops::ru8(rcx.wrapping_add(r11).wrapping_add(0x5218u64))) as u8 as u64)); r8 = ((r8).wrapping_sub(rax)) as u64; rax = (crate::simd_ops::ru32(r8.wrapping_sub(0x4u64))) as u32 as u64; crate::simd_ops::wu64(r9.wrapping_add(0x8u64), (r8) as u64); rax = (((rax as u32)).wrapping_shr(((rcx as u32) & 31))) as u32 as u64; crate::simd_ops::wu32(r9.wrapping_add(0x18u64), ((rax as u32)) as u32); rcx = ((crate::simd_ops::ru8(r8)) as u32) as u32 as u64; rcx = ((((rcx as u32)) & (0xfu32))) as u32 as u64; rax = (((crate::simd_ops::ru8(rcx.wrapping_add(r11).wrapping_add(0x5208u64))) as i8 as i64) as u64) as u64; rcx = (rcx & !0xffu64) | (((crate::simd_ops::ru8(rcx.wrapping_add(r11).wrapping_add(0x5218u64))) as u8 as u64)); r8 = ((r8).wrapping_sub(rax)) as u64; rax = (crate::simd_ops::ru32(r8.wrapping_sub(0x4u64))) as u32 as u64; crate::simd_ops::wu64(r9.wrapping_add(0x8u64), (r8) as u64); rax = (((rax as u32)).wrapping_shr(((rcx as u32) & 31))) as u32 as u64; crate::simd_ops::wu32(r9.wrapping_add(0x1cu64), ((rax as u32)) as u32); rcx = ((crate::simd_ops::ru8(r8)) as u32) as u32 as u64; rcx = ((((rcx as u32)) & (0xfu32))) as u32 as u64; rax = (((crate::simd_ops::ru8(rcx.wrapping_add(r11).wrapping_add(0x5208u64))) as i8 as i64) as u64) as u64; rcx = (rcx & !0xffu64) | (((crate::simd_ops::ru8(rcx.wrapping_add(r11).wrapping_add(0x5218u64))) as u8 as u64)); r8 = ((r8).wrapping_sub(rax)) as u64; rax = (crate::simd_ops::ru32(r8.wrapping_sub(0x4u64))) as u32 as u64; rdx = ((r8.wrapping_add(0x4u64)) as u64) as u64; crate::simd_ops::wu64(r9.wrapping_add(0x8u64), (r8) as u64); rax = (((rax as u32)).wrapping_shr(((rcx as u32) & 31))) as u32 as u64; crate::simd_ops::wu32(r9.wrapping_add(0x20u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(r8)) as u32 as u64; crate::simd_ops::wu64(r9.wrapping_add(0x8u64), (rdx) as u64); crate::simd_ops::wu32(r9.wrapping_add(0x24u64), ((rax as u32)) as u32); rcx = ((crate::simd_ops::ru8(rdx)) as u32) as u32 as u64; rcx = ((((rcx as u32)) & (0xfu32))) as u32 as u64; rax = (((crate::simd_ops::ru8(rcx.wrapping_add(r11).wrapping_add(0x5208u64))) as i8 as i64) as u64) as u64; rcx = (rcx & !0xffu64) | (((crate::simd_ops::ru8(rcx.wrapping_add(r11).wrapping_add(0x5218u64))) as u8 as u64)); rdx = ((rdx).wrapping_sub(rax)) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_sub(0x4u64))) as u32 as u64; rax = (((rax as u32)).wrapping_shr(((rcx as u32) & 31))) as u32 as u64; crate::simd_ops::wu64(r9.wrapping_add(0x8u64), (rdx) as u64); crate::simd_ops::wu32(r9.wrapping_add(0x18u64), ((rax as u32)) as u32); rcx = ((crate::simd_ops::ru8(rdx)) as u32) as u32 as u64; rcx = ((((rcx as u32)) & (0xfu32))) as u32 as u64; rax = (((crate::simd_ops::ru8(rcx.wrapping_add(r11).wrapping_add(0x5208u64))) as i8 as i64) as u64) as u64; rcx = (rcx & !0xffu64) | (((crate::simd_ops::ru8(rcx.wrapping_add(r11).wrapping_add(0x5218u64))) as u8 as u64)); rdx = ((rdx).wrapping_sub(rax)) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_sub(0x4u64))) as u32 as u64; rax = (((rax as u32)).wrapping_shr(((rcx as u32) & 31))) as u32 as u64; crate::simd_ops::wu64(r9.wrapping_add(0x8u64), (rdx) as u64); crate::simd_ops::wu32(r9.wrapping_add(0x1cu64), ((rax as u32)) as u32); rcx = ((crate::simd_ops::ru8(rdx)) as u32) as u32 as u64; rcx = ((((rcx as u32)) & (0xfu32))) as u32 as u64; rax = (((crate::simd_ops::ru8(rcx.wrapping_add(r11).wrapping_add(0x5208u64))) as i8 as i64) as u64) as u64; rcx = (rcx & !0xffu64) | (((crate::simd_ops::ru8(rcx.wrapping_add(r11).wrapping_add(0x5218u64))) as u8 as u64)); rdx = ((rdx).wrapping_sub(rax)) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_sub(0x4u64))) as u32 as u64; r8 = ((rdx.wrapping_add(0x4u64)) as u64) as u64; rax = (((rax as u32)).wrapping_shr(((rcx as u32) & 31))) as u32 as u64; crate::simd_ops::wu64(r9.wrapping_add(0x8u64), (rdx) as u64); crate::simd_ops::wu32(r9.wrapping_add(0x20u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdx)) as u32 as u64; crate::simd_ops::wu64(r9.wrapping_add(0x8u64), (r8) as u64); crate::simd_ops::wu32(r9.wrapping_add(0x24u64), ((rax as u32)) as u32); fl_l = (r10) as u64 as u64; fl_r = (0x1u64) as u64 as u64; r10 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 3; continue; }
            3 => { return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001ec7ff0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001ec7524; const P1: u64 = 0x58; const P2: u64 = 0x8;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = (rcx) as u64;
    crate::simd_ops::wu64(rcx, (rdx) as u64);
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    fl_l = (rbx) as u64 as u64; fl_r = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64 as u64;
    if !((((fl_l as u64)) >= ((fl_r as u64)))) { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rcx = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64; } else { rcx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; }
    crate::simd_ops::wu64(rbx.wrapping_add(P2), (rcx) as u64);
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    crate::simd_ops::wu64(rax.wrapping_add(P1), (rbx) as u64);
    rax = (rbx) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = core::ptr::read_unaligned(rsp as *const u64);
    rsp = rsp.wrapping_add(8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001ec802c(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001ec7524; const P1: u64 = 0x58; const P2: u64 = 0x8; const P3: u64 = 0x7001ec9138; const P4: u64 = 0x7001ec8069;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rbx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = (rcx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); fl_l = (rdi) as u64 as u64; fl_r = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 4; continue; } bb = 1; continue; }
            1 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rdx = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64; fl_l = (rdx) as u64 as u64; fl_r = (rdx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 2; continue; }
            2 => { rbx = (crate::simd_ops::ru64(rdx.wrapping_add(P2))) as u64; fl_l = (rdi) as u64 as u64; fl_r = (rdx) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 5; continue; } bb = 3; continue; }
            3 => { rdx = (rbx) as u64; fl_l = (rbx) as u64 as u64; fl_r = (rbx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 2; continue; } bb = 4; continue; }
            4 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P3)) as u64; if t == 0x7001ec87f0 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); crate::lift_ops::trap(0x80000003, P4, rcx, rsp); return rax; }
            5 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); crate::simd_ops::wu64(rax.wrapping_add(P1), (rbx) as u64); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001ec81a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001ec7330; const P1: u64 = 0x7001ec731c; const P2: u64 = 0x7001ec72ec; const P3: u64 = 0x7001ecc200; const P4: u64 = 0x7001ec7340;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 9; continue; } bb = 1; continue; }
            1 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = (0x1u32) as u32 as u64; rdx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 7; continue; } bb = 2; continue; }
            2 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = (0x1u32) as u32 as u64; rdx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 6; continue; } bb = 3; continue; }
            3 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = (0x1u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 11; continue; } bb = 4; continue; }
            4 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 5; continue; }
            5 => { rbx = (((rax as u8)) as u32) as u32 as u64; bb = 11; continue; }
            6 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 5; continue; }
            7 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 11; continue; } bb = 8; continue; }
            8 => { fl_l = (crate::simd_ops::lk_add_u32(P3, (1) as u32)) as u32 as u64; rbx = (0x1u32) as u32 as u64; bb = 11; continue; }
            9 => { rax = (crate::simd_ops::ru32(P3)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 11; continue; } bb = 10; continue; }
            10 => { rax = (((rax as u32)).wrapping_sub(1)) as u32 as u64; fl_l = (r8) as u64 as u64; fl_r = (r8) as u64 as u64; crate::simd_ops::wu32(P3, ((rax as u32)) as u32); rcx = (rcx & !0xffu64) | (((if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { 1u8 } else { 0u8 }) as u8 as u64)); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; rbx = (rbx & !0xffu64) | (((if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { 1u8 } else { 0u8 }) as u8 as u64)); bb = 11; continue; }
            11 => { rax = ((rbx as u32)) as u32 as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001ec8210(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001ec856c; const P1: u64 = 0x7001ec81a0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rbx) as u64);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rsi) as u64);
    rsp = rsp.wrapping_sub(8);
    core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rdi = (r8) as u64;
    rbx = ((rdx as u32)) as u32 as u64;
    rsi = (rcx) as u64;
    fl_l = ((rdx as u32)) as u32 as u64; fl_r = (0x1u32) as u32 as u64;
    if !((((fl_l as u32)) != ((fl_r as u32)))) { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); }
    r8 = (rdi) as u64;
    rdx = ((rbx as u32)) as u32 as u64;
    rcx = (rsi) as u64;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = core::ptr::read_unaligned(rsp as *const u64);
    rsp = rsp.wrapping_add(8);
    let s5 = crate::simd_ops::read_s5_tail(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x40) as *const u64); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001ec8250(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001eca240; const P1: u64 = 0x7001ec87a5;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rax = ((P0) as u64) as u64;
    rbx = (rcx) as u64;
    crate::simd_ops::wu64(rcx, (rax) as u64);
    fl_l = ((rdx as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64;
    if !(((((fl_l as u8)) & ((fl_r as u8))) == 0)) { rdx = (0x18u32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); }
    rax = (rbx) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = core::ptr::read_unaligned(rsp as *const u64);
    rsp = rsp.wrapping_add(8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001ec8284(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x38; const P1: u64 = 0x7001ec82a4;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    r8 = (crate::simd_ops::ru64(r9.wrapping_add(P0))) as u64;
    rcx = (rdx) as u64;
    rdx = (r9) as u64;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    rax = (0x1u32) as u32 as u64;
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001ec82a4(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x4; const P2: u64 = 0x10; const P3: u64 = 0x3; const P4: u64 = 0x7001ec83a0;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 1>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64);
    r11 = (crate::simd_ops::ru32(r8)) as u32 as u64;
    rbx = (rdx) as u64;
    r11 = ((((r11 as u32)) & (0xfffffff8u32))) as u32 as u64;
    r9 = (rcx) as u64;
    fl_l = (crate::simd_ops::ru8(r8)) as u8 as u64; fl_r = (0x4u8) as u8 as u64;
    r10 = (rcx) as u64;
    if !(((((fl_l as u8)) & ((fl_r as u8))) == 0)) { rax = (crate::simd_ops::ru32(r8.wrapping_add(P0))) as u32 as u64; r10 = (((crate::simd_ops::ru32(r8.wrapping_add(P1))) as i32 as i64) as u64) as u64; rax = (0u32.wrapping_sub((rax as u32))) as u32 as u64; r10 = ((r10).wrapping_add(rcx)) as u64; rcx = ((((rax as u32)) as i32 as i64) as u64) as u64; fl_l = (((r10) & (rcx))) as u64 as u64; fl_r = fl_l; r10 = ((fl_l as u64)) as u64; }
    rax = ((((r11 as u32)) as i32 as i64) as u64) as u64;
    rdx = (crate::simd_ops::ru64(rax.wrapping_add(r10))) as u64;
    rax = (crate::simd_ops::ru64(rbx.wrapping_add(P2))) as u64;
    rcx = (crate::simd_ops::ru32(rax.wrapping_add(P0))) as u32 as u64;
    rax = (crate::simd_ops::ru64(rbx.wrapping_add(P0))) as u64;
    fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(rax).wrapping_add(P3))) as u8 as u64; fl_r = (0xfu8) as u8 as u64;
    if !(((((fl_l as u8)) & ((fl_r as u8))) == 0)) { rax = ((crate::simd_ops::ru8(rcx.wrapping_add(rax).wrapping_add(P3))) as u32) as u32 as u64; rax = ((((rax as u32)) & (0xfffffff0u32))) as u32 as u64; fl_l = ((r9).wrapping_add(rax)) as u64 as u64; fl_r = fl_l; r9 = ((fl_l as u64)) as u64; }
    fl_l = (((r9) ^ (rdx))) as u64 as u64; fl_r = fl_l;
    r9 = ((fl_l as u64)) as u64;
    rcx = (r9) as u64;
    rbx = core::ptr::read_unaligned(rsp as *const u64);
    rsp = rsp.wrapping_add(8);
    let s5 = crate::simd_ops::read_s5_tail(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x40) as *const u64); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001ec8300(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0x18; const P3: u64 = 0x20; const P4: u64 = 0x38; const P5: u64 = 0x4; const P6: u64 = 0x7001ec82a4; const P7: u64 = 0x7001ec87ab;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = (rsp) as u64; crate::simd_ops::wu64(rax.wrapping_add(P0), (rbx) as u64); crate::simd_ops::wu64(rax.wrapping_add(P1), (rbp) as u64); crate::simd_ops::wu64(rax.wrapping_add(P2), (rsi) as u64); crate::simd_ops::wu64(rax.wrapping_add(P3), (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; r10 = (crate::simd_ops::ru64(r9.wrapping_add(P4))) as u64; rsi = (rdx) as u64; r14 = (r8) as u64; rbp = (rcx) as u64; rdx = (r9) as u64; rcx = (rsi) as u64; rdi = (r9) as u64; rbx = (crate::simd_ops::ru32(r10)) as u32 as u64; rbx = ((rbx).wrapping_shl(4u32)) as u64; rbx = ((rbx).wrapping_add(r10)) as u64; r8 = ((rbx.wrapping_add(P5)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x4u64))) as u32 as u64; rax = (rax & !0xffu64) | ((((((rax as u8)) & (0x66u8))) as u8 as u64)); fl_l = 0; fl_r = ((rax as u8)) as u8 as u64; rax = (rax & !0xffu64) | (((0u8.wrapping_sub(fl_r as u8)) as u8 as u64)); rax = (0x1u32) as u32 as u64; rdx = (if (((fl_l as u8)) < ((fl_r as u8))) { 0xffffffffu32 } else { 0u32 }) as u32 as u64; rdx = (0u32.wrapping_sub((rdx as u32))) as u32 as u64; rdx = (((rdx as u32)).wrapping_add((rax as u32))) as u32 as u64; fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(P5))) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { r9 = (rdi) as u64; r8 = (r14) as u64; rdx = (rsi) as u64; rcx = (rbp) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P7) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 2; continue; }
            2 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001ec83a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001ecc040; const P1: u64 = 0x7001ec8660;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    fl_l = (rcx) as u64 as u64; fl_r = (crate::simd_ops::ru64(P0)) as u64 as u64;
    if (((fl_l as u64)) != ((fl_r as u64))) { crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
    rcx = ((rcx).rotate_left(16u32)) as u64;
    fl_l = ((rcx as u16)) as u16 as u64; fl_r = (0xffffu16) as u16 as u64;
    if !(((((fl_l as u16)) & ((fl_r as u16))) != 0)) { return rax; }
    rcx = ((rcx).rotate_right(16u32)) as u64;
    crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001ec83c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001ecc098; const P1: u64 = 0x7001ecc090; const P2: u64 = 0x7001ecc204; const P3: u64 = 0x7001ecc088; const P4: u64 = 0x7001ecc08c;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 3>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = ((rsp).wrapping_sub(0x10u64)) as u64; rax = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rcx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; let (cp_a, cp_b, cp_c, cp_d) = crate::simd_ops::cpuid_x64(rax as u32, rcx as u32); rax = cp_a; rbx = cp_b; rcx = cp_c; rdx = cp_d; r8 = ((rcx as u32)) as u32 as u64; r11 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; r10 = ((rdx as u32)) as u32 as u64; r8 = ((((r8 as u32)) ^ (0x6c65746eu32))) as u32 as u64; r10 = ((((r10 as u32)) ^ (0x49656e69u32))) as u32 as u64; r9 = ((rbx as u32)) as u32 as u64; rsi = ((rax as u32)) as u32 as u64; rcx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rax = ((r11.wrapping_add(0x1u64)) as u32) as u32 as u64; r10 = ((((r10 as u32)) | ((r8 as u32)))) as u32 as u64; let (cp_a, cp_b, cp_c, cp_d) = crate::simd_ops::cpuid_x64(rax as u32, rcx as u32); rax = cp_a; rbx = cp_b; rcx = cp_c; rdx = cp_d; r9 = ((((r9 as u32)) ^ (0x756e6547u32))) as u32 as u64; crate::simd_ops::wu32(rsp, ((rax as u32)) as u32); fl_l = ((((r10 as u32)) | ((r9 as u32)))) as u32 as u64; fl_r = fl_l; r10 = ((fl_l as u32)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x4u64), ((rbx as u32)) as u32); rdi = ((rcx as u32)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x8u64), ((rcx as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0xcu64), ((rdx as u32)) as u32); if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 7; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::lk_or_u64(P0, (0xffffffffffffffffu64) as u64); rax = ((((rax as u32)) & (0xfff3ff0u32))) as u32 as u64; crate::simd_ops::wu64(P1, (0x8000u64) as u64); fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x106c0u32) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 6; continue; } bb = 2; continue; }
            2 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x20660u32) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 6; continue; } bb = 3; continue; }
            3 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x20670u32) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 6; continue; } bb = 4; continue; }
            4 => { rax = (((rax as u32)).wrapping_add(0xfffcf9b0u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x20u32) as u32 as u64; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 7; continue; } bb = 5; continue; }
            5 => { rcx = (0x100010001u64) as u64; fl_l = (((rcx) >> (((rax) as u32 & 63))) & 1) as u64; if ((fl_l as u64) == 0) { bb = 7; continue; } bb = 6; continue; }
            6 => { r8 = (crate::simd_ops::ru32(P2)) as u32 as u64; fl_l = ((((r8 as u32)) | (0x1u32))) as u32 as u64; fl_r = fl_l; r8 = ((fl_l as u32)) as u32 as u64; crate::simd_ops::wu32(P2, ((r8 as u32)) as u32); bb = 8; continue; }
            7 => { r8 = (crate::simd_ops::ru32(P2)) as u32 as u64; bb = 8; continue; }
            8 => { rax = (0x7u32) as u32 as u64; r9 = ((rax.wrapping_sub(0x5u64)) as u32) as u32 as u64; fl_l = ((rsi as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 11; continue; } bb = 9; continue; }
            9 => { rcx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; let (cp_a, cp_b, cp_c, cp_d) = crate::simd_ops::cpuid_x64(rax as u32, rcx as u32); rax = cp_a; rbx = cp_b; rcx = cp_c; rdx = cp_d; crate::simd_ops::wu32(rsp, ((rax as u32)) as u32); r11 = ((rbx as u32)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x4u64), ((rbx as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x8u64), ((rcx as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0xcu64), ((rdx as u32)) as u32); fl_l = ((((rbx as u32)) >> (9u32)) & 1) as u64; if ((fl_l as u32) == 0) { bb = 11; continue; } bb = 10; continue; }
            10 => { fl_l = ((((r8 as u32)) | ((r9 as u32)))) as u32 as u64; fl_r = fl_l; r8 = ((fl_l as u32)) as u32 as u64; crate::simd_ops::wu32(P2, ((r8 as u32)) as u32); bb = 11; continue; }
            11 => { crate::simd_ops::wu32(P3, (0x1u32) as u32); crate::simd_ops::wu32(P4, ((r9 as u32)) as u32); fl_l = ((((rdi as u32)) >> (20u32)) & 1) as u64; if ((fl_l as u32) == 0) { bb = 19; continue; } bb = 12; continue; }
            12 => { crate::simd_ops::wu32(P3, ((r9 as u32)) as u32); rbx = (0x6u32) as u32 as u64; crate::simd_ops::wu32(P4, ((rbx as u32)) as u32); fl_l = ((((rdi as u32)) >> (27u32)) & 1) as u64; if ((fl_l as u32) == 0) { bb = 19; continue; } bb = 13; continue; }
            13 => { fl_l = ((((rdi as u32)) >> (28u32)) & 1) as u64; if ((fl_l as u32) == 0) { bb = 19; continue; } bb = 14; continue; }
            14 => { rcx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rax = 7; rdx = 0; rdx = ((rdx).wrapping_shl(32u32)) as u64; rdx = (((rdx) | (rax))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rdx) as u64); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; rax = (rax & !0xffu64) | ((((((rax as u8)) & ((rbx as u8)))) as u8 as u64)); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rbx as u8)) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 19; continue; } bb = 15; continue; }
            15 => { rax = (crate::simd_ops::ru32(P4)) as u32 as u64; rax = ((((rax as u32)) | (0x8u32))) as u32 as u64; crate::simd_ops::wu32(P3, (0x3u32) as u32); crate::simd_ops::wu32(P4, ((rax as u32)) as u32); fl_l = ((r11 as u8)) as u8 as u64; fl_r = (0x20u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 19; continue; } bb = 16; continue; }
            16 => { rax = ((((rax as u32)) | (0x20u32))) as u32 as u64; crate::simd_ops::wu32(P3, (0x5u32) as u32); crate::simd_ops::wu32(P4, ((rax as u32)) as u32); rax = (0xd0030000u32) as u32 as u64; r11 = ((((r11 as u32)) & ((rax as u32)))) as u32 as u64; fl_l = ((r11 as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 19; continue; } bb = 17; continue; }
            17 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; rax = (rax & !0xffu64) | ((((((rax as u8)) & (0xe0u8))) as u8 as u64)); fl_l = ((rax as u8)) as u8 as u64; fl_r = (0xe0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 19; continue; } bb = 18; continue; }
            18 => { fl_l = (crate::simd_ops::lk_or_u32(P4, (0x40u32) as u32)) as u32 as u64; fl_r = fl_l; crate::simd_ops::wu32(P3, ((rbx as u32)) as u32); bb = 19; continue; }
            19 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; rax = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x10u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001ec856c(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001ecc040; const P1: u64 = 0x7001ec9038; const P2: u64 = 0x7001ec9040; const P3: u64 = 0x7001ec9048; const P4: u64 = 0x7001ec9050; const P5: u64 = 0x7001ecc080;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (rbx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64); rbp = (rsp) as u64; rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rax = (crate::simd_ops::ru64(P0)) as u64; rbx = (0x2b992ddfa232u64) as u64; fl_l = (rax) as u64 as u64; fl_r = (rbx) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::lk_and_u64(rbp.wrapping_add(0x10u64), (0x0u64) as u64); rcx = ((rbp.wrapping_add(0x10u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P1)) as u64; if t == 0x7001ec87f0 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x10u64))) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x10u64), (rax) as u64); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P2)) as u64; if t == 0x7001ec87f0 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rax = ((rax as u32)) as u32 as u64; crate::simd_ops::lk_xor_u64(rbp.wrapping_sub(0x10u64), (rax) as u64); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P3)) as u64; if t == 0x7001ec87f0 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rax = ((rax as u32)) as u32 as u64; rcx = ((rbp.wrapping_add(0x18u64)) as u64) as u64; crate::simd_ops::lk_xor_u64(rbp.wrapping_sub(0x10u64), (rax) as u64); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P4)) as u64; if t == 0x7001ec87f0 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x18u64))) as u32 as u64; rcx = ((rbp.wrapping_sub(0x10u64)) as u64) as u64; rax = ((rax).wrapping_shl(32u32)) as u64; rax = (((rax) ^ (crate::simd_ops::ru64(rbp.wrapping_add(0x18u64))))) as u64; rax = (((rax) ^ (crate::simd_ops::ru64(rbp.wrapping_sub(0x10u64))))) as u64; rax = (((rax) ^ (rcx))) as u64; rcx = (0xffffffffffffu64) as u64; rax = (((rax) & (rcx))) as u64; rcx = (0x2b992ddfa233u64) as u64; fl_l = (rax) as u64 as u64; fl_r = (rbx) as u64 as u64; rax = (if (((fl_l as u64)) == ((fl_r as u64))) { rcx } else { rax }) as u64; crate::simd_ops::wu64(P0, (rax) as u64); bb = 2; continue; }
            2 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rax = (!(rax)) as u64; crate::simd_ops::wu64(P5, (rax) as u64); rsp = ((rsp).wrapping_add(0x30u64)) as u64; rbp = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001ec8628(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001ec9018; const P1: u64 = 0x7001ec9020; const P2: u64 = 0x7001ec9010; const P3: u64 = 0x7001ec9008;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = (rcx) as u64;
    rcx = (0u32) as u32 as u64;
    fl_l = 0; fl_r = 0;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P0)) as u64; if t == 0x7001ec87f0 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    rcx = (rbx) as u64;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P1)) as u64; if t == 0x7001ec87f0 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P2)) as u64; if t == 0x7001ec87f0 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    rcx = (rax) as u64;
    rdx = (0xc0000409u32) as u32 as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = core::ptr::read_unaligned(rsp as *const u64);
    rsp = rsp.wrapping_add(8);
    let s5 = crate::simd_ops::read_s5_tail(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x40) as *const u64); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P3)) as u64; if t == 0x7001ec87f0 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001ec8660(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001ec9000; const P1: u64 = 0x7001ec867d; const P2: u64 = 0x7001ecc2b0; const P3: u64 = 0x7001ec8734; const P4: u64 = 0x7001ecc3a8; const P5: u64 = 0x7001ecc348; const P6: u64 = 0x7001ecc220; const P7: u64 = 0x7001ecc330; const P8: u64 = 0x7001ecc210; const P9: u64 = 0x7001ecc214; const P10: u64 = 0x7001ecc228; const P11: u64 = 0x7001ecc230; const P12: u64 = 0x7001ecc040; const P13: u64 = 0x7001ecc080; const P14: u64 = 0x7001eca248; const P15: u64 = 0x7001ec8628;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rcx) as u64); rsp = ((rsp).wrapping_sub(0x38u64)) as u64; rcx = (0x17u32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P0)) as u64; if t == 0x7001ec87f0 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rcx = (0x2u32) as u32 as u64; crate::lift_ops::trap(0xc0000409, P1, rcx, rsp); return rax; bb = 2; continue; }
            2 => { rcx = ((P2) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; crate::simd_ops::wu64(P4, (rax) as u64); rax = ((rsp.wrapping_add(0x38u64)) as u64) as u64; rax = ((rax).wrapping_add(0x8u64)) as u64; crate::simd_ops::wu64(P5, (rax) as u64); rax = (crate::simd_ops::ru64(P4)) as u64; crate::simd_ops::wu64(P6, (rax) as u64); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; crate::simd_ops::wu64(P7, (rax) as u64); crate::simd_ops::wu32(P8, (0xc0000409u32) as u32); crate::simd_ops::wu32(P9, (0x1u32) as u32); crate::simd_ops::wu32(P10, (0x1u32) as u32); rax = (0x8u32) as u32 as u64; rax = (((rax) as i64).wrapping_mul((0x0u64) as i64) as u64) as u64; rcx = ((P11) as u64) as u64; crate::simd_ops::wu64(rcx.wrapping_add(rax), (0x2u64) as u64); rax = (0x8u32) as u32 as u64; rax = (((rax) as i64).wrapping_mul((0x0u64) as i64) as u64) as u64; rcx = (crate::simd_ops::ru64(P12)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(rax).wrapping_add(0x20u64), (rcx) as u64); rax = (0x8u32) as u32 as u64; rax = (((rax) as i64).wrapping_mul((0x1u64) as i64) as u64) as u64; rcx = (crate::simd_ops::ru64(P13)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(rax).wrapping_add(0x20u64), (rcx) as u64); rcx = ((P14) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P15) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rsp = ((rsp).wrapping_add(0x38u64)) as u64; return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001ec87f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((rax) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::set_xmm_ret(c_x0); return c_rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001ec8810(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001ec9170;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P0)) as u64; if t == 0x7001ec87f0 { rax } else { t } }) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::set_xmm_ret(c_x0); return c_rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001ec8816(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x18; const P1: u64 = 0x20; const P2: u64 = 0x7001ec7524; const P3: u64 = 0x8; const P4: u64 = 0x28; const P5: u64 = 0x7001ec9130;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64); rsp = ((rsp).wrapping_sub(0x28u64)) as u64; rbp = (rdx) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x38u64), (rcx) as u64); crate::simd_ops::wu64(rbp.wrapping_add(0x30u64), (rcx) as u64); fl_l = (crate::simd_ops::ru8(rbp.wrapping_add(0x58u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 7; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x30u64))) as u64; rcx = (crate::simd_ops::ru64(rax)) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x28u64), (rcx) as u64); rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x28u64))) as u64; fl_l = (crate::simd_ops::ru32(rax)) as u32 as u64; fl_r = (0xe06d7363u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 7; continue; } bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x28u64))) as u64; fl_l = (crate::simd_ops::ru32(rax.wrapping_add(P0))) as u32 as u64; fl_r = (0x4u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 7; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x28u64))) as u64; fl_l = (crate::simd_ops::ru32(rax.wrapping_add(P1))) as u32 as u64; fl_r = (0x19930520u32) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 6; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x28u64))) as u64; fl_l = (crate::simd_ops::ru32(rax.wrapping_add(P1))) as u32 as u64; fl_r = (0x19930521u32) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x28u64))) as u64; fl_l = (crate::simd_ops::ru32(rax.wrapping_add(P1))) as u32 as u64; fl_r = (0x19930522u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 7; continue; } bb = 6; continue; }
            6 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x28u64))) as u64; crate::simd_ops::wu64(rax.wrapping_add(P1), (rcx) as u64); rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x30u64))) as u64; rbx = (crate::simd_ops::ru64(rax.wrapping_add(P3))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); crate::simd_ops::wu64(rax.wrapping_add(P4), (rbx) as u64); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P5)) as u64; if t == 0x7001ec87f0 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 7; continue; }
            7 => { crate::simd_ops::wu32(rbp.wrapping_add(0x20u64), (0x0u32) as u32); rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x20u64))) as u32 as u64; rsp = ((rsp).wrapping_add(0x28u64)) as u64; rbp = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rbx = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001ec88ad(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001ec7448;
    let (mut rax, mut rbp) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64);
    rsp = ((rsp).wrapping_sub(0x40u64)) as u64;
    rbp = (rdx) as u64;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    crate::simd_ops::wu32(rax, (0xfffffffeu32) as u32);
    rsp = ((rsp).wrapping_add(0x40u64)) as u64;
    rbp = core::ptr::read_unaligned(rsp as *const u64);
    rsp = rsp.wrapping_add(8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001ec88c8(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001ec6a64;
    let (mut rax, mut rbp) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbp = (rdx) as u64;
    crate::simd_ops::wu64(rbp.wrapping_add(0x80u64), (rcx) as u64);
    r9 = ((rbp.wrapping_add(0x20u64)) as u64) as u64;
    r8 = (crate::simd_ops::ru32(rbp.wrapping_add(0xe8u64))) as u32 as u64;
    rdx = (crate::simd_ops::ru64(rbp.wrapping_add(0xf8u64))) as u64;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbp = core::ptr::read_unaligned(rsp as *const u64);
    rsp = rsp.wrapping_add(8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001ec88f7(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001ec802c; const P1: u64 = 0x18; const P2: u64 = 0x20; const P3: u64 = 0x28; const P4: u64 = 0x7001ec5214; const P5: u64 = 0x7001ec51a0; const P6: u64 = 0x7001ec7524; const P7: u64 = 0x7001ec7448;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64); rsp = ((rsp).wrapping_sub(0x28u64)) as u64; rbp = (rdx) as u64; rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x48u64))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); fl_l = (crate::simd_ops::ru32(rbp.wrapping_add(0x20u64))) as u32 as u64; fl_r = (0x0u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 8; continue; } bb = 1; continue; }
            1 => { rbx = (crate::simd_ops::ru64(rbp.wrapping_add(0xf8u64))) as u64; fl_l = (crate::simd_ops::ru32(rbx)) as u32 as u64; fl_r = (0xe06d7363u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 8; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(P1))) as u32 as u64; fl_r = (0x4u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 8; continue; } bb = 3; continue; }
            3 => { fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(P2))) as u32 as u64; fl_r = (0x19930520u32) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 6; continue; } bb = 4; continue; }
            4 => { fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(P2))) as u32 as u64; fl_r = (0x19930521u32) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 6; continue; } bb = 5; continue; }
            5 => { fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(P2))) as u32 as u64; fl_r = (0x19930522u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 8; continue; } bb = 6; continue; }
            6 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P3))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 8; continue; } bb = 7; continue; }
            7 => { rdx = (rdx & !0xffu64) | (((0x1u8) as u8 as u64)); rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 8; continue; }
            8 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu64(rax.wrapping_add(P2), (rcx) as u64); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x38u64))) as u64; crate::simd_ops::wu64(rax.wrapping_add(P3), (rcx) as u64); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P7) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rcx = (crate::simd_ops::ru32(rbp.wrapping_add(0xe0u64))) as u32 as u64; crate::simd_ops::wu32(rax, ((rcx as u32)) as u32); rsp = ((rsp).wrapping_add(0x28u64)) as u64; rbp = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rbx = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001ec8987(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001ec5268;
    let (mut rax, mut rbp) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64);
    rsp = ((rsp).wrapping_sub(0x30u64)) as u64;
    rbp = (rdx) as u64;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    rsp = ((rsp).wrapping_add(0x30u64)) as u64;
    rbp = core::ptr::read_unaligned(rsp as *const u64);
    rsp = rsp.wrapping_add(8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001ec899d(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001ec7524; const P1: u64 = 0x30;
    let (mut rax, mut fl_l, mut fl_r, mut rbp) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64);
    rsp = ((rsp).wrapping_sub(0x30u64)) as u64;
    rbp = (rdx) as u64;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    fl_l = (crate::simd_ops::ru32(rax.wrapping_add(P1))) as u32 as u64; fl_r = (0x0u32) as u32 as u64;
    if !(((((fl_l as u32)) as i32) <= (((fl_r as u32)) as i32))) { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); fl_l = (crate::simd_ops::lk_sub_u32(rax.wrapping_add(P1), (1) as u32)) as u32 as u64; }
    rsp = ((rsp).wrapping_add(0x30u64)) as u64;
    rbp = core::ptr::read_unaligned(rsp as *const u64);
    rsp = rsp.wrapping_add(8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001ec89c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001ec7524; const P1: u64 = 0x70; const P2: u64 = 0x8; const P3: u64 = 0x60; const P4: u64 = 0x38; const P5: u64 = 0x68; const P6: u64 = 0x7001ec7448; const P7: u64 = 0x7001ec6354;
    let (mut rax, mut rbx, mut rbp, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rbp = (rdx) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x50u64), (rcx) as u64); crate::simd_ops::wu64(rbp.wrapping_add(0x48u64), (rcx) as u64); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x80u64))) as u64; crate::simd_ops::wu64(rax.wrapping_add(P1), (rcx) as u64); rdi = (crate::simd_ops::ru64(rbp.wrapping_add(0x98u64))) as u64; rbx = (crate::simd_ops::ru64(rdi.wrapping_add(P2))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); crate::simd_ops::wu64(rax.wrapping_add(P3), (rbx) as u64); rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x48u64))) as u64; rcx = (crate::simd_ops::ru64(rax)) as u64; rbx = (crate::simd_ops::ru64(rcx.wrapping_add(P4))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); crate::simd_ops::wu64(rax.wrapping_add(P5), (rbx) as u64); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rcx = (crate::simd_ops::ru32(rbp.wrapping_add(0xb8u64))) as u32 as u64; crate::simd_ops::wu32(rax, ((rcx as u32)) as u32); rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x48u64))) as u64; crate::simd_ops::wu8(rsp.wrapping_add(0x38u64), (0x1u8) as u8); crate::simd_ops::lk_and_u64(rsp.wrapping_add(0x30u64), (0x0u64) as u64); crate::simd_ops::lk_and_u32(rsp.wrapping_add(0x28u64), (0x0u32) as u32); rax = (crate::simd_ops::ru64(rbp.wrapping_add(0xa0u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rax) as u64); r9 = (rdi) as u64; r8 = (crate::simd_ops::ru64(rbp.wrapping_add(0x90u64))) as u64; rdx = (crate::simd_ops::ru64(rbp.wrapping_add(0x88u64))) as u64; rcx = (crate::simd_ops::ru64(rcx)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P7) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); crate::simd_ops::lk_and_u64(rax.wrapping_add(P1), (0x0u64) as u64); crate::simd_ops::wu32(rbp.wrapping_add(0x40u64), (0x1u32) as u32); rax = (0x1u32) as u32 as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rbp = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rbx = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            _ => { return rax; }
        }
    }
}
