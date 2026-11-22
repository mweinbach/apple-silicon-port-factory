#![allow(unused_imports, unused_variables, unused_mut, unused_assignments, non_snake_case, dead_code, clippy::all, unreachable_code, unused_parens)]

#[inline(never)]
pub unsafe extern "win64" fn fn_142185880(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1426450b0; const P1: u64 = 0x14183acc2; const P2: u64 = 0x1426455a8; const P3: u64 = 0x141292c70; const P4: u64 = 0x141292870; const P5: u64 = 0x142645598; const P6: u64 = 0x141292f00; const P7: u64 = 0x141292b90;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedStack::<1>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x142185880, funcinfo_rva: 0x37aeda4, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0x19c8, conts: &[] };
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (rbx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rbp = ((rsp.wrapping_sub(0x18b0u64)) as u64) as u64; rax = (0x19b0u32) as u32 as u64; rsp = ((rsp).wrapping_sub(rax)) as u64; rdi = (rcx) as u64; fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0xa78u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 3; continue; } bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(rcx)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1421858b5); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rcx = (crate::simd_ops::ru64(rax.wrapping_add(0x1928u64))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1421858bf); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P0)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 3; continue; }
            3 => { rdx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; r8 = (0x1910u32) as u32 as u64; rcx = ((rbp.wrapping_sub(0x60u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1421858d1); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rax = ((rbp.wrapping_sub(0x50u64)) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x60u64), (rax) as u64); crate::simd_ops::wu32(rbp.wrapping_sub(0x58u64), (0x0u32) as u32); crate::simd_ops::wu32(rbp.wrapping_sub(0x54u64), (0x28u32) as u32); rax = (crate::simd_ops::ru64(rdi)) as u64; rcx = (rdi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1421858f2); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rsi = (rax) as u64; rcx = ((rax.wrapping_add(0x1910u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1421858ff); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P2)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); r8 = (crate::simd_ops::ru64(rsi)) as u64; rcx = (crate::simd_ops::ru32(rsi.wrapping_add(0x8u64))) as u32 as u64; rdx = ((rcx.wrapping_add(rcx.wrapping_mul(4))) as u64) as u64; rdx = ((rdx).wrapping_shl(5u32)) as u64; rdx = ((rdx).wrapping_add(r8)) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x18d0u64), (rdx) as u64); crate::simd_ops::wu64(rbp.wrapping_add(0x18d8u64), (r8) as u64); r8 = ((rbp.wrapping_add(0x18d0u64)) as u64) as u64; rdx = ((rbp.wrapping_add(0x18d8u64)) as u64) as u64; rcx = ((rbp.wrapping_sub(0x60u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142185936); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rcx = (rsi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14218593f); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rcx = ((rsi.wrapping_add(0x1910u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14218594b); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P5)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rbx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x60u64))) as u64; rax = (crate::simd_ops::ru32(rbp.wrapping_sub(0x58u64))) as u32 as u64; rsi = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rsi = ((rsi).wrapping_shl(5u32)) as u64; rsi = ((rsi).wrapping_add(rbx)) as u64; fl_l = (rbx) as u64 as u64; fl_r = (rsi) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 6; continue; } bb = 4; continue; }
            4 => { bb = 5; continue; }
            5 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rdi) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (rdi) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (rdi) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), (rdi) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), (rdi) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), (rdi) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), (rdi) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), (rdi) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), (rdi) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), (rdi) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), (rdi) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x78u64), (rdi) as u64); crate::simd_ops::wu64(rbp.wrapping_sub(0x80u64), (rdi) as u64); crate::simd_ops::wu64(rbp.wrapping_sub(0x78u64), (rdi) as u64); crate::simd_ops::wu64(rbp.wrapping_sub(0x70u64), (rdi) as u64); rcx = (((crate::simd_ops::ru8(rbx.wrapping_add(0x98u64))) as i8 as i64) as u64) as u64; rcx = ((rcx).wrapping_add(1)) as u64; r8 = (rbx) as u64; rdx = ((rsp.wrapping_add(0x20u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1421859cb); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rbx = ((rbx).wrapping_add(0xa0u64)) as u64; fl_l = (rbx) as u64 as u64; fl_r = (rsi) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 5; continue; } bb = 6; continue; }
            6 => { rcx = ((rbp.wrapping_sub(0x60u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1421859e0); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P7) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x19e0u64))) as u64; rsp = ((rsp).wrapping_add(0x19b0u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rsi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rbp = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); bb = 7; continue; }
            7 => { return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142185ac0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    crate::simd_ops::forward_incoming_to_outgoing();
    const P0: u64 = 0xa70; const P1: u64 = 0x14219bba0;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(P0))) as u8 as u64; fl_r = (0x0u8) as u8 as u64;
    if !((((fl_l as u8)) == ((fl_r as u8)))) { rax = 0x1; return rax; }
    crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8(P1, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142185fe0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x1910; const P2: u64 = 0x1426455a8; const P3: u64 = 0x8; const P4: u64 = 0xc; const P5: u64 = 0x1412934c0; const P6: u64 = 0x98; const P7: u64 = 0x141293bd0; const P8: u64 = 0x142645598;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rsi = rdx; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64, rax, rcx, rdx, r8, r9, rsp); rdi = rax; rcx = ((rax.wrapping_add(P1)) as u64) as u64; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P2)) as u64, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru32(rdi.wrapping_add(P3))) as u32 as u64; r8 = (crate::simd_ops::ru32(rdi.wrapping_add(P4))) as u32 as u64; rdx = ((r8.wrapping_add(r8.wrapping_mul(4))) as u64) as u64; rdx = ((rdx).wrapping_shl(5u32)) as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rcx = ((rcx).wrapping_shl(5u32)) as u64; fl_l = rcx; fl_r = rdx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } rax = ((rax).wrapping_add(1)) as u64; rdx = ((r8 as u32)) as u32 as u64; rdx = ((rdx).wrapping_shr(1u32)) as u64; rdx = ((rdx).wrapping_add(r8)) as u64; fl_l = rdx; fl_r = rax; rdx = (if (((fl_l as u64)) < ((fl_r as u64))) { rax } else { rdx }) as u64; rcx = rdi; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru32(rdi.wrapping_add(P3))) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rcx = ((rcx).wrapping_shl(5u32)) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rdi))) as u64; crate::simd_ops::wu8(rcx.wrapping_add(P6), (0xffu8) as u8); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rcx); rcx = (((crate::simd_ops::ru8(rsi.wrapping_add(P6))) as i8 as i64) as u64) as u64; rcx = ((rcx).wrapping_add(1)) as u64; r8 = rsi; rdx = rsp.wrapping_add(0x30); rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::lk_add_u32(rdi.wrapping_add(P3), (1) as u32); rcx = ((rdi.wrapping_add(P1)) as u64) as u64; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P8)) as u64, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); bb = 2; continue; }
            2 => { return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1421860b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x98;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = (crate::simd_ops::ru64(rdx)) as u64;
    crate::simd_ops::wu64(rcx, rax);
    rax = rcx;
    crate::simd_ops::wu8(rcx.wrapping_add(P0), (0x2u8) as u8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1421860d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0x20; const P3: u64 = 0x48; const P4: u64 = 0x14240aa00; const P5: u64 = 0x141292640; const P6: u64 = 0x90; const P7: u64 = 0x98; const P8: u64 = 0x14240ab10;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 25>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { r11 = rsp; crate::simd_ops::wu64(r11.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0xc0u64)) as u64; rdi = ((r8 as u32)) as u32 as u64; r10 = rdx; rbx = rcx; rcx = 0; fl_l = 0; fl_r = 0; rax = 0; fl_l = 0; fl_r = 0; xmm0 = 0u128; ymm0_hi = 0; let y_addr: u64 = rsp.wrapping_add(0x48u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); crate::simd_ops::wu64(r11.wrapping_sub(0x60u64), rcx); crate::simd_ops::wu64(r11.wrapping_sub(0x58u64), (0x8000008u64) as u64); crate::simd_ops::wu8(rsp.wrapping_add(0x78u64), ((rax as u8)) as u8); crate::simd_ops::wu32(r11.wrapping_sub(0x10u64), ((rcx as u32)) as u32); rax = (crate::simd_ops::ru64(rdx)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax); rax = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((rax as u32)) as u32); xmm0 = (crate::simd_ops::ru128(rdx.wrapping_add(P1))) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rsp.wrapping_add(0x38u64), (xmm0) as u128); rax = ((rdx.wrapping_add(P2)) as u64) as u64; rcx = r11.wrapping_sub(0x80); fl_l = rcx; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 3; continue; } r8 = r11.wrapping_sub(0x80); r11 = ((rax.wrapping_add(P2)) as u64) as u64; rcx = (crate::simd_ops::ru64(r11)) as u64; rcx = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = rax; fl_r = rcx; rdx = (rdx & !0xffu64) | (((if (((fl_l as u64)) != ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); fl_l = ((rdx as u8)) as u8 as u64; fl_r = ((rdx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 2; continue; } r9 = r8; bb = 1; continue; }
            1 => { rdx = (crate::simd_ops::ru64(rax)) as u64; crate::simd_ops::wu64(r8, rdx); r8 = ((r9.wrapping_add(P0)) as u64) as u64; rax = ((rax).wrapping_add(0x8u64)) as u64; r9 = r8; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(r11)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), rax); bb = 3; continue; }
            3 => { rdx = ((r10.wrapping_add(P3)) as u64) as u64; rcx = rsp.wrapping_add(0x70); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu32(rsp.wrapping_add(0xb8u64), ((rdi as u32)) as u32); rdx = rsp.wrapping_add(0x28); rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru32(rsp.wrapping_add(0xb8u64))) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(P6), ((rax as u32)) as u32); crate::simd_ops::wu8(rbx.wrapping_add(P7), (0x3u8) as u8); rcx = rsp.wrapping_add(0x70); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P8, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = rbx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0xd0u64))) as u64; rsp = ((rsp).wrapping_add(0xc0u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1421861d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x98;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = (crate::simd_ops::ru64(rdx)) as u64;
    crate::simd_ops::wu64(rcx, rax);
    rax = rcx;
    crate::simd_ops::wu8(rcx.wrapping_add(P0), (0x4u8) as u8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1421861f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xa78; const P1: u64 = 0x779; const P2: u64 = 0x10; const P3: u64 = 0x1928; const P4: u64 = 0x1426450e0; const P5: u64 = 0x4;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(P0))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; rbx = rcx; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 3; continue; } fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(P1))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 3; continue; } rax = (crate::simd_ops::ru64(rdx)) as u64; rcx = rdx; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rax.wrapping_add(P3))) as u64; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P4)) as u64, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 2; continue; } fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x14; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 1; continue; } crate::simd_ops::wu32(rbx, (0xdu32) as u32); rax = rbx; crate::simd_ops::wu8(rbx.wrapping_add(P5), (0x0u8) as u8); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            1 => { crate::simd_ops::wu8(rbx, (0x0u8) as u8); rax = rbx; crate::simd_ops::wu8(rbx.wrapping_add(P5), (0x1u8) as u8); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            2 => { crate::simd_ops::wu8(rbx, (0x1u8) as u8); rax = rbx; crate::simd_ops::wu8(rbx.wrapping_add(P5), (0x1u8) as u8); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            3 => { crate::simd_ops::wu8(rcx, (0x0u8) as u8); rax = rbx; crate::simd_ops::wu8(rcx.wrapping_add(P5), (0x1u8) as u8); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1421864c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rcx = (crate::simd_ops::ru64(rcx)) as u64;
    rax = (((r8) as i64).wrapping_mul((0x78u64) as i64) as u64) as u64;
    fl_l = rcx; fl_r = rax;
    rcx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64;
    rax = rdx;
    crate::simd_ops::wu64(rdx, rcx);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142186650(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = (((r8) as i64).wrapping_mul((0x78u64) as i64) as u64) as u64;
    fl_l = ((rax).wrapping_add(crate::simd_ops::ru64(rcx))) as u64; fl_r = fl_l;
    rax = ((fl_l as u64)) as u64;
    crate::simd_ops::wu64(rdx, rax);
    rax = rdx;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142186690(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    crate::simd_ops::forward_incoming_to_outgoing();
    const P0: u64 = 0x14241c0a0;
    let (mut rax, mut fl_l) = (crate::simd_ops::get_rax_in(), 0u64);
    rcx = ((rdx.wrapping_add(rdx.wrapping_mul(2))) as u64) as u64;
    rdx = 0x4;
    fl_l = ((rcx).wrapping_shl(4u32)) as u64;
    rcx = ((fl_l as u64)) as u64;
    crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8(P0, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1421867d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    fl_l = (crate::simd_ops::lk_add_u64(rcx, (0xffffffffffffff88u64) as u64)) as u64; fl_r = fl_l;
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    crate::simd_ops::wu64(rdx, rax);
    rax = rdx;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142186970(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    fl_l = rax; fl_r = (0x78u64) as u64;
    rax = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142186ab0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    fl_l = (crate::simd_ops::lk_add_u64(rcx, (0x78u64) as u64)) as u64; fl_r = fl_l;
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    crate::simd_ops::wu64(rdx, rax);
    rax = rdx;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142186ad0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    r8 = (crate::simd_ops::ru64(rcx)) as u64;
    rax = (0x8888888888888889u64) as u64;
    r8 = ((r8).wrapping_sub(crate::simd_ops::ru64(rdx))) as u64;
    let prod = (rax as i64 as i128).wrapping_mul((r8) as i64 as i128) as u128;
    rax = prod as u64; rdx = (prod >> 64) as u64;
    rdx = ((rdx).wrapping_add(r8)) as u64;
    rdx = ((((rdx) as i64).wrapping_shr(6u32) as u64)) as u64;
    rax = rdx;
    rax = ((rax).wrapping_shr(63u32)) as u64;
    fl_l = ((rax).wrapping_add(rdx)) as u64; fl_r = fl_l;
    rax = ((fl_l as u64)) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142187820(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1421878d0;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 21>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x142187820, funcinfo_rva: 0x37af12c, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0xa8, conts: &[] };
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { r11 = (rsp) as u64; rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64); rsp = ((rsp).wrapping_sub(0xa0u64)) as u64; rbx = (rcx) as u64; rax = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(r11.wrapping_sub(0x68u64), (rdx) as u64); crate::simd_ops::wu64(r11.wrapping_sub(0x60u64), (rax) as u64); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xe0u64))) as u64; xmm0 = (crate::simd_ops::ru128(rax)) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x50u64), (xmm0) as u128); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xd8u64))) as u64; xmm1 = (crate::simd_ops::ru128(rax)) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x60u64), (xmm1) as u128); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xd0u64))) as u64; xmm0 = (crate::simd_ops::ru128(rax)) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x70u64), (xmm0) as u128); xmm1 = (crate::simd_ops::ru128(r9)) as u128; crate::simd_ops::wu128(r11.wrapping_sub(0x28u64), (xmm1) as u128); xmm0 = (crate::simd_ops::ru128(r8)) as u128; crate::simd_ops::wu128(r11.wrapping_sub(0x18u64), (xmm0) as u128); rax = ((r11.wrapping_sub(0x58u64)) as u64) as u64; crate::simd_ops::wu64(r11.wrapping_sub(0x70u64), (rax) as u64); rax = ((r11.wrapping_sub(0x48u64)) as u64) as u64; crate::simd_ops::wu64(r11.wrapping_sub(0x78u64), (rax) as u64); rax = ((r11.wrapping_sub(0x38u64)) as u64) as u64; crate::simd_ops::wu64(r11.wrapping_sub(0x80u64), (rax) as u64); rax = ((r11.wrapping_sub(0x28u64)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rax) as u64); r9 = ((r11.wrapping_sub(0x18u64)) as u64) as u64; r8 = (rdx) as u64; rdx = ((r11.wrapping_sub(0x68u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1421878b0); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = (rbx) as u64; rsp = ((rsp).wrapping_add(0xa0u64)) as u64; rbx = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142187fa0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14218b4a0; const P1: u64 = 0x142491220; const P2: u64 = 0x4; const P3: u64 = 0x14240ab10;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 15>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x142187fa0, funcinfo_rva: 0x37af1d8, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0x78, conts: &[] };
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64); rsp = ((rsp).wrapping_sub(0x70u64)) as u64; rbx = (rcx) as u64; rax = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (rdx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), (rax) as u64); xmm0 = (crate::simd_ops::ru128(r9)) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128); xmm1 = (crate::simd_ops::ru128(r8)) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm1) as u128); r9 = ((rsp.wrapping_add(0x20u64)) as u64) as u64; r8 = ((rsp.wrapping_add(0x40u64)) as u64) as u64; rcx = ((rsp.wrapping_add(0x50u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142187fda); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdx = ((crate::simd_ops::ru8(rsp.wrapping_add(0x53u64))) as u32) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = (0x80u32) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rdx = ((rdx.wrapping_mul(8)) as u32) as u32 as u64; bb = 3; continue; }
            2 => { rdx = (((rdx as u32)).wrapping_shl(8u32)) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = (0x7c00u32) as u32 as u64; rdx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x50u64))) as u32 as u64; rax = ((((rax as u32)) & (0xffffffu32))) as u32 as u64; rax = (((rax as u32)).wrapping_shl(3u32)) as u32 as u64; rcx = ((rsp.wrapping_add(0x58u64)) as u64) as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; rcx = (if (((fl_l as u32)) > ((fl_r as u32))) { crate::simd_ops::ru64(rsp.wrapping_add(0x58u64)) } else { rcx }) as u64; rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x54u64))) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rcx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (rax) as u64); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm0) as u128); rcx = ((rsp.wrapping_add(0x40u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142188037); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 6; continue; }
            5 => { crate::simd_ops::wu32(rbx, (0x23u32) as u32); rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 6; continue; }
            6 => { crate::simd_ops::wu8(rbx.wrapping_add(P2), ((rax as u8)) as u8); rcx = ((rsp.wrapping_add(0x50u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142188054); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = (rbx) as u64; rsp = ((rsp).wrapping_add(0x70u64)) as u64; rbx = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142188070(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14218b4a0; const P1: u64 = 0x142491220; const P2: u64 = 0x4; const P3: u64 = 0x14240ab10;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x142188070, funcinfo_rva: 0x37af208, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0x68, conts: &[] };
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64); rsp = ((rsp).wrapping_sub(0x60u64)) as u64; rdx = (r8) as u64; rbx = (rcx) as u64; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x90u64))) as u64; xmm0 = (crate::simd_ops::ru128(rax)) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128); xmm1 = (crate::simd_ops::ru128(r9)) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm1) as u128); r9 = ((rsp.wrapping_add(0x20u64)) as u64) as u64; r8 = ((rsp.wrapping_add(0x30u64)) as u64) as u64; rcx = ((rsp.wrapping_add(0x40u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1421880a8); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdx = ((crate::simd_ops::ru8(rsp.wrapping_add(0x43u64))) as u32) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = (0x80u32) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rdx = ((rdx.wrapping_mul(8)) as u32) as u32 as u64; bb = 3; continue; }
            2 => { rdx = (((rdx as u32)).wrapping_shl(8u32)) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = (0x7c00u32) as u32 as u64; rdx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x40u64))) as u32 as u64; rax = ((((rax as u32)) & (0xffffffu32))) as u32 as u64; rax = (((rax as u32)).wrapping_shl(3u32)) as u32 as u64; rcx = ((rsp.wrapping_add(0x48u64)) as u64) as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; rcx = (if (((fl_l as u32)) > ((fl_r as u32))) { crate::simd_ops::ru64(rsp.wrapping_add(0x48u64)) } else { rcx }) as u64; rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x44u64))) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rcx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (rax) as u64); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm0) as u128); rcx = ((rsp.wrapping_add(0x30u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142188105); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { crate::simd_ops::wu8(rbx.wrapping_add(P2), (0x1u8) as u8); bb = 6; continue; }
            5 => { crate::simd_ops::wu32(rbx, (0x23u32) as u32); crate::simd_ops::wu8(rbx.wrapping_add(P2), (0x0u8) as u8); bb = 6; continue; }
            6 => { rcx = ((rsp.wrapping_add(0x40u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142188123); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = (rbx) as u64; rsp = ((rsp).wrapping_add(0x60u64)) as u64; rbx = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142188dd0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    crate::simd_ops::forward_incoming_to_outgoing();
    const P0: u64 = 0x8; const P1: u64 = 0x28;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    rdx = (crate::simd_ops::ru64(rcx)) as u64;
    rcx = rax;
    r8 = (crate::simd_ops::ru64(rax)) as u64;
    rdx = (crate::simd_ops::ru64(rdx)) as u64;
    crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((crate::simd_ops::ru64(r8.wrapping_add(P1))) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1421890a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14218b2f0; const P1: u64 = 0x3; const P2: u64 = 0x4; const P3: u64 = 0x8; const P4: u64 = 0x14240a730; const P5: u64 = 0x18; const P6: u64 = 0x14240ab10;
    let (mut rax, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 19>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x1421890a0, funcinfo_rva: 0x37af3f4, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0x98, conts: &[] };
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rcx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64); rsp = ((rsp).wrapping_sub(0x90u64)) as u64; rbx = (rcx) as u64; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xc0u64))) as u64; xmm0 = (crate::simd_ops::ru128(rax)) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm0) as u128); xmm1 = (crate::simd_ops::ru128(r9)) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x50u64), (xmm1) as u128); xmm0 = (crate::simd_ops::ru128(r8)) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x60u64), (xmm0) as u128); rax = ((rsp.wrapping_add(0x40u64)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rax) as u64); r9 = ((rsp.wrapping_add(0x50u64)) as u64) as u64; r8 = ((rsp.wrapping_add(0x60u64)) as u64) as u64; rcx = ((rsp.wrapping_add(0x70u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1421890f1); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu64(rsp.wrapping_add(0xa0u64), (rbx) as u64); crate::simd_ops::lk_and_u32(rbx, (0xff000002u32) as u32); crate::simd_ops::lk_or_u32(rbx, (0x2u32) as u32); crate::simd_ops::wu8(rbx.wrapping_add(P1), (0x2u8) as u8); crate::simd_ops::wu32(rbx.wrapping_add(P2), (0x0u32) as u32); crate::simd_ops::wu8(rbx.wrapping_add(P3), (0x0u8) as u8); rdx = (rax) as u64; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14218911d); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu8(rbx.wrapping_add(P5), (0x1u8) as u8); rcx = ((rsp.wrapping_add(0x70u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14218912c); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = (rbx) as u64; rsp = ((rsp).wrapping_add(0x90u64)) as u64; rbx = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14218a2b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x30; const P1: u64 = 0x828; const P2: u64 = 0x1401df980; const P3: u64 = 0x1426cdcb0; const P4: u64 = 0x1401df850; const P5: u64 = 0x1401e06c0; const P6: u64 = 0x268; const P7: u64 = 0x14240a730; const P8: u64 = 0x14240ab10; const P9: u64 = 0x270; const P10: u64 = 0x26b; const P11: u64 = 0x26c; const P12: u64 = 0x143a08320; const P13: u64 = 0x14248e060;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut xmm6: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 17>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x14218a2b0, funcinfo_rva: 0x37af650, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0x88, conts: &[] };
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rbx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = ((rsp).wrapping_sub(0x80u64)) as u64; crate::simd_ops::wu128(rsp.wrapping_add(0x70u64), (xmm6) as u128); rbx = (rdx) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14218a2c9); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdi = (rax) as u64; rdx = ((rbx.wrapping_add(P1)) as u64) as u64; rcx = ((rsp.wrapping_add(0x40u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14218a2db); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; xmm6 = (crate::simd_ops::ru128(rax)) as u128; rax = ((P3) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rax) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (0x0u64) as u64); rdx = ((rsp.wrapping_add(0x20u64)) as u64) as u64; rcx = ((rsp.wrapping_add(0x30u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14218a303); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm6) as u128); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x30u64))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm0) as u128); r8 = ((rsp.wrapping_add(0x20u64)) as u64) as u64; rdx = ((rsp.wrapping_add(0x30u64)) as u64) as u64; rcx = ((rsp.wrapping_add(0x50u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14218a329); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdx = (rax) as u64; rcx = ((rdi.wrapping_add(P6)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14218a339); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P7) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = ((rsp.wrapping_add(0x50u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14218a344); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P8) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdx = ((rdi.wrapping_add(P9)) as u64) as u64; rcx = ((crate::simd_ops::ru8(rdi.wrapping_add(P10))) as u32) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = (0x80u32) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rcx = ((rcx.wrapping_mul(8)) as u32) as u32 as u64; bb = 3; continue; }
            2 => { rcx = (((rcx as u32)).wrapping_shl(8u32)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = (0x7c00u32) as u32 as u64; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru32(rdi.wrapping_add(P6))) as u32 as u64; rax = ((((rax as u32)) & (0xffffffu32))) as u32 as u64; rax = (((rax as u32)).wrapping_shl(3u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 5; continue; } bb = 4; continue; }
            4 => { rdx = (crate::simd_ops::ru64(rdx)) as u64; bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru32(rdi.wrapping_add(P11))) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rdx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (rax) as u64); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm0) as u128); rdx = (crate::simd_ops::ru64(P12)) as u64; rcx = ((rsp.wrapping_add(0x30u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14218a3ae); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P13) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = (0x1u32) as u32 as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x98u64))) as u64; xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x70u64))) as u128; rsp = ((rsp).wrapping_add(0x80u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14218a3e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14340ecd8; const P1: u64 = 0x142497fd0; const P2: u64 = 0x142498740; const P3: u64 = 0x142497c10; const P4: u64 = 0x143a08320; const P5: u64 = 0x142498af0; const P6: u64 = 0x14183acc2; const P7: u64 = 0x1423f3530; const P8: u64 = 0x1401df680; const P9: u64 = 0x1413d0070; const P10: u64 = 0x1413d2510; const P11: u64 = 0x14218a509;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 179>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x14218a3e0, funcinfo_rva: 0x37af684, handler: 1, shape: 2, frame_reg: 0, establisher_off: 0x598, conts: &[0x14218a4a1] };
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rcx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = ((rsp).wrapping_sub(0x590u64)) as u64; rdi = (r9) as u64; rbx = (rcx) as u64; rax = ((P0) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), (rax) as u64); rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x48u64))) as u32 as u64; rax = ((((rax as u32)) & (0xff000020u32))) as u32 as u64; rax = ((((rax as u32)) | (0x20u32))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x48u64), ((rax as u32)) as u32); crate::simd_ops::wu8(rsp.wrapping_add(0x4bu64), (0x20u8) as u8); crate::simd_ops::wu32(rsp.wrapping_add(0x4cu64), (0x0u32) as u32); crate::simd_ops::wu8(rsp.wrapping_add(0x50u64), (0x0u8) as u8); xmm0 = (crate::simd_ops::ru128(r8)) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128); r8 = (0x1u32) as u32 as u64; rdx = ((rsp.wrapping_add(0x20u64)) as u64) as u64; rcx = ((rsp.wrapping_add(0x40u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14218a441); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = match eh.call_xmm(0x14218a441, (P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]) { Ok(v) => v, Err(l) => match l.cont_va { 0x14218a4a1 => { rax = crate::dll_native::lift::eh::landed_rax(); bb = 2; continue; } _ => l.unknown(0x14218a3e0) } }; rax = c_rax; xmm0 = c_x0; r8 = (crate::simd_ops::ru64(rdi)) as u64; rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; rcx = ((rsp.wrapping_add(0x40u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14218a453); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = match eh.call_xmm(0x14218a453, (P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]) { Ok(v) => v, Err(l) => match l.cont_va { 0x14218a4a1 => { rax = crate::dll_native::lift::eh::landed_rax(); bb = 2; continue; } _ => l.unknown(0x14218a3e0) } }; rax = c_rax; xmm0 = c_x0; rcx = ((rsp.wrapping_add(0x40u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14218a45d); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = match eh.call_xmm(0x14218a45d, (P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]) { Ok(v) => v, Err(l) => match l.cont_va { 0x14218a4a1 => { rax = crate::dll_native::lift::eh::landed_rax(); bb = 2; continue; } _ => l.unknown(0x14218a3e0) } }; rax = c_rax; xmm0 = c_x0; rcx = (crate::simd_ops::ru64(P4)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x158u64))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14218a474); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = match eh.call_xmm(0x14218a474, ({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0xa8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]) { Ok(v) => v, Err(l) => match l.cont_va { 0x14218a4a1 => { rax = crate::dll_native::lift::eh::landed_rax(); bb = 2; continue; } _ => l.unknown(0x14218a3e0) } }; rax = c_rax; xmm0 = c_x0; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { rcx = ((rsp.wrapping_add(0x40u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14218a483); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = match eh.call_xmm(0x14218a483, (P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]) { Ok(v) => v, Err(l) => match l.cont_va { 0x14218a4a1 => { rax = crate::dll_native::lift::eh::landed_rax(); bb = 2; continue; } _ => l.unknown(0x14218a3e0) } }; rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu8(rbx.wrapping_add(0x4u64), (0x1u8) as u8); rax = (rbx) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x5a8u64))) as u64; rsp = ((rsp).wrapping_add(0x590u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            2 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x5a0u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x5a8u64))) as u64; rsp = ((rsp).wrapping_add(0x590u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            3 => { rdx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; r8 = (0x400u32) as u32 as u64; rcx = ((rsp.wrapping_add(0x190u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14218a4ca); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = match eh.call_xmm(0x14218a4ca, (P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]) { Ok(v) => v, Err(l) => match l.cont_va { 0x14218a4a1 => { rax = crate::dll_native::lift::eh::landed_rax(); bb = 2; continue; } _ => l.unknown(0x14218a3e0) } }; rax = c_rax; xmm0 = c_x0; rdx = (0x400u32) as u32 as u64; rcx = ((rsp.wrapping_add(0x190u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14218a4dc); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = match eh.call_xmm(0x14218a4dc, (P7) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]) { Ok(v) => v, Err(l) => match l.cont_va { 0x14218a4a1 => { rax = crate::dll_native::lift::eh::landed_rax(); bb = 2; continue; } _ => l.unknown(0x14218a3e0) } }; rax = c_rax; xmm0 = c_x0; rcx = ((rsp.wrapping_add(0x48u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14218a4e6); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = match eh.call_xmm(0x14218a4e6, (P8) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]) { Ok(v) => v, Err(l) => match l.cont_va { 0x14218a4a1 => { rax = crate::dll_native::lift::eh::landed_rax(); bb = 2; continue; } _ => l.unknown(0x14218a3e0) } }; rax = c_rax; xmm0 = c_x0; rdx = (rax) as u64; r8 = ((rsp.wrapping_add(0x190u64)) as u64) as u64; rcx = ((rsp.wrapping_add(0x20u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14218a4fb); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = match eh.call_xmm(0x14218a4fb, (P9) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]) { Ok(v) => v, Err(l) => match l.cont_va { 0x14218a4a1 => { rax = crate::dll_native::lift::eh::landed_rax(); bb = 2; continue; } _ => l.unknown(0x14218a3e0) } }; rax = c_rax; xmm0 = c_x0; rcx = (rax) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14218a504); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = match eh.call_xmm(0x14218a504, (P10) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]) { Ok(v) => v, Err(l) => match l.cont_va { 0x14218a4a1 => { rax = crate::dll_native::lift::eh::landed_rax(); bb = 2; continue; } _ => l.unknown(0x14218a3e0) } }; rax = c_rax; xmm0 = c_x0; crate::lift_ops::trap(0x80000003, P11, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14218a700(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142491220;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rsp = ((rsp).wrapping_sub(0x38u64)) as u64;
    xmm0 = (crate::simd_ops::ru128(rdx)) as u128;
    rcx = rsp.wrapping_add(0x20);
    crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128);
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    rsp = ((rsp).wrapping_add(0x38u64)) as u64;
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14218a730(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143a08320; const P1: u64 = 0x50;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rsp = ((rsp).wrapping_sub(0x38u64)) as u64;
    xmm0 = (crate::simd_ops::ru128(rdx)) as u128;
    rcx = (crate::simd_ops::ru64(P0)) as u64;
    rdx = rsp.wrapping_add(0x20);
    crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128);
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    (rax, xmm0) = crate::simd_ops::call_stk_xmm((crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    rsp = ((rsp).wrapping_add(0x38u64)) as u64;
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14218b000(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    crate::simd_ops::forward_incoming_to_outgoing();
    const P0: u64 = 0x8; const P1: u64 = 0xa0;
    let (mut rax, mut r10) = (crate::simd_ops::get_rax_in(), 0u64);
    rax = (crate::simd_ops::ru64(rdx)) as u64;
    r10 = rdx;
    r9 = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    rdx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    rcx = r10;
    r8 = (crate::simd_ops::ru64(r8)) as u64;
    crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14218b220(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x30; const P1: u64 = 0x1401df850; const P2: u64 = 0x26b; const P3: u64 = 0x270; const P4: u64 = 0x268; const P5: u64 = 0x26c; const P6: u64 = 0x1401e06c0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x60u64)) as u64; rax = (crate::simd_ops::ru64(rdx)) as u64; rsi = rcx; rcx = rdx; rbx = r8; (rax, xmm0) = crate::simd_ops::call_stk_xmm((crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm0 = (crate::simd_ops::ru128(rbx)) as u128; rdx = rsp.wrapping_add(0x30); rdi = rax; rcx = rsp.wrapping_add(0x50); crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm0) as u128); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = ((crate::simd_ops::ru8(rdi.wrapping_add(P2))) as u32) as u32 as u64; rdx = ((rdi.wrapping_add(P3)) as u64) as u64; r8 = rax; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x80; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 1; continue; } rcx = ((rcx.wrapping_mul(8)) as u32) as u32 as u64; bb = 2; continue; }
            1 => { rcx = (((rcx as u32)).wrapping_shl(8u32)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x7c00; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru32(rdi.wrapping_add(P4))) as u32 as u64; rax = ((((rax as u32)) & (0xffffffu32))) as u32 as u64; rax = (((rax as u32)).wrapping_shl(3u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 3; continue; } rdx = (crate::simd_ops::ru64(rdx)) as u64; bb = 3; continue; }
            3 => { xmm0 = (crate::simd_ops::ru128(r8)) as u128; rax = (crate::simd_ops::ru32(rdi.wrapping_add(P5))) as u32 as u64; r8 = rsp.wrapping_add(0x40); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rdx); rcx = rsi; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rax); rdx = rsp.wrapping_add(0x30); xmm1 = (crate::simd_ops::ru128(rsp.wrapping_add(0x30u64))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm1) as u128); crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm0) as u128); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rax = rsi; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; rsp = ((rsp).wrapping_add(0x60u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14218b600(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x30; const P1: u64 = 0x26b; const P2: u64 = 0x270; const P3: u64 = 0x268; const P4: u64 = 0x26c; const P5: u64 = 0x1401e06c0;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x50u64)) as u64; rax = (crate::simd_ops::ru64(rdx)) as u64; rbx = rcx; rcx = rdx; rdi = r8; (rax, xmm0) = crate::simd_ops::call_stk_xmm((crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r9 = rax; rdx = ((crate::simd_ops::ru8(rax.wrapping_add(P1))) as u32) as u32 as u64; r10 = ((rax.wrapping_add(P2)) as u64) as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0x80; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 1; continue; } rdx = ((rdx.wrapping_mul(8)) as u32) as u32 as u64; bb = 2; continue; }
            1 => { rdx = (((rdx as u32)).wrapping_shl(8u32)) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0x7c00; rdx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru32(rax.wrapping_add(P3))) as u32 as u64; rax = ((((rax as u32)) & (0xffffffu32))) as u32 as u64; rax = (((rax as u32)).wrapping_shl(3u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 3; continue; } r10 = (crate::simd_ops::ru64(r10)) as u64; bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru32(r9.wrapping_add(P4))) as u32 as u64; r8 = rsp.wrapping_add(0x40); xmm0 = (crate::simd_ops::ru128(rdi)) as u128; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rax); rdx = rsp.wrapping_add(0x30); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), r10); rcx = rbx; xmm1 = (crate::simd_ops::ru128(rsp.wrapping_add(0x30u64))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm1) as u128); crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm0) as u128); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = rbx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rsp = ((rsp).wrapping_add(0x50u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14218e030(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x30; const P1: u64 = 0x110; const P2: u64 = 0x130; const P3: u64 = 0x14129bf30; const P4: u64 = 0x108; const P5: u64 = 0x142645748; const P6: u64 = 0x143406c90; const P7: u64 = 0x1423f0780; const P8: u64 = 0x14136eec0; const P9: u64 = 0x14218e09d;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x14218e030, funcinfo_rva: 0x37b00b8, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0x38, conts: &[] };
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rbx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rdi = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rax = (crate::simd_ops::ru64(rcx)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14218e03f); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rbx = ((rax.wrapping_add(P1)) as u64) as u64; let tmp_xchg: u8 = crate::simd_ops::atomic_xchg_u8(rbx.wrapping_add(P2), ((rdi as u8)) as u8); rdi = (rdi & !0xffu64) | (((tmp_xchg) as u8 as u64)); rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14218e053); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P4))) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rdx = (0xffffffffu32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14218e069); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P5)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { rax = (0x1u32) as u32 as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            3 => { rdx = ((P6) as u64) as u64; rcx = ((rsp.wrapping_add(0x20u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14218e08f); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P7) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rcx = (rax) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14218e098); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P8) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); crate::lift_ops::trap(0x80000003, P9, rcx, rsp); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14218e920(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x88; const P1: u64 = 0x78; const P2: u64 = 0x141299040;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x38u64)) as u64;
    rax = ((rdx.wrapping_add(P0)) as u64) as u64;
    r8 = ((rdx.wrapping_add(P1)) as u64) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax);
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r8);
    rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x38u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14218edc0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141299b50; const P1: u64 = 0x14243b5f0; const P2: u64 = 0x14183acc2; const P3: u64 = 0x1433e7798; const P4: u64 = 0x1426469b8; const P5: u64 = 0x1433e7548;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 41>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x140u64)) as u64; rbx = rcx; crate::simd_ops::wu64(rsp.wrapping_add(0x150u64), rcx); rdi = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rcx, ((rdi as u32)) as u32); rcx = ((rcx).wrapping_add(0x8u64)) as u64; rdx = ((rdi.wrapping_add(0x20u64)) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rbx.wrapping_add(0xa8); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (0x8u64) as u64); crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((rdi as u32)) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (0x100u64) as u64); r9 = ((rdi.wrapping_add(0x20u64)) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rbx.wrapping_add(0xe8u64), rdi); crate::simd_ops::wu64(rbx.wrapping_add(0xf0u64), rdi); crate::simd_ops::wu32(rbx.wrapping_add(0xf8u64), ((rdi as u32)) as u32); crate::simd_ops::wu8(rbx.wrapping_add(0xfcu64), ((rdi as u8)) as u8); crate::simd_ops::wu64(rbx.wrapping_add(0x100u64), rdi); crate::simd_ops::wu64(rbx.wrapping_add(0x108u64), rdi); crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rdi); crate::simd_ops::wu32(rsp.wrapping_add(0x48u64), (0xffffffffu32) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x4cu64), (0x4u32) as u32); crate::simd_ops::wu8(rsp.wrapping_add(0x50u64), ((rdi as u8)) as u8); rdx = 0; fl_l = 0; fl_r = 0; r8 = 0xef; rcx = rsp.wrapping_add(0x51); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r8 = P3; rdx = 0xef; rcx = rsp.wrapping_add(0x51); (rax, xmm0) = crate::simd_ops::call_stk_xmm((crate::simd_ops::ru64(P4)) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rbx.wrapping_add(0x118); rax = rsp.wrapping_add(0x40); rdx = ((rdi.wrapping_add(0x2u64)) as u32) as u32 as u64; bb = 1; continue; }
            1 => { let y_addr: u64 = rax; xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rcx; crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); let y_addr: u64 = rax.wrapping_add(0x20u64); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rcx.wrapping_add(0x20u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); let y_addr: u64 = rax.wrapping_add(0x40u64); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rcx.wrapping_add(0x40u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); xmm0 = (crate::simd_ops::ru128(rax.wrapping_add(0x60u64))) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rcx.wrapping_add(0x60u64), (xmm0) as u128); rcx = rcx.wrapping_add(0x80); xmm1 = (crate::simd_ops::ru128(rax.wrapping_add(0x70u64))) as u128; crate::simd_ops::wu128(rcx.wrapping_sub(0x10u64), (xmm1) as u128); rax = rax.wrapping_add(0x80); fl_l = rdx; fl_r = (0x1u64) as u64; rdx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } crate::simd_ops::wu64(rbx.wrapping_add(0x218u64), rdi); crate::simd_ops::wu64(rbx.wrapping_add(0x220u64), rdi); crate::simd_ops::wu8(rbx.wrapping_add(0x228u64), ((rdx as u8)) as u8); rax = P5; crate::simd_ops::wu64(rbx.wrapping_add(0x110u64), rax); crate::simd_ops::wu64(rbx.wrapping_add(0x230u64), rdi); crate::simd_ops::wu64(rbx.wrapping_add(0x238u64), rdi); crate::simd_ops::wu8(rbx.wrapping_add(0x240u64), (0x1u8) as u8); crate::simd_ops::wu64(rbx.wrapping_add(0x248u64), rdi); crate::simd_ops::wu64(rbx.wrapping_add(0x250u64), rdi); crate::simd_ops::wu64(rbx.wrapping_add(0x258u64), rdi); crate::simd_ops::wu64(rbx.wrapping_add(0x260u64), rdi); crate::simd_ops::lk_and_u32(rbx.wrapping_add(0x268u64), (0xff000002u32) as u32); crate::simd_ops::lk_or_u32(rbx.wrapping_add(0x268u64), (0x2u32) as u32); crate::simd_ops::wu8(rbx.wrapping_add(0x26bu64), (0x2u8) as u8); crate::simd_ops::wu32(rbx.wrapping_add(0x26cu64), ((rdi as u32)) as u32); crate::simd_ops::wu8(rbx.wrapping_add(0x270u64), ((rdx as u8)) as u8); crate::simd_ops::wu64(rbx.wrapping_add(0x280u64), rdi); rax = rbx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x158u64))) as u64; rsp = ((rsp).wrapping_add(0x140u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14218f370(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x8;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = (crate::simd_ops::ru64(rdx)) as u64;
    crate::simd_ops::wu64(rdx, (0x0u64) as u64);
    crate::simd_ops::wu64(rcx, rax);
    crate::simd_ops::wu64(rcx.wrapping_add(P0), rax);
    rax = rcx;
    crate::simd_ops::wu64(rcx.wrapping_add(P1), r8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14218f950(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x4; const P1: u64 = 0x14049ebe0; const P2: u64 = 0x14049fb80; const P3: u64 = 0x14049d200; const P4: u64 = 0x14049eb20; const P5: u64 = 0x1401e4ba0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), r8); crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rbx = rcx; rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rax as u32)) as u32); crate::simd_ops::wu8(rsp.wrapping_add(0x64u64), (0x1u8) as u8); rcx = rsp.wrapping_add(0x28); fl_l = (crate::simd_ops::ru8(r9.wrapping_add(P0))) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 1; continue; } crate::simd_ops::wu32(rsp.wrapping_add(0x60u64), ((rax as u32)) as u32); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rdx = rsp.wrapping_add(0x60); rcx = rsp.wrapping_add(0x28); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); r8 = 0; fl_l = 0; fl_r = 0; rdx = rbx; rcx = rsp.wrapping_add(0x28); rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rcx = rsp.wrapping_add(0x28); rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); bb = 2; continue; }
            1 => { crate::simd_ops::wu32(rsp.wrapping_add(0x60u64), (0x1u32) as u32); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rdx = rsp.wrapping_add(0x60); rcx = rsp.wrapping_add(0x28); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); r8 = 0; fl_l = 0; fl_r = 0; rdx = rbx; rcx = rsp.wrapping_add(0x28); rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rcx = rsp.wrapping_add(0x28); rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); bb = 2; continue; }
            2 => { rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rax = rbx; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14218fdd0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x4; const P1: u64 = 0x14049ebe0; const P2: u64 = 0x14049fb80; const P3: u64 = 0x14049d200; const P4: u64 = 0x14049eb20; const P5: u64 = 0x1401e4ba0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), r8); crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rbx = rcx; rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rax as u32)) as u32); crate::simd_ops::wu8(rsp.wrapping_add(0x64u64), (0x1u8) as u8); rcx = rsp.wrapping_add(0x28); fl_l = (crate::simd_ops::ru8(r9.wrapping_add(P0))) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 1; continue; } crate::simd_ops::wu32(rsp.wrapping_add(0x60u64), ((rax as u32)) as u32); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rdx = rsp.wrapping_add(0x60); rcx = rsp.wrapping_add(0x28); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); r8 = 0; fl_l = 0; fl_r = 0; rdx = rbx; rcx = rsp.wrapping_add(0x28); rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rcx = rsp.wrapping_add(0x28); rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); bb = 2; continue; }
            1 => { crate::simd_ops::wu32(rsp.wrapping_add(0x60u64), (0x1u32) as u32); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rdx = rsp.wrapping_add(0x60); rcx = rsp.wrapping_add(0x28); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); r8 = 0; fl_l = 0; fl_r = 0; rdx = rbx; rcx = rsp.wrapping_add(0x28); rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rcx = rsp.wrapping_add(0x28); rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); bb = 2; continue; }
            2 => { rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rax = rbx; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142190390(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x30; const P1: u64 = 0x34; const P2: u64 = 0x28; const P3: u64 = 0x20; const P4: u64 = 0xc; const P5: u64 = 0x18; const P6: u64 = 0x10; const P7: u64 = 0x8;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 3>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x10u64)) as u64; rdi = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; rbx = 0; fl_l = 0; fl_r = 0; r10 = ((rbx as u32)) as u32 as u64; r8 = rdx; r9 = rcx; bb = 1; continue; }
            1 => { rax = (((crate::simd_ops::ru64(r9)) as i64).wrapping_mul((0xf4240u64) as i64) as u64) as u64; fl_l = ((rbx as u32)) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 2; continue; } rdx = ((rbx as u32)) as u32 as u64; r11 = ((r10 as u32)) as u32 as u64; r10 = (crate::simd_ops::ru32(r9.wrapping_add(P1))) as u32 as u64; rcx = r8; rcx = ((rcx).wrapping_sub(rax)) as u64; rbx = (((rbx as u32)).wrapping_add(1)) as u32 as u64; rax = (crate::simd_ops::ru32(r9.wrapping_add(P2))) as u32 as u64; rax = ((rax).wrapping_add(rdx)) as u64; rdx = 0; fl_l = 0; fl_r = 0; let d = (r10) as u128; let n = ((rdx as u128) << 64) | (rax as u128); if d != 0 { rax = (n / d) as u64; rdx = (n % d) as u64; } rax = (crate::simd_ops::ru64(r9.wrapping_add(P3))) as u64; r10 = ((r11.wrapping_add(0x1u64)) as u32) as u32 as u64; fl_l = rcx; fl_r = (crate::simd_ops::ru64(rax.wrapping_add(rdx.wrapping_mul(8)))) as u64; r10 = (if ((((fl_l as u64)) as i64) >= (((fl_r as u64)) as i64)) { (r11 as u32) } else { (r10 as u32) }) as u32 as u64; bb = 1; continue; }
            2 => { r10 = ((((r10 as u32)) as i32).wrapping_mul((crate::simd_ops::ru32(r9.wrapping_add(P4))) as i32) as u32) as u32 as u64; r8 = ((r8).wrapping_sub(rax)) as u64; fl_l = r8; fl_r = (crate::simd_ops::ru64(r9.wrapping_add(P5))) as u64; if ((((fl_l as u64)) as i64) >= (((fl_r as u64)) as i64)) { bb = 3; continue; } fl_l = (((r10 as u32)).wrapping_add(crate::simd_ops::ru32(r9.wrapping_add(P6)))) as u32 as u64; fl_r = fl_l; r10 = ((fl_l as u32)) as u32 as u64; bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru32(r9.wrapping_add(P7))) as u32 as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; rax = (((rax as u32)).wrapping_sub((r10 as u32))) as u32 as u64; rsp = ((rsp).wrapping_add(0x10u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1421905a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x18; const P1: u64 = 0x28; const P2: u64 = 0x30; const P3: u64 = 0x8; const P4: u64 = 0x10;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    fl_l = rax; fl_r = rdx;
    rax = (if ((((fl_l as u64)) as i64) < (((fl_r as u64)) as i64)) { rdx } else { rax }) as u64;
    rdx = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu64(rcx.wrapping_add(P1), rdx);
    crate::simd_ops::wu32(rcx.wrapping_add(P2), ((rdx as u32)) as u32);
    crate::simd_ops::wu64(rcx.wrapping_add(P0), rax);
    rax = (crate::simd_ops::ru32(rcx.wrapping_add(P3))) as u32 as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(P4), ((rax as u32)) as u32);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1421905d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x18; const P1: u64 = 0x30; const P2: u64 = 0x34; const P3: u64 = 0x28; const P4: u64 = 0x20; const P5: u64 = 0xc; const P6: u64 = 0x10; const P7: u64 = 0x8;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rdi) = (0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 3>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x10u64)) as u64; r8 = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; r9 = rcx; rdi = (crate::simd_ops::ru32(rcx.wrapping_add(P1))) as u32 as u64; fl_l = r8; fl_r = rdx; r8 = (if ((((fl_l as u64)) as i64) < (((fl_r as u64)) as i64)) { rdx } else { r8 }) as u64; rbx = 0; fl_l = 0; fl_r = 0; r10 = ((rbx as u32)) as u32 as u64; bb = 1; continue; }
            1 => { rax = (((crate::simd_ops::ru64(r9)) as i64).wrapping_mul((0xf4240u64) as i64) as u64) as u64; fl_l = ((rbx as u32)) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 2; continue; } rdx = ((rbx as u32)) as u32 as u64; r11 = ((r10 as u32)) as u32 as u64; r10 = (crate::simd_ops::ru32(r9.wrapping_add(P2))) as u32 as u64; rcx = r8; rcx = ((rcx).wrapping_sub(rax)) as u64; rbx = (((rbx as u32)).wrapping_add(1)) as u32 as u64; rax = (crate::simd_ops::ru32(r9.wrapping_add(P3))) as u32 as u64; rax = ((rax).wrapping_add(rdx)) as u64; rdx = 0; fl_l = 0; fl_r = 0; let d = (r10) as u128; let n = ((rdx as u128) << 64) | (rax as u128); if d != 0 { rax = (n / d) as u64; rdx = (n % d) as u64; } rax = (crate::simd_ops::ru64(r9.wrapping_add(P4))) as u64; r10 = ((r11.wrapping_add(0x1u64)) as u32) as u32 as u64; fl_l = rcx; fl_r = (crate::simd_ops::ru64(rax.wrapping_add(rdx.wrapping_mul(8)))) as u64; r10 = (if ((((fl_l as u64)) as i64) >= (((fl_r as u64)) as i64)) { (r11 as u32) } else { (r10 as u32) }) as u32 as u64; bb = 1; continue; }
            2 => { r10 = ((((r10 as u32)) as i32).wrapping_mul((crate::simd_ops::ru32(r9.wrapping_add(P5))) as i32) as u32) as u32 as u64; r8 = ((r8).wrapping_sub(rax)) as u64; fl_l = r8; fl_r = (crate::simd_ops::ru64(r9.wrapping_add(P0))) as u64; if ((((fl_l as u64)) as i64) >= (((fl_r as u64)) as i64)) { bb = 3; continue; } fl_l = (((r10 as u32)).wrapping_add(crate::simd_ops::ru32(r9.wrapping_add(P6)))) as u32 as u64; fl_r = fl_l; r10 = ((fl_l as u32)) as u32 as u64; bb = 3; continue; }
            3 => { fl_l = (crate::simd_ops::ru32(r9.wrapping_add(P7))) as u32 as u64; fl_r = ((r10 as u32)) as u32 as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; rax = (rax & !0xffu64) | (((if (((fl_l as u32)) != ((fl_r as u32))) { 1u8 } else { 0u8 }) as u8 as u64)); rsp = ((rsp).wrapping_add(0x10u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142190ec0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1401fa0a0; const P1: u64 = 0x420; const P2: u64 = 0x14240ab10; const P3: u64 = 0x3f8; const P4: u64 = 0x3d8; const P5: u64 = 0x3b0; const P6: u64 = 0x388; const P7: u64 = 0x360; const P8: u64 = 0x338; const P9: u64 = 0x1e8; const P10: u64 = 0x142191030; const P11: u64 = 0x1c8; const P12: u64 = 0x1401ea690; const P13: u64 = 0x141296050; const P14: u64 = 0x1421910b0; const P15: u64 = 0x140fe3a40; const P16: u64 = 0x140fe2920;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = rcx; rcx = ((rcx).wrapping_add(0x450u64)) as u64; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rcx = ((rdi.wrapping_add(P1)) as u64) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rcx = ((rdi.wrapping_add(P3)) as u64) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rcx = ((rdi.wrapping_add(P4)) as u64) as u64; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rcx = ((rdi.wrapping_add(P5)) as u64) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rcx = ((rdi.wrapping_add(P6)) as u64) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rcx = ((rdi.wrapping_add(P7)) as u64) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rcx = ((rdi.wrapping_add(P8)) as u64) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rcx = ((rdi.wrapping_add(P9)) as u64) as u64; rax = crate::simd_ops::call_stk(P10, rax, rcx, rdx, r8, r9, rsp); rcx = ((rdi.wrapping_add(P11)) as u64) as u64; fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0x18u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 1; continue; } fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0x10u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 1; continue; } rax = crate::simd_ops::call_stk(P12, rax, rcx, rdx, r8, r9, rsp); bb = 1; continue; }
            1 => { rcx = rdi.wrapping_add(0x1b8); rax = crate::simd_ops::call_stk(P13, rax, rcx, rdx, r8, r9, rsp); rcx = rdi.wrapping_add(0x30); rax = crate::simd_ops::call_stk(P14, rax, rcx, rdx, r8, r9, rsp); rcx = rdi.wrapping_add(0x10); fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0x18u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 2; continue; } fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0x10u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 2; continue; } rax = crate::simd_ops::call_stk(P15, rax, rcx, rdx, r8, r9, rsp); bb = 2; continue; }
            2 => { rcx = rdi; rax = crate::simd_ops::call_stk(P16, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1421910b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1401fa0a0; const P1: u64 = 0x140; const P2: u64 = 0x14240ab10; const P3: u64 = 0x118; const P4: u64 = 0xf8; const P5: u64 = 0x18; const P6: u64 = 0x10; const P7: u64 = 0x140fe3a40; const P8: u64 = 0xe8; const P9: u64 = 0x140fe2920; const P10: u64 = 0xc0; const P11: u64 = 0x98; const P12: u64 = 0x1401f33e0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rcx; rcx = ((rcx).wrapping_add(0x168u64)) as u64; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rcx = ((rbx.wrapping_add(P1)) as u64) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rcx = ((rbx.wrapping_add(P3)) as u64) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rcx = ((rbx.wrapping_add(P4)) as u64) as u64; fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(P5))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 1; continue; } fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(P6))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 1; continue; } rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); bb = 1; continue; }
            1 => { rcx = ((rbx.wrapping_add(P8)) as u64) as u64; rax = crate::simd_ops::call_stk(P9, rax, rcx, rdx, r8, r9, rsp); rcx = ((rbx.wrapping_add(P10)) as u64) as u64; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rcx = ((rbx.wrapping_add(P11)) as u64) as u64; rax = crate::simd_ops::call_stk(P12, rax, rcx, rdx, r8, r9, rsp); rcx = rbx.wrapping_add(0x78); rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rcx = rbx.wrapping_add(0x30); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rcx = rbx.wrapping_add(0x8); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(P2, rax, rcx, rdx, r8, r9, rsp); }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142191320(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14240ab10; const P1: u64 = 0x1401fa0a0; const P2: u64 = 0x1412a6710; const P3: u64 = 0x1412a4410; const P4: u64 = 0x1412a3fc0;
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = rcx; rcx = ((rcx).wrapping_add(0x9c0u64)) as u64; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rcx = rdi.wrapping_add(0x9a0); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rcx = rdi.wrapping_add(0x978); rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rcx = rdi.wrapping_add(0x958); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rcx = rdi.wrapping_add(0x930); rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rcx = rdi.wrapping_add(0x910); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rcx = rdi.wrapping_add(0x8e8); rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rcx = rdi.wrapping_add(0x8c8); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rcx = rdi.wrapping_add(0x8a0); rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rcx = rdi.wrapping_add(0x880); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rcx = rdi.wrapping_add(0x858); rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rcx = rdi.wrapping_add(0x838); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rcx = rdi.wrapping_add(0x810); rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rcx = rdi.wrapping_add(0x7f0); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rcx = rdi.wrapping_add(0x7c8); rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rcx = rdi.wrapping_add(0x7a8); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rcx = rdi.wrapping_add(0x780); rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rcx = rdi.wrapping_add(0x760); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rcx = rdi.wrapping_add(0x738); rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rcx = rdi.wrapping_add(0x718); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rcx = rdi.wrapping_add(0x6f0); rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rcx = rdi.wrapping_add(0x6d0); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rcx = rdi.wrapping_add(0x6a8); rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rcx = rdi.wrapping_add(0x688); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rcx = rdi.wrapping_add(0x660); rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rcx = rdi.wrapping_add(0x640); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rcx = rdi.wrapping_add(0x618); rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rcx = rdi.wrapping_add(0x5f8); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rcx = rdi.wrapping_add(0x5d0); rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rcx = rdi.wrapping_add(0x5b0); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rcx = rdi.wrapping_add(0x588); rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rcx = rdi.wrapping_add(0x568); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rcx = rdi.wrapping_add(0x3c0); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rcx = rdi.wrapping_add(0x390); rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rcx = rdi.wrapping_add(0x360); rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rcx = rdi.wrapping_add(0x330); rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rcx = rdi.wrapping_add(0x300); rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rcx = rdi; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(P4, rax, rcx, rdx, r8, r9, rsp); }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142191570(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x148; const P2: u64 = 0x10; const P3: u64 = 0x18; const P4: u64 = 0x142535070; const P5: u64 = 0x1426456c8; const P6: u64 = 0x1426453a0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rax = ((crate::exe::tls_pointer() as u64)) as u64; rbx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rcx, rbx); rdi = rcx; crate::simd_ops::wu64(rcx.wrapping_add(P0), rbx); rcx = 0x6348; rdx = (crate::simd_ops::ru64(rax)) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(rdx))) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64; rdx = (crate::simd_ops::ru64(rcx.wrapping_add(P2))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } fl_l = ((rdx as u8)) as u8 as u64; fl_r = (0xfu8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 3; continue; } fl_l = (crate::simd_ops::lk_add_u32(rcx.wrapping_add(P3), (1) as u32)) as u32 as u64; rax = (crate::simd_ops::ru64(rdx)) as u64; crate::simd_ops::wu64(rcx.wrapping_add(P2), rax); bb = 1; continue; }
            1 => { rbx = rdx; bb = 2; continue; }
            2 => { crate::simd_ops::wu64(rbx, rbx); rax = rdi; crate::simd_ops::wu64(rbx.wrapping_add(P0), rbx); crate::simd_ops::wu64(rbx.wrapping_add(P2), rbx); crate::simd_ops::wu16(rbx.wrapping_add(P3), (0x101u16) as u16); crate::simd_ops::wu64(rdi, rbx); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            3 => { rdx = 0x140; r8 = 0x10; rcx = rax; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rdx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 1; continue; } rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P5)) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = rax; rdx = 0xdeadd00d; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P6)) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 2; continue; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1421916a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x4;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    crate::simd_ops::wu32(rcx, (0x1u32) as u32);
    rax = rcx;
    crate::simd_ops::wu32(rcx.wrapping_add(P0), (0x1u32) as u32);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1421916c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1412a64e0; const P1: u64 = 0x1430782a0;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x28u64)) as u64; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rdx = 0; fl_l = 0; fl_r = 0; rax = P1; crate::simd_ops::wu64(rcx.wrapping_add(0x300u64), rax); crate::simd_ops::wu64(rcx.wrapping_add(0x308u64), rdx); crate::simd_ops::wu64(rcx.wrapping_add(0x310u64), rdx); crate::simd_ops::wu64(rcx.wrapping_add(0x318u64), rdx); crate::simd_ops::wu64(rcx.wrapping_add(0x328u64), rdx); crate::simd_ops::wu64(rcx.wrapping_add(0x330u64), rax); crate::simd_ops::wu64(rcx.wrapping_add(0x338u64), rdx); crate::simd_ops::wu64(rcx.wrapping_add(0x340u64), rdx); crate::simd_ops::wu64(rcx.wrapping_add(0x348u64), rdx); crate::simd_ops::wu64(rcx.wrapping_add(0x358u64), rdx); crate::simd_ops::wu64(rcx.wrapping_add(0x360u64), rax); crate::simd_ops::wu64(rcx.wrapping_add(0x368u64), rdx); crate::simd_ops::wu64(rcx.wrapping_add(0x370u64), rdx); crate::simd_ops::wu64(rcx.wrapping_add(0x378u64), rdx); crate::simd_ops::wu64(rcx.wrapping_add(0x388u64), rdx); crate::simd_ops::wu64(rcx.wrapping_add(0x390u64), rax); crate::simd_ops::wu64(rcx.wrapping_add(0x398u64), rdx); crate::simd_ops::wu64(rcx.wrapping_add(0x3a0u64), rdx); crate::simd_ops::wu64(rcx.wrapping_add(0x3a8u64), rdx); crate::simd_ops::wu64(rcx.wrapping_add(0x3b8u64), rdx); crate::simd_ops::lk_and_u32(rcx.wrapping_add(0x3c0u64), (0xff000004u32) as u32); crate::simd_ops::lk_or_u32(rcx.wrapping_add(0x3c0u64), (0x4u32) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x3c3u64), (0x4u8) as u8); crate::simd_ops::wu32(rcx.wrapping_add(0x3c4u64), ((rdx as u32)) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x3c8u64), ((rdx as u8)) as u8); crate::simd_ops::lk_and_u32(rcx.wrapping_add(0x3e8u64), (0xff000004u32) as u32); crate::simd_ops::lk_or_u32(rcx.wrapping_add(0x3e8u64), (0x4u32) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x3ebu64), (0x4u8) as u8); crate::simd_ops::wu32(rcx.wrapping_add(0x3ecu64), ((rdx as u32)) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x3f0u64), ((rdx as u8)) as u8); crate::simd_ops::lk_and_u32(rcx.wrapping_add(0x410u64), (0xff000004u32) as u32); crate::simd_ops::lk_or_u32(rcx.wrapping_add(0x410u64), (0x4u32) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x413u64), (0x4u8) as u8); crate::simd_ops::wu32(rcx.wrapping_add(0x414u64), ((rdx as u32)) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x418u64), ((rdx as u8)) as u8); crate::simd_ops::wu64(rcx.wrapping_add(0x438u64), rax); crate::simd_ops::wu64(rcx.wrapping_add(0x440u64), rdx); crate::simd_ops::wu64(rcx.wrapping_add(0x448u64), rdx); crate::simd_ops::wu64(rcx.wrapping_add(0x450u64), rdx); crate::simd_ops::wu64(rcx.wrapping_add(0x460u64), rdx); crate::simd_ops::wu64(rcx.wrapping_add(0x468u64), rax); crate::simd_ops::wu64(rcx.wrapping_add(0x470u64), rdx); crate::simd_ops::wu64(rcx.wrapping_add(0x478u64), rdx); crate::simd_ops::wu64(rcx.wrapping_add(0x480u64), rdx); crate::simd_ops::wu64(rcx.wrapping_add(0x490u64), rdx); crate::simd_ops::wu64(rcx.wrapping_add(0x498u64), rax); crate::simd_ops::wu64(rcx.wrapping_add(0x4a0u64), rdx); crate::simd_ops::wu64(rcx.wrapping_add(0x4a8u64), rdx); crate::simd_ops::wu64(rcx.wrapping_add(0x4b0u64), rdx); crate::simd_ops::wu64(rcx.wrapping_add(0x4c0u64), rdx); crate::simd_ops::wu64(rcx.wrapping_add(0x4c8u64), rax); crate::simd_ops::wu64(rcx.wrapping_add(0x4d0u64), rdx); crate::simd_ops::wu64(rcx.wrapping_add(0x4d8u64), rdx); crate::simd_ops::wu64(rcx.wrapping_add(0x4e0u64), rdx); crate::simd_ops::wu64(rcx.wrapping_add(0x4f0u64), rdx); crate::simd_ops::wu64(rcx.wrapping_add(0x4f8u64), rax); crate::simd_ops::wu64(rcx.wrapping_add(0x500u64), rdx); crate::simd_ops::wu64(rcx.wrapping_add(0x508u64), rdx); crate::simd_ops::wu64(rcx.wrapping_add(0x510u64), rdx); crate::simd_ops::wu64(rcx.wrapping_add(0x520u64), rdx); crate::simd_ops::wu64(rcx.wrapping_add(0x528u64), rax); crate::simd_ops::wu64(rcx.wrapping_add(0x530u64), rdx); crate::simd_ops::wu64(rcx.wrapping_add(0x538u64), rdx); crate::simd_ops::wu64(rcx.wrapping_add(0x540u64), rdx); crate::simd_ops::wu64(rcx.wrapping_add(0x550u64), rdx); crate::simd_ops::wu64(rcx.wrapping_add(0x558u64), rdx); crate::simd_ops::wu32(rcx.wrapping_add(0x560u64), (0x1u32) as u32); crate::simd_ops::wu64(rcx.wrapping_add(0x568u64), rdx); crate::simd_ops::wu32(rcx.wrapping_add(0x570u64), ((rdx as u32)) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x580u64), ((rdx as u8)) as u8); crate::simd_ops::lk_and_u32(rcx.wrapping_add(0x588u64), (0xff000004u32) as u32); crate::simd_ops::lk_or_u32(rcx.wrapping_add(0x588u64), (0x4u32) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x58bu64), (0x4u8) as u8); crate::simd_ops::wu32(rcx.wrapping_add(0x58cu64), ((rdx as u32)) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x590u64), ((rdx as u8)) as u8); crate::simd_ops::wu64(rcx.wrapping_add(0x5b0u64), rdx); crate::simd_ops::wu32(rcx.wrapping_add(0x5b8u64), ((rdx as u32)) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x5c8u64), ((rdx as u8)) as u8); crate::simd_ops::lk_and_u32(rcx.wrapping_add(0x5d0u64), (0xff000004u32) as u32); crate::simd_ops::lk_or_u32(rcx.wrapping_add(0x5d0u64), (0x4u32) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x5d3u64), (0x4u8) as u8); crate::simd_ops::wu32(rcx.wrapping_add(0x5d4u64), ((rdx as u32)) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x5d8u64), ((rdx as u8)) as u8); crate::simd_ops::wu64(rcx.wrapping_add(0x5f8u64), rdx); crate::simd_ops::wu32(rcx.wrapping_add(0x600u64), ((rdx as u32)) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x610u64), ((rdx as u8)) as u8); crate::simd_ops::lk_and_u32(rcx.wrapping_add(0x618u64), (0xff000004u32) as u32); crate::simd_ops::lk_or_u32(rcx.wrapping_add(0x618u64), (0x4u32) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x61bu64), (0x4u8) as u8); crate::simd_ops::wu32(rcx.wrapping_add(0x61cu64), ((rdx as u32)) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x620u64), ((rdx as u8)) as u8); crate::simd_ops::wu64(rcx.wrapping_add(0x640u64), rdx); crate::simd_ops::wu32(rcx.wrapping_add(0x648u64), ((rdx as u32)) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x658u64), ((rdx as u8)) as u8); crate::simd_ops::lk_and_u32(rcx.wrapping_add(0x660u64), (0xff000004u32) as u32); crate::simd_ops::lk_or_u32(rcx.wrapping_add(0x660u64), (0x4u32) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x663u64), (0x4u8) as u8); crate::simd_ops::wu32(rcx.wrapping_add(0x664u64), ((rdx as u32)) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x668u64), ((rdx as u8)) as u8); crate::simd_ops::wu64(rcx.wrapping_add(0x688u64), rdx); crate::simd_ops::wu32(rcx.wrapping_add(0x690u64), ((rdx as u32)) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x6a0u64), ((rdx as u8)) as u8); crate::simd_ops::lk_and_u32(rcx.wrapping_add(0x6a8u64), (0xff000004u32) as u32); crate::simd_ops::lk_or_u32(rcx.wrapping_add(0x6a8u64), (0x4u32) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x6abu64), (0x4u8) as u8); crate::simd_ops::wu32(rcx.wrapping_add(0x6acu64), ((rdx as u32)) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x6b0u64), ((rdx as u8)) as u8); crate::simd_ops::wu64(rcx.wrapping_add(0x6d0u64), rdx); crate::simd_ops::wu32(rcx.wrapping_add(0x6d8u64), ((rdx as u32)) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x6e8u64), ((rdx as u8)) as u8); crate::simd_ops::lk_and_u32(rcx.wrapping_add(0x6f0u64), (0xff000004u32) as u32); crate::simd_ops::lk_or_u32(rcx.wrapping_add(0x6f0u64), (0x4u32) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x6f3u64), (0x4u8) as u8); crate::simd_ops::wu32(rcx.wrapping_add(0x6f4u64), ((rdx as u32)) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x6f8u64), ((rdx as u8)) as u8); crate::simd_ops::wu64(rcx.wrapping_add(0x718u64), rdx); crate::simd_ops::wu32(rcx.wrapping_add(0x720u64), ((rdx as u32)) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x730u64), ((rdx as u8)) as u8); crate::simd_ops::lk_and_u32(rcx.wrapping_add(0x738u64), (0xff000004u32) as u32); crate::simd_ops::lk_or_u32(rcx.wrapping_add(0x738u64), (0x4u32) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x73bu64), (0x4u8) as u8); crate::simd_ops::wu32(rcx.wrapping_add(0x73cu64), ((rdx as u32)) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x740u64), ((rdx as u8)) as u8); crate::simd_ops::wu64(rcx.wrapping_add(0x760u64), rdx); crate::simd_ops::wu32(rcx.wrapping_add(0x768u64), ((rdx as u32)) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x778u64), ((rdx as u8)) as u8); crate::simd_ops::lk_and_u32(rcx.wrapping_add(0x780u64), (0xff000004u32) as u32); crate::simd_ops::lk_or_u32(rcx.wrapping_add(0x780u64), (0x4u32) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x783u64), (0x4u8) as u8); crate::simd_ops::wu32(rcx.wrapping_add(0x784u64), ((rdx as u32)) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x788u64), ((rdx as u8)) as u8); crate::simd_ops::wu64(rcx.wrapping_add(0x7a8u64), rdx); crate::simd_ops::wu32(rcx.wrapping_add(0x7b0u64), ((rdx as u32)) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x7c0u64), ((rdx as u8)) as u8); crate::simd_ops::lk_and_u32(rcx.wrapping_add(0x7c8u64), (0xff000004u32) as u32); crate::simd_ops::lk_or_u32(rcx.wrapping_add(0x7c8u64), (0x4u32) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x7cbu64), (0x4u8) as u8); crate::simd_ops::wu32(rcx.wrapping_add(0x7ccu64), ((rdx as u32)) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x7d0u64), ((rdx as u8)) as u8); crate::simd_ops::wu64(rcx.wrapping_add(0x7f0u64), rdx); crate::simd_ops::wu32(rcx.wrapping_add(0x7f8u64), ((rdx as u32)) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x808u64), ((rdx as u8)) as u8); crate::simd_ops::lk_and_u32(rcx.wrapping_add(0x810u64), (0xff000004u32) as u32); crate::simd_ops::lk_or_u32(rcx.wrapping_add(0x810u64), (0x4u32) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x813u64), (0x4u8) as u8); crate::simd_ops::wu32(rcx.wrapping_add(0x814u64), ((rdx as u32)) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x818u64), ((rdx as u8)) as u8); crate::simd_ops::wu64(rcx.wrapping_add(0x838u64), rdx); crate::simd_ops::wu32(rcx.wrapping_add(0x840u64), ((rdx as u32)) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x850u64), ((rdx as u8)) as u8); crate::simd_ops::lk_and_u32(rcx.wrapping_add(0x858u64), (0xff000004u32) as u32); crate::simd_ops::lk_or_u32(rcx.wrapping_add(0x858u64), (0x4u32) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x85bu64), (0x4u8) as u8); crate::simd_ops::wu32(rcx.wrapping_add(0x85cu64), ((rdx as u32)) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x860u64), ((rdx as u8)) as u8); crate::simd_ops::wu64(rcx.wrapping_add(0x880u64), rdx); crate::simd_ops::wu32(rcx.wrapping_add(0x888u64), ((rdx as u32)) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x898u64), ((rdx as u8)) as u8); crate::simd_ops::lk_and_u32(rcx.wrapping_add(0x8a0u64), (0xff000004u32) as u32); crate::simd_ops::lk_or_u32(rcx.wrapping_add(0x8a0u64), (0x4u32) as u32); rax = rcx; crate::simd_ops::wu8(rcx.wrapping_add(0x8a3u64), (0x4u8) as u8); crate::simd_ops::wu32(rcx.wrapping_add(0x8a4u64), ((rdx as u32)) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x8a8u64), ((rdx as u8)) as u8); crate::simd_ops::wu64(rcx.wrapping_add(0x8c8u64), rdx); crate::simd_ops::wu32(rcx.wrapping_add(0x8d0u64), ((rdx as u32)) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x8e0u64), ((rdx as u8)) as u8); crate::simd_ops::lk_and_u32(rcx.wrapping_add(0x8e8u64), (0xff000004u32) as u32); crate::simd_ops::lk_or_u32(rcx.wrapping_add(0x8e8u64), (0x4u32) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x8ebu64), (0x4u8) as u8); crate::simd_ops::wu32(rcx.wrapping_add(0x8ecu64), ((rdx as u32)) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x8f0u64), ((rdx as u8)) as u8); crate::simd_ops::wu64(rcx.wrapping_add(0x910u64), rdx); crate::simd_ops::wu32(rcx.wrapping_add(0x918u64), ((rdx as u32)) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x928u64), ((rdx as u8)) as u8); crate::simd_ops::lk_and_u32(rcx.wrapping_add(0x930u64), (0xff000004u32) as u32); crate::simd_ops::lk_or_u32(rcx.wrapping_add(0x930u64), (0x4u32) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x933u64), (0x4u8) as u8); crate::simd_ops::wu32(rcx.wrapping_add(0x934u64), ((rdx as u32)) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x938u64), ((rdx as u8)) as u8); crate::simd_ops::wu64(rcx.wrapping_add(0x958u64), rdx); crate::simd_ops::wu32(rcx.wrapping_add(0x960u64), ((rdx as u32)) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x970u64), ((rdx as u8)) as u8); crate::simd_ops::lk_and_u32(rcx.wrapping_add(0x978u64), (0xff000004u32) as u32); crate::simd_ops::lk_or_u32(rcx.wrapping_add(0x978u64), (0x4u32) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x97bu64), (0x4u8) as u8); crate::simd_ops::wu32(rcx.wrapping_add(0x97cu64), ((rdx as u32)) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x980u64), ((rdx as u8)) as u8); crate::simd_ops::wu64(rcx.wrapping_add(0x9a0u64), rdx); crate::simd_ops::wu32(rcx.wrapping_add(0x9a8u64), ((rdx as u32)) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x9b8u64), ((rdx as u8)) as u8); crate::simd_ops::lk_and_u32(rcx.wrapping_add(0x9c0u64), (0xff000004u32) as u32); crate::simd_ops::lk_or_u32(rcx.wrapping_add(0x9c0u64), (0x4u32) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x9c3u64), (0x4u8) as u8); crate::simd_ops::wu32(rcx.wrapping_add(0x9c4u64), ((rdx as u32)) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x9c8u64), ((rdx as u8)) as u8); rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142191c30(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x40; const P2: u64 = 0x10; const P3: u64 = 0x18; const P4: u64 = 0xffffffffffffffd8; const P5: u64 = 0x142535070; const P6: u64 = 0x1426456c8; const P7: u64 = 0x1426453a0; const P8: u64 = 0x14; const P9: u64 = 0x20; const P10: u64 = 0x30; const P11: u64 = 0x38;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rax = ((crate::exe::tls_pointer() as u64)) as u64; rdi = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rcx, rdi); rbx = rcx; crate::simd_ops::wu64(rcx.wrapping_add(P0), rdi); rcx = 0x6348; rdx = (crate::simd_ops::ru64(rax)) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(rdx))) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64; rdx = (crate::simd_ops::ru64(rcx.wrapping_add(P2))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 1; continue; } fl_l = ((rdx as u8)) as u8 as u64; fl_r = (0xfu8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 1; continue; } fl_l = (crate::simd_ops::lk_add_u32(rcx.wrapping_add(P3), (1) as u32)) as u32 as u64; rax = (crate::simd_ops::ru64(rdx)) as u64; crate::simd_ops::wu64(rcx.wrapping_add(P2), rax); bb = 2; continue; }
            1 => { rdx = 0x38; rcx = rax; r8 = ((rdx.wrapping_add(P4)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rdx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 2; continue; } rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P6)) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = rax; rdx = 0xdeadd00d; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P7)) as u64, rax, rcx, rdx, r8, r9, rsp); rdx = rdi; bb = 2; continue; }
            2 => { crate::simd_ops::wu64(rdx, rdx); rax = rbx; crate::simd_ops::wu64(rdx.wrapping_add(P0), rdx); crate::simd_ops::wu64(rdx.wrapping_add(P2), rdx); crate::simd_ops::wu16(rdx.wrapping_add(P3), (0x101u16) as u16); crate::simd_ops::wu64(rbx, rdx); crate::simd_ops::wu32(rbx.wrapping_add(P2), ((rdi as u32)) as u32); crate::simd_ops::wu32(rbx.wrapping_add(P8), (0x1u32) as u32); crate::simd_ops::wu64(rbx.wrapping_add(P3), rdi); crate::simd_ops::wu32(rbx.wrapping_add(P9), ((rdi as u32)) as u32); crate::simd_ops::wu8(rbx.wrapping_add(P10), ((rdi as u8)) as u8); crate::simd_ops::wu64(rbx.wrapping_add(P11), rdi); crate::simd_ops::wu32(rbx.wrapping_add(0x40u64), ((rdi as u32)) as u32); crate::simd_ops::wu8(rbx.wrapping_add(0x50u64), ((rdi as u8)) as u8); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142191f10(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0xc; const P2: u64 = 0x10; const P3: u64 = 0xb; const P4: u64 = 0x30; const P5: u64 = 0x34; const P6: u64 = 0x38; const P7: u64 = 0x33; const P8: u64 = 0x78; const P9: u64 = 0x7c; const P10: u64 = 0x80; const P11: u64 = 0x7b;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rcx, rax); crate::simd_ops::lk_and_u32(rcx.wrapping_add(P0), (0xff000004u32) as u32); crate::simd_ops::lk_or_u32(rcx.wrapping_add(P0), (0x4u32) as u32); crate::simd_ops::wu32(rcx.wrapping_add(P1), ((rax as u32)) as u32); crate::simd_ops::wu8(rcx.wrapping_add(P2), ((rax as u8)) as u8); crate::simd_ops::wu8(rcx.wrapping_add(P3), (0x4u8) as u8); crate::simd_ops::lk_and_u32(rcx.wrapping_add(P4), (0xff000008u32) as u32); crate::simd_ops::lk_or_u32(rcx.wrapping_add(P4), (0x8u32) as u32); crate::simd_ops::wu32(rcx.wrapping_add(P5), ((rax as u32)) as u32); crate::simd_ops::wu8(rcx.wrapping_add(P6), ((rax as u8)) as u8); crate::simd_ops::wu8(rcx.wrapping_add(P7), (0x8u8) as u8); crate::simd_ops::lk_and_u32(rcx.wrapping_add(P8), (0xff000004u32) as u32); crate::simd_ops::lk_or_u32(rcx.wrapping_add(P8), (0x4u32) as u32); crate::simd_ops::wu32(rcx.wrapping_add(P9), ((rax as u32)) as u32); crate::simd_ops::wu8(rcx.wrapping_add(P10), ((rax as u8)) as u8); crate::simd_ops::wu8(rcx.wrapping_add(P11), (0x4u8) as u8); crate::simd_ops::wu64(rcx.wrapping_add(0xa0u64), rax); crate::simd_ops::wu32(rcx.wrapping_add(0xa8u64), ((rax as u32)) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0xb8u64), ((rax as u8)) as u8); crate::simd_ops::wu64(rcx.wrapping_add(0xc0u64), rax); crate::simd_ops::wu32(rcx.wrapping_add(0xc8u64), ((rax as u32)) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0xe0u64), ((rax as u8)) as u8); crate::simd_ops::wu64(rcx.wrapping_add(0xe8u64), rax); crate::simd_ops::wu32(rcx.wrapping_add(0xf0u64), ((rax as u32)) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x100u64), ((rax as u8)) as u8); crate::simd_ops::lk_and_u32(rcx.wrapping_add(0x108u64), (0xff000004u32) as u32); fl_l = (crate::simd_ops::lk_or_u32(rcx.wrapping_add(0x108u64), (0x4u32) as u32)) as u32 as u64; fl_r = fl_l; crate::simd_ops::wu32(rcx.wrapping_add(0x10cu64), ((rax as u32)) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x110u64), ((rax as u8)) as u8); crate::simd_ops::wu8(rcx.wrapping_add(0x10bu64), (0x4u8) as u8); crate::simd_ops::wu64(rcx.wrapping_add(0x130u64), rax); crate::simd_ops::wu32(rcx.wrapping_add(0x138u64), ((rax as u32)) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x148u64), ((rax as u8)) as u8); rax = rcx; return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142192040(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0xc; const P2: u64 = 0x10; const P3: u64 = 0xb; const P4: u64 = 0x30; const P5: u64 = 0x34; const P6: u64 = 0x38; const P7: u64 = 0x33; const P8: u64 = 0x78; const P9: u64 = 0x80; const P10: u64 = 0x90; const P11: u64 = 0x98;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rcx, rax); crate::simd_ops::lk_and_u32(rcx.wrapping_add(P0), (0xff000004u32) as u32); crate::simd_ops::lk_or_u32(rcx.wrapping_add(P0), (0x4u32) as u32); crate::simd_ops::wu32(rcx.wrapping_add(P1), ((rax as u32)) as u32); crate::simd_ops::wu8(rcx.wrapping_add(P2), ((rax as u8)) as u8); crate::simd_ops::wu8(rcx.wrapping_add(P3), (0x4u8) as u8); crate::simd_ops::lk_and_u32(rcx.wrapping_add(P4), (0xff000008u32) as u32); crate::simd_ops::lk_or_u32(rcx.wrapping_add(P4), (0x8u32) as u32); crate::simd_ops::wu32(rcx.wrapping_add(P5), ((rax as u32)) as u32); crate::simd_ops::wu8(rcx.wrapping_add(P6), ((rax as u8)) as u8); crate::simd_ops::wu8(rcx.wrapping_add(P7), (0x8u8) as u8); crate::simd_ops::wu64(rcx.wrapping_add(P8), rax); crate::simd_ops::wu32(rcx.wrapping_add(P9), ((rax as u32)) as u32); crate::simd_ops::wu8(rcx.wrapping_add(P10), ((rax as u8)) as u8); crate::simd_ops::wu64(rcx.wrapping_add(P11), rax); crate::simd_ops::wu32(rcx.wrapping_add(0xa0u64), ((rax as u32)) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0xb8u64), ((rax as u8)) as u8); crate::simd_ops::wu64(rcx.wrapping_add(0xc0u64), rax); crate::simd_ops::wu32(rcx.wrapping_add(0xc8u64), ((rax as u32)) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0xd8u64), ((rax as u8)) as u8); crate::simd_ops::wu64(rcx.wrapping_add(0xe0u64), rax); crate::simd_ops::wu64(rcx.wrapping_add(0xe8u64), rax); crate::simd_ops::wu32(rcx.wrapping_add(0xf0u64), ((rax as u32)) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x110u64), ((rax as u8)) as u8); crate::simd_ops::lk_and_u32(rcx.wrapping_add(0x118u64), (0xff000004u32) as u32); crate::simd_ops::lk_or_u32(rcx.wrapping_add(0x118u64), (0x4u32) as u32); crate::simd_ops::wu32(rcx.wrapping_add(0x11cu64), ((rax as u32)) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x120u64), ((rax as u8)) as u8); crate::simd_ops::wu8(rcx.wrapping_add(0x11bu64), (0x4u8) as u8); crate::simd_ops::lk_and_u32(rcx.wrapping_add(0x140u64), (0xff000004u32) as u32); fl_l = (crate::simd_ops::lk_or_u32(rcx.wrapping_add(0x140u64), (0x4u32) as u32)) as u32 as u64; fl_r = fl_l; crate::simd_ops::wu32(rcx.wrapping_add(0x144u64), ((rax as u32)) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x148u64), ((rax as u8)) as u8); crate::simd_ops::wu8(rcx.wrapping_add(0x143u64), (0x4u8) as u8); crate::simd_ops::wu64(rcx.wrapping_add(0x168u64), rax); crate::simd_ops::wu32(rcx.wrapping_add(0x170u64), ((rax as u32)) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x180u64), ((rax as u8)) as u8); rax = rcx; return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142192180(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rdx = 0; fl_l = 0; fl_r = 0; rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rcx, rdx); crate::simd_ops::wu32(rcx.wrapping_add(0x8u64), ((rdx as u32)) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x28u64), ((rdx as u8)) as u8); crate::simd_ops::wu64(rcx.wrapping_add(0x30u64), rax); crate::simd_ops::wu32(rcx.wrapping_add(0x34u64), ((rdx as u32)) as u32); crate::simd_ops::lk_and_u32(rcx.wrapping_add(0x38u64), (0xff000004u32) as u32); crate::simd_ops::lk_or_u32(rcx.wrapping_add(0x38u64), (0x4u32) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x3bu64), (0x4u8) as u8); crate::simd_ops::wu32(rcx.wrapping_add(0x3cu64), ((rdx as u32)) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x40u64), ((rax as u8)) as u8); crate::simd_ops::lk_and_u32(rcx.wrapping_add(0x60u64), (0xff000008u32) as u32); crate::simd_ops::lk_or_u32(rcx.wrapping_add(0x60u64), (0x8u32) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x63u64), (0x8u8) as u8); crate::simd_ops::wu32(rcx.wrapping_add(0x64u64), ((rdx as u32)) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x68u64), ((rax as u8)) as u8); crate::simd_ops::wu64(rcx.wrapping_add(0xa8u64), rdx); crate::simd_ops::wu32(rcx.wrapping_add(0xb0u64), ((rdx as u32)) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0xc0u64), ((rax as u8)) as u8); crate::simd_ops::wu64(rcx.wrapping_add(0xc8u64), rdx); crate::simd_ops::wu32(rcx.wrapping_add(0xd0u64), ((rdx as u32)) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0xe8u64), ((rax as u8)) as u8); crate::simd_ops::wu64(rcx.wrapping_add(0xf0u64), rdx); crate::simd_ops::wu32(rcx.wrapping_add(0xf8u64), ((rdx as u32)) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x108u64), ((rax as u8)) as u8); crate::simd_ops::wu64(rcx.wrapping_add(0x110u64), rdx); crate::simd_ops::wu64(rcx.wrapping_add(0x118u64), rdx); crate::simd_ops::wu32(rcx.wrapping_add(0x120u64), ((rdx as u32)) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x140u64), ((rax as u8)) as u8); crate::simd_ops::lk_and_u32(rcx.wrapping_add(0x148u64), (0xff000004u32) as u32); crate::simd_ops::lk_or_u32(rcx.wrapping_add(0x148u64), (0x4u32) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x14bu64), (0x4u8) as u8); crate::simd_ops::wu32(rcx.wrapping_add(0x14cu64), ((rdx as u32)) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x150u64), ((rax as u8)) as u8); crate::simd_ops::lk_and_u32(rcx.wrapping_add(0x170u64), (0xff000004u32) as u32); crate::simd_ops::lk_or_u32(rcx.wrapping_add(0x170u64), (0x4u32) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x173u64), (0x4u8) as u8); crate::simd_ops::wu32(rcx.wrapping_add(0x174u64), ((rdx as u32)) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x178u64), ((rax as u8)) as u8); crate::simd_ops::wu64(rcx.wrapping_add(0x198u64), rdx); crate::simd_ops::wu32(rcx.wrapping_add(0x1a0u64), ((rdx as u32)) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x1b0u64), ((rax as u8)) as u8); crate::simd_ops::wu64(rcx.wrapping_add(0x1b8u64), rdx); crate::simd_ops::wu32(rcx.wrapping_add(0x1c0u64), ((rdx as u32)) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x1e0u64), ((rax as u8)) as u8); crate::simd_ops::wu64(rcx.wrapping_add(0x1e8u64), rax); crate::simd_ops::wu32(rcx.wrapping_add(0x1ecu64), ((rdx as u32)) as u32); crate::simd_ops::lk_and_u32(rcx.wrapping_add(0x1f0u64), (0xff000004u32) as u32); crate::simd_ops::lk_or_u32(rcx.wrapping_add(0x1f0u64), (0x4u32) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x1f3u64), (0x4u8) as u8); crate::simd_ops::wu32(rcx.wrapping_add(0x1f4u64), ((rdx as u32)) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x1f8u64), ((rax as u8)) as u8); crate::simd_ops::lk_and_u32(rcx.wrapping_add(0x218u64), (0xff000008u32) as u32); crate::simd_ops::lk_or_u32(rcx.wrapping_add(0x218u64), (0x8u32) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x21bu64), (0x8u8) as u8); crate::simd_ops::wu32(rcx.wrapping_add(0x21cu64), ((rdx as u32)) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x220u64), ((rax as u8)) as u8); crate::simd_ops::lk_and_u32(rcx.wrapping_add(0x260u64), (0xff000004u32) as u32); crate::simd_ops::lk_or_u32(rcx.wrapping_add(0x260u64), (0x4u32) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x263u64), (0x4u8) as u8); crate::simd_ops::wu32(rcx.wrapping_add(0x264u64), ((rdx as u32)) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x268u64), ((rax as u8)) as u8); crate::simd_ops::wu64(rcx.wrapping_add(0x288u64), rdx); crate::simd_ops::wu32(rcx.wrapping_add(0x290u64), ((rdx as u32)) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x2a0u64), ((rax as u8)) as u8); crate::simd_ops::wu64(rcx.wrapping_add(0x2a8u64), rdx); crate::simd_ops::wu32(rcx.wrapping_add(0x2b0u64), ((rdx as u32)) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x2c8u64), ((rax as u8)) as u8); crate::simd_ops::wu64(rcx.wrapping_add(0x2d0u64), rdx); crate::simd_ops::wu32(rcx.wrapping_add(0x2d8u64), ((rdx as u32)) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x2e8u64), ((rax as u8)) as u8); crate::simd_ops::lk_and_u32(rcx.wrapping_add(0x2f0u64), (0xff000004u32) as u32); crate::simd_ops::lk_or_u32(rcx.wrapping_add(0x2f0u64), (0x4u32) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x2f3u64), (0x4u8) as u8); crate::simd_ops::wu32(rcx.wrapping_add(0x2f4u64), ((rdx as u32)) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x2f8u64), ((rax as u8)) as u8); crate::simd_ops::wu64(rcx.wrapping_add(0x318u64), rdx); crate::simd_ops::wu32(rcx.wrapping_add(0x320u64), ((rdx as u32)) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x330u64), ((rax as u8)) as u8); crate::simd_ops::lk_and_u32(rcx.wrapping_add(0x338u64), (0xff000004u32) as u32); crate::simd_ops::lk_or_u32(rcx.wrapping_add(0x338u64), (0x4u32) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x340u64), ((rax as u8)) as u8); crate::simd_ops::wu8(rcx.wrapping_add(0x33bu64), (0x4u8) as u8); crate::simd_ops::wu32(rcx.wrapping_add(0x33cu64), ((rdx as u32)) as u32); crate::simd_ops::lk_and_u32(rcx.wrapping_add(0x360u64), (0xff000004u32) as u32); crate::simd_ops::lk_or_u32(rcx.wrapping_add(0x360u64), (0x4u32) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x368u64), ((rax as u8)) as u8); crate::simd_ops::wu8(rcx.wrapping_add(0x363u64), (0x4u8) as u8); crate::simd_ops::wu32(rcx.wrapping_add(0x364u64), ((rdx as u32)) as u32); crate::simd_ops::lk_and_u32(rcx.wrapping_add(0x388u64), (0xff000004u32) as u32); crate::simd_ops::lk_or_u32(rcx.wrapping_add(0x388u64), (0x4u32) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x390u64), ((rax as u8)) as u8); crate::simd_ops::wu8(rcx.wrapping_add(0x38bu64), (0x4u8) as u8); crate::simd_ops::wu32(rcx.wrapping_add(0x38cu64), ((rdx as u32)) as u32); crate::simd_ops::lk_and_u32(rcx.wrapping_add(0x3b0u64), (0xff000004u32) as u32); crate::simd_ops::lk_or_u32(rcx.wrapping_add(0x3b0u64), (0x4u32) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x3b8u64), ((rax as u8)) as u8); crate::simd_ops::wu8(rcx.wrapping_add(0x3b3u64), (0x4u8) as u8); crate::simd_ops::wu32(rcx.wrapping_add(0x3b4u64), ((rdx as u32)) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x3f0u64), ((rax as u8)) as u8); crate::simd_ops::wu64(rcx.wrapping_add(0x3d8u64), rdx); crate::simd_ops::wu32(rcx.wrapping_add(0x3e0u64), ((rdx as u32)) as u32); crate::simd_ops::lk_and_u32(rcx.wrapping_add(0x3f8u64), (0xff000004u32) as u32); crate::simd_ops::lk_or_u32(rcx.wrapping_add(0x3f8u64), (0x4u32) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x400u64), ((rax as u8)) as u8); crate::simd_ops::wu8(rcx.wrapping_add(0x3fbu64), (0x4u8) as u8); crate::simd_ops::wu32(rcx.wrapping_add(0x3fcu64), ((rdx as u32)) as u32); crate::simd_ops::lk_and_u32(rcx.wrapping_add(0x420u64), (0xff000004u32) as u32); fl_l = (crate::simd_ops::lk_or_u32(rcx.wrapping_add(0x420u64), (0x4u32) as u32)) as u32 as u64; fl_r = fl_l; crate::simd_ops::wu8(rcx.wrapping_add(0x428u64), ((rax as u8)) as u8); crate::simd_ops::wu8(rcx.wrapping_add(0x423u64), (0x4u8) as u8); crate::simd_ops::wu32(rcx.wrapping_add(0x424u64), ((rdx as u32)) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x468u64), ((rax as u8)) as u8); rax = rcx; crate::simd_ops::wu32(rcx.wrapping_add(0x448u64), (0x100000u32) as u32); crate::simd_ops::wu32(rcx.wrapping_add(0x44cu64), (0x1u32) as u32); crate::simd_ops::wu64(rcx.wrapping_add(0x450u64), rdx); crate::simd_ops::wu32(rcx.wrapping_add(0x458u64), ((rdx as u32)) as u32); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142192490(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x18;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu64(rcx, rax);
    crate::simd_ops::wu32(rcx.wrapping_add(P0), ((rax as u32)) as u32);
    crate::simd_ops::wu8(rcx.wrapping_add(P1), ((rax as u8)) as u8);
    rax = rcx;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1421925b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x18; const P2: u64 = 0x3; const P3: u64 = 0x4; const P4: u64 = 0x1433e7d88; const P5: u64 = 0x9; const P6: u64 = 0x142409b50; const P7: u64 = 0x68; const P8: u64 = 0x6c; const P9: u64 = 0x70; const P10: u64 = 0x78; const P11: u64 = 0x88;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { r11 = rsp; crate::simd_ops::wu64(r11.wrapping_add(0x18u64), rbx); crate::simd_ops::wu64(r11.wrapping_add(0x8u64), rcx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rbx = rcx; crate::simd_ops::wu64(r11.wrapping_add(0x8u64), rcx); rdi = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rcx, rdi); crate::simd_ops::wu32(rcx.wrapping_add(P0), ((rdi as u32)) as u32); crate::simd_ops::wu8(rcx.wrapping_add(P1), ((rdi as u8)) as u8); rcx = ((rcx).wrapping_add(0x20u64)) as u64; crate::simd_ops::wu64(r11.wrapping_add(0x10u64), rcx); crate::simd_ops::lk_and_u32(rcx, (0xff000008u32) as u32); crate::simd_ops::lk_or_u32(rcx, (0x8u32) as u32); crate::simd_ops::wu8(rcx.wrapping_add(P2), (0x8u8) as u8); crate::simd_ops::wu32(rcx.wrapping_add(P3), ((rdi as u32)) as u32); crate::simd_ops::wu8(rcx.wrapping_add(P0), ((rdi as u8)) as u8); rax = P4; crate::simd_ops::wu64(r11.wrapping_sub(0x18u64), rax); crate::simd_ops::wu64(r11.wrapping_sub(0x10u64), (0x9u64) as u64); r9 = ((rdi.wrapping_add(P5)) as u32) as u32 as u64; r8 = 0; fl_l = 0; fl_r = 0; rdx = r11.wrapping_sub(0x18); rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu32(rbx.wrapping_add(P7), ((rdi as u32)) as u32); crate::simd_ops::wu8(rbx.wrapping_add(P8), (0x1u8) as u8); crate::simd_ops::wu64(rbx.wrapping_add(P9), rdi); crate::simd_ops::wu32(rbx.wrapping_add(P10), ((rdi as u32)) as u32); crate::simd_ops::wu8(rbx.wrapping_add(P11), ((rdi as u8)) as u8); crate::simd_ops::lk_and_u32(rbx.wrapping_add(0x90u64), (0xff000008u32) as u32); crate::simd_ops::lk_or_u32(rbx.wrapping_add(0x90u64), (0x8u32) as u32); crate::simd_ops::wu8(rbx.wrapping_add(0x93u64), (0x8u8) as u8); crate::simd_ops::wu32(rbx.wrapping_add(0x94u64), ((rdi as u32)) as u32); crate::simd_ops::wu8(rbx.wrapping_add(0x98u64), ((rdi as u8)) as u8); crate::simd_ops::lk_and_u32(rbx.wrapping_add(0xd8u64), (0xff000008u32) as u32); crate::simd_ops::lk_or_u32(rbx.wrapping_add(0xd8u64), (0x8u32) as u32); crate::simd_ops::wu8(rbx.wrapping_add(0xdbu64), (0x8u8) as u8); crate::simd_ops::wu32(rbx.wrapping_add(0xdcu64), ((rdi as u32)) as u32); crate::simd_ops::wu8(rbx.wrapping_add(0xe0u64), ((rdi as u8)) as u8); crate::simd_ops::lk_and_u32(rbx.wrapping_add(0x120u64), (0xff000008u32) as u32); crate::simd_ops::lk_or_u32(rbx.wrapping_add(0x120u64), (0x8u32) as u32); crate::simd_ops::wu8(rbx.wrapping_add(0x123u64), (0x8u8) as u8); crate::simd_ops::wu32(rbx.wrapping_add(0x124u64), ((rdi as u32)) as u32); crate::simd_ops::wu8(rbx.wrapping_add(0x128u64), ((rdi as u8)) as u8); crate::simd_ops::wu64(rbx.wrapping_add(0x168u64), rdi); crate::simd_ops::wu32(rbx.wrapping_add(0x170u64), ((rdi as u32)) as u32); crate::simd_ops::wu8(rbx.wrapping_add(0x180u64), ((rdi as u8)) as u8); crate::simd_ops::wu32(rbx.wrapping_add(0x188u64), ((rdi as u32)) as u32); rax = rbx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1421926d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x143cee30c; const P1: u64 = 0x143cee308;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    crate::simd_ops::wu8(P0, (0x1u8) as u8);
    rax = P1;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142192700(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143a89278; const P1: u64 = 0x1401e6ff0;
    let mut rax: u64 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    rcx = rdx;
    rdx = P0;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rax = (rax & !0xffu64) | ((((((rax as u8)) ^ (0x1u8))) as u8 as u64));
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142192730(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x28; const P1: u64 = 0x143a89278; const P2: u64 = 0x1401e6ff0; const P3: u64 = 0x140208a00;
    let (mut rax, mut fl_l, mut fl_r) = (0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x28u64)) as u64; rax = (((crate::simd_ops::ru8(rcx.wrapping_add(P0))) as i8 as i64) as u64) as u64; fl_l = ((rax).wrapping_add(0x1u64)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; if (((fl_l as u64)) == 0) { bb = 2; continue; } fl_l = rax; fl_r = (0x1u64) as u64; rax = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 1; continue; } fl_l = rax; fl_r = (0x1u64) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } rdx = P1; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rax = (rax & !0xffu64) | ((((((rax as u8)) ^ (0x1u8))) as u8 as u64)); rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
            1 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
            2 => { rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x14219276a, rcx, rsp); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142192810(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1433e7cd0; const P1: u64 = 0x1412a4560; const P2: u64 = 0x14240aa00; const P3: u64 = 0x1412a4720; const P4: u64 = 0x1412a5fd0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r9); crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; r14 = r8; rbp = rdx; r15 = rcx; rax = P0; crate::simd_ops::wu64(rcx, rax); rsi = rcx.wrapping_add(0x8); crate::simd_ops::wu64(rsp.wrapping_add(0x88u64), rsi); rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = ((crate::simd_ops::ru8(rbp.wrapping_add(0x778u64))) as u32) as u32 as u64; crate::simd_ops::wu8(rsi.wrapping_add(0x778u64), ((rax as u8)) as u8); rbx = rsi.wrapping_add(0x77c); crate::simd_ops::wu64(rsp.wrapping_add(0x78u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbx); crate::simd_ops::lk_and_u32(rbx, (0xff000008u32) as u32); crate::simd_ops::lk_or_u32(rbx, (0x8u32) as u32); crate::simd_ops::wu8(rbx.wrapping_add(0x3u64), (0x8u8) as u8); r12 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rbx.wrapping_add(0x4u64), ((r12 as u32)) as u32); crate::simd_ops::wu8(rbx.wrapping_add(0x8u64), ((r12 as u8)) as u8); rdx = rbp.wrapping_add(0x77c); rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rbx.wrapping_add(0x48); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rcx); crate::simd_ops::lk_and_u32(rcx, (0xff000008u32) as u32); crate::simd_ops::lk_or_u32(rcx, (0x8u32) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x3u64), (0x8u8) as u8); crate::simd_ops::wu32(rcx.wrapping_add(0x4u64), ((r12 as u32)) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x8u64), ((r12 as u8)) as u8); rdx = rbp.wrapping_add(0x7c4); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x810u64))) as u64; crate::simd_ops::wu64(rsi.wrapping_add(0x810u64), rax); rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x818u64))) as u64; crate::simd_ops::wu64(rsi.wrapping_add(0x818u64), rax); rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x820u64))) as u64; crate::simd_ops::wu64(rsi.wrapping_add(0x820u64), rax); rcx = rsi.wrapping_add(0x828); crate::simd_ops::wu64(rsp.wrapping_add(0x78u64), rcx); crate::simd_ops::lk_and_u32(rcx, (0xff000020u32) as u32); crate::simd_ops::lk_or_u32(rcx, (0x20u32) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x3u64), (0x20u8) as u8); crate::simd_ops::wu32(rcx.wrapping_add(0x4u64), ((r12 as u32)) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x8u64), ((r12 as u8)) as u8); rdx = rbp.wrapping_add(0x828); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = ((crate::simd_ops::ru8(rbp.wrapping_add(0x930u64))) as u32) as u32 as u64; crate::simd_ops::wu8(rsi.wrapping_add(0x930u64), ((rax as u8)) as u8); rax = ((crate::simd_ops::ru8(rbp.wrapping_add(0x931u64))) as u32) as u32 as u64; crate::simd_ops::wu8(rsi.wrapping_add(0x931u64), ((rax as u8)) as u8); xmm0 = (crate::simd_ops::ru128(rbp.wrapping_add(0x938u64))) as u128; crate::simd_ops::wu128(rsi.wrapping_add(0x938u64), (xmm0) as u128); xmm1 = ((crate::simd_ops::ru64(rbp.wrapping_add(0x948u64))) as u64) as u128; crate::simd_ops::wu64(rsi.wrapping_add(0x948u64), ((xmm1 as u64)) as u64); rax = ((crate::simd_ops::ru8(rbp.wrapping_add(0x950u64))) as u32) as u32 as u64; crate::simd_ops::wu8(rsi.wrapping_add(0x950u64), ((rax as u8)) as u8); rax = ((crate::simd_ops::ru8(rbp.wrapping_add(0x951u64))) as u32) as u32 as u64; crate::simd_ops::wu8(rsi.wrapping_add(0x951u64), ((rax as u8)) as u8); rax = ((crate::simd_ops::ru8(rbp.wrapping_add(0x952u64))) as u32) as u32 as u64; crate::simd_ops::wu8(rsi.wrapping_add(0x952u64), ((rax as u8)) as u8); rax = ((crate::simd_ops::ru8(rbp.wrapping_add(0x954u64))) as u32) as u32 as u64; crate::simd_ops::wu8(rsi.wrapping_add(0x954u64), ((rax as u8)) as u8); rcx = rsi.wrapping_add(0x958); crate::simd_ops::wu64(rsp.wrapping_add(0x78u64), rcx); crate::simd_ops::lk_and_u32(rcx, (0xff000020u32) as u32); crate::simd_ops::lk_or_u32(rcx, (0x20u32) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x3u64), (0x20u8) as u8); crate::simd_ops::wu32(rcx.wrapping_add(0x4u64), ((r12 as u32)) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x8u64), ((r12 as u8)) as u8); rdx = rbp.wrapping_add(0x958); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = ((crate::simd_ops::ru8(rbp.wrapping_add(0xa60u64))) as u32) as u32 as u64; crate::simd_ops::wu8(rsi.wrapping_add(0xa60u64), ((rax as u8)) as u8); rax = ((crate::simd_ops::ru16(rbp.wrapping_add(0xa61u64))) as u32) as u32 as u64; crate::simd_ops::wu16(rsi.wrapping_add(0xa61u64), ((rax as u16)) as u16); rax = ((crate::simd_ops::ru8(rbp.wrapping_add(0xa63u64))) as u32) as u32 as u64; crate::simd_ops::wu8(rsi.wrapping_add(0xa63u64), ((rax as u8)) as u8); rax = ((crate::simd_ops::ru8(rbp.wrapping_add(0xa64u64))) as u32) as u32 as u64; crate::simd_ops::wu8(rsi.wrapping_add(0xa64u64), ((rax as u8)) as u8); rax = ((crate::simd_ops::ru8(rbp.wrapping_add(0xa65u64))) as u32) as u32 as u64; crate::simd_ops::wu8(rsi.wrapping_add(0xa65u64), ((rax as u8)) as u8); rax = ((crate::simd_ops::ru8(rbp.wrapping_add(0xa66u64))) as u32) as u32 as u64; crate::simd_ops::wu8(rsi.wrapping_add(0xa66u64), ((rax as u8)) as u8); rax = (crate::simd_ops::ru32(rbp.wrapping_add(0xa68u64))) as u32 as u64; crate::simd_ops::wu32(rsi.wrapping_add(0xa68u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rbp.wrapping_add(0xa6cu64))) as u32 as u64; crate::simd_ops::wu32(rsi.wrapping_add(0xa6cu64), ((rax as u32)) as u32); rax = ((crate::simd_ops::ru8(rbp.wrapping_add(0xa70u64))) as u32) as u32 as u64; crate::simd_ops::wu8(rsi.wrapping_add(0xa70u64), ((rax as u8)) as u8); rcx = r15.wrapping_add(0xa80); rdx = r14; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = r15.wrapping_add(0xca0); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = r15; rsp = ((rsp).wrapping_add(0x30u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142193110(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0xa78;
    let (mut rax, mut fl_l, mut fl_r) = (0u64, 0u64, 0u64);
    fl_l = rcx; fl_r = rcx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) != 0)) { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; return rax; }
    rax = ((crate::simd_ops::ru8(rcx.wrapping_add(P0))) as u32) as u32 as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142193130(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1412a6060; const P1: u64 = 0x88; const P2: u64 = 0x142442de0; const P3: u64 = 0x8; const P4: u64 = 0x1421906e0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x142193130, funcinfo_rva: 0x37b0a14, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0x48, conts: &[] };
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rdi = (rcx) as u64; r9 = (r8) as u64; r8 = (rdx) as u64; rcx = ((rsp.wrapping_add(0x30u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14219314d); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rbx = (crate::simd_ops::ru64(rax)) as u64; crate::simd_ops::wu64(rax, (0x0u64) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), (rbx) as u64); rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; fl_l = (rsi) as u64 as u64; fl_r = (rsi) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(rsi)) as u64; rdx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rcx = (rsi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142193173); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rcx = (rsi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14219317c); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = ((rsp.wrapping_add(0x68u64)) as u64) as u64; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14219318d); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); fl_l = (crate::simd_ops::ru8(rsp.wrapping_add(0x6cu64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 5; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x68u64))) as u32 as u64; crate::simd_ops::wu32(rdi, ((rax as u32)) as u32); crate::simd_ops::wu8(rdi.wrapping_add(P3), (0x0u8) as u8); fl_l = (rbx) as u64 as u64; fl_r = (rbx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1421931ad); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1421931b6); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rax = (rdi) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            5 => { rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1421931d2); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); crate::simd_ops::wu64(rdi, (rbx) as u64); crate::simd_ops::wu8(rdi.wrapping_add(P3), (0x1u8) as u8); bb = 6; continue; }
            6 => { rax = (rdi) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142193250(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143cee304; const P1: u64 = 0x143cee30c; const P2: u64 = 0x4; const P3: u64 = 0x143cee308; const P4: u64 = 0x141834a78; const P5: u64 = 0x1426242a0; const P6: u64 = 0x141834d18; const P7: u64 = 0x141834a0c;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = ((crate::exe::tls_pointer() as u64)) as u64; rbx = rcx; rcx = 0x74; rax = (crate::simd_ops::ru64(rax)) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(rax))) as u32 as u64; fl_l = (crate::simd_ops::ru32(P0)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) > (((fl_r as u32)) as i32)) { bb = 3; continue; } bb = 1; continue; }
            1 => { rax = ((crate::simd_ops::ru8(P1)) as u32) as u32 as u64; crate::simd_ops::wu8(rbx.wrapping_add(P2), ((rax as u8)) as u8); fl_l = (crate::simd_ops::ru8(P1)) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 2; continue; } rax = (crate::simd_ops::ru32(P3)) as u32 as u64; crate::simd_ops::wu32(rbx, ((rax as u32)) as u32); bb = 2; continue; }
            2 => { rax = rbx; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            3 => { rcx = P0; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru32(P0)) as u32 as u64; fl_r = 0xffffffff; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 1; continue; } rcx = P5; crate::simd_ops::wu8(P1, (0x1u8) as u8); rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); rcx = P0; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); bb = 1; continue; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1421934a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x143c8c6a0;
    let (mut rax, mut fl_l) = (crate::simd_ops::get_rax_in(), 0u64);
    rax = 0x1;
    let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(P0, xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64;
    rax = (xa_old) as u32 as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1421935c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x3; const P1: u64 = 0x4; const P2: u64 = 0x8; const P3: u64 = 0x14240a730; const P4: u64 = 0x48; const P5: u64 = 0xd0; const P6: u64 = 0x88; const P7: u64 = 0xd4; const P8: u64 = 0x15c;
    let (mut rax, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rsi = rdx; rbp = rcx; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rcx); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rcx); crate::simd_ops::lk_and_u32(rcx, (0xff000008u32) as u32); crate::simd_ops::lk_or_u32(rcx, (0x8u32) as u32); crate::simd_ops::wu8(rcx.wrapping_add(P0), (0x8u8) as u8); crate::simd_ops::wu32(rcx.wrapping_add(P1), (0x0u32) as u32); crate::simd_ops::wu8(rcx.wrapping_add(P2), (0x0u8) as u8); rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rdi = rbp.wrapping_add(0x48); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rdi); crate::simd_ops::lk_and_u32(rdi, (0xff000010u32) as u32); crate::simd_ops::lk_or_u32(rdi, (0x10u32) as u32); crate::simd_ops::wu8(rdi.wrapping_add(P0), (0x10u8) as u8); crate::simd_ops::wu32(rdi.wrapping_add(P1), (0x0u32) as u32); crate::simd_ops::wu8(rdi.wrapping_add(P2), (0x0u8) as u8); rdx = ((rsi.wrapping_add(P4)) as u64) as u64; rcx = rdi; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rax = ((crate::simd_ops::ru16(rsi.wrapping_add(P5))) as u32) as u32 as u64; crate::simd_ops::wu16(rdi.wrapping_add(P6), ((rax as u16)) as u16); rdi = rbp.wrapping_add(0xd4); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rdi); crate::simd_ops::lk_and_u32(rdi, (0xff000010u32) as u32); crate::simd_ops::lk_or_u32(rdi, (0x10u32) as u32); crate::simd_ops::wu8(rdi.wrapping_add(P0), (0x10u8) as u8); crate::simd_ops::wu32(rdi.wrapping_add(P1), (0x0u32) as u32); crate::simd_ops::wu8(rdi.wrapping_add(P2), (0x0u8) as u8); rdx = ((rsi.wrapping_add(P7)) as u64) as u64; rcx = rdi; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rax = ((crate::simd_ops::ru16(rsi.wrapping_add(P8))) as u32) as u32 as u64; crate::simd_ops::wu16(rdi.wrapping_add(P6), ((rax as u16)) as u16); rax = rbp; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142193690(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xa78; const P1: u64 = 0x141298060; const P2: u64 = 0x9; const P3: u64 = 0x14128fd40;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(P0))) as u8 as u64; fl_r = (0x0u8) as u8 as u64;
    rax = rdx;
    if !((((fl_l as u8)) == ((fl_r as u8)))) { rdx = rsp.wrapping_add(0x30); crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), (0x7u32) as u32); rcx = rax; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
    rcx = (crate::simd_ops::ru64(rax)) as u64;
    crate::simd_ops::wu8(rdx.wrapping_add(P2), (0x1u8) as u8);
    rdx = rsp.wrapping_add(0x30);
    crate::simd_ops::wu8(rsp.wrapping_add(0x34u64), (0x1u8) as u8);
    rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142193820(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0xa62;
    let (mut rax, mut fl_l, mut fl_r) = (0u64, 0u64, 0u64);
    fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(P0))) as u8 as u64; fl_r = (0x0u8) as u8 as u64;
    rax = (rax & !0xffu64) | (((if (((fl_l as u8)) != ((fl_r as u8))) { 1u8 } else { 0u8 }) as u8 as u64));
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142193890(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141299b50; const P1: u64 = 0x14243b5f0; const P2: u64 = 0x14183acc2; const P3: u64 = 0x1433e7798; const P4: u64 = 0x1426469b8; const P5: u64 = 0x1433e99c0;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 41>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = rsp; crate::simd_ops::wu64(rax.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rax.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rax.wrapping_add(0x8u64), rcx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x140u64)) as u64; rdi = rcx; rbx = rcx.wrapping_add(0x8); crate::simd_ops::wu64(rax.wrapping_add(0x8u64), rbx); rsi = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rbx, ((rsi as u32)) as u32); rcx = rbx.wrapping_add(0x8); rdx = ((rsi.wrapping_add(0x8u64)) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rbx.wrapping_add(0xa8); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (0x8u64) as u64); crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((rsi as u32)) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (0x178u64) as u64); r9 = ((rsi.wrapping_add(0x8u64)) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rbx.wrapping_add(0xe8u64), rsi); crate::simd_ops::wu64(rbx.wrapping_add(0xf0u64), rsi); crate::simd_ops::wu32(rbx.wrapping_add(0xf8u64), ((rsi as u32)) as u32); crate::simd_ops::wu8(rbx.wrapping_add(0xfcu64), ((rsi as u8)) as u8); crate::simd_ops::wu64(rbx.wrapping_add(0x100u64), rsi); crate::simd_ops::wu64(rbx.wrapping_add(0x108u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rsi); crate::simd_ops::wu32(rsp.wrapping_add(0x48u64), (0xffffffffu32) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x4cu64), (0x4u32) as u32); crate::simd_ops::wu8(rsp.wrapping_add(0x50u64), ((rsi as u8)) as u8); rdx = 0; fl_l = 0; fl_r = 0; r8 = 0xef; rcx = rsp.wrapping_add(0x51); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r8 = P3; rdx = 0xef; rcx = rsp.wrapping_add(0x51); (rax, xmm0) = crate::simd_ops::call_stk_xmm((crate::simd_ops::ru64(P4)) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rbx.wrapping_add(0x118); rax = rsp.wrapping_add(0x40); rdx = ((rsi.wrapping_add(0x2u64)) as u32) as u32 as u64; bb = 1; continue; }
            1 => { let y_addr: u64 = rax; xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rcx; crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); let y_addr: u64 = rax.wrapping_add(0x20u64); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rcx.wrapping_add(0x20u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); let y_addr: u64 = rax.wrapping_add(0x40u64); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rcx.wrapping_add(0x40u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); xmm0 = (crate::simd_ops::ru128(rax.wrapping_add(0x60u64))) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rcx.wrapping_add(0x60u64), (xmm0) as u128); rcx = rcx.wrapping_add(0x80); xmm1 = (crate::simd_ops::ru128(rax.wrapping_add(0x70u64))) as u128; crate::simd_ops::wu128(rcx.wrapping_sub(0x10u64), (xmm1) as u128); rax = rax.wrapping_add(0x80); fl_l = rdx; fl_r = (0x1u64) as u64; rdx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } crate::simd_ops::wu64(rbx.wrapping_add(0x218u64), rsi); crate::simd_ops::wu64(rbx.wrapping_add(0x220u64), rsi); crate::simd_ops::wu8(rbx.wrapping_add(0x228u64), ((rdx as u8)) as u8); rax = P5; crate::simd_ops::wu64(rbx.wrapping_add(0x110u64), rax); crate::simd_ops::wu64(rbx.wrapping_add(0x230u64), rsi); crate::simd_ops::wu64(rbx.wrapping_add(0x238u64), rsi); crate::simd_ops::wu8(rbx.wrapping_add(0x240u64), (0x1u8) as u8); crate::simd_ops::wu64(rdi.wrapping_add(0x250u64), rsi); rax = rdi; r11 = rsp.wrapping_add(0x140); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x18u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x20u64))) as u64; rsp = r11; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142193e50(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    crate::simd_ops::forward_incoming_to_outgoing();
    const P0: u64 = 0x1412a88c0;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = 0;
    fl_l = 0; fl_r = 0;
    let tmp_xchg: u64 = crate::simd_ops::atomic_xchg_u64(rcx, rax);
    rax = (tmp_xchg) as u64;
    fl_l = ((rcx).wrapping_add(0x10u64)) as u64; fl_r = fl_l;
    rcx = ((fl_l as u64)) as u64;
    crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8(P0, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142193eb0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x4;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    crate::simd_ops::wu32(rcx, (0x1u32) as u32);
    rax = rcx;
    crate::simd_ops::wu8(rcx.wrapping_add(P0), (0x0u8) as u8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142193fc0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x1; const P1: u64 = 0x8;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    crate::simd_ops::wu8(rcx.wrapping_add(P0), (0x0u8) as u8);
    rax = rcx;
    crate::simd_ops::wu64(rcx.wrapping_add(P1), (0x0u64) as u64);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142194160(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x959; const P1: u64 = 0xa78;
    let (mut rax, mut fl_l, mut fl_r) = (0u64, 0u64, 0u64);
    fl_l = rcx; fl_r = rcx;
    if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; return rax; }
    fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(P0))) as u8 as u64; fl_r = (0x0u8) as u8 as u64;
    if !((((fl_l as u8)) != ((fl_r as u8)))) { fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(P1))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; rax = (rax & !0xffu64) | (((if (((fl_l as u8)) != ((fl_r as u8))) { 1u8 } else { 0u8 }) as u8 as u64)); return rax; }
    rax = (rax & !0xffu64) | (((0u8) as u8 as u64));
    fl_l = 0; fl_r = 0;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1421941b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x50; const P1: u64 = 0x10; const P2: u64 = 0x1426455a8; const P3: u64 = 0x8; const P4: u64 = 0x4; const P5: u64 = 0x142645598;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (crate::simd_ops::ru64(rdx)) as u64; rsi = rcx; rcx = rdx; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64, rax, rcx, rdx, r8, r9, rsp); rdi = rax; rcx = ((rax.wrapping_add(P1)) as u64) as u64; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P2)) as u64, rax, rcx, rdx, r8, r9, rsp); rdx = (crate::simd_ops::ru64(rdi)) as u64; rax = (crate::simd_ops::ru32(rdi.wrapping_add(P3))) as u32 as u64; rax = ((rdx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; fl_l = rdx; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 3; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru32(rdx.wrapping_add(P4))) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 2; continue; } rdx = ((rdx).wrapping_add(0x8u64)) as u64; fl_l = rdx; fl_r = rax; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } bb = 3; continue; }
            2 => { crate::simd_ops::wu32(rdx.wrapping_add(P4), (0x0u32) as u32); bb = 3; continue; }
            3 => { rcx = ((rdi.wrapping_add(P1)) as u64) as u64; crate::simd_ops::wu8(rsi.wrapping_add(P4), (0x1u8) as u8); rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P5)) as u64, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = rsi; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142194440(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x50; const P1: u64 = 0x10; const P2: u64 = 0x1426455a8; const P3: u64 = 0x8; const P4: u64 = 0x4; const P5: u64 = 0x18; const P6: u64 = 0x142645598;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rcx = (crate::simd_ops::ru64(rcx)) as u64; r14 = r8; rsi = rdx; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64, rax, rcx, rdx, r8, r9, rsp); rdi = rax; rcx = ((rax.wrapping_add(P1)) as u64) as u64; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P2)) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rdi)) as u64; r8 = 0; fl_l = 0; fl_r = 0; rax = (crate::simd_ops::ru32(rdi.wrapping_add(P3))) as u32 as u64; r10 = ((r8 as u32)) as u32 as u64; r9 = 0xe10; rax = ((rcx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; fl_l = rcx; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 3; continue; } rdx = (crate::simd_ops::ru32(r14)) as u32 as u64; bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru32(rcx)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 2; continue; } rcx = ((rcx).wrapping_add(0x8u64)) as u64; fl_l = rcx; fl_r = rax; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } bb = 3; continue; }
            2 => { fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(P4))) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 3; continue; } r10 = 0x64; r9 = r8; bb = 3; continue; }
            3 => { rcx = ((rdi.wrapping_add(P1)) as u64) as u64; crate::simd_ops::wu32(rsi, ((r10 as u32)) as u32); crate::simd_ops::wu64(rsi.wrapping_add(P3), r8); crate::simd_ops::wu64(rsi.wrapping_add(P1), r8); crate::simd_ops::wu64(rsi.wrapping_add(P5), r9); rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P6)) as u64, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = rsi; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1421948d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x50; const P1: u64 = 0x10; const P2: u64 = 0x1426455a8; const P3: u64 = 0x8; const P4: u64 = 0x142645598; const P5: u64 = 0x4;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rcx = (crate::simd_ops::ru64(rcx)) as u64; rsi = rdx; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64, rax, rcx, rdx, r8, r9, rsp); rdi = rax; rbx = ((rax.wrapping_add(P1)) as u64) as u64; rcx = rbx; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P2)) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rdi)) as u64; rax = (crate::simd_ops::ru32(rdi.wrapping_add(P3))) as u32 as u64; rax = ((rcx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; fl_l = rcx; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 2; continue; } rdx = (crate::simd_ops::ru32(rsi)) as u32 as u64; bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru32(rcx)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 4; continue; } rcx = ((rcx).wrapping_add(0x8u64)) as u64; fl_l = rcx; fl_r = rax; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } bb = 2; continue; }
            2 => { rdi = 0x2; bb = 3; continue; }
            3 => { rcx = rbx; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P4)) as u64, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = ((rdi as u32)) as u32 as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            4 => { rdi = (crate::simd_ops::ru32(rcx.wrapping_add(P5))) as u32 as u64; bb = 3; continue; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142194a30(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x4;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = (crate::simd_ops::ru32(r8)) as u32 as u64;
    crate::simd_ops::wu32(rdx, ((rax as u32)) as u32);
    rax = rdx;
    crate::simd_ops::wu8(rdx.wrapping_add(P0), (0x1u8) as u8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142194a50(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x18; const P1: u64 = 0x140208a00;
    let (mut rax, mut r10, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x28u64)) as u64; rax = (((crate::simd_ops::ru8(rdx.wrapping_add(P0))) as i8 as i64) as u64) as u64; r8 = rdx; fl_l = ((rax).wrapping_add(0x1u64)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; if (((fl_l as u64)) == 0) { bb = 2; continue; } fl_l = rax; fl_r = (0x1u64) as u64; r9 = rcx; rax = 0x4; r10 = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 1; continue; } r8 = 0x77; rdx = (rdx & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rcx, ((r8 as u32)) as u32); crate::simd_ops::wu8(rcx.wrapping_add(rax), ((rdx as u8)) as u8); rax = rcx; rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
            1 => { r8 = (crate::simd_ops::ru32(r8)) as u32 as u64; rdx = (rdx & !0xffu64) | (((0x1u8) as u8 as u64)); crate::simd_ops::wu32(r9, ((r8 as u32)) as u32); crate::simd_ops::wu8(r10.wrapping_add(rax), ((rdx as u8)) as u8); rax = rcx; rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
            2 => { rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x142194aa2, rcx, rsp); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142194c30(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xa78; const P1: u64 = 0x1401fb9f0; const P2: u64 = 0x9; const P3: u64 = 0x1412aa230;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rbp = rsp; rsp = ((rsp).wrapping_sub(0x50u64)) as u64; rbx = rdx; xmm0 = 0u128; crate::simd_ops::wu128(rbp.wrapping_sub(0x30u64), (xmm0) as u128); rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rbp.wrapping_sub(0x30u64), rax); crate::simd_ops::wu64(rbp.wrapping_sub(0x28u64), rax); crate::simd_ops::wu8(rbp.wrapping_sub(0x20u64), (0x1u8) as u8); crate::simd_ops::wu8(rbp.wrapping_sub(0x8u64), (0x1u8) as u8); fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(P0))) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 1; continue; } crate::simd_ops::wu64(rbp.wrapping_sub(0x30u64), rax); crate::simd_ops::wu64(rbp.wrapping_sub(0x28u64), rax); crate::simd_ops::wu64(rbp.wrapping_sub(0x18u64), rax); crate::simd_ops::wu64(rbp.wrapping_sub(0x10u64), rax); crate::simd_ops::wu8(rbp.wrapping_sub(0x20u64), (0x1u8) as u8); rcx = rbp.wrapping_sub(0x18); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 2; continue; }
            1 => { crate::simd_ops::wu64(rbp.wrapping_sub(0x18u64), rax); crate::simd_ops::wu64(rbp.wrapping_sub(0x10u64), rax); rcx = rbp.wrapping_sub(0x30); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru64(rbp.wrapping_sub(0x18u64))) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x30u64), rax); rax = (crate::simd_ops::ru32(rbp.wrapping_sub(0x10u64))) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_sub(0x28u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rbp.wrapping_sub(0xcu64))) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_sub(0x24u64), ((rax as u32)) as u32); rax = ((crate::simd_ops::ru8(rbp.wrapping_sub(0x8u64))) as u32) as u32 as u64; crate::simd_ops::wu8(rbp.wrapping_sub(0x20u64), ((rax as u8)) as u8); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 2; continue; } rcx = rbp.wrapping_sub(0x18); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 2; continue; }
            2 => { crate::simd_ops::wu8(rbx.wrapping_add(P2), (0x1u8) as u8); rdx = rbp.wrapping_sub(0x30); rcx = (crate::simd_ops::ru64(rbx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = (crate::simd_ops::ru8(rbp.wrapping_sub(0x20u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 3; continue; } rcx = rbp.wrapping_sub(0x30); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 3; continue; }
            3 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rsp = ((rsp).wrapping_add(0x50u64)) as u64; rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142194fb0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x9; const P1: u64 = 0x1412a9520;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    rcx = (crate::simd_ops::ru64(rdx)) as u64;
    crate::simd_ops::wu8(rdx.wrapping_add(P0), (0x1u8) as u8);
    rdx = rsp.wrapping_add(0x30);
    crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), (0x2u32) as u32);
    crate::simd_ops::wu8(rsp.wrapping_add(0x34u64), (0x1u8) as u8);
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142194ff0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x11; const P2: u64 = 0x14128fd40;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    crate::simd_ops::wu8(rdx.wrapping_add(P1), (0x1u8) as u8);
    rdx = rsp.wrapping_add(0x30);
    crate::simd_ops::wu8(rsp.wrapping_add(0x34u64), (0x1u8) as u8);
    rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142195370(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141299b50; const P1: u64 = 0x14243b5f0; const P2: u64 = 0x14183acc2; const P3: u64 = 0x1433e7798; const P4: u64 = 0x1426469b8; const P5: u64 = 0x1433e9a80;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 41>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = rsp; crate::simd_ops::wu64(rax.wrapping_add(0x18u64), rbx); crate::simd_ops::wu64(rax.wrapping_add(0x20u64), rsi); crate::simd_ops::wu64(rax.wrapping_add(0x8u64), rcx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x140u64)) as u64; rdi = rcx; rsi = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rcx, rsi); crate::simd_ops::wu32(rcx.wrapping_add(0x8u64), ((rsi as u32)) as u32); crate::simd_ops::wu32(rcx.wrapping_add(0xcu64), ((rsi as u32)) as u32); crate::simd_ops::wu64(rcx.wrapping_add(0x10u64), rsi); rbx = rcx.wrapping_add(0x18); crate::simd_ops::wu64(rax.wrapping_add(0x10u64), rbx); crate::simd_ops::wu32(rbx, ((rsi as u32)) as u32); rcx = rbx.wrapping_add(0x8); rdx = ((rsi.wrapping_add(0x20u64)) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rbx.wrapping_add(0xa8); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (0x8u64) as u64); crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((rsi as u32)) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (0x40u64) as u64); r9 = ((rsi.wrapping_add(0x20u64)) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rbx.wrapping_add(0xe8u64), rsi); crate::simd_ops::wu64(rbx.wrapping_add(0xf0u64), rsi); crate::simd_ops::wu32(rbx.wrapping_add(0xf8u64), ((rsi as u32)) as u32); crate::simd_ops::wu8(rbx.wrapping_add(0xfcu64), ((rsi as u8)) as u8); crate::simd_ops::wu64(rbx.wrapping_add(0x100u64), rsi); crate::simd_ops::wu64(rbx.wrapping_add(0x108u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rsi); crate::simd_ops::wu32(rsp.wrapping_add(0x48u64), (0xffffffffu32) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x4cu64), (0x4u32) as u32); crate::simd_ops::wu8(rsp.wrapping_add(0x50u64), ((rsi as u8)) as u8); rdx = 0; fl_l = 0; fl_r = 0; r8 = 0xef; rcx = rsp.wrapping_add(0x51); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r8 = P3; rdx = 0xef; rcx = rsp.wrapping_add(0x51); (rax, xmm0) = crate::simd_ops::call_stk_xmm((crate::simd_ops::ru64(P4)) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rbx.wrapping_add(0x118); rax = rsp.wrapping_add(0x40); rdx = ((rsi.wrapping_add(0x2u64)) as u32) as u32 as u64; bb = 1; continue; }
            1 => { let y_addr: u64 = rax; xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rcx; crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); let y_addr: u64 = rax.wrapping_add(0x20u64); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rcx.wrapping_add(0x20u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); let y_addr: u64 = rax.wrapping_add(0x40u64); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rcx.wrapping_add(0x40u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); xmm0 = (crate::simd_ops::ru128(rax.wrapping_add(0x60u64))) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rcx.wrapping_add(0x60u64), (xmm0) as u128); rcx = rcx.wrapping_add(0x80); xmm1 = (crate::simd_ops::ru128(rax.wrapping_add(0x70u64))) as u128; crate::simd_ops::wu128(rcx.wrapping_sub(0x10u64), (xmm1) as u128); rax = rax.wrapping_add(0x80); fl_l = rdx; fl_r = (0x1u64) as u64; rdx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } crate::simd_ops::wu64(rbx.wrapping_add(0x218u64), rsi); crate::simd_ops::wu64(rbx.wrapping_add(0x220u64), rsi); crate::simd_ops::wu8(rbx.wrapping_add(0x228u64), ((rdx as u8)) as u8); rax = P5; crate::simd_ops::wu64(rbx.wrapping_add(0x110u64), rax); crate::simd_ops::wu64(rbx.wrapping_add(0x230u64), rsi); crate::simd_ops::wu64(rbx.wrapping_add(0x238u64), rsi); crate::simd_ops::wu8(rbx.wrapping_add(0x240u64), (0x1u8) as u8); crate::simd_ops::wu64(rdi.wrapping_add(0x260u64), rsi); rax = rdi; r11 = rsp.wrapping_add(0x140); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x20u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x28u64))) as u64; rsp = r11; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142195b30(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x18; const P1: u64 = 0x10; const P2: u64 = 0x28;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (crate::simd_ops::ru64(r9)) as u64; rbp = rcx; rcx = r9; rdi = r9; r14 = r8; rbx = rdx; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64, rax, rcx, rdx, r8, r9, rsp); rsi = rax; rcx = rdi; rax = (crate::simd_ops::ru64(rdi)) as u64; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64, rax, rcx, rdx, r8, r9, rsp); fl_l = rsi; fl_r = rax; rsi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } rax = 0; fl_l = 0; fl_r = 0; bb = 3; continue; }
            1 => { r9 = (crate::simd_ops::ru64(rdi)) as u64; rdx = rbp; rbx = (((rbx) as i64).wrapping_mul((r14) as i64) as u64) as u64; rcx = rdi; fl_l = rbx; fl_r = rsi; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 2; continue; } r8 = rsi; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(r9.wrapping_add(P2))) as u64, rax, rcx, rdx, r8, r9, rsp); rax = rsi; bb = 3; continue; }
            2 => { r8 = rbx; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(r9.wrapping_add(P2))) as u64, rax, rcx, rdx, r8, r9, rsp); rax = rbx; bb = 3; continue; }
            3 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1421967f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142646e60;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    rcx = 0x3;
    rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P0)) as u64, rax, rcx, rdx, r8, r9, rsp);
    rcx = 0x1;
    rdx = 0x79;
    fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64;
    rcx = (if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { (rdx as u32) } else { (rcx as u32) }) as u32 as u64;
    rax = ((rcx as u32)) as u32 as u64;
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142196ba0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141299b50; const P1: u64 = 0x14243b5f0; const P2: u64 = 0x14183acc2; const P3: u64 = 0x1433e7798; const P4: u64 = 0x1426469b8; const P5: u64 = 0x1433e9c10;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 41>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = rsp; crate::simd_ops::wu64(rax.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rax.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rax.wrapping_add(0x8u64), rcx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x140u64)) as u64; rdi = rcx; rsi = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rcx.wrapping_add(0x8u64), rsi); rbx = rcx.wrapping_add(0x10); crate::simd_ops::wu64(rax.wrapping_add(0x8u64), rbx); crate::simd_ops::wu32(rbx, ((rsi as u32)) as u32); rcx = rbx.wrapping_add(0x8); rdx = ((rsi.wrapping_add(0x20u64)) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rbx.wrapping_add(0xa8); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (0x8u64) as u64); crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((rsi as u32)) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (0x270u64) as u64); r9 = ((rsi.wrapping_add(0x20u64)) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rbx.wrapping_add(0xe8u64), rsi); crate::simd_ops::wu64(rbx.wrapping_add(0xf0u64), rsi); crate::simd_ops::wu32(rbx.wrapping_add(0xf8u64), ((rsi as u32)) as u32); crate::simd_ops::wu8(rbx.wrapping_add(0xfcu64), ((rsi as u8)) as u8); crate::simd_ops::wu64(rbx.wrapping_add(0x100u64), rsi); crate::simd_ops::wu64(rbx.wrapping_add(0x108u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rsi); crate::simd_ops::wu32(rsp.wrapping_add(0x48u64), (0xffffffffu32) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x4cu64), (0x4u32) as u32); crate::simd_ops::wu8(rsp.wrapping_add(0x50u64), ((rsi as u8)) as u8); rdx = 0; fl_l = 0; fl_r = 0; r8 = 0xef; rcx = rsp.wrapping_add(0x51); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r8 = P3; rdx = 0xef; rcx = rsp.wrapping_add(0x51); (rax, xmm0) = crate::simd_ops::call_stk_xmm((crate::simd_ops::ru64(P4)) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rbx.wrapping_add(0x118); rax = rsp.wrapping_add(0x40); rdx = ((rsi.wrapping_add(0x2u64)) as u32) as u32 as u64; bb = 1; continue; }
            1 => { let y_addr: u64 = rax; xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rcx; crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); let y_addr: u64 = rax.wrapping_add(0x20u64); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rcx.wrapping_add(0x20u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); let y_addr: u64 = rax.wrapping_add(0x40u64); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rcx.wrapping_add(0x40u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); xmm0 = (crate::simd_ops::ru128(rax.wrapping_add(0x60u64))) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rcx.wrapping_add(0x60u64), (xmm0) as u128); rcx = rcx.wrapping_add(0x80); xmm1 = (crate::simd_ops::ru128(rax.wrapping_add(0x70u64))) as u128; crate::simd_ops::wu128(rcx.wrapping_sub(0x10u64), (xmm1) as u128); rax = rax.wrapping_add(0x80); fl_l = rdx; fl_r = (0x1u64) as u64; rdx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } crate::simd_ops::wu64(rbx.wrapping_add(0x218u64), rsi); crate::simd_ops::wu64(rbx.wrapping_add(0x220u64), rsi); crate::simd_ops::wu8(rbx.wrapping_add(0x228u64), ((rdx as u8)) as u8); rax = P5; crate::simd_ops::wu64(rbx.wrapping_add(0x110u64), rax); crate::simd_ops::wu64(rbx.wrapping_add(0x230u64), rsi); crate::simd_ops::wu64(rbx.wrapping_add(0x238u64), rsi); crate::simd_ops::wu8(rbx.wrapping_add(0x240u64), (0x1u8) as u8); crate::simd_ops::wu64(rdi.wrapping_add(0x258u64), rsi); rax = rdi; r11 = rsp.wrapping_add(0x140); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x18u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x20u64))) as u64; rsp = r11; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142196d30(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1423f0b30; const P1: u64 = 0x1412aac90; const P2: u64 = 0x141299f40; const P3: u64 = 0x1421968d0; const P4: u64 = 0x1433e9c28; const P5: u64 = 0x1401fa380; const P6: u64 = 0x142646e60;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 15>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x70u64)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rbp = rdx; rbx = rcx; (rax, xmm0) = crate::simd_ops::call_stk_xmm((crate::simd_ops::ru64(rax.wrapping_add(0x70u64))) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rsi = rax; rdi = rax.wrapping_add(0x10); crate::simd_ops::wu64(rdi.wrapping_add(0x230u64), rbx); crate::simd_ops::wu64(rdi.wrapping_add(0x238u64), rdi); crate::simd_ops::wu32(rdi.wrapping_add(0x11cu64), (0x10000u32) as u32); rax = (crate::simd_ops::ru32(rbp.wrapping_add(0xa6cu64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 1; continue; } crate::simd_ops::wu32(rdi.wrapping_add(0x120u64), ((rax as u32)) as u32); bb = 1; continue; }
            1 => { rcx = rdi.wrapping_add(0x110); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = ((rax as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rsi.wrapping_add(0x118u64))) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 2; continue; } rsi = ((rsi).wrapping_add(0x110u64)) as u64; rdx = ((rbx.wrapping_add(0x1u64)) as u32) as u32 as u64; rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 3; continue; }
            2 => { fl_l = ((rsi).wrapping_add(0x110u64)) as u64; fl_r = fl_l; rsi = ((fl_l as u64)) as u64; bb = 3; continue; }
            3 => { rax = P3; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax); rcx = rsp.wrapping_add(0x30); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; rax = P4; rdx = rbx; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); rax = rsp.wrapping_add(0x30); rdx = ((rdx).wrapping_shl(6u32)) as u64; rdx = ((rdx).wrapping_add(crate::simd_ops::ru64(rsi))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), rax); crate::simd_ops::wu128(rsp.wrapping_add(0x38u64), (xmm0) as u128); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = rsp.wrapping_add(0x30); fl_l = rcx; fl_r = rdx; rdx = (rdx & !0xffu64) | (((if (((fl_l as u64)) != ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); (rax, xmm0) = crate::simd_ops::call_stk_xmm((crate::simd_ops::ru64(rax.wrapping_add(0x20u64))) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 4; continue; }
            4 => { rcx = 0x3; (rax, xmm0) = crate::simd_ops::call_stk_xmm((crate::simd_ops::ru64(P6)) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r11 = rsp.wrapping_add(0x70); rcx = 0x79; rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x10u64))) as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; rbp = (crate::simd_ops::ru64(r11.wrapping_add(0x18u64))) as u64; rdx = 0x1; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x20u64))) as u64; rcx = (if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { (rdx as u32) } else { (rcx as u32) }) as u32 as u64; rax = ((rcx as u32)) as u32 as u64; rsp = r11; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142196e50(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x70; const P1: u64 = 0x250; const P2: u64 = 0x120; const P3: u64 = 0x14129bf30; const P4: u64 = 0x228; const P5: u64 = 0x142645748; const P6: u64 = 0x142646e58; const P7: u64 = 0x143406c90; const P8: u64 = 0x1423f0780; const P9: u64 = 0x14136eec0; const P10: u64 = 0x142196eb9;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x142196e50, funcinfo_rva: 0x37b138c, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0x38, conts: &[] };
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142196e59); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rbx = (rax) as u64; rcx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; let tmp_xchg: u8 = crate::simd_ops::atomic_xchg_u8(rax.wrapping_add(P1), ((rcx as u8)) as u8); rcx = (rcx & !0xffu64) | (((tmp_xchg) as u8 as u64)); rcx = ((rax.wrapping_add(P2)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142196e6e); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P4))) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rdx = (0xffffffffu32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142196e84); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P5)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142196e8e); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P6)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rax = (0x1u32) as u32 as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rbx = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            3 => { rdx = ((P7) as u64) as u64; rcx = ((rsp.wrapping_add(0x20u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142196eab); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P8) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rcx = (rax) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142196eb4); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P9) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); crate::lift_ops::trap(0x80000003, P10, rcx, rsp); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142196f80(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x78; const P1: u64 = 0x230; const P2: u64 = 0x238; const P3: u64 = 0x11c; const P4: u64 = 0xa6c; const P5: u64 = 0x120; const P6: u64 = 0x110; const P7: u64 = 0x1423f0b30; const P8: u64 = 0x1412aafc0; const P9: u64 = 0x108; const P10: u64 = 0x1; const P11: u64 = 0x100; const P12: u64 = 0x141299f40; const P13: u64 = 0x142196f20; const P14: u64 = 0x1433e9c60; const P15: u64 = 0x1401fa380;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 15>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x70u64)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rdi = rdx; rbx = rcx; (rax, xmm0) = crate::simd_ops::call_stk_xmm((crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rsi = rax; crate::simd_ops::wu64(rax.wrapping_add(P1), rbx); crate::simd_ops::wu64(rax.wrapping_add(P2), rax); crate::simd_ops::wu32(rax.wrapping_add(P3), (0x10000u32) as u32); rax = (crate::simd_ops::ru32(rdi.wrapping_add(P4))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 1; continue; } crate::simd_ops::wu32(rsi.wrapping_add(P5), ((rax as u32)) as u32); bb = 1; continue; }
            1 => { rcx = ((rsi.wrapping_add(P6)) as u64) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P8, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = ((rax as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rsi.wrapping_add(P9))) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 2; continue; } rdx = ((rbx.wrapping_add(P10)) as u32) as u32 as u64; rcx = ((rsi.wrapping_add(P11)) as u64) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P12, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 2; continue; }
            2 => { rax = P13; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax); rcx = rsp.wrapping_add(0x30); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; rax = P14; rdx = rbx; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); rax = rsp.wrapping_add(0x30); rdx = ((rdx).wrapping_shl(6u32)) as u64; rdx = ((rdx).wrapping_add(crate::simd_ops::ru64(rsi.wrapping_add(0x100u64)))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), rax); crate::simd_ops::wu128(rsp.wrapping_add(0x38u64), (xmm0) as u128); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P15, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = rsp.wrapping_add(0x30); fl_l = rcx; fl_r = rdx; rdx = (rdx & !0xffu64) | (((if (((fl_l as u64)) != ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); (rax, xmm0) = crate::simd_ops::call_stk_xmm((crate::simd_ops::ru64(rax.wrapping_add(0x20u64))) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 3; continue; }
            3 => { r11 = rsp.wrapping_add(0x70); rax = 0x1; rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x10u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x18u64))) as u64; rsp = r11; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142197080(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x78; const P1: u64 = 0x110; const P2: u64 = 0x130; const P3: u64 = 0x14129bf30; const P4: u64 = 0x108; const P5: u64 = 0x142645748; const P6: u64 = 0x143406c90; const P7: u64 = 0x1423f0780; const P8: u64 = 0x14136eec0; const P9: u64 = 0x1421970e3;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x142197080, funcinfo_rva: 0x37b13cc, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0x38, conts: &[] };
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142197089); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rbx = ((rax.wrapping_add(P1)) as u64) as u64; rax = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; let tmp_xchg: u8 = crate::simd_ops::atomic_xchg_u8(rbx.wrapping_add(P2), ((rax as u8)) as u8); rax = (rax & !0xffu64) | (((tmp_xchg) as u8 as u64)); rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14219709e); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P4))) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rdx = (0xffffffffu32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1421970b4); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P5)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { rax = (0x1u32) as u32 as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rbx = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            3 => { rdx = ((P6) as u64) as u64; rcx = ((rsp.wrapping_add(0x20u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1421970d5); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P7) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rcx = (rax) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1421970de); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P8) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); crate::lift_ops::trap(0x80000003, P9, rcx, rsp); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1421970f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x10;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(P0))) as u8 as u64; fl_r = (0x0u8) as u8 as u64;
    if !((((fl_l as u8)) == ((fl_r as u8)))) { xmm0 = (crate::simd_ops::ru128(rdx)) as u128; crate::simd_ops::wu128(rcx, (xmm0) as u128); rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P0))) as u32) as u32 as u64; crate::simd_ops::wu8(rcx.wrapping_add(P0), ((rax as u8)) as u8); rax = rcx; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
    rax = (crate::simd_ops::ru32(rdx)) as u32 as u64;
    crate::simd_ops::wu32(rcx, ((rax as u32)) as u32);
    rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P0))) as u32) as u32 as u64;
    crate::simd_ops::wu8(rcx.wrapping_add(P0), ((rax as u8)) as u8);
    rax = rcx;
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142197370(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x18; const P1: u64 = 0x10;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(P0))) as u8 as u64; fl_r = (0x0u8) as u8 as u64;
    if !((((fl_l as u8)) == ((fl_r as u8)))) { fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(P1))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if !((((fl_l as u8)) == ((fl_r as u8)))) { xmm0 = (crate::simd_ops::ru128(rdx)) as u128; crate::simd_ops::wu128(rcx, (xmm0) as u128); rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P1))) as u32) as u32 as u64; crate::simd_ops::wu8(rcx.wrapping_add(P1), ((rax as u8)) as u8); crate::simd_ops::set_xmm_ret(xmm0); return rax; } rax = (crate::simd_ops::ru32(rdx)) as u32 as u64; crate::simd_ops::wu32(rcx, ((rax as u32)) as u32); rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P1))) as u32) as u32 as u64; crate::simd_ops::wu8(rcx.wrapping_add(P1), ((rax as u8)) as u8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
    rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P1))) as u32) as u32 as u64;
    crate::simd_ops::wu8(rcx.wrapping_add(P1), ((rax as u8)) as u8);
    fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64;
    if !(((((fl_l as u8)) & ((fl_r as u8))) == 0)) { xmm0 = (crate::simd_ops::ru128(rdx)) as u128; crate::simd_ops::wu128(rcx, (xmm0) as u128); crate::simd_ops::wu8(rcx.wrapping_add(P0), (0x1u8) as u8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
    rax = (crate::simd_ops::ru32(rdx)) as u32 as u64;
    crate::simd_ops::wu32(rcx, ((rax as u32)) as u32);
    crate::simd_ops::wu8(rcx.wrapping_add(P0), (0x1u8) as u8);
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142197d40(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x10;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P0))) as u32) as u32 as u64;
    crate::simd_ops::wu8(rcx.wrapping_add(P0), ((rax as u8)) as u8);
    fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64;
    if !(((((fl_l as u8)) & ((fl_r as u8))) == 0)) { xmm0 = (crate::simd_ops::ru128(rdx)) as u128; crate::simd_ops::wu128(rcx, (xmm0) as u128); rax = rcx; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
    rax = (crate::simd_ops::ru32(rdx)) as u32 as u64;
    crate::simd_ops::wu32(rcx, ((rax as u32)) as u32);
    rax = rcx;
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1421985d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x20; const P1: u64 = 0x28; const P2: u64 = 0x8;
    let (mut rax, mut fl_l) = (crate::simd_ops::get_rax_in(), 0u64);
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    crate::simd_ops::wu64(rdx, rax);
    crate::simd_ops::wu8(rcx.wrapping_add(P1), (0x1u8) as u8);
    crate::simd_ops::wu32(rdx.wrapping_add(P2), ((r8 as u32)) as u32);
    fl_l = (crate::simd_ops::atomic_add_u32(rax.wrapping_add(P0), 1)) as u32 as u64;
    rax = rdx;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1421985f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142199fd0; const P1: u64 = 0x20; const P2: u64 = 0x1412ad250; const P3: u64 = 0x28; const P4: u64 = 0x8; const P5: u64 = 0x142442de0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        core::sync::atomic::fence(core::sync::atomic::Ordering::SeqCst);
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rbx = rcx; rcx = ((rcx).wrapping_add(0x8u64)) as u64; r8 = rdx; rdx = rsp.wrapping_add(0x20); rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rcx = ((rbx.wrapping_add(P1)) as u64) as u64; rdx = rax; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } rax = 0xffffffff; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rbx.wrapping_add(P1), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 2; continue; } fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } rdi = (crate::simd_ops::ru64(rbx.wrapping_add(P3))) as u64; fl_l = (((rdi) & (0xfffffffffffffffcu64))) as u64; fl_r = fl_l; rdi = ((fl_l as u64)) as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 1; continue; } rax = (crate::simd_ops::ru64(rdi)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = rdi; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(rax.wrapping_add(P4))) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = rdi; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); bb = 1; continue; }
            1 => { rcx = rbx; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); bb = 2; continue; }
            2 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142198960(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x18; const P1: u64 = 0x20; const P2: u64 = 0x8;
    let (mut rax, mut fl_l) = (crate::simd_ops::get_rax_in(), 0u64);
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    crate::simd_ops::wu64(rdx, rax);
    crate::simd_ops::wu8(rcx.wrapping_add(P1), (0x1u8) as u8);
    crate::simd_ops::wu32(rdx.wrapping_add(P2), ((r8 as u32)) as u32);
    fl_l = (crate::simd_ops::atomic_add_u32(rax.wrapping_add(P1), 1)) as u32 as u64;
    rax = rdx;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142198fa0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0x11; const P3: u64 = 0x1412ad320; const P4: u64 = 0x142198400; const P5: u64 = 0x142442de0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rax = (crate::simd_ops::ru32(rcx)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x13; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 3; continue; } rbx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rbx); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 1; continue; } rax = (crate::simd_ops::ru64(rbx)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); rax = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax); crate::simd_ops::wu8(rsp.wrapping_add(0x30u64), (0x1u8) as u8); bb = 2; continue; }
            1 => { crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), (0x11u32) as u32); crate::simd_ops::wu8(rsp.wrapping_add(0x30u64), (0x0u8) as u8); bb = 2; continue; }
            2 => { crate::simd_ops::wu8(rbx.wrapping_add(P2), (0x1u8) as u8); rdx = rsp.wrapping_add(0x20); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P0))) as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } rcx = ((rbx.wrapping_add(P0)) as u64) as u64; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rcx = rbx; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); bb = 3; continue; }
            3 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142199020(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0x11; const P3: u64 = 0x1412ad320; const P4: u64 = 0x142198400; const P5: u64 = 0x142442de0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rax = (crate::simd_ops::ru32(rdx)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x13; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 3; continue; } rbx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rbx); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 1; continue; } rax = (crate::simd_ops::ru64(rbx)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); rax = (crate::simd_ops::ru64(rdx.wrapping_add(P1))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax); crate::simd_ops::wu8(rsp.wrapping_add(0x30u64), (0x1u8) as u8); bb = 2; continue; }
            1 => { crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), (0x11u32) as u32); crate::simd_ops::wu8(rsp.wrapping_add(0x30u64), (0x0u8) as u8); bb = 2; continue; }
            2 => { crate::simd_ops::wu8(rbx.wrapping_add(P2), (0x1u8) as u8); rdx = rsp.wrapping_add(0x20); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P0))) as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } rcx = ((rbx.wrapping_add(P0)) as u64) as u64; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rcx = rbx; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); bb = 3; continue; }
            3 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1421991e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x28; const P1: u64 = 0x8; const P2: u64 = 0x142442de0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    fl_l = (((rbx) & (0xfffffffffffffffcu64))) as u64; fl_r = fl_l;
    rbx = ((fl_l as u64)) as u64;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = rbx; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = rbx; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); }
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142199470(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x4;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    fl_l = (crate::simd_ops::ru32(r8)) as u32 as u64; fl_r = 0x0;
    rax = rdx;
    if !((((fl_l as u32)) != ((fl_r as u32)))) { crate::simd_ops::wu8(rdx.wrapping_add(P0), (0x1u8) as u8); return rax; }
    crate::simd_ops::wu32(rdx, (0x11u32) as u32);
    crate::simd_ops::wu8(rdx.wrapping_add(P0), (0x0u8) as u8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142199490(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    crate::simd_ops::forward_incoming_to_outgoing();
    const P0: u64 = 0x142645168;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rcx = (crate::simd_ops::ru64(rcx)) as u64;
    rcx = (crate::simd_ops::ru64(rcx)) as u64;
    crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((crate::simd_ops::ru64(P0)) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142199770(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x120; const P1: u64 = 0x8; const P2: u64 = 0x98; const P3: u64 = 0x140200070;
    let (mut rax, mut fl_l, mut fl_r) = (0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x28u64)) as u64; fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(P0))) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 1; continue; } rax = (crate::simd_ops::ru64(rcx)) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64; fl_l = (crate::simd_ops::ru64(rdx.wrapping_add(P2))) as u64; fl_r = rcx; rax = (rax & !0xffu64) | (((if (((fl_l as u64)) == ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
            1 => { rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x142199798, rcx, rsp); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142199df0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1412ac9d0; const P1: u64 = 0x1412900a0; const P2: u64 = 0x141290a00; const P3: u64 = 0x142442de0; const P4: u64 = 0x1426451d0; const P5: u64 = 0x1412acb60; const P6: u64 = 0x142198ba0; const P7: u64 = 0x1426451c8;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 27>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x142199df0, funcinfo_rva: 0x37b1d7c, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0xd8, conts: &[] };
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rcx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rbp = ((rsp.wrapping_sub(0x47u64)) as u64) as u64; rsp = ((rsp).wrapping_sub(0xc0u64)) as u64; rbx = (rdx) as u64; rsi = (rcx) as u64; rdi = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rbp.wrapping_sub(0x59u64), ((rdi as u32)) as u32); xmm0 = 0u128; ymm0_hi = 0; let y_addr: u64 = rbp.wrapping_add(0xfu64); core::ptr::write_unaligned(y_addr as *mut u128, xmm0); core::ptr::write_unaligned(y_addr.wrapping_add(16) as *mut u128, ymm0_hi); crate::simd_ops::wu128(rbp.wrapping_add(0x2fu64), (xmm0) as u128); crate::simd_ops::wu32(rbp.wrapping_add(0xfu64), (0x3u32) as u32); rax = ((rbp.wrapping_add(0xfu64)) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x31u64), (rax) as u64); rax = ((rbp.wrapping_add(0xfu64)) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x29u64), (rax) as u64); rax = ((rbp.wrapping_add(0xfu64)) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x21u64), (rax) as u64); rax = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; crate::simd_ops::wu8(rbp.wrapping_sub(0x19u64), ((rax as u8)) as u8); rdx = (r8) as u64; rcx = ((rbp.wrapping_sub(0x31u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142199e50); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 7; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu32(rbp.wrapping_sub(0x1u64), (0x1au32) as u32); crate::simd_ops::wu8(rbp.wrapping_add(0x7u64), ((rdi as u8)) as u8); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142199e64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rbx = (rax) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x11u64), (rax) as u64); crate::simd_ops::wu16(rbp.wrapping_sub(0x9u64), (0x100u16) as u16); rdx = ((rbp.wrapping_sub(0x1u64)) as u64) as u64; rcx = (rax) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142199e7d); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu64(rsi, (rbx) as u64); crate::simd_ops::wu32(rsi.wrapping_add(0x8u64), ((rdi as u32)) as u32); crate::simd_ops::lk_add_u32(rbx.wrapping_add(0x18u64), (1) as u32); crate::simd_ops::wu32(rbp.wrapping_sub(0x59u64), (0x2u32) as u32); fl_l = (rbx) as u64 as u64; fl_r = (rbx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 2; continue; }
            2 => { rax = (0xffffffffu32) as u32 as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rbx.wrapping_add(0x18u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x1u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 6; continue; } bb = 3; continue; }
            3 => { rdi = (crate::simd_ops::ru64(rbx.wrapping_add(0x20u64))) as u64; fl_l = (((rdi) & (0xfffffffffffffffcu64))) as u64 as u64; fl_r = fl_l; rdi = ((fl_l as u64)) as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(rdi)) as u64; rdx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rcx = (rdi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142199eb9); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = (rdi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142199ebf); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 5; continue; }
            5 => { rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142199ec8); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 6; continue; }
            6 => { bb = 8; continue; }
            7 => { crate::simd_ops::wu64(rbp.wrapping_sub(0x49u64), (0x2u64) as u64); crate::simd_ops::wu64(rbp.wrapping_sub(0x39u64), (0x1u64) as u64); rax = ((rbp.wrapping_add(0xfu64)) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x41u64), (rax) as u64); crate::simd_ops::wu64(rbp.wrapping_sub(0x51u64), (rbx) as u64); rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142199ef2); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = (crate::simd_ops::ru64(rax.wrapping_add(0x1928u64))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142199efc); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P4)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rbx = (rax) as u64; r8 = ((rbp.wrapping_sub(0x51u64)) as u64) as u64; rcx = ((rbp.wrapping_add(0x7fu64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142199f0d); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x7fu64))) as u64; crate::simd_ops::wu8(rcx.wrapping_add(0x10u64), (0x1u8) as u8); rdx = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(rsi, (rdx) as u64); crate::simd_ops::wu32(rsi.wrapping_add(0x8u64), ((rdi as u32)) as u32); crate::simd_ops::lk_add_u32(rdx.wrapping_add(0x18u64), (1) as u32); crate::simd_ops::wu32(rbp.wrapping_sub(0x59u64), (0x8u32) as u32); r8 = (crate::simd_ops::ru64(rbp.wrapping_add(0x7fu64))) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x7fu64), (rdi) as u64); r9 = ((P6) as u64) as u64; rdx = ((rbp.wrapping_sub(0x49u64)) as u64) as u64; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142199f46); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P7)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 8; continue; }
            8 => { rax = (rsi) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0xe8u64))) as u64; rsp = ((rsp).wrapping_add(0xc0u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rsi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rbp = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142199fd0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1426450f8; const P1: u64 = 0x1412ac7a0; const P2: u64 = 0x142198fa0; const P3: u64 = 0x142645150; const P4: u64 = 0x1412ab5d0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        core::sync::atomic::fence(core::sync::atomic::Ordering::SeqCst);
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rdi = rdx; rsi = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rsi as u32)) as u32); rax = (crate::simd_ops::ru32(r8)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 1; continue; } rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax); rax = (crate::simd_ops::ru64(r8.wrapping_add(0x10u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); crate::simd_ops::wu8(rsp.wrapping_add(0x38u64), (0x1u8) as u8); bb = 3; continue; }
            1 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x3; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 2; continue; } rax = (crate::simd_ops::ru64(r8.wrapping_add(0x18u64))) as u64; rdx = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; rcx = (crate::simd_ops::ru64(rcx)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (0x1u64) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x78u64), rdx); rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rax.wrapping_add(0x1928u64))) as u64; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P0)) as u64, rax, rcx, rdx, r8, r9, rsp); rbx = rax; r8 = rsp.wrapping_add(0x78); rcx = rsp.wrapping_add(0x70); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; crate::simd_ops::wu8(rcx.wrapping_add(0x10u64), (0x1u8) as u8); rdx = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(rdi, rdx); crate::simd_ops::wu32(rdi.wrapping_add(0x8u64), ((rsi as u32)) as u32); let _ = crate::simd_ops::atomic_add_u32(rdx.wrapping_add(0x20u64), (1) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), (0x8u32) as u32); r8 = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), rsi); r9 = P2; rdx = rsp.wrapping_add(0x28); rcx = rbx; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P3)) as u64, rax, rcx, rdx, r8, r9, rsp); rax = rdi; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            2 => { crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), (0x19u32) as u32); crate::simd_ops::wu8(rsp.wrapping_add(0x38u64), ((rsi as u8)) as u8); bb = 3; continue; }
            3 => { rdx = rsp.wrapping_add(0x28); rcx = rdi; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rax = rdi; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14219a0d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut ymm0_hi: u128 = 0;
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let y_addr: u64 = r8; xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16));
    let y_addr: u64 = rdx; crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi);
    rax = rdx;
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14219a120(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1426451d0; const P1: u64 = 0x142645198; const P2: u64 = 0x1412ab5d0; const P3: u64 = 0x1426450f8; const P4: u64 = 0x1412ac860; const P5: u64 = 0x142198e40; const P6: u64 = 0x1426451a8; const P7: u64 = 0x1412ab680; const P8: u64 = 0x1412ab0b0; const P9: u64 = 0x142442de0; const P10: u64 = 0x1426451f0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 27>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        core::sync::atomic::fence(core::sync::atomic::Ordering::SeqCst);
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rbp = rsp.wrapping_sub(0x27); rsp = ((rsp).wrapping_sub(0xa0u64)) as u64; r15 = r8; r14 = rdx; rsi = rcx; r12 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rbp.wrapping_sub(0x59u64), ((r12 as u32)) as u32); crate::simd_ops::wu64(rbp.wrapping_add(0x77u64), r12); crate::simd_ops::wu8(rbp.wrapping_sub(0x1u64), (0x1u8) as u8); rax = rbp.wrapping_add(0x77); crate::simd_ops::wu64(rbp.wrapping_add(0x7u64), rax); crate::simd_ops::wu32(rbp.wrapping_add(0x6fu64), (0x1u32) as u32); rax = (crate::simd_ops::ru64(rdx)) as u64; rcx = rdx; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rax.wrapping_add(0x1928u64))) as u64; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P0)) as u64, rax, rcx, rdx, r8, r9, rsp); r9 = rbp.wrapping_add(0x77); r8 = r15; rdx = rbp.wrapping_add(0x6f); rcx = rax; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P1)) as u64, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 1; continue; } crate::simd_ops::wu32(rbp.wrapping_add(0xfu64), (0x19u32) as u32); crate::simd_ops::wu8(rbp.wrapping_add(0x1fu64), ((r12 as u8)) as u8); rdx = rbp.wrapping_add(0xf); rcx = rsi; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 3; continue; }
            1 => { crate::simd_ops::wu32(rbp.wrapping_sub(0x19u64), (0x1u32) as u32); crate::simd_ops::wu32(rbp.wrapping_sub(0x9u64), ((r12 as u32)) as u32); rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x77u64))) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x11u64), rcx); crate::simd_ops::wu64(rbp.wrapping_sub(0x41u64), (0x2u64) as u64); rax = rbp.wrapping_sub(0x19); crate::simd_ops::wu64(rbp.wrapping_sub(0x39u64), rax); crate::simd_ops::wu64(rbp.wrapping_sub(0x31u64), r12); rax = (crate::simd_ops::ru64(r14)) as u64; rcx = r14; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rax.wrapping_add(0x1928u64))) as u64; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P3)) as u64, rax, rcx, rdx, r8, r9, rsp); rbx = rax; rcx = rbp.wrapping_add(0x7f); rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x7fu64))) as u64; crate::simd_ops::wu8(rcx.wrapping_add(0x10u64), (0x1u8) as u8); rdi = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x51u64), rdi); crate::simd_ops::wu32(rbp.wrapping_sub(0x49u64), ((r12 as u32)) as u32); let _ = crate::simd_ops::atomic_add_u32(rdi.wrapping_add(0x28u64), (1) as u32); crate::simd_ops::wu32(rbp.wrapping_sub(0x59u64), (0x4u32) as u32); r8 = (crate::simd_ops::ru64(rbp.wrapping_add(0x7fu64))) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x7fu64), r12); r9 = P5; rdx = rbp.wrapping_sub(0x41); rcx = rbx; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P6)) as u64, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu32(rbp.wrapping_sub(0x59u64), (0x2u32) as u32); crate::simd_ops::wu64(rbp.wrapping_sub(0x29u64), r14); crate::simd_ops::wu64(rbp.wrapping_sub(0x21u64), r15); r9 = rbp.wrapping_sub(0x29); r8 = 0; fl_l = 0; fl_r = 0; rdx = rsi; rcx = rdi; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu32(rbp.wrapping_sub(0x59u64), (0xau32) as u32); rcx = rbp.wrapping_sub(0x51); rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu32(rbp.wrapping_sub(0x59u64), (0x3u32) as u32); rcx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x51u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } rax = 0xffffffff; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rcx.wrapping_add(0x28u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 3; continue; } rax = (crate::simd_ops::ru64(rbp.wrapping_sub(0x51u64))) as u64; rbx = (crate::simd_ops::ru64(rax.wrapping_add(0x30u64))) as u64; fl_l = (((rbx) & (0xfffffffffffffffcu64))) as u64; fl_r = fl_l; rbx = ((fl_l as u64)) as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = rbx; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = rbx; rax = crate::simd_ops::call_stk(P9, rax, rcx, rdx, r8, r9, rsp); bb = 2; continue; }
            2 => { rcx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x51u64))) as u64; rax = crate::simd_ops::call_stk(P9, rax, rcx, rdx, r8, r9, rsp); bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x77u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P10)) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 4; continue; }
            4 => { rax = rsi; rsp = ((rsp).wrapping_add(0xa0u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14219a2f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x8;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    fl_l = (crate::simd_ops::ru32(r8)) as u32 as u64; fl_r = 0x0;
    if !((((fl_l as u32)) != ((fl_r as u32)))) { rax = (crate::simd_ops::ru64(rcx)) as u64; r8 = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64; crate::simd_ops::wu64(rdx, rax); rax = rdx; crate::simd_ops::wu64(rdx.wrapping_add(P1), r8); crate::simd_ops::wu8(rdx.wrapping_add(P0), (0x1u8) as u8); return rax; }
    crate::simd_ops::wu32(rdx, (0x11u32) as u32);
    rax = rdx;
    crate::simd_ops::wu8(rdx.wrapping_add(P0), (0x0u8) as u8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14219a320(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x1928; const P2: u64 = 0x1426450f8; const P3: u64 = 0x1412ac7a0; const P4: u64 = 0x8; const P5: u64 = 0x20; const P6: u64 = 0x142198fa0; const P7: u64 = 0x142645150;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        core::sync::atomic::fence(core::sync::atomic::Ordering::SeqCst);
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rdi = rcx; rsi = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rsi as u32)) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (0x1u64) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), r9); crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), r8); rax = (crate::simd_ops::ru64(rdx)) as u64; rcx = rdx; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P2)) as u64, rax, rcx, rdx, r8, r9, rsp); rbx = rax; r8 = rsp.wrapping_add(0x70); rcx = rsp.wrapping_add(0x68); rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; crate::simd_ops::wu8(rcx.wrapping_add(P0), (0x1u8) as u8); rdx = (crate::simd_ops::ru64(rcx.wrapping_add(P4))) as u64; crate::simd_ops::wu64(rdi, rdx); crate::simd_ops::wu32(rdi.wrapping_add(P4), ((rsi as u32)) as u32); let _ = crate::simd_ops::atomic_add_u32(rdx.wrapping_add(P5), (1) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), (0x4u32) as u32); r8 = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), rsi); r9 = P6; rdx = rsp.wrapping_add(0x28); rcx = rbx; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P7)) as u64, rax, rcx, rdx, r8, r9, rsp); rax = rdi; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14219ac60(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x142645198; const P2: u64 = 0x1426451f0; const P3: u64 = 0x1426451b0; const P4: u64 = 0x1426451b8; const P5: u64 = 0x142645160;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; r8 = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; r9 = rsp.wrapping_add(0x50); rbp = rdx; crate::simd_ops::wu32(rsp.wrapping_add(0x48u64), (0x1u32) as u32); rdx = rsp.wrapping_add(0x48); rdi = rcx; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P1)) as u64, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 1; continue; } rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P2)) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 1; continue; }
            1 => { rcx = rdi; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P3)) as u64, rax, rcx, rdx, r8, r9, rsp); rbx = 0; fl_l = 0; fl_r = 0; rsi = ((rax as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { rdx = ((rbx as u32)) as u32 as u64; rcx = rdi; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P4)) as u64, rax, rcx, rdx, r8, r9, rsp); rdx = (crate::simd_ops::ru64(rbp.wrapping_add(0x10u64))) as u64; rcx = rdi; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P5)) as u64, rax, rcx, rdx, r8, r9, rsp); rbx = (((rbx as u32)).wrapping_add(1)) as u32 as u64; fl_l = ((rbx as u32)) as u32 as u64; fl_r = ((rsi as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 2; continue; } bb = 3; continue; }
            3 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14219ad00(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142645158; const P1: u64 = 0x4; const P2: u64 = 0x3; const P3: u64 = 0x8; const P4: u64 = 0x142409b50;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 15>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { r11 = rsp; crate::simd_ops::wu64(r11.wrapping_add(0x8u64), rcx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x70u64)) as u64; rax = rdx; rbx = rcx; crate::simd_ops::wu32(r11.wrapping_add(0x18u64), (0x21u32) as u32); r8 = r11.wrapping_add(0x18); rdx = r11.wrapping_sub(0x38); rcx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm((crate::simd_ops::ru64(P0)) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::lk_and_u32(rbx, (0xff000008u32) as u32); crate::simd_ops::wu32(rbx.wrapping_add(P1), (0x0u32) as u32); crate::simd_ops::lk_or_u32(rbx, (0x8u32) as u32); crate::simd_ops::wu8(rbx.wrapping_add(P2), (0x8u8) as u8); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 1; continue; } rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x90u64))) as u32 as u64; rax = (((rax as u32)).wrapping_sub(1)) as u32 as u64; r9 = ((((rax as u32)) as i32 as i64) as u64) as u64; crate::simd_ops::wu8(rbx.wrapping_add(P3), (0x0u8) as u8); rax = rsp.wrapping_add(0x40); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), r9); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x30u64))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm0) as u128); r8 = 0; fl_l = 0; fl_r = 0; rdx = rsp.wrapping_add(0x30); rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = rbx; rsp = ((rsp).wrapping_add(0x70u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            1 => { crate::simd_ops::wu8(rbx.wrapping_add(P3), (0x0u8) as u8); rax = rbx; rsp = ((rsp).wrapping_add(0x70u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14219adc0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x28; const P1: u64 = 0x10; const P2: u64 = 0x1928; const P3: u64 = 0x1426451d0; const P4: u64 = 0x142199970; const P5: u64 = 0x142645188; const P6: u64 = 0x2d70; const P7: u64 = 0x1426450f8; const P8: u64 = 0x1421997a0; const P9: u64 = 0x1426451e0; const P10: u64 = 0x2d78;
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rbx = rcx; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64, rax, rcx, rdx, r8, r9, rsp); rdx = (crate::simd_ops::ru64(rbx)) as u64; rcx = rbx; rdi = rax; crate::simd_ops::wu32(rsp.wrapping_add(0x38u64), (0x1u32) as u32); rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(rdx.wrapping_add(P1))) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P3)) as u64, rax, rcx, rdx, r8, r9, rsp); r9 = P4; r8 = rbx; rcx = rax; rdx = rsp.wrapping_add(0x38); rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P5)) as u64, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rdi.wrapping_add(P6), rax); rcx = rbx; rdx = (crate::simd_ops::ru64(rbx)) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), (0x1u32) as u32); rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(rdx.wrapping_add(P1))) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P7)) as u64, rax, rcx, rdx, r8, r9, rsp); r9 = P8; r8 = rbx; rcx = rax; rdx = rsp.wrapping_add(0x30); rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P9)) as u64, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; crate::simd_ops::wu64(rdi.wrapping_add(P10), rax); rax = 0x1; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14219b6f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1426451d8;
    let (mut rax, mut fl_l, mut fl_r) = (0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P0)) as u64, rax, rcx, rdx, r8, r9, rsp);
    fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64;
    rax = (rax & !0xffu64) | (((if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { 1u8 } else { 0u8 }) as u8 as u64));
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14219bb50(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    crate::simd_ops::forward_incoming_to_outgoing();
    const P0: u64 = 0x143a8a010; const P1: u64 = 0x14248c9d0;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rdx = rcx;
    rcx = P0;
    crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8(P1, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14219bb70(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    crate::simd_ops::forward_incoming_to_outgoing();
    const P0: u64 = 0x143a8a010; const P1: u64 = 0x14248c910;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    r8 = rdx;
    rdx = rcx;
    rcx = P0;
    crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8(P1, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14219c010(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x8; const P2: u64 = 0x18; const P3: u64 = 0xffffffffffffffc8; const P4: u64 = 0x142645158; const P5: u64 = 0x3; const P6: u64 = 0x4; const P7: u64 = 0x142409b50; const P8: u64 = 0x784; const P9: u64 = 0x14240aa00; const P10: u64 = 0x140200070; const P11: u64 = 0x14219c0f2;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 15>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x14219c010, funcinfo_rva: 0x37b2228, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0x78, conts: &[] };
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = (rsp) as u64; crate::simd_ops::wu64(rax.wrapping_add(P0), (rbx) as u64); crate::simd_ops::wu64(rax.wrapping_add(P1), (rcx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = ((rsp).wrapping_sub(0x70u64)) as u64; rdi = (rdx) as u64; rbx = (rcx) as u64; fl_l = (crate::simd_ops::ru8(r8.wrapping_add(P2))) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 5; continue; } bb = 1; continue; }
            1 => { rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::wu32(rax.wrapping_add(P2), (0x21u32) as u32); r8 = ((rax.wrapping_add(P2)) as u64) as u64; rdx = ((rax.wrapping_add(P3)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14219c049); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P4)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x90u64))) as u32 as u64; rax = (((rax as u32)).wrapping_sub(1)) as u32 as u64; r9 = ((((rax as u32)) as i32 as i64) as u64) as u64; crate::simd_ops::lk_and_u32(rbx, (0xff000004u32) as u32); crate::simd_ops::lk_or_u32(rbx, (0x4u32) as u32); crate::simd_ops::wu8(rbx.wrapping_add(P5), (0x4u8) as u8); crate::simd_ops::wu32(rbx.wrapping_add(P6), (0x0u32) as u32); crate::simd_ops::wu8(rbx.wrapping_add(P1), (0x0u8) as u8); rax = ((rsp.wrapping_add(0x40u64)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (rax) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), (r9) as u64); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x30u64))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm0) as u128); r8 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rdx = ((rsp.wrapping_add(0x30u64)) as u64) as u64; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14219c09d); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P7) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = (rbx) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x88u64))) as u64; rsp = ((rsp).wrapping_add(0x70u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            4 => { crate::simd_ops::lk_and_u32(rbx, (0xff000004u32) as u32); crate::simd_ops::lk_or_u32(rbx, (0x4u32) as u32); crate::simd_ops::wu8(rbx.wrapping_add(P5), (0x4u8) as u8); crate::simd_ops::wu32(rbx.wrapping_add(P6), (0x0u32) as u32); crate::simd_ops::wu8(rbx.wrapping_add(P1), (0x0u8) as u8); rdx = ((rdi.wrapping_add(P8)) as u64) as u64; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14219c0d6); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P9) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = (rbx) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x88u64))) as u64; rsp = ((rsp).wrapping_add(0x70u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            5 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14219c0ed); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P10) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::lift_ops::trap(0x80000003, P11, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14219c4a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14219c010; const P1: u64 = 0x1401df980; const P2: u64 = 0x142189a90; const P3: u64 = 0x14240ab10; const P4: u64 = 0x141295e70; const P5: u64 = 0x141295ca0; const P6: u64 = 0x1412943e0;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 25>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x14219c4a0, funcinfo_rva: 0x37b22ec, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0xc8, conts: &[] };
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (rsi) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rcx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rbp = ((rsp.wrapping_sub(0x47u64)) as u64) as u64; rsp = ((rsp).wrapping_sub(0xb0u64)) as u64; rsi = (r8) as u64; rbx = (rdx) as u64; rdi = (rcx) as u64; r14 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rbp.wrapping_sub(0x29u64), ((r14 as u32)) as u32); rcx = ((rbp.wrapping_add(0x17u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14219c4d3); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdx = (rax) as u64; rcx = ((rbp.wrapping_sub(0x9u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14219c4e0); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; xmm0 = (crate::simd_ops::ru128(rax)) as u128; crate::simd_ops::wu128(rbp.wrapping_add(0x7u64), (xmm0) as u128); r8 = ((rbp.wrapping_add(0x7u64)) as u64) as u64; rdx = (rbx) as u64; rcx = ((rbp.wrapping_sub(0x21u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14219c4f9); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = ((rbp.wrapping_add(0x17u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14219c503); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; fl_l = (crate::simd_ops::ru8(rbp.wrapping_sub(0x11u64))) as u8 as u64; fl_r = ((r14 as u8)) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru32(rbp.wrapping_sub(0x21u64))) as u32 as u64; crate::simd_ops::wu32(rdi, ((rax as u32)) as u32); crate::simd_ops::wu8(rdi.wrapping_add(0x10u64), ((r14 as u8)) as u8); bb = 6; continue; }
            2 => { xmm0 = 0u128; crate::simd_ops::wu128(rbp.wrapping_sub(0x39u64), (xmm0) as u128); crate::simd_ops::wu64(rbp.wrapping_sub(0x39u64), (r14) as u64); crate::simd_ops::wu64(rbp.wrapping_sub(0x31u64), (r14) as u64); r9 = (crate::simd_ops::ru64(rbp.wrapping_sub(0x19u64))) as u64; rax = ((r9 as u32)) as u32 as u64; rcx = (((rax) as i64).wrapping_mul((0x70u64) as i64) as u64) as u64; r8 = (crate::simd_ops::ru64(rbp.wrapping_sub(0x21u64))) as u64; rax = (0x4924924924924925u64) as u64; let prod = (rax as i64 as i128).wrapping_mul((rcx) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; rdx = ((((rdx) as i64).wrapping_shr(5u32) as u64)) as u64; rax = (rdx) as u64; rax = ((rax).wrapping_shr(63u32)) as u64; fl_l = ((rdx).wrapping_add(rax)) as u64 as u64; fl_r = fl_l; rdx = ((fl_l as u64)) as u64; if (((fl_l as u64)) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { rcx = ((rbp.wrapping_sub(0x39u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14219c55d); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; r9 = (crate::simd_ops::ru64(rbp.wrapping_sub(0x19u64))) as u64; r8 = (crate::simd_ops::ru64(rbp.wrapping_sub(0x21u64))) as u64; bb = 4; continue; }
            4 => { rax = ((r9 as u32)) as u32 as u64; rcx = (((rax) as i64).wrapping_mul((0x70u64) as i64) as u64) as u64; rcx = ((rcx).wrapping_add(r8)) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x7fu64), (rcx) as u64); crate::simd_ops::wu64(rbp.wrapping_sub(0x9u64), (r8) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rsi) as u64); r9 = ((rbp.wrapping_sub(0x39u64)) as u64) as u64; r8 = ((rbp.wrapping_add(0x7fu64)) as u64) as u64; rdx = ((rbp.wrapping_sub(0x9u64)) as u64) as u64; rcx = ((rbp.wrapping_add(0x7u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14219c591); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; xmm0 = 0u128; crate::simd_ops::wu128(rdi, (xmm0) as u128); crate::simd_ops::wu64(rdi.wrapping_add(0x8u64), (r14) as u64); rax = (crate::simd_ops::ru64(rbp.wrapping_sub(0x39u64))) as u64; crate::simd_ops::wu64(rdi, (rax) as u64); rax = (crate::simd_ops::ru32(rbp.wrapping_sub(0x31u64))) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x8u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rbp.wrapping_sub(0x2du64))) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0xcu64), ((rax as u32)) as u32); crate::simd_ops::wu8(rdi.wrapping_add(0x10u64), (0x1u8) as u8); crate::simd_ops::wu32(rbp.wrapping_sub(0x29u64), (0x1u32) as u32); crate::simd_ops::wu64(rbp.wrapping_sub(0x39u64), (r14) as u64); crate::simd_ops::wu64(rbp.wrapping_sub(0x31u64), (r14) as u64); fl_l = (crate::simd_ops::ru8(rbp.wrapping_sub(0x11u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rcx = ((rbp.wrapping_sub(0x21u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14219c5d2); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 6; continue; }
            6 => { rax = (rdi) as u64; r11 = ((rsp.wrapping_add(0xb0u64)) as u64) as u64; rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x28u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x30u64))) as u64; rsp = (r11) as u64; r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rbp = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14219c600(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x2b0; const P1: u64 = 0x2ab; const P2: u64 = 0x2a8; const P3: u64 = 0x2ac; const P4: u64 = 0x14219c010; const P5: u64 = 0x1401df980; const P6: u64 = 0x142186fa0; const P7: u64 = 0x14240ab10;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 19>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x14219c600, funcinfo_rva: 0x37b2350, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0x98, conts: &[] };
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rbx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = ((rsp).wrapping_sub(0x90u64)) as u64; rdi = (rdx) as u64; rbx = (rcx) as u64; r10 = ((r8.wrapping_add(P0)) as u64) as u64; r9 = ((crate::simd_ops::ru8(r8.wrapping_add(P1))) as u32) as u32 as u64; fl_l = ((r9 as u32)) as u32 as u64; fl_r = (0x80u32) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 2; continue; } bb = 1; continue; }
            1 => { r9 = ((r9.wrapping_mul(8)) as u32) as u32 as u64; bb = 3; continue; }
            2 => { r9 = (((r9 as u32)).wrapping_shl(8u32)) as u32 as u64; fl_l = ((r9 as u32)) as u32 as u64; fl_r = (0x7c00u32) as u32 as u64; r9 = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru32(r8.wrapping_add(P2))) as u32 as u64; rax = ((((rax as u32)) & (0xffffffu32))) as u32 as u64; rax = (((rax as u32)).wrapping_shl(3u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((r9 as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 5; continue; } bb = 4; continue; }
            4 => { r10 = (crate::simd_ops::ru64(r10)) as u64; bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru32(r8.wrapping_add(P3))) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (r10) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), (rax) as u64); rcx = ((rsp.wrapping_add(0x60u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14219c66d); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x30u64))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm0) as u128); rdx = (rax) as u64; rcx = ((rsp.wrapping_add(0x50u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14219c687); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; xmm0 = (crate::simd_ops::ru128(rax)) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm0) as u128); r9 = ((rsp.wrapping_add(0x30u64)) as u64) as u64; r8 = ((rsp.wrapping_add(0x40u64)) as u64) as u64; rdx = (rdi) as u64; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14219c6a6); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = ((rsp.wrapping_add(0x60u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14219c6b1); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P7) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = (rbx) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0xa0u64))) as u64; rsp = ((rsp).wrapping_add(0x90u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14219ca60(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x2b0; const P1: u64 = 0x2ab; const P2: u64 = 0x2a8; const P3: u64 = 0x2ac; const P4: u64 = 0x14219c010; const P5: u64 = 0x1401df980; const P6: u64 = 0x142187560; const P7: u64 = 0x14240ab10;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 23>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x14219ca60, funcinfo_rva: 0x37b241c, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0xb8, conts: &[] };
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = ((rsp).wrapping_sub(0xb0u64)) as u64; rsi = (r9) as u64; rbx = (rdx) as u64; rdi = (rcx) as u64; r11 = ((r8.wrapping_add(P0)) as u64) as u64; r10 = ((crate::simd_ops::ru8(r8.wrapping_add(P1))) as u32) as u32 as u64; fl_l = ((r10 as u32)) as u32 as u64; fl_r = (0x80u32) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 2; continue; } bb = 1; continue; }
            1 => { r10 = ((r10.wrapping_mul(8)) as u32) as u32 as u64; bb = 3; continue; }
            2 => { r10 = (((r10 as u32)).wrapping_shl(8u32)) as u32 as u64; fl_l = ((r10 as u32)) as u32 as u64; fl_r = (0x7c00u32) as u32 as u64; r10 = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru32(r8.wrapping_add(P2))) as u32 as u64; rax = ((((rax as u32)) & (0xffffffu32))) as u32 as u64; rax = (((rax as u32)).wrapping_shl(3u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((r10 as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 5; continue; } bb = 4; continue; }
            4 => { r11 = (crate::simd_ops::ru64(r11)) as u64; bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru32(r8.wrapping_add(P3))) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (r11) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), (rax) as u64); rcx = ((rsp.wrapping_add(0x80u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14219cad8); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdx = (rax) as u64; rcx = ((rsp.wrapping_add(0x70u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14219cae6); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), (0x0u64) as u64); xmm0 = (crate::simd_ops::ru128(rsi)) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x50u64), (xmm0) as u128); xmm1 = (crate::simd_ops::ru128(rsp.wrapping_add(0x30u64))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm1) as u128); xmm0 = (crate::simd_ops::ru128(rax)) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x60u64), (xmm0) as u128); rax = ((rsp.wrapping_add(0x50u64)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (rax) as u64); rax = ((rsp.wrapping_add(0x30u64)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rax) as u64); r9 = ((rsp.wrapping_add(0x60u64)) as u64) as u64; r8 = (rbx) as u64; rdx = ((rsp.wrapping_add(0x40u64)) as u64) as u64; rcx = (rdi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14219cb3d); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = ((rsp.wrapping_add(0x80u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14219cb4b); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P7) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = (rdi) as u64; r11 = ((rsp.wrapping_add(0xb0u64)) as u64) as u64; rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x18u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x20u64))) as u64; rsp = (r11) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14219d030(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x4;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    crate::simd_ops::wu32(rcx, (0x5au32) as u32);
    rax = rcx;
    crate::simd_ops::wu8(rcx.wrapping_add(P0), (0x0u8) as u8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14219d0d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x4;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    crate::simd_ops::wu32(rcx, (0x0u32) as u32);
    rax = rcx;
    crate::simd_ops::wu8(rcx.wrapping_add(P0), (0x1u8) as u8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14219d1a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142646e60;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    rcx = 0x3;
    rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P0)) as u64, rax, rcx, rdx, r8, r9, rsp);
    rcx = 0x79;
    rdx = 0x1;
    fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64;
    rcx = (if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { (rdx as u32) } else { (rcx as u32) }) as u32 as u64;
    rax = ((rcx as u32)) as u32 as u64;
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14219d3e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x4;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    crate::simd_ops::wu32(rcx, (0x4du32) as u32);
    rax = rcx;
    crate::simd_ops::wu8(rcx.wrapping_add(P0), (0x0u8) as u8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14219d400(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x98;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    crate::simd_ops::wu32(rcx, (0x4cu32) as u32);
    rax = rcx;
    crate::simd_ops::wu8(rcx.wrapping_add(P0), (0x0u8) as u8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14219d420(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x4;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    crate::simd_ops::wu32(rcx, (0x49u32) as u32);
    rax = rcx;
    crate::simd_ops::wu8(rcx.wrapping_add(P0), (0x0u8) as u8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14219d460(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1403cc710; const P1: u64 = 0x14240ab10; const P2: u64 = 0x14240aa00; const P3: u64 = 0x1412adfe0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 41>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdx); crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rbp = rsp.wrapping_sub(0x30); rsp = ((rsp).wrapping_sub(0x130u64)) as u64; rbx = r8; rdi = rcx; rsi = 0; fl_l = 0; fl_r = 0; rcx = rsp.wrapping_add(0x30); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu8(rbp.wrapping_sub(0x80u64), ((rsi as u8)) as u8); crate::simd_ops::wu32(rbp.wrapping_sub(0x7cu64), (0x1u32) as u32); crate::simd_ops::wu64(rbp.wrapping_sub(0x78u64), rsi); crate::simd_ops::wu64(rbp.wrapping_sub(0x70u64), rsi); rcx = rbp.wrapping_sub(0x68); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rbp.wrapping_sub(0x18u64), (0x8000008u64) as u64); crate::simd_ops::wu8(rbp.wrapping_sub(0x10u64), ((rsi as u8)) as u8); rcx = rsp.wrapping_add(0x38); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru64(rbx)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); rax = rsp.wrapping_add(0x38); crate::simd_ops::wu64(rbp.wrapping_add(0x58u64), rax); rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x38u64))) as u32 as u64; rax = ((((rax as u32)) & (0xff000008u32))) as u32 as u64; rax = ((((rax as u32)) | (0x8u32))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x38u64), ((rax as u32)) as u32); crate::simd_ops::wu8(rsp.wrapping_add(0x3bu64), (0x8u8) as u8); crate::simd_ops::wu32(rsp.wrapping_add(0x3cu64), ((rsi as u32)) as u32); crate::simd_ops::wu8(rsp.wrapping_add(0x40u64), ((rsi as u8)) as u8); rdx = rbx.wrapping_add(0x8); rcx = rsp.wrapping_add(0x38); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = ((crate::simd_ops::ru8(rbx.wrapping_add(0x50u64))) as u32) as u32 as u64; crate::simd_ops::wu8(rbp.wrapping_sub(0x80u64), ((rax as u8)) as u8); crate::simd_ops::wu64(rbp.wrapping_add(0x58u64), rdi); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu64(rdi, rax); rcx = rdi.wrapping_add(0x8); crate::simd_ops::wu64(rbp.wrapping_add(0x50u64), rcx); crate::simd_ops::lk_and_u32(rcx, (0xff000008u32) as u32); crate::simd_ops::lk_or_u32(rcx, (0x8u32) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x3u64), (0x8u8) as u8); crate::simd_ops::wu32(rcx.wrapping_add(0x4u64), ((rsi as u32)) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x8u64), ((rsi as u8)) as u8); rdx = rsp.wrapping_add(0x38); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = ((crate::simd_ops::ru8(rbp.wrapping_sub(0x80u64))) as u32) as u32 as u64; crate::simd_ops::wu8(rdi.wrapping_add(0x50u64), ((rax as u8)) as u8); rax = (crate::simd_ops::ru32(rbp.wrapping_sub(0x7cu64))) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x54u64), ((rax as u32)) as u32); xmm0 = (crate::simd_ops::ru128(rbp.wrapping_sub(0x78u64))) as u128; crate::simd_ops::wu128(rdi.wrapping_add(0x58u64), (xmm0) as u128); rcx = rdi.wrapping_add(0x68); rdx = rbp.wrapping_sub(0x68); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu8(rdi.wrapping_add(0x100u64), (0x1u8) as u8); rcx = rbp.wrapping_sub(0x18); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rbp.wrapping_sub(0x60); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rsp.wrapping_add(0x38); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = rdi; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x160u64))) as u64; rsp = ((rsp).wrapping_add(0x130u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14219d5c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x98;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    crate::simd_ops::wu32(rcx, (0x57u32) as u32);
    rax = rcx;
    crate::simd_ops::wu8(rcx.wrapping_add(P0), (0x0u8) as u8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14219d600(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x50; const P1: u64 = 0x4;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = ((crate::simd_ops::ru8(r8.wrapping_add(P0))) as u32) as u32 as u64;
    crate::simd_ops::wu8(rcx, ((rax as u8)) as u8);
    rax = rcx;
    crate::simd_ops::wu8(rcx.wrapping_add(P1), (0x1u8) as u8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14219d800(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1421abc80; const P1: u64 = 0x1412add20; const P2: u64 = 0x14240ab10;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 25>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x14219d800, funcinfo_rva: 0x37b2698, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0xc8, conts: &[] };
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = ((rsp).wrapping_sub(0xc0u64)) as u64; rdi = (r8) as u64; rbx = (rdx) as u64; rsi = (rcx) as u64; rcx = ((rsp.wrapping_add(0x38u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14219d820); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (rdi) as u64); r8 = ((rsp.wrapping_add(0x28u64)) as u64) as u64; rdx = (rax) as u64; rcx = (rsi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14219d83b); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); fl_l = (crate::simd_ops::ru8(rsp.wrapping_add(0xb8u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 6; continue; } bb = 1; continue; }
            1 => { rax = (((crate::simd_ops::ru8(rsp.wrapping_add(0xb0u64))) as i8 as i64) as u64) as u64; fl_l = ((rax).wrapping_add(0x1u64)) as u64 as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; if (((fl_l as u64)) == 0) { bb = 5; continue; } bb = 2; continue; }
            2 => { fl_l = (rax) as u64 as u64; fl_r = (0x1u64) as u64 as u64; rax = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 5; continue; } bb = 3; continue; }
            3 => { fl_l = (rax) as u64 as u64; fl_r = (0x1u64) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 5; continue; } bb = 4; continue; }
            4 => { rcx = ((rsp.wrapping_add(0x88u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14219d86e); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 5; continue; }
            5 => { rcx = ((rsp.wrapping_add(0x40u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14219d878); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 6; continue; }
            6 => { rax = (rsi) as u64; r11 = ((rsp.wrapping_add(0xc0u64)) as u64) as u64; rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x10u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x18u64))) as u64; rsp = (r11) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14219dec0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x3; const P1: u64 = 0x8;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P0))) as u32) as u32 as u64;
    fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x80;
    if !((((fl_l as u32)) >= ((fl_r as u32)))) { rax = ((rax.wrapping_mul(8)) as u32) as u32 as u64; } else { rax = (((rax as u32)).wrapping_shl(8u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x7c00; rax = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; }
    rcx = (crate::simd_ops::ru32(rdx)) as u32 as u64;
    rcx = ((((rcx as u32)) & (0xffffffu32))) as u32 as u64;
    rcx = (((rcx as u32)).wrapping_shl(3u32)) as u32 as u64;
    fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64;
    rax = ((rdx.wrapping_add(P1)) as u64) as u64;
    if !((((fl_l as u32)) <= ((fl_r as u32)))) { rax = (crate::simd_ops::ru64(rdx.wrapping_add(P1))) as u64; }
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14219eda0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x142645090; const P2: u64 = 0x14340ecd8; const P3: u64 = 0x142497fd0; const P4: u64 = 0x142498740; const P5: u64 = 0x1426456f0; const P6: u64 = 0x142498af0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 59>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x14219eda0, funcinfo_rva: 0x37b298c, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0x1d8, conts: &[] };
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64); rsp = ((rsp).wrapping_sub(0x1d0u64)) as u64; rbx = (r8) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x50u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru64(rdx)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), (rax) as u64); rax = (crate::simd_ops::ru32(rcx)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x60u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), (rax) as u64); xmm0 = 0u128; crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm0) as u128); rax = ((rsp.wrapping_add(0x40u64)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (rax) as u64); crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), (0x1u32) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (0x0u64) as u64); r9 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; r8 = ((rsp.wrapping_add(0x50u64)) as u64) as u64; rdx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rcx = ((rsp.wrapping_add(0x60u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14219edfe); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P1)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x1u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = ((P2) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x80u64), (rax) as u64); rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x88u64))) as u32 as u64; rax = ((((rax as u32)) & (0xff000020u32))) as u32 as u64; rax = ((((rax as u32)) | (0x20u32))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x88u64), ((rax as u32)) as u32); crate::simd_ops::wu8(rsp.wrapping_add(0x8bu64), (0x20u8) as u8); crate::simd_ops::wu32(rsp.wrapping_add(0x8cu64), (0x0u32) as u32); crate::simd_ops::wu8(rsp.wrapping_add(0x90u64), (0x0u8) as u8); xmm0 = (crate::simd_ops::ru128(rbx)) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x70u64), (xmm0) as u128); r8 = (0x1u32) as u32 as u64; rdx = ((rsp.wrapping_add(0x70u64)) as u64) as u64; rcx = ((rsp.wrapping_add(0x80u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14219ee6a); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; r8 = (crate::simd_ops::ru32(rsp.wrapping_add(0x40u64))) as u32 as u64; rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rcx = ((rsp.wrapping_add(0x80u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14219ee82); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14219ee8c); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P5)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = ((rsp.wrapping_add(0x80u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14219ee9b); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); rsp = ((rsp).wrapping_add(0x1d0u64)) as u64; rbx = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            2 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; rsp = ((rsp).wrapping_add(0x1d0u64)) as u64; rbx = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14219f0d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x18; const P1: u64 = 0x10; const P2: u64 = 0x14219ad00; const P3: u64 = 0x140200070;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(P0))) as u8 as u64; fl_r = (0x1u8) as u8 as u64; rbx = rcx; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 1; continue; } rdx = (crate::simd_ops::ru64(rdx.wrapping_add(P1))) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rax = rbx; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            1 => { rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x14219f0f6, rcx, rsp); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14219f100(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x78; const P1: u64 = 0x50; const P2: u64 = 0x140200070;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x28u64)) as u64; fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(P0))) as u8 as u64; fl_r = (0x1u8) as u8 as u64; rax = ((rcx.wrapping_add(P1)) as u64) as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 1; continue; } rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
            1 => { rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x14219f118, rcx, rsp); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14219f7d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0xc; const P1: u64 = 0x8;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = (crate::simd_ops::ru64(rdx)) as u64;
    crate::simd_ops::wu64(rcx, rax);
    rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P0))) as u32) as u32 as u64;
    crate::simd_ops::wu8(rcx.wrapping_add(P0), ((rax as u8)) as u8);
    fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64;
    if !(((((fl_l as u8)) & ((fl_r as u8))) != 0)) { rax = (crate::simd_ops::ru32(rdx.wrapping_add(P1))) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(P1), ((rax as u32)) as u32); }
    rax = rcx;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1421a16e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x4; const P1: u64 = 0x8; const P2: u64 = 0x38; const P3: u64 = 0xd0; const P4: u64 = 0xd1; const P5: u64 = 0x18; const P6: u64 = 0x21; const P7: u64 = 0x14128fd40;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        core::sync::atomic::fence(core::sync::atomic::Ordering::SeqCst);
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = ((crate::simd_ops::ru8(rdx.wrapping_add(P0))) as u32) as u32 as u64; rbx = rcx; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; crate::simd_ops::wu8(rsp.wrapping_add(0x34u64), ((rdi as u8)) as u8); fl_l = ((rdi as u8)) as u8 as u64; fl_r = ((rdi as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 1; continue; } rax = (crate::simd_ops::ru32(rdx)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = 0x1; let tmp_xchg: u8 = crate::simd_ops::atomic_xchg_u8(rax.wrapping_add(P3), ((rcx as u8)) as u8); rcx = (rcx & !0xffu64) | (((tmp_xchg) as u8 as u64)); rcx = 0; fl_l = 0; fl_r = 0; let tmp_xchg: u8 = crate::simd_ops::atomic_xchg_u8(rax.wrapping_add(P4), ((rcx as u8)) as u8); rcx = (rcx & !0xffu64) | (((tmp_xchg) as u8 as u64)); rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x30u64))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x38u64), ((rax as u32)) as u32); bb = 2; continue; }
            1 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = 0; fl_l = 0; fl_r = 0; let tmp_xchg: u8 = crate::simd_ops::atomic_xchg_u8(rax.wrapping_add(P4), ((rcx as u8)) as u8); rcx = (rcx & !0xffu64) | (((tmp_xchg) as u8 as u64)); bb = 2; continue; }
            2 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P5))) as u64; rdx = rsp.wrapping_add(0x38); crate::simd_ops::wu8(rsp.wrapping_add(0x3cu64), ((rdi as u8)) as u8); crate::simd_ops::wu8(rbx.wrapping_add(P6), (0x1u8) as u8); rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1421a1800(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x4; const P1: u64 = 0x19; const P2: u64 = 0x10; const P3: u64 = 0x14128fd40;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    r8 = ((crate::simd_ops::ru8(rdx.wrapping_add(P0))) as u32) as u32 as u64;
    crate::simd_ops::wu8(rsp.wrapping_add(0x34u64), ((r8 as u8)) as u8);
    fl_l = ((r8 as u8)) as u8 as u64; fl_r = ((r8 as u8)) as u8 as u64;
    if !(((((fl_l as u8)) & ((fl_r as u8))) != 0)) { rax = (crate::simd_ops::ru32(rdx)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((rax as u32)) as u32); }
    crate::simd_ops::wu8(rcx.wrapping_add(P1), (0x1u8) as u8);
    rdx = rsp.wrapping_add(0x30);
    rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P2))) as u64;
    rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1421a18e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14; const P1: u64 = 0x4; const P2: u64 = 0x8; const P3: u64 = 0x38; const P4: u64 = 0xd0; const P5: u64 = 0x10; const P6: u64 = 0x20; const P7: u64 = 0x29; const P8: u64 = 0x14128fd40;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        core::sync::atomic::fence(core::sync::atomic::Ordering::SeqCst);
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(P0))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; rbx = rcx; rdi = ((crate::simd_ops::ru8(rdx.wrapping_add(P1))) as u32) as u32 as u64; rax = (((rdi as u8)) as u32) as u32 as u64; crate::simd_ops::wu8(rsp.wrapping_add(0x34u64), ((rax as u8)) as u8); if (((fl_l as u8)) == ((fl_r as u8))) { bb = 1; continue; } rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P2))) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(rax.wrapping_add(P3))) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = 0x1; let tmp_xchg: u8 = crate::simd_ops::atomic_xchg_u8(rax.wrapping_add(P4), ((rcx as u8)) as u8); rcx = (rcx & !0xffu64) | (((tmp_xchg) as u8 as u64)); rax = ((crate::simd_ops::ru8(rsp.wrapping_add(0x34u64))) as u32) as u32 as u64; bb = 1; continue; }
            1 => { fl_l = ((rdi as u8)) as u8 as u64; fl_r = ((rdi as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 2; continue; } rax = ((crate::simd_ops::ru8(rbx.wrapping_add(P0))) as u32) as u32 as u64; crate::simd_ops::wu8(rsp.wrapping_add(0x34u64), ((rax as u8)) as u8); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 3; continue; } rax = (crate::simd_ops::ru32(rbx.wrapping_add(P5))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((rax as u32)) as u32); bb = 3; continue; }
            2 => { crate::simd_ops::wu8(rsp.wrapping_add(0x34u64), ((rax as u8)) as u8); bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P6))) as u64; rdx = rsp.wrapping_add(0x30); crate::simd_ops::wu8(rbx.wrapping_add(P7), (0x1u8) as u8); rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1421a68b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x40; const P1: u64 = 0x18; const P2: u64 = 0x1412b5cb0; const P3: u64 = 0xc; const P4: u64 = 0x10; const P5: u64 = 0x8; const P6: u64 = 0x142442de0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        core::sync::atomic::fence(core::sync::atomic::Ordering::SeqCst);
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rbx = rdx; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64, rax, rcx, rdx, r8, r9, rsp); rdx = ((rax.wrapping_add(P1)) as u64) as u64; r8 = rbx; rcx = rsp.wrapping_add(0x20); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } rax = 0xffffffff; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rbx.wrapping_add(P3), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 2; continue; } fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } rdi = (crate::simd_ops::ru64(rbx.wrapping_add(P4))) as u64; fl_l = (((rdi) & (0xfffffffffffffffcu64))) as u64; fl_r = fl_l; rdi = ((fl_l as u64)) as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 1; continue; } rax = (crate::simd_ops::ru64(rdi)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = rdi; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(rax.wrapping_add(P5))) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = rdi; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); bb = 1; continue; }
            1 => { rcx = rbx; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); bb = 2; continue; }
            2 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1421a6930(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x40; const P1: u64 = 0x260; const P2: u64 = 0x1426455a8; const P3: u64 = 0x268; const P4: u64 = 0x270; const P5: u64 = 0x28; const P6: u64 = 0x50; const P7: u64 = 0x1401e6ff0; const P8: u64 = 0xffffffffffffffb0; const P9: u64 = 0x142645598;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; r14 = rdx; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64, rax, rcx, rdx, r8, r9, rsp); rbx = rax; rdi = ((rax.wrapping_add(P1)) as u64) as u64; rcx = rdi; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P2)) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P3))) as u64; rbx = (crate::simd_ops::ru32(rbx.wrapping_add(P4))) as u32 as u64; rbx = ((rbx).wrapping_shl(7u32)) as u64; rbx = ((rbx).wrapping_add(rcx)) as u64; fl_l = rcx; fl_r = rbx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 3; continue; } rbp = (((crate::simd_ops::ru8(r14.wrapping_add(P5))) as i8 as i64) as u64) as u64; rsi = ((rcx.wrapping_add(P6)) as u64) as u64; bb = 1; continue; }
            1 => { rax = (((crate::simd_ops::ru8(rsi.wrapping_add(P5))) as i8 as i64) as u64) as u64; fl_l = rbp; fl_r = rax; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } fl_l = ((rax).wrapping_add(0x1u64)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; if (((fl_l as u64)) == 0) { bb = 5; continue; } fl_l = rax; fl_r = (0x1u64) as u64; rax = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 5; continue; } fl_l = rax; fl_r = (0x1u64) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 5; continue; } rdx = rsi; rcx = r14; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 5; continue; } bb = 2; continue; }
            2 => { rsi = ((rsi).wrapping_sub(0xffffffffffffff80u64)) as u64; rax = ((rsi.wrapping_add(P8)) as u64) as u64; fl_l = rax; fl_r = rbx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } bb = 3; continue; }
            3 => { rbx = (rbx & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 4; continue; }
            4 => { rcx = rdi; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P9)) as u64, rax, rcx, rdx, r8, r9, rsp); rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rax = (((rbx as u8)) as u32) as u32 as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            5 => { rbx = (rbx & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 4; continue; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1421a6a00(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x40; const P1: u64 = 0x260; const P2: u64 = 0x1426455a8; const P3: u64 = 0x268; const P4: u64 = 0x270; const P5: u64 = 0x142645598;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rsi = rdx; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64, rax, rcx, rdx, r8, r9, rsp); rdi = rax; rbx = ((rax.wrapping_add(P1)) as u64) as u64; rcx = rbx; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P2)) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rdi.wrapping_add(P3))) as u64; rax = (crate::simd_ops::ru32(rdi.wrapping_add(P4))) as u32 as u64; rax = ((rax).wrapping_shl(7u32)) as u64; rax = ((rax).wrapping_add(rcx)) as u64; fl_l = rcx; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 2; continue; } rdx = (crate::simd_ops::ru64(rsi)) as u64; bb = 1; continue; }
            1 => { fl_l = rdx; fl_r = (crate::simd_ops::ru64(rcx)) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 4; continue; } rcx = ((rcx).wrapping_sub(0xffffffffffffff80u64)) as u64; fl_l = rcx; fl_r = rax; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } bb = 2; continue; }
            2 => { rdi = (rdi & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 3; continue; }
            3 => { rcx = rbx; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P5)) as u64, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = (((rdi as u8)) as u32) as u32 as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            4 => { rdi = (rdi & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 3; continue; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1421abac0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x40; const P1: u64 = 0x10; const P2: u64 = 0x1426455a8; const P3: u64 = 0x8; const P4: u64 = 0x141298060; const P5: u64 = 0xa78; const P6: u64 = 0x1421a6460; const P7: u64 = 0x14219d8a0; const P8: u64 = 0x4; const P9: u64 = 0x19; const P10: u64 = 0x14128fd40; const P11: u64 = 0x142645598;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x1421abac0, funcinfo_rva: 0x37b4d98, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0x38, conts: &[] };
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = (rdx) as u64; rsi = (rcx) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1421abad1); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rbx = ((rax.wrapping_add(P1)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), (rbx) as u64); rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1421abae0); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P2)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); r9 = (crate::simd_ops::ru64(rdi.wrapping_add(P3))) as u64; fl_l = (r9) as u64 as u64; fl_r = (r9) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rcx = ((rdi.wrapping_add(P1)) as u64) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x40u64), (0x5u32) as u32); rdx = ((rsp.wrapping_add(0x40u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1421abb01); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 8; continue; }
            2 => { rdx = (rsi) as u64; fl_l = (crate::simd_ops::ru8(rsi.wrapping_add(P5))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 4; continue; } bb = 3; continue; }
            3 => { r8 = (rdi) as u64; rcx = ((rsp.wrapping_add(0x50u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1421abb1c); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 5; continue; }
            4 => { rcx = ((rsp.wrapping_add(0x58u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1421abb28); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P7) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 5; continue; }
            5 => { rcx = (rax) as u64; rax = ((crate::simd_ops::ru8(rax.wrapping_add(P8))) as u32) as u32 as u64; crate::simd_ops::wu8(rsp.wrapping_add(0x44u64), ((rax as u8)) as u8); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 7; continue; } bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru32(rcx)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x40u64), ((rax as u32)) as u32); bb = 7; continue; }
            7 => { crate::simd_ops::wu8(rdi.wrapping_add(P9), (0x1u8) as u8); rdx = ((rsp.wrapping_add(0x40u64)) as u64) as u64; rcx = (crate::simd_ops::ru64(rdi.wrapping_add(P1))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1421abb4f); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P10) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 8; continue; }
            8 => { rcx = (rbx) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rsi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rbx = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); drop(_eh); let s5 = crate::simd_ops::read_s5_tail(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x40) as *const u64); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P11)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1421abc40(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x4;
    let (mut rax, mut fl_l, mut fl_r) = (0u64, 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = (crate::simd_ops::ru32(rcx)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 1; continue; } fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = ((crate::simd_ops::ru8(rcx.wrapping_add(P0))) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 4; continue; } fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 3; continue; } fl_l = ((r8 as u8)) as u8 as u64; fl_r = ((r8 as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; return rax; }
            3 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x2u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 4; continue; } fl_l = ((r8 as u8)) as u8 as u64; fl_r = ((r8 as u8)) as u8 as u64; rax = (rax & !0xffu64) | (((if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { 1u8 } else { 0u8 }) as u8 as u64)); return rax; }
            4 => { rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1421abc80(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x40; const P1: u64 = 0x260; const P2: u64 = 0x1426455a8; const P3: u64 = 0x268; const P4: u64 = 0x270; const P5: u64 = 0x80; const P6: u64 = 0x142645598; const P7: u64 = 0x1412b4e30;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; r14 = r8; rsi = rcx; rax = (crate::simd_ops::ru64(rdx)) as u64; rcx = rdx; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64, rax, rcx, rdx, r8, r9, rsp); rdi = rax; rbx = ((rax.wrapping_add(P1)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rbx); rcx = rbx; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P2)) as u64, rax, rcx, rdx, r8, r9, rsp); rdx = (crate::simd_ops::ru64(rdi.wrapping_add(P3))) as u64; rcx = (crate::simd_ops::ru32(rdi.wrapping_add(P4))) as u32 as u64; rcx = ((rcx).wrapping_shl(7u32)) as u64; rcx = ((rcx).wrapping_add(rdx)) as u64; fl_l = rdx; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 2; continue; } rax = (crate::simd_ops::ru64(r14)) as u64; bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru64(rdx)) as u64; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 4; continue; } rdx = ((rdx).wrapping_sub(0xffffffffffffff80u64)) as u64; fl_l = rdx; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::wu32(rsi, (0x54u32) as u32); crate::simd_ops::wu8(rsi.wrapping_add(P5), (0x0u8) as u8); bb = 3; continue; }
            3 => { rcx = rbx; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P6)) as u64, rax, rcx, rdx, r8, r9, rsp); rax = rsi; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            4 => { rcx = rsi; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu8(rsi.wrapping_add(P5), (0x1u8) as u8); bb = 3; continue; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1421abdb0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141299b50; const P1: u64 = 0x14243b5f0; const P2: u64 = 0x14183acc2; const P3: u64 = 0x1433e7798; const P4: u64 = 0x1426469b8; const P5: u64 = 0x1433ea2e8;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 41>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = rsp; crate::simd_ops::wu64(rax.wrapping_add(0x18u64), rbx); crate::simd_ops::wu64(rax.wrapping_add(0x20u64), rsi); crate::simd_ops::wu64(rax.wrapping_add(0x8u64), rcx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x140u64)) as u64; rdi = rcx; rsi = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rcx, rsi); crate::simd_ops::wu32(rcx.wrapping_add(0x8u64), ((rsi as u32)) as u32); crate::simd_ops::wu32(rcx.wrapping_add(0xcu64), ((rsi as u32)) as u32); crate::simd_ops::wu64(rcx.wrapping_add(0x10u64), rsi); rbx = rcx.wrapping_add(0x18); crate::simd_ops::wu64(rax.wrapping_add(0x10u64), rbx); crate::simd_ops::wu32(rbx, ((rsi as u32)) as u32); rcx = rbx.wrapping_add(0x8); rdx = ((rsi.wrapping_add(0x20u64)) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rbx.wrapping_add(0xa8); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (0x8u64) as u64); crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((rsi as u32)) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (0x60u64) as u64); r9 = ((rsi.wrapping_add(0x20u64)) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rbx.wrapping_add(0xe8u64), rsi); crate::simd_ops::wu64(rbx.wrapping_add(0xf0u64), rsi); crate::simd_ops::wu32(rbx.wrapping_add(0xf8u64), ((rsi as u32)) as u32); crate::simd_ops::wu8(rbx.wrapping_add(0xfcu64), ((rsi as u8)) as u8); crate::simd_ops::wu64(rbx.wrapping_add(0x100u64), rsi); crate::simd_ops::wu64(rbx.wrapping_add(0x108u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rsi); crate::simd_ops::wu32(rsp.wrapping_add(0x48u64), (0xffffffffu32) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x4cu64), (0x4u32) as u32); crate::simd_ops::wu8(rsp.wrapping_add(0x50u64), ((rsi as u8)) as u8); rdx = 0; fl_l = 0; fl_r = 0; r8 = 0xef; rcx = rsp.wrapping_add(0x51); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r8 = P3; rdx = 0xef; rcx = rsp.wrapping_add(0x51); (rax, xmm0) = crate::simd_ops::call_stk_xmm((crate::simd_ops::ru64(P4)) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rbx.wrapping_add(0x118); rax = rsp.wrapping_add(0x40); rdx = ((rsi.wrapping_add(0x2u64)) as u32) as u32 as u64; bb = 1; continue; }
            1 => { let y_addr: u64 = rax; xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rcx; crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); let y_addr: u64 = rax.wrapping_add(0x20u64); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rcx.wrapping_add(0x20u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); let y_addr: u64 = rax.wrapping_add(0x40u64); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rcx.wrapping_add(0x40u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); xmm0 = (crate::simd_ops::ru128(rax.wrapping_add(0x60u64))) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rcx.wrapping_add(0x60u64), (xmm0) as u128); rcx = rcx.wrapping_add(0x80); xmm1 = (crate::simd_ops::ru128(rax.wrapping_add(0x70u64))) as u128; crate::simd_ops::wu128(rcx.wrapping_sub(0x10u64), (xmm1) as u128); rax = rax.wrapping_add(0x80); fl_l = rdx; fl_r = (0x1u64) as u64; rdx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } crate::simd_ops::wu64(rbx.wrapping_add(0x218u64), rsi); crate::simd_ops::wu64(rbx.wrapping_add(0x220u64), rsi); crate::simd_ops::wu8(rbx.wrapping_add(0x228u64), ((rdx as u8)) as u8); rax = P5; crate::simd_ops::wu64(rbx.wrapping_add(0x110u64), rax); crate::simd_ops::wu64(rbx.wrapping_add(0x230u64), rsi); crate::simd_ops::wu64(rbx.wrapping_add(0x238u64), rsi); crate::simd_ops::wu8(rbx.wrapping_add(0x240u64), (0x1u8) as u8); crate::simd_ops::wu64(rdi.wrapping_add(0x260u64), rsi); crate::simd_ops::wu64(rdi.wrapping_add(0x268u64), rsi); crate::simd_ops::wu64(rdi.wrapping_add(0x270u64), rsi); crate::simd_ops::wu64(rdi.wrapping_add(0x278u64), rsi); crate::simd_ops::wu64(rdi.wrapping_add(0x280u64), rsi); rax = rdi; r11 = rsp.wrapping_add(0x140); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x20u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x28u64))) as u64; rsp = r11; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1421ac930(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1412b6990; const P1: u64 = 0x14028da20; const P2: u64 = 0x1401fa0a0; const P3: u64 = 0x142442de0; const P4: u64 = 0x14129bf30; const P5: u64 = 0x142645748; const P6: u64 = 0x143406c90; const P7: u64 = 0x1423f0780; const P8: u64 = 0x14136eec0; const P9: u64 = 0x1421aca2f;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x1421ac930, funcinfo_rva: 0x37b4e90, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0x48, conts: &[] };
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (rbp) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rsi = (rcx) as u64; rbp = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rsp.wrapping_add(0x50u64), ((rbp as u32)) as u32); rax = (crate::simd_ops::ru64(rcx)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1421ac950); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x40u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rdx = ((rax.wrapping_add(0x18u64)) as u64) as u64; r8 = ((rsp.wrapping_add(0x50u64)) as u64) as u64; rcx = ((rsp.wrapping_add(0x20u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1421ac961); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1421ac96c); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rcx = ((rsp.wrapping_add(0x20u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1421ac976); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; fl_l = (rbx) as u64 as u64; fl_r = (rbx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 1; continue; }
            1 => { rax = (0xffffffffu32) as u32 as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rbx.wrapping_add(0xcu64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x1u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 6; continue; } bb = 2; continue; }
            2 => { fl_l = (rbx) as u64 as u64; fl_r = (rbx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 3; continue; }
            3 => { rdi = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; fl_l = (((rdi) & (0xfffffffffffffffcu64))) as u64 as u64; fl_r = fl_l; rdi = ((fl_l as u64)) as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(rdi)) as u64; rdx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rcx = (rdi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1421ac9ac); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rcx = (rdi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1421ac9b2); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 5; continue; }
            5 => { rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1421ac9bb); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru64(rsi)) as u64; rcx = (rsi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1421ac9c7); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x40u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rbx = (rax) as u64; let tmp_xchg: u8 = crate::simd_ops::atomic_xchg_u8(rax.wrapping_add(0x258u64), ((rbp as u8)) as u8); rbp = (rbp & !0xffu64) | (((tmp_xchg) as u8 as u64)); rcx = ((rax.wrapping_add(0x128u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1421ac9db); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x230u64))) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 7; continue; }
            7 => { rdx = (0xffffffffu32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1421ac9f1); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P5)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 9; continue; } bb = 8; continue; }
            8 => { rax = (0x1u32) as u32 as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            9 => { rdx = ((P6) as u64) as u64; rcx = ((rsp.wrapping_add(0x30u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1421aca21); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P7) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rcx = (rax) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1421aca2a); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P8) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); crate::lift_ops::trap(0x80000003, P9, rcx, rsp); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1421acb30(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143a8a520; const P1: u64 = 0x1401e6ff0; const P2: u64 = 0x14240aa00; const P3: u64 = 0x1421a6a80; const P4: u64 = 0x1403cc710; const P5: u64 = 0x8; const P6: u64 = 0x3; const P7: u64 = 0x4; const P8: u64 = 0x50; const P9: u64 = 0x14240ab10;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 17>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x1421acb30, funcinfo_rva: 0x37b4f58, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0x88, conts: &[] };
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (r9) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rsi) as u64); rsp = ((rsp).wrapping_sub(0x80u64)) as u64; rsi = (rdx) as u64; rbx = (rcx) as u64; rdx = ((P0) as u64) as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0xb8u64))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1421acb57); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 5; continue; } bb = 1; continue; }
            1 => { rax = ((rsp.wrapping_add(0x28u64)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xa8u64), (rax) as u64); rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x28u64))) as u32 as u64; rax = ((((rax as u32)) & (0xff000004u32))) as u32 as u64; rax = ((((rax as u32)) | (0x4u32))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((rax as u32)) as u32); crate::simd_ops::wu8(rsp.wrapping_add(0x2bu64), (0x4u8) as u8); crate::simd_ops::wu32(rsp.wrapping_add(0x2cu64), (0x0u32) as u32); crate::simd_ops::wu8(rsp.wrapping_add(0x30u64), (0x0u8) as u8); rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0xb8u64))) as u64; rcx = ((rsp.wrapping_add(0x28u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1421acb9c); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); crate::simd_ops::wu8(rsp.wrapping_add(0x50u64), (0x1u8) as u8); r9 = (crate::simd_ops::ru64(rsp.wrapping_add(0xc0u64))) as u64; r8 = ((rsp.wrapping_add(0x28u64)) as u64) as u64; rdx = (rsi) as u64; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1421acbba); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rax = (((crate::simd_ops::ru8(rsp.wrapping_add(0x50u64))) as i8 as i64) as u64) as u64; fl_l = ((rax).wrapping_add(0x1u64)) as u64 as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; if (((fl_l as u64)) == 0) { bb = 7; continue; } bb = 2; continue; }
            2 => { fl_l = (rax) as u64 as u64; fl_r = (0x1u64) as u64 as u64; rax = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 7; continue; } bb = 3; continue; }
            3 => { fl_l = (rax) as u64 as u64; fl_r = (0x1u64) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 7; continue; } bb = 4; continue; }
            4 => { rcx = ((rsp.wrapping_add(0x28u64)) as u64) as u64; bb = 6; continue; }
            5 => { rcx = ((rsp.wrapping_add(0x28u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1421acbe4); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rcx = (crate::simd_ops::ru64(rax)) as u64; crate::simd_ops::wu64(rbx, (rcx) as u64); rcx = ((rbx.wrapping_add(P5)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xa8u64), (rcx) as u64); crate::simd_ops::lk_and_u32(rcx, (0xff000008u32) as u32); crate::simd_ops::lk_or_u32(rcx, (0x8u32) as u32); crate::simd_ops::wu8(rcx.wrapping_add(P6), (0x8u8) as u8); crate::simd_ops::wu32(rcx.wrapping_add(P7), (0x0u32) as u32); crate::simd_ops::wu8(rcx.wrapping_add(P5), (0x0u8) as u8); rdx = ((rax.wrapping_add(P5)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1421acc18); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); crate::simd_ops::wu8(rbx.wrapping_add(P8), (0x1u8) as u8); rcx = ((rsp.wrapping_add(0x30u64)) as u64) as u64; bb = 6; continue; }
            6 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1421acc27); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P9) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 7; continue; }
            7 => { rax = (rbx) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x90u64))) as u64; rsp = ((rsp).wrapping_add(0x80u64)) as u64; rsi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1421ad6b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rdx = (crate::simd_ops::ru64(rcx)) as u64;
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64;
    rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64;
    rax = (crate::simd_ops::ru64(rdx)) as u64;
    rax = ((rax.wrapping_add(rcx.wrapping_mul(4))) as u64) as u64;
    fl_l = r8; fl_r = r8;
    if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { return rax; }
    rcx = 0;
    fl_l = 0; fl_r = 0;
    loop { crate::simd_ops::wu64(rax, rcx); crate::simd_ops::wu32(rax.wrapping_add(P0), ((rcx as u32)) as u32); rax = ((rax).wrapping_add(0xcu64)) as u64; fl_l = r8; fl_r = (0x1u64) as u64; r8 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if !((((fl_l as u64)) != ((fl_r as u64)))) { break; } }
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1421adef0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x8; const P2: u64 = 0xc;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = ((rcx.wrapping_add(P0)) as u64) as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(P1), (0x0u32) as u32);
    crate::simd_ops::wu64(rcx, rax);
    rax = rcx;
    crate::simd_ops::wu32(rcx.wrapping_add(P2), (0x80u32) as u32);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1421ae350(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x8; const P2: u64 = 0x4; const P3: u64 = 0x3; const P4: u64 = 0x1421835d0; const P5: u64 = 0xffffffffffffffdd; const P6: u64 = 0x1; const P7: u64 = 0x20; const P8: u64 = 0x29; const P9: u64 = 0x14128fd40;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rdx; rdi = rcx; rdx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(P2))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 1; continue; } r9 = 0x1; r8 = ((r9.wrapping_add(P3)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu8(rsp.wrapping_add(0x34u64), (0x1u8) as u8); bb = 4; continue; }
            1 => { r9 = (crate::simd_ops::ru32(rbx)) as u32 as u64; rax = ((r9.wrapping_add(P5)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 2; continue; } fl_l = ((r9 as u32)) as u32 as u64; fl_r = 0x42; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 2; continue; } r8 = 0x3; bb = 3; continue; }
            2 => { r9 = 0x1; r8 = ((r9.wrapping_add(P6)) as u32) as u32 as u64; bb = 3; continue; }
            3 => { rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rax = ((crate::simd_ops::ru8(rbx.wrapping_add(P2))) as u32) as u32 as u64; crate::simd_ops::wu8(rsp.wrapping_add(0x34u64), ((rax as u8)) as u8); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 4; continue; } rax = (crate::simd_ops::ru32(rbx)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((rax as u32)) as u32); bb = 4; continue; }
            4 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(P7))) as u64; rdx = rsp.wrapping_add(0x30); crate::simd_ops::wu8(rdi.wrapping_add(P8), (0x1u8) as u8); rax = crate::simd_ops::call_stk(P9, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142206050(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x1426455a8; const P2: u64 = 0x142207930; const P3: u64 = 0x142645598; const P4: u64 = 0x1433efa98; const P5: u64 = 0x1412debd0; const P6: u64 = 0x1412decf0; const P7: u64 = 0x1422075a0;
    let (mut rax, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rbp = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; r14 = rcx; rcx = rbp.wrapping_add(0x2d8); rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P1)) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = rbp.wrapping_add(0x2a8); rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P1)) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = rbp; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rsi = (crate::simd_ops::ru32(rbp.wrapping_add(0x2d0u64))) as u32 as u64; rcx = rbp.wrapping_add(0x2a8); rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P3)) as u64, rax, rcx, rdx, r8, r9, rsp); rdx = ((rsi as u32)) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x2e8u64), (0x0u32) as u32); rcx = rbp.wrapping_add(0x2e0); r8 = P4; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x2f0u64))) as u64; rdx = rsp.wrapping_add(0x20); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbp); crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((rsi as u32)) as u32); rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); rcx = rbp.wrapping_add(0x2d8); rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P3)) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(r14.wrapping_add(P0))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(P7, rax, rcx, rdx, r8, r9, rsp); }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142250ba0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x4; const P1: u64 = 0x8;
    let (mut rax, mut fl_l, mut fl_r) = (0u64, 0u64, 0u64);
    rax = (crate::simd_ops::ru32(rdx)) as u32 as u64;
    fl_l = (crate::simd_ops::ru32(rcx)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64;
    if ((((fl_l as u32)) as i32) > (((fl_r as u32)) as i32)) { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; return rax; }
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64;
    fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64;
    if ((((fl_l as u32)) as i32) > (((fl_r as u32)) as i32)) { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; return rax; }
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P1))) as u32 as u64;
    fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(P1))) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64;
    if !(((((fl_l as u32)) as i32) > (((fl_r as u32)) as i32))) { rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); return rax; }
    rax = (rax & !0xffu64) | (((0u8) as u8 as u64));
    fl_l = 0; fl_r = 0;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142250bd0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0xa8; const P1: u64 = 0x120;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = ((rcx.wrapping_add(P0)) as u64) as u64;
    rcx = ((rax.wrapping_add(P1)) as u64) as u64;
    fl_l = rax; fl_r = rcx;
    if (((fl_l as u64)) == ((fl_r as u64))) { return rax; }
    loop { crate::simd_ops::wu8(rax, (0x0u8) as u8); rax = ((rax).wrapping_add(0x48u64)) as u64; fl_l = rax; fl_r = rcx; if !((((fl_l as u64)) != ((fl_r as u64)))) { break; } }
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142250c00(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0xa8;
    let (mut rax, mut fl_l, mut fl_r) = (0u64, 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = ((rcx.wrapping_add(P0)) as u64) as u64; rcx = ((rcx).wrapping_add(0x1c8u64)) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru8(rax)) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 3; continue; } rax = ((rax).wrapping_add(0x48u64)) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } bb = 2; continue; }
            2 => { rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); return rax; }
            3 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142250de0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = (crate::simd_ops::ru64(r8)) as u64;
    crate::simd_ops::wu64(rcx.wrapping_add(P0), rax);
    rax = rcx;
    crate::simd_ops::wu64(rcx, rdx);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142252cf0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142256910; const P1: u64 = 0x1411db210; const P2: u64 = 0x14209d670; const P3: u64 = 0x1411e7fc0; const P4: u64 = 0x142442de0; const P5: u64 = 0x142075af0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x142252cf0, funcinfo_rva: 0x37c18b0, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0x58, conts: &[] };
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rbx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = ((rsp).wrapping_sub(0x50u64)) as u64; rdi = (rcx) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x44u64), (0x5u32) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x40u64), (0x4u32) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x48u64), (0x8u32) as u32); rcx = ((rcx).wrapping_add(0x1c8u64)) as u64; rdx = ((rsp.wrapping_add(0x40u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142252d21); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu32(rdi.wrapping_add(0xd0u64), (0x4u32) as u32); crate::simd_ops::wu32(rdi.wrapping_add(0xd4u64), (0x5u32) as u32); crate::simd_ops::wu32(rdi.wrapping_add(0xd8u64), (0x8u32) as u32); rax = (crate::simd_ops::ru32(rdi.wrapping_add(0xd4u64))) as u32 as u64; rax = ((rax.wrapping_mul(2).wrapping_add(0x1u64)) as u32) as u32 as u64; rax = ((((rax as u32)) as i32).wrapping_mul((crate::simd_ops::ru32(rdi.wrapping_add(0xd8u64))) as i32) as u32) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0xacu64), ((rax as u32)) as u32); crate::simd_ops::wu32(rdi.wrapping_add(0xb0u64), ((rax as u32)) as u32); crate::simd_ops::wu32(rdi.wrapping_add(0xb4u64), ((rax as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x40u64), (0x8u32) as u32); xmm0 = ((crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64) as u128; crate::simd_ops::wu64(rdi.wrapping_add(0x118u64), ((xmm0 as u64)) as u64); crate::simd_ops::wu32(rdi.wrapping_add(0x120u64), (0x8u32) as u32); rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x11cu64))) as u32 as u64; rax = ((rax.wrapping_mul(2).wrapping_add(0x1u64)) as u32) as u32 as u64; rax = ((((rax as u32)) as i32).wrapping_mul((crate::simd_ops::ru32(rdi.wrapping_add(0x120u64))) as i32) as u32) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0xf4u64), ((rax as u32)) as u32); crate::simd_ops::wu32(rdi.wrapping_add(0xf8u64), ((rax as u32)) as u32); crate::simd_ops::wu32(rdi.wrapping_add(0xfcu64), ((rax as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x40u64), (0x10u32) as u32); xmm0 = ((crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64) as u128; crate::simd_ops::wu64(rdi.wrapping_add(0x160u64), ((xmm0 as u64)) as u64); crate::simd_ops::wu32(rdi.wrapping_add(0x168u64), (0x8u32) as u32); rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x164u64))) as u32 as u64; rax = ((rax.wrapping_mul(2).wrapping_add(0x1u64)) as u32) as u32 as u64; rax = ((((rax as u32)) as i32).wrapping_mul((crate::simd_ops::ru32(rdi.wrapping_add(0x168u64))) as i32) as u32) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x13cu64), ((rax as u32)) as u32); crate::simd_ops::wu32(rdi.wrapping_add(0x140u64), ((rax as u32)) as u32); crate::simd_ops::wu32(rdi.wrapping_add(0x144u64), ((rax as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x40u64), (0x20u32) as u32); xmm0 = ((crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64) as u128; crate::simd_ops::wu64(rdi.wrapping_add(0x1a8u64), ((xmm0 as u64)) as u64); crate::simd_ops::wu32(rdi.wrapping_add(0x1b0u64), (0x8u32) as u32); rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x1acu64))) as u32 as u64; rax = ((rax.wrapping_mul(2).wrapping_add(0x1u64)) as u32) as u32 as u64; rax = ((((rax as u32)) as i32).wrapping_mul((crate::simd_ops::ru32(rdi.wrapping_add(0x1b0u64))) as i32) as u32) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x184u64), ((rax as u32)) as u32); crate::simd_ops::wu32(rdi.wrapping_add(0x188u64), ((rax as u32)) as u32); crate::simd_ops::wu32(rdi.wrapping_add(0x18cu64), ((rax as u32)) as u32); crate::simd_ops::wu32(rdi.wrapping_add(0x298u64), (0x0u32) as u32); rcx = ((rsp.wrapping_add(0x60u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142252e4b); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdx = (crate::simd_ops::ru64(rax)) as u64; crate::simd_ops::wu64(rax, (0x0u64) as u64); rbx = (crate::simd_ops::ru64(rdi.wrapping_add(0x2a0u64))) as u64; crate::simd_ops::wu64(rdi.wrapping_add(0x2a0u64), (rdx) as u64); fl_l = (rbx) as u64 as u64; fl_r = (rbx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142252e70); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = ((rbx.wrapping_add(0x58u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142252e79); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142252e82); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 2; continue; }
            2 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; fl_l = (rbx) as u64 as u64; fl_r = (rbx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142252e95); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = ((rbx.wrapping_add(0x58u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142252e9e); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142252ea7); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 4; continue; }
            4 => { rbx = (crate::simd_ops::ru64(rdi.wrapping_add(0x2a0u64))) as u64; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142252eb7); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (rbx) as u64); crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), (0x0u32) as u32); r9 = (0x1u32) as u32 as u64; r8 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rdx = ((r9.wrapping_add(0x2fu64)) as u32) as u32 as u64; rcx = (0xc0u32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142252edb); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu32(rbx.wrapping_add(0x50u64), (0x4u32) as u32); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rsp = ((rsp).wrapping_add(0x50u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1422548f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x14397ff10; const P1: u64 = 0x48; const P2: u64 = 0x50; const P3: u64 = 0x1cc; const P4: u64 = 0x1c8; const P5: u64 = 0x1d0;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    r8 = (crate::simd_ops::ru64(P0)) as u64;
    fl_l = r8; fl_r = r8;
    if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { xmm0 = 0u128; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
    rdx = (crate::simd_ops::ru64(r8.wrapping_add(P1))) as u64;
    rax = (crate::simd_ops::ru32(r8.wrapping_add(P2))) as u32 as u64;
    rcx = ((rdx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64;
    fl_l = rdx; fl_r = rcx;
    if !((((fl_l as u64)) == ((fl_r as u64)))) { rcx = (crate::simd_ops::ru32(r8.wrapping_add(P3))) as u32 as u64; rax = (crate::simd_ops::ru32(r8.wrapping_add(P4))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; rdx = (crate::simd_ops::ru32(r8.wrapping_add(P5))) as u32 as u64; rcx = (if ((((fl_l as u32)) as i32) > (((fl_r as u32)) as i32)) { (rax as u32) } else { (rcx as u32) }) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; xmm0 = 0u128; rdx = (if ((((fl_l as u32)) as i32) > (((fl_r as u32)) as i32)) { (rcx as u32) } else { (rdx as u32) }) as u32 as u64; xmm0 = (xmm0 & !0xffff_ffffu128) | (((((rdx as u32)) as i32) as f32).to_bits() as u128); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
    xmm0 = 0u128;
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1422554d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143a37918; const P1: u64 = 0x14206b020; const P2: u64 = 0x142442de0; const P3: u64 = 0x14397ff10; const P4: u64 = 0x1412f5b10; const P5: u64 = 0x141834ee8; const P6: u64 = 0x14209d670; const P7: u64 = 0x1411e7fc0; const P8: u64 = 0x142256c90; const P9: u64 = 0x142256ee0; const P10: u64 = 0x1401de400; const P11: u64 = 0x1401de830; const P12: u64 = 0x1401e5380; const P13: u64 = 0x1412f6080; const P14: u64 = 0x1401e66f0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rcx; rdi = (crate::simd_ops::ru64(P0)) as u64; crate::simd_ops::wu64(P0, (0x0u64) as u64); fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } rsi = (crate::simd_ops::ru64(rdi)) as u64; fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 1; continue; } rcx = rsi; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rcx = rsi; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 1; continue; }
            1 => { rcx = rdi; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 2; continue; }
            2 => { crate::simd_ops::wu64(P3, (0x0u64) as u64); rcx = rbx.wrapping_add(0x2c8); r9 = P4; rdx = 0x10; r8 = ((rdx.wrapping_sub(0xeu64)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rdi = (crate::simd_ops::ru64(rbx.wrapping_add(0x2a0u64))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } rcx = rdi; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); rcx = rdi.wrapping_add(0x58); rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); rcx = rdi; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 3; continue; }
            3 => { rcx = rbx.wrapping_add(0x1c8); rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); rcx = rbx.wrapping_add(0xa8); r9 = P9; rdx = 0x48; r8 = ((rdx.wrapping_sub(0x44u64)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rcx = rbx.wrapping_add(0x68); r9 = P10; rdx = 0x10; r8 = ((rdx.wrapping_sub(0xcu64)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rcx = rbx.wrapping_add(0x58); rax = crate::simd_ops::call_stk(P11, rax, rcx, rdx, r8, r9, rsp); rcx = rbx.wrapping_add(0x48); rax = crate::simd_ops::call_stk(P11, rax, rcx, rdx, r8, r9, rsp); rcx = rbx.wrapping_add(0x20); rax = crate::simd_ops::call_stk(P12, rax, rcx, rdx, r8, r9, rsp); rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; rax = crate::simd_ops::call_stk(P13, rax, rcx, rdx, r8, r9, rsp); rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(P14, rax, rcx, rdx, r8, r9, rsp); }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142255e30(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x1412f5d40; const P2: u64 = 0xc;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        core::sync::atomic::fence(core::sync::atomic::Ordering::SeqCst);
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x50u64)) as u64; rcx = ((rcx).wrapping_add(0x8u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rdx); rax = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 1; continue; } fl_l = (crate::simd_ops::atomic_add_u32(rax.wrapping_add(P0), 1)) as u32 as u64; bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(r8)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rax); rax = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rax); r8 = rsp.wrapping_add(0x30); rdx = rsp.wrapping_add(0x20); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } rdi = 0xffffffff; rax = ((rdi as u32)) as u32 as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rcx.wrapping_add(P0), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 2; continue; } rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = rbx; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(rax)) as u64, rax, rcx, rdx, r8, r9, rsp); let xa_src: u32 = (rdi as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rbx.wrapping_add(P2), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rdi = (xa_old) as u32 as u64; fl_l = ((rdi as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 2; continue; } rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 2; continue; }
            2 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rsp = ((rsp).wrapping_add(0x50u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142255ec0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142645790; const P1: u64 = 0x1426455a8; const P2: u64 = 0x1412f64a0; const P3: u64 = 0x142645598;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        core::sync::atomic::fence(core::sync::atomic::Ordering::SeqCst);
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rsi = rdx; rdi = rcx; rbx = rcx.wrapping_add(0x2b8); rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P0)) as u64, rax, rcx, rdx, r8, r9, rsp); rbp = ((rax as u32)) as u32 as u64; r8 = (crate::simd_ops::ru32(rbx.wrapping_add(0x8u64))) as u32 as u64; fl_l = ((r8 as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 1; continue; } rcx = rbx; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P1)) as u64, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu32(rbx.wrapping_add(0x8u64), ((rbp as u32)) as u32); bb = 1; continue; }
            1 => { let _ = crate::simd_ops::atomic_add_u32(rbx.wrapping_add(0xcu64), (1) as u32); rax = (((crate::simd_ops::ru32(rdi.wrapping_add(0x2e8u64))) as i32 as i64) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rdi = ((rdi).wrapping_add(0x2c8u64)) as u64; rdi = ((rdi).wrapping_add(rax)) as u64; rdx = (crate::simd_ops::ru32(rdi.wrapping_add(0x8u64))) as u32 as u64; r8 = (crate::simd_ops::ru32(rdi.wrapping_add(0xcu64))) as u32 as u64; rcx = ((r8.wrapping_add(r8.wrapping_mul(4))) as u64) as u64; rcx = ((rcx).wrapping_shl(3u32)) as u64; rax = ((rdx.wrapping_add(rdx.wrapping_mul(4))) as u64) as u64; rax = ((rax).wrapping_shl(3u32)) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } rax = rdx.wrapping_add(0x1); rdx = ((r8 as u32)) as u32 as u64; rdx = ((rdx).wrapping_shr(1u32)) as u64; rdx = ((rdx).wrapping_add(r8)) as u64; fl_l = rdx; fl_r = rax; rdx = (if (((fl_l as u64)) < ((fl_r as u64))) { rax } else { rdx }) as u64; rcx = rdi; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x8u64))) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = (crate::simd_ops::ru64(rdi)) as u64; rdx = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; rax = (crate::simd_ops::ru64(rsi)) as u64; crate::simd_ops::wu64(rdx, rax); rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rdx.wrapping_add(0x8u64), rcx); crate::simd_ops::wu64(rdx.wrapping_add(0x10u64), rcx); rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x10u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } fl_l = (crate::simd_ops::atomic_add_u32(rax.wrapping_add(0x8u64), 1)) as u32 as u64; bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(rdx.wrapping_add(0x8u64), rax); rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x10u64))) as u64; crate::simd_ops::wu64(rdx.wrapping_add(0x10u64), rax); crate::simd_ops::wu64(rdx.wrapping_add(0x18u64), rcx); crate::simd_ops::wu64(rdx.wrapping_add(0x20u64), rcx); rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x20u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } fl_l = (crate::simd_ops::atomic_add_u32(rax.wrapping_add(0x8u64), 1)) as u32 as u64; bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x18u64))) as u64; crate::simd_ops::wu64(rdx.wrapping_add(0x18u64), rax); rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(rdx.wrapping_add(0x20u64), rax); let _ = crate::simd_ops::atomic_add_u32(rdi.wrapping_add(0x8u64), (1) as u32); fl_l = (crate::simd_ops::atomic_add_u32(rbx.wrapping_add(0xcu64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; if (((fl_l as u32)) != 0) { bb = 5; continue; } rax = 0xffffffff; let tmp_xchg: u32 = crate::simd_ops::atomic_xchg_u32(rbx.wrapping_add(0x8u64), ((rax as u32)) as u32); rax = (tmp_xchg) as u32 as u64; rcx = rbx; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P3)) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 5; continue; }
            5 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1422568c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x24;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = 0x1;
    fl_l = ((rax as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64;
    rax = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(P0), ((rax as u32)) as u32);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1422568e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x24; const P1: u64 = 0x28;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = 0x1;
    fl_l = ((rax as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64;
    rax = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64;
    rax = (rax as u32 as i32 as i64) as u64;
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(rax.wrapping_mul(8)).wrapping_add(P1))) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142256c90(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x40; const P1: u64 = 0x14209d670; const P2: u64 = 0x58; const P3: u64 = 0x1411e7fc0; const P4: u64 = 0x142442de0; const P5: u64 = 0x38; const P6: u64 = 0x28; const P7: u64 = 0x1412cf770; const P8: u64 = 0xfffffffffffffffa; const P9: u64 = 0x141834ee8;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = rcx; rbx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 1; continue; } rcx = rbx; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rcx = ((rbx.wrapping_add(P2)) as u64) as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rcx = rbx; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); bb = 1; continue; }
            1 => { rbx = (crate::simd_ops::ru64(rdi.wrapping_add(P5))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } rcx = rbx; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rcx = ((rbx.wrapping_add(P2)) as u64) as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rcx = rbx; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); bb = 2; continue; }
            2 => { rcx = ((rdi.wrapping_add(P6)) as u64) as u64; r9 = P7; rdx = 0x8; r8 = ((rdx.wrapping_add(P8)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P9, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142256d10(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0x18; const P3: u64 = 0x1c; const P4: u64 = 0x20; const P5: u64 = 0x1412cf770; const P6: u64 = 0x140200580; const P7: u64 = 0x2; const P8: u64 = 0x141834fc0; const P9: u64 = 0x38; const P10: u64 = 0x40; const P11: u64 = 0x48; const P12: u64 = 0x143427178; const P13: u64 = 0x143159a38; const P14: u64 = 0x143159a40;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rbx = rcx; rdi = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rcx, rdi); crate::simd_ops::wu64(rcx.wrapping_add(P0), rdi); crate::simd_ops::wu64(rcx.wrapping_add(P1), rdi); crate::simd_ops::wu32(rcx.wrapping_add(P2), (0x8u32) as u32); crate::simd_ops::wu32(rcx.wrapping_add(P3), (0x2u32) as u32); crate::simd_ops::wu64(rcx.wrapping_add(P4), (0x10u64) as u64); rcx = ((rcx).wrapping_add(0x28u64)) as u64; rax = P5; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); r9 = P6; rdx = ((rdi.wrapping_add(P0)) as u32) as u32 as u64; r8 = ((rdi.wrapping_add(P7)) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P8, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rbx.wrapping_add(P9), rdi); crate::simd_ops::wu64(rbx.wrapping_add(P10), rdi); crate::simd_ops::wu64(rbx.wrapping_add(P11), rdi); xmm1 = ((crate::simd_ops::ru32(P12)) as u32) as u128; xmm0 = ((crate::simd_ops::ru64(P13)) as u64) as u128; crate::simd_ops::wu64(rbx.wrapping_add(0x50u64), ((xmm0 as u64)) as u64); rax = (crate::simd_ops::ru32(P14)) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x58u64), ((rax as u32)) as u32); crate::simd_ops::wu32(rbx.wrapping_add(0x5cu64), ((xmm1 as u32)) as u32); crate::simd_ops::wu32(rbx.wrapping_add(0x60u64), ((xmm1 as u32)) as u32); crate::simd_ops::wu32(rbx.wrapping_add(0x64u64), ((xmm1 as u32)) as u32); xmm0 = ((crate::simd_ops::ru64(P13)) as u64) as u128; crate::simd_ops::wu64(rbx.wrapping_add(0x68u64), ((xmm0 as u64)) as u64); rax = (crate::simd_ops::ru32(P14)) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x70u64), ((rax as u32)) as u32); crate::simd_ops::wu32(rbx.wrapping_add(0x74u64), ((xmm1 as u32)) as u32); crate::simd_ops::wu32(rbx.wrapping_add(0x78u64), ((xmm1 as u32)) as u32); crate::simd_ops::wu32(rbx.wrapping_add(0x7cu64), ((xmm1 as u32)) as u32); xmm0 = ((crate::simd_ops::ru64(P13)) as u64) as u128; crate::simd_ops::wu64(rbx.wrapping_add(0x80u64), ((xmm0 as u64)) as u64); rax = (crate::simd_ops::ru32(P14)) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x88u64), ((rax as u32)) as u32); crate::simd_ops::wu32(rbx.wrapping_add(0x8cu64), ((xmm1 as u32)) as u32); crate::simd_ops::wu32(rbx.wrapping_add(0x90u64), ((xmm1 as u32)) as u32); crate::simd_ops::wu32(rbx.wrapping_add(0x94u64), ((xmm1 as u32)) as u32); xmm0 = ((crate::simd_ops::ru64(P13)) as u64) as u128; crate::simd_ops::wu64(rbx.wrapping_add(0x98u64), ((xmm0 as u64)) as u64); rax = (crate::simd_ops::ru32(P14)) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0xa0u64), ((rax as u32)) as u32); crate::simd_ops::wu32(rbx.wrapping_add(0xa4u64), ((xmm1 as u32)) as u32); crate::simd_ops::wu32(rbx.wrapping_add(0xa8u64), ((xmm1 as u32)) as u32); crate::simd_ops::wu32(rbx.wrapping_add(0xacu64), ((xmm1 as u32)) as u32); crate::simd_ops::wu64(rbx.wrapping_add(0xb0u64), rdi); crate::simd_ops::wu8(rbx.wrapping_add(0xb8u64), ((rdi as u8)) as u8); crate::simd_ops::wu64(rbx.wrapping_add(0xc0u64), rdi); crate::simd_ops::wu64(rbx.wrapping_add(0xc8u64), rdi); rax = rbx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142256e60(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x40; const P1: u64 = 0x143a00170; const P2: u64 = 0x143426abc; const P3: u64 = 0x38; const P4: u64 = 0xfffffffffffffffc;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rdx = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 2; continue; } xmm1 = ((crate::simd_ops::ru64(P1)) as u64) as u128; xmm2 = ((crate::simd_ops::ru32(P2)) as u32) as u128; xmm1 = crate::simd_ops::vcvtpd2ps(xmm1); bb = 1; continue; }
            1 => { rax = ((rdx as u32)) as u32 as u64; rax = ((rax).wrapping_shl(5u32)) as u64; rax = ((rax).wrapping_add(crate::simd_ops::ru64(rcx.wrapping_add(P3)))) as u64; xmm0 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) - f32::from_bits(crate::simd_ops::ru32(rax.wrapping_add(P4)))).to_bits() as u128); fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = ((xmm2 as u32)) as u32 as u64; if !(f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 2; continue; } fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0x1; rdx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(P0), ((rdx as u32)) as u32); if (((fl_l as u32)) != ((fl_r as u32))) { bb = 1; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142256eb0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x28; const P1: u64 = 0x8; const P2: u64 = 0x30; const P3: u64 = 0x2c; const P4: u64 = 0x1; const P5: u64 = 0x4; const P6: u64 = 0xc;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    xmm0 = ((crate::simd_ops::ru64(rdx)) as u64) as u128;
    crate::simd_ops::wu64(rcx.wrapping_add(P0), ((xmm0 as u64)) as u64);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P1))) as u32 as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(P2), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rcx.wrapping_add(P3))) as u32 as u64;
    rax = ((rax.wrapping_mul(2).wrapping_add(P4)) as u32) as u32 as u64;
    rax = ((((rax as u32)) as i32).wrapping_mul((crate::simd_ops::ru32(rcx.wrapping_add(P2))) as i32) as u32) as u32 as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(P5), ((rax as u32)) as u32);
    crate::simd_ops::wu32(rcx.wrapping_add(P1), ((rax as u32)) as u32);
    crate::simd_ops::wu32(rcx.wrapping_add(P6), ((rax as u32)) as u32);
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142256f60(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1434268d0; const P1: u64 = 0x143428760; const P2: u64 = 0x8; const P3: u64 = 0x10; const P4: u64 = 0x1c; const P5: u64 = 0x24; const P6: u64 = 0x14; const P7: u64 = 0x18; const P8: u64 = 0x4; const P9: u64 = 0xc; const P10: u64 = 0x28; const P11: u64 = 0x143426780;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut xmm4: u128 = 0; let mut xmm5: u128 = 0; let mut xmm6: u128 = 0; let mut xmm7: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x38u64)) as u64; xmm2 = ((crate::simd_ops::ru32(P0)) as u32) as u128; xmm3 = ((crate::simd_ops::ru32(P1)) as u32) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm6) as u128); rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rdx, rax); crate::simd_ops::wu64(rdx.wrapping_add(P2), rax); crate::simd_ops::wu64(rdx.wrapping_add(P3), rax); crate::simd_ops::wu64(rdx.wrapping_add(P4), rax); crate::simd_ops::wu32(rdx.wrapping_add(P5), ((rax as u32)) as u32); rax = rdx; xmm1 = ((crate::simd_ops::ru64(rcx.wrapping_add(P4))) as u64) as u128; xmm5 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) - f32::from_bits(crate::simd_ops::ru32(rcx.wrapping_add(P3)))).to_bits() as u128); crate::simd_ops::wu128(rsp.wrapping_add(0x10u64), (xmm7) as u128); xmm0 = crate::simd_ops::vshufps(xmm1, xmm1, 0x55u8); xmm6 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) - f32::from_bits(crate::simd_ops::ru32(rcx.wrapping_add(P6)))).to_bits() as u128); xmm0 = ((crate::simd_ops::ru32(rcx.wrapping_add(P5))) as u32) as u128; xmm7 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) - f32::from_bits(crate::simd_ops::ru32(rcx.wrapping_add(P7)))).to_bits() as u128); xmm1 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) / f32::from_bits(xmm6 as u32)).to_bits() as u128); crate::simd_ops::wu32(rdx.wrapping_add(P8), ((xmm1 as u32)) as u32); xmm0 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) / f32::from_bits(xmm5 as u32)).to_bits() as u128); crate::simd_ops::wu32(rdx, ((xmm0 as u32)) as u32); xmm0 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) / f32::from_bits(xmm7 as u32)).to_bits() as u128); crate::simd_ops::wu32(rdx.wrapping_add(P2), ((xmm0 as u32)) as u32); xmm0 = ((crate::simd_ops::ru32(rcx.wrapping_add(P3))) as u32) as u128; xmm2 = ((crate::simd_ops::ru32(rcx.wrapping_add(P6))) as u32) as u128; xmm4 = ((crate::simd_ops::ru32(rcx.wrapping_add(P7))) as u32) as u128; xmm0 = (xmm0) ^ (xmm3); xmm1 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) / f32::from_bits(xmm5 as u32)).to_bits() as u128); crate::simd_ops::wu32(rdx.wrapping_add(P9), ((xmm1 as u32)) as u32); xmm2 = (xmm2) ^ (xmm3); xmm0 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) / f32::from_bits(xmm6 as u32)).to_bits() as u128); crate::simd_ops::wu32(rdx.wrapping_add(P3), ((xmm0 as u32)) as u32); xmm0 = 0u128; xmm1 = (xmm4) ^ (xmm3); xmm2 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) / f32::from_bits(xmm7 as u32)).to_bits() as u128); crate::simd_ops::wu32(rdx.wrapping_add(P6), ((xmm2 as u32)) as u32); xmm0 = (xmm0 & !0xffff_ffffu128) | ((((crate::simd_ops::ru32(rcx.wrapping_add(P10))) as i32) as f32).to_bits() as u128); xmm1 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(crate::simd_ops::ru32(P11))).to_bits() as u128); xmm0 = 0u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((((crate::simd_ops::ru32(rcx.wrapping_add(0x30u64))) as i32) as f32).to_bits() as u128); xmm2 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) / f32::from_bits(xmm0 as u32)).to_bits() as u128); crate::simd_ops::wu32(rdx.wrapping_add(0x1cu64), ((xmm2 as u32)) as u32); crate::simd_ops::wu32(rdx.wrapping_add(0x20u64), ((xmm2 as u32)) as u32); crate::simd_ops::wu32(rdx.wrapping_add(0x24u64), ((xmm2 as u32)) as u32); xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; xmm7 = (crate::simd_ops::ru128(rsp.wrapping_add(0x10u64))) as u128; xmm0 = 0u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((((crate::simd_ops::ru32(rcx.wrapping_add(0x28u64))) as i32) as f32).to_bits() as u128); crate::simd_ops::wu32(rdx.wrapping_add(0x18u64), ((xmm0 as u32)) as u32); rsp = ((rsp).wrapping_add(0x38u64)) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142257060(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1434268d0; const P1: u64 = 0x143427080; const P2: u64 = 0x143426780; const P3: u64 = 0x143426830; const P4: u64 = 0x1434267bc; const P5: u64 = 0x143426ee8; const P6: u64 = 0x14342668c;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut xmm4: u128 = 0; let mut xmm5: u128 = 0; let mut xmm6: u128 = 0; let mut xmm7: u128 = 0; let mut xmm8: u128 = 0; let mut xmm9: u128 = 0; let mut xmm10: u128 = 0; let mut xmm11: u128 = 0; let mut xmm12: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 15>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x78u64)) as u64; crate::simd_ops::wu128(rsp.wrapping_add(0x60u64), (xmm6) as u128); r8 = rcx; xmm6 = ((crate::simd_ops::ru32(P0)) as u32) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x50u64), (xmm7) as u128); rdx = 0; fl_l = 0; fl_r = 0; xmm7 = ((crate::simd_ops::ru32(P1)) as u32) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm8) as u128); xmm8 = ((crate::simd_ops::ru32(P2)) as u32) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm9) as u128); xmm9 = ((crate::simd_ops::ru32(P3)) as u32) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm10) as u128); xmm10 = ((crate::simd_ops::ru32(P4)) as u32) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x10u64), (xmm11) as u128); xmm11 = ((crate::simd_ops::ru32(P5)) as u32) as u128; crate::simd_ops::wu128(rsp, (xmm12) as u128); xmm12 = ((crate::simd_ops::ru32(P6)) as u32) as u128; xmm3 = (xmm8) as u128; xmm2 = (xmm8) as u128; bb = 1; continue; }
            1 => { xmm3 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) + f32::from_bits(xmm9 as u32)).to_bits() as u128); fl_l = ((xmm3 as u32)) as u32 as u64; fl_r = ((xmm6 as u32)) as u32 as u64; xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) + f32::from_bits(xmm10 as u32)).to_bits() as u128); if !(f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { bb = 2; continue; } xmm3 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) + f32::from_bits(xmm7 as u32)).to_bits() as u128); bb = 2; continue; }
            2 => { fl_l = ((xmm2 as u32)) as u32 as u64; fl_r = ((xmm6 as u32)) as u32 as u64; if !(f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { bb = 3; continue; } xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) + f32::from_bits(xmm7 as u32)).to_bits() as u128); bb = 3; continue; }
            3 => { xmm4 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) - f32::from_bits(xmm8 as u32)).to_bits() as u128); xmm5 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) - f32::from_bits(xmm8 as u32)).to_bits() as u128); xmm1 = (xmm5 & !0xffff_ffffu128) | ((f32::from_bits(xmm5 as u32) * f32::from_bits(xmm5 as u32)).to_bits() as u128); xmm0 = (xmm4 & !0xffff_ffffu128) | ((f32::from_bits(xmm4 as u32) * f32::from_bits(xmm4 as u32)).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) + f32::from_bits(xmm0 as u32)).to_bits() as u128); fl_l = ((xmm1 as u32)) as u32 as u64; fl_r = ((xmm12 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 4; continue; } xmm0 = (xmm5 & !0xffff_ffffu128) | ((f32::from_bits(xmm5 as u32) * f32::from_bits(xmm11 as u32)).to_bits() as u128); rcx = (((crate::simd_ops::cvt_f32_i32(f32::from_bits(xmm0 as u32)) as i32)) as u32) as u32 as u64; crate::simd_ops::wu8(rdx.wrapping_add(r8), ((rcx as u8)) as u8); rdx = (((rdx as u32)).wrapping_add(1)) as u32 as u64; xmm0 = (xmm4 & !0xffff_ffffu128) | ((f32::from_bits(xmm4 as u32) * f32::from_bits(xmm11 as u32)).to_bits() as u128); rcx = (((crate::simd_ops::cvt_f32_i32(f32::from_bits(xmm0 as u32)) as i32)) as u32) as u32 as u64; crate::simd_ops::wu8(rdx.wrapping_add(r8), ((rcx as u8)) as u8); fl_l = (((rdx as u32)).wrapping_add(1)) as u32 as u64; rdx = ((fl_l as u32)) as u32 as u64; bb = 4; continue; }
            4 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0x4000; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 1; continue; } xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x60u64))) as u128; xmm7 = (crate::simd_ops::ru128(rsp.wrapping_add(0x50u64))) as u128; xmm8 = (crate::simd_ops::ru128(rsp.wrapping_add(0x40u64))) as u128; xmm9 = (crate::simd_ops::ru128(rsp.wrapping_add(0x30u64))) as u128; xmm10 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; xmm11 = (crate::simd_ops::ru128(rsp.wrapping_add(0x10u64))) as u128; xmm12 = (crate::simd_ops::ru128(rsp)) as u128; rsp = ((rsp).wrapping_add(0x78u64)) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142257180(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0xf; const P1: u64 = 0x4;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rdx = (((rdx as u32)).wrapping_add(0xfu32)) as u32 as u64;
    rax = ((r8.wrapping_add(P0)) as u32) as u32 as u64;
    rax = (((rax as u32)).wrapping_shr(4u32)) as u32 as u64;
    rdx = (((rdx as u32)).wrapping_shr(4u32)) as u32 as u64;
    rdx = (((rdx as u32)).wrapping_shl(8u32)) as u32 as u64;
    rax = ((((rax as u32)) as i32).wrapping_mul(((rdx as u32)) as i32) as u32) as u32 as u64;
    crate::simd_ops::wu32(rcx, ((rdx as u32)) as u32);
    crate::simd_ops::wu32(rcx.wrapping_add(P1), ((rax as u32)) as u32);
    rax = rcx;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1422586a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143a33638; const P1: u64 = 0x143428240; const P2: u64 = 0x143c49500; const P3: u64 = 0x24; const P4: u64 = 0x28; const P5: u64 = 0x14241d6f0; const P6: u64 = 0x143a335e8; const P7: u64 = 0x14397ff04; const P8: u64 = 0x14397d82f; const P9: u64 = 0x14397d82e; const P10: u64 = 0x14397ff20; const P11: u64 = 0x14397d82a; const P12: u64 = 0x14397d82b;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        core::sync::atomic::fence(core::sync::atomic::Ordering::SeqCst);
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdx = (crate::simd_ops::ru64(P0)) as u64; rdi = 0xffffffff; xmm0 = (crate::simd_ops::ru128(P1)) as u128; crate::simd_ops::wu128(P2, (xmm0) as u128); fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 1; continue; } rax = ((rdi as u32)) as u32 as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rdx.wrapping_add(P3), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 1; continue; } rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P4))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 1; continue; } (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 1; continue; }
            1 => { rdx = (crate::simd_ops::ru64(P6)) as u64; rbx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(P0, rbx); fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } let xa_src: u32 = (rdi as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rdx.wrapping_add(P3), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rdi = (xa_old) as u32 as u64; fl_l = ((rdi as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 2; continue; } rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P4))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 2; continue; }
            2 => { crate::simd_ops::wu64(P6, rbx); rax = ((rbx as u32)) as u32 as u64; rcx = ((rbx as u32)) as u32 as u64; xmm0 = 0u128; crate::simd_ops::wu32(P7, ((xmm0 as u32)) as u32); let tmp_xchg: u8 = crate::simd_ops::atomic_xchg_u8(P8, ((rax as u8)) as u8); rax = (rax & !0xffu64) | (((tmp_xchg) as u8 as u64)); let tmp_xchg: u8 = crate::simd_ops::atomic_xchg_u8(P9, ((rcx as u8)) as u8); rcx = (rcx & !0xffu64) | (((tmp_xchg) as u8 as u64)); rax = ((rbx as u32)) as u32 as u64; rcx = ((rbx as u32)) as u32 as u64; let tmp_xchg: u32 = crate::simd_ops::atomic_xchg_u32(P10, ((rax as u32)) as u32); rax = (tmp_xchg) as u32 as u64; let tmp_xchg: u8 = crate::simd_ops::atomic_xchg_u8(P11, ((rcx as u8)) as u8); rcx = (rcx & !0xffu64) | (((tmp_xchg) as u8 as u64)); let tmp_xchg: u8 = crate::simd_ops::atomic_xchg_u8(P12, ((rbx as u8)) as u8); rbx = (rbx & !0xffu64) | (((tmp_xchg) as u8 as u64)); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1422589f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143bbcff0; const P1: u64 = 0x1403b3290; const P2: u64 = 0x143bbcff8; const P3: u64 = 0x143bbd000; const P4: u64 = 0x143bbd008; const P5: u64 = 0x143bbd010; const P6: u64 = 0x143bbd018; const P7: u64 = 0x143bbd020; const P8: u64 = 0x143bbd028; const P9: u64 = 0x143bbd030; const P10: u64 = 0x143bbd038; const P11: u64 = 0x14241d6f0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        core::sync::atomic::fence(core::sync::atomic::Ordering::SeqCst);
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rcx; rdx = rcx; rbp = P0; rcx = rbp; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rdx = rbx.wrapping_add(0x8); rcx = P2; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rdx = rbx.wrapping_add(0x10); rcx = P3; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rdx = rbx.wrapping_add(0x18); rcx = P4; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rdx = rbx.wrapping_add(0x20); rcx = P5; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rdx = rbx.wrapping_add(0x28); rcx = P6; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rdx = rbx.wrapping_add(0x30); rcx = P7; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rdx = rbx.wrapping_add(0x38); rcx = P8; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rdx = rbx.wrapping_add(0x40); rcx = P9; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rdx = rbx.wrapping_add(0x48); rcx = P10; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rdi = 0; fl_l = 0; fl_r = 0; rsi = rbx.wrapping_add(0x50); bb = 1; continue; }
            1 => { rbx = (crate::simd_ops::ru64(rdi.wrapping_add(rsi))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rbx); fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } rax = 0x1; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rbx.wrapping_add(0x24u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; rax = (((rax as u32)).wrapping_add(1)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 2; continue; } rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x28u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } rdx = rbx; rax = crate::simd_ops::call_stk(P11, rax, rcx, rdx, r8, r9, rsp); bb = 2; continue; }
            2 => { rdx = (crate::simd_ops::ru64(rdi.wrapping_add(rbp).wrapping_add(0x50u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } rax = 0xffffffff; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rdx.wrapping_add(0x24u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 3; continue; } rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x28u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } rax = crate::simd_ops::call_stk(P11, rax, rcx, rdx, r8, r9, rsp); bb = 3; continue; }
            3 => { crate::simd_ops::wu64(rdi.wrapping_add(rbp).wrapping_add(0x50u64), rbx); rdi = ((rdi).wrapping_add(0x8u64)) as u64; fl_l = rdi; fl_r = (0xc8u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 1; continue; } rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142258b30(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    crate::simd_ops::forward_incoming_to_outgoing();
    const P0: u64 = 0x14397ff58; const P1: u64 = 0x1412f7ac0;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rcx = (crate::simd_ops::ru64(P0)) as u64;
    rdx = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8(P1, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142258e80(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x142417470; const P2: u64 = 0x608; const P3: u64 = 0x600; const P4: u64 = 0x1412f9000;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        core::sync::atomic::fence(core::sync::atomic::Ordering::SeqCst);
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x28u64)) as u64; rdi = rcx; rbx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; fl_l = ((rbx).wrapping_add(0x5f8u64)) as u64; fl_r = fl_l; rbx = ((fl_l as u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), (0x64u64) as u64); bb = 1; continue; }
            1 => { rax = ((crate::simd_ops::ru8(rbx)) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { rax = ((crate::simd_ops::ru8(rbx)) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 2; continue; } bb = 3; continue; }
            3 => { rax = 0x1; let tmp_xchg: u8 = crate::simd_ops::atomic_xchg_u8(rbx, ((rax as u8)) as u8); rax = (rax & !0xffu64) | (((tmp_xchg) as u8 as u64)); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 4; continue; } rcx = rsp.wrapping_add(0x40); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 1; continue; }
            4 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(P0))) as u64; r8 = (crate::simd_ops::ru32(rax.wrapping_add(P2))) as u32 as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(P3))) as u64; rcx = ((rax.wrapping_add(r8.wrapping_mul(4))) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rcx); crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rax); r9 = ((crate::simd_ops::ru8(rsp.wrapping_add(0x40u64))) as u32) as u32 as u64; rdx = rsp.wrapping_add(0x48); rcx = rsp.wrapping_add(0x50); rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; crate::simd_ops::wu8(rax, (0x0u8) as u8); rsp = ((rsp).wrapping_add(0x28u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142258f20(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x1433f3360; const P1: u64 = 0x8;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = P0;
    crate::simd_ops::wu8(rcx.wrapping_add(P1), ((rdx as u8)) as u8);
    crate::simd_ops::wu64(rcx, rax);
    rax = rcx;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142258f50(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    crate::simd_ops::forward_incoming_to_outgoing();
    const P0: u64 = 0x143a082f8; const P1: u64 = 0x14397ff38; const P2: u64 = 0x142485780;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rcx = (crate::simd_ops::ru64(P0)) as u64;
    rdx = P1;
    crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8(P2, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1422591f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x298; const P1: u64 = 0x2c0; const P2: u64 = 0x290; const P3: u64 = 0x142330950; const P4: u64 = 0x143a082f8; const P5: u64 = 0x10; const P6: u64 = 0x142407c80; const P7: u64 = 0x48; const P8: u64 = 0x142485aa0; const P9: u64 = 0x1433f3360; const P10: u64 = 0x8; const P11: u64 = 0x18; const P12: u64 = 0x14397ff38; const P13: u64 = 0x142486460; const P14: u64 = 0x142645580;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        core::sync::atomic::fence(core::sync::atomic::Ordering::SeqCst);
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rsi = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; rbx = 0; fl_l = 0; fl_r = 0; r14 = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; rdi = rcx; fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(P2))) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(rbx.wrapping_mul(8)))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } rdx = (crate::simd_ops::ru32(r14.wrapping_add(rbx.wrapping_mul(4)))) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 2; continue; }
            2 => { rbx = ((rbx).wrapping_add(1)) as u64; fl_l = rbx; fl_r = rsi; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 1; continue; } bb = 3; continue; }
            3 => { rdi = (crate::simd_ops::ru64(P4)) as u64; rbx = (crate::simd_ops::ru64(rdi.wrapping_add(P5))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = ((rax as u32)) as u32 as u64; rdx = rsp.wrapping_add(0x20); rcx = ((rcx).wrapping_shl(6u32)) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(rbx).wrapping_add(P7))) as u64; rcx = ((rcx).wrapping_add(0x40u64)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P8, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; rcx = 0x1; let tmp_xchg: u32 = crate::simd_ops::atomic_xchg_u32(rax, ((rcx as u32)) as u32); rcx = (tmp_xchg) as u32 as u64; r8 = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; rax = P9; xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; crate::simd_ops::wu64(r8.wrapping_add(P10), (0x0u64) as u64); crate::simd_ops::wu64(r8.wrapping_add(P5), rax); rax = ((crate::simd_ops::ru8(rsp.wrapping_add(0x40u64))) as u32) as u32 as u64; crate::simd_ops::wu8(r8.wrapping_add(P11), ((rax as u8)) as u8); crate::simd_ops::wu128(P12, (xmm0) as u128); rax = (crate::simd_ops::ru32(r8)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 4; continue; } rbx = (crate::simd_ops::ru64(rdi.wrapping_add(0x10u64))) as u64; rdx = (crate::simd_ops::ru64(P12)) as u64; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P13, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru64(rbx)) as u64; r8 = 0; fl_l = 0; fl_r = 0; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(0x1008u64))) as u64; rdx = ((r8.wrapping_add(0x1u64)) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm((crate::simd_ops::ru64(P14)) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 4; continue; }
            4 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14225bfe0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x3f; const P1: u64 = 0x40; const P2: u64 = 0x20; const P3: u64 = 0x38; const P4: u64 = 0xffffffffffffffa0; const P5: u64 = 0x10; const P6: u64 = 0xffffffffffffffa8; const P7: u64 = 0xffffffffffffffc8; const P8: u64 = 0xffffffffffffffb8;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut xmm4: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 1>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rdi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); r15 = (crate::simd_ops::ru64(rcx)) as u64; rdi = ((rdx.wrapping_add(P0)) as u64) as u64; rdx = (crate::simd_ops::ru32(r8.wrapping_add(P1))) as u32 as u64; rbx = 0; fl_l = 0; fl_r = 0; rcx = (crate::simd_ops::ru32(r8.wrapping_add(P2))) as u32 as u64; r14 = r9; rsi = (((rdx) as i64).wrapping_mul((0x38u64) as i64) as u64) as u64; rcx = (((rcx as u32)).wrapping_add((rdx as u32))) as u32 as u64; rdi = ((rdi).wrapping_shr(6u32)) as u64; rsi = ((rsi).wrapping_add(crate::simd_ops::ru64(r8.wrapping_add(P3)))) as u64; r10 = ((rbx as u32)) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(P1), ((rcx as u32)) as u32); fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } r11 = ((rbx as u32)) as u32 as u64; bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(r15.wrapping_add(r10.wrapping_mul(8)))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } r9 = (((rbx) as i64).wrapping_mul((0x38u64) as i64) as u64) as u64; r9 = ((r9).wrapping_add(0x28u64)) as u64; fl_l = ((r9).wrapping_add(rsi)) as u64; fl_r = fl_l; r9 = ((fl_l as u64)) as u64; bb = 2; continue; }
            2 => { let bit_in: u64 = rax; r8 = ((bit_in.trailing_zeros() as u64)) as u64; fl_l = (r8 as u64) as u64; r8 = ((r8 as u32)) as u32 as u64; r9 = ((r9.wrapping_add(P3)) as u64) as u64; rbx = ((rbx).wrapping_add(1)) as u64; fl_l = (((rax) >> (((r8) as u32 & 63))) & 1) as u64; rax = ((rax) ^ ((1u64) << (((r8) as u32 & 63)))) as u64; rcx = ((r11.wrapping_add(r8)) as u64) as u64; crate::simd_ops::wu32(r9.wrapping_add(P4), ((rcx as u32)) as u32); rdx = ((rcx.wrapping_add(rcx.wrapping_mul(2))) as u64) as u64; rcx = (crate::simd_ops::ru64(r14)) as u64; rdx = ((rdx).wrapping_add(rdx)) as u64; xmm0 = (crate::simd_ops::ru128(rcx.wrapping_add(rdx.wrapping_mul(8)).wrapping_add(P5))) as u128; xmm4 = (crate::simd_ops::ru128(rcx.wrapping_add(rdx.wrapping_mul(8)).wrapping_add(P2))) as u128; xmm3 = (crate::simd_ops::ru128(rcx.wrapping_add(rdx.wrapping_mul(8)))) as u128; xmm1 = crate::simd_ops::vshufps(xmm0, xmm4, 0x9eu8); xmm2 = crate::simd_ops::vshufps(xmm3, xmm0, 0x49u8); xmm0 = crate::simd_ops::vshufps(xmm3, xmm1, 0x8cu8); crate::simd_ops::wu128(r9.wrapping_add(P6), (xmm0) as u128); xmm0 = crate::simd_ops::vshufps(xmm2, xmm4, 0xcdu8); xmm1 = crate::simd_ops::vshufps(xmm2, xmm1, 0xd8u8); crate::simd_ops::wu128(r9.wrapping_add(P7), (xmm0) as u128); crate::simd_ops::wu128(r9.wrapping_add(P8), (xmm1) as u128); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 2; continue; } bb = 3; continue; }
            3 => { r10 = ((r10).wrapping_add(1)) as u64; r11 = ((r11).wrapping_add(0x40u64)) as u64; fl_l = r10; fl_r = rdi; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 1; continue; } bb = 4; continue; }
            4 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x10u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x18u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14225c7e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14397fed0; const P1: u64 = 0x8; const P2: u64 = 0xf0;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 0>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rdi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r14); rsi = (crate::simd_ops::ru64(rdx)) as u64; r9 = 0; fl_l = 0; fl_r = 0; rax = (crate::simd_ops::ru64(P0)) as u64; rsi = ((rsi).wrapping_add(0x3fu64)) as u64; r14 = (crate::simd_ops::ru64(rcx)) as u64; r10 = ((r9 as u32)) as u32 as u64; rbx = (crate::simd_ops::ru64(rdx.wrapping_add(P1))) as u64; rsi = ((rsi).wrapping_shr(6u32)) as u64; rdi = (crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64; fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } r11 = ((r9 as u32)) as u32 as u64; bb = 1; continue; }
            1 => { rcx = (crate::simd_ops::ru64(r14.wrapping_add(r10.wrapping_mul(8)))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { let bit_in: u64 = rcx; r8 = ((bit_in.trailing_zeros() as u64)) as u64; fl_l = (r8 as u64) as u64; r8 = ((r8 as u32)) as u32 as u64; fl_l = (((rcx) >> (((r8) as u32 & 63))) & 1) as u64; rcx = ((rcx) ^ ((1u64) << (((r8) as u32 & 63)))) as u64; rax = ((r11.wrapping_add(r8)) as u64) as u64; rdx = (crate::simd_ops::ru32(rbx.wrapping_add(rax.wrapping_mul(8)))) as u32 as u64; rax = (crate::simd_ops::ru32(rdi.wrapping_add(rdx.wrapping_mul(4)))) as u32 as u64; r9 = ((r9).wrapping_add(rax)) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 2; continue; } bb = 3; continue; }
            3 => { r10 = ((r10).wrapping_add(1)) as u64; r11 = ((r11).wrapping_add(0x40u64)) as u64; fl_l = r10; fl_r = rsi; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 1; continue; } bb = 4; continue; }
            4 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x8u64))) as u64; rax = r9; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x10u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x18u64))) as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14225c8f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (0u64, 0u64, 0u64);
    fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64;
    rax = (rax & !0xffu64) | (((if (((fl_l as u32)) > ((fl_r as u32))) { 1u8 } else { 0u8 }) as u8 as u64));
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14225c900(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142417470; const P1: u64 = 0x608; const P2: u64 = 0x600; const P3: u64 = 0x1412f9000;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        core::sync::atomic::fence(core::sync::atomic::Ordering::SeqCst);
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x28u64)) as u64; rdi = rcx; rbx = (crate::simd_ops::ru64(rcx)) as u64; fl_l = ((rbx).wrapping_add(0x5f8u64)) as u64; fl_r = fl_l; rbx = ((fl_l as u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), (0x64u64) as u64); bb = 1; continue; }
            1 => { rax = ((crate::simd_ops::ru8(rbx)) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { rax = ((crate::simd_ops::ru8(rbx)) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 2; continue; } bb = 3; continue; }
            3 => { rax = 0x1; let tmp_xchg: u8 = crate::simd_ops::atomic_xchg_u8(rbx, ((rax as u8)) as u8); rax = (rax & !0xffu64) | (((tmp_xchg) as u8 as u64)); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 4; continue; } rcx = rsp.wrapping_add(0x40); rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); bb = 1; continue; }
            4 => { rax = (crate::simd_ops::ru64(rdi)) as u64; r8 = (crate::simd_ops::ru32(rax.wrapping_add(P1))) as u32 as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64; rcx = ((rax.wrapping_add(r8.wrapping_mul(4))) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rcx); crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rax); r9 = ((crate::simd_ops::ru8(rsp.wrapping_add(0x40u64))) as u32) as u32 as u64; rdx = rsp.wrapping_add(0x48); rcx = rsp.wrapping_add(0x50); rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; crate::simd_ops::wu8(rax, (0x0u8) as u8); rsp = ((rsp).wrapping_add(0x28u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14225dd30(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142261ae0; const P1: u64 = 0x14397ff48; const P2: u64 = 0x1401de830; const P3: u64 = 0x1412f7ec0; const P4: u64 = 0x1412f7dc0; const P5: u64 = 0x14065f150; const P6: u64 = 0x1412f7d50;
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = rcx; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(P1, (0x0u64) as u64); rcx = rdi.wrapping_add(0x6f8); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rcx = rdi.wrapping_add(0x6e8); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rcx = rdi.wrapping_add(0x6d8); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rcx = rdi.wrapping_add(0x6c8); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rcx = rdi.wrapping_add(0x6b0); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rcx = rdi.wrapping_add(0x698); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rcx = rdi.wrapping_add(0x680); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rcx = rdi.wrapping_add(0x668); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rcx = rdi.wrapping_add(0x650); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rcx = rdi.wrapping_add(0x638); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rcx = rdi.wrapping_add(0x618); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rcx = rdi.wrapping_add(0x600); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rcx = rdi.wrapping_add(0x2d0); rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rcx = rdi.wrapping_add(0x180); rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rcx = rdi.wrapping_add(0x170); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rcx = rdi.wrapping_add(0x160); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rcx = rdi.wrapping_add(0x150); rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rcx = rdi.wrapping_add(0x140); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rcx = rdi.wrapping_add(0x130); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rcx = rdi.wrapping_add(0x38); rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); rcx = rdi.wrapping_add(0x28); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rcx = rdi.wrapping_add(0x18); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rcx = rdi; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14225f220(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x18; const P1: u64 = 0x320; const P2: u64 = 0x318; const P3: u64 = 0x14209d6f0; const P4: u64 = 0x50; const P5: u64 = 0x40; const P6: u64 = 0x68; const P7: u64 = 0x60; const P8: u64 = 0x70; const P9: u64 = 0xf8; const P10: u64 = 0xe8; const P11: u64 = 0x110;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; rbx = rcx; crate::simd_ops::wu8(rcx, (0x0u8) as u8); rdx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P2))) as u64; rdx = ((rax.wrapping_add(rdx.wrapping_mul(4))) as u64) as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rdi = 0; fl_l = 0; fl_r = 0; fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(P4))) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 1; continue; } rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P5))) as u64; rdx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx.wrapping_add(P6), rdi); crate::simd_ops::wu8(rbx.wrapping_add(P7), ((rdi as u8)) as u8); crate::simd_ops::wu32(rbx.wrapping_add(P8), ((rdi as u32)) as u32); bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(P9))) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 2; continue; } rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P10))) as u64; rdx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx.wrapping_add(P11), rdi); crate::simd_ops::wu8(rbx.wrapping_add(0x108u64), ((rdi as u8)) as u8); crate::simd_ops::wu32(rbx.wrapping_add(0x118u64), ((rdi as u32)) as u32); bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x1a0u64))) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 3; continue; } rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x190u64))) as u64; rdx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx.wrapping_add(0x1b8u64), rdi); crate::simd_ops::wu8(rbx.wrapping_add(0x1b0u64), ((rdi as u8)) as u8); crate::simd_ops::wu32(rbx.wrapping_add(0x1c0u64), ((rdi as u32)) as u32); bb = 3; continue; }
            3 => { fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x240u64))) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 4; continue; } rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x230u64))) as u64; rdx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx.wrapping_add(0x258u64), rdi); crate::simd_ops::wu8(rbx.wrapping_add(0x250u64), ((rdi as u8)) as u8); crate::simd_ops::wu32(rbx.wrapping_add(0x260u64), ((rdi as u32)) as u32); bb = 4; continue; }
            4 => { crate::simd_ops::wu64(rbx.wrapping_add(0x320u64), rdi); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1422645d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x98; const P2: u64 = 0x142336b40; const P3: u64 = 0x143a082f8; const P4: u64 = 0x142407c80; const P5: u64 = 0x48; const P6: u64 = 0x142485aa0; const P7: u64 = 0x1433f38f8; const P8: u64 = 0x8; const P9: u64 = 0x18; const P10: u64 = 0x143c8c930; const P11: u64 = 0x142486460; const P12: u64 = 0x142645580;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        core::sync::atomic::fence(core::sync::atomic::Ordering::SeqCst);
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rsi = rcx; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru64(P3)) as u64; rbx = (crate::simd_ops::ru64(rsi.wrapping_add(P0))) as u64; rdi = (crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = ((rax as u32)) as u32 as u64; rdx = ((rdx).wrapping_shl(6u32)) as u64; rcx = (crate::simd_ops::ru64(rdx.wrapping_add(rdi).wrapping_add(P5))) as u64; rdx = rsp.wrapping_add(0x20); rcx = ((rcx).wrapping_add(0x40u64)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; rcx = 0x1; let tmp_xchg: u32 = crate::simd_ops::atomic_xchg_u32(rax, ((rcx as u32)) as u32); rcx = (tmp_xchg) as u32 as u64; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; rcx = P7; xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; crate::simd_ops::wu64(rax.wrapping_add(P8), (0x0u64) as u64); crate::simd_ops::wu64(rax.wrapping_add(P9), rbx); rbx = (crate::simd_ops::ru64(P3)) as u64; crate::simd_ops::wu64(rax.wrapping_add(P0), rcx); crate::simd_ops::wu128(P10, (xmm0) as u128); rax = (crate::simd_ops::ru32(rax)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 1; continue; } rbx = (crate::simd_ops::ru64(rbx.wrapping_add(P0))) as u64; rdx = (crate::simd_ops::ru64(P10)) as u64; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P11, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru64(rbx)) as u64; r8 = 0; fl_l = 0; fl_r = 0; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(0x1008u64))) as u64; rdx = ((r8.wrapping_add(0x1u64)) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm((crate::simd_ops::ru64(P12)) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; let _ = crate::simd_ops::atomic_sub_u32(rax, (1) as u32); rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1423534e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    crate::simd_ops::forward_incoming_to_outgoing();
    const P0: u64 = 0x14241c0a0;
    let (mut rax, mut fl_l) = (crate::simd_ops::get_rax_in(), 0u64);
    rcx = ((rdx.wrapping_add(rdx.wrapping_mul(4))) as u64) as u64;
    rdx = 0x10;
    fl_l = ((rcx).wrapping_shl(5u32)) as u64;
    rcx = ((fl_l as u64)) as u64;
    crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8(P0, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1423f4310(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0x1426455b0; const P3: u64 = 0x142645708; const P4: u64 = 0x20; const P5: u64 = 0x1401f3ee0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rsi = rcx; rax = (crate::simd_ops::ru32(rcx.wrapping_add(P1))) as u32 as u64; rdi = ((rbx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 3; continue; } bb = 1; continue; }
            1 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P2)) as u64, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 2; continue; } rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P3)) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 2; continue; }
            2 => { rbx = ((rbx).wrapping_add(0x8u64)) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } bb = 3; continue; }
            3 => { rcx = ((rsi.wrapping_add(P4)) as u64) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(P5, rax, rcx, rdx, r8, r9, rsp); }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1423f43d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xfff; const P1: u64 = 0x142496730;
    let (mut rax, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rcx = (crate::simd_ops::ru64(r8)) as u64; rbp = rdx; rax = (crate::simd_ops::ru64(r9)) as u64; rdi = rcx; rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rdi = (((rdi) & (0xfffffffffffff000u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax); r14 = r9; rbx = ((rcx.wrapping_add(P0)) as u64) as u64; rsi = r8; rbx = ((rbx).wrapping_add(rax)) as u64; rcx = ((rcx).wrapping_sub(rdi)) as u64; rbx = (((rbx) & (0xfffffffffffff000u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rcx); rbx = ((rbx).wrapping_sub(rdi)) as u64; r9 = 0x1000; r8 = rbx; rcx = rbp; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rsi, rdi); rax = rbp; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; crate::simd_ops::wu64(r14, rbx); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1423f44c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x1426455b0; const P2: u64 = 0x142645708;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = (crate::simd_ops::ru64(rcx)) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; rdi = ((rbx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 3; continue; } bb = 1; continue; }
            1 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P1)) as u64, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 2; continue; } rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P2)) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 2; continue; }
            2 => { rbx = ((rbx).wrapping_add(0x8u64)) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } bb = 3; continue; }
            3 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1423f4580(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1423f9300; const P1: u64 = 0x1; const P2: u64 = 0x1426454f8; const P3: u64 = 0x142442de0; const P4: u64 = 0x142645708; const P5: u64 = 0x8;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 79>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x1423f4580, funcinfo_rva: 0x37dd01c, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0x278, conts: &[] };
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = ((rsp).wrapping_sub(0x270u64)) as u64; rbx = (rdx) as u64; xmm0 = (crate::simd_ops::ru128(rcx)) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm0) as u128); rdx = ((rsp.wrapping_add(0x40u64)) as u64) as u64; rcx = ((rsp.wrapping_add(0x50u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1423f45a9); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rsi = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (rsi) as u64); crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), (0x60000080u32) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), (0x3u32) as u32); r9 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rdx = (0x80000000u32) as u32 as u64; r8 = ((rsi.wrapping_add(P1)) as u32) as u32 as u64; rcx = (crate::simd_ops::ru64(rax)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1423f45d4); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P2)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdi = (rax) as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { fl_l = (rcx) as u64 as u64; fl_r = (rdx) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 3; continue; } bb = 2; continue; }
            2 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1423f45f1); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; bb = 3; continue; }
            3 => { fl_l = (rdx) as u64 as u64; fl_r = (rdx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { crate::simd_ops::wu16(rdx, ((rsi as u16)) as u16); bb = 5; continue; }
            5 => { fl_l = (rdi) as u64 as u64; fl_r = (0xffffffffffffffffu64) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 7; continue; } bb = 6; continue; }
            6 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1423f4609); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P4)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu32(rbx, ((rax as u32)) as u32); crate::simd_ops::wu8(rbx.wrapping_add(P5), (0x0u8) as u8); bb = 8; continue; }
            7 => { crate::simd_ops::wu64(rbx, (rdi) as u64); crate::simd_ops::wu8(rbx.wrapping_add(P5), (0x1u8) as u8); bb = 8; continue; }
            8 => { rax = (rbx) as u64; r11 = ((rsp.wrapping_add(0x270u64)) as u64) as u64; rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x10u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x18u64))) as u64; rsp = (r11) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1423f48b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x22; const P1: u64 = 0x26; const P2: u64 = 0x8; const P3: u64 = 0x10; const P4: u64 = 0x18; const P5: u64 = 0x20; const P6: u64 = 0x28;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rcx = 0;
    fl_l = 0; fl_r = 0;
    rax = rdx;
    crate::simd_ops::wu32(rdx.wrapping_add(P0), ((rcx as u32)) as u32);
    crate::simd_ops::wu16(rdx.wrapping_add(P1), ((rcx as u16)) as u16);
    crate::simd_ops::wu64(rdx, rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(P2), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(P3), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(P4), rcx);
    crate::simd_ops::wu16(rdx.wrapping_add(P5), (0x10u16) as u16);
    crate::simd_ops::wu64(rdx.wrapping_add(P6), rcx);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1423f48e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu64(rdx.wrapping_add(P0), rax);
    crate::simd_ops::wu64(rdx, rax);
    rax = rdx;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1423f4990(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu64(rdx.wrapping_add(P0), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(P1), rax);
    crate::simd_ops::wu64(rdx, rax);
    rax = rdx;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1423f4b40(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x28; const P2: u64 = 0x142645528; const P3: u64 = 0x10; const P4: u64 = 0x14; const P5: u64 = 0x18; const P6: u64 = 0x1c; const P7: u64 = 0x142645520; const P8: u64 = 0x4; const P9: u64 = 0x142645708;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x50u64)) as u64; rsi = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; r8 = rsp.wrapping_add(0x20); rax = 0; fl_l = 0; fl_r = 0; rbx = rdx; xmm0 = 0u128; ymm0_hi = 0; rdi = rcx; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rax); rcx = (crate::simd_ops::ru64(rsi.wrapping_add(P0))) as u64; rdx = 0; fl_l = 0; fl_r = 0; r9 = ((rax.wrapping_add(P1)) as u32) as u32 as u64; let y_addr: u64 = rsp.wrapping_add(0x20u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); (rax, xmm0) = crate::simd_ops::call_stk_xmm((crate::simd_ops::ru64(P2)) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 1; continue; } rax = (crate::simd_ops::ru32(rdi.wrapping_add(P3))) as u32 as u64; r8 = rsp.wrapping_add(0x20); rcx = (crate::simd_ops::ru64(rsi.wrapping_add(P0))) as u64; r9 = 0x28; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rax as u32)) as u32); rdx = 0; fl_l = 0; fl_r = 0; rax = (crate::simd_ops::ru32(rdi.wrapping_add(P4))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x24u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdi.wrapping_add(P5))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdi.wrapping_add(P6))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x34u64), ((rax as u32)) as u32); (rax, xmm0) = crate::simd_ops::call_stk_xmm((crate::simd_ops::ru64(P7)) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 1; continue; } crate::simd_ops::wu8(rbx.wrapping_add(P8), (0x1u8) as u8); rax = rbx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rsp = ((rsp).wrapping_add(0x50u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            1 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm((crate::simd_ops::ru64(P9)) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu32(rbx, ((rax as u32)) as u32); rax = rbx; crate::simd_ops::wu8(rbx.wrapping_add(P8), (0x0u8) as u8); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rsp = ((rsp).wrapping_add(0x50u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1423f5600(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0x18;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 0>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rcx, rdx);
    rax = (crate::simd_ops::ru64(r8)) as u64;
    crate::simd_ops::wu64(rcx.wrapping_add(P0), rax);
    rax = (crate::simd_ops::ru64(r9)) as u64;
    crate::simd_ops::wu64(rcx.wrapping_add(P1), rax);
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64;
    rdx = (crate::simd_ops::ru32(rax)) as u32 as u64;
    rax = rcx;
    crate::simd_ops::wu32(rcx.wrapping_add(P2), ((rdx as u32)) as u32);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1423f5630(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x18; const P2: u64 = 0x1; const P3: u64 = 0x10; const P4: u64 = 0x142645538; const P5: u64 = 0x142645708;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rbx = rdx; rdx = (crate::simd_ops::ru32(rcx.wrapping_add(P1))) as u32 as u64; r9 = 0; fl_l = 0; fl_r = 0; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 1; continue; } fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0x1; rdx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 2; continue; } fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 2; continue; } r9 = ((rdx.wrapping_add(P2)) as u32) as u32 as u64; bb = 2; continue; }
            1 => { r9 = 0x1; bb = 2; continue; }
            2 => { rdx = (crate::simd_ops::ru64(rcx.wrapping_add(P3))) as u64; r8 = rsp.wrapping_add(0x30); rcx = (crate::simd_ops::ru64(rdi.wrapping_add(P0))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (0xffffffffffffffffu64) as u64); rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P4)) as u64, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 3; continue; } rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu64(rdi.wrapping_add(P1), rax); crate::simd_ops::wu64(rbx, rax); rax = rbx; crate::simd_ops::wu8(rbx.wrapping_add(P0), (0x1u8) as u8); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            3 => { rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P5)) as u64, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu32(rbx, ((rax as u32)) as u32); rax = rbx; crate::simd_ops::wu8(rbx.wrapping_add(P0), (0x0u8) as u8); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1423f5d70(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1423f9300; const P1: u64 = 0x1426454b8; const P2: u64 = 0x4; const P3: u64 = 0x142645708; const P4: u64 = 0x142442de0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 75>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x1423f5d70, funcinfo_rva: 0x37dd394, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0x258, conts: &[] };
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64); rsp = ((rsp).wrapping_sub(0x250u64)) as u64; rbx = (rdx) as u64; xmm0 = (crate::simd_ops::ru128(rcx)) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128); rdx = ((rsp.wrapping_add(0x20u64)) as u64) as u64; rcx = ((rsp.wrapping_add(0x30u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1423f5d90); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1423f5d9c); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P1)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu8(rbx.wrapping_add(P2), (0x1u8) as u8); bb = 3; continue; }
            2 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1423f5dac); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P3)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu32(rbx, ((rax as u32)) as u32); crate::simd_ops::wu8(rbx.wrapping_add(P2), (0x0u8) as u8); bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 4; continue; }
            4 => { fl_l = (rcx) as u64 as u64; fl_r = (rax) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 6; continue; } bb = 5; continue; }
            5 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1423f5dcc); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; bb = 6; continue; }
            6 => { fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 7; continue; }
            7 => { rcx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; crate::simd_ops::wu16(rax, ((rcx as u16)) as u16); bb = 8; continue; }
            8 => { rax = (rbx) as u64; rsp = ((rsp).wrapping_add(0x250u64)) as u64; rbx = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1423f6630(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    crate::simd_ops::wu64(rcx, rdx);
    rax = (crate::simd_ops::ru32(r8)) as u32 as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(P0), ((rax as u32)) as u32);
    rax = rcx;
    xmm0 = (crate::simd_ops::ru128(r9)) as u128;
    crate::simd_ops::wu128(rcx.wrapping_add(P1), (xmm0) as u128);
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1423f69e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0xfffffffffffffffe;
    let (mut rax, mut fl_l, mut fl_r) = (0u64, 0u64, 0u64);
    rax = ((rcx.wrapping_add(P0)) as u32) as u32 as u64;
    fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1;
    if (((fl_l as u32)) <= ((fl_r as u32))) { rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); return rax; }
    fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x7b;
    if !((((fl_l as u32)) == ((fl_r as u32)))) { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; return rax; }
    rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64));
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1423f70d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141371040;
    let (mut rax, mut r11, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    r11 = rsp;
    rsp = ((rsp).wrapping_sub(0x58u64)) as u64;
    fl_l = rdx; fl_r = rdx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rax = (crate::simd_ops::ru64(r8)) as u64; r8 = r11.wrapping_sub(0x38); crate::simd_ops::wu64(r11.wrapping_sub(0x18u64), rax); rax = (crate::simd_ops::ru64(r9)) as u64; crate::simd_ops::wu64(r11.wrapping_sub(0x38u64), rcx); crate::simd_ops::wu64(r11.wrapping_sub(0x30u64), rdx); crate::simd_ops::wu64(r11.wrapping_sub(0x28u64), rcx); rcx = r11.wrapping_add(0x10); crate::simd_ops::wu64(r11.wrapping_sub(0x20u64), rdx); rdx = r11.wrapping_sub(0x28); crate::simd_ops::wu64(r11.wrapping_sub(0x10u64), rax); rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); }
    rsp = ((rsp).wrapping_add(0x58u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1423f7350(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x18;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = (0xffffffffffffffffu64) as u64;
    fl_l = rdx; fl_r = rdx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rax = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; }
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1423f78d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x58; const P1: u64 = 0x142645568;
    let (mut rax, mut fl_l, mut fl_r) = (0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    rcx = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64;
    rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P1)) as u64, rax, rcx, rdx, r8, r9, rsp);
    fl_l = rax; fl_r = rax;
    rax = (rax & !0xffu64) | (((if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { 1u8 } else { 0u8 }) as u8 as u64));
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1423f78f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    crate::simd_ops::forward_incoming_to_outgoing();
    const P0: u64 = 0x58; const P1: u64 = 0x142645570;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rcx = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64;
    rdx = (((rdx as u8)) as u32) as u32 as u64;
    fl_l = (((rdx) ^ (0x1u64))) as u64; fl_r = fl_l;
    rdx = ((fl_l as u64)) as u64;
    crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((crate::simd_ops::ru64(P1)) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1423f7c60(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x1426455a8; const P2: u64 = 0x9d8; const P3: u64 = 0x9a0; const P4: u64 = 0x998; const P5: u64 = 0x9a4; const P6: u64 = 0x990; const P7: u64 = 0x28; const P8: u64 = 0x3a; const P9: u64 = 0x988; const P10: u64 = 0x142645598;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rsi = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; rdi = rcx; r15 = rsi; rcx = ((rcx).wrapping_add(0x988u64)) as u64; r15 = ((r15).wrapping_shr(32u32)) as u64; rbp = r9; r14 = r8; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P1)) as u64, rax, rcx, rdx, r8, r9, rsp); rax = r15; rdx = ((r15 as u32)) as u32 as u64; rax = ((rax).wrapping_shr(24u32)) as u64; rdx = ((((rdx as u32)) & (0xffffffu32))) as u32 as u64; rsi = (((rsi as u32)).wrapping_sub(crate::simd_ops::ru32(rdi.wrapping_add(rax.wrapping_mul(4)).wrapping_add(P2)))) as u32 as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; r10 = ((rdi.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rdx = (((rdx as u32)).wrapping_add((rsi as u32))) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(r10.wrapping_add(P3))) as u32 as u64; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 3; continue; } rax = r15; rax = ((rax).wrapping_shr(24u32)) as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = 0; fl_l = 0; fl_r = 0; fl_l = ((rsi as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rdi.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(P3))) as u32 as u64; rax = (if (((fl_l as u32)) < ((fl_r as u32))) { (rsi as u32) } else { (rax as u32) }) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 3; continue; } fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; rdx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; r8 = ((rax as u32)) as u32 as u64; r9 = ((rdx as u32)) as u32 as u64; bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru32(r10.wrapping_add(P4))) as u32 as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = (crate::simd_ops::ru32(r10.wrapping_add(P5))) as u32 as u64; rax = ((rax).wrapping_add(r8)) as u64; let d = (rcx) as u128; let n = ((rdx as u128) << 64) | (rax as u128); if d != 0 { rax = (n / d) as u64; rdx = (n % d) as u64; } rdx = ((rdx).wrapping_shl(6u32)) as u64; rdx = ((rdx).wrapping_add(crate::simd_ops::ru64(r10.wrapping_add(P6)))) as u64; rax = (crate::simd_ops::ru64(rdx.wrapping_add(P7))) as u64; rax = (((rax) & (r14))) as u64; fl_l = rax; fl_r = rbp; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } crate::simd_ops::wu8(rdx.wrapping_add(P8), (0x1u8) as u8); bb = 2; continue; }
            2 => { r8 = ((r8).wrapping_add(1)) as u64; fl_l = r9; fl_r = (0x1u64) as u64; r9 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } bb = 3; continue; }
            3 => { rcx = ((rdi.wrapping_add(P9)) as u64) as u64; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P10)) as u64, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1423f7d40(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x1426455a8; const P2: u64 = 0x9d8; const P3: u64 = 0x9a0; const P4: u64 = 0x998; const P5: u64 = 0x9a4; const P6: u64 = 0x990; const P7: u64 = 0x3a; const P8: u64 = 0x988; const P9: u64 = 0x142645598;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rsi = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; rdi = rcx; rbp = rsi; rcx = ((rcx).wrapping_add(0x988u64)) as u64; rbp = ((rbp).wrapping_shr(32u32)) as u64; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P1)) as u64, rax, rcx, rdx, r8, r9, rsp); rax = rbp; rdx = ((rbp as u32)) as u32 as u64; rax = ((rax).wrapping_shr(24u32)) as u64; rdx = ((((rdx as u32)) & (0xffffffu32))) as u32 as u64; rsi = (((rsi as u32)).wrapping_sub(crate::simd_ops::ru32(rdi.wrapping_add(rax.wrapping_mul(4)).wrapping_add(P2)))) as u32 as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; r9 = ((rdi.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rdx = (((rdx as u32)).wrapping_add((rsi as u32))) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(r9.wrapping_add(P3))) as u32 as u64; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 2; continue; } rax = rbp; rax = ((rax).wrapping_shr(24u32)) as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = 0; fl_l = 0; fl_r = 0; fl_l = ((rsi as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rdi.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(P3))) as u32 as u64; rax = (if (((fl_l as u32)) < ((fl_r as u32))) { (rsi as u32) } else { (rax as u32) }) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 2; continue; } fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; rdx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; r8 = ((rax as u32)) as u32 as u64; r10 = ((rdx as u32)) as u32 as u64; bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru32(r9.wrapping_add(P4))) as u32 as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = (crate::simd_ops::ru32(r9.wrapping_add(P5))) as u32 as u64; rax = ((rax).wrapping_add(r8)) as u64; let d = (rcx) as u128; let n = ((rdx as u128) << 64) | (rax as u128); if d != 0 { rax = (n / d) as u64; rdx = (n % d) as u64; } rax = (crate::simd_ops::ru64(r9.wrapping_add(P6))) as u64; r8 = ((r8).wrapping_add(1)) as u64; rdx = ((rdx).wrapping_shl(6u32)) as u64; crate::simd_ops::wu8(rdx.wrapping_add(rax).wrapping_add(P7), (0x1u8) as u8); fl_l = r10; fl_r = (0x1u64) as u64; r10 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } bb = 2; continue; }
            2 => { rcx = ((rdi.wrapping_add(P8)) as u64) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk((crate::simd_ops::ru64(P9)) as u64, rax, rcx, rdx, r8, r9, rsp); }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1423f8210(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0xfff; const P2: u64 = 0x142496730; const P3: u64 = 0x20; const P4: u64 = 0x10; const P5: u64 = 0x1423fc340;
    let (mut rax, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x50u64)) as u64; rbp = r9; r14 = rdx; crate::simd_ops::wu32(rsp.wrapping_add(0x40u64), (0x0u32) as u32); rsi = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x90u64))) as u64; rdi = rcx; rdi = (((rdi) & (0xfffffffffffff000u64))) as u64; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x98u64))) as u64; rbx = ((rax.wrapping_add(P1)) as u64) as u64; rbx = ((rbx).wrapping_add(rcx)) as u64; rbx = (((rbx) & (0xfffffffffffff000u64))) as u64; rbx = ((rbx).wrapping_sub(rdi)) as u64; rcx = ((rcx).wrapping_sub(rdi)) as u64; r9 = 0x1000; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rcx); r8 = rbx; rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0xa0u64))) as u64; rcx = r14; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu32(rsp.wrapping_add(0x40u64), (0x3u32) as u32); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xa8u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rax); rax = (crate::simd_ops::ru64(rsi.wrapping_add(P3))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); rax = (crate::simd_ops::ru64(r14.wrapping_add(P4))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbx); r9 = rdi; r8 = (crate::simd_ops::ru64(rbp.wrapping_add(0x8u64))) as u64; rdx = rsi; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rax = r14; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; rsp = ((rsp).wrapping_add(0x50u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1423f8340(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14243b450; const P1: u64 = 0x8; const P2: u64 = 0x20; const P3: u64 = 0x28; const P4: u64 = 0x10; const P5: u64 = 0x18; const P6: u64 = 0x1401f3d90;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbp = rcx; r15 = ((r8 as u32)) as u32 as u64; rcx = ((rcx).wrapping_add(0x60u64)) as u64; r14 = rdx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdi = 0; fl_l = 0; fl_r = 0; rbx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 1; continue; } crate::simd_ops::wu64(rax.wrapping_add(P1), rdi); crate::simd_ops::wu64(rax.wrapping_add(P2), rdi); crate::simd_ops::wu64(rax.wrapping_add(P3), rdi); crate::simd_ops::wu64(rax, rdi); crate::simd_ops::wu64(rax.wrapping_add(P4), rdi); crate::simd_ops::wu64(rax.wrapping_add(P5), rdi); bb = 2; continue; }
            1 => { rbx = rdi; bb = 2; continue; }
            2 => { rcx = rbp.wrapping_add(0x1c0); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rsi = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } xmm0 = 0u128; rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu128(rsi, (xmm0) as u128); crate::simd_ops::wu64(rsi.wrapping_add(P4), rax); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rsi, rax); crate::simd_ops::wu16(rsi.wrapping_add(P1), (0x0u16) as u16); crate::simd_ops::wu32(rsi.wrapping_add(P4), ((rdi as u32)) as u32); bb = 4; continue; }
            3 => { rsi = rdi; bb = 4; continue; }
            4 => { crate::simd_ops::wu64(rbx.wrapping_add(P2), rsi); rax = r14; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; crate::simd_ops::wu32(rbx.wrapping_add(P3), ((r15 as u32)) as u32); crate::simd_ops::wu64(r14, rbp); rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; crate::simd_ops::wu64(r14.wrapping_add(P1), rbx); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1423f8d50(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x1426455b0; const P2: u64 = 0x142645708;
    let (mut rax, mut fl_l, mut fl_r) = (0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P1)) as u64, rax, rcx, rdx, r8, r9, rsp);
    fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64;
    if !(((((fl_l as u32)) & ((fl_r as u32))) == 0)) { rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
    rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P2)) as u64, rax, rcx, rdx, r8, r9, rsp);
    rax = (rax & !0xffu64) | (((0u8) as u8 as u64));
    fl_l = 0; fl_r = 0;
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1423f8f70(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x1426454e8; const P2: u64 = 0x142645708;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    rdx = rsp.wrapping_add(0x38);
    crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), (0x0u64) as u64);
    rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P1)) as u64, rax, rcx, rdx, r8, r9, rsp);
    fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64;
    if !(((((fl_l as u32)) & ((fl_r as u32))) != 0)) { rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P2)) as u64, rax, rcx, rdx, r8, r9, rsp); rax = 0; fl_l = 0; fl_r = 0; rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64;
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1423f90c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1423f9300; const P1: u64 = 0x1426454e0; const P2: u64 = 0x20; const P3: u64 = 0x24; const P4: u64 = 0x142645708; const P5: u64 = 0x142442de0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 81>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x1423f90c0, funcinfo_rva: 0x37ddacc, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0x288, conts: &[] };
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64); rsp = ((rsp).wrapping_sub(0x280u64)) as u64; rbx = (rcx) as u64; xmm0 = (crate::simd_ops::ru128(rdx)) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rsp.wrapping_add(0x50u64), (xmm0) as u128); rdx = ((rsp.wrapping_add(0x50u64)) as u64) as u64; rcx = ((rsp.wrapping_add(0x60u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1423f90e0); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; xmm0 = 0u128; ymm0_hi = 0; rax = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; let y_addr: u64 = rsp.wrapping_add(0x20u64); core::ptr::write_unaligned(y_addr as *mut u128, xmm0); core::ptr::write_unaligned(y_addr.wrapping_add(16) as *mut u128, ymm0_hi); crate::simd_ops::wu32(rsp.wrapping_add(0x40u64), ((rax as u32)) as u32); r8 = ((rsp.wrapping_add(0x20u64)) as u64) as u64; rdx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1423f9104); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P1)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 4; continue; } bb = 1; continue; }
            1 => { let y_addr: u64 = rsp.wrapping_add(0x20u64); xmm0 = core::ptr::read_unaligned(y_addr as *const u128); ymm0_hi = core::ptr::read_unaligned(y_addr.wrapping_add(16) as *const u128); let y_addr: u64 = rbx; core::ptr::write_unaligned(y_addr as *mut u128, xmm0); core::ptr::write_unaligned(y_addr.wrapping_add(16) as *mut u128, ymm0_hi); rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x40u64))) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(P2), ((rax as u32)) as u32); crate::simd_ops::wu8(rbx.wrapping_add(P3), (0x1u8) as u8); rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 2; continue; }
            2 => { fl_l = (rcx) as u64 as u64; fl_r = (rax) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 7; continue; } bb = 3; continue; }
            3 => { bb = 6; continue; }
            4 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1423f913c); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P4)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu32(rbx, ((rax as u32)) as u32); crate::simd_ops::wu8(rbx.wrapping_add(P3), (0x0u8) as u8); rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 5; continue; }
            5 => { fl_l = (rcx) as u64 as u64; fl_r = (rax) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 7; continue; } bb = 6; continue; }
            6 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1423f915c); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; bb = 7; continue; }
            7 => { fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 8; continue; }
            8 => { rcx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; crate::simd_ops::wu16(rax, ((rcx as u16)) as u16); bb = 9; continue; }
            9 => { rax = (rbx) as u64; rsp = ((rsp).wrapping_add(0x280u64)) as u64; rbx = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1423f9d20(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14240ab10;
    let (mut rax, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rcx;
    rcx = ((rcx).wrapping_add(0x260u64)) as u64;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rax = rbx;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1423f9db0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x860; const P1: u64 = 0x11; const P2: u64 = 0x142645540; const P3: u64 = 0xffffffffffffffff; const P4: u64 = 0x1;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; r8 = 0; fl_l = 0; fl_r = 0; rdi = rdx; rsi = rcx; rdx = ((rcx.wrapping_add(P0)) as u64) as u64; r9 = 0xffffffff; rcx = ((r8.wrapping_add(P1)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P2)) as u64, rax, rcx, rdx, r8, r9, rsp); rdx = ((rax as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 1; continue; } rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbx); rbx = ((rax.wrapping_add(P3)) as u32) as u32 as u64; rax = 0x1; rax = (((rax as u32)).wrapping_shl(((((rbx as u32)) as u32) & 31))) as u32 as u64; crate::simd_ops::wu32(rdi, ((rax as u32)) as u32); fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0x10; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 3; continue; } bb = 2; continue; }
            2 => { rcx = 0xf; rax = ((rbx as u32)) as u32 as u64; rax = ((rax).wrapping_add(0x10eu64)) as u64; rcx = (((rcx as u32)).wrapping_sub((rbx as u32))) as u32 as u64; r9 = 0; fl_l = 0; fl_r = 0; r8 = 0; fl_l = 0; fl_r = 0; rdx = ((rsi.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P2)) as u64, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x102; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 3; continue; } rax = (((rax as u32)).wrapping_add(1)) as u32 as u64; rbx = (((rbx as u32)).wrapping_add((rax as u32))) as u32 as u64; rax = (crate::simd_ops::ru32(rdi)) as u32 as u64; fl_l = ((((rax as u32)) >> ((((rbx as u32)) as u32 & 31))) & 1) as u64; rax = (((rax as u32)) | ((1u32) << ((((rbx as u32)) as u32 & 31)))) as u32 as u64; crate::simd_ops::wu32(rdi, ((rax as u32)) as u32); rax = ((rbx.wrapping_add(P4)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x10; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 2; continue; } bb = 3; continue; }
            3 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1423f9e70(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142645548; const P1: u64 = 0x142645708;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0; let mut ymm1_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; r10 = (crate::simd_ops::ru64(rdx)) as u64; rdi = rdx; rax = ((crate::simd_ops::ru8(rdx.wrapping_add(0x39u64))) as u32) as u32 as u64; r9 = r10; rbx = (crate::simd_ops::ru64(rdx.wrapping_add(0x8u64))) as u64; r9 = ((r9).wrapping_shr(12u32)) as u64; rdx = r9; rdx = ((rdx).wrapping_shl(12u32)) as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; rdx = (if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { r10 } else { rdx }) as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 1; continue; } rbx = ((rbx).wrapping_add(0xfffu64)) as u64; rbx = ((rbx).wrapping_add(r10)) as u64; rbx = ((rbx).wrapping_shr(12u32)) as u64; rbx = ((rbx).wrapping_sub(r9)) as u64; fl_l = ((rbx).wrapping_shl(12u32)) as u64; rbx = ((fl_l as u64)) as u64; bb = 1; continue; }
            1 => { let y_addr: u64 = rdi; xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); rax = ((r8 as u32)) as u32 as u64; rbp = (((rax) as i64).wrapping_mul((0x70u64) as i64) as u64) as u64; rax = ((r8.wrapping_add(0x1u64)) as u32) as u32 as u64; rbp = ((rbp).wrapping_add(0x160u64)) as u64; rbp = ((rbp).wrapping_add(rcx)) as u64; let y_addr: u64 = rbp.wrapping_add(0x20u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); let y_addr: u64 = rdi.wrapping_add(0x20u64); xmm1 = crate::simd_ops::ru128(y_addr); ymm1_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rbp.wrapping_add(0x40u64); crate::simd_ops::wu128(y_addr, xmm1); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm1_hi); xmm0 = 0u128; ymm0_hi = 0; let y_addr: u64 = rbp; crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); crate::simd_ops::wu32(rbp.wrapping_add(0x10u64), ((rdx as u32)) as u32); rdx = ((rdx).wrapping_shr(32u32)) as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x14u64), ((rdx as u32)) as u32); rax = (crate::simd_ops::ru64(rcx.wrapping_add(rax.wrapping_mul(8)).wrapping_add(0x860u64))) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x18u64), rax); fl_l = (crate::simd_ops::ru8(rdi.wrapping_add(0x39u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 2; continue; } rcx = ((rcx).wrapping_add(0x950u64)) as u64; r8 = 0x1000; rdx = rbx; rax = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm((crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rbp.wrapping_add(0x60u64), rax); crate::simd_ops::wu64(rbp.wrapping_add(0x68u64), rbx); fl_l = (crate::simd_ops::ru8(rdi.wrapping_add(0x39u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 2; continue; } rdx = (crate::simd_ops::ru64(rbp.wrapping_add(0x60u64))) as u64; bb = 3; continue; }
            2 => { rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0x18u64))) as u64; bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x30u64))) as u64; r9 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbp); r8 = ((rbx as u32)) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm((crate::simd_ops::ru64(P0)) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 4; continue; } (rax, xmm0) = crate::simd_ops::call_stk_xmm((crate::simd_ops::ru64(P1)) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 4; continue; }
            4 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1423fa630(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    crate::simd_ops::forward_incoming_to_outgoing();
    const P0: u64 = 0x8; const P1: u64 = 0x10;
    let (mut rax, mut r10, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    r10 = (crate::simd_ops::ru64(rdx)) as u64;
    rcx = ((rcx).wrapping_add(0x950u64)) as u64;
    fl_l = r10; fl_r = r10;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { r8 = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; r9 = 0x1000; rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = r10; crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1423fa6f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x28; const P1: u64 = 0x8; const P2: u64 = 0x3a;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    r8 = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    r8 = (((r8) & (crate::simd_ops::ru64(rax)))) as u64;
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64;
    fl_l = r8; fl_r = (crate::simd_ops::ru64(rax)) as u64;
    if !((((fl_l as u64)) != ((fl_r as u64)))) { crate::simd_ops::wu8(rdx.wrapping_add(P2), (0x1u8) as u8); }
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1423fa710(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x3a;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    crate::simd_ops::wu8(rdx.wrapping_add(P0), (0x1u8) as u8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1423fa870(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0x18; const P3: u64 = 0x20;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    crate::simd_ops::wu64(rcx, (0x1000u64) as u64);
    rax = rcx;
    crate::simd_ops::wu64(rcx.wrapping_add(P0), (0x200000u64) as u64);
    crate::simd_ops::wu64(rcx.wrapping_add(P1), (0x400000u64) as u64);
    crate::simd_ops::wu64(rcx.wrapping_add(P2), (0x100000u64) as u64);
    crate::simd_ops::wu64(rcx.wrapping_add(P3), (0x2000u64) as u64);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1423faf20(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1423f9300; const P1: u64 = 0x1426454b8; const P2: u64 = 0x4; const P3: u64 = 0x142645708; const P4: u64 = 0x142442de0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 75>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x1423faf20, funcinfo_rva: 0x37dddbc, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0x258, conts: &[] };
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64); rsp = ((rsp).wrapping_sub(0x250u64)) as u64; rbx = (rcx) as u64; xmm0 = (crate::simd_ops::ru128(rdx)) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128); rdx = ((rsp.wrapping_add(0x20u64)) as u64) as u64; rcx = ((rsp.wrapping_add(0x30u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1423faf40); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1423faf4c); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P1)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu8(rbx.wrapping_add(P2), (0x1u8) as u8); bb = 3; continue; }
            2 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1423faf5c); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P3)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu32(rbx, ((rax as u32)) as u32); crate::simd_ops::wu8(rbx.wrapping_add(P2), (0x0u8) as u8); bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 4; continue; }
            4 => { fl_l = (rcx) as u64 as u64; fl_r = (rax) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 6; continue; } bb = 5; continue; }
            5 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1423faf7c); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; bb = 6; continue; }
            6 => { fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 7; continue; }
            7 => { rcx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; crate::simd_ops::wu16(rax, ((rcx as u16)) as u16); bb = 8; continue; }
            8 => { rax = (rbx) as u64; rsp = ((rsp).wrapping_add(0x250u64)) as u64; rbx = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1423fb2c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x18;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1423fb2d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1; const P1: u64 = 0x8; const P2: u64 = 0x142645538; const P3: u64 = 0x18; const P4: u64 = 0x142645708;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rcx = (crate::simd_ops::ru32(rsp.wrapping_add(0x50u64))) as u32 as u64; r10 = r9; r9 = 0; fl_l = 0; fl_r = 0; rdi = r8; rbx = rdx; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 1; continue; } fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x1; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 2; continue; } fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 2; continue; } r9 = ((rcx.wrapping_add(P0)) as u32) as u32 as u64; bb = 2; continue; }
            1 => { r9 = 0x1; bb = 2; continue; }
            2 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(P1))) as u64; r8 = rsp.wrapping_add(0x30); rdx = r10; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (0xffffffffffffffffu64) as u64); rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P2)) as u64, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 3; continue; } rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu64(rdi.wrapping_add(P3), rax); crate::simd_ops::wu64(rbx, rax); rax = rbx; crate::simd_ops::wu8(rbx.wrapping_add(P1), (0x1u8) as u8); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            3 => { rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P4)) as u64, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu32(rbx, ((rax as u32)) as u32); rax = rbx; crate::simd_ops::wu8(rbx.wrapping_add(P1), (0x0u8) as u8); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1423fb540(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x28; const P2: u64 = 0x142645528; const P3: u64 = 0x4; const P4: u64 = 0x142645520; const P5: u64 = 0x142645708;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x50u64)) as u64; rsi = r8; rax = 0; fl_l = 0; fl_r = 0; xmm0 = 0u128; ymm0_hi = 0; rdi = r9; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rax); rbx = rdx; r8 = rsp.wrapping_add(0x20); rcx = (crate::simd_ops::ru64(rsi.wrapping_add(P0))) as u64; r9 = ((rax.wrapping_add(P1)) as u32) as u32 as u64; let y_addr: u64 = rsp.wrapping_add(0x20u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); rdx = 0; fl_l = 0; fl_r = 0; (rax, xmm0) = crate::simd_ops::call_stk_xmm((crate::simd_ops::ru64(P2)) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 1; continue; } rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; r8 = rsp.wrapping_add(0x20); rax = (crate::simd_ops::ru32(rdi)) as u32 as u64; r9 = 0x28; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rax as u32)) as u32); rdx = 0; fl_l = 0; fl_r = 0; rax = (crate::simd_ops::ru32(rdi.wrapping_add(P3))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x24u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rcx)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rcx.wrapping_add(P3))) as u32 as u64; rcx = (crate::simd_ops::ru64(rsi.wrapping_add(P0))) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x34u64), ((rax as u32)) as u32); (rax, xmm0) = crate::simd_ops::call_stk_xmm((crate::simd_ops::ru64(P4)) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 1; continue; } crate::simd_ops::wu8(rbx.wrapping_add(P3), (0x1u8) as u8); rax = rbx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rsp = ((rsp).wrapping_add(0x50u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            1 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm((crate::simd_ops::ru64(P5)) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; crate::simd_ops::wu32(rbx, ((rax as u32)) as u32); rax = rbx; crate::simd_ops::wu8(rbx.wrapping_add(P3), (0x0u8) as u8); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rsp = ((rsp).wrapping_add(0x50u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1423fb600(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1423f9300; const P1: u64 = 0x1426454f8; const P2: u64 = 0x1426455b0; const P3: u64 = 0x142442de0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 79>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x1423fb600, funcinfo_rva: 0x37ddee8, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0x278, conts: &[] };
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rbx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = ((rsp).wrapping_sub(0x270u64)) as u64; rbx = ((rdx as u32)) as u32 as u64; xmm0 = (crate::simd_ops::ru128(rcx)) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm0) as u128); rdx = ((rsp.wrapping_add(0x40u64)) as u64) as u64; rcx = ((rsp.wrapping_add(0x50u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1423fb623); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = (0x3u32) as u32 as u64; rdi = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; fl_l = ((rbx as u32)) as u32 as u64; fl_r = ((rbx as u32)) as u32 as u64; rax = (if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { (rdi as u32) } else { (rax as u32) }) as u32 as u64; rdx = (0x10000u32) as u32 as u64; rdx = (if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { (rdi as u32) } else { (rdx as u32) }) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (rdi) as u64); crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), (0x80u32) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rax as u32)) as u32); r9 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; r8 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1423fb658); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P1)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; fl_l = (rax) as u64 as u64; fl_r = (0xffffffffffffffffu64) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rcx = (rax) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1423fb667); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P2)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rbx = (rbx & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 3; continue; }
            2 => { rbx = (rbx & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 4; continue; }
            4 => { fl_l = (rcx) as u64 as u64; fl_r = (rax) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 6; continue; } bb = 5; continue; }
            5 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1423fb687); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; bb = 6; continue; }
            6 => { fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 7; continue; }
            7 => { crate::simd_ops::wu16(rax, ((rdi as u16)) as u16); bb = 8; continue; }
            8 => { rax = (((rbx as u8)) as u32) as u32 as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x280u64))) as u64; rsp = ((rsp).wrapping_add(0x270u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1423fb800(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1423f9300; const P1: u64 = 0x1426454d8; const P2: u64 = 0x142442de0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 75>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x1423fb800, funcinfo_rva: 0x37ddf4c, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0x258, conts: &[] };
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rbx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = ((rsp).wrapping_sub(0x250u64)) as u64; rbx = (r8) as u64; rdi = (rdx) as u64; xmm0 = (crate::simd_ops::ru128(rcx)) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128); rdx = ((rsp.wrapping_add(0x20u64)) as u64) as u64; rcx = ((rsp.wrapping_add(0x30u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1423fb827); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; r9 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; r8 = ((rsp.wrapping_add(0x260u64)) as u64) as u64; rdx = ((rsp.wrapping_add(0x278u64)) as u64) as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1423fb844); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P1)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x260u64))) as u64; crate::simd_ops::wu64(rdi, (rax) as u64); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x278u64))) as u64; crate::simd_ops::wu64(rbx, (rax) as u64); rbx = (rbx & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 3; continue; }
            2 => { rbx = (rbx & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 4; continue; }
            4 => { fl_l = (rcx) as u64 as u64; fl_r = (rax) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 6; continue; } bb = 5; continue; }
            5 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1423fb87e); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; bb = 6; continue; }
            6 => { fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 7; continue; }
            7 => { rcx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; crate::simd_ops::wu16(rax, ((rcx as u16)) as u16); bb = 8; continue; }
            8 => { rax = (((rbx as u8)) as u32) as u32 as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x268u64))) as u64; rsp = ((rsp).wrapping_add(0x250u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1423fc010(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1434073e0; const P1: u64 = 0x860; const P2: u64 = 0x142645560; const P3: u64 = 0x108; const P4: u64 = 0x142645748; const P5: u64 = 0x950; const P6: u64 = 0x14241c980; const P7: u64 = 0x8f0; const P8: u64 = 0x1403496f0; const P9: u64 = 0xffffffffffffffeb; const P10: u64 = 0x141834ee8; const P11: u64 = 0x120; const P12: u64 = 0x14243b540; const P13: u64 = 0x1423f0c20; const P14: u64 = 0x143406c90; const P15: u64 = 0x1423f0780; const P16: u64 = 0x14136eec0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rbx = rcx; rax = P0; crate::simd_ops::wu64(rcx, rax); rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P2)) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P3))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 1; continue; } rdx = 0xffffffff; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P4)) as u64, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rcx = ((rbx.wrapping_add(P5)) as u64) as u64; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); rcx = ((rbx.wrapping_add(P7)) as u64) as u64; r9 = P8; rdx = 0x18; r8 = ((rdx.wrapping_add(P9)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P10, rax, rcx, rdx, r8, r9, rsp); rcx = ((rbx.wrapping_add(P11)) as u64) as u64; rax = crate::simd_ops::call_stk(P12, rax, rcx, rdx, r8, r9, rsp); rcx = rbx; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(P13, rax, rcx, rdx, r8, r9, rsp); }
            2 => { rdx = P14; rcx = rsp.wrapping_add(0x20); rax = crate::simd_ops::call_stk(P15, rax, rcx, rdx, r8, r9, rsp); rcx = rax; rax = crate::simd_ops::call_stk(P16, rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x1423fc0a7, rcx, rsp); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1423fc920(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1426455a8; const P1: u64 = 0x938; const P2: u64 = 0x900; const P3: u64 = 0x8f8; const P4: u64 = 0x904; const P5: u64 = 0x8f0; const P6: u64 = 0x3a; const P7: u64 = 0x8e8; const P8: u64 = 0x142645598;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rsi = rcx; rbx = rdx; rcx = ((rcx).wrapping_add(0x8e8u64)) as u64; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P0)) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru32(rsp.wrapping_add(0x3cu64))) as u32 as u64; rax = ((rcx as u32)) as u32 as u64; rdx = ((rcx as u32)) as u32 as u64; rax = ((rax).wrapping_shr(24u32)) as u64; rdx = ((((rdx as u32)) & (0xffffffu32))) as u32 as u64; rbx = (((rbx as u32)).wrapping_sub(crate::simd_ops::ru32(rsi.wrapping_add(rax.wrapping_mul(4)).wrapping_add(P1)))) as u32 as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; r9 = ((rsi.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rdx = (((rdx as u32)).wrapping_add((rbx as u32))) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(r9.wrapping_add(P2))) as u32 as u64; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 2; continue; } rax = ((rcx as u32)) as u32 as u64; rax = ((rax).wrapping_shr(24u32)) as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = 0; fl_l = 0; fl_r = 0; fl_l = ((rbx as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rsi.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(P2))) as u32 as u64; rax = (if (((fl_l as u32)) < ((fl_r as u32))) { (rbx as u32) } else { (rax as u32) }) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 2; continue; } fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; rdx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; r8 = ((rax as u32)) as u32 as u64; r10 = ((rdx as u32)) as u32 as u64; bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru32(r9.wrapping_add(P3))) as u32 as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = (crate::simd_ops::ru32(r9.wrapping_add(P4))) as u32 as u64; rax = ((rax).wrapping_add(r8)) as u64; let d = (rcx) as u128; let n = ((rdx as u128) << 64) | (rax as u128); if d != 0 { rax = (n / d) as u64; rdx = (n % d) as u64; } rax = (crate::simd_ops::ru64(r9.wrapping_add(P5))) as u64; r8 = ((r8).wrapping_add(1)) as u64; rdx = ((rdx).wrapping_shl(6u32)) as u64; crate::simd_ops::wu8(rdx.wrapping_add(rax).wrapping_add(P6), (0x1u8) as u8); fl_l = r10; fl_r = (0x1u64) as u64; r10 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } bb = 2; continue; }
            2 => { rcx = ((rsi.wrapping_add(P7)) as u64) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk((crate::simd_ops::ru64(P8)) as u64, rax, rcx, rdx, r8, r9, rsp); }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1423fca00(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1426455a8; const P1: u64 = 0x938; const P2: u64 = 0x900; const P3: u64 = 0x8f8; const P4: u64 = 0x904; const P5: u64 = 0x8f0; const P6: u64 = 0x28; const P7: u64 = 0x3a; const P8: u64 = 0x8e8; const P9: u64 = 0x142645598;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rsi = rcx; rbp = r9; rcx = ((rcx).wrapping_add(0x8e8u64)) as u64; r14 = r8; rbx = rdx; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P0)) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru32(rsp.wrapping_add(0x4cu64))) as u32 as u64; rax = ((rcx as u32)) as u32 as u64; rdx = ((rcx as u32)) as u32 as u64; rax = ((rax).wrapping_shr(24u32)) as u64; rdx = ((((rdx as u32)) & (0xffffffu32))) as u32 as u64; rbx = (((rbx as u32)).wrapping_sub(crate::simd_ops::ru32(rsi.wrapping_add(rax.wrapping_mul(4)).wrapping_add(P1)))) as u32 as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; r10 = ((rsi.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rdx = (((rdx as u32)).wrapping_add((rbx as u32))) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(r10.wrapping_add(P2))) as u32 as u64; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 3; continue; } rax = ((rcx as u32)) as u32 as u64; rax = ((rax).wrapping_shr(24u32)) as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = 0; fl_l = 0; fl_r = 0; fl_l = ((rbx as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rsi.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(P2))) as u32 as u64; rax = (if (((fl_l as u32)) < ((fl_r as u32))) { (rbx as u32) } else { (rax as u32) }) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 3; continue; } fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; rdx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; r8 = ((rax as u32)) as u32 as u64; r9 = ((rdx as u32)) as u32 as u64; bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru32(r10.wrapping_add(P3))) as u32 as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = (crate::simd_ops::ru32(r10.wrapping_add(P4))) as u32 as u64; rax = ((rax).wrapping_add(r8)) as u64; let d = (rcx) as u128; let n = ((rdx as u128) << 64) | (rax as u128); if d != 0 { rax = (n / d) as u64; rdx = (n % d) as u64; } rdx = ((rdx).wrapping_shl(6u32)) as u64; rdx = ((rdx).wrapping_add(crate::simd_ops::ru64(r10.wrapping_add(P5)))) as u64; rax = (crate::simd_ops::ru64(rdx.wrapping_add(P6))) as u64; rax = (((rax) & (r14))) as u64; fl_l = rax; fl_r = rbp; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } crate::simd_ops::wu8(rdx.wrapping_add(P7), (0x1u8) as u8); bb = 2; continue; }
            2 => { r8 = ((r8).wrapping_add(1)) as u64; fl_l = r9; fl_r = (0x1u64) as u64; r9 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } bb = 3; continue; }
            3 => { rcx = ((rsi.wrapping_add(P8)) as u64) as u64; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P9)) as u64, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1423fcae0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x4; const P1: u64 = 0x10; const P2: u64 = 0x8; const P3: u64 = 0xc; const P4: u64 = 0x14113f6e0; const P5: u64 = 0x1; const P6: u64 = 0x20;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0; let mut ymm1_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { r11 = rsp; crate::simd_ops::wu64(r11.wrapping_add(0x8u64), rcx); rsp = ((rsp).wrapping_sub(0x68u64)) as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x90u64))) as u64; r10 = rdx; crate::simd_ops::wu16(rsp.wrapping_add(0x59u64), (0x1u16) as u16); crate::simd_ops::wu64(r11.wrapping_sub(0x48u64), r9); crate::simd_ops::wu64(r11.wrapping_sub(0x18u64), r8); rax = (crate::simd_ops::ru32(rcx)) as u32 as u64; crate::simd_ops::wu64(r11.wrapping_sub(0x40u64), rax); rax = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0xa0u64))) as u64; crate::simd_ops::wu64(r11.wrapping_sub(0x38u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x98u64))) as u64; crate::simd_ops::wu64(r11.wrapping_sub(0x30u64), rax); crate::simd_ops::lk_add_u32(rcx.wrapping_add(P1), (1) as u32); rdx = (crate::simd_ops::ru32(rdx.wrapping_add(P2))) as u32 as u64; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xa8u64))) as u64; crate::simd_ops::wu64(r11.wrapping_sub(0x28u64), rcx); rcx = ((rdx as u32)) as u32 as u64; crate::simd_ops::wu64(r11.wrapping_sub(0x20u64), rax); rax = (crate::simd_ops::ru32(r10.wrapping_add(P3))) as u32 as u64; rcx = ((rcx).wrapping_shl(6u32)) as u64; rax = ((((rax as u32)) & (0x3fffffffu32))) as u32 as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(r10))) as u64; crate::simd_ops::wu8(rsp.wrapping_add(0x58u64), (0x1u8) as u8); fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 1; continue; } crate::simd_ops::wu64(r11.wrapping_add(0x8u64), rcx); r8 = r11.wrapping_sub(0x48); rcx = r10; rdx = r11.wrapping_add(0x8); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rsp = ((rsp).wrapping_add(0x68u64)) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            1 => { let y_addr: u64 = rsp.wrapping_add(0x20u64); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rsp.wrapping_add(0x40u64); xmm1 = crate::simd_ops::ru128(y_addr); ymm1_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); rax = ((rdx.wrapping_add(P5)) as u32) as u32 as u64; crate::simd_ops::wu32(r10.wrapping_add(P2), ((rax as u32)) as u32); let y_addr: u64 = rcx; crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); let y_addr: u64 = rcx.wrapping_add(P6); crate::simd_ops::wu128(y_addr, xmm1); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm1_hi); rsp = ((rsp).wrapping_add(0x68u64)) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1423fcc70(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143428830; const P1: u64 = 0x143428240;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut xmm4: u128 = 0; let mut xmm5: u128 = 0; let mut xmm6: u128 = 0; let mut xmm7: u128 = 0; let mut xmm8: u128 = 0; let mut xmm9: u128 = 0; let mut xmm10: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = rsp; rsp = ((rsp).wrapping_sub(0x58u64)) as u64; xmm1 = crate::simd_ops::vcvtpd2ps(crate::simd_ops::ru128(rcx.wrapping_add(0x250u64))); xmm0 = crate::simd_ops::vcvtpd2ps(crate::simd_ops::ru128(rcx.wrapping_add(0x260u64))); xmm2 = crate::simd_ops::vcvtpd2ps(crate::simd_ops::ru128(rcx.wrapping_add(0x270u64))); crate::simd_ops::wu128(rax.wrapping_sub(0x18u64), (xmm6) as u128); xmm6 = (crate::simd_ops::ru128(P0)) as u128; crate::simd_ops::wu128(rax.wrapping_sub(0x28u64), (xmm7) as u128); xmm7 = (crate::simd_ops::ru128(P1)) as u128; crate::simd_ops::wu128(rax.wrapping_sub(0x38u64), (xmm8) as u128); crate::simd_ops::wu128(rax.wrapping_sub(0x48u64), (xmm9) as u128); crate::simd_ops::wu128(rsp, (xmm10) as u128); xmm10 = crate::simd_ops::vshufps(xmm1, xmm0, 0x44u8); xmm0 = crate::simd_ops::vcvtpd2ps(crate::simd_ops::ru128(rcx.wrapping_add(0x280u64))); xmm1 = crate::simd_ops::vcvtpd2ps(crate::simd_ops::ru128(rcx.wrapping_add(0x290u64))); xmm8 = crate::simd_ops::vshufps(xmm2, xmm0, 0x44u8); xmm0 = crate::simd_ops::vcvtpd2ps(crate::simd_ops::ru128(rcx.wrapping_add(0x2a0u64))); xmm5 = crate::simd_ops::vshufps(xmm1, xmm0, 0x44u8); xmm0 = crate::simd_ops::vcvtpd2ps(crate::simd_ops::ru128(rcx.wrapping_add(0x2c0u64))); xmm2 = crate::simd_ops::vcvtpd2ps(crate::simd_ops::ru128(rcx.wrapping_add(0x2b0u64))); xmm4 = crate::simd_ops::vshufps(xmm2, xmm0, 0x44u8); xmm0 = 0u128; xmm9 = crate::simd_ops::vmulps(xmm7, xmm4); xmm3 = (xmm9) as u128; xmm3 = crate::simd_ops::vfmadd_ps(xmm0, xmm5, xmm3, false, false); xmm9 = crate::simd_ops::vfmadd_ps(xmm7, xmm5, xmm9, false, false); xmm2 = (xmm3) as u128; xmm2 = crate::simd_ops::vfmadd_ps(xmm7, xmm8, xmm2, false, false); xmm3 = crate::simd_ops::vfmadd_ps(xmm6, xmm8, xmm3, false, false); xmm1 = (xmm2) as u128; xmm1 = crate::simd_ops::vfmadd_ps(xmm6, xmm10, xmm1, false, false); xmm0 = crate::simd_ops::vpermilps(xmm1, 0xffu8); xmm1 = crate::simd_ops::vdivps(xmm1, xmm0); crate::simd_ops::wu128(rdx, (xmm1) as u128); xmm2 = crate::simd_ops::vfmadd_ps(xmm7, xmm10, xmm2, false, false); xmm0 = crate::simd_ops::vpermilps(xmm2, 0xffu8); xmm1 = crate::simd_ops::vdivps(xmm2, xmm0); crate::simd_ops::wu128(rdx.wrapping_add(0x10u64), (xmm1) as u128); xmm1 = (xmm3) as u128; xmm1 = crate::simd_ops::vfmadd_ps(xmm7, xmm10, xmm1, false, false); xmm0 = crate::simd_ops::vpermilps(xmm1, 0xffu8); xmm1 = crate::simd_ops::vdivps(xmm1, xmm0); crate::simd_ops::wu128(rdx.wrapping_add(0x20u64), (xmm1) as u128); xmm3 = crate::simd_ops::vfmadd_ps(xmm6, xmm10, xmm3, false, false); xmm0 = crate::simd_ops::vpermilps(xmm3, 0xffu8); xmm1 = crate::simd_ops::vdivps(xmm3, xmm0); crate::simd_ops::wu128(rdx.wrapping_add(0x30u64), (xmm1) as u128); xmm1 = (xmm9) as u128; xmm1 = crate::simd_ops::vfmadd_ps(xmm7, xmm8, xmm1, false, false); xmm1 = crate::simd_ops::vfmadd_ps(xmm6, xmm10, xmm1, false, false); xmm0 = crate::simd_ops::vpermilps(xmm1, 0xffu8); xmm1 = crate::simd_ops::vdivps(xmm1, xmm0); crate::simd_ops::wu128(rdx.wrapping_add(0x40u64), (xmm1) as u128); xmm9 = crate::simd_ops::vfmadd_ps(xmm6, xmm8, xmm9, false, false); xmm1 = crate::simd_ops::vmulps(xmm7, xmm4); xmm1 = crate::simd_ops::vfmadd_ps(xmm7, xmm5, xmm1, false, false); xmm1 = crate::simd_ops::vfmadd_ps(xmm7, xmm8, xmm1, false, false); xmm8 = (crate::simd_ops::ru128(rax.wrapping_sub(0x38u64))) as u128; xmm1 = crate::simd_ops::vfmadd_ps(xmm7, xmm10, xmm1, false, false); xmm0 = (xmm7) as u128; xmm0 = crate::simd_ops::vpermilps(xmm1, 0xffu8); xmm1 = crate::simd_ops::vdivps(xmm1, xmm0); crate::simd_ops::wu128(rdx.wrapping_add(0x50u64), (xmm1) as u128); xmm1 = (xmm9) as u128; xmm1 = crate::simd_ops::vfmadd_ps(xmm7, xmm10, xmm1, false, false); xmm7 = (crate::simd_ops::ru128(rax.wrapping_sub(0x28u64))) as u128; xmm9 = crate::simd_ops::vfmadd_ps(xmm6, xmm10, xmm9, false, false); xmm6 = (crate::simd_ops::ru128(rax.wrapping_sub(0x18u64))) as u128; xmm10 = (crate::simd_ops::ru128(rsp)) as u128; xmm0 = crate::simd_ops::vpermilps(xmm1, 0xffu8); xmm1 = crate::simd_ops::vdivps(xmm1, xmm0); xmm0 = crate::simd_ops::vpermilps(xmm9, 0xffu8); crate::simd_ops::wu128(rdx.wrapping_add(0x60u64), (xmm1) as u128); xmm1 = crate::simd_ops::vdivps(xmm9, xmm0); xmm9 = (crate::simd_ops::ru128(rax.wrapping_sub(0x48u64))) as u128; crate::simd_ops::wu128(rdx.wrapping_add(0x70u64), (xmm1) as u128); rsp = ((rsp).wrapping_add(0x58u64)) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1423fcfa0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xffffffffffffffe8; const P1: u64 = 0xffffffffffffffd8; const P2: u64 = 0xffffffffffffffc8; const P3: u64 = 0xffffffffffffffb8; const P4: u64 = 0xffffffffffffffa8; const P5: u64 = 0xffffffffffffff98; const P6: u64 = 0x1423fcc70; const P7: u64 = 0x1434286e0; const P8: u64 = 0x1434289c0; const P9: u64 = 0xc; const P10: u64 = 0x8; const P11: u64 = 0x14;
    let (mut rax, mut r11, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut xmm4: u128 = 0; let mut xmm5: u128 = 0; let mut xmm6: u128 = 0; let mut xmm7: u128 = 0; let mut xmm8: u128 = 0; let mut xmm9: u128 = 0; let mut xmm10: u128 = 0; let mut xmm11: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 33>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = rsp; rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x100u64)) as u64; crate::simd_ops::wu128(rax.wrapping_add(P0), (xmm6) as u128); rbx = rdx; crate::simd_ops::wu128(rax.wrapping_add(P1), (xmm7) as u128); rdx = rsp.wrapping_add(0x20); crate::simd_ops::wu128(rax.wrapping_add(P2), (xmm8) as u128); crate::simd_ops::wu128(rax.wrapping_add(P3), (xmm9) as u128); crate::simd_ops::wu128(rax.wrapping_add(P4), (xmm10) as u128); crate::simd_ops::wu128(rax.wrapping_add(P5), (xmm11) as u128); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm11 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; xmm1 = crate::simd_ops::vminps(xmm11, crate::simd_ops::ru128(P7)); xmm9 = (crate::simd_ops::ru128(rsp.wrapping_add(0x30u64))) as u128; xmm8 = (crate::simd_ops::ru128(rsp.wrapping_add(0x50u64))) as u128; xmm7 = (crate::simd_ops::ru128(rsp.wrapping_add(0x60u64))) as u128; xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x70u64))) as u128; xmm3 = (crate::simd_ops::ru128(rsp.wrapping_add(0x40u64))) as u128; xmm5 = (crate::simd_ops::ru128(rsp.wrapping_add(0x80u64))) as u128; xmm4 = (crate::simd_ops::ru128(rsp.wrapping_add(0x90u64))) as u128; xmm2 = crate::simd_ops::vminps(xmm9, xmm1); xmm0 = crate::simd_ops::vminps(xmm3, xmm2); r11 = rsp.wrapping_add(0x100); xmm1 = crate::simd_ops::vminps(xmm8, xmm0); xmm2 = crate::simd_ops::vminps(xmm7, xmm1); xmm0 = crate::simd_ops::vminps(xmm6, xmm2); xmm1 = crate::simd_ops::vminps(xmm5, xmm0); xmm10 = crate::simd_ops::vminps(xmm4, xmm1); xmm1 = crate::simd_ops::vmaxps(xmm11, crate::simd_ops::ru128(P8)); xmm11 = (crate::simd_ops::ru128(r11.wrapping_sub(0x60u64))) as u128; xmm2 = crate::simd_ops::vmaxps(xmm9, xmm1); xmm9 = (crate::simd_ops::ru128(r11.wrapping_sub(0x40u64))) as u128; xmm3 = crate::simd_ops::vmaxps(xmm3, xmm2); xmm0 = crate::simd_ops::vmaxps(xmm8, xmm3); xmm8 = (crate::simd_ops::ru128(r11.wrapping_sub(0x30u64))) as u128; xmm1 = crate::simd_ops::vmaxps(xmm7, xmm0); xmm7 = (crate::simd_ops::ru128(r11.wrapping_sub(0x20u64))) as u128; xmm2 = crate::simd_ops::vmaxps(xmm6, xmm1); xmm6 = (crate::simd_ops::ru128(r11.wrapping_sub(0x10u64))) as u128; crate::simd_ops::wu64(rbx, ((xmm10 as u64)) as u64); xmm3 = crate::simd_ops::vmaxps(xmm5, xmm2); xmm1 = crate::simd_ops::vmaxps(xmm4, xmm3); crate::simd_ops::wu64(rbx.wrapping_add(P9), ((xmm1 as u64)) as u64); crate::simd_ops::wu32(rbx.wrapping_add(P10), (((xmm10 >> 64) as u32) as u32) as u32); xmm10 = (crate::simd_ops::ru128(r11.wrapping_sub(0x50u64))) as u128; crate::simd_ops::wu32(rbx.wrapping_add(P11), (((xmm1 >> 64) as u32) as u32) as u32); rsp = r11; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1423fd0b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1434277a0; const P1: u64 = 0x14330bca8; const P2: u64 = 0x143428360;
    let (mut rax, mut fl_l, mut fl_r, mut rbp) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0; let mut ymm1_hi: u128 = 0; let mut xmm4: u128 = 0; let mut xmm5: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 17>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rbp = rsp; rsp = ((rsp).wrapping_sub(0x80u64)) as u64; xmm1 = (crate::simd_ops::ru128(P0)) as u128; ymm1_hi = 0; crate::simd_ops::wu128(rbp.wrapping_sub(0x60u64), (xmm1) as u128); crate::simd_ops::wu128(rbp.wrapping_sub(0x40u64), (xmm1) as u128); xmm3 = (xmm1) as u128; xmm5 = 0u128; crate::simd_ops::wu64(rbp.wrapping_sub(0x68u64), ((xmm5 as u64)) as u64); rdx = (0x3ff0000000000000u64) as u64; rax = P1; crate::simd_ops::wu64(rcx, rax); rax = 0; fl_l = 0; fl_r = 0; xmm4 = 0u128; crate::simd_ops::wu128(rbp.wrapping_sub(0x30u64), (xmm4) as u128); crate::simd_ops::wu64(rcx.wrapping_add(0x10u64), rdx); crate::simd_ops::wu64(rcx.wrapping_add(0x18u64), rax); crate::simd_ops::wu64(rcx.wrapping_add(0x20u64), rax); xmm0 = (crate::simd_ops::ru128(rbp.wrapping_sub(0x68u64))) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rcx.wrapping_add(0x28u64), (xmm0) as u128); xmm0 = (crate::simd_ops::ru128(rbp.wrapping_sub(0x38u64))) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rcx.wrapping_add(0x58u64), (xmm0) as u128); crate::simd_ops::wu64(rcx.wrapping_add(0x38u64), (((xmm1 >> 64) as u64)) as u64); crate::simd_ops::wu64(rcx.wrapping_add(0x68u64), (((xmm4 >> 64) as u64)) as u64); crate::simd_ops::wu128(rbp.wrapping_sub(0x30u64), (xmm5) as u128); crate::simd_ops::wu64(rbp.wrapping_sub(0x68u64), ((xmm5 as u64)) as u64); xmm2 = 0u128; crate::simd_ops::wu128(rcx.wrapping_add(0x40u64), (xmm2) as u128); xmm2 = (crate::simd_ops::ru128(P0)) as u128; crate::simd_ops::wu64(rcx.wrapping_add(0x50u64), ((xmm3 as u64)) as u64); crate::simd_ops::wu128(rbp.wrapping_sub(0x60u64), (xmm2) as u128); crate::simd_ops::wu128(rbp.wrapping_sub(0x40u64), (xmm2) as u128); crate::simd_ops::wu64(rcx.wrapping_add(0x70u64), rdx); crate::simd_ops::wu64(rcx.wrapping_add(0x78u64), rax); crate::simd_ops::wu64(rcx.wrapping_add(0x80u64), rax); xmm0 = (crate::simd_ops::ru128(rbp.wrapping_sub(0x68u64))) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rcx.wrapping_add(0x88u64), (xmm0) as u128); xmm0 = (crate::simd_ops::ru128(rbp.wrapping_sub(0x38u64))) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rcx.wrapping_add(0xb8u64), (xmm0) as u128); crate::simd_ops::wu64(rcx.wrapping_add(0x98u64), (((xmm2 >> 64) as u64)) as u64); crate::simd_ops::wu64(rcx.wrapping_add(0xc8u64), (((xmm5 >> 64) as u64)) as u64); xmm0 = 0u128; ymm0_hi = 0; crate::simd_ops::wu128(rbp.wrapping_sub(0x50u64), (xmm0) as u128); xmm0 = (crate::simd_ops::ru128(P0)) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rbp.wrapping_sub(0x30u64), (xmm0) as u128); xmm3 = 0u128; crate::simd_ops::wu128(rcx.wrapping_add(0xa0u64), (xmm3) as u128); xmm1 = 0u128; ymm1_hi = 0; crate::simd_ops::wu128(rbp.wrapping_sub(0x20u64), (xmm1) as u128); crate::simd_ops::wu128(rbp.wrapping_sub(0x40u64), (xmm1) as u128); xmm4 = (xmm2) as u128; xmm2 = (crate::simd_ops::ru128(P2)) as u128; crate::simd_ops::wu64(rcx.wrapping_add(0xb0u64), ((xmm4 as u64)) as u64); crate::simd_ops::wu128(rbp.wrapping_sub(0x60u64), (xmm2) as u128); crate::simd_ops::wu128(rbp.wrapping_sub(0x10u64), (xmm2) as u128); crate::simd_ops::wu64(rcx.wrapping_add(0xd0u64), rdx); crate::simd_ops::wu64(rcx.wrapping_add(0xd8u64), rax); crate::simd_ops::wu64(rcx.wrapping_add(0xe0u64), rax); crate::simd_ops::wu64(rcx.wrapping_add(0xe8u64), rax); let y_addr: u64 = rbp.wrapping_sub(0x40u64); xmm1 = crate::simd_ops::ru128(y_addr); ymm1_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rcx.wrapping_add(0x110u64); crate::simd_ops::wu128(y_addr, xmm1); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm1_hi); xmm1 = (crate::simd_ops::ru128(P2)) as u128; ymm1_hi = 0; xmm0 = (xmm2) as u128; ymm0_hi = 0; let y_addr: u64 = rbp.wrapping_sub(0x60u64); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rcx.wrapping_add(0xf0u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); let y_addr: u64 = rbp.wrapping_sub(0x20u64); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rcx.wrapping_add(0x130u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); crate::simd_ops::wu128(rbp.wrapping_sub(0x60u64), (xmm1) as u128); xmm0 = 0u128; ymm0_hi = 0; crate::simd_ops::wu128(rbp.wrapping_sub(0x50u64), (xmm0) as u128); xmm0 = (crate::simd_ops::ru128(P0)) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rbp.wrapping_sub(0x30u64), (xmm0) as u128); xmm0 = (crate::simd_ops::ru128(P2)) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rbp.wrapping_sub(0x10u64), (xmm0) as u128); xmm1 = 0u128; ymm1_hi = 0; crate::simd_ops::wu128(rbp.wrapping_sub(0x40u64), (xmm1) as u128); crate::simd_ops::wu128(rbp.wrapping_sub(0x20u64), (xmm1) as u128); crate::simd_ops::wu64(rcx.wrapping_add(0x150u64), rdx); crate::simd_ops::wu64(rcx.wrapping_add(0x158u64), rax); crate::simd_ops::wu64(rcx.wrapping_add(0x160u64), rax); crate::simd_ops::wu64(rcx.wrapping_add(0x168u64), rax); let y_addr: u64 = rbp.wrapping_sub(0x60u64); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rbp.wrapping_sub(0x40u64); xmm1 = crate::simd_ops::ru128(y_addr); ymm1_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rcx.wrapping_add(0x170u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); let y_addr: u64 = rbp.wrapping_sub(0x20u64); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rcx.wrapping_add(0x1b0u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); let y_addr: u64 = rcx.wrapping_add(0x190u64); crate::simd_ops::wu128(y_addr, xmm1); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm1_hi); xmm1 = (crate::simd_ops::ru128(P2)) as u128; ymm1_hi = 0; crate::simd_ops::wu128(rbp.wrapping_sub(0x60u64), (xmm1) as u128); xmm0 = 0u128; ymm0_hi = 0; crate::simd_ops::wu128(rbp.wrapping_sub(0x50u64), (xmm0) as u128); xmm0 = (crate::simd_ops::ru128(P0)) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rbp.wrapping_sub(0x30u64), (xmm0) as u128); xmm0 = (crate::simd_ops::ru128(P2)) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rbp.wrapping_sub(0x10u64), (xmm0) as u128); xmm1 = 0u128; ymm1_hi = 0; crate::simd_ops::wu128(rbp.wrapping_sub(0x20u64), (xmm1) as u128); crate::simd_ops::wu128(rbp.wrapping_sub(0x40u64), (xmm1) as u128); crate::simd_ops::wu64(rcx.wrapping_add(0x1d0u64), rdx); crate::simd_ops::wu64(rcx.wrapping_add(0x1d8u64), rax); crate::simd_ops::wu64(rcx.wrapping_add(0x1e0u64), rax); crate::simd_ops::wu64(rcx.wrapping_add(0x1e8u64), rax); let y_addr: u64 = rbp.wrapping_sub(0x40u64); xmm1 = crate::simd_ops::ru128(y_addr); ymm1_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rbp.wrapping_sub(0x60u64); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rcx.wrapping_add(0x1f0u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); let y_addr: u64 = rbp.wrapping_sub(0x20u64); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rcx.wrapping_add(0x230u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); let y_addr: u64 = rcx.wrapping_add(0x210u64); crate::simd_ops::wu128(y_addr, xmm1); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm1_hi); xmm1 = (crate::simd_ops::ru128(P2)) as u128; ymm1_hi = 0; crate::simd_ops::wu128(rbp.wrapping_sub(0x60u64), (xmm1) as u128); xmm0 = 0u128; ymm0_hi = 0; crate::simd_ops::wu128(rbp.wrapping_sub(0x50u64), (xmm0) as u128); xmm0 = (crate::simd_ops::ru128(P0)) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rbp.wrapping_sub(0x30u64), (xmm0) as u128); xmm0 = (crate::simd_ops::ru128(P2)) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rbp.wrapping_sub(0x10u64), (xmm0) as u128); xmm1 = 0u128; ymm1_hi = 0; crate::simd_ops::wu128(rbp.wrapping_sub(0x20u64), (xmm1) as u128); crate::simd_ops::wu128(rbp.wrapping_sub(0x40u64), (xmm1) as u128); crate::simd_ops::wu64(rcx.wrapping_add(0x250u64), rdx); crate::simd_ops::wu64(rcx.wrapping_add(0x258u64), rax); crate::simd_ops::wu64(rcx.wrapping_add(0x260u64), rax); crate::simd_ops::wu64(rcx.wrapping_add(0x268u64), rax); rax = rcx; let y_addr: u64 = rbp.wrapping_sub(0x60u64); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rbp.wrapping_sub(0x40u64); xmm1 = crate::simd_ops::ru128(y_addr); ymm1_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rcx.wrapping_add(0x270u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); let y_addr: u64 = rbp.wrapping_sub(0x20u64); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rcx.wrapping_add(0x2b0u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); let y_addr: u64 = rcx.wrapping_add(0x290u64); crate::simd_ops::wu128(y_addr, xmm1); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm1_hi); crate::simd_ops::wu32(rcx.wrapping_add(0x2d0u64), (0x3f800000u32) as u32); rsp = ((rsp).wrapping_add(0x80u64)) as u64; rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1423fd3c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1d0; const P1: u64 = 0x1e0; const P2: u64 = 0x1f0; const P3: u64 = 0x200; const P4: u64 = 0x210; const P5: u64 = 0x220; const P6: u64 = 0x230; const P7: u64 = 0x240; const P8: u64 = 0x143428760; const P9: u64 = 0x180; const P10: u64 = 0x10; const P11: u64 = 0x143428460; const P12: u64 = 0x143428120;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut xmm4: u128 = 0; let mut xmm5: u128 = 0; let mut xmm6: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 3>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x18u64)) as u64; xmm1 = crate::simd_ops::vcvtpd2ps(crate::simd_ops::ru128(rcx.wrapping_add(P0))); xmm0 = crate::simd_ops::vcvtpd2ps(crate::simd_ops::ru128(rcx.wrapping_add(P1))); xmm2 = crate::simd_ops::vcvtpd2ps(crate::simd_ops::ru128(rcx.wrapping_add(P2))); crate::simd_ops::wu128(rsp, (xmm6) as u128); rax = 0; fl_l = 0; fl_r = 0; xmm6 = crate::simd_ops::vshufps(xmm1, xmm0, 0x44u8); xmm0 = crate::simd_ops::vcvtpd2ps(crate::simd_ops::ru128(rcx.wrapping_add(P3))); xmm1 = crate::simd_ops::vcvtpd2ps(crate::simd_ops::ru128(rcx.wrapping_add(P4))); xmm4 = crate::simd_ops::vshufps(xmm2, xmm0, 0x44u8); xmm0 = crate::simd_ops::vcvtpd2ps(crate::simd_ops::ru128(rcx.wrapping_add(P5))); xmm2 = crate::simd_ops::vcvtpd2ps(crate::simd_ops::ru128(rcx.wrapping_add(P6))); xmm5 = crate::simd_ops::vshufps(xmm1, xmm0, 0x44u8); xmm0 = crate::simd_ops::vcvtpd2ps(crate::simd_ops::ru128(rcx.wrapping_add(P7))); xmm1 = crate::simd_ops::vshufps(xmm2, xmm0, 0x44u8); xmm2 = crate::simd_ops::vshufps(xmm5, xmm1, 0xeeu8); xmm0 = crate::simd_ops::vshufps(xmm5, xmm1, 0x44u8); xmm3 = crate::simd_ops::vshufps(xmm6, xmm4, 0x44u8); xmm1 = crate::simd_ops::vshufps(xmm3, xmm0, 0x88u8); xmm5 = crate::simd_ops::vshufps(xmm3, xmm0, 0xddu8); xmm4 = crate::simd_ops::vshufps(xmm6, xmm4, 0xeeu8); xmm3 = crate::simd_ops::vshufps(xmm4, xmm2, 0xddu8); xmm0 = crate::simd_ops::vaddps(xmm3, xmm1); xmm6 = crate::simd_ops::vshufps(xmm4, xmm2, 0x88u8); xmm2 = (crate::simd_ops::ru128(P8)) as u128; rcx = ((rdx.wrapping_add(P9)) as u64) as u64; crate::simd_ops::wu32(rcx, (0x6u32) as u32); crate::simd_ops::wu128(rdx, (xmm0) as u128); xmm0 = (xmm1) ^ (xmm2); xmm1 = crate::simd_ops::vaddps(xmm0, xmm3); xmm0 = (xmm5) ^ (xmm2); crate::simd_ops::wu128(rdx.wrapping_add(P10), (xmm1) as u128); xmm1 = crate::simd_ops::vaddps(xmm0, xmm3); xmm0 = crate::simd_ops::vaddps(xmm3, xmm5); xmm5 = (crate::simd_ops::ru128(P11)) as u128; crate::simd_ops::wu128(rdx.wrapping_add(0x30u64), (xmm0) as u128); xmm0 = (xmm6) ^ (xmm2); crate::simd_ops::wu128(rdx.wrapping_add(0x20u64), (xmm1) as u128); xmm1 = crate::simd_ops::vaddps(xmm0, xmm3); crate::simd_ops::wu128(rdx.wrapping_add(0x40u64), (xmm6) as u128); xmm6 = (crate::simd_ops::ru128(P12)) as u128; crate::simd_ops::wu128(rdx.wrapping_add(0x50u64), (xmm1) as u128); bb = 1; continue; }
            1 => { xmm4 = (crate::simd_ops::ru128(rdx)) as u128; xmm0 = crate::simd_ops::vdpps(xmm4, xmm4, 0x7fu8); xmm2 = crate::simd_ops::vrsqrtps(xmm0); xmm0 = crate::simd_ops::vmulps(xmm0, xmm2); xmm1 = crate::simd_ops::vmulps(xmm0, xmm2); xmm3 = crate::simd_ops::vsubps(xmm5, xmm1); xmm2 = crate::simd_ops::vmulps(xmm6, xmm2); xmm0 = crate::simd_ops::vmulps(xmm3, xmm2); xmm1 = crate::simd_ops::vmulps(xmm0, xmm4); crate::simd_ops::wu128(rdx, (xmm1) as u128); rax = (((rax as u32)).wrapping_add(1)) as u32 as u64; rdx = rdx.wrapping_add(0x10); fl_l = ((rax as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rcx)) as u32 as u64; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 1; continue; } xmm6 = (crate::simd_ops::ru128(rsp)) as u128; rsp = ((rsp).wrapping_add(0x18u64)) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1423fda20(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    crate::simd_ops::wu64(rdx, (0x0u64) as u64);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1423fda50(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    crate::simd_ops::forward_incoming_to_outgoing();
    const P0: u64 = 0xfffffffffffffff8; const P1: u64 = 0xfffffffffffffff0; const P2: u64 = 0x142442c80;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    fl_l = rdx; fl_r = rdx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { r8 = 0; fl_l = 0; fl_r = 0; rcx = ((r8 as u32)) as u32 as u64; fl_l = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { rcx = rdx; crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8(P2, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); } loop { rax = rcx; rcx = ((rcx).wrapping_add(1)) as u64; rax = (((rax) as i64).wrapping_mul((crate::simd_ops::ru64(rdx.wrapping_add(P1))) as i64) as u64) as u64; crate::simd_ops::wu64(rax.wrapping_add(rdx), r8); rax = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64; fl_l = rcx; fl_r = rax; if !((((fl_l as u64)) < ((fl_r as u64)))) { break; } } rcx = rdx; crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8(P2, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1423fdc40(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (0u64, 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } r9 = ((crate::simd_ops::ru8(rdx)) as u32) as u32 as u64; r8 = 0; fl_l = 0; fl_r = 0; fl_l = ((r9 as u8)) as u8 as u64; fl_r = ((r9 as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 2; continue; } fl_l = rdx; fl_r = rcx; rdx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; bb = 1; continue; }
            1 => { rax = ((r8 as u32)) as u32 as u64; r8 = (((r8 as u32)).wrapping_add(1)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x7fe; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 3; continue; } crate::simd_ops::wu8(rcx, ((r9 as u8)) as u8); rcx = ((rcx).wrapping_add(1)) as u64; r9 = ((crate::simd_ops::ru8(rdx.wrapping_add(rcx))) as u32) as u32 as u64; fl_l = ((r9 as u8)) as u8 as u64; fl_r = ((r9 as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 1; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::wu8(rcx, (0x0u8) as u8); rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); return rax; }
            3 => { crate::simd_ops::wu8(rcx, (0x0u8) as u8); bb = 4; continue; }
            4 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1423fdcf0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1426469d8; const P1: u64 = 0x2;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rsi = rcx; rdi = rdx; fl_l = rsi; fl_r = rdx; rsi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; bb = 1; continue; }
            1 => { rcx = ((crate::simd_ops::ru16(rsi.wrapping_add(rdi))) as u32) as u32 as u64; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P0)) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = ((crate::simd_ops::ru16(rdi)) as u32) as u32 as u64; rdi = ((rdi.wrapping_add(P1)) as u64) as u64; rbx = (((rax as u16)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P0)) as u64, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rbx as u16)) as u16 as u64; fl_r = ((rbx as u16)) as u16 as u64; if ((((fl_l as u16)) & ((fl_r as u16))) == 0) { bb = 2; continue; } fl_l = ((rbx as u16)) as u16 as u64; fl_r = ((rax as u16)) as u16 as u64; if (((fl_l as u16)) == ((fl_r as u16))) { bb = 1; continue; } bb = 2; continue; }
            2 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rcx = (((rax as u16)) as u32) as u32 as u64; rax = ((rbx as u32)) as u32 as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = (((rax as u32)).wrapping_sub((rcx as u32))) as u32 as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1423fddd0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x20000; const P1: u64 = 0x1000; const P2: u64 = 0x1423fdd60;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), r8); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r9); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = ((crate::exe::tls_pointer() as u64)) as u64; r10 = rcx; rcx = 0x6312; r11 = 0x463d0; r9 = 0x83460; r8 = (crate::simd_ops::ru64(rax)) as u64; fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(r8))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 1; continue; } rdx = 0x463d8; rax = 0x4b460; fl_l = ((rax).wrapping_add(r8)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; crate::simd_ops::wu8(rcx.wrapping_add(r8), (0x1u8) as u8); crate::simd_ops::wu64(rdx.wrapping_add(r8), rax); rax = ((r8.wrapping_add(r9)) as u64) as u64; crate::simd_ops::wu64(r8.wrapping_add(r11), rax); bb = 2; continue; }
            1 => { rax = ((r8.wrapping_add(r9)) as u64) as u64; bb = 2; continue; }
            2 => { rbx = (crate::simd_ops::ru64(r8.wrapping_add(r11))) as u64; rcx = ((r9.wrapping_add(P0)) as u64) as u64; rcx = ((rcx).wrapping_add(r8)) as u64; rdx = ((rbx.wrapping_add(P1)) as u64) as u64; crate::simd_ops::wu64(r8.wrapping_add(r11), rdx); fl_l = rdx; fl_r = rcx; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 3; continue; } crate::simd_ops::wu64(r8.wrapping_add(r11), rax); bb = 3; continue; }
            3 => { r9 = rsp.wrapping_add(0x38); r8 = r10; rcx = rbx; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rax = rbx; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1423fde80(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x1427e54e0; const P1: u64 = 0x1;
    let (mut rax, mut r10, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { r9 = rcx; r10 = P0; fl_l = r9; fl_r = rdx; r9 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; r8 = rdx; bb = 1; continue; }
            1 => { rax = ((crate::simd_ops::ru8(r9.wrapping_add(r8))) as u32) as u32 as u64; rdx = ((crate::simd_ops::ru8(r8)) as u32) as u32 as u64; r8 = ((r8.wrapping_add(P1)) as u64) as u64; rax = ((crate::simd_ops::ru8(rax.wrapping_add(r10))) as u32) as u32 as u64; rdx = ((crate::simd_ops::ru8(rdx.wrapping_add(r10))) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 2; continue; } fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 1; continue; } bb = 2; continue; }
            2 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; rax = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1423fdec0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143d0100c; const P1: u64 = 0x143d01010; const P2: u64 = 0x141834a78; const P3: u64 = 0x142645458; const P4: u64 = 0x141834a0c;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x28u64)) as u64; rax = ((crate::exe::tls_pointer() as u64)) as u64; rcx = 0x74; rax = (crate::simd_ops::ru64(rax)) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(rax))) as u32 as u64; fl_l = (crate::simd_ops::ru32(P0)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) > (((fl_r as u32)) as i32)) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = ((crate::simd_ops::ru8(P1)) as u32) as u32 as u64; rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
            2 => { rcx = P0; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru32(P0)) as u32 as u64; fl_r = 0xffffffff; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 1; continue; } rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P3)) as u64, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; rcx = P0; crate::simd_ops::wu8(P1, (if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { 1u8 } else { 0u8 }) as u8); rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); bb = 1; continue; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1423fdf30(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x1427e54e0; const P1: u64 = 0x1;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { r9 = rdx; r10 = rcx; fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } fl_l = r10; fl_r = rdx; r10 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; r11 = P0; bb = 1; continue; }
            1 => { rax = ((crate::simd_ops::ru8(r10.wrapping_add(r9))) as u32) as u32 as u64; rcx = ((crate::simd_ops::ru8(r9)) as u32) as u32 as u64; r9 = ((r9.wrapping_add(P1)) as u64) as u64; rax = ((crate::simd_ops::ru8(rax.wrapping_add(r11))) as u32) as u32 as u64; rcx = ((crate::simd_ops::ru8(rcx.wrapping_add(r11))) as u32) as u32 as u64; fl_l = r8; fl_r = (0x1u64) as u64; r8 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 2; continue; } fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 2; continue; } fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 1; continue; } bb = 2; continue; }
            2 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; rax = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; return rax; }
            3 => { rax = 0; fl_l = 0; fl_r = 0; return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1423fdf90(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xffffffffffffffff; const P1: u64 = 0x141373300;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x38u64)) as u64;
    rax = r8;
    crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), r9);
    r9 = rax;
    r8 = ((rdx.wrapping_add(P0)) as u64) as u64;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x38u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1423fe2a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x142c61100;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 0>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rax = (crate::simd_ops::ru64(rcx)) as u64; rbx = 0; fl_l = 0; fl_r = 0; r10 = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; r9 = ((rbx as u32)) as u32 as u64; r10 = ((r10).wrapping_add(rax)) as u64; rcx = (rcx & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; fl_l = rax; fl_r = r10; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 4; continue; } r11 = P1; bb = 1; continue; }
            1 => { rcx = ((crate::simd_ops::ru8(rax)) as u32) as u32 as u64; r9 = ((r9).wrapping_add(1)) as u64; rax = ((rax).wrapping_add(1)) as u64; r8 = ((crate::simd_ops::ru8(rcx.wrapping_add(r11))) as u32) as u32 as u64; fl_l = ((r8 as u8)) as u8 as u64; fl_r = ((r8 as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 2; continue; } fl_l = ((r8 as u8)) as u8 as u64; fl_r = (0x3u8) as u8 as u64; rcx = (rcx & !0xffu64) | (((if (((fl_l as u8)) > ((fl_r as u8))) { 1u8 } else { 0u8 }) as u8 as u64)); if (((fl_l as u8)) > ((fl_r as u8))) { bb = 4; continue; } rdx = ((rax.wrapping_add(r8)) as u64) as u64; fl_l = rdx; fl_r = r10; rcx = (rcx & !0xffu64) | (((if (((fl_l as u64)) > ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); if (((fl_l as u64)) > ((fl_r as u64))) { bb = 4; continue; } rcx = ((crate::simd_ops::ru8(rax)) as u32) as u32 as u64; rax = ((rax).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(r11))) as u8 as u64; fl_r = (0x8u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 3; continue; } fl_l = ((r8 as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 2; continue; } rcx = ((crate::simd_ops::ru8(rax)) as u32) as u32 as u64; rax = ((rax).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(r11))) as u8 as u64; fl_r = (0x8u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 3; continue; } fl_l = ((r8 as u8)) as u8 as u64; fl_r = (0x2u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 2; continue; } rcx = ((crate::simd_ops::ru8(rax)) as u32) as u32 as u64; r9 = ((r9).wrapping_add(1)) as u64; rax = ((rax).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(r11))) as u8 as u64; fl_r = (0x8u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 3; continue; } bb = 2; continue; }
            2 => { rcx = (rcx & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; fl_l = rax; fl_r = r10; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 1; continue; } fl_l = ((rcx as u8)) as u8 as u64; fl_r = ((rcx as u8)) as u8 as u64; r9 = (if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { rbx } else { r9 }) as u64; rax = r9; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x8u64))) as u64; return rax; }
            3 => { rcx = (rcx & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 4; continue; }
            4 => { fl_l = ((rcx as u8)) as u8 as u64; fl_r = ((rcx as u8)) as u8 as u64; r9 = (if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { rbx } else { r9 }) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x8u64))) as u64; rax = r9; return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1423fe400(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0xffffffffffff2800;
    let (mut rax, mut fl_l, mut fl_r) = (0u64, 0u64, 0u64);
    rax = ((rcx.wrapping_add(P0)) as u32) as u32 as u64;
    fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x3ff;
    rax = (rax & !0xffu64) | (((if (((fl_l as u32)) <= ((fl_r as u32))) { 1u8 } else { 0u8 }) as u8 as u64));
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1423febc0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142646980; const P1: u64 = 0x141121ad0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rbx = rcx; rdi = r8; rsi = rdx; rcx = (((crate::simd_ops::ru8(rax)) as i8 as i32) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm((crate::simd_ops::ru64(P0)) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::lk_add_u64(rbx, (1) as u64); rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = (((crate::simd_ops::ru8(rax)) as i8 as i32) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm((crate::simd_ops::ru64(P0)) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 1; continue; } bb = 2; continue; }
            2 => { rdx = (crate::simd_ops::ru64(rbx)) as u64; r9 = rsp.wrapping_add(0x50); r8 = rsi; rcx = rsp.wrapping_add(0x30); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm0 = (crate::simd_ops::ru128(rax)) as u128; rax = (((xmm0 >> 64) as u32) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 3; continue; } crate::simd_ops::wu8(rdi, (0x1u8) as u8); rax = 0; fl_l = 0; fl_r = 0; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            3 => { rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x50u64))) as u32 as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; crate::simd_ops::wu64(rbx, ((xmm0 as u64)) as u64); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1423fec60(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142646980; const P1: u64 = 0x141376a70;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rbx = rcx; rdi = r8; rsi = rdx; rcx = (((crate::simd_ops::ru8(rax)) as i8 as i32) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm((crate::simd_ops::ru64(P0)) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::lk_add_u64(rbx, (1) as u64); rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = (((crate::simd_ops::ru8(rax)) as i8 as i32) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm((crate::simd_ops::ru64(P0)) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 1; continue; } bb = 2; continue; }
            2 => { rdx = (crate::simd_ops::ru64(rbx)) as u64; r9 = rsp.wrapping_add(0x50); r8 = rsi; rcx = rsp.wrapping_add(0x30); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = (crate::simd_ops::ru32(rsp.wrapping_add(0x38u64))) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 3; continue; } crate::simd_ops::wu8(rdi, (0x1u8) as u8); xmm0 = 0u128; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            3 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; xmm0 = ((crate::simd_ops::ru32(rsp.wrapping_add(0x50u64))) as u32) as u128; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; crate::simd_ops::wu64(rbx, rax); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1423ffc30(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0xffffffffffffffff; const P2: u64 = 0x1423feac0; const P3: u64 = 0x3; const P4: u64 = 0x2; const P5: u64 = 0x1;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rdi = rdx; r9 = rcx; xmm0 = (crate::simd_ops::ru128(rcx)) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } r8 = ((xmm0 as u64)) as u64; fl_l = (crate::simd_ops::ru8(r8)) as u8 as u64; fl_r = (0x28u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 1; continue; } r8 = ((r8).wrapping_add(1)) as u64; fl_l = ((rax).wrapping_sub(1)) as u64; rax = ((fl_l as u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r8); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; bb = 1; continue; }
            1 => { fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } fl_l = (crate::simd_ops::ru8(r8.wrapping_add(rax).wrapping_add(P1))) as u8 as u64; fl_r = (0x29u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 2; continue; } fl_l = ((rax).wrapping_sub(1)) as u64; rax = ((fl_l as u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; bb = 2; continue; }
            2 => { crate::simd_ops::wu128(rcx, (xmm0) as u128); rcx = (crate::simd_ops::ru64(rcx)) as u64; r8 = rsp.wrapping_add(0x40); rbx = (crate::simd_ops::ru64(r9.wrapping_add(P0))) as u64; rbx = ((rbx).wrapping_add(rcx)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rcx); rdx = rbx; crate::simd_ops::wu8(rsp.wrapping_add(0x40u64), (0x0u8) as u8); rcx = rsp.wrapping_add(0x48); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r8 = rsp.wrapping_add(0x40); crate::simd_ops::wu8(rdi.wrapping_add(P3), ((rax as u8)) as u8); rdx = rbx; rcx = rsp.wrapping_add(0x48); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r8 = rsp.wrapping_add(0x40); crate::simd_ops::wu8(rdi.wrapping_add(P4), ((rax as u8)) as u8); rdx = rbx; rcx = rsp.wrapping_add(0x48); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r8 = rsp.wrapping_add(0x40); crate::simd_ops::wu8(rdi.wrapping_add(P5), ((rax as u8)) as u8); rdx = rbx; rcx = rsp.wrapping_add(0x48); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = (crate::simd_ops::ru8(rsp.wrapping_add(0x40u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; crate::simd_ops::wu8(rdi, ((rax as u8)) as u8); rax = (rax & !0xffu64) | (((if (((fl_l as u8)) == ((fl_r as u8))) { 1u8 } else { 0u8 }) as u8 as u64)); rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142401330(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0xffffffffffffffff;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { r9 = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; r8 = r9; fl_l = r9; fl_r = r9; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 1; continue; } rax = (crate::simd_ops::ru64(rdx)) as u64; fl_l = (crate::simd_ops::ru8(rax)) as u8 as u64; fl_r = (0x28u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 2; continue; } fl_l = ((rax).wrapping_add(1)) as u64; rax = ((fl_l as u64)) as u64; r8 = ((r9.wrapping_add(P1)) as u64) as u64; crate::simd_ops::wu64(rdx, rax); crate::simd_ops::wu64(rdx.wrapping_add(P0), r8); bb = 1; continue; }
            1 => { fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(rdx)) as u64; fl_l = (crate::simd_ops::ru8(rax.wrapping_add(r8).wrapping_add(P1))) as u8 as u64; fl_r = (0x29u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 3; continue; } rax = ((r8.wrapping_add(P1)) as u64) as u64; crate::simd_ops::wu64(rdx.wrapping_add(P0), rax); bb = 3; continue; }
            3 => { xmm0 = (crate::simd_ops::ru128(rdx)) as u128; crate::simd_ops::wu128(rcx, (xmm0) as u128); rax = rcx; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142402410(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x4; const P1: u64 = 0x1423fe8f0; const P2: u64 = 0x143407674; const P3: u64 = 0x1426e7d15; const P4: u64 = 0x1405fc3c0; const P5: u64 = 0x14240ab10;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 43>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x150u64)) as u64; rbx = rdx; rdi = rcx; xmm1 = ((crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32) as u128; rcx = rsp.wrapping_add(0xbc); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r8 = ((crate::simd_ops::ru8(rsp.wrapping_add(0xbfu64))) as u32) as u32 as u64; fl_l = ((r8 as u32)) as u32 as u64; fl_r = 0x80; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 1; continue; } r8 = ((r8.wrapping_mul(8)) as u32) as u32 as u64; bb = 2; continue; }
            1 => { r8 = (((r8 as u32)).wrapping_shl(8u32)) as u32 as u64; fl_l = ((r8 as u32)) as u32 as u64; fl_r = 0x7c00; r8 = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru32(rsp.wrapping_add(0xbcu64))) as u32 as u64; rax = ((((rax as u32)) & (0xffffffu32))) as u32 as u64; rax = (((rax as u32)).wrapping_shl(3u32)) as u32 as u64; rsi = rsp.wrapping_add(0xc4); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; rsi = (if (((fl_l as u32)) > ((fl_r as u32))) { crate::simd_ops::ru64(rsp.wrapping_add(0xc4u64)) } else { rsi }) as u64; xmm1 = ((crate::simd_ops::ru32(rbx)) as u32) as u128; rcx = rsp.wrapping_add(0x34); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = ((crate::simd_ops::ru8(rsp.wrapping_add(0x37u64))) as u32) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x80; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 3; continue; } rcx = ((rcx.wrapping_mul(8)) as u32) as u32 as u64; bb = 4; continue; }
            3 => { rcx = (((rcx as u32)).wrapping_shl(8u32)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x7c00; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x34u64))) as u32 as u64; rax = ((((rax as u32)) & (0xffffffu32))) as u32 as u64; rax = (((rax as u32)).wrapping_shl(3u32)) as u32 as u64; r9 = rsp.wrapping_add(0x3c); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; r9 = (if (((fl_l as u32)) > ((fl_r as u32))) { crate::simd_ops::ru64(rsp.wrapping_add(0x3cu64)) } else { r9 }) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); r8 = P2; rdx = ((crate::simd_ops::ru8(P3)) as u32) as u32 as u64; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rsp.wrapping_add(0x34); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rsp.wrapping_add(0xbc); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = rdi; r11 = rsp.wrapping_add(0x150); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x10u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x18u64))) as u64; rsp = r11; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142402f90(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x1424b16a0; const P2: u64 = 0x141377100; const P3: u64 = 0x10; const P4: u64 = 0x140213f20; const P5: u64 = 0x70; const P6: u64 = 0x140600350;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rdi = rdx; rbp = rcx; rdx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; rsi = r8; rcx = (crate::simd_ops::ru64(rdi)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r9 = rax; rcx = rbp.wrapping_add(0x8); rax = (0xde5fb9d2630458e9u64) as u64; r8 = rdi; let prod = (rax as u128).wrapping_mul((r9) as u128); rax = prod as u64; rdx = (prod >> 64) as u64; r9 = ((rdx.wrapping_add(rax)) as u64) as u64; rdx = rsp.wrapping_add(0x20); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x20u64))) as u64; rax = ((rax).wrapping_add(crate::simd_ops::ru64(rbp.wrapping_add(0x8u64)))) as u64; fl_l = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 1; continue; } rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; rdx = rsp.wrapping_add(0x20); r8 = rsi; rcx = rbp.wrapping_add(0x8); rbx = (crate::simd_ops::ru64(rax.wrapping_add(P3))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = ((crate::simd_ops::ru8(rsp.wrapping_add(0x28u64))) as u32) as u32 as u64; fl_l = ((rdx as u8)) as u8 as u64; fl_r = ((rdx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 1; continue; } xmm0 = (crate::simd_ops::ru128(rsi)) as u128; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; rdx = rsi; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x10u64))) as u64; crate::simd_ops::wu128(rax.wrapping_add(rcx.wrapping_mul(8)), (xmm0) as u128); crate::simd_ops::wu64(rax.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(P3), rbx); rcx = ((rbx.wrapping_add(P5)) as u64) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 1; continue; }
            1 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1424032f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x20; const P1: u64 = 0x8; const P2: u64 = 0x1424b16a0; const P3: u64 = 0x141377100; const P4: u64 = 0x141376f10; const P5: u64 = 0x140213f20; const P6: u64 = 0x10;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rdi = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rsi = rdx; rbx = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; rbp = rcx; rdx = (crate::simd_ops::ru64(rdx.wrapping_add(P1))) as u64; rcx = (crate::simd_ops::ru64(rsi)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r8 = rax; rcx = rbp.wrapping_add(0x8); rax = (0xde5fb9d2630458e9u64) as u64; let prod = (rax as u128).wrapping_mul((r8) as u128); rax = prod as u64; rdx = (prod >> 64) as u64; r8 = rsi; r9 = ((rdx.wrapping_add(rax)) as u64) as u64; rdx = rsp.wrapping_add(0x20); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = ((rdi.wrapping_add(rbx)) as u64) as u64; fl_l = (crate::simd_ops::ru64(rax)) as u64; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } r8 = rsi; rdx = rbp.wrapping_add(0x38); rcx = rbp.wrapping_add(0x38); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = (crate::simd_ops::ru64(rbp.wrapping_add(0x38u64))) as u64; rdx = rsp.wrapping_add(0x20); r8 = rsi; rcx = rbp.wrapping_add(0x8); rbx = ((rbx).wrapping_add(0x8u64)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = (crate::simd_ops::ru8(rsp.wrapping_add(0x28u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 1; continue; } rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; xmm0 = (crate::simd_ops::ru128(rsi)) as u128; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x10u64))) as u64; crate::simd_ops::wu128(rax.wrapping_add(rcx.wrapping_mul(8)), (xmm0) as u128); crate::simd_ops::wu64(rax.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(P6), rbx); bb = 1; continue; }
            1 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142403530(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x1424b16a0; const P2: u64 = 0x141377100; const P3: u64 = 0x20; const P4: u64 = 0x10;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rdi = rdx; rsi = rcx; rdx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; rcx = (crate::simd_ops::ru64(rdi)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r8 = rax; rcx = ((rsi.wrapping_add(P0)) as u64) as u64; rax = (0xde5fb9d2630458e9u64) as u64; let prod = (rax as u128).wrapping_mul((r8) as u128); rax = prod as u64; rdx = (prod >> 64) as u64; r8 = rdi; r9 = ((rdx.wrapping_add(rax)) as u64) as u64; rdx = rsp.wrapping_add(0x20); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru64(rsi.wrapping_add(P3))) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rsi.wrapping_add(P0)))) as u64; xmm0 = (crate::simd_ops::ru128(rax)) as u128; rax = ((xmm0 as u64)) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 1; continue; } rax = (((xmm0 >> 64) as u64)) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(P4))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            1 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rax = 0; fl_l = 0; fl_r = 0; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1424035c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143407770; const P1: u64 = 0x141376e30; const P2: u64 = 0x18; const P3: u64 = 0x20; const P4: u64 = 0x8; const P5: u64 = 0x1401de4e0; const P6: u64 = 0x10; const P7: u64 = 0x14183acc2; const P8: u64 = 0x30; const P9: u64 = 0x38;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rcx; rax = P0; crate::simd_ops::wu64(rcx, rax); rcx = ((rcx).wrapping_add(0x38u64)) as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru64(rbx.wrapping_add(P2))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 2; continue; } rdi = (crate::simd_ops::ru64(rbx.wrapping_add(P3))) as u64; fl_l = rdi; fl_r = (0x7fu64) as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 1; continue; } rcx = ((rbx.wrapping_add(P4)) as u64) as u64; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); bb = 2; continue; }
            1 => { fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } crate::simd_ops::wu64(rbx.wrapping_add(P2), (0x0u64) as u64); r8 = ((rdi.wrapping_add(P6)) as u64) as u64; rdx = 0xffffff80; rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P4))) as u64; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rbx.wrapping_add(P4))) as u64; crate::simd_ops::wu8(rdi.wrapping_add(rax), (0xffu8) as u8); rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P3))) as u64; rax = rdx; rax = ((rax).wrapping_shr(3u32)) as u64; rdx = ((rdx).wrapping_sub(rax)) as u64; fl_l = rdx; fl_r = (crate::simd_ops::ru64(rbx.wrapping_add(P2))) as u64; rdx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; crate::simd_ops::wu64(rbx.wrapping_add(P8), rdx); bb = 2; continue; }
            2 => { rcx = ((rbx.wrapping_add(P9)) as u64) as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rcx = ((rbx.wrapping_add(P4)) as u64) as u64; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142405290(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x8; const P2: u64 = 0x18; const P3: u64 = 0x1a; const P4: u64 = 0x1c; const P5: u64 = 0x14;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu32(rcx.wrapping_add(P0), (0x3f000000u32) as u32);
    crate::simd_ops::wu64(rcx, rax);
    crate::simd_ops::wu64(rcx.wrapping_add(P1), rax);
    crate::simd_ops::wu16(rcx.wrapping_add(P2), ((rax as u16)) as u16);
    crate::simd_ops::wu8(rcx.wrapping_add(P3), ((rax as u8)) as u8);
    crate::simd_ops::wu32(rcx.wrapping_add(P4), ((rax as u32)) as u32);
    rax = rcx;
    crate::simd_ops::wu32(rcx.wrapping_add(P5), (0x3e4ccccdu32) as u32);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1424054d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x19; const P1: u64 = 0x50; const P2: u64 = 0x10; const P3: u64 = 0x140f366d0; const P4: u64 = 0x142405dd0; const P5: u64 = 0x48; const P6: u64 = 0x142405060; const P7: u64 = 0x70;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rdx; rdi = rcx; crate::simd_ops::wu8(rcx.wrapping_add(P0), (0x1u8) as u8); rsi = ((rcx.wrapping_add(P1)) as u64) as u64; rcx = ((rsi.wrapping_add(P2)) as u64) as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rdx = rbx; rcx = rsi; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rdx = (crate::simd_ops::ru64(rdi.wrapping_add(P5))) as u64; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); rbx = rax; crate::simd_ops::wu64(rdi.wrapping_add(P7), rax); rdi = (crate::simd_ops::ru64(rdi.wrapping_add(P5))) as u64; rdi = ((rdi).wrapping_shl(5u32)) as u64; rdi = ((rdi).wrapping_add(rax)) as u64; fl_l = rax; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rdx = rsi; rcx = rbx; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rbx = ((rbx).wrapping_add(0x20u64)) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } bb = 2; continue; }
            2 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142405550(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x8; const P2: u64 = 0x18; const P3: u64 = 0x1a; const P4: u64 = 0x1c; const P5: u64 = 0x14; const P6: u64 = 0x28; const P7: u64 = 0x30; const P8: u64 = 0x48; const P9: u64 = 0x38; const P10: u64 = 0x40; const P11: u64 = 0x50; const P12: u64 = 0x143426fc8;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rcx.wrapping_add(P0), (0x3f000000u32) as u32); crate::simd_ops::wu64(rcx, rax); crate::simd_ops::wu64(rcx.wrapping_add(P1), rax); crate::simd_ops::wu16(rcx.wrapping_add(P2), ((rax as u16)) as u16); crate::simd_ops::wu8(rcx.wrapping_add(P3), ((rax as u8)) as u8); crate::simd_ops::wu32(rcx.wrapping_add(P4), ((rax as u32)) as u32); crate::simd_ops::wu32(rcx.wrapping_add(P5), (0x3e4ccccdu32) as u32); crate::simd_ops::wu64(rcx.wrapping_add(P6), rax); crate::simd_ops::wu64(rcx.wrapping_add(P7), rax); crate::simd_ops::wu64(rcx.wrapping_add(P8), (0x20u64) as u64); crate::simd_ops::wu64(rcx.wrapping_add(P9), rax); crate::simd_ops::wu64(rcx.wrapping_add(P10), rax); crate::simd_ops::wu64(rcx.wrapping_add(P11), rax); crate::simd_ops::wu64(rcx.wrapping_add(0x58u64), rax); crate::simd_ops::wu64(rcx.wrapping_add(0x60u64), rax); crate::simd_ops::wu64(rcx.wrapping_add(0x68u64), rax); rdx = (crate::simd_ops::ru64(rcx.wrapping_add(0x48u64))) as u64; crate::simd_ops::wu64(rcx.wrapping_add(0x70u64), rax); xmm1 = 0u128; fl_l = rdx; fl_r = rdx; if (((((fl_l as u64)) & ((fl_r as u64))) as i64) < 0) { bb = 1; continue; } xmm1 = (xmm1 & !0xffff_ffffu128) | ((((rdx) as i64) as f32).to_bits() as u128); bb = 2; continue; }
            1 => { rax = rdx; rdx = ((((rdx as u32)) & (0x1u32))) as u32 as u64; rax = ((rax).wrapping_shr(1u32)) as u64; fl_l = (((rax) | (rdx))) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; xmm1 = (xmm1 & !0xffff_ffffu128) | ((((rax) as i64) as f32).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) + f32::from_bits(xmm1 as u32)).to_bits() as u128); bb = 2; continue; }
            2 => { xmm2 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(crate::simd_ops::ru32(rcx.wrapping_add(0x10u64)))).to_bits() as u128); xmm0 = ((crate::simd_ops::ru32(P12)) as u32) as u128; rdx = 0; fl_l = 0; fl_r = 0; r8 = (0x8000000000000000u64) as u64; fl_l = ((xmm2 as u32)) as u32 as u64; fl_r = ((xmm0 as u32)) as u32 as u64; if !(f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { bb = 3; continue; } xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) - f32::from_bits(xmm0 as u32)).to_bits() as u128); fl_l = ((xmm2 as u32)) as u32 as u64; fl_r = ((xmm0 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { bb = 3; continue; } rdx = r8; bb = 3; continue; }
            3 => { rax = (((crate::simd_ops::cvt_f32_i64(f32::from_bits(xmm2 as u32)) as i64)) as u64) as u64; xmm2 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(crate::simd_ops::ru32(rcx.wrapping_add(0x14u64)))).to_bits() as u128); rax = ((rax).wrapping_add(rdx)) as u64; rdx = 0; fl_l = 0; fl_r = 0; fl_l = ((xmm2 as u32)) as u32 as u64; fl_r = ((xmm0 as u32)) as u32 as u64; crate::simd_ops::wu64(rcx, rax); if !(f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { bb = 4; continue; } xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) - f32::from_bits(xmm0 as u32)).to_bits() as u128); fl_l = ((xmm2 as u32)) as u32 as u64; fl_r = ((xmm0 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { bb = 4; continue; } rdx = r8; bb = 4; continue; }
            4 => { rax = (((crate::simd_ops::cvt_f32_i64(f32::from_bits(xmm2 as u32)) as i64)) as u64) as u64; fl_l = ((rax).wrapping_add(rdx)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; crate::simd_ops::wu8(rcx.wrapping_add(0x18u64), (0x0u8) as u8); crate::simd_ops::wu64(rcx.wrapping_add(0x8u64), rax); rax = rcx; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1424058a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x8; const P2: u64 = 0x18; const P3: u64 = 0x1a; const P4: u64 = 0x1c; const P5: u64 = 0x28; const P6: u64 = 0x30; const P7: u64 = 0x14; const P8: u64 = 0x48; const P9: u64 = 0x38; const P10: u64 = 0x40; const P11: u64 = 0x50; const P12: u64 = 0x143426fc8;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rcx.wrapping_add(P0), (0x3f000000u32) as u32); crate::simd_ops::wu64(rcx, rax); crate::simd_ops::wu64(rcx.wrapping_add(P1), rax); crate::simd_ops::wu16(rcx.wrapping_add(P2), ((rax as u16)) as u16); crate::simd_ops::wu8(rcx.wrapping_add(P3), ((rax as u8)) as u8); crate::simd_ops::wu32(rcx.wrapping_add(P4), ((rax as u32)) as u32); crate::simd_ops::wu64(rcx.wrapping_add(P5), rax); crate::simd_ops::wu64(rcx.wrapping_add(P6), rax); crate::simd_ops::wu32(rcx.wrapping_add(P7), (0x3e4ccccdu32) as u32); crate::simd_ops::wu64(rcx.wrapping_add(P8), (0x20u64) as u64); crate::simd_ops::wu64(rcx.wrapping_add(P9), rax); crate::simd_ops::wu64(rcx.wrapping_add(P10), rax); crate::simd_ops::wu64(rcx.wrapping_add(P11), rax); crate::simd_ops::wu64(rcx.wrapping_add(0x58u64), rax); crate::simd_ops::wu64(rcx.wrapping_add(0x60u64), rax); crate::simd_ops::wu64(rcx.wrapping_add(0x68u64), rax); rdx = (crate::simd_ops::ru64(rcx.wrapping_add(0x48u64))) as u64; crate::simd_ops::wu64(rcx.wrapping_add(0x70u64), rax); xmm1 = 0u128; fl_l = rdx; fl_r = rdx; if (((((fl_l as u64)) & ((fl_r as u64))) as i64) < 0) { bb = 1; continue; } xmm1 = (xmm1 & !0xffff_ffffu128) | ((((rdx) as i64) as f32).to_bits() as u128); bb = 2; continue; }
            1 => { rax = rdx; rdx = ((((rdx as u32)) & (0x1u32))) as u32 as u64; rax = ((rax).wrapping_shr(1u32)) as u64; fl_l = (((rax) | (rdx))) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; xmm1 = (xmm1 & !0xffff_ffffu128) | ((((rax) as i64) as f32).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) + f32::from_bits(xmm1 as u32)).to_bits() as u128); bb = 2; continue; }
            2 => { xmm2 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(crate::simd_ops::ru32(rcx.wrapping_add(0x10u64)))).to_bits() as u128); xmm0 = ((crate::simd_ops::ru32(P12)) as u32) as u128; rdx = 0; fl_l = 0; fl_r = 0; r8 = (0x8000000000000000u64) as u64; fl_l = ((xmm2 as u32)) as u32 as u64; fl_r = ((xmm0 as u32)) as u32 as u64; if !(f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { bb = 3; continue; } xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) - f32::from_bits(xmm0 as u32)).to_bits() as u128); fl_l = ((xmm2 as u32)) as u32 as u64; fl_r = ((xmm0 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { bb = 3; continue; } rdx = r8; bb = 3; continue; }
            3 => { rax = (((crate::simd_ops::cvt_f32_i64(f32::from_bits(xmm2 as u32)) as i64)) as u64) as u64; xmm2 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(crate::simd_ops::ru32(rcx.wrapping_add(0x14u64)))).to_bits() as u128); rax = ((rax).wrapping_add(rdx)) as u64; rdx = 0; fl_l = 0; fl_r = 0; fl_l = ((xmm2 as u32)) as u32 as u64; fl_r = ((xmm0 as u32)) as u32 as u64; crate::simd_ops::wu64(rcx, rax); if !(f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { bb = 4; continue; } xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) - f32::from_bits(xmm0 as u32)).to_bits() as u128); fl_l = ((xmm2 as u32)) as u32 as u64; fl_r = ((xmm0 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { bb = 4; continue; } rdx = r8; bb = 4; continue; }
            4 => { rax = (((crate::simd_ops::cvt_f32_i64(f32::from_bits(xmm2 as u32)) as i64)) as u64) as u64; fl_l = ((rax).wrapping_add(rdx)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; crate::simd_ops::wu8(rcx.wrapping_add(0x18u64), (0x0u8) as u8); crate::simd_ops::wu64(rcx.wrapping_add(0x8u64), rax); rax = rcx; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142405990(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141377260;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rax = ((rax).wrapping_add(0x10u64)) as u64;
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142405dd0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x18; const P2: u64 = 0x1424037c0; const P3: u64 = 0x1c;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rsi = rcx; xmm0 = (crate::simd_ops::ru128(rdx)) as u128; crate::simd_ops::wu128(rcx, (xmm0) as u128); rdi = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rcx.wrapping_add(P0), rdi); crate::simd_ops::wu64(rcx.wrapping_add(P1), rdi); rax = (crate::simd_ops::ru32(rdx.wrapping_add(P1))) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rcx = ((rcx.wrapping_mul(8)) as u64) as u64; rbx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; rbp = ((rbx.wrapping_add(rcx)) as u64) as u64; rax = (0x2aaaaaaaaaaaaaabu64) as u64; let prod = (rax as i64 as i128).wrapping_mul((rcx) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; r14 = rdx; r14 = ((((r14) as i64).wrapping_shr(2u32) as u64)) as u64; rax = r14; rax = ((rax).wrapping_shr(63u32)) as u64; r14 = ((r14).wrapping_add(rax)) as u64; fl_l = r14; fl_r = r14; if (((((fl_l as u64)) & ((fl_r as u64))) as i64) <= 0) { bb = 1; continue; } rdx = r14; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rsi.wrapping_add(P0), rax); crate::simd_ops::wu32(rsi.wrapping_add(P1), ((rdi as u32)) as u32); crate::simd_ops::wu32(rsi.wrapping_add(P3), ((r14 as u32)) as u32); rdi = rax; bb = 1; continue; }
            1 => { fl_l = rbx; fl_r = rbp; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 3; continue; } bb = 2; continue; }
            2 => { xmm0 = (crate::simd_ops::ru128(rbx)) as u128; crate::simd_ops::wu128(rdi, (xmm0) as u128); xmm1 = ((crate::simd_ops::ru64(rbx.wrapping_add(P0))) as u64) as u128; crate::simd_ops::wu64(rdi.wrapping_add(P0), ((xmm1 as u64)) as u64); rdi = ((rdi.wrapping_add(P1)) as u64) as u64; rbx = ((rbx).wrapping_add(0x18u64)) as u64; fl_l = rbx; fl_r = rbp; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 3; continue; }
            3 => { crate::simd_ops::wu32(rsi.wrapping_add(P1), ((r14 as u32)) as u32); rax = rsi; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142405ec0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x1426d01e0;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rdi) = (0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 0>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdi); r11 = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; rbx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rax = (crate::simd_ops::ru64(rdx)) as u64; fl_l = r11; fl_r = rbx; xmm0 = (crate::simd_ops::ru128(rcx)) as u128; rdx = rbx; rdx = (if (((fl_l as u64)) < ((fl_r as u64))) { r11 } else { rdx }) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } r10 = ((xmm0 as u64)) as u64; fl_l = r10; fl_r = rax; r10 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; rdi = P1; bb = 1; continue; }
            1 => { rcx = ((crate::simd_ops::ru8(r10.wrapping_add(rax))) as u32) as u32 as u64; rdx = ((rdx).wrapping_sub(1)) as u64; r8 = ((crate::simd_ops::ru8(rcx.wrapping_add(rdi))) as u32) as u32 as u64; rcx = ((crate::simd_ops::ru8(rax)) as u32) as u32 as u64; fl_l = ((r8 as u8)) as u8 as u64; fl_r = (crate::simd_ops::ru8(rcx.wrapping_add(rdi))) as u8 as u64; if ((((fl_l as u8)) as i8) < (((fl_r as u8)) as i8)) { bb = 4; continue; } if ((((fl_l as u8)) as i8) > (((fl_r as u8)) as i8)) { bb = 3; continue; } rax = ((rax).wrapping_add(1)) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 1; continue; } bb = 2; continue; }
            2 => { fl_l = rbx; fl_r = r11; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 4; continue; } rax = 0; fl_l = 0; fl_r = 0; fl_l = rbx; fl_r = r11; rax = (rax & !0xffu64) | (((if (((fl_l as u64)) > ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); fl_l = (((rax as u32)).wrapping_shr(31u32)) as u32 as u64; rax = ((fl_l as u32)) as u32 as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x8u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x10u64))) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            3 => { rax = 0x0; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x8u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x10u64))) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            4 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x8u64))) as u64; rax = 0x1; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x10u64))) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142406430(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x143a001ac; const P1: u64 = 0x143a00290;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rdx = (((crate::simd_ops::ru32(P0)) as i32 as i64) as u64) as u64;
    fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0x1000;
    if !(((((fl_l as u32)) as i32) >= (((fl_r as u32)) as i32))) { r8 = P1; crate::simd_ops::wu64(r8.wrapping_add(rdx.wrapping_mul(8)), rcx); fl_l = (((rdx as u32)).wrapping_add(1)) as u32 as u64; rdx = ((fl_l as u32)) as u32 as u64; crate::simd_ops::wu32(P0, ((rdx as u32)) as u32); }
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1424064f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x1434077a8; const P1: u64 = 0x8; const P2: u64 = 0xc;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = P0;
    crate::simd_ops::wu32(rcx.wrapping_add(P1), ((r8 as u32)) as u32);
    crate::simd_ops::wu64(rcx, rax);
    rax = rcx;
    crate::simd_ops::wu8(rcx.wrapping_add(P2), (0x0u8) as u8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142406540(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142646a40; const P1: u64 = 0x143a28df6; const P2: u64 = 0x142417470; const P3: u64 = 0x142646a70; const P4: u64 = 0x1433e73f8; const P5: u64 = 0x142646a68; const P6: u64 = 0x3; const P7: u64 = 0x4; const P8: u64 = 0x8; const P9: u64 = 0x142409b50;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 25>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        core::sync::atomic::fence(core::sync::atomic::Ordering::SeqCst);
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdx); crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0xc0u64)) as u64; rbx = rcx; rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P0)) as u64, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rsp.wrapping_add(0xe0u64), rax); rax = P1; crate::simd_ops::wu64(rsp.wrapping_add(0xe8u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0xd8u64), (0x64u64) as u64); bb = 1; continue; }
            1 => { rax = ((crate::simd_ops::ru8(P1)) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { rax = ((crate::simd_ops::ru8(P1)) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 2; continue; } bb = 3; continue; }
            3 => { rax = 0x1; let tmp_xchg: u8 = crate::simd_ops::atomic_xchg_u8(P1, ((rax as u8)) as u8); rax = (rax & !0xffu64) | (((tmp_xchg) as u8 as u64)); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 4; continue; } rcx = rsp.wrapping_add(0xd8); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 1; continue; }
            4 => { rcx = rsp.wrapping_add(0xe0); rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P3)) as u64, rax, rcx, rdx, r8, r9, rsp); r9 = rax; r8 = P4; rdx = 0x80; rcx = rsp.wrapping_add(0x40); rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P5)) as u64, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru32(rbx)) as u32 as u64; rax = ((((rax as u32)) & (0xff000010u32))) as u32 as u64; fl_l = ((((rax as u32)) | (0x10u32))) as u32 as u64; fl_r = fl_l; rax = ((fl_l as u32)) as u32 as u64; crate::simd_ops::wu32(rbx, ((rax as u32)) as u32); crate::simd_ops::wu8(rbx.wrapping_add(P6), (0x10u8) as u8); crate::simd_ops::wu32(rbx.wrapping_add(P7), (0x0u32) as u32); crate::simd_ops::wu8(rbx.wrapping_add(P8), (0x0u8) as u8); rax = rsp.wrapping_add(0x40); r9 = (0xffffffffffffffffu64) as u64; bb = 5; continue; }
            5 => { r9 = ((r9).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru8(rax.wrapping_add(r9))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 5; continue; } rax = rsp.wrapping_add(0x40); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), r9); r8 = 0; fl_l = 0; fl_r = 0; rdx = rsp.wrapping_add(0x30); rcx = rbx; rax = crate::simd_ops::call_stk(P9, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu8(P1, (0x0u8) as u8); rax = rbx; rsp = ((rsp).wrapping_add(0xc0u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142406660(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142646a40; const P1: u64 = 0x143a28df6; const P2: u64 = 0x142417470; const P3: u64 = 0x142646a70; const P4: u64 = 0x4; const P5: u64 = 0x1; const P6: u64 = 0x8; const P7: u64 = 0x2;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        core::sync::atomic::fence(core::sync::atomic::Ordering::SeqCst);
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rcx; rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P0)) as u64, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (0x64u64) as u64); bb = 1; continue; }
            1 => { rax = ((crate::simd_ops::ru8(P1)) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { rax = ((crate::simd_ops::ru8(P1)) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 2; continue; } bb = 3; continue; }
            3 => { rax = 0x1; let tmp_xchg: u8 = crate::simd_ops::atomic_xchg_u8(P1, ((rax as u8)) as u8); rax = (rax & !0xffu64) | (((tmp_xchg) as u8 as u64)); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 4; continue; } rcx = rsp.wrapping_add(0x30); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 1; continue; }
            4 => { rcx = rsp.wrapping_add(0x38); rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P3)) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = ((crate::simd_ops::ru8(rax)) as u32) as u32 as u64; crate::simd_ops::wu8(rbx, ((rcx as u8)) as u8); rcx = ((crate::simd_ops::ru8(rax.wrapping_add(P4))) as u32) as u32 as u64; crate::simd_ops::wu8(rbx.wrapping_add(P5), ((rcx as u8)) as u8); rax = ((crate::simd_ops::ru8(rax.wrapping_add(P6))) as u32) as u32 as u64; crate::simd_ops::wu8(rbx.wrapping_add(P7), ((rax as u8)) as u8); crate::simd_ops::wu8(P1, (0x0u8) as u8); rax = rbx; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1424066f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143a28df6; const P1: u64 = 0x142417470; const P2: u64 = 0x142646a70; const P3: u64 = 0x4; const P4: u64 = 0x18; const P5: u64 = 0xc; const P6: u64 = 0x10; const P7: u64 = 0x14; const P8: u64 = 0x8; const P9: u64 = 0x76c;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx) = (0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        core::sync::atomic::fence(core::sync::atomic::Ordering::SeqCst);
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rcx; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (0x64u64) as u64); bb = 1; continue; }
            1 => { rax = ((crate::simd_ops::ru8(P0)) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { rax = ((crate::simd_ops::ru8(P0)) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 2; continue; } bb = 3; continue; }
            3 => { rax = 0x1; let tmp_xchg: u8 = crate::simd_ops::atomic_xchg_u8(P0, ((rax as u8)) as u8); rax = (rax & !0xffu64) | (((tmp_xchg) as u8 as u64)); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 4; continue; } rcx = rsp.wrapping_add(0x30); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 1; continue; }
            4 => { rcx = rsp.wrapping_add(0x38); rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P2)) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru32(rax.wrapping_add(P3))) as u32 as u64; rdx = (crate::simd_ops::ru32(rax.wrapping_add(P4))) as u32 as u64; r8 = (crate::simd_ops::ru32(rax.wrapping_add(P5))) as u32 as u64; r9 = (crate::simd_ops::ru32(rax.wrapping_add(P6))) as u32 as u64; r10 = (crate::simd_ops::ru32(rax.wrapping_add(P7))) as u32 as u64; rax = (crate::simd_ops::ru32(rax.wrapping_add(P8))) as u32 as u64; crate::simd_ops::wu32(rbx, ((rax as u32)) as u32); crate::simd_ops::wu32(rbx.wrapping_add(P3), ((rcx as u32)) as u32); crate::simd_ops::wu32(rbx.wrapping_add(P8), ((rdx as u32)) as u32); crate::simd_ops::wu32(rbx.wrapping_add(P5), ((r8 as u32)) as u32); crate::simd_ops::wu32(rbx.wrapping_add(P6), ((r9 as u32)) as u32); rax = ((r10.wrapping_add(P9)) as u32) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(P7), ((rax as u32)) as u32); crate::simd_ops::wu8(P0, (0x0u8) as u8); rax = rbx; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1424067a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143a28df6; const P1: u64 = 0x142417470; const P2: u64 = 0x142646a70; const P3: u64 = 0x1c;
    let (mut rax, mut fl_l, mut fl_r) = (0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        core::sync::atomic::fence(core::sync::atomic::Ordering::SeqCst);
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx); rsp = ((rsp).wrapping_sub(0x28u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), (0x64u64) as u64); bb = 1; continue; }
            1 => { rax = ((crate::simd_ops::ru8(P0)) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { rax = ((crate::simd_ops::ru8(P0)) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 2; continue; } bb = 3; continue; }
            3 => { rax = 0x1; let tmp_xchg: u8 = crate::simd_ops::atomic_xchg_u8(P0, ((rax as u8)) as u8); rax = (rax & !0xffu64) | (((tmp_xchg) as u8 as u64)); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 4; continue; } rcx = rsp.wrapping_add(0x38); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 1; continue; }
            4 => { rcx = rsp.wrapping_add(0x30); rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P2)) as u64, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru32(rax.wrapping_add(P3))) as u32 as u64; crate::simd_ops::wu8(P0, (0x0u8) as u8); rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1424068a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143d01014; const P1: u64 = 0x143d01020; const P2: u64 = 0x141834a78; const P3: u64 = 0x1426456f8; const P4: u64 = 0x141834a0c;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x28u64)) as u64; rax = ((crate::exe::tls_pointer() as u64)) as u64; rcx = 0x74; rax = (crate::simd_ops::ru64(rax)) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(rax))) as u32 as u64; fl_l = (crate::simd_ops::ru32(P0)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) > (((fl_r as u32)) as i32)) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(P1)) as u64; rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
            2 => { rcx = P0; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru32(P0)) as u32 as u64; fl_r = 0xffffffff; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 1; continue; } rcx = rsp.wrapping_add(0x30); rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P3)) as u64, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rcx = P0; crate::simd_ops::wu64(P1, rax); rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(P1)) as u64; rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142406b80(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x2;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    fl_l = ((rcx as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64;
    if !(((((fl_l as u8)) & ((fl_r as u8))) == 0)) { rax = 0; fl_l = 0; fl_r = 0; return rax; }
    rax = ((rcx as u32)) as u32 as u64;
    rax = (((rax as u32)).wrapping_shr(1u32)) as u32 as u64;
    fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64;
    if !(((((fl_l as u8)) & ((fl_r as u8))) == 0)) { rax = 0x1; return rax; }
    rcx = (((rcx as u32)).wrapping_shr(2u32)) as u32 as u64;
    rax = 0xffffffff;
    fl_l = ((rcx as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64;
    rcx = ((rdx.wrapping_add(P0)) as u32) as u32 as u64;
    rax = (if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { (rcx as u32) } else { (rax as u32) }) as u32 as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142406bb0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x130; const P2: u64 = 0x131; const P3: u64 = 0x1; const P4: u64 = 0x1008; const P5: u64 = 0x142645580; const P6: u64 = 0x142645868; const P7: u64 = 0x108; const P8: u64 = 0x142645748; const P9: u64 = 0x1423f0c20; const P10: u64 = 0x142442de0; const P11: u64 = 0x143406c90; const P12: u64 = 0x1423f0780; const P13: u64 = 0x14136eec0; const P14: u64 = 0x142406c6f;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x142406bb0, funcinfo_rva: 0x37df48c, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0x38, conts: &[] };
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rbx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rdi = (rcx) as u64; rdx = ((rdx as u32)) as u32 as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rbx = (crate::simd_ops::ru64(rax.wrapping_add(rdx.wrapping_mul(8)))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), (rbx) as u64); rcx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rax.wrapping_add(rdx.wrapping_mul(8)), (rcx) as u64); fl_l = (rbx) as u64 as u64; fl_r = (rbx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 1; continue; }
            1 => { let tmp_xchg: u8 = crate::simd_ops::atomic_xchg_u8(rbx.wrapping_add(P1), ((rcx as u8)) as u8); rcx = (rcx & !0xffu64) | (((tmp_xchg) as u8 as u64)); rax = ((crate::simd_ops::ru8(rbx.wrapping_add(P2))) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 4; continue; } bb = 2; continue; }
            2 => { bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rdi)) as u64; r8 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rdx = ((r8.wrapping_add(P3)) as u32) as u32 as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(P4))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142406c01); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P5)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142406c07); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P6)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rax = ((crate::simd_ops::ru8(rbx.wrapping_add(P2))) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 3; continue; } bb = 4; continue; }
            4 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P7))) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { rdx = (0xffffffffu32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142406c2a); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P8)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 9; continue; } bb = 6; continue; }
            6 => { fl_l = (rbx) as u64 as u64; fl_r = (rbx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 7; continue; }
            7 => { rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142406c3c); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P9) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142406c44); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P10) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 8; continue; }
            8 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            9 => { rdx = ((P11) as u64) as u64; rcx = ((rsp.wrapping_add(0x20u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142406c61); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P12) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rcx = (rax) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142406c6a); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P13) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); crate::lift_ops::trap(0x80000003, P14, rcx, rsp); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142406c80(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1434077d8; const P1: u64 = 0x1423fe030; const P2: u64 = 0x8; const P3: u64 = 0x141378580; const P4: u64 = 0x10; const P5: u64 = 0x1423f0c20; const P6: u64 = 0x142442de0; const P7: u64 = 0x1423f0b30; const P8: u64 = 0x131; const P9: u64 = 0x142645868;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x142406c80, funcinfo_rva: 0x37df4c8, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0x48, conts: &[] };
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu32(rsp.wrapping_add(0x10u64), ((rdx as u32)) as u32); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rbx = ((rdx as u32)) as u32 as u64; rdi = (rcx) as u64; rcx = ((P0) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142406c97); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), (rax) as u64); rax = ((rsp.wrapping_add(0x50u64)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (rax) as u64); rax = ((rsp.wrapping_add(0x58u64)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rax) as u64); r9 = (crate::simd_ops::ru64(rdi.wrapping_add(P2))) as u64; r8 = (crate::simd_ops::ru64(rdi)) as u64; rcx = ((rsp.wrapping_add(0x60u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142406cc1); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rdx = (crate::simd_ops::ru64(rdi.wrapping_add(P4))) as u64; rsi = ((rbx.wrapping_mul(8)) as u64) as u64; r8 = (crate::simd_ops::ru64(rax)) as u64; crate::simd_ops::wu64(rax, (0x0u64) as u64); rbx = (crate::simd_ops::ru64(rsi.wrapping_add(rdx))) as u64; crate::simd_ops::wu64(rsi.wrapping_add(rdx), (r8) as u64); fl_l = (rbx) as u64 as u64; fl_r = (rbx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142406cec); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142406cf4); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 2; continue; }
            2 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; fl_l = (rbx) as u64 as u64; fl_r = (rbx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142406d07); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142406d0f); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(P4))) as u64; rbx = (crate::simd_ops::ru64(rsi.wrapping_add(rax))) as u64; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142406d20); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P7) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 8; continue; } bb = 5; continue; }
            5 => { bb = 6; continue; }
            6 => { rax = ((crate::simd_ops::ru8(rbx.wrapping_add(P8))) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 8; continue; } bb = 7; continue; }
            7 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142406d3c); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P9)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 6; continue; }
            8 => { rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rsi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rbx = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142407340(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1423f0b30; const P1: u64 = 0x131; const P2: u64 = 0x142645868;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rcx; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rdi = (((rax as u8)) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { rdx = ((crate::simd_ops::ru8(rbx.wrapping_add(P1))) as u32) as u32 as u64; fl_l = ((rdx as u8)) as u8 as u64; fl_r = ((rdx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 2; continue; } rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P2)) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 1; continue; }
            2 => { rax = (((rdi as u8)) as u32) as u32 as u64; bb = 3; continue; }
            3 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142407390(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14183acc2; const P1: u64 = 0x1426469b8; const P2: u64 = 0x8; const P3: u64 = 0x2; const P4: u64 = 0x80; const P5: u64 = 0x70; const P6: u64 = 0xffffffffffffff80; const P7: u64 = 0xffffffffffffffa0; const P8: u64 = 0xffffffffffffffc0; const P9: u64 = 0xffffffffffffffe0; const P10: u64 = 0xfffffffffffffff0; const P11: u64 = 0x1434077f8;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 37>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x120u64)) as u64; rsi = r8; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (0xdu64) as u64); rbp = rdx; crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), (0xffffffffu32) as u32); rbx = rcx; crate::simd_ops::wu32(rsp.wrapping_add(0x2cu64), (0x4u32) as u32); r14 = 0; fl_l = 0; fl_r = 0; rcx = rsp.wrapping_add(0x31); rdx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu8(rsp.wrapping_add(0x30u64), ((r14 as u8)) as u8); r8 = 0xef; rdi = ((r9 as u32)) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r8 = (crate::simd_ops::ru64(rsp.wrapping_add(0x150u64))) as u64; rcx = rsp.wrapping_add(0x31); rdx = 0xef; (rax, xmm0) = crate::simd_ops::call_stk_xmm((crate::simd_ops::ru64(P1)) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = ((rbx.wrapping_add(P2)) as u64) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x2cu64), (0x4u32) as u32); rax = rsp.wrapping_add(0x20); rdx = ((r14.wrapping_add(P3)) as u32) as u32 as u64; bb = 1; continue; }
            1 => { rcx = ((rcx.wrapping_add(P4)) as u64) as u64; let y_addr: u64 = rax; xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); xmm1 = (crate::simd_ops::ru128(rax.wrapping_add(P5))) as u128; rax = ((rax.wrapping_add(P4)) as u64) as u64; let y_addr: u64 = rcx.wrapping_add(P6); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); let y_addr: u64 = rax.wrapping_add(P7); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rcx.wrapping_add(P7); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); let y_addr: u64 = rax.wrapping_add(P8); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rcx.wrapping_add(P8); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); xmm0 = (crate::simd_ops::ru128(rax.wrapping_add(P9))) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rcx.wrapping_add(P9), (xmm0) as u128); crate::simd_ops::wu128(rcx.wrapping_add(P10), (xmm1) as u128); fl_l = rdx; fl_r = (0x1u64) as u64; rdx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } rax = P11; crate::simd_ops::wu64(rbx.wrapping_add(0x108u64), r14); crate::simd_ops::wu64(rbx, rax); rax = rbx; crate::simd_ops::wu64(rbx.wrapping_add(0x110u64), r14); crate::simd_ops::wu8(rbx.wrapping_add(0x118u64), ((r14 as u8)) as u8); crate::simd_ops::wu64(rbx.wrapping_add(0x120u64), rbp); crate::simd_ops::wu64(rbx.wrapping_add(0x128u64), rsi); crate::simd_ops::wu16(rbx.wrapping_add(0x130u64), (0x1u16) as u16); crate::simd_ops::wu32(rbx.wrapping_add(0x134u64), ((rdi as u32)) as u32); r11 = rsp.wrapping_add(0x120); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x10u64))) as u64; rbp = (crate::simd_ops::ru64(r11.wrapping_add(0x18u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x20u64))) as u64; rdi = (crate::simd_ops::ru64(r11.wrapping_add(0x28u64))) as u64; rsp = r11; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142407b30(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143d01030; const P1: u64 = 0x143d01040; const P2: u64 = 0x141834a78; const P3: u64 = 0x143d01048; const P4: u64 = 0x14183acc2; const P5: u64 = 0x14263f610; const P6: u64 = 0x143d01148; const P7: u64 = 0x143d01150; const P8: u64 = 0x141834d18; const P9: u64 = 0x141834a0c;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x28u64)) as u64; rax = ((crate::exe::tls_pointer() as u64)) as u64; rcx = 0x74; rax = (crate::simd_ops::ru64(rax)) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(rax))) as u32 as u64; fl_l = (crate::simd_ops::ru32(P0)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) > (((fl_r as u32)) as i32)) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = P1; rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
            2 => { rcx = P0; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru32(P0)) as u32 as u64; fl_r = 0xffffffff; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 1; continue; } rdx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(P1, (0x0u64) as u64); r8 = 0x108; rcx = P3; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rcx = P5; crate::simd_ops::wu64(P6, (0x0u64) as u64); crate::simd_ops::wu32(P7, (0x0u32) as u32); rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); rcx = P0; rax = crate::simd_ops::call_stk(P9, rax, rcx, rdx, r8, r9, rsp); rax = P1; rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142407d90(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l) = (crate::simd_ops::get_rax_in(), 0u64);
    r8 = rdx;
    rdx = ((((rdx as u32)) & (0x3fu32))) as u32 as u64;
    r8 = ((r8).wrapping_shr(6u32)) as u64;
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(r8.wrapping_mul(8)))) as u64;
    fl_l = (((rax) >> (((rdx) as u32 & 63))) & 1) as u64;
    rax = ((rax) & !((1u64) << (((rdx) as u32 & 63)))) as u64;
    crate::simd_ops::wu64(rcx.wrapping_add(r8.wrapping_mul(8)), rax);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142407dc0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x1;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = ((rdx.wrapping_add(P0)) as u64) as u64;
    rax = ((rcx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142407e20(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x143c4ac00;
    let (mut rax, mut fl_l) = (crate::simd_ops::get_rax_in(), 0u64);
    r8 = ((rdx as u32)) as u32 as u64;
    rcx = ((rdx as u32)) as u32 as u64;
    r8 = ((((r8 as u32)) & (0x3fu32))) as u32 as u64;
    rcx = ((rcx).wrapping_shr(6u32)) as u64;
    rdx = P0;
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(rcx.wrapping_mul(8)))) as u64;
    fl_l = (((rax) >> (((r8) as u32 & 63))) & 1) as u64;
    rax = ((rax) & !((1u64) << (((r8) as u32 & 63)))) as u64;
    crate::simd_ops::wu64(rdx.wrapping_add(rcx.wrapping_mul(8)), rax);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142407e50(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x143c4ac08;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = ((rdx as u32)) as u32 as u64;
    rcx = P0;
    rax = ((rcx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142407e70(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x143c4ac00; const P1: u64 = 0x8;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    r8 = ((rdx as u32)) as u32 as u64;
    rcx = P0;
    rax = ((rdx as u32)) as u32 as u64;
    r8 = ((((r8 as u32)) & (0x3fu32))) as u32 as u64;
    rdx = ((rcx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64;
    rax = ((rax).wrapping_shr(6u32)) as u64;
    r9 = ((rcx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64;
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(rax.wrapping_mul(8)))) as u64;
    fl_l = (((rax) >> (((r8) as u32 & 63))) & 1) as u64;
    if !(((fl_l as u64) != 0)) { rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rdx.wrapping_add(P1), rax); rcx = (crate::simd_ops::ru64(r9)) as u64; fl_l = (((rcx) >> (((r8) as u32 & 63))) & 1) as u64; rcx = ((rcx) & !((1u64) << (((r8) as u32 & 63)))) as u64; fl_l = (((rcx) >> (((r8) as u32 & 63))) & 1) as u64; rcx = ((rcx) | ((1u64) << (((r8) as u32 & 63)))) as u64; crate::simd_ops::wu64(r9, rcx); }
    rax = ((rdx.wrapping_add(P1)) as u64) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142407ec0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142645790;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P0)) as u64, rax, rcx, rdx, r8, r9, rsp);
    rax = ((rax as u32)) as u32 as u64;
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142407ee0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142407c80; const P1: u64 = 0x143c4aa00; const P2: u64 = 0x143407818;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rax = ((rax as u32)) as u32 as u64;
    rcx = P1;
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(rax.wrapping_mul(8)))) as u64;
    fl_l = rax; fl_r = rax;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rax = ((rax).wrapping_add(0x11u64)) as u64; rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
    rax = P2;
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142407f20(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x143c4aa00; const P1: u64 = 0x143407818;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = P0;
    rax = (crate::simd_ops::ru64(rax.wrapping_add(rcx.wrapping_mul(8)))) as u64;
    fl_l = rax; fl_r = rax;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { fl_l = ((rax).wrapping_add(0x11u64)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; return rax; }
    rax = P1;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142407f50(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x140000000; const P2: u64 = 0x3c4ac00; const P3: u64 = 0x3c4aa00;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64;
    r9 = P1;
    fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64;
    if !(((((fl_l as u32)) & ((fl_r as u32))) == 0)) { rdx = ((rax as u32)) as u32 as u64; r8 = ((rax as u32)) as u32 as u64; rdx = ((rdx).wrapping_shr(6u32)) as u64; r8 = ((((r8 as u32)) & (0x3fu32))) as u32 as u64; rax = (crate::simd_ops::ru64(r9.wrapping_add(rdx.wrapping_mul(8)).wrapping_add(P2))) as u64; fl_l = (((rax) >> (((r8) as u32 & 63))) & 1) as u64; rax = ((rax) & !((1u64) << (((r8) as u32 & 63)))) as u64; crate::simd_ops::wu64(r9.wrapping_add(rdx.wrapping_mul(8)).wrapping_add(P2), rax); }
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    crate::simd_ops::wu64(r9.wrapping_add(rax.wrapping_mul(8)).wrapping_add(P3), (0x0u64) as u64);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142408260(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x143c4aa00;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = P0;
    rax = (crate::simd_ops::ru64(rax.wrapping_add(rcx.wrapping_mul(8)))) as u64;
    fl_l = rax; fl_r = rax;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rax = (crate::simd_ops::ru32(rax)) as u32 as u64; return rax; }
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142408280(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x143c4ac0c;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = ((rcx as u32)) as u32 as u64;
    rcx = P0;
    rax = (crate::simd_ops::ru32(rcx.wrapping_add(rax.wrapping_mul(8)))) as u32 as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142408310(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    crate::simd_ops::forward_incoming_to_outgoing();
    const P0: u64 = 0x80; const P1: u64 = 0xffffffffffffff80; const P2: u64 = 0xffffffffffffff90; const P3: u64 = 0xffffffffffffffa0; const P4: u64 = 0xffffffffffffffb0; const P5: u64 = 0xffffffffffffffc0; const P6: u64 = 0xffffffffffffffd0; const P7: u64 = 0xffffffffffffffe0; const P8: u64 = 0xfffffffffffffff0; const P9: u64 = 0x142408090;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = ((crate::exe::tls_pointer() as u64)) as u64; r8 = 0xa86b8; r9 = 0xa86c0; rdx = (crate::simd_ops::ru64(rax)) as u64; rax = (crate::simd_ops::ru32(r8.wrapping_add(rdx))) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 2; continue; } fl_l = ((((rax as u32)) | (0x1u32))) as u32 as u64; fl_r = fl_l; rax = ((fl_l as u32)) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(rdx), ((rax as u32)) as u32); r8 = ((rdx.wrapping_add(r9)) as u64) as u64; rax = r8; rdx = 0x2; bb = 1; continue; }
            1 => { rax = ((rax.wrapping_add(P0)) as u64) as u64; xmm0 = (crate::simd_ops::ru128(rcx)) as u128; rcx = ((rcx.wrapping_add(P0)) as u64) as u64; crate::simd_ops::wu128(rax.wrapping_add(P1), (xmm0) as u128); xmm1 = (crate::simd_ops::ru128(rcx.wrapping_add(P2))) as u128; crate::simd_ops::wu128(rax.wrapping_add(P2), (xmm1) as u128); xmm0 = (crate::simd_ops::ru128(rcx.wrapping_add(P3))) as u128; crate::simd_ops::wu128(rax.wrapping_add(P3), (xmm0) as u128); xmm1 = (crate::simd_ops::ru128(rcx.wrapping_add(P4))) as u128; crate::simd_ops::wu128(rax.wrapping_add(P4), (xmm1) as u128); xmm0 = (crate::simd_ops::ru128(rcx.wrapping_add(P5))) as u128; crate::simd_ops::wu128(rax.wrapping_add(P5), (xmm0) as u128); xmm1 = (crate::simd_ops::ru128(rcx.wrapping_add(P6))) as u128; crate::simd_ops::wu128(rax.wrapping_add(P6), (xmm1) as u128); xmm0 = (crate::simd_ops::ru128(rcx.wrapping_add(P7))) as u128; crate::simd_ops::wu128(rax.wrapping_add(P7), (xmm0) as u128); xmm1 = (crate::simd_ops::ru128(rcx.wrapping_add(P8))) as u128; crate::simd_ops::wu128(rax.wrapping_add(P8), (xmm1) as u128); fl_l = rdx; fl_r = (0x1u64) as u64; rdx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } rcx = r8; crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(P9, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::set_xmm_ret(c_x0); return c_rax; }
            2 => { r8 = ((rdx.wrapping_add(r9)) as u64) as u64; rcx = r8; crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(P9, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::set_xmm_ret(c_x0); return c_rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142408440(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142407c80; const P1: u64 = 0x143c4aa00;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rax = ((rax as u32)) as u32 as u64;
    rcx = P1;
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(rax.wrapping_mul(8)))) as u64;
    fl_l = rax; fl_r = rax;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rax = (crate::simd_ops::ru32(rax)) as u32 as u64; rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142408e00(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    crate::simd_ops::forward_incoming_to_outgoing();
    const P0: u64 = 0x142409500;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rdx = (rdx & !0xffu64) | (((0x5cu8) as u8 as u64));
    crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8(P0, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14240b140(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141378aa0; const P1: u64 = 0x8; const P2: u64 = 0x1401e1a40;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x50u64)) as u64; xmm0 = (crate::simd_ops::ru128(r8)) as u128; xmm1 = (crate::simd_ops::ru128(rdx)) as u128; rax = ((crate::simd_ops::ru8(rsp.wrapping_add(0x80u64))) as u32) as u32 as u64; rbp = r8; rdi = rdx; crate::simd_ops::wu8(rsp.wrapping_add(0x20u64), ((rax as u8)) as u8); r8 = rsp.wrapping_add(0x30); rbx = (((r9 as u8)) as u32) as u32 as u64; rdx = rsp.wrapping_add(0x40); rsi = rcx; crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm0) as u128); crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm1) as u128); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru64(rsi.wrapping_add(P1))) as u64; rax = (crate::simd_ops::ru64(rsi)) as u64; r14 = (crate::simd_ops::ru64(rdi)) as u64; rax = ((rax).wrapping_add(1)) as u64; rcx = ((rcx).wrapping_sub(r14)) as u64; rax = ((rax).wrapping_add(rcx)) as u64; fl_l = ((rbx as u8)) as u8 as u64; fl_r = ((rbx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 1; continue; } xmm0 = (crate::simd_ops::ru128(rbp)) as u128; r8 = rax; rdx = rsp.wrapping_add(0x40); rcx = rdi; crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm0) as u128); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 1; continue; }
            1 => { rdx = (crate::simd_ops::ru64(rdi.wrapping_add(P1))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; fl_l = rax; fl_r = rdx; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rcx = rdx; rcx = (if (((fl_l as u64)) < ((fl_r as u64))) { rax } else { rcx }) as u64; rdx = ((rdx).wrapping_sub(rcx)) as u64; crate::simd_ops::wu64(rdi.wrapping_add(P1), rdx); rax = ((r14.wrapping_add(rcx)) as u64) as u64; crate::simd_ops::wu64(rdi, rax); rax = rsi; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; rsp = ((rsp).wrapping_add(0x50u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14240b250(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142535070; const P1: u64 = 0x1426456c8; const P2: u64 = 0x1426453a0; const P3: u64 = 0x14020bcb0; const P4: u64 = 0x14183acbc;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; r12 = r8; rsi = rdx; rbp = rcx; r15 = (crate::simd_ops::ru64(r8.wrapping_add(0x8u64))) as u64; rdi = rcx.wrapping_add(0x60); rax = (crate::simd_ops::ru32(rdi)) as u32 as u64; rax = ((rax).wrapping_add(r15)) as u64; fl_l = rax; fl_r = (0x100000u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 4; continue; } r14 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rdi, ((r14 as u32)) as u32); r9 = 0x6348; rax = ((crate::exe::tls_pointer() as u64)) as u64; rcx = (crate::simd_ops::ru64(rax)) as u64; rdx = 0x100000; r8 = ((r14.wrapping_add(0x10u64)) as u32) as u32 as u64; rcx = (crate::simd_ops::ru64(r9.wrapping_add(rcx))) as u64; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 1; continue; } rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P1)) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = rax; rdx = 0xdeadd00d; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P2)) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 2; continue; }
            1 => { r14 = rax; bb = 2; continue; }
            2 => { rdx = (crate::simd_ops::ru32(rbp.wrapping_add(0x58u64))) as u32 as u64; rcx = (crate::simd_ops::ru32(rbp.wrapping_add(0x5cu64))) as u32 as u64; rcx = ((rcx).wrapping_shl(3u32)) as u64; rax = ((rdx.wrapping_mul(8)) as u64) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } rdx = ((rdx).wrapping_add(1)) as u64; rcx = rbp.wrapping_add(0x50); rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rdi = rbp.wrapping_add(0x60); bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru32(rbp.wrapping_add(0x58u64))) as u32 as u64; rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x50u64))) as u64; crate::simd_ops::wu64(rax.wrapping_add(rcx.wrapping_mul(8)), r14); fl_l = (crate::simd_ops::lk_add_u32(rbp.wrapping_add(0x58u64), (1) as u32)) as u32 as u64; bb = 4; continue; }
            4 => { rcx = (crate::simd_ops::ru32(rbp.wrapping_add(0x58u64))) as u32 as u64; rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x50u64))) as u64; rbx = (crate::simd_ops::ru32(rdi)) as u32 as u64; rbx = ((rbx).wrapping_add(crate::simd_ops::ru64(rax.wrapping_add(rcx.wrapping_mul(8)).wrapping_sub(0x8u64)))) as u64; r8 = r15; rdx = (crate::simd_ops::ru64(r12)) as u64; rcx = rbx; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu8(rbx.wrapping_add(r15), ((rax as u8)) as u8); rax = ((r15.wrapping_add(0x1u64)) as u32) as u32 as u64; fl_l = (crate::simd_ops::lk_add_u32(rdi, ((rax as u32)) as u32)) as u32 as u64; fl_r = fl_l; crate::simd_ops::wu64(rsi, rbx); rax = (0xffffffffffffffffu64) as u64; bb = 5; continue; }
            5 => { rax = ((rax).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(rax))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 5; continue; } crate::simd_ops::wu64(rsi.wrapping_add(0x8u64), rax); rax = rsi; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14240b560(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142645790; const P1: u64 = 0x48; const P2: u64 = 0x40; const P3: u64 = 0x141379040; const P4: u64 = 0x8; const P5: u64 = 0x1413790c0; const P6: u64 = 0x18;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        core::sync::atomic::fence(core::sync::atomic::Ordering::SeqCst);
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), r8); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = rdx; rsi = rcx; (rax, xmm0) = crate::simd_ops::call_stk_xmm((crate::simd_ops::ru64(P0)) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r8 = ((rax as u32)) as u32 as u64; fl_l = (crate::simd_ops::ru64(rsi.wrapping_add(P1))) as u64; fl_r = r8; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } fl_l = (crate::simd_ops::atomic_add_u64(rsi.wrapping_add(P2), 1)) as u64; bb = 2; continue; }
            1 => { r8 = 0x1; rcx = ((rsi.wrapping_add(P2)) as u64) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 2; continue; }
            2 => { rax = 0; fl_l = 0; fl_r = 0; r8 = rsp.wrapping_add(0x40); rdx = rsp.wrapping_add(0x30); crate::simd_ops::wu64(rdi, rax); rcx = rsi; crate::simd_ops::wu64(rdi.wrapping_add(P4), rax); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru64(rax)) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(rsi.wrapping_add(P4))) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 3; continue; } xmm0 = (crate::simd_ops::ru128(rax.wrapping_add(P6))) as u128; crate::simd_ops::wu128(rdi, (xmm0) as u128); bb = 3; continue; }
            3 => { let _ = crate::simd_ops::atomic_sub_u64(rsi.wrapping_add(P2), (1) as u64); rax = rdi; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14240b5f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14240b380;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x38u64)) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r8);
    rax = (0xffffffffffffffffu64) as u64;
    loop { rax = ((rax).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru8(r8.wrapping_add(rax))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if !((((fl_l as u8)) != ((fl_r as u8)))) { break; } }
    r8 = rsp.wrapping_add(0x20);
    crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax);
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x38u64)) as u64;
    let _ = &stack; return rax;
}
