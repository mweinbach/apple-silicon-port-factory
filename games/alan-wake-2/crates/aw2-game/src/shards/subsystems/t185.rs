#![allow(unused_imports, unused_variables, unused_mut, unused_assignments, non_snake_case, dead_code, clippy::all, unreachable_code, unused_parens)]

#[inline(never)]
pub unsafe extern "win64" fn fn_1412faf90(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x8; const P2: u64 = 0x142535070; const P3: u64 = 0x1426456c8; const P4: u64 = 0x1426453a0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x40u64)) as u64;
    rax = ((crate::exe::tls_pointer() as u64)) as u64;
    rdi = rcx;
    rbx = 0;
    fl_l = 0; fl_r = 0;
    rcx = 0x6348;
    rsi = r8;
    rdx = (crate::simd_ops::ru64(rax)) as u64;
    r8 = ((rbx.wrapping_add(P0)) as u32) as u32 as u64;
    rcx = (crate::simd_ops::ru64(rcx.wrapping_add(rdx))) as u64;
    rdx = ((rbx.wrapping_add(P1)) as u32) as u32 as u64;
    rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp);
    fl_l = rax; fl_r = rax;
    if !(((((fl_l as u64)) & ((fl_r as u64))) != 0)) { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); rcx = rax; rdx = 0xdeadd00d; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp); } else { rbx = rax; }
    rax = (crate::simd_ops::ru64(rsi)) as u64;
    crate::simd_ops::wu64(rbx, rax);
    rax = rdi;
    crate::simd_ops::wu64(rdi, rbx);
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64;
    rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64;
    rsp = ((rsp).wrapping_add(0x40u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1412fb010(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14207aec0; const P1: u64 = 0x142535070; const P2: u64 = 0x1426456c8; const P3: u64 = 0x1426453a0;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x1412fb010, funcinfo_rva: 0x3698f40, handler: 1, shape: 2, frame_reg: 0, establisher_off: 0x68, conts: &[] };
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (rsi) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rdi) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rdx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = ((rsp).wrapping_sub(0x60u64)) as u64; rsi = (r9) as u64; r14 = (r8) as u64; rdi = (rcx) as u64; rbx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rcx = (0x6348u32) as u32 as u64; rax = ((crate::exe::tls_pointer() as u64)) as u64; rdx = (crate::simd_ops::ru64(rax)) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(rdx))) as u64; r10 = (crate::simd_ops::ru64(rcx.wrapping_add(0x78u64))) as u64; rdx = (crate::simd_ops::ru64(r10.wrapping_add(0x10u64))) as u64; rax = (rdx) as u64; rax = ((((rax as u32)) & (0xfu32))) as u32 as u64; fl_l = (rdx) as u64 as u64; fl_r = (rdx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 1; continue; }
            1 => { fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 5; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::lk_add_u32(r10.wrapping_add(0x18u64), (1) as u32)) as u32 as u64; rax = (crate::simd_ops::ru64(rdx)) as u64; crate::simd_ops::wu64(r10.wrapping_add(0x10u64), (rax) as u64); bb = 3; continue; }
            3 => { rbx = (rdx) as u64; bb = 4; continue; }
            4 => { crate::simd_ops::wu64(rsp.wrapping_add(0x78u64), (rbx) as u64); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xb8u64))) as u64; rcx = ((crate::simd_ops::ru8(rax)) as u32) as u32 as u64; crate::simd_ops::wu8(rsp.wrapping_add(0x40u64), ((rcx as u8)) as u8); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xb0u64))) as u64; rdx = ((crate::simd_ops::ru8(rax)) as u32) as u32 as u64; crate::simd_ops::wu8(rsp.wrapping_add(0x38u64), ((rdx as u8)) as u8); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xa8u64))) as u64; rdx = ((crate::simd_ops::ru8(rax)) as u32) as u32 as u64; crate::simd_ops::wu8(rsp.wrapping_add(0x30u64), ((rdx as u8)) as u8); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xa0u64))) as u64; rdx = ((crate::simd_ops::ru8(rax)) as u32) as u32 as u64; crate::simd_ops::wu8(rsp.wrapping_add(0x28u64), ((rdx as u8)) as u8); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x98u64))) as u64; rdx = (crate::simd_ops::ru32(rax)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rdx as u32)) as u32); r9 = (crate::simd_ops::ru64(rsp.wrapping_add(0x90u64))) as u64; r9 = (crate::simd_ops::ru32(r9)) as u32 as u64; r8 = (crate::simd_ops::ru32(rsi)) as u32 as u64; rdx = (r14) as u64; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1412fb0db); crate::simd_ops::replace_rax_in(rax); rax = match eh.call(0x1412fb0db, (P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8) { Ok(v) => v, Err(l) => l.unknown(0x1412fb010) }; crate::simd_ops::wu64(rdi, (rax) as u64); rax = (rdi) as u64; r11 = ((rsp.wrapping_add(0x60u64)) as u64) as u64; rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x10u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x20u64))) as u64; rdi = (crate::simd_ops::ru64(r11.wrapping_add(0x28u64))) as u64; rsp = (r11) as u64; r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            5 => { rdx = (0x70u32) as u32 as u64; r8 = ((rdx.wrapping_sub(0x60u64)) as u32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1412fb106); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rdx = (rax) as u64; fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 3; continue; } bb = 6; continue; }
            6 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1412fb117); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P2)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rcx = (rax) as u64; rdx = (0xdeadd00du32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1412fb125); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P3)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 4; continue; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1412fb130(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xa8; const P1: u64 = 0x10; const P2: u64 = 0x18; const P3: u64 = 0x1420561a0; const P4: u64 = 0x142535070; const P5: u64 = 0x1426456c8; const P6: u64 = 0x1426453a0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rax = ((crate::exe::tls_pointer() as u64)) as u64; rdi = rcx; rcx = 0x6348; rbx = 0; fl_l = 0; fl_r = 0; rdx = (crate::simd_ops::ru64(rax)) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(rdx))) as u64; r8 = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rdx = (crate::simd_ops::ru64(r8.wrapping_add(P1))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 1; continue; }
            1 => { fl_l = ((rdx as u8)) as u8 as u64; fl_r = (0xfu8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 5; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::lk_add_u32(r8.wrapping_add(P2), (1) as u32)) as u32 as u64; rax = (crate::simd_ops::ru64(rdx)) as u64; crate::simd_ops::wu64(r8.wrapping_add(P1), rax); bb = 3; continue; }
            3 => { rbx = rdx; bb = 4; continue; }
            4 => { rcx = rbx; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rdi, rax); rax = rdi; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            5 => { rdx = 0xa0; r8 = 0x10; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rdx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 3; continue; } bb = 6; continue; }
            6 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp); rcx = rax; rdx = 0xdeadd00d; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P6), rax), rax, rcx, rdx, r8, r9, rsp); bb = 4; continue; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1412fb1c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x40; const P1: u64 = 0x10; const P2: u64 = 0x18; const P3: u64 = 0xffffffffffffffd8; const P4: u64 = 0x142535070; const P5: u64 = 0x1426456c8; const P6: u64 = 0x1426453a0; const P7: u64 = 0x1433f2eb0; const P8: u64 = 0x14; const P9: u64 = 0x8; const P10: u64 = 0x20; const P11: u64 = 0x28; const P12: u64 = 0x14397ff18;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rax = ((crate::exe::tls_pointer() as u64)) as u64; rbx = rcx; rcx = 0x6348; rdi = 0; fl_l = 0; fl_r = 0; rdx = (crate::simd_ops::ru64(rax)) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(rdx))) as u64; r8 = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rdx = (crate::simd_ops::ru64(r8.wrapping_add(P1))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { fl_l = ((rdx as u8)) as u8 as u64; fl_r = (0xfu8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::lk_add_u32(r8.wrapping_add(P2), (1) as u32)) as u32 as u64; rax = (crate::simd_ops::ru64(rdx)) as u64; crate::simd_ops::wu64(r8.wrapping_add(P1), rax); bb = 5; continue; }
            3 => { rdx = 0x38; r8 = ((rdx.wrapping_add(P3)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rdx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp); rcx = rax; rdx = 0xdeadd00d; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P6), rax), rax, rcx, rdx, r8, r9, rsp); rdx = rdi; bb = 5; continue; }
            5 => { crate::simd_ops::wu32(rdx.wrapping_add(P1), (0xffffffffu32) as u32); rax = P7; crate::simd_ops::wu64(rdx, rax); rax = rbx; crate::simd_ops::wu32(rdx.wrapping_add(P8), ((rdi as u32)) as u32); crate::simd_ops::wu64(rdx.wrapping_add(P9), rdi); crate::simd_ops::wu64(rdx.wrapping_add(P2), rdi); crate::simd_ops::wu64(rdx.wrapping_add(P10), rdi); crate::simd_ops::wu64(rdx.wrapping_add(P11), rdi); crate::simd_ops::wu64(rdx.wrapping_add(0x30u64), rdi); crate::simd_ops::wu64(rbx, rdx); crate::simd_ops::wu64(P12, rdx); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1412fb270(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x18; const P1: u64 = 0x10; const P2: u64 = 0x142537280; const P3: u64 = 0x1426456c8; const P4: u64 = 0x1426453a0; const P5: u64 = 0x8; const P6: u64 = 0x143a00210;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rax = ((crate::exe::tls_pointer() as u64)) as u64; rbx = rcx; rcx = 0x6348; rdi = 0; fl_l = 0; fl_r = 0; rdx = (crate::simd_ops::ru64(rax)) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(rdx))) as u64; r8 = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rdx = (crate::simd_ops::ru64(r8.wrapping_add(P1))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { r9 = 0; fl_l = 0; fl_r = 0; rdx = ((rdi.wrapping_add(P1)) as u32) as u32 as u64; r8 = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rdx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 4; continue; } bb = 2; continue; }
            2 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); rcx = rax; rdx = 0xdeadd00d; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp); rdx = ((rdi as u32)) as u32 as u64; bb = 4; continue; }
            3 => { fl_l = (crate::simd_ops::lk_add_u32(r8.wrapping_add(P0), (1) as u32)) as u32 as u64; rax = (crate::simd_ops::ru64(rdx)) as u64; crate::simd_ops::wu64(r8.wrapping_add(P1), rax); bb = 4; continue; }
            4 => { crate::simd_ops::wu64(rdx, rdi); rax = rbx; crate::simd_ops::wu64(rdx.wrapping_add(P5), rdi); crate::simd_ops::wu64(rbx, rdx); crate::simd_ops::wu64(P6, rdx); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1412fb300(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x142535070; const P2: u64 = 0x1426456c8; const P3: u64 = 0x1426453a0; const P4: u64 = 0x14223e800;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x40u64)) as u64;
    rdi = rcx;
    rbx = 0;
    fl_l = 0; fl_r = 0;
    rcx = 0x6348;
    rax = ((crate::exe::tls_pointer() as u64)) as u64;
    rdx = (crate::simd_ops::ru64(rax)) as u64;
    rcx = (crate::simd_ops::ru64(rcx.wrapping_add(rdx))) as u64;
    rdx = 0x12c0;
    r8 = ((rbx.wrapping_add(P0)) as u32) as u32 as u64;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    fl_l = rax; fl_r = rax;
    if !(((((fl_l as u64)) & ((fl_r as u64))) != 0)) { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); rcx = rax; rdx = 0xdeadd00d; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); } else { rbx = rax; }
    crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rbx);
    rcx = rbx;
    rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp);
    crate::simd_ops::wu64(rdi, rax);
    rax = rdi;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64;
    rsp = ((rsp).wrapping_add(0x40u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1412fb380(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x142535070; const P2: u64 = 0x1426456c8; const P3: u64 = 0x1426453a0; const P4: u64 = 0x14183acc2; const P5: u64 = 0x1412ea8c0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x40u64)) as u64;
    rax = ((crate::exe::tls_pointer() as u64)) as u64;
    rdi = rcx;
    rcx = 0x6348;
    rbx = 0;
    fl_l = 0; fl_r = 0;
    rdx = (crate::simd_ops::ru64(rax)) as u64;
    r8 = ((rbx.wrapping_add(P0)) as u32) as u32 as u64;
    rcx = (crate::simd_ops::ru64(rcx.wrapping_add(rdx))) as u64;
    rdx = 0x7d0;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    fl_l = rax; fl_r = rax;
    if !(((((fl_l as u64)) & ((fl_r as u64))) != 0)) { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); rcx = rax; rdx = 0xdeadd00d; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); } else { rbx = rax; }
    rdx = 0;
    fl_l = 0; fl_r = 0;
    r8 = 0x7d0;
    rcx = rbx;
    rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp);
    rcx = rbx;
    rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp);
    crate::simd_ops::wu64(rdi, rax);
    rax = rdi;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64;
    rsp = ((rsp).wrapping_add(0x40u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1412fb400(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xf8; const P1: u64 = 0x10; const P2: u64 = 0x18; const P3: u64 = 0x14230c160; const P4: u64 = 0x142535070; const P5: u64 = 0x1426456c8; const P6: u64 = 0x1426453a0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rdi = rcx; rbx = 0; fl_l = 0; fl_r = 0; rcx = 0x6348; rax = ((crate::exe::tls_pointer() as u64)) as u64; rdx = (crate::simd_ops::ru64(rax)) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(rdx))) as u64; r8 = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rdx = (crate::simd_ops::ru64(r8.wrapping_add(P1))) as u64; rax = rdx; rax = ((((rax as u32)) & (0xfu32))) as u32 as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 1; continue; }
            1 => { fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 5; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::lk_add_u32(r8.wrapping_add(P2), (1) as u32)) as u32 as u64; rax = (crate::simd_ops::ru64(rdx)) as u64; crate::simd_ops::wu64(r8.wrapping_add(P1), rax); bb = 3; continue; }
            3 => { rbx = rdx; bb = 4; continue; }
            4 => { crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rbx); rcx = rbx; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rdi, rax); rax = rdi; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            5 => { rdx = 0xf0; r8 = 0x10; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rdx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 3; continue; } bb = 6; continue; }
            6 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp); rcx = rax; rdx = 0xdeadd00d; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P6), rax), rax, rcx, rdx, r8, r9, rsp); bb = 4; continue; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1412fb4a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x318; const P1: u64 = 0x10; const P2: u64 = 0x18; const P3: u64 = 0x142208150; const P4: u64 = 0x142535070; const P5: u64 = 0x1426456c8; const P6: u64 = 0x1426453a0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rdi = rcx; rbx = 0; fl_l = 0; fl_r = 0; rcx = 0x6348; rax = ((crate::exe::tls_pointer() as u64)) as u64; rdx = (crate::simd_ops::ru64(rax)) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(rdx))) as u64; r8 = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rdx = (crate::simd_ops::ru64(r8.wrapping_add(P1))) as u64; rax = rdx; rax = ((((rax as u32)) & (0xfu32))) as u32 as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 1; continue; }
            1 => { fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 5; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::lk_add_u32(r8.wrapping_add(P2), (1) as u32)) as u32 as u64; rax = (crate::simd_ops::ru64(rdx)) as u64; crate::simd_ops::wu64(r8.wrapping_add(P1), rax); bb = 3; continue; }
            3 => { rbx = rdx; bb = 4; continue; }
            4 => { crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rbx); rcx = rbx; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rdi, rax); rax = rdi; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            5 => { rdx = 0x310; r8 = 0x10; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rdx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 3; continue; } bb = 6; continue; }
            6 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp); rcx = rax; rdx = 0xdeadd00d; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P6), rax), rax, rcx, rdx, r8, r9, rsp); bb = 4; continue; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1412fb540(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142535070; const P1: u64 = 0x1426456c8; const P2: u64 = 0x1426453a0; const P3: u64 = 0x14183acc2;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rsi = rcx; rdi = 0; fl_l = 0; fl_r = 0; rcx = 0x6348; rax = ((crate::exe::tls_pointer() as u64)) as u64; rdx = (crate::simd_ops::ru64(rax)) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(rdx))) as u64; rdx = 0x4078; r8 = ((rdi.wrapping_add(0x10u64)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rbx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp); rcx = rax; rdx = 0xdeadd00d; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); rbx = ((rdi as u32)) as u32 as u64; bb = 2; continue; }
            2 => { crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rbx); crate::simd_ops::wu64(rbx, rdi); crate::simd_ops::wu64(rbx.wrapping_add(0x8u64), rdi); crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), rdi); crate::simd_ops::wu64(rbx.wrapping_add(0x18u64), rdi); crate::simd_ops::wu64(rbx.wrapping_add(0x20u64), rdi); crate::simd_ops::wu64(rbx.wrapping_add(0x28u64), rdi); crate::simd_ops::wu8(rbx.wrapping_add(0x30u64), (0x0u8) as u8); crate::simd_ops::wu64(rbx.wrapping_add(0x38u64), rdi); crate::simd_ops::wu64(rbx.wrapping_add(0x40u64), rdi); crate::simd_ops::wu64(rbx.wrapping_add(0x48u64), rdi); crate::simd_ops::wu64(rbx.wrapping_add(0x50u64), rdi); crate::simd_ops::wu64(rbx.wrapping_add(0x58u64), rdi); crate::simd_ops::wu64(rbx.wrapping_add(0x60u64), rdi); crate::simd_ops::wu64(rbx.wrapping_add(0x68u64), rdi); crate::simd_ops::wu64(rbx.wrapping_add(0x70u64), (0xffffffffffffffffu64) as u64); rdx = 0x400; rcx = ((rdx as u32)) as u32 as u64; r8 = rbx.wrapping_add(0x78); bb = 3; continue; }
            3 => { rax = rcx; rcx = ((rcx).wrapping_sub(1)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(r8, rax); fl_l = ((r8).wrapping_add(0x8u64)) as u64; fl_r = fl_l; r8 = ((fl_l as u64)) as u64; bb = 3; continue; }
            5 => { rcx = rbx.wrapping_add(0x2078); bb = 6; continue; }
            6 => { rax = rdx; rdx = ((rdx).wrapping_sub(1)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 7; continue; }
            7 => { rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rcx, rax); fl_l = ((rcx).wrapping_add(0x8u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; bb = 6; continue; }
            8 => { rdx = 0; fl_l = 0; fl_r = 0; r8 = 0x2000; rcx = rbx.wrapping_add(0x78); rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rdx = 0; fl_l = 0; fl_r = 0; r8 = 0x2000; rcx = rbx.wrapping_add(0x2078); rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rsi, rbx); rax = rsi; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1412fb670(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142535070; const P1: u64 = 0x1426456c8; const P2: u64 = 0x1426453a0; const P3: u64 = 0x1408b4ad0; const P4: u64 = 0x140fff0a0; const P5: u64 = 0x1401deba0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x50u64)) as u64; r14 = rcx; rbp = 0; fl_l = 0; fl_r = 0; rcx = 0x6348; rax = ((crate::exe::tls_pointer() as u64)) as u64; rdx = (crate::simd_ops::ru64(rax)) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(rdx))) as u64; rdx = (crate::simd_ops::ru64(rcx.wrapping_add(0x38u64))) as u64; rbx = (crate::simd_ops::ru64(rdx.wrapping_add(0x10u64))) as u64; rax = rbx; rax = ((((rax as u32)) & (0xfu32))) as u32 as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::lk_add_u32(rdx.wrapping_add(0x18u64), (1) as u32)) as u32 as u64; rax = (crate::simd_ops::ru64(rbx)) as u64; crate::simd_ops::wu64(rdx.wrapping_add(0x10u64), rax); bb = 5; continue; }
            3 => { rdx = 0x30; r8 = ((rdx.wrapping_sub(0x20u64)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rbx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp); rcx = rax; rdx = 0xdeadd00d; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); rbx = rbp; bb = 5; continue; }
            5 => { crate::simd_ops::wu64(rbx, rbp); crate::simd_ops::wu64(rbx.wrapping_add(0x8u64), rbp); crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rbx.wrapping_add(0x18u64), rbp); crate::simd_ops::wu64(rbx.wrapping_add(0x20u64), rbp); crate::simd_ops::wu64(rbx.wrapping_add(0x28u64), rbp); rdi = 0x1; rdx = ((rdi as u32)) as u32 as u64; rcx = rbx.wrapping_add(0x10); rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x20u64), rdi); fl_l = rdx; fl_r = rdi; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 7; continue; } bb = 6; continue; }
            6 => { crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rdi); rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rdi); rdi = ((rdi).wrapping_sub(rdx)) as u64; r9 = 0; fl_l = 0; fl_r = 0; r8 = rdi; rcx = rsp.wrapping_add(0x38); rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); bb = 7; continue; }
            7 => { rdx = (crate::simd_ops::ru32(rbx.wrapping_add(0x8u64))) as u32 as u64; r8 = (crate::simd_ops::ru32(rbx.wrapping_add(0xcu64))) as u32 as u64; rcx = ((r8.wrapping_mul(8)) as u64) as u64; rax = ((rdx.wrapping_mul(8)) as u64) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 9; continue; } bb = 8; continue; }
            8 => { rax = rdx.wrapping_add(0x1); rdx = ((r8 as u32)) as u32 as u64; rdx = ((rdx).wrapping_shr(1u32)) as u64; rdx = ((rdx).wrapping_add(r8)) as u64; fl_l = rdx; fl_r = rax; rdx = (if (((fl_l as u64)) < ((fl_r as u64))) { rax } else { rdx }) as u64; rcx = rbx; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); bb = 9; continue; }
            9 => { rcx = (crate::simd_ops::ru32(rbx.wrapping_add(0x8u64))) as u32 as u64; rax = (crate::simd_ops::ru64(rbx)) as u64; crate::simd_ops::wu64(rax.wrapping_add(rcx.wrapping_mul(8)), rbp); crate::simd_ops::lk_add_u32(rbx.wrapping_add(0x8u64), (1) as u32); crate::simd_ops::wu64(r14, rbx); rax = r14; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; rsp = ((rsp).wrapping_add(0x50u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1412fb7c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1f0; const P1: u64 = 0x10; const P2: u64 = 0x18; const P3: u64 = 0x142323630; const P4: u64 = 0x142535070; const P5: u64 = 0x1426456c8; const P6: u64 = 0x1426453a0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rdi = rcx; rbx = 0; fl_l = 0; fl_r = 0; rcx = 0x6348; rax = ((crate::exe::tls_pointer() as u64)) as u64; rdx = (crate::simd_ops::ru64(rax)) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(rdx))) as u64; r8 = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rdx = (crate::simd_ops::ru64(r8.wrapping_add(P1))) as u64; rax = rdx; rax = ((((rax as u32)) & (0xfu32))) as u32 as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 1; continue; }
            1 => { fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 5; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::lk_add_u32(r8.wrapping_add(P2), (1) as u32)) as u32 as u64; rax = (crate::simd_ops::ru64(rdx)) as u64; crate::simd_ops::wu64(r8.wrapping_add(P1), rax); bb = 3; continue; }
            3 => { rbx = rdx; bb = 4; continue; }
            4 => { crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rbx); rcx = rbx; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rdi, rax); rax = rdi; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            5 => { rdx = 0x1e8; r8 = 0x10; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rdx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 3; continue; } bb = 6; continue; }
            6 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp); rcx = rax; rdx = 0xdeadd00d; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P6), rax), rax, rcx, rdx, r8, r9, rsp); bb = 4; continue; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1412fb860(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142535070; const P1: u64 = 0x1426456c8; const P2: u64 = 0x1426453a0; const P3: u64 = 0x1433036e4; const P4: u64 = 0x142409b50; const P5: u64 = 0x8; const P6: u64 = 0x1423241a0; const P7: u64 = 0x14240ab10;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 27>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r9); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0xd0u64)) as u64; rsi = r8; rdi = rcx; rcx = 0x6348; rax = ((crate::exe::tls_pointer() as u64)) as u64; rdx = (crate::simd_ops::ru64(rax)) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(rdx))) as u64; rdx = 0x4f8; r8 = 0x10; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rax; rdx = 0xdeadd00d; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 2; continue; }
            2 => { crate::simd_ops::wu64(rsp.wrapping_add(0xf8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x44u64), (0x10000010u64) as u64); crate::simd_ops::wu8(rsp.wrapping_add(0x4cu64), (0x0u8) as u8); rax = P3; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), (0x6u64) as u64); r9 = 0x6; r8 = 0; fl_l = 0; fl_r = 0; rdx = rsp.wrapping_add(0x30); rcx = rsp.wrapping_add(0x44); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru32(rsi.wrapping_add(P5))) as u32 as u64; rdx = (crate::simd_ops::ru64(rsi)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rdx); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x30u64))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm0) as u128); rdx = rsp.wrapping_add(0x30); rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rdi, rax); rcx = rsp.wrapping_add(0x44); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = rdi; r11 = rsp.wrapping_add(0xd0); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x10u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x18u64))) as u64; rsp = r11; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1412fb9b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142535070; const P1: u64 = 0x1426456c8; const P2: u64 = 0x1426453a0; const P3: u64 = 0x1433f4a20; const P4: u64 = 0x142409b50; const P5: u64 = 0x1426cf830; const P6: u64 = 0x1423242e0; const P7: u64 = 0x14240ab10;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 27>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), r8); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0xd0u64)) as u64; rsi = r9; rdi = rcx; rcx = 0x6348; rax = ((crate::exe::tls_pointer() as u64)) as u64; rdx = (crate::simd_ops::ru64(rax)) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(rdx))) as u64; rdx = 0x4f8; r8 = 0x10; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rax; rdx = 0xdeadd00d; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 2; continue; }
            2 => { crate::simd_ops::wu64(rsp.wrapping_add(0xf0u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x44u64), (0x10000010u64) as u64); crate::simd_ops::wu8(rsp.wrapping_add(0x4cu64), (0x0u8) as u8); rax = P3; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), (0x18u64) as u64); r9 = 0x18; r8 = 0; fl_l = 0; fl_r = 0; rdx = rsp.wrapping_add(0x30); rcx = rsp.wrapping_add(0x44); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = P5; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), (0x5u64) as u64); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x30u64))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm0) as u128); r8 = (crate::simd_ops::ru64(rsi)) as u64; rdx = rsp.wrapping_add(0x30); rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rdi, rax); rcx = rsp.wrapping_add(0x44); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = rdi; r11 = rsp.wrapping_add(0xd0); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x10u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x18u64))) as u64; rsp = r11; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1412fbab0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142535070; const P1: u64 = 0x1426456c8; const P2: u64 = 0x1426453a0; const P3: u64 = 0x1433f4a08; const P4: u64 = 0x142409b50; const P5: u64 = 0x1426cf830; const P6: u64 = 0x1423242e0; const P7: u64 = 0x14240ab10;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 27>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), r8); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0xd0u64)) as u64; rsi = r9; rdi = rcx; rcx = 0x6348; rax = ((crate::exe::tls_pointer() as u64)) as u64; rdx = (crate::simd_ops::ru64(rax)) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(rdx))) as u64; rdx = 0x4f8; r8 = 0x10; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rax; rdx = 0xdeadd00d; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 2; continue; }
            2 => { crate::simd_ops::wu64(rsp.wrapping_add(0xf0u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x44u64), (0x10000010u64) as u64); crate::simd_ops::wu8(rsp.wrapping_add(0x4cu64), (0x0u8) as u8); rax = P3; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), (0x12u64) as u64); r9 = 0x12; r8 = 0; fl_l = 0; fl_r = 0; rdx = rsp.wrapping_add(0x30); rcx = rsp.wrapping_add(0x44); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = P5; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), (0x5u64) as u64); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x30u64))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm0) as u128); r8 = (crate::simd_ops::ru64(rsi)) as u64; rdx = rsp.wrapping_add(0x30); rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rdi, rax); rcx = rsp.wrapping_add(0x44); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = rdi; r11 = rsp.wrapping_add(0xd0); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x10u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x18u64))) as u64; rsp = r11; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1412fbbb0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142535070; const P1: u64 = 0x1426456c8; const P2: u64 = 0x1426453a0; const P3: u64 = 0x1433f4908; const P4: u64 = 0x142409b50; const P5: u64 = 0x1426cf938; const P6: u64 = 0x1423242e0; const P7: u64 = 0x14240ab10;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 27>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), r8); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0xd0u64)) as u64; rsi = r9; rdi = rcx; rcx = 0x6348; rax = ((crate::exe::tls_pointer() as u64)) as u64; rdx = (crate::simd_ops::ru64(rax)) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(rdx))) as u64; rdx = 0x4f8; r8 = 0x10; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rax; rdx = 0xdeadd00d; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 2; continue; }
            2 => { crate::simd_ops::wu64(rsp.wrapping_add(0xf0u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x44u64), (0x10000010u64) as u64); crate::simd_ops::wu8(rsp.wrapping_add(0x4cu64), (0x0u8) as u8); rax = P3; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), (0x10u64) as u64); r9 = 0x10; r8 = 0; fl_l = 0; fl_r = 0; rdx = rsp.wrapping_add(0x30); rcx = rsp.wrapping_add(0x44); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = P5; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), (0x3u64) as u64); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x30u64))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm0) as u128); r8 = (crate::simd_ops::ru64(rsi)) as u64; rdx = rsp.wrapping_add(0x30); rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rdi, rax); rcx = rsp.wrapping_add(0x44); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = rdi; r11 = rsp.wrapping_add(0xd0); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x10u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x18u64))) as u64; rsp = r11; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1412fbcb0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142535070; const P1: u64 = 0x1426456c8; const P2: u64 = 0x1426453a0; const P3: u64 = 0x1433f48f8; const P4: u64 = 0x142409b50; const P5: u64 = 0x1426cf908; const P6: u64 = 0x1423242e0; const P7: u64 = 0x14240ab10;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 27>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), r8); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0xd0u64)) as u64; rsi = r9; rdi = rcx; rcx = 0x6348; rax = ((crate::exe::tls_pointer() as u64)) as u64; rdx = (crate::simd_ops::ru64(rax)) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(rdx))) as u64; rdx = 0x4f8; r8 = 0x10; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rax; rdx = 0xdeadd00d; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 2; continue; }
            2 => { crate::simd_ops::wu64(rsp.wrapping_add(0xf0u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x44u64), (0x10000010u64) as u64); crate::simd_ops::wu8(rsp.wrapping_add(0x4cu64), (0x0u8) as u8); rax = P3; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), (0x8u64) as u64); r9 = 0x8; r8 = 0; fl_l = 0; fl_r = 0; rdx = rsp.wrapping_add(0x30); rcx = rsp.wrapping_add(0x44); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = P5; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), (0x8u64) as u64); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x30u64))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm0) as u128); r8 = (crate::simd_ops::ru64(rsi)) as u64; rdx = rsp.wrapping_add(0x30); rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rdi, rax); rcx = rsp.wrapping_add(0x44); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = rdi; r11 = rsp.wrapping_add(0xd0); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x10u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x18u64))) as u64; rsp = r11; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1412fbdb0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142535070; const P1: u64 = 0x1426456c8; const P2: u64 = 0x1426453a0; const P3: u64 = 0x1433f48f8; const P4: u64 = 0x142409b50; const P5: u64 = 0x142327360; const P6: u64 = 0x14240ab10;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 29>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rdi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r9); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0xe0u64)) as u64; r14 = r8; rdi = rcx; rcx = 0x6348; rax = ((crate::exe::tls_pointer() as u64)) as u64; rdx = (crate::simd_ops::ru64(rax)) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(rdx))) as u64; rdx = (crate::simd_ops::ru64(rcx.wrapping_add(0x138u64))) as u64; rbx = (crate::simd_ops::ru64(rdx.wrapping_add(0x10u64))) as u64; rax = rbx; rax = ((((rax as u32)) & (0xfu32))) as u32 as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::lk_add_u32(rdx.wrapping_add(0x18u64), (1) as u32)) as u32 as u64; rax = (crate::simd_ops::ru64(rbx)) as u64; crate::simd_ops::wu64(rdx.wrapping_add(0x10u64), rax); bb = 5; continue; }
            3 => { rdx = 0x130; r8 = 0x10; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rbx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp); rcx = rax; rdx = 0xdeadd00d; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); bb = 5; continue; }
            5 => { crate::simd_ops::wu64(rsp.wrapping_add(0x108u64), rbx); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x110u64))) as u64; rsi = (crate::simd_ops::ru32(rax)) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), (0x10000010u64) as u64); crate::simd_ops::wu8(rsp.wrapping_add(0x58u64), (0x0u8) as u8); rax = P3; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), (0x8u64) as u64); r9 = 0x8; r8 = 0; fl_l = 0; fl_r = 0; rdx = rsp.wrapping_add(0x40); rcx = rsp.wrapping_add(0x50); rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x118u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); r9 = ((rsi as u32)) as u32 as u64; rdx = (crate::simd_ops::ru64(r14)) as u64; rcx = rbx; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rdi, rax); rcx = rsp.wrapping_add(0x50); rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); rax = rdi; r11 = rsp.wrapping_add(0xe0); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x10u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x18u64))) as u64; rdi = (crate::simd_ops::ru64(r11.wrapping_add(0x20u64))) as u64; rsp = r11; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1412fbed0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142535070; const P1: u64 = 0x1426456c8; const P2: u64 = 0x1426453a0; const P3: u64 = 0x1433f48e8; const P4: u64 = 0x142409b50; const P5: u64 = 0x8; const P6: u64 = 0x1423241a0; const P7: u64 = 0x14240ab10;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 27>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r9); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0xd0u64)) as u64; rsi = r8; rdi = rcx; rcx = 0x6348; rax = ((crate::exe::tls_pointer() as u64)) as u64; rdx = (crate::simd_ops::ru64(rax)) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(rdx))) as u64; rdx = 0x4f8; r8 = 0x10; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rax; rdx = 0xdeadd00d; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 2; continue; }
            2 => { crate::simd_ops::wu64(rsp.wrapping_add(0xf8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x44u64), (0x10000010u64) as u64); crate::simd_ops::wu8(rsp.wrapping_add(0x4cu64), (0x0u8) as u8); rax = P3; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), (0xfu64) as u64); r9 = 0xf; r8 = 0; fl_l = 0; fl_r = 0; rdx = rsp.wrapping_add(0x30); rcx = rsp.wrapping_add(0x44); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru32(rsi.wrapping_add(P5))) as u32 as u64; rdx = (crate::simd_ops::ru64(rsi)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rdx); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x30u64))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm0) as u128); rdx = rsp.wrapping_add(0x30); rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rdi, rax); rcx = rsp.wrapping_add(0x44); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = rdi; r11 = rsp.wrapping_add(0xd0); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x10u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x18u64))) as u64; rsp = r11; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1412fbfd0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142535070; const P1: u64 = 0x1426456c8; const P2: u64 = 0x1426453a0; const P3: u64 = 0x1433f48e8; const P4: u64 = 0x142409b50; const P5: u64 = 0x142327360; const P6: u64 = 0x14240ab10;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 29>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rdi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r9); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0xe0u64)) as u64; r14 = r8; rdi = rcx; rcx = 0x6348; rax = ((crate::exe::tls_pointer() as u64)) as u64; rdx = (crate::simd_ops::ru64(rax)) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(rdx))) as u64; rdx = (crate::simd_ops::ru64(rcx.wrapping_add(0x138u64))) as u64; rbx = (crate::simd_ops::ru64(rdx.wrapping_add(0x10u64))) as u64; rax = rbx; rax = ((((rax as u32)) & (0xfu32))) as u32 as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::lk_add_u32(rdx.wrapping_add(0x18u64), (1) as u32)) as u32 as u64; rax = (crate::simd_ops::ru64(rbx)) as u64; crate::simd_ops::wu64(rdx.wrapping_add(0x10u64), rax); bb = 5; continue; }
            3 => { rdx = 0x130; r8 = 0x10; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rbx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp); rcx = rax; rdx = 0xdeadd00d; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); bb = 5; continue; }
            5 => { crate::simd_ops::wu64(rsp.wrapping_add(0x108u64), rbx); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x110u64))) as u64; rsi = (crate::simd_ops::ru32(rax)) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), (0x10000010u64) as u64); crate::simd_ops::wu8(rsp.wrapping_add(0x58u64), (0x0u8) as u8); rax = P3; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), (0xfu64) as u64); r9 = 0xf; r8 = 0; fl_l = 0; fl_r = 0; rdx = rsp.wrapping_add(0x40); rcx = rsp.wrapping_add(0x50); rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x118u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); r9 = ((rsi as u32)) as u32 as u64; rdx = (crate::simd_ops::ru64(r14)) as u64; rcx = rbx; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rdi, rax); rcx = rsp.wrapping_add(0x50); rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); rax = rdi; r11 = rsp.wrapping_add(0xe0); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x10u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x18u64))) as u64; rdi = (crate::simd_ops::ru64(r11.wrapping_add(0x20u64))) as u64; rsp = r11; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1412fc0f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142535070; const P1: u64 = 0x1426456c8; const P2: u64 = 0x1426453a0; const P3: u64 = 0x1433036d8; const P4: u64 = 0x142409b50; const P5: u64 = 0x8; const P6: u64 = 0x1423241a0; const P7: u64 = 0x14240ab10;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 27>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r9); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0xd0u64)) as u64; rsi = r8; rdi = rcx; rcx = 0x6348; rax = ((crate::exe::tls_pointer() as u64)) as u64; rdx = (crate::simd_ops::ru64(rax)) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(rdx))) as u64; rdx = 0x4f8; r8 = 0x10; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rax; rdx = 0xdeadd00d; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 2; continue; }
            2 => { crate::simd_ops::wu64(rsp.wrapping_add(0xf8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x44u64), (0x10000010u64) as u64); crate::simd_ops::wu8(rsp.wrapping_add(0x4cu64), (0x0u8) as u8); rax = P3; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), (0xbu64) as u64); r9 = 0xb; r8 = 0; fl_l = 0; fl_r = 0; rdx = rsp.wrapping_add(0x30); rcx = rsp.wrapping_add(0x44); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru32(rsi.wrapping_add(P5))) as u32 as u64; rdx = (crate::simd_ops::ru64(rsi)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rdx); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x30u64))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm0) as u128); rdx = rsp.wrapping_add(0x30); rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rdi, rax); rcx = rsp.wrapping_add(0x44); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = rdi; r11 = rsp.wrapping_add(0xd0); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x10u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x18u64))) as u64; rsp = r11; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1412fc1f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142535070; const P1: u64 = 0x1426456c8; const P2: u64 = 0x1426453a0; const P3: u64 = 0x1433036d8; const P4: u64 = 0x142409b50; const P5: u64 = 0x142327360; const P6: u64 = 0x14240ab10;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 29>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rdi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r9); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0xe0u64)) as u64; r14 = r8; rdi = rcx; rcx = 0x6348; rax = ((crate::exe::tls_pointer() as u64)) as u64; rdx = (crate::simd_ops::ru64(rax)) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(rdx))) as u64; rdx = (crate::simd_ops::ru64(rcx.wrapping_add(0x138u64))) as u64; rbx = (crate::simd_ops::ru64(rdx.wrapping_add(0x10u64))) as u64; rax = rbx; rax = ((((rax as u32)) & (0xfu32))) as u32 as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::lk_add_u32(rdx.wrapping_add(0x18u64), (1) as u32)) as u32 as u64; rax = (crate::simd_ops::ru64(rbx)) as u64; crate::simd_ops::wu64(rdx.wrapping_add(0x10u64), rax); bb = 5; continue; }
            3 => { rdx = 0x130; r8 = 0x10; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rbx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp); rcx = rax; rdx = 0xdeadd00d; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); bb = 5; continue; }
            5 => { crate::simd_ops::wu64(rsp.wrapping_add(0x108u64), rbx); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x110u64))) as u64; rsi = (crate::simd_ops::ru32(rax)) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), (0x10000010u64) as u64); crate::simd_ops::wu8(rsp.wrapping_add(0x58u64), (0x0u8) as u8); rax = P3; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), (0xbu64) as u64); r9 = 0xb; r8 = 0; fl_l = 0; fl_r = 0; rdx = rsp.wrapping_add(0x40); rcx = rsp.wrapping_add(0x50); rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x118u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); r9 = ((rsi as u32)) as u32 as u64; rdx = (crate::simd_ops::ru64(r14)) as u64; rcx = rbx; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rdi, rax); rcx = rsp.wrapping_add(0x50); rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); rax = rdi; r11 = rsp.wrapping_add(0xe0); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x10u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x18u64))) as u64; rdi = (crate::simd_ops::ru64(r11.wrapping_add(0x20u64))) as u64; rsp = r11; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1412fc310(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142535070; const P1: u64 = 0x1426456c8; const P2: u64 = 0x1426453a0; const P3: u64 = 0x143106cec; const P4: u64 = 0x142409b50; const P5: u64 = 0x142327360; const P6: u64 = 0x14240ab10;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 29>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rdi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r9); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0xe0u64)) as u64; r14 = r8; rdi = rcx; rcx = 0x6348; rax = ((crate::exe::tls_pointer() as u64)) as u64; rdx = (crate::simd_ops::ru64(rax)) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(rdx))) as u64; rdx = (crate::simd_ops::ru64(rcx.wrapping_add(0x138u64))) as u64; rbx = (crate::simd_ops::ru64(rdx.wrapping_add(0x10u64))) as u64; rax = rbx; rax = ((((rax as u32)) & (0xfu32))) as u32 as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::lk_add_u32(rdx.wrapping_add(0x18u64), (1) as u32)) as u32 as u64; rax = (crate::simd_ops::ru64(rbx)) as u64; crate::simd_ops::wu64(rdx.wrapping_add(0x10u64), rax); bb = 5; continue; }
            3 => { rdx = 0x130; r8 = 0x10; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rbx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp); rcx = rax; rdx = 0xdeadd00d; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); bb = 5; continue; }
            5 => { crate::simd_ops::wu64(rsp.wrapping_add(0x108u64), rbx); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x110u64))) as u64; rsi = (crate::simd_ops::ru32(rax)) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), (0x10000010u64) as u64); crate::simd_ops::wu8(rsp.wrapping_add(0x58u64), (0x0u8) as u8); rax = P3; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), (0x6u64) as u64); r9 = 0x6; r8 = 0; fl_l = 0; fl_r = 0; rdx = rsp.wrapping_add(0x40); rcx = rsp.wrapping_add(0x50); rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x118u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); r9 = ((rsi as u32)) as u32 as u64; rdx = (crate::simd_ops::ru64(r14)) as u64; rcx = rbx; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rdi, rax); rcx = rsp.wrapping_add(0x50); rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); rax = rdi; r11 = rsp.wrapping_add(0xe0); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x10u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x18u64))) as u64; rdi = (crate::simd_ops::ru64(r11.wrapping_add(0x20u64))) as u64; rsp = r11; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1412fc430(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142535070; const P1: u64 = 0x1426456c8; const P2: u64 = 0x1426453a0; const P3: u64 = 0x1433f48d8; const P4: u64 = 0x142409b50; const P5: u64 = 0x1426cf8f0; const P6: u64 = 0x1423242e0; const P7: u64 = 0x14240ab10;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 27>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), r8); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0xd0u64)) as u64; rsi = r9; rdi = rcx; rcx = 0x6348; rax = ((crate::exe::tls_pointer() as u64)) as u64; rdx = (crate::simd_ops::ru64(rax)) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(rdx))) as u64; rdx = 0x4f8; r8 = 0x10; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rax; rdx = 0xdeadd00d; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 2; continue; }
            2 => { crate::simd_ops::wu64(rsp.wrapping_add(0xf0u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x44u64), (0x10000010u64) as u64); crate::simd_ops::wu8(rsp.wrapping_add(0x4cu64), (0x0u8) as u8); rax = P3; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), (0xau64) as u64); r9 = 0xa; r8 = 0; fl_l = 0; fl_r = 0; rdx = rsp.wrapping_add(0x30); rcx = rsp.wrapping_add(0x44); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = P5; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), (0x10u64) as u64); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x30u64))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm0) as u128); r8 = (crate::simd_ops::ru64(rsi)) as u64; rdx = rsp.wrapping_add(0x30); rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rdi, rax); rcx = rsp.wrapping_add(0x44); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = rdi; r11 = rsp.wrapping_add(0xd0); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x10u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x18u64))) as u64; rsp = r11; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1412fc530(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142535070; const P1: u64 = 0x1426456c8; const P2: u64 = 0x1426453a0; const P3: u64 = 0x1433f48d8; const P4: u64 = 0x142409b50; const P5: u64 = 0x142327360; const P6: u64 = 0x14240ab10;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 29>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rdi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r9); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0xe0u64)) as u64; r14 = r8; rdi = rcx; rcx = 0x6348; rax = ((crate::exe::tls_pointer() as u64)) as u64; rdx = (crate::simd_ops::ru64(rax)) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(rdx))) as u64; rdx = (crate::simd_ops::ru64(rcx.wrapping_add(0x138u64))) as u64; rbx = (crate::simd_ops::ru64(rdx.wrapping_add(0x10u64))) as u64; rax = rbx; rax = ((((rax as u32)) & (0xfu32))) as u32 as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::lk_add_u32(rdx.wrapping_add(0x18u64), (1) as u32)) as u32 as u64; rax = (crate::simd_ops::ru64(rbx)) as u64; crate::simd_ops::wu64(rdx.wrapping_add(0x10u64), rax); bb = 5; continue; }
            3 => { rdx = 0x130; r8 = 0x10; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rbx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp); rcx = rax; rdx = 0xdeadd00d; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); bb = 5; continue; }
            5 => { crate::simd_ops::wu64(rsp.wrapping_add(0x108u64), rbx); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x110u64))) as u64; rsi = (crate::simd_ops::ru32(rax)) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), (0x10000010u64) as u64); crate::simd_ops::wu8(rsp.wrapping_add(0x58u64), (0x0u8) as u8); rax = P3; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), (0xau64) as u64); r9 = 0xa; r8 = 0; fl_l = 0; fl_r = 0; rdx = rsp.wrapping_add(0x40); rcx = rsp.wrapping_add(0x50); rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x118u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); r9 = ((rsi as u32)) as u32 as u64; rdx = (crate::simd_ops::ru64(r14)) as u64; rcx = rbx; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rdi, rax); rcx = rsp.wrapping_add(0x50); rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); rax = rdi; r11 = rsp.wrapping_add(0xe0); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x10u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x18u64))) as u64; rdi = (crate::simd_ops::ru64(r11.wrapping_add(0x20u64))) as u64; rsp = r11; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1412fc650(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x142535070; const P2: u64 = 0x1426456c8; const P3: u64 = 0x1426453a0; const P4: u64 = 0x1423c0410;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x40u64)) as u64;
    rdi = rcx;
    rbx = 0;
    fl_l = 0; fl_r = 0;
    rcx = 0x6348;
    rax = ((crate::exe::tls_pointer() as u64)) as u64;
    rdx = (crate::simd_ops::ru64(rax)) as u64;
    rcx = (crate::simd_ops::ru64(rcx.wrapping_add(rdx))) as u64;
    rdx = 0xf0f68;
    r8 = ((rbx.wrapping_add(P0)) as u32) as u32 as u64;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    fl_l = rax; fl_r = rax;
    if !(((((fl_l as u64)) & ((fl_r as u64))) != 0)) { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); rcx = rax; rdx = 0xdeadd00d; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); } else { rbx = rax; }
    crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rbx);
    rcx = rbx;
    rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp);
    crate::simd_ops::wu64(rdi, rax);
    rax = rdi;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64;
    rsp = ((rsp).wrapping_add(0x40u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1412fc6d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x1; const P2: u64 = 0x142535070; const P3: u64 = 0x1426456c8; const P4: u64 = 0x1426453a0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x40u64)) as u64;
    rax = ((crate::exe::tls_pointer() as u64)) as u64;
    rbx = rcx;
    rdi = 0;
    fl_l = 0; fl_r = 0;
    rcx = 0x6348;
    rdx = (crate::simd_ops::ru64(rax)) as u64;
    r8 = ((rdi.wrapping_add(P0)) as u32) as u32 as u64;
    rcx = (crate::simd_ops::ru64(rcx.wrapping_add(rdx))) as u64;
    rdx = ((rdi.wrapping_add(P1)) as u32) as u32 as u64;
    rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp);
    fl_l = rax; fl_r = rax;
    if !(((((fl_l as u64)) & ((fl_r as u64))) != 0)) { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); rcx = rax; rdx = 0xdeadd00d; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp); } else { rdi = rax; }
    crate::simd_ops::wu64(rbx, rdi);
    rax = rbx;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64;
    rsp = ((rsp).wrapping_add(0x40u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1412fc730(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x78; const P1: u64 = 0x10; const P2: u64 = 0x18; const P3: u64 = 0x1422fc440; const P4: u64 = 0xffffffffffffffa0; const P5: u64 = 0x142535070; const P6: u64 = 0x1426456c8; const P7: u64 = 0x1426453a0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rsi = r8; rdi = rcx; rbx = 0; fl_l = 0; fl_r = 0; rcx = 0x6348; rax = ((crate::exe::tls_pointer() as u64)) as u64; rdx = (crate::simd_ops::ru64(rax)) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(rdx))) as u64; r9 = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rdx = (crate::simd_ops::ru64(r9.wrapping_add(P1))) as u64; rax = rdx; rax = ((((rax as u32)) & (0xfu32))) as u32 as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 1; continue; }
            1 => { fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 5; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::lk_add_u32(r9.wrapping_add(P2), (1) as u32)) as u32 as u64; rax = (crate::simd_ops::ru64(rdx)) as u64; crate::simd_ops::wu64(r9.wrapping_add(P1), rax); bb = 3; continue; }
            3 => { rbx = rdx; bb = 4; continue; }
            4 => { crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rbx); rdx = (crate::simd_ops::ru32(rsi)) as u32 as u64; rcx = rbx; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rdi, rax); rax = rdi; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            5 => { rdx = 0x70; r8 = ((rdx.wrapping_add(P4)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rdx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 3; continue; } bb = 6; continue; }
            6 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P6), rax), rax, rcx, rdx, r8, r9, rsp); rcx = rax; rdx = 0xdeadd00d; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P7), rax), rax, rcx, rdx, r8, r9, rsp); bb = 4; continue; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1412fc7e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x38; const P1: u64 = 0x10; const P2: u64 = 0x18; const P3: u64 = 0x142396810; const P4: u64 = 0xffffffffffffffe0; const P5: u64 = 0x142535070; const P6: u64 = 0x1426456c8; const P7: u64 = 0x1426453a0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rdi = rcx; rbx = 0; fl_l = 0; fl_r = 0; rcx = 0x6348; rax = ((crate::exe::tls_pointer() as u64)) as u64; rdx = (crate::simd_ops::ru64(rax)) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(rdx))) as u64; r8 = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rdx = (crate::simd_ops::ru64(r8.wrapping_add(P1))) as u64; rax = rdx; rax = ((((rax as u32)) & (0xfu32))) as u32 as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 1; continue; }
            1 => { fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 5; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::lk_add_u32(r8.wrapping_add(P2), (1) as u32)) as u32 as u64; rax = (crate::simd_ops::ru64(rdx)) as u64; crate::simd_ops::wu64(r8.wrapping_add(P1), rax); bb = 3; continue; }
            3 => { rbx = rdx; bb = 4; continue; }
            4 => { crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rbx); rcx = rbx; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rdi, rax); rax = rdi; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            5 => { rdx = 0x30; r8 = ((rdx.wrapping_add(P4)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rdx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 3; continue; } bb = 6; continue; }
            6 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P6), rax), rax, rcx, rdx, r8, r9, rsp); rcx = rax; rdx = 0xdeadd00d; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P7), rax), rax, rcx, rdx, r8, r9, rsp); bb = 4; continue; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1412fc920(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x108; const P1: u64 = 0x10; const P2: u64 = 0x142537280; const P3: u64 = 0x1426456c8; const P4: u64 = 0x1426453a0; const P5: u64 = 0x18; const P6: u64 = 0x142391e80;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rdi = rcx; rbx = 0; fl_l = 0; fl_r = 0; rcx = 0x6348; rax = ((crate::exe::tls_pointer() as u64)) as u64; rdx = (crate::simd_ops::ru64(rax)) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(rdx))) as u64; r8 = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rdx = (crate::simd_ops::ru64(r8.wrapping_add(P1))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { r9 = 0; fl_l = 0; fl_r = 0; r8 = 0; fl_l = 0; fl_r = 0; rdx = 0x100; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rdx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 4; continue; } bb = 2; continue; }
            2 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); rcx = rax; rdx = 0xdeadd00d; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp); bb = 5; continue; }
            3 => { fl_l = (crate::simd_ops::lk_add_u32(r8.wrapping_add(P5), (1) as u32)) as u32 as u64; rax = (crate::simd_ops::ru64(rdx)) as u64; crate::simd_ops::wu64(r8.wrapping_add(P1), rax); bb = 4; continue; }
            4 => { rbx = rdx; bb = 5; continue; }
            5 => { crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rbx); rcx = rbx; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rdi, rax); rax = rdi; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1412fc9c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x120; const P1: u64 = 0x10; const P2: u64 = 0x18; const P3: u64 = 0x142379c70; const P4: u64 = 0x142535070; const P5: u64 = 0x1426456c8; const P6: u64 = 0x1426453a0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x1412fc9c0, funcinfo_rva: 0x359b5c0, handler: 1, shape: 2, frame_reg: 0, establisher_off: 0x48, conts: &[] };
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rdx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rdi = (rcx) as u64; rbx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rcx = (0x6348u32) as u32 as u64; rax = ((crate::exe::tls_pointer() as u64)) as u64; rdx = (crate::simd_ops::ru64(rax)) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(rdx))) as u64; r8 = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rdx = (crate::simd_ops::ru64(r8.wrapping_add(P1))) as u64; rax = (rdx) as u64; rax = ((((rax as u32)) & (0xfu32))) as u32 as u64; fl_l = (rdx) as u64 as u64; fl_r = (rdx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 1; continue; }
            1 => { fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 5; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::lk_add_u32(r8.wrapping_add(P2), (1) as u32)) as u32 as u64; rax = (crate::simd_ops::ru64(rdx)) as u64; crate::simd_ops::wu64(r8.wrapping_add(P1), (rax) as u64); bb = 3; continue; }
            3 => { rbx = (rdx) as u64; bb = 4; continue; }
            4 => { crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), (rbx) as u64); rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1412fca1a); crate::simd_ops::replace_rax_in(rax); rax = match eh.call(0x1412fca1a, (P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8) { Ok(v) => v, Err(l) => l.unknown(0x1412fc9c0) }; crate::simd_ops::wu64(rdi, (rax) as u64); rax = (rdi) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            5 => { rdx = (0x118u32) as u32 as u64; r8 = (0x10u32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1412fca3b); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rdx = (rax) as u64; fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 3; continue; } bb = 6; continue; }
            6 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1412fca48); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P5)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rcx = (rax) as u64; rdx = (0xdeadd00du32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1412fca56); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P6)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 4; continue; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1412fca60(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142535070; const P1: u64 = 0x1426456c8; const P2: u64 = 0x1426453a0; const P3: u64 = 0x143869be0; const P4: u64 = 0x1433f17c0; const P5: u64 = 0x1433f48c8; const P6: u64 = 0x1412cf0e0; const P7: u64 = 0x1433f48b8; const P8: u64 = 0x142242db0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x1412fca60, funcinfo_rva: 0x36990d0, handler: 1, shape: 2, frame_reg: 0, establisher_off: 0x68, conts: &[] };
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (r9) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rdx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r15) as u64); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; r14 = (r8) as u64; rsi = (rcx) as u64; r15 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rcx = (0x6348u32) as u32 as u64; rax = ((crate::exe::tls_pointer() as u64)) as u64; rdx = (crate::simd_ops::ru64(rax)) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(rdx))) as u64; rdx = (crate::simd_ops::ru64(rcx.wrapping_add(0x80u64))) as u64; rdi = (crate::simd_ops::ru64(rdx.wrapping_add(0x10u64))) as u64; rax = (rdi) as u64; rax = ((((rax as u32)) & (0xfu32))) as u32 as u64; fl_l = (rdi) as u64 as u64; fl_r = (rdi) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::lk_add_u32(rdx.wrapping_add(0x18u64), (1) as u32)) as u32 as u64; rax = (crate::simd_ops::ru64(rdi)) as u64; crate::simd_ops::wu64(rdx.wrapping_add(0x10u64), (rax) as u64); bb = 5; continue; }
            3 => { rdx = (0x78u32) as u32 as u64; r8 = ((rdx.wrapping_sub(0x68u64)) as u32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1412fcac3); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rdi = (rax) as u64; fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1412fcad0); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P1)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rcx = (rax) as u64; rdx = (0xdeadd00du32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1412fcade); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P2)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rdi = (r15) as u64; bb = 5; continue; }
            5 => { crate::simd_ops::wu64(rsp.wrapping_add(0x78u64), (rdi) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x80u64), (rdi) as u64); rbx = (crate::simd_ops::ru64(P3)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x88u64), (rbx) as u64); r14 = ((crate::simd_ops::ru8(r14)) as u32) as u32 as u64; rax = ((P4) as u64) as u64; crate::simd_ops::wu64(rdi, (rax) as u64); crate::simd_ops::wu32(rdi.wrapping_add(0x10u64), (0xffffffffu32) as u32); crate::simd_ops::wu32(rdi.wrapping_add(0x14u64), ((r15 as u32)) as u32); crate::simd_ops::wu64(rdi.wrapping_add(0x8u64), (r15) as u64); crate::simd_ops::wu64(rdi.wrapping_add(0x18u64), (r15) as u64); crate::simd_ops::wu64(rdi.wrapping_add(0x20u64), (r15) as u64); crate::simd_ops::wu64(rdi.wrapping_add(0x28u64), (r15) as u64); rcx = ((rdi.wrapping_add(0x30u64)) as u64) as u64; rdx = ((P5) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1412fcb37); crate::simd_ops::replace_rax_in(rax); rax = match eh.call(0x1412fcb37, (P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8) { Ok(v) => v, Err(l) => l.unknown(0x1412fca60) }; crate::simd_ops::wu32(rdi.wrapping_add(0x54u64), ((r15 as u32)) as u32); crate::simd_ops::wu64(rdi.wrapping_add(0x68u64), (r15) as u64); crate::simd_ops::wu64(rdi.wrapping_add(0x70u64), (r15) as u64); crate::simd_ops::wu32(rdi.wrapping_add(0x5cu64), ((rbx as u32)) as u32); rcx = (crate::simd_ops::ru32(rsp.wrapping_add(0x8cu64))) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x58u64), ((rcx as u32)) as u32); crate::simd_ops::wu32(rdi.wrapping_add(0x60u64), (0x200u32) as u32); crate::simd_ops::wu8(rdi.wrapping_add(0x50u64), ((r14 as u8)) as u8); rax = ((P7) as u64) as u64; crate::simd_ops::wu64(rdi.wrapping_add(0x48u64), (rax) as u64); rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x5cu64))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x88u64), ((rax as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x8cu64), ((rcx as u32)) as u32); rdx = ((rsp.wrapping_add(0x88u64)) as u64) as u64; rcx = (rdi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1412fcb88); crate::simd_ops::replace_rax_in(rax); rax = match eh.call(0x1412fcb88, (P8) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8) { Ok(v) => v, Err(l) => l.unknown(0x1412fca60) }; crate::simd_ops::wu64(rsi, (rdi) as u64); rax = (rsi) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; r15 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rsi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rbx = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1412fcbb0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x142535070; const P2: u64 = 0x1426456c8; const P3: u64 = 0x1426453a0; const P4: u64 = 0x142243f50;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x1412fcbb0, funcinfo_rva: 0x3699124, handler: 1, shape: 2, frame_reg: 0, establisher_off: 0x48, conts: &[] };
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rsi) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (r8) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rsi = (r9) as u64; rdi = (rcx) as u64; rbx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rcx = (0x6348u32) as u32 as u64; rax = ((crate::exe::tls_pointer() as u64)) as u64; rdx = (crate::simd_ops::ru64(rax)) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(rdx))) as u64; rdx = (0x2058u32) as u32 as u64; r8 = ((rbx.wrapping_add(P0)) as u32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1412fcbea); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1412fcbf4); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P2)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rcx = (rax) as u64; rdx = (0xdeadd00du32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1412fcc02); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P3)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 3; continue; }
            2 => { rbx = (rax) as u64; bb = 3; continue; }
            3 => { crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), (rbx) as u64); r9 = (r9 & !0xffu64) | (((0xbu8) as u8 as u64)); r8 = (crate::simd_ops::ru32(rsi)) as u32 as u64; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1412fcc1b); crate::simd_ops::replace_rax_in(rax); rax = match eh.call(0x1412fcc1b, (P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8) { Ok(v) => v, Err(l) => l.unknown(0x1412fcbb0) }; crate::simd_ops::wu64(rdi, (rax) as u64); rax = (rdi) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1412fcc40(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x40; const P1: u64 = 0x10; const P2: u64 = 0x18; const P3: u64 = 0x14224b6a0; const P4: u64 = 0xffffffffffffffd8; const P5: u64 = 0x142535070; const P6: u64 = 0x1426456c8; const P7: u64 = 0x1426453a0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rdi = rcx; rbx = 0; fl_l = 0; fl_r = 0; rcx = 0x6348; rax = ((crate::exe::tls_pointer() as u64)) as u64; rdx = (crate::simd_ops::ru64(rax)) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(rdx))) as u64; r8 = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rdx = (crate::simd_ops::ru64(r8.wrapping_add(P1))) as u64; rax = rdx; rax = ((((rax as u32)) & (0xfu32))) as u32 as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 1; continue; }
            1 => { fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 5; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::lk_add_u32(r8.wrapping_add(P2), (1) as u32)) as u32 as u64; rax = (crate::simd_ops::ru64(rdx)) as u64; crate::simd_ops::wu64(r8.wrapping_add(P1), rax); bb = 3; continue; }
            3 => { rbx = rdx; bb = 4; continue; }
            4 => { crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rbx); rcx = rbx; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rdi, rax); rax = rdi; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            5 => { rdx = 0x38; r8 = ((rdx.wrapping_add(P4)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rdx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 3; continue; } bb = 6; continue; }
            6 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P6), rax), rax, rcx, rdx, r8, r9, rsp); rcx = rax; rdx = 0xdeadd00d; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P7), rax), rax, rcx, rdx, r8, r9, rsp); bb = 4; continue; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1412fcce0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x18; const P1: u64 = 0x10; const P2: u64 = 0x142537280; const P3: u64 = 0x1426456c8; const P4: u64 = 0x1426453a0; const P5: u64 = 0x142204110;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x1412fcce0, funcinfo_rva: 0x369914c, handler: 1, shape: 2, frame_reg: 0, establisher_off: 0x38, conts: &[] };
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rdx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rdi = (rcx) as u64; rbx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rcx = (0x6348u32) as u32 as u64; rax = ((crate::exe::tls_pointer() as u64)) as u64; rdx = (crate::simd_ops::ru64(rax)) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(rdx))) as u64; r8 = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rdx = (crate::simd_ops::ru64(r8.wrapping_add(P1))) as u64; fl_l = (rdx) as u64 as u64; fl_r = (rdx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { r9 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; r8 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rdx = ((rbx.wrapping_add(P1)) as u32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1412fcd1f); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rdx = (rax) as u64; fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 4; continue; } bb = 2; continue; }
            2 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1412fcd2c); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P3)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rcx = (rax) as u64; rdx = (0xdeadd00du32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1412fcd3a); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P4)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 5; continue; }
            3 => { fl_l = (crate::simd_ops::lk_add_u32(r8.wrapping_add(P0), (1) as u32)) as u32 as u64; rax = (crate::simd_ops::ru64(rdx)) as u64; crate::simd_ops::wu64(r8.wrapping_add(P1), (rax) as u64); bb = 4; continue; }
            4 => { rbx = (rdx) as u64; bb = 5; continue; }
            5 => { crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), (rbx) as u64); rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1412fcd58); crate::simd_ops::replace_rax_in(rax); rax = match eh.call(0x1412fcd58, (P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8) { Ok(v) => v, Err(l) => l.unknown(0x1412fcce0) }; crate::simd_ops::wu64(rdi, (rax) as u64); rax = (rdi) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1412fce10(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142535070; const P1: u64 = 0x1426456c8; const P2: u64 = 0x1426453a0; const P3: u64 = 0x1422230e0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x1412fce10, funcinfo_rva: 0x3699178, handler: 1, shape: 2, frame_reg: 0, establisher_off: 0x58, conts: &[] };
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (rsi) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rdx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = ((rsp).wrapping_sub(0x50u64)) as u64; rdi = (r8) as u64; rsi = (rcx) as u64; rbx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rcx = (0x6348u32) as u32 as u64; rax = ((crate::exe::tls_pointer() as u64)) as u64; rdx = (crate::simd_ops::ru64(rax)) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(rdx))) as u64; r8 = (crate::simd_ops::ru64(rcx.wrapping_add(0x1a8u64))) as u64; rdx = (crate::simd_ops::ru64(r8.wrapping_add(0x10u64))) as u64; rax = (rdx) as u64; rax = ((((rax as u32)) & (0xfu32))) as u32 as u64; fl_l = (rdx) as u64 as u64; fl_r = (rdx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 1; continue; }
            1 => { fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 6; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::lk_add_u32(r8.wrapping_add(0x18u64), (1) as u32)) as u32 as u64; rax = (crate::simd_ops::ru64(rdx)) as u64; crate::simd_ops::wu64(r8.wrapping_add(0x10u64), (rax) as u64); bb = 3; continue; }
            3 => { rbx = (rdx) as u64; bb = 4; continue; }
            4 => { crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), (rbx) as u64); rdx = ((rdi.wrapping_add(0x8u64)) as u64) as u64; rcx = ((crate::simd_ops::ru8(rdi.wrapping_add(0x3u64))) as u32) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = (0x80u32) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 8; continue; } bb = 5; continue; }
            5 => { rcx = ((rcx.wrapping_mul(8)) as u32) as u32 as u64; bb = 9; continue; }
            6 => { rdx = (0x1a0u32) as u32 as u64; r8 = (0x10u32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1412fce93); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdx = (rax) as u64; fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 3; continue; } bb = 7; continue; }
            7 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1412fcea0); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P1)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = (rax) as u64; rdx = (0xdeadd00du32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1412fceae); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P2)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 4; continue; }
            8 => { rcx = (((rcx as u32)).wrapping_shl(8u32)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = (0x7c00u32) as u32 as u64; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 9; continue; }
            9 => { rax = (crate::simd_ops::ru32(rdi)) as u32 as u64; rax = ((((rax as u32)) & (0xffffffu32))) as u32 as u64; rax = (((rax as u32)).wrapping_shl(3u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 11; continue; } bb = 10; continue; }
            10 => { rdx = (crate::simd_ops::ru64(rdx)) as u64; bb = 11; continue; }
            11 => { rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x4u64))) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), (rdx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), (rax) as u64); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x40u64))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm0) as u128); rdx = ((rsp.wrapping_add(0x40u64)) as u64) as u64; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1412fcef1); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = match eh.call_xmm(0x1412fcef1, (P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]) { Ok(v) => v, Err(l) => l.unknown(0x1412fce10) }; rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu64(rsi, (rax) as u64); rax = (rsi) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rsp = ((rsp).wrapping_add(0x50u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1412fcf10(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xc8; const P1: u64 = 0x10; const P2: u64 = 0x18; const P3: u64 = 0x1421e9b20; const P4: u64 = 0x142535070; const P5: u64 = 0x1426456c8; const P6: u64 = 0x1426453a0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r9); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rdi = rcx; rbx = 0; fl_l = 0; fl_r = 0; rcx = 0x6348; rax = ((crate::exe::tls_pointer() as u64)) as u64; rdx = (crate::simd_ops::ru64(rax)) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(rdx))) as u64; r8 = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rdx = (crate::simd_ops::ru64(r8.wrapping_add(P1))) as u64; rax = rdx; rax = ((((rax as u32)) & (0xfu32))) as u32 as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 1; continue; }
            1 => { fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 5; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::lk_add_u32(r8.wrapping_add(P2), (1) as u32)) as u32 as u64; rax = (crate::simd_ops::ru64(rdx)) as u64; crate::simd_ops::wu64(r8.wrapping_add(P1), rax); bb = 3; continue; }
            3 => { rbx = rdx; bb = 4; continue; }
            4 => { crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), rbx); rcx = rbx; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rdi, rax); rax = rdi; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            5 => { rdx = 0xc0; r8 = 0x10; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rdx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 3; continue; } bb = 6; continue; }
            6 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp); rcx = rax; rdx = 0xdeadd00d; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P6), rax), rax, rcx, rdx, r8, r9, rsp); bb = 4; continue; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1412fcfb0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xd0; const P1: u64 = 0x10; const P2: u64 = 0x18; const P3: u64 = 0x1423b6c90; const P4: u64 = 0x142535070; const P5: u64 = 0x1426456c8; const P6: u64 = 0x1426453a0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rdi = rcx; rbx = 0; fl_l = 0; fl_r = 0; rcx = 0x6348; rax = ((crate::exe::tls_pointer() as u64)) as u64; rdx = (crate::simd_ops::ru64(rax)) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(rdx))) as u64; r8 = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rdx = (crate::simd_ops::ru64(r8.wrapping_add(P1))) as u64; rax = rdx; rax = ((((rax as u32)) & (0xfu32))) as u32 as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 1; continue; }
            1 => { fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 5; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::lk_add_u32(r8.wrapping_add(P2), (1) as u32)) as u32 as u64; rax = (crate::simd_ops::ru64(rdx)) as u64; crate::simd_ops::wu64(r8.wrapping_add(P1), rax); bb = 3; continue; }
            3 => { rbx = rdx; bb = 4; continue; }
            4 => { crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rbx); rcx = rbx; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rdi, rax); rax = rdi; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            5 => { rdx = 0xc8; r8 = 0x10; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rdx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 3; continue; } bb = 6; continue; }
            6 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp); rcx = rax; rdx = 0xdeadd00d; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P6), rax), rax, rcx, rdx, r8, r9, rsp); bb = 4; continue; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1412fd050(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142537280; const P1: u64 = 0x1426456c8; const P2: u64 = 0x1426453a0; const P3: u64 = 0x143a00238; const P4: u64 = 0x1434059b8; const P5: u64 = 0x143a00210; const P6: u64 = 0x1423970b0; const P7: u64 = 0x1434059a8; const P8: u64 = 0x142398a00; const P9: u64 = 0x1434059d8; const P10: u64 = 0x1434059c8; const P11: u64 = 0x143405978;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rdi = rcx; rbx = 0; fl_l = 0; fl_r = 0; rcx = 0x6348; rax = ((crate::exe::tls_pointer() as u64)) as u64; rdx = (crate::simd_ops::ru64(rax)) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(rdx))) as u64; r8 = (crate::simd_ops::ru64(rcx.wrapping_add(0x28u64))) as u64; rdx = (crate::simd_ops::ru64(r8.wrapping_add(0x10u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { r9 = 0; fl_l = 0; fl_r = 0; r8 = 0; fl_l = 0; fl_r = 0; rdx = ((rbx.wrapping_add(0x20u64)) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 4; continue; } bb = 2; continue; }
            2 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rax; rdx = 0xdeadd00d; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 5; continue; }
            3 => { fl_l = (crate::simd_ops::lk_add_u32(r8.wrapping_add(0x18u64), (1) as u32)) as u32 as u64; rax = (crate::simd_ops::ru64(rdx)) as u64; crate::simd_ops::wu64(r8.wrapping_add(0x10u64), rax); bb = 4; continue; }
            4 => { rbx = rdx; bb = 5; continue; }
            5 => { crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rbx); crate::simd_ops::wu64(P3, rbx); rax = P4; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), (0x8u64) as u64); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x30u64))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm0) as u128); rdx = rsp.wrapping_add(0x30); rcx = (crate::simd_ops::ru64(P5)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r11 = rax; rax = P7; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), (0x9u64) as u64); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x30u64))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm0) as u128); rdx = rsp.wrapping_add(0x30); rcx = r11; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P8, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rbx, rax); rax = P9; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), (0x8u64) as u64); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x30u64))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm0) as u128); rdx = rsp.wrapping_add(0x30); rcx = r11; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P8, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rbx.wrapping_add(0x8u64), rax); rax = P10; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), (0x8u64) as u64); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x30u64))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm0) as u128); rdx = rsp.wrapping_add(0x30); rcx = r11; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P8, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), rax); rax = P11; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), (0xeu64) as u64); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x30u64))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm0) as u128); rdx = rsp.wrapping_add(0x30); rcx = r11; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P8, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rbx.wrapping_add(0x18u64), rax); crate::simd_ops::wu64(rdi, rbx); rax = rdi; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1412fd1e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x28; const P1: u64 = 0x10; const P2: u64 = 0x20; const P3: u64 = 0x142537280; const P4: u64 = 0x1426456c8; const P5: u64 = 0x1426453a0; const P6: u64 = 0x18; const P7: u64 = 0x8; const P8: u64 = 0xc; const P9: u64 = 0x14; const P10: u64 = 0x1433f9068; const P11: u64 = 0x143a00210; const P12: u64 = 0x1423970b0; const P13: u64 = 0x1433ef278; const P14: u64 = 0x142398a00;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rdi = rcx; rcx = 0x6348; rax = ((crate::exe::tls_pointer() as u64)) as u64; rdx = (crate::simd_ops::ru64(rax)) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(rdx))) as u64; rdx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rbx = (crate::simd_ops::ru64(rdx.wrapping_add(P1))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { r9 = 0; fl_l = 0; fl_r = 0; r8 = 0; fl_l = 0; fl_r = 0; rdx = ((rbx.wrapping_add(P2)) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 4; continue; } bb = 2; continue; }
            2 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rax; rdx = 0xdeadd00d; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 4; continue; }
            3 => { fl_l = (crate::simd_ops::lk_add_u32(rdx.wrapping_add(P6), (1) as u32)) as u32 as u64; rax = (crate::simd_ops::ru64(rbx)) as u64; crate::simd_ops::wu64(rdx.wrapping_add(P1), rax); bb = 4; continue; }
            4 => { crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rbx); crate::simd_ops::wu32(rbx.wrapping_add(P7), (0x3f800000u32) as u32); crate::simd_ops::wu32(rbx.wrapping_add(P8), (0x3f800000u32) as u32); crate::simd_ops::wu32(rbx.wrapping_add(P1), (0x3f800000u32) as u32); crate::simd_ops::wu64(rbx.wrapping_add(P9), (0x3f800000u64) as u64); rax = P10; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), (0x8u64) as u64); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x30u64))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm0) as u128); rdx = rsp.wrapping_add(0x30); rcx = (crate::simd_ops::ru64(P11)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P12, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = P13; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rcx); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), (0x4u64) as u64); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x30u64))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm0) as u128); rdx = rsp.wrapping_add(0x30); rcx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P14, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rbx, rax); crate::simd_ops::wu64(rdi, rbx); rax = rdi; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1412fd430(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142537280; const P1: u64 = 0x1426456c8; const P2: u64 = 0x1426453a0; const P3: u64 = 0x143a00220; const P4: u64 = 0x143404ab8; const P5: u64 = 0x143a00210; const P6: u64 = 0x1423970b0; const P7: u64 = 0x143404a70; const P8: u64 = 0x142398a00; const P9: u64 = 0x14220e240; const P10: u64 = 0x143404a58; const P11: u64 = 0x143404a48; const P12: u64 = 0x143404a30;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x1412fd430, funcinfo_rva: 0x36991f4, handler: 1, shape: 2, frame_reg: 0, establisher_off: 0x48, conts: &[] };
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rsi) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rdx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rdi = (rcx) as u64; rsi = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rcx = (0x6348u32) as u32 as u64; rax = ((crate::exe::tls_pointer() as u64)) as u64; rdx = (crate::simd_ops::ru64(rax)) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(rdx))) as u64; rdx = (crate::simd_ops::ru64(rcx.wrapping_add(0x48u64))) as u64; rbx = (crate::simd_ops::ru64(rdx.wrapping_add(0x10u64))) as u64; fl_l = (rbx) as u64 as u64; fl_r = (rbx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { r9 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; r8 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rdx = ((rsi.wrapping_add(0x40u64)) as u32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1412fd474); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rbx = (rax) as u64; fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 4; continue; } bb = 2; continue; }
            2 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1412fd481); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P1)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = (rax) as u64; rdx = (0xdeadd00du32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1412fd48f); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P2)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rbx = ((rsi as u32)) as u32 as u64; bb = 4; continue; }
            3 => { fl_l = (crate::simd_ops::lk_add_u32(rdx.wrapping_add(0x18u64), (1) as u32)) as u32 as u64; rax = (crate::simd_ops::ru64(rbx)) as u64; crate::simd_ops::wu64(rdx.wrapping_add(0x10u64), (rax) as u64); bb = 4; continue; }
            4 => { crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), (rbx) as u64); crate::simd_ops::wu64(rbx.wrapping_add(0x20u64), (rsi) as u64); crate::simd_ops::wu64(rbx.wrapping_add(0x28u64), (rsi) as u64); crate::simd_ops::wu32(rbx.wrapping_add(0x34u64), (0x42800000u32) as u32); crate::simd_ops::wu64(P3, (rbx) as u64); rax = ((P4) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (rax) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), (0x10u64) as u64); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x30u64))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm0) as u128); rdx = ((rsp.wrapping_add(0x30u64)) as u64) as u64; rcx = (crate::simd_ops::ru64(P5)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1412fd4f0); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = match eh.call_xmm(0x1412fd4f0, (P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]) { Ok(v) => v, Err(l) => l.unknown(0x1412fd430) }; rax = c_rax; xmm0 = c_x0; rsi = (rax) as u64; rax = ((P7) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (rax) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), (0x11u64) as u64); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x30u64))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm0) as u128); rdx = ((rsp.wrapping_add(0x30u64)) as u64) as u64; rcx = (rsi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1412fd521); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = match eh.call_xmm(0x1412fd521, (P8) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]) { Ok(v) => v, Err(l) => l.unknown(0x1412fd430) }; rax = c_rax; xmm0 = c_x0; rdx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rcx = (rax) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1412fd52b); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = match eh.call_xmm(0x1412fd52b, (P9) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]) { Ok(v) => v, Err(l) => l.unknown(0x1412fd430) }; rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu64(rbx, (rax) as u64); rax = ((P10) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (rax) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), (0x11u64) as u64); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x30u64))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm0) as u128); rdx = ((rsp.wrapping_add(0x30u64)) as u64) as u64; rcx = (rsi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1412fd55c); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = match eh.call_xmm(0x1412fd55c, (P8) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]) { Ok(v) => v, Err(l) => l.unknown(0x1412fd430) }; rax = c_rax; xmm0 = c_x0; rdx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rcx = (rax) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1412fd566); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = match eh.call_xmm(0x1412fd566, (P9) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]) { Ok(v) => v, Err(l) => l.unknown(0x1412fd430) }; rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu64(rbx.wrapping_add(0x8u64), (rax) as u64); rax = ((P11) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (rax) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), (0xeu64) as u64); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x30u64))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm0) as u128); rdx = ((rsp.wrapping_add(0x30u64)) as u64) as u64; rcx = (rsi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1412fd598); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = match eh.call_xmm(0x1412fd598, (P8) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]) { Ok(v) => v, Err(l) => l.unknown(0x1412fd430) }; rax = c_rax; xmm0 = c_x0; rdx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rcx = (rax) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1412fd5a2); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = match eh.call_xmm(0x1412fd5a2, (P9) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]) { Ok(v) => v, Err(l) => l.unknown(0x1412fd430) }; rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), (rax) as u64); rax = ((P12) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (rax) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), (0x13u64) as u64); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x30u64))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm0) as u128); rdx = ((rsp.wrapping_add(0x30u64)) as u64) as u64; rcx = (rsi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1412fd5d4); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = match eh.call_xmm(0x1412fd5d4, (P8) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]) { Ok(v) => v, Err(l) => l.unknown(0x1412fd430) }; rax = c_rax; xmm0 = c_x0; rdx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rcx = (rax) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1412fd5de); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = match eh.call_xmm(0x1412fd5de, (P9) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]) { Ok(v) => v, Err(l) => l.unknown(0x1412fd430) }; rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu64(rbx.wrapping_add(0x18u64), (rax) as u64); crate::simd_ops::wu32(rbx.wrapping_add(0x34u64), (0x42800000u32) as u32); crate::simd_ops::wu64(rdi, (rbx) as u64); rax = (rdi) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1412fd610(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x18; const P1: u64 = 0x10; const P2: u64 = 0x142537280; const P3: u64 = 0x1426456c8; const P4: u64 = 0x1426453a0; const P5: u64 = 0x14239aae0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rdi = rcx; rbx = 0; fl_l = 0; fl_r = 0; rcx = 0x6348; rax = ((crate::exe::tls_pointer() as u64)) as u64; rdx = (crate::simd_ops::ru64(rax)) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(rdx))) as u64; r8 = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rdx = (crate::simd_ops::ru64(r8.wrapping_add(P1))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { r9 = 0; fl_l = 0; fl_r = 0; r8 = 0; fl_l = 0; fl_r = 0; rdx = ((rbx.wrapping_add(P1)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rdx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 4; continue; } bb = 2; continue; }
            2 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); rcx = rax; rdx = 0xdeadd00d; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp); bb = 5; continue; }
            3 => { fl_l = (crate::simd_ops::lk_add_u32(r8.wrapping_add(P0), (1) as u32)) as u32 as u64; rax = (crate::simd_ops::ru64(rdx)) as u64; crate::simd_ops::wu64(r8.wrapping_add(P1), rax); bb = 4; continue; }
            4 => { rbx = rdx; bb = 5; continue; }
            5 => { crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rbx); rcx = rbx; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rdi, rax); rax = rdi; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1412fd6a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x1; const P2: u64 = 0x142535070; const P3: u64 = 0x1426456c8; const P4: u64 = 0x1426453a0; const P5: u64 = 0x143a001f0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x40u64)) as u64;
    rax = ((crate::exe::tls_pointer() as u64)) as u64;
    rdi = rcx;
    rbx = 0;
    fl_l = 0; fl_r = 0;
    rcx = 0x6348;
    rdx = (crate::simd_ops::ru64(rax)) as u64;
    r8 = ((rbx.wrapping_add(P0)) as u32) as u32 as u64;
    rcx = (crate::simd_ops::ru64(rcx.wrapping_add(rdx))) as u64;
    rdx = ((rbx.wrapping_add(P1)) as u32) as u32 as u64;
    rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp);
    fl_l = rax; fl_r = rax;
    if !(((((fl_l as u64)) & ((fl_r as u64))) != 0)) { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); rcx = rax; rdx = 0xdeadd00d; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp); } else { rbx = rax; }
    crate::simd_ops::wu64(P5, rbx);
    rax = rdi;
    crate::simd_ops::wu64(rdi, rbx);
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64;
    rsp = ((rsp).wrapping_add(0x40u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1412fd710(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x58; const P1: u64 = 0x10; const P2: u64 = 0x18; const P3: u64 = 0x1423c10f0; const P4: u64 = 0xffffffffffffffc0; const P5: u64 = 0x142535070; const P6: u64 = 0x1426456c8; const P7: u64 = 0x1426453a0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rdi = rcx; rbx = 0; fl_l = 0; fl_r = 0; rcx = 0x6348; rax = ((crate::exe::tls_pointer() as u64)) as u64; rdx = (crate::simd_ops::ru64(rax)) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(rdx))) as u64; r8 = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rdx = (crate::simd_ops::ru64(r8.wrapping_add(P1))) as u64; rax = rdx; rax = ((((rax as u32)) & (0xfu32))) as u32 as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 1; continue; }
            1 => { fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 5; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::lk_add_u32(r8.wrapping_add(P2), (1) as u32)) as u32 as u64; rax = (crate::simd_ops::ru64(rdx)) as u64; crate::simd_ops::wu64(r8.wrapping_add(P1), rax); bb = 3; continue; }
            3 => { rbx = rdx; bb = 4; continue; }
            4 => { crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rbx); rcx = rbx; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rdi, rax); rax = rdi; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            5 => { rdx = 0x50; r8 = ((rdx.wrapping_add(P4)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rdx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 3; continue; } bb = 6; continue; }
            6 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P6), rax), rax, rcx, rdx, r8, r9, rsp); rcx = rax; rdx = 0xdeadd00d; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P7), rax), rax, rcx, rdx, r8, r9, rsp); bb = 4; continue; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1412fd7b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x142535070; const P2: u64 = 0x1426456c8; const P3: u64 = 0x1426453a0; const P4: u64 = 0x142226260;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x40u64)) as u64;
    rdi = rcx;
    rbx = 0;
    fl_l = 0; fl_r = 0;
    rcx = 0x6348;
    rax = ((crate::exe::tls_pointer() as u64)) as u64;
    rdx = (crate::simd_ops::ru64(rax)) as u64;
    rcx = (crate::simd_ops::ru64(rcx.wrapping_add(rdx))) as u64;
    rdx = 0x900;
    r8 = ((rbx.wrapping_add(P0)) as u32) as u32 as u64;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    fl_l = rax; fl_r = rax;
    if !(((((fl_l as u64)) & ((fl_r as u64))) != 0)) { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); rcx = rax; rdx = 0xdeadd00d; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); } else { rbx = rax; }
    crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rbx);
    rcx = rbx;
    rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp);
    crate::simd_ops::wu64(rdi, rax);
    rax = rdi;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64;
    rsp = ((rsp).wrapping_add(0x40u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1412fd8d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x68; const P1: u64 = 0x10; const P2: u64 = 0x18; const P3: u64 = 0x1423107f0; const P4: u64 = 0xffffffffffffffb0; const P5: u64 = 0x142535070; const P6: u64 = 0x1426456c8; const P7: u64 = 0x1426453a0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x1412fd8d0, funcinfo_rva: 0x359b5e8, handler: 1, shape: 2, frame_reg: 0, establisher_off: 0x48, conts: &[] };
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rdx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rdi = (rcx) as u64; rbx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rcx = (0x6348u32) as u32 as u64; rax = ((crate::exe::tls_pointer() as u64)) as u64; rdx = (crate::simd_ops::ru64(rax)) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(rdx))) as u64; r8 = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rdx = (crate::simd_ops::ru64(r8.wrapping_add(P1))) as u64; rax = (rdx) as u64; rax = ((((rax as u32)) & (0xfu32))) as u32 as u64; fl_l = (rdx) as u64 as u64; fl_r = (rdx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 1; continue; }
            1 => { fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 5; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::lk_add_u32(r8.wrapping_add(P2), (1) as u32)) as u32 as u64; rax = (crate::simd_ops::ru64(rdx)) as u64; crate::simd_ops::wu64(r8.wrapping_add(P1), (rax) as u64); bb = 3; continue; }
            3 => { rbx = (rdx) as u64; bb = 4; continue; }
            4 => { crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), (rbx) as u64); rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1412fd927); crate::simd_ops::replace_rax_in(rax); rax = match eh.call(0x1412fd927, (P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8) { Ok(v) => v, Err(l) => l.unknown(0x1412fd8d0) }; crate::simd_ops::wu64(rdi, (rax) as u64); rax = (rdi) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            5 => { rdx = (0x60u32) as u32 as u64; r8 = ((rdx.wrapping_add(P4)) as u32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1412fd946); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rdx = (rax) as u64; fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 3; continue; } bb = 6; continue; }
            6 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1412fd953); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P6)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rcx = (rax) as u64; rdx = (0xdeadd00du32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1412fd961); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P7)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 4; continue; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1412fd970(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142535070; const P1: u64 = 0x1426456c8; const P2: u64 = 0x1426453a0; const P3: u64 = 0x1434061c0; const P4: u64 = 0x1411db190; const P5: u64 = 0x140200580; const P6: u64 = 0x141834fc0; const P7: u64 = 0x1411d2db0; const P8: u64 = 0x143a00240;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rdi = rcx; rsi = 0; fl_l = 0; fl_r = 0; rcx = 0x6348; rax = ((crate::exe::tls_pointer() as u64)) as u64; rdx = (crate::simd_ops::ru64(rax)) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(rdx))) as u64; rdx = (crate::simd_ops::ru64(rcx.wrapping_add(0x68u64))) as u64; rbx = (crate::simd_ops::ru64(rdx.wrapping_add(0x10u64))) as u64; rax = rbx; rax = ((((rax as u32)) & (0xfu32))) as u32 as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::lk_add_u32(rdx.wrapping_add(0x18u64), (1) as u32)) as u32 as u64; rax = (crate::simd_ops::ru64(rbx)) as u64; crate::simd_ops::wu64(rdx.wrapping_add(0x10u64), rax); bb = 5; continue; }
            3 => { rdx = 0x60; r8 = ((rdx.wrapping_sub(0x50u64)) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rax; rdx = 0xdeadd00d; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = rsi; bb = 5; continue; }
            5 => { crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rbx); rax = P3; crate::simd_ops::wu64(rbx, rax); crate::simd_ops::wu64(rbx.wrapping_add(0x8u64), rsi); rcx = rbx.wrapping_add(0x10); rax = P4; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); r9 = P5; rdx = 0x8; r8 = ((rdx.wrapping_sub(0x6u64)) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rbx.wrapping_add(0x20); rax = P7; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); r9 = P5; rdx = 0x8; r8 = ((rdx.wrapping_sub(0x6u64)) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rbx.wrapping_add(0x30u64), rsi); xmm0 = 0u128; crate::simd_ops::wu32(rsp.wrapping_add(0x58u64), ((xmm0 as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x5cu64), ((xmm0 as u32)) as u32); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x38u64), rax); crate::simd_ops::wu64(rbx.wrapping_add(0x40u64), rsi); crate::simd_ops::wu64(rbx.wrapping_add(0x48u64), rsi); crate::simd_ops::wu64(rbx.wrapping_add(0x50u64), rsi); crate::simd_ops::wu16(rbx.wrapping_add(0x58u64), (0x101u16) as u16); crate::simd_ops::wu64(P8, rbx); crate::simd_ops::wu64(rdi, rbx); rax = rdi; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1412fdaa0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x142535070; const P2: u64 = 0x1426456c8; const P3: u64 = 0x1426453a0; const P4: u64 = 0x14220cc60;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x1412fdaa0, funcinfo_rva: 0x369926c, handler: 1, shape: 2, frame_reg: 0, establisher_off: 0x48, conts: &[] };
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rsi) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (r8) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rsi = (r9) as u64; rdi = (rcx) as u64; rbx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rcx = (0x6348u32) as u32 as u64; rax = ((crate::exe::tls_pointer() as u64)) as u64; rdx = (crate::simd_ops::ru64(rax)) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(rdx))) as u64; rdx = (0xc0e8u32) as u32 as u64; r8 = ((rbx.wrapping_add(P0)) as u32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1412fdada); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1412fdae4); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P2)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rcx = (rax) as u64; rdx = (0xdeadd00du32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1412fdaf2); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P3)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 3; continue; }
            2 => { rbx = (rax) as u64; bb = 3; continue; }
            3 => { crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), (rbx) as u64); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; rdx = ((crate::simd_ops::ru8(rax)) as u32) as u32 as u64; crate::simd_ops::wu8(rsp.wrapping_add(0x20u64), ((rdx as u8)) as u8); r9 = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; r9 = (crate::simd_ops::ru32(r9)) as u32 as u64; r8 = (crate::simd_ops::ru32(rsi)) as u32 as u64; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1412fdb1c); crate::simd_ops::replace_rax_in(rax); rax = match eh.call(0x1412fdb1c, (P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8) { Ok(v) => v, Err(l) => l.unknown(0x1412fdaa0) }; crate::simd_ops::wu64(rdi, (rax) as u64); rax = (rdi) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1412fdb40(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x20; const P1: u64 = 0x10; const P2: u64 = 0x18; const P3: u64 = 0x14236a490; const P4: u64 = 0xfffffffffffffff8; const P5: u64 = 0x142535070; const P6: u64 = 0x1426456c8; const P7: u64 = 0x1426453a0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rdi = rcx; rbx = 0; fl_l = 0; fl_r = 0; rcx = 0x6348; rax = ((crate::exe::tls_pointer() as u64)) as u64; rdx = (crate::simd_ops::ru64(rax)) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(rdx))) as u64; r8 = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rdx = (crate::simd_ops::ru64(r8.wrapping_add(P1))) as u64; rax = rdx; rax = ((((rax as u32)) & (0xfu32))) as u32 as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 1; continue; }
            1 => { fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 5; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::lk_add_u32(r8.wrapping_add(P2), (1) as u32)) as u32 as u64; rax = (crate::simd_ops::ru64(rdx)) as u64; crate::simd_ops::wu64(r8.wrapping_add(P1), rax); bb = 3; continue; }
            3 => { rbx = rdx; bb = 4; continue; }
            4 => { crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rbx); rcx = rbx; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rdi, rax); rax = rdi; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            5 => { rdx = 0x18; r8 = ((rdx.wrapping_add(P4)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rdx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 3; continue; } bb = 6; continue; }
            6 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P6), rax), rax, rcx, rdx, r8, r9, rsp); rcx = rax; rdx = 0xdeadd00d; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P7), rax), rax, rcx, rdx, r8, r9, rsp); bb = 4; continue; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1412fdbe0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x142535070; const P2: u64 = 0x1426456c8; const P3: u64 = 0x1426453a0; const P4: u64 = 0x14225de80;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x40u64)) as u64;
    rdi = rcx;
    rbx = 0;
    fl_l = 0; fl_r = 0;
    rcx = 0x6348;
    rax = ((crate::exe::tls_pointer() as u64)) as u64;
    rdx = (crate::simd_ops::ru64(rax)) as u64;
    rcx = (crate::simd_ops::ru64(rcx.wrapping_add(rdx))) as u64;
    rdx = 0x708;
    r8 = ((rbx.wrapping_add(P0)) as u32) as u32 as u64;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    fl_l = rax; fl_r = rax;
    if !(((((fl_l as u64)) & ((fl_r as u64))) != 0)) { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); rcx = rax; rdx = 0xdeadd00d; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); } else { rbx = rax; }
    crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rbx);
    rcx = rbx;
    rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp);
    crate::simd_ops::wu64(rdi, rax);
    rax = rdi;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64;
    rsp = ((rsp).wrapping_add(0x40u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1412fdc60(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142535070; const P1: u64 = 0x1426456c8; const P2: u64 = 0x1426453a0; const P3: u64 = 0x14397fed8;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rsi = rcx; r14 = 0; fl_l = 0; fl_r = 0; r15 = 0x6348; rax = ((crate::exe::tls_pointer() as u64)) as u64; rdi = (crate::simd_ops::ru64(rax)) as u64; rcx = (crate::simd_ops::ru64(r15.wrapping_add(rdi))) as u64; rdx = (crate::simd_ops::ru64(rcx.wrapping_add(0x80u64))) as u64; rbx = (crate::simd_ops::ru64(rdx.wrapping_add(0x10u64))) as u64; rax = rbx; rax = ((((rax as u32)) & (0xfu32))) as u32 as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::lk_add_u32(rdx.wrapping_add(0x18u64), (1) as u32)) as u32 as u64; rax = (crate::simd_ops::ru64(rbx)) as u64; crate::simd_ops::wu64(rdx.wrapping_add(0x10u64), rax); bb = 5; continue; }
            3 => { rdx = 0x78; r8 = ((rdx.wrapping_sub(0x68u64)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rbx = rax; rcx = (crate::simd_ops::ru64(r15.wrapping_add(rdi))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp); rcx = rax; rdx = 0xdeadd00d; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); rbx = r14; rcx = (crate::simd_ops::ru64(r15.wrapping_add(rdi))) as u64; bb = 5; continue; }
            5 => { crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), rbx); crate::simd_ops::wu64(rbx, r14); crate::simd_ops::wu64(rbx.wrapping_add(0x8u64), r14); rdx = 0x710; r8 = 0x10; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 7; continue; } bb = 6; continue; }
            6 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp); rcx = rax; rdx = 0xdeadd00d; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); rax = r14; bb = 7; continue; }
            7 => { crate::simd_ops::wu64(rax, rax); crate::simd_ops::wu64(rax.wrapping_add(0x8u64), rax); crate::simd_ops::wu64(rax.wrapping_add(0x10u64), rax); crate::simd_ops::wu16(rax.wrapping_add(0x18u64), (0x101u16) as u16); crate::simd_ops::wu64(rbx, rax); crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), r14); crate::simd_ops::wu64(rbx.wrapping_add(0x18u64), r14); crate::simd_ops::wu32(rbx.wrapping_add(0x20u64), ((r14 as u32)) as u32); crate::simd_ops::wu64(rbx.wrapping_add(0x30u64), r14); crate::simd_ops::wu64(rbx.wrapping_add(0x38u64), r14); crate::simd_ops::wu64(rbx.wrapping_add(0x40u64), r14); crate::simd_ops::wu64(rbx.wrapping_add(0x48u64), r14); crate::simd_ops::wu64(rbx.wrapping_add(0x50u64), r14); crate::simd_ops::wu64(rbx.wrapping_add(0x58u64), r14); crate::simd_ops::wu64(rbx.wrapping_add(0x60u64), r14); crate::simd_ops::wu64(rbx.wrapping_add(0x68u64), r14); crate::simd_ops::wu64(rbx.wrapping_add(0x70u64), r14); rax = rbx.wrapping_add(0x38); crate::simd_ops::wu64(rbx.wrapping_add(0x28u64), rax); crate::simd_ops::wu32(rbx.wrapping_add(0x30u64), ((r14 as u32)) as u32); crate::simd_ops::wu32(rbx.wrapping_add(0x34u64), (0x4u32) as u32); crate::simd_ops::wu64(P3, rbx); crate::simd_ops::wu64(rsi, rbx); rax = rsi; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1412fddb0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x60; const P1: u64 = 0x10; const P2: u64 = 0x18; const P3: u64 = 0xffffffffffffffb8; const P4: u64 = 0x142535070; const P5: u64 = 0x1426456c8; const P6: u64 = 0x1426453a0; const P7: u64 = 0x1412cf770; const P8: u64 = 0x140200580; const P9: u64 = 0xfffffffffffffffa; const P10: u64 = 0x141834fc0; const P11: u64 = 0xfffffffffffffffc;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rdi = rcx; rsi = 0; fl_l = 0; fl_r = 0; rcx = 0x6348; rax = ((crate::exe::tls_pointer() as u64)) as u64; rdx = (crate::simd_ops::ru64(rax)) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(rdx))) as u64; rdx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rbx = (crate::simd_ops::ru64(rdx.wrapping_add(P1))) as u64; rax = rbx; rax = ((((rax as u32)) & (0xfu32))) as u32 as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::lk_add_u32(rdx.wrapping_add(P2), (1) as u32)) as u32 as u64; rax = (crate::simd_ops::ru64(rbx)) as u64; crate::simd_ops::wu64(rdx.wrapping_add(P1), rax); bb = 5; continue; }
            3 => { rdx = 0x58; r8 = ((rdx.wrapping_add(P3)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rbx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp); rcx = rax; rdx = 0xdeadd00d; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P6), rax), rax, rcx, rdx, r8, r9, rsp); rbx = rsi; bb = 5; continue; }
            5 => { crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), rbx); r14 = P7; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r14); r9 = P8; rdx = 0x8; r8 = ((rdx.wrapping_add(P9)) as u32) as u32 as u64; rcx = rbx; rax = crate::simd_ops::call_stk(P10, rax, rcx, rdx, r8, r9, rsp); rcx = ((rbx.wrapping_add(P1)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r14); r9 = P8; rdx = 0x8; r8 = ((rdx.wrapping_add(P11)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P10, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx.wrapping_add(0x30u64), rsi); crate::simd_ops::wu64(rbx.wrapping_add(0x38u64), rsi); crate::simd_ops::wu64(rbx.wrapping_add(0x40u64), rsi); crate::simd_ops::wu64(rbx.wrapping_add(0x48u64), rsi); crate::simd_ops::wu16(rbx.wrapping_add(0x50u64), (0x0u16) as u16); crate::simd_ops::wu64(rdi, rbx); rax = rdi; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1412fdeb0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x80; const P1: u64 = 0x10; const P2: u64 = 0x18; const P3: u64 = 0x1421f22b0; const P4: u64 = 0xffffffffffffff98; const P5: u64 = 0x142535070; const P6: u64 = 0x1426456c8; const P7: u64 = 0x1426453a0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rdi = rcx; rbx = 0; fl_l = 0; fl_r = 0; rcx = 0x6348; rax = ((crate::exe::tls_pointer() as u64)) as u64; rdx = (crate::simd_ops::ru64(rax)) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(rdx))) as u64; r8 = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rdx = (crate::simd_ops::ru64(r8.wrapping_add(P1))) as u64; rax = rdx; rax = ((((rax as u32)) & (0xfu32))) as u32 as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 1; continue; }
            1 => { fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 5; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::lk_add_u32(r8.wrapping_add(P2), (1) as u32)) as u32 as u64; rax = (crate::simd_ops::ru64(rdx)) as u64; crate::simd_ops::wu64(r8.wrapping_add(P1), rax); bb = 3; continue; }
            3 => { rbx = rdx; bb = 4; continue; }
            4 => { crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rbx); rcx = rbx; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rdi, rax); rax = rdi; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            5 => { rdx = 0x78; r8 = ((rdx.wrapping_add(P4)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rdx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 3; continue; } bb = 6; continue; }
            6 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P6), rax), rax, rcx, rdx, r8, r9, rsp); rcx = rax; rdx = 0xdeadd00d; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P7), rax), rax, rcx, rdx, r8, r9, rsp); bb = 4; continue; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1412fdf50(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x118; const P1: u64 = 0x10; const P2: u64 = 0x18; const P3: u64 = 0x1423c6c00; const P4: u64 = 0x142535070; const P5: u64 = 0x1426456c8; const P6: u64 = 0x1426453a0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x1412fdf50, funcinfo_rva: 0x359b5c0, handler: 1, shape: 2, frame_reg: 0, establisher_off: 0x48, conts: &[] };
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rdx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rdi = (rcx) as u64; rbx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rcx = (0x6348u32) as u32 as u64; rax = ((crate::exe::tls_pointer() as u64)) as u64; rdx = (crate::simd_ops::ru64(rax)) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(rdx))) as u64; r8 = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rdx = (crate::simd_ops::ru64(r8.wrapping_add(P1))) as u64; rax = (rdx) as u64; rax = ((((rax as u32)) & (0xfu32))) as u32 as u64; fl_l = (rdx) as u64 as u64; fl_r = (rdx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 1; continue; }
            1 => { fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 5; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::lk_add_u32(r8.wrapping_add(P2), (1) as u32)) as u32 as u64; rax = (crate::simd_ops::ru64(rdx)) as u64; crate::simd_ops::wu64(r8.wrapping_add(P1), (rax) as u64); bb = 3; continue; }
            3 => { rbx = (rdx) as u64; bb = 4; continue; }
            4 => { crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), (rbx) as u64); rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1412fdfaa); crate::simd_ops::replace_rax_in(rax); rax = match eh.call(0x1412fdfaa, (P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8) { Ok(v) => v, Err(l) => l.unknown(0x1412fdf50) }; crate::simd_ops::wu64(rdi, (rax) as u64); rax = (rdi) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            5 => { rdx = (0x110u32) as u32 as u64; r8 = (0x10u32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1412fdfcb); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rdx = (rax) as u64; fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 3; continue; } bb = 6; continue; }
            6 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1412fdfd8); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P5)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rcx = (rax) as u64; rdx = (0xdeadd00du32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1412fdfe6); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P6)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 4; continue; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1412fdff0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xc8; const P1: u64 = 0x10; const P2: u64 = 0x18; const P3: u64 = 0x14224a2f0; const P4: u64 = 0x142535070; const P5: u64 = 0x1426456c8; const P6: u64 = 0x1426453a0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x1412fdff0, funcinfo_rva: 0x359b5c0, handler: 1, shape: 2, frame_reg: 0, establisher_off: 0x48, conts: &[] };
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rdx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rdi = (rcx) as u64; rbx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rcx = (0x6348u32) as u32 as u64; rax = ((crate::exe::tls_pointer() as u64)) as u64; rdx = (crate::simd_ops::ru64(rax)) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(rdx))) as u64; r8 = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rdx = (crate::simd_ops::ru64(r8.wrapping_add(P1))) as u64; rax = (rdx) as u64; rax = ((((rax as u32)) & (0xfu32))) as u32 as u64; fl_l = (rdx) as u64 as u64; fl_r = (rdx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 1; continue; }
            1 => { fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 5; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::lk_add_u32(r8.wrapping_add(P2), (1) as u32)) as u32 as u64; rax = (crate::simd_ops::ru64(rdx)) as u64; crate::simd_ops::wu64(r8.wrapping_add(P1), (rax) as u64); bb = 3; continue; }
            3 => { rbx = (rdx) as u64; bb = 4; continue; }
            4 => { crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), (rbx) as u64); rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1412fe04a); crate::simd_ops::replace_rax_in(rax); rax = match eh.call(0x1412fe04a, (P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8) { Ok(v) => v, Err(l) => l.unknown(0x1412fdff0) }; crate::simd_ops::wu64(rdi, (rax) as u64); rax = (rdi) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            5 => { rdx = (0xc0u32) as u32 as u64; r8 = (0x10u32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1412fe06b); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rdx = (rax) as u64; fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 3; continue; } bb = 6; continue; }
            6 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1412fe078); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P5)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rcx = (rax) as u64; rdx = (0xdeadd00du32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1412fe086); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P6)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 4; continue; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1412fe090(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x20; const P1: u64 = 0x10; const P2: u64 = 0x18; const P3: u64 = 0xfffffffffffffff8; const P4: u64 = 0x142535070; const P5: u64 = 0x1426456c8; const P6: u64 = 0x1426453a0; const P7: u64 = 0x8;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rax = ((crate::exe::tls_pointer() as u64)) as u64; rbx = rcx; rcx = 0x6348; rdi = 0; fl_l = 0; fl_r = 0; rdx = (crate::simd_ops::ru64(rax)) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(rdx))) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64; rdx = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { fl_l = ((rdx as u8)) as u8 as u64; fl_r = (0xfu8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::lk_add_u32(rcx.wrapping_add(P2), (1) as u32)) as u32 as u64; rax = (crate::simd_ops::ru64(rdx)) as u64; crate::simd_ops::wu64(rcx.wrapping_add(P1), rax); bb = 5; continue; }
            3 => { rdx = 0x18; rcx = rax; r8 = ((rdx.wrapping_add(P3)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rdx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp); rcx = rax; rdx = 0xdeadd00d; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P6), rax), rax, rcx, rdx, r8, r9, rsp); rdx = rdi; bb = 5; continue; }
            5 => { crate::simd_ops::wu64(rdx, rdi); rax = rbx; crate::simd_ops::wu64(rdx.wrapping_add(P7), rdi); crate::simd_ops::wu64(rdx.wrapping_add(P1), rdi); crate::simd_ops::wu64(rbx, rdx); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1412fe120(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x200; const P1: u64 = 0x10; const P2: u64 = 0x18; const P3: u64 = 0x142348ec0; const P4: u64 = 0x142535070; const P5: u64 = 0x1426456c8; const P6: u64 = 0x1426453a0;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x1412fe120, funcinfo_rva: 0x3699300, handler: 1, shape: 2, frame_reg: 0, establisher_off: 0x48, conts: &[] };
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (rsi) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rdi) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rdx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rsi = (r9) as u64; r14 = (r8) as u64; rdi = (rcx) as u64; rbx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rcx = (0x6348u32) as u32 as u64; rax = ((crate::exe::tls_pointer() as u64)) as u64; rdx = (crate::simd_ops::ru64(rax)) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(rdx))) as u64; r10 = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rdx = (crate::simd_ops::ru64(r10.wrapping_add(P1))) as u64; rax = (rdx) as u64; rax = ((((rax as u32)) & (0xfu32))) as u32 as u64; fl_l = (rdx) as u64 as u64; fl_r = (rdx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 1; continue; }
            1 => { fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 5; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::lk_add_u32(r10.wrapping_add(P2), (1) as u32)) as u32 as u64; rax = (crate::simd_ops::ru64(rdx)) as u64; crate::simd_ops::wu64(r10.wrapping_add(P1), (rax) as u64); bb = 3; continue; }
            3 => { rbx = (rdx) as u64; bb = 4; continue; }
            4 => { crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), (rbx) as u64); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; rdx = (crate::simd_ops::ru32(rax)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((rdx as u32)) as u32); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; rdx = (crate::simd_ops::ru32(rax)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rdx as u32)) as u32); r9 = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; r9 = (crate::simd_ops::ru32(r9)) as u32 as u64; r8 = (crate::simd_ops::ru32(rsi)) as u32 as u64; rdx = ((crate::simd_ops::ru8(r14)) as u32) as u32 as u64; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1412fe1b3); crate::simd_ops::replace_rax_in(rax); rax = match eh.call(0x1412fe1b3, (P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8) { Ok(v) => v, Err(l) => l.unknown(0x1412fe120) }; crate::simd_ops::wu64(rdi, (rax) as u64); rax = (rdi) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            5 => { rdx = (0x1f8u32) as u32 as u64; r8 = (0x10u32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1412fe1df); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rdx = (rax) as u64; fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 3; continue; } bb = 6; continue; }
            6 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1412fe1ec); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P5)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rcx = (rax) as u64; rdx = (0xdeadd00du32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1412fe1fa); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P6)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 4; continue; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1412fe210(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x128; const P1: u64 = 0x10; const P2: u64 = 0x18; const P3: u64 = 0x142535070; const P4: u64 = 0x1426456c8; const P5: u64 = 0x1426453a0; const P6: u64 = 0x143a08298; const P7: u64 = 0x1433ffba8; const P8: u64 = 0x8; const P9: u64 = 0x1433ffb80; const P10: u64 = 0x20; const P11: u64 = 0x28; const P12: u64 = 0x143a000f8; const P13: u64 = 0x14240d5c0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rax = ((crate::exe::tls_pointer() as u64)) as u64; rdi = rcx; rcx = 0x6348; rsi = 0; fl_l = 0; fl_r = 0; rdx = (crate::simd_ops::ru64(rax)) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(rdx))) as u64; rdx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rbx = (crate::simd_ops::ru64(rdx.wrapping_add(P1))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { fl_l = ((rbx as u8)) as u8 as u64; fl_r = (0xfu8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::lk_add_u32(rdx.wrapping_add(P2), (1) as u32)) as u32 as u64; rax = (crate::simd_ops::ru64(rbx)) as u64; crate::simd_ops::wu64(rdx.wrapping_add(P1), rax); bb = 5; continue; }
            3 => { rdx = 0x120; r8 = 0x10; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rbx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp); rcx = rax; rdx = 0xdeadd00d; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp); rbx = rsi; bb = 5; continue; }
            5 => { rcx = (crate::simd_ops::ru64(P6)) as u64; rax = P7; crate::simd_ops::wu64(rbx, rax); rdx = rbx; crate::simd_ops::wu64(rbx.wrapping_add(P8), rsi); rax = P9; crate::simd_ops::wu64(rbx.wrapping_add(P1), rsi); crate::simd_ops::wu64(rbx.wrapping_add(P2), rsi); crate::simd_ops::wu64(rbx.wrapping_add(P10), rsi); crate::simd_ops::wu64(rbx.wrapping_add(P11), rsi); crate::simd_ops::wu64(rbx.wrapping_add(0x30u64), rsi); crate::simd_ops::wu64(rbx.wrapping_add(0x38u64), rsi); crate::simd_ops::wu64(rbx.wrapping_add(0x40u64), rsi); crate::simd_ops::wu64(rbx.wrapping_add(0xa8u64), rsi); crate::simd_ops::wu64(rbx.wrapping_add(0xb0u64), rax); crate::simd_ops::wu64(rbx.wrapping_add(0xf8u64), rsi); crate::simd_ops::wu32(rbx.wrapping_add(0x100u64), ((rsi as u32)) as u32); crate::simd_ops::wu64(rbx.wrapping_add(0x108u64), rsi); crate::simd_ops::wu32(rbx.wrapping_add(0x118u64), (0xffffffffu32) as u32); crate::simd_ops::wu32(rbx.wrapping_add(0x11cu64), ((rsi as u32)) as u32); crate::simd_ops::wu64(rbx.wrapping_add(0x110u64), rsi); crate::simd_ops::wu64(P12, rbx); rax = crate::simd_ops::call_stk(P13, rax, rcx, rdx, r8, r9, rsp); rax = rdi; crate::simd_ops::wu64(rdi, rbx); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1412fe320(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x20; const P1: u64 = 0x10; const P2: u64 = 0x18; const P3: u64 = 0xfffffffffffffff8; const P4: u64 = 0x142535070; const P5: u64 = 0x1426456c8; const P6: u64 = 0x1426453a0; const P7: u64 = 0x4; const P8: u64 = 0x8; const P9: u64 = 0x14397fee8; const P10: u64 = 0x14397fee0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rax = ((crate::exe::tls_pointer() as u64)) as u64; rbx = rcx; rcx = 0x6348; rsi = 0; fl_l = 0; fl_r = 0; rdi = r8; rdx = (crate::simd_ops::ru64(rax)) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(rdx))) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64; rdx = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { fl_l = ((rdx as u8)) as u8 as u64; fl_r = (0xfu8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::lk_add_u32(rcx.wrapping_add(P2), (1) as u32)) as u32 as u64; rax = (crate::simd_ops::ru64(rdx)) as u64; crate::simd_ops::wu64(rcx.wrapping_add(P1), rax); bb = 5; continue; }
            3 => { rdx = 0x18; rcx = rax; r8 = ((rdx.wrapping_add(P3)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rdx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp); rcx = rax; rdx = 0xdeadd00d; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P6), rax), rax, rcx, rdx, r8, r9, rsp); rdx = rsi; bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru32(rdi)) as u32 as u64; crate::simd_ops::wu32(rdx, ((rax as u32)) as u32); crate::simd_ops::wu8(rdx.wrapping_add(P7), ((rsi as u8)) as u8); crate::simd_ops::wu64(rdx.wrapping_add(P8), rsi); crate::simd_ops::wu64(rdx.wrapping_add(P1), rsi); rcx = (crate::simd_ops::ru32(rdx)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 8; continue; } bb = 6; continue; }
            6 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 9; continue; } bb = 7; continue; }
            7 => { rax = rdx; let tmp_xchg: u64 = crate::simd_ops::atomic_xchg_u64(P9, rax); rax = (tmp_xchg) as u64; bb = 9; continue; }
            8 => { rax = rdx; let tmp_xchg: u64 = crate::simd_ops::atomic_xchg_u64(P10, rax); rax = (tmp_xchg) as u64; bb = 9; continue; }
            9 => { crate::simd_ops::wu64(rbx, rdx); rax = rbx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1412fe3f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x2f8; const P1: u64 = 0x10; const P2: u64 = 0x18; const P3: u64 = 0x1422555f0; const P4: u64 = 0x142535070; const P5: u64 = 0x1426456c8; const P6: u64 = 0x1426453a0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x1412fe3f0, funcinfo_rva: 0x359b708, handler: 1, shape: 2, frame_reg: 0, establisher_off: 0x48, conts: &[] };
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (rsi) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rdx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rsi = (r8) as u64; rdi = (rcx) as u64; rbx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rcx = (0x6348u32) as u32 as u64; rax = ((crate::exe::tls_pointer() as u64)) as u64; rdx = (crate::simd_ops::ru64(rax)) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(rdx))) as u64; r9 = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rdx = (crate::simd_ops::ru64(r9.wrapping_add(P1))) as u64; rax = (rdx) as u64; rax = ((((rax as u32)) & (0xfu32))) as u32 as u64; fl_l = (rdx) as u64 as u64; fl_r = (rdx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 1; continue; }
            1 => { fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 5; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::lk_add_u32(r9.wrapping_add(P2), (1) as u32)) as u32 as u64; rax = (crate::simd_ops::ru64(rdx)) as u64; crate::simd_ops::wu64(r9.wrapping_add(P1), (rax) as u64); bb = 3; continue; }
            3 => { rbx = (rdx) as u64; bb = 4; continue; }
            4 => { crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), (rbx) as u64); rdx = ((crate::simd_ops::ru8(rsi)) as u32) as u32 as u64; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1412fe455); crate::simd_ops::replace_rax_in(rax); rax = match eh.call(0x1412fe455, (P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8) { Ok(v) => v, Err(l) => l.unknown(0x1412fe3f0) }; crate::simd_ops::wu64(rdi, (rax) as u64); rax = (rdi) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            5 => { rdx = (0x2f0u32) as u32 as u64; r8 = (0x10u32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1412fe47b); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rdx = (rax) as u64; fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 3; continue; } bb = 6; continue; }
            6 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1412fe488); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P5)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rcx = (rax) as u64; rdx = (0xdeadd00du32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1412fe496); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P6)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 4; continue; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1412fe4a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x40; const P1: u64 = 0x10; const P2: u64 = 0x18; const P3: u64 = 0xffffffffffffffd8; const P4: u64 = 0x142535070; const P5: u64 = 0x1426456c8; const P6: u64 = 0x1426453a0; const P7: u64 = 0x143a08298; const P8: u64 = 0x1433f2b18; const P9: u64 = 0x14; const P10: u64 = 0x8; const P11: u64 = 0x20; const P12: u64 = 0x14397ff08; const P13: u64 = 0x14240d5c0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rax = ((crate::exe::tls_pointer() as u64)) as u64; rdi = rcx; rcx = 0x6348; rsi = 0; fl_l = 0; fl_r = 0; rdx = (crate::simd_ops::ru64(rax)) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(rdx))) as u64; rdx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rbx = (crate::simd_ops::ru64(rdx.wrapping_add(P1))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { fl_l = ((rbx as u8)) as u8 as u64; fl_r = (0xfu8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::lk_add_u32(rdx.wrapping_add(P2), (1) as u32)) as u32 as u64; rax = (crate::simd_ops::ru64(rbx)) as u64; crate::simd_ops::wu64(rdx.wrapping_add(P1), rax); bb = 5; continue; }
            3 => { rdx = 0x38; r8 = ((rdx.wrapping_add(P3)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rbx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp); rcx = rax; rdx = 0xdeadd00d; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P6), rax), rax, rcx, rdx, r8, r9, rsp); rbx = rsi; bb = 5; continue; }
            5 => { rcx = (crate::simd_ops::ru64(P7)) as u64; rax = P8; crate::simd_ops::wu64(rbx, rax); rdx = rbx; crate::simd_ops::wu32(rbx.wrapping_add(P1), (0xffffffffu32) as u32); crate::simd_ops::wu32(rbx.wrapping_add(P9), ((rsi as u32)) as u32); crate::simd_ops::wu64(rbx.wrapping_add(P10), rsi); crate::simd_ops::wu64(rbx.wrapping_add(P2), rsi); crate::simd_ops::wu64(rbx.wrapping_add(P11), rsi); crate::simd_ops::wu64(rbx.wrapping_add(0x28u64), rsi); crate::simd_ops::wu64(rbx.wrapping_add(0x30u64), rsi); crate::simd_ops::wu64(P12, rbx); rax = crate::simd_ops::call_stk(P13, rax, rcx, rdx, r8, r9, rsp); rax = rdi; crate::simd_ops::wu64(rdi, rbx); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1412fe570(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x78; const P1: u64 = 0x10; const P2: u64 = 0x18; const P3: u64 = 0xffffffffffffffa0; const P4: u64 = 0x142535070; const P5: u64 = 0x1426456c8; const P6: u64 = 0x1426453a0; const P7: u64 = 0x8; const P8: u64 = 0x20; const P9: u64 = 0x28; const P10: u64 = 0x30; const P11: u64 = 0x38; const P12: u64 = 0x1433f11a8; const P13: u64 = 0x143a367f8;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rax = ((crate::exe::tls_pointer() as u64)) as u64; rbx = rcx; rcx = 0x6348; rdi = 0; fl_l = 0; fl_r = 0; rdx = (crate::simd_ops::ru64(rax)) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(rdx))) as u64; r8 = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rdx = (crate::simd_ops::ru64(r8.wrapping_add(P1))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { fl_l = ((rdx as u8)) as u8 as u64; fl_r = (0xfu8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::lk_add_u32(r8.wrapping_add(P2), (1) as u32)) as u32 as u64; rax = (crate::simd_ops::ru64(rdx)) as u64; crate::simd_ops::wu64(r8.wrapping_add(P1), rax); bb = 5; continue; }
            3 => { rdx = 0x70; r8 = ((rdx.wrapping_add(P3)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rdx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp); rcx = rax; rdx = 0xdeadd00d; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P6), rax), rax, rcx, rdx, r8, r9, rsp); rdx = rdi; bb = 5; continue; }
            5 => { crate::simd_ops::wu64(rdx, rdi); rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rdx.wrapping_add(P7), rdi); crate::simd_ops::wu64(rdx.wrapping_add(P1), rdi); crate::simd_ops::wu64(rdx.wrapping_add(P2), rdi); crate::simd_ops::wu64(rdx.wrapping_add(P8), rdi); crate::simd_ops::wu64(rdx.wrapping_add(P9), rdi); crate::simd_ops::wu64(rdx.wrapping_add(P10), rdi); crate::simd_ops::wu64(rdx.wrapping_add(P11), rdi); crate::simd_ops::wu64(rdx.wrapping_add(0x40u64), rdi); crate::simd_ops::wu64(rdx.wrapping_add(0x48u64), rdi); crate::simd_ops::wu64(rdx.wrapping_add(0x50u64), rdi); crate::simd_ops::wu64(rdx.wrapping_add(0x58u64), rdi); crate::simd_ops::wu64(rdx.wrapping_add(0x60u64), rdi); crate::simd_ops::wu64(rdx.wrapping_add(0x68u64), rax); fl_l = (crate::simd_ops::ru32(P12)) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; crate::simd_ops::wu64(P13, rdx); if (((fl_l as u32)) == ((fl_r as u32))) { bb = 7; continue; } bb = 6; continue; }
            6 => { crate::simd_ops::wu64(rdx.wrapping_add(0x68u64), (0xffffffffffffffffu64) as u64); bb = 7; continue; }
            7 => { crate::simd_ops::wu64(rbx, rdx); rax = rbx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1412fe650(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x28; const P1: u64 = 0x10; const P2: u64 = 0x20; const P3: u64 = 0x142537280; const P4: u64 = 0x1426456c8; const P5: u64 = 0x1426453a0; const P6: u64 = 0x18; const P7: u64 = 0x8; const P8: u64 = 0x14397ff68;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rax = ((crate::exe::tls_pointer() as u64)) as u64; rbx = rcx; rcx = 0x6348; rdi = 0; fl_l = 0; fl_r = 0; rdx = (crate::simd_ops::ru64(rax)) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(rdx))) as u64; r8 = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rdx = (crate::simd_ops::ru64(r8.wrapping_add(P1))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { r9 = 0; fl_l = 0; fl_r = 0; rdx = ((rdi.wrapping_add(P2)) as u32) as u32 as u64; r8 = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rdx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 4; continue; } bb = 2; continue; }
            2 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp); rcx = rax; rdx = 0xdeadd00d; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp); rdx = ((rdi as u32)) as u32 as u64; bb = 4; continue; }
            3 => { fl_l = (crate::simd_ops::lk_add_u32(r8.wrapping_add(P6), (1) as u32)) as u32 as u64; rax = (crate::simd_ops::ru64(rdx)) as u64; crate::simd_ops::wu64(r8.wrapping_add(P1), rax); bb = 4; continue; }
            4 => { crate::simd_ops::wu64(rdx, rdi); rax = rbx; crate::simd_ops::wu64(rdx.wrapping_add(P7), rdi); crate::simd_ops::wu64(rdx.wrapping_add(P1), rdi); crate::simd_ops::wu64(rdx.wrapping_add(P6), rdi); crate::simd_ops::wu64(rbx, rdx); crate::simd_ops::wu64(P8, rdx); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1412fe6f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x142535070; const P2: u64 = 0x1426456c8; const P3: u64 = 0x1426453a0; const P4: u64 = 0x1422af250;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x40u64)) as u64;
    rdi = rcx;
    rbx = 0;
    fl_l = 0; fl_r = 0;
    rcx = 0x6348;
    rax = ((crate::exe::tls_pointer() as u64)) as u64;
    rdx = (crate::simd_ops::ru64(rax)) as u64;
    rcx = (crate::simd_ops::ru64(rcx.wrapping_add(rdx))) as u64;
    rdx = 0x548;
    r8 = ((rbx.wrapping_add(P0)) as u32) as u32 as u64;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    fl_l = rax; fl_r = rax;
    if !(((((fl_l as u64)) & ((fl_r as u64))) != 0)) { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); rcx = rax; rdx = 0xdeadd00d; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); } else { rbx = rax; }
    crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rbx);
    rcx = rbx;
    rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp);
    crate::simd_ops::wu64(rdi, rax);
    rax = rdi;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64;
    rsp = ((rsp).wrapping_add(0x40u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1412fe770(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x38; const P1: u64 = 0x10; const P2: u64 = 0x18; const P3: u64 = 0xffffffffffffffe0; const P4: u64 = 0x142535070; const P5: u64 = 0x1426456c8; const P6: u64 = 0x1426453a0; const P7: u64 = 0x8; const P8: u64 = 0x20; const P9: u64 = 0x28;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rax = ((crate::exe::tls_pointer() as u64)) as u64; rbx = rcx; rcx = 0x6348; rdi = 0; fl_l = 0; fl_r = 0; rdx = (crate::simd_ops::ru64(rax)) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(rdx))) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64; rdx = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { fl_l = ((rdx as u8)) as u8 as u64; fl_r = (0xfu8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::lk_add_u32(rcx.wrapping_add(P2), (1) as u32)) as u32 as u64; rax = (crate::simd_ops::ru64(rdx)) as u64; crate::simd_ops::wu64(rcx.wrapping_add(P1), rax); bb = 5; continue; }
            3 => { rdx = 0x30; rcx = rax; r8 = ((rdx.wrapping_add(P3)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rdx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp); rcx = rax; rdx = 0xdeadd00d; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P6), rax), rax, rcx, rdx, r8, r9, rsp); rdx = rdi; bb = 5; continue; }
            5 => { crate::simd_ops::wu64(rdx, rdi); rax = rbx; crate::simd_ops::wu64(rdx.wrapping_add(P7), rdi); crate::simd_ops::wu64(rdx.wrapping_add(P1), rdi); crate::simd_ops::wu64(rdx.wrapping_add(P2), rdi); crate::simd_ops::wu64(rdx.wrapping_add(P8), rdi); crate::simd_ops::wu16(rdx.wrapping_add(P9), ((rdi as u16)) as u16); crate::simd_ops::wu64(rbx, rdx); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1412fe810(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x60; const P1: u64 = 0x10; const P2: u64 = 0x18; const P3: u64 = 0xffffffffffffffb8; const P4: u64 = 0x142535070; const P5: u64 = 0x1426456c8; const P6: u64 = 0x1426453a0; const P7: u64 = 0x20; const P8: u64 = 0x40; const P9: u64 = 0x30; const P10: u64 = 0x38; const P11: u64 = 0x48;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rax = ((crate::exe::tls_pointer() as u64)) as u64; rbx = rcx; rcx = 0x6348; rdi = 0; fl_l = 0; fl_r = 0; rdx = (crate::simd_ops::ru64(rax)) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(rdx))) as u64; r8 = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rdx = (crate::simd_ops::ru64(r8.wrapping_add(P1))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { fl_l = ((rdx as u8)) as u8 as u64; fl_r = (0xfu8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::lk_add_u32(r8.wrapping_add(P2), (1) as u32)) as u32 as u64; rax = (crate::simd_ops::ru64(rdx)) as u64; crate::simd_ops::wu64(r8.wrapping_add(P1), rax); bb = 5; continue; }
            3 => { rdx = 0x58; r8 = ((rdx.wrapping_add(P3)) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rax; rdx = 0xdeadd00d; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P6), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = rdi; bb = 5; continue; }
            5 => { xmm0 = 0u128; ymm0_hi = 0; let y_addr: u64 = rdx; crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); let y_addr: u64 = rdx.wrapping_add(P7); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); crate::simd_ops::wu128(rdx.wrapping_add(P8), (xmm0) as u128); crate::simd_ops::wu64(rdx.wrapping_add(P9), rdi); rax = rbx; crate::simd_ops::wu64(rdx.wrapping_add(P10), rdi); crate::simd_ops::wu64(rdx.wrapping_add(P8), rdi); crate::simd_ops::wu64(rdx.wrapping_add(P11), rdi); crate::simd_ops::wu64(rdx.wrapping_add(0x50u64), rdi); crate::simd_ops::wu64(rbx, rdx); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1412fe8c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x28; const P1: u64 = 0x10; const P2: u64 = 0x20; const P3: u64 = 0x142537280; const P4: u64 = 0x1426456c8; const P5: u64 = 0x1426453a0; const P6: u64 = 0x18; const P7: u64 = 0x8;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rax = ((crate::exe::tls_pointer() as u64)) as u64; rbx = rcx; rcx = 0x6348; rdi = r8; rdx = (crate::simd_ops::ru64(rax)) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(rdx))) as u64; r9 = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rdx = (crate::simd_ops::ru64(r9.wrapping_add(P1))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { r9 = 0; fl_l = 0; fl_r = 0; r8 = 0; fl_l = 0; fl_r = 0; rdx = ((r9.wrapping_add(P2)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rdx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 4; continue; } bb = 2; continue; }
            2 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp); rcx = rax; rdx = 0xdeadd00d; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp); rdx = 0; fl_l = 0; fl_r = 0; bb = 4; continue; }
            3 => { fl_l = (crate::simd_ops::lk_add_u32(r9.wrapping_add(P6), (1) as u32)) as u32 as u64; rax = (crate::simd_ops::ru64(rdx)) as u64; crate::simd_ops::wu64(r9.wrapping_add(P1), rax); bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(rdi)) as u64; crate::simd_ops::wu64(rdx, rax); rax = rbx; crate::simd_ops::wu64(rdx.wrapping_add(P7), (0x0u64) as u64); crate::simd_ops::wu64(rdx.wrapping_add(P1), (0x0u64) as u64); crate::simd_ops::wu8(rdx.wrapping_add(P6), (0x0u8) as u8); crate::simd_ops::wu64(rbx, rdx); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1412fe960(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x30; const P1: u64 = 0x10; const P2: u64 = 0x18; const P3: u64 = 0xffffffffffffffe8; const P4: u64 = 0x142535070; const P5: u64 = 0x1426456c8; const P6: u64 = 0x1426453a0; const P7: u64 = 0x8; const P8: u64 = 0x14; const P9: u64 = 0x20;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rax = ((crate::exe::tls_pointer() as u64)) as u64; rbx = rcx; rcx = 0x6348; rdi = 0; fl_l = 0; fl_r = 0; rdx = (crate::simd_ops::ru64(rax)) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(rdx))) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64; rdx = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { fl_l = ((rdx as u8)) as u8 as u64; fl_r = (0xfu8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::lk_add_u32(rcx.wrapping_add(P2), (1) as u32)) as u32 as u64; rax = (crate::simd_ops::ru64(rdx)) as u64; crate::simd_ops::wu64(rcx.wrapping_add(P1), rax); bb = 5; continue; }
            3 => { rdx = 0x28; rcx = rax; r8 = ((rdx.wrapping_add(P3)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rdx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp); rcx = rax; rdx = 0xdeadd00d; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P6), rax), rax, rcx, rdx, r8, r9, rsp); rdx = rdi; bb = 5; continue; }
            5 => { crate::simd_ops::wu64(rdx, rdi); rax = rbx; crate::simd_ops::wu64(rdx.wrapping_add(P7), rdi); crate::simd_ops::wu8(rdx.wrapping_add(P1), (0x18u8) as u8); crate::simd_ops::wu64(rdx.wrapping_add(P8), rdi); crate::simd_ops::wu64(rdx.wrapping_add(P9), rdi); crate::simd_ops::wu64(rbx, rdx); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1412fea00(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x18; const P1: u64 = 0x10; const P2: u64 = 0x142537280; const P3: u64 = 0x1426456c8; const P4: u64 = 0x1426453a0; const P5: u64 = 0x8;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rax = ((crate::exe::tls_pointer() as u64)) as u64; rbx = rcx; rcx = 0x6348; rdx = (crate::simd_ops::ru64(rax)) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(rdx))) as u64; r8 = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rdx = (crate::simd_ops::ru64(r8.wrapping_add(P1))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { r9 = 0; fl_l = 0; fl_r = 0; r8 = 0; fl_l = 0; fl_r = 0; rdx = ((r9.wrapping_add(P1)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rdx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 4; continue; } bb = 2; continue; }
            2 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); rcx = rax; rdx = 0xdeadd00d; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp); rdx = 0; fl_l = 0; fl_r = 0; bb = 4; continue; }
            3 => { fl_l = (crate::simd_ops::lk_add_u32(r8.wrapping_add(P0), (1) as u32)) as u32 as u64; rax = (crate::simd_ops::ru64(rdx)) as u64; crate::simd_ops::wu64(r8.wrapping_add(P1), rax); bb = 4; continue; }
            4 => { crate::simd_ops::wu64(rdx, (0x0u64) as u64); rax = rbx; crate::simd_ops::wu8(rdx.wrapping_add(P5), (0x18u8) as u8); crate::simd_ops::wu64(rbx, rdx); rsp = ((rsp).wrapping_add(0x30u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1412fea80(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x4; const P2: u64 = 0x142535070; const P3: u64 = 0x1426456c8; const P4: u64 = 0x1426453a0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x40u64)) as u64;
    rax = ((crate::exe::tls_pointer() as u64)) as u64;
    rdi = rcx;
    rbx = 0;
    fl_l = 0; fl_r = 0;
    rcx = 0x6348;
    rdx = (crate::simd_ops::ru64(rax)) as u64;
    r8 = ((rbx.wrapping_add(P0)) as u32) as u32 as u64;
    rcx = (crate::simd_ops::ru64(rcx.wrapping_add(rdx))) as u64;
    rdx = ((rbx.wrapping_add(P1)) as u32) as u32 as u64;
    rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp);
    fl_l = rax; fl_r = rax;
    if !(((((fl_l as u64)) & ((fl_r as u64))) != 0)) { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); rcx = rax; rdx = 0xdeadd00d; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp); } else { rbx = rax; }
    crate::simd_ops::wu32(rbx, (0xffffffffu32) as u32);
    rax = rdi;
    crate::simd_ops::wu64(rdi, rbx);
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64;
    rsp = ((rsp).wrapping_add(0x40u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1412feaf0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x58; const P1: u64 = 0x10; const P2: u64 = 0x18; const P3: u64 = 0xffffffffffffffc0; const P4: u64 = 0x142535070; const P5: u64 = 0x1426456c8; const P6: u64 = 0x1426453a0; const P7: u64 = 0x8;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rax = ((crate::exe::tls_pointer() as u64)) as u64; rbx = rcx; rcx = 0x6348; rdi = 0; fl_l = 0; fl_r = 0; rdx = (crate::simd_ops::ru64(rax)) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(rdx))) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64; rdx = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { fl_l = ((rdx as u8)) as u8 as u64; fl_r = (0xfu8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::lk_add_u32(rcx.wrapping_add(P2), (1) as u32)) as u32 as u64; rax = (crate::simd_ops::ru64(rdx)) as u64; crate::simd_ops::wu64(rcx.wrapping_add(P1), rax); bb = 5; continue; }
            3 => { rdx = 0x50; rcx = rax; r8 = ((rdx.wrapping_add(P3)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rdx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp); rcx = rax; rdx = 0xdeadd00d; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P6), rax), rax, rcx, rdx, r8, r9, rsp); rdx = rdi; bb = 5; continue; }
            5 => { crate::simd_ops::wu64(rdx, rdi); rax = rbx; crate::simd_ops::wu64(rdx.wrapping_add(P7), rdi); crate::simd_ops::wu64(rdx.wrapping_add(P1), rdi); crate::simd_ops::wu64(rdx.wrapping_add(P2), rdi); crate::simd_ops::wu64(rbx, rdx); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1412feb90(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x28; const P1: u64 = 0x10; const P2: u64 = 0x20; const P3: u64 = 0x142537280; const P4: u64 = 0x1426456c8; const P5: u64 = 0x1426453a0; const P6: u64 = 0x18; const P7: u64 = 0x5; const P8: u64 = 0x7; const P9: u64 = 0x4; const P10: u64 = 0x8;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rax = ((crate::exe::tls_pointer() as u64)) as u64; rbx = rcx; rcx = 0x6348; rdi = 0; fl_l = 0; fl_r = 0; rdx = (crate::simd_ops::ru64(rax)) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(rdx))) as u64; r8 = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rdx = (crate::simd_ops::ru64(r8.wrapping_add(P1))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { r9 = 0; fl_l = 0; fl_r = 0; rdx = ((rdi.wrapping_add(P2)) as u32) as u32 as u64; r8 = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rdx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 4; continue; } bb = 2; continue; }
            2 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp); rcx = rax; rdx = 0xdeadd00d; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp); rdx = ((rdi as u32)) as u32 as u64; bb = 4; continue; }
            3 => { fl_l = (crate::simd_ops::lk_add_u32(r8.wrapping_add(P6), (1) as u32)) as u32 as u64; rax = (crate::simd_ops::ru64(rdx)) as u64; crate::simd_ops::wu64(r8.wrapping_add(P1), rax); bb = 4; continue; }
            4 => { crate::simd_ops::wu16(rdx.wrapping_add(P7), ((rdi as u16)) as u16); rax = rbx; crate::simd_ops::wu8(rdx.wrapping_add(P8), ((rdi as u8)) as u8); crate::simd_ops::wu32(rdx, (0x1u32) as u32); crate::simd_ops::wu8(rdx.wrapping_add(P9), ((rdi as u8)) as u8); crate::simd_ops::wu64(rdx.wrapping_add(P10), rdi); crate::simd_ops::wu64(rdx.wrapping_add(P1), rdi); crate::simd_ops::wu64(rdx.wrapping_add(P6), rdi); crate::simd_ops::wu64(rbx, rdx); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1412fec30(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x142535070; const P2: u64 = 0x1426456c8; const P3: u64 = 0x1426453a0; const P4: u64 = 0x14238b130;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x40u64)) as u64;
    rdi = rcx;
    rbx = 0;
    fl_l = 0; fl_r = 0;
    rcx = 0x6348;
    rax = ((crate::exe::tls_pointer() as u64)) as u64;
    rdx = (crate::simd_ops::ru64(rax)) as u64;
    rcx = (crate::simd_ops::ru64(rcx.wrapping_add(rdx))) as u64;
    rdx = 0x418;
    r8 = ((rbx.wrapping_add(P0)) as u32) as u32 as u64;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    fl_l = rax; fl_r = rax;
    if !(((((fl_l as u64)) & ((fl_r as u64))) != 0)) { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); rcx = rax; rdx = 0xdeadd00d; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); } else { rbx = rax; }
    crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rbx);
    rcx = rbx;
    rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp);
    crate::simd_ops::wu64(rdi, rax);
    rax = rdi;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64;
    rsp = ((rsp).wrapping_add(0x40u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1412fed50(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x190; const P1: u64 = 0x10; const P2: u64 = 0x18; const P3: u64 = 0x14222b5d0; const P4: u64 = 0x142535070; const P5: u64 = 0x1426456c8; const P6: u64 = 0x1426453a0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rdi = rcx; rbx = 0; fl_l = 0; fl_r = 0; rcx = 0x6348; rax = ((crate::exe::tls_pointer() as u64)) as u64; rdx = (crate::simd_ops::ru64(rax)) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(rdx))) as u64; r8 = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rdx = (crate::simd_ops::ru64(r8.wrapping_add(P1))) as u64; rax = rdx; rax = ((((rax as u32)) & (0xfu32))) as u32 as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 1; continue; }
            1 => { fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 5; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::lk_add_u32(r8.wrapping_add(P2), (1) as u32)) as u32 as u64; rax = (crate::simd_ops::ru64(rdx)) as u64; crate::simd_ops::wu64(r8.wrapping_add(P1), rax); bb = 3; continue; }
            3 => { rbx = rdx; bb = 4; continue; }
            4 => { crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rbx); rcx = rbx; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rdi, rax); rax = rdi; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            5 => { rdx = 0x188; r8 = 0x10; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rdx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 3; continue; } bb = 6; continue; }
            6 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp); rcx = rax; rdx = 0xdeadd00d; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P6), rax), rax, rcx, rdx, r8, r9, rsp); bb = 4; continue; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1412fedf0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x30; const P1: u64 = 0x10; const P2: u64 = 0x18; const P3: u64 = 0xffffffffffffffe8; const P4: u64 = 0x142535070; const P5: u64 = 0x1426456c8; const P6: u64 = 0x1426453a0; const P7: u64 = 0x20; const P8: u64 = 0x8;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rax = ((crate::exe::tls_pointer() as u64)) as u64; rbx = rcx; rcx = 0x6348; rdi = 0; fl_l = 0; fl_r = 0; rdx = (crate::simd_ops::ru64(rax)) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(rdx))) as u64; r8 = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rdx = (crate::simd_ops::ru64(r8.wrapping_add(P1))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { fl_l = ((rdx as u8)) as u8 as u64; fl_r = (0xfu8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::lk_add_u32(r8.wrapping_add(P2), (1) as u32)) as u32 as u64; rax = (crate::simd_ops::ru64(rdx)) as u64; crate::simd_ops::wu64(r8.wrapping_add(P1), rax); bb = 5; continue; }
            3 => { rdx = 0x28; r8 = ((rdx.wrapping_add(P3)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rdx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp); rcx = rax; rdx = 0xdeadd00d; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P6), rax), rax, rcx, rdx, r8, r9, rsp); rdx = rdi; bb = 5; continue; }
            5 => { crate::simd_ops::wu64(rdx.wrapping_add(P2), rdi); rax = rbx; crate::simd_ops::wu64(rdx.wrapping_add(P7), rdi); crate::simd_ops::wu64(rdx.wrapping_add(P8), rdi); crate::simd_ops::wu32(rdx.wrapping_add(P1), ((rdi as u32)) as u32); crate::simd_ops::wu64(rdx, rdi); crate::simd_ops::wu64(rbx, rdx); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1412feea0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1433f45a0; const P1: u64 = 0x1433f45ad; const P2: u64 = 0x143bbd170; const P3: u64 = 0x1413103c0; const P4: u64 = 0x143bbd168; const P5: u64 = 0x18; const P6: u64 = 0x20; const P7: u64 = 0x4; const P8: u64 = 0x10; const P9: u64 = 0x28;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rsi = (0xcbf29ce484222325u64) as u64; rcx = (0xffffffffffffffffu64) as u64; rbx = rsi; bb = 1; continue; }
            1 => { rcx = ((rcx).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru8(r8.wrapping_add(rcx))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 1; continue; } bb = 2; continue; }
            2 => { rcx = ((rcx).wrapping_add(r8)) as u64; r9 = (0x100000001b3u64) as u64; fl_l = r8; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 5; continue; } bb = 3; continue; }
            3 => { bb = 4; continue; }
            4 => { rax = (((crate::simd_ops::ru8(r8)) as i8 as i64) as u64) as u64; r8 = ((r8).wrapping_add(1)) as u64; rbx = (((rbx) ^ (rax))) as u64; rbx = (((rbx) as i64).wrapping_mul((r9) as i64) as u64) as u64; fl_l = r8; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 4; continue; } bb = 5; continue; }
            5 => { rcx = P0; rdi = rsi; rdx = P1; bb = 6; continue; }
            6 => { rax = (((crate::simd_ops::ru8(rcx)) as i8 as i64) as u64) as u64; rcx = ((rcx).wrapping_add(1)) as u64; rdi = (((rdi) ^ (rax))) as u64; rdi = (((rdi) as i64).wrapping_mul((r9) as i64) as u64) as u64; fl_l = rcx; fl_r = rdx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 6; continue; } bb = 7; continue; }
            7 => { rcx = P2; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru32(P4)) as u32 as u64; crate::simd_ops::wu64(rax.wrapping_add(P5), rbx); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu64(rax.wrapping_add(P6), rsi); rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; crate::simd_ops::wu32(rax, ((rcx as u32)) as u32); crate::simd_ops::wu32(rax.wrapping_add(P7), (0x2u32) as u32); crate::simd_ops::wu64(rax.wrapping_add(P8), rdi); crate::simd_ops::wu32(rax.wrapping_add(P9), (0x0u32) as u32); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1412fef70(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143bbd170; const P1: u64 = 0x1413103c0; const P2: u64 = 0x143bbd168; const P3: u64 = 0x10; const P4: u64 = 0x20; const P5: u64 = 0x4; const P6: u64 = 0x18; const P7: u64 = 0x28;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rsi = (0xcbf29ce484222325u64) as u64; rcx = (0xffffffffffffffffu64) as u64; rdi = rsi; r9 = rcx; bb = 1; continue; }
            1 => { r9 = ((r9).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru8(r8.wrapping_add(r9))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 1; continue; } bb = 2; continue; }
            2 => { r9 = ((r9).wrapping_add(r8)) as u64; r10 = (0x100000001b3u64) as u64; fl_l = r8; fl_r = r9; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rax = (((crate::simd_ops::ru8(r8)) as i8 as i64) as u64) as u64; r8 = ((r8).wrapping_add(1)) as u64; rdi = (((rdi) ^ (rax))) as u64; rdi = (((rdi) as i64).wrapping_mul((r10) as i64) as u64) as u64; fl_l = r8; fl_r = r9; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } bb = 4; continue; }
            4 => { rbx = rsi; bb = 5; continue; }
            5 => { rcx = ((rcx).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(rcx))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 5; continue; } bb = 6; continue; }
            6 => { rcx = ((rcx).wrapping_add(rdx)) as u64; fl_l = rdx; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 8; continue; } bb = 7; continue; }
            7 => { rax = (((crate::simd_ops::ru8(rdx)) as i8 as i64) as u64) as u64; rdx = ((rdx).wrapping_add(1)) as u64; rbx = (((rbx) ^ (rax))) as u64; rbx = (((rbx) as i64).wrapping_mul((r10) as i64) as u64) as u64; fl_l = rdx; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 7; continue; } bb = 8; continue; }
            8 => { rcx = P0; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru32(P2)) as u32 as u64; crate::simd_ops::wu64(rax.wrapping_add(P3), rbx); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu64(rax.wrapping_add(P4), rsi); rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; crate::simd_ops::wu32(rax, ((rcx as u32)) as u32); crate::simd_ops::wu32(rax.wrapping_add(P5), (0x2u32) as u32); crate::simd_ops::wu64(rax.wrapping_add(P6), rdi); crate::simd_ops::wu32(rax.wrapping_add(P7), (0x0u32) as u32); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1412ff040(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1433f5b08; const P1: u64 = 0x1433f5b13; const P2: u64 = 0x1433f5b7e; const P3: u64 = 0x1433f5b70; const P4: u64 = 0x143bbd170; const P5: u64 = 0x1413103c0; const P6: u64 = 0x143bbd168; const P7: u64 = 0x10; const P8: u64 = 0x20; const P9: u64 = 0x4; const P10: u64 = 0x18; const P11: u64 = 0x28;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rsi = (0xcbf29ce484222325u64) as u64; rcx = (0xffffffffffffffffu64) as u64; rbx = rsi; bb = 1; continue; }
            1 => { rcx = ((rcx).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru8(r9.wrapping_add(rcx))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 1; continue; } bb = 2; continue; }
            2 => { rcx = ((rcx).wrapping_add(r9)) as u64; r8 = (0x100000001b3u64) as u64; fl_l = r9; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 5; continue; } bb = 3; continue; }
            3 => { bb = 4; continue; }
            4 => { rax = (((crate::simd_ops::ru8(r9)) as i8 as i64) as u64) as u64; r9 = ((r9).wrapping_add(1)) as u64; rbx = (((rbx) ^ (rax))) as u64; rbx = (((rbx) as i64).wrapping_mul((r8) as i64) as u64) as u64; fl_l = r9; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 4; continue; } bb = 5; continue; }
            5 => { rcx = P0; rdi = rsi; rdx = P1; bb = 6; continue; }
            6 => { rax = (((crate::simd_ops::ru8(rcx)) as i8 as i64) as u64) as u64; rcx = ((rcx).wrapping_add(1)) as u64; rdi = (((rdi) ^ (rax))) as u64; rdi = (((rdi) as i64).wrapping_mul((r8) as i64) as u64) as u64; fl_l = rcx; fl_r = rdx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 6; continue; } bb = 7; continue; }
            7 => { rdx = P2; rcx = P3; bb = 8; continue; }
            8 => { rax = (((crate::simd_ops::ru8(rcx)) as i8 as i64) as u64) as u64; rcx = ((rcx).wrapping_add(1)) as u64; rsi = (((rsi) ^ (rax))) as u64; rsi = (((rsi) as i64).wrapping_mul((r8) as i64) as u64) as u64; fl_l = rcx; fl_r = rdx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 8; continue; } bb = 9; continue; }
            9 => { rcx = P4; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru32(P6)) as u32 as u64; crate::simd_ops::wu64(rax.wrapping_add(P7), rsi); rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; crate::simd_ops::wu64(rax.wrapping_add(P8), rbx); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu32(rax, ((rcx as u32)) as u32); crate::simd_ops::wu32(rax.wrapping_add(P9), (0x2u32) as u32); crate::simd_ops::wu64(rax.wrapping_add(P10), rdi); crate::simd_ops::wu32(rax.wrapping_add(P11), (0x0u32) as u32); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1412ff130(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x208; const P1: u64 = 0x8;
    let (mut rax, mut r10, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { r10 = rcx; r9 = (0xcbf29ce484222325u64) as u64; r8 = (0xffffffffffffffffu64) as u64; bb = 1; continue; }
            1 => { r8 = ((r8).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(r8))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 1; continue; } bb = 2; continue; }
            2 => { r8 = ((r8).wrapping_add(rdx)) as u64; fl_l = rdx; fl_r = r8; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 5; continue; } bb = 3; continue; }
            3 => { rcx = (0x100000001b3u64) as u64; bb = 4; continue; }
            4 => { rax = (((crate::simd_ops::ru8(rdx)) as i8 as i64) as u64) as u64; rdx = ((rdx).wrapping_add(1)) as u64; r9 = (((r9) ^ (rax))) as u64; r9 = (((r9) as i64).wrapping_mul((rcx) as i64) as u64) as u64; fl_l = rdx; fl_r = r8; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 4; continue; } bb = 5; continue; }
            5 => { rax = 0; fl_l = 0; fl_r = 0; rcx = ((r10.wrapping_add(P0)) as u64) as u64; bb = 6; continue; }
            6 => { rdx = (crate::simd_ops::ru64(rcx)) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 7; continue; }
            7 => { fl_l = rdx; fl_r = r9; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 10; continue; } bb = 8; continue; }
            8 => { rax = (((rax as u32)).wrapping_add(1)) as u32 as u64; rcx = ((rcx).wrapping_add(0x8u64)) as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x40; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 6; continue; } bb = 9; continue; }
            9 => { rax = 0; fl_l = 0; fl_r = 0; return rax; }
            10 => { rax = (rax as u32 as i32 as i64) as u64; rax = (crate::simd_ops::ru64(r10.wrapping_add(rax.wrapping_mul(8)).wrapping_add(P1))) as u64; return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1412ff320(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143bbd1c0; const P1: u64 = 0x1413100d0; const P2: u64 = 0x10; const P3: u64 = 0x58; const P4: u64 = 0x40; const P5: u64 = 0x50; const P6: u64 = 0x54; const P7: u64 = 0x60; const P8: u64 = 0x68;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rsi = ((r9 as u32)) as u32 as u64; rdi = r8; rbx = (0xcbf29ce484222325u64) as u64; rcx = (0xffffffffffffffffu64) as u64; bb = 1; continue; }
            1 => { rcx = ((rcx).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(rcx))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 1; continue; } bb = 2; continue; }
            2 => { rcx = ((rcx).wrapping_add(rdx)) as u64; fl_l = rdx; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 5; continue; } bb = 3; continue; }
            3 => { r8 = (0x100000001b3u64) as u64; bb = 4; continue; }
            4 => { rax = (((crate::simd_ops::ru8(rdx)) as i8 as i64) as u64) as u64; rdx = ((rdx).wrapping_add(1)) as u64; rbx = (((rbx) ^ (rax))) as u64; rbx = (((rbx) as i64).wrapping_mul((r8) as i64) as u64) as u64; fl_l = rdx; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 4; continue; } bb = 5; continue; }
            5 => { rcx = P0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm0 = (crate::simd_ops::ru128(rdi)) as u128; rcx = (crate::simd_ops::ru32(rdi.wrapping_add(P2))) as u32 as u64; crate::simd_ops::wu64(rax, rbx); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu32(rax.wrapping_add(P3), ((rsi as u32)) as u32); rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; crate::simd_ops::wu128(rax.wrapping_add(P4), (xmm0) as u128); crate::simd_ops::wu32(rax.wrapping_add(P5), ((rcx as u32)) as u32); crate::simd_ops::wu32(rax.wrapping_add(P6), (0x1u32) as u32); crate::simd_ops::wu64(rax.wrapping_add(P7), (0x0u64) as u64); crate::simd_ops::wu64(rax.wrapping_add(P8), (0xffffffffffffffffu64) as u64); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1412ff530(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143bbd170; const P1: u64 = 0x1413103c0; const P2: u64 = 0x143bbd168;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = (0xcbf29ce484222325u64) as u64; rcx = (0xffffffffffffffffu64) as u64; rsi = rbx; r10 = rcx; r11 = r8; bb = 1; continue; }
            1 => { r10 = ((r10).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru8(r9.wrapping_add(r10))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 1; continue; } bb = 2; continue; }
            2 => { r10 = ((r10).wrapping_add(r9)) as u64; rbp = (0x100000001b3u64) as u64; fl_l = r9; fl_r = r10; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 5; continue; } bb = 3; continue; }
            3 => { bb = 4; continue; }
            4 => { rax = (((crate::simd_ops::ru8(r9)) as i8 as i64) as u64) as u64; r9 = ((r9).wrapping_add(1)) as u64; rsi = (((rsi) ^ (rax))) as u64; rsi = (((rsi) as i64).wrapping_mul((rbp) as i64) as u64) as u64; fl_l = r9; fl_r = r10; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 4; continue; } bb = 5; continue; }
            5 => { rdi = rbx; r8 = rcx; bb = 6; continue; }
            6 => { r8 = ((r8).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru8(r11.wrapping_add(r8))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 6; continue; } bb = 7; continue; }
            7 => { r8 = ((r8).wrapping_add(r11)) as u64; fl_l = r11; fl_r = r8; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 9; continue; } bb = 8; continue; }
            8 => { rax = (((crate::simd_ops::ru8(r11)) as i8 as i64) as u64) as u64; r11 = ((r11).wrapping_add(1)) as u64; rdi = (((rdi) ^ (rax))) as u64; rdi = (((rdi) as i64).wrapping_mul((rbp) as i64) as u64) as u64; fl_l = r11; fl_r = r8; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 8; continue; } bb = 9; continue; }
            9 => { rcx = ((rcx).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(rcx))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 9; continue; } bb = 10; continue; }
            10 => { rcx = ((rcx).wrapping_add(rdx)) as u64; fl_l = rdx; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 13; continue; } bb = 11; continue; }
            11 => { bb = 12; continue; }
            12 => { rax = (((crate::simd_ops::ru8(rdx)) as i8 as i64) as u64) as u64; rdx = ((rdx).wrapping_add(1)) as u64; rbx = (((rbx) ^ (rax))) as u64; rbx = (((rbx) as i64).wrapping_mul((rbp) as i64) as u64) as u64; fl_l = rdx; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 12; continue; } bb = 13; continue; }
            13 => { rcx = P0; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru32(P2)) as u32 as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; crate::simd_ops::wu64(rax.wrapping_add(0x10u64), rbx); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu64(rax.wrapping_add(0x20u64), rsi); rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; crate::simd_ops::wu32(rax, ((rcx as u32)) as u32); crate::simd_ops::wu32(rax.wrapping_add(0x4u64), (0x2u32) as u32); crate::simd_ops::wu64(rax.wrapping_add(0x18u64), rdi); crate::simd_ops::wu32(rax.wrapping_add(0x28u64), (0x0u32) as u32); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1412ff9e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141834d64; const P1: u64 = 0x1433f3910; const P2: u64 = 0x8; const P3: u64 = 0x20; const P4: u64 = 0x28; const P5: u64 = 0x40; const P6: u64 = 0x48; const P7: u64 = 0x143bbd168; const P8: u64 = 0x143bbd180; const P9: u64 = 0x14128cba0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0; let mut ymm1_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x1412ff9e0, funcinfo_rva: 0x3699398, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0x68, conts: &[] };
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rcx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64); rsp = ((rsp).wrapping_sub(0x60u64)) as u64; rbx = (rdx) as u64; rax = ((rsp.wrapping_add(0x20u64)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), (rax) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), (0x0u64) as u64); rcx = (0x50u32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1412ffa05); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = ((P1) as u64) as u64; crate::simd_ops::wu64(rax, (rcx) as u64); let y_addr: u64 = rbx; xmm0 = core::ptr::read_unaligned(y_addr as *const u128); ymm0_hi = core::ptr::read_unaligned(y_addr.wrapping_add(16) as *const u128); let y_addr: u64 = rax.wrapping_add(P2); core::ptr::write_unaligned(y_addr as *mut u128, xmm0); core::ptr::write_unaligned(y_addr.wrapping_add(16) as *mut u128, ymm0_hi); let y_addr: u64 = rbx.wrapping_add(P3); xmm1 = core::ptr::read_unaligned(y_addr as *const u128); ymm1_hi = core::ptr::read_unaligned(y_addr.wrapping_add(16) as *const u128); let y_addr: u64 = rax.wrapping_add(P4); core::ptr::write_unaligned(y_addr as *mut u128, xmm1); core::ptr::write_unaligned(y_addr.wrapping_add(16) as *mut u128, ymm1_hi); xmm0 = ((crate::simd_ops::ru64(rbx.wrapping_add(P5))) as u64) as u128; ymm0_hi = 0; crate::simd_ops::wu64(rax.wrapping_add(P6), ((xmm0 as u64)) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), (rax) as u64); rax = ((rsp.wrapping_add(0x20u64)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), (rax) as u64); rax = (((crate::simd_ops::ru32(P7)) as i32 as i64) as u64) as u64; rcx = (((rax) as i64).wrapping_mul((0x2e0u64) as i64) as u64) as u64; rax = (crate::simd_ops::ru64(P8)) as u64; rax = ((rax).wrapping_add(0x10u64)) as u64; rcx = ((rcx).wrapping_add(rax)) as u64; rdx = ((rsp.wrapping_add(0x20u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1412ffa64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P9) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = ((rsp.wrapping_add(0x20u64)) as u64) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rdx) as u64 as u64; rdx = (rdx & !0xffu64) | (((if (((fl_l as u64)) != ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1412ffa82); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P3))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 2; continue; }
            2 => { rsp = ((rsp).wrapping_add(0x60u64)) as u64; rbx = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1412ffd10(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143bbd1c0; const P1: u64 = 0x1413100d0; const P2: u64 = 0x10; const P3: u64 = 0x40; const P4: u64 = 0x50; const P5: u64 = 0x54; const P6: u64 = 0x60; const P7: u64 = 0x68;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = r8; rbx = (0xcbf29ce484222325u64) as u64; rcx = (0xffffffffffffffffu64) as u64; bb = 1; continue; }
            1 => { rcx = ((rcx).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(rcx))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 1; continue; } bb = 2; continue; }
            2 => { rcx = ((rcx).wrapping_add(rdx)) as u64; fl_l = rdx; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 5; continue; } bb = 3; continue; }
            3 => { r8 = (0x100000001b3u64) as u64; bb = 4; continue; }
            4 => { rax = (((crate::simd_ops::ru8(rdx)) as i8 as i64) as u64) as u64; rdx = ((rdx).wrapping_add(1)) as u64; rbx = (((rbx) ^ (rax))) as u64; rbx = (((rbx) as i64).wrapping_mul((r8) as i64) as u64) as u64; fl_l = rdx; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 4; continue; } bb = 5; continue; }
            5 => { rcx = P0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm0 = (crate::simd_ops::ru128(rdi)) as u128; rcx = (crate::simd_ops::ru32(rdi.wrapping_add(P2))) as u32 as u64; rdx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rax, rbx); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu128(rax.wrapping_add(P3), (xmm0) as u128); crate::simd_ops::wu32(rax.wrapping_add(P4), ((rcx as u32)) as u32); crate::simd_ops::wu64(rax.wrapping_add(P5), (0x1u64) as u64); crate::simd_ops::wu64(rax.wrapping_add(P6), rdx); crate::simd_ops::wu64(rax.wrapping_add(P7), (0xffffffffffffffffu64) as u64); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1412fff40(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0xc; const P2: u64 = 0x1411de810; const P3: u64 = 0x20;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = rdx; rbx = rcx; rax = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rcx = ((rcx).wrapping_shl(3u32)) as u64; rax = (crate::simd_ops::ru32(rbx.wrapping_add(P1))) as u32 as u64; r8 = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; r8 = ((r8).wrapping_shl(3u32)) as u64; fl_l = rcx; fl_r = r8; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = (0x6666666666666667u64) as u64; let prod = (rax as i64 as i128).wrapping_mul((rcx) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; rdx = ((((rdx) as i64).wrapping_shr(4u32) as u64)) as u64; rax = rdx; rax = ((rax).wrapping_shr(63u32)) as u64; rdx = ((rdx).wrapping_add(1)) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(P0))) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = (crate::simd_ops::ru64(rbx)) as u64; let y_addr: u64 = rdi; xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rax.wrapping_add(rcx.wrapping_mul(8)); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); xmm1 = ((crate::simd_ops::ru64(rdi.wrapping_add(P3))) as u64) as u128; crate::simd_ops::wu64(rax.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(P3), ((xmm1 as u64)) as u64); crate::simd_ops::lk_add_u32(rbx.wrapping_add(P0), (1) as u32); rax = (crate::simd_ops::ru32(rbx.wrapping_add(P0))) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = (crate::simd_ops::ru64(rbx)) as u64; rax = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; rax = ((rax).wrapping_add(0xffffffffffffffd8u64)) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1412fffe0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0xc; const P2: u64 = 0x1413040a0; const P3: u64 = 0x14183acc2; const P4: u64 = 0x1412ead60;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = rcx; rax = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; rcx = (((rax) as i64).wrapping_mul((0xf0u64) as i64) as u64) as u64; rax = (crate::simd_ops::ru32(rdi.wrapping_add(P1))) as u32 as u64; rdx = (((rax) as i64).wrapping_mul((0xf0u64) as i64) as u64) as u64; fl_l = rcx; fl_r = rdx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = (0x8888888888888889u64) as u64; let prod = (rax as i64 as i128).wrapping_mul((rcx) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; rdx = ((rdx).wrapping_add(rcx)) as u64; rdx = ((((rdx) as i64).wrapping_shr(7u32) as u64)) as u64; rax = rdx; rax = ((rax).wrapping_shr(63u32)) as u64; rdx = ((rdx).wrapping_add(1)) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; rcx = rdi; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru32(rdi.wrapping_add(P0))) as u32 as u64; rbx = (((rax) as i64).wrapping_mul((0xf0u64) as i64) as u64) as u64; rbx = ((rbx).wrapping_add(crate::simd_ops::ru64(rdi))) as u64; rdx = 0; fl_l = 0; fl_r = 0; r8 = 0xf0; rcx = rbx; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rcx = rbx; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::lk_add_u32(rdi.wrapping_add(P0), (1) as u32); rax = (crate::simd_ops::ru32(rdi.wrapping_add(P0))) as u32 as u64; rax = (((rax) as i64).wrapping_mul((0xf0u64) as i64) as u64) as u64; rax = ((rax).wrapping_add(0xffffffffffffff10u64)) as u64; rax = ((rax).wrapping_add(crate::simd_ops::ru64(rdi))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141300080(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141834d64; const P1: u64 = 0x1433f3ab0; const P2: u64 = 0x8; const P3: u64 = 0x20; const P4: u64 = 0x28; const P5: u64 = 0x30; const P6: u64 = 0x38; const P7: u64 = 0x143bbd168; const P8: u64 = 0x143bbd180; const P9: u64 = 0x14128cba0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x141300080, funcinfo_rva: 0x3699398, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0x68, conts: &[] };
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rcx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64); rsp = ((rsp).wrapping_sub(0x60u64)) as u64; rbx = (rdx) as u64; rax = ((rsp.wrapping_add(0x20u64)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), (rax) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), (0x0u64) as u64); rcx = (0x40u32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1413000a5); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = ((P1) as u64) as u64; crate::simd_ops::wu64(rax, (rcx) as u64); let y_addr: u64 = rbx; xmm0 = core::ptr::read_unaligned(y_addr as *const u128); ymm0_hi = core::ptr::read_unaligned(y_addr.wrapping_add(16) as *const u128); let y_addr: u64 = rax.wrapping_add(P2); core::ptr::write_unaligned(y_addr as *mut u128, xmm0); core::ptr::write_unaligned(y_addr.wrapping_add(16) as *mut u128, ymm0_hi); xmm1 = (crate::simd_ops::ru128(rbx.wrapping_add(P3))) as u128; crate::simd_ops::wu128(rax.wrapping_add(P4), (xmm1) as u128); xmm0 = ((crate::simd_ops::ru64(rbx.wrapping_add(P5))) as u64) as u128; ymm0_hi = 0; crate::simd_ops::wu64(rax.wrapping_add(P6), ((xmm0 as u64)) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), (rax) as u64); rax = ((rsp.wrapping_add(0x20u64)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), (rax) as u64); rax = (((crate::simd_ops::ru32(P7)) as i32 as i64) as u64) as u64; rcx = (((rax) as i64).wrapping_mul((0x2e0u64) as i64) as u64) as u64; rax = (crate::simd_ops::ru64(P8)) as u64; rax = ((rax).wrapping_add(0x10u64)) as u64; rcx = ((rcx).wrapping_add(rax)) as u64; rdx = ((rsp.wrapping_add(0x20u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x141300104); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P9) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = ((rsp.wrapping_add(0x20u64)) as u64) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rdx) as u64 as u64; rdx = (rdx & !0xffu64) | (((if (((fl_l as u64)) != ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x141300122); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P3))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 2; continue; }
            2 => { rsp = ((rsp).wrapping_add(0x60u64)) as u64; rbx = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141300130(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1433ef040; const P1: u64 = 0x1433ef05f; const P2: u64 = 0x143bbd170; const P3: u64 = 0x1413103c0; const P4: u64 = 0x143bbd168; const P5: u64 = 0x10; const P6: u64 = 0x18; const P7: u64 = 0x4; const P8: u64 = 0x20; const P9: u64 = 0x28;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = (0xcbf29ce484222325u64) as u64; rdx = P0; rdi = rbx; r8 = P1; rcx = rdx; r9 = (0x100000001b3u64) as u64; bb = 1; continue; }
            1 => { rax = (((crate::simd_ops::ru8(rcx)) as i8 as i64) as u64) as u64; rcx = ((rcx).wrapping_add(1)) as u64; rdi = (((rdi) ^ (rax))) as u64; rdi = (((rdi) as i64).wrapping_mul((r9) as i64) as u64) as u64; fl_l = rcx; fl_r = r8; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } bb = 2; continue; }
            2 => { rsi = rbx; rcx = rdx; bb = 3; continue; }
            3 => { rax = (((crate::simd_ops::ru8(rcx)) as i8 as i64) as u64) as u64; rcx = ((rcx).wrapping_add(1)) as u64; rsi = (((rsi) ^ (rax))) as u64; rsi = (((rsi) as i64).wrapping_mul((r9) as i64) as u64) as u64; fl_l = rcx; fl_r = r8; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } bb = 4; continue; }
            4 => { bb = 5; continue; }
            5 => { rax = (((crate::simd_ops::ru8(rdx)) as i8 as i64) as u64) as u64; rdx = ((rdx).wrapping_add(1)) as u64; rbx = (((rbx) ^ (rax))) as u64; rbx = (((rbx) as i64).wrapping_mul((r9) as i64) as u64) as u64; fl_l = rdx; fl_r = r8; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 5; continue; } bb = 6; continue; }
            6 => { rcx = P2; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru32(P4)) as u32 as u64; crate::simd_ops::wu64(rax.wrapping_add(P5), rbx); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu64(rax.wrapping_add(P6), rsi); rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; crate::simd_ops::wu32(rax, ((rcx as u32)) as u32); crate::simd_ops::wu32(rax.wrapping_add(P7), (0x2u32) as u32); crate::simd_ops::wu64(rax.wrapping_add(P8), rdi); crate::simd_ops::wu32(rax.wrapping_add(P9), (0x0u32) as u32); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141300310(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1433f5b9a; const P1: u64 = 0x1433f5b90; const P2: u64 = 0x143bbd1c0; const P3: u64 = 0x1413100d0; const P4: u64 = 0x10; const P5: u64 = 0x40; const P6: u64 = 0x50; const P7: u64 = 0x54; const P8: u64 = 0x60; const P9: u64 = 0x68;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rdi = r8;
    rdx = P0;
    r8 = (0x100000001b3u64) as u64;
    rcx = P1;
    rbx = (0xcbf29ce484222325u64) as u64;
    loop { rax = (((crate::simd_ops::ru8(rcx)) as i8 as i64) as u64) as u64; rcx = ((rcx).wrapping_add(1)) as u64; rbx = (((rbx) ^ (rax))) as u64; rbx = (((rbx) as i64).wrapping_mul((r8) as i64) as u64) as u64; fl_l = rcx; fl_r = rdx; if !((((fl_l as u64)) != ((fl_r as u64)))) { break; } }
    rcx = P2;
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    xmm0 = (crate::simd_ops::ru128(rdi)) as u128;
    rcx = (crate::simd_ops::ru32(rdi.wrapping_add(P4))) as u32 as u64;
    rdx = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu64(rax, rbx);
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    crate::simd_ops::wu128(rax.wrapping_add(P5), (xmm0) as u128);
    crate::simd_ops::wu32(rax.wrapping_add(P6), ((rcx as u32)) as u32);
    crate::simd_ops::wu64(rax.wrapping_add(P7), (0x1u64) as u64);
    crate::simd_ops::wu64(rax.wrapping_add(P8), rdx);
    crate::simd_ops::wu64(rax.wrapping_add(P9), (0xffffffffffffffffu64) as u64);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141300580(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143bbd1c0; const P1: u64 = 0x1413100d0; const P2: u64 = 0x8; const P3: u64 = 0x54; const P4: u64 = 0x60; const P5: u64 = 0x68;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = (0xcbf29ce484222325u64) as u64; rcx = (0xffffffffffffffffu64) as u64; rdi = rbx; bb = 1; continue; }
            1 => { rcx = ((rcx).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru8(r8.wrapping_add(rcx))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 1; continue; } bb = 2; continue; }
            2 => { rcx = ((rcx).wrapping_add(r8)) as u64; r9 = (0x100000001b3u64) as u64; fl_l = r8; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 5; continue; } bb = 3; continue; }
            3 => { bb = 4; continue; }
            4 => { rax = (((crate::simd_ops::ru8(r8)) as i8 as i64) as u64) as u64; r8 = ((r8).wrapping_add(1)) as u64; rdi = (((rdi) ^ (rax))) as u64; rdi = (((rdi) as i64).wrapping_mul((r9) as i64) as u64) as u64; fl_l = r8; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 4; continue; } bb = 5; continue; }
            5 => { rcx = (0xffffffffffffffffu64) as u64; bb = 6; continue; }
            6 => { rcx = ((rcx).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(rcx))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 6; continue; } bb = 7; continue; }
            7 => { rcx = ((rcx).wrapping_add(rdx)) as u64; fl_l = rdx; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 9; continue; } bb = 8; continue; }
            8 => { rax = (((crate::simd_ops::ru8(rdx)) as i8 as i64) as u64) as u64; rdx = ((rdx).wrapping_add(1)) as u64; rbx = (((rbx) ^ (rax))) as u64; rbx = (((rbx) as i64).wrapping_mul((r9) as i64) as u64) as u64; fl_l = rdx; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 8; continue; } bb = 9; continue; }
            9 => { rcx = P0; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rax, rbx); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu64(rax.wrapping_add(P2), rdi); crate::simd_ops::wu64(rax.wrapping_add(P3), (0x2u64) as u64); crate::simd_ops::wu64(rax.wrapping_add(P4), rcx); crate::simd_ops::wu64(rax.wrapping_add(P5), (0xffffffffffffffffu64) as u64); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1413007c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1433f4018; const P1: u64 = 0x143bbd168; const P2: u64 = 0x143bbd180; const P3: u64 = 0x14128cba0; const P4: u64 = 0x20;
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
    rcx = ((rcx).wrapping_add(0x50u64)) as u64;
    rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(P2))) as u64;
    rdx = r11.wrapping_sub(0x48);
    rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp);
    rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64;
    fl_l = rcx; fl_r = rcx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = rsp.wrapping_add(0x20); fl_l = rcx; fl_r = rdx; rdx = (rdx & !0xffu64) | (((if (((fl_l as u64)) != ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P4))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); }
    rsp = ((rsp).wrapping_add(0x68u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141300bf0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1433f54d8; const P1: u64 = 0x1433f54c8; const P2: u64 = 0x143bbd1c0; const P3: u64 = 0x1413100d0; const P4: u64 = 0x10; const P5: u64 = 0x40; const P6: u64 = 0x50; const P7: u64 = 0x54; const P8: u64 = 0x58; const P9: u64 = 0x60; const P10: u64 = 0x68;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rdi = r8;
    rdx = P0;
    r8 = (0x100000001b3u64) as u64;
    rcx = P1;
    rbx = (0xcbf29ce484222325u64) as u64;
    loop { rax = (((crate::simd_ops::ru8(rcx)) as i8 as i64) as u64) as u64; rcx = ((rcx).wrapping_add(1)) as u64; rbx = (((rbx) ^ (rax))) as u64; rbx = (((rbx) as i64).wrapping_mul((r8) as i64) as u64) as u64; fl_l = rcx; fl_r = rdx; if !((((fl_l as u64)) != ((fl_r as u64)))) { break; } }
    rcx = P2;
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    xmm0 = (crate::simd_ops::ru128(rdi)) as u128;
    rcx = (crate::simd_ops::ru32(rdi.wrapping_add(P4))) as u32 as u64;
    crate::simd_ops::wu64(rax, rbx);
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    crate::simd_ops::wu128(rax.wrapping_add(P5), (xmm0) as u128);
    crate::simd_ops::wu32(rax.wrapping_add(P6), ((rcx as u32)) as u32);
    crate::simd_ops::wu32(rax.wrapping_add(P7), (0x1u32) as u32);
    crate::simd_ops::wu32(rax.wrapping_add(P8), (0x1u32) as u32);
    crate::simd_ops::wu64(rax.wrapping_add(P9), (0x0u64) as u64);
    crate::simd_ops::wu64(rax.wrapping_add(P10), (0xffffffffffffffffu64) as u64);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141300d00(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1433f3af8; const P1: u64 = 0x20; const P2: u64 = 0x143bbd168; const P3: u64 = 0x143bbd180; const P4: u64 = 0x14128cba0;
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
    xmm1 = (crate::simd_ops::ru128(rdx.wrapping_add(P1))) as u128;
    crate::simd_ops::wu128(rsp.wrapping_add(0x48u64), (xmm1) as u128);
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
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = rsp.wrapping_add(0x20); fl_l = rcx; fl_r = rdx; rdx = (rdx & !0xffu64) | (((if (((fl_l as u64)) != ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); }
    rsp = ((rsp).wrapping_add(0x68u64)) as u64;
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141300f10(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x18; const P2: u64 = 0x28; const P3: u64 = 0x30; const P4: u64 = 0x40; const P5: u64 = 0x48; const P6: u64 = 0x58; const P7: u64 = 0x8; const P8: u64 = 0x20; const P9: u64 = 0x38; const P10: u64 = 0x50;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { xmm0 = (crate::simd_ops::ru128(rdx)) as u128; crate::simd_ops::wu128(rcx, (xmm0) as u128); xmm1 = ((crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64) as u128; crate::simd_ops::wu64(rcx.wrapping_add(P0), ((xmm1 as u64)) as u64); xmm0 = (crate::simd_ops::ru128(rdx.wrapping_add(P1))) as u128; crate::simd_ops::wu128(rcx.wrapping_add(P1), (xmm0) as u128); xmm1 = ((crate::simd_ops::ru64(rdx.wrapping_add(P2))) as u64) as u128; crate::simd_ops::wu64(rcx.wrapping_add(P2), ((xmm1 as u64)) as u64); xmm0 = (crate::simd_ops::ru128(rdx.wrapping_add(P3))) as u128; crate::simd_ops::wu128(rcx.wrapping_add(P3), (xmm0) as u128); xmm1 = ((crate::simd_ops::ru64(rdx.wrapping_add(P4))) as u64) as u128; crate::simd_ops::wu64(rcx.wrapping_add(P4), ((xmm1 as u64)) as u64); xmm0 = (crate::simd_ops::ru128(rdx.wrapping_add(P5))) as u128; crate::simd_ops::wu128(rcx.wrapping_add(P5), (xmm0) as u128); xmm1 = ((crate::simd_ops::ru64(rdx.wrapping_add(P6))) as u64) as u128; crate::simd_ops::wu64(rcx.wrapping_add(P6), ((xmm1 as u64)) as u64); xmm0 = ((crate::simd_ops::ru64(rcx)) as u64) as u128; xmm2 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm0 as u64) - f64::from_bits(crate::simd_ops::ru64(r8))).to_bits() as u128); xmm0 = ((crate::simd_ops::ru64(rcx.wrapping_add(P7))) as u64) as u128; crate::simd_ops::wu64(rcx, ((xmm2 as u64)) as u64); xmm1 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm0 as u64) - f64::from_bits(crate::simd_ops::ru64(r8.wrapping_add(P7)))).to_bits() as u128); xmm0 = ((crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64) as u128; crate::simd_ops::wu64(rcx.wrapping_add(P7), ((xmm1 as u64)) as u64); xmm1 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm0 as u64) - f64::from_bits(crate::simd_ops::ru64(r8.wrapping_add(P0)))).to_bits() as u128); xmm0 = ((crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64) as u128; crate::simd_ops::wu64(rcx.wrapping_add(P0), ((xmm1 as u64)) as u64); xmm1 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm0 as u64) - f64::from_bits(crate::simd_ops::ru64(r8.wrapping_add(P1)))).to_bits() as u128); xmm0 = ((crate::simd_ops::ru64(rcx.wrapping_add(P8))) as u64) as u128; crate::simd_ops::wu64(rcx.wrapping_add(P1), ((xmm1 as u64)) as u64); xmm1 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm0 as u64) - f64::from_bits(crate::simd_ops::ru64(r8.wrapping_add(P8)))).to_bits() as u128); xmm0 = ((crate::simd_ops::ru64(rcx.wrapping_add(P2))) as u64) as u128; crate::simd_ops::wu64(rcx.wrapping_add(P8), ((xmm1 as u64)) as u64); xmm1 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm0 as u64) - f64::from_bits(crate::simd_ops::ru64(r8.wrapping_add(P2)))).to_bits() as u128); xmm0 = ((crate::simd_ops::ru64(rcx.wrapping_add(P3))) as u64) as u128; crate::simd_ops::wu64(rcx.wrapping_add(P2), ((xmm1 as u64)) as u64); xmm1 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm0 as u64) - f64::from_bits(crate::simd_ops::ru64(r8.wrapping_add(P3)))).to_bits() as u128); xmm0 = ((crate::simd_ops::ru64(rcx.wrapping_add(P9))) as u64) as u128; crate::simd_ops::wu64(rcx.wrapping_add(P3), ((xmm1 as u64)) as u64); xmm1 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm0 as u64) - f64::from_bits(crate::simd_ops::ru64(r8.wrapping_add(P9)))).to_bits() as u128); xmm0 = ((crate::simd_ops::ru64(rcx.wrapping_add(P4))) as u64) as u128; crate::simd_ops::wu64(rcx.wrapping_add(P9), ((xmm1 as u64)) as u64); xmm1 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm0 as u64) - f64::from_bits(crate::simd_ops::ru64(r8.wrapping_add(P4)))).to_bits() as u128); xmm0 = ((crate::simd_ops::ru64(rcx.wrapping_add(P5))) as u64) as u128; crate::simd_ops::wu64(rcx.wrapping_add(P4), ((xmm1 as u64)) as u64); xmm1 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm0 as u64) - f64::from_bits(crate::simd_ops::ru64(r8.wrapping_add(P5)))).to_bits() as u128); xmm0 = ((crate::simd_ops::ru64(rcx.wrapping_add(P10))) as u64) as u128; crate::simd_ops::wu64(rcx.wrapping_add(P5), ((xmm1 as u64)) as u64); xmm1 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm0 as u64) - f64::from_bits(crate::simd_ops::ru64(r8.wrapping_add(P10)))).to_bits() as u128); xmm0 = ((crate::simd_ops::ru64(rcx.wrapping_add(P6))) as u64) as u128; crate::simd_ops::wu64(rcx.wrapping_add(P10), ((xmm1 as u64)) as u64); xmm1 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm0 as u64) - f64::from_bits(crate::simd_ops::ru64(r8.wrapping_add(P6)))).to_bits() as u128); crate::simd_ops::wu64(rcx.wrapping_add(P6), ((xmm1 as u64)) as u64); rax = rcx; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141301020(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0xc; const P2: u64 = 0x14241c0a0; const P3: u64 = 0x14183acc8; const P4: u64 = 0x14241c190;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; r15 = rdx; rbx = rcx; rbp = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; r14 = ((rbp.wrapping_mul(4)) as u64) as u64; rdi = (crate::simd_ops::ru64(rcx)) as u64; rcx = (crate::simd_ops::ru32(rcx.wrapping_add(P1))) as u32 as u64; rax = ((rcx.wrapping_mul(4)) as u64) as u64; fl_l = r14; fl_r = rax; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 4; continue; } bb = 1; continue; }
            1 => { rax = rbp.wrapping_add(0x1); rsi = ((rcx as u32)) as u32 as u64; rsi = ((rsi).wrapping_shr(1u32)) as u64; rsi = ((rsi).wrapping_add(rcx)) as u64; fl_l = rsi; fl_r = rax; rsi = (if (((fl_l as u64)) < ((fl_r as u64))) { rax } else { rsi }) as u64; rcx = ((rsi.wrapping_mul(4)) as u64) as u64; rdx = 0x4; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rdi = rax; r8 = (crate::simd_ops::ru32(rbx.wrapping_add(P0))) as u32 as u64; r8 = ((r8).wrapping_shl(2u32)) as u64; rdx = (crate::simd_ops::ru64(rbx)) as u64; rcx = rax; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rbx)) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); bb = 3; continue; }
            3 => { crate::simd_ops::wu64(rbx, rdi); crate::simd_ops::wu32(rbx.wrapping_add(P0), ((rbp as u32)) as u32); crate::simd_ops::wu32(rbx.wrapping_add(P1), ((rsi as u32)) as u32); bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru32(r15)) as u32 as u64; crate::simd_ops::wu32(r14.wrapping_add(rdi), ((rax as u32)) as u32); crate::simd_ops::lk_add_u32(rbx.wrapping_add(P0), (1) as u32); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141301210(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1433f549d; const P1: u64 = 0x1433f5480; const P2: u64 = 0x143bbd1c0; const P3: u64 = 0x1413100d0; const P4: u64 = 0x20; const P5: u64 = 0x10; const P6: u64 = 0x30; const P7: u64 = 0x54; const P8: u64 = 0x58; const P9: u64 = 0x60; const P10: u64 = 0x68;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rdi = r8;
    rdx = P0;
    r8 = (0x100000001b3u64) as u64;
    rcx = P1;
    rbx = (0xcbf29ce484222325u64) as u64;
    loop { rax = (((crate::simd_ops::ru8(rcx)) as i8 as i64) as u64) as u64; rcx = ((rcx).wrapping_add(1)) as u64; rbx = (((rbx) ^ (rax))) as u64; rbx = (((rbx) as i64).wrapping_mul((r8) as i64) as u64) as u64; fl_l = rcx; fl_r = rdx; if !((((fl_l as u64)) != ((fl_r as u64)))) { break; } }
    rcx = P2;
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    let y_addr: u64 = rdi; xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16));
    xmm1 = (crate::simd_ops::ru128(rdi.wrapping_add(P4))) as u128;
    rcx = 0;
    fl_l = 0; fl_r = 0;
    let y_addr: u64 = rax.wrapping_add(P5); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi);
    crate::simd_ops::wu128(rax.wrapping_add(P6), (xmm1) as u128);
    crate::simd_ops::wu64(rax, rbx);
    crate::simd_ops::wu32(rax.wrapping_add(P7), ((rcx as u32)) as u32);
    crate::simd_ops::wu32(rax.wrapping_add(P8), (0x1u32) as u32);
    crate::simd_ops::wu64(rax.wrapping_add(P9), rcx);
    crate::simd_ops::wu64(rax.wrapping_add(P10), (0xffffffffffffffffu64) as u64);
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141301ba0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = 0x2;
    fl_l = rcx; fl_r = rax;
    rcx = (if (((fl_l as u64)) > ((fl_r as u64))) { rax } else { rcx }) as u64;
    rax = rcx;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141302330(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1433f6148; const P1: u64 = 0x1433f6140; const P2: u64 = 0x143bbd1c0; const P3: u64 = 0x1413100d0; const P4: u64 = 0x20; const P5: u64 = 0x10; const P6: u64 = 0x30; const P7: u64 = 0x54; const P8: u64 = 0x60; const P9: u64 = 0x68;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rdi = r8;
    rdx = P0;
    r8 = (0x100000001b3u64) as u64;
    rcx = P1;
    rbx = (0xcbf29ce484222325u64) as u64;
    loop { rax = (((crate::simd_ops::ru8(rcx)) as i8 as i64) as u64) as u64; rcx = ((rcx).wrapping_add(1)) as u64; rbx = (((rbx) ^ (rax))) as u64; rbx = (((rbx) as i64).wrapping_mul((r8) as i64) as u64) as u64; fl_l = rcx; fl_r = rdx; if !((((fl_l as u64)) != ((fl_r as u64)))) { break; } }
    rcx = P2;
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    let y_addr: u64 = rdi; xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16));
    xmm1 = (crate::simd_ops::ru128(rdi.wrapping_add(P4))) as u128;
    rcx = 0;
    fl_l = 0; fl_r = 0;
    let y_addr: u64 = rax.wrapping_add(P5); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi);
    crate::simd_ops::wu128(rax.wrapping_add(P6), (xmm1) as u128);
    crate::simd_ops::wu64(rax, rbx);
    crate::simd_ops::wu64(rax.wrapping_add(P7), rcx);
    crate::simd_ops::wu64(rax.wrapping_add(P8), rcx);
    crate::simd_ops::wu64(rax.wrapping_add(P9), (0xffffffffffffffffu64) as u64);
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1413023c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1433f6140; const P1: u64 = 0x1433f6148; const P2: u64 = 0x143bbd170; const P3: u64 = 0x1413103c0; const P4: u64 = 0x143bbd168; const P5: u64 = 0x18; const P6: u64 = 0x20; const P7: u64 = 0x4; const P8: u64 = 0x10; const P9: u64 = 0x28;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rsi = (0xcbf29ce484222325u64) as u64; rdx = P0; rbx = rsi; r8 = P1; rcx = rdx; r9 = (0x100000001b3u64) as u64; bb = 1; continue; }
            1 => { rax = (((crate::simd_ops::ru8(rcx)) as i8 as i64) as u64) as u64; rcx = ((rcx).wrapping_add(1)) as u64; rbx = (((rbx) ^ (rax))) as u64; rbx = (((rbx) as i64).wrapping_mul((r9) as i64) as u64) as u64; fl_l = rcx; fl_r = r8; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } bb = 2; continue; }
            2 => { rdi = rsi; bb = 3; continue; }
            3 => { rax = (((crate::simd_ops::ru8(rdx)) as i8 as i64) as u64) as u64; rdx = ((rdx).wrapping_add(1)) as u64; rdi = (((rdi) ^ (rax))) as u64; rdi = (((rdi) as i64).wrapping_mul((r9) as i64) as u64) as u64; fl_l = rdx; fl_r = r8; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } bb = 4; continue; }
            4 => { rcx = P2; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru32(P4)) as u32 as u64; crate::simd_ops::wu64(rax.wrapping_add(P5), rsi); rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; crate::simd_ops::wu64(rax.wrapping_add(P6), rbx); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu32(rax, ((rcx as u32)) as u32); crate::simd_ops::wu32(rax.wrapping_add(P7), (0x2u32) as u32); crate::simd_ops::wu64(rax.wrapping_add(P8), rdi); crate::simd_ops::wu32(rax.wrapping_add(P9), (0x0u32) as u32); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1413027a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x30; const P1: u64 = 0x10; const P2: u64 = 0x18; const P3: u64 = 0xffffffffffffffe8; const P4: u64 = 0x142535070; const P5: u64 = 0x1426456c8; const P6: u64 = 0x1426453a0; const P7: u64 = 0x8; const P8: u64 = 0x20;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rax = ((crate::exe::tls_pointer() as u64)) as u64; rbx = rcx; rcx = 0x6348; rdi = 0; fl_l = 0; fl_r = 0; rdx = (crate::simd_ops::ru64(rax)) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(rdx))) as u64; r8 = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rdx = (crate::simd_ops::ru64(r8.wrapping_add(P1))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { fl_l = ((rdx as u8)) as u8 as u64; fl_r = (0xfu8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::lk_add_u32(r8.wrapping_add(P2), (1) as u32)) as u32 as u64; rax = (crate::simd_ops::ru64(rdx)) as u64; crate::simd_ops::wu64(r8.wrapping_add(P1), rax); bb = 5; continue; }
            3 => { rdx = 0x28; r8 = ((rdx.wrapping_add(P3)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rdx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp); rcx = rax; rdx = 0xdeadd00d; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P6), rax), rax, rcx, rdx, r8, r9, rsp); rdx = rdi; bb = 5; continue; }
            5 => { crate::simd_ops::wu64(rdx, rdi); rax = rbx; crate::simd_ops::wu64(rdx.wrapping_add(P7), rdi); crate::simd_ops::wu64(rdx.wrapping_add(P1), rdi); crate::simd_ops::wu64(rdx.wrapping_add(P2), rdi); crate::simd_ops::wu64(rdx.wrapping_add(P8), rdi); crate::simd_ops::wu64(rbx, rdx); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141302900(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1433f557f; const P1: u64 = 0x1433f5570; const P2: u64 = 0x143bbd1d0; const P3: u64 = 0x143bbd1d8; const P4: u64 = 0x40; const P5: u64 = 0x60;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut ymm2_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 15>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x78u64)) as u64; r8 = (0xcbf29ce484222325u64) as u64; rdx = P0; rcx = P1; r9 = (0x100000001b3u64) as u64; bb = 1; continue; }
            1 => { rax = (((crate::simd_ops::ru8(rcx)) as i8 as i64) as u64) as u64; rcx = ((rcx).wrapping_add(1)) as u64; r8 = (((r8) ^ (rax))) as u64; r8 = (((r8) as i64).wrapping_mul((r9) as i64) as u64) as u64; fl_l = rcx; fl_r = rdx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(P2)) as u64; rcx = (crate::simd_ops::ru32(P3)) as u32 as u64; rcx = (((rcx) as i64).wrapping_mul((0x70u64) as i64) as u64) as u64; rcx = ((rcx).wrapping_add(rax)) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 7; continue; } bb = 3; continue; }
            3 => { bb = 4; continue; }
            4 => { let y_addr: u64 = rax.wrapping_add(P4); xmm2 = crate::simd_ops::ru128(y_addr); ymm2_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); xmm3 = (crate::simd_ops::ru128(rax.wrapping_add(P5))) as u128; fl_l = (crate::simd_ops::ru64(rax)) as u64; fl_r = r8; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 6; continue; } bb = 5; continue; }
            5 => { xmm0 = (ymm2_hi) as u128; rdx = (((xmm0 >> 32) as u32) as u32) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 8; continue; } bb = 6; continue; }
            6 => { rax = ((rax).wrapping_add(0x70u64)) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 4; continue; } bb = 7; continue; }
            7 => { rax = 0; fl_l = 0; fl_r = 0; rsp = ((rsp).wrapping_add(0x78u64)) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            8 => { rax = ((xmm3 as u64)) as u64; rsp = ((rsp).wrapping_add(0x78u64)) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141302c80(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142645790; const P1: u64 = 0x1426455a8; const P2: u64 = 0x1401de830; const P3: u64 = 0x142442de0; const P4: u64 = 0x142645598; const P5: u64 = 0x1403496f0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 15; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rdx; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); rdi = ((rax as u32)) as u32 as u64; rcx = (crate::simd_ops::ru32(rbx.wrapping_add(0x8u64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 3; continue; } bb = 2; continue; }
            2 => { rcx = rbx; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu32(rbx.wrapping_add(0x8u64), ((rdi as u32)) as u32); bb = 3; continue; }
            3 => { crate::simd_ops::lk_add_u32(rbx.wrapping_add(0xcu64), (1) as u32); fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x20u64))) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 12; continue; } bb = 4; continue; }
            4 => { rbp = 0; fl_l = 0; fl_r = 0; bb = 5; continue; }
            5 => { r8 = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x18u64))) as u32 as u64; rcx = (crate::simd_ops::ru32(rbx.wrapping_add(0x24u64))) as u32 as u64; rdx = 0; fl_l = 0; fl_r = 0; let d = (rcx) as u128; let n = ((rdx as u128) << 64) | (rax as u128); if d != 0 { rax = (n / d) as u64; rdx = (n % d) as u64; } rdi = (crate::simd_ops::ru64(r8.wrapping_add(rdx.wrapping_mul(8)))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 6; continue; }
            6 => { crate::simd_ops::wu32(rdi.wrapping_add(0x8u64), ((rbp as u32)) as u32); rcx = rdi; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rcx = rdi; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 7; continue; }
            7 => { rcx = (crate::simd_ops::ru32(rbx.wrapping_add(0x20u64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 11; continue; } bb = 8; continue; }
            8 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x18u64))) as u32 as u64; rax = (((rax as u32)).wrapping_add(1)) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x18u64), ((rax as u32)) as u32); fl_l = ((rax as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rbx.wrapping_add(0x24u64))) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 10; continue; } bb = 9; continue; }
            9 => { crate::simd_ops::wu32(rbx.wrapping_add(0x18u64), ((rbp as u32)) as u32); bb = 10; continue; }
            10 => { fl_l = (((rcx as u32)).wrapping_sub(1)) as u32 as u64; rcx = ((fl_l as u32)) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x20u64), ((rcx as u32)) as u32); bb = 11; continue; }
            11 => { crate::simd_ops::lk_sub_u64(rbx.wrapping_add(0x28u64), (1) as u64); fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 5; continue; } bb = 12; continue; }
            12 => { fl_l = (crate::simd_ops::lk_add_u32(rbx.wrapping_add(0xcu64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; if (((fl_l as u32)) != 0) { bb = 14; continue; } bb = 13; continue; }
            13 => { rax = 0xffffffff; let tmp_xchg: u32 = crate::simd_ops::atomic_xchg_u32(rbx.wrapping_add(0x8u64), ((rax as u32)) as u32); rax = (tmp_xchg) as u32 as u64; rcx = rbx; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp); bb = 14; continue; }
            14 => { rcx = rbx.wrapping_add(0x10); rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rcx = rbx; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); bb = 15; continue; }
            15 => { return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141302d60(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14209d670; const P1: u64 = 0x1411e7fc0; const P2: u64 = 0x142442de0; const P3: u64 = 0x1401de830;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbp = rdx; rbx = (crate::simd_ops::ru64(rdx.wrapping_add(0x8u64))) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x10u64))) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rdi = ((rbx.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; r12 = 0; fl_l = 0; fl_r = 0; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 5; continue; } bb = 2; continue; }
            2 => { rsi = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { rcx = rsi; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rcx = rsi.wrapping_add(0x58); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rcx = rsi; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 4; continue; }
            4 => { crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), r12); rbx = ((rbx).wrapping_add(0x28u64)) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 5; continue; }
            5 => { rbx = (crate::simd_ops::ru64(rbp.wrapping_add(0x18u64))) as u64; rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x20u64))) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rdi = ((rbx.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 9; continue; } bb = 6; continue; }
            6 => { rsi = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 7; continue; }
            7 => { rcx = rsi; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rcx = rsi.wrapping_add(0x58); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rcx = rsi; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 8; continue; }
            8 => { crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), r12); rbx = ((rbx).wrapping_add(0x28u64)) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 6; continue; } bb = 9; continue; }
            9 => { rcx = rbp.wrapping_add(0x18); rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rcx = rbp.wrapping_add(0x8); rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rcx = rbp; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); bb = 10; continue; }
            10 => { return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141302e60(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14397fed0; const P1: u64 = 0x170; const P2: u64 = 0x1401de830; const P3: u64 = 0x158; const P4: u64 = 0x1403496f0; const P5: u64 = 0x148; const P6: u64 = 0x1412ec030; const P7: u64 = 0x130; const P8: u64 = 0x120; const P9: u64 = 0x1412ebf30; const P10: u64 = 0x110; const P11: u64 = 0x100; const P12: u64 = 0x1433f11b8; const P13: u64 = 0x1412eb320; const P14: u64 = 0x142442de0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rdx; crate::simd_ops::wu64(P0, (0x0u64) as u64); rcx = ((rdx.wrapping_add(P1)) as u64) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rcx = ((rbx.wrapping_add(P3)) as u64) as u64; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rcx = ((rbx.wrapping_add(P5)) as u64) as u64; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); rcx = ((rbx.wrapping_add(P7)) as u64) as u64; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rcx = ((rbx.wrapping_add(P8)) as u64) as u64; rax = crate::simd_ops::call_stk(P9, rax, rcx, rdx, r8, r9, rsp); rcx = ((rbx.wrapping_add(P10)) as u64) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rcx = ((rbx.wrapping_add(P11)) as u64) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rcx = rbx.wrapping_add(0xf0); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rcx = rbx.wrapping_add(0xe0); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rcx = rbx.wrapping_add(0xd0); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rcx = rbx.wrapping_add(0xc0); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rcx = rbx.wrapping_add(0xa8); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rcx = rbx.wrapping_add(0x98); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rax = P12; crate::simd_ops::wu64(rbx, rax); rcx = rbx; rax = crate::simd_ops::call_stk(P13, rax, rcx, rdx, r8, r9, rsp); rcx = rbx; rax = crate::simd_ops::call_stk(P14, rax, rcx, rdx, r8, r9, rsp); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); bb = 2; continue; }
            2 => { return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141302f50(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14397ff70; const P1: u64 = 0xb0; const P2: u64 = 0x1401de830; const P3: u64 = 0xa0; const P4: u64 = 0x90; const P5: u64 = 0x78; const P6: u64 = 0x60; const P7: u64 = 0x48; const P8: u64 = 0x14130d260; const P9: u64 = 0x30; const P10: u64 = 0x14130cc70; const P11: u64 = 0x20; const P12: u64 = 0x142442de0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rdx; crate::simd_ops::wu64(P0, (0x0u64) as u64); rcx = ((rdx.wrapping_add(P1)) as u64) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rcx = ((rbx.wrapping_add(P3)) as u64) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rcx = ((rbx.wrapping_add(P4)) as u64) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rcx = ((rbx.wrapping_add(P5)) as u64) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rcx = ((rbx.wrapping_add(P6)) as u64) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rcx = ((rbx.wrapping_add(P7)) as u64) as u64; rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); rcx = ((rbx.wrapping_add(P9)) as u64) as u64; rax = crate::simd_ops::call_stk(P10, rax, rcx, rdx, r8, r9, rsp); rcx = ((rbx.wrapping_add(P11)) as u64) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rcx = rbx.wrapping_add(0x10); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rcx = rbx; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rcx = rbx; rax = crate::simd_ops::call_stk(P12, rax, rcx, rdx, r8, r9, rsp); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); bb = 2; continue; }
            2 => { return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141303090(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x14204f860; const P2: u64 = 0x142442de0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    fl_l = rdx; fl_r = rdx;
    if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { return rax; }
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rdx;
    rdi = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
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
pub unsafe extern "win64" fn fn_1413030f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x20; const P1: u64 = 0x14397ff18; const P2: u64 = 0x142258080; const P3: u64 = 0x18; const P4: u64 = 0x10; const P5: u64 = 0x8; const P6: u64 = 0x142442de0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 1; continue; }
            1 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rdx; rdx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { rcx = (crate::simd_ops::ru64(P1)) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 3; continue; }
            3 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P3))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rcx = (crate::simd_ops::ru64(P1)) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 5; continue; }
            5 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P4))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 6; continue; }
            6 => { rcx = (crate::simd_ops::ru64(P1)) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 7; continue; }
            7 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P5))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 8; continue; }
            8 => { rcx = (crate::simd_ops::ru64(P1)) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 9; continue; }
            9 => { rdx = (crate::simd_ops::ru64(rbx)) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 10; continue; }
            10 => { rcx = (crate::simd_ops::ru64(P1)) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 11; continue; }
            11 => { rcx = rbx; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); bb = 12; continue; }
            12 => { return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141303180(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1422b57f0; const P1: u64 = 0x8; const P2: u64 = 0x1413087a0; const P3: u64 = 0x10; const P4: u64 = 0x18; const P5: u64 = 0x14397ff68; const P6: u64 = 0x1413086e0; const P7: u64 = 0x142442de0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = rdx; rcx = rdx; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rdx = (crate::simd_ops::ru64(rdi)) as u64; r8 = (crate::simd_ops::ru32(rdi.wrapping_add(P1))) as u32 as u64; r8 = ((r8).wrapping_shl(5u32)) as u64; r8 = ((r8).wrapping_add(rdx)) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu32(rdi.wrapping_add(P1), (0x0u32) as u32); rdx = (crate::simd_ops::ru64(rdi.wrapping_add(P3))) as u64; r8 = (crate::simd_ops::ru32(rdi.wrapping_add(P4))) as u32 as u64; r8 = ((r8).wrapping_shl(5u32)) as u64; r8 = ((r8).wrapping_add(rdx)) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu32(rdi.wrapping_add(P4), (0x0u32) as u32); crate::simd_ops::wu64(P5, (0x0u64) as u64); rcx = ((rdi.wrapping_add(P3)) as u64) as u64; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); rcx = rdi; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); rcx = rdi; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); bb = 2; continue; }
            2 => { return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141303210(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143a00228; const P1: u64 = 0xf0f60; const P2: u64 = 0x14209d670; const P3: u64 = 0x58; const P4: u64 = 0x1411e7fc0; const P5: u64 = 0x142442de0; const P6: u64 = 0x40; const P7: u64 = 0x1401de830; const P8: u64 = 0x30; const P9: u64 = 0x141369010; const P10: u64 = 0x1401e5070;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rdx; crate::simd_ops::wu64(P0, (0x0u64) as u64); rdi = (crate::simd_ops::ru64(rdx.wrapping_add(P1))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { rcx = rdi; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rcx = ((rdi.wrapping_add(P3)) as u64) as u64; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rcx = rdi; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); bb = 3; continue; }
            3 => { rdi = (crate::simd_ops::ru64(rbx.wrapping_add(P3))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rcx = rdi; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rcx = ((rdi.wrapping_add(P3)) as u64) as u64; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rcx = rdi; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); bb = 5; continue; }
            5 => { rcx = ((rbx.wrapping_add(P6)) as u64) as u64; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); rcx = ((rbx.wrapping_add(P8)) as u64) as u64; rax = crate::simd_ops::call_stk(P9, rax, rcx, rdx, r8, r9, rsp); rcx = rbx; rax = crate::simd_ops::call_stk(P10, rax, rcx, rdx, r8, r9, rsp); rcx = rbx; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); bb = 6; continue; }
            6 => { return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1413032b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143a37918; const P1: u64 = 0x14206b020; const P2: u64 = 0x142442de0; const P3: u64 = 0x14397ff10; const P4: u64 = 0x1412f5b10; const P5: u64 = 0x141834ee8; const P6: u64 = 0x14209d670; const P7: u64 = 0x1411e7fc0; const P8: u64 = 0x142256c90; const P9: u64 = 0x142256ee0; const P10: u64 = 0x1401de400; const P11: u64 = 0x1401de830; const P12: u64 = 0x1401e5380; const P13: u64 = 0x1412f6080; const P14: u64 = 0x1401e66f0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rdx; rdi = (crate::simd_ops::ru64(P0)) as u64; crate::simd_ops::wu64(P0, (0x0u64) as u64); fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 2; continue; }
            2 => { rsi = (crate::simd_ops::ru64(rdi)) as u64; fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { rcx = rsi; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rcx = rsi; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 4; continue; }
            4 => { rcx = rdi; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 5; continue; }
            5 => { crate::simd_ops::wu64(P3, (0x0u64) as u64); rcx = rbx.wrapping_add(0x2c8); r9 = P4; rdx = 0x10; r8 = ((rdx.wrapping_sub(0xeu64)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rdi = (crate::simd_ops::ru64(rbx.wrapping_add(0x2a0u64))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 6; continue; }
            6 => { rcx = rdi; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); rcx = rdi.wrapping_add(0x58); rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); rcx = rdi; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 7; continue; }
            7 => { rcx = rbx.wrapping_add(0x1c8); rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); rcx = rbx.wrapping_add(0xa8); r9 = P9; rdx = 0x48; r8 = ((rdx.wrapping_sub(0x44u64)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rcx = rbx.wrapping_add(0x68); r9 = P10; rdx = 0x10; r8 = ((rdx.wrapping_sub(0xcu64)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rcx = rbx.wrapping_add(0x58); rax = crate::simd_ops::call_stk(P11, rax, rcx, rdx, r8, r9, rsp); rcx = rbx.wrapping_add(0x48); rax = crate::simd_ops::call_stk(P11, rax, rcx, rdx, r8, r9, rsp); rcx = rbx.wrapping_add(0x20); rax = crate::simd_ops::call_stk(P12, rax, rcx, rdx, r8, r9, rsp); rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; rax = crate::simd_ops::call_stk(P13, rax, rcx, rdx, r8, r9, rsp); rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; rax = crate::simd_ops::call_stk(P14, rax, rcx, rdx, r8, r9, rsp); rcx = rbx; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); bb = 8; continue; }
            8 => { return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1413033e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143a082f8; const P1: u64 = 0x142485780; const P2: u64 = 0x14397fea8; const P3: u64 = 0x300; const P4: u64 = 0x1411d3020; const P5: u64 = 0x2f0; const P6: u64 = 0x143a365b8; const P7: u64 = 0x310; const P8: u64 = 0x1401de830; const P9: u64 = 0x1c0; const P10: u64 = 0x1412ddf50; const P11: u64 = 0x1412ddfb0; const P12: u64 = 0x142442de0; const P13: u64 = 0x1412df190; const P14: u64 = 0x1412deb50; const P15: u64 = 0x141834ee8; const P16: u64 = 0x1410c05d0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rdx; rdx = ((rdx).wrapping_add(0x28u64)) as u64; rcx = (crate::simd_ops::ru64(P0)) as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(P2, (0x0u64) as u64); rcx = ((rbx.wrapping_add(P3)) as u64) as u64; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rdi = (crate::simd_ops::ru64(rbx.wrapping_add(P5))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::wu64(P6, (0x0u64) as u64); rcx = ((rdi.wrapping_add(P7)) as u64) as u64; rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); rcx = ((rdi.wrapping_add(P9)) as u64) as u64; rax = crate::simd_ops::call_stk(P10, rax, rcx, rdx, r8, r9, rsp); rcx = rdi; rax = crate::simd_ops::call_stk(P11, rax, rcx, rdx, r8, r9, rsp); rcx = rdi; rax = crate::simd_ops::call_stk(P12, rax, rcx, rdx, r8, r9, rsp); bb = 3; continue; }
            3 => { rcx = rbx.wrapping_add(0x2e0); rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); rcx = rbx.wrapping_add(0x2c0); rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); rcx = rbx.wrapping_add(0x2b0); rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); rcx = rbx.wrapping_add(0x280); rax = crate::simd_ops::call_stk(P13, rax, rcx, rdx, r8, r9, rsp); rcx = rbx.wrapping_add(0x170); r9 = P14; rdx = 0x10; r8 = ((rdx.wrapping_add(0x1u64)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P15, rax, rcx, rdx, r8, r9, rsp); rcx = rbx.wrapping_add(0x60); r9 = P14; rdx = 0x10; r8 = ((rdx.wrapping_add(0x1u64)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P15, rax, rcx, rdx, r8, r9, rsp); rcx = rbx.wrapping_add(0x50); rax = crate::simd_ops::call_stk(P16, rax, rcx, rdx, r8, r9, rsp); rcx = rbx; rax = crate::simd_ops::call_stk(P12, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); bb = 4; continue; }
            4 => { return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1413034f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x38; const P1: u64 = 0x14397ff18; const P2: u64 = 0x142258080; const P3: u64 = 0x30; const P4: u64 = 0x14206b020; const P5: u64 = 0x142442de0; const P6: u64 = 0x10; const P7: u64 = 0x1412cf770; const P8: u64 = 0xfffffffffffffffc; const P9: u64 = 0x141834ee8; const P10: u64 = 0xfffffffffffffffa;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rdx; rdx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { rcx = (crate::simd_ops::ru64(P1)) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 3; continue; }
            3 => { rdi = (crate::simd_ops::ru64(rbx.wrapping_add(P3))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 4; continue; }
            4 => { rsi = (crate::simd_ops::ru64(rdi)) as u64; fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { rcx = rsi; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rcx = rsi; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); bb = 6; continue; }
            6 => { rcx = rdi; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); bb = 7; continue; }
            7 => { rcx = ((rbx.wrapping_add(P6)) as u64) as u64; r9 = P7; rdx = 0x8; r8 = ((rdx.wrapping_add(P8)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P9, rax, rcx, rdx, r8, r9, rsp); r9 = P7; rdx = 0x8; r8 = ((rdx.wrapping_add(P10)) as u32) as u32 as u64; rcx = rbx; rax = crate::simd_ops::call_stk(P9, rax, rcx, rdx, r8, r9, rsp); rcx = rbx; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); bb = 8; continue; }
            8 => { return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1413035a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14397fed8; const P1: u64 = 0x28; const P2: u64 = 0x1403b8050; const P3: u64 = 0x10; const P4: u64 = 0x1401de830; const P5: u64 = 0x8; const P6: u64 = 0x1412ec900; const P7: u64 = 0x1401e66f0; const P8: u64 = 0x142442de0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    fl_l = rdx; fl_r = rdx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rdx; crate::simd_ops::wu64(P0, (0x0u64) as u64); rcx = ((rdx.wrapping_add(P1)) as u64) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rcx = ((rbx.wrapping_add(P3)) as u64) as u64; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); r8 = (crate::simd_ops::ru64(rbx)) as u64; r8 = (crate::simd_ops::ru64(r8.wrapping_add(P5))) as u64; rdx = rbx; rcx = rbx; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); rdx = (crate::simd_ops::ru64(rbx)) as u64; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); rcx = rbx; rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); }
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141303600(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143a37318; const P1: u64 = 0x14204db80; const P2: u64 = 0x142442de0; const P3: u64 = 0x143a37718; const P4: u64 = 0x14209a1e0; const P5: u64 = 0x143a37528; const P6: u64 = 0x143a373a8; const P7: u64 = 0x143a373f8; const P8: u64 = 0x143a00110; const P9: u64 = 0x1401de830;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 20; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = rdx; rbx = (crate::simd_ops::ru64(P0)) as u64; rsi = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(P0, rsi); fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { rcx = rbx; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rcx = rbx; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 3; continue; }
            3 => { rbx = (crate::simd_ops::ru64(P3)) as u64; crate::simd_ops::wu64(P3, rsi); fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 4; continue; }
            4 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { rdx = (crate::simd_ops::ru32(rbx)) as u32 as u64; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); bb = 6; continue; }
            6 => { rcx = rbx; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 7; continue; }
            7 => { rbx = (crate::simd_ops::ru64(P5)) as u64; crate::simd_ops::wu64(P5, rsi); fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 8; continue; }
            8 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 9; continue; }
            9 => { rdx = (crate::simd_ops::ru32(rbx)) as u32 as u64; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); bb = 10; continue; }
            10 => { rcx = rbx; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 11; continue; }
            11 => { rbx = (crate::simd_ops::ru64(P6)) as u64; crate::simd_ops::wu64(P6, rsi); fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 15; continue; } bb = 12; continue; }
            12 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 14; continue; } bb = 13; continue; }
            13 => { rdx = (crate::simd_ops::ru32(rbx)) as u32 as u64; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); bb = 14; continue; }
            14 => { rcx = rbx; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 15; continue; }
            15 => { rbx = (crate::simd_ops::ru64(P7)) as u64; crate::simd_ops::wu64(P7, rsi); fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 19; continue; } bb = 16; continue; }
            16 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 18; continue; } bb = 17; continue; }
            17 => { rdx = (crate::simd_ops::ru32(rbx)) as u32 as u64; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); bb = 18; continue; }
            18 => { rcx = rbx; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 19; continue; }
            19 => { crate::simd_ops::wu64(P8, rsi); rcx = rdi; rax = crate::simd_ops::call_stk(P9, rax, rcx, rdx, r8, r9, rsp); rcx = rdi; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); bb = 20; continue; }
            20 => { return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141303720(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143a00218; const P1: u64 = 0x14209d670; const P2: u64 = 0x58; const P3: u64 = 0x1411e7fc0; const P4: u64 = 0x142442de0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    fl_l = rdx; fl_r = rdx;
    if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { return rax; }
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rdx;
    crate::simd_ops::wu64(P0, (0x0u64) as u64);
    rdi = (crate::simd_ops::ru64(rdx)) as u64;
    fl_l = rdi; fl_r = rdi;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rcx = rdi; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rcx = ((rdi.wrapping_add(P2)) as u64) as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rcx = rdi; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); }
    rcx = rbx;
    rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp);
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141303780(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143a00220; const P1: u64 = 0x28; const P2: u64 = 0x14397ff18; const P3: u64 = 0x142258080; const P4: u64 = 0x20; const P5: u64 = 0x142442de0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    fl_l = rdx; fl_r = rdx;
    if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { return rax; }
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rdx;
    crate::simd_ops::wu64(P0, (0x0u64) as u64);
    rdx = (crate::simd_ops::ru64(rdx.wrapping_add(P1))) as u64;
    fl_l = rdx; fl_r = rdx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rcx = (crate::simd_ops::ru64(P2)) as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); }
    rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P4))) as u64;
    fl_l = rdx; fl_r = rdx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rcx = (crate::simd_ops::ru64(P2)) as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); }
    rcx = rbx;
    rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1413037e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14397fe98; const P1: u64 = 0x105; const P2: u64 = 0x130; const P3: u64 = 0x14209d670; const P4: u64 = 0x58; const P5: u64 = 0x1411e7fc0; const P6: u64 = 0x142442de0; const P7: u64 = 0x128; const P8: u64 = 0x120; const P9: u64 = 0x118; const P10: u64 = 0x14397ff18; const P11: u64 = 0x142258080;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rdx; crate::simd_ops::wu64(P0, (0x0u64) as u64); crate::simd_ops::wu8(rdx.wrapping_add(P1), (0x0u8) as u8); rdi = (crate::simd_ops::ru64(rdx.wrapping_add(P2))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { rcx = rdi; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rcx = ((rdi.wrapping_add(P4)) as u64) as u64; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rcx = rdi; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); bb = 3; continue; }
            3 => { rdi = (crate::simd_ops::ru64(rbx.wrapping_add(P7))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rcx = rdi; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rcx = ((rdi.wrapping_add(P4)) as u64) as u64; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rcx = rdi; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); bb = 5; continue; }
            5 => { rdi = (crate::simd_ops::ru64(rbx.wrapping_add(P8))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 6; continue; }
            6 => { rcx = rdi; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rcx = ((rdi.wrapping_add(P4)) as u64) as u64; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rcx = rdi; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); bb = 7; continue; }
            7 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P9))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 8; continue; }
            8 => { rcx = (crate::simd_ops::ru64(P10)) as u64; rax = crate::simd_ops::call_stk(P11, rax, rcx, rdx, r8, r9, rsp); bb = 9; continue; }
            9 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x110u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 10; continue; }
            10 => { rcx = (crate::simd_ops::ru64(P10)) as u64; rax = crate::simd_ops::call_stk(P11, rax, rcx, rdx, r8, r9, rsp); bb = 11; continue; }
            11 => { rcx = rbx; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); bb = 12; continue; }
            12 => { return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1413038d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x14243b310; const P2: u64 = 0x14397feb0; const P3: u64 = 0xd8; const P4: u64 = 0x14397ff18; const P5: u64 = 0x142258080; const P6: u64 = 0xc8; const P7: u64 = 0x1411df630; const P8: u64 = 0xfffffffffffffffa; const P9: u64 = 0x141834ee8; const P10: u64 = 0xb8; const P11: u64 = 0x14204f860; const P12: u64 = 0x142442de0; const P13: u64 = 0x140227190; const P14: u64 = 0x14243b540;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rdx; rcx = ((rdx.wrapping_add(P0)) as u64) as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(P2, (0x0u64) as u64); rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P3))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { rcx = (crate::simd_ops::ru64(P4)) as u64; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); bb = 3; continue; }
            3 => { rcx = ((rbx.wrapping_add(P6)) as u64) as u64; r9 = P7; rdx = 0x8; r8 = ((rdx.wrapping_add(P8)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P9, rax, rcx, rdx, r8, r9, rsp); rdi = (crate::simd_ops::ru64(rbx.wrapping_add(P10))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rcx = rdi; rax = crate::simd_ops::call_stk(P11, rax, rcx, rdx, r8, r9, rsp); rcx = rdi; rax = crate::simd_ops::call_stk(P12, rax, rcx, rdx, r8, r9, rsp); bb = 5; continue; }
            5 => { rdi = (crate::simd_ops::ru64(rbx.wrapping_add(0xb0u64))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 6; continue; }
            6 => { rcx = rdi; rax = crate::simd_ops::call_stk(P11, rax, rcx, rdx, r8, r9, rsp); rcx = rdi; rax = crate::simd_ops::call_stk(P12, rax, rcx, rdx, r8, r9, rsp); bb = 7; continue; }
            7 => { rcx = rbx.wrapping_add(0x50); rax = crate::simd_ops::call_stk(P13, rax, rcx, rdx, r8, r9, rsp); rcx = rbx.wrapping_add(0x10); rax = crate::simd_ops::call_stk(P14, rax, rcx, rdx, r8, r9, rsp); rcx = rbx; rax = crate::simd_ops::call_stk(P12, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); bb = 8; continue; }
            8 => { return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1413039a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x28; const P1: u64 = 0x1411e8aa0; const P2: u64 = 0x142442de0; const P3: u64 = 0x20; const P4: u64 = 0x18; const P5: u64 = 0x1411e3240; const P6: u64 = 0x10;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rdx; rdi = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { rcx = rdi; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rcx = rdi; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 3; continue; }
            3 => { rdi = (crate::simd_ops::ru64(rbx.wrapping_add(P3))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rcx = rdi; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rcx = rdi; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 5; continue; }
            5 => { rdi = (crate::simd_ops::ru64(rbx.wrapping_add(P4))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 6; continue; }
            6 => { rcx = rdi; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rcx = rdi; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 7; continue; }
            7 => { rdi = (crate::simd_ops::ru64(rbx.wrapping_add(P6))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 8; continue; }
            8 => { rcx = rdi; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rcx = rdi; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 9; continue; }
            9 => { rcx = rbx; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); bb = 10; continue; }
            10 => { return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141303a40(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143a00210; const P1: u64 = 0x142645790; const P2: u64 = 0x14386e4d8; const P3: u64 = 0x14386e4d0; const P4: u64 = 0x1426455a8; const P5: u64 = 0x14386e4dc; const P6: u64 = 0x143bbde88; const P7: u64 = 0x143bbde80; const P8: u64 = 0xffffffffffffffff; const P9: u64 = 0x142645598; const P10: u64 = 0x141359840; const P11: u64 = 0x142442de0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rdx; rsi = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(P0, rsi); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp); rdi = ((rax as u32)) as u32 as u64; rcx = (crate::simd_ops::ru32(P2)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 3; continue; } bb = 2; continue; }
            2 => { rcx = P3; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu32(P2, ((rdi as u32)) as u32); bb = 3; continue; }
            3 => { r9 = (crate::simd_ops::ru32(P5)) as u32 as u64; r9 = (((r9 as u32)).wrapping_add(1)) as u32 as u64; crate::simd_ops::wu32(P5, ((r9 as u32)) as u32); rax = (crate::simd_ops::ru32(P6)) as u32 as u64; rdx = (crate::simd_ops::ru64(P7)) as u64; r8 = ((rdx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; r8 = ((r8).wrapping_sub(rdx)) as u64; r8 = ((((r8) as i64).wrapping_shr(3u32) as u64)) as u64; rcx = rsi; fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 4; continue; }
            4 => { bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru64(rdx.wrapping_add(rcx.wrapping_mul(8)))) as u64; crate::simd_ops::wu64(rax, rsi); rcx = ((rcx).wrapping_add(1)) as u64; fl_l = rcx; fl_r = r8; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 7; continue; } bb = 6; continue; }
            6 => { rdx = (crate::simd_ops::ru64(P7)) as u64; bb = 5; continue; }
            7 => { r9 = (crate::simd_ops::ru32(P5)) as u32 as u64; bb = 8; continue; }
            8 => { crate::simd_ops::wu32(P6, ((rsi as u32)) as u32); rax = ((r9.wrapping_add(P8)) as u32) as u32 as u64; crate::simd_ops::wu32(P5, ((rax as u32)) as u32); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 10; continue; } bb = 9; continue; }
            9 => { rax = 0xffffffff; let tmp_xchg: u32 = crate::simd_ops::atomic_xchg_u32(P2, ((rax as u32)) as u32); rax = (tmp_xchg) as u32 as u64; rcx = P3; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P9), rax), rax, rcx, rdx, r8, r9, rsp); bb = 10; continue; }
            10 => { rcx = rbx; rax = crate::simd_ops::call_stk(P10, rax, rcx, rdx, r8, r9, rsp); rcx = rbx; rax = crate::simd_ops::call_stk(P11, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); bb = 11; continue; }
            11 => { return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141303b30(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143c49490; const P1: u64 = 0x143c49500; const P2: u64 = 0x14209d670; const P3: u64 = 0x1411e7fc0; const P4: u64 = 0x142442de0; const P5: u64 = 0x142201e30; const P6: u64 = 0x1412ed800; const P7: u64 = 0x141834ee8;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rsi = rdx; rbx = P0; rbp = P1; r14 = 0; fl_l = 0; fl_r = 0; bb = 2; continue; }
            2 => { rdi = (crate::simd_ops::ru64(rbx)) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { rcx = rdi; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rcx = rdi.wrapping_add(0x58); rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rcx = rdi; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); bb = 4; continue; }
            4 => { crate::simd_ops::wu64(rbx, r14); rbx = ((rbx).wrapping_add(0x8u64)) as u64; fl_l = rbx; fl_r = rbp; if ((((fl_l as u64)) as i64) < (((fl_r as u64)) as i64)) { bb = 2; continue; } bb = 5; continue; }
            5 => { rcx = rsi.wrapping_add(0x1298); rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rcx = rsi.wrapping_add(0x1278); rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rcx = rsi.wrapping_add(0x1260); rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rcx = rsi.wrapping_add(0x1248); rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rcx = rsi.wrapping_add(0x1230); rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rcx = rsi.wrapping_add(0x1218); rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rcx = rsi.wrapping_add(0x1200); rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rcx = rsi.wrapping_add(0x11e8); rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rcx = rsi.wrapping_add(0x11d0); rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rcx = rsi.wrapping_add(0x11b0); rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rcx = rsi.wrapping_add(0x1190); rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rcx = rsi.wrapping_add(0x1140); rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rcx = rsi.wrapping_add(0x1128); rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rcx = rsi.wrapping_add(0x1110); rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rcx = rsi.wrapping_add(0x10f8); rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rcx = rsi.wrapping_add(0x10d8); rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rcx = rsi.wrapping_add(0x10b8); rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rcx = rsi.wrapping_add(0x1098); rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rcx = rsi.wrapping_add(0x1078); rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rcx = rsi.wrapping_add(0x1060); rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rcx = rsi.wrapping_add(0x1010); rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); r9 = P6; rdx = 0xd8; r8 = 0x13; rcx = rsi; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); rcx = rsi; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); bb = 6; continue; }
            6 => { return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141303ce0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x28; const P1: u64 = 0x14209d670; const P2: u64 = 0x58; const P3: u64 = 0x1411e7fc0; const P4: u64 = 0x142442de0; const P5: u64 = 0x10; const P6: u64 = 0x1401de830;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    fl_l = rdx; fl_r = rdx;
    if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { return rax; }
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rdx;
    rdi = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    fl_l = rdi; fl_r = rdi;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rcx = rdi; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rcx = ((rdi.wrapping_add(P2)) as u64) as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rcx = rdi; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); }
    rcx = ((rbx.wrapping_add(P5)) as u64) as u64;
    rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp);
    rcx = rbx;
    rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp);
    rcx = rbx;
    rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp);
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141303d40(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14205d410; const P1: u64 = 0x1411df630; const P2: u64 = 0x141834ee8; const P3: u64 = 0x1411d2db0; const P4: u64 = 0x1401de830; const P5: u64 = 0x14209a1e0; const P6: u64 = 0x142442de0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 13; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = rdx; rax = (crate::simd_ops::ru64(rdx)) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), (0x0u32) as u32); rax = (crate::simd_ops::ru64(rcx)) as u64; r8 = 0; fl_l = 0; fl_r = 0; rdx = rsp.wrapping_add(0x30); rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x58u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x8007000e; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 3; continue; } bb = 2; continue; }
            2 => { rcx = ((rax.wrapping_add(0x7785fffbu64)) as u32) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x2; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rdx = ((rax as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); bb = 4; continue; }
            4 => { fl_l = (crate::simd_ops::ru32(rsp.wrapping_add(0x30u64))) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru64(rdi)) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; r8 = 0; fl_l = 0; fl_r = 0; rdx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x50u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 6; continue; }
            6 => { rcx = rdi.wrapping_add(0x20); r9 = P1; rdx = 0x8; r8 = ((rdx.wrapping_sub(0x5u64)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); r9 = P3; rdx = 0x8; r8 = ((rdx.wrapping_sub(0x5u64)) as u32) as u32 as u64; rcx = rdi.wrapping_add(0x8); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(rdi)) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 7; continue; }
            7 => { rcx = rbx.wrapping_add(0x30); rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x20u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 8; continue; }
            8 => { rdx = (crate::simd_ops::ru32(rbx.wrapping_add(0x18u64))) as u32 as u64; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); bb = 9; continue; }
            9 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 10; continue; }
            10 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 11; continue; }
            11 => { crate::simd_ops::wu64(rbx.wrapping_add(0x8u64), (0x0u64) as u64); rcx = rbx; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); bb = 12; continue; }
            12 => { rcx = rdi; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); bb = 13; continue; }
            13 => { return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141303e40(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14136b090; const P1: u64 = 0x143a379b8; const P2: u64 = 0x142645790; const P3: u64 = 0x8; const P4: u64 = 0x1426455a8; const P5: u64 = 0xc; const P6: u64 = 0x20; const P7: u64 = 0x24; const P8: u64 = 0x10; const P9: u64 = 0x1411daad0; const P10: u64 = 0x1c; const P11: u64 = 0x142645598; const P12: u64 = 0x1401de830;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rsi = rcx; rcx = (crate::simd_ops::ru64(rcx)) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 1; continue; }
            1 => { rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rbp = (crate::simd_ops::ru64(rsi)) as u64; rbx = (crate::simd_ops::ru64(P1)) as u64; fl_l = rbp; fl_r = rbp; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 2; continue; }
            2 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); rdi = ((rax as u32)) as u32 as u64; rcx = (crate::simd_ops::ru32(rbx.wrapping_add(P3))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rcx = rbx; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu32(rbx.wrapping_add(P3), ((rdi as u32)) as u32); bb = 4; continue; }
            4 => { crate::simd_ops::lk_add_u32(rbx.wrapping_add(P5), (1) as u32); rdx = (crate::simd_ops::ru32(rbx.wrapping_add(P6))) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rbx.wrapping_add(P7))) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rdx = (((rdx as u32)).wrapping_add(1)) as u32 as u64; rcx = ((rbx.wrapping_add(P8)) as u64) as u64; rax = crate::simd_ops::call_stk(P9, rax, rcx, rdx, r8, r9, rsp); bb = 6; continue; }
            6 => { rcx = (crate::simd_ops::ru32(rbx.wrapping_add(P10))) as u32 as u64; rax = (crate::simd_ops::ru64(rbx.wrapping_add(P8))) as u64; crate::simd_ops::wu64(rax.wrapping_add(rcx.wrapping_mul(8)), rbp); crate::simd_ops::lk_add_u32(rbx.wrapping_add(P10), (1) as u32); rax = (crate::simd_ops::ru32(rbx.wrapping_add(P10))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rbx.wrapping_add(P7))) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 8; continue; } bb = 7; continue; }
            7 => { crate::simd_ops::wu32(rbx.wrapping_add(P10), (0x0u32) as u32); bb = 8; continue; }
            8 => { crate::simd_ops::lk_add_u32(rbx.wrapping_add(P6), (1) as u32); fl_l = (crate::simd_ops::lk_add_u32(rbx.wrapping_add(P5), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; if (((fl_l as u32)) != 0) { bb = 10; continue; } bb = 9; continue; }
            9 => { rax = 0xffffffff; let tmp_xchg: u32 = crate::simd_ops::atomic_xchg_u32(rbx.wrapping_add(P3), ((rax as u32)) as u32); rax = (tmp_xchg) as u32 as u64; rcx = rbx; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P11), rax), rax, rcx, rdx, r8, r9, rsp); bb = 10; continue; }
            10 => { rcx = rsi.wrapping_add(0x18); rax = crate::simd_ops::call_stk(P12, rax, rcx, rdx, r8, r9, rsp); rax = rsi; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141303f00(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x28; const P2: u64 = 0x1401de830; const P3: u64 = 0x14240ab10; const P4: u64 = 0xffffffffffffffff; const P5: u64 = 0x108; const P6: u64 = 0x4; const P7: u64 = 0x100; const P8: u64 = 0xe; const P9: u64 = 0x20; const P10: u64 = 0x18; const P11: u64 = 0x1425371a0; const P12: u64 = 0x142533200;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = rcx; rbx = (crate::simd_ops::ru64(rcx)) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; rsi = (((rax) as i64).wrapping_mul((0x38u64) as i64) as u64) as u64; rsi = ((rsi).wrapping_add(rbx)) as u64; fl_l = rbx; fl_r = rsi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rcx = ((rbx.wrapping_add(P1)) as u64) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rcx = rbx; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rbx = ((rbx).wrapping_add(0x38u64)) as u64; fl_l = rbx; fl_r = rsi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } bb = 2; continue; }
            2 => { r9 = (crate::simd_ops::ru64(rdi)) as u64; fl_l = r9; fl_r = r9; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 3; continue; }
            3 => { rcx = ((r9.wrapping_add(P4)) as u64) as u64; rcx = (((rcx) & (0xfffffffffe000000u64))) as u64; rax = r9; rax = ((rax).wrapping_sub(rcx)) as u64; rax = ((rax).wrapping_shr(16u32)) as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rdx = ((rcx.wrapping_add(P5)) as u64) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(P6))) as u32 as u64; rdx = ((rdx).wrapping_sub(rax)) as u64; rax = ((crate::exe::teb_self() as u64)) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(rcx.wrapping_add(P7))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 8; continue; } bb = 4; continue; }
            4 => { fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(P8))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 7; continue; } bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru64(rdx.wrapping_add(P9))) as u64; crate::simd_ops::wu64(r9, rax); crate::simd_ops::wu64(rdx.wrapping_add(P9), r9); fl_l = (crate::simd_ops::lk_add_u32(rdx.wrapping_add(P10), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; if (((fl_l as u32)) != 0) { bb = 10; continue; } bb = 6; continue; }
            6 => { rcx = rdx; rax = crate::simd_ops::call_stk(P11, rax, rcx, rdx, r8, r9, rsp); bb = 10; continue; }
            7 => { r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 9; continue; }
            8 => { r8 = 0; fl_l = 0; fl_r = 0; bb = 9; continue; }
            9 => { rax = crate::simd_ops::call_stk(P12, rax, rcx, rdx, r8, r9, rsp); bb = 10; continue; }
            10 => { rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rdi, rax); crate::simd_ops::wu64(rdi.wrapping_add(0x8u64), rax); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141303fd0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x1413048d0; const P2: u64 = 0xffffffffffffffff; const P3: u64 = 0x108; const P4: u64 = 0x4; const P5: u64 = 0x100; const P6: u64 = 0xe; const P7: u64 = 0x20; const P8: u64 = 0x18; const P9: u64 = 0x1425371a0; const P10: u64 = 0x142533200;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = (crate::simd_ops::ru64(rcx)) as u64; rdi = rcx; rax = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; rsi = (((rax) as i64).wrapping_mul((0x2e0u64) as i64) as u64) as u64; rsi = ((rsi).wrapping_add(rbx)) as u64; fl_l = rbx; fl_r = rsi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rcx = rbx; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rbx = ((rbx).wrapping_add(0x2e0u64)) as u64; fl_l = rbx; fl_r = rsi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } bb = 2; continue; }
            2 => { r9 = (crate::simd_ops::ru64(rdi)) as u64; fl_l = r9; fl_r = r9; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 3; continue; }
            3 => { rcx = ((r9.wrapping_add(P2)) as u64) as u64; rax = r9; rcx = (((rcx) & (0xfffffffffe000000u64))) as u64; rax = ((rax).wrapping_sub(rcx)) as u64; rax = ((rax).wrapping_shr(16u32)) as u64; rdx = ((rcx.wrapping_add(P3)) as u64) as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(P4))) as u32 as u64; rdx = ((rdx).wrapping_sub(rax)) as u64; rax = ((crate::exe::teb_self() as u64)) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(rcx.wrapping_add(P5))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 8; continue; } bb = 4; continue; }
            4 => { fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(P6))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 7; continue; } bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru64(rdx.wrapping_add(P7))) as u64; crate::simd_ops::wu64(r9, rax); fl_l = (crate::simd_ops::lk_add_u32(rdx.wrapping_add(P8), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; crate::simd_ops::wu64(rdx.wrapping_add(P7), r9); if (((fl_l as u32)) != 0) { bb = 10; continue; } bb = 6; continue; }
            6 => { rcx = rdx; rax = crate::simd_ops::call_stk(P9, rax, rcx, rdx, r8, r9, rsp); bb = 10; continue; }
            7 => { r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 9; continue; }
            8 => { r8 = 0; fl_l = 0; fl_r = 0; bb = 9; continue; }
            9 => { rax = crate::simd_ops::call_stk(P10, rax, rcx, rdx, r8, r9, rsp); bb = 10; continue; }
            10 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = 0; fl_l = 0; fl_r = 0; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; crate::simd_ops::wu64(rdi, rax); crate::simd_ops::wu64(rdi.wrapping_add(P0), rax); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1413040a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14241c0a0; const P1: u64 = 0x14241c190;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(0xcu64))) as u32 as u64; rbx = rcx; rbp = (crate::simd_ops::ru32(rcx.wrapping_add(0x8u64))) as u32 as u64; rdi = ((rax as u32)) as u32 as u64; rdi = ((rdi).wrapping_shr(1u32)) as u64; rdi = ((rdi).wrapping_add(rax)) as u64; fl_l = rdi; fl_r = rdx; rdi = (if (((fl_l as u64)) < ((fl_r as u64))) { rdx } else { rdi }) as u64; rdx = 0x8; rcx = (((rdi) as i64).wrapping_mul((0xf0u64) as i64) as u64) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r8 = (crate::simd_ops::ru64(rbx)) as u64; rsi = rax; rcx = (crate::simd_ops::ru32(rbx.wrapping_add(0x8u64))) as u32 as u64; r9 = (((rcx) as i64).wrapping_mul((0xf0u64) as i64) as u64) as u64; r9 = ((r9).wrapping_add(r8)) as u64; fl_l = r8; fl_r = r9; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 3; continue; } bb = 1; continue; }
            1 => { rcx = rax.wrapping_add(0x98); r10 = 0; fl_l = 0; fl_r = 0; rdx = r8.wrapping_add(0x98); bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru32(r8)) as u32 as u64; r8 = ((r8).wrapping_add(0xf0u64)) as u64; crate::simd_ops::wu32(rcx.wrapping_sub(0x98u64), ((rax as u32)) as u32); rcx = rcx.wrapping_add(0xf0); rax = (crate::simd_ops::ru64(rdx.wrapping_sub(0x90u64))) as u64; rdx = rdx.wrapping_add(0xf0); crate::simd_ops::wu64(rcx.wrapping_sub(0x180u64), rax); rax = (crate::simd_ops::ru64(rdx.wrapping_sub(0x178u64))) as u64; crate::simd_ops::wu64(rcx.wrapping_sub(0x178u64), rax); xmm0 = (crate::simd_ops::ru128(rdx.wrapping_sub(0x170u64))) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rcx.wrapping_sub(0x170u64), (xmm0) as u128); rax = (crate::simd_ops::ru64(rdx.wrapping_sub(0x160u64))) as u64; crate::simd_ops::wu64(rcx.wrapping_sub(0x160u64), rax); rax = (crate::simd_ops::ru64(rdx.wrapping_sub(0x158u64))) as u64; crate::simd_ops::wu64(rcx.wrapping_sub(0x158u64), rax); rax = (crate::simd_ops::ru64(rdx.wrapping_sub(0x150u64))) as u64; crate::simd_ops::wu64(rcx.wrapping_sub(0x150u64), rax); rax = (crate::simd_ops::ru32(rdx.wrapping_sub(0x148u64))) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_sub(0x148u64), ((rax as u32)) as u32); let y_addr: u64 = rdx.wrapping_sub(0x140u64); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rcx.wrapping_sub(0x140u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); xmm1 = ((crate::simd_ops::ru64(rdx.wrapping_sub(0x120u64))) as u64) as u128; crate::simd_ops::wu64(rcx.wrapping_sub(0x120u64), ((xmm1 as u64)) as u64); crate::simd_ops::wu64(rcx.wrapping_sub(0x118u64), r10); crate::simd_ops::wu64(rcx.wrapping_sub(0x110u64), r10); rax = (crate::simd_ops::ru64(rdx.wrapping_sub(0x110u64))) as u64; crate::simd_ops::wu64(rcx.wrapping_sub(0x110u64), rax); rax = (crate::simd_ops::ru64(rdx.wrapping_sub(0x118u64))) as u64; crate::simd_ops::wu64(rcx.wrapping_sub(0x108u64), r10); crate::simd_ops::wu64(rcx.wrapping_sub(0x100u64), r10); crate::simd_ops::wu64(rcx.wrapping_sub(0x118u64), rax); rax = (crate::simd_ops::ru64(rdx.wrapping_sub(0x100u64))) as u64; crate::simd_ops::wu64(rcx.wrapping_sub(0x100u64), rax); rax = (crate::simd_ops::ru64(rdx.wrapping_sub(0x108u64))) as u64; crate::simd_ops::wu64(rcx.wrapping_sub(0xf8u64), r10); crate::simd_ops::wu64(rcx.wrapping_sub(0xf0u64), r10); crate::simd_ops::wu64(rcx.wrapping_sub(0x108u64), rax); rax = (crate::simd_ops::ru64(rdx.wrapping_sub(0xf0u64))) as u64; crate::simd_ops::wu64(rcx.wrapping_sub(0xf0u64), rax); rax = (crate::simd_ops::ru64(rdx.wrapping_sub(0xf8u64))) as u64; crate::simd_ops::wu64(rcx.wrapping_sub(0xe8u64), r10); crate::simd_ops::wu64(rcx.wrapping_sub(0xe0u64), r10); crate::simd_ops::wu64(rcx.wrapping_sub(0xf8u64), rax); rax = (crate::simd_ops::ru64(rdx.wrapping_sub(0xe0u64))) as u64; crate::simd_ops::wu64(rcx.wrapping_sub(0xe0u64), rax); rax = (crate::simd_ops::ru64(rdx.wrapping_sub(0xe8u64))) as u64; crate::simd_ops::wu64(rcx.wrapping_sub(0xd8u64), r10); crate::simd_ops::wu64(rcx.wrapping_sub(0xd0u64), r10); crate::simd_ops::wu64(rcx.wrapping_sub(0xe8u64), rax); rax = (crate::simd_ops::ru64(rdx.wrapping_sub(0xd0u64))) as u64; crate::simd_ops::wu64(rcx.wrapping_sub(0xd0u64), rax); rax = (crate::simd_ops::ru64(rdx.wrapping_sub(0xd8u64))) as u64; crate::simd_ops::wu64(rcx.wrapping_sub(0xc8u64), r10); crate::simd_ops::wu64(rcx.wrapping_sub(0xc0u64), r10); crate::simd_ops::wu64(rcx.wrapping_sub(0xd8u64), rax); rax = (crate::simd_ops::ru64(rdx.wrapping_sub(0xc0u64))) as u64; crate::simd_ops::wu64(rcx.wrapping_sub(0xc0u64), rax); rax = (crate::simd_ops::ru64(rdx.wrapping_sub(0xc8u64))) as u64; crate::simd_ops::wu64(rcx.wrapping_sub(0xc8u64), rax); rax = (crate::simd_ops::ru32(rdx.wrapping_sub(0xb8u64))) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_sub(0xb8u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru64(rdx.wrapping_sub(0xb0u64))) as u64; crate::simd_ops::wu64(rcx.wrapping_sub(0xb0u64), rax); rax = ((crate::simd_ops::ru8(rdx.wrapping_sub(0xa8u64))) as u32) as u32 as u64; crate::simd_ops::wu8(rcx.wrapping_sub(0xa8u64), ((rax as u8)) as u8); rax = ((crate::simd_ops::ru8(rdx.wrapping_sub(0xa7u64))) as u32) as u32 as u64; crate::simd_ops::wu8(rcx.wrapping_sub(0xa7u64), ((rax as u8)) as u8); rax = ((crate::simd_ops::ru8(rdx.wrapping_sub(0xa6u64))) as u32) as u32 as u64; crate::simd_ops::wu8(rcx.wrapping_sub(0xa6u64), ((rax as u8)) as u8); rax = ((crate::simd_ops::ru8(rdx.wrapping_sub(0xa5u64))) as u32) as u32 as u64; crate::simd_ops::wu8(rcx.wrapping_sub(0xa5u64), ((rax as u8)) as u8); rax = ((crate::simd_ops::ru8(rdx.wrapping_sub(0xa4u64))) as u32) as u32 as u64; crate::simd_ops::wu8(rcx.wrapping_sub(0xa4u64), ((rax as u8)) as u8); rax = ((crate::simd_ops::ru8(rdx.wrapping_sub(0xa3u64))) as u32) as u32 as u64; crate::simd_ops::wu8(rcx.wrapping_sub(0xa3u64), ((rax as u8)) as u8); rax = ((crate::simd_ops::ru8(rdx.wrapping_sub(0xa2u64))) as u32) as u32 as u64; crate::simd_ops::wu8(rcx.wrapping_sub(0xa2u64), ((rax as u8)) as u8); rax = (crate::simd_ops::ru32(rdx.wrapping_sub(0xa0u64))) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_sub(0xa0u64), ((rax as u32)) as u32); fl_l = r8; fl_r = r9; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 5; continue; }
            5 => { crate::simd_ops::wu64(rbx, rsi); crate::simd_ops::wu32(rbx.wrapping_add(0x8u64), ((rbp as u32)) as u32); crate::simd_ops::wu32(rbx.wrapping_add(0xcu64), ((rdi as u32)) as u32); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141304370(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0xc; const P2: u64 = 0x1413040a0; const P3: u64 = 0x141304700;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = rdx; rbx = rcx; rax = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; rcx = (((rax) as i64).wrapping_mul((0xf0u64) as i64) as u64) as u64; rax = (crate::simd_ops::ru32(rbx.wrapping_add(P1))) as u32 as u64; r8 = (((rax) as i64).wrapping_mul((0xf0u64) as i64) as u64) as u64; fl_l = rcx; fl_r = r8; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = (0x8888888888888889u64) as u64; let prod = (rax as i64 as i128).wrapping_mul((rcx) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; rdx = ((rdx).wrapping_add(rcx)) as u64; rdx = ((((rdx) as i64).wrapping_shr(7u32) as u64)) as u64; rax = rdx; rax = ((rax).wrapping_shr(63u32)) as u64; rdx = ((rdx).wrapping_add(1)) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; rcx = rbx; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(P0))) as u32 as u64; rcx = (((rax) as i64).wrapping_mul((0xf0u64) as i64) as u64) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rbx))) as u64; rdx = rdi; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::lk_add_u32(rbx.wrapping_add(P0), (1) as u32); rax = (crate::simd_ops::ru32(rbx.wrapping_add(P0))) as u32 as u64; rax = (((rax) as i64).wrapping_mul((0xf0u64) as i64) as u64) as u64; rax = ((rax).wrapping_add(0xffffffffffffff10u64)) as u64; rax = ((rax).wrapping_add(crate::simd_ops::ru64(rbx))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141304400(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x38; const P1: u64 = 0x141834d64; const P2: u64 = 0x1433f42d0; const P3: u64 = 0x8; const P4: u64 = 0x20; const P5: u64 = 0x28; const P6: u64 = 0x40; const P7: u64 = 0x48; const P8: u64 = 0x60; const P9: u64 = 0x68;
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut ymm0_hi: u128 = 0; let mut ymm1_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x141304400, funcinfo_rva: 0x3699820, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0x28, conts: &[] };
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rbx) as u64);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rcx) as u64);
    rsp = rsp.wrapping_sub(8);
    core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = (rdx) as u64;
    rdi = (rcx) as u64;
    crate::simd_ops::wu64(rcx.wrapping_add(P0), (0x0u64) as u64);
    rcx = (0x78u32) as u32 as u64;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    eh.site(0x141304422); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0;
    rcx = ((P2) as u64) as u64;
    crate::simd_ops::wu64(rax, (rcx) as u64);
    let y_addr: u64 = rbx; xmm0 = core::ptr::read_unaligned(y_addr as *const u128); ymm0_hi = core::ptr::read_unaligned(y_addr.wrapping_add(16) as *const u128);
    let y_addr: u64 = rax.wrapping_add(P3); core::ptr::write_unaligned(y_addr as *mut u128, xmm0); core::ptr::write_unaligned(y_addr.wrapping_add(16) as *mut u128, ymm0_hi);
    let y_addr: u64 = rbx.wrapping_add(P4); xmm1 = core::ptr::read_unaligned(y_addr as *const u128); ymm1_hi = core::ptr::read_unaligned(y_addr.wrapping_add(16) as *const u128);
    let y_addr: u64 = rax.wrapping_add(P5); core::ptr::write_unaligned(y_addr as *mut u128, xmm1); core::ptr::write_unaligned(y_addr.wrapping_add(16) as *mut u128, ymm1_hi);
    let y_addr: u64 = rbx.wrapping_add(P6); xmm0 = core::ptr::read_unaligned(y_addr as *const u128); ymm0_hi = core::ptr::read_unaligned(y_addr.wrapping_add(16) as *const u128);
    let y_addr: u64 = rax.wrapping_add(P7); core::ptr::write_unaligned(y_addr as *mut u128, xmm0); core::ptr::write_unaligned(y_addr.wrapping_add(16) as *mut u128, ymm0_hi);
    xmm1 = (crate::simd_ops::ru128(rbx.wrapping_add(P8))) as u128; ymm1_hi = 0;
    crate::simd_ops::wu128(rax.wrapping_add(P9), (xmm1) as u128);
    crate::simd_ops::wu64(rdi.wrapping_add(P0), (rax) as u64);
    rax = (rdi) as u64;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = core::ptr::read_unaligned(rsp as *const u64);
    rsp = rsp.wrapping_add(8);
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141304560(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x38; const P1: u64 = 0x1433f3c80; const P2: u64 = 0x8;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    crate::simd_ops::wu64(rcx.wrapping_add(P0), (0x0u64) as u64);
    rax = P1;
    crate::simd_ops::wu64(rcx, rax);
    rax = rcx;
    xmm0 = (crate::simd_ops::ru128(rdx)) as u128;
    crate::simd_ops::wu128(rcx.wrapping_add(P2), (xmm0) as u128);
    crate::simd_ops::wu64(rcx.wrapping_add(P0), rcx);
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141304680(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x1433f4228; const P1: u64 = 0x38;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = P0;
    crate::simd_ops::wu64(rcx.wrapping_add(P1), rcx);
    crate::simd_ops::wu64(rcx, rax);
    rax = rcx;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141304700(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut ymm0_hi: u128 = 0;
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = (crate::simd_ops::ru32(rdx)) as u32 as u64; r8 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rcx, ((rax as u32)) as u32); rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(rcx.wrapping_add(0x8u64), rax); rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x10u64))) as u64; crate::simd_ops::wu64(rcx.wrapping_add(0x10u64), rax); xmm0 = (crate::simd_ops::ru128(rdx.wrapping_add(0x18u64))) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rcx.wrapping_add(0x18u64), (xmm0) as u128); rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x28u64))) as u64; crate::simd_ops::wu64(rcx.wrapping_add(0x28u64), rax); rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu64(rcx.wrapping_add(0x30u64), rax); rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x38u64))) as u64; crate::simd_ops::wu64(rcx.wrapping_add(0x38u64), rax); rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x40u64))) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(0x40u64), ((rax as u32)) as u32); let y_addr: u64 = rdx.wrapping_add(0x48u64); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rcx.wrapping_add(0x48u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); xmm1 = ((crate::simd_ops::ru64(rdx.wrapping_add(0x68u64))) as u64) as u128; crate::simd_ops::wu64(rcx.wrapping_add(0x68u64), ((xmm1 as u64)) as u64); crate::simd_ops::wu64(rcx.wrapping_add(0x70u64), r8); crate::simd_ops::wu64(rcx.wrapping_add(0x78u64), r8); rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x78u64))) as u64; crate::simd_ops::wu64(rcx.wrapping_add(0x78u64), rax); rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x70u64))) as u64; crate::simd_ops::wu64(rcx.wrapping_add(0x70u64), rax); crate::simd_ops::wu64(rcx.wrapping_add(0x80u64), r8); crate::simd_ops::wu64(rcx.wrapping_add(0x88u64), r8); rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x88u64))) as u64; crate::simd_ops::wu64(rcx.wrapping_add(0x88u64), rax); rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x80u64))) as u64; crate::simd_ops::wu64(rcx.wrapping_add(0x80u64), rax); crate::simd_ops::wu64(rcx.wrapping_add(0x90u64), r8); crate::simd_ops::wu64(rcx.wrapping_add(0x98u64), r8); rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x98u64))) as u64; crate::simd_ops::wu64(rcx.wrapping_add(0x98u64), rax); rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x90u64))) as u64; crate::simd_ops::wu64(rcx.wrapping_add(0x90u64), rax); crate::simd_ops::wu64(rcx.wrapping_add(0xa0u64), r8); crate::simd_ops::wu64(rcx.wrapping_add(0xa8u64), r8); rax = (crate::simd_ops::ru64(rdx.wrapping_add(0xa8u64))) as u64; crate::simd_ops::wu64(rcx.wrapping_add(0xa8u64), rax); rax = (crate::simd_ops::ru64(rdx.wrapping_add(0xa0u64))) as u64; crate::simd_ops::wu64(rcx.wrapping_add(0xa0u64), rax); crate::simd_ops::wu64(rcx.wrapping_add(0xb0u64), r8); crate::simd_ops::wu64(rcx.wrapping_add(0xb8u64), r8); rax = (crate::simd_ops::ru64(rdx.wrapping_add(0xb8u64))) as u64; crate::simd_ops::wu64(rcx.wrapping_add(0xb8u64), rax); rax = (crate::simd_ops::ru64(rdx.wrapping_add(0xb0u64))) as u64; crate::simd_ops::wu64(rcx.wrapping_add(0xb0u64), rax); crate::simd_ops::wu64(rcx.wrapping_add(0xc0u64), r8); crate::simd_ops::wu64(rcx.wrapping_add(0xc8u64), r8); rax = (crate::simd_ops::ru64(rdx.wrapping_add(0xc8u64))) as u64; crate::simd_ops::wu64(rcx.wrapping_add(0xc8u64), rax); rax = (crate::simd_ops::ru64(rdx.wrapping_add(0xc0u64))) as u64; crate::simd_ops::wu64(rcx.wrapping_add(0xc0u64), rax); rax = (crate::simd_ops::ru32(rdx.wrapping_add(0xd0u64))) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(0xd0u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru64(rdx.wrapping_add(0xd8u64))) as u64; crate::simd_ops::wu64(rcx.wrapping_add(0xd8u64), rax); rax = ((crate::simd_ops::ru8(rdx.wrapping_add(0xe0u64))) as u32) as u32 as u64; crate::simd_ops::wu8(rcx.wrapping_add(0xe0u64), ((rax as u8)) as u8); rax = ((crate::simd_ops::ru8(rdx.wrapping_add(0xe1u64))) as u32) as u32 as u64; crate::simd_ops::wu8(rcx.wrapping_add(0xe1u64), ((rax as u8)) as u8); rax = ((crate::simd_ops::ru8(rdx.wrapping_add(0xe2u64))) as u32) as u32 as u64; crate::simd_ops::wu8(rcx.wrapping_add(0xe2u64), ((rax as u8)) as u8); rax = ((crate::simd_ops::ru8(rdx.wrapping_add(0xe3u64))) as u32) as u32 as u64; crate::simd_ops::wu8(rcx.wrapping_add(0xe3u64), ((rax as u8)) as u8); rax = ((crate::simd_ops::ru8(rdx.wrapping_add(0xe4u64))) as u32) as u32 as u64; crate::simd_ops::wu8(rcx.wrapping_add(0xe4u64), ((rax as u8)) as u8); rax = ((crate::simd_ops::ru8(rdx.wrapping_add(0xe5u64))) as u32) as u32 as u64; crate::simd_ops::wu8(rcx.wrapping_add(0xe5u64), ((rax as u8)) as u8); rax = ((crate::simd_ops::ru8(rdx.wrapping_add(0xe6u64))) as u32) as u32 as u64; crate::simd_ops::wu8(rcx.wrapping_add(0xe6u64), ((rax as u8)) as u8); rax = (crate::simd_ops::ru32(rdx.wrapping_add(0xe8u64))) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(0xe8u64), ((rax as u32)) as u32); rax = rcx; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1413048d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x90; const P1: u64 = 0x38; const P2: u64 = 0x20; const P3: u64 = 0x50; const P4: u64 = 0x48;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = ((rcx.wrapping_add(P0)) as u64) as u64; rbx = rcx; rcx = (crate::simd_ops::ru64(rdi.wrapping_add(P1))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(rcx)) as u64; fl_l = rcx; fl_r = rdi; rdx = (rdx & !0xffu64) | (((if (((fl_l as u64)) != ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rdi.wrapping_add(P1), (0x0u64) as u64); bb = 2; continue; }
            2 => { rdi = ((rbx.wrapping_add(P3)) as u64) as u64; rcx = (crate::simd_ops::ru64(rdi.wrapping_add(P1))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rcx)) as u64; fl_l = rcx; fl_r = rdi; rdx = (rdx & !0xffu64) | (((if (((fl_l as u64)) != ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rdi.wrapping_add(P1), (0x0u64) as u64); bb = 4; continue; }
            4 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P4))) as u64; rbx = ((rbx).wrapping_add(0x10u64)) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru64(rcx)) as u64; fl_l = rcx; fl_r = rbx; rdx = (rdx & !0xffu64) | (((if (((fl_l as u64)) != ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx.wrapping_add(P1), (0x0u64) as u64); bb = 6; continue; }
            6 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141304950(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1433f6488; const P1: u64 = 0x14397ff58; const P2: u64 = 0x141304b90; const P3: u64 = 0x142645748; const P4: u64 = 0x1426455b0; const P5: u64 = 0x142442de0; const P6: u64 = 0x1401e3450; const P7: u64 = 0x1401de830; const P8: u64 = 0x1423f0c20; const P9: u64 = 0x141834d30; const P10: u64 = 0x143406c90; const P11: u64 = 0x1423f0780; const P12: u64 = 0x14136eec0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rsi = ((rdx as u32)) as u32 as u64; rdi = rcx; rax = P0; crate::simd_ops::wu64(rcx, rax); rdx = rcx; rcx = (crate::simd_ops::ru64(P1)) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x108u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rdx = 0xffffffff; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 11; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::wu64(P1, (0x0u64) as u64); rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x1a0u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rdi.wrapping_add(0x1a0u64), (0x0u64) as u64); bb = 4; continue; }
            4 => { rbx = (crate::simd_ops::ru64(rdi.wrapping_add(0x168u64))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 5; continue; }
            5 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 6; continue; }
            6 => { rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); bb = 7; continue; }
            7 => { crate::simd_ops::wu64(rbx.wrapping_add(0x8u64), (0x0u64) as u64); rcx = rbx; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); bb = 8; continue; }
            8 => { rcx = rdi.wrapping_add(0x150); rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); rcx = rdi.wrapping_add(0x140); rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); rcx = rdi.wrapping_add(0x130); rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); rcx = rdi.wrapping_add(0x120); rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); rcx = rdi; rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rsi as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 10; continue; } bb = 9; continue; }
            9 => { rdx = 0x1a8; rcx = rdi; rax = crate::simd_ops::call_stk(P9, rax, rcx, rdx, r8, r9, rsp); bb = 10; continue; }
            10 => { rax = rdi; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            11 => { rdx = P10; rcx = rsp.wrapping_add(0x20); rax = crate::simd_ops::call_stk(P11, rax, rcx, rdx, r8, r9, rsp); rcx = rax; rax = crate::simd_ops::call_stk(P12, rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x141304a6b, rcx, rsp); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141304a70(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x142442de0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = (crate::simd_ops::ru64(rcx)) as u64;
    fl_l = rbx; fl_r = rbx;
    if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P0))) as u64;
    fl_l = rcx; fl_r = rcx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); }
    crate::simd_ops::wu64(rbx.wrapping_add(P0), (0x0u64) as u64);
    rcx = rbx;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141304ab0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x48; const P1: u64 = 0x10; const P2: u64 = 0x40; const P3: u64 = 0x142537280; const P4: u64 = 0x1426456c8; const P5: u64 = 0x1426453a0; const P6: u64 = 0x18; const P7: u64 = 0x8; const P8: u64 = 0x20; const P9: u64 = 0x28; const P10: u64 = 0x30; const P11: u64 = 0x38; const P12: u64 = 0x1423082d0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), r8); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rdi = rcx; rsi = 0; fl_l = 0; fl_r = 0; rcx = 0x6348; rax = ((crate::exe::tls_pointer() as u64)) as u64; rdx = (crate::simd_ops::ru64(rax)) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(rdx))) as u64; rdx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rbx = (crate::simd_ops::ru64(rdx.wrapping_add(P1))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { r9 = 0; fl_l = 0; fl_r = 0; r8 = 0; fl_l = 0; fl_r = 0; rdx = ((rsi.wrapping_add(P2)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rbx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 4; continue; } bb = 2; continue; }
            2 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp); rcx = rax; rdx = 0xdeadd00d; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp); rbx = ((rsi as u32)) as u32 as u64; bb = 4; continue; }
            3 => { fl_l = (crate::simd_ops::lk_add_u32(rdx.wrapping_add(P6), (1) as u32)) as u32 as u64; rax = (crate::simd_ops::ru64(rbx)) as u64; crate::simd_ops::wu64(rdx.wrapping_add(P1), rax); bb = 4; continue; }
            4 => { crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rbx); crate::simd_ops::wu32(rbx, ((rsi as u32)) as u32); crate::simd_ops::wu64(rbx.wrapping_add(P7), rsi); crate::simd_ops::wu64(rbx.wrapping_add(P1), rsi); crate::simd_ops::wu64(rbx.wrapping_add(P6), rsi); crate::simd_ops::wu64(rbx.wrapping_add(P8), rsi); crate::simd_ops::wu64(rbx.wrapping_add(P9), rsi); crate::simd_ops::wu64(rbx.wrapping_add(P10), rsi); crate::simd_ops::wu32(rbx.wrapping_add(P11), (0x2u32) as u32); r9 = 0x1000000; rax = crate::simd_ops::call_stk(P12, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx.wrapping_add(0x8u64), rax); rcx = rax.wrapping_add(0x1000000); crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), rcx); let tmp_xchg: u64 = crate::simd_ops::atomic_xchg_u64(rbx.wrapping_add(0x20u64), rax); rax = (tmp_xchg) as u64; rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; let tmp_xchg: u64 = crate::simd_ops::atomic_xchg_u64(rbx.wrapping_add(0x28u64), rax); rax = (tmp_xchg) as u64; crate::simd_ops::wu64(rdi, rbx); rax = rdi; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141304b90(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142407c80; const P1: u64 = 0x143c4aa00; const P2: u64 = 0x1426455a8; const P3: u64 = 0x142645598; const P4: u64 = 0x1422a36b0; const P5: u64 = 0x142645580;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rdx; rbp = rcx; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rax = ((rax as u32)) as u32 as u64; rdx = P1; rdx = (crate::simd_ops::ru64(rdx.wrapping_add(rax.wrapping_mul(8)))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru32(rdx)) as u32 as u64; fl_r = 0x2; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 3; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::wu8(rbx.wrapping_add(0x160u64), (0x1u8) as u8); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            3 => { crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rsi); rsi = (crate::simd_ops::ru64(rbp.wrapping_add(0x168u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rdi); crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), r14); rcx = rsi.wrapping_add(0x18); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); rdx = (crate::simd_ops::ru64(rsi.wrapping_add(0x20u64))) as u64; r14 = (crate::simd_ops::ru64(rsi.wrapping_add(0x28u64))) as u64; r8 = r14.wrapping_add(0x20); fl_l = r8; fl_r = (crate::simd_ops::ru64(rsi.wrapping_add(0x10u64))) as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 11; continue; } bb = 4; continue; }
            4 => { fl_l = r14; fl_r = rdx; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 8; continue; } bb = 5; continue; }
            5 => { fl_l = r8; fl_r = rdx; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 8; continue; } bb = 6; continue; }
            6 => { rcx = (crate::simd_ops::ru32(rsi.wrapping_add(0x30u64))) as u32 as u64; rax = (crate::simd_ops::ru32(rsi.wrapping_add(0x34u64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 8; continue; } bb = 7; continue; }
            7 => { rdx = (crate::simd_ops::ru64(rsi.wrapping_add(0x20u64))) as u64; fl_l = r14; fl_r = rdx; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 5; continue; } bb = 8; continue; }
            8 => { rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x10u64))) as u64; rax = ((rax).wrapping_sub(0x8u64)) as u64; fl_l = r14; fl_r = rax; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 10; continue; } bb = 9; continue; }
            9 => { rax = (0xffffffffffu64) as u64; crate::simd_ops::wu64(r14, rax); bb = 10; continue; }
            10 => { r14 = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; r8 = r14.wrapping_add(0x20); bb = 11; continue; }
            11 => { fl_l = r14; fl_r = rdx; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 16; continue; } bb = 12; continue; }
            12 => { bb = 13; continue; }
            13 => { fl_l = r8; fl_r = rdx; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 16; continue; } bb = 14; continue; }
            14 => { rcx = (crate::simd_ops::ru32(rsi.wrapping_add(0x30u64))) as u32 as u64; rax = (crate::simd_ops::ru32(rsi.wrapping_add(0x34u64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 16; continue; } bb = 15; continue; }
            15 => { rdx = (crate::simd_ops::ru64(rsi.wrapping_add(0x20u64))) as u64; fl_l = r14; fl_r = rdx; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 13; continue; } bb = 16; continue; }
            16 => { rax = (0x69fafafafaffu64) as u64; rcx = rsi.wrapping_add(0x18); crate::simd_ops::wu64(r14, rax); let tmp_xchg: u64 = crate::simd_ops::atomic_xchg_u64(rsi.wrapping_add(0x28u64), r8); r8 = (tmp_xchg) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); rax = P4; crate::simd_ops::wu32(r14.wrapping_add(0x8u64), (0x0u32) as u32); crate::simd_ops::wu64(r14.wrapping_add(0xcu64), rax); crate::simd_ops::wu64(r14.wrapping_add(0x14u64), rbx); rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x168u64))) as u64; crate::simd_ops::wu64(r14, (0x14u64) as u64); crate::simd_ops::lk_add_u32(rax.wrapping_add(0x30u64), (1) as u32); fl_l = (crate::simd_ops::ru8(rbp.wrapping_add(0x198u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 18; continue; } bb = 17; continue; }
            17 => { rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x1a0u64))) as u64; r8 = 0; fl_l = 0; fl_r = 0; rdx = ((r8.wrapping_add(0x1u64)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp); bb = 18; continue; }
            18 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141304f70(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142407c80; const P1: u64 = 0x143c4aa00; const P2: u64 = 0x1426455a8; const P3: u64 = 0x142645598; const P4: u64 = 0x1422a3640; const P5: u64 = 0x142645580;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rdx; rbp = rcx; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rax = ((rax as u32)) as u32 as u64; rdx = P1; rdx = (crate::simd_ops::ru64(rdx.wrapping_add(rax.wrapping_mul(8)))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru32(rdx)) as u32 as u64; fl_r = 0x2; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 3; continue; } bb = 2; continue; }
            2 => { rdx = 0x1; let tmp_xchg: u8 = crate::simd_ops::atomic_xchg_u8(rbx, ((rdx as u8)) as u8); rdx = (rdx & !0xffu64) | (((tmp_xchg) as u8 as u64)); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            3 => { crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rsi); rsi = (crate::simd_ops::ru64(rbp.wrapping_add(0x168u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rdi); crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), r14); rcx = rsi.wrapping_add(0x18); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); rdx = (crate::simd_ops::ru64(rsi.wrapping_add(0x20u64))) as u64; r14 = (crate::simd_ops::ru64(rsi.wrapping_add(0x28u64))) as u64; r8 = r14.wrapping_add(0x20); fl_l = r8; fl_r = (crate::simd_ops::ru64(rsi.wrapping_add(0x10u64))) as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 11; continue; } bb = 4; continue; }
            4 => { fl_l = r14; fl_r = rdx; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 8; continue; } bb = 5; continue; }
            5 => { fl_l = r8; fl_r = rdx; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 8; continue; } bb = 6; continue; }
            6 => { rcx = (crate::simd_ops::ru32(rsi.wrapping_add(0x30u64))) as u32 as u64; rax = (crate::simd_ops::ru32(rsi.wrapping_add(0x34u64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 8; continue; } bb = 7; continue; }
            7 => { rdx = (crate::simd_ops::ru64(rsi.wrapping_add(0x20u64))) as u64; fl_l = r14; fl_r = rdx; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 5; continue; } bb = 8; continue; }
            8 => { rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x10u64))) as u64; rax = ((rax).wrapping_sub(0x8u64)) as u64; fl_l = r14; fl_r = rax; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 10; continue; } bb = 9; continue; }
            9 => { rax = (0xffffffffffu64) as u64; crate::simd_ops::wu64(r14, rax); bb = 10; continue; }
            10 => { r14 = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; r8 = r14.wrapping_add(0x20); bb = 11; continue; }
            11 => { fl_l = r14; fl_r = rdx; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 16; continue; } bb = 12; continue; }
            12 => { bb = 13; continue; }
            13 => { fl_l = r8; fl_r = rdx; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 16; continue; } bb = 14; continue; }
            14 => { rcx = (crate::simd_ops::ru32(rsi.wrapping_add(0x30u64))) as u32 as u64; rax = (crate::simd_ops::ru32(rsi.wrapping_add(0x34u64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 16; continue; } bb = 15; continue; }
            15 => { rdx = (crate::simd_ops::ru64(rsi.wrapping_add(0x20u64))) as u64; fl_l = r14; fl_r = rdx; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 13; continue; } bb = 16; continue; }
            16 => { rax = (0x69fafafafaffu64) as u64; rcx = rsi.wrapping_add(0x18); crate::simd_ops::wu64(r14, rax); let tmp_xchg: u64 = crate::simd_ops::atomic_xchg_u64(rsi.wrapping_add(0x28u64), r8); r8 = (tmp_xchg) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); rax = P4; crate::simd_ops::wu32(r14.wrapping_add(0x8u64), (0x0u32) as u32); crate::simd_ops::wu64(r14.wrapping_add(0xcu64), rax); crate::simd_ops::wu64(r14.wrapping_add(0x14u64), rbx); rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x168u64))) as u64; crate::simd_ops::wu64(r14, (0x14u64) as u64); crate::simd_ops::lk_add_u32(rax.wrapping_add(0x30u64), (1) as u32); fl_l = (crate::simd_ops::ru8(rbp.wrapping_add(0x198u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 18; continue; } bb = 17; continue; }
            17 => { rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x1a0u64))) as u64; r8 = 0; fl_l = 0; fl_r = 0; rdx = ((r8.wrapping_add(0x1u64)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp); bb = 18; continue; }
            18 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1413050d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1412de5d0; const P1: u64 = 0x142442de0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rbx = r9; r15 = rcx; rbp = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rcx, rdx); r9 = ((rbp as u32)) as u32 as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(r8.wrapping_add(0x8u64))) as u64; r8 = rcx; fl_l = r8; fl_r = rax; r8 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; bb = 2; continue; }
            2 => { rcx = (crate::simd_ops::ru32(rax)) as u32 as u64; crate::simd_ops::wu32(rax.wrapping_add(r8).wrapping_add(0xa8u64), ((rcx as u32)) as u32); r9 = ((r9).wrapping_add(1)) as u64; rax = rax.wrapping_add(0x4); rdx = (crate::simd_ops::ru64(r15)) as u64; fl_l = r9; fl_r = rdx; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 2; continue; } bb = 3; continue; }
            3 => { fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 4; continue; }
            4 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x98u64))) as u64; r13 = rsi; r13 = ((r13).wrapping_shr(32u32)) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x90u64))) as u64; r12 = rdi; r12 = ((r12).wrapping_shr(32u32)) as u64; r8 = rbx; fl_l = ((r8).wrapping_shr(32u32)) as u64; r8 = ((fl_l as u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r8); rcx = ((rbx.wrapping_add(rdi)) as u32) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x70u64), ((rcx as u32)) as u32); r14 = r15.wrapping_add(0x10); bb = 5; continue; }
            5 => { crate::simd_ops::wu64(r14.wrapping_sub(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x78u64), rbx); crate::simd_ops::wu32(rsp.wrapping_add(0x78u64), ((rcx as u32)) as u32); crate::simd_ops::lk_add_u32(rsp.wrapping_add(0x7cu64), ((r12 as u32)) as u32); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x78u64), rax); rcx = (((rcx as u32)).wrapping_add((rsi as u32))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x78u64), ((rcx as u32)) as u32); crate::simd_ops::lk_add_u32(rsp.wrapping_add(0x7cu64), ((r13 as u32)) as u32); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; crate::simd_ops::wu64(r14, rax); rax = ((rcx.wrapping_add(0x7u64)) as u32) as u32 as u64; rax = ((((rax as u32)) & (0xfffffff8u32))) as u32 as u64; crate::simd_ops::wu32(r14, ((rax as u32)) as u32); crate::simd_ops::wu64(r14.wrapping_add(0x8u64), rdi); rdx = (crate::simd_ops::ru64(r14)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x78u64), rdx); rcx = ((rdx as u32)) as u32 as u64; rcx = (((rcx as u32)).wrapping_sub((rbx as u32))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x78u64), ((rcx as u32)) as u32); crate::simd_ops::lk_sub_u32(rsp.wrapping_add(0x7cu64), ((r8 as u32)) as u32); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x78u64), rax); rcx = (((rcx as u32)).wrapping_sub((rdi as u32))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x78u64), ((rcx as u32)) as u32); crate::simd_ops::lk_sub_u32(rsp.wrapping_add(0x7cu64), ((r12 as u32)) as u32); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; crate::simd_ops::wu64(r14.wrapping_add(0x10u64), rax); r9 = rdx; r9 = ((r9).wrapping_shr(32u32)) as u64; r9 = ((((r9 as u32)) as i32).wrapping_mul(((rdx as u32)) as i32) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rdx = r14.wrapping_add(0x18); rcx = rsp.wrapping_add(0x28); fl_l = rdx; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 8; continue; } bb = 6; continue; }
            6 => { rcx = (crate::simd_ops::ru64(rdx)) as u64; crate::simd_ops::wu64(rdx, rax); fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 7; continue; }
            7 => { rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 10; continue; }
            8 => { fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 9; continue; }
            9 => { rcx = rax; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 10; continue; }
            10 => { rbp = ((rbp).wrapping_add(1)) as u64; r14 = ((r14).wrapping_add(0x28u64)) as u64; fl_l = rbp; fl_r = (crate::simd_ops::ru64(r15)) as u64; r8 = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; rcx = (crate::simd_ops::ru32(rsp.wrapping_add(0x70u64))) as u32 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 5; continue; } bb = 11; continue; }
            11 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141305250(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142407c80; const P1: u64 = 0x140000000; const P2: u64 = 0x14226c9b0; const P3: u64 = 0x1426455a8; const P4: u64 = 0x142645598; const P5: u64 = 0x1422a5b30; const P6: u64 = 0x1422a5bc0; const P7: u64 = 0x142645580; const P8: u64 = 0x140fbd5e0; const P9: u64 = 0x14330bca8;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x141305250, funcinfo_rva: 0x3699ba8, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0x38, conts: &[] };
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (rbp) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rsi) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rdx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r15) as u64); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbp = (rdx) as u64; r15 = (rcx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x141305273); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rax = ((rax as u32)) as u32 as u64; rcx = ((P1) as u64) as u64; rdx = (crate::simd_ops::ru64(rcx.wrapping_add(rax.wrapping_mul(8)).wrapping_add(0x3c4aa00u64))) as u64; fl_l = (rdx) as u64 as u64; fl_r = (rdx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru32(rdx)) as u32 as u64; fl_r = (0x2u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 3; continue; } bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru32(rbp)) as u32 as u64; r9 = ((rbp.wrapping_add(0x5e0u64)) as u64) as u64; r8 = ((rbp.wrapping_add(0x300u64)) as u64) as u64; rdx = ((rbp.wrapping_add(0x10u64)) as u64) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(rax.wrapping_mul(8)).wrapping_add(0x3bd6048u64))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1413052b0); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 18; continue; }
            3 => { rdi = (crate::simd_ops::ru64(r15.wrapping_add(0x168u64))) as u64; rcx = ((rdi.wrapping_add(0x18u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1413052c5); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P3)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0x20u64))) as u64; rsi = (crate::simd_ops::ru64(rdi.wrapping_add(0x28u64))) as u64; r8 = ((rsi.wrapping_add(0x618u64)) as u64) as u64; fl_l = (r8) as u64 as u64; fl_r = (crate::simd_ops::ru64(rdi.wrapping_add(0x10u64))) as u64 as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 11; continue; } bb = 4; continue; }
            4 => { fl_l = (rsi) as u64 as u64; fl_r = (rdx) as u64 as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 8; continue; } bb = 5; continue; }
            5 => { fl_l = (r8) as u64 as u64; fl_r = (rdx) as u64 as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 8; continue; } bb = 6; continue; }
            6 => { rcx = (crate::simd_ops::ru32(rdi.wrapping_add(0x30u64))) as u32 as u64; rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x34u64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 8; continue; } bb = 7; continue; }
            7 => { rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0x20u64))) as u64; fl_l = (rsi) as u64 as u64; fl_r = (rdx) as u64 as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 5; continue; } bb = 8; continue; }
            8 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x10u64))) as u64; rax = ((rax).wrapping_sub(0x8u64)) as u64; fl_l = (rsi) as u64 as u64; fl_r = (rax) as u64 as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 10; continue; } bb = 9; continue; }
            9 => { rax = (0xffffffffffu64) as u64; crate::simd_ops::wu64(rsi, (rax) as u64); bb = 10; continue; }
            10 => { rsi = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; r8 = ((rsi.wrapping_add(0x618u64)) as u64) as u64; bb = 11; continue; }
            11 => { fl_l = (rsi) as u64 as u64; fl_r = (rdx) as u64 as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 16; continue; } bb = 12; continue; }
            12 => { bb = 13; continue; }
            13 => { fl_l = (r8) as u64 as u64; fl_r = (rdx) as u64 as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 16; continue; } bb = 14; continue; }
            14 => { rcx = (crate::simd_ops::ru32(rdi.wrapping_add(0x30u64))) as u32 as u64; rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x34u64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 16; continue; } bb = 15; continue; }
            15 => { rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0x20u64))) as u64; fl_l = (rsi) as u64 as u64; fl_r = (rdx) as u64 as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 13; continue; } bb = 16; continue; }
            16 => { rax = (0x69fafafafaffu64) as u64; crate::simd_ops::wu64(rsi, (rax) as u64); let tmp_xchg: u64 = crate::simd_ops::atomic_xchg_u64(rdi.wrapping_add(0x28u64), (r8) as u64); r8 = (tmp_xchg) as u64; rcx = ((rdi.wrapping_add(0x18u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x141305360); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P4)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); crate::simd_ops::wu32(rsi.wrapping_add(0x8u64), (0x0u32) as u32); rax = ((P5) as u64) as u64; crate::simd_ops::wu64(rsi.wrapping_add(0xcu64), (rax) as u64); rcx = ((rsi.wrapping_add(0x14u64)) as u64) as u64; rdx = (rbp) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14130537f); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); r10 = (crate::simd_ops::ru64(r15.wrapping_add(0x168u64))) as u64; crate::simd_ops::wu64(rsi, (0x60cu64) as u64); crate::simd_ops::lk_add_u32(r10.wrapping_add(0x30u64), (1) as u32); fl_l = (crate::simd_ops::ru8(r15.wrapping_add(0x198u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 18; continue; } bb = 17; continue; }
            17 => { r8 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rdx = ((r8.wrapping_add(0x1u64)) as u32) as u32 as u64; rcx = (crate::simd_ops::ru64(r15.wrapping_add(0x1a0u64))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1413053af); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P7)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 18; continue; }
            18 => { rcx = ((rbp.wrapping_add(0x5e0u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1413053bd); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P8) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rax = ((P9) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x300u64), (rax) as u64); crate::simd_ops::wu64(rbp.wrapping_add(0x20u64), (rax) as u64); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1413053f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14397ff58; const P1: u64 = 0x1426455a8; const P2: u64 = 0x142645598; const P3: u64 = 0x14240aa00; const P4: u64 = 0x142645580;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rdi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; r14 = rdx; r15 = rcx; rdi = (crate::simd_ops::ru64(P0)) as u64; rdi = (crate::simd_ops::ru64(rdi.wrapping_add(0x168u64))) as u64; rcx = rdi.wrapping_add(0x18); (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r8 = (crate::simd_ops::ru64(rdi.wrapping_add(0x20u64))) as u64; rsi = (crate::simd_ops::ru64(rdi.wrapping_add(0x28u64))) as u64; rdx = rsi.wrapping_add(0x98); fl_l = rdx; fl_r = (crate::simd_ops::ru64(rdi.wrapping_add(0x10u64))) as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 8; continue; } bb = 1; continue; }
            1 => { fl_l = rsi; fl_r = r8; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 5; continue; } bb = 2; continue; }
            2 => { fl_l = rdx; fl_r = r8; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 5; continue; } bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru32(rdi.wrapping_add(0x30u64))) as u32 as u64; rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x34u64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 5; continue; } bb = 4; continue; }
            4 => { r8 = (crate::simd_ops::ru64(rdi.wrapping_add(0x20u64))) as u64; fl_l = rsi; fl_r = r8; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 2; continue; } bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x10u64))) as u64; rax = ((rax).wrapping_sub(0x8u64)) as u64; fl_l = rsi; fl_r = rax; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 7; continue; } bb = 6; continue; }
            6 => { rax = (0xffffffffffu64) as u64; crate::simd_ops::wu64(rsi, rax); bb = 7; continue; }
            7 => { rsi = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; rdx = rsi.wrapping_add(0x98); bb = 8; continue; }
            8 => { fl_l = rsi; fl_r = r8; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 13; continue; } bb = 9; continue; }
            9 => { bb = 10; continue; }
            10 => { fl_l = rdx; fl_r = r8; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 13; continue; } bb = 11; continue; }
            11 => { rcx = (crate::simd_ops::ru32(rdi.wrapping_add(0x30u64))) as u32 as u64; rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x34u64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 13; continue; } bb = 12; continue; }
            12 => { r8 = (crate::simd_ops::ru64(rdi.wrapping_add(0x20u64))) as u64; fl_l = rsi; fl_r = r8; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 10; continue; } bb = 13; continue; }
            13 => { rax = (0x69fafafafaffu64) as u64; crate::simd_ops::wu64(rsi, rax); rsi = ((rsi).wrapping_add(0x8u64)) as u64; let tmp_xchg: u64 = crate::simd_ops::atomic_xchg_u64(rdi.wrapping_add(0x28u64), rdx); rdx = (tmp_xchg) as u64; rcx = rdi.wrapping_add(0x18); (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu32(rsi, (0x284u32) as u32); rcx = rsi.wrapping_add(0x4); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rcx); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rcx); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), (0x8cu64) as u64); rax = (crate::simd_ops::ru32(r15)) as u32 as u64; crate::simd_ops::wu32(rcx, ((rax as u32)) as u32); rax = (crate::simd_ops::ru64(r14)) as u64; crate::simd_ops::wu64(rcx.wrapping_add(0x8u64), rax); rax = (crate::simd_ops::ru32(r14.wrapping_add(0x8u64))) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(0x10u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(r14.wrapping_add(0xcu64))) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(0x14u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(r14.wrapping_add(0x10u64))) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(0x18u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(r14.wrapping_add(0x14u64))) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(0x1cu64), ((rax as u32)) as u32); rax = ((crate::simd_ops::ru8(r14.wrapping_add(0x18u64))) as u32) as u32 as u64; crate::simd_ops::wu8(rcx.wrapping_add(0x20u64), ((rax as u8)) as u8); rax = ((crate::simd_ops::ru8(r14.wrapping_add(0x19u64))) as u32) as u32 as u64; crate::simd_ops::wu8(rcx.wrapping_add(0x21u64), ((rax as u8)) as u8); rax = ((crate::simd_ops::ru8(r14.wrapping_add(0x1au64))) as u32) as u32 as u64; crate::simd_ops::wu8(rcx.wrapping_add(0x22u64), ((rax as u8)) as u8); rax = ((crate::simd_ops::ru8(r14.wrapping_add(0x1bu64))) as u32) as u32 as u64; crate::simd_ops::wu8(rcx.wrapping_add(0x23u64), ((rax as u8)) as u8); rax = ((crate::simd_ops::ru8(r14.wrapping_add(0x1cu64))) as u32) as u32 as u64; crate::simd_ops::wu8(rcx.wrapping_add(0x24u64), ((rax as u8)) as u8); rax = ((crate::simd_ops::ru8(r14.wrapping_add(0x1du64))) as u32) as u32 as u64; crate::simd_ops::wu8(rcx.wrapping_add(0x25u64), ((rax as u8)) as u8); rax = ((crate::simd_ops::ru8(r14.wrapping_add(0x1eu64))) as u32) as u32 as u64; crate::simd_ops::wu8(rcx.wrapping_add(0x26u64), ((rax as u8)) as u8); rax = ((crate::simd_ops::ru8(r14.wrapping_add(0x1fu64))) as u32) as u32 as u64; crate::simd_ops::wu8(rcx.wrapping_add(0x27u64), ((rax as u8)) as u8); rax = ((crate::simd_ops::ru8(r14.wrapping_add(0x20u64))) as u32) as u32 as u64; crate::simd_ops::wu8(rcx.wrapping_add(0x28u64), ((rax as u8)) as u8); rax = ((crate::simd_ops::ru8(r14.wrapping_add(0x21u64))) as u32) as u32 as u64; crate::simd_ops::wu8(rcx.wrapping_add(0x29u64), ((rax as u8)) as u8); rax = ((crate::simd_ops::ru8(r14.wrapping_add(0x22u64))) as u32) as u32 as u64; crate::simd_ops::wu8(rcx.wrapping_add(0x2au64), ((rax as u8)) as u8); rax = (crate::simd_ops::ru32(r14.wrapping_add(0x24u64))) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(0x2cu64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(r14.wrapping_add(0x28u64))) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(0x30u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(r14.wrapping_add(0x2cu64))) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(0x34u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(r14.wrapping_add(0x30u64))) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(0x38u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(r14.wrapping_add(0x34u64))) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(0x3cu64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(r14.wrapping_add(0x38u64))) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(0x40u64), ((rax as u32)) as u32); rax = ((crate::simd_ops::ru8(r14.wrapping_add(0x3cu64))) as u32) as u32 as u64; crate::simd_ops::wu8(rcx.wrapping_add(0x44u64), ((rax as u8)) as u8); rax = (crate::simd_ops::ru32(r14.wrapping_add(0x40u64))) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(0x48u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(r14.wrapping_add(0x44u64))) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(0x4cu64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(r14.wrapping_add(0x48u64))) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(0x50u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(r14.wrapping_add(0x4cu64))) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(0x54u64), ((rax as u32)) as u32); rax = ((crate::simd_ops::ru8(r14.wrapping_add(0x50u64))) as u32) as u32 as u64; crate::simd_ops::wu8(rcx.wrapping_add(0x58u64), ((rax as u8)) as u8); rax = (crate::simd_ops::ru64(r14.wrapping_add(0x58u64))) as u64; crate::simd_ops::wu64(rcx.wrapping_add(0x60u64), rax); rax = ((crate::simd_ops::ru8(r14.wrapping_add(0x60u64))) as u32) as u32 as u64; crate::simd_ops::wu8(rcx.wrapping_add(0x68u64), ((rax as u8)) as u8); rax = ((crate::simd_ops::ru8(r14.wrapping_add(0x61u64))) as u32) as u32 as u64; crate::simd_ops::wu8(rcx.wrapping_add(0x69u64), ((rax as u8)) as u8); rax = ((crate::simd_ops::ru8(r14.wrapping_add(0x62u64))) as u32) as u32 as u64; crate::simd_ops::wu8(rcx.wrapping_add(0x6au64), ((rax as u8)) as u8); rcx = ((rcx).wrapping_add(0x6cu64)) as u64; crate::simd_ops::lk_and_u32(rcx, (0xff000002u32) as u32); crate::simd_ops::lk_or_u32(rcx, (0x2u32) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x3u64), (0x2u8) as u8); crate::simd_ops::wu32(rcx.wrapping_add(0x4u64), (0x0u32) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x8u64), (0x0u8) as u8); rdx = r14.wrapping_add(0x64); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru64(P0)) as u64; let y_addr: u64 = rsp.wrapping_add(0x20u64); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); rax = ((xmm0 as u64)) as u64; crate::simd_ops::lk_and_u32(rax, (0x1ffu32) as u32); crate::simd_ops::lk_or_u32(rax, (0x200u32) as u32); rdx = (crate::simd_ops::ru64(rcx.wrapping_add(0x168u64))) as u64; xmm0 = (ymm0_hi) as u128; ymm0_hi = 0; crate::simd_ops::wu64(rsi.wrapping_sub(0x8u64), (((xmm0 >> 64) as u64)) as u64); crate::simd_ops::lk_add_u32(rdx.wrapping_add(0x30u64), (1) as u32); fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0x198u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 15; continue; } bb = 14; continue; }
            14 => { r8 = 0; fl_l = 0; fl_r = 0; rdx = ((r8.wrapping_add(0x1u64)) as u32) as u32 as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(0x1a0u64))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 15; continue; }
            15 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141305680(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14397ff58; const P1: u64 = 0x1426455a8; const P2: u64 = 0x142645598; const P3: u64 = 0x142645580;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rax = (crate::simd_ops::ru64(P0)) as u64; r14 = rcx; rdi = (crate::simd_ops::ru64(rax.wrapping_add(0x168u64))) as u64; rcx = rdi.wrapping_add(0x18); (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x20u64))) as u64; rsi = (crate::simd_ops::ru64(rdi.wrapping_add(0x28u64))) as u64; r8 = rsi.wrapping_add(0x10); fl_l = r8; fl_r = (crate::simd_ops::ru64(rdi.wrapping_add(0x10u64))) as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 9; continue; } bb = 1; continue; }
            1 => { fl_l = rsi; fl_r = rax; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 6; continue; } bb = 2; continue; }
            2 => { bb = 3; continue; }
            3 => { fl_l = r8; fl_r = rax; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 6; continue; } bb = 4; continue; }
            4 => { rdx = (crate::simd_ops::ru32(rdi.wrapping_add(0x30u64))) as u32 as u64; rcx = (crate::simd_ops::ru32(rdi.wrapping_add(0x34u64))) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x20u64))) as u64; fl_l = rsi; fl_r = rax; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 3; continue; } bb = 6; continue; }
            6 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x10u64))) as u64; rcx = ((rcx).wrapping_sub(0x8u64)) as u64; fl_l = rsi; fl_r = rcx; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 8; continue; } bb = 7; continue; }
            7 => { rcx = (0xffffffffffu64) as u64; crate::simd_ops::wu64(rsi, rcx); bb = 8; continue; }
            8 => { rsi = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; r8 = rsi.wrapping_add(0x10); bb = 9; continue; }
            9 => { fl_l = rsi; fl_r = rax; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 13; continue; } bb = 10; continue; }
            10 => { fl_l = r8; fl_r = rax; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 13; continue; } bb = 11; continue; }
            11 => { rcx = (crate::simd_ops::ru32(rdi.wrapping_add(0x30u64))) as u32 as u64; rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x34u64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 13; continue; } bb = 12; continue; }
            12 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x20u64))) as u64; fl_l = rsi; fl_r = rax; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 10; continue; } bb = 13; continue; }
            13 => { rax = (0x69fafafafaffu64) as u64; rcx = rdi.wrapping_add(0x18); crate::simd_ops::wu64(rsi, rax); let tmp_xchg: u64 = crate::simd_ops::atomic_xchg_u64(rdi.wrapping_add(0x28u64), r8); r8 = (tmp_xchg) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu32(rsi.wrapping_add(0x8u64), (0x285u32) as u32); r8 = rsi.wrapping_add(0x8); rax = (crate::simd_ops::ru32(r14)) as u32 as u64; rcx = r8.wrapping_add(0x4); crate::simd_ops::wu32(rcx, ((rax as u32)) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rcx); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rcx); rcx = (crate::simd_ops::ru64(P0)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r8); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), (0x8u64) as u64); let y_addr: u64 = rsp.wrapping_add(0x20u64); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); rax = ((xmm0 as u64)) as u64; xmm0 = (ymm0_hi) as u128; ymm0_hi = 0; crate::simd_ops::lk_and_u32(rax, (0x1ffu32) as u32); crate::simd_ops::lk_or_u32(rax, (0x200u32) as u32); rdx = (crate::simd_ops::ru64(rcx.wrapping_add(0x168u64))) as u64; crate::simd_ops::wu64(r8.wrapping_sub(0x8u64), (((xmm0 >> 64) as u64)) as u64); crate::simd_ops::lk_add_u32(rdx.wrapping_add(0x30u64), (1) as u32); fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0x198u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 15; continue; } bb = 14; continue; }
            14 => { rcx = (crate::simd_ops::ru64(rcx.wrapping_add(0x1a0u64))) as u64; r8 = 0; fl_l = 0; fl_r = 0; rdx = ((r8.wrapping_add(0x1u64)) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 15; continue; }
            15 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1413057f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x20; const P1: u64 = 0xfffffffffffffff8; const P2: u64 = 0xc; const P3: u64 = 0xffffffffffffffe0; const P4: u64 = 0x4; const P5: u64 = 0xfffffffffffffff0; const P6: u64 = 0xffffffffffffffec; const P7: u64 = 0xfffffffffffffff4; const P8: u64 = 0xfffffffffffffffc; const P9: u64 = 0x30; const P10: u64 = 0x34; const P11: u64 = 0x33;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 0>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rbx = rcx;
    r10 = ((rcx.wrapping_add(P0)) as u64) as u64;
    fl_l = ((rdx).wrapping_add(0x8u64)) as u64; fl_r = fl_l;
    rdx = ((fl_l as u64)) as u64;
    r11 = 0x4;
    loop { rax = (crate::simd_ops::ru32(rdx.wrapping_add(P1))) as u32 as u64; rdx = ((rdx.wrapping_add(P2)) as u64) as u64; crate::simd_ops::wu32(r10.wrapping_add(P3), ((rax as u32)) as u32); r10 = ((r10.wrapping_add(P4)) as u64) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(P5))) as u32 as u64; crate::simd_ops::wu32(r10.wrapping_add(P6), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdx.wrapping_add(P7))) as u32 as u64; crate::simd_ops::wu32(r10.wrapping_add(P8), ((rax as u32)) as u32); fl_l = r11; fl_r = (0x1u64) as u64; r11 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if !((((fl_l as u64)) != ((fl_r as u64)))) { break; } }
    rcx = (crate::simd_ops::ru32(rcx.wrapping_add(P9))) as u32 as u64;
    rax = ((crate::simd_ops::ru8(rsp.wrapping_add(0x30u64))) as u32) as u32 as u64;
    rcx = ((((rcx as u32)) ^ ((r8 as u32)))) as u32 as u64;
    rcx = ((((rcx as u32)) & (0xffffffu32))) as u32 as u64;
    crate::simd_ops::lk_xor_u32(rbx.wrapping_add(P9), ((rcx as u32)) as u32);
    rcx = (crate::simd_ops::ru32(rbx.wrapping_add(P10))) as u32 as u64;
    rcx = ((((rcx as u32)) ^ (crate::simd_ops::ru32(rsp.wrapping_add(0x28u64))))) as u32 as u64;
    rcx = ((((rcx as u32)) & (0xffffffu32))) as u32 as u64;
    crate::simd_ops::wu8(rbx.wrapping_add(P11), ((r9 as u8)) as u8);
    fl_l = (crate::simd_ops::lk_xor_u32(rbx.wrapping_add(0x34u64), ((rcx as u32)) as u32)) as u32 as u64; fl_r = fl_l;
    crate::simd_ops::wu8(rbx.wrapping_add(0x37u64), ((rax as u8)) as u8);
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64;
    crate::simd_ops::wu64(rbx.wrapping_add(0x38u64), rax);
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x8u64))) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141305a20(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1411e8620;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rdx; rdi = rcx; rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x8u64))) as u32 as u64; r8 = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; r8 = ((r8.wrapping_mul(8)) as u64) as u64; rax = (0x6666666666666667u64) as u64; let prod = (rax as i64 as i128).wrapping_mul((r8) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; rsi = rdx; rsi = ((((rsi) as i64).wrapping_shr(4u32) as u64)) as u64; rax = rsi; rax = ((rax).wrapping_shr(63u32)) as u64; rsi = ((rsi).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(0xcu64))) as u32 as u64; fl_l = rbx; fl_r = rax; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 5; continue; } bb = 1; continue; }
            1 => { rdx = ((rax as u32)) as u32 as u64; rdx = ((rdx).wrapping_shr(1u32)) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; fl_l = rdx; fl_r = rbx; rdx = (if (((fl_l as u64)) < ((fl_r as u64))) { rbx } else { rdx }) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r9 = rbx; fl_l = r9; fl_r = rsi; r9 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 9; continue; } bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x8u64))) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = (crate::simd_ops::ru64(rdi)) as u64; rdx = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; rax = rdx.wrapping_add(0x18); r8 = 0; fl_l = 0; fl_r = 0; bb = 3; continue; }
            3 => { xmm0 = 0u128; ymm0_hi = 0; rcx = 0; fl_l = 0; fl_r = 0; let y_addr: u64 = rdx; crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); crate::simd_ops::wu64(rdx.wrapping_add(0x20u64), rcx); crate::simd_ops::wu32(rdx, (0xffffffffu32) as u32); crate::simd_ops::wu32(rax.wrapping_sub(0x14u64), (0xffffffffu32) as u32); crate::simd_ops::wu64(rax.wrapping_sub(0x10u64), r8); crate::simd_ops::wu64(rax.wrapping_sub(0x8u64), (0xffffffffffffffffu64) as u64); crate::simd_ops::wu64(rax, (0xffffffffffffffffu64) as u64); crate::simd_ops::wu32(rax.wrapping_add(0x8u64), (0xffffffffu32) as u32); rdx = rdx.wrapping_add(0x28); rax = rax.wrapping_add(0x28); fl_l = r9; fl_r = (0x1u64) as u64; r9 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } bb = 4; continue; }
            4 => { crate::simd_ops::wu32(rdi.wrapping_add(0x8u64), ((rbx as u32)) as u32); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            5 => { fl_l = rbx; fl_r = rsi; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 10; continue; } bb = 6; continue; }
            6 => { rdx = rbx; fl_l = rdx; fl_r = rsi; rdx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 9; continue; } bb = 7; continue; }
            7 => { rcx = (crate::simd_ops::ru64(rcx)) as u64; rcx = ((rcx).wrapping_add(r8)) as u64; rax = rcx.wrapping_add(0x18); r8 = 0; fl_l = 0; fl_r = 0; bb = 8; continue; }
            8 => { xmm0 = 0u128; ymm0_hi = 0; r9 = 0; fl_l = 0; fl_r = 0; let y_addr: u64 = rcx; crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); crate::simd_ops::wu64(rcx.wrapping_add(0x20u64), r9); crate::simd_ops::wu32(rcx, (0xffffffffu32) as u32); crate::simd_ops::wu32(rax.wrapping_sub(0x14u64), (0xffffffffu32) as u32); crate::simd_ops::wu64(rax.wrapping_sub(0x10u64), r8); crate::simd_ops::wu64(rax.wrapping_sub(0x8u64), (0xffffffffffffffffu64) as u64); crate::simd_ops::wu64(rax, (0xffffffffffffffffu64) as u64); crate::simd_ops::wu32(rax.wrapping_add(0x8u64), (0xffffffffu32) as u32); rcx = rcx.wrapping_add(0x28); rax = rax.wrapping_add(0x28); fl_l = rdx; fl_r = (0x1u64) as u64; rdx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 8; continue; } bb = 9; continue; }
            9 => { crate::simd_ops::wu32(rdi.wrapping_add(0x8u64), ((rbx as u32)) as u32); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            10 => { if (((fl_l as u64)) == ((fl_r as u64))) { bb = 12; continue; } bb = 11; continue; }
            11 => { crate::simd_ops::wu32(rcx.wrapping_add(0x8u64), ((rbx as u32)) as u32); bb = 12; continue; }
            12 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141305bb0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1413083f0; const P1: u64 = 0x1412ec110; const P2: u64 = 0x1424069a0; const P3: u64 = 0x143868298; const P4: u64 = 0x143426a48; const P5: u64 = 0x14209d670; const P6: u64 = 0x142075af0; const P7: u64 = 0x14209c560; const P8: u64 = 0x14183acbc; const P9: u64 = 0x14209d6f0; const P10: u64 = 0x1422ab7b0;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0; let mut ymm1_hi: u128 = 0; let mut xmm6: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 19>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x70u64)) as u64; crate::simd_ops::wu128(rsp.wrapping_add(0x60u64), (xmm6) as u128); rbx = rcx; r14 = (crate::simd_ops::ru64(rcx.wrapping_add(0xa8u64))) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(0xb8u64))) as u64; rcx = ((rcx).wrapping_add(0x3fu64)) as u64; rcx = (((rcx) & (0xffffffffffffffc0u64))) as u64; rcx = ((rcx).wrapping_shr(6u32)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xa8u64), rcx); r15 = 0; fl_l = 0; fl_r = 0; rax = ((r15 as u32)) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xa0u64), rax); fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 1; continue; }
            1 => { bb = 2; continue; }
            2 => { rcx = (crate::simd_ops::ru64(r14)) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 3; continue; }
            3 => { r13 = ((rax as u32)) as u32 as u64; fl_l = (((r13 as u32)).wrapping_shl(6u32)) as u32 as u64; r13 = ((fl_l as u32)) as u32 as u64; bb = 4; continue; }
            4 => { let bit_in: u64 = rcx; rsi = ((bit_in.trailing_zeros() as u64)) as u64; fl_l = (rsi as u64) as u64; r12 = ((rsi.wrapping_add(r13)) as u32) as u32 as u64; r15 = ((r12 as u32)) as u32 as u64; fl_l = r15; fl_r = (crate::simd_ops::ru64(rbx.wrapping_add(0x90u64))) as u64; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 8; continue; } bb = 5; continue; }
            5 => { rdi = rbx.wrapping_add(0xc0); rdx = (crate::simd_ops::ru32(rdi.wrapping_add(0x8u64))) as u32 as u64; r8 = (crate::simd_ops::ru32(rdi.wrapping_add(0xcu64))) as u32 as u64; rcx = ((r8.wrapping_add(r8.wrapping_mul(8))) as u64) as u64; rcx = ((rcx).wrapping_shl(3u32)) as u64; rax = ((rdx.wrapping_add(rdx.wrapping_mul(8))) as u64) as u64; rax = ((rax).wrapping_shl(3u32)) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 7; continue; } bb = 6; continue; }
            6 => { rax = rdx.wrapping_add(0x1); rdx = ((r8 as u32)) as u32 as u64; rdx = ((rdx).wrapping_shr(1u32)) as u64; rdx = ((rdx).wrapping_add(r8)) as u64; fl_l = rdx; fl_r = rax; rdx = (if (((fl_l as u64)) < ((fl_r as u64))) { rax } else { rdx }) as u64; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 7; continue; }
            7 => { rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x8u64))) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rax = (crate::simd_ops::ru64(rdi)) as u64; xmm0 = 0u128; ymm0_hi = 0; rdx = 0; fl_l = 0; fl_r = 0; let y_addr: u64 = rax.wrapping_add(rcx.wrapping_mul(8)); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); let y_addr: u64 = rax.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x20u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); crate::simd_ops::wu64(rax.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x40u64), rdx); crate::simd_ops::lk_add_u32(rdi.wrapping_add(0x8u64), (1) as u32); rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x8u64))) as u32 as u64; rdx = ((rax.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rcx = (crate::simd_ops::ru64(rdi)) as u64; crate::simd_ops::wu32(rcx.wrapping_add(rdx.wrapping_mul(8)).wrapping_sub(0x48u64), ((r12 as u32)) as u32); fl_l = ((r15).wrapping_shl(6u32)) as u64; r15 = ((fl_l as u64)) as u64; rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x98u64))) as u64; let y_addr: u64 = r15.wrapping_add(rax); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rcx.wrapping_add(rdx.wrapping_mul(8)).wrapping_sub(0x40u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); let y_addr: u64 = r15.wrapping_add(rax).wrapping_add(0x20u64); xmm1 = crate::simd_ops::ru128(y_addr); ymm1_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rcx.wrapping_add(rdx.wrapping_mul(8)).wrapping_sub(0x20u64); crate::simd_ops::wu128(y_addr, xmm1); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm1_hi); bb = 8; continue; }
            8 => { rax = ((rsi as u32)) as u32 as u64; rcx = (crate::simd_ops::ru64(r14)) as u64; fl_l = (((rcx) >> (((rax) as u32 & 63))) & 1) as u64; rcx = ((rcx) ^ ((1u64) << (((rax) as u32 & 63)))) as u64; crate::simd_ops::wu64(r14, rcx); fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 4; continue; } bb = 9; continue; }
            9 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xa0u64))) as u64; bb = 10; continue; }
            10 => { rax = ((rax).wrapping_add(1)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xa0u64), rax); r14 = ((r14).wrapping_add(0x8u64)) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(rsp.wrapping_add(0xa8u64))) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 2; continue; } bb = 11; continue; }
            11 => { r15 = 0; fl_l = 0; fl_r = 0; bb = 12; continue; }
            12 => { rdx = (crate::simd_ops::ru32(rbx.wrapping_add(0xa0u64))) as u32 as u64; r8 = rbx.wrapping_add(0x58); rcx = rbx.wrapping_add(0x8); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rsi = (crate::simd_ops::ru32(rbx.wrapping_add(0xc8u64))) as u32 as u64; fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 26; continue; } bb = 13; continue; }
            13 => { rax = ((rsi as u32)) as u32 as u64; rax = (((rax) as i64).wrapping_mul((crate::simd_ops::ru64(rbx.wrapping_add(0x78u64))) as i64) as u64) as u64; rdi = (crate::simd_ops::ru64(rbx.wrapping_add(0x80u64))) as u64; fl_l = rax; fl_r = rdi; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 15; continue; } bb = 14; continue; }
            14 => { rcx = ((rdi.wrapping_add(rax)) as u64) as u64; rdx = 0; fl_l = 0; fl_r = 0; rax = rcx.wrapping_sub(0x1); let d = (rdi) as u128; let n = ((rdx as u128) << 64) | (rax as u128); if d != 0 { rax = (n / d) as u64; rdx = (n % d) as u64; } rdi = rcx.wrapping_sub(0x1); fl_l = rdi; fl_r = rdx; rdi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; bb = 15; continue; }
            15 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm6 = (xmm0) as u128; r14 = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; rax = (crate::simd_ops::ru64(r14.wrapping_add(0x10u64))) as u64; fl_l = rax; fl_r = rdi; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 19; continue; } bb = 16; continue; }
            16 => { fl_l = (crate::simd_ops::ru8(P3)) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 22; continue; } bb = 17; continue; }
            17 => { fl_l = rax; fl_r = rdi; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 22; continue; } bb = 18; continue; }
            18 => { xmm1 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm0 as u64) - f64::from_bits(crate::simd_ops::ru64(rbx.wrapping_add(0x88u64)))).to_bits() as u128); ymm1_hi = 0; fl_l = ((xmm1 as u64)) as u64; fl_r = (crate::simd_ops::ru64(P4)) as u64; if !(f64::from_bits(fl_l) > f64::from_bits(fl_r)) { bb = 22; continue; } bb = 19; continue; }
            19 => { rcx = r14; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu32(rsp.wrapping_add(0x40u64), (0x1u32) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x44u64), (0x10u32) as u32); r9 = ((r15 as u32)) as u32 as u64; rax = rsp.wrapping_add(0x40); bb = 20; continue; }
            20 => { r9 = ((((r9 as u32)) | (crate::simd_ops::ru32(rax)))) as u32 as u64; rax = ((rax).wrapping_add(0x4u64)) as u64; rcx = rsp.wrapping_add(0x48); fl_l = rax; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 20; continue; } bb = 21; continue; }
            21 => { crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), r14); crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((r15 as u32)) as u32); r8 = 0; fl_l = 0; fl_r = 0; rdx = 0; fl_l = 0; fl_r = 0; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = ((r15 as u32)) as u32 as u64; fl_l = rdi; fl_r = (0x400000u64) as u64; rax = (rax & !0xffu64) | (((if (((fl_l as u64)) > ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); fl_l = (((rax as u32)).wrapping_add(0x2u32)) as u32 as u64; fl_r = fl_l; rax = ((fl_l as u32)) as u32 as u64; crate::simd_ops::wu32(r14.wrapping_add(0x50u64), ((rax as u32)) as u32); crate::simd_ops::wu64(rbx.wrapping_add(0x88u64), ((xmm6 as u64)) as u64); bb = 22; continue; }
            22 => { fl_l = ((rsi as u32)) as u32 as u64; fl_r = ((rsi as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 24; continue; } bb = 23; continue; }
            23 => { r8 = ((rsi.wrapping_add(rsi.wrapping_mul(8))) as u64) as u64; r8 = ((r8).wrapping_shl(3u32)) as u64; r9 = 0x2; rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rbx.wrapping_add(0x38u64), rax); crate::simd_ops::wu8(rbx.wrapping_add(0x30u64), (0x1u8) as u8); crate::simd_ops::wu32(rbx.wrapping_add(0x20u64), ((rsi as u32)) as u32); bb = 24; continue; }
            24 => { rcx = (crate::simd_ops::ru32(rbx.wrapping_add(0x40u64))) as u32 as u64; rax = ((rsi.wrapping_add(rcx)) as u32) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x40u64), ((rax as u32)) as u32); r8 = ((rsi.wrapping_add(rsi.wrapping_mul(8))) as u64) as u64; r8 = ((r8).wrapping_shl(3u32)) as u64; rcx = ((rcx.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x38u64))) as u64; rcx = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0xc0u64))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P8, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x20u64))) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 26; continue; } bb = 25; continue; }
            25 => { rdx = 0; fl_l = 0; fl_r = 0; rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P9, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rbx.wrapping_add(0x38u64), r15); crate::simd_ops::wu8(rbx.wrapping_add(0x30u64), (0x0u8) as u8); crate::simd_ops::wu32(rbx.wrapping_add(0x40u64), ((r15 as u32)) as u32); bb = 26; continue; }
            26 => { r8 = (crate::simd_ops::ru32(rbx.wrapping_add(0x20u64))) as u32 as u64; fl_l = ((r8 as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 29; continue; } bb = 27; continue; }
            27 => { fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(0x29u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 29; continue; } bb = 28; continue; }
            28 => { crate::simd_ops::wu8(rsp.wrapping_add(0x20u64), (0x0u8) as u8); r9 = 0x40; rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P10, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu32(rbx.wrapping_add(0x20u64), ((r15 as u32)) as u32); bb = 29; continue; }
            29 => { fl_l = (crate::simd_ops::ru8(P3)) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 33; continue; } bb = 30; continue; }
            30 => { rcx = rbx.wrapping_add(0xc0); rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x8u64))) as u32 as u64; r9 = ((rax.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; r9 = ((r9).wrapping_shl(3u32)) as u64; rax = (0xe38e38e38e38e39u64) as u64; let prod = (rax as i64 as i128).wrapping_mul((r9) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; rdx = ((((rdx) as i64).wrapping_shr(2u32) as u64)) as u64; rax = rdx; rax = ((rax).wrapping_shr(63u32)) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; rax = ((rdx.wrapping_add(rdx.wrapping_mul(8))) as u64) as u64; rax = ((rax).wrapping_shl(3u32)) as u64; fl_l = rax; fl_r = (0x100000u64) as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 33; continue; } bb = 31; continue; }
            31 => { rax = (crate::simd_ops::ru32(rcx.wrapping_add(0xcu64))) as u32 as u64; r8 = ((rax.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; r8 = ((r8).wrapping_shl(3u32)) as u64; fl_l = r9; fl_r = r8; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 33; continue; } bb = 32; continue; }
            32 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 33; continue; }
            33 => { crate::simd_ops::wu32(rbx.wrapping_add(0xc8u64), ((r15 as u32)) as u32); r11 = rsp.wrapping_add(0x70); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x40u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x48u64))) as u64; xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x60u64))) as u128; rsp = r11; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141305f20(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141247db0; const P1: u64 = 0x140fff1e0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0; let mut ymm1_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = r8; r8 = rdx; rbp = rcx; rbx = (crate::simd_ops::ru64(rcx.wrapping_add(0x58u64))) as u64; fl_l = rdx; fl_r = rbx; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rcx = ((rbx.wrapping_add(rdx)) as u64) as u64; rdx = 0; fl_l = 0; fl_r = 0; rax = rcx.wrapping_sub(0x1); let d = (rbx) as u128; let n = ((rdx as u128) << 64) | (rax as u128); if d != 0 { rax = (n / d) as u64; rdx = (n % d) as u64; } rbx = rcx.wrapping_sub(0x1); fl_l = rbx; fl_r = rdx; rbx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; bb = 2; continue; }
            2 => { crate::simd_ops::wu64(rbp.wrapping_add(0x90u64), r8); r14 = rbp.wrapping_add(0x98); rsi = (crate::simd_ops::ru32(r14.wrapping_add(0x8u64))) as u32 as u64; rax = (crate::simd_ops::ru32(r14.wrapping_add(0xcu64))) as u32 as u64; fl_l = rbx; fl_r = rax; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 7; continue; } bb = 3; continue; }
            3 => { rdx = rbx; rcx = r14; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru32(r14.wrapping_add(0x8u64))) as u32 as u64; rax = ((rax).wrapping_shl(6u32)) as u64; rax = ((rax).wrapping_add(crate::simd_ops::ru64(r14))) as u64; rcx = rbx; fl_l = rcx; fl_r = rsi; rcx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 13; continue; } bb = 4; continue; }
            4 => { bb = 5; continue; }
            5 => { let y_addr: u64 = rdi; xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rax; crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); let y_addr: u64 = rdi.wrapping_add(0x20u64); xmm1 = crate::simd_ops::ru128(y_addr); ymm1_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rax.wrapping_add(0x20u64); crate::simd_ops::wu128(y_addr, xmm1); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm1_hi); rax = rax.wrapping_add(0x40); fl_l = rcx; fl_r = (0x1u64) as u64; rcx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 5; continue; } bb = 6; continue; }
            6 => { bb = 13; continue; }
            7 => { fl_l = rbx; fl_r = rsi; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 12; continue; } bb = 8; continue; }
            8 => { rax = rsi; rax = ((rax).wrapping_shl(6u32)) as u64; rax = ((rax).wrapping_add(crate::simd_ops::ru64(r14))) as u64; rcx = rbx; fl_l = rcx; fl_r = rsi; rcx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 13; continue; } bb = 9; continue; }
            9 => { bb = 10; continue; }
            10 => { let y_addr: u64 = rdi; xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rax; crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); let y_addr: u64 = rdi.wrapping_add(0x20u64); xmm1 = crate::simd_ops::ru128(y_addr); ymm1_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rax.wrapping_add(0x20u64); crate::simd_ops::wu128(y_addr, xmm1); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm1_hi); rax = rax.wrapping_add(0x40); fl_l = rcx; fl_r = (0x1u64) as u64; rcx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 10; continue; } bb = 11; continue; }
            11 => { bb = 13; continue; }
            12 => { if (((fl_l as u64)) == ((fl_r as u64))) { bb = 14; continue; } bb = 13; continue; }
            13 => { crate::simd_ops::wu32(r14.wrapping_add(0x8u64), ((rbx as u32)) as u32); rsi = ((rbx as u32)) as u32 as u64; bb = 14; continue; }
            14 => { rdx = ((rsi as u32)) as u32 as u64; rdx = ((rdx).wrapping_shl(6u32)) as u64; rax = (crate::simd_ops::ru32(r14.wrapping_add(0xcu64))) as u32 as u64; rax = ((rax).wrapping_shl(6u32)) as u64; fl_l = rdx; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 16; continue; } bb = 15; continue; }
            15 => { rdx = ((((rdx) as i64).wrapping_shr(6u32) as u64)) as u64; rcx = r14; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 16; continue; }
            16 => { rcx = rbp.wrapping_add(0xa8); rdx = rbx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141306060(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x58; const P1: u64 = 0xffffffffffffffff; const P2: u64 = 0x90; const P3: u64 = 0x98; const P4: u64 = 0x1413072b0; const P5: u64 = 0xa0; const P6: u64 = 0xa4; const P7: u64 = 0x141247db0; const P8: u64 = 0xa8; const P9: u64 = 0x140fff1e0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; r8 = rdx; rdi = rcx; rbx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; fl_l = rdx; fl_r = rbx; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rcx = ((rbx.wrapping_add(rdx)) as u64) as u64; rdx = 0; fl_l = 0; fl_r = 0; rax = ((rcx.wrapping_add(P1)) as u64) as u64; let d = (rbx) as u128; let n = ((rdx as u128) << 64) | (rax as u128); if d != 0 { rax = (n / d) as u64; rdx = (n % d) as u64; } rbx = ((rcx.wrapping_add(P1)) as u64) as u64; fl_l = rbx; fl_r = rdx; rbx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; bb = 2; continue; }
            2 => { crate::simd_ops::wu64(rdi.wrapping_add(P2), r8); rdx = rbx; rcx = ((rdi.wrapping_add(P3)) as u64) as u64; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rdx = (crate::simd_ops::ru32(rdi.wrapping_add(P5))) as u32 as u64; rdx = ((rdx).wrapping_shl(6u32)) as u64; rax = (crate::simd_ops::ru32(rdi.wrapping_add(P6))) as u32 as u64; rax = ((rax).wrapping_shl(6u32)) as u64; fl_l = rdx; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rdx = ((((rdx) as i64).wrapping_shr(6u32) as u64)) as u64; rcx = ((rdi.wrapping_add(P3)) as u64) as u64; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); bb = 4; continue; }
            4 => { rcx = ((rdi.wrapping_add(P8)) as u64) as u64; rdx = rbx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(P9, rax, rcx, rdx, r8, r9, rsp); }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141306100(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x90; const P1: u64 = 0x58; const P2: u64 = 0xffffffffffffffff; const P3: u64 = 0x98; const P4: u64 = 0x1413072b0; const P5: u64 = 0xa0; const P6: u64 = 0xa4; const P7: u64 = 0x141247db0; const P8: u64 = 0xa8; const P9: u64 = 0x140fff1e0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = rcx; r8 = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; r8 = ((r8).wrapping_add(rdx)) as u64; rbx = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; fl_l = r8; fl_r = rbx; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rcx = ((rbx.wrapping_add(r8)) as u64) as u64; rdx = 0; fl_l = 0; fl_r = 0; rax = ((rcx.wrapping_add(P2)) as u64) as u64; let d = (rbx) as u128; let n = ((rdx as u128) << 64) | (rax as u128); if d != 0 { rax = (n / d) as u64; rdx = (n % d) as u64; } rbx = ((rcx.wrapping_add(P2)) as u64) as u64; fl_l = rbx; fl_r = rdx; rbx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; bb = 2; continue; }
            2 => { crate::simd_ops::wu64(rdi.wrapping_add(P0), r8); rdx = rbx; rcx = ((rdi.wrapping_add(P3)) as u64) as u64; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rdx = (crate::simd_ops::ru32(rdi.wrapping_add(P5))) as u32 as u64; rdx = ((rdx).wrapping_shl(6u32)) as u64; rax = (crate::simd_ops::ru32(rdi.wrapping_add(P6))) as u32 as u64; rax = ((rax).wrapping_shl(6u32)) as u64; fl_l = rdx; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rdx = ((((rdx) as i64).wrapping_shr(6u32) as u64)) as u64; rcx = ((rdi.wrapping_add(P3)) as u64) as u64; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); bb = 4; continue; }
            4 => { rcx = ((rdi.wrapping_add(P8)) as u64) as u64; rdx = rbx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(P9, rax, rcx, rdx, r8, r9, rsp); }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1413061a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x20; const P1: u64 = 0x29; const P2: u64 = 0x10; const P3: u64 = 0x8; const P4: u64 = 0x1422ab7b0;
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
    if !((((fl_l as u8)) != ((fl_r as u8)))) { rdx = (crate::simd_ops::ru64(rcx.wrapping_add(P2))) as u64; r9 = 0x40; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P3))) as u64; crate::simd_ops::wu8(rsp.wrapping_add(0x20u64), (0x0u8) as u8); rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu32(rbx.wrapping_add(P0), (0x0u32) as u32); }
    rsp = ((rsp).wrapping_add(0x30u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1413061e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x29; const P1: u64 = 0x20; const P2: u64 = 0x10; const P3: u64 = 0x8; const P4: u64 = 0x1422ab7b0;
    let (mut rax, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x30u64)) as u64;
    rax = ((crate::simd_ops::ru8(rcx.wrapping_add(P0))) as u32) as u32 as u64;
    rbx = rcx;
    r8 = (crate::simd_ops::ru32(rcx.wrapping_add(P1))) as u32 as u64;
    r9 = 0x40;
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
pub unsafe extern "win64" fn fn_141306220(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0xe0; const P1: u64 = 0xd0; const P2: u64 = 0x90; const P3: u64 = 0xa0;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu32(rcx.wrapping_add(P0), ((rax as u32)) as u32);
    crate::simd_ops::wu32(rcx.wrapping_add(P1), ((rax as u32)) as u32);
    crate::simd_ops::wu64(rcx.wrapping_add(P2), rax);
    crate::simd_ops::wu32(rcx.wrapping_add(P3), ((rax as u32)) as u32);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141306240(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x58; const P1: u64 = 0xffffffffffffffff; const P2: u64 = 0x142229d10; const P3: u64 = 0x90; const P4: u64 = 0x98; const P5: u64 = 0x141307210; const P6: u64 = 0xa0; const P7: u64 = 0xa4; const P8: u64 = 0x140a5b830; const P9: u64 = 0xa8; const P10: u64 = 0x140fff1e0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; r14 = r8; rbp = rdx; rsi = rcx; rcx = ((rcx).wrapping_add(0xd0u64)) as u64; rbx = (crate::simd_ops::ru32(rcx)) as u32 as u64; rdi = (crate::simd_ops::ru64(rsi.wrapping_add(P0))) as u64; fl_l = rdx; fl_r = rdi; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { r8 = ((rdi.wrapping_add(rdx)) as u64) as u64; rdx = 0; fl_l = 0; fl_r = 0; rax = ((r8.wrapping_add(P1)) as u64) as u64; let d = (rdi) as u128; let n = ((rdx as u128) << 64) | (rax as u128); if d != 0 { rax = (n / d) as u64; rdx = (n % d) as u64; } rdi = ((r8.wrapping_add(P1)) as u64) as u64; fl_l = rdi; fl_r = rdx; rdi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; bb = 2; continue; }
            2 => { rdx = ((rdi as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rsi.wrapping_add(P3), rbp); r8 = r14; rdx = rdi; rcx = ((rsi.wrapping_add(P4)) as u64) as u64; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rdx = (crate::simd_ops::ru32(rsi.wrapping_add(P6))) as u32 as u64; r8 = (crate::simd_ops::ru32(rsi.wrapping_add(P7))) as u32 as u64; r8 = ((r8).wrapping_shl(4u32)) as u64; rax = ((rdx as u32)) as u32 as u64; rax = ((rax).wrapping_shl(4u32)) as u64; fl_l = rax; fl_r = r8; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rcx = ((rsi.wrapping_add(P4)) as u64) as u64; rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); bb = 4; continue; }
            4 => { rdx = rdi; rcx = ((rsi.wrapping_add(P9)) as u64) as u64; rax = crate::simd_ops::call_stk(P10, rax, rcx, rdx, r8, r9, rsp); fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 7; continue; } bb = 5; continue; }
            5 => { bb = 6; continue; }
            6 => { r8 = (crate::simd_ops::ru64(rsi.wrapping_add(P9))) as u64; rcx = rbx; rcx = ((rcx).wrapping_shr(6u32)) as u64; rdx = rbx; rdx = ((((rdx as u32)) & (0x3fu32))) as u32 as u64; rax = (crate::simd_ops::ru64(r8.wrapping_add(rcx.wrapping_mul(8)))) as u64; fl_l = (((rax) >> (((rdx) as u32 & 63))) & 1) as u64; rax = ((rax) | ((1u64) << (((rdx) as u32 & 63)))) as u64; crate::simd_ops::wu64(r8.wrapping_add(rcx.wrapping_mul(8)), rax); rbx = ((rbx).wrapping_add(1)) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 6; continue; } bb = 7; continue; }
            7 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141306620(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1408ce8a0; const P1: u64 = 0x143868280; const P2: u64 = 0x142261020; const P3: u64 = 0x1412f9440; const P4: u64 = 0x14209c560; const P5: u64 = 0x14183acbc; const P6: u64 = 0x14209d6f0; const P7: u64 = 0x1422ab7b0; const P8: u64 = 0x143868298;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rdi = rcx; r14 = (crate::simd_ops::ru64(rcx.wrapping_add(0xa8u64))) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(0xb8u64))) as u64; rax = ((rax).wrapping_add(0x3fu64)) as u64; rax = (((rax) & (0xffffffffffffffc0u64))) as u64; rax = ((rax).wrapping_shr(6u32)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), rax); rbp = 0; fl_l = 0; fl_r = 0; r13 = ((rbp as u32)) as u32 as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 1; continue; }
            1 => { bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(r14)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 3; continue; }
            3 => { r12 = ((r13 as u32)) as u32 as u64; fl_l = (((r12 as u32)).wrapping_shl(6u32)) as u32 as u64; r12 = ((fl_l as u32)) as u32 as u64; bb = 4; continue; }
            4 => { let bit_in: u64 = rax; rsi = ((bit_in.trailing_zeros() as u64)) as u64; fl_l = (rsi as u64) as u64; r15 = ((r12.wrapping_add(rsi)) as u32) as u32 as u64; rbp = ((r15 as u32)) as u32 as u64; fl_l = rbp; fl_r = (crate::simd_ops::ru64(rdi.wrapping_add(0x90u64))) as u64; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 8; continue; } bb = 5; continue; }
            5 => { rbx = rdi.wrapping_add(0xc0); rdx = (crate::simd_ops::ru32(rbx.wrapping_add(0x8u64))) as u32 as u64; r8 = (crate::simd_ops::ru32(rbx.wrapping_add(0xcu64))) as u32 as u64; rcx = ((r8.wrapping_add(r8.wrapping_mul(2))) as u64) as u64; rcx = ((rcx).wrapping_shl(3u32)) as u64; rax = ((rdx.wrapping_add(rdx.wrapping_mul(2))) as u64) as u64; rax = ((rax).wrapping_shl(3u32)) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 7; continue; } bb = 6; continue; }
            6 => { rax = rdx.wrapping_add(0x1); rdx = ((r8 as u32)) as u32 as u64; rdx = ((rdx).wrapping_shr(1u32)) as u64; rdx = ((rdx).wrapping_add(r8)) as u64; fl_l = rdx; fl_r = rax; rdx = (if (((fl_l as u64)) < ((fl_r as u64))) { rax } else { rdx }) as u64; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 7; continue; }
            7 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x8u64))) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = (crate::simd_ops::ru64(rbx)) as u64; xmm0 = 0u128; rdx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu128(rax.wrapping_add(rcx.wrapping_mul(8)), (xmm0) as u128); crate::simd_ops::wu64(rax.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x10u64), rdx); r8 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rax.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0xcu64), r8); crate::simd_ops::wu32(rax.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x14u64), ((r8 as u32)) as u32); crate::simd_ops::lk_add_u32(rbx.wrapping_add(0x8u64), (1) as u32); rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x8u64))) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = (crate::simd_ops::ru64(rbx)) as u64; crate::simd_ops::wu32(rax.wrapping_add(rcx.wrapping_mul(8)).wrapping_sub(0x18u64), ((r15 as u32)) as u32); fl_l = ((rbp).wrapping_add(rbp)) as u64; fl_r = fl_l; rbp = ((fl_l as u64)) as u64; rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0x98u64))) as u64; xmm0 = (crate::simd_ops::ru128(rdx.wrapping_add(rbp.wrapping_mul(8)))) as u128; crate::simd_ops::wu128(rax.wrapping_add(rcx.wrapping_mul(8)).wrapping_sub(0x10u64), (xmm0) as u128); bb = 8; continue; }
            8 => { rcx = ((rsi as u32)) as u32 as u64; rax = (crate::simd_ops::ru64(r14)) as u64; fl_l = (((rax) >> (((rcx) as u32 & 63))) & 1) as u64; rax = ((rax) ^ ((1u64) << (((rcx) as u32 & 63)))) as u64; crate::simd_ops::wu64(r14, rax); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 4; continue; } bb = 9; continue; }
            9 => { r13 = ((r13).wrapping_add(1)) as u64; r14 = ((r14).wrapping_add(0x8u64)) as u64; fl_l = r13; fl_r = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 2; continue; } bb = 10; continue; }
            10 => { rbp = 0; fl_l = 0; fl_r = 0; bb = 11; continue; }
            11 => { rax = (crate::simd_ops::ru32(rdi.wrapping_add(0xa0u64))) as u32 as u64; r8 = (crate::simd_ops::ru64(rdi.wrapping_add(0x60u64))) as u64; rcx = r8; rcx = (((rcx) as i64).wrapping_mul((rax) as i64) as u64) as u64; r9 = (crate::simd_ops::ru64(rdi.wrapping_add(0x68u64))) as u64; fl_l = rcx; fl_r = r9; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 13; continue; } bb = 12; continue; }
            12 => { rdx = 0; fl_l = 0; fl_r = 0; rax = r9.wrapping_sub(0x1); rax = ((rax).wrapping_add(rcx)) as u64; let d = (r9) as u128; let n = ((rdx as u128) << 64) | (rax as u128); if d != 0 { rax = (n / d) as u64; rdx = (n % d) as u64; } r9 = ((r9).wrapping_sub(1)) as u64; r9 = ((r9).wrapping_add(rcx)) as u64; fl_l = r9; fl_r = rdx; r9 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; bb = 13; continue; }
            13 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; rdx = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; fl_l = rdx; fl_r = r9; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 16; continue; } bb = 14; continue; }
            14 => { fl_l = (crate::simd_ops::ru8(P1)) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 17; continue; } bb = 15; continue; }
            15 => { fl_l = rdx; fl_r = r9; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 17; continue; } bb = 16; continue; }
            16 => { rdx = r9; rcx = rdi.wrapping_add(0x8); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 17; continue; }
            17 => { rbx = (crate::simd_ops::ru32(rdi.wrapping_add(0xc8u64))) as u32 as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 22; continue; } bb = 18; continue; }
            18 => { r9 = rdi.wrapping_add(0x88); r8 = rdi.wrapping_add(0x70); rdx = ((rbx as u32)) as u32 as u64; rcx = rdi.wrapping_add(0x10); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rbx as u32)) as u32 as u64; fl_r = ((rbx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 20; continue; } bb = 19; continue; }
            19 => { r8 = ((rbx.wrapping_add(rbx.wrapping_mul(2))) as u64) as u64; r8 = ((r8).wrapping_shl(3u32)) as u64; r9 = 0x2; rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x10u64))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rdi.wrapping_add(0x38u64), rax); crate::simd_ops::wu8(rdi.wrapping_add(0x30u64), (0x1u8) as u8); crate::simd_ops::wu32(rdi.wrapping_add(0x20u64), ((rbx as u32)) as u32); bb = 20; continue; }
            20 => { rcx = (crate::simd_ops::ru32(rdi.wrapping_add(0x40u64))) as u32 as u64; rax = ((rbx.wrapping_add(rcx)) as u32) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x40u64), ((rax as u32)) as u32); r8 = ((rbx.wrapping_add(rbx.wrapping_mul(2))) as u64) as u64; r8 = ((r8).wrapping_shl(3u32)) as u64; rcx = ((rcx.wrapping_add(rcx.wrapping_mul(2))) as u64) as u64; rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x38u64))) as u64; rcx = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0xc0u64))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = (crate::simd_ops::ru32(rdi.wrapping_add(0x20u64))) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 22; continue; } bb = 21; continue; }
            21 => { rdx = 0; fl_l = 0; fl_r = 0; rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x10u64))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rdi.wrapping_add(0x38u64), rbp); crate::simd_ops::wu8(rdi.wrapping_add(0x30u64), (0x0u8) as u8); crate::simd_ops::wu32(rdi.wrapping_add(0x40u64), ((rbp as u32)) as u32); bb = 22; continue; }
            22 => { r8 = (crate::simd_ops::ru32(rdi.wrapping_add(0x20u64))) as u32 as u64; fl_l = ((r8 as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 25; continue; } bb = 23; continue; }
            23 => { fl_l = (crate::simd_ops::ru8(rdi.wrapping_add(0x29u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 25; continue; } bb = 24; continue; }
            24 => { crate::simd_ops::wu8(rsp.wrapping_add(0x20u64), (0x0u8) as u8); r9 = 0x10; rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0x10u64))) as u64; rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu32(rdi.wrapping_add(0x20u64), ((rbp as u32)) as u32); bb = 25; continue; }
            25 => { fl_l = (crate::simd_ops::ru8(P8)) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 29; continue; } bb = 26; continue; }
            26 => { rcx = rdi.wrapping_add(0xc0); rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x8u64))) as u32 as u64; r9 = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; r9 = ((r9).wrapping_shl(3u32)) as u64; rax = (0x2aaaaaaaaaaaaaabu64) as u64; let prod = (rax as i64 as i128).wrapping_mul((r9) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; rdx = ((((rdx) as i64).wrapping_shr(2u32) as u64)) as u64; rax = rdx; rax = ((rax).wrapping_shr(63u32)) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; rax = ((rdx.wrapping_add(rdx.wrapping_mul(2))) as u64) as u64; rax = ((rax).wrapping_shl(3u32)) as u64; fl_l = rax; fl_r = (0x100000u64) as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 29; continue; } bb = 27; continue; }
            27 => { rax = (crate::simd_ops::ru32(rcx.wrapping_add(0xcu64))) as u32 as u64; r8 = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; r8 = ((r8).wrapping_shl(3u32)) as u64; fl_l = r9; fl_r = r8; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 29; continue; } bb = 28; continue; }
            28 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 29; continue; }
            29 => { crate::simd_ops::wu32(rdi.wrapping_add(0xc8u64), ((rbp as u32)) as u32); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141306970(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142229c90; const P1: u64 = 0x14397ff48; const P2: u64 = 0x14397fed0; const P3: u64 = 0x14222edd0;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0; let mut ymm1_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (crate::simd_ops::ru64(rdx)) as u64; rsi = rdx; rdx = (crate::simd_ops::ru32(rdx.wrapping_add(0xcu64))) as u32 as u64; rdi = rcx; rcx = ((rcx).wrapping_add(0x1a0u64)) as u64; r8 = ((crate::simd_ops::ru16(rax.wrapping_add(0x20u64))) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r11 = (crate::simd_ops::ru64(rdi.wrapping_add(0x1e8u64))) as u64; r11 = ((r11).wrapping_sub(1)) as u64; crate::simd_ops::wu64(rdi.wrapping_add(0x1e8u64), r11); r10 = r11; r9 = (crate::simd_ops::ru32(rsi.wrapping_add(0x10u64))) as u32 as u64; r10 = ((((r10 as u32)) & (0x3fu32))) as u32 as u64; rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x98u64))) as u64; rcx = ((r9 as u32)) as u32 as u64; r8 = (crate::simd_ops::ru64(rdi.wrapping_add(0xa8u64))) as u64; rdx = r11; rcx = ((rcx).wrapping_shr(6u32)) as u64; rdx = ((rdx).wrapping_shl(6u32)) as u64; let y_addr: u64 = rdx.wrapping_add(rax); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rdx.wrapping_add(rax).wrapping_add(0x20u64); xmm1 = crate::simd_ops::ru128(y_addr); ymm1_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); rax = (crate::simd_ops::ru64(r8.wrapping_add(rcx.wrapping_mul(8)))) as u64; rdx = ((r9 as u32)) as u32 as u64; r9 = ((r9).wrapping_shl(6u32)) as u64; rdx = ((((rdx as u32)) & (0x3fu32))) as u32 as u64; fl_l = (((rax) >> (((rdx) as u32 & 63))) & 1) as u64; rax = ((rax) | ((1u64) << (((rdx) as u32 & 63)))) as u64; crate::simd_ops::wu64(r8.wrapping_add(rcx.wrapping_mul(8)), rax); rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x98u64))) as u64; let y_addr: u64 = r9.wrapping_add(rax); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); let y_addr: u64 = r9.wrapping_add(rax).wrapping_add(0x20u64); crate::simd_ops::wu128(y_addr, xmm1); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm1_hi); rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0xa8u64))) as u64; rax = r11; rax = ((rax).wrapping_shr(6u32)) as u64; r9 = ((rax.wrapping_mul(8)) as u64) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(rax.wrapping_mul(8)))) as u64; fl_l = (((rax) >> (((r10) as u32 & 63))) & 1) as u64; rax = ((rax) & !((1u64) << (((r10) as u32 & 63)))) as u64; crate::simd_ops::wu64(rcx.wrapping_add(r9), rax); r8 = (crate::simd_ops::ru32(rsi.wrapping_add(0x10u64))) as u32 as u64; rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x458u64))) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(r11.wrapping_mul(4)))) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(r8.wrapping_mul(4)), ((rax as u32)) as u32); rcx = ((r8 as u32)) as u32 as u64; rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0x468u64))) as u64; r8 = ((((r8 as u32)) & (0x3fu32))) as u32 as u64; rcx = ((rcx).wrapping_shr(6u32)) as u64; rax = (crate::simd_ops::ru64(rdx.wrapping_add(rcx.wrapping_mul(8)))) as u64; fl_l = (((rax) >> (((r8) as u32 & 63))) & 1) as u64; rax = ((rax) | ((1u64) << (((r8) as u32 & 63)))) as u64; crate::simd_ops::wu64(rdx.wrapping_add(rcx.wrapping_mul(8)), rax); rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x468u64))) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(r9))) as u64; fl_l = (((rax) >> (((r10) as u32 & 63))) & 1) as u64; rax = ((rax) & !((1u64) << (((r10) as u32 & 63)))) as u64; crate::simd_ops::wu64(rcx.wrapping_add(r9), rax); rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0x1d8u64))) as u64; rax = (crate::simd_ops::ru32(rsi.wrapping_add(0x10u64))) as u32 as u64; rcx = (crate::simd_ops::ru64(rdx.wrapping_add(r11.wrapping_mul(8)))) as u64; crate::simd_ops::wu64(rdx.wrapping_add(rax.wrapping_mul(8)), rcx); rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x1d8u64))) as u64; crate::simd_ops::wu32(rax.wrapping_add(r11.wrapping_mul(8)), (0xffffffffu32) as u32); r9 = (crate::simd_ops::ru32(rsi.wrapping_add(0x10u64))) as u32 as u64; fl_l = r9; fl_r = (crate::simd_ops::ru64(rdi.wrapping_add(0x1e8u64))) as u64; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 8; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x1d8u64))) as u64; fl_l = (crate::simd_ops::ru8(rax.wrapping_add(r9.wrapping_mul(8)).wrapping_add(0x4u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; r8 = ((rax.wrapping_add(r9.wrapping_mul(8))) as u64) as u64; rax = (crate::simd_ops::ru32(rax.wrapping_add(r9.wrapping_mul(8)))) as u32 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 7; continue; } bb = 2; continue; }
            2 => { rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbx); rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x270u64))) as u64; rdx = (crate::simd_ops::ru32(rax.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x8u64))) as u32 as u64; crate::simd_ops::wu32(rax.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x10u64), ((r9 as u32)) as u32); rbx = ((rdx as u32)) as u32 as u64; rax = (crate::simd_ops::ru32(r8)) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x270u64))) as u64; r8 = (crate::simd_ops::ru64(rax.wrapping_add(rcx.wrapping_mul(8)))) as u64; rax = (crate::simd_ops::ru64(P1)) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; rax = (crate::simd_ops::ru32(rsi.wrapping_add(0x10u64))) as u32 as u64; rdx = (crate::simd_ops::ru64(rcx.wrapping_add(rdx.wrapping_mul(8)))) as u64; crate::simd_ops::wu32(rdx.wrapping_add(0x68u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru64(r8.wrapping_add(0x8u64))) as u64; rcx = (crate::simd_ops::ru32(rax.wrapping_add(0x54u64))) as u32 as u64; rax = (crate::simd_ops::ru64(P2)) as u64; rcx = ((rcx.wrapping_add(rcx.wrapping_mul(4))) as u64) as u64; rcx = ((rcx).wrapping_shl(5u32)) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rax.wrapping_add(0x120u64)))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r8 = (crate::simd_ops::ru64(P1)) as u64; rax = (crate::simd_ops::ru64(r8.wrapping_add(0x18u64))) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(rbx.wrapping_mul(8)))) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x5cu64))) as u32 as u64; r9 = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = (crate::simd_ops::ru64(P2)) as u64; r9 = ((r9).wrapping_shl(4u32)) as u64; r9 = ((r9).wrapping_add(crate::simd_ops::ru64(rax.wrapping_add(0x148u64)))) as u64; rax = (crate::simd_ops::ru64(r8.wrapping_add(0x1a8u64))) as u64; rcx = ((crate::simd_ops::ru8(rbx.wrapping_add(rax))) as u32) as u32 as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; fl_l = ((rcx as u8)) as u8 as u64; fl_r = (0x8u8) as u8 as u64; if (((fl_l as u8)) >= ((fl_r as u8))) { bb = 8; continue; } bb = 3; continue; }
            3 => { rcx = ((crate::simd_ops::ru8(rcx.wrapping_add(r9).wrapping_add(0xcu64))) as u32) as u32 as u64; fl_l = ((rcx as u8)) as u8 as u64; fl_r = (0xffu8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 8; continue; } bb = 4; continue; }
            4 => { rax = ((rcx as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_add(crate::simd_ops::ru32(r9.wrapping_add(0x48u64)))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0xffffffff; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 8; continue; } bb = 5; continue; }
            5 => { rcx = ((rax as u32)) as u32 as u64; rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x280u64))) as u64; rcx = ((rcx).wrapping_add(rcx)) as u64; rdx = (crate::simd_ops::ru32(rax.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0xcu64))) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0xffffffff; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 8; continue; } bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x4a8u64))) as u64; rcx = ((rdx.wrapping_add(rdx.wrapping_mul(2))) as u64) as u64; crate::simd_ops::wu8(rax.wrapping_add(rcx.wrapping_mul(4)), (0x1u8) as u8); bb = 8; continue; }
            7 => { rax = ((rax).wrapping_shl(5u32)) as u64; fl_l = ((rax).wrapping_add(crate::simd_ops::ru64(rdi.wrapping_add(0x290u64)))) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; crate::simd_ops::wu32(rax.wrapping_add(0x14u64), ((r9 as u32)) as u32); bb = 8; continue; }
            8 => { rax = (crate::simd_ops::ru64(P1)) as u64; rdx = (crate::simd_ops::ru32(rsi.wrapping_add(0x8u64))) as u32 as u64; r8 = (crate::simd_ops::ru64(rsi)) as u64; crate::simd_ops::wu32(rsi.wrapping_add(0x10u64), (0xffffffffu32) as u32); rcx = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(rdx.wrapping_mul(8)))) as u64; crate::simd_ops::wu32(rax.wrapping_add(0x68u64), (0xffffffffu32) as u32); rax = (crate::simd_ops::ru64(r8.wrapping_add(0x8u64))) as u64; rcx = (crate::simd_ops::ru32(rax.wrapping_add(0x54u64))) as u32 as u64; rax = (crate::simd_ops::ru64(P2)) as u64; rcx = ((rcx.wrapping_add(rcx.wrapping_mul(4))) as u64) as u64; rcx = ((rcx).wrapping_shl(5u32)) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rax.wrapping_add(0x120u64)))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141306c00(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142229c90; const P1: u64 = 0x14397ff48; const P2: u64 = 0x14397fed0; const P3: u64 = 0x14222edd0;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0; let mut ymm1_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (crate::simd_ops::ru64(rdx)) as u64; rdi = rdx; rdx = (crate::simd_ops::ru32(rdx.wrapping_add(0x10u64))) as u32 as u64; rsi = rcx; rcx = ((rcx).wrapping_add(0x1a0u64)) as u64; r8 = (crate::simd_ops::ru64(rax)) as u64; r8 = (crate::simd_ops::ru32(r8.wrapping_add(0x30u64))) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r11 = (crate::simd_ops::ru64(rsi.wrapping_add(0x1e8u64))) as u64; r11 = ((r11).wrapping_sub(1)) as u64; crate::simd_ops::wu64(rsi.wrapping_add(0x1e8u64), r11); r10 = r11; rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x98u64))) as u64; r10 = ((((r10 as u32)) & (0x3fu32))) as u32 as u64; r9 = (crate::simd_ops::ru32(rdi.wrapping_add(0x14u64))) as u32 as u64; rcx = r11; r8 = (crate::simd_ops::ru64(rsi.wrapping_add(0xa8u64))) as u64; rdx = ((r9 as u32)) as u32 as u64; rcx = ((rcx).wrapping_shl(6u32)) as u64; rdx = ((((rdx as u32)) & (0x3fu32))) as u32 as u64; let y_addr: u64 = rcx.wrapping_add(rax); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rcx.wrapping_add(rax).wrapping_add(0x20u64); xmm1 = crate::simd_ops::ru128(y_addr); ymm1_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); rcx = ((r9 as u32)) as u32 as u64; r9 = ((r9).wrapping_shl(6u32)) as u64; rcx = ((rcx).wrapping_shr(6u32)) as u64; rax = (crate::simd_ops::ru64(r8.wrapping_add(rcx.wrapping_mul(8)))) as u64; fl_l = (((rax) >> (((rdx) as u32 & 63))) & 1) as u64; rax = ((rax) | ((1u64) << (((rdx) as u32 & 63)))) as u64; crate::simd_ops::wu64(r8.wrapping_add(rcx.wrapping_mul(8)), rax); rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x98u64))) as u64; let y_addr: u64 = r9.wrapping_add(rax); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); let y_addr: u64 = r9.wrapping_add(rax).wrapping_add(0x20u64); crate::simd_ops::wu128(y_addr, xmm1); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm1_hi); rcx = (crate::simd_ops::ru64(rsi.wrapping_add(0xa8u64))) as u64; rax = r11; rax = ((rax).wrapping_shr(6u32)) as u64; r9 = ((rax.wrapping_mul(8)) as u64) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(rax.wrapping_mul(8)))) as u64; fl_l = (((rax) >> (((r10) as u32 & 63))) & 1) as u64; rax = ((rax) & !((1u64) << (((r10) as u32 & 63)))) as u64; crate::simd_ops::wu64(rcx.wrapping_add(r9), rax); r8 = (crate::simd_ops::ru32(rdi.wrapping_add(0x14u64))) as u32 as u64; rcx = (crate::simd_ops::ru64(rsi.wrapping_add(0x458u64))) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(r11.wrapping_mul(4)))) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(r8.wrapping_mul(4)), ((rax as u32)) as u32); rcx = ((r8 as u32)) as u32 as u64; rdx = (crate::simd_ops::ru64(rsi.wrapping_add(0x468u64))) as u64; r8 = ((((r8 as u32)) & (0x3fu32))) as u32 as u64; rcx = ((rcx).wrapping_shr(6u32)) as u64; rax = (crate::simd_ops::ru64(rdx.wrapping_add(rcx.wrapping_mul(8)))) as u64; fl_l = (((rax) >> (((r8) as u32 & 63))) & 1) as u64; rax = ((rax) | ((1u64) << (((r8) as u32 & 63)))) as u64; crate::simd_ops::wu64(rdx.wrapping_add(rcx.wrapping_mul(8)), rax); rcx = (crate::simd_ops::ru64(rsi.wrapping_add(0x468u64))) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(r9))) as u64; fl_l = (((rax) >> (((r10) as u32 & 63))) & 1) as u64; rax = ((rax) & !((1u64) << (((r10) as u32 & 63)))) as u64; crate::simd_ops::wu64(rcx.wrapping_add(r9), rax); rdx = (crate::simd_ops::ru64(rsi.wrapping_add(0x1d8u64))) as u64; rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x14u64))) as u32 as u64; rcx = (crate::simd_ops::ru64(rdx.wrapping_add(r11.wrapping_mul(8)))) as u64; crate::simd_ops::wu64(rdx.wrapping_add(rax.wrapping_mul(8)), rcx); rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x1d8u64))) as u64; crate::simd_ops::wu32(rax.wrapping_add(r11.wrapping_mul(8)), (0xffffffffu32) as u32); r9 = (crate::simd_ops::ru32(rdi.wrapping_add(0x14u64))) as u32 as u64; fl_l = r9; fl_r = (crate::simd_ops::ru64(rsi.wrapping_add(0x1e8u64))) as u64; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 8; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x1d8u64))) as u64; fl_l = (crate::simd_ops::ru8(rax.wrapping_add(r9.wrapping_mul(8)).wrapping_add(0x4u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; r8 = ((rax.wrapping_add(r9.wrapping_mul(8))) as u64) as u64; rax = (crate::simd_ops::ru32(rax.wrapping_add(r9.wrapping_mul(8)))) as u32 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 7; continue; } bb = 2; continue; }
            2 => { rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbx); rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x270u64))) as u64; rdx = (crate::simd_ops::ru32(rax.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x8u64))) as u32 as u64; crate::simd_ops::wu32(rax.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x10u64), ((r9 as u32)) as u32); rbx = ((rdx as u32)) as u32 as u64; rax = (crate::simd_ops::ru32(r8)) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x270u64))) as u64; r8 = (crate::simd_ops::ru64(rax.wrapping_add(rcx.wrapping_mul(8)))) as u64; rax = (crate::simd_ops::ru64(P1)) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x14u64))) as u32 as u64; rdx = (crate::simd_ops::ru64(rcx.wrapping_add(rdx.wrapping_mul(8)))) as u64; crate::simd_ops::wu32(rdx.wrapping_add(0x68u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru64(r8.wrapping_add(0x8u64))) as u64; rcx = (crate::simd_ops::ru32(rax.wrapping_add(0x54u64))) as u32 as u64; rax = (crate::simd_ops::ru64(P2)) as u64; rcx = ((rcx.wrapping_add(rcx.wrapping_mul(4))) as u64) as u64; rcx = ((rcx).wrapping_shl(5u32)) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rax.wrapping_add(0x120u64)))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r8 = (crate::simd_ops::ru64(P1)) as u64; rax = (crate::simd_ops::ru64(r8.wrapping_add(0x18u64))) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(rbx.wrapping_mul(8)))) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x5cu64))) as u32 as u64; r9 = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = (crate::simd_ops::ru64(P2)) as u64; r9 = ((r9).wrapping_shl(4u32)) as u64; r9 = ((r9).wrapping_add(crate::simd_ops::ru64(rax.wrapping_add(0x148u64)))) as u64; rax = (crate::simd_ops::ru64(r8.wrapping_add(0x1a8u64))) as u64; rcx = ((crate::simd_ops::ru8(rbx.wrapping_add(rax))) as u32) as u32 as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; fl_l = ((rcx as u8)) as u8 as u64; fl_r = (0x8u8) as u8 as u64; if (((fl_l as u8)) >= ((fl_r as u8))) { bb = 8; continue; } bb = 3; continue; }
            3 => { rcx = ((crate::simd_ops::ru8(rcx.wrapping_add(r9).wrapping_add(0xcu64))) as u32) as u32 as u64; fl_l = ((rcx as u8)) as u8 as u64; fl_r = (0xffu8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 8; continue; } bb = 4; continue; }
            4 => { rax = ((rcx as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_add(crate::simd_ops::ru32(r9.wrapping_add(0x48u64)))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0xffffffff; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 8; continue; } bb = 5; continue; }
            5 => { rcx = ((rax as u32)) as u32 as u64; rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x280u64))) as u64; rcx = ((rcx).wrapping_add(rcx)) as u64; rdx = (crate::simd_ops::ru32(rax.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0xcu64))) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0xffffffff; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 8; continue; } bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x4a8u64))) as u64; rcx = ((rdx.wrapping_add(rdx.wrapping_mul(2))) as u64) as u64; crate::simd_ops::wu8(rax.wrapping_add(rcx.wrapping_mul(4)), (0x1u8) as u8); bb = 8; continue; }
            7 => { rax = ((rax).wrapping_shl(5u32)) as u64; fl_l = ((rax).wrapping_add(crate::simd_ops::ru64(rsi.wrapping_add(0x290u64)))) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; crate::simd_ops::wu32(rax.wrapping_add(0x14u64), ((r9 as u32)) as u32); bb = 8; continue; }
            8 => { crate::simd_ops::wu32(rdi.wrapping_add(0x14u64), (0xffffffffu32) as u32); rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141306e50(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14020bcb0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = rdx; rbx = rcx; rdx = (crate::simd_ops::ru32(rcx.wrapping_add(0x8u64))) as u32 as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(0xcu64))) as u32 as u64; rax = ((rax).wrapping_sub(rdx)) as u64; fl_l = rax; fl_r = (0x20u64) as u64; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rdx = ((rdx).wrapping_add(0x20u64)) as u64; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); bb = 2; continue; }
            2 => { rcx = (crate::simd_ops::ru32(rbx.wrapping_add(0x8u64))) as u32 as u64; rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rdx, rax); crate::simd_ops::wu32(rdx, (0xffffffffu32) as u32); crate::simd_ops::wu64(rdx.wrapping_add(0x8u64), rax); crate::simd_ops::wu32(rdx.wrapping_add(0x8u64), (0xffffffffu32) as u32); crate::simd_ops::wu64(rdx.wrapping_add(0x10u64), rax); crate::simd_ops::wu32(rdx.wrapping_add(0x10u64), (0xffffffffu32) as u32); crate::simd_ops::wu64(rdx.wrapping_add(0x18u64), rax); crate::simd_ops::wu32(rdx.wrapping_add(0x18u64), (0xffffffffu32) as u32); crate::simd_ops::wu64(rdx.wrapping_add(0x20u64), rax); crate::simd_ops::wu32(rdx.wrapping_add(0x20u64), (0xffffffffu32) as u32); crate::simd_ops::wu64(rdx.wrapping_add(0x28u64), rax); crate::simd_ops::wu32(rdx.wrapping_add(0x28u64), (0xffffffffu32) as u32); crate::simd_ops::wu64(rdx.wrapping_add(0x30u64), rax); crate::simd_ops::wu32(rdx.wrapping_add(0x30u64), (0xffffffffu32) as u32); crate::simd_ops::wu64(rdx.wrapping_add(0x38u64), rax); crate::simd_ops::wu32(rdx.wrapping_add(0x38u64), (0xffffffffu32) as u32); crate::simd_ops::wu64(rdx.wrapping_add(0x40u64), rax); crate::simd_ops::wu32(rdx.wrapping_add(0x40u64), (0xffffffffu32) as u32); crate::simd_ops::wu64(rdx.wrapping_add(0x48u64), rax); crate::simd_ops::wu32(rdx.wrapping_add(0x48u64), (0xffffffffu32) as u32); crate::simd_ops::wu64(rdx.wrapping_add(0x50u64), rax); crate::simd_ops::wu32(rdx.wrapping_add(0x50u64), (0xffffffffu32) as u32); crate::simd_ops::wu64(rdx.wrapping_add(0x58u64), rax); crate::simd_ops::wu32(rdx.wrapping_add(0x58u64), (0xffffffffu32) as u32); crate::simd_ops::wu64(rdx.wrapping_add(0x60u64), rax); crate::simd_ops::wu32(rdx.wrapping_add(0x60u64), (0xffffffffu32) as u32); crate::simd_ops::wu64(rdx.wrapping_add(0x68u64), rax); crate::simd_ops::wu32(rdx.wrapping_add(0x68u64), (0xffffffffu32) as u32); crate::simd_ops::wu64(rdx.wrapping_add(0x70u64), rax); crate::simd_ops::wu32(rdx.wrapping_add(0x70u64), (0xffffffffu32) as u32); crate::simd_ops::wu64(rdx.wrapping_add(0x78u64), rax); crate::simd_ops::wu32(rdx.wrapping_add(0x78u64), (0xffffffffu32) as u32); crate::simd_ops::wu64(rdx.wrapping_add(0x80u64), rax); crate::simd_ops::wu32(rdx.wrapping_add(0x80u64), (0xffffffffu32) as u32); crate::simd_ops::wu64(rdx.wrapping_add(0x88u64), rax); crate::simd_ops::wu32(rdx.wrapping_add(0x88u64), (0xffffffffu32) as u32); crate::simd_ops::wu64(rdx.wrapping_add(0x90u64), rax); crate::simd_ops::wu32(rdx.wrapping_add(0x90u64), (0xffffffffu32) as u32); crate::simd_ops::wu64(rdx.wrapping_add(0x98u64), rax); crate::simd_ops::wu32(rdx.wrapping_add(0x98u64), (0xffffffffu32) as u32); crate::simd_ops::wu64(rdx.wrapping_add(0xa0u64), rax); crate::simd_ops::wu32(rdx.wrapping_add(0xa0u64), (0xffffffffu32) as u32); crate::simd_ops::wu64(rdx.wrapping_add(0xa8u64), rax); crate::simd_ops::wu32(rdx.wrapping_add(0xa8u64), (0xffffffffu32) as u32); crate::simd_ops::wu64(rdx.wrapping_add(0xb0u64), rax); crate::simd_ops::wu32(rdx.wrapping_add(0xb0u64), (0xffffffffu32) as u32); crate::simd_ops::wu64(rdx.wrapping_add(0xb8u64), rax); crate::simd_ops::wu32(rdx.wrapping_add(0xb8u64), (0xffffffffu32) as u32); crate::simd_ops::wu64(rdx.wrapping_add(0xc0u64), rax); crate::simd_ops::wu32(rdx.wrapping_add(0xc0u64), (0xffffffffu32) as u32); crate::simd_ops::wu64(rdx.wrapping_add(0xc8u64), rax); crate::simd_ops::wu32(rdx.wrapping_add(0xc8u64), (0xffffffffu32) as u32); crate::simd_ops::wu64(rdx.wrapping_add(0xd0u64), rax); crate::simd_ops::wu32(rdx.wrapping_add(0xd0u64), (0xffffffffu32) as u32); crate::simd_ops::wu64(rdx.wrapping_add(0xd8u64), rax); crate::simd_ops::wu32(rdx.wrapping_add(0xd8u64), (0xffffffffu32) as u32); crate::simd_ops::wu64(rdx.wrapping_add(0xe0u64), rax); crate::simd_ops::wu32(rdx.wrapping_add(0xe0u64), (0xffffffffu32) as u32); crate::simd_ops::wu64(rdx.wrapping_add(0xe8u64), rax); crate::simd_ops::wu32(rdx.wrapping_add(0xe8u64), (0xffffffffu32) as u32); crate::simd_ops::wu64(rdx.wrapping_add(0xf0u64), rax); crate::simd_ops::wu32(rdx.wrapping_add(0xf0u64), (0xffffffffu32) as u32); crate::simd_ops::wu64(rdx.wrapping_add(0xf8u64), rax); crate::simd_ops::wu32(rdx.wrapping_add(0xf8u64), (0xffffffffu32) as u32); crate::simd_ops::lk_add_u32(rbx.wrapping_add(0x8u64), (0x20u32) as u32); crate::simd_ops::wu64(rdi, rdx); rax = rdi; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141307060(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141306e50; const P1: u64 = 0x141307be0; const P2: u64 = 0x14397ff48; const P3: u64 = 0x14397fed0; const P4: u64 = 0x14222edd0; const P5: u64 = 0x141307ff0; const P6: u64 = 0x141306970;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x10u64))) as u32 as u64; rbp = ((r9 as u32)) as u32 as u64; rdi = rdx; rbx = rcx; fl_l = ((r8 as u8)) as u8 as u64; fl_r = ((r8 as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 7; continue; } bb = 1; continue; }
            1 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = 0xffffffff; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 6; continue; } bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x1e8u64))) as u32 as u64; crate::simd_ops::lk_add_u64(rcx.wrapping_add(0x1e8u64), (1) as u64); crate::simd_ops::wu32(rdx.wrapping_add(0x10u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru64(rcx)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rsi); rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x38u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru64(rbx.wrapping_add(0x1e8u64))) as u64; fl_r = rax; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = 0x20; rcx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x30u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rsi = rbx.wrapping_add(0x1d8); rcx = rsi; rdx = rsp.wrapping_add(0x20); rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rcx = rbx.wrapping_add(0x3c0); rdx = 0x20; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x30u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 5; continue; }
            4 => { rsi = rbx.wrapping_add(0x1d8); bb = 5; continue; }
            5 => { rdx = rdi; rcx = rbx; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rsi)) as u64; rcx = (crate::simd_ops::ru32(rdi.wrapping_add(0x10u64))) as u32 as u64; crate::simd_ops::wu32(rax.wrapping_add(rcx.wrapping_mul(8)), ((rbp as u32)) as u32); crate::simd_ops::wu8(rax.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x4u64), (0x1u8) as u8); rcx = (crate::simd_ops::ru32(rdi.wrapping_add(0x8u64))) as u32 as u64; r8 = (crate::simd_ops::ru64(rdi)) as u64; rax = (crate::simd_ops::ru64(P2)) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; rdx = (crate::simd_ops::ru64(rax.wrapping_add(rcx.wrapping_mul(8)))) as u64; rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x10u64))) as u32 as u64; crate::simd_ops::wu32(rdx.wrapping_add(0x68u64), ((rax as u32)) as u32); rcx = (crate::simd_ops::ru64(r8.wrapping_add(0x8u64))) as u64; rax = (crate::simd_ops::ru64(P3)) as u64; rdx = (crate::simd_ops::ru32(rcx.wrapping_add(0x54u64))) as u32 as u64; rcx = ((rdx.wrapping_add(rdx.wrapping_mul(4))) as u64) as u64; rcx = ((rcx).wrapping_shl(5u32)) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rax.wrapping_add(0x120u64)))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(P4, rax, rcx, rdx, r8, r9, rsp); }
            6 => { r8 = ((crate::simd_ops::ru8(rsp.wrapping_add(0x68u64))) as u32) as u32 as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(P5, rax, rcx, rdx, r8, r9, rsp); }
            7 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = 0xffffffff; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 9; continue; } bb = 8; continue; }
            8 => { rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); bb = 9; continue; }
            9 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141307210(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0xc; const P2: u64 = 0x140a5b830; const P3: u64 = 0x10;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbp = r8; rdi = rdx; rsi = rcx; rbx = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(P1))) as u32 as u64; fl_l = rdx; fl_r = rax; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 5; continue; } bb = 1; continue; }
            1 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = rdi; fl_l = rdx; fl_r = rbx; rdx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 4; continue; } bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru32(rsi.wrapping_add(P0))) as u32 as u64; rax = ((rax).wrapping_shl(4u32)) as u64; fl_l = ((rax).wrapping_add(crate::simd_ops::ru64(rsi))) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; bb = 3; continue; }
            3 => { xmm0 = (crate::simd_ops::ru128(rbp)) as u128; crate::simd_ops::wu128(rax, (xmm0) as u128); rax = ((rax.wrapping_add(P3)) as u64) as u64; fl_l = rdx; fl_r = (0x1u64) as u64; rdx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } bb = 4; continue; }
            4 => { crate::simd_ops::wu32(rsi.wrapping_add(P0), ((rdi as u32)) as u32); bb = 12; continue; }
            5 => { fl_l = rdi; fl_r = rbx; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 10; continue; } bb = 6; continue; }
            6 => { rax = rdi; fl_l = rax; fl_r = rbx; rax = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 11; continue; } bb = 7; continue; }
            7 => { rbx = ((rbx).wrapping_shl(4u32)) as u64; fl_l = ((rbx).wrapping_add(crate::simd_ops::ru64(rcx))) as u64; fl_r = fl_l; rbx = ((fl_l as u64)) as u64; bb = 8; continue; }
            8 => { xmm0 = (crate::simd_ops::ru128(r8)) as u128; crate::simd_ops::wu128(rbx, (xmm0) as u128); rbx = ((rbx.wrapping_add(P3)) as u64) as u64; fl_l = rax; fl_r = (0x1u64) as u64; rax = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 8; continue; } bb = 9; continue; }
            9 => { bb = 11; continue; }
            10 => { if (((fl_l as u64)) == ((fl_r as u64))) { bb = 12; continue; } bb = 11; continue; }
            11 => { crate::simd_ops::wu32(rcx.wrapping_add(P0), ((rdi as u32)) as u32); bb = 12; continue; }
            12 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1413072b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0xc; const P2: u64 = 0x141247db0; const P3: u64 = 0x14183acc2;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rdx; rdi = rcx; rsi = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; rcx = ((rsi as u32)) as u32 as u64; rcx = ((rcx).wrapping_shl(6u32)) as u64; rax = (crate::simd_ops::ru32(rdi.wrapping_add(P1))) as u32 as u64; fl_l = rdx; fl_r = rax; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 3; continue; } bb = 1; continue; }
            1 => { rcx = rdi; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); r8 = rbx; fl_l = r8; fl_r = rsi; r8 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 7; continue; } bb = 2; continue; }
            2 => { rcx = (crate::simd_ops::ru32(rdi.wrapping_add(P0))) as u32 as u64; rcx = ((rcx).wrapping_shl(6u32)) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rdi))) as u64; r8 = ((r8).wrapping_shl(6u32)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu32(rdi.wrapping_add(P0), ((rbx as u32)) as u32); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            3 => { fl_l = rbx; fl_r = rsi; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 6; continue; } bb = 4; continue; }
            4 => { r8 = rbx; fl_l = r8; fl_r = rsi; r8 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 7; continue; } bb = 5; continue; }
            5 => { rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rdi))) as u64; r8 = ((r8).wrapping_shl(6u32)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu32(rdi.wrapping_add(P0), ((rbx as u32)) as u32); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            6 => { if (((fl_l as u64)) == ((fl_r as u64))) { bb = 8; continue; } bb = 7; continue; }
            7 => { crate::simd_ops::wu32(rdi.wrapping_add(P0), ((rbx as u32)) as u32); bb = 8; continue; }
            8 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141307360(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x141307420; const P2: u64 = 0xffffffffffffffff; const P3: u64 = 0x108; const P4: u64 = 0x4; const P5: u64 = 0x100; const P6: u64 = 0xe; const P7: u64 = 0x20; const P8: u64 = 0x18; const P9: u64 = 0x1425371a0; const P10: u64 = 0x142533200;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdx = (crate::simd_ops::ru64(rcx)) as u64; rbx = rcx; r8 = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; r8 = ((r8).wrapping_shl(6u32)) as u64; r8 = ((r8).wrapping_add(rdx)) as u64; fl_l = rdx; fl_r = r8; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 2; continue; }
            2 => { r9 = (crate::simd_ops::ru64(rbx)) as u64; fl_l = r9; fl_r = r9; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 3; continue; }
            3 => { rcx = ((r9.wrapping_add(P2)) as u64) as u64; rax = r9; rcx = (((rcx) & (0xfffffffffe000000u64))) as u64; rax = ((rax).wrapping_sub(rcx)) as u64; rax = ((rax).wrapping_shr(16u32)) as u64; rdx = ((rcx.wrapping_add(P3)) as u64) as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(P4))) as u32 as u64; rdx = ((rdx).wrapping_sub(rax)) as u64; rax = ((crate::exe::teb_self() as u64)) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(rcx.wrapping_add(P5))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 8; continue; } bb = 4; continue; }
            4 => { fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(P6))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 7; continue; } bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru64(rdx.wrapping_add(P7))) as u64; crate::simd_ops::wu64(r9, rax); fl_l = (crate::simd_ops::lk_add_u32(rdx.wrapping_add(P8), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; crate::simd_ops::wu64(rdx.wrapping_add(P7), r9); if (((fl_l as u32)) != 0) { bb = 10; continue; } bb = 6; continue; }
            6 => { rcx = rdx; rax = crate::simd_ops::call_stk(P9, rax, rcx, rdx, r8, r9, rsp); rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rbx, rax); crate::simd_ops::wu64(rbx.wrapping_add(P0), rax); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            7 => { r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 9; continue; }
            8 => { r8 = 0; fl_l = 0; fl_r = 0; bb = 9; continue; }
            9 => { rax = crate::simd_ops::call_stk(P10, rax, rcx, rdx, r8, r9, rsp); bb = 10; continue; }
            10 => { rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rbx, rax); crate::simd_ops::wu64(rbx.wrapping_add(P0), rax); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141307420(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x24; const P1: u64 = 0x28; const P2: u64 = 0x14241d6f0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { fl_l = rdx; fl_r = r8; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 8; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = r8; rbx = rdx; rsi = 0; fl_l = 0; fl_r = 0; bb = 2; continue; }
            2 => { rdx = (crate::simd_ops::ru64(rbx)) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 3; continue; }
            3 => { rax = 0xffffffff; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rdx.wrapping_add(P0), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 6; continue; } bb = 4; continue; }
            4 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P1))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 6; continue; }
            6 => { crate::simd_ops::wu64(rbx, rsi); rbx = ((rbx).wrapping_add(0x40u64)) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 7; continue; }
            7 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); bb = 8; continue; }
            8 => { return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141307490(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141305a20; const P1: u64 = 0x140fff0a0; const P2: u64 = 0x1422b3cd0; const P3: u64 = 0x14397ff48; const P4: u64 = 0x1402741e0;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0; let mut xmm4: u128 = 0; let mut xmm5: u128 = 0; let mut xmm6: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 23>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = rsp; crate::simd_ops::wu64(rax.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rax.wrapping_add(0x18u64), rbp); crate::simd_ops::wu64(rax.wrapping_add(0x20u64), rsi); crate::simd_ops::wu64(rax.wrapping_add(0x8u64), rcx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x90u64)) as u64; crate::simd_ops::wu128(rax.wrapping_sub(0x38u64), (xmm6) as u128); rdi = rcx.wrapping_add(0xd0); rax = (crate::simd_ops::ru64(rdx)) as u64; r13 = rcx; rcx = rdi; r14 = rdx; r8 = (crate::simd_ops::ru64(rax)) as u64; rax = (crate::simd_ops::ru64(rdi)) as u64; rbp = (((crate::simd_ops::ru32(r8.wrapping_add(0x30u64))) as i32 as i64) as u64) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x38u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = (crate::simd_ops::ru64(rdi)) as u64; rsi = r14.wrapping_add(0x10); rcx = rdi; rbx = rax; r8 = (crate::simd_ops::ru64(rdx.wrapping_add(0x40u64))) as u64; rdx = ((rbp as u32)) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(r8, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu32(rsi, ((rax as u32)) as u32); rcx = rdi; rdx = (crate::simd_ops::ru64(rdi)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rdx.wrapping_add(0x38u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = rbx; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(rdi)) as u64; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x38u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = rax; rcx = r13.wrapping_add(0x1c8); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rsi = r14.wrapping_add(0x10); bb = 2; continue; }
            2 => { rcx = (crate::simd_ops::ru64(r13.wrapping_add(0x188u64))) as u64; r9 = (r9 & !0xffu64) | (((0x1u8) as u8 as u64)); rbx = (crate::simd_ops::ru32(r14.wrapping_add(0x10u64))) as u32 as u64; r8 = rbp; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rcx); rdx = ((rbx as u32)) as u32 as u64; r15 = rbp; rax = rcx.wrapping_add(0x3f); rax = ((rax).wrapping_shr(6u32)) as u64; rcx = rsp.wrapping_add(0x20); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); rax = (crate::simd_ops::ru64(r13.wrapping_add(0x178u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru64(r13.wrapping_add(0x168u64))) as u64; rdi = 0; fl_l = 0; fl_r = 0; rbx = ((rbx).wrapping_shl(4u32)) as u64; fl_l = ((rbp as u32)) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 14; continue; } bb = 3; continue; }
            3 => { rbx = ((rbx).wrapping_add(0x4u64)) as u64; fl_l = ((rbx).wrapping_add(rax)) as u64; fl_r = fl_l; rbx = ((fl_l as u64)) as u64; bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru32(r14.wrapping_add(0x10u64))) as u32 as u64; rcx = (crate::simd_ops::ru64(r14)) as u64; rax = (((rax as u32)).wrapping_add((rdi as u32))) as u32 as u64; rbp = (crate::simd_ops::ru64(r13.wrapping_add(0x1c8u64))) as u64; rsi = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = ((((rdi as u32)) as i32 as i64) as u64) as u64; r8 = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = (crate::simd_ops::ru64(rax.wrapping_add(0x38u64))) as u64; let y_addr: u64 = rdx.wrapping_add(r8.wrapping_mul(8)); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); xmm6 = ((crate::simd_ops::ru64(rdx.wrapping_add(r8.wrapping_mul(8)).wrapping_add(0x20u64))) as u64) as u128; let y_addr: u64 = rsp.wrapping_add(0x20u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); rdx = ((xmm0 as u32)) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = ((crate::simd_ops::ru16(rax.wrapping_add(0x20u64))) as u32) as u32 as u64; rax = (crate::simd_ops::ru64(P3)) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x38u64), ((rcx as u32)) as u32); let y_addr: u64 = rsp.wrapping_add(0x20u64); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rbp.wrapping_add(rsi.wrapping_mul(8)); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); crate::simd_ops::wu64(rbp.wrapping_add(rsi.wrapping_mul(8)).wrapping_add(0x20u64), ((xmm6 as u64)) as u64); rcx = (crate::simd_ops::ru32(r14.wrapping_add(0x18u64))) as u32 as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; rdx = (((crate::simd_ops::ru32(rbp.wrapping_add(rsi.wrapping_mul(8)))) as i32 as i64) as u64) as u64; r10 = (crate::simd_ops::ru64(rax.wrapping_add(rcx.wrapping_mul(8)))) as u64; r9 = (crate::simd_ops::ru64(r10.wrapping_add(0x10u64))) as u64; fl_l = r9; fl_r = r9; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru64(r10.wrapping_add(0x8u64))) as u64; rcx = (((crate::simd_ops::ru32(rax.wrapping_add(0x138u64))) as i32 as i64) as u64) as u64; fl_l = rdx; fl_r = rcx; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 7; continue; } bb = 6; continue; }
            6 => { rax = rdx; rax = ((rax).wrapping_add(rax)) as u64; rcx = (crate::simd_ops::ru64(r9.wrapping_add(rax.wrapping_mul(8)).wrapping_add(0x8u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 12; continue; } bb = 7; continue; }
            7 => { rax = ((crate::simd_ops::ru8(r10.wrapping_add(0x4fu64))) as u32) as u32 as u64; r9 = (crate::simd_ops::ru64(r10.wrapping_add(0x8u64))) as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 10; continue; } bb = 8; continue; }
            8 => { fl_l = (crate::simd_ops::ru32(r9.wrapping_add(0x178u64))) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 10; continue; } bb = 9; continue; }
            9 => { rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = (crate::simd_ops::ru64(r9.wrapping_add(0x170u64))) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(rcx.wrapping_mul(8)).wrapping_sub(0x10u64))) as u64; rax = ((rcx.wrapping_add(rdx.wrapping_mul(8))) as u64) as u64; bb = 11; continue; }
            10 => { rax = (crate::simd_ops::ru64(r9.wrapping_add(0x160u64))) as u64; rax = ((rax.wrapping_add(rdx.wrapping_mul(8))) as u64) as u64; bb = 11; continue; }
            11 => { rax = (crate::simd_ops::ru64(rax)) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(0xc8u64))) as u64; bb = 12; continue; }
            12 => { rax = ((crate::simd_ops::ru16(rcx.wrapping_add(0x20u64))) as u32) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_add(rsi.wrapping_mul(8)).wrapping_add(0x18u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru64(r14)) as u64; rdx = ((((rdi as u32)) as i32 as i64) as u64) as u64; rdi = (((rdi as u32)).wrapping_add(1)) as u32 as u64; rdx = ((rdx).wrapping_add(rdx)) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(0x20u64))) as u64; xmm0 = (crate::simd_ops::ru128(rcx.wrapping_add(rdx.wrapping_mul(8)))) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rbx.wrapping_sub(0x4u64), (xmm0) as u128); rax = (crate::simd_ops::ru32(rbx)) as u32 as u64; rax = ((((rax as u32)) & (0xff000000u32))) as u32 as u64; rax = ((((rax as u32)) | (crate::simd_ops::ru32(rbp.wrapping_add(rsi.wrapping_mul(8)).wrapping_add(0x18u64))))) as u32 as u64; crate::simd_ops::wu32(rbx, ((rax as u32)) as u32); rbx = ((rbx).wrapping_add(0x10u64)) as u64; fl_l = r15; fl_r = (0x1u64) as u64; r15 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 4; continue; } bb = 13; continue; }
            13 => { r13 = (crate::simd_ops::ru64(rsp.wrapping_add(0xc0u64))) as u64; rsi = r14.wrapping_add(0x10); bb = 14; continue; }
            14 => { rax = (crate::simd_ops::ru64(r14)) as u64; rcx = rsp.wrapping_add(0x50); xmm3 = 0u128; xmm6 = ((crate::simd_ops::ru32(rax.wrapping_add(0x40u64))) as u32) as u128; xmm4 = ((crate::simd_ops::ru32(rax.wrapping_add(0x34u64))) as u32) as u128; xmm2 = ((crate::simd_ops::ru32(rax.wrapping_add(0x38u64))) as u32) as u128; xmm5 = ((crate::simd_ops::ru32(rax.wrapping_add(0x3cu64))) as u32) as u128; rbx = ((crate::simd_ops::ru8(rax.wrapping_add(0x30u64))) as u32) as u32 as u64; rax = (crate::simd_ops::ru32(r14.wrapping_add(0x18u64))) as u32 as u64; xmm0 = crate::simd_ops::vunpcklps(xmm3, xmm6); ymm0_hi = 0; rdx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = (crate::simd_ops::ru64(P3)) as u64; rdx = ((rdx).wrapping_shl(4u32)) as u64; rdx = ((rdx).wrapping_add(crate::simd_ops::ru64(rax.wrapping_add(0x5c8u64)))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x5cu64), ((xmm0 as u64)) as u64); xmm0 = crate::simd_ops::vunpcklps(xmm3, xmm3); ymm0_hi = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), ((xmm0 as u64)) as u64); xmm0 = crate::simd_ops::vunpcklps(xmm4, xmm2); ymm0_hi = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x74u64), ((xmm0 as u64)) as u64); crate::simd_ops::wu32(rsp.wrapping_add(0x50u64), ((xmm6 as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x54u64), ((xmm3 as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x58u64), ((xmm3 as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x64u64), ((xmm3 as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x70u64), ((xmm6 as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x7cu64), ((xmm5 as u32)) as u32); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r9 = (crate::simd_ops::ru32(r14.wrapping_add(0x14u64))) as u32 as u64; r8 = (crate::simd_ops::ru64(r13.wrapping_add(0xa8u64))) as u64; rcx = ((r9 as u32)) as u32 as u64; rdx = ((r9 as u32)) as u32 as u64; rcx = ((((rcx as u32)) & (0x3fu32))) as u32 as u64; rdx = ((rdx).wrapping_shr(6u32)) as u64; r9 = ((r9).wrapping_shl(6u32)) as u64; rax = (crate::simd_ops::ru64(r8.wrapping_add(rdx.wrapping_mul(8)))) as u64; fl_l = (((rax) >> (((rcx) as u32 & 63))) & 1) as u64; rax = ((rax) | ((1u64) << (((rcx) as u32 & 63)))) as u64; crate::simd_ops::wu64(r8.wrapping_add(rdx.wrapping_mul(8)), rax); rax = (crate::simd_ops::ru64(r14)) as u64; r9 = ((r9).wrapping_add(crate::simd_ops::ru64(r13.wrapping_add(0x98u64)))) as u64; rcx = (crate::simd_ops::ru64(rax)) as u64; fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(0xd0u64))) as u32 as u64; fl_r = 0x2; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 17; continue; } bb = 15; continue; }
            15 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x88u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 17; continue; } bb = 16; continue; }
            16 => { r10 = (crate::simd_ops::ru32(rcx.wrapping_add(0x90u64))) as u32 as u64; bb = 18; continue; }
            17 => { r10 = (crate::simd_ops::ru32(rcx.wrapping_add(0x70u64))) as u32 as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x68u64))) as u64; bb = 18; continue; }
            18 => { r8 = (crate::simd_ops::ru32(rsi)) as u32 as u64; rdx = rsp.wrapping_add(0x54); rdi = (crate::simd_ops::ru32(rcx.wrapping_add(0xdcu64))) as u32 as u64; rcx = 0x20; fl_l = ((r10).wrapping_add(crate::simd_ops::ru64(rax))) as u64; fl_r = fl_l; r10 = ((fl_l as u64)) as u64; r11 = ((r8.wrapping_add(r8.wrapping_mul(2))) as u32) as u32 as u64; bb = 19; continue; }
            19 => { rax = (crate::simd_ops::ru32(rdx.wrapping_sub(0x4u64))) as u32 as u64; rdx = rdx.wrapping_add(0xc); crate::simd_ops::wu32(r9.wrapping_add(rcx).wrapping_sub(0x20u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdx.wrapping_sub(0xcu64))) as u32 as u64; crate::simd_ops::wu32(r9.wrapping_add(rcx).wrapping_sub(0x10u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdx.wrapping_sub(0x8u64))) as u32 as u64; crate::simd_ops::wu32(r9.wrapping_add(rcx), ((rax as u32)) as u32); rcx = ((rcx).wrapping_add(0x4u64)) as u64; fl_l = rcx; fl_r = (0x30u64) as u64; if ((((fl_l as u64)) as i64) < (((fl_r as u64)) as i64)) { bb = 19; continue; } bb = 20; continue; }
            20 => { r8 = ((((r8 as u32)) ^ (crate::simd_ops::ru32(r9.wrapping_add(0x30u64))))) as u32 as u64; r11 = ((((r11 as u32)) ^ (crate::simd_ops::ru32(r9.wrapping_add(0x34u64))))) as u32 as u64; r8 = ((((r8 as u32)) & (0xffffffu32))) as u32 as u64; crate::simd_ops::lk_xor_u32(r9.wrapping_add(0x30u64), ((r8 as u32)) as u32); r11 = ((((r11 as u32)) & (0xffffffu32))) as u32 as u64; crate::simd_ops::lk_xor_u32(r9.wrapping_add(0x34u64), ((r11 as u32)) as u32); r11 = rsp.wrapping_add(0x90); rbp = (crate::simd_ops::ru64(r11.wrapping_add(0x40u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x48u64))) as u64; xmm6 = (crate::simd_ops::ru128(r11.wrapping_sub(0x10u64))) as u128; crate::simd_ops::wu8(r9.wrapping_add(0x37u64), ((rbx as u8)) as u8); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x38u64))) as u64; crate::simd_ops::wu8(r9.wrapping_add(0x33u64), ((rdi as u8)) as u8); crate::simd_ops::wu64(r9.wrapping_add(0x38u64), r10); r8 = (crate::simd_ops::ru32(r14.wrapping_add(0x14u64))) as u32 as u64; rax = (crate::simd_ops::ru64(r13.wrapping_add(0x458u64))) as u64; rcx = ((r8 as u32)) as u32 as u64; rcx = ((rcx).wrapping_shr(6u32)) as u64; crate::simd_ops::wu32(rax.wrapping_add(r8.wrapping_mul(4)), (0xffffffffu32) as u32); r8 = ((((r8 as u32)) & (0x3fu32))) as u32 as u64; rdx = (crate::simd_ops::ru64(r13.wrapping_add(0x468u64))) as u64; rax = (crate::simd_ops::ru64(rdx.wrapping_add(rcx.wrapping_mul(8)))) as u64; fl_l = (((rax) >> (((r8) as u32 & 63))) & 1) as u64; rax = ((rax) | ((1u64) << (((r8) as u32 & 63)))) as u64; crate::simd_ops::wu64(rdx.wrapping_add(rcx.wrapping_mul(8)), rax); rsp = r11; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141307860(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x140fff0a0; const P1: u64 = 0x1422b3cd0; const P2: u64 = 0x14397ff48; const P3: u64 = 0x1402741e0;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0; let mut xmm4: u128 = 0; let mut xmm5: u128 = 0; let mut xmm6: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 17>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { r11 = rsp; rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x68u64)) as u64; rax = (crate::simd_ops::ru64(rdx)) as u64; r14 = rdx; rbx = (crate::simd_ops::ru32(rdx.wrapping_add(0x10u64))) as u32 as u64; r13 = rcx; rdx = (crate::simd_ops::ru64(rcx.wrapping_add(0x188u64))) as u64; r9 = (r9 & !0xffu64) | (((0x1u8) as u8 as u64)); crate::simd_ops::wu64(r11.wrapping_sub(0x58u64), rdx); r8 = (crate::simd_ops::ru64(rax)) as u64; crate::simd_ops::wu64(r11.wrapping_add(0x10u64), rbp); rax = rdx.wrapping_add(0x3f); crate::simd_ops::wu128(rsp.wrapping_add(0x50u64), (xmm6) as u128); rax = ((rax).wrapping_shr(6u32)) as u64; rdx = ((rbx as u32)) as u32 as u64; rdi = (((crate::simd_ops::ru32(r8.wrapping_add(0x30u64))) as i32 as i64) as u64) as u64; crate::simd_ops::wu64(r11.wrapping_sub(0x68u64), rax); r8 = rdi; rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x178u64))) as u64; rcx = r11.wrapping_sub(0x68); crate::simd_ops::wu64(r11.wrapping_sub(0x60u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x90u64), rdi); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru64(r13.wrapping_add(0x168u64))) as u64; rbp = 0; fl_l = 0; fl_r = 0; rbx = ((rbx).wrapping_shl(4u32)) as u64; fl_l = rdi; fl_r = rdi; if (((((fl_l as u64)) & ((fl_r as u64))) as i64) <= 0) { bb = 12; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0xa0u64), rsi); rbx = ((rbx).wrapping_add(0x4u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xa8u64), r12); fl_l = ((rbx).wrapping_add(rax)) as u64; fl_r = fl_l; rbx = ((fl_l as u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), r15); r15 = ((rbp as u32)) as u32 as u64; bb = 2; continue; }
            2 => { rcx = (crate::simd_ops::ru32(r14.wrapping_add(0x10u64))) as u32 as u64; rsi = (crate::simd_ops::ru64(r13.wrapping_add(0x1c8u64))) as u64; rcx = (((rcx as u32)).wrapping_add((rbp as u32))) as u32 as u64; r12 = ((rbp as u32)) as u32 as u64; rdi = ((rcx.wrapping_add(rcx.wrapping_mul(4))) as u64) as u64; rcx = (crate::simd_ops::ru64(r14)) as u64; r8 = ((r12.wrapping_add(r12.wrapping_mul(4))) as u64) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = (crate::simd_ops::ru64(rax.wrapping_add(0x38u64))) as u64; let y_addr: u64 = rdx.wrapping_add(r8.wrapping_mul(8)); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); xmm6 = ((crate::simd_ops::ru64(rdx.wrapping_add(r8.wrapping_mul(8)).wrapping_add(0x20u64))) as u64) as u128; let y_addr: u64 = rsp.wrapping_add(0x20u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); rdx = ((xmm0 as u32)) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = ((crate::simd_ops::ru16(rax.wrapping_add(0x20u64))) as u32) as u32 as u64; rax = (crate::simd_ops::ru64(P2)) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x38u64), ((rcx as u32)) as u32); let y_addr: u64 = rsp.wrapping_add(0x20u64); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rsi.wrapping_add(rdi.wrapping_mul(8)); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); crate::simd_ops::wu64(rsi.wrapping_add(rdi.wrapping_mul(8)).wrapping_add(0x20u64), ((xmm6 as u64)) as u64); rcx = (crate::simd_ops::ru32(r14.wrapping_add(0x18u64))) as u32 as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; rdx = (((crate::simd_ops::ru32(rsi.wrapping_add(rdi.wrapping_mul(8)))) as i32 as i64) as u64) as u64; r10 = (crate::simd_ops::ru64(rax.wrapping_add(rcx.wrapping_mul(8)))) as u64; r9 = (crate::simd_ops::ru64(r10.wrapping_add(0x10u64))) as u64; fl_l = r9; fl_r = r9; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(r10.wrapping_add(0x8u64))) as u64; rcx = (((crate::simd_ops::ru32(rax.wrapping_add(0x138u64))) as i32 as i64) as u64) as u64; fl_l = rdx; fl_r = rcx; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = rdx; rax = ((rax).wrapping_add(rax)) as u64; rcx = (crate::simd_ops::ru64(r9.wrapping_add(rax.wrapping_mul(8)).wrapping_add(0x8u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 10; continue; } bb = 5; continue; }
            5 => { rax = ((crate::simd_ops::ru8(r10.wrapping_add(0x4fu64))) as u32) as u32 as u64; r9 = (crate::simd_ops::ru64(r10.wrapping_add(0x8u64))) as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 8; continue; } bb = 6; continue; }
            6 => { fl_l = (crate::simd_ops::ru32(r9.wrapping_add(0x178u64))) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 8; continue; } bb = 7; continue; }
            7 => { rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = (crate::simd_ops::ru64(r9.wrapping_add(0x170u64))) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(rcx.wrapping_mul(8)).wrapping_sub(0x10u64))) as u64; rax = ((rcx.wrapping_add(rdx.wrapping_mul(8))) as u64) as u64; bb = 9; continue; }
            8 => { rax = (crate::simd_ops::ru64(r9.wrapping_add(0x160u64))) as u64; rax = ((rax.wrapping_add(rdx.wrapping_mul(8))) as u64) as u64; bb = 9; continue; }
            9 => { rax = (crate::simd_ops::ru64(rax)) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(0xc8u64))) as u64; bb = 10; continue; }
            10 => { rax = ((crate::simd_ops::ru16(rcx.wrapping_add(0x20u64))) as u32) as u32 as u64; rdx = r12; crate::simd_ops::wu32(rsi.wrapping_add(rdi.wrapping_mul(8)).wrapping_add(0x18u64), ((rax as u32)) as u32); rdx = ((rdx).wrapping_add(rdx)) as u64; rax = (crate::simd_ops::ru64(r14)) as u64; rbp = (((rbp as u32)).wrapping_add(1)) as u32 as u64; r15 = ((r15).wrapping_add(1)) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(0x20u64))) as u64; xmm0 = (crate::simd_ops::ru128(rcx.wrapping_add(rdx.wrapping_mul(8)))) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rbx.wrapping_sub(0x4u64), (xmm0) as u128); rax = (crate::simd_ops::ru32(rbx)) as u32 as u64; rax = ((((rax as u32)) & (0xff000000u32))) as u32 as u64; rax = ((((rax as u32)) | (crate::simd_ops::ru32(rsi.wrapping_add(rdi.wrapping_mul(8)).wrapping_add(0x18u64))))) as u32 as u64; crate::simd_ops::wu32(rbx, ((rax as u32)) as u32); rbx = ((rbx).wrapping_add(0x10u64)) as u64; fl_l = r15; fl_r = (crate::simd_ops::ru64(rsp.wrapping_add(0x90u64))) as u64; if ((((fl_l as u64)) as i64) < (((fl_r as u64)) as i64)) { bb = 2; continue; } bb = 11; continue; }
            11 => { r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0xa8u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0xa0u64))) as u64; bb = 12; continue; }
            12 => { r9 = (crate::simd_ops::ru32(r14.wrapping_add(0x14u64))) as u32 as u64; rax = (crate::simd_ops::ru64(r14)) as u64; rcx = ((r9 as u32)) as u32 as u64; r8 = (crate::simd_ops::ru64(r13.wrapping_add(0xa8u64))) as u64; rdx = ((r9 as u32)) as u32 as u64; xmm3 = 0u128; rdx = ((rdx).wrapping_shr(6u32)) as u64; rcx = ((((rcx as u32)) & (0x3fu32))) as u32 as u64; rdi = ((crate::simd_ops::ru8(rax.wrapping_add(0x30u64))) as u32) as u32 as u64; r9 = ((r9).wrapping_shl(6u32)) as u64; rax = (crate::simd_ops::ru64(r8.wrapping_add(rdx.wrapping_mul(8)))) as u64; fl_l = (((rax) >> (((rcx) as u32 & 63))) & 1) as u64; rax = ((rax) | ((1u64) << (((rcx) as u32 & 63)))) as u64; crate::simd_ops::wu64(r8.wrapping_add(rdx.wrapping_mul(8)), rax); rcx = rsp.wrapping_add(0x20); rax = (crate::simd_ops::ru64(r14)) as u64; rbx = (crate::simd_ops::ru64(r13.wrapping_add(0x98u64))) as u64; rbx = ((rbx).wrapping_add(r9)) as u64; xmm6 = ((crate::simd_ops::ru32(rax.wrapping_add(0x40u64))) as u32) as u128; xmm4 = ((crate::simd_ops::ru32(rax.wrapping_add(0x34u64))) as u32) as u128; xmm2 = ((crate::simd_ops::ru32(rax.wrapping_add(0x38u64))) as u32) as u128; xmm5 = ((crate::simd_ops::ru32(rax.wrapping_add(0x3cu64))) as u32) as u128; rax = (crate::simd_ops::ru32(r14.wrapping_add(0x18u64))) as u32 as u64; xmm0 = crate::simd_ops::vunpcklps(xmm3, xmm6); ymm0_hi = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x2cu64), ((xmm0 as u64)) as u64); xmm0 = crate::simd_ops::vunpcklps(xmm3, xmm3); ymm0_hi = 0; rdx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = (crate::simd_ops::ru64(P2)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), ((xmm0 as u64)) as u64); xmm0 = crate::simd_ops::vunpcklps(xmm4, xmm2); ymm0_hi = 0; rdx = ((rdx).wrapping_shl(4u32)) as u64; rdx = ((rdx).wrapping_add(crate::simd_ops::ru64(rax.wrapping_add(0x5c8u64)))) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((xmm6 as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x24u64), ((xmm3 as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((xmm3 as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x34u64), ((xmm3 as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x40u64), ((xmm6 as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x4cu64), ((xmm5 as u32)) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x44u64), ((xmm0 as u64)) as u64); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru64(r14)) as u64; xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x50u64))) as u128; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x98u64))) as u64; rcx = (crate::simd_ops::ru64(rax)) as u64; fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(0xd0u64))) as u32 as u64; fl_r = 0x2; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 15; continue; } bb = 13; continue; }
            13 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x88u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 15; continue; } bb = 14; continue; }
            14 => { r9 = (crate::simd_ops::ru32(rcx.wrapping_add(0x90u64))) as u32 as u64; bb = 16; continue; }
            15 => { r9 = (crate::simd_ops::ru32(rcx.wrapping_add(0x70u64))) as u32 as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x68u64))) as u64; bb = 16; continue; }
            16 => { r8 = (crate::simd_ops::ru32(r14.wrapping_add(0x10u64))) as u32 as u64; rdx = rsp.wrapping_add(0x24); r11 = (crate::simd_ops::ru32(rcx.wrapping_add(0xdcu64))) as u32 as u64; rcx = 0x20; fl_l = ((r9).wrapping_add(crate::simd_ops::ru64(rax))) as u64; fl_r = fl_l; r9 = ((fl_l as u64)) as u64; r10 = ((r8.wrapping_add(r8.wrapping_mul(2))) as u32) as u32 as u64; bb = 17; continue; }
            17 => { rax = (crate::simd_ops::ru32(rdx.wrapping_sub(0x4u64))) as u32 as u64; rdx = rdx.wrapping_add(0xc); crate::simd_ops::wu32(rcx.wrapping_add(rbx).wrapping_sub(0x20u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdx.wrapping_sub(0xcu64))) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(rbx).wrapping_sub(0x10u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdx.wrapping_sub(0x8u64))) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(rbx), ((rax as u32)) as u32); rcx = ((rcx).wrapping_add(0x4u64)) as u64; fl_l = rcx; fl_r = (0x30u64) as u64; if ((((fl_l as u64)) as i64) < (((fl_r as u64)) as i64)) { bb = 17; continue; } bb = 18; continue; }
            18 => { r8 = ((((r8 as u32)) ^ (crate::simd_ops::ru32(rbx.wrapping_add(0x30u64))))) as u32 as u64; r10 = ((((r10 as u32)) ^ (crate::simd_ops::ru32(rbx.wrapping_add(0x34u64))))) as u32 as u64; r8 = ((((r8 as u32)) & (0xffffffu32))) as u32 as u64; crate::simd_ops::lk_xor_u32(rbx.wrapping_add(0x30u64), ((r8 as u32)) as u32); r10 = ((((r10 as u32)) & (0xffffffu32))) as u32 as u64; crate::simd_ops::wu8(rbx.wrapping_add(0x33u64), ((r11 as u8)) as u8); crate::simd_ops::lk_xor_u32(rbx.wrapping_add(0x34u64), ((r10 as u32)) as u32); crate::simd_ops::wu8(rbx.wrapping_add(0x37u64), ((rdi as u8)) as u8); crate::simd_ops::wu64(rbx.wrapping_add(0x38u64), r9); r8 = (crate::simd_ops::ru32(r14.wrapping_add(0x14u64))) as u32 as u64; rax = (crate::simd_ops::ru64(r13.wrapping_add(0x458u64))) as u64; rcx = ((r8 as u32)) as u32 as u64; rcx = ((rcx).wrapping_shr(6u32)) as u64; crate::simd_ops::wu32(rax.wrapping_add(r8.wrapping_mul(4)), (0xffffffffu32) as u32); r8 = ((((r8 as u32)) & (0x3fu32))) as u32 as u64; rdx = (crate::simd_ops::ru64(r13.wrapping_add(0x468u64))) as u64; rax = (crate::simd_ops::ru64(rdx.wrapping_add(rcx.wrapping_mul(8)))) as u64; fl_l = (((rax) >> (((r8) as u32 & 63))) & 1) as u64; rax = ((rax) | ((1u64) << (((r8) as u32 & 63)))) as u64; crate::simd_ops::wu64(rdx.wrapping_add(rcx.wrapping_mul(8)), rax); rsp = ((rsp).wrapping_add(0x68u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141307be0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141305a20; const P1: u64 = 0x140fff0a0; const P2: u64 = 0x14397ff48; const P3: u64 = 0x1434268d0; const P4: u64 = 0x143a001e8; const P5: u64 = 0x1402741e0;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0; let mut xmm4: u128 = 0; let mut xmm5: u128 = 0; let mut xmm6: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 21>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x80u64)) as u64; rax = (crate::simd_ops::ru64(rdx)) as u64; rsi = rcx.wrapping_add(0xd0); rbp = rcx; crate::simd_ops::wu128(rsp.wrapping_add(0x70u64), (xmm6) as u128); rcx = rsi; rdi = rdx; r14 = ((crate::simd_ops::ru16(rax.wrapping_add(0x20u64))) as u32) as u32 as u64; rax = (crate::simd_ops::ru64(rsi)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x38u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r8 = (crate::simd_ops::ru64(rsi)) as u64; rdx = ((r14 as u32)) as u32 as u64; rcx = rsi; rbx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(r8.wrapping_add(0x40u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu32(rdi.wrapping_add(0xcu64), ((rax as u32)) as u32); rcx = rsi; rdx = (crate::simd_ops::ru64(rsi)) as u64; r13 = rbp.wrapping_add(0x1c8); (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rdx.wrapping_add(0x38u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = rbx; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(rsi)) as u64; rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x38u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = rax; rcx = r13; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 2; continue; }
            2 => { rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x188u64))) as u64; r9 = (r9 & !0xffu64) | (((0x1u8) as u8 as u64)); rbx = (crate::simd_ops::ru32(rdi.wrapping_add(0xcu64))) as u32 as u64; r8 = r14; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rcx); rdx = ((rbx as u32)) as u32 as u64; r12 = r14; rax = rcx.wrapping_add(0x3f); rax = ((rax).wrapping_shr(6u32)) as u64; rcx = rsp.wrapping_add(0x20); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x178u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x168u64))) as u64; rsi = 0; fl_l = 0; fl_r = 0; rbx = ((rbx).wrapping_shl(4u32)) as u64; r15 = ((rsi as u32)) as u32 as u64; fl_l = ((r14 as u32)) as u32 as u64; fl_r = ((r14 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 14; continue; } bb = 3; continue; }
            3 => { rbx = ((rbx).wrapping_add(0x4u64)) as u64; rbp = ((rsi as u32)) as u32 as u64; rsi = (crate::simd_ops::ru64(P2)) as u64; fl_l = ((rbx).wrapping_add(rax)) as u64; fl_r = fl_l; rbx = ((fl_l as u64)) as u64; bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru32(rdi.wrapping_add(0xcu64))) as u32 as u64; r11 = (crate::simd_ops::ru64(r13)) as u64; rax = (((rax as u32)).wrapping_add((r15 as u32))) as u32 as u64; r10 = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = (crate::simd_ops::ru64(rdi)) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; let y_addr: u64 = rcx.wrapping_add(rbp).wrapping_add(0x8u64); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = r11.wrapping_add(r10.wrapping_mul(8)); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); xmm1 = ((crate::simd_ops::ru64(rcx.wrapping_add(rbp).wrapping_add(0x28u64))) as u64) as u128; crate::simd_ops::wu64(r11.wrapping_add(r10.wrapping_mul(8)).wrapping_add(0x20u64), ((xmm1 as u64)) as u64); rcx = (crate::simd_ops::ru32(rdi.wrapping_add(0x8u64))) as u32 as u64; rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x18u64))) as u64; rdx = (((crate::simd_ops::ru32(r11.wrapping_add(r10.wrapping_mul(8)))) as i32 as i64) as u64) as u64; r14 = (crate::simd_ops::ru64(rax.wrapping_add(rcx.wrapping_mul(8)))) as u64; r9 = (crate::simd_ops::ru64(r14.wrapping_add(0x10u64))) as u64; fl_l = r9; fl_r = r9; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru64(r14.wrapping_add(0x8u64))) as u64; rcx = (((crate::simd_ops::ru32(rax.wrapping_add(0x138u64))) as i32 as i64) as u64) as u64; fl_l = rdx; fl_r = rcx; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 7; continue; } bb = 6; continue; }
            6 => { rax = rdx; rax = ((rax).wrapping_add(rax)) as u64; rcx = (crate::simd_ops::ru64(r9.wrapping_add(rax.wrapping_mul(8)).wrapping_add(0x8u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 12; continue; } bb = 7; continue; }
            7 => { rax = ((crate::simd_ops::ru8(r14.wrapping_add(0x4fu64))) as u32) as u32 as u64; r9 = (crate::simd_ops::ru64(r14.wrapping_add(0x8u64))) as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 10; continue; } bb = 8; continue; }
            8 => { fl_l = (crate::simd_ops::ru32(r9.wrapping_add(0x178u64))) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 10; continue; } bb = 9; continue; }
            9 => { rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = (crate::simd_ops::ru64(r9.wrapping_add(0x170u64))) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(rcx.wrapping_mul(8)).wrapping_sub(0x10u64))) as u64; rax = ((rcx.wrapping_add(rdx.wrapping_mul(8))) as u64) as u64; bb = 11; continue; }
            10 => { rax = (crate::simd_ops::ru64(r9.wrapping_add(0x160u64))) as u64; rax = ((rax.wrapping_add(rdx.wrapping_mul(8))) as u64) as u64; bb = 11; continue; }
            11 => { rax = (crate::simd_ops::ru64(rax)) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(0xc8u64))) as u64; bb = 12; continue; }
            12 => { rax = ((crate::simd_ops::ru16(rcx.wrapping_add(0x20u64))) as u32) as u32 as u64; r15 = (((r15 as u32)).wrapping_add(1)) as u32 as u64; crate::simd_ops::wu32(r11.wrapping_add(r10.wrapping_mul(8)).wrapping_add(0x18u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru64(rdi)) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; xmm0 = (crate::simd_ops::ru128(rcx.wrapping_add(rbp).wrapping_add(0x30u64))) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rbx.wrapping_sub(0x4u64), (xmm0) as u128); rax = (crate::simd_ops::ru32(rbx)) as u32 as u64; rbp = ((rbp).wrapping_add(0xd0u64)) as u64; rax = ((((rax as u32)) & (0xff000000u32))) as u32 as u64; rax = ((((rax as u32)) | (crate::simd_ops::ru32(r11.wrapping_add(r10.wrapping_mul(8)).wrapping_add(0x18u64))))) as u32 as u64; crate::simd_ops::wu32(rbx, ((rax as u32)) as u32); rbx = ((rbx).wrapping_add(0x10u64)) as u64; fl_l = r12; fl_r = (0x1u64) as u64; r12 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 4; continue; } bb = 13; continue; }
            13 => { rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0xb0u64))) as u64; rsi = 0; fl_l = 0; fl_r = 0; bb = 14; continue; }
            14 => { rdx = (crate::simd_ops::ru64(rdi)) as u64; xmm3 = 0u128; r8 = (crate::simd_ops::ru64(rdx.wrapping_add(0x8u64))) as u64; rbx = ((crate::simd_ops::ru8(rdx.wrapping_add(0x26u64))) as u32) as u32 as u64; rax = (crate::simd_ops::ru64(r8.wrapping_add(0x18u64))) as u64; rax = ((rax).wrapping_sub(1)) as u64; fl_l = rax; fl_r = (0xfffffffffffffffdu64) as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 16; continue; } bb = 15; continue; }
            15 => { xmm2 = ((crate::simd_ops::ru32(P3)) as u32) as u128; xmm6 = 0u128; xmm4 = 0u128; xmm5 = 0u128; bb = 22; continue; }
            16 => { rcx = ((crate::simd_ops::ru16(r8.wrapping_add(0x2au64))) as u32) as u32 as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 20; continue; } bb = 17; continue; }
            17 => { rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x10u64))) as u64; rdx = (crate::simd_ops::ru32(rax.wrapping_add(0x8u64))) as u32 as u64; rax = (crate::simd_ops::ru64(r8.wrapping_add(0x8u64))) as u64; bb = 18; continue; }
            18 => { fl_l = (crate::simd_ops::ru32(rax)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 25; continue; } bb = 19; continue; }
            19 => { rsi = ((rsi).wrapping_add(1)) as u64; rax = ((rax).wrapping_add(0x68u64)) as u64; fl_l = rsi; fl_r = rcx; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 18; continue; } bb = 20; continue; }
            20 => { rcx = 0xffffffff; bb = 21; continue; }
            21 => { rax = (crate::simd_ops::ru64(P4)) as u64; rdx = ((rcx as u32)) as u32 as u64; fl_l = ((rdx).wrapping_shl(5u32)) as u64; rdx = ((fl_l as u64)) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(0x3d8u64))) as u64; let y_addr: u64 = rdx.wrapping_add(rcx); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); xmm2 = (ymm0_hi) as u128; xmm4 = (ymm0_hi) as u128; xmm5 = (ymm0_hi) as u128; xmm2 = crate::simd_ops::vshufps(xmm2, xmm2, 0xffu8); xmm4 = crate::simd_ops::vshufps(xmm4, xmm4, 0x55u8); xmm5 = crate::simd_ops::vshufps(xmm5, xmm5, 0xaau8); xmm6 = (ymm0_hi) as u128; bb = 22; continue; }
            22 => { rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x8u64))) as u32 as u64; rcx = rsp.wrapping_add(0x40); xmm0 = crate::simd_ops::vunpcklps(xmm3, xmm2); ymm0_hi = 0; rdx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = (crate::simd_ops::ru64(P2)) as u64; rdx = ((rdx).wrapping_shl(4u32)) as u64; rdx = ((rdx).wrapping_add(crate::simd_ops::ru64(rax.wrapping_add(0x5c8u64)))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x4cu64), ((xmm0 as u64)) as u64); xmm0 = crate::simd_ops::vunpcklps(xmm3, xmm3); ymm0_hi = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), ((xmm0 as u64)) as u64); xmm0 = crate::simd_ops::vunpcklps(xmm6, xmm4); ymm0_hi = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x64u64), ((xmm0 as u64)) as u64); crate::simd_ops::wu32(rsp.wrapping_add(0x40u64), ((xmm2 as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x44u64), ((xmm3 as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x48u64), ((xmm3 as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x54u64), ((xmm3 as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x60u64), ((xmm2 as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x6cu64), ((xmm5 as u32)) as u32); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r9 = (crate::simd_ops::ru32(rdi.wrapping_add(0x10u64))) as u32 as u64; r8 = (crate::simd_ops::ru64(rbp.wrapping_add(0xa8u64))) as u64; rcx = ((r9 as u32)) as u32 as u64; rcx = ((((rcx as u32)) & (0x3fu32))) as u32 as u64; rdx = ((r9 as u32)) as u32 as u64; rdx = ((rdx).wrapping_shr(6u32)) as u64; r9 = ((r9).wrapping_shl(6u32)) as u64; rax = (crate::simd_ops::ru64(r8.wrapping_add(rdx.wrapping_mul(8)))) as u64; fl_l = (((rax) >> (((rcx) as u32 & 63))) & 1) as u64; rax = ((rax) | ((1u64) << (((rcx) as u32 & 63)))) as u64; crate::simd_ops::wu64(r8.wrapping_add(rdx.wrapping_mul(8)), rax); rdx = rsp.wrapping_add(0x44); rcx = (crate::simd_ops::ru64(rdi)) as u64; r8 = (crate::simd_ops::ru32(rdi.wrapping_add(0xcu64))) as u32 as u64; r9 = ((r9).wrapping_add(crate::simd_ops::ru64(rbp.wrapping_add(0x98u64)))) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x50u64))) as u64; r11 = (crate::simd_ops::ru32(rcx.wrapping_add(0x60u64))) as u32 as u64; r10 = ((r8.wrapping_add(r8.wrapping_mul(2))) as u32) as u32 as u64; rsi = (crate::simd_ops::ru32(rcx.wrapping_add(0x18u64))) as u32 as u64; rcx = 0x20; fl_l = ((r11).wrapping_add(crate::simd_ops::ru64(rax))) as u64; fl_r = fl_l; r11 = ((fl_l as u64)) as u64; bb = 23; continue; }
            23 => { rax = (crate::simd_ops::ru32(rdx.wrapping_sub(0x4u64))) as u32 as u64; rdx = rdx.wrapping_add(0xc); crate::simd_ops::wu32(rcx.wrapping_add(r9).wrapping_sub(0x20u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdx.wrapping_sub(0xcu64))) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(r9).wrapping_sub(0x10u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdx.wrapping_sub(0x8u64))) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(r9), ((rax as u32)) as u32); rcx = ((rcx).wrapping_add(0x4u64)) as u64; fl_l = rcx; fl_r = (0x30u64) as u64; if ((((fl_l as u64)) as i64) < (((fl_r as u64)) as i64)) { bb = 23; continue; } bb = 24; continue; }
            24 => { r8 = ((((r8 as u32)) ^ (crate::simd_ops::ru32(r9.wrapping_add(0x30u64))))) as u32 as u64; r10 = ((((r10 as u32)) ^ (crate::simd_ops::ru32(r9.wrapping_add(0x34u64))))) as u32 as u64; r8 = ((((r8 as u32)) & (0xffffffu32))) as u32 as u64; crate::simd_ops::lk_xor_u32(r9.wrapping_add(0x30u64), ((r8 as u32)) as u32); r10 = ((((r10 as u32)) & (0xffffffu32))) as u32 as u64; crate::simd_ops::lk_xor_u32(r9.wrapping_add(0x34u64), ((r10 as u32)) as u32); xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x70u64))) as u128; crate::simd_ops::wu8(r9.wrapping_add(0x33u64), ((rsi as u8)) as u8); crate::simd_ops::wu8(r9.wrapping_add(0x37u64), ((rbx as u8)) as u8); crate::simd_ops::wu64(r9.wrapping_add(0x38u64), r11); r11 = rsp.wrapping_add(0x80); r8 = (crate::simd_ops::ru32(rdi.wrapping_add(0x10u64))) as u32 as u64; rax = (crate::simd_ops::ru64(rdi)) as u64; rdx = (crate::simd_ops::ru64(rbp.wrapping_add(0x458u64))) as u64; rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x48u64))) as u64; rcx = (crate::simd_ops::ru32(rax.wrapping_add(0x70u64))) as u32 as u64; crate::simd_ops::wu32(rdx.wrapping_add(r8.wrapping_mul(4)), ((rcx as u32)) as u32); rcx = ((r8 as u32)) as u32 as u64; rdx = (crate::simd_ops::ru64(rbp.wrapping_add(0x468u64))) as u64; r8 = ((((r8 as u32)) & (0x3fu32))) as u32 as u64; rbp = (crate::simd_ops::ru64(r11.wrapping_add(0x40u64))) as u64; rcx = ((rcx).wrapping_shr(6u32)) as u64; rax = (crate::simd_ops::ru64(rdx.wrapping_add(rcx.wrapping_mul(8)))) as u64; fl_l = (((rax) >> (((r8) as u32 & 63))) & 1) as u64; rax = ((rax) | ((1u64) << (((r8) as u32 & 63)))) as u64; crate::simd_ops::wu64(rdx.wrapping_add(rcx.wrapping_mul(8)), rax); rsp = r11; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            25 => { rcx = (crate::simd_ops::ru32(rax.wrapping_add(0x4u64))) as u32 as u64; bb = 21; continue; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141307ff0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x140fff0a0; const P1: u64 = 0x14397ff48; const P2: u64 = 0x1434268d0; const P3: u64 = 0x143a001e8; const P4: u64 = 0x1402741e0;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0; let mut xmm4: u128 = 0; let mut xmm5: u128 = 0; let mut xmm6: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 19>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu8(rsp.wrapping_add(0x18u64), ((r8 as u8)) as u8); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = ((rsp).wrapping_sub(0x78u64)) as u64; r8 = (crate::simd_ops::ru64(rcx.wrapping_add(0x188u64))) as u64; rdi = rdx; rax = (crate::simd_ops::ru64(rdx)) as u64; r13 = rcx; rbx = (crate::simd_ops::ru32(rdx.wrapping_add(0xcu64))) as u32 as u64; r9 = (r9 & !0xffu64) | (((0x1u8) as u8 as u64)); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), r8); rdx = ((rbx as u32)) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), r12); rsi = ((crate::simd_ops::ru16(rax.wrapping_add(0x20u64))) as u32) as u32 as u64; rax = r8.wrapping_add(0x3f); rax = ((rax).wrapping_shr(6u32)) as u64; r8 = ((rsi as u32)) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x178u64))) as u64; rcx = rsp.wrapping_add(0x20); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru64(r13.wrapping_add(0x168u64))) as u64; r10 = 0; fl_l = 0; fl_r = 0; rbx = ((rbx).wrapping_shl(4u32)) as u64; r12 = ((r10 as u32)) as u32 as u64; fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0xa8u64), rbp); r11 = rax.wrapping_add(0x4); crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), r14); rbp = ((r10 as u32)) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), r15); fl_l = ((r11).wrapping_add(rbx)) as u64; fl_r = fl_l; r11 = ((fl_l as u64)) as u64; r15 = ((r10 as u32)) as u32 as u64; r10 = ((rsi as u32)) as u32 as u64; bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru32(rdi.wrapping_add(0xcu64))) as u32 as u64; rsi = (crate::simd_ops::ru64(r13.wrapping_add(0x1c8u64))) as u64; rax = (((rax as u32)).wrapping_add((r12 as u32))) as u32 as u64; rbx = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = (crate::simd_ops::ru64(rdi)) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; rax = (crate::simd_ops::ru64(P1)) as u64; let y_addr: u64 = rcx.wrapping_add(rbp).wrapping_add(0x8u64); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rsi.wrapping_add(rbx.wrapping_mul(8)); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); xmm1 = ((crate::simd_ops::ru64(rcx.wrapping_add(rbp).wrapping_add(0x28u64))) as u64) as u128; crate::simd_ops::wu64(rsi.wrapping_add(rbx.wrapping_mul(8)).wrapping_add(0x20u64), ((xmm1 as u64)) as u64); rcx = (crate::simd_ops::ru32(rdi.wrapping_add(0x8u64))) as u32 as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; rdx = (((crate::simd_ops::ru32(rsi.wrapping_add(rbx.wrapping_mul(8)))) as i32 as i64) as u64) as u64; r14 = (crate::simd_ops::ru64(rax.wrapping_add(rcx.wrapping_mul(8)))) as u64; r9 = (crate::simd_ops::ru64(r14.wrapping_add(0x10u64))) as u64; fl_l = r9; fl_r = r9; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(r14.wrapping_add(0x8u64))) as u64; rcx = (((crate::simd_ops::ru32(rax.wrapping_add(0x138u64))) as i32 as i64) as u64) as u64; fl_l = rdx; fl_r = rcx; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = rdx; rax = ((rax).wrapping_add(rax)) as u64; rcx = (crate::simd_ops::ru64(r9.wrapping_add(rax.wrapping_mul(8)).wrapping_add(0x8u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 10; continue; } bb = 5; continue; }
            5 => { rax = ((crate::simd_ops::ru8(r14.wrapping_add(0x4fu64))) as u32) as u32 as u64; r9 = (crate::simd_ops::ru64(r14.wrapping_add(0x8u64))) as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 8; continue; } bb = 6; continue; }
            6 => { fl_l = (crate::simd_ops::ru32(r9.wrapping_add(0x178u64))) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 8; continue; } bb = 7; continue; }
            7 => { rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = (crate::simd_ops::ru64(r9.wrapping_add(0x170u64))) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(rcx.wrapping_mul(8)).wrapping_sub(0x10u64))) as u64; rax = ((rcx.wrapping_add(rdx.wrapping_mul(8))) as u64) as u64; bb = 9; continue; }
            8 => { rax = (crate::simd_ops::ru64(r9.wrapping_add(0x160u64))) as u64; rax = ((rax.wrapping_add(rdx.wrapping_mul(8))) as u64) as u64; bb = 9; continue; }
            9 => { rax = (crate::simd_ops::ru64(rax)) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(0xc8u64))) as u64; bb = 10; continue; }
            10 => { rax = ((crate::simd_ops::ru16(rcx.wrapping_add(0x20u64))) as u32) as u32 as u64; r12 = (((r12 as u32)).wrapping_add(1)) as u32 as u64; crate::simd_ops::wu32(rsi.wrapping_add(rbx.wrapping_mul(8)).wrapping_add(0x18u64), ((rax as u32)) as u32); r15 = ((r15).wrapping_add(1)) as u64; rax = (crate::simd_ops::ru64(rdi)) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; xmm0 = (crate::simd_ops::ru128(rcx.wrapping_add(rbp).wrapping_add(0x30u64))) as u128; ymm0_hi = 0; crate::simd_ops::wu128(r11.wrapping_sub(0x4u64), (xmm0) as u128); rax = (crate::simd_ops::ru32(r11)) as u32 as u64; rbp = ((rbp).wrapping_add(0xd0u64)) as u64; rax = ((((rax as u32)) & (0xff000000u32))) as u32 as u64; rax = ((((rax as u32)) | (crate::simd_ops::ru32(rsi.wrapping_add(rbx.wrapping_mul(8)).wrapping_add(0x18u64))))) as u32 as u64; crate::simd_ops::wu32(r11, ((rax as u32)) as u32); r11 = ((r11).wrapping_add(0x10u64)) as u64; fl_l = r15; fl_r = r10; if ((((fl_l as u64)) as i64) < (((fl_r as u64)) as i64)) { bb = 2; continue; } bb = 11; continue; }
            11 => { r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; r10 = 0; fl_l = 0; fl_r = 0; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0xa8u64))) as u64; bb = 12; continue; }
            12 => { r9 = (crate::simd_ops::ru32(rdi.wrapping_add(0x10u64))) as u32 as u64; rax = (crate::simd_ops::ru64(rdi)) as u64; rcx = ((r9 as u32)) as u32 as u64; r8 = (crate::simd_ops::ru64(r13.wrapping_add(0xa8u64))) as u64; rdx = ((r9 as u32)) as u32 as u64; r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rcx = ((((rcx as u32)) & (0x3fu32))) as u32 as u64; rdx = ((rdx).wrapping_shr(6u32)) as u64; rsi = ((crate::simd_ops::ru8(rax.wrapping_add(0x26u64))) as u32) as u32 as u64; r9 = ((r9).wrapping_shl(6u32)) as u64; rax = (crate::simd_ops::ru64(r8.wrapping_add(rdx.wrapping_mul(8)))) as u64; fl_l = (((rax) >> (((rcx) as u32 & 63))) & 1) as u64; rax = ((rax) | ((1u64) << (((rcx) as u32 & 63)))) as u64; crate::simd_ops::wu64(r8.wrapping_add(rdx.wrapping_mul(8)), rax); rbx = (crate::simd_ops::ru64(r13.wrapping_add(0x98u64))) as u64; rdx = (crate::simd_ops::ru64(rdi)) as u64; rbx = ((rbx).wrapping_add(r9)) as u64; fl_l = (crate::simd_ops::ru8(rsp.wrapping_add(0xb0u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 25; continue; } bb = 13; continue; }
            13 => { r8 = (crate::simd_ops::ru64(rdx.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu128(rsp.wrapping_add(0x50u64), (xmm6) as u128); xmm3 = 0u128; rax = (crate::simd_ops::ru64(r8.wrapping_add(0x18u64))) as u64; rax = ((rax).wrapping_sub(1)) as u64; fl_l = rax; fl_r = (0xfffffffffffffffdu64) as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 15; continue; } bb = 14; continue; }
            14 => { xmm2 = ((crate::simd_ops::ru32(P2)) as u32) as u128; xmm6 = 0u128; xmm4 = 0u128; xmm5 = 0u128; bb = 21; continue; }
            15 => { rcx = ((crate::simd_ops::ru16(r8.wrapping_add(0x2au64))) as u32) as u32 as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 19; continue; } bb = 16; continue; }
            16 => { rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x10u64))) as u64; rdx = (crate::simd_ops::ru32(rax.wrapping_add(0x8u64))) as u32 as u64; rax = (crate::simd_ops::ru64(r8.wrapping_add(0x8u64))) as u64; bb = 17; continue; }
            17 => { fl_l = (crate::simd_ops::ru32(rax)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 24; continue; } bb = 18; continue; }
            18 => { r10 = ((r10).wrapping_add(1)) as u64; rax = ((rax).wrapping_add(0x68u64)) as u64; fl_l = r10; fl_r = rcx; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 17; continue; } bb = 19; continue; }
            19 => { rcx = 0xffffffff; bb = 20; continue; }
            20 => { rax = (crate::simd_ops::ru64(P3)) as u64; rdx = ((rcx as u32)) as u32 as u64; fl_l = ((rdx).wrapping_shl(5u32)) as u64; rdx = ((fl_l as u64)) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(0x3d8u64))) as u64; let y_addr: u64 = rdx.wrapping_add(rcx); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); xmm2 = (ymm0_hi) as u128; xmm4 = (ymm0_hi) as u128; xmm5 = (ymm0_hi) as u128; xmm2 = crate::simd_ops::vshufps(xmm2, xmm2, 0xffu8); xmm4 = crate::simd_ops::vshufps(xmm4, xmm4, 0x55u8); xmm5 = crate::simd_ops::vshufps(xmm5, xmm5, 0xaau8); xmm6 = (ymm0_hi) as u128; bb = 21; continue; }
            21 => { rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x8u64))) as u32 as u64; rcx = rsp.wrapping_add(0x20); xmm0 = crate::simd_ops::vunpcklps(xmm3, xmm2); ymm0_hi = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x2cu64), ((xmm0 as u64)) as u64); xmm0 = crate::simd_ops::vunpcklps(xmm3, xmm3); ymm0_hi = 0; rdx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = (crate::simd_ops::ru64(P1)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), ((xmm0 as u64)) as u64); xmm0 = crate::simd_ops::vunpcklps(xmm6, xmm4); ymm0_hi = 0; rdx = ((rdx).wrapping_shl(4u32)) as u64; rdx = ((rdx).wrapping_add(crate::simd_ops::ru64(rax.wrapping_add(0x5c8u64)))) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((xmm2 as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x24u64), ((xmm3 as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((xmm3 as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x34u64), ((xmm3 as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x40u64), ((xmm2 as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x4cu64), ((xmm5 as u32)) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x44u64), ((xmm0 as u64)) as u64); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru64(rdi)) as u64; rdx = rsp.wrapping_add(0x24); r8 = (crate::simd_ops::ru32(rdi.wrapping_add(0xcu64))) as u32 as u64; xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x50u64))) as u128; rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x50u64))) as u64; r11 = (crate::simd_ops::ru32(rcx.wrapping_add(0x60u64))) as u32 as u64; r9 = ((r8.wrapping_add(r8.wrapping_mul(2))) as u32) as u32 as u64; r10 = (crate::simd_ops::ru32(rcx.wrapping_add(0x18u64))) as u32 as u64; rcx = 0x20; fl_l = ((r11).wrapping_add(crate::simd_ops::ru64(rax))) as u64; fl_r = fl_l; r11 = ((fl_l as u64)) as u64; bb = 22; continue; }
            22 => { rax = (crate::simd_ops::ru32(rdx.wrapping_sub(0x4u64))) as u32 as u64; rdx = rdx.wrapping_add(0xc); crate::simd_ops::wu32(rcx.wrapping_add(rbx).wrapping_sub(0x20u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdx.wrapping_sub(0xcu64))) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(rbx).wrapping_sub(0x10u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdx.wrapping_sub(0x8u64))) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(rbx), ((rax as u32)) as u32); rcx = ((rcx).wrapping_add(0x4u64)) as u64; fl_l = rcx; fl_r = (0x30u64) as u64; if ((((fl_l as u64)) as i64) < (((fl_r as u64)) as i64)) { bb = 22; continue; } bb = 23; continue; }
            23 => { r8 = ((((r8 as u32)) ^ (crate::simd_ops::ru32(rbx.wrapping_add(0x30u64))))) as u32 as u64; rcx = (crate::simd_ops::ru32(rbx.wrapping_add(0x34u64))) as u32 as u64; r8 = ((((r8 as u32)) & (0xffffffu32))) as u32 as u64; crate::simd_ops::lk_xor_u32(rbx.wrapping_add(0x30u64), ((r8 as u32)) as u32); fl_l = ((((rcx as u32)) ^ ((r9 as u32)))) as u32 as u64; fl_r = fl_l; rcx = ((fl_l as u32)) as u32 as u64; crate::simd_ops::wu8(rbx.wrapping_add(0x33u64), ((r10 as u8)) as u8); bb = 26; continue; }
            24 => { rcx = (crate::simd_ops::ru32(rax.wrapping_add(0x4u64))) as u32 as u64; bb = 20; continue; }
            25 => { rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x50u64))) as u64; r10 = (crate::simd_ops::ru32(rdi.wrapping_add(0xcu64))) as u32 as u64; r11 = (crate::simd_ops::ru32(rdx.wrapping_add(0x60u64))) as u32 as u64; rcx = (crate::simd_ops::ru32(rbx.wrapping_add(0x30u64))) as u32 as u64; r9 = (crate::simd_ops::ru64(rax)) as u64; rcx = ((((rcx as u32)) ^ ((r10 as u32)))) as u32 as u64; rdx = (crate::simd_ops::ru32(rdx.wrapping_add(0x18u64))) as u32 as u64; rcx = ((((rcx as u32)) & (0xffffffu32))) as u32 as u64; crate::simd_ops::lk_xor_u32(rbx.wrapping_add(0x30u64), ((rcx as u32)) as u32); r11 = ((r11).wrapping_add(r9)) as u64; rcx = ((r10.wrapping_add(r10.wrapping_mul(2))) as u32) as u32 as u64; crate::simd_ops::wu8(rbx.wrapping_add(0x33u64), ((rdx as u8)) as u8); fl_l = ((((rcx as u32)) ^ (crate::simd_ops::ru32(rbx.wrapping_add(0x34u64))))) as u32 as u64; fl_r = fl_l; rcx = ((fl_l as u32)) as u32 as u64; bb = 26; continue; }
            26 => { rcx = ((((rcx as u32)) & (0xffffffu32))) as u32 as u64; rcx = ((((rcx as u32)) ^ (crate::simd_ops::ru32(rbx.wrapping_add(0x34u64))))) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x34u64), ((rcx as u32)) as u32); crate::simd_ops::wu8(rbx.wrapping_add(0x37u64), ((rsi as u8)) as u8); crate::simd_ops::wu64(rbx.wrapping_add(0x38u64), r11); r8 = (crate::simd_ops::ru32(rdi.wrapping_add(0x10u64))) as u32 as u64; rax = (crate::simd_ops::ru64(rdi)) as u64; rdx = (crate::simd_ops::ru64(r13.wrapping_add(0x458u64))) as u64; rcx = (crate::simd_ops::ru32(rax.wrapping_add(0x70u64))) as u32 as u64; crate::simd_ops::wu32(rdx.wrapping_add(r8.wrapping_mul(4)), ((rcx as u32)) as u32); rcx = ((r8 as u32)) as u32 as u64; rdx = (crate::simd_ops::ru64(r13.wrapping_add(0x468u64))) as u64; r8 = ((((r8 as u32)) & (0x3fu32))) as u32 as u64; rcx = ((rcx).wrapping_shr(6u32)) as u64; rax = (crate::simd_ops::ru64(rdx.wrapping_add(rcx.wrapping_mul(8)))) as u64; fl_l = (((rax) >> (((r8) as u32 & 63))) & 1) as u64; rax = ((rax) | ((1u64) << (((r8) as u32 & 63)))) as u64; crate::simd_ops::wu64(rdx.wrapping_add(rcx.wrapping_mul(8)), rax); rsp = ((rsp).wrapping_add(0x78u64)) as u64; r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141308530(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142537280; const P1: u64 = 0x142535070; const P2: u64 = 0x1426456c8; const P3: u64 = 0x1426453a0; const P4: u64 = 0x141307360;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rsi = rdx; rdi = rcx; rbp = (crate::simd_ops::ru32(rcx.wrapping_add(0x8u64))) as u32 as u64; rdx = ((rdx).wrapping_shl(6u32)) as u64; r10 = 0x6348; rax = ((crate::exe::tls_pointer() as u64)) as u64; r8 = (crate::simd_ops::ru64(rax)) as u64; r10 = (crate::simd_ops::ru64(r10.wrapping_add(r8))) as u64; r14 = 0; fl_l = 0; fl_r = 0; fl_l = ((rdx) & ((rdx).wrapping_sub(1))) as u64; rax = ((fl_l as u64)) as u64; if (((fl_l as u64)) != 0) { bb = 5; continue; } bb = 1; continue; }
            1 => { rax = rdx.wrapping_sub(0x10); fl_l = rax; fl_r = (0x3f0u64) as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 5; continue; } bb = 2; continue; }
            2 => { rax = rdx.wrapping_add(0x7); rax = ((rax).wrapping_shr(3u32)) as u64; r8 = (crate::simd_ops::ru64(r10.wrapping_add(rax.wrapping_mul(8)).wrapping_add(0x8u64))) as u64; rbx = (crate::simd_ops::ru64(r8.wrapping_add(0x10u64))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { r9 = 0; fl_l = 0; fl_r = 0; r8 = 0; fl_l = 0; fl_r = 0; rcx = r10; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 12; continue; }
            4 => { fl_l = (crate::simd_ops::lk_add_u32(r8.wrapping_add(0x18u64), (1) as u32)) as u32 as u64; rax = (crate::simd_ops::ru64(rbx)) as u64; crate::simd_ops::wu64(r8.wrapping_add(0x10u64), rax); bb = 13; continue; }
            5 => { rax = (0x7fffffffffffffffu64) as u64; fl_l = rdx; fl_r = rax; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 7; continue; } bb = 6; continue; }
            6 => { rbx = r14; bb = 13; continue; }
            7 => { rax = rdx.wrapping_sub(0x10); fl_l = rax; fl_r = (0x3f0u64) as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 11; continue; } bb = 8; continue; }
            8 => { rax = rdx.wrapping_add(0x7); rax = ((rax).wrapping_shr(3u32)) as u64; rcx = (crate::simd_ops::ru64(r10.wrapping_add(rax.wrapping_mul(8)).wrapping_add(0x8u64))) as u64; rbx = (crate::simd_ops::ru64(rcx.wrapping_add(0x10u64))) as u64; rax = rbx; rax = ((((rax as u32)) & (0xfu32))) as u32 as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 9; continue; }
            9 => { fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 11; continue; } bb = 10; continue; }
            10 => { fl_l = (crate::simd_ops::lk_add_u32(rcx.wrapping_add(0x18u64), (1) as u32)) as u32 as u64; rax = (crate::simd_ops::ru64(rbx)) as u64; crate::simd_ops::wu64(rcx.wrapping_add(0x10u64), rax); bb = 13; continue; }
            11 => { r8 = 0x10; rcx = r10; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 12; continue; }
            12 => { rbx = rax; bb = 13; continue; }
            13 => { fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 15; continue; } bb = 14; continue; }
            14 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rax; rdx = 0xdeadd00d; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = r14; bb = 15; continue; }
            15 => { r8 = (crate::simd_ops::ru64(rdi)) as u64; r9 = (crate::simd_ops::ru32(rdi.wrapping_add(0x8u64))) as u32 as u64; r9 = ((r9).wrapping_shl(6u32)) as u64; r9 = ((r9).wrapping_add(r8)) as u64; fl_l = r8; fl_r = r9; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 18; continue; } bb = 16; continue; }
            16 => { rcx = rbx.wrapping_add(0x28); rdx = r8.wrapping_add(0x28); bb = 17; continue; }
            17 => { rax = (crate::simd_ops::ru64(r8)) as u64; crate::simd_ops::wu64(r8, r14); crate::simd_ops::wu64(rcx.wrapping_sub(0x28u64), rax); xmm0 = ((crate::simd_ops::ru64(rdx.wrapping_sub(0x18u64))) as u64) as u128; crate::simd_ops::wu64(rcx.wrapping_sub(0x18u64), ((xmm0 as u64)) as u64); rax = (crate::simd_ops::ru32(rdx.wrapping_sub(0x10u64))) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_sub(0x10u64), ((rax as u32)) as u32); xmm0 = ((crate::simd_ops::ru64(rdx.wrapping_sub(0xcu64))) as u64) as u128; crate::simd_ops::wu64(rcx.wrapping_sub(0xcu64), ((xmm0 as u64)) as u64); rax = (crate::simd_ops::ru32(rdx.wrapping_sub(0x4u64))) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_sub(0x4u64), ((rax as u32)) as u32); xmm0 = ((crate::simd_ops::ru64(rdx)) as u64) as u128; crate::simd_ops::wu64(rcx, ((xmm0 as u64)) as u64); rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x8u64))) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(0x8u64), ((rax as u32)) as u32); xmm0 = ((crate::simd_ops::ru64(rdx.wrapping_add(0xcu64))) as u64) as u128; crate::simd_ops::wu64(rcx.wrapping_add(0xcu64), ((xmm0 as u64)) as u64); rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x14u64))) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(0x14u64), ((rax as u32)) as u32); rcx = rcx.wrapping_add(0x40); r8 = ((r8).wrapping_add(0x40u64)) as u64; rdx = rdx.wrapping_add(0x40); fl_l = r8; fl_r = r9; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 17; continue; } bb = 18; continue; }
            18 => { rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rdi, rbx); crate::simd_ops::wu32(rdi.wrapping_add(0x8u64), ((rbp as u32)) as u32); crate::simd_ops::wu32(rdi.wrapping_add(0xcu64), ((rsi as u32)) as u32); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1413087a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0x14209d670; const P3: u64 = 0x58; const P4: u64 = 0x1411e7fc0; const P5: u64 = 0x142442de0; const P6: u64 = 0x1401de830; const P7: u64 = 0xfffffffffffffff8;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { fl_l = rdx; fl_r = r8; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 6; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rsi = r8; rbx = ((rdx.wrapping_add(P0)) as u64) as u64; bb = 2; continue; }
            2 => { rdi = (crate::simd_ops::ru64(rbx.wrapping_add(P1))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { rcx = rdi; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rcx = ((rdi.wrapping_add(P3)) as u64) as u64; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rcx = rdi; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); bb = 4; continue; }
            4 => { rcx = rbx; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); rbx = ((rbx).wrapping_add(0x20u64)) as u64; rax = ((rbx.wrapping_add(P7)) as u64) as u64; fl_l = rax; fl_r = rsi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 5; continue; }
            5 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); bb = 6; continue; }
            6 => { return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141308810(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142537280; const P1: u64 = 0x142535070; const P2: u64 = 0x1426456c8; const P3: u64 = 0x1426453a0; const P4: u64 = 0x1413086e0;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rsi = rdx; rdi = rcx; rbp = (crate::simd_ops::ru32(rcx.wrapping_add(0x8u64))) as u32 as u64; rdx = ((rdx).wrapping_shl(5u32)) as u64; r10 = 0x6348; rax = ((crate::exe::tls_pointer() as u64)) as u64; r8 = (crate::simd_ops::ru64(rax)) as u64; r10 = (crate::simd_ops::ru64(r10.wrapping_add(r8))) as u64; r14 = 0; fl_l = 0; fl_r = 0; fl_l = ((rdx) & ((rdx).wrapping_sub(1))) as u64; rax = ((fl_l as u64)) as u64; if (((fl_l as u64)) != 0) { bb = 5; continue; } bb = 1; continue; }
            1 => { rax = rdx.wrapping_sub(0x10); fl_l = rax; fl_r = (0x3f0u64) as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 5; continue; } bb = 2; continue; }
            2 => { rax = rdx.wrapping_add(0x7); rax = ((rax).wrapping_shr(3u32)) as u64; r8 = (crate::simd_ops::ru64(r10.wrapping_add(rax.wrapping_mul(8)).wrapping_add(0x8u64))) as u64; rbx = (crate::simd_ops::ru64(r8.wrapping_add(0x10u64))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { r9 = 0; fl_l = 0; fl_r = 0; r8 = 0; fl_l = 0; fl_r = 0; rcx = r10; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); bb = 12; continue; }
            4 => { fl_l = (crate::simd_ops::lk_add_u32(r8.wrapping_add(0x18u64), (1) as u32)) as u32 as u64; rax = (crate::simd_ops::ru64(rbx)) as u64; crate::simd_ops::wu64(r8.wrapping_add(0x10u64), rax); bb = 13; continue; }
            5 => { rax = (0x7fffffffffffffffu64) as u64; fl_l = rdx; fl_r = rax; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 7; continue; } bb = 6; continue; }
            6 => { rbx = r14; bb = 13; continue; }
            7 => { rax = rdx.wrapping_sub(0x10); fl_l = rax; fl_r = (0x3f0u64) as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 11; continue; } bb = 8; continue; }
            8 => { rax = rdx.wrapping_add(0x7); rax = ((rax).wrapping_shr(3u32)) as u64; rcx = (crate::simd_ops::ru64(r10.wrapping_add(rax.wrapping_mul(8)).wrapping_add(0x8u64))) as u64; rbx = (crate::simd_ops::ru64(rcx.wrapping_add(0x10u64))) as u64; rax = rbx; rax = ((((rax as u32)) & (0xfu32))) as u32 as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 9; continue; }
            9 => { fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 11; continue; } bb = 10; continue; }
            10 => { fl_l = (crate::simd_ops::lk_add_u32(rcx.wrapping_add(0x18u64), (1) as u32)) as u32 as u64; rax = (crate::simd_ops::ru64(rbx)) as u64; crate::simd_ops::wu64(rcx.wrapping_add(0x10u64), rax); bb = 13; continue; }
            11 => { r8 = 0x10; rcx = r10; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 12; continue; }
            12 => { rbx = rax; bb = 13; continue; }
            13 => { fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 15; continue; } bb = 14; continue; }
            14 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); rcx = rax; rdx = 0xdeadd00d; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); rbx = r14; bb = 15; continue; }
            15 => { r8 = (crate::simd_ops::ru64(rdi)) as u64; r9 = (crate::simd_ops::ru32(rdi.wrapping_add(0x8u64))) as u32 as u64; r9 = ((r9).wrapping_shl(5u32)) as u64; r9 = ((r9).wrapping_add(r8)) as u64; fl_l = r8; fl_r = r9; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 18; continue; } bb = 16; continue; }
            16 => { rcx = r8.wrapping_add(0x14); rdx = rbx.wrapping_add(0x14); bb = 17; continue; }
            17 => { rax = (crate::simd_ops::ru32(r8)) as u32 as u64; crate::simd_ops::wu32(rdx.wrapping_sub(0x14u64), ((rax as u32)) as u32); crate::simd_ops::wu64(rdx.wrapping_sub(0xcu64), r14); crate::simd_ops::wu64(rdx.wrapping_sub(0x4u64), r14); rax = (crate::simd_ops::ru64(rcx.wrapping_sub(0xcu64))) as u64; crate::simd_ops::wu64(rdx.wrapping_sub(0xcu64), rax); rax = (crate::simd_ops::ru32(rcx.wrapping_sub(0x4u64))) as u32 as u64; crate::simd_ops::wu32(rdx.wrapping_sub(0x4u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rcx)) as u32 as u64; crate::simd_ops::wu32(rdx, ((rax as u32)) as u32); crate::simd_ops::wu64(rcx.wrapping_sub(0xcu64), r14); crate::simd_ops::wu64(rcx.wrapping_sub(0x4u64), r14); rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x4u64))) as u64; crate::simd_ops::wu64(rcx.wrapping_add(0x4u64), r14); crate::simd_ops::wu64(rdx.wrapping_add(0x4u64), rax); rdx = rdx.wrapping_add(0x20); r8 = ((r8).wrapping_add(0x20u64)) as u64; rcx = rcx.wrapping_add(0x20); fl_l = r8; fl_r = r9; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 17; continue; } bb = 18; continue; }
            18 => { rcx = rdi; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rdi, rbx); crate::simd_ops::wu32(rdi.wrapping_add(0x8u64), ((rbp as u32)) as u32); crate::simd_ops::wu32(rdi.wrapping_add(0xcu64), ((rsi as u32)) as u32); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1413089b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x18; const P1: u64 = 0x141308af0; const P2: u64 = 0x4; const P3: u64 = 0x8; const P4: u64 = 0x142442de0; const P5: u64 = 0x10;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rcx;
    rdx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    fl_l = rdx; fl_r = rdx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); }
    fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(P2))) as u32 as u64; fl_r = 0x1;
    if (((fl_l as u32)) <= ((fl_r as u32))) { rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P3))) as u64;
    fl_l = rcx; fl_r = rcx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); }
    rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P5))) as u64;
    fl_l = rcx; fl_r = rcx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); }
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141308a10(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x108; const P1: u64 = 0x10; const P2: u64 = 0x142537280; const P3: u64 = 0x1426456c8; const P4: u64 = 0x1426453a0; const P5: u64 = 0x18; const P6: u64 = 0x1422b2dd0;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rsi = r9; r14 = r8; rdi = rcx; rbx = 0; fl_l = 0; fl_r = 0; rcx = 0x6348; rax = ((crate::exe::tls_pointer() as u64)) as u64; rdx = (crate::simd_ops::ru64(rax)) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(rdx))) as u64; r10 = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rdx = (crate::simd_ops::ru64(r10.wrapping_add(P1))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { r9 = 0; fl_l = 0; fl_r = 0; r8 = 0; fl_l = 0; fl_r = 0; rdx = 0x100; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rdx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 4; continue; } bb = 2; continue; }
            2 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); rcx = rax; rdx = 0xdeadd00d; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp); bb = 5; continue; }
            3 => { fl_l = (crate::simd_ops::lk_add_u32(r10.wrapping_add(P5), (1) as u32)) as u32 as u64; rax = (crate::simd_ops::ru64(rdx)) as u64; crate::simd_ops::wu64(r10.wrapping_add(P1), rax); bb = 4; continue; }
            4 => { rbx = rdx; bb = 5; continue; }
            5 => { crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rbx); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; rdx = (crate::simd_ops::ru32(rax)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((rdx as u32)) as u32); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); r9 = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; r8 = ((crate::simd_ops::ru8(rsi)) as u32) as u32 as u64; rdx = (crate::simd_ops::ru64(r14)) as u64; rcx = rbx; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rdi, rax); rax = rdi; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141308af0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143a367f8; const P1: u64 = 0x1422b1900; const P2: u64 = 0x1401de830; const P3: u64 = 0x142442de0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 1; continue; }
            1 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rdx; rdx = (((crate::simd_ops::ru32(rdx)) as i32 as i64) as u64) as u64; r9 = (crate::simd_ops::ru64(P0)) as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 5; continue; } bb = 2; continue; }
            2 => { rcx = (crate::simd_ops::ru32(r9.wrapping_add(0x60u64))) as u32 as u64; rax = (crate::simd_ops::ru64(r9.wrapping_add(0x58u64))) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(rcx.wrapping_mul(8)).wrapping_sub(0x8u64))) as u64; crate::simd_ops::wu32(rcx, ((rdx as u32)) as u32); rax = (crate::simd_ops::ru64(r9.wrapping_add(0x58u64))) as u64; rcx = ((rax.wrapping_add(rdx.wrapping_mul(8))) as u64) as u64; rdx = (crate::simd_ops::ru32(r9.wrapping_add(0x60u64))) as u32 as u64; rax = ((rax.wrapping_add(rdx.wrapping_mul(8))) as u64) as u64; rax = ((rax).wrapping_add(0xfffffffffffffff8u64)) as u64; fl_l = rcx; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rax)) as u64; crate::simd_ops::wu64(rcx, rax); bb = 4; continue; }
            4 => { fl_l = (crate::simd_ops::lk_sub_u32(r9.wrapping_add(0x60u64), (1) as u32)) as u32 as u64; crate::simd_ops::wu32(rbx, (0xffffffffu32) as u32); bb = 5; continue; }
            5 => { r8 = rbx.wrapping_add(0x68); fl_l = (crate::simd_ops::ru64(r8)) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 7; continue; } bb = 6; continue; }
            6 => { rdx = r9.wrapping_add(0x18); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); r9 = (crate::simd_ops::ru64(P0)) as u64; bb = 7; continue; }
            7 => { r8 = rbx.wrapping_add(0x78); fl_l = (crate::simd_ops::ru64(r8)) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 9; continue; } bb = 8; continue; }
            8 => { rdx = r9; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); r9 = (crate::simd_ops::ru64(P0)) as u64; bb = 9; continue; }
            9 => { r8 = rbx.wrapping_add(0x88); fl_l = (crate::simd_ops::ru64(r8)) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 11; continue; } bb = 10; continue; }
            10 => { rdx = r9.wrapping_add(0x30); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 11; continue; }
            11 => { rcx = rbx.wrapping_add(0xf0); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rcx = rbx.wrapping_add(0xe0); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rcx = rbx.wrapping_add(0x58); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rcx = rbx.wrapping_add(0x48); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rcx = rbx.wrapping_add(0x38); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rcx = rbx.wrapping_add(0x28); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rcx = rbx.wrapping_add(0x18); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rcx = rbx; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); bb = 12; continue; }
            12 => { return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141308ea0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x4; const P1: u64 = 0x8; const P2: u64 = 0x14183acda; const P3: u64 = 0x10;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64; rbx = r8; rax = (crate::simd_ops::ru32(r8)) as u32 as u64; r14 = ((rdi as u32)) as u32 as u64; r14 = ((r14).wrapping_shl(3u32)) as u64; rbp = rdx; fl_l = (crate::simd_ops::ru32(rdx)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 13; continue; } bb = 1; continue; }
            1 => { fl_l = ((rdi as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(r8.wrapping_add(P0))) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 13; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru32(r8.wrapping_add(P0))) as u32 as u64; fl_r = 0x1; rdx = ((r8.wrapping_add(P1)) as u64) as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rdx = (crate::simd_ops::ru64(rdx)) as u64; bb = 4; continue; }
            4 => { rcx = rbp.wrapping_add(0x8); fl_l = ((rdi as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rcx = (crate::simd_ops::ru64(rcx)) as u64; bb = 6; continue; }
            6 => { r8 = r14; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 13; continue; } bb = 7; continue; }
            7 => { fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(P0))) as u32 as u64; fl_r = 0x1; rdx = ((rbx.wrapping_add(P3)) as u64) as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 9; continue; } bb = 8; continue; }
            8 => { rdx = (crate::simd_ops::ru64(rdx)) as u64; bb = 9; continue; }
            9 => { rcx = rbp.wrapping_add(0x10); fl_l = ((rdi as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 11; continue; } bb = 10; continue; }
            10 => { rcx = (crate::simd_ops::ru64(rcx)) as u64; bb = 11; continue; }
            11 => { r8 = r14; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 13; continue; } bb = 12; continue; }
            12 => { rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 14; continue; }
            13 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 14; continue; }
            14 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1413095a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142330720; const P1: u64 = 0x143a367f8; const P2: u64 = 0x1422b1900; const P3: u64 = 0x1401de830; const P4: u64 = 0x1425371a0; const P5: u64 = 0x142533200; const P6: u64 = 0x142442de0;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = r8; rbx = rdx; rdx = r8; rcx = rbx; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x18u64))) as u64; crate::simd_ops::wu64(rdi.wrapping_add(0x18u64), (0x0u64) as u64); crate::simd_ops::wu64(rbx.wrapping_add(0x18u64), rax); rbx = (crate::simd_ops::ru64(rdi.wrapping_add(0x18u64))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 18; continue; } bb = 1; continue; }
            1 => { rdx = (((crate::simd_ops::ru32(rbx)) as i32 as i64) as u64) as u64; r9 = (crate::simd_ops::ru64(P1)) as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 5; continue; } bb = 2; continue; }
            2 => { rcx = (crate::simd_ops::ru32(r9.wrapping_add(0x60u64))) as u32 as u64; rax = (crate::simd_ops::ru64(r9.wrapping_add(0x58u64))) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(rcx.wrapping_mul(8)).wrapping_sub(0x8u64))) as u64; crate::simd_ops::wu32(rcx, ((rdx as u32)) as u32); rax = (crate::simd_ops::ru64(r9.wrapping_add(0x58u64))) as u64; rcx = ((rax.wrapping_add(rdx.wrapping_mul(8))) as u64) as u64; rdx = (crate::simd_ops::ru32(r9.wrapping_add(0x60u64))) as u32 as u64; rax = ((rax.wrapping_add(rdx.wrapping_mul(8))) as u64) as u64; rax = ((rax).wrapping_add(0xfffffffffffffff8u64)) as u64; fl_l = rcx; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rax)) as u64; crate::simd_ops::wu64(rcx, rax); bb = 4; continue; }
            4 => { fl_l = (crate::simd_ops::lk_sub_u32(r9.wrapping_add(0x60u64), (1) as u32)) as u32 as u64; crate::simd_ops::wu32(rbx, (0xffffffffu32) as u32); bb = 5; continue; }
            5 => { r8 = rbx.wrapping_add(0x68); fl_l = (crate::simd_ops::ru64(r8)) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 7; continue; } bb = 6; continue; }
            6 => { rdx = r9.wrapping_add(0x18); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); r9 = (crate::simd_ops::ru64(P1)) as u64; bb = 7; continue; }
            7 => { r8 = rbx.wrapping_add(0x78); fl_l = (crate::simd_ops::ru64(r8)) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 9; continue; } bb = 8; continue; }
            8 => { rdx = r9; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); r9 = (crate::simd_ops::ru64(P1)) as u64; bb = 9; continue; }
            9 => { r8 = rbx.wrapping_add(0x88); fl_l = (crate::simd_ops::ru64(r8)) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 11; continue; } bb = 10; continue; }
            10 => { rdx = r9.wrapping_add(0x30); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 11; continue; }
            11 => { rcx = rbx.wrapping_add(0xf0); rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rcx = rbx.wrapping_add(0xe0); rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rcx = rbx.wrapping_add(0x58); rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rcx = rbx.wrapping_add(0x48); rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rcx = rbx.wrapping_add(0x38); rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rcx = rbx.wrapping_add(0x28); rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rcx = rbx.wrapping_add(0x18); rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); r10 = rbx.wrapping_sub(0x1); r10 = (((r10) & (0xfffffffffe000000u64))) as u64; rax = ((crate::exe::teb_self() as u64)) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(r10.wrapping_add(0x100u64))) as u64; rcx = (rcx & !0xffu64) | (((if (((fl_l as u64)) == ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); rax = rbx; rax = ((rax).wrapping_sub(r10)) as u64; rax = ((rax).wrapping_shr(16u32)) as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rdx = r10.wrapping_add(0x108); rdx = ((rdx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x4u64))) as u32 as u64; rdx = ((rdx).wrapping_sub(rax)) as u64; fl_l = ((rcx as u8)) as u8 as u64; fl_r = ((rcx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 16; continue; } bb = 12; continue; }
            12 => { fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(0xeu64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 15; continue; } bb = 13; continue; }
            13 => { rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(rbx, rax); crate::simd_ops::wu64(rdx.wrapping_add(0x20u64), rbx); fl_l = (crate::simd_ops::lk_add_u32(rdx.wrapping_add(0x18u64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; if (((fl_l as u32)) != 0) { bb = 18; continue; } bb = 14; continue; }
            14 => { rcx = rdx; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); bb = 18; continue; }
            15 => { r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 17; continue; }
            16 => { r8 = 0; fl_l = 0; fl_r = 0; bb = 17; continue; }
            17 => { r9 = rbx; rcx = r10; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); bb = 18; continue; }
            18 => { fl_l = (crate::simd_ops::ru32(rdi.wrapping_add(0x4u64))) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 23; continue; } bb = 19; continue; }
            19 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 21; continue; } bb = 20; continue; }
            20 => { rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); bb = 21; continue; }
            21 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x10u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 23; continue; } bb = 22; continue; }
            22 => { rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); bb = 23; continue; }
            23 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1413097e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x18; const P2: u64 = 0x14386a988; const P3: u64 = 0x142229ab0; const P4: u64 = 0x10; const P5: u64 = 0x4; const P6: u64 = 0x14209d670; const P7: u64 = 0x58; const P8: u64 = 0x1411e7fc0; const P9: u64 = 0x142442de0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; r14 = 0; fl_l = 0; fl_r = 0; rbp = ((r14 as u32)) as u32 as u64; rbx = (crate::simd_ops::ru64(rdx)) as u64; rdi = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64; rdi = ((rdi).wrapping_shl(5u32)) as u64; rdi = ((rdi).wrapping_add(rbx)) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 10; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru64(rbx.wrapping_add(P1))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 9; continue; } bb = 2; continue; }
            2 => { rsi = (crate::simd_ops::ru32(P2)) as u32 as u64; rcx = rbx; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rcx = ((r14 as u32)) as u32 as u64; rax = (crate::simd_ops::ru64(rbx.wrapping_add(P0))) as u64; rdx = (crate::simd_ops::ru32(rbx.wrapping_add(P4))) as u32 as u64; r8 = ((rax.wrapping_add(rdx.wrapping_mul(8))) as u64) as u64; fl_l = rax; fl_r = r8; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 5; continue; } bb = 3; continue; }
            3 => { bb = 4; continue; }
            4 => { rcx = (((rcx as u32)).wrapping_add(crate::simd_ops::ru32(rax.wrapping_add(P5)))) as u32 as u64; rax = ((rax).wrapping_add(0x8u64)) as u64; fl_l = rax; fl_r = r8; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 4; continue; } bb = 5; continue; }
            5 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rbx)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 9; continue; } bb = 6; continue; }
            6 => { rbp = (((rbp as u32)).wrapping_add(1)) as u32 as u64; fl_l = ((rbp as u32)) as u32 as u64; fl_r = ((rsi as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 9; continue; } bb = 7; continue; }
            7 => { rsi = (crate::simd_ops::ru64(rbx.wrapping_add(P1))) as u64; crate::simd_ops::wu64(rbx.wrapping_add(P1), r14); fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 8; continue; }
            8 => { rcx = rsi; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); rcx = ((rsi.wrapping_add(P7)) as u64) as u64; rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); rcx = rsi; rax = crate::simd_ops::call_stk(P9, rax, rcx, rdx, r8, r9, rsp); bb = 9; continue; }
            9 => { rbx = ((rbx).wrapping_add(0x20u64)) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } bb = 10; continue; }
            10 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1413098b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14241bff0; const P1: u64 = 0x14241c0a0; const P2: u64 = 0x14183acc8; const P3: u64 = 0x14241c190;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x8u64))) as u32 as u64; rdi = rdx; rbx = rcx; fl_l = rdx; fl_r = rax; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 16; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru32(rcx.wrapping_add(0xcu64))) as u32 as u64; rax = ((((rax as u32)) & (0x3fffffffu32))) as u32 as u64; fl_l = rax; fl_r = rdx; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 12; continue; } bb = 2; continue; }
            2 => { rcx = (crate::simd_ops::ru64(rcx)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rsi); rsi = (((rdi) as i64).wrapping_mul((0x88u64) as i64) as u64) as u64; rdx = rsi.wrapping_add(0x4); rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { crate::simd_ops::wu64(rbx, rax); bb = 11; continue; }
            4 => { rdx = 0x8; rcx = rsi; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rdx = (crate::simd_ops::ru64(rbx)) as u64; rsi = rax; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 5; continue; }
            5 => { rcx = (crate::simd_ops::ru32(rbx.wrapping_add(0x8u64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 7; continue; } bb = 6; continue; }
            6 => { r8 = (((rcx) as i64).wrapping_mul((0x88u64) as i64) as u64) as u64; rcx = rax; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rdx = (crate::simd_ops::ru64(rbx)) as u64; bb = 7; continue; }
            7 => { fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0xcu64))) as u32 as u64; fl_r = 0x40000000; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 10; continue; } bb = 8; continue; }
            8 => { fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 9; continue; }
            9 => { rcx = rdx; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 10; continue; }
            10 => { crate::simd_ops::wu64(rbx, rsi); bb = 11; continue; }
            11 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = ((rdi as u32)) as u32 as u64; fl_l = ((((rax as u32)) & (0x3fffffffu32))) as u32 as u64; fl_r = fl_l; rax = ((fl_l as u32)) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0xcu64), ((rax as u32)) as u32); bb = 12; continue; }
            12 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x8u64))) as u32 as u64; r8 = (((rax) as i64).wrapping_mul((0x88u64) as i64) as u64) as u64; r9 = (((rdi) as i64).wrapping_mul((0x88u64) as i64) as u64) as u64; r8 = ((r8).wrapping_add(crate::simd_ops::ru64(rbx))) as u64; r9 = ((r9).wrapping_add(crate::simd_ops::ru64(rbx))) as u64; fl_l = r8; fl_r = r9; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 15; continue; } bb = 13; continue; }
            13 => { rax = r8.wrapping_add(0x60); rdx = 0; fl_l = 0; fl_r = 0; bb = 14; continue; }
            14 => { crate::simd_ops::wu32(r8, ((rdx as u32)) as u32); rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rax.wrapping_sub(0x5cu64), ((rdx as u32)) as u32); r8 = ((r8).wrapping_add(0x88u64)) as u64; crate::simd_ops::wu16(rax.wrapping_sub(0x3bu64), ((rcx as u16)) as u16); crate::simd_ops::wu8(rax.wrapping_sub(0x39u64), ((rcx as u8)) as u8); crate::simd_ops::wu64(rax.wrapping_sub(0x18u64), rcx); crate::simd_ops::wu64(rax.wrapping_add(0x20u64), rcx); crate::simd_ops::wu64(rax.wrapping_sub(0x58u64), rdx); crate::simd_ops::wu64(rax.wrapping_sub(0x50u64), rdx); crate::simd_ops::wu64(rax.wrapping_sub(0x48u64), rdx); crate::simd_ops::wu32(rax.wrapping_sub(0x40u64), ((rdx as u32)) as u32); crate::simd_ops::wu8(rax.wrapping_sub(0x3cu64), (0x2u8) as u8); crate::simd_ops::wu64(rax.wrapping_sub(0x38u64), rdx); crate::simd_ops::wu64(rax.wrapping_sub(0x30u64), rdx); crate::simd_ops::wu64(rax.wrapping_sub(0x28u64), rdx); crate::simd_ops::wu64(rax.wrapping_sub(0x20u64), rdx); crate::simd_ops::wu64(rax.wrapping_sub(0x10u64), rdx); crate::simd_ops::wu64(rax.wrapping_sub(0x8u64), rdx); crate::simd_ops::wu64(rax, rdx); crate::simd_ops::wu64(rax.wrapping_add(0x8u64), rdx); crate::simd_ops::wu64(rax.wrapping_add(0x10u64), rdx); crate::simd_ops::wu64(rax.wrapping_add(0x18u64), rdx); rax = rax.wrapping_add(0x88); fl_l = r8; fl_r = r9; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 14; continue; } bb = 15; continue; }
            15 => { crate::simd_ops::wu32(rbx.wrapping_add(0x8u64), ((rdi as u32)) as u32); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            16 => { crate::simd_ops::wu32(rcx.wrapping_add(0x8u64), ((rdi as u32)) as u32); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14130a300(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14130a340; const P1: u64 = 0x4; const P2: u64 = 0x8; const P3: u64 = 0x142442de0; const P4: u64 = 0x10;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rcx;
    rcx = ((rcx).wrapping_add(0x18u64)) as u64;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(P1))) as u32 as u64; fl_r = 0x1;
    if (((fl_l as u32)) <= ((fl_r as u32))) { rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P2))) as u64;
    fl_l = rcx; fl_r = rcx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); }
    rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P4))) as u64;
    fl_l = rcx; fl_r = rcx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); }
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14130a340(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x98; const P1: u64 = 0x1423301f0; const P2: u64 = 0x142330010; const P3: u64 = 0x30; const P4: u64 = 0x143a367f8; const P5: u64 = 0x1422b1900; const P6: u64 = 0x38; const P7: u64 = 0xa0; const P8: u64 = 0x14209d670; const P9: u64 = 0x58; const P10: u64 = 0x1411e7fc0; const P11: u64 = 0x142442de0; const P12: u64 = 0x1401de830;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = (crate::simd_ops::ru64(rcx)) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 1; continue; }
            1 => { rcx = (crate::simd_ops::ru32(rbx.wrapping_add(P0))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0xffffffff; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 3; continue; } bb = 2; continue; }
            2 => { rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru32(rbx.wrapping_add(P0))) as u32 as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu32(rbx.wrapping_add(P0), (0xffffffffu32) as u32); bb = 3; continue; }
            3 => { fl_l = (crate::simd_ops::ru64(rbx.wrapping_add(P3))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 5; continue; } bb = 4; continue; }
            4 => { rdx = (crate::simd_ops::ru64(P4)) as u64; rdx = ((rdx).wrapping_add(0x18u64)) as u64; r8 = ((rbx.wrapping_add(P3)) as u64) as u64; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); bb = 5; continue; }
            5 => { rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rbx.wrapping_add(P3), rax); crate::simd_ops::wu64(rbx.wrapping_add(P6), rax); rdi = (crate::simd_ops::ru64(rbx.wrapping_add(P7))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 6; continue; }
            6 => { rcx = rdi; rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); rcx = ((rdi.wrapping_add(P9)) as u64) as u64; rax = crate::simd_ops::call_stk(P10, rax, rcx, rdx, r8, r9, rsp); rcx = rdi; rax = crate::simd_ops::call_stk(P11, rax, rcx, rdx, r8, r9, rsp); bb = 7; continue; }
            7 => { rcx = rbx.wrapping_add(0x20); rax = crate::simd_ops::call_stk(P12, rax, rcx, rdx, r8, r9, rsp); rcx = rbx; rax = crate::simd_ops::call_stk(P11, rax, rcx, rdx, r8, r9, rsp); bb = 8; continue; }
            8 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14130a3f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xb0; const P1: u64 = 0x10; const P2: u64 = 0x18; const P3: u64 = 0x1422dbec0; const P4: u64 = 0x142535070; const P5: u64 = 0x1426456c8; const P6: u64 = 0x1426453a0;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rsi = r9; r14 = r8; rdi = rcx; rbx = 0; fl_l = 0; fl_r = 0; rcx = 0x6348; rax = ((crate::exe::tls_pointer() as u64)) as u64; rdx = (crate::simd_ops::ru64(rax)) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(rdx))) as u64; r10 = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rdx = (crate::simd_ops::ru64(r10.wrapping_add(P1))) as u64; rax = rdx; rax = ((((rax as u32)) & (0xfu32))) as u32 as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 1; continue; }
            1 => { fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 5; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::lk_add_u32(r10.wrapping_add(P2), (1) as u32)) as u32 as u64; rax = (crate::simd_ops::ru64(rdx)) as u64; crate::simd_ops::wu64(r10.wrapping_add(P1), rax); bb = 3; continue; }
            3 => { rbx = rdx; bb = 4; continue; }
            4 => { crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rbx); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); r9 = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; r9 = (crate::simd_ops::ru32(r9)) as u32 as u64; r8 = (crate::simd_ops::ru32(rsi)) as u32 as u64; rdx = (crate::simd_ops::ru64(r14)) as u64; rcx = rbx; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rdi, rax); rax = rdi; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            5 => { rdx = 0xa8; r8 = 0x10; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rdx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 3; continue; } bb = 6; continue; }
            6 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp); rcx = rax; rdx = 0xdeadd00d; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P6), rax), rax, rcx, rdx, r8, r9, rsp); bb = 4; continue; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14130ade0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142330720; const P1: u64 = 0x1423301f0; const P2: u64 = 0x142330010; const P3: u64 = 0x143a367f8; const P4: u64 = 0x1422b1900; const P5: u64 = 0x14209d670; const P6: u64 = 0x1411e7fc0; const P7: u64 = 0x142442de0; const P8: u64 = 0x1401de830; const P9: u64 = 0x1425371a0; const P10: u64 = 0x142533200;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rsi = r8; rbx = rdx; rdx = r8; rcx = rbx; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x18u64))) as u64; rbp = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsi.wrapping_add(0x18u64), rbp); crate::simd_ops::wu64(rbx.wrapping_add(0x18u64), rax); rbx = (crate::simd_ops::ru64(rsi.wrapping_add(0x18u64))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 14; continue; } bb = 1; continue; }
            1 => { rcx = (crate::simd_ops::ru32(rbx.wrapping_add(0x98u64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0xffffffff; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 3; continue; } bb = 2; continue; }
            2 => { rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru32(rbx.wrapping_add(0x98u64))) as u32 as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu32(rbx.wrapping_add(0x98u64), (0xffffffffu32) as u32); bb = 3; continue; }
            3 => { fl_l = (crate::simd_ops::ru64(rbx.wrapping_add(0x30u64))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 5; continue; } bb = 4; continue; }
            4 => { rdx = (crate::simd_ops::ru64(P3)) as u64; rdx = ((rdx).wrapping_add(0x18u64)) as u64; r8 = rbx.wrapping_add(0x30); rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); bb = 5; continue; }
            5 => { crate::simd_ops::wu64(rbx.wrapping_add(0x30u64), rbp); crate::simd_ops::wu64(rbx.wrapping_add(0x38u64), rbp); rdi = (crate::simd_ops::ru64(rbx.wrapping_add(0xa0u64))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 6; continue; }
            6 => { rcx = rdi; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rcx = rdi.wrapping_add(0x58); rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); rcx = rdi; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); bb = 7; continue; }
            7 => { rcx = rbx.wrapping_add(0x20); rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); r10 = rbx.wrapping_sub(0x1); r10 = (((r10) & (0xfffffffffe000000u64))) as u64; rax = ((crate::exe::teb_self() as u64)) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(r10.wrapping_add(0x100u64))) as u64; rdx = (rdx & !0xffu64) | (((if (((fl_l as u64)) == ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); rax = rbx; rax = ((rax).wrapping_sub(r10)) as u64; rax = ((rax).wrapping_shr(16u32)) as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rcx = r10.wrapping_add(0x108); rcx = ((rcx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x4u64))) as u32 as u64; rcx = ((rcx).wrapping_sub(rax)) as u64; fl_l = ((rdx as u8)) as u8 as u64; fl_r = ((rdx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 12; continue; } bb = 8; continue; }
            8 => { fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0xeu64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 11; continue; } bb = 9; continue; }
            9 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(rbx, rax); crate::simd_ops::wu64(rcx.wrapping_add(0x20u64), rbx); fl_l = (crate::simd_ops::lk_add_u32(rcx.wrapping_add(0x18u64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; if (((fl_l as u32)) != 0) { bb = 14; continue; } bb = 10; continue; }
            10 => { rax = crate::simd_ops::call_stk(P9, rax, rcx, rdx, r8, r9, rsp); bb = 14; continue; }
            11 => { r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 13; continue; }
            12 => { r8 = 0; fl_l = 0; fl_r = 0; bb = 13; continue; }
            13 => { rdx = rcx; r9 = rbx; rcx = r10; rax = crate::simd_ops::call_stk(P10, rax, rcx, rdx, r8, r9, rsp); bb = 14; continue; }
            14 => { fl_l = (crate::simd_ops::ru32(rsi.wrapping_add(0x4u64))) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 19; continue; } bb = 15; continue; }
            15 => { rcx = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 17; continue; } bb = 16; continue; }
            16 => { rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); bb = 17; continue; }
            17 => { rcx = (crate::simd_ops::ru64(rsi.wrapping_add(0x10u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 19; continue; } bb = 18; continue; }
            18 => { rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); bb = 19; continue; }
            19 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14130afd0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xa8; const P1: u64 = 0x10; const P2: u64 = 0x18; const P3: u64 = 0x142535070; const P4: u64 = 0x1426456c8; const P5: u64 = 0x1426453a0; const P6: u64 = 0x20; const P7: u64 = 0x30; const P8: u64 = 0x38; const P9: u64 = 0x3c; const P10: u64 = 0x40; const P11: u64 = 0x48;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rax = ((crate::exe::tls_pointer() as u64)) as u64; rbx = rcx; rcx = 0x6348; rsi = 0; fl_l = 0; fl_r = 0; rdi = r8; rdx = (crate::simd_ops::ru64(rax)) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(rdx))) as u64; r9 = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rdx = (crate::simd_ops::ru64(r9.wrapping_add(P1))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { fl_l = ((rdx as u8)) as u8 as u64; fl_r = (0xfu8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::lk_add_u32(r9.wrapping_add(P2), (1) as u32)) as u32 as u64; rax = (crate::simd_ops::ru64(rdx)) as u64; crate::simd_ops::wu64(r9.wrapping_add(P1), rax); bb = 5; continue; }
            3 => { rdx = 0xa0; r8 = 0x10; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rax; rdx = 0xdeadd00d; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = rsi; bb = 5; continue; }
            5 => { let y_addr: u64 = rdi; xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rdx; crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); xmm1 = ((crate::simd_ops::ru64(rdi.wrapping_add(P6))) as u64) as u128; crate::simd_ops::wu64(rdx.wrapping_add(P6), ((xmm1 as u64)) as u64); crate::simd_ops::wu64(rdx.wrapping_add(P7), rsi); rax = rbx; crate::simd_ops::wu32(rdx.wrapping_add(P8), (0xffffffffu32) as u32); crate::simd_ops::wu32(rdx.wrapping_add(P9), ((rsi as u32)) as u32); crate::simd_ops::wu64(rdx.wrapping_add(P10), rsi); crate::simd_ops::wu64(rdx.wrapping_add(P11), rsi); crate::simd_ops::wu64(rdx.wrapping_add(0x50u64), rsi); crate::simd_ops::wu64(rdx.wrapping_add(0x58u64), rsi); crate::simd_ops::wu32(rdx.wrapping_add(0x64u64), ((rsi as u32)) as u32); crate::simd_ops::wu64(rdx.wrapping_add(0x68u64), rsi); crate::simd_ops::wu64(rdx.wrapping_add(0x70u64), rsi); crate::simd_ops::wu64(rdx.wrapping_add(0x78u64), rsi); crate::simd_ops::wu64(rdx.wrapping_add(0x80u64), rsi); crate::simd_ops::wu64(rdx.wrapping_add(0x88u64), rsi); crate::simd_ops::wu8(rdx.wrapping_add(0x60u64), (0x4u8) as u8); crate::simd_ops::wu8(rdx.wrapping_add(0x28u64), ((rsi as u8)) as u8); crate::simd_ops::wu32(rdx.wrapping_add(0x2cu64), ((rsi as u32)) as u32); crate::simd_ops::wu64(rdx.wrapping_add(0x90u64), rsi); crate::simd_ops::wu32(rdx.wrapping_add(0x98u64), (0xffffffffu32) as u32); crate::simd_ops::wu64(rbx, rdx); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}
