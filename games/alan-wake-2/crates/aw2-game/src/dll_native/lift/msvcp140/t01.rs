#![allow(unused_imports, unused_variables, unused_mut, unused_assignments, non_snake_case, dead_code, clippy::all, unreachable_code, unused_parens)]

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e18050(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001e6ba90; const P1: u64 = 0x8; const P2: u64 = 0x7001e16f90; const P3: u64 = 0x10; const P4: u64 = 0x20;
    let (mut rax, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rsp = rsp.wrapping_sub(8);
    core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64);
    rsp = ((rsp).wrapping_sub(0x40u64)) as u64;
    rax = ((P0) as u64) as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(P1), ((r8 as u32)) as u32);
    crate::simd_ops::wu64(rcx, (rax) as u64);
    rbx = (rcx) as u64;
    rcx = ((rsp.wrapping_add(0x20u64)) as u64) as u64;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0;
    xmm0 = (crate::simd_ops::ru128(rax)) as u128;
    crate::simd_ops::wu128(rbx.wrapping_add(P3), (xmm0) as u128);
    xmm1 = (crate::simd_ops::ru128(rax.wrapping_add(P3))) as u128;
    rax = (rbx) as u64;
    crate::simd_ops::wu128(rbx.wrapping_add(P4), (xmm1) as u128);
    rsp = ((rsp).wrapping_add(0x40u64)) as u64;
    rbx = core::ptr::read_unaligned(rsp as *const u64);
    rsp = rsp.wrapping_add(8);
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e18090(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x7001e6ba90; const P2: u64 = 0x7001e6b880; const P3: u64 = 0x7001e184f0; const P4: u64 = 0x7001e16f90; const P5: u64 = 0x10; const P6: u64 = 0x20; const P7: u64 = 0x7001e18dc0; const P8: u64 = 0x18; const P9: u64 = 0x7001e693a8; const P10: u64 = 0x7001e6649c; const P11: u64 = 0x28; const P12: u64 = 0x7001e25930; const P13: u64 = 0x7001e17a54;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 23>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&super::EH_META[29], rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { r11 = (rsp) as u64; crate::simd_ops::wu64(r11.wrapping_add(0x10u64), (rbx) as u64); crate::simd_ops::wu64(r11.wrapping_add(0x18u64), (rbp) as u64); crate::simd_ops::wu64(r11.wrapping_add(0x20u64), (rsi) as u64); crate::simd_ops::wu64(r11.wrapping_add(0x8u64), (rcx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = ((rsp).wrapping_sub(0xb0u64)) as u64; rbp = (((r8 as u8)) as u32) as u32 as u64; rsi = (rdx) as u64; rdi = (rcx) as u64; crate::simd_ops::wu32(rcx.wrapping_add(P0), ((r9 as u32)) as u32); rax = ((P1) as u64) as u64; crate::simd_ops::wu64(rcx, (rax) as u64); rdx = ((P2) as u64) as u64; rcx = ((r11.wrapping_sub(0x78u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e180ce); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = ((rsp.wrapping_add(0x20u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e180d8); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; xmm0 = (crate::simd_ops::ru128(rax)) as u128; crate::simd_ops::wu128(rdi.wrapping_add(P5), (xmm0) as u128); xmm1 = (crate::simd_ops::ru128(rax.wrapping_add(P5))) as u128; crate::simd_ops::wu128(rdi.wrapping_add(P6), (xmm1) as u128); rcx = ((rsp.wrapping_add(0x40u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e180f1); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P7) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rbx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; fl_l = (crate::simd_ops::ru32(rdi.wrapping_add(P6))) as u32 as u64; fl_r = ((rbx as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) <= (((fl_r as u32)) as i32)) { bb = 2; continue; } bb = 1; continue; }
            1 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(P8))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e18101); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P9)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 4; continue; }
            2 => { if ((((fl_l as u32)) as i32) >= (((fl_r as u32)) as i32)) { bb = 4; continue; } bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(P8))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e1810f); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P10) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 4; continue; }
            4 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(P11))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e18118); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P9)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; fl_l = (rsi) as u64 as u64; fl_r = (rsi) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { rbx = ((rbp as u32)) as u32 as u64; rbx = ((((rbx as u32)) ^ (0x1u32))) as u32 as u64; fl_l = (((rbx as u32)).wrapping_sub(1)) as u32 as u64; rbx = ((fl_l as u32)) as u32 as u64; bb = 7; continue; }
            6 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e1812c); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P12) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = (rax) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e18134); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P13) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rsi = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; bb = 7; continue; }
            7 => { crate::simd_ops::wu64(rdi.wrapping_add(0x18u64), (rsi) as u64); crate::simd_ops::wu32(rdi.wrapping_add(0x20u64), ((rbx as u32)) as u32); rax = (rdi) as u64; r11 = ((rsp.wrapping_add(0xb0u64)) as u64) as u64; rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x18u64))) as u64; rbp = (crate::simd_ops::ru64(r11.wrapping_add(0x20u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x28u64))) as u64; rsp = (r11) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e181d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x7001e6bb70; const P2: u64 = 0x7001e6b880; const P3: u64 = 0x7001e184f0; const P4: u64 = 0x7001e16f90; const P5: u64 = 0x10; const P6: u64 = 0x20; const P7: u64 = 0x7001e4d900; const P8: u64 = 0x30; const P9: u64 = 0x40; const P10: u64 = 0x50; const P11: u64 = 0x28; const P12: u64 = 0x7001e18dc0;
    let (mut rax, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 25>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&super::EH_META[30], rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rcx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64); rsp = ((rsp).wrapping_sub(0xc0u64)) as u64; rbx = (rcx) as u64; crate::simd_ops::wu32(rcx.wrapping_add(P0), ((rdx as u32)) as u32); rax = ((P1) as u64) as u64; crate::simd_ops::wu64(rcx, (rax) as u64); rdx = ((P2) as u64) as u64; rcx = ((rsp.wrapping_add(0x50u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e181f9); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = ((rsp.wrapping_add(0x20u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e18203); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; xmm0 = (crate::simd_ops::ru128(rax)) as u128; crate::simd_ops::wu128(rbx.wrapping_add(P5), (xmm0) as u128); xmm1 = (crate::simd_ops::ru128(rax.wrapping_add(P5))) as u128; crate::simd_ops::wu128(rbx.wrapping_add(P6), (xmm1) as u128); rcx = ((rsp.wrapping_add(0x20u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e1821c); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P7) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; xmm0 = (crate::simd_ops::ru128(rax)) as u128; crate::simd_ops::wu128(rbx.wrapping_add(P8), (xmm0) as u128); xmm1 = (crate::simd_ops::ru128(rax.wrapping_add(P5))) as u128; crate::simd_ops::wu128(rbx.wrapping_add(P9), (xmm1) as u128); xmm0 = ((crate::simd_ops::ru64(rax.wrapping_add(P6))) as u64) as u128; crate::simd_ops::wu64(rbx.wrapping_add(P10), ((xmm0 as u64)) as u64); rax = (crate::simd_ops::ru32(rax.wrapping_add(P11))) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x58u64), ((rax as u32)) as u32); rcx = ((rsp.wrapping_add(0x50u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e18245); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P12) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = (rbx) as u64; rsp = ((rsp).wrapping_add(0xc0u64)) as u64; rbx = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e182d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x7001e6baf0; const P2: u64 = 0x7001e6b880; const P3: u64 = 0x7001e184f0; const P4: u64 = 0x7001e16f90; const P5: u64 = 0x10; const P6: u64 = 0x20; const P7: u64 = 0x7001e4d900; const P8: u64 = 0x30; const P9: u64 = 0x40; const P10: u64 = 0x50; const P11: u64 = 0x28; const P12: u64 = 0x7001e18dc0;
    let (mut rax, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 25>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&super::EH_META[31], rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rcx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64); rsp = ((rsp).wrapping_sub(0xc0u64)) as u64; rbx = (rcx) as u64; crate::simd_ops::wu32(rcx.wrapping_add(P0), ((rdx as u32)) as u32); rax = ((P1) as u64) as u64; crate::simd_ops::wu64(rcx, (rax) as u64); rdx = ((P2) as u64) as u64; rcx = ((rsp.wrapping_add(0x50u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e182f9); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = ((rsp.wrapping_add(0x20u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e18303); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; xmm0 = (crate::simd_ops::ru128(rax)) as u128; crate::simd_ops::wu128(rbx.wrapping_add(P5), (xmm0) as u128); xmm1 = (crate::simd_ops::ru128(rax.wrapping_add(P5))) as u128; crate::simd_ops::wu128(rbx.wrapping_add(P6), (xmm1) as u128); rcx = ((rsp.wrapping_add(0x20u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e1831c); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P7) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; xmm0 = (crate::simd_ops::ru128(rax)) as u128; crate::simd_ops::wu128(rbx.wrapping_add(P8), (xmm0) as u128); xmm1 = (crate::simd_ops::ru128(rax.wrapping_add(P5))) as u128; crate::simd_ops::wu128(rbx.wrapping_add(P9), (xmm1) as u128); xmm0 = ((crate::simd_ops::ru64(rax.wrapping_add(P6))) as u64) as u128; crate::simd_ops::wu64(rbx.wrapping_add(P10), ((xmm0 as u64)) as u64); rax = (crate::simd_ops::ru32(rax.wrapping_add(P11))) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x58u64), ((rax as u32)) as u32); rcx = ((rsp.wrapping_add(0x50u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e18345); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P12) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = (rbx) as u64; rsp = ((rsp).wrapping_add(0xc0u64)) as u64; rbx = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e18380(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x7001e6b590;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = ((P0) as u64) as u64;
    crate::simd_ops::wu64(rcx, (rax) as u64);
    rax = (rcx) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e18390(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x7001e6b8b0; const P2: u64 = 0x10; const P3: u64 = 0x18; const P4: u64 = 0x20; const P5: u64 = 0x24; const P6: u64 = 0x28; const P7: u64 = 0x30; const P8: u64 = 0x7001e18ef0; const P9: u64 = 0x7001e22f90;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&super::EH_META[32], rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rcx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = (rdx) as u64; rbx = (rcx) as u64; crate::simd_ops::wu32(rcx.wrapping_add(P0), (0x1u32) as u32); rax = ((P1) as u64) as u64; crate::simd_ops::wu64(rcx, (rax) as u64); r8 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rcx.wrapping_add(P2), (r8) as u64); rax = (crate::simd_ops::ru64(rdx.wrapping_add(P3))) as u64; crate::simd_ops::wu64(rcx.wrapping_add(P3), (rax) as u64); rax = (crate::simd_ops::ru32(rdx.wrapping_add(P4))) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(P4), ((rax as u32)) as u32); rax = (rax & !0xffu64) | (((crate::simd_ops::ru8(rdx.wrapping_add(P5))) as u8 as u64)); crate::simd_ops::wu8(rcx.wrapping_add(P5), ((rax as u8)) as u8); rcx = ((rcx).wrapping_add(0x28u64)) as u64; rdx = (crate::simd_ops::ru64(rdx.wrapping_add(P6))) as u64; fl_l = (rdx) as u64 as u64; fl_r = (rdx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rdx = ((rdi.wrapping_add(P7)) as u64) as u64; bb = 2; continue; }
            2 => { crate::simd_ops::wu64(rcx, (r8) as u64); crate::simd_ops::wu8(rcx.wrapping_add(P0), ((r8 as u8)) as u8); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e183e9); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P8) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rdx = (rdi) as u64; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e183f5); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P9) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rax = (rbx) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e18410(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x24; const P1: u64 = 0x7001e6b8b0; const P2: u64 = 0x7001e6b8c8; const P3: u64 = 0x8; const P4: u64 = 0x10; const P5: u64 = 0x18; const P6: u64 = 0x20; const P7: u64 = 0x7001e18ef0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    crate::simd_ops::wu8(rcx.wrapping_add(P0), ((rdx as u8)) as u8);
    rax = ((P1) as u64) as u64;
    crate::simd_ops::wu64(rcx, (rax) as u64);
    rdx = ((P2) as u64) as u64;
    rax = (0u32) as u32 as u64;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu32(rcx.wrapping_add(P3), (0x1u32) as u32);
    crate::simd_ops::wu64(rcx.wrapping_add(P4), (rax) as u64);
    rbx = (rcx) as u64;
    crate::simd_ops::wu64(rcx.wrapping_add(P5), (rax) as u64);
    crate::simd_ops::wu32(rcx.wrapping_add(P6), ((rax as u32)) as u32);
    rcx = ((rcx).wrapping_add(0x28u64)) as u64;
    crate::simd_ops::wu64(rcx, (rax) as u64);
    crate::simd_ops::wu8(rcx.wrapping_add(P3), ((rax as u8)) as u8);
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P7) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    rax = (rbx) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = core::ptr::read_unaligned(rsp as *const u64);
    rsp = rsp.wrapping_add(8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e18460(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001e49e20; const P1: u64 = 0x8; const P2: u64 = 0x10; const P3: u64 = 0x18; const P4: u64 = 0x20; const P5: u64 = 0x28; const P6: u64 = 0x30; const P7: u64 = 0x38; const P8: u64 = 0x40; const P9: u64 = 0x48; const P10: u64 = 0x50; const P11: u64 = 0x58; const P12: u64 = 0x7001e23140; const P13: u64 = 0x7001e6b5a8; const P14: u64 = 0x7001e4d5b0; const P15: u64 = 0x7001e184e4;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&super::EH_META[33], rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (rsi) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rcx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = (r8) as u64; rsi = ((rdx as u32)) as u32 as u64; rbx = (rcx) as u64; rdx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e1847e); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rax = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rbx.wrapping_add(P1), (rax) as u64); crate::simd_ops::wu8(rbx.wrapping_add(P2), ((rax as u8)) as u8); crate::simd_ops::wu64(rbx.wrapping_add(P3), (rax) as u64); crate::simd_ops::wu8(rbx.wrapping_add(P4), ((rax as u8)) as u8); crate::simd_ops::wu64(rbx.wrapping_add(P5), (rax) as u64); crate::simd_ops::wu16(rbx.wrapping_add(P6), ((rax as u16)) as u16); crate::simd_ops::wu64(rbx.wrapping_add(P7), (rax) as u64); crate::simd_ops::wu16(rbx.wrapping_add(P8), ((rax as u16)) as u16); crate::simd_ops::wu64(rbx.wrapping_add(P9), (rax) as u64); crate::simd_ops::wu8(rbx.wrapping_add(P10), ((rax as u8)) as u8); crate::simd_ops::wu64(rbx.wrapping_add(P11), (rax) as u64); crate::simd_ops::wu8(rbx.wrapping_add(0x60u64), ((rax as u8)) as u8); fl_l = (rdi) as u64 as u64; fl_r = (rdi) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { r8 = (rdi) as u64; rdx = ((rsi as u32)) as u32 as u64; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e184bf); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P12) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rax = (rbx) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            2 => { rcx = ((P13) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e184df); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P14) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); crate::lift_ops::trap(0x80000003, P15, rcx, rsp); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e184f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001e49e20; const P1: u64 = 0x8; const P2: u64 = 0x10; const P3: u64 = 0x18; const P4: u64 = 0x20; const P5: u64 = 0x28; const P6: u64 = 0x30; const P7: u64 = 0x38; const P8: u64 = 0x40; const P9: u64 = 0x48; const P10: u64 = 0x50; const P11: u64 = 0x58; const P12: u64 = 0x7001e257b0; const P13: u64 = 0x7001e6b5a8; const P14: u64 = 0x7001e4d5b0; const P15: u64 = 0x7001e18566;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&super::EH_META[34], rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rcx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = (rdx) as u64; rbx = (rcx) as u64; rdx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e18507); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rax = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rbx.wrapping_add(P1), (rax) as u64); crate::simd_ops::wu8(rbx.wrapping_add(P2), ((rax as u8)) as u8); crate::simd_ops::wu64(rbx.wrapping_add(P3), (rax) as u64); crate::simd_ops::wu8(rbx.wrapping_add(P4), ((rax as u8)) as u8); crate::simd_ops::wu64(rbx.wrapping_add(P5), (rax) as u64); crate::simd_ops::wu16(rbx.wrapping_add(P6), ((rax as u16)) as u16); crate::simd_ops::wu64(rbx.wrapping_add(P7), (rax) as u64); crate::simd_ops::wu16(rbx.wrapping_add(P8), ((rax as u16)) as u16); crate::simd_ops::wu64(rbx.wrapping_add(P9), (rax) as u64); crate::simd_ops::wu8(rbx.wrapping_add(P10), ((rax as u8)) as u8); crate::simd_ops::wu64(rbx.wrapping_add(P11), (rax) as u64); crate::simd_ops::wu8(rbx.wrapping_add(0x60u64), ((rax as u8)) as u8); fl_l = (rdi) as u64 as u64; fl_r = (rdi) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rdx = (rdi) as u64; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e18546); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P12) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rax = (rbx) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            2 => { rcx = ((P13) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e18561); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P14) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); crate::lift_ops::trap(0x80000003, P15, rcx, rsp); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e18568(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001e97380; const P1: u64 = 0x7001e175dc; const P2: u64 = 0x7001e1a87c; const P3: u64 = 0x7001e6b378; const P4: u64 = 0x7001e6642c; const P5: u64 = 0x7001e6b4b8; const P6: u64 = 0x7001e69480; const P7: u64 = 0x7001e18646; const P8: u64 = 0x7001e6649c; const P9: u64 = 0x7001e6b4d0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 19>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rbp = (rsp) as u64; rsp = ((rsp).wrapping_sub(0x80u64)) as u64; rax = (crate::simd_ops::ru64(P0)) as u64; rax = (((rax) ^ (rsp))) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x10u64), (rax) as u64); crate::simd_ops::wu64(rbp.wrapping_sub(0x60u64), (rcx) as u64); rax = (r8) as u64; crate::simd_ops::lk_and_u64(rbp.wrapping_sub(0x40u64), (0x0u64) as u64); xmm0 = 0u128; crate::simd_ops::lk_and_u64(rbp.wrapping_sub(0x38u64), (0x0u64) as u64); rdi = (rdx) as u64; r8 = (crate::simd_ops::ru64(r8.wrapping_add(0x10u64))) as u64; rbx = (rcx) as u64; fl_l = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64 as u64; fl_r = (0xfu64) as u64 as u64; crate::simd_ops::wu128(rbp.wrapping_sub(0x50u64), (xmm0) as u128); if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(rax)) as u64; bb = 2; continue; }
            2 => { rdx = (rax) as u64; rcx = ((rbp.wrapping_sub(0x50u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; xmm0 = (crate::simd_ops::ru128(rdi)) as u128; r8 = ((rbp.wrapping_sub(0x50u64)) as u64) as u64; rdx = ((rbp.wrapping_sub(0x60u64)) as u64) as u64; rcx = ((rbp.wrapping_sub(0x30u64)) as u64) as u64; crate::simd_ops::wu128(rbp.wrapping_sub(0x60u64), (xmm0) as u128); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; fl_l = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64 as u64; fl_r = (0xfu64) as u64 as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rax)) as u64; bb = 4; continue; }
            4 => { rcx = ((P3) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x60u64), (rax) as u64); crate::simd_ops::wu64(rbx, (rcx) as u64); rdx = ((rbx.wrapping_add(0x8u64)) as u64) as u64; xmm0 = 0u128; crate::simd_ops::wu8(rbp.wrapping_sub(0x58u64), (0x1u8) as u8); rcx = ((rbp.wrapping_sub(0x60u64)) as u64) as u64; crate::simd_ops::wu128(rdx, (xmm0) as u128); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x18u64))) as u64; rax = ((P5) as u64) as u64; crate::simd_ops::wu64(rbx, (rax) as u64); fl_l = (rdx) as u64 as u64; fl_r = (0xfu64) as u64 as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 9; continue; } bb = 5; continue; }
            5 => { rcx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x30u64))) as u64; rdx = ((rdx).wrapping_add(1)) as u64; rax = (rcx) as u64; fl_l = (rdx) as u64 as u64; fl_r = (0x1000u64) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 8; continue; } bb = 6; continue; }
            6 => { rcx = (crate::simd_ops::ru64(rcx.wrapping_sub(0x8u64))) as u64; rdx = ((rdx).wrapping_add(0x27u64)) as u64; rax = ((rax).wrapping_sub(rcx)) as u64; rax = ((rax).wrapping_add(0xfffffffffffffff8u64)) as u64; fl_l = (rax) as u64 as u64; fl_r = (0x1fu64) as u64 as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 8; continue; } bb = 7; continue; }
            7 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P6)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::lift_ops::trap(0x80000003, P7, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            8 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P8) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 9; continue; }
            9 => { xmm0 = (crate::simd_ops::ru128(rdi)) as u128; rax = ((P9) as u64) as u64; crate::simd_ops::wu64(rbx, (rax) as u64); rax = (rbx) as u64; crate::simd_ops::wu128(rbx.wrapping_add(0x18u64), (xmm0) as u128); rcx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x10u64))) as u64; rcx = (((rcx) ^ (rsp))) as u64; rsp = ((rsp).wrapping_add(0x80u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rbx = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rbp = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e186d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001e693a8;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rbx) as u64);
    rsp = rsp.wrapping_sub(8);
    core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    crate::simd_ops::lk_and_u64(rcx, (0x0u64) as u64);
    rdi = (rdx) as u64;
    rbx = (rcx) as u64;
    fl_l = (rcx) as u64 as u64; fl_r = (rdx) as u64 as u64;
    if !((((fl_l as u64)) == ((fl_r as u64)))) { rcx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P0)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rax = (crate::simd_ops::ru64(rdi)) as u64; crate::simd_ops::wu64(rbx, (rax) as u64); fl_l = (crate::simd_ops::lk_and_u64(rdi, (0x0u64) as u64)) as u64; fl_r = fl_l; }
    rax = (rbx) as u64;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = core::ptr::read_unaligned(rsp as *const u64);
    rsp = rsp.wrapping_add(8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e18710(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    crate::simd_ops::wu64(rcx, (rdx) as u64);
    rax = (rcx) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e187f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x7001e6ba70; const P1: u64 = 0x8; const P2: u64 = 0x7001e18801; const P3: u64 = 0x7001e18802; const P4: u64 = 0x7001e18803;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = ((P0) as u64) as u64; crate::simd_ops::wu32(rcx.wrapping_add(P1), ((rdx as u32)) as u32); crate::simd_ops::wu64(rcx, (rax) as u64); rax = (rcx) as u64; return rax; }
            1 => { crate::lift_ops::trap(0x80000003, P2, rcx, 0); return rax; }
            2 => { crate::lift_ops::trap(0x80000003, P3, rcx, 0); return rax; }
            3 => { crate::lift_ops::trap(0x80000003, P4, rcx, 0); return rax; }
            4 => { crate::simd_ops::wu32(rcx, ((rdx as u32)) as u32); rax = (rcx) as u64; crate::simd_ops::wu64(rcx.wrapping_add(P1), (r8) as u64); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e18804(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    crate::simd_ops::wu32(rcx, ((rdx as u32)) as u32);
    rax = (rcx) as u64;
    crate::simd_ops::wu64(rcx.wrapping_add(P0), (r8) as u64);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e1887c(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001e18974; const P1: u64 = 0x7001e6bbf0;
    let (mut rax, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rsp = rsp.wrapping_sub(8);
    core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64);
    rsp = ((rsp).wrapping_sub(0x30u64)) as u64;
    xmm0 = (crate::simd_ops::ru128(r8)) as u128;
    r8 = (rdx) as u64;
    rbx = (rcx) as u64;
    rdx = ((rsp.wrapping_add(0x20u64)) as u64) as u64;
    crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128);
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0;
    rax = ((P1) as u64) as u64;
    crate::simd_ops::wu64(rbx, (rax) as u64);
    rax = (rbx) as u64;
    rsp = ((rsp).wrapping_add(0x30u64)) as u64;
    rbx = core::ptr::read_unaligned(rsp as *const u64);
    rsp = rsp.wrapping_add(8);
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e188b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x7001e6bc08; const P1: u64 = 0x8; const P2: u64 = 0x10; const P3: u64 = 0x18; const P4: u64 = 0x20; const P5: u64 = 0x28; const P6: u64 = 0x30; const P7: u64 = 0x38; const P8: u64 = 0x40;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = ((P0) as u64) as u64;
    crate::simd_ops::wu64(rcx, (rax) as u64);
    rax = (0u32) as u32 as u64;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu64(rcx.wrapping_add(P1), (rax) as u64);
    crate::simd_ops::wu64(rcx.wrapping_add(P2), (rax) as u64);
    crate::simd_ops::wu32(rcx.wrapping_add(P3), ((rax as u32)) as u32);
    crate::simd_ops::wu64(rcx.wrapping_add(P4), (rax) as u64);
    crate::simd_ops::wu64(rcx.wrapping_add(P5), (rax) as u64);
    crate::simd_ops::wu64(rcx.wrapping_add(P6), (rax) as u64);
    crate::simd_ops::wu64(rcx.wrapping_add(P7), (rax) as u64);
    crate::simd_ops::wu64(rcx.wrapping_add(P8), (rax) as u64);
    rax = (rcx) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e18974(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001e97380; const P1: u64 = 0x7001e174fc; const P2: u64 = 0x7001e18568; const P3: u64 = 0xfffffffffffffff8; const P4: u64 = 0x7001e69480; const P5: u64 = 0x7001e18a1e; const P6: u64 = 0x7001e6649c; const P7: u64 = 0x7001e6b4f0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&super::EH_META[35], rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rbx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = ((rsp).wrapping_sub(0x60u64)) as u64; rax = (crate::simd_ops::ru64(P0)) as u64; rax = (((rax) ^ (rsp))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), (rax) as u64); rax = (r8) as u64; rdi = (rdx) as u64; rbx = (rcx) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rcx) as u64); xmm0 = 0u128; crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm0) as u128); xmm1 = 0u128; crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm1) as u128); fl_l = (((r8) | (0xffffffffffffffffu64))) as u64 as u64; fl_r = fl_l; r8 = ((fl_l as u64)) as u64; bb = 1; continue; }
            1 => { r8 = ((r8).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru8(rax.wrapping_add(r8))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 1; continue; } bb = 2; continue; }
            2 => { rdx = (rax) as u64; rcx = ((rsp.wrapping_add(0x30u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e189c2); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; xmm0 = (crate::simd_ops::ru128(rdi)) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128); r8 = ((rsp.wrapping_add(0x30u64)) as u64) as u64; rdx = ((rsp.wrapping_add(0x20u64)) as u64) as u64; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e189de); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; fl_l = (rdx) as u64 as u64; fl_r = (0xfu64) as u64 as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 7; continue; } bb = 3; continue; }
            3 => { rdx = ((rdx).wrapping_add(1)) as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = (rcx) as u64; fl_l = (rdx) as u64 as u64; fl_r = (0x1000u64) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 6; continue; } bb = 4; continue; }
            4 => { rdx = ((rdx).wrapping_add(0x27u64)) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P3))) as u64; rax = ((rax).wrapping_sub(rcx)) as u64; rax = ((rax).wrapping_add(0xfffffffffffffff8u64)) as u64; fl_l = (rax) as u64 as u64; fl_r = (0x1fu64) as u64 as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 6; continue; } bb = 5; continue; }
            5 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e18a18); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P4)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::lift_ops::trap(0x80000003, P5, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            6 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e18a1f); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 7; continue; }
            7 => { rax = ((P7) as u64) as u64; crate::simd_ops::wu64(rbx, (rax) as u64); rax = (rbx) as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rcx = (((rcx) ^ (rsp))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x88u64))) as u64; rsp = ((rsp).wrapping_add(0x60u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e18a70(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001e693a8;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = (rcx) as u64;
    rcx = (crate::simd_ops::ru64(rcx)) as u64;
    fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P0)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); }
    crate::simd_ops::lk_and_u64(rbx, (0x0u64) as u64);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = core::ptr::read_unaligned(rsp as *const u64);
    rsp = rsp.wrapping_add(8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e18a94(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001e6bd28; const P1: u64 = 0x80; const P2: u64 = 0x18; const P3: u64 = 0x70; const P4: u64 = 0x90; const P5: u64 = 0x88; const P6: u64 = 0x38; const P7: u64 = 0x50; const P8: u64 = 0x7c; const P9: u64 = 0x7001e1adc8; const P10: u64 = 0x7001e6bca8; const P11: u64 = 0x60; const P12: u64 = 0x7001e696a0; const P13: u64 = 0x7001e6649c;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&super::EH_META[36], rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = (rcx) as u64; rax = ((P0) as u64) as u64; crate::simd_ops::wu64(rcx, (rax) as u64); fl_l = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64 as u64; fl_r = (0x0u64) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 3; continue; } bb = 1; continue; }
            1 => { rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P2))) as u64; rax = ((rbx.wrapping_add(P3)) as u64) as u64; fl_l = (crate::simd_ops::ru64(rcx)) as u64 as u64; fl_r = (rax) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } bb = 2; continue; }
            2 => { r8 = (crate::simd_ops::ru64(rbx.wrapping_add(P4))) as u64; rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P5))) as u64; crate::simd_ops::wu64(rcx, (rdx) as u64); rax = (crate::simd_ops::ru64(rbx.wrapping_add(P6))) as u64; crate::simd_ops::wu64(rax, (rdx) as u64); fl_l = ((r8 as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; r8 = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; rax = (crate::simd_ops::ru64(rbx.wrapping_add(P7))) as u64; crate::simd_ops::wu32(rax, ((r8 as u32)) as u32); bb = 3; continue; }
            3 => { fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(P8))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 5; continue; } bb = 4; continue; }
            4 => { rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e18ae9); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P9) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 5; continue; }
            5 => { rax = ((P10) as u64) as u64; crate::simd_ops::wu64(rbx, (rax) as u64); rbx = (crate::simd_ops::ru64(rbx.wrapping_add(P11))) as u64; fl_l = (rbx) as u64 as u64; fl_r = (rbx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 6; continue; }
            6 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 7; continue; }
            7 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e18b11); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P12)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rcx = (rax) as u64; fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 8; continue; }
            8 => { rdx = (crate::simd_ops::ru64(rax)) as u64; rax = (crate::simd_ops::ru64(rdx)) as u64; rdx = (0x1u32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e18b2a); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P12)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 9; continue; }
            9 => { rdx = (0x10u32) as u32 as u64; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e18b38); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P13) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 10; continue; }
            10 => { rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e18b50(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001e6bc08; const P1: u64 = 0x7001e1efd0;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&super::EH_META[37], rsp); let _eh = eh.push();
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    rax = ((P0) as u64) as u64;
    crate::simd_ops::wu64(rcx, (rax) as u64);
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    eh.site(0x7001e18b5e); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e18bfc(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x18; const P1: u64 = 0xfffffffffffffff8; const P2: u64 = 0x7001e6649c; const P3: u64 = 0x10; const P4: u64 = 0x7001e69480; const P5: u64 = 0x7001e18c57;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rbx = (rcx) as u64; fl_l = (rdx) as u64 as u64; fl_r = (0xfu64) as u64 as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 5; continue; } bb = 1; continue; }
            1 => { rcx = (crate::simd_ops::ru64(rcx)) as u64; rdx = ((rdx).wrapping_add(1)) as u64; fl_l = (rdx) as u64 as u64; fl_r = (0x1000u64) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 4; continue; } bb = 2; continue; }
            2 => { r8 = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; rdx = ((rdx).wrapping_add(0x27u64)) as u64; rcx = ((rcx).wrapping_sub(r8)) as u64; rax = ((rcx.wrapping_add(P1)) as u64) as u64; fl_l = (rax) as u64 as u64; fl_r = (0x1fu64) as u64 as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 6; continue; } bb = 3; continue; }
            3 => { rcx = (r8) as u64; bb = 4; continue; }
            4 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 5; continue; }
            5 => { crate::simd_ops::lk_and_u64(rbx.wrapping_add(P3), (0x0u64) as u64); crate::simd_ops::wu64(rbx.wrapping_add(P0), (0xfu64) as u64); crate::simd_ops::wu8(rbx, (0x0u8) as u8); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            6 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P4)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); crate::lift_ops::trap(0x80000003, P5, rcx, rsp); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e18c60(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x7001e6b590;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = ((P0) as u64) as u64;
    crate::simd_ops::wu64(rcx, (rax) as u64);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e18c70(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001e6ba90; const P1: u64 = 0x20; const P2: u64 = 0x18; const P3: u64 = 0x7001e693a8; const P4: u64 = 0x7001e6649c; const P5: u64 = 0x28; const P6: u64 = 0x7001e6b590;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = ((P0) as u64) as u64; rbx = (rcx) as u64; crate::simd_ops::wu64(rcx, (rax) as u64); rax = (crate::simd_ops::ru32(rcx.wrapping_add(P1))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P2))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P3)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 4; continue; }
            2 => { if (((((fl_l as u32)) & ((fl_r as u32))) as i32) >= 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P2))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 4; continue; }
            4 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P5))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P3)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rax = ((P6) as u64) as u64; crate::simd_ops::wu64(rbx, (rax) as u64); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e18d40(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001e696a0;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    rcx = (crate::simd_ops::ru64(rcx)) as u64;
    fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = (0x1u32) as u32 as u64; rax = (crate::simd_ops::ru64(rax)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P0)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); }
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e18d70(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001e1f1c0;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&super::EH_META[38], rsp); let _eh = eh.push();
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    eh.site(0x7001e18d74); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e18d80(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001e6b8b0; const P1: u64 = 0x7001e25730; const P2: u64 = 0x28; const P3: u64 = 0x7001e693a8; const P4: u64 = 0x7001e6b590;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&super::EH_META[39], rsp); let _eh = eh.push();
    rsp = rsp.wrapping_sub(8);
    core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = (rcx) as u64;
    rax = ((P0) as u64) as u64;
    crate::simd_ops::wu64(rcx, (rax) as u64);
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    eh.site(0x7001e18d93); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P2))) as u64;
    fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e18da1); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P3)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); }
    crate::simd_ops::lk_and_u64(rbx.wrapping_add(P2), (0x0u64) as u64);
    rax = ((P4) as u64) as u64;
    crate::simd_ops::wu64(rbx, (rax) as u64);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = core::ptr::read_unaligned(rsp as *const u64);
    rsp = rsp.wrapping_add(8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e18dc0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001e25820; const P1: u64 = 0x58; const P2: u64 = 0x7001e693a8; const P3: u64 = 0x48; const P4: u64 = 0x38; const P5: u64 = 0x28; const P6: u64 = 0x18; const P7: u64 = 0x8; const P8: u64 = 0x7001e49ec0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&super::EH_META[40], rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = (rcx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e18dc9); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P1))) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e18dd7); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P2)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 2; continue; }
            2 => { crate::simd_ops::lk_and_u64(rbx.wrapping_add(P1), (0x0u64) as u64); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P3))) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e18deb); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P2)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 4; continue; }
            4 => { crate::simd_ops::lk_and_u64(rbx.wrapping_add(P3), (0x0u64) as u64); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P4))) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e18dff); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P2)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 6; continue; }
            6 => { crate::simd_ops::lk_and_u64(rbx.wrapping_add(P4), (0x0u64) as u64); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P5))) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 7; continue; }
            7 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e18e13); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P2)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 8; continue; }
            8 => { crate::simd_ops::lk_and_u64(rbx.wrapping_add(P5), (0x0u64) as u64); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P6))) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 9; continue; }
            9 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e18e27); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P2)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 10; continue; }
            10 => { crate::simd_ops::lk_and_u64(rbx.wrapping_add(P6), (0x0u64) as u64); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P7))) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 11; continue; }
            11 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e18e3b); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P2)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 12; continue; }
            12 => { crate::simd_ops::lk_and_u64(rbx.wrapping_add(P7), (0x0u64) as u64); rcx = (rbx) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); drop(_eh); let s5 = crate::simd_ops::read_s5_tail(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x40) as *const u64); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P8) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e18e60(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001e693a8; const P1: u64 = 0x7001e18e6a; const P2: u64 = 0x7001e18e6b; const P3: u64 = 0x7001e6b378; const P4: u64 = 0x7001e66432;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rcx = (crate::simd_ops::ru64(rcx)) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P0)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            1 => { crate::lift_ops::trap(0x80000003, P1, rcx, 0); return rax; }
            2 => { crate::lift_ops::trap(0x80000003, P2, rcx, 0); return rax; }
            3 => { rax = ((P3) as u64) as u64; crate::simd_ops::wu64(rcx, (rax) as u64); fl_l = ((rcx).wrapping_add(0x8u64)) as u64 as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P4) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e18e80(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001e6bc08; const P1: u64 = 0x7001e1efd0;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&super::EH_META[41], rsp); let _eh = eh.push();
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    rax = ((P0) as u64) as u64;
    crate::simd_ops::wu64(rcx, (rax) as u64);
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    eh.site(0x7001e18e8e); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e18e9c(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0x7001e696a0;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64;
    if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    rax = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P2)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    r8 = (rax) as u64;
    fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rcx = (crate::simd_ops::ru64(rax)) as u64; rdx = (0x1u32) as u32 as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rcx = (r8) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P2)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); }
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e18ee0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = (rcx) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e18ef0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001e693a8; const P1: u64 = 0x1; const P2: u64 = 0x7001e693c8; const P3: u64 = 0x7001e66450;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rsi = (rcx) as u64; rdi = (rdx) as u64; rcx = (crate::simd_ops::ru64(rcx)) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rdx) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 8; continue; } bb = 1; continue; }
            1 => { fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P0)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 3; continue; }
            3 => { crate::simd_ops::lk_and_u64(rsi, (0x0u64) as u64); fl_l = (rdi) as u64 as u64; fl_r = (rdi) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 4; continue; }
            4 => { fl_l = (crate::simd_ops::ru8(rdi)) as u8 as u64; fl_r = (0x0u8) as u8 as u64; rbx = (rdi) as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rbx = ((rbx).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru8(rbx)) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 5; continue; } bb = 6; continue; }
            6 => { rbx = ((rbx).wrapping_sub(rdi)) as u64; rcx = ((rbx.wrapping_add(P1)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P2)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); crate::simd_ops::wu64(rsi, (rax) as u64); fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 7; continue; }
            7 => { r8 = ((rbx.wrapping_add(P1)) as u64) as u64; rdx = (rdi) as u64; rcx = (rax) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 8; continue; }
            8 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = (rsi) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e18f70(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001e693a8; const P1: u64 = 0x7001e693c8; const P2: u64 = 0x7001e66450;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rbp) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rsi = (rcx) as u64; rdi = (rdx) as u64; rcx = (crate::simd_ops::ru64(rcx)) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rdx) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 8; continue; } bb = 1; continue; }
            1 => { rbp = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P0)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 3; continue; }
            3 => { crate::simd_ops::wu64(rsi, (rbp) as u64); fl_l = (rdi) as u64 as u64; fl_r = (rdi) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 4; continue; }
            4 => { rbx = (rdi) as u64; fl_l = (crate::simd_ops::ru16(rdi)) as u16 as u64; fl_r = ((rbp as u16)) as u16 as u64; if (((fl_l as u16)) == ((fl_r as u16))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rbx = ((rbx).wrapping_add(0x2u64)) as u64; fl_l = (crate::simd_ops::ru16(rbx)) as u16 as u64; fl_r = ((rbp as u16)) as u16 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { bb = 5; continue; } bb = 6; continue; }
            6 => { rbx = ((rbx).wrapping_sub(rdi)) as u64; rbx = ((rbx).wrapping_add(0x2u64)) as u64; rbx = ((((rbx) as i64).wrapping_shr(1u32) as u64)) as u64; rbx = ((rbx).wrapping_add(rbx)) as u64; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P1)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); crate::simd_ops::wu64(rsi, (rax) as u64); fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 7; continue; }
            7 => { r8 = (rbx) as u64; rdx = (rdi) as u64; rcx = (rax) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 8; continue; }
            8 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = (rsi) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e19000(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001e693a8;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rbx) as u64);
    rsp = rsp.wrapping_sub(8);
    core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rdi = (rdx) as u64;
    rbx = (rcx) as u64;
    fl_l = (rcx) as u64 as u64; fl_r = (rdx) as u64 as u64;
    if !((((fl_l as u64)) == ((fl_r as u64)))) { rcx = (crate::simd_ops::ru64(rcx)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P0)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rax = (crate::simd_ops::ru64(rdi)) as u64; crate::simd_ops::wu64(rbx, (rax) as u64); fl_l = (crate::simd_ops::lk_and_u64(rdi, (0x0u64) as u64)) as u64; fl_r = fl_l; }
    rax = (rbx) as u64;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = core::ptr::read_unaligned(rsp as *const u64);
    rsp = rsp.wrapping_add(8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e19040(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x10;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(P0))) as u8 as u64; fl_r = (0x6u8) as u8 as u64;
    rax = (rax & !0xffu64) | (((if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { 1u8 } else { 0u8 }) as u8 as u64));
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e19050(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001e49e20; const P1: u64 = 0x7001e99198; const P2: u64 = 0x7001e49ec0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    fl_l = (crate::simd_ops::ru64(rcx)) as u64 as u64; fl_r = (0x0u64) as u64 as u64;
    rbx = (rcx) as u64;
    if (((fl_l as u64)) != ((fl_r as u64))) { rax = (crate::simd_ops::ru64(rbx)) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
    rdx = (0u32) as u32 as u64;
    fl_l = 0; fl_r = 0;
    rcx = ((rsp.wrapping_add(0x30u64)) as u64) as u64;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    fl_l = (crate::simd_ops::ru64(rbx)) as u64 as u64; fl_r = (0x0u64) as u64 as u64;
    if !((((fl_l as u64)) != ((fl_r as u64)))) { rax = (crate::simd_ops::ru32(P1)) as u32 as u64; fl_l = (((rax as u32)).wrapping_add(1)) as u32 as u64; rax = ((fl_l as u32)) as u32 as u64; crate::simd_ops::wu32(P1, ((rax as u32)) as u32); rax = (rax as u32 as i32 as i64) as u64; crate::simd_ops::wu64(rbx, (rax) as u64); }
    rcx = ((rsp.wrapping_add(0x30u64)) as u64) as u64;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    rax = (crate::simd_ops::ru64(rbx)) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = core::ptr::read_unaligned(rsp as *const u64);
    rsp = rsp.wrapping_add(8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e190a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x10;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(P0))) as u8 as u64; fl_r = (0x6u8) as u8 as u64;
    rax = (rax & !0xffu64) | (((if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { 1u8 } else { 0u8 }) as u8 as u64));
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e190b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x4; const P1: u64 = 0x7001e6bc90; const P2: u64 = 0xfffffffffffffff0; const P3: u64 = 0xfffffffffffffffc; const P4: u64 = 0x7001e6bc08; const P5: u64 = 0x10; const P6: u64 = 0x7001e1efd0;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&super::EH_META[42], rsp); let _eh = eh.push();
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    rdx = (((crate::simd_ops::ru32(rax.wrapping_add(P0))) as i32 as i64) as u64) as u64;
    rax = ((P1) as u64) as u64;
    crate::simd_ops::wu64(rdx.wrapping_add(rcx), (rax) as u64);
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    rdx = (((crate::simd_ops::ru32(rax.wrapping_add(P0))) as i32 as i64) as u64) as u64;
    r8 = ((rdx.wrapping_add(P2)) as u32) as u32 as u64;
    crate::simd_ops::wu32(rdx.wrapping_add(rcx).wrapping_add(P3), ((r8 as u32)) as u32);
    rax = ((P4) as u64) as u64;
    crate::simd_ops::wu64(rcx.wrapping_add(P5), (rax) as u64);
    rcx = ((rcx).wrapping_add(0x10u64)) as u64;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    eh.site(0x7001e190e5); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e19100(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xfffffffffffffff8; const P1: u64 = 0x7001e18c60; const P2: u64 = 0x7001e6592c; const P3: u64 = 0x7001e6649c; const P4: u64 = 0x7001e6b590;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&super::EH_META[43], rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = (rcx) as u64; rdi = ((rdx as u32)) as u32 as u64; rdi = ((((rdi as u32)) & (0x1u32))) as u32 as u64; fl_l = ((rdx as u8)) as u8 as u64; fl_r = (0x2u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 4; continue; } bb = 1; continue; }
            1 => { rsi = ((rcx.wrapping_add(P0)) as u64) as u64; r9 = ((P1) as u64) as u64; r8 = (crate::simd_ops::ru64(rsi)) as u64; rdx = (0x40u32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e1912f); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); fl_l = ((rdi as u32)) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { rdx = (crate::simd_ops::ru64(rsi)) as u64; rdx = ((rdx).wrapping_shl(6u32)) as u64; rdx = ((rdx).wrapping_add(0x8u64)) as u64; rcx = (rsi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e19146); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 3; continue; }
            3 => { rax = (rsi) as u64; bb = 7; continue; }
            4 => { rax = ((P4) as u64) as u64; crate::simd_ops::wu64(rcx, (rax) as u64); fl_l = ((rdi as u32)) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { rdx = (0x40u32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e19163); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 6; continue; }
            6 => { rax = (rbx) as u64; bb = 7; continue; }
            7 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e19180(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xfffffffffffffff8; const P1: u64 = 0x7001e18c60; const P2: u64 = 0x7001e6592c; const P3: u64 = 0x8; const P4: u64 = 0x7001e6649c; const P5: u64 = 0x7001e6b590;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&super::EH_META[44], rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = (rcx) as u64; rdi = ((rdx as u32)) as u32 as u64; rdi = ((((rdi as u32)) & (0x1u32))) as u32 as u64; fl_l = ((rdx as u8)) as u8 as u64; fl_r = (0x2u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 4; continue; } bb = 1; continue; }
            1 => { rsi = ((rcx.wrapping_add(P0)) as u64) as u64; r9 = ((P1) as u64) as u64; r8 = (crate::simd_ops::ru64(rsi)) as u64; rdx = (0x18u32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e191af); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); fl_l = ((rdi as u32)) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { rcx = (crate::simd_ops::ru64(rsi)) as u64; rdx = ((rcx.wrapping_add(rcx.wrapping_mul(2))) as u64) as u64; rdx = ((rdx.wrapping_mul(8).wrapping_add(P3)) as u64) as u64; rcx = (rsi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e191ca); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 3; continue; }
            3 => { rax = (rsi) as u64; bb = 7; continue; }
            4 => { rax = ((P5) as u64) as u64; crate::simd_ops::wu64(rcx, (rax) as u64); fl_l = ((rdi as u32)) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { rdx = (0x18u32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e191e7); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 6; continue; }
            6 => { rax = (rbx) as u64; bb = 7; continue; }
            7 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e19200(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xfffffffffffffff8; const P1: u64 = 0x7001e18c70; const P2: u64 = 0x7001e6592c; const P3: u64 = 0x7001e6649c; const P4: u64 = 0x7001e6ba90; const P5: u64 = 0x20; const P6: u64 = 0x18; const P7: u64 = 0x7001e693a8; const P8: u64 = 0x28; const P9: u64 = 0x7001e6b590;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&super::EH_META[45], rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rsi = ((rdx as u32)) as u32 as u64; rbx = (rcx) as u64; fl_l = ((rdx as u8)) as u8 as u64; fl_r = (0x2u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 4; continue; } bb = 1; continue; }
            1 => { rdi = ((rcx.wrapping_add(P0)) as u64) as u64; r9 = ((P1) as u64) as u64; r8 = (crate::simd_ops::ru64(rdi)) as u64; rdx = (0x30u32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e1922c); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); fl_l = ((rsi as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { rcx = (crate::simd_ops::ru64(rdi)) as u64; rdx = ((rcx.wrapping_add(rcx.wrapping_mul(2))) as u64) as u64; rdx = ((rdx).wrapping_shl(4u32)) as u64; rdx = ((rdx).wrapping_add(0x8u64)) as u64; rcx = (rdi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e19249); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 3; continue; }
            3 => { rax = (rdi) as u64; bb = 11; continue; }
            4 => { rax = ((P4) as u64) as u64; crate::simd_ops::wu64(rcx, (rax) as u64); rax = (crate::simd_ops::ru32(rcx.wrapping_add(P5))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P6))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e19268); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P7)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 8; continue; }
            6 => { if (((((fl_l as u32)) & ((fl_r as u32))) as i32) >= 0) { bb = 8; continue; } bb = 7; continue; }
            7 => { rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P6))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e19276); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 8; continue; }
            8 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P8))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e1927f); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P7)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rax = ((P9) as u64) as u64; crate::simd_ops::wu64(rbx, (rax) as u64); fl_l = ((rsi as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 10; continue; } bb = 9; continue; }
            9 => { rdx = (0x30u32) as u32 as u64; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e1929d); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 10; continue; }
            10 => { rax = (rbx) as u64; bb = 11; continue; }
            11 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e192c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xfffffffffffffff8; const P1: u64 = 0x7001e18cc0; const P2: u64 = 0x7001e6592c; const P3: u64 = 0x7001e6649c; const P4: u64 = 0x7001e6bb70; const P5: u64 = 0x20; const P6: u64 = 0x18; const P7: u64 = 0x7001e693a8; const P8: u64 = 0x28; const P9: u64 = 0x7001e6b590;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&super::EH_META[46], rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rsi = ((rdx as u32)) as u32 as u64; rbx = (rcx) as u64; fl_l = ((rdx as u8)) as u8 as u64; fl_r = (0x2u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 4; continue; } bb = 1; continue; }
            1 => { rdi = ((rcx.wrapping_add(P0)) as u64) as u64; r9 = ((P1) as u64) as u64; r8 = (crate::simd_ops::ru64(rdi)) as u64; rdx = (0x60u32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e192ec); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); fl_l = ((rsi as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { rcx = (crate::simd_ops::ru64(rdi)) as u64; rdx = ((rcx.wrapping_add(rcx.wrapping_mul(2))) as u64) as u64; rdx = ((rdx).wrapping_shl(5u32)) as u64; rdx = ((rdx).wrapping_add(0x8u64)) as u64; rcx = (rdi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e19309); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 3; continue; }
            3 => { rax = (rdi) as u64; bb = 9; continue; }
            4 => { rax = ((P4) as u64) as u64; crate::simd_ops::wu64(rcx, (rax) as u64); fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(P5))) as u32 as u64; fl_r = (0x0u32) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P6))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e19327); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P7)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 6; continue; }
            6 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P8))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e19331); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P7)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rax = ((P9) as u64) as u64; crate::simd_ops::wu64(rbx, (rax) as u64); fl_l = ((rsi as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 8; continue; } bb = 7; continue; }
            7 => { rdx = (0x60u32) as u32 as u64; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e1934f); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 8; continue; }
            8 => { rax = (rbx) as u64; bb = 9; continue; }
            9 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e19370(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xfffffffffffffff8; const P1: u64 = 0x7001e18d00; const P2: u64 = 0x7001e6592c; const P3: u64 = 0x7001e6649c; const P4: u64 = 0x7001e6baf0; const P5: u64 = 0x20; const P6: u64 = 0x18; const P7: u64 = 0x7001e693a8; const P8: u64 = 0x28; const P9: u64 = 0x7001e6b590;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&super::EH_META[47], rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rsi = ((rdx as u32)) as u32 as u64; rbx = (rcx) as u64; fl_l = ((rdx as u8)) as u8 as u64; fl_r = (0x2u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 4; continue; } bb = 1; continue; }
            1 => { rdi = ((rcx.wrapping_add(P0)) as u64) as u64; r9 = ((P1) as u64) as u64; r8 = (crate::simd_ops::ru64(rdi)) as u64; rdx = (0x60u32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e1939c); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); fl_l = ((rsi as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { rcx = (crate::simd_ops::ru64(rdi)) as u64; rdx = ((rcx.wrapping_add(rcx.wrapping_mul(2))) as u64) as u64; rdx = ((rdx).wrapping_shl(5u32)) as u64; rdx = ((rdx).wrapping_add(0x8u64)) as u64; rcx = (rdi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e193b9); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 3; continue; }
            3 => { rax = (rdi) as u64; bb = 9; continue; }
            4 => { rax = ((P4) as u64) as u64; crate::simd_ops::wu64(rcx, (rax) as u64); fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(P5))) as u32 as u64; fl_r = (0x0u32) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P6))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e193d7); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P7)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 6; continue; }
            6 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P8))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e193e1); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P7)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rax = ((P9) as u64) as u64; crate::simd_ops::wu64(rbx, (rax) as u64); fl_l = ((rsi as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 8; continue; } bb = 7; continue; }
            7 => { rdx = (0x60u32) as u32 as u64; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e193ff); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 8; continue; }
            8 => { rax = (rbx) as u64; bb = 9; continue; }
            9 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e19420(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xfffffffffffffff8; const P1: u64 = 0x7001e18c60; const P2: u64 = 0x7001e6592c; const P3: u64 = 0x8; const P4: u64 = 0x7001e6649c; const P5: u64 = 0x7001e6b590;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&super::EH_META[48], rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = (rcx) as u64; rdi = ((rdx as u32)) as u32 as u64; rdi = ((((rdi as u32)) & (0x1u32))) as u32 as u64; fl_l = ((rdx as u8)) as u8 as u64; fl_r = (0x2u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 4; continue; } bb = 1; continue; }
            1 => { rsi = ((rcx.wrapping_add(P0)) as u64) as u64; r9 = ((P1) as u64) as u64; r8 = (crate::simd_ops::ru64(rsi)) as u64; rdx = (0x8u32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e1944f); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); fl_l = ((rdi as u32)) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { rdx = (crate::simd_ops::ru64(rsi)) as u64; rdx = ((rdx.wrapping_mul(8).wrapping_add(P3)) as u64) as u64; rcx = (rsi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e19466); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 3; continue; }
            3 => { rax = (rsi) as u64; bb = 7; continue; }
            4 => { rax = ((P5) as u64) as u64; crate::simd_ops::wu64(rcx, (rax) as u64); fl_l = ((rdi as u32)) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { rdx = (0x8u32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e19483); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 6; continue; }
            6 => { rax = (rbx) as u64; bb = 7; continue; }
            7 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e194a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xfffffffffffffff8; const P1: u64 = 0x7001e18d80; const P2: u64 = 0x7001e6592c; const P3: u64 = 0x7001e6649c; const P4: u64 = 0x7001e6b8b0; const P5: u64 = 0x7001e25730; const P6: u64 = 0x28; const P7: u64 = 0x7001e693a8; const P8: u64 = 0x7001e6b590;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&super::EH_META[49], rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rsi = ((rdx as u32)) as u32 as u64; rbx = (rcx) as u64; fl_l = ((rdx as u8)) as u8 as u64; fl_r = (0x2u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 4; continue; } bb = 1; continue; }
            1 => { rdi = ((rcx.wrapping_add(P0)) as u64) as u64; r9 = ((P1) as u64) as u64; r8 = (crate::simd_ops::ru64(rdi)) as u64; rdx = (0x38u32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e194cc); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); fl_l = ((rsi as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { rdx = (((crate::simd_ops::ru64(rdi)) as i64).wrapping_mul((0x38u64) as i64) as u64) as u64; rdx = ((rdx).wrapping_add(0x8u64)) as u64; rcx = (rdi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e194e2); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 3; continue; }
            3 => { rax = (rdi) as u64; bb = 9; continue; }
            4 => { rax = ((P4) as u64) as u64; crate::simd_ops::wu64(rcx, (rax) as u64); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e194f6); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P6))) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e19504); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P7)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 6; continue; }
            6 => { crate::simd_ops::lk_and_u64(rbx.wrapping_add(P6), (0x0u64) as u64); rax = ((P8) as u64) as u64; crate::simd_ops::wu64(rbx, (rax) as u64); fl_l = ((rsi as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 8; continue; } bb = 7; continue; }
            7 => { rdx = (0x38u32) as u32 as u64; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e19527); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 8; continue; }
            8 => { rax = (rbx) as u64; bb = 9; continue; }
            9 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e19540(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xfffffffffffffff8; const P1: u64 = 0x7001e18c60; const P2: u64 = 0x7001e6592c; const P3: u64 = 0x7001e6649c; const P4: u64 = 0x7001e6b590;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&super::EH_META[50], rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = (rcx) as u64; rdi = ((rdx as u32)) as u32 as u64; rdi = ((((rdi as u32)) & (0x1u32))) as u32 as u64; fl_l = ((rdx as u8)) as u8 as u64; fl_r = (0x2u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 4; continue; } bb = 1; continue; }
            1 => { rsi = ((rcx.wrapping_add(P0)) as u64) as u64; r9 = ((P1) as u64) as u64; r8 = (crate::simd_ops::ru64(rsi)) as u64; rdx = (0x10u32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e1956f); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); fl_l = ((rdi as u32)) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { rdx = (crate::simd_ops::ru64(rsi)) as u64; rdx = ((rdx).wrapping_shl(4u32)) as u64; rdx = ((rdx).wrapping_add(0x8u64)) as u64; rcx = (rsi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e19586); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 3; continue; }
            3 => { rax = (rsi) as u64; bb = 7; continue; }
            4 => { rax = ((P4) as u64) as u64; crate::simd_ops::wu64(rcx, (rax) as u64); fl_l = ((rdi as u32)) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { rdx = (0x10u32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e195a3); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 6; continue; }
            6 => { rax = (rbx) as u64; bb = 7; continue; }
            7 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e195c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xfffffffffffffff8; const P1: u64 = 0x7001e18e80; const P2: u64 = 0x7001e6592c; const P3: u64 = 0x8; const P4: u64 = 0x7001e6649c; const P5: u64 = 0x7001e6bc08; const P6: u64 = 0x7001e1efd0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&super::EH_META[51], rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = (rcx) as u64; rdi = ((rdx as u32)) as u32 as u64; rdi = ((((rdi as u32)) & (0x1u32))) as u32 as u64; fl_l = ((rdx as u8)) as u8 as u64; fl_r = (0x2u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 4; continue; } bb = 1; continue; }
            1 => { rsi = ((rcx.wrapping_add(P0)) as u64) as u64; r9 = ((P1) as u64) as u64; r8 = (crate::simd_ops::ru64(rsi)) as u64; rdx = (0x48u32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e195ef); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); fl_l = ((rdi as u32)) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { rcx = (crate::simd_ops::ru64(rsi)) as u64; rdx = ((rcx.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; rdx = ((rdx.wrapping_mul(8).wrapping_add(P3)) as u64) as u64; rcx = (rsi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e1960a); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 3; continue; }
            3 => { rax = (rsi) as u64; bb = 7; continue; }
            4 => { rax = ((P5) as u64) as u64; crate::simd_ops::wu64(rcx, (rax) as u64); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e1961e); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); fl_l = ((rdi as u32)) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { rdx = (0x48u32) as u32 as u64; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e19630); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 6; continue; }
            6 => { rax = (rbx) as u64; bb = 7; continue; }
            7 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e19660(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x7001e6b910; const P2: u64 = 0x10; const P3: u64 = 0x14; const P4: u64 = 0x7001e6b880; const P5: u64 = 0x7001e184f0; const P6: u64 = 0x7001e18dc0;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 19>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&super::EH_META[52], rsp); let _eh = eh.push();
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rcx) as u64);
    rsp = ((rsp).wrapping_sub(0x98u64)) as u64;
    crate::simd_ops::lk_and_u32(rcx.wrapping_add(P0), (0x0u32) as u32);
    rax = ((P1) as u64) as u64;
    crate::simd_ops::wu64(rcx, (rax) as u64);
    crate::simd_ops::wu32(rcx.wrapping_add(P2), (0x10ffffu32) as u32);
    crate::simd_ops::wu32(rcx.wrapping_add(P3), (0x4u32) as u32);
    rdx = ((P4) as u64) as u64;
    rcx = ((rsp.wrapping_add(0x20u64)) as u64) as u64;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    eh.site(0x7001e19694); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    rcx = ((rsp.wrapping_add(0x20u64)) as u64) as u64;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    eh.site(0x7001e1969e); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    rsp = ((rsp).wrapping_add(0x98u64)) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e196b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x7001e6b968; const P2: u64 = 0x10; const P3: u64 = 0x14; const P4: u64 = 0x7001e6b880; const P5: u64 = 0x7001e184f0; const P6: u64 = 0x7001e18dc0;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 19>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&super::EH_META[53], rsp); let _eh = eh.push();
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rcx) as u64);
    rsp = ((rsp).wrapping_sub(0x98u64)) as u64;
    crate::simd_ops::lk_and_u32(rcx.wrapping_add(P0), (0x0u32) as u32);
    rax = ((P1) as u64) as u64;
    crate::simd_ops::wu64(rcx, (rax) as u64);
    crate::simd_ops::lk_or_u32(rcx.wrapping_add(P2), (0xffffffffu32) as u32);
    crate::simd_ops::wu32(rcx.wrapping_add(P3), (0x4u32) as u32);
    rdx = ((P4) as u64) as u64;
    rcx = ((rsp.wrapping_add(0x20u64)) as u64) as u64;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    eh.site(0x7001e196e1); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    rcx = ((rsp.wrapping_add(0x20u64)) as u64) as u64;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    eh.site(0x7001e196eb); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    rsp = ((rsp).wrapping_add(0x98u64)) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e19710(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001e18090;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    r9 = (0u32) as u32 as u64;
    fl_l = 0; fl_r = 0;
    r8 = (0u32) as u32 as u64;
    fl_l = 0; fl_r = 0;
    rdx = (0u32) as u32 as u64;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e19740(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001e6b880; const P1: u64 = 0x7001e184f0;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rdx = ((P0) as u64) as u64;
    crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e19750(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    fl_l = (crate::simd_ops::lk_and_u64(rcx, (0x0u64) as u64)) as u64; fl_r = fl_l;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e197e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001e6649c;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = (rcx) as u64;
    fl_l = ((rdx as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64;
    if !(((((fl_l as u8)) & ((fl_r as u8))) == 0)) { rdx = (0x10u32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); }
    rax = (rbx) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = core::ptr::read_unaligned(rsp as *const u64);
    rsp = rsp.wrapping_add(8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e19810(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001e6b378; const P1: u64 = 0x7001e66432; const P2: u64 = 0x7001e6649c;
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
    if !(((((fl_l as u8)) & ((fl_r as u8))) == 0)) { rdx = (0x28u32) as u32 as u64; rcx = (rdi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); }
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rax = (rdi) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = core::ptr::read_unaligned(rsp as *const u64);
    rsp = rsp.wrapping_add(8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e19860(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001e6b378; const P1: u64 = 0x7001e66432; const P2: u64 = 0x7001e6649c;
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
pub unsafe extern "win64" fn fn_7001e198b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001e23070; const P1: u64 = 0x7001e6b5a8; const P2: u64 = 0x7001e4d5b0; const P3: u64 = 0x7001e198ce;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x28u64)) as u64; fl_l = (r8) as u64 as u64; fl_r = (r8) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rsp = ((rsp).wrapping_add(0x28u64)) as u64; let s5 = crate::simd_ops::read_s5_tail(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x40) as *const u64); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); }
            2 => { rcx = ((P1) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); crate::lift_ops::trap(0x80000003, P3, rcx, rsp); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e198e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64;
    if !(((((fl_l as u64)) & ((fl_r as u64))) != 0)) { rax = ((rcx.wrapping_add(P0)) as u64) as u64; }
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e198f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x38; const P1: u64 = 0x8; const P2: u64 = 0x10; const P3: u64 = 0x7001e696a0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rsi = ((rdx as u32)) as u32 as u64; rdi = (rcx) as u64; bb = 2; continue; }
            1 => { r8 = (crate::simd_ops::ru32(rbx.wrapping_add(P1))) as u32 as u64; rdx = (rdi) as u64; rax = (crate::simd_ops::ru64(rbx.wrapping_add(P2))) as u64; rcx = ((rsi as u32)) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P3)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rbx = (crate::simd_ops::ru64(rbx)) as u64; bb = 2; continue; }
            2 => { fl_l = (rbx) as u64 as u64; fl_r = (rbx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 1; continue; } bb = 3; continue; }
            3 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e19940(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    fl_l = (crate::simd_ops::lk_add_u32(rcx.wrapping_add(P0), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l;
    rax = (0x0u32) as u32 as u64;
    rax = (if (((fl_l as u32)) == 0) { rcx } else { rax }) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e19950(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001e97380; const P1: u64 = 0x30; const P2: u64 = 0x7001e4d9a0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64);
    rsp = ((rsp).wrapping_sub(0x40u64)) as u64;
    rax = (crate::simd_ops::ru64(P0)) as u64;
    rax = (((rax) ^ (rsp))) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (rax) as u64);
    crate::simd_ops::lk_and_u64(rsp.wrapping_add(0x20u64), (0x0u64) as u64);
    r9 = ((rcx.wrapping_add(P1)) as u64) as u64;
    rbx = (((r8 as u8)) as u32) as u32 as u64;
    rcx = ((rsp.wrapping_add(0x28u64)) as u64) as u64;
    r8 = ((rsp.wrapping_add(0x20u64)) as u64) as u64;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    rdx = ((crate::simd_ops::ru8(rsp.wrapping_add(0x28u64))) as u32) as u32 as u64;
    fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x1u32) as u32 as u64;
    rdx = (if (((fl_l as u32)) != ((fl_r as u32))) { (rbx as u32) } else { (rdx as u32) }) as u32 as u64;
    rax = (rax & !0xffu64) | ((((rdx as u8)) as u8 as u64));
    rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rcx = (((rcx) ^ (rsp))) as u64;
    rsp = ((rsp).wrapping_add(0x40u64)) as u64;
    rbx = core::ptr::read_unaligned(rsp as *const u64);
    rsp = rsp.wrapping_add(8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e199b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x30; const P1: u64 = 0x7001e4aae0;
    let (mut rax, mut r11, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu8(rsp.wrapping_add(0x10u64), ((rdx as u8)) as u8);
    r11 = (rsp) as u64;
    rsp = ((rsp).wrapping_sub(0x38u64)) as u64;
    crate::simd_ops::lk_and_u64(r11.wrapping_add(0x18u64), (0x0u64) as u64);
    rax = ((rcx.wrapping_add(P0)) as u64) as u64;
    rcx = ((r11.wrapping_add(0x8u64)) as u64) as u64;
    crate::simd_ops::wu64(r11.wrapping_sub(0x18u64), (rax) as u64);
    r9 = ((r11.wrapping_add(0x18u64)) as u64) as u64;
    r8 = (0x1u32) as u32 as u64;
    rdx = ((r11.wrapping_add(0x10u64)) as u64) as u64;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64;
    if !((((((fl_l as u32)) & ((fl_r as u32))) as i32) >= 0)) { rax = (0xffffu32) as u32 as u64; } else { rax = ((crate::simd_ops::ru16(rsp.wrapping_add(0x40u64))) as u32) as u32 as u64; }
    rsp = ((rsp).wrapping_add(0x38u64)) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e19a00(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x30; const P1: u64 = 0x7001e4aae0;
    let (mut rax, mut r11, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu8(rsp.wrapping_add(0x10u64), ((rdx as u8)) as u8);
    r11 = (rsp) as u64;
    rsp = ((rsp).wrapping_sub(0x38u64)) as u64;
    crate::simd_ops::lk_and_u64(r11.wrapping_add(0x18u64), (0x0u64) as u64);
    rax = ((rcx.wrapping_add(P0)) as u64) as u64;
    rcx = ((r11.wrapping_add(0x8u64)) as u64) as u64;
    crate::simd_ops::wu64(r11.wrapping_sub(0x18u64), (rax) as u64);
    r9 = ((r11.wrapping_add(0x18u64)) as u64) as u64;
    r8 = (0x1u32) as u32 as u64;
    rdx = ((r11.wrapping_add(0x10u64)) as u64) as u64;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    rcx = ((crate::simd_ops::ru16(rsp.wrapping_add(0x40u64))) as u32) as u32 as u64;
    fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64;
    rdx = (0xffffu32) as u32 as u64;
    rcx = (rcx & !0xffffu64) | (((if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { (rdx as u16) } else { (rcx as u16) }) as u16 as u64));
    rax = (((rcx as u16)) as u32) as u32 as u64;
    rsp = ((rsp).wrapping_add(0x38u64)) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e19a50(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    fl_l = (crate::simd_ops::ru64(rcx)) as u64 as u64; fl_r = (0x0u64) as u64 as u64;
    rax = (rax & !0xffu64) | (((if (((fl_l as u64)) == ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64));
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e19a58(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001e97380; const P1: u64 = 0x68; const P2: u64 = 0x71; const P3: u64 = 0x18; const P4: u64 = 0x7001e696a0; const P5: u64 = 0x74; const P6: u64 = 0x40; const P7: u64 = 0x80; const P8: u64 = 0x7001e69578;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rbx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = ((rsp).wrapping_sub(0x60u64)) as u64; rax = (crate::simd_ops::ru64(P0)) as u64; rax = (((rax) ^ (rsp))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), (rax) as u64); fl_l = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64 as u64; fl_r = (0x0u64) as u64 as u64; rbx = (rcx) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 7; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(P2))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 7; continue; } bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = ((((rdx as u32)) | (0xffffffffu32))) as u32 as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(P3))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P4)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); fl_l = ((rax as u32)) as u32 as u64; fl_r = (0xffffffffu32) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 13; continue; } bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P1))) as u64; r8 = ((rsp.wrapping_add(0x30u64)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (r8) as u64); rdx = ((rbx.wrapping_add(P5)) as u64) as u64; r9 = ((rsp.wrapping_add(0x58u64)) as u64) as u64; r8 = ((rsp.wrapping_add(0x38u64)) as u64) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(P6))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P4)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 9; continue; } bb = 4; continue; }
            4 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x1u32) as u32 as u64; rax = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 10; continue; } bb = 5; continue; }
            5 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x2u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 13; continue; } bb = 6; continue; }
            6 => { crate::simd_ops::wu8(rbx.wrapping_add(P2), (0x0u8) as u8); bb = 7; continue; }
            7 => { rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 8; continue; }
            8 => { rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rcx = (((rcx) ^ (rsp))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; rsp = ((rsp).wrapping_add(0x60u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            9 => { crate::simd_ops::wu8(rbx.wrapping_add(P2), (0x0u8) as u8); bb = 10; continue; }
            10 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = ((rsp.wrapping_add(0x38u64)) as u64) as u64; fl_l = (rdi) as u64 as u64; fl_r = (rax) as u64 as u64; rdi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 12; continue; } bb = 11; continue; }
            11 => { r9 = (crate::simd_ops::ru64(rbx.wrapping_add(P7))) as u64; rcx = ((rsp.wrapping_add(0x38u64)) as u64) as u64; r8 = (rdi) as u64; rdx = (0x1u32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P8)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); fl_l = (rdi) as u64 as u64; fl_r = (rax) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 13; continue; } bb = 12; continue; }
            12 => { fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(P2))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; rax = (rax & !0xffu64) | (((if (((fl_l as u8)) == ((fl_r as u8))) { 1u8 } else { 0u8 }) as u8 as u64)); bb = 8; continue; }
            13 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 8; continue; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e19b30(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x30; const P1: u64 = 0x8; const P2: u64 = 0xc; const P3: u64 = 0x10; const P4: u64 = 0x7001e659fc;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rsi = ((rcx.wrapping_add(P0)) as u64) as u64; r8 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rax = (crate::simd_ops::ru64(rsi)) as u64; rbx = ((rdx as u32)) as u32 as u64; rdi = (rsi) as u64; fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru32(rax.wrapping_add(P1))) as u32 as u64; fl_r = ((rbx as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 13; continue; } bb = 2; continue; }
            2 => { fl_l = (r8) as u64 as u64; fl_r = (r8) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 5; continue; } bb = 3; continue; }
            3 => { fl_l = (crate::simd_ops::ru32(rax.wrapping_add(P2))) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 5; continue; } bb = 4; continue; }
            4 => { fl_l = (crate::simd_ops::ru64(rax.wrapping_add(P3))) as u64 as u64; fl_r = (r8) as u64 as u64; r8 = (if (((fl_l as u64)) == ((fl_r as u64))) { rax } else { r8 }) as u64; bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru64(rax)) as u64; fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 1; continue; } bb = 6; continue; }
            6 => { fl_l = (r8) as u64 as u64; fl_r = (r8) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 7; continue; }
            7 => { crate::simd_ops::wu32(r8.wrapping_add(P1), ((rbx as u32)) as u32); rax = (r8) as u64; bb = 13; continue; }
            8 => { rdi = ((rcx.wrapping_add(P0)) as u64) as u64; bb = 9; continue; }
            9 => { rcx = (0x18u32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (rax) as u64); rcx = (rax) as u64; fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 10; continue; }
            10 => { rax = (crate::simd_ops::ru64(rsi)) as u64; crate::simd_ops::lk_and_u32(rcx.wrapping_add(P2), (0x0u32) as u32); fl_l = (crate::simd_ops::lk_and_u64(rcx.wrapping_add(P3), (0x0u64) as u64)) as u64; fl_r = fl_l; crate::simd_ops::wu64(rcx, (rax) as u64); crate::simd_ops::wu32(rcx.wrapping_add(P1), ((rbx as u32)) as u32); bb = 12; continue; }
            11 => { rcx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; bb = 12; continue; }
            12 => { crate::simd_ops::wu64(rdi, (rcx) as u64); rax = (rcx) as u64; bb = 13; continue; }
            13 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e19bd0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x7001e659fc; const P2: u64 = 0x28; const P3: u64 = 0x30; const P4: u64 = 0x7001e6b8ca; const P5: u64 = 0x7001e184f0; const P6: u64 = 0x8; const P7: u64 = 0x7001e6bda8; const P8: u64 = 0x7001e18dc0;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 19>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&super::EH_META[54], rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (rbp) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = ((rsp).wrapping_sub(0x90u64)) as u64; rbp = (rdx) as u64; rdi = (rcx) as u64; rsi = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rsp.wrapping_add(0xa0u64), ((rsi as u32)) as u32); fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru64(rcx)) as u64 as u64; fl_r = (rsi) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 11; continue; } bb = 2; continue; }
            2 => { rcx = ((rsi.wrapping_add(P0)) as u32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e19c03); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rbx = (rax) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xa0u64), (rax) as u64); fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x8u64))) as u64; fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 4; continue; }
            4 => { rdx = (crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64; fl_l = (rdx) as u64 as u64; fl_r = (rdx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 7; continue; } bb = 5; continue; }
            5 => { rdx = ((rax.wrapping_add(P3)) as u64) as u64; bb = 7; continue; }
            6 => { rdx = ((P4) as u64) as u64; bb = 7; continue; }
            7 => { rcx = ((rsp.wrapping_add(0x20u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e19c3c); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rsi = (0x1u32) as u32 as u64; fl_l = (crate::simd_ops::lk_and_u32(rbx.wrapping_add(P6), (0x0u32) as u32)) as u32 as u64; fl_r = fl_l; rax = ((P7) as u64) as u64; crate::simd_ops::wu64(rbx, (rax) as u64); bb = 9; continue; }
            8 => { rbx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; bb = 9; continue; }
            9 => { crate::simd_ops::wu64(rdi, (rbx) as u64); fl_l = ((rsi as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 11; continue; } bb = 10; continue; }
            10 => { rcx = ((rsp.wrapping_add(0x20u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e19c66); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P8) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 11; continue; }
            11 => { rax = (0x2u32) as u32 as u64; r11 = ((rsp.wrapping_add(0x90u64)) as u64) as u64; rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x18u64))) as u64; rbp = (crate::simd_ops::ru64(r11.wrapping_add(0x20u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x28u64))) as u64; rsp = (r11) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e19c90(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x40; const P1: u64 = 0x7001e659fc; const P2: u64 = 0x28; const P3: u64 = 0x30; const P4: u64 = 0x7001e6b8ca; const P5: u64 = 0x7001e184f0; const P6: u64 = 0x8; const P7: u64 = 0x7001e6ba18; const P8: u64 = 0x7001e4d900; const P9: u64 = 0x10; const P10: u64 = 0x20; const P11: u64 = 0x38; const P12: u64 = 0x7001e18dc0;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 25>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&super::EH_META[55], rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (rbp) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = ((rsp).wrapping_sub(0xc0u64)) as u64; rbp = (rdx) as u64; rdi = (rcx) as u64; rsi = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rsp.wrapping_add(0xd0u64), ((rsi as u32)) as u32); fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru64(rcx)) as u64 as u64; fl_r = (rsi) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 11; continue; } bb = 2; continue; }
            2 => { rcx = ((rsi.wrapping_add(P0)) as u32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e19ccb); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rbx = (rax) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xd0u64), (rax) as u64); fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x8u64))) as u64; fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 4; continue; }
            4 => { rdx = (crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64; fl_l = (rdx) as u64 as u64; fl_r = (rdx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 7; continue; } bb = 5; continue; }
            5 => { rdx = ((rax.wrapping_add(P3)) as u64) as u64; bb = 7; continue; }
            6 => { rdx = ((P4) as u64) as u64; bb = 7; continue; }
            7 => { rcx = ((rsp.wrapping_add(0x50u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e19d04); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rsi = (0x1u32) as u32 as u64; crate::simd_ops::lk_and_u32(rbx.wrapping_add(P6), (0x0u32) as u32); rax = ((P7) as u64) as u64; crate::simd_ops::wu64(rbx, (rax) as u64); rcx = ((rsp.wrapping_add(0x20u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e19d21); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P8) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; xmm0 = (crate::simd_ops::ru128(rax)) as u128; crate::simd_ops::wu128(rbx.wrapping_add(P9), (xmm0) as u128); xmm1 = (crate::simd_ops::ru128(rax.wrapping_add(P9))) as u128; crate::simd_ops::wu128(rbx.wrapping_add(P10), (xmm1) as u128); xmm0 = ((crate::simd_ops::ru64(rax.wrapping_add(P10))) as u64) as u128; crate::simd_ops::wu64(rbx.wrapping_add(P3), ((xmm0 as u64)) as u64); rax = (crate::simd_ops::ru32(rax.wrapping_add(P2))) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(P11), ((rax as u32)) as u32); bb = 9; continue; }
            8 => { rbx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; bb = 9; continue; }
            9 => { crate::simd_ops::wu64(rdi, (rbx) as u64); fl_l = ((rsi as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 11; continue; } bb = 10; continue; }
            10 => { rcx = ((rsp.wrapping_add(0x50u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e19d57); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P12) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 11; continue; }
            11 => { rax = (0x2u32) as u32 as u64; r11 = ((rsp.wrapping_add(0xc0u64)) as u64) as u64; rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x18u64))) as u64; rbp = (crate::simd_ops::ru64(r11.wrapping_add(0x20u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x28u64))) as u64; rsp = (r11) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e19d80(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x18; const P1: u64 = 0x7001e659fc; const P2: u64 = 0x28; const P3: u64 = 0x30; const P4: u64 = 0x7001e6b8ca; const P5: u64 = 0x7001e184f0; const P6: u64 = 0x8; const P7: u64 = 0x7001e6b910; const P8: u64 = 0x10; const P9: u64 = 0x14; const P10: u64 = 0x7001e18dc0;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 19>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&super::EH_META[56], rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (rbp) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = ((rsp).wrapping_sub(0x90u64)) as u64; rbp = (rdx) as u64; rdi = (rcx) as u64; rsi = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rsp.wrapping_add(0xa0u64), ((rsi as u32)) as u32); fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru64(rcx)) as u64 as u64; fl_r = (rsi) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 11; continue; } bb = 2; continue; }
            2 => { rcx = ((rsi.wrapping_add(P0)) as u32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e19db3); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rbx = (rax) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xa0u64), (rax) as u64); fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x8u64))) as u64; fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 4; continue; }
            4 => { rdx = (crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64; fl_l = (rdx) as u64 as u64; fl_r = (rdx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 7; continue; } bb = 5; continue; }
            5 => { rdx = ((rax.wrapping_add(P3)) as u64) as u64; bb = 7; continue; }
            6 => { rdx = ((P4) as u64) as u64; bb = 7; continue; }
            7 => { rcx = ((rsp.wrapping_add(0x20u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e19dec); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rsi = (0x1u32) as u32 as u64; fl_l = (crate::simd_ops::lk_and_u32(rbx.wrapping_add(P6), (0x0u32) as u32)) as u32 as u64; fl_r = fl_l; rax = ((P7) as u64) as u64; crate::simd_ops::wu64(rbx, (rax) as u64); crate::simd_ops::wu32(rbx.wrapping_add(P8), (0x10ffffu32) as u32); crate::simd_ops::wu32(rbx.wrapping_add(P9), (0x4u32) as u32); bb = 9; continue; }
            8 => { rbx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; bb = 9; continue; }
            9 => { crate::simd_ops::wu64(rdi, (rbx) as u64); fl_l = ((rsi as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 11; continue; } bb = 10; continue; }
            10 => { rcx = ((rsp.wrapping_add(0x20u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e19e24); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P10) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 11; continue; }
            11 => { rax = (0x2u32) as u32 as u64; r11 = ((rsp.wrapping_add(0x90u64)) as u64) as u64; rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x18u64))) as u64; rbp = (crate::simd_ops::ru64(r11.wrapping_add(0x20u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x28u64))) as u64; rsp = (r11) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e19e50(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x18; const P1: u64 = 0x7001e659fc; const P2: u64 = 0x28; const P3: u64 = 0x30; const P4: u64 = 0x7001e6b8ca; const P5: u64 = 0x7001e184f0; const P6: u64 = 0x8; const P7: u64 = 0x7001e6b968; const P8: u64 = 0x10; const P9: u64 = 0x14; const P10: u64 = 0x7001e18dc0;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 19>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&super::EH_META[57], rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (rbp) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = ((rsp).wrapping_sub(0x90u64)) as u64; rbp = (rdx) as u64; rdi = (rcx) as u64; rsi = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rsp.wrapping_add(0xa0u64), ((rsi as u32)) as u32); fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru64(rcx)) as u64 as u64; fl_r = (rsi) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 11; continue; } bb = 2; continue; }
            2 => { rcx = ((rsi.wrapping_add(P0)) as u32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e19e83); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rbx = (rax) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xa0u64), (rax) as u64); fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x8u64))) as u64; fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 4; continue; }
            4 => { rdx = (crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64; fl_l = (rdx) as u64 as u64; fl_r = (rdx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 7; continue; } bb = 5; continue; }
            5 => { rdx = ((rax.wrapping_add(P3)) as u64) as u64; bb = 7; continue; }
            6 => { rdx = ((P4) as u64) as u64; bb = 7; continue; }
            7 => { rcx = ((rsp.wrapping_add(0x20u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e19ebc); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rsi = (0x1u32) as u32 as u64; crate::simd_ops::lk_and_u32(rbx.wrapping_add(P6), (0x0u32) as u32); rax = ((P7) as u64) as u64; crate::simd_ops::wu64(rbx, (rax) as u64); fl_l = (crate::simd_ops::lk_or_u32(rbx.wrapping_add(P8), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; crate::simd_ops::wu32(rbx.wrapping_add(P9), (0x4u32) as u32); bb = 9; continue; }
            8 => { rbx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; bb = 9; continue; }
            9 => { crate::simd_ops::wu64(rdi, (rbx) as u64); fl_l = ((rsi as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 11; continue; } bb = 10; continue; }
            10 => { rcx = ((rsp.wrapping_add(0x20u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e19ef1); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P10) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 11; continue; }
            11 => { rax = (0x2u32) as u32 as u64; r11 = ((rsp.wrapping_add(0x90u64)) as u64) as u64; rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x18u64))) as u64; rbp = (crate::simd_ops::ru64(r11.wrapping_add(0x20u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x28u64))) as u64; rsp = (r11) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e19f20(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x40; const P1: u64 = 0x7001e659fc; const P2: u64 = 0x28; const P3: u64 = 0x30; const P4: u64 = 0x7001e6b8ca; const P5: u64 = 0x7001e184f0; const P6: u64 = 0x8; const P7: u64 = 0x7001e6b9c0; const P8: u64 = 0x7001e4d900; const P9: u64 = 0x10; const P10: u64 = 0x20; const P11: u64 = 0x38; const P12: u64 = 0x7001e18dc0;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 25>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&super::EH_META[58], rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (rbp) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = ((rsp).wrapping_sub(0xc0u64)) as u64; rbp = (rdx) as u64; rdi = (rcx) as u64; rsi = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rsp.wrapping_add(0xd0u64), ((rsi as u32)) as u32); fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru64(rcx)) as u64 as u64; fl_r = (rsi) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 11; continue; } bb = 2; continue; }
            2 => { rcx = ((rsi.wrapping_add(P0)) as u32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e19f5b); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rbx = (rax) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xd0u64), (rax) as u64); fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x8u64))) as u64; fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 4; continue; }
            4 => { rdx = (crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64; fl_l = (rdx) as u64 as u64; fl_r = (rdx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 7; continue; } bb = 5; continue; }
            5 => { rdx = ((rax.wrapping_add(P3)) as u64) as u64; bb = 7; continue; }
            6 => { rdx = ((P4) as u64) as u64; bb = 7; continue; }
            7 => { rcx = ((rsp.wrapping_add(0x50u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e19f94); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rsi = (0x1u32) as u32 as u64; crate::simd_ops::lk_and_u32(rbx.wrapping_add(P6), (0x0u32) as u32); rax = ((P7) as u64) as u64; crate::simd_ops::wu64(rbx, (rax) as u64); rcx = ((rsp.wrapping_add(0x20u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e19fb1); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P8) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; xmm0 = (crate::simd_ops::ru128(rax)) as u128; crate::simd_ops::wu128(rbx.wrapping_add(P9), (xmm0) as u128); xmm1 = (crate::simd_ops::ru128(rax.wrapping_add(P9))) as u128; crate::simd_ops::wu128(rbx.wrapping_add(P10), (xmm1) as u128); xmm0 = ((crate::simd_ops::ru64(rax.wrapping_add(P10))) as u64) as u128; crate::simd_ops::wu64(rbx.wrapping_add(P3), ((xmm0 as u64)) as u64); rax = (crate::simd_ops::ru32(rax.wrapping_add(P2))) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(P11), ((rax as u32)) as u32); bb = 9; continue; }
            8 => { rbx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; bb = 9; continue; }
            9 => { crate::simd_ops::wu64(rdi, (rbx) as u64); fl_l = ((rsi as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 11; continue; } bb = 10; continue; }
            10 => { rcx = ((rsp.wrapping_add(0x50u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e19fe7); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P12) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 11; continue; }
            11 => { rax = (0x2u32) as u32 as u64; r11 = ((rsp.wrapping_add(0xc0u64)) as u64) as u64; rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x18u64))) as u64; rbp = (crate::simd_ops::ru64(r11.wrapping_add(0x20u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x28u64))) as u64; rsp = (r11) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e1a010(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x30; const P1: u64 = 0x7001e659fc; const P2: u64 = 0x28; const P3: u64 = 0x7001e6b8ca; const P4: u64 = 0x7001e184f0; const P5: u64 = 0x8; const P6: u64 = 0x7001e6ba90; const P7: u64 = 0x7001e16f90; const P8: u64 = 0x10; const P9: u64 = 0x20; const P10: u64 = 0x7001e18dc0;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 23>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&super::EH_META[59], rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (rbp) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = ((rsp).wrapping_sub(0xb0u64)) as u64; rbp = (rdx) as u64; rdi = (rcx) as u64; rsi = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rsp.wrapping_add(0xc0u64), ((rsi as u32)) as u32); fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru64(rcx)) as u64 as u64; fl_r = (rsi) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 11; continue; } bb = 2; continue; }
            2 => { rcx = ((rsi.wrapping_add(P0)) as u32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e1a04b); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rbx = (rax) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xc0u64), (rax) as u64); fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x8u64))) as u64; fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 4; continue; }
            4 => { rdx = (crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64; fl_l = (rdx) as u64 as u64; fl_r = (rdx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 7; continue; } bb = 5; continue; }
            5 => { rdx = ((rax.wrapping_add(P0)) as u64) as u64; bb = 7; continue; }
            6 => { rdx = ((P3) as u64) as u64; bb = 7; continue; }
            7 => { rcx = ((rsp.wrapping_add(0x40u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e1a084); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rsi = (0x1u32) as u32 as u64; crate::simd_ops::lk_and_u32(rbx.wrapping_add(P5), (0x0u32) as u32); rax = ((P6) as u64) as u64; crate::simd_ops::wu64(rbx, (rax) as u64); rcx = ((rsp.wrapping_add(0x20u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e1a0a1); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P7) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; xmm0 = (crate::simd_ops::ru128(rax)) as u128; crate::simd_ops::wu128(rbx.wrapping_add(P8), (xmm0) as u128); xmm1 = (crate::simd_ops::ru128(rax.wrapping_add(P8))) as u128; crate::simd_ops::wu128(rbx.wrapping_add(P9), (xmm1) as u128); bb = 9; continue; }
            8 => { rbx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; bb = 9; continue; }
            9 => { crate::simd_ops::wu64(rdi, (rbx) as u64); fl_l = ((rsi as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 11; continue; } bb = 10; continue; }
            10 => { rcx = ((rsp.wrapping_add(0x40u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e1a0c7); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P10) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 11; continue; }
            11 => { rax = (0x2u32) as u32 as u64; r11 = ((rsp.wrapping_add(0xb0u64)) as u64) as u64; rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x18u64))) as u64; rbp = (crate::simd_ops::ru64(r11.wrapping_add(0x20u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x28u64))) as u64; rsp = (r11) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e1a0f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001e659fc; const P1: u64 = 0x7001e6b8ca; const P2: u64 = 0x7001e184f0; const P3: u64 = 0x7001e6bb70; const P4: u64 = 0x7001e16f90; const P5: u64 = 0x7001e4d900; const P6: u64 = 0x7001e18dc0;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 29>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&super::EH_META[60], rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (rbp) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = ((rsp).wrapping_sub(0xe0u64)) as u64; rbp = (rdx) as u64; rdi = (rcx) as u64; rsi = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rsp.wrapping_add(0xf0u64), ((rsi as u32)) as u32); fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru64(rcx)) as u64 as u64; fl_r = (rsi) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 11; continue; } bb = 2; continue; }
            2 => { rcx = ((rsi.wrapping_add(0x60u64)) as u32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e1a12b); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rbx = (rax) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xf0u64), (rax) as u64); fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x8u64))) as u64; fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 4; continue; }
            4 => { rdx = (crate::simd_ops::ru64(rax.wrapping_add(0x28u64))) as u64; fl_l = (rdx) as u64 as u64; fl_r = (rdx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 7; continue; } bb = 5; continue; }
            5 => { rdx = ((rax.wrapping_add(0x30u64)) as u64) as u64; bb = 7; continue; }
            6 => { rdx = ((P1) as u64) as u64; bb = 7; continue; }
            7 => { rcx = ((rsp.wrapping_add(0x70u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e1a168); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rsi = (0x1u32) as u32 as u64; crate::simd_ops::lk_and_u32(rbx.wrapping_add(0x8u64), (0x0u32) as u32); rax = ((P3) as u64) as u64; crate::simd_ops::wu64(rbx, (rax) as u64); rcx = ((rsp.wrapping_add(0x20u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e1a185); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; xmm0 = (crate::simd_ops::ru128(rax)) as u128; crate::simd_ops::wu128(rbx.wrapping_add(0x10u64), (xmm0) as u128); xmm1 = (crate::simd_ops::ru128(rax.wrapping_add(0x10u64))) as u128; crate::simd_ops::wu128(rbx.wrapping_add(0x20u64), (xmm1) as u128); rcx = ((rsp.wrapping_add(0x40u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e1a19e); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; xmm0 = (crate::simd_ops::ru128(rax)) as u128; crate::simd_ops::wu128(rbx.wrapping_add(0x30u64), (xmm0) as u128); xmm1 = (crate::simd_ops::ru128(rax.wrapping_add(0x10u64))) as u128; crate::simd_ops::wu128(rbx.wrapping_add(0x40u64), (xmm1) as u128); xmm0 = ((crate::simd_ops::ru64(rax.wrapping_add(0x20u64))) as u64) as u128; crate::simd_ops::wu64(rbx.wrapping_add(0x50u64), ((xmm0 as u64)) as u64); rax = (crate::simd_ops::ru32(rax.wrapping_add(0x28u64))) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x58u64), ((rax as u32)) as u32); bb = 9; continue; }
            8 => { rbx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; bb = 9; continue; }
            9 => { crate::simd_ops::wu64(rdi, (rbx) as u64); fl_l = ((rsi as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 11; continue; } bb = 10; continue; }
            10 => { rcx = ((rsp.wrapping_add(0x70u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e1a1d4); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 11; continue; }
            11 => { rax = (0x2u32) as u32 as u64; r11 = ((rsp.wrapping_add(0xe0u64)) as u64) as u64; rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x18u64))) as u64; rbp = (crate::simd_ops::ru64(r11.wrapping_add(0x20u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x28u64))) as u64; rsp = (r11) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e1a200(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001e659fc; const P1: u64 = 0x7001e6b8ca; const P2: u64 = 0x7001e184f0; const P3: u64 = 0x7001e6baf0; const P4: u64 = 0x7001e16f90; const P5: u64 = 0x7001e4d900; const P6: u64 = 0x7001e18dc0;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 29>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&super::EH_META[61], rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (rbp) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = ((rsp).wrapping_sub(0xe0u64)) as u64; rbp = (rdx) as u64; rdi = (rcx) as u64; rsi = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rsp.wrapping_add(0xf0u64), ((rsi as u32)) as u32); fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru64(rcx)) as u64 as u64; fl_r = (rsi) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 11; continue; } bb = 2; continue; }
            2 => { rcx = ((rsi.wrapping_add(0x60u64)) as u32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e1a23b); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rbx = (rax) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xf0u64), (rax) as u64); fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x8u64))) as u64; fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 4; continue; }
            4 => { rdx = (crate::simd_ops::ru64(rax.wrapping_add(0x28u64))) as u64; fl_l = (rdx) as u64 as u64; fl_r = (rdx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 7; continue; } bb = 5; continue; }
            5 => { rdx = ((rax.wrapping_add(0x30u64)) as u64) as u64; bb = 7; continue; }
            6 => { rdx = ((P1) as u64) as u64; bb = 7; continue; }
            7 => { rcx = ((rsp.wrapping_add(0x70u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e1a278); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rsi = (0x1u32) as u32 as u64; crate::simd_ops::lk_and_u32(rbx.wrapping_add(0x8u64), (0x0u32) as u32); rax = ((P3) as u64) as u64; crate::simd_ops::wu64(rbx, (rax) as u64); rcx = ((rsp.wrapping_add(0x20u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e1a295); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; xmm0 = (crate::simd_ops::ru128(rax)) as u128; crate::simd_ops::wu128(rbx.wrapping_add(0x10u64), (xmm0) as u128); xmm1 = (crate::simd_ops::ru128(rax.wrapping_add(0x10u64))) as u128; crate::simd_ops::wu128(rbx.wrapping_add(0x20u64), (xmm1) as u128); rcx = ((rsp.wrapping_add(0x40u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e1a2ae); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; xmm0 = (crate::simd_ops::ru128(rax)) as u128; crate::simd_ops::wu128(rbx.wrapping_add(0x30u64), (xmm0) as u128); xmm1 = (crate::simd_ops::ru128(rax.wrapping_add(0x10u64))) as u128; crate::simd_ops::wu128(rbx.wrapping_add(0x40u64), (xmm1) as u128); xmm0 = ((crate::simd_ops::ru64(rax.wrapping_add(0x20u64))) as u64) as u128; crate::simd_ops::wu64(rbx.wrapping_add(0x50u64), ((xmm0 as u64)) as u64); rax = (crate::simd_ops::ru32(rax.wrapping_add(0x28u64))) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x58u64), ((rax as u32)) as u32); bb = 9; continue; }
            8 => { rbx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; bb = 9; continue; }
            9 => { crate::simd_ops::wu64(rdi, (rbx) as u64); fl_l = ((rsi as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 11; continue; } bb = 10; continue; }
            10 => { rcx = ((rsp.wrapping_add(0x70u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e1a2e4); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 11; continue; }
            11 => { rax = (0x2u32) as u32 as u64; r11 = ((rsp.wrapping_add(0xe0u64)) as u64) as u64; rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x18u64))) as u64; rbp = (crate::simd_ops::ru64(r11.wrapping_add(0x20u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x28u64))) as u64; rsp = (r11) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e1a310(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    fl_l = (((rax) | (0xffffffffffffffffu64))) as u64 as u64; fl_r = fl_l;
    rax = ((fl_l as u64)) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e1a320(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001e4ce00;
    let (mut rax, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rsp = rsp.wrapping_sub(8);
    core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64);
    rsp = ((rsp).wrapping_sub(0x30u64)) as u64;
    rcx = ((rsp.wrapping_add(0x20u64)) as u64) as u64;
    rbx = (rdx) as u64;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0;
    xmm0 = (crate::simd_ops::ru128(rax)) as u128;
    rax = (rbx) as u64;
    crate::simd_ops::wu128(rbx, (xmm0) as u128);
    rsp = ((rsp).wrapping_add(0x30u64)) as u64;
    rbx = core::ptr::read_unaligned(rsp as *const u64);
    rsp = rsp.wrapping_add(8);
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e1a350(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001e16f90; const P1: u64 = 0x10;
    let (mut rax, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rsp = rsp.wrapping_sub(8);
    core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64);
    rsp = ((rsp).wrapping_sub(0x40u64)) as u64;
    rcx = ((rsp.wrapping_add(0x20u64)) as u64) as u64;
    rbx = (rdx) as u64;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0;
    xmm0 = (crate::simd_ops::ru128(rax)) as u128;
    xmm1 = (crate::simd_ops::ru128(rax.wrapping_add(P1))) as u128;
    rax = (rbx) as u64;
    crate::simd_ops::wu128(rbx, (xmm0) as u128);
    crate::simd_ops::wu128(rbx.wrapping_add(P1), (xmm1) as u128);
    rsp = ((rsp).wrapping_add(0x40u64)) as u64;
    rbx = core::ptr::read_unaligned(rsp as *const u64);
    rsp = rsp.wrapping_add(8);
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e1a380(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001e4d900; const P1: u64 = 0x10; const P2: u64 = 0x20; const P3: u64 = 0x28;
    let (mut rax, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rsp = rsp.wrapping_sub(8);
    core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64);
    rsp = ((rsp).wrapping_sub(0x50u64)) as u64;
    rcx = ((rsp.wrapping_add(0x20u64)) as u64) as u64;
    rbx = (rdx) as u64;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0;
    xmm0 = (crate::simd_ops::ru128(rax)) as u128;
    xmm1 = (crate::simd_ops::ru128(rax.wrapping_add(P1))) as u128;
    crate::simd_ops::wu128(rbx, (xmm0) as u128);
    xmm0 = ((crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64) as u128;
    rax = (crate::simd_ops::ru32(rax.wrapping_add(P3))) as u32 as u64;
    crate::simd_ops::wu128(rbx.wrapping_add(P1), (xmm1) as u128);
    crate::simd_ops::wu64(rbx.wrapping_add(P2), ((xmm0 as u64)) as u64);
    crate::simd_ops::wu32(rbx.wrapping_add(P3), ((rax as u32)) as u32);
    rax = (rbx) as u64;
    rsp = ((rsp).wrapping_add(0x50u64)) as u64;
    rbx = core::ptr::read_unaligned(rsp as *const u64);
    rsp = rsp.wrapping_add(8);
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e1a490(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x58; const P1: u64 = 0x60;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64;
    if !(((((fl_l as u64)) & ((fl_r as u64))) != 0)) { rax = ((rcx.wrapping_add(P1)) as u64) as u64; }
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e1a4a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e1a500(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x50; const P1: u64 = 0x38;
    let (mut rax, mut fl_l) = (crate::simd_ops::get_rax_in(), 0u64);
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    crate::simd_ops::lk_add_u32(rax, (1) as u32);
    rdx = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64;
    rax = (crate::simd_ops::ru64(rdx)) as u64;
    fl_l = ((rax).wrapping_sub(1)) as u64 as u64;
    rax = ((fl_l as u64)) as u64;
    crate::simd_ops::wu64(rdx, (rax) as u64);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e1a540(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8;
    let (mut rax, mut fl_l) = (crate::simd_ops::get_rax_in(), 0u64);
    fl_l = (crate::simd_ops::lk_add_u32(rcx.wrapping_add(P0), (1) as u32)) as u32 as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e1a548(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x71; const P1: u64 = 0x8; const P2: u64 = 0x18; const P3: u64 = 0x28; const P4: u64 = 0x38; const P5: u64 = 0x10; const P6: u64 = 0x20; const P7: u64 = 0x7c; const P8: u64 = 0x48; const P9: u64 = 0x50; const P10: u64 = 0x4c; const P11: u64 = 0x58; const P12: u64 = 0x7001e69590; const P13: u64 = 0x7001e98cc0;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { r11 = (rsp) as u64; crate::simd_ops::wu64(r11.wrapping_add(0x18u64), (rbx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; crate::simd_ops::wu8(rcx.wrapping_add(P0), (0x0u8) as u8); r10 = ((rcx.wrapping_add(P1)) as u64) as u64; crate::simd_ops::wu64(rcx.wrapping_add(P2), (r10) as u64); r9 = ((rcx.wrapping_add(P3)) as u64) as u64; crate::simd_ops::wu64(rcx.wrapping_add(P4), (r9) as u64); rbx = (rcx) as u64; fl_l = ((r8 as u32)) as u32 as u64; fl_r = (0x1u32) as u32 as u64; rdi = (rdx) as u64; r8 = ((rcx.wrapping_add(P5)) as u64) as u64; rax = (rax & !0xffu64) | (((if (((fl_l as u32)) == ((fl_r as u32))) { 1u8 } else { 0u8 }) as u8 as u64)); crate::simd_ops::wu64(rcx.wrapping_add(P6), (r8) as u64); crate::simd_ops::wu8(rcx.wrapping_add(P7), ((rax as u8)) as u8); rdx = ((rbx.wrapping_add(P8)) as u64) as u64; crate::simd_ops::wu64(rbx.wrapping_add(P9), (rdx) as u64); rax = ((rbx.wrapping_add(P10)) as u64) as u64; crate::simd_ops::wu64(rbx.wrapping_add(P11), (rax) as u64); rcx = ((rcx).wrapping_add(0x30u64)) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x40u64), (rcx) as u64); crate::simd_ops::lk_and_u64(r8, (0x0u64) as u64); crate::simd_ops::lk_and_u64(rcx, (0x0u64) as u64); crate::simd_ops::lk_and_u32(rax, (0x0u32) as u32); crate::simd_ops::lk_and_u64(r10, (0x0u64) as u64); crate::simd_ops::lk_and_u64(r9, (0x0u64) as u64); crate::simd_ops::lk_and_u32(rdx, (0x0u32) as u32); fl_l = (rdi) as u64 as u64; fl_r = (rdi) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::lk_and_u64(r11.wrapping_add(0x8u64), (0x0u64) as u64); r9 = ((r11.wrapping_add(0x20u64)) as u64) as u64; crate::simd_ops::lk_and_u64(r11.wrapping_add(0x10u64), (0x0u64) as u64); r8 = ((r11.wrapping_add(0x10u64)) as u64) as u64; crate::simd_ops::lk_and_u64(r11.wrapping_add(0x20u64), (0x0u64) as u64); rdx = ((r11.wrapping_add(0x8u64)) as u64) as u64; rcx = (rdi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P12)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x18u64), (rax) as u64); crate::simd_ops::wu64(rbx.wrapping_add(0x20u64), (rax) as u64); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x38u64), (rax) as u64); crate::simd_ops::wu64(rbx.wrapping_add(0x40u64), (rax) as u64); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x50u64), (rax) as u64); crate::simd_ops::wu64(rbx.wrapping_add(0x58u64), (rax) as u64); bb = 2; continue; }
            2 => { crate::simd_ops::wu64(rbx.wrapping_add(0x80u64), (rdi) as u64); rax = (crate::simd_ops::ru64(P13)) as u64; crate::simd_ops::lk_and_u64(rbx.wrapping_add(0x68u64), (0x0u64) as u64); crate::simd_ops::wu64(rbx.wrapping_add(0x74u64), (rax) as u64); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e1a620(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x20; const P1: u64 = 0x40; const P2: u64 = 0x58; const P3: u64 = 0x18; const P4: u64 = 0x38; const P5: u64 = 0x50;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 0>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64;
    crate::simd_ops::wu64(rcx.wrapping_add(P0), (rax) as u64);
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    crate::simd_ops::wu64(rcx.wrapping_add(P1), (rax) as u64);
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64;
    crate::simd_ops::wu64(rcx.wrapping_add(P2), (rax) as u64);
    crate::simd_ops::wu64(rcx.wrapping_add(P3), (rdx) as u64);
    crate::simd_ops::wu64(rcx.wrapping_add(P4), (r8) as u64);
    crate::simd_ops::wu64(rcx.wrapping_add(P5), (r9) as u64);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e1a650(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x18; const P2: u64 = 0x10; const P3: u64 = 0x20; const P4: u64 = 0x28; const P5: u64 = 0x38; const P6: u64 = 0x30; const P7: u64 = 0x40; const P8: u64 = 0x48; const P9: u64 = 0x50; const P10: u64 = 0x4c; const P11: u64 = 0x58;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    r11 = ((rcx.wrapping_add(P0)) as u64) as u64;
    crate::simd_ops::wu64(rcx.wrapping_add(P1), (r11) as u64);
    r9 = ((rcx.wrapping_add(P2)) as u64) as u64;
    crate::simd_ops::wu64(rcx.wrapping_add(P3), (r9) as u64);
    r10 = ((rcx.wrapping_add(P4)) as u64) as u64;
    crate::simd_ops::wu64(rcx.wrapping_add(P5), (r10) as u64);
    rdx = ((rcx.wrapping_add(P6)) as u64) as u64;
    crate::simd_ops::wu64(rcx.wrapping_add(P7), (rdx) as u64);
    r8 = ((rcx.wrapping_add(P8)) as u64) as u64;
    crate::simd_ops::wu64(rcx.wrapping_add(P9), (r8) as u64);
    rax = ((rcx.wrapping_add(P10)) as u64) as u64;
    crate::simd_ops::wu64(rcx.wrapping_add(P11), (rax) as u64);
    rcx = (0u32) as u32 as u64;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu64(r9, (rcx) as u64);
    crate::simd_ops::wu64(rdx, (rcx) as u64);
    crate::simd_ops::wu32(rax, ((rcx as u32)) as u32);
    crate::simd_ops::wu64(r11, (rcx) as u64);
    crate::simd_ops::wu64(r10, (rcx) as u64);
    crate::simd_ops::wu32(r8, ((rcx as u32)) as u32);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e1a6a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001e4d900; const P1: u64 = 0x10; const P2: u64 = 0x20; const P3: u64 = 0x30; const P4: u64 = 0x28; const P5: u64 = 0x38;
    let (mut rax, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rsp = rsp.wrapping_sub(8);
    core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64);
    rsp = ((rsp).wrapping_sub(0x50u64)) as u64;
    rbx = (rcx) as u64;
    rcx = ((rsp.wrapping_add(0x20u64)) as u64) as u64;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0;
    xmm0 = (crate::simd_ops::ru128(rax)) as u128;
    crate::simd_ops::wu128(rbx.wrapping_add(P1), (xmm0) as u128);
    xmm1 = (crate::simd_ops::ru128(rax.wrapping_add(P1))) as u128;
    crate::simd_ops::wu128(rbx.wrapping_add(P2), (xmm1) as u128);
    xmm0 = ((crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64) as u128;
    crate::simd_ops::wu64(rbx.wrapping_add(P3), ((xmm0 as u64)) as u64);
    rax = (crate::simd_ops::ru32(rax.wrapping_add(P4))) as u32 as u64;
    crate::simd_ops::wu32(rbx.wrapping_add(P5), ((rax as u32)) as u32);
    rsp = ((rsp).wrapping_add(0x50u64)) as u64;
    rbx = core::ptr::read_unaligned(rsp as *const u64);
    rsp = rsp.wrapping_add(8);
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e1a6e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001e16f90; const P1: u64 = 0x10; const P2: u64 = 0x20;
    let (mut rax, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rsp = rsp.wrapping_sub(8);
    core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64);
    rsp = ((rsp).wrapping_sub(0x40u64)) as u64;
    rbx = (rcx) as u64;
    rcx = ((rsp.wrapping_add(0x20u64)) as u64) as u64;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0;
    xmm0 = (crate::simd_ops::ru128(rax)) as u128;
    crate::simd_ops::wu128(rbx.wrapping_add(P1), (xmm0) as u128);
    xmm1 = (crate::simd_ops::ru128(rax.wrapping_add(P1))) as u128;
    crate::simd_ops::wu128(rbx.wrapping_add(P2), (xmm1) as u128);
    rsp = ((rsp).wrapping_add(0x40u64)) as u64;
    rbx = core::ptr::read_unaligned(rsp as *const u64);
    rsp = rsp.wrapping_add(8);
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e1a710(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001e16f90; const P1: u64 = 0x10; const P2: u64 = 0x20; const P3: u64 = 0x7001e4d900; const P4: u64 = 0x30; const P5: u64 = 0x40; const P6: u64 = 0x50; const P7: u64 = 0x28; const P8: u64 = 0x58;
    let (mut rax, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rsp = rsp.wrapping_sub(8);
    core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64);
    rsp = ((rsp).wrapping_sub(0x50u64)) as u64;
    rbx = (rcx) as u64;
    rcx = ((rsp.wrapping_add(0x20u64)) as u64) as u64;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0;
    rcx = ((rsp.wrapping_add(0x20u64)) as u64) as u64;
    xmm0 = (crate::simd_ops::ru128(rax)) as u128;
    crate::simd_ops::wu128(rbx.wrapping_add(P1), (xmm0) as u128);
    xmm1 = (crate::simd_ops::ru128(rax.wrapping_add(P1))) as u128;
    crate::simd_ops::wu128(rbx.wrapping_add(P2), (xmm1) as u128);
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0;
    xmm0 = (crate::simd_ops::ru128(rax)) as u128;
    crate::simd_ops::wu128(rbx.wrapping_add(P4), (xmm0) as u128);
    xmm1 = (crate::simd_ops::ru128(rax.wrapping_add(P1))) as u128;
    crate::simd_ops::wu128(rbx.wrapping_add(P5), (xmm1) as u128);
    xmm0 = ((crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64) as u128;
    crate::simd_ops::wu64(rbx.wrapping_add(P6), ((xmm0 as u64)) as u64);
    rax = (crate::simd_ops::ru32(rax.wrapping_add(P7))) as u32 as u64;
    crate::simd_ops::wu32(rbx.wrapping_add(P8), ((rax as u32)) as u32);
    rsp = ((rsp).wrapping_add(0x50u64)) as u64;
    rbx = core::ptr::read_unaligned(rsp as *const u64);
    rsp = rsp.wrapping_add(8);
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e1a770(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x40; const P1: u64 = 0x8; const P2: u64 = 0x14; const P3: u64 = 0x18; const P4: u64 = 0x20; const P5: u64 = 0x28; const P6: u64 = 0x30; const P7: u64 = 0x38; const P8: u64 = 0x7001e1acd0; const P9: u64 = 0x7001e659fc; const P10: u64 = 0x7001e25690;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&super::EH_META[62], rsp); let _eh = eh.push();
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rbx) as u64);
    rsp = rsp.wrapping_sub(8);
    core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = (rcx) as u64;
    crate::simd_ops::lk_and_u64(rcx.wrapping_add(P0), (0x0u64) as u64);
    crate::simd_ops::lk_and_u64(rcx.wrapping_add(P1), (0x0u64) as u64);
    crate::simd_ops::lk_and_u32(rcx.wrapping_add(P2), (0x0u32) as u32);
    crate::simd_ops::wu32(rcx.wrapping_add(P3), (0x201u32) as u32);
    crate::simd_ops::wu64(rcx.wrapping_add(P4), (0x6u64) as u64);
    crate::simd_ops::lk_and_u64(rcx.wrapping_add(P5), (0x0u64) as u64);
    crate::simd_ops::lk_and_u64(rcx.wrapping_add(P6), (0x0u64) as u64);
    crate::simd_ops::lk_and_u64(rcx.wrapping_add(P7), (0x0u64) as u64);
    rdx = (0u32) as u32 as u64;
    fl_l = 0; fl_r = 0;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    eh.site(0x7001e1a7ab); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P8) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    rcx = (0x10u32) as u32 as u64;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    eh.site(0x7001e1a7b5); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P9) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    rdi = (rax) as u64;
    fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rcx = (rcx & !0xffu64) | (((0x1u8) as u8 as u64)); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e1a7c4); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P10) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); crate::simd_ops::wu64(rdi.wrapping_add(P1), (rax) as u64); } else { rdi = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; }
    crate::simd_ops::wu64(rbx.wrapping_add(P0), (rdi) as u64);
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = core::ptr::read_unaligned(rsp as *const u64);
    rsp = rsp.wrapping_add(8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e1a7e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x18; const P1: u64 = 0x7001e696a0; const P2: u64 = 0x68; const P3: u64 = 0x8; const P4: u64 = 0x10; const P5: u64 = 0x20; const P6: u64 = 0x28; const P7: u64 = 0x38; const P8: u64 = 0x30; const P9: u64 = 0x40; const P10: u64 = 0x48; const P11: u64 = 0x50;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rbx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (crate::simd_ops::ru64(rdx)) as u64; rbx = (rcx) as u64; rcx = (rdx) as u64; rdi = (rdx) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P1)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); r11 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rbx.wrapping_add(P2), (r11) as u64); bb = 3; continue; }
            2 => { crate::simd_ops::wu64(rbx.wrapping_add(P2), (rdi) as u64); r10 = ((rbx.wrapping_add(P3)) as u64) as u64; crate::simd_ops::wu64(rbx.wrapping_add(P0), (r10) as u64); r8 = ((rbx.wrapping_add(P4)) as u64) as u64; crate::simd_ops::wu64(rbx.wrapping_add(P5), (r8) as u64); r9 = ((rbx.wrapping_add(P6)) as u64) as u64; crate::simd_ops::wu64(rbx.wrapping_add(P7), (r9) as u64); rcx = ((rbx.wrapping_add(P8)) as u64) as u64; crate::simd_ops::wu64(rbx.wrapping_add(P9), (rcx) as u64); rdx = ((rbx.wrapping_add(P10)) as u64) as u64; crate::simd_ops::wu64(rbx.wrapping_add(P11), (rdx) as u64); rax = ((rbx.wrapping_add(0x4cu64)) as u64) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x58u64), (rax) as u64); crate::simd_ops::wu64(r8, (r11) as u64); crate::simd_ops::wu64(rcx, (r11) as u64); crate::simd_ops::wu32(rax, ((r11 as u32)) as u32); crate::simd_ops::wu64(r10, (r11) as u64); crate::simd_ops::wu64(r9, (r11) as u64); crate::simd_ops::wu32(rdx, ((r11 as u32)) as u32); bb = 3; continue; }
            3 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e1a87c(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001e97380; const P1: u64 = 0x7001e6b4e0; const P2: u64 = 0x7001e1ac00; const P3: u64 = 0x7001e696a0; const P4: u64 = 0x7001e69480; const P5: u64 = 0x7001e1a941; const P6: u64 = 0x7001e6649c;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 15>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&super::EH_META[63], rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (r8) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rcx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = ((rsp).wrapping_sub(0x60u64)) as u64; rax = (crate::simd_ops::ru64(P0)) as u64; rax = (((rax) ^ (rsp))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), (rax) as u64); rbx = (r8) as u64; rsi = (rdx) as u64; rdi = (rcx) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (rcx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (rbx) as u64); fl_l = (crate::simd_ops::ru64(r8.wrapping_add(0x10u64))) as u64 as u64; fl_r = (0x0u64) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { r8 = (0x2u32) as u32 as u64; rdx = ((P1) as u64) as u64; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e1a8c6); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 2; continue; }
            2 => { rcx = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; r8 = (crate::simd_ops::ru32(rsi)) as u32 as u64; rdx = ((rsp.wrapping_add(0x30u64)) as u64) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e1a8de); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P3)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdx = ((rsp.wrapping_add(0x30u64)) as u64) as u64; rsi = (0xfu32) as u32 as u64; fl_l = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64 as u64; fl_r = (rsi) as u64 as u64; rdx = (if (((fl_l as u64)) > ((fl_r as u64))) { crate::simd_ops::ru64(rsp.wrapping_add(0x30u64)) } else { rdx }) as u64; r8 = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e1a902); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; fl_l = (rdx) as u64 as u64; fl_r = (rsi) as u64 as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 7; continue; } bb = 3; continue; }
            3 => { rdx = ((rdx).wrapping_add(1)) as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = (rcx) as u64; fl_l = (rdx) as u64 as u64; fl_r = (0x1000u64) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 6; continue; } bb = 4; continue; }
            4 => { rdx = ((rdx).wrapping_add(0x27u64)) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_sub(0x8u64))) as u64; rax = ((rax).wrapping_sub(rcx)) as u64; rax = ((rax).wrapping_add(0xfffffffffffffff8u64)) as u64; fl_l = (rax) as u64 as u64; fl_r = (0x1fu64) as u64 as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 6; continue; } bb = 5; continue; }
            5 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e1a93b); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P4)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::lift_ops::trap(0x80000003, P5, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            6 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e1a942); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 7; continue; }
            7 => { xmm0 = 0u128; crate::simd_ops::wu128(rdi, (xmm0) as u128); crate::simd_ops::lk_and_u64(rdi.wrapping_add(0x10u64), (0x0u64) as u64); crate::simd_ops::lk_and_u64(rdi.wrapping_add(0x18u64), (0x0u64) as u64); xmm0 = (crate::simd_ops::ru128(rbx)) as u128; crate::simd_ops::wu128(rdi, (xmm0) as u128); xmm1 = (crate::simd_ops::ru128(rbx.wrapping_add(0x10u64))) as u128; crate::simd_ops::wu128(rdi.wrapping_add(0x10u64), (xmm1) as u128); crate::simd_ops::lk_and_u64(rbx.wrapping_add(0x10u64), (0x0u64) as u64); crate::simd_ops::wu64(rbx.wrapping_add(0x18u64), (rsi) as u64); crate::simd_ops::wu8(rbx, (0x0u8) as u8); crate::simd_ops::lk_and_u64(rbx.wrapping_add(0x10u64), (0x0u64) as u64); crate::simd_ops::wu64(rbx.wrapping_add(0x18u64), (rsi) as u64); crate::simd_ops::wu8(rbx, (0x0u8) as u8); rax = (rdi) as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rcx = (((rcx) ^ (rsp))) as u64; rsp = ((rsp).wrapping_add(0x60u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rsi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rbx = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e1aa20(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x20; const P1: u64 = 0x18; const P2: u64 = 0x7001e693a8; const P3: u64 = 0x7001e6649c; const P4: u64 = 0x28;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; rbx = (rcx) as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P2)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 4; continue; }
            2 => { if (((((fl_l as u32)) & ((fl_r as u32))) as i32) >= 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 4; continue; }
            4 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P4))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); let s5 = crate::simd_ops::read_s5_tail(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x40) as *const u64); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P2)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e1aa60(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x38; const P1: u64 = 0x8; const P2: u64 = 0x10; const P3: u64 = 0x7001e696a0; const P4: u64 = 0x30; const P5: u64 = 0x7001e6649c;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&super::EH_META[64], rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rbx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = (rcx) as u64; rbx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; bb = 2; continue; }
            1 => { r8 = (crate::simd_ops::ru32(rbx.wrapping_add(P1))) as u32 as u64; rdx = (rdi) as u64; rcx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rax = (crate::simd_ops::ru64(rbx.wrapping_add(P2))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e1aa80); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P3)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rbx = (crate::simd_ops::ru64(rbx)) as u64; bb = 2; continue; }
            2 => { fl_l = (rbx) as u64 as u64; fl_r = (rbx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 1; continue; } bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(P4))) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rbx = (crate::simd_ops::ru64(rcx)) as u64; rdx = (0x18u32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e1aa9f); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rcx = (rbx) as u64; fl_l = (rbx) as u64 as u64; fl_r = (rbx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 4; continue; } bb = 5; continue; }
            5 => { crate::simd_ops::lk_and_u64(rdi.wrapping_add(P4), (0x0u64) as u64); rcx = (crate::simd_ops::ru64(rdi.wrapping_add(P0))) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 6; continue; }
            6 => { rbx = (crate::simd_ops::ru64(rcx)) as u64; rdx = (0x18u32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e1aac2); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rcx = (rbx) as u64; fl_l = (rbx) as u64 as u64; fl_r = (rbx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 6; continue; } bb = 7; continue; }
            7 => { crate::simd_ops::lk_and_u64(rdi.wrapping_add(P0), (0x0u64) as u64); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e1ac00(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x18; const P2: u64 = 0x7001e6646c; const P3: u64 = 0x7001e176a4;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rdi = (rcx) as u64; rsi = (r8) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rax = (crate::simd_ops::ru64(rdi.wrapping_add(P1))) as u64; rax = ((rax).wrapping_sub(rcx)) as u64; fl_l = (r8) as u64 as u64; fl_r = (rax) as u64 as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 4; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru64(rdi.wrapping_add(P1))) as u64 as u64; fl_r = (0xfu64) as u64 as u64; rax = ((rcx.wrapping_add(r8)) as u64) as u64; crate::simd_ops::wu64(rdi.wrapping_add(P0), (rax) as u64); rax = (rdi) as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 3; continue; } bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(rdi)) as u64; bb = 3; continue; }
            3 => { rbx = ((rax.wrapping_add(rcx)) as u64) as u64; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rax = (rdi) as u64; crate::simd_ops::wu8(rbx.wrapping_add(rsi), (0x0u8) as u8); bb = 5; continue; }
            4 => { r8 = (r8 & !0xffu64) | (((crate::simd_ops::ru8(rsp.wrapping_add(0x40u64))) as u8 as u64)); r9 = (rdx) as u64; rdx = (rsi) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rsi) as u64); rcx = (rdi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 5; continue; }
            5 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e1ac80(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x10;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64;
    rax = (((rax as u32)).wrapping_shr(2u32)) as u32 as u64;
    fl_l = ((((rax as u8)) & (0x1u8))) as u8 as u64; fl_r = fl_l;
    rax = (rax & !0xffu64) | ((((fl_l as u8)) as u8 as u64));
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e1ac90(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001e25930; const P1: u64 = 0x7001e17a54; const P2: u64 = 0x18;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&super::EH_META[65], rsp); let _eh = eh.push();
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    eh.site(0x7001e1ac94); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    rcx = (rax) as u64;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    eh.site(0x7001e1ac9c); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    rax = (crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64;
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e1acb0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x48; const P1: u64 = 0x7001e1ad40;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    r9 = ((rdx as u32)) as u32 as u64;
    fl_l = 0; fl_r = (rax) as u64 as u64;
    rax = (0u64.wrapping_sub(fl_r as u64)) as u64;
    rdx = (if (((fl_l as u64)) < ((fl_r as u64))) { 0xffffffffu32 } else { 0u32 }) as u32 as u64;
    rdx = (!((rdx as u32))) as u32 as u64;
    rdx = ((((rdx as u32)) & (0x4u32))) as u32 as u64;
    fl_l = ((((rdx as u32)) | ((r9 as u32)))) as u32 as u64; fl_r = fl_l;
    rdx = ((fl_l as u32)) as u32 as u64;
    crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e1acd0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x14; const P2: u64 = 0x7001e6bc10; const P3: u64 = 0x7001e6bc28; const P4: u64 = 0x7001e6bc40; const P5: u64 = 0x7001e1c690; const P6: u64 = 0x7001e1887c; const P7: u64 = 0x7001e76d68; const P8: u64 = 0x7001e66444; const P9: u64 = 0x7001e1ad3a;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64); rsp = ((rsp).wrapping_sub(0x60u64)) as u64; rdx = ((((rdx as u32)) & (0x17u32))) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(P0), ((rdx as u32)) as u32); fl_l = ((((rdx as u32)) & (crate::simd_ops::ru32(rcx.wrapping_add(P1))))) as u32 as u64; fl_r = fl_l; rdx = ((fl_l as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rsp = ((rsp).wrapping_add(0x60u64)) as u64; rbx = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            2 => { fl_l = ((rdx as u8)) as u8 as u64; fl_r = (0x4u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { rbx = ((P2) as u64) as u64; bb = 5; continue; }
            4 => { fl_l = ((rdx as u8)) as u8 as u64; fl_r = (0x2u8) as u8 as u64; rbx = ((P3) as u64) as u64; rax = ((P4) as u64) as u64; rbx = (if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { rax } else { rbx }) as u64; bb = 5; continue; }
            5 => { rdx = (0x1u32) as u32 as u64; rcx = ((rsp.wrapping_add(0x20u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); r8 = (rax) as u64; rcx = ((rsp.wrapping_add(0x30u64)) as u64) as u64; rdx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rdx = ((P7) as u64) as u64; rcx = ((rsp.wrapping_add(0x30u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P8) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); crate::lift_ops::trap(0x80000003, P9, rcx, rsp); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e1ad40(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x14; const P2: u64 = 0x7001e66444; const P3: u64 = 0x7001e1ad67; const P4: u64 = 0x7001e6bc10; const P5: u64 = 0x7001e6bc28; const P6: u64 = 0x7001e6bc40; const P7: u64 = 0x7001e1c690; const P8: u64 = 0x7001e1887c; const P9: u64 = 0x7001e76d68; const P10: u64 = 0x7001e1adbb;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64); rsp = ((rsp).wrapping_sub(0x60u64)) as u64; rdx = ((((rdx as u32)) & (0x17u32))) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(P0), ((rdx as u32)) as u32); fl_l = ((((rdx as u32)) & (crate::simd_ops::ru32(rcx.wrapping_add(P1))))) as u32 as u64; fl_r = fl_l; rdx = ((fl_l as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { fl_l = ((r8 as u8)) as u8 as u64; fl_r = ((r8 as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 5; continue; } bb = 2; continue; }
            2 => { bb = 4; continue; }
            3 => { rsp = ((rsp).wrapping_add(0x60u64)) as u64; rbx = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            4 => { rdx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rcx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); crate::lift_ops::trap(0x80000003, P3, rcx, rsp); return rax; }
            5 => { fl_l = ((rdx as u8)) as u8 as u64; fl_r = (0x4u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 7; continue; } bb = 6; continue; }
            6 => { rbx = ((P4) as u64) as u64; bb = 8; continue; }
            7 => { fl_l = ((rdx as u8)) as u8 as u64; fl_r = (0x2u8) as u8 as u64; rbx = ((P5) as u64) as u64; rax = ((P6) as u64) as u64; rbx = (if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { rax } else { rbx }) as u64; bb = 8; continue; }
            8 => { rdx = (0x1u32) as u32 as u64; rcx = ((rsp.wrapping_add(0x20u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P7) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); r8 = (rax) as u64; rcx = ((rsp.wrapping_add(0x30u64)) as u64) as u64; rdx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P8) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rdx = ((P9) as u64) as u64; rcx = ((rsp.wrapping_add(0x30u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); crate::lift_ops::trap(0x80000003, P10, rcx, rsp); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e1adc8(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x80; const P1: u64 = 0x18; const P2: u64 = 0x70; const P3: u64 = 0x88; const P4: u64 = 0x90; const P5: u64 = 0x38; const P6: u64 = 0x50; const P7: u64 = 0x7001e19a58; const P8: u64 = 0x7001e69598; const P9: u64 = 0x2; const P10: u64 = 0x7001e1a548;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rsi = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rdi = (rcx) as u64; fl_l = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64 as u64; fl_r = (rsi) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 4; continue; } bb = 1; continue; }
            1 => { rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; rax = ((rdi.wrapping_add(P2)) as u64) as u64; fl_l = (crate::simd_ops::ru64(rcx)) as u64 as u64; fl_r = (rax) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } bb = 2; continue; }
            2 => { rdx = (crate::simd_ops::ru64(rdi.wrapping_add(P3))) as u64; r8 = (crate::simd_ops::ru64(rdi.wrapping_add(P4))) as u64; crate::simd_ops::wu64(rcx, (rdx) as u64); fl_l = ((r8 as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; r8 = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; rax = (crate::simd_ops::ru64(rdi.wrapping_add(P5))) as u64; crate::simd_ops::wu64(rax, (rdx) as u64); rax = (crate::simd_ops::ru64(rdi.wrapping_add(P6))) as u64; crate::simd_ops::wu32(rax, ((r8 as u32)) as u32); bb = 3; continue; }
            3 => { rcx = (rdi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P7) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rcx = (crate::simd_ops::ru64(rdi.wrapping_add(P0))) as u64; fl_l = 0; fl_r = ((rax as u8)) as u8 as u64; rax = (rax & !0xffu64) | (((0u8.wrapping_sub(fl_r as u8)) as u8 as u64)); rbx = (if (((fl_l as u8)) < ((fl_r as u8))) { 0xffffffffffffffffu64 } else { 0u64 }) as u64; rbx = (((rbx) & (rdi))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P8)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; rsi = (if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { rbx } else { rsi }) as u64; bb = 4; continue; }
            4 => { rdx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rcx = (rdi) as u64; r8 = ((rdx.wrapping_add(P9)) as u32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P10) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = (rsi) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e1ae60(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001e1aa60; const P1: u64 = 0x7001e696a0; const P2: u64 = 0x7001e659fc; const P3: u64 = 0x7001e1acd0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (rbp) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r12) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rsi = (rdx) as u64; rbx = (rcx) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rdx) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 44; continue; } bb = 1; continue; }
            1 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rbp = (crate::simd_ops::ru64(rbx.wrapping_add(0x40u64))) as u64; rdi = (crate::simd_ops::ru64(rsi.wrapping_add(0x40u64))) as u64; rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x8u64))) as u64; fl_l = (rcx) as u64 as u64; fl_r = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 5; continue; } bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P1)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); r8 = (rax) as u64; fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru64(rax)) as u64; rdx = (0x1u32) as u32 as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rcx = (r8) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P1)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 4; continue; }
            4 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x8u64), (rcx) as u64); rax = (crate::simd_ops::ru64(rcx)) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P1)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru32(rsi.wrapping_add(0x18u64))) as u32 as u64; r12 = (0x18u32) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x18u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x20u64), (rax) as u64); rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x28u64))) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x28u64), (rax) as u64); rdi = (crate::simd_ops::ru64(rsi.wrapping_add(0x30u64))) as u64; fl_l = (crate::simd_ops::lk_and_u64(rbx.wrapping_add(0x30u64), (0x0u64) as u64)) as u64; fl_r = fl_l; bb = 34; continue; }
            6 => { r14 = (crate::simd_ops::ru32(rdi.wrapping_add(0xcu64))) as u32 as u64; fl_l = ((r14 as u32)) as u32 as u64; fl_r = ((r14 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 8; continue; } bb = 7; continue; }
            7 => { fl_l = (crate::simd_ops::ru64(rdi.wrapping_add(0x10u64))) as u64 as u64; fl_r = (0x0u64) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 33; continue; } bb = 8; continue; }
            8 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x30u64))) as u64; rax = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rbp = (crate::simd_ops::ru32(rdi.wrapping_add(0x8u64))) as u32 as u64; fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 17; continue; } bb = 9; continue; }
            9 => { fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(0x8u64))) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 16; continue; } bb = 10; continue; }
            10 => { fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 13; continue; } bb = 11; continue; }
            11 => { fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(0xcu64))) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 13; continue; } bb = 12; continue; }
            12 => { fl_l = (crate::simd_ops::ru64(rcx.wrapping_add(0x10u64))) as u64 as u64; fl_r = (rax) as u64 as u64; rax = (if (((fl_l as u64)) == ((fl_r as u64))) { rcx } else { rax }) as u64; bb = 13; continue; }
            13 => { rcx = (crate::simd_ops::ru64(rcx)) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 9; continue; } bb = 14; continue; }
            14 => { fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 17; continue; } bb = 15; continue; }
            15 => { crate::simd_ops::wu32(rax.wrapping_add(0x8u64), ((rbp as u32)) as u32); bb = 20; continue; }
            16 => { rax = (rcx) as u64; bb = 20; continue; }
            17 => { rcx = (r12) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), (rax) as u64); fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 19; continue; } bb = 18; continue; }
            18 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x30u64))) as u64; crate::simd_ops::lk_and_u32(rax.wrapping_add(0xcu64), (0x0u32) as u32); fl_l = (crate::simd_ops::lk_and_u64(rax.wrapping_add(0x10u64), (0x0u64) as u64)) as u64; fl_r = fl_l; crate::simd_ops::wu64(rax, (rcx) as u64); crate::simd_ops::wu32(rax.wrapping_add(0x8u64), ((rbp as u32)) as u32); bb = 19; continue; }
            19 => { crate::simd_ops::wu64(rbx.wrapping_add(0x30u64), (rax) as u64); bb = 20; continue; }
            20 => { crate::simd_ops::wu32(rax.wrapping_add(0xcu64), ((r14 as u32)) as u32); rax = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x30u64))) as u64; r14 = (crate::simd_ops::ru64(rdi.wrapping_add(0x10u64))) as u64; rbp = (crate::simd_ops::ru32(rdi.wrapping_add(0x8u64))) as u32 as u64; fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 29; continue; } bb = 21; continue; }
            21 => { fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(0x8u64))) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 28; continue; } bb = 22; continue; }
            22 => { fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 25; continue; } bb = 23; continue; }
            23 => { fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(0xcu64))) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 25; continue; } bb = 24; continue; }
            24 => { fl_l = (crate::simd_ops::ru64(rcx.wrapping_add(0x10u64))) as u64 as u64; fl_r = (rax) as u64 as u64; rax = (if (((fl_l as u64)) == ((fl_r as u64))) { rcx } else { rax }) as u64; bb = 25; continue; }
            25 => { rcx = (crate::simd_ops::ru64(rcx)) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 21; continue; } bb = 26; continue; }
            26 => { fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 29; continue; } bb = 27; continue; }
            27 => { crate::simd_ops::wu32(rax.wrapping_add(0x8u64), ((rbp as u32)) as u32); bb = 32; continue; }
            28 => { rax = (rcx) as u64; bb = 32; continue; }
            29 => { rcx = (r12) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), (rax) as u64); fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 31; continue; } bb = 30; continue; }
            30 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x30u64))) as u64; crate::simd_ops::lk_and_u32(rax.wrapping_add(0xcu64), (0x0u32) as u32); fl_l = (crate::simd_ops::lk_and_u64(rax.wrapping_add(0x10u64), (0x0u64) as u64)) as u64; fl_r = fl_l; crate::simd_ops::wu64(rax, (rcx) as u64); crate::simd_ops::wu32(rax.wrapping_add(0x8u64), ((rbp as u32)) as u32); bb = 31; continue; }
            31 => { crate::simd_ops::wu64(rbx.wrapping_add(0x30u64), (rax) as u64); bb = 32; continue; }
            32 => { crate::simd_ops::wu64(rax.wrapping_add(0x10u64), (r14) as u64); bb = 33; continue; }
            33 => { rdi = (crate::simd_ops::ru64(rdi)) as u64; bb = 34; continue; }
            34 => { fl_l = (rdi) as u64 as u64; fl_r = (rdi) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 6; continue; } bb = 35; continue; }
            35 => { rdi = (crate::simd_ops::ru64(rsi.wrapping_add(0x38u64))) as u64; bb = 39; continue; }
            36 => { rbp = (crate::simd_ops::ru32(rdi.wrapping_add(0x8u64))) as u32 as u64; rcx = (r12) as u64; r14 = (crate::simd_ops::ru64(rdi.wrapping_add(0x10u64))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), (rax) as u64); fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 38; continue; } bb = 37; continue; }
            37 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x38u64))) as u64; crate::simd_ops::wu64(rax, (rcx) as u64); crate::simd_ops::wu32(rax.wrapping_add(0x8u64), ((rbp as u32)) as u32); crate::simd_ops::wu64(rax.wrapping_add(0x10u64), (r14) as u64); bb = 38; continue; }
            38 => { crate::simd_ops::wu64(rbx.wrapping_add(0x38u64), (rax) as u64); rdi = (crate::simd_ops::ru64(rdi)) as u64; bb = 39; continue; }
            39 => { fl_l = (rdi) as u64 as u64; fl_r = (rdi) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 36; continue; } bb = 40; continue; }
            40 => { rdi = (crate::simd_ops::ru64(rbx.wrapping_add(0x38u64))) as u64; bb = 42; continue; }
            41 => { r8 = (crate::simd_ops::ru32(rdi.wrapping_add(0x8u64))) as u32 as u64; rdx = (rbx) as u64; rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x10u64))) as u64; rcx = (0x2u32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P1)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rdi = (crate::simd_ops::ru64(rdi)) as u64; bb = 42; continue; }
            42 => { fl_l = (rdi) as u64 as u64; fl_r = (rdi) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 41; continue; } bb = 43; continue; }
            43 => { rax = (crate::simd_ops::ru32(rsi.wrapping_add(0x14u64))) as u32 as u64; rcx = (rbx) as u64; rdx = (crate::simd_ops::ru32(rbx.wrapping_add(0x10u64))) as u32 as u64; rax = ((((rax as u32)) & (0x17u32))) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x14u64), ((rax as u32)) as u32); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 44; continue; }
            44 => { rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rax = (rbx) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r12 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e1b090(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    crate::simd_ops::wu32(rdx, ((r8 as u32)) as u32);
    rax = (rdx) as u64;
    crate::simd_ops::wu64(rdx.wrapping_add(P0), (rcx) as u64);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e1b0a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64));
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e1b0b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = (rax & !0xffu64) | (((0u8) as u8 as u64));
    fl_l = 0; fl_r = 0;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e1b0c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x14;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = (0u32) as u32 as u64;
    fl_l = 0; fl_r = 0;
    fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; fl_r = (0x1u32) as u32 as u64;
    rax = (rax & !0xffu64) | (((if (((fl_l as u32)) == ((fl_r as u32))) { 1u8 } else { 0u8 }) as u8 as u64));
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e1b0d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x14;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64;
    rax = (rax & !0xffu64) | ((((((rax as u8)) & (0x6u8))) as u8 as u64));
    fl_l = 0; fl_r = ((rax as u8)) as u8 as u64;
    rax = (rax & !0xffu64) | (((0u8.wrapping_sub(fl_r as u8)) as u8 as u64));
    rax = (if (((fl_l as u8)) < ((fl_r as u8))) { 0xffffffffu32 } else { 0u32 }) as u32 as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e1b0e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = (0x1u32) as u32 as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e1b0f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 0>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64;
    rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64;
    crate::simd_ops::wu64(rax, (r8) as u64);
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    crate::simd_ops::wu64(rcx, (rax) as u64);
    rax = (0x3u32) as u32 as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e1b110(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001e4aae0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rbp) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rbp = (rcx) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x88u64))) as u64; rsi = (r9) as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; crate::simd_ops::lk_and_u64(rsp.wrapping_add(0x60u64), (0x0u64) as u64); crate::simd_ops::wu64(rbx, (r8) as u64); crate::simd_ops::wu64(rdi, (rcx) as u64); rdx = (crate::simd_ops::ru64(rbx)) as u64; fl_l = (rdx) as u64 as u64; fl_r = (r9) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 6; continue; } bb = 1; continue; }
            1 => { r14 = (0x1u32) as u32 as u64; bb = 2; continue; }
            2 => { fl_l = (rcx) as u64 as u64; fl_r = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 9; continue; } bb = 3; continue; }
            3 => { rax = ((rbp.wrapping_add(0x10u64)) as u64) as u64; r8 = (rsi) as u64; r8 = ((r8).wrapping_sub(rdx)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rax) as u64); r9 = ((rsp.wrapping_add(0x60u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); fl_l = ((rax as u32)) as u32 as u64; fl_r = (0xfffffffeu32) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 9; continue; } bb = 4; continue; }
            4 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = (0xffffffffu32) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 8; continue; } bb = 5; continue; }
            5 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; rax = (if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { (r14 as u32) } else { (rax as u32) }) as u32 as u64; rax = (rax as u32 as i32 as i64) as u64; crate::simd_ops::lk_add_u64(rbx, (rax) as u64); crate::simd_ops::lk_add_u64(rdi, (0x2u64) as u64); rdx = (crate::simd_ops::ru64(rbx)) as u64; rcx = (crate::simd_ops::ru64(rdi)) as u64; fl_l = (rdx) as u64 as u64; fl_r = (rsi) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 6; continue; }
            6 => { rax = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; bb = 7; continue; }
            7 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rsi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            8 => { rax = (0x2u32) as u32 as u64; bb = 7; continue; }
            9 => { rax = ((r14 as u32)) as u32 as u64; bb = 7; continue; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e1b1c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001e696a0;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 15>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rbp) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r12) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r13) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r15) as u64); rsp = ((rsp).wrapping_sub(0x50u64)) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0xa0u64))) as u64; rsi = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; r10 = (crate::simd_ops::ru64(rsp.wrapping_add(0xb8u64))) as u64; r15 = (r9) as u64; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xa8u64))) as u64; rbp = (r8) as u64; r14 = (rdx) as u64; r13 = (rcx) as u64; crate::simd_ops::wu64(rdi, (r8) as u64); crate::simd_ops::wu64(r10, (rax) as u64); rbx = (crate::simd_ops::ru64(rdi)) as u64; fl_l = (rbx) as u64 as u64; fl_r = (r9) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 34; continue; } bb = 1; continue; }
            1 => { r9 = ((rsi.wrapping_add(0x1u64)) as u32) as u32 as u64; bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xb0u64))) as u64; fl_l = (crate::simd_ops::ru64(r10)) as u64 as u64; fl_r = (rax) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 34; continue; } bb = 3; continue; }
            3 => { rcx = ((crate::simd_ops::ru8(rbx)) as u32) as u32 as u64; fl_l = (crate::simd_ops::ru16(r14)) as u16 as u64; fl_r = ((r9 as u16)) as u16 as u64; if (((fl_l as u16)) <= ((fl_r as u16))) { bb = 6; continue; } bb = 4; continue; }
            4 => { rax = ((rcx.wrapping_sub(0x80u64)) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x3fu8) as u8 as u64; if (((fl_l as u8)) > ((fl_r as u8))) { bb = 40; continue; } bb = 5; continue; }
            5 => { rcx = (rcx & !0xffu64) | ((((((rcx as u8)) & (0x3fu8))) as u8 as u64)); rax = ((rbx.wrapping_add(0x1u64)) as u64) as u64; crate::simd_ops::wu64(rdi, (rax) as u64); rax = (crate::simd_ops::ru64(r10)) as u64; rcx = (((rcx as u8)) as u32) as u32 as u64; rcx = (rcx & !0xffffu64) | ((((((rcx as u16)) | (crate::simd_ops::ru16(r14)))) as u16 as u64)); crate::simd_ops::wu16(rax, ((rcx as u16)) as u16); fl_l = (crate::simd_ops::lk_add_u64(r10, (0x2u64) as u64)) as u64; fl_r = fl_l; crate::simd_ops::wu16(r14, ((r9 as u16)) as u16); bb = 33; continue; }
            6 => { fl_l = ((rcx as u8)) as u8 as u64; fl_r = (0x80u8) as u8 as u64; if (((fl_l as u8)) >= ((fl_r as u8))) { bb = 8; continue; } bb = 7; continue; }
            7 => { rdx = ((rcx as u32)) as u32 as u64; r8 = ((rsi as u32)) as u32 as u64; bb = 16; continue; }
            8 => { fl_l = ((rcx as u8)) as u8 as u64; fl_r = (0xc0u8) as u8 as u64; if (((fl_l as u8)) < ((fl_r as u8))) { bb = 39; continue; } bb = 9; continue; }
            9 => { rdx = ((rcx as u32)) as u32 as u64; fl_l = ((rcx as u8)) as u8 as u64; fl_r = (0xe0u8) as u8 as u64; if (((fl_l as u8)) >= ((fl_r as u8))) { bb = 11; continue; } bb = 10; continue; }
            10 => { fl_l = ((((rdx as u32)) & (0x1fu32))) as u32 as u64; fl_r = fl_l; rdx = ((fl_l as u32)) as u32 as u64; r8 = ((r9 as u32)) as u32 as u64; bb = 16; continue; }
            11 => { fl_l = ((rcx as u8)) as u8 as u64; fl_r = (0xf0u8) as u8 as u64; if (((fl_l as u8)) >= ((fl_r as u8))) { bb = 13; continue; } bb = 12; continue; }
            12 => { fl_l = ((((rdx as u32)) & (0xfu32))) as u32 as u64; fl_r = fl_l; rdx = ((fl_l as u32)) as u32 as u64; r8 = (0x2u32) as u32 as u64; bb = 16; continue; }
            13 => { fl_l = ((rcx as u8)) as u8 as u64; fl_r = (0xf8u8) as u8 as u64; if (((fl_l as u8)) >= ((fl_r as u8))) { bb = 15; continue; } bb = 14; continue; }
            14 => { fl_l = ((((rdx as u32)) & (0x7u32))) as u32 as u64; fl_r = fl_l; rdx = ((fl_l as u32)) as u32 as u64; r8 = (0x3u32) as u32 as u64; bb = 16; continue; }
            15 => { rdx = ((((rdx as u32)) & (0x3u32))) as u32 as u64; fl_l = ((rcx as u8)) as u8 as u64; fl_r = (0xfcu8) as u8 as u64; r8 = (if (((fl_l as u8)) < ((fl_r as u8))) { 0xffffffffu32 } else { 0u32 }) as u32 as u64; fl_l = (((r8 as u32)).wrapping_add(0x5u32)) as u32 as u64; fl_r = fl_l; r8 = ((fl_l as u32)) as u32 as u64; bb = 16; continue; }
            16 => { fl_l = ((r8 as u32)) as u32 as u64; fl_r = (0x3u32) as u32 as u64; r11 = ((rsi as u32)) as u32 as u64; r12 = ((r8 as u32)) as u32 as u64; rbp = (rbx) as u64; r11 = (r11 & !0xffu64) | (((if (((fl_l as u32)) >= ((fl_r as u32))) { 1u8 } else { 0u8 }) as u8 as u64)); fl_l = ((r8 as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 18; continue; } bb = 17; continue; }
            17 => { r9 = ((rbx.wrapping_add(0x1u64)) as u64) as u64; crate::simd_ops::wu64(rdi, (r9) as u64); bb = 22; continue; }
            18 => { rax = ((r8 as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_sub((r11 as u32))) as u32 as u64; rax = (((rax as u32)).wrapping_add((r9 as u32))) as u32 as u64; rcx = ((((rax as u32)) as i32 as i64) as u64) as u64; rax = (r15) as u64; rax = ((rax).wrapping_sub(rbx)) as u64; fl_l = (rax) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) as i64) < (((fl_r as u64)) as i64)) { bb = 34; continue; } bb = 19; continue; }
            19 => { r9 = ((rbx.wrapping_add(0x1u64)) as u64) as u64; crate::simd_ops::wu64(rdi, (r9) as u64); fl_l = ((r11 as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 22; continue; } bb = 20; continue; }
            20 => { rax = (rax & !0xffu64) | (((crate::simd_ops::ru8(r9)) as u8 as u64)); rax = (rax & !0xffu64) | (((((rax as u8)).wrapping_sub(0x80u8)) as u8 as u64)); fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x3fu8) as u8 as u64; if (((fl_l as u8)) > ((fl_r as u8))) { bb = 40; continue; } bb = 21; continue; }
            21 => { rcx = ((crate::simd_ops::ru8(r9)) as u32) as u32 as u64; rax = ((rdx as u32)) as u32 as u64; rcx = ((((rcx as u32)) & (0x3fu32))) as u32 as u64; rax = (((rax as u32)).wrapping_shl(6u32)) as u32 as u64; r8 = (((r8 as u32)).wrapping_sub(1)) as u32 as u64; rdx = ((rcx as u32)) as u32 as u64; rdx = ((((rdx as u32)) | ((rax as u32)))) as u32 as u64; r9 = ((r9).wrapping_add(1)) as u64; crate::simd_ops::wu64(rdi, (r9) as u64); fl_l = ((r11 as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 20; continue; } bb = 22; continue; }
            22 => { r8 = ((rdx as u32)) as u32 as u64; r8 = (((r8 as u32)).wrapping_shl(6u32)) as u32 as u64; fl_l = ((r11 as u32)) as u32 as u64; fl_r = ((r11 as u32)) as u32 as u64; r8 = (if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { (rdx as u32) } else { (r8 as u32) }) as u32 as u64; rdx = (0x10ffffu32) as u32 as u64; fl_l = (crate::simd_ops::ru32(r13.wrapping_add(0x10u64))) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; rax = ((rdx as u32)) as u32 as u64; rax = (if (((fl_l as u32)) < ((fl_r as u32))) { crate::simd_ops::ru32(r13.wrapping_add(0x10u64)) } else { (rax as u32) }) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 40; continue; } bb = 23; continue; }
            23 => { fl_l = ((r8 as u32)) as u32 as u64; fl_r = (0xffffu32) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 25; continue; } bb = 24; continue; }
            24 => { rcx = ((r8 as u32)) as u32 as u64; rax = (0xd800u32) as u32 as u64; rcx = (((rcx as u32)).wrapping_shr(10u32)) as u32 as u64; r9 = (0x1u32) as u32 as u64; rcx = (rcx & !0xffffu64) | (((((rcx as u16)).wrapping_sub(0x40u16)) as u16 as u64)); rcx = (rcx & !0xffffu64) | ((((((rcx as u16)) | ((rax as u16)))) as u16 as u64)); rax = (crate::simd_ops::ru64(r10)) as u64; crate::simd_ops::wu16(rax, ((rcx as u16)) as u16); rax = (0x3ffu32) as u32 as u64; crate::simd_ops::lk_add_u64(r10, (0x2u64) as u64); r8 = (r8 & !0xffffu64) | ((((((r8 as u16)) & ((rax as u16)))) as u16 as u64)); rax = (0xdc00u32) as u32 as u64; fl_l = ((((r8 as u16)) | ((rax as u16)))) as u16 as u64; fl_r = fl_l; r8 = (r8 & !0xffffu64) | ((((fl_l as u16)) as u16 as u64)); crate::simd_ops::wu16(r14, ((r8 as u16)) as u16); bb = 33; continue; }
            25 => { fl_l = ((r12 as u32)) as u32 as u64; fl_r = (0x3u32) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 29; continue; } bb = 26; continue; }
            26 => { fl_l = (r9) as u64 as u64; fl_r = (r15) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 36; continue; } bb = 27; continue; }
            27 => { rcx = ((crate::simd_ops::ru8(r9)) as u32) as u32 as u64; rax = ((r9.wrapping_add(0x1u64)) as u64) as u64; crate::simd_ops::wu64(rdi, (rax) as u64); rax = ((rcx.wrapping_sub(0x80u64)) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x3fu8) as u8 as u64; if (((fl_l as u8)) > ((fl_r as u8))) { bb = 40; continue; } bb = 28; continue; }
            28 => { rax = ((rcx as u32)) as u32 as u64; rax = ((((rax as u32)) & (0x3fu32))) as u32 as u64; fl_l = ((((r8 as u32)) | ((rax as u32)))) as u32 as u64; fl_r = fl_l; r8 = ((fl_l as u32)) as u32 as u64; bb = 29; continue; }
            29 => { r9 = (0x1u32) as u32 as u64; fl_l = (crate::simd_ops::ru16(r14)) as u16 as u64; fl_r = ((rsi as u16)) as u16 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { bb = 32; continue; } bb = 30; continue; }
            30 => { crate::simd_ops::wu16(r14, ((r9 as u16)) as u16); fl_l = (crate::simd_ops::ru8(r13.wrapping_add(0x14u64))) as u8 as u64; fl_r = (0x4u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 32; continue; } bb = 31; continue; }
            31 => { fl_l = ((r8 as u32)) as u32 as u64; fl_r = (0xfeffu32) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 37; continue; } bb = 32; continue; }
            32 => { rax = (crate::simd_ops::ru64(r10)) as u64; crate::simd_ops::wu16(rax, ((r8 as u16)) as u16); fl_l = (crate::simd_ops::lk_add_u64(r10, (0x2u64) as u64)) as u64; fl_r = fl_l; bb = 33; continue; }
            33 => { rbx = (crate::simd_ops::ru64(rdi)) as u64; fl_l = (rbx) as u64 as u64; fl_r = (r15) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 34; continue; }
            34 => { fl_l = (rbp) as u64 as u64; fl_r = (rbx) as u64 as u64; rsi = (rsi & !0xffu64) | (((if (((fl_l as u64)) == ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); rax = ((rsi as u32)) as u32 as u64; bb = 35; continue; }
            35 => { r11 = ((rsp.wrapping_add(0x50u64)) as u64) as u64; rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(r11.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x40u64))) as u64; rsp = (r11) as u64; r15 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r13 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r12 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            36 => { crate::simd_ops::wu64(rdi, (rbx) as u64); bb = 34; continue; }
            37 => { rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0xb0u64))) as u64; r9 = (r15) as u64; rax = (crate::simd_ops::ru64(r13)) as u64; rdx = (r14) as u64; r8 = (crate::simd_ops::ru64(rdi)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), (r10) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (rcx) as u64); rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0xa8u64))) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (rcx) as u64); rcx = (r13) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rdi) as u64); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P0)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x1u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 35; continue; } bb = 38; continue; }
            38 => { crate::simd_ops::wu16(r14, ((rsi as u16)) as u16); crate::simd_ops::wu64(rdi, (rbx) as u64); bb = 35; continue; }
            39 => { rax = ((rbx.wrapping_add(0x1u64)) as u64) as u64; crate::simd_ops::wu64(rdi, (rax) as u64); bb = 40; continue; }
            40 => { rax = (0x2u32) as u32 as u64; bb = 35; continue; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e1b480(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001e696a0;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 15>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rbp) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r12) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r13) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r15) as u64); rsp = ((rsp).wrapping_sub(0x50u64)) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0xa0u64))) as u64; rdi = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; r11 = (crate::simd_ops::ru64(rsp.wrapping_add(0xb8u64))) as u64; r15 = (r9) as u64; r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0xa8u64))) as u64; rbp = (r8) as u64; rsi = (rdx) as u64; r14 = (rcx) as u64; crate::simd_ops::wu64(rbx, (r8) as u64); crate::simd_ops::wu64(r11, (r12) as u64); r10 = (crate::simd_ops::ru64(rbx)) as u64; fl_l = (r10) as u64 as u64; fl_r = (r9) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 22; continue; } bb = 1; continue; }
            1 => { r13 = (crate::simd_ops::ru64(rsp.wrapping_add(0xb0u64))) as u64; rax = (r12) as u64; bb = 2; continue; }
            2 => { fl_l = (rax) as u64 as u64; fl_r = (r13) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 22; continue; } bb = 3; continue; }
            3 => { rax = ((crate::simd_ops::ru8(r10)) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x80u8) as u8 as u64; if (((fl_l as u8)) >= ((fl_r as u8))) { bb = 5; continue; } bb = 4; continue; }
            4 => { r8 = ((rax as u32)) as u32 as u64; rax = ((r10.wrapping_add(0x1u64)) as u64) as u64; crate::simd_ops::wu64(rbx, (rax) as u64); bb = 17; continue; }
            5 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = (0xc0u8) as u8 as u64; if (((fl_l as u8)) < ((fl_r as u8))) { bb = 26; continue; } bb = 6; continue; }
            6 => { r8 = ((rax as u32)) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0xe0u8) as u8 as u64; if (((fl_l as u8)) >= ((fl_r as u8))) { bb = 8; continue; } bb = 7; continue; }
            7 => { fl_l = ((((r8 as u32)) & (0x1fu32))) as u32 as u64; fl_r = fl_l; r8 = ((fl_l as u32)) as u32 as u64; r9 = (0x1u32) as u32 as u64; bb = 13; continue; }
            8 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = (0xf0u8) as u8 as u64; if (((fl_l as u8)) >= ((fl_r as u8))) { bb = 10; continue; } bb = 9; continue; }
            9 => { fl_l = ((((r8 as u32)) & (0xfu32))) as u32 as u64; fl_r = fl_l; r8 = ((fl_l as u32)) as u32 as u64; r9 = (0x2u32) as u32 as u64; bb = 13; continue; }
            10 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = (0xf8u8) as u8 as u64; if (((fl_l as u8)) >= ((fl_r as u8))) { bb = 12; continue; } bb = 11; continue; }
            11 => { fl_l = ((((r8 as u32)) & (0x7u32))) as u32 as u64; fl_r = fl_l; r8 = ((fl_l as u32)) as u32 as u64; r9 = (0x3u32) as u32 as u64; bb = 13; continue; }
            12 => { r8 = ((((r8 as u32)) & (0x3u32))) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0xfcu8) as u8 as u64; r9 = (if (((fl_l as u8)) < ((fl_r as u8))) { 0xffffffffu32 } else { 0u32 }) as u32 as u64; fl_l = (((r9 as u32)).wrapping_add(0x5u32)) as u32 as u64; fl_r = fl_l; r9 = ((fl_l as u32)) as u32 as u64; bb = 13; continue; }
            13 => { rdx = (r15) as u64; rcx = ((r9.wrapping_add(0x1u64)) as u32) as u32 as u64; rdx = ((rdx).wrapping_sub(r10)) as u64; fl_l = (rdx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) as i64) < (((fl_r as u64)) as i64)) { bb = 22; continue; } bb = 14; continue; }
            14 => { fl_l = ((r10).wrapping_add(1)) as u64 as u64; r10 = ((fl_l as u64)) as u64; crate::simd_ops::wu64(rbx, (r10) as u64); bb = 15; continue; }
            15 => { rax = (rax & !0xffu64) | (((crate::simd_ops::ru8(r10)) as u8 as u64)); rax = (rax & !0xffu64) | (((((rax as u8)).wrapping_sub(0x80u8)) as u8 as u64)); fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x3fu8) as u8 as u64; if (((fl_l as u8)) > ((fl_r as u8))) { bb = 27; continue; } bb = 16; continue; }
            16 => { rcx = ((crate::simd_ops::ru8(r10)) as u32) as u32 as u64; rax = ((r8 as u32)) as u32 as u64; rcx = ((((rcx as u32)) & (0x3fu32))) as u32 as u64; rax = (((rax as u32)).wrapping_shl(6u32)) as u32 as u64; r9 = (((r9 as u32)).wrapping_sub(1)) as u32 as u64; r8 = ((rcx as u32)) as u32 as u64; r8 = ((((r8 as u32)) | ((rax as u32)))) as u32 as u64; r10 = ((r10).wrapping_add(1)) as u64; crate::simd_ops::wu64(rbx, (r10) as u64); fl_l = ((r9 as u32)) as u32 as u64; fl_r = ((r9 as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) > 0) { bb = 15; continue; } bb = 17; continue; }
            17 => { fl_l = (crate::simd_ops::ru8(rsi)) as u8 as u64; fl_r = ((rdi as u8)) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 20; continue; } bb = 18; continue; }
            18 => { crate::simd_ops::wu8(rsi, (0x1u8) as u8); fl_l = (crate::simd_ops::ru8(r14.wrapping_add(0x14u64))) as u8 as u64; fl_r = (0x4u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 20; continue; } bb = 19; continue; }
            19 => { fl_l = ((r8 as u32)) as u32 as u64; fl_r = (0xfeffu32) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 24; continue; } bb = 20; continue; }
            20 => { fl_l = (crate::simd_ops::ru32(r14.wrapping_add(0x10u64))) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 27; continue; } bb = 21; continue; }
            21 => { rax = (crate::simd_ops::ru64(r11)) as u64; crate::simd_ops::wu32(rax, ((r8 as u32)) as u32); crate::simd_ops::lk_add_u64(r11, (0x4u64) as u64); r10 = (crate::simd_ops::ru64(rbx)) as u64; rax = (crate::simd_ops::ru64(r11)) as u64; fl_l = (r10) as u64 as u64; fl_r = (r15) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 22; continue; }
            22 => { fl_l = (rbp) as u64 as u64; fl_r = (r10) as u64 as u64; rdi = (rdi & !0xffu64) | (((if (((fl_l as u64)) == ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); rax = ((rdi as u32)) as u32 as u64; bb = 23; continue; }
            23 => { r11 = ((rsp.wrapping_add(0x50u64)) as u64) as u64; rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(r11.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x40u64))) as u64; rsp = (r11) as u64; r15 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r13 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r12 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            24 => { rax = (crate::simd_ops::ru64(r14)) as u64; r9 = (r15) as u64; r8 = (crate::simd_ops::ru64(rbx)) as u64; rdx = (rsi) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), (r11) as u64); rcx = (r14) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (r13) as u64); rax = (crate::simd_ops::ru64(rax.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (r12) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rbx) as u64); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P0)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x1u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 23; continue; } bb = 25; continue; }
            25 => { crate::simd_ops::wu8(rsi, ((rdi as u8)) as u8); crate::simd_ops::wu64(rbx, (rbp) as u64); bb = 23; continue; }
            26 => { rax = ((r10.wrapping_add(0x1u64)) as u64) as u64; crate::simd_ops::wu64(rbx, (rax) as u64); bb = 27; continue; }
            27 => { rax = (0x2u32) as u32 as u64; bb = 23; continue; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e1b640(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x7001e433f0;
    let (mut rax, mut r10, mut r11) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = (r9) as u64;
    r10 = (r8) as u64;
    r11 = (rdx) as u64;
    r9 = ((rcx.wrapping_add(P0)) as u64) as u64;
    r8 = (rax) as u64;
    rdx = (r10) as u64;
    rcx = (r11) as u64;
    crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e1b660(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x7001e433c0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = (((rdx as u16)) as u32) as u32 as u64;
    rdx = ((rcx.wrapping_add(P0)) as u64) as u64;
    rcx = (((r8 as u16)) as u32) as u32 as u64;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    fl_l = ((rbx as u16)) as u16 as u64; fl_r = ((rax as u16)) as u16 as u64;
    rax = (rax & !0xffu64) | (((if ((((fl_l as u16)) & ((fl_r as u16))) != 0) { 1u8 } else { 0u8 }) as u8 as u64));
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = core::ptr::read_unaligned(rsp as *const u64);
    rsp = rsp.wrapping_add(8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e1b690(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 0>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    r9 = ((r9).wrapping_sub(r8)) as u64;
    rax = (0x7fffffffu32) as u32 as u64;
    fl_l = (r9) as u64 as u64; fl_r = (rax) as u64 as u64;
    r9 = (if ((((fl_l as u64)) as i64) > (((fl_r as u64)) as i64)) { rax } else { r9 }) as u64;
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64;
    fl_l = (r9) as u64 as u64; fl_r = (rax) as u64 as u64;
    rax = (if (((fl_l as u64)) < ((fl_r as u64))) { (r9 as u32) } else { (rax as u32) }) as u32 as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e1b6b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0x18; const P3: u64 = 0x20; const P4: u64 = 0x7001e4aae0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9, 32>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    // stackargs: A=32 max=5
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = (rsp) as u64; crate::simd_ops::wu64(rax.wrapping_add(P0), (rbx) as u64); crate::simd_ops::wu64(rax.wrapping_add(P1), (rbp) as u64); crate::simd_ops::wu64(rax.wrapping_add(P2), (rsi) as u64); crate::simd_ops::wu64(rax.wrapping_add(P3), (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r12) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r15) as u64); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rsi = (r9) as u64; rbx = (r8) as u64; r14 = (rdx) as u64; r15 = (rcx) as u64; rbp = (r8) as u64; fl_l = (rdi) as u64 as u64; fl_r = (rdi) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 1; continue; }
            1 => { r12 = (0x1u32) as u32 as u64; bb = 2; continue; }
            2 => { fl_l = (rbx) as u64 as u64; fl_r = (rsi) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 5; continue; } bb = 3; continue; }
            3 => { rax = ((r15.wrapping_add(P1)) as u64) as u64; r8 = (rsi) as u64; r8 = ((r8).wrapping_sub(rbx)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rax) as u64); r9 = (r14) as u64; rcx = ((rsp.wrapping_add(0x70u64)) as u64) as u64; rdx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = (if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { (r12 as u32) } else { (rax as u32) }) as u32 as u64; rbx = ((rbx).wrapping_add(rax)) as u64; fl_l = (rdi) as u64 as u64; fl_r = (r12) as u64 as u64; rdi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 5; continue; }
            5 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rbx = ((rbx).wrapping_sub(rbp)) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rax = (0x7fffffffu32) as u32 as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; fl_l = (rbx) as u64 as u64; fl_r = (rax) as u64 as u64; rbx = (if ((((fl_l as u64)) as i64) > (((fl_r as u64)) as i64)) { (rax as u32) } else { (rbx as u32) }) as u32 as u64; rax = ((rbx as u32)) as u32 as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; r15 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r12 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e1b780(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x14;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64;
    fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x4u8) as u8 as u64;
    if !(((((fl_l as u8)) & ((fl_r as u8))) == 0)) { rax = (0x9u32) as u32 as u64; return rax; }
    rax = ((((rax as u32)) & (0x2u32))) as u32 as u64;
    rax = ((((rax as u32)) | (0xcu32))) as u32 as u64;
    fl_l = (((rax as u32)).wrapping_shr(1u32)) as u32 as u64;
    rax = ((fl_l as u32)) as u32 as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e1b7a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x14;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64;
    rax = (rax & !0xffu64) | ((((((rax as u8)) & (0x6u8))) as u8 as u64));
    fl_l = 0; fl_r = ((rax as u8)) as u8 as u64;
    rax = (rax & !0xffu64) | (((0u8.wrapping_sub(fl_r as u8)) as u8 as u64));
    rax = (if (((fl_l as u8)) < ((fl_r as u8))) { 0xffffffffu32 } else { 0u32 }) as u32 as u64;
    rax = ((((rax as u32)) & (0x3u32))) as u32 as u64;
    fl_l = (((rax as u32)).wrapping_add(0x6u32)) as u32 as u64; fl_r = fl_l;
    rax = ((fl_l as u32)) as u32 as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e1b7b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = (rax & !0xffu64) | ((((rdx as u8)) as u8 as u64));
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e1b7c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001e66450;
    let (mut rax, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64;
    rbx = (r8) as u64;
    r8 = ((r8).wrapping_sub(rdx)) as u64;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    rax = (rbx) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = core::ptr::read_unaligned(rsp as *const u64);
    rsp = rsp.wrapping_add(8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e1b7e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001e97380; const P1: u64 = 0x30; const P2: u64 = 0x7001e4d9a0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rbp) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rax = (crate::simd_ops::ru64(P0)) as u64; rax = (((rax) ^ (rsp))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (rax) as u64); rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; rdi = (r8) as u64; r14 = (((r9 as u8)) as u32) as u32 as u64; rbx = (rdx) as u64; fl_l = (rdx) as u64 as u64; fl_r = (r8) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 3; continue; } bb = 1; continue; }
            1 => { rbp = ((rcx.wrapping_add(P1)) as u64) as u64; bb = 2; continue; }
            2 => { rdx = ((crate::simd_ops::ru16(rbx)) as u32) as u32 as u64; r8 = ((rsp.wrapping_add(0x20u64)) as u64) as u64; crate::simd_ops::lk_and_u64(rsp.wrapping_add(0x20u64), (0x0u64) as u64); rcx = ((rsp.wrapping_add(0x28u64)) as u64) as u64; r9 = (rbp) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rdx = ((crate::simd_ops::ru8(rsp.wrapping_add(0x28u64))) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x1u32) as u32 as u64; rdx = (if (((fl_l as u32)) != ((fl_r as u32))) { (r14 as u32) } else { (rdx as u32) }) as u32 as u64; rbx = ((rbx).wrapping_add(0x2u64)) as u64; crate::simd_ops::wu8(rsi, ((rdx as u8)) as u8); rsi = ((rsi).wrapping_add(1)) as u64; fl_l = (rbx) as u64 as u64; fl_r = (rdi) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 3; continue; }
            3 => { rax = (rbx) as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rcx = (((rcx) ^ (rsp))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rsi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e1b880(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001e97380; const P1: u64 = 0x7001e4d9a0; const P2: u64 = 0x7001e66450;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rbp) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r12) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r15) as u64); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rax = (crate::simd_ops::ru64(P0)) as u64; rax = (((rax) ^ (rsp))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (rax) as u64); rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x90u64))) as u64; rax = (rcx) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0xa8u64))) as u64; r15 = (rdx) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rcx) as u64); rbp = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x98u64))) as u64; r12 = (r9) as u64; crate::simd_ops::wu64(rdi, (r8) as u64); crate::simd_ops::wu64(rsi, (rcx) as u64); rdx = (crate::simd_ops::ru64(rdi)) as u64; fl_l = (rdx) as u64 as u64; fl_r = (r9) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 9; continue; } bb = 1; continue; }
            1 => { fl_l = (rcx) as u64 as u64; fl_r = (crate::simd_ops::ru64(rsp.wrapping_add(0xa0u64))) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 9; continue; } bb = 2; continue; }
            2 => { rdx = ((crate::simd_ops::ru16(rdx)) as u32) as u32 as u64; r9 = ((rax.wrapping_add(0x10u64)) as u64) as u64; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xa0u64))) as u64; r8 = (r15) as u64; rax = ((rax).wrapping_sub(rcx)) as u64; fl_l = (rax) as u64 as u64; fl_r = (0x5u64) as u64 as u64; if ((((fl_l as u64)) as i64) < (((fl_r as u64)) as i64)) { bb = 5; continue; } bb = 3; continue; }
            3 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 12; continue; } bb = 4; continue; }
            4 => { crate::simd_ops::lk_add_u64(rdi, (0x2u64) as u64); rax = (rax as u32 as i32 as i64) as u64; fl_l = (crate::simd_ops::lk_add_u64(rsi, (rax) as u64)) as u64; fl_r = fl_l; bb = 8; continue; }
            5 => { rbx = (crate::simd_ops::ru64(r15)) as u64; rcx = ((rsp.wrapping_add(0x28u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 12; continue; } bb = 6; continue; }
            6 => { r14 = ((((rax as u32)) as i32 as i64) as u64) as u64; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xa0u64))) as u64; rax = ((rax).wrapping_sub(crate::simd_ops::ru64(rsi))) as u64; fl_l = (rax) as u64 as u64; fl_r = (r14) as u64 as u64; if ((((fl_l as u64)) as i64) < (((fl_r as u64)) as i64)) { bb = 11; continue; } bb = 7; continue; }
            7 => { rcx = (crate::simd_ops::ru64(rsi)) as u64; rdx = ((rsp.wrapping_add(0x28u64)) as u64) as u64; r8 = (r14) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); crate::simd_ops::lk_add_u64(rdi, (0x2u64) as u64); fl_l = (crate::simd_ops::lk_add_u64(rsi, (r14) as u64)) as u64; fl_r = fl_l; bb = 8; continue; }
            8 => { rdx = (crate::simd_ops::ru64(rdi)) as u64; rcx = (crate::simd_ops::ru64(rsi)) as u64; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; fl_l = (rdx) as u64 as u64; fl_r = (r12) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } bb = 9; continue; }
            9 => { fl_l = (crate::simd_ops::ru64(rdi)) as u64 as u64; fl_r = (r12) as u64 as u64; rbp = (rbp & !0xffu64) | (((if (((fl_l as u64)) != ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); rax = ((rbp as u32)) as u32 as u64; bb = 10; continue; }
            10 => { rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; fl_l = (((rcx) ^ (rsp))) as u64 as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; r11 = ((rsp.wrapping_add(0x40u64)) as u64) as u64; rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x40u64))) as u64; rbp = (crate::simd_ops::ru64(r11.wrapping_add(0x48u64))) as u64; rsp = (r11) as u64; r15 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r12 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rsi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            11 => { crate::simd_ops::wu64(r15, (rbx) as u64); bb = 9; continue; }
            12 => { rax = (0x2u32) as u32 as u64; bb = 10; continue; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e1b9b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 8>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = (rsp) as u64; crate::simd_ops::wu64(rax.wrapping_add(0x20u64), (r9) as u64); crate::simd_ops::wu64(rax.wrapping_add(0x18u64), (r8) as u64); crate::simd_ops::wu64(rax.wrapping_add(0x10u64), (rdx) as u64); crate::simd_ops::wu64(rax.wrapping_add(0x8u64), (rcx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r12) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r13) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r15) as u64); r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rbx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; r10 = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; crate::simd_ops::wu64(r14, (r8) as u64); crate::simd_ops::wu64(r10, (rax) as u64); r11 = (crate::simd_ops::ru64(r14)) as u64; fl_l = (r11) as u64 as u64; fl_r = (r9) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 28; continue; } bb = 1; continue; }
            1 => { rsi = (r11) as u64; r15 = ((rbx.wrapping_add(0x1u64)) as u32) as u32 as u64; bb = 2; continue; }
            2 => { r12 = (crate::simd_ops::ru64(r10)) as u64; r9 = (0x3ffu32) as u32 as u64; r11 = (rsi) as u64; fl_l = (r12) as u64 as u64; fl_r = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 28; continue; } bb = 3; continue; }
            3 => { r13 = (r13 & !0xffu64) | ((((rbx as u8)) as u8 as u64)); rcx = ((crate::simd_ops::ru16(rsi)) as u32) as u32 as u64; fl_l = ((r15 as u16)) as u16 as u64; fl_r = (crate::simd_ops::ru16(rdx)) as u16 as u64; if (((fl_l as u16)) >= ((fl_r as u16))) { bb = 6; continue; } bb = 4; continue; }
            4 => { rax = (0x2400u32) as u32 as u64; rax = (((rax as u32)).wrapping_add((rcx as u32))) as u32 as u64; fl_l = ((rax as u16)) as u16 as u64; fl_r = ((r9 as u16)) as u16 as u64; if (((fl_l as u16)) > ((fl_r as u16))) { bb = 30; continue; } bb = 5; continue; }
            5 => { rdx = ((crate::simd_ops::ru16(rdx)) as u32) as u32 as u64; rax = ((rcx.wrapping_sub(0xdc00u64)) as u32) as u32 as u64; rdx = (((rdx as u32)).wrapping_shl(10u32)) as u32 as u64; fl_l = ((((rdx as u32)) | ((rax as u32)))) as u32 as u64; fl_r = fl_l; rdx = ((fl_l as u32)) as u32 as u64; bb = 8; continue; }
            6 => { rax = (0x2800u32) as u32 as u64; rdx = ((rcx as u32)) as u32 as u64; rcx = (rcx & !0xffffu64) | (((((rcx as u16)).wrapping_add((rax as u16))) as u16 as u64)); fl_l = ((rcx as u16)) as u16 as u64; fl_r = ((r9 as u16)) as u16 as u64; if (((fl_l as u16)) > ((fl_r as u16))) { bb = 8; continue; } bb = 7; continue; }
            7 => { rdx = (((rdx as u32)).wrapping_shl(10u32)) as u32 as u64; r13 = (r13 & !0xffu64) | ((((r15 as u8)) as u8 as u64)); fl_l = ((rdx as u32)) as u32 as u64; fl_r = (0x35f0000u32) as u32 as u64; rdx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 8; continue; }
            8 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = (0x80u32) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 10; continue; } bb = 9; continue; }
            9 => { r8 = (r8 & !0xffu64) | ((((rdx as u8)) as u8 as u64)); r9 = ((rbx as u32)) as u32 as u64; bb = 15; continue; }
            10 => { r8 = ((rdx as u32)) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = (0x800u32) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 12; continue; } bb = 11; continue; }
            11 => { r8 = (((r8 as u32)).wrapping_shr(6u32)) as u32 as u64; r9 = ((r15 as u32)) as u32 as u64; fl_l = ((((r8 as u8)) | (0xc0u8))) as u8 as u64; fl_r = fl_l; r8 = (r8 & !0xffu64) | ((((fl_l as u8)) as u8 as u64)); bb = 15; continue; }
            12 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = (0x10000u32) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 14; continue; } bb = 13; continue; }
            13 => { r8 = (((r8 as u32)).wrapping_shr(12u32)) as u32 as u64; r9 = (0x2u32) as u32 as u64; fl_l = ((((r8 as u8)) | (0xe0u8))) as u8 as u64; fl_r = fl_l; r8 = (r8 & !0xffu64) | ((((fl_l as u8)) as u8 as u64)); bb = 15; continue; }
            14 => { r8 = (((r8 as u32)).wrapping_shr(18u32)) as u32 as u64; r9 = (0x3u32) as u32 as u64; fl_l = ((((r8 as u8)) | (0xf0u8))) as u8 as u64; fl_r = fl_l; r8 = (r8 & !0xffu64) | ((((fl_l as u8)) as u8 as u64)); bb = 15; continue; }
            15 => { rbp = (((r13 as u8)) as u32) as u32 as u64; rbp = ((((rbp as u32)) ^ ((r15 as u32)))) as u32 as u64; r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; rbp = (((rbp as u32)).wrapping_add((rbp as u32))) as u32 as u64; fl_l = ((r9 as u32)) as u32 as u64; fl_r = (0x3u32) as u32 as u64; rbp = (if (((fl_l as u32)) < ((fl_r as u32))) { (r9 as u32) } else { (rbp as u32) }) as u32 as u64; r15 = ((r15).wrapping_sub(r12)) as u64; rdi = ((rbp.wrapping_add(0x1u64)) as u32) as u32 as u64; rax = ((rdi as u32)) as u32 as u64; fl_l = (r15) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) as i64) < (((fl_r as u64)) as i64)) { bb = 28; continue; } bb = 16; continue; }
            16 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; fl_l = (crate::simd_ops::ru16(rax)) as u16 as u64; fl_r = ((rbx as u16)) as u16 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { bb = 20; continue; } bb = 17; continue; }
            17 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; fl_l = (crate::simd_ops::ru8(rax.wrapping_add(0x14u64))) as u8 as u64; fl_r = (0x2u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 20; continue; } bb = 18; continue; }
            18 => { rcx = ((rdi.wrapping_add(0x3u64)) as u32) as u32 as u64; fl_l = (r15) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) as i64) < (((fl_r as u64)) as i64)) { bb = 28; continue; } bb = 19; continue; }
            19 => { crate::simd_ops::wu8(r12, (0xefu8) as u8); r15 = (0x1u32) as u32 as u64; crate::simd_ops::lk_add_u64(r10, (r15) as u64); rax = (crate::simd_ops::ru64(r10)) as u64; crate::simd_ops::wu8(rax, (0xbbu8) as u8); crate::simd_ops::lk_add_u64(r10, (r15) as u64); rax = (crate::simd_ops::ru64(r10)) as u64; crate::simd_ops::wu8(rax, (0xbfu8) as u8); fl_l = (crate::simd_ops::lk_add_u64(r10, (r15) as u64)) as u64; fl_r = fl_l; bb = 21; continue; }
            20 => { r15 = (0x1u32) as u32 as u64; bb = 21; continue; }
            21 => { crate::simd_ops::lk_add_u64(r14, (0x2u64) as u64); fl_l = ((r13 as u8)) as u8 as u64; fl_r = ((r13 as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 23; continue; } bb = 22; continue; }
            22 => { fl_l = ((r9 as u32)) as u32 as u64; fl_r = (0x3u32) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 24; continue; } bb = 23; continue; }
            23 => { rax = (crate::simd_ops::ru64(r10)) as u64; rdi = ((rbp as u32)) as u32 as u64; crate::simd_ops::wu8(rax, ((r8 as u8)) as u8); fl_l = (crate::simd_ops::lk_add_u64(r10, (r15) as u64)) as u64; fl_r = fl_l; bb = 24; continue; }
            24 => { r8 = (crate::simd_ops::ru64(r10)) as u64; fl_l = ((rdi as u32)) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 27; continue; } bb = 25; continue; }
            25 => { rcx = ((r9.wrapping_add(r9.wrapping_mul(2))) as u32) as u32 as u64; fl_l = (((rcx as u32)).wrapping_add((rcx as u32))) as u32 as u64; fl_r = fl_l; rcx = ((fl_l as u32)) as u32 as u64; bb = 26; continue; }
            26 => { rcx = (((rcx as u32)).wrapping_sub(0x6u32)) as u32 as u64; rax = ((rdx as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_shr(((rcx as u32) & 31))) as u32 as u64; rdi = (((rdi as u32)).wrapping_sub((r15 as u32))) as u32 as u64; rax = (rax & !0xffu64) | ((((((rax as u8)) & (0x3fu8))) as u8 as u64)); rax = (rax & !0xffu64) | ((((((rax as u8)) | (0x80u8))) as u8 as u64)); crate::simd_ops::wu8(r8, ((rax as u8)) as u8); crate::simd_ops::lk_add_u64(r10, (r15) as u64); r8 = (crate::simd_ops::ru64(r10)) as u64; fl_l = ((rdi as u32)) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) > 0) { bb = 26; continue; } bb = 27; continue; }
            27 => { rdx = (((rdx as u32)).wrapping_shr(10u32)) as u32 as u64; rax = ((r15 as u32)) as u32 as u64; fl_l = ((r13 as u8)) as u8 as u64; fl_r = ((r13 as u8)) as u8 as u64; rax = (rax & !0xffffu64) | (((if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { (rdx as u16) } else { (rax as u16) }) as u16 as u64)); rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; crate::simd_ops::wu16(rdx, ((rax as u16)) as u16); r11 = (crate::simd_ops::ru64(r14)) as u64; rsi = (r11) as u64; fl_l = (r11) as u64 as u64; fl_r = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 28; continue; }
            28 => { fl_l = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64 as u64; fl_r = (r11) as u64 as u64; rbx = (rbx & !0xffu64) | (((if (((fl_l as u64)) == ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); rax = ((rbx as u32)) as u32 as u64; bb = 29; continue; }
            29 => { r15 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r13 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r12 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rsi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rbp = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rbx = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            30 => { rax = (0x2u32) as u32 as u64; bb = 29; continue; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e1bba0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 3>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = (rsp) as u64; crate::simd_ops::wu64(rax.wrapping_add(0x8u64), (rbx) as u64); crate::simd_ops::wu64(rax.wrapping_add(0x10u64), (rsi) as u64); crate::simd_ops::wu64(rax.wrapping_add(0x18u64), (rdi) as u64); crate::simd_ops::wu64(rax.wrapping_add(0x20u64), (r12) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r13) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r15) as u64); rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rbx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; r10 = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; r13 = (r9) as u64; r11 = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; r14 = (r8) as u64; r12 = (rdx) as u64; r15 = (rcx) as u64; crate::simd_ops::wu64(rdi, (r8) as u64); crate::simd_ops::wu64(r10, (r11) as u64); rax = (crate::simd_ops::ru64(rdi)) as u64; fl_l = (rax) as u64 as u64; fl_r = (r9) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 23; continue; } bb = 1; continue; }
            1 => { fl_l = (r11) as u64 as u64; fl_r = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 23; continue; } bb = 2; continue; }
            2 => { r8 = (crate::simd_ops::ru32(rax)) as u32 as u64; fl_l = (crate::simd_ops::ru32(r15.wrapping_add(0x10u64))) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 26; continue; } bb = 3; continue; }
            3 => { fl_l = ((r8 as u32)) as u32 as u64; fl_r = (0x80u32) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 5; continue; } bb = 4; continue; }
            4 => { r11 = (r11 & !0xffu64) | ((((r8 as u8)) as u8 as u64)); r9 = ((rbx as u32)) as u32 as u64; bb = 14; continue; }
            5 => { r11 = ((r8 as u32)) as u32 as u64; fl_l = ((r8 as u32)) as u32 as u64; fl_r = (0x800u32) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 7; continue; } bb = 6; continue; }
            6 => { r11 = (((r11 as u32)).wrapping_shr(6u32)) as u32 as u64; r9 = (0x1u32) as u32 as u64; fl_l = ((((r11 as u8)) | (0xc0u8))) as u8 as u64; fl_r = fl_l; r11 = (r11 & !0xffu64) | ((((fl_l as u8)) as u8 as u64)); bb = 14; continue; }
            7 => { fl_l = ((r8 as u32)) as u32 as u64; fl_r = (0x10000u32) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 9; continue; } bb = 8; continue; }
            8 => { r11 = (((r11 as u32)).wrapping_shr(12u32)) as u32 as u64; r9 = (0x2u32) as u32 as u64; fl_l = ((((r11 as u8)) | (0xe0u8))) as u8 as u64; fl_r = fl_l; r11 = (r11 & !0xffu64) | ((((fl_l as u8)) as u8 as u64)); bb = 14; continue; }
            9 => { fl_l = ((r8 as u32)) as u32 as u64; fl_r = (0x200000u32) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 11; continue; } bb = 10; continue; }
            10 => { r11 = (((r11 as u32)).wrapping_shr(18u32)) as u32 as u64; r9 = (0x3u32) as u32 as u64; fl_l = ((((r11 as u8)) | (0xf0u8))) as u8 as u64; fl_r = fl_l; r11 = (r11 & !0xffu64) | ((((fl_l as u8)) as u8 as u64)); bb = 14; continue; }
            11 => { fl_l = ((r8 as u32)) as u32 as u64; fl_r = (0x4000000u32) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 13; continue; } bb = 12; continue; }
            12 => { r11 = (((r11 as u32)).wrapping_shr(24u32)) as u32 as u64; r9 = (0x4u32) as u32 as u64; fl_l = ((((r11 as u8)) | (0xf8u8))) as u8 as u64; fl_r = fl_l; r11 = (r11 & !0xffu64) | ((((fl_l as u8)) as u8 as u64)); bb = 14; continue; }
            13 => { r11 = (((r11 as u32)).wrapping_shr(30u32)) as u32 as u64; r9 = (0x5u32) as u32 as u64; fl_l = ((((r11 as u8)) | (0xfcu8))) as u8 as u64; fl_r = fl_l; r11 = (r11 & !0xffu64) | ((((fl_l as u8)) as u8 as u64)); bb = 14; continue; }
            14 => { fl_l = (crate::simd_ops::ru8(r12)) as u8 as u64; fl_r = ((rbx as u8)) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 18; continue; } bb = 15; continue; }
            15 => { crate::simd_ops::wu8(r12, (0x1u8) as u8); fl_l = (crate::simd_ops::ru8(r15.wrapping_add(0x14u64))) as u8 as u64; fl_r = (0x2u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 18; continue; } bb = 16; continue; }
            16 => { rsi = (crate::simd_ops::ru64(r10)) as u64; rcx = ((r9.wrapping_add(0x4u64)) as u32) as u32 as u64; rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rdx = ((rdx).wrapping_sub(rsi)) as u64; fl_l = (rdx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) as i64) < (((fl_r as u64)) as i64)) { bb = 25; continue; } bb = 17; continue; }
            17 => { crate::simd_ops::wu8(rsi, (0xefu8) as u8); crate::simd_ops::lk_add_u64(r10, (1) as u64); rax = (crate::simd_ops::ru64(r10)) as u64; crate::simd_ops::wu8(rax, (0xbbu8) as u8); crate::simd_ops::lk_add_u64(r10, (1) as u64); rax = (crate::simd_ops::ru64(r10)) as u64; crate::simd_ops::wu8(rax, (0xbfu8) as u8); fl_l = (crate::simd_ops::lk_add_u64(r10, (1) as u64)) as u64; bb = 18; continue; }
            18 => { rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rcx = ((r9.wrapping_add(0x1u64)) as u32) as u32 as u64; rdx = ((rdx).wrapping_sub(crate::simd_ops::ru64(r10))) as u64; fl_l = (rdx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) as i64) < (((fl_r as u64)) as i64)) { bb = 23; continue; } bb = 19; continue; }
            19 => { crate::simd_ops::lk_add_u64(rdi, (0x4u64) as u64); rax = (crate::simd_ops::ru64(r10)) as u64; crate::simd_ops::wu8(rax, ((r11 as u8)) as u8); r11 = (crate::simd_ops::ru64(r10)) as u64; r11 = ((r11).wrapping_add(1)) as u64; crate::simd_ops::wu64(r10, (r11) as u64); fl_l = ((r9 as u32)) as u32 as u64; fl_r = ((r9 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 22; continue; } bb = 20; continue; }
            20 => { rcx = ((r9.wrapping_add(r9.wrapping_mul(2))) as u32) as u32 as u64; fl_l = (((rcx as u32)).wrapping_add((rcx as u32))) as u32 as u64; fl_r = fl_l; rcx = ((fl_l as u32)) as u32 as u64; bb = 21; continue; }
            21 => { rcx = (((rcx as u32)).wrapping_sub(0x6u32)) as u32 as u64; rax = ((r8 as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_shr(((rcx as u32) & 31))) as u32 as u64; r9 = (((r9 as u32)).wrapping_sub(1)) as u32 as u64; rax = (rax & !0xffu64) | ((((((rax as u8)) & (0x3fu8))) as u8 as u64)); rax = (rax & !0xffu64) | ((((((rax as u8)) | (0x80u8))) as u8 as u64)); crate::simd_ops::wu8(r11, ((rax as u8)) as u8); r11 = (crate::simd_ops::ru64(r10)) as u64; r11 = ((r11).wrapping_add(1)) as u64; crate::simd_ops::wu64(r10, (r11) as u64); fl_l = ((r9 as u32)) as u32 as u64; fl_r = ((r9 as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) > 0) { bb = 21; continue; } bb = 22; continue; }
            22 => { rax = (crate::simd_ops::ru64(rdi)) as u64; fl_l = (rax) as u64 as u64; fl_r = (r13) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } bb = 23; continue; }
            23 => { fl_l = (r14) as u64 as u64; fl_r = (crate::simd_ops::ru64(rdi)) as u64 as u64; rbx = (rbx & !0xffu64) | (((if (((fl_l as u64)) == ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); rax = ((rbx as u32)) as u32 as u64; bb = 24; continue; }
            24 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; r15 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r13 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            25 => { rax = (0x1u32) as u32 as u64; bb = 24; continue; }
            26 => { rax = (0x2u32) as u32 as u64; bb = 24; continue; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e1bd60(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x20; const P1: u64 = 0x7001e696a0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rbp) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = (r9) as u64; rbx = (r8) as u64; rbp = (((rdx as u16)) as u32) as u32 as u64; rsi = (rcx) as u64; fl_l = (r8) as u64 as u64; fl_r = (r9) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 3; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(rsi)) as u64; rdx = (((rbp as u16)) as u32) as u32 as u64; r8 = ((crate::simd_ops::ru16(rbx)) as u32) as u32 as u64; rcx = (rsi) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P1)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { rbx = ((rbx).wrapping_add(0x2u64)) as u64; fl_l = (rbx) as u64 as u64; fl_r = (rdi) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } bb = 3; continue; }
            3 => { rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rax = (rbx) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e1bdd0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x20; const P1: u64 = 0x7001e696a0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rbp) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = (r9) as u64; rbx = (r8) as u64; rbp = (((rdx as u16)) as u32) as u32 as u64; rsi = (rcx) as u64; fl_l = (r8) as u64 as u64; fl_r = (r9) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 3; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(rsi)) as u64; rdx = (((rbp as u16)) as u32) as u32 as u64; r8 = ((crate::simd_ops::ru16(rbx)) as u32) as u32 as u64; rcx = (rsi) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P1)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { rbx = ((rbx).wrapping_add(0x2u64)) as u64; fl_l = (rbx) as u64 as u64; fl_r = (rdi) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } bb = 3; continue; }
            3 => { rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rax = (rbx) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e1bfc0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 0>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64;
    crate::simd_ops::wu64(rax, (r8) as u64);
    rax = (0x3u32) as u32 as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e1bfd0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001e97380; const P1: u64 = 0x10; const P2: u64 = 0x7001e4d9a0; const P3: u64 = 0x7001e66450;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (rbx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r15) as u64); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rax = (crate::simd_ops::ru64(P0)) as u64; rax = (((rax) ^ (rsp))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (rax) as u64); r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; r15 = (rdx) as u64; rbp = (r9) as u64; rdi = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; r9 = ((rcx.wrapping_add(P1)) as u64) as u64; rcx = ((rsp.wrapping_add(0x20u64)) as u64) as u64; crate::simd_ops::wu64(r14, (r8) as u64); r8 = (r15) as u64; rbx = (crate::simd_ops::ru64(rdx)) as u64; rdx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) > 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rdi = (0x2u32) as u32 as u64; bb = 6; continue; }
            2 => { rbp = ((rbp).wrapping_sub(crate::simd_ops::ru64(r14))) as u64; rax = (((rax as u32)).wrapping_sub(1)) as u32 as u64; rsi = ((((rax as u32)) as i32 as i64) as u64) as u64; fl_l = (rbp) as u64 as u64; fl_r = (rsi) as u64 as u64; if ((((fl_l as u64)) as i64) >= (((fl_r as u64)) as i64)) { bb = 4; continue; } bb = 3; continue; }
            3 => { crate::simd_ops::wu64(r15, (rbx) as u64); rdi = (0x1u32) as u32 as u64; bb = 6; continue; }
            4 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { rcx = (crate::simd_ops::ru64(r14)) as u64; rdx = ((rsp.wrapping_add(0x20u64)) as u64) as u64; r8 = (rsi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); fl_l = (crate::simd_ops::lk_add_u64(r14, (rsi) as u64)) as u64; fl_r = fl_l; bb = 6; continue; }
            6 => { rax = ((rdi as u32)) as u32 as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; rcx = (((rcx) ^ (rsp))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; r15 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rsi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rbp = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e1c080(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 0>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64;
    crate::simd_ops::wu64(rax, (r8) as u64);
    rax = (0x1u32) as u32 as u64;
    fl_l = ((rax as u16)) as u16 as u64; fl_r = (crate::simd_ops::ru16(rdx)) as u16 as u64;
    rax = (if (((fl_l as u16)) < ((fl_r as u16))) { 0xffffffffu32 } else { 0u32 }) as u32 as u64;
    fl_l = ((((rax as u32)) & (0x2u32))) as u32 as u64; fl_r = fl_l;
    rax = ((fl_l as u32)) as u32 as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e1c0a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001e66450;
    let (mut rax, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = (r8) as u64;
    rcx = (r9) as u64;
    r8 = ((r8).wrapping_sub(rdx)) as u64;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    rax = (rbx) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = core::ptr::read_unaligned(rsp as *const u64);
    rsp = rsp.wrapping_add(8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e1c0c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x30; const P1: u64 = 0x7001e4aae0;
    let (mut rax, mut r11, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    r11 = (rsp) as u64;
    rsp = ((rsp).wrapping_sub(0x38u64)) as u64;
    crate::simd_ops::lk_and_u64(r11.wrapping_add(0x18u64), (0x0u64) as u64);
    rax = ((rcx.wrapping_add(P0)) as u64) as u64;
    crate::simd_ops::wu8(rsp.wrapping_add(0x48u64), ((rdx as u8)) as u8);
    rcx = ((r11.wrapping_add(0x8u64)) as u64) as u64;
    rdx = ((r11.wrapping_add(0x10u64)) as u64) as u64;
    crate::simd_ops::wu64(r11.wrapping_sub(0x18u64), (rax) as u64);
    r9 = ((r11.wrapping_add(0x18u64)) as u64) as u64;
    r8 = (0x1u32) as u32 as u64;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64;
    if !((((((fl_l as u32)) & ((fl_r as u32))) as i32) >= 0)) { rax = (0xffffu32) as u32 as u64; } else { rax = ((crate::simd_ops::ru16(rsp.wrapping_add(0x40u64))) as u32) as u32 as u64; }
    rsp = ((rsp).wrapping_add(0x38u64)) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e1c110(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x30; const P1: u64 = 0x7001e4aae0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rbx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rdi = (r9) as u64; rsi = (r8) as u64; rbx = (rdx) as u64; fl_l = (rdx) as u64 as u64; fl_r = (r8) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 6; continue; } bb = 1; continue; }
            1 => { rbp = ((rcx.wrapping_add(P0)) as u64) as u64; bb = 2; continue; }
            2 => { rax = (rax & !0xffu64) | (((crate::simd_ops::ru8(rbx)) as u8 as u64)); r9 = ((rsp.wrapping_add(0x30u64)) as u64) as u64; crate::simd_ops::lk_and_u64(rsp.wrapping_add(0x30u64), (0x0u64) as u64); rdx = ((rsp.wrapping_add(0x68u64)) as u64) as u64; r8 = (0x1u32) as u32 as u64; crate::simd_ops::wu8(rsp.wrapping_add(0x68u64), ((rax as u8)) as u8); rcx = ((rsp.wrapping_add(0x70u64)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rbp) as u64); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) >= 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { rax = (0xffffu32) as u32 as u64; bb = 5; continue; }
            4 => { rax = ((crate::simd_ops::ru16(rsp.wrapping_add(0x70u64))) as u32) as u32 as u64; bb = 5; continue; }
            5 => { crate::simd_ops::wu16(rdi, ((rax as u16)) as u16); rbx = ((rbx).wrapping_add(1)) as u64; rdi = ((rdi).wrapping_add(0x2u64)) as u64; fl_l = (rbx) as u64 as u64; fl_r = (rsi) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 6; continue; }
            6 => { rax = (rbx) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rsi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rbp = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e1c190(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x30; const P1: u64 = 0x7001e4aae0; const P2: u64 = 0x2;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rbx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rsi = (r9) as u64; rdi = (r8) as u64; rbx = (rdx) as u64; fl_l = (rdx) as u64 as u64; fl_r = (r8) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 3; continue; } bb = 1; continue; }
            1 => { rbp = ((rcx.wrapping_add(P0)) as u64) as u64; bb = 2; continue; }
            2 => { rax = (rax & !0xffu64) | (((crate::simd_ops::ru8(rbx)) as u8 as u64)); r9 = ((rsp.wrapping_add(0x30u64)) as u64) as u64; crate::simd_ops::lk_and_u64(rsp.wrapping_add(0x30u64), (0x0u64) as u64); rdx = ((rsp.wrapping_add(0x68u64)) as u64) as u64; r8 = (0x1u32) as u32 as u64; crate::simd_ops::wu8(rsp.wrapping_add(0x68u64), ((rax as u8)) as u8); rcx = ((rsp.wrapping_add(0x70u64)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rbp) as u64); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rcx = ((crate::simd_ops::ru16(rsp.wrapping_add(0x70u64))) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; rdx = (0xffffu32) as u32 as u64; rcx = (rcx & !0xffffu64) | (((if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { (rdx as u16) } else { (rcx as u16) }) as u16 as u64)); rbx = ((rbx).wrapping_add(1)) as u64; crate::simd_ops::wu16(rsi, ((rcx as u16)) as u16); rsi = ((rsi.wrapping_add(P2)) as u64) as u64; fl_l = (rbx) as u64 as u64; fl_r = (rdi) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 3; continue; }
            3 => { rax = (rbx) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rsi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rbp = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e1c210(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x30; const P1: u64 = 0x7001e4aae0;
    let (mut rax, mut r11, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    r11 = (rsp) as u64;
    rsp = ((rsp).wrapping_sub(0x38u64)) as u64;
    crate::simd_ops::lk_and_u64(r11.wrapping_add(0x18u64), (0x0u64) as u64);
    rax = ((rcx.wrapping_add(P0)) as u64) as u64;
    crate::simd_ops::wu8(rsp.wrapping_add(0x48u64), ((rdx as u8)) as u8);
    rcx = ((r11.wrapping_add(0x8u64)) as u64) as u64;
    rdx = ((r11.wrapping_add(0x10u64)) as u64) as u64;
    crate::simd_ops::wu64(r11.wrapping_sub(0x18u64), (rax) as u64);
    r9 = ((r11.wrapping_add(0x18u64)) as u64) as u64;
    r8 = (0x1u32) as u32 as u64;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    rcx = ((crate::simd_ops::ru16(rsp.wrapping_add(0x40u64))) as u32) as u32 as u64;
    fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64;
    rdx = (0xffffu32) as u32 as u64;
    rcx = (rcx & !0xffffu64) | (((if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { (rdx as u16) } else { (rcx as u16) }) as u16 as u64));
    rax = (((rcx as u16)) as u32) as u32 as u64;
    rsp = ((rsp).wrapping_add(0x38u64)) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e1c270(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x50; const P1: u64 = 0x38;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64;
    rax = (((crate::simd_ops::ru32(rax)) as i32 as i64) as u64) as u64;
    fl_l = ((rax).wrapping_add(crate::simd_ops::ru64(rcx))) as u64 as u64; fl_r = fl_l;
    rax = ((fl_l as u64)) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e1c290(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x10;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = (rax & !0xffu64) | (((crate::simd_ops::ru8(rcx.wrapping_add(P0))) as u8 as u64));
    fl_l = ((((rax as u8)) & (0x1u8))) as u8 as u64; fl_r = fl_l;
    rax = (rax & !0xffu64) | ((((fl_l as u8)) as u8 as u64));
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e1c2a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x58; const P1: u64 = 0x40; const P2: u64 = 0x7001e1c2af; const P3: u64 = 0x8;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; rax = (((crate::simd_ops::ru32(rax)) as i32 as i64) as u64) as u64; fl_l = ((rax).wrapping_add(crate::simd_ops::ru64(rcx))) as u64 as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; return rax; }
            1 => { crate::lift_ops::trap(0x80000003, P2, rcx, 0); return rax; }
            2 => { rax = (crate::simd_ops::ru64(rdx.wrapping_add(P3))) as u64; r9 = (crate::simd_ops::ru64(rax.wrapping_add(P3))) as u64; fl_l = (crate::simd_ops::ru64(rcx.wrapping_add(P3))) as u64 as u64; fl_r = (r9) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 5; continue; } bb = 3; continue; }
            3 => { fl_l = (crate::simd_ops::ru32(rdx)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); return rax; }
            5 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e1c2b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    r9 = (crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64;
    fl_l = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64 as u64; fl_r = (r9) as u64 as u64;
    if (((fl_l as u64)) != ((fl_r as u64))) { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; return rax; }
    fl_l = (crate::simd_ops::ru32(rdx)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64;
    if !((((fl_l as u32)) != ((fl_r as u32)))) { rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); return rax; }
    rax = (rax & !0xffu64) | (((0u8) as u8 as u64));
    fl_l = 0; fl_r = 0;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e1c2d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x18; const P1: u64 = 0x7001e696a0; const P2: u64 = 0x8;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rbx = (r8) as u64; r8 = ((rdx as u32)) as u32 as u64; rdx = ((rsp.wrapping_add(0x20u64)) as u64) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P1)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P2))) as u64; r9 = (crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64; rdx = (crate::simd_ops::ru64(rcx.wrapping_add(P2))) as u64; fl_l = (crate::simd_ops::ru64(r9.wrapping_add(P2))) as u64 as u64; fl_r = (rdx) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } bb = 1; continue; }
            1 => { rcx = (crate::simd_ops::ru32(rbx)) as u32 as u64; fl_l = (crate::simd_ops::ru32(rax)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 3; continue; } bb = 2; continue; }
            2 => { rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 4; continue; }
            3 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 4; continue; }
            4 => { rsp = ((rsp).wrapping_add(0x30u64)) as u64; rbx = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e1c320(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14; const P1: u64 = 0x10; const P2: u64 = 0x7001e1acd0;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    fl_l = ((((rdx as u32)) & (0x17u32))) as u32 as u64; fl_r = fl_l;
    rdx = ((fl_l as u32)) as u32 as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(P0), ((rdx as u32)) as u32);
    rdx = (crate::simd_ops::ru32(rcx.wrapping_add(P1))) as u32 as u64;
    crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e1c330(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x18;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64;
    rdx = (((rdx as u16)) as u32) as u32 as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(P0), ((rdx as u32)) as u32);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e1c3a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x10;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; fl_r = (0x0u32) as u32 as u64;
    rax = (rax & !0xffu64) | (((if (((fl_l as u32)) == ((fl_r as u32))) { 1u8 } else { 0u8 }) as u8 as u64));
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e1c3f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x18; const P2: u64 = 0x20; const P3: u64 = 0x10; const P4: u64 = 0xffffffffffffffe8; const P5: u64 = 0x40; const P6: u64 = 0x7001e696a0; const P7: u64 = 0x38;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&super::EH_META[66], rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = (rsp) as u64; crate::simd_ops::wu64(rax.wrapping_add(P0), (rbx) as u64); crate::simd_ops::wu64(rax.wrapping_add(P1), (rbp) as u64); crate::simd_ops::wu64(rax.wrapping_add(P2), (rsi) as u64); crate::simd_ops::wu64(rax.wrapping_add(P3), (rdx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rbx = (r8) as u64; rsi = (rdx) as u64; rdi = (rcx) as u64; crate::simd_ops::lk_and_u32(rax.wrapping_add(P4), (0x0u32) as u32); rax = (crate::simd_ops::ru64(rcx.wrapping_add(P5))) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64; crate::simd_ops::wu64(rdx.wrapping_add(P0), (rcx) as u64); rax = (crate::simd_ops::ru64(rcx)) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e1c428); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P6)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), (0x1u32) as u32); rbp = (crate::simd_ops::ru64(rdi.wrapping_add(P5))) as u64; rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x8u64))) as u64; fl_l = (rcx) as u64 as u64; fl_r = (crate::simd_ops::ru64(rbx.wrapping_add(P0))) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 4; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(P3))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e1c44b); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P6)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); r8 = (rax) as u64; fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { rcx = (crate::simd_ops::ru64(rax)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = (0x1u32) as u32 as u64; rcx = (r8) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e1c467); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P6)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P0))) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x8u64), (rcx) as u64); rax = (crate::simd_ops::ru64(rcx)) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e1c47c); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P6)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 4; continue; }
            4 => { rbx = (crate::simd_ops::ru64(rdi.wrapping_add(P7))) as u64; bb = 6; continue; }
            5 => { r8 = (crate::simd_ops::ru32(rbx.wrapping_add(P0))) as u32 as u64; rdx = (rdi) as u64; rcx = (0x1u32) as u32 as u64; rax = (crate::simd_ops::ru64(rbx.wrapping_add(P3))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e1c498); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P6)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rbx = (crate::simd_ops::ru64(rbx)) as u64; bb = 6; continue; }
            6 => { fl_l = (rbx) as u64 as u64; fl_r = (rbx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 5; continue; } bb = 7; continue; }
            7 => { rax = (rsi) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e1c510(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001e1a770; const P1: u64 = 0x50; const P2: u64 = 0x48; const P3: u64 = 0x7001e1d400; const P4: u64 = 0x58; const P5: u64 = 0x10; const P6: u64 = 0x14; const P7: u64 = 0x7001e1ef50; const P8: u64 = 0x7001e6bc10; const P9: u64 = 0x7001e6bc28; const P10: u64 = 0x7001e6bc40; const P11: u64 = 0x7001e1c690; const P12: u64 = 0x7001e1887c; const P13: u64 = 0x7001e76d68; const P14: u64 = 0x7001e66444; const P15: u64 = 0x7001e1c5c9;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = ((rsp).wrapping_sub(0x60u64)) as u64; rsi = (rsi & !0xffu64) | ((((r8 as u8)) as u8 as u64)); rbx = (rdx) as u64; rdi = (rcx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); crate::simd_ops::lk_and_u64(rdi.wrapping_add(P1), (0x0u64) as u64); rdx = (rdx & !0xffu64) | (((0x20u8) as u8 as u64)); rcx = (rdi) as u64; crate::simd_ops::wu64(rdi.wrapping_add(P2), (rbx) as u64); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); fl_l = (crate::simd_ops::ru64(rdi.wrapping_add(P2))) as u64 as u64; fl_r = (0x0u64) as u64 as u64; crate::simd_ops::wu8(rdi.wrapping_add(P4), ((rax as u8)) as u8); if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru32(rdi.wrapping_add(P5))) as u32 as u64; rax = ((((rax as u32)) & (0x13u32))) as u32 as u64; rax = ((((rax as u32)) | (0x4u32))) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(P5), ((rax as u32)) as u32); fl_l = ((((rax as u32)) & (crate::simd_ops::ru32(rdi.wrapping_add(P6))))) as u32 as u64; fl_r = fl_l; rax = ((fl_l as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 5; continue; } bb = 2; continue; }
            2 => { fl_l = ((rsi as u8)) as u8 as u64; fl_r = ((rsi as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { rcx = (rdi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P7) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 4; continue; }
            4 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; rsp = ((rsp).wrapping_add(0x60u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            5 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x4u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 7; continue; } bb = 6; continue; }
            6 => { rbx = ((P8) as u64) as u64; bb = 8; continue; }
            7 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x2u8) as u8 as u64; rbx = ((P9) as u64) as u64; rax = ((P10) as u64) as u64; rbx = (if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { rax } else { rbx }) as u64; bb = 8; continue; }
            8 => { rdx = (0x1u32) as u32 as u64; rcx = ((rsp.wrapping_add(0x20u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P11) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); r8 = (rax) as u64; rcx = ((rsp.wrapping_add(0x30u64)) as u64) as u64; rdx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P12) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rdx = ((P13) as u64) as u64; rcx = ((rsp.wrapping_add(0x30u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P14) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); crate::lift_ops::trap(0x80000003, P15, rcx, rsp); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e1c5d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x18; const P1: u64 = 0x2;
    let (mut rax, mut r10, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { bb = 2; continue; }
            1 => { r10 = ((crate::simd_ops::ru8(rdx)) as u32) as u32 as u64; fl_l = ((rdx).wrapping_add(1)) as u64 as u64; rdx = ((fl_l as u64)) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; r10 = ((crate::simd_ops::ru16(rax.wrapping_add(r10.wrapping_mul(2)))) as u32) as u32 as u64; crate::simd_ops::wu16(r9, ((r10 as u16)) as u16); r9 = ((r9.wrapping_add(P1)) as u64) as u64; bb = 2; continue; }
            2 => { fl_l = (rdx) as u64 as u64; fl_r = (r8) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } bb = 3; continue; }
            3 => { rax = (rdx) as u64; return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e1c600(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x18;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    r8 = (((r8 as u8)) as u32) as u32 as u64;
    fl_l = (crate::simd_ops::ru16(rax.wrapping_add(r8.wrapping_mul(2)))) as u16 as u64; fl_r = ((rdx as u16)) as u16 as u64;
    rax = (rax & !0xffu64) | (((if ((((fl_l as u16)) & ((fl_r as u16))) != 0) { 1u8 } else { 0u8 }) as u8 as u64));
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e1c650(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001e19b30;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    rax = ((rax).wrapping_add(0xcu64)) as u64;
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e1c690(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x7001e97180; const P1: u64 = 0x8;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = ((P0) as u64) as u64;
    crate::simd_ops::wu32(rcx, ((rdx as u32)) as u32);
    crate::simd_ops::wu64(rcx.wrapping_add(P1), (rax) as u64);
    rax = (rcx) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e1c6b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x18; const P2: u64 = 0x7001e6b548; const P3: u64 = 0x7001e25eb0; const P4: u64 = 0x7001e174fc;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rbx = (rdx) as u64; fl_l = ((r8 as u32)) as u32 as u64; fl_r = (0x1u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 2; continue; } bb = 1; continue; }
            1 => { xmm0 = 0u128; r8 = (0x15u32) as u32 as u64; crate::simd_ops::wu128(rdx, (xmm0) as u128); crate::simd_ops::lk_and_u64(rdx.wrapping_add(P0), (0x0u64) as u64); fl_l = (crate::simd_ops::lk_and_u64(rdx.wrapping_add(P1), (0x0u64) as u64)) as u64; fl_r = fl_l; rdx = ((P2) as u64) as u64; bb = 5; continue; }
            2 => { rcx = ((r8 as u32)) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; xmm0 = 0u128; crate::simd_ops::wu128(rbx, (xmm0) as u128); crate::simd_ops::lk_and_u64(rbx.wrapping_add(P0), (0x0u64) as u64); crate::simd_ops::lk_and_u64(rbx.wrapping_add(P1), (0x0u64) as u64); fl_l = (((r8) | (0xffffffffffffffffu64))) as u64 as u64; fl_r = fl_l; r8 = ((fl_l as u64)) as u64; bb = 3; continue; }
            3 => { r8 = ((r8).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru8(rax.wrapping_add(r8))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 3; continue; } bb = 4; continue; }
            4 => { rdx = (rax) as u64; bb = 5; continue; }
            5 => { rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = (rbx) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rbx = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e1c7c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001e97380; const P1: u64 = 0x7001e69558; const P2: u64 = 0x7001e696a0; const P3: u64 = 0x7001e69578;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 21>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut fl_k: u32 = 0;
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rbp = ((rsp.wrapping_sub(0x47u64)) as u64) as u64; rsp = ((rsp).wrapping_sub(0x90u64)) as u64; rax = (crate::simd_ops::ru64(P0)) as u64; rax = (((rax) ^ (rsp))) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x3fu64), (rax) as u64); rdi = ((((rdi as u32)) | (0xffffffffu32))) as u32 as u64; rsi = ((rdx as u32)) as u32 as u64; rbx = (rcx) as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; fl_k = 2; bb = 20; continue; }
            2 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x40u64))) as u64; fl_l = (crate::simd_ops::ru64(rax)) as u64 as u64; fl_r = (0x0u64) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 5; continue; } bb = 3; continue; }
            3 => { rdx = (crate::simd_ops::ru64(rcx.wrapping_add(0x58u64))) as u64; r8 = (((crate::simd_ops::ru32(rdx)) as i32 as i64) as u64) as u64; rcx = (r8) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rax))) as u64; fl_l = (crate::simd_ops::ru64(rax)) as u64 as u64; fl_r = (rcx) as u64 as u64; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 5; continue; } bb = 4; continue; }
            4 => { rcx = ((r8.wrapping_sub(0x1u64)) as u32) as u32 as u64; rax = ((rsi as u32)) as u32 as u64; crate::simd_ops::wu32(rdx, ((rcx as u32)) as u32); rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x40u64))) as u64; r8 = (crate::simd_ops::ru64(rdx)) as u64; rcx = ((r8.wrapping_add(0x1u64)) as u64) as u64; crate::simd_ops::wu64(rdx, (rcx) as u64); crate::simd_ops::wu8(r8, ((rsi as u8)) as u8); fl_k = 1; bb = 20; continue; }
            5 => { fl_l = (crate::simd_ops::ru64(rbx.wrapping_add(0x80u64))) as u64 as u64; fl_r = (0x0u64) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { fl_k = 1; bb = 19; continue; } bb = 6; continue; }
            6 => { r8 = (crate::simd_ops::ru64(rbx.wrapping_add(0x18u64))) as u64; rax = ((rbx.wrapping_add(0x70u64)) as u64) as u64; fl_l = (crate::simd_ops::ru64(r8)) as u64 as u64; fl_r = (rax) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { fl_k = 1; bb = 8; continue; } bb = 7; continue; }
            7 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x88u64))) as u64; rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x90u64))) as u64; crate::simd_ops::wu64(r8, (rcx) as u64); fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; rdx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x38u64))) as u64; crate::simd_ops::wu64(rax, (rcx) as u64); rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x50u64))) as u64; crate::simd_ops::wu32(rax, ((rdx as u32)) as u32); fl_k = 0; bb = 8; continue; }
            8 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x68u64))) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 11; continue; } bb = 9; continue; }
            9 => { rcx = ((((rsi as u8)) as i8 as i32) as u32) as u32 as u64; fl_k = 3; bb = 10; continue; }
            10 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x80u64))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P1)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; fl_k = 0; bb = 18; continue; }
            11 => { r8 = ((rbp.wrapping_add(0xfu64)) as u64) as u64; crate::simd_ops::wu8(rbp.wrapping_add(0x7u64), ((rsi as u8)) as u8); rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = ((rbx.wrapping_add(0x74u64)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), (r8) as u64); r9 = ((rbp.wrapping_add(0x8u64)) as u64) as u64; r8 = ((rbp.wrapping_add(0x3fu64)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (r8) as u64); r8 = ((rbp.wrapping_add(0x1fu64)) as u64) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(0x38u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (r8) as u64); r8 = ((rbp.wrapping_add(0x17u64)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (r8) as u64); r8 = ((rbp.wrapping_add(0x7u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P2)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { fl_k = 2; bb = 15; continue; } bb = 12; continue; }
            12 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x1u32) as u32 as u64; rax = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { fl_k = 0; bb = 15; continue; } bb = 13; continue; }
            13 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x2u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { fl_k = 0; bb = 19; continue; } bb = 14; continue; }
            14 => { rcx = (((crate::simd_ops::ru8(rbp.wrapping_add(0x7u64))) as i8 as i32) as u32) as u32 as u64; fl_k = 0; bb = 10; continue; }
            15 => { r14 = (crate::simd_ops::ru64(rbp.wrapping_add(0xfu64))) as u64; rax = ((rbp.wrapping_add(0x1fu64)) as u64) as u64; fl_l = (r14) as u64 as u64; fl_r = (rax) as u64 as u64; r14 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 17; continue; } bb = 16; continue; }
            16 => { r9 = (crate::simd_ops::ru64(rbx.wrapping_add(0x80u64))) as u64; rcx = ((rbp.wrapping_add(0x1fu64)) as u64) as u64; r8 = (r14) as u64; rdx = (0x1u32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P3)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); fl_l = (r14) as u64 as u64; fl_r = (rax) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { fl_k = 1; bb = 19; continue; } bb = 17; continue; }
            17 => { rax = ((rbp.wrapping_add(0x7u64)) as u64) as u64; crate::simd_ops::wu8(rbx.wrapping_add(0x71u64), (0x1u8) as u8); fl_l = (crate::simd_ops::ru64(rbp.wrapping_add(0x17u64))) as u64 as u64; fl_r = (rax) as u64 as u64; fl_k = 1; bb = 18; continue; }
            18 => { rdi = (if (match fl_k { 0 => (((fl_l as u32)) != ((fl_r as u32))), 1 => (((fl_l as u64)) != ((fl_r as u64))), _ => false }) { (rsi as u32) } else { (rdi as u32) }) as u32 as u64; bb = 19; continue; }
            19 => { rax = ((rdi as u32)) as u32 as u64; bb = 20; continue; }
            20 => { rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x3fu64))) as u64; fl_l = (((rcx) ^ (rsp))) as u64 as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; r11 = ((rsp.wrapping_add(0x90u64)) as u64) as u64; rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x38u64))) as u64; rsp = (r11) as u64; r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rbp = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e1c950(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    fl_l = ((((rax as u32)) | (0xffffffffu32))) as u32 as u64; fl_r = fl_l;
    rax = ((fl_l as u32)) as u32 as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e1c960(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001e695a0;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rbx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x38u64))) as u64; rbx = (rcx) as u64; r8 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rdi = ((rdx as u32)) as u32 as u64; rcx = (crate::simd_ops::ru64(rax)) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x18u64))) as u64; fl_l = (crate::simd_ops::ru64(rax)) as u64 as u64; fl_r = (rcx) as u64 as u64; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 6; continue; } bb = 2; continue; }
            2 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = (0xffffffffu32) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rax = ((crate::simd_ops::ru8(rcx.wrapping_sub(0x1u64))) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 6; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x50u64))) as u64; crate::simd_ops::lk_add_u32(rax, (1) as u32); rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x38u64))) as u64; crate::simd_ops::lk_sub_u64(rax, (1) as u64); fl_l = ((rdi as u32)) as u32 as u64; fl_r = (0xffffffffu32) as u32 as u64; rdi = (if (((fl_l as u32)) == ((fl_r as u32))) { (r8 as u32) } else { (rdi as u32) }) as u32 as u64; bb = 5; continue; }
            5 => { rax = ((rdi as u32)) as u32 as u64; bb = 15; continue; }
            6 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x80u64))) as u64; fl_l = (rdx) as u64 as u64; fl_r = (rdx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 14; continue; } bb = 7; continue; }
            7 => { fl_l = ((rdi as u32)) as u32 as u64; fl_r = (0xffffffffu32) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 14; continue; } bb = 8; continue; }
            8 => { fl_l = (crate::simd_ops::ru64(rbx.wrapping_add(0x68u64))) as u64 as u64; fl_r = (r8) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 10; continue; } bb = 9; continue; }
            9 => { rcx = (((rdi as u8)) as u32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P0)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); fl_l = ((rax as u32)) as u32 as u64; fl_r = (0xffffffffu32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 5; continue; } bb = 10; continue; }
            10 => { r9 = (crate::simd_ops::ru64(rbx.wrapping_add(0x38u64))) as u64; rdx = ((rbx.wrapping_add(0x70u64)) as u64) as u64; fl_l = (crate::simd_ops::ru64(r9)) as u64 as u64; fl_r = (rdx) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 14; continue; } bb = 11; continue; }
            11 => { r10 = (crate::simd_ops::ru64(rbx.wrapping_add(0x18u64))) as u64; r8 = ((rbx.wrapping_add(0x50u64)) as u64) as u64; crate::simd_ops::wu8(rdx, ((rdi as u8)) as u8); rax = (crate::simd_ops::ru64(r10)) as u64; fl_l = (rax) as u64 as u64; fl_r = (rdx) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 13; continue; } bb = 12; continue; }
            12 => { crate::simd_ops::wu64(rbx.wrapping_add(0x88u64), (rax) as u64); rax = (crate::simd_ops::ru64(r8)) as u64; rcx = (((crate::simd_ops::ru32(rax)) as i32 as i64) as u64) as u64; fl_l = ((rcx).wrapping_add(crate::simd_ops::ru64(r9))) as u64 as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x90u64), (rcx) as u64); bb = 13; continue; }
            13 => { crate::simd_ops::wu64(r10, (rdx) as u64); rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x38u64))) as u64; rbx = (((rbx as u32)).wrapping_sub((rdx as u32))) as u32 as u64; fl_l = (((rbx as u32)).wrapping_add(0x71u32)) as u32 as u64; fl_r = fl_l; rbx = ((fl_l as u32)) as u32 as u64; crate::simd_ops::wu64(rax, (rdx) as u64); rax = (crate::simd_ops::ru64(r8)) as u64; crate::simd_ops::wu32(rax, ((rbx as u32)) as u32); bb = 5; continue; }
            14 => { fl_l = ((((rax as u32)) | (0xffffffffu32))) as u32 as u64; fl_r = fl_l; rax = ((fl_l as u32)) as u32 as u64; bb = 15; continue; }
            15 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e1ca80(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001e19b30;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    rax = ((rax).wrapping_add(0x10u64)) as u64;
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e1cab0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001e659fc; const P1: u64 = 0x38; const P2: u64 = 0x8; const P3: u64 = 0x10;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rbx) as u64);
    crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (rsi) as u64);
    rsp = rsp.wrapping_sub(8);
    core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = (rcx) as u64;
    rdi = ((r8 as u32)) as u32 as u64;
    rcx = (0x18u32) as u32 as u64;
    rsi = (rdx) as u64;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (rax) as u64);
    fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { r9 = (crate::simd_ops::ru64(rbx.wrapping_add(P1))) as u64; crate::simd_ops::wu64(rax, (r9) as u64); crate::simd_ops::wu32(rax.wrapping_add(P2), ((rdi as u32)) as u32); crate::simd_ops::wu64(rax.wrapping_add(P3), (rsi) as u64); }
    rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64;
    crate::simd_ops::wu64(rbx.wrapping_add(P1), (rax) as u64);
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = core::ptr::read_unaligned(rsp as *const u64);
    rsp = rsp.wrapping_add(8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e1cb00(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x18;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { fl_l = (r8) as u64 as u64; fl_r = (r9) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 4; continue; } bb = 1; continue; }
            1 => { rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; bb = 2; continue; }
            2 => { rax = ((crate::simd_ops::ru8(r8)) as u32) as u32 as u64; fl_l = (crate::simd_ops::ru16(rcx.wrapping_add(rax.wrapping_mul(2)))) as u16 as u64; fl_r = ((rdx as u16)) as u16 as u64; if ((((fl_l as u16)) & ((fl_r as u16))) != 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { r8 = ((r8).wrapping_add(1)) as u64; fl_l = (r8) as u64 as u64; fl_r = (r9) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 4; continue; }
            4 => { rax = (r8) as u64; return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e1cb40(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x18;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { fl_l = (r8) as u64 as u64; fl_r = (r9) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 4; continue; } bb = 1; continue; }
            1 => { rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; bb = 2; continue; }
            2 => { rax = ((crate::simd_ops::ru8(r8)) as u32) as u32 as u64; fl_l = (crate::simd_ops::ru16(rcx.wrapping_add(rax.wrapping_mul(2)))) as u16 as u64; fl_r = ((rdx as u16)) as u16 as u64; if ((((fl_l as u16)) & ((fl_r as u16))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { r8 = ((r8).wrapping_add(1)) as u64; fl_l = (r8) as u64 as u64; fl_r = (r9) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 4; continue; }
            4 => { rax = (r8) as u64; return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e1cb80(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001e19a58; const P1: u64 = 0x7001e69570; const P2: u64 = 0x7001e69550;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (rbp) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x38u64))) as u64; r14 = ((rcx.wrapping_add(0x70u64)) as u64) as u64; rbp = ((r9 as u32)) as u32 as u64; rsi = (r8) as u64; rbx = (rdx) as u64; rdi = (rcx) as u64; fl_l = (crate::simd_ops::ru64(rax)) as u64 as u64; fl_r = (r14) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 4; continue; } bb = 1; continue; }
            1 => { fl_l = ((r9 as u32)) as u32 as u64; fl_r = (0x1u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 4; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru64(rcx.wrapping_add(0x68u64))) as u64 as u64; fl_r = (0x0u64) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 4; continue; } bb = 3; continue; }
            3 => { fl_l = ((rsi).wrapping_sub(1)) as u64 as u64; rsi = ((fl_l as u64)) as u64; bb = 4; continue; }
            4 => { fl_l = (crate::simd_ops::ru64(rcx.wrapping_add(0x80u64))) as u64 as u64; fl_r = (0x0u64) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 13; continue; } bb = 5; continue; }
            5 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 13; continue; } bb = 6; continue; }
            6 => { fl_l = (rsi) as u64 as u64; fl_r = (rsi) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 8; continue; } bb = 7; continue; }
            7 => { fl_l = ((rbp as u32)) as u32 as u64; fl_r = (0x1u32) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 9; continue; } bb = 8; continue; }
            8 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x80u64))) as u64; r8 = ((rbp as u32)) as u32 as u64; rdx = (rsi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P1)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 13; continue; } bb = 9; continue; }
            9 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x80u64))) as u64; rdx = ((rsp.wrapping_add(0x40u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P2)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 13; continue; } bb = 10; continue; }
            10 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x18u64))) as u64; fl_l = (crate::simd_ops::ru64(rax)) as u64 as u64; fl_r = (r14) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 12; continue; } bb = 11; continue; }
            11 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x88u64))) as u64; rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0x90u64))) as u64; crate::simd_ops::wu64(rax, (rcx) as u64); fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; rdx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x38u64))) as u64; crate::simd_ops::wu64(rax, (rcx) as u64); rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x50u64))) as u64; crate::simd_ops::wu32(rax, ((rdx as u32)) as u32); bb = 12; continue; }
            12 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x74u64))) as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; fl_l = (crate::simd_ops::lk_and_u64(rbx.wrapping_add(0x8u64), (0x0u64) as u64)) as u64; fl_r = fl_l; crate::simd_ops::wu64(rbx, (rcx) as u64); bb = 14; continue; }
            13 => { crate::simd_ops::lk_or_u64(rbx, (0xffffffffffffffffu64) as u64); crate::simd_ops::lk_and_u64(rbx.wrapping_add(0x8u64), (0x0u64) as u64); rax = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; bb = 14; continue; }
            14 => { rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), (rax) as u64); rax = (rbx) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rsi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e1cc70(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    crate::simd_ops::lk_or_u64(rdx, (0xffffffffffffffffu64) as u64);
    rax = (0u32) as u32 as u64;
    fl_l = 0; fl_r = 0;
    fl_l = (crate::simd_ops::lk_and_u64(rdx.wrapping_add(P0), (0x0u64) as u64)) as u64; fl_r = fl_l;
    crate::simd_ops::wu64(rdx.wrapping_add(P1), (rax) as u64);
    rax = (rdx) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e1cc90(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x80; const P2: u64 = 0x7001e19a58; const P3: u64 = 0x7001e69568; const P4: u64 = 0x10; const P5: u64 = 0x18; const P6: u64 = 0x74; const P7: u64 = 0x70; const P8: u64 = 0x88; const P9: u64 = 0x90; const P10: u64 = 0x38; const P11: u64 = 0x50;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64; rsi = (r8) as u64; rax = ((rax).wrapping_add(crate::simd_ops::ru64(r8))) as u64; rbx = (rdx) as u64; fl_l = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64 as u64; fl_r = (0x0u64) as u64 as u64; rdi = (rcx) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (rax) as u64); if (((fl_l as u64)) == ((fl_r as u64))) { bb = 6; continue; } bb = 1; continue; }
            1 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 6; continue; } bb = 2; continue; }
            2 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(P1))) as u64; rdx = ((rsp.wrapping_add(0x30u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P3)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 6; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rsi.wrapping_add(P4))) as u64; r8 = (crate::simd_ops::ru64(rdi.wrapping_add(P5))) as u64; crate::simd_ops::wu64(rdi.wrapping_add(P6), (rax) as u64); rax = ((rdi.wrapping_add(P7)) as u64) as u64; fl_l = (crate::simd_ops::ru64(r8)) as u64 as u64; fl_r = (rax) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 5; continue; } bb = 4; continue; }
            4 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(P8))) as u64; rdx = (crate::simd_ops::ru64(rdi.wrapping_add(P9))) as u64; crate::simd_ops::wu64(r8, (rcx) as u64); fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; rdx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; rax = (crate::simd_ops::ru64(rdi.wrapping_add(P10))) as u64; crate::simd_ops::wu64(rax, (rcx) as u64); rax = (crate::simd_ops::ru64(rdi.wrapping_add(P11))) as u64; crate::simd_ops::wu32(rax, ((rdx as u32)) as u32); bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x74u64))) as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; fl_l = (crate::simd_ops::lk_and_u64(rbx.wrapping_add(0x8u64), (0x0u64) as u64)) as u64; fl_r = fl_l; crate::simd_ops::wu64(rbx, (rcx) as u64); bb = 7; continue; }
            6 => { crate::simd_ops::lk_or_u64(rbx, (0xffffffffffffffffu64) as u64); crate::simd_ops::lk_and_u64(rbx.wrapping_add(0x8u64), (0x0u64) as u64); rax = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; bb = 7; continue; }
            7 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), (rax) as u64); rax = (rbx) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e1cd50(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x4; const P1: u64 = 0x80; const P2: u64 = 0x7001e69580; const P3: u64 = 0x1; const P4: u64 = 0x7001e1a548;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; r9 = (r8) as u64; rbx = (rcx) as u64; fl_l = (rdx) as u64 as u64; fl_r = (rdx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { fl_l = (r8) as u64 as u64; fl_r = (r8) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { r8 = ((r9.wrapping_add(P0)) as u32) as u32 as u64; bb = 4; continue; }
            3 => { r8 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; bb = 4; continue; }
            4 => { rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 5; continue; }
            5 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P2)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 7; continue; } bb = 6; continue; }
            6 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P1))) as u64; r8 = ((rax.wrapping_add(P3)) as u32) as u32 as u64; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rax = (rbx) as u64; bb = 8; continue; }
            7 => { rax = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; bb = 8; continue; }
            8 => { rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e1cdb0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x18;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64;
    rdx = (((rdx as u16)) as u32) as u32 as u64;
    fl_l = ((((rdx as u32)) | ((rax as u32)))) as u32 as u64; fl_r = fl_l;
    rdx = ((fl_l as u32)) as u32 as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(P0), ((rdx as u32)) as u32);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e1cdc0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x18;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64;
    r9 = ((r8 as u32)) as u32 as u64;
    r9 = (!((r9 as u32))) as u32 as u64;
    rdx = ((((rdx as u32)) & ((r8 as u32)))) as u32 as u64;
    r9 = ((((r9 as u32)) & ((rax as u32)))) as u32 as u64;
    rdx = (((rdx as u16)) as u32) as u32 as u64;
    fl_l = ((((r9 as u32)) | ((rdx as u32)))) as u32 as u64; fl_r = fl_l;
    r9 = ((fl_l as u32)) as u32 as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(P0), ((r9 as u32)) as u32);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e1ce00(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x20; const P1: u64 = 0x40; const P2: u64 = 0x58;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    fl_l = ((r8 as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64;
    r8 = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64;
    crate::simd_ops::wu64(rax, (rdx) as u64);
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64;
    crate::simd_ops::wu64(rax, (rdx) as u64);
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P2))) as u64;
    crate::simd_ops::wu32(rax, ((r8 as u32)) as u32);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e1ce20(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x48; const P1: u64 = 0x10; const P2: u64 = 0x7001e1ad40;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    r9 = ((rdx as u32)) as u32 as u64;
    fl_l = 0; fl_r = (rax) as u64 as u64;
    rax = (0u64.wrapping_sub(fl_r as u64)) as u64;
    rdx = (if (((fl_l as u64)) < ((fl_r as u64))) { 0xffffffffu32 } else { 0u32 }) as u32 as u64;
    rdx = (!((rdx as u32))) as u32 as u64;
    rdx = ((((rdx as u32)) & (0x4u32))) as u32 as u64;
    rdx = ((((rdx as u32)) | (crate::simd_ops::ru32(rcx.wrapping_add(P1))))) as u32 as u64;
    fl_l = ((((rdx as u32)) | ((r9 as u32)))) as u32 as u64; fl_r = fl_l;
    rdx = ((fl_l as u32)) as u32 as u64;
    crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e1ce40(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x14; const P2: u64 = 0x7001e6bc10; const P3: u64 = 0x7001e6bc28; const P4: u64 = 0x7001e6bc40; const P5: u64 = 0x7001e1c690; const P6: u64 = 0x7001e1887c; const P7: u64 = 0x7001e76d68; const P8: u64 = 0x7001e66444; const P9: u64 = 0x7001e1cead;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64); rsp = ((rsp).wrapping_sub(0x60u64)) as u64; rdx = ((((rdx as u32)) | (crate::simd_ops::ru32(rcx.wrapping_add(P0))))) as u32 as u64; rdx = ((((rdx as u32)) & (0x17u32))) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(P0), ((rdx as u32)) as u32); fl_l = ((((rdx as u32)) & (crate::simd_ops::ru32(rcx.wrapping_add(P1))))) as u32 as u64; fl_r = fl_l; rdx = ((fl_l as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rsp = ((rsp).wrapping_add(0x60u64)) as u64; rbx = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            2 => { fl_l = ((rdx as u8)) as u8 as u64; fl_r = (0x4u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { rbx = ((P2) as u64) as u64; bb = 5; continue; }
            4 => { fl_l = ((rdx as u8)) as u8 as u64; fl_r = (0x2u8) as u8 as u64; rbx = ((P3) as u64) as u64; rax = ((P4) as u64) as u64; rbx = (if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { rax } else { rbx }) as u64; bb = 5; continue; }
            5 => { rdx = (0x1u32) as u32 as u64; rcx = ((rsp.wrapping_add(0x20u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); r8 = (rax) as u64; rcx = ((rsp.wrapping_add(0x30u64)) as u64) as u64; rdx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rdx = ((P7) as u64) as u64; rcx = ((rsp.wrapping_add(0x30u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P8) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); crate::lift_ops::trap(0x80000003, P9, rcx, rsp); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e1ceb0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x7001e1ad40;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    fl_l = ((((rdx as u32)) | (crate::simd_ops::ru32(rcx.wrapping_add(P0))))) as u32 as u64; fl_r = fl_l;
    rdx = ((fl_l as u32)) as u32 as u64;
    crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e1ced0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x14; const P2: u64 = 0x18; const P3: u64 = 0x20; const P4: u64 = 0x28; const P5: u64 = 0x30; const P6: u64 = 0x38; const P7: u64 = 0x40;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { r9 = (rcx) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rdx) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { r8 = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(P0), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdx.wrapping_add(P1))) as u32 as u64; crate::simd_ops::wu32(rdx.wrapping_add(P0), ((r8 as u32)) as u32); r8 = (crate::simd_ops::ru32(rcx.wrapping_add(P1))) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(P1), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdx.wrapping_add(P2))) as u32 as u64; crate::simd_ops::wu32(rdx.wrapping_add(P1), ((r8 as u32)) as u32); rcx = (crate::simd_ops::ru32(rcx.wrapping_add(P2))) as u32 as u64; crate::simd_ops::wu32(r9.wrapping_add(P2), ((rax as u32)) as u32); rax = (crate::simd_ops::ru64(rdx.wrapping_add(P3))) as u64; crate::simd_ops::wu32(rdx.wrapping_add(P2), ((rcx as u32)) as u32); rcx = (crate::simd_ops::ru64(r9.wrapping_add(P3))) as u64; crate::simd_ops::wu64(r9.wrapping_add(P3), (rax) as u64); rax = (crate::simd_ops::ru64(rdx.wrapping_add(P4))) as u64; crate::simd_ops::wu64(rdx.wrapping_add(P3), (rcx) as u64); rcx = (crate::simd_ops::ru64(r9.wrapping_add(P4))) as u64; crate::simd_ops::wu64(r9.wrapping_add(P4), (rax) as u64); rax = (crate::simd_ops::ru64(rdx.wrapping_add(P5))) as u64; crate::simd_ops::wu64(rdx.wrapping_add(P4), (rcx) as u64); rcx = (crate::simd_ops::ru64(r9.wrapping_add(P5))) as u64; crate::simd_ops::wu64(r9.wrapping_add(P5), (rax) as u64); rax = (crate::simd_ops::ru64(rdx.wrapping_add(P6))) as u64; crate::simd_ops::wu64(rdx.wrapping_add(P5), (rcx) as u64); rcx = (crate::simd_ops::ru64(r9.wrapping_add(P6))) as u64; crate::simd_ops::wu64(r9.wrapping_add(P6), (rax) as u64); rax = (crate::simd_ops::ru64(rdx.wrapping_add(P7))) as u64; crate::simd_ops::wu64(rdx.wrapping_add(P6), (rcx) as u64); rcx = (crate::simd_ops::ru64(r9.wrapping_add(P7))) as u64; crate::simd_ops::wu64(r9.wrapping_add(P7), (rax) as u64); crate::simd_ops::wu64(rdx.wrapping_add(P7), (rcx) as u64); bb = 2; continue; }
            2 => { return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e1cf60(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x80; const P1: u64 = 0x18; const P2: u64 = 0x7001e696a0; const P3: u64 = 0x7001e69548; const P4: u64 = 0xffffffffffffffff;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rbx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rdi = (rcx) as u64; fl_l = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64 as u64; fl_r = (rbx) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 3; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = ((((rdx as u32)) | (0xffffffffu32))) as u32 as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P2)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); fl_l = ((rax as u32)) as u32 as u64; fl_r = (0xffffffffu32) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 3; continue; } bb = 2; continue; }
            2 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(P0))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P3)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; rbx = (rbx & !0xffu64) | (((if (((((fl_l as u32)) & ((fl_r as u32))) as i32) >= 0) { 1u8 } else { 0u8 }) as u8 as u64)); rax = ((rbx.wrapping_add(P4)) as u32) as u32 as u64; bb = 4; continue; }
            3 => { rax = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; bb = 4; continue; }
            4 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e1cfc0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001e49e20; const P1: u64 = 0x7001e97230; const P2: u64 = 0x7001e49ec0;
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rbx) as u64);
    rsp = rsp.wrapping_sub(8);
    core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rdi = (rdi & !0xffu64) | ((((rcx as u8)) as u8 as u64));
    rdx = (0x2u32) as u32 as u64;
    rcx = ((rsp.wrapping_add(0x30u64)) as u64) as u64;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    rbx = (rbx & !0xffu64) | (((crate::simd_ops::ru8(P1)) as u8 as u64));
    rcx = ((rsp.wrapping_add(0x30u64)) as u64) as u64;
    crate::simd_ops::wu8(P1, ((rdi as u8)) as u8);
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    rax = (rax & !0xffu64) | ((((rbx as u8)) as u8 as u64));
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = core::ptr::read_unaligned(rsp as *const u64);
    rsp = rsp.wrapping_add(8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e1d060(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001e97380; const P1: u64 = 0x7001e69508; const P2: u64 = 0x7001e177fc; const P3: u64 = 0x7001e696a0; const P4: u64 = 0x7001e6646c; const P5: u64 = 0x7001e695a0; const P6: u64 = 0x7001e69480; const P7: u64 = 0x7001e1d2e7; const P8: u64 = 0x7001e6649c;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 21>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&super::EH_META[67], rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rbp = ((rsp.wrapping_sub(0x47u64)) as u64) as u64; rsp = ((rsp).wrapping_sub(0x90u64)) as u64; rax = (crate::simd_ops::ru64(P0)) as u64; rax = (((rax) ^ (rsp))) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x3fu64), (rax) as u64); rsi = (rcx) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x38u64))) as u64; rcx = (crate::simd_ops::ru64(rax)) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { rdx = (crate::simd_ops::ru64(rsi.wrapping_add(0x50u64))) as u64; r9 = (((crate::simd_ops::ru32(rdx)) as i32 as i64) as u64) as u64; rax = ((rcx.wrapping_add(r9)) as u64) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rax) as u64 as u64; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 3; continue; } bb = 2; continue; }
            2 => { rax = ((r9.wrapping_sub(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu32(rdx, ((rax as u32)) as u32); rcx = (crate::simd_ops::ru64(rsi.wrapping_add(0x38u64))) as u64; rdx = (crate::simd_ops::ru64(rcx)) as u64; rax = ((rdx.wrapping_add(0x1u64)) as u64) as u64; crate::simd_ops::wu64(rcx, (rax) as u64); rax = ((crate::simd_ops::ru8(rdx)) as u32) as u32 as u64; bb = 32; continue; }
            3 => { fl_l = (crate::simd_ops::ru64(rsi.wrapping_add(0x80u64))) as u64 as u64; fl_r = (0x0u64) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 5; continue; } bb = 4; continue; }
            4 => { fl_l = ((((rax as u32)) | (0xffffffffu32))) as u32 as u64; fl_r = fl_l; rax = ((fl_l as u32)) as u32 as u64; bb = 32; continue; }
            5 => { r9 = (crate::simd_ops::ru64(rsi.wrapping_add(0x18u64))) as u64; rax = ((rsi.wrapping_add(0x70u64)) as u64) as u64; fl_l = (crate::simd_ops::ru64(r9)) as u64 as u64; fl_r = (rax) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 7; continue; } bb = 6; continue; }
            6 => { rdx = (crate::simd_ops::ru64(rsi.wrapping_add(0x90u64))) as u64; rcx = (crate::simd_ops::ru64(rsi.wrapping_add(0x88u64))) as u64; crate::simd_ops::wu64(r9, (rcx) as u64); rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x38u64))) as u64; crate::simd_ops::wu64(rax, (rcx) as u64); fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; rdx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x50u64))) as u64; crate::simd_ops::wu32(rax, ((rdx as u32)) as u32); bb = 7; continue; }
            7 => { rcx = (crate::simd_ops::ru64(rsi.wrapping_add(0x80u64))) as u64; fl_l = (crate::simd_ops::ru64(rsi.wrapping_add(0x68u64))) as u64 as u64; fl_r = (0x0u64) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 10; continue; } bb = 8; continue; }
            8 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e1d110); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P1)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdi = ((((rdi as u32)) | (0xffffffffu32))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 31; continue; } bb = 9; continue; }
            9 => { rdi = (((rax as u8)) as u32) as u32 as u64; bb = 31; continue; }
            10 => { xmm0 = 0u128; crate::simd_ops::wu128(rbp.wrapping_add(0x1fu64), (xmm0) as u128); crate::simd_ops::lk_and_u64(rbp.wrapping_add(0x2fu64), (0x0u64) as u64); crate::simd_ops::wu64(rbp.wrapping_add(0x37u64), (0xfu64) as u64); crate::simd_ops::wu8(rbp.wrapping_add(0x1fu64), (0x0u8) as u8); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e1d141); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P1)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; fl_l = ((((rdi as u32)) | (0xffffffffu32))) as u32 as u64; fl_r = fl_l; rdi = ((fl_l as u32)) as u32 as u64; bb = 18; continue; }
            11 => { rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x2fu64))) as u64; fl_l = (rcx) as u64 as u64; fl_r = (crate::simd_ops::ru64(rbp.wrapping_add(0x37u64))) as u64 as u64; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 13; continue; } bb = 12; continue; }
            12 => { rax = ((rcx.wrapping_add(0x1u64)) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x2fu64), (rax) as u64); rax = ((rbp.wrapping_add(0x1fu64)) as u64) as u64; fl_l = (crate::simd_ops::ru64(rbp.wrapping_add(0x37u64))) as u64 as u64; fl_r = (0xfu64) as u64 as u64; rax = (if (((fl_l as u64)) > ((fl_r as u64))) { crate::simd_ops::ru64(rbp.wrapping_add(0x1fu64)) } else { rax }) as u64; crate::simd_ops::wu8(rax.wrapping_add(rcx), ((rdx as u8)) as u8); crate::simd_ops::wu8(rax.wrapping_add(rcx).wrapping_add(0x1u64), (0x0u8) as u8); bb = 14; continue; }
            13 => { r9 = (r9 & !0xffu64) | ((((rdx as u8)) as u8 as u64)); r8 = (r8 & !0xffu64) | (((crate::simd_ops::ru8(rbp.wrapping_add(0x8u64))) as u8 as u64)); rdx = (0x1u32) as u32 as u64; rcx = ((rbp.wrapping_add(0x1fu64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e1d189); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 14; continue; }
            14 => { r10 = (crate::simd_ops::ru64(rsi.wrapping_add(0x68u64))) as u64; rcx = ((rbp.wrapping_add(0x1fu64)) as u64) as u64; fl_l = (crate::simd_ops::ru64(rbp.wrapping_add(0x37u64))) as u64 as u64; fl_r = (0xfu64) as u64 as u64; rdx = (rdx & !0xffu64) | (((if (((fl_l as u64)) > ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); rcx = (if (((fl_l as u64)) > ((fl_r as u64))) { crate::simd_ops::ru64(rbp.wrapping_add(0x1fu64)) } else { rcx }) as u64; r9 = (crate::simd_ops::ru64(rbp.wrapping_add(0x2fu64))) as u64; r9 = ((r9).wrapping_add(rcx)) as u64; r8 = ((rbp.wrapping_add(0x1fu64)) as u64) as u64; fl_l = ((rdx as u8)) as u8 as u64; fl_r = ((rdx as u8)) as u8 as u64; r8 = (if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { crate::simd_ops::ru64(rbp.wrapping_add(0x1fu64)) } else { r8 }) as u64; rax = (crate::simd_ops::ru64(r10)) as u64; rcx = ((rbp.wrapping_add(0x17u64)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), (rcx) as u64); rcx = ((rbp.wrapping_add(0x8u64)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (rcx) as u64); rcx = ((rbp.wrapping_add(0x7u64)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (rcx) as u64); rcx = ((rbp.wrapping_add(0xfu64)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rcx) as u64); rdx = ((rsi.wrapping_add(0x74u64)) as u64) as u64; rcx = (r10) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(0x30u64))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e1d1e7); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P3)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 16; continue; } bb = 15; continue; }
            15 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x1u32) as u32 as u64; rax = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 20; continue; } bb = 16; continue; }
            16 => { rax = ((rbp.wrapping_add(0x7u64)) as u64) as u64; rbx = (crate::simd_ops::ru64(rbp.wrapping_add(0x2fu64))) as u64; fl_l = (crate::simd_ops::ru64(rbp.wrapping_add(0x17u64))) as u64 as u64; fl_r = (rax) as u64 as u64; rax = ((rbp.wrapping_add(0x1fu64)) as u64) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 22; continue; } bb = 17; continue; }
            17 => { fl_l = (crate::simd_ops::ru64(rbp.wrapping_add(0x37u64))) as u64 as u64; fl_r = (0xfu64) as u64 as u64; rdx = (rdx & !0xffu64) | (((if (((fl_l as u64)) > ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); rax = (if (((fl_l as u64)) > ((fl_r as u64))) { crate::simd_ops::ru64(rbp.wrapping_add(0x1fu64)) } else { rax }) as u64; r9 = (crate::simd_ops::ru64(rbp.wrapping_add(0xfu64))) as u64; r9 = ((r9).wrapping_sub(rax)) as u64; fl_l = (rbx) as u64 as u64; fl_r = (r9) as u64 as u64; r9 = (if (((fl_l as u64)) < ((fl_r as u64))) { rbx } else { r9 }) as u64; rcx = ((rbp.wrapping_add(0x1fu64)) as u64) as u64; fl_l = ((rdx as u8)) as u8 as u64; fl_r = ((rdx as u8)) as u8 as u64; rcx = (if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { crate::simd_ops::ru64(rbp.wrapping_add(0x1fu64)) } else { rcx }) as u64; rbx = ((rbx).wrapping_sub(r9)) as u64; r8 = ((rbx.wrapping_add(0x1u64)) as u64) as u64; rdx = ((rcx.wrapping_add(r9)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e1d239); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu64(rbp.wrapping_add(0x2fu64), (rbx) as u64); rcx = (crate::simd_ops::ru64(rsi.wrapping_add(0x80u64))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e1d249); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P1)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 18; continue; }
            18 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; rdx = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 11; continue; } bb = 19; continue; }
            19 => { bb = 26; continue; }
            20 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x2u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 26; continue; } bb = 21; continue; }
            21 => { rax = ((rbp.wrapping_add(0x1fu64)) as u64) as u64; fl_l = (crate::simd_ops::ru64(rbp.wrapping_add(0x37u64))) as u64 as u64; fl_r = (0xfu64) as u64 as u64; rax = (if (((fl_l as u64)) > ((fl_r as u64))) { crate::simd_ops::ru64(rbp.wrapping_add(0x1fu64)) } else { rax }) as u64; rdi = (((crate::simd_ops::ru8(rax)) as i8 as i32) as u32) as u32 as u64; bb = 26; continue; }
            22 => { fl_l = (crate::simd_ops::ru64(rbp.wrapping_add(0x37u64))) as u64 as u64; fl_r = (0xfu64) as u64 as u64; rax = (if (((fl_l as u64)) > ((fl_r as u64))) { crate::simd_ops::ru64(rbp.wrapping_add(0x1fu64)) } else { rax }) as u64; rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0xfu64))) as u64; rbx = ((rbx).wrapping_sub(rcx)) as u64; rbx = ((rbx).wrapping_add(rax)) as u64; fl_l = (rbx) as u64 as u64; fl_r = (rbx) as u64 as u64; if (((((fl_l as u64)) & ((fl_r as u64))) as i64) <= 0) { bb = 25; continue; } bb = 23; continue; }
            23 => { rbx = ((rbx).wrapping_sub(1)) as u64; rcx = (((crate::simd_ops::ru8(rbx.wrapping_add(rcx))) as i8 as i32) as u32) as u32 as u64; rdx = (crate::simd_ops::ru64(rsi.wrapping_add(0x80u64))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e1d29a); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P5)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; fl_l = (rbx) as u64 as u64; fl_r = (rbx) as u64 as u64; if (((((fl_l as u64)) & ((fl_r as u64))) as i64) <= 0) { bb = 25; continue; } bb = 24; continue; }
            24 => { rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0xfu64))) as u64; bb = 23; continue; }
            25 => { rdi = ((crate::simd_ops::ru8(rbp.wrapping_add(0x7u64))) as u32) as u32 as u64; bb = 26; continue; }
            26 => { rdx = (crate::simd_ops::ru64(rbp.wrapping_add(0x37u64))) as u64; fl_l = (rdx) as u64 as u64; fl_r = (0xfu64) as u64 as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 31; continue; } bb = 27; continue; }
            27 => { rdx = ((rdx).wrapping_add(1)) as u64; rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x1fu64))) as u64; rax = (rcx) as u64; fl_l = (rdx) as u64 as u64; fl_r = (0x1000u64) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 30; continue; } bb = 28; continue; }
            28 => { rdx = ((rdx).wrapping_add(0x27u64)) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_sub(0x8u64))) as u64; rax = ((rax).wrapping_sub(rcx)) as u64; rax = ((rax).wrapping_add(0xfffffffffffffff8u64)) as u64; fl_l = (rax) as u64 as u64; fl_r = (0x1fu64) as u64 as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 30; continue; } bb = 29; continue; }
            29 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e1d2e1); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P6)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::lift_ops::trap(0x80000003, P7, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            30 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e1d2e8); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P8) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 31; continue; }
            31 => { rax = ((rdi as u32)) as u32 as u64; bb = 32; continue; }
            32 => { rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x3fu64))) as u64; fl_l = (((rcx) ^ (rsp))) as u64 as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; r11 = ((rsp.wrapping_add(0x90u64)) as u64) as u64; rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x28u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x30u64))) as u64; rsp = (r11) as u64; r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rbp = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e1d320(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x30; const P1: u64 = 0x7001e696a0; const P2: u64 = 0x50; const P3: u64 = 0x38; const P4: u64 = 0x1;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    rbx = (rcx) as u64;
    rax = (crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P1)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    rdx = ((((rdx as u32)) | (0xffffffffu32))) as u32 as u64;
    fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64;
    if !((((fl_l as u32)) == ((fl_r as u32)))) { rax = (crate::simd_ops::ru64(rbx.wrapping_add(P2))) as u64; fl_l = (crate::simd_ops::lk_add_u32(rax, ((rdx as u32)) as u32)) as u32 as u64; fl_r = fl_l; rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P3))) as u64; rdx = (crate::simd_ops::ru64(rcx)) as u64; rax = ((rdx.wrapping_add(P4)) as u64) as u64; crate::simd_ops::wu64(rcx, (rax) as u64); rdx = ((crate::simd_ops::ru8(rdx)) as u32) as u32 as u64; }
    rax = ((rdx as u32)) as u32 as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = core::ptr::read_unaligned(rsp as *const u64);
    rsp = rsp.wrapping_add(8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e1d360(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x38; const P1: u64 = 0x50; const P2: u64 = 0x7001e696a0; const P3: u64 = 0x20;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rbx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rbx = (rcx) as u64; r8 = (crate::simd_ops::ru64(rax)) as u64; fl_l = (r8) as u64 as u64; fl_r = (r8) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; rdx = (((crate::simd_ops::ru32(rax)) as i32 as i64) as u64) as u64; rdx = ((rdx).wrapping_add(r8)) as u64; fl_l = (r8) as u64 as u64; fl_r = (rdx) as u64 as u64; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 3; continue; } bb = 2; continue; }
            2 => { rax = ((crate::simd_ops::ru8(r8)) as u32) as u32 as u64; bb = 5; continue; }
            3 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P2)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rdi = ((rax as u32)) as u32 as u64; rax = ((((rax as u32)) | (0xffffffffu32))) as u32 as u64; fl_l = ((rdi as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = ((rdi as u32)) as u32 as u64; rcx = (rbx) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(P3))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P2)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rax = ((rdi as u32)) as u32 as u64; bb = 5; continue; }
            5 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e1d3d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x18;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rdx = (!((rdx as u32))) as u32 as u64;
    fl_l = (crate::simd_ops::lk_and_u32(rcx.wrapping_add(P0), ((rdx as u32)) as u32)) as u32 as u64; fl_r = fl_l;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e1d400(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x40; const P1: u64 = 0x8; const P2: u64 = 0x7001e696a0; const P3: u64 = 0x7001e17a54; const P4: u64 = 0x10;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&super::EH_META[68], rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rbx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rbx = (rbx & !0xffu64) | ((((rdx as u8)) as u8 as u64)); rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rdi = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (rdi) as u64); rax = (crate::simd_ops::ru64(rdi)) as u64; rcx = (rdi) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e1d423); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P2)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rcx = ((rsp.wrapping_add(0x20u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e1d42f); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); r8 = (rax) as u64; rcx = (crate::simd_ops::ru64(rax)) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rdx = (rdx & !0xffu64) | ((((rbx as u8)) as u8 as u64)); rcx = (r8) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e1d443); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P2)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rbx = (rbx & !0xffu64) | ((((rax as u8)) as u8 as u64)); fl_l = (rdi) as u64 as u64; fl_r = (rdi) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { rdx = (crate::simd_ops::ru64(rdi)) as u64; rcx = (rdi) as u64; rax = (crate::simd_ops::ru64(rdx.wrapping_add(P4))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e1d45a); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P2)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); r8 = (rax) as u64; fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { rcx = (crate::simd_ops::ru64(rax)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = (0x1u32) as u32 as u64; rcx = (r8) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e1d476); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P2)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 3; continue; }
            3 => { rax = (rax & !0xffu64) | ((((rbx as u8)) as u8 as u64)); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e1d500(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001e49e20; const P1: u64 = 0x7001e99050; const P2: u64 = 0x1; const P3: u64 = 0x7001e49ec0;
    let (mut rax, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rdx = (0x2u32) as u32 as u64;
    rcx = ((rsp.wrapping_add(0x30u64)) as u64) as u64;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    rbx = (crate::simd_ops::ru32(P1)) as u32 as u64;
    rcx = ((rbx.wrapping_add(P2)) as u32) as u32 as u64;
    crate::simd_ops::wu32(P1, ((rcx as u32)) as u32);
    rcx = ((rsp.wrapping_add(0x30u64)) as u64) as u64;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    rax = ((rbx as u32)) as u32 as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = core::ptr::read_unaligned(rsp as *const u64);
    rsp = rsp.wrapping_add(8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e1d540(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001e1d680; const P1: u64 = 0x7001e66450; const P2: u64 = 0x7001e69560;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = (rsp) as u64; crate::simd_ops::wu64(rax.wrapping_add(0x8u64), (rbx) as u64); crate::simd_ops::wu64(rax.wrapping_add(0x10u64), (rbp) as u64); crate::simd_ops::wu64(rax.wrapping_add(0x18u64), (rsi) as u64); crate::simd_ops::wu64(rax.wrapping_add(0x20u64), (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbp = (r8) as u64; r14 = (rdx) as u64; rdi = (rcx) as u64; fl_l = (r8) as u64 as u64; fl_r = (r8) as u64 as u64; if (((((fl_l as u64)) & ((fl_r as u64))) as i64) > 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; bb = 18; continue; }
            2 => { fl_l = (crate::simd_ops::ru64(rcx.wrapping_add(0x68u64))) as u64 as u64; fl_r = (0x0u64) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 4; continue; } bb = 3; continue; }
            3 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 18; continue; }
            4 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x38u64))) as u64; rsi = (rbp) as u64; rdx = (crate::simd_ops::ru64(rax)) as u64; fl_l = (rdx) as u64 as u64; fl_r = (rdx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x50u64))) as u64; rcx = (crate::simd_ops::ru32(rax)) as u32 as u64; bb = 7; continue; }
            6 => { rcx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; bb = 7; continue; }
            7 => { rax = ((((rcx as u32)) as i32 as i64) as u64) as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 9; continue; } bb = 8; continue; }
            8 => { fl_l = (rax) as u64 as u64; fl_r = (rbp) as u64 as u64; rbx = (rbp) as u64; rcx = (r14) as u64; rbx = (if (((fl_l as u64)) < ((fl_r as u64))) { rax } else { rbx }) as u64; r8 = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x50u64))) as u64; r14 = ((r14).wrapping_add(rbx)) as u64; rsi = ((rsi).wrapping_sub(rbx)) as u64; crate::simd_ops::lk_sub_u32(rax, ((rbx as u32)) as u32); rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x38u64))) as u64; rax = ((((rbx as u32)) as i32 as i64) as u64) as u64; fl_l = (crate::simd_ops::lk_add_u64(rcx, (rax) as u64)) as u64; fl_r = fl_l; bb = 9; continue; }
            9 => { fl_l = (crate::simd_ops::ru64(rdi.wrapping_add(0x80u64))) as u64 as u64; fl_r = (0x0u64) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 17; continue; } bb = 10; continue; }
            10 => { r8 = (crate::simd_ops::ru64(rdi.wrapping_add(0x18u64))) as u64; rax = ((rdi.wrapping_add(0x70u64)) as u64) as u64; fl_l = (crate::simd_ops::ru64(r8)) as u64 as u64; fl_r = (rax) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 12; continue; } bb = 11; continue; }
            11 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x88u64))) as u64; rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0x90u64))) as u64; crate::simd_ops::wu64(r8, (rcx) as u64); fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; rdx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x38u64))) as u64; crate::simd_ops::wu64(rax, (rcx) as u64); rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x50u64))) as u64; crate::simd_ops::wu32(rax, ((rdx as u32)) as u32); bb = 12; continue; }
            12 => { rbx = (0xfffu32) as u32 as u64; bb = 14; continue; }
            13 => { r9 = (crate::simd_ops::ru64(rdi.wrapping_add(0x80u64))) as u64; r8 = (rbx) as u64; rdx = (0x1u32) as u32 as u64; rcx = (r14) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P2)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); r14 = ((r14).wrapping_add(rax)) as u64; rsi = ((rsi).wrapping_sub(rax)) as u64; fl_l = (rax) as u64 as u64; fl_r = (rbx) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 17; continue; } bb = 14; continue; }
            14 => { fl_l = (rsi) as u64 as u64; fl_r = (rbx) as u64 as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 13; continue; } bb = 15; continue; }
            15 => { fl_l = (rsi) as u64 as u64; fl_r = (rsi) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 17; continue; } bb = 16; continue; }
            16 => { r9 = (crate::simd_ops::ru64(rdi.wrapping_add(0x80u64))) as u64; r8 = (rsi) as u64; rdx = (0x1u32) as u32 as u64; rcx = (r14) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P2)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); fl_l = (rsi) as u64 as u64; fl_r = (rax) as u64 as u64; rsi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; bb = 17; continue; }
            17 => { fl_l = (rbp) as u64 as u64; fl_r = (rsi) as u64 as u64; rbp = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; rax = (rbp) as u64; bb = 18; continue; }
            18 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e1d680(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0x18; const P3: u64 = 0x20; const P4: u64 = 0x38; const P5: u64 = 0x50; const P6: u64 = 0x7001e66450; const P7: u64 = 0x7001e696a0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = (rsp) as u64; crate::simd_ops::wu64(rax.wrapping_add(P0), (rbx) as u64); crate::simd_ops::wu64(rax.wrapping_add(P1), (rbp) as u64); crate::simd_ops::wu64(rax.wrapping_add(P2), (rsi) as u64); crate::simd_ops::wu64(rax.wrapping_add(P3), (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r12) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r15) as u64); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = (r8) as u64; r14 = (rdx) as u64; rdi = (rcx) as u64; rsi = (r8) as u64; fl_l = (r8) as u64 as u64; fl_r = (r8) as u64 as u64; if (((((fl_l as u64)) & ((fl_r as u64))) as i64) <= 0) { bb = 9; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(P4))) as u64; r15 = ((rdi.wrapping_add(P5)) as u64) as u64; rdx = (crate::simd_ops::ru64(rax)) as u64; fl_l = (rdx) as u64 as u64; fl_r = (rdx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(r15)) as u64; rcx = (crate::simd_ops::ru32(rax)) as u32 as u64; bb = 4; continue; }
            3 => { rcx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; bb = 4; continue; }
            4 => { rax = ((((rcx as u32)) as i32 as i64) as u64) as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { fl_l = (rbx) as u64 as u64; fl_r = (rax) as u64 as u64; rbp = (rbx) as u64; rcx = (r14) as u64; rbp = (if ((((fl_l as u64)) as i64) >= (((fl_r as u64)) as i64)) { rax } else { rbp }) as u64; r8 = (rbp) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rax = (crate::simd_ops::ru64(r15)) as u64; rbx = ((rbx).wrapping_sub(rbp)) as u64; crate::simd_ops::lk_sub_u32(rax, ((rbp as u32)) as u32); rcx = (crate::simd_ops::ru64(rdi.wrapping_add(P4))) as u64; rax = ((((rbp as u32)) as i32 as i64) as u64) as u64; fl_l = (crate::simd_ops::lk_add_u64(rcx, (rax) as u64)) as u64; fl_r = fl_l; bb = 8; continue; }
            6 => { rax = (crate::simd_ops::ru64(rdi)) as u64; rcx = (rdi) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(P4))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P7)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); fl_l = ((rax as u32)) as u32 as u64; fl_r = (0xffffffffu32) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 9; continue; } bb = 7; continue; }
            7 => { crate::simd_ops::wu8(r14, ((rax as u8)) as u8); fl_l = ((rbx).wrapping_sub(1)) as u64 as u64; rbx = ((fl_l as u64)) as u64; rbp = (0x1u32) as u32 as u64; bb = 8; continue; }
            8 => { r14 = ((r14).wrapping_add(rbp)) as u64; fl_l = (rbx) as u64 as u64; fl_r = (rbx) as u64 as u64; if (((((fl_l as u64)) & ((fl_r as u64))) as i64) > 0) { bb = 1; continue; } bb = 9; continue; }
            9 => { rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsi = ((rsi).wrapping_sub(rbx)) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rax = (rsi) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r12 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e1d750(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x68; const P1: u64 = 0x7001e1d810; const P2: u64 = 0x40; const P3: u64 = 0x58; const P4: u64 = 0x7001e66450; const P5: u64 = 0x80; const P6: u64 = 0x7001e69578;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rbp) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r15) as u64); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; fl_l = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64 as u64; fl_r = (0x0u64) as u64 as u64; rbx = (r8) as u64; r15 = (rdx) as u64; rdi = (rcx) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 11; continue; }
            2 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(P2))) as u64; rsi = (rbx) as u64; r9 = (crate::simd_ops::ru64(rax)) as u64; fl_l = (r9) as u64 as u64; fl_r = (r9) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(P3))) as u64; rcx = (crate::simd_ops::ru32(rax)) as u32 as u64; bb = 5; continue; }
            4 => { rcx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; bb = 5; continue; }
            5 => { rbp = ((((rcx as u32)) as i32 as i64) as u64) as u64; fl_l = (rbx) as u64 as u64; fl_r = (rbx) as u64 as u64; if (((((fl_l as u64)) & ((fl_r as u64))) as i64) <= 0) { bb = 10; continue; } bb = 6; continue; }
            6 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 8; continue; } bb = 7; continue; }
            7 => { fl_l = (rbx) as u64 as u64; fl_r = (rbp) as u64 as u64; rcx = (r9) as u64; rbp = (if ((((fl_l as u64)) as i64) < (((fl_r as u64)) as i64)) { rbx } else { rbp }) as u64; r8 = (rbp) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rax = (crate::simd_ops::ru64(rdi.wrapping_add(P3))) as u64; r15 = ((r15).wrapping_add(rbp)) as u64; rbx = ((rbx).wrapping_sub(rbp)) as u64; crate::simd_ops::lk_sub_u32(rax, ((rbp as u32)) as u32); rcx = (crate::simd_ops::ru64(rdi.wrapping_add(P2))) as u64; rax = ((((rbp as u32)) as i32 as i64) as u64) as u64; crate::simd_ops::lk_add_u64(rcx, (rax) as u64); fl_l = (rbx) as u64 as u64; fl_r = (rbx) as u64 as u64; if (((((fl_l as u64)) & ((fl_r as u64))) as i64) <= 0) { bb = 10; continue; } bb = 8; continue; }
            8 => { r9 = (crate::simd_ops::ru64(rdi.wrapping_add(P5))) as u64; fl_l = (r9) as u64 as u64; fl_r = (r9) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 9; continue; }
            9 => { r8 = (rbx) as u64; rdx = (0x1u32) as u32 as u64; rcx = (r15) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P6)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); fl_l = (rbx) as u64 as u64; fl_r = (rax) as u64 as u64; rbx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; bb = 10; continue; }
            10 => { fl_l = (rsi) as u64 as u64; fl_r = (rbx) as u64 as u64; rsi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; rax = (rsi) as u64; bb = 11; continue; }
            11 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e1d810(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0x18; const P3: u64 = 0x20; const P4: u64 = 0x40; const P5: u64 = 0x58; const P6: u64 = 0x7001e66450; const P7: u64 = 0x7001e696a0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = (rsp) as u64; crate::simd_ops::wu64(rax.wrapping_add(P0), (rbx) as u64); crate::simd_ops::wu64(rax.wrapping_add(P1), (rbp) as u64); crate::simd_ops::wu64(rax.wrapping_add(P2), (rsi) as u64); crate::simd_ops::wu64(rax.wrapping_add(P3), (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r12) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r15) as u64); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = (r8) as u64; r14 = (rdx) as u64; rdi = (rcx) as u64; rsi = (r8) as u64; fl_l = (r8) as u64 as u64; fl_r = (r8) as u64 as u64; if (((((fl_l as u64)) & ((fl_r as u64))) as i64) <= 0) { bb = 9; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(P4))) as u64; r15 = ((rdi.wrapping_add(P5)) as u64) as u64; r9 = (crate::simd_ops::ru64(rax)) as u64; fl_l = (r9) as u64 as u64; fl_r = (r9) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(r15)) as u64; rcx = (crate::simd_ops::ru32(rax)) as u32 as u64; bb = 4; continue; }
            3 => { rcx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; bb = 4; continue; }
            4 => { rax = ((((rcx as u32)) as i32 as i64) as u64) as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { fl_l = (rbx) as u64 as u64; fl_r = (rax) as u64 as u64; rbp = (rbx) as u64; rdx = (r14) as u64; rcx = (r9) as u64; rbp = (if ((((fl_l as u64)) as i64) >= (((fl_r as u64)) as i64)) { rax } else { rbp }) as u64; r8 = (rbp) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rax = (crate::simd_ops::ru64(r15)) as u64; rbx = ((rbx).wrapping_sub(rbp)) as u64; crate::simd_ops::lk_sub_u32(rax, ((rbp as u32)) as u32); rcx = (crate::simd_ops::ru64(rdi.wrapping_add(P4))) as u64; rax = ((((rbp as u32)) as i32 as i64) as u64) as u64; fl_l = (crate::simd_ops::lk_add_u64(rcx, (rax) as u64)) as u64; fl_r = fl_l; bb = 8; continue; }
            6 => { rax = (crate::simd_ops::ru64(rdi)) as u64; rcx = (rdi) as u64; rdx = ((crate::simd_ops::ru8(r14)) as u32) as u32 as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P7)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); fl_l = ((rax as u32)) as u32 as u64; fl_r = (0xffffffffu32) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 9; continue; } bb = 7; continue; }
            7 => { fl_l = ((rbx).wrapping_sub(1)) as u64 as u64; rbx = ((fl_l as u64)) as u64; rbp = (0x1u32) as u32 as u64; bb = 8; continue; }
            8 => { r14 = ((r14).wrapping_add(rbp)) as u64; fl_l = (rbx) as u64 as u64; fl_r = (rbx) as u64 as u64; if (((((fl_l as u64)) & ((fl_r as u64))) as i64) > 0) { bb = 1; continue; } bb = 9; continue; }
            9 => { rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsi = ((rsi).wrapping_sub(rbx)) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rax = (rsi) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r12 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e1d8e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001e6be20; const P1: u64 = 0x20; const P2: u64 = 0x28; const P3: u64 = 0x30; const P4: u64 = 0x38; const P5: u64 = 0x40; const P6: u64 = 0x48; const P7: u64 = 0x50; const P8: u64 = 0x58; const P9: u64 = 0x7001e6bc80; const P10: u64 = 0x18; const P11: u64 = 0x60; const P12: u64 = 0x7001e6be18; const P13: u64 = 0x7001e1c510;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&super::EH_META[69], rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rcx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; r10 = (rdx) as u64; rbx = (rcx) as u64; r11 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rsp.wrapping_add(0x48u64), ((r11 as u32)) as u32); fl_l = ((r9 as u32)) as u32 as u64; fl_r = ((r9 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = ((P0) as u64) as u64; crate::simd_ops::wu64(rcx, (rax) as u64); crate::simd_ops::wu64(rcx.wrapping_add(P1), (r11) as u64); crate::simd_ops::wu64(rcx.wrapping_add(P2), (r11) as u64); crate::simd_ops::wu32(rcx.wrapping_add(P3), ((r11 as u32)) as u32); crate::simd_ops::wu64(rcx.wrapping_add(P4), (r11) as u64); crate::simd_ops::wu64(rcx.wrapping_add(P5), (r11) as u64); crate::simd_ops::wu64(rcx.wrapping_add(P6), (r11) as u64); crate::simd_ops::wu64(rcx.wrapping_add(P7), (r11) as u64); crate::simd_ops::wu64(rcx.wrapping_add(P8), (r11) as u64); rax = ((P9) as u64) as u64; crate::simd_ops::wu64(rcx.wrapping_add(P10), (rax) as u64); crate::simd_ops::wu64(rcx.wrapping_add(P11), (r11) as u64); crate::simd_ops::wu64(rcx.wrapping_add(0x68u64), (r11) as u64); crate::simd_ops::wu8(rcx.wrapping_add(0x70u64), ((r11 as u8)) as u8); crate::simd_ops::wu32(rsp.wrapping_add(0x48u64), (0x1u32) as u32); bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rcx = (((crate::simd_ops::ru32(rax.wrapping_add(0x4u64))) as i32 as i64) as u64) as u64; rax = ((P12) as u64) as u64; crate::simd_ops::wu64(rcx.wrapping_add(rbx), (rax) as u64); rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = (((crate::simd_ops::ru32(rax.wrapping_add(0x4u64))) as i32 as i64) as u64) as u64; rdx = ((rcx.wrapping_sub(0x18u64)) as u32) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(rbx).wrapping_sub(0x4u64), ((rdx as u32)) as u32); crate::simd_ops::wu64(rbx.wrapping_add(0x8u64), (r11) as u64); rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = (((crate::simd_ops::ru32(rax.wrapping_add(0x4u64))) as i32 as i64) as u64) as u64; rcx = ((rcx).wrapping_add(rbx)) as u64; rdx = (r10) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e1d977); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P13) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rax = (rbx) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e1d9c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x4; const P1: u64 = 0x7001e6be18; const P2: u64 = 0xffffffffffffffe8; const P3: u64 = 0xfffffffffffffffc; const P4: u64 = 0x7001e6bc08; const P5: u64 = 0x18; const P6: u64 = 0x7001e1efd0;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&super::EH_META[70], rsp); let _eh = eh.push();
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    rdx = (((crate::simd_ops::ru32(rax.wrapping_add(P0))) as i32 as i64) as u64) as u64;
    rax = ((P1) as u64) as u64;
    crate::simd_ops::wu64(rdx.wrapping_add(rcx), (rax) as u64);
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    rdx = (((crate::simd_ops::ru32(rax.wrapping_add(P0))) as i32 as i64) as u64) as u64;
    r8 = ((rdx.wrapping_add(P2)) as u32) as u32 as u64;
    crate::simd_ops::wu32(rdx.wrapping_add(rcx).wrapping_add(P3), ((r8 as u32)) as u32);
    rax = ((P4) as u64) as u64;
    crate::simd_ops::wu64(rcx.wrapping_add(P5), (rax) as u64);
    rcx = ((rcx).wrapping_add(0x18u64)) as u64;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    eh.site(0x7001e1d9f5); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e1da0c(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001e66450; const P1: u64 = 0x7001e17310; const P2: u64 = 0x7001e659fc; const P3: u64 = 0x7001e1a9d8; const P4: u64 = 0x7001e1db0b; const P5: u64 = 0x7001e1abbc; const P6: u64 = 0x7001e1db11;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rbp) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r15) as u64); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; r14 = (0x7ffffffffffffffeu64) as u64; rbx = (r8) as u64; r15 = (rdx) as u64; rdi = (rcx) as u64; fl_l = (r8) as u64 as u64; fl_r = (r14) as u64 as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 15; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rcx.wrapping_add(0x18u64), (0x7u64) as u64); fl_l = (rbx) as u64 as u64; fl_r = (0x7u64) as u64 as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 3; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::wu64(rcx.wrapping_add(0x10u64), (rbx) as u64); rbx = ((rbx).wrapping_add(rbx)) as u64; r8 = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rsi = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; crate::simd_ops::wu16(rbx.wrapping_add(rdi), ((rsi as u16)) as u16); bb = 13; continue; }
            3 => { rax = (rbx) as u64; rsi = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rax = (((rax) | (0x7u64))) as u64; fl_l = (rax) as u64 as u64; fl_r = (r14) as u64 as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = (0x7fffffffffffffffu64) as u64; rcx = ((rax.wrapping_add(rax)) as u64) as u64; bb = 7; continue; }
            5 => { rcx = (0xau32) as u32 as u64; r14 = (rax) as u64; fl_l = (rax) as u64 as u64; fl_r = (rcx) as u64 as u64; rax = (0x7fffffffffffffffu64) as u64; r14 = (if (((fl_l as u64)) < ((fl_r as u64))) { rcx } else { r14 }) as u64; rcx = ((r14.wrapping_add(0x1u64)) as u64) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rax) as u64 as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 14; continue; } bb = 6; continue; }
            6 => { rcx = ((rcx).wrapping_add(rcx)) as u64; fl_l = (rcx) as u64 as u64; fl_r = (0x1000u64) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 9; continue; } bb = 7; continue; }
            7 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 8; continue; }
            8 => { rbp = (rax) as u64; bb = 12; continue; }
            9 => { fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 10; continue; }
            10 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 8; continue; }
            11 => { rbp = (rsi) as u64; bb = 12; continue; }
            12 => { crate::simd_ops::wu64(rdi.wrapping_add(0x10u64), (rbx) as u64); rdx = (r15) as u64; rbx = ((rbx).wrapping_add(rbx)) as u64; crate::simd_ops::wu64(rdi, (rbp) as u64); r8 = (rbx) as u64; crate::simd_ops::wu64(rdi.wrapping_add(0x18u64), (r14) as u64); rcx = (rbp) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); crate::simd_ops::wu16(rbx.wrapping_add(rbp), ((rsi as u16)) as u16); bb = 13; continue; }
            13 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            14 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); crate::lift_ops::trap(0x80000003, P4, rcx, rsp); return rax; }
            15 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); crate::lift_ops::trap(0x80000003, P6, rcx, rsp); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e1db14(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001e17310; const P1: u64 = 0x7001e659fc; const P2: u64 = 0x7001e66450; const P3: u64 = 0x7001e6649c; const P4: u64 = 0x7001e69480; const P5: u64 = 0x7001e1dc53; const P6: u64 = 0x7001e1a9d8; const P7: u64 = 0x7001e1dc99; const P8: u64 = 0x7001e1abbc; const P9: u64 = 0x7001e1dc9f;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rbp) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r12) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r13) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r15) as u64); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; r14 = (crate::simd_ops::ru64(rcx.wrapping_add(0x10u64))) as u64; rbx = (0x7ffffffffffffffeu64) as u64; rax = (rbx) as u64; r13 = (r9) as u64; rax = ((rax).wrapping_sub(r14)) as u64; rsi = (rcx) as u64; fl_l = (rax) as u64 as u64; fl_r = (rdx) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 20; continue; } bb = 1; continue; }
            1 => { rbp = (crate::simd_ops::ru64(rcx.wrapping_add(0x18u64))) as u64; r15 = ((r14.wrapping_add(rdx)) as u64) as u64; rdx = (r15) as u64; r8 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rdx = (((rdx) | (0x7u64))) as u64; fl_l = (rdx) as u64 as u64; fl_r = (rbx) as u64 as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 3; continue; } bb = 2; continue; }
            2 => { rcx = (rbp) as u64; rax = (rbx) as u64; rcx = ((rcx).wrapping_shr(1u32)) as u64; rax = ((rax).wrapping_sub(rcx)) as u64; fl_l = (rbp) as u64 as u64; fl_r = (rax) as u64 as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rax = (0x7fffffffffffffffu64) as u64; rcx = ((rax.wrapping_add(rax)) as u64) as u64; bb = 6; continue; }
            4 => { rax = ((rcx.wrapping_add(rbp)) as u64) as u64; rbx = (rdx) as u64; fl_l = (rdx) as u64 as u64; fl_r = (rax) as u64 as u64; rbx = (if (((fl_l as u64)) < ((fl_r as u64))) { rax } else { rbx }) as u64; rax = (0x7fffffffffffffffu64) as u64; rcx = ((rbx.wrapping_add(0x1u64)) as u64) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rax) as u64 as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 19; continue; } bb = 5; continue; }
            5 => { rcx = ((rcx).wrapping_add(rcx)) as u64; fl_l = (rcx) as u64 as u64; fl_r = (0x1000u64) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 8; continue; } bb = 6; continue; }
            6 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 7; continue; }
            7 => { rdi = (rax) as u64; bb = 11; continue; }
            8 => { fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 9; continue; }
            9 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 7; continue; }
            10 => { rdi = (r8) as u64; bb = 11; continue; }
            11 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; r8 = ((r14.wrapping_add(r14)) as u64) as u64; r14 = ((r14).wrapping_add(rax)) as u64; crate::simd_ops::wu64(rsi.wrapping_add(0x10u64), (r15) as u64); crate::simd_ops::wu64(rsi.wrapping_add(0x18u64), (rbx) as u64); r12 = ((r8.wrapping_add(rdi)) as u64) as u64; rcx = (rdi) as u64; r15 = ((rax.wrapping_add(rax)) as u64) as u64; fl_l = (rbp) as u64 as u64; fl_r = (0x7u64) as u64 as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 17; continue; } bb = 12; continue; }
            12 => { rbx = (crate::simd_ops::ru64(rsi)) as u64; rdx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); r8 = (r15) as u64; rdx = (r13) as u64; rcx = (r12) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rax = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rdx = ((rbp.wrapping_mul(2).wrapping_add(0x2u64)) as u64) as u64; crate::simd_ops::wu16(rdi.wrapping_add(r14.wrapping_mul(2)), ((rax as u16)) as u16); fl_l = (rdx) as u64 as u64; fl_r = (0x1000u64) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 15; continue; } bb = 13; continue; }
            13 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_sub(0x8u64))) as u64; rdx = ((rdx).wrapping_add(0x27u64)) as u64; rbx = ((rbx).wrapping_sub(rcx)) as u64; rax = ((rbx.wrapping_sub(0x8u64)) as u64) as u64; fl_l = (rax) as u64 as u64; fl_r = (0x1fu64) as u64 as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 16; continue; } bb = 14; continue; }
            14 => { rbx = (rcx) as u64; bb = 15; continue; }
            15 => { rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 18; continue; }
            16 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P4)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); crate::lift_ops::trap(0x80000003, P5, rcx, rsp); return rax; }
            17 => { rdx = (rsi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); r8 = (r15) as u64; rdx = (r13) as u64; rcx = (r12) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rax = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; crate::simd_ops::wu16(rdi.wrapping_add(r14.wrapping_mul(2)), ((rax as u16)) as u16); bb = 18; continue; }
            18 => { crate::simd_ops::wu64(rsi, (rdi) as u64); rax = (rsi) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r13 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r12 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            19 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); crate::lift_ops::trap(0x80000003, P7, rcx, rsp); return rax; }
            20 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P8) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); crate::lift_ops::trap(0x80000003, P9, rcx, rsp); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e1dca0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001e17310; const P1: u64 = 0x7001e659fc; const P2: u64 = 0x7001e66450; const P3: u64 = 0x7001e6649c; const P4: u64 = 0x7001e69480; const P5: u64 = 0x7001e1ddc8; const P6: u64 = 0x7001e1a9d8; const P7: u64 = 0x7001e1de04; const P8: u64 = 0x7001e1abbc; const P9: u64 = 0x7001e1de0a;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rbp) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r12) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r13) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r15) as u64); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbp = (crate::simd_ops::ru64(rcx.wrapping_add(0x10u64))) as u64; rbx = (0x7ffffffffffffffeu64) as u64; rax = (rbx) as u64; r15 = (((r9 as u16)) as u32) as u32 as u64; rax = ((rax).wrapping_sub(rbp)) as u64; rsi = (rcx) as u64; fl_l = (rax) as u64 as u64; fl_r = (rdx) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 20; continue; } bb = 1; continue; }
            1 => { r14 = (crate::simd_ops::ru64(rcx.wrapping_add(0x18u64))) as u64; r12 = ((rdx.wrapping_add(rbp)) as u64) as u64; rdx = (r12) as u64; r13 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rdx = (((rdx) | (0x7u64))) as u64; fl_l = (rdx) as u64 as u64; fl_r = (rbx) as u64 as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 3; continue; } bb = 2; continue; }
            2 => { rcx = (r14) as u64; rax = (rbx) as u64; rcx = ((rcx).wrapping_shr(1u32)) as u64; rax = ((rax).wrapping_sub(rcx)) as u64; fl_l = (r14) as u64 as u64; fl_r = (rax) as u64 as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rax = (0x7fffffffffffffffu64) as u64; rcx = ((rax.wrapping_add(rax)) as u64) as u64; bb = 6; continue; }
            4 => { rax = ((rcx.wrapping_add(r14)) as u64) as u64; rbx = (rdx) as u64; fl_l = (rdx) as u64 as u64; fl_r = (rax) as u64 as u64; rbx = (if (((fl_l as u64)) < ((fl_r as u64))) { rax } else { rbx }) as u64; rax = (0x7fffffffffffffffu64) as u64; rcx = ((rbx.wrapping_add(0x1u64)) as u64) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rax) as u64 as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 19; continue; } bb = 5; continue; }
            5 => { rcx = ((rcx).wrapping_add(rcx)) as u64; fl_l = (rcx) as u64 as u64; fl_r = (0x1000u64) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 8; continue; } bb = 6; continue; }
            6 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 7; continue; }
            7 => { rdi = (rax) as u64; bb = 11; continue; }
            8 => { fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 9; continue; }
            9 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 7; continue; }
            10 => { rdi = (r13) as u64; bb = 11; continue; }
            11 => { rbp = ((rbp).wrapping_add(rbp)) as u64; crate::simd_ops::wu64(rsi.wrapping_add(0x10u64), (r12) as u64); crate::simd_ops::wu64(rsi.wrapping_add(0x18u64), (rbx) as u64); r8 = (rbp) as u64; rcx = (rdi) as u64; fl_l = (r14) as u64 as u64; fl_r = (0x7u64) as u64 as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 17; continue; } bb = 12; continue; }
            12 => { rbx = (crate::simd_ops::ru64(rsi)) as u64; rdx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rdx = ((r14.wrapping_mul(2).wrapping_add(0x2u64)) as u64) as u64; crate::simd_ops::wu16(rdi.wrapping_add(rbp), ((r15 as u16)) as u16); crate::simd_ops::wu16(rdi.wrapping_add(rbp).wrapping_add(0x2u64), ((r13 as u16)) as u16); fl_l = (rdx) as u64 as u64; fl_r = (0x1000u64) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 15; continue; } bb = 13; continue; }
            13 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_sub(0x8u64))) as u64; rdx = ((rdx).wrapping_add(0x27u64)) as u64; rbx = ((rbx).wrapping_sub(rcx)) as u64; rax = ((rbx.wrapping_sub(0x8u64)) as u64) as u64; fl_l = (rax) as u64 as u64; fl_r = (0x1fu64) as u64 as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 16; continue; } bb = 14; continue; }
            14 => { rbx = (rcx) as u64; bb = 15; continue; }
            15 => { rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 18; continue; }
            16 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P4)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); crate::lift_ops::trap(0x80000003, P5, rcx, rsp); return rax; }
            17 => { rdx = (rsi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); crate::simd_ops::wu16(rdi.wrapping_add(rbp), ((r15 as u16)) as u16); crate::simd_ops::wu16(rdi.wrapping_add(rbp).wrapping_add(0x2u64), ((r13 as u16)) as u16); bb = 18; continue; }
            18 => { crate::simd_ops::wu64(rsi, (rdi) as u64); rax = (rsi) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r13 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r12 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            19 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); crate::lift_ops::trap(0x80000003, P7, rcx, rsp); return rax; }
            20 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P8) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); crate::lift_ops::trap(0x80000003, P9, rcx, rsp); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e1de14(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x18; const P1: u64 = 0x2; const P2: u64 = 0xfffffffffffffff8; const P3: u64 = 0x7001e6649c; const P4: u64 = 0x10; const P5: u64 = 0x7001e69480; const P6: u64 = 0x7001e1de75;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rbx = (rcx) as u64; fl_l = (rdx) as u64 as u64; fl_r = (0x7u64) as u64 as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 5; continue; } bb = 1; continue; }
            1 => { rcx = (crate::simd_ops::ru64(rcx)) as u64; rdx = ((rdx.wrapping_mul(2).wrapping_add(P1)) as u64) as u64; fl_l = (rdx) as u64 as u64; fl_r = (0x1000u64) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 4; continue; } bb = 2; continue; }
            2 => { r8 = (crate::simd_ops::ru64(rcx.wrapping_add(P2))) as u64; rdx = ((rdx).wrapping_add(0x27u64)) as u64; rcx = ((rcx).wrapping_sub(r8)) as u64; rax = ((rcx.wrapping_add(P2)) as u64) as u64; fl_l = (rax) as u64 as u64; fl_r = (0x1fu64) as u64 as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 6; continue; } bb = 3; continue; }
            3 => { rcx = (r8) as u64; bb = 4; continue; }
            4 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 5; continue; }
            5 => { rax = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rbx.wrapping_add(P0), (0x7u64) as u64); crate::simd_ops::wu64(rbx.wrapping_add(P4), (rax) as u64); crate::simd_ops::wu16(rbx, ((rax as u16)) as u16); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            6 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P5)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); crate::lift_ops::trap(0x80000003, P6, rcx, rsp); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e1de78(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x18; const P2: u64 = 0x7001e6646c; const P3: u64 = 0x7001e1db14;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rbx = (rcx) as u64; r10 = (r8) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rax = (crate::simd_ops::ru64(rbx.wrapping_add(P1))) as u64; rax = ((rax).wrapping_sub(rcx)) as u64; fl_l = (r8) as u64 as u64; fl_r = (rax) as u64 as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 4; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru64(rbx.wrapping_add(P1))) as u64 as u64; fl_r = (0x7u64) as u64 as u64; rsi = ((rcx.wrapping_add(r8)) as u64) as u64; crate::simd_ops::wu64(rbx.wrapping_add(P0), (rsi) as u64); rdi = (rbx) as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 3; continue; } bb = 2; continue; }
            2 => { rdi = (crate::simd_ops::ru64(rbx)) as u64; bb = 3; continue; }
            3 => { rcx = ((rdi.wrapping_add(rcx.wrapping_mul(2))) as u64) as u64; r8 = ((r8).wrapping_add(r8)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rax = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; crate::simd_ops::wu16(rdi.wrapping_add(rsi.wrapping_mul(2)), ((rax as u16)) as u16); rax = (rbx) as u64; bb = 5; continue; }
            4 => { r8 = (r8 & !0xffu64) | (((crate::simd_ops::ru8(rsp.wrapping_add(0x40u64))) as u8 as u64)); r9 = (rdx) as u64; rdx = (r10) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (r10) as u64); rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 5; continue; }
            5 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e1df00(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001e97380; const P1: u64 = 0x7001e69088; const P2: u64 = 0x7001e69288;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = ((rsp).wrapping_sub(0x50u64)) as u64; rax = (crate::simd_ops::ru64(P0)) as u64; rax = (((rax) ^ (rsp))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), (rax) as u64); rdi = ((rdx as u32)) as u32 as u64; r8 = ((rsp.wrapping_add(0x20u64)) as u64) as u64; rdx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rsi = (rcx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P1)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rbx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { fl_l = ((((rax as u32)) | (0xffffffffu32))) as u32 as u64; fl_r = fl_l; rax = ((fl_l as u32)) as u32 as u64; bb = 6; continue; }
            2 => { r8 = (crate::simd_ops::ru32(rsp.wrapping_add(0x20u64))) as u32 as u64; rax = ((r8 as u32)) as u32 as u64; rax = ((((rax as u32)) & (0xfffffffeu32))) as u32 as u64; rdx = ((r8 as u32)) as u32 as u64; rdx = ((((rdx as u32)) | (0x1u32))) as u32 as u64; fl_l = ((rdi as u8)) as u8 as u64; fl_r = (0x92u8) as u8 as u64; rdx = (if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { (rax as u32) } else { (rdx as u32) }) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 5; continue; } bb = 3; continue; }
            3 => { rcx = (rsi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P2)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { fl_l = ((((rbx as u32)) | (0xffffffffu32))) as u32 as u64; fl_r = fl_l; rbx = ((fl_l as u32)) as u32 as u64; bb = 5; continue; }
            5 => { rax = ((rbx as u32)) as u32 as u64; bb = 6; continue; }
            6 => { rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rcx = (((rcx) ^ (rsp))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rsp = ((rsp).wrapping_add(0x50u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e1dfa0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001e69258; const P1: u64 = 0x7001e69260;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    r8 = (0u32) as u32 as u64;
    fl_l = 0; fl_r = 0;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P0)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    rcx = (0u32) as u32 as u64;
    fl_l = 0; fl_r = 0;
    fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64;
    if !(((((fl_l as u32)) & ((fl_r as u32))) != 0)) { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P1)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rcx = ((rax as u32)) as u32 as u64; }
    rax = ((rcx as u32)) as u32 as u64;
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e1dfd0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001e6be28; const P1: u64 = 0x7001e695d8; const P2: u64 = 0x7001e69360;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    r8 = ((P0) as u64) as u64;
    rdx = (0x104u32) as u32 as u64;
    rbx = (rcx) as u64;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P1)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    rdx = (0x104u32) as u32 as u64;
    rcx = (rbx) as u64;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P2)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64;
    rax = (rax & !0xffu64) | (((if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { 1u8 } else { 0u8 }) as u8 as u64));
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = core::ptr::read_unaligned(rsp as *const u64);
    rsp = rsp.wrapping_add(8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e1e010(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001e69378;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P0)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64;
    rax = (rax & !0xffu64) | (((if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { 1u8 } else { 0u8 }) as u8 as u64));
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e1e030(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001e97380; const P1: u64 = 0x7001e69060; const P2: u64 = 0x7001e69290; const P3: u64 = 0x7001e69268;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 29>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (rbx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r15) as u64); rbp = ((rsp.wrapping_sub(0x37u64)) as u64) as u64; rsp = ((rsp).wrapping_sub(0xc0u64)) as u64; rax = (crate::simd_ops::ru64(P0)) as u64; rax = (((rax) ^ (rsp))) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x27u64), (rax) as u64); rax = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; xmm0 = 0u128; xmm1 = 0u128; crate::simd_ops::wu32(rbp.wrapping_add(0x1fu64), ((rax as u32)) as u32); rbx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rbp.wrapping_sub(0x19u64), ((rax as u32)) as u32); r15 = (rdx) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (rbx) as u64); r8 = ((rax.wrapping_add(0x7u64)) as u32) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), (0x2000000u32) as u32); r9 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), (0x3u32) as u32); rdx = (0x80u32) as u32 as u64; r14 = (r14 & !0xffu64) | ((((rbx as u8)) as u8 as u64)); crate::simd_ops::wu128(rbp.wrapping_sub(0x11u64), (xmm0) as u128); rdi = (rdi & !0xffu64) | ((((rbx as u8)) as u8 as u64)); crate::simd_ops::wu128(rbp.wrapping_sub(0x1u64), (xmm0) as u128); crate::simd_ops::wu128(rbp.wrapping_add(0xfu64), (xmm0) as u128); crate::simd_ops::wu128(rbp.wrapping_sub(0x49u64), (xmm1) as u128); crate::simd_ops::wu128(rbp.wrapping_sub(0x39u64), (xmm1) as u128); crate::simd_ops::wu128(rbp.wrapping_sub(0x29u64), (xmm1) as u128); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P1)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rsi = (rax) as u64; fl_l = (rax) as u64 as u64; fl_r = (0xffffffffffffffffu64) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rdx = ((rbp.wrapping_sub(0x11u64)) as u64) as u64; rcx = (rax) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P2)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; rcx = (rsi) as u64; r14 = (r14 & !0xffu64) | (((if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { 1u8 } else { 0u8 }) as u8 as u64)); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P3)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 2; continue; }
            2 => { r9 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (rbx) as u64); crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), (0x2000000u32) as u32); rdx = (0x80u32) as u32 as u64; rcx = (r15) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), (0x3u32) as u32); r8 = ((r9.wrapping_add(0x7u64)) as u32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P1)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rsi = (rax) as u64; fl_l = (rax) as u64 as u64; fl_r = (0xffffffffffffffffu64) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rdx = ((rbp.wrapping_sub(0x49u64)) as u64) as u64; rcx = (rax) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P2)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; rcx = (rsi) as u64; rdi = (rdi & !0xffu64) | (((if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { 1u8 } else { 0u8 }) as u8 as u64)); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P3)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 4; continue; }
            4 => { fl_l = ((r14 as u8)) as u8 as u64; fl_r = ((r14 as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { fl_l = 0; fl_r = ((rdi as u8)) as u8 as u64; rdi = (rdi & !0xffu64) | (((0u8.wrapping_sub(fl_r as u8)) as u8 as u64)); rax = (if (((fl_l as u8)) < ((fl_r as u8))) { 0xffffffffu32 } else { 0u32 }) as u32 as u64; rax = (0u32.wrapping_sub((rax as u32))) as u32 as u64; fl_l = (((rax as u32)).wrapping_sub(1)) as u32 as u64; rax = ((fl_l as u32)) as u32 as u64; bb = 13; continue; }
            6 => { fl_l = ((rdi as u8)) as u8 as u64; fl_r = ((rdi as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 12; continue; } bb = 7; continue; }
            7 => { rax = (crate::simd_ops::ru32(rbp.wrapping_sub(0x2du64))) as u32 as u64; fl_l = (crate::simd_ops::ru32(rbp.wrapping_add(0xbu64))) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 11; continue; } bb = 8; continue; }
            8 => { rax = (crate::simd_ops::ru32(rbp.wrapping_sub(0x1du64))) as u32 as u64; fl_l = (crate::simd_ops::ru32(rbp.wrapping_add(0x1bu64))) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 11; continue; } bb = 9; continue; }
            9 => { rax = (crate::simd_ops::ru32(rbp.wrapping_sub(0x19u64))) as u32 as u64; fl_l = (crate::simd_ops::ru32(rbp.wrapping_add(0x1fu64))) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 11; continue; } bb = 10; continue; }
            10 => { rbx = (0x1u32) as u32 as u64; bb = 11; continue; }
            11 => { rax = ((rbx as u32)) as u32 as u64; bb = 13; continue; }
            12 => { rax = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; bb = 13; continue; }
            13 => { rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x27u64))) as u64; rcx = (((rcx) ^ (rsp))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x100u64))) as u64; rsp = ((rsp).wrapping_add(0xc0u64)) as u64; r15 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rsi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rbp = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e1e180(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001e97380; const P1: u64 = 0x7001e69088;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x58u64)) as u64;
    rax = (crate::simd_ops::ru64(P0)) as u64;
    rax = (((rax) ^ (rsp))) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), (rax) as u64);
    r8 = ((rsp.wrapping_add(0x20u64)) as u64) as u64;
    rdx = (0u32) as u32 as u64;
    fl_l = 0; fl_r = 0;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P1)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64;
    if !(((((fl_l as u32)) & ((fl_r as u32))) == 0)) { rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x3cu64))) as u32 as u64; rcx = (crate::simd_ops::ru32(rsp.wrapping_add(0x40u64))) as u32 as u64; rax = ((rax).wrapping_shl(32u32)) as u64; fl_l = (((rax) | (rcx))) as u64 as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; } else { fl_l = (((rax) | (0xffffffffffffffffu64))) as u64 as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; }
    rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64;
    rcx = (((rcx) ^ (rsp))) as u64;
    rsp = ((rsp).wrapping_add(0x58u64)) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e1e1d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001e97380; const P1: u64 = 0x7; const P2: u64 = 0x7001e69060; const P3: u64 = 0x7001e69290; const P4: u64 = 0x7001e69268;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 17>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = ((rsp).wrapping_sub(0x80u64)) as u64; rax = (crate::simd_ops::ru64(P0)) as u64; rax = (((rax) ^ (rsp))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x78u64), (rax) as u64); crate::simd_ops::lk_and_u64(rsp.wrapping_add(0x30u64), (0x0u64) as u64); r9 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), (0x2000000u32) as u32); rdx = (0x80u32) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), (0x3u32) as u32); r8 = ((r9.wrapping_add(P1)) as u32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P2)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdi = (((rdi) | (0xffffffffffffffffu64))) as u64; rsi = (rax) as u64; fl_l = (rax) as u64 as u64; fl_r = (rdi) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { xmm0 = 0u128; rdx = ((rsp.wrapping_add(0x40u64)) as u64) as u64; rax = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rcx = (rsi) as u64; crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm0) as u128); crate::simd_ops::wu32(rsp.wrapping_add(0x70u64), ((rax as u32)) as u32); crate::simd_ops::wu128(rsp.wrapping_add(0x50u64), (xmm0) as u128); crate::simd_ops::wu128(rsp.wrapping_add(0x60u64), (xmm0) as u128); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P3)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = (rsi) as u64; rbx = ((rax as u32)) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P4)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = (crate::simd_ops::ru32(rsp.wrapping_add(0x68u64))) as u32 as u64; fl_l = ((rbx as u32)) as u32 as u64; fl_r = ((rbx as u32)) as u32 as u64; rdi = (if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { rcx } else { rdi }) as u64; bb = 2; continue; }
            2 => { rax = (rdi) as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; fl_l = (((rcx) ^ (rsp))) as u64 as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; r11 = ((rsp.wrapping_add(0x80u64)) as u64) as u64; rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x18u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x20u64))) as u64; rsp = (r11) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e1e290(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001e97380; const P1: u64 = 0x7001e69088;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x58u64)) as u64;
    rax = (crate::simd_ops::ru64(P0)) as u64;
    rax = (((rax) ^ (rsp))) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), (rax) as u64);
    r8 = ((rsp.wrapping_add(0x20u64)) as u64) as u64;
    rdx = (0u32) as u32 as u64;
    fl_l = 0; fl_r = 0;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P1)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64;
    if !(((((fl_l as u32)) & ((fl_r as u32))) != 0)) { fl_l = (((rax) | (0xffffffffffffffffu64))) as u64 as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; } else { rcx = (crate::simd_ops::ru32(rsp.wrapping_add(0x34u64))) as u32 as u64; rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x38u64))) as u32 as u64; rax = ((rax).wrapping_shl(32u32)) as u64; rax = (((rax) | (rcx))) as u64; rcx = (0x19db1ded53e8000u64) as u64; fl_l = (rax) as u64 as u64; fl_r = (rcx) as u64 as u64; rax = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; }
    rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64;
    rcx = (((rcx) ^ (rsp))) as u64;
    rsp = ((rsp).wrapping_add(0x58u64)) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e1e2f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001e69250; const P1: u64 = 0x7001e69260;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    rax = (rdx) as u64;
    r8 = (0u32) as u32 as u64;
    fl_l = 0; fl_r = 0;
    rdx = (rcx) as u64;
    rcx = (rax) as u64;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P0)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    rcx = (0u32) as u32 as u64;
    fl_l = 0; fl_r = 0;
    fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64;
    if !(((((fl_l as u32)) & ((fl_r as u32))) != 0)) { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P1)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rcx = ((rax as u32)) as u32 as u64; }
    rax = ((rcx as u32)) as u32 as u64;
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e1e330(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001e69058; const P1: u64 = 0x7001e69260;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    rdx = (0u32) as u32 as u64;
    fl_l = 0; fl_r = 0;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P0)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64;
    if !(((((fl_l as u32)) & ((fl_r as u32))) == 0)) { rax = (0x1u32) as u32 as u64; } else { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P1)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rax = (((rax as u32)).wrapping_sub(0xb7u32)) as u32 as u64; fl_l = 0; fl_r = ((rax as u32)) as u32 as u64; rax = (0u32.wrapping_sub(fl_r as u32)) as u32 as u64; rax = (if (((fl_l as u32)) < ((fl_r as u32))) { 0xffffffffu32 } else { 0u32 }) as u32 as u64; fl_l = ((rax as u32)) as u64; }
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e1e360(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001e97380; const P1: u64 = 0x7001e1da0c; const P2: u64 = 0x7001e6be2c; const P3: u64 = 0x7001e1de78; const P4: u64 = 0x7001e69070; const P5: u64 = 0x7001e1e500; const P6: u64 = 0x7001e69068; const P7: u64 = 0x7001e695d8; const P8: u64 = 0x7001e69480; const P9: u64 = 0x7001e1e4cf; const P10: u64 = 0x7001e6649c;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 93>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r12) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r15) as u64); rbp = ((rsp.wrapping_sub(0x1b0u64)) as u64) as u64; rsp = ((rsp).wrapping_sub(0x2b0u64)) as u64; rax = (crate::simd_ops::ru64(P0)) as u64; rax = (((rax) ^ (rsp))) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x1a0u64), (rax) as u64); rbx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; xmm0 = 0u128; r15 = (r8) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), (rbx) as u64); crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm0) as u128); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), (rbx) as u64); fl_l = (((r8) | (0xffffffffffffffffu64))) as u64 as u64; fl_r = fl_l; r8 = ((fl_l as u64)) as u64; r12 = (r9) as u64; rsi = (rcx) as u64; bb = 1; continue; }
            1 => { r8 = ((r8).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru16(rdx.wrapping_add(r8.wrapping_mul(2)))) as u16 as u64; fl_r = ((rbx as u16)) as u16 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { bb = 1; continue; } bb = 2; continue; }
            2 => { rcx = ((rsp.wrapping_add(0x30u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; r14 = (0x2u32) as u32 as u64; fl_l = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64 as u64; fl_r = (rbx) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 4; continue; } bb = 3; continue; }
            3 => { r8 = ((r14 as u32)) as u32 as u64; rdx = ((P2) as u64) as u64; rcx = ((rsp.wrapping_add(0x30u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 4; continue; }
            4 => { fl_l = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64 as u64; fl_r = (0x7u64) as u64 as u64; rcx = ((rsp.wrapping_add(0x30u64)) as u64) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((rbx as u32)) as u32); r8 = ((rsp.wrapping_add(0x50u64)) as u64) as u64; rcx = (if (((fl_l as u64)) > ((fl_r as u64))) { crate::simd_ops::ru64(rsp.wrapping_add(0x30u64)) } else { rcx }) as u64; r9 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rdx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rbx) as u64); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P4)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdi = (rax) as u64; fl_l = (rax) as u64 as u64; fl_r = (0xffffffffffffffffu64) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 6; continue; } bb = 5; continue; }
            5 => { crate::simd_ops::wu32(r15, (0xa1u32) as u32); crate::simd_ops::wu16(rsi, ((rbx as u16)) as u16); bb = 16; continue; }
            6 => { fl_l = (crate::simd_ops::ru16(rsp.wrapping_add(0x7cu64))) as u16 as u64; fl_r = (0x2eu16) as u16 as u64; crate::simd_ops::wu32(r15, ((rbx as u32)) as u32); if (((fl_l as u16)) != ((fl_r as u16))) { bb = 12; continue; } bb = 7; continue; }
            7 => { fl_l = (crate::simd_ops::ru16(rsp.wrapping_add(0x7eu64))) as u16 as u64; fl_r = ((rbx as u16)) as u16 as u64; if (((fl_l as u16)) == ((fl_r as u16))) { bb = 10; continue; } bb = 8; continue; }
            8 => { fl_l = (crate::simd_ops::ru16(rsp.wrapping_add(0x7eu64))) as u16 as u64; fl_r = (0x2eu16) as u16 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { bb = 12; continue; } bb = 9; continue; }
            9 => { fl_l = (crate::simd_ops::ru16(rbp.wrapping_sub(0x80u64))) as u16 as u64; fl_r = ((rbx as u16)) as u16 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { bb = 12; continue; } bb = 10; continue; }
            10 => { r8 = (r12) as u64; rdx = (rdi) as u64; rcx = (rsi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; fl_l = (crate::simd_ops::ru16(rsi)) as u16 as u64; fl_r = ((rbx as u16)) as u16 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { bb = 15; continue; } bb = 11; continue; }
            11 => { rcx = (rdi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P6)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 16; continue; }
            12 => { r8 = ((rsp.wrapping_add(0x7cu64)) as u64) as u64; rdx = (0x104u32) as u32 as u64; rcx = (rsi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P7)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x50u64))) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x10u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 14; continue; } bb = 13; continue; }
            13 => { rax = ((((rax as u32)) & (0x7ba7u32))) as u32 as u64; fl_l = 0; fl_r = ((rax as u32)) as u32 as u64; rax = (0u32.wrapping_sub(fl_r as u32)) as u32 as u64; rax = (if (((fl_l as u32)) < ((fl_r as u32))) { 0xffffffffu32 } else { 0u32 }) as u32 as u64; fl_l = ((((rax as u32)) & (0xfffffff9u32))) as u32 as u64; fl_r = fl_l; rax = ((fl_l as u32)) as u32 as u64; r14 = ((rax.wrapping_add(0x8u64)) as u32) as u32 as u64; bb = 14; continue; }
            14 => { crate::simd_ops::wu32(r12, ((r14 as u32)) as u32); bb = 15; continue; }
            15 => { rbx = (rdi) as u64; bb = 16; continue; }
            16 => { rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; fl_l = (rdx) as u64 as u64; fl_r = (0x7u64) as u64 as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 21; continue; } bb = 17; continue; }
            17 => { rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rdx = ((rdx.wrapping_mul(2).wrapping_add(0x2u64)) as u64) as u64; rax = (rcx) as u64; fl_l = (rdx) as u64 as u64; fl_r = (0x1000u64) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 20; continue; } bb = 18; continue; }
            18 => { rcx = (crate::simd_ops::ru64(rcx.wrapping_sub(0x8u64))) as u64; rdx = ((rdx).wrapping_add(0x27u64)) as u64; rax = ((rax).wrapping_sub(rcx)) as u64; rax = ((rax).wrapping_add(0xfffffffffffffff8u64)) as u64; fl_l = (rax) as u64 as u64; fl_r = (0x1fu64) as u64 as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 20; continue; } bb = 19; continue; }
            19 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P8)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::lift_ops::trap(0x80000003, P9, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            20 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P10) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 21; continue; }
            21 => { rax = (rbx) as u64; rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x1a0u64))) as u64; rcx = (((rcx) ^ (rsp))) as u64; rsp = ((rsp).wrapping_add(0x2b0u64)) as u64; r15 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r12 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rsi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rbx = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rbp = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e1e500(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001e97380; const P1: u64 = 0x7001e69078; const P2: u64 = 0x7001e6be28; const P3: u64 = 0x7001e695d8;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 83>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rbx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = ((rsp).wrapping_sub(0x280u64)) as u64; rax = (crate::simd_ops::ru64(P0)) as u64; rax = (((rax) ^ (rsp))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x270u64), (rax) as u64); rsi = (rdx) as u64; rbx = (rcx) as u64; rcx = (rsi) as u64; rdx = ((rsp.wrapping_add(0x20u64)) as u64) as u64; rdi = (r8) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P1)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rbp = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; bb = 6; continue; }
            1 => { fl_l = (crate::simd_ops::ru16(rsp.wrapping_add(0x4cu64))) as u16 as u64; fl_r = (0x2eu16) as u16 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { bb = 9; continue; } bb = 2; continue; }
            2 => { rax = ((crate::simd_ops::ru16(rsp.wrapping_add(0x4eu64))) as u32) as u32 as u64; fl_l = ((rax as u16)) as u16 as u64; fl_r = ((rax as u16)) as u16 as u64; if ((((fl_l as u16)) & ((fl_r as u16))) == 0) { bb = 5; continue; } bb = 3; continue; }
            3 => { fl_l = ((rax as u16)) as u16 as u64; fl_r = (0x2eu16) as u16 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { bb = 9; continue; } bb = 4; continue; }
            4 => { fl_l = (crate::simd_ops::ru16(rsp.wrapping_add(0x50u64))) as u16 as u64; fl_r = ((rbp as u16)) as u16 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { bb = 9; continue; } bb = 5; continue; }
            5 => { rdx = ((rsp.wrapping_add(0x20u64)) as u64) as u64; rcx = (rsi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P1)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 6; continue; }
            6 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 1; continue; } bb = 7; continue; }
            7 => { crate::simd_ops::wu32(rdi, (0x8u32) as u32); r8 = ((P2) as u64) as u64; bb = 8; continue; }
            8 => { rdx = (0x104u32) as u32 as u64; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P3)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rax = (rbx) as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x270u64))) as u64; rcx = (((rcx) ^ (rsp))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x2b8u64))) as u64; rsp = ((rsp).wrapping_add(0x280u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rsi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rbp = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            9 => { rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x20u64))) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x10u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 11; continue; } bb = 10; continue; }
            10 => { rax = (0x2u32) as u32 as u64; bb = 12; continue; }
            11 => { rax = ((((rax as u32)) & (0x7ba7u32))) as u32 as u64; fl_l = 0; fl_r = ((rax as u32)) as u32 as u64; rax = (0u32.wrapping_sub(fl_r as u32)) as u32 as u64; rax = (if (((fl_l as u32)) < ((fl_r as u32))) { 0xffffffffu32 } else { 0u32 }) as u32 as u64; rax = ((((rax as u32)) & (0xfffffff9u32))) as u32 as u64; fl_l = (((rax as u32)).wrapping_add(0x8u32)) as u32 as u64; fl_r = fl_l; rax = ((fl_l as u32)) as u32 as u64; bb = 12; continue; }
            12 => { crate::simd_ops::wu32(rdi, ((rax as u32)) as u32); r8 = ((rsp.wrapping_add(0x4cu64)) as u64) as u64; bb = 8; continue; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e1e5e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001e69390;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P0)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    fl_l = ((rax as u32)) as u32 as u64; fl_r = (0xffffffffu32) as u32 as u64;
    rax = (rax & !0xffu64) | (((if (((fl_l as u32)) != ((fl_r as u32))) { 1u8 } else { 0u8 }) as u8 as u64));
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e1e620(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0xffffffffffffffe8; const P3: u64 = 0xffffffffffffffe0; const P4: u64 = 0xffffffffffffffd8; const P5: u64 = 0x7; const P6: u64 = 0x7001e69060; const P7: u64 = 0x7001e69260; const P8: u64 = 0xfffffffffffffffe; const P9: u64 = 0x7001e69280; const P10: u64 = 0x7001e69268;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = (rsp) as u64; crate::simd_ops::wu64(rax.wrapping_add(P0), (rbx) as u64); crate::simd_ops::wu64(rax.wrapping_add(P1), (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rdi = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rbx = (rdx) as u64; crate::simd_ops::wu64(rax.wrapping_add(P2), (rdi) as u64); r9 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rax.wrapping_add(P3), ((rdi as u32)) as u32); rdx = (0x120116u32) as u32 as u64; crate::simd_ops::wu32(rax.wrapping_add(P4), (0x3u32) as u32); r8 = ((rdi.wrapping_add(P5)) as u32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P6)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rsi = (rax) as u64; fl_l = (rax) as u64 as u64; fl_r = (0xffffffffffffffffu64) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P7)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 5; continue; }
            2 => { r9 = (0x8u32) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), (rbx) as u64); r8 = ((rsp.wrapping_add(0x60u64)) as u64) as u64; rcx = (rsi) as u64; rdx = ((r9.wrapping_add(P8)) as u32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P9)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rcx = (rsi) as u64; rbx = ((rax as u32)) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P10)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); fl_l = ((rbx as u32)) as u32 as u64; fl_r = ((rbx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P7)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rdi = ((rax as u32)) as u32 as u64; bb = 4; continue; }
            4 => { rax = ((rdi as u32)) as u32 as u64; bb = 5; continue; }
            5 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e1e6b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0xffffffffffffffe8; const P2: u64 = 0xffffffffffffffe0; const P3: u64 = 0xffffffffffffffd8; const P4: u64 = 0x7; const P5: u64 = 0x7001e69060; const P6: u64 = 0x7001e69278; const P7: u64 = 0x7001e69268;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = (rsp) as u64; crate::simd_ops::wu64(rax.wrapping_add(P0), (rbx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; crate::simd_ops::lk_and_u64(rax.wrapping_add(P1), (0x0u64) as u64); r9 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rbx = (rdx) as u64; crate::simd_ops::wu32(rax.wrapping_add(P2), (0x2000000u32) as u32); rdx = (0x100u32) as u32 as u64; crate::simd_ops::wu32(rax.wrapping_add(P3), (0x3u32) as u32); r8 = ((r9.wrapping_add(P4)) as u32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P5)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rdi = (rax) as u64; fl_l = (rax) as u64 as u64; fl_r = (0xffffffffffffffffu64) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; bb = 3; continue; }
            2 => { rax = (0x19db1ded53e8000u64) as u64; r9 = ((rsp.wrapping_add(0x60u64)) as u64) as u64; rax = ((rax).wrapping_add(rbx)) as u64; r8 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rsp.wrapping_add(0x60u64), ((rax as u32)) as u32); rdx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rax = ((rax).wrapping_shr(32u32)) as u64; rcx = (rdi) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x64u64), ((rax as u32)) as u32); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P6)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rcx = (rdi) as u64; rbx = ((rax as u32)) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P7)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rax = ((rbx as u32)) as u32 as u64; bb = 3; continue; }
            3 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e1e740(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001e97380; const P1: u64 = 0x7001e69088; const P2: u64 = 0x7001e69260;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64); rsp = ((rsp).wrapping_sub(0x50u64)) as u64; rax = (crate::simd_ops::ru64(P0)) as u64; rax = (((rax) ^ (rsp))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), (rax) as u64); rbx = (rdx) as u64; r8 = ((rsp.wrapping_add(0x20u64)) as u64) as u64; rdx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P1)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 7; continue; } bb = 1; continue; }
            1 => { rdx = (crate::simd_ops::ru32(rsp.wrapping_add(0x20u64))) as u32 as u64; fl_l = (rbx) as u64 as u64; fl_r = (rbx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { rax = ((rdx as u32)) as u32 as u64; rax = (rax & !0xffu64) | ((((((rax as u8)) & (0x1u8))) as u8 as u64)); fl_l = 0; fl_r = ((rax as u8)) as u8 as u64; rax = (rax & !0xffu64) | (((0u8.wrapping_sub(fl_r as u8)) as u8 as u64)); rcx = (if (((fl_l as u8)) < ((fl_r as u8))) { 0xffffffffu32 } else { 0u32 }) as u32 as u64; rcx = ((((rcx as u32)) & (0xffffff6eu32))) as u32 as u64; fl_l = (((rcx as u32)).wrapping_add(0x1ffu32)) as u32 as u64; fl_r = fl_l; rcx = ((fl_l as u32)) as u32 as u64; crate::simd_ops::wu32(rbx, ((rcx as u32)) as u32); bb = 3; continue; }
            3 => { fl_l = ((rdx as u8)) as u8 as u64; fl_r = (0x10u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rcx = (0x2u32) as u32 as u64; bb = 6; continue; }
            5 => { rdx = ((((rdx as u32)) & (0x7ba7u32))) as u32 as u64; fl_l = 0; fl_r = ((rdx as u32)) as u32 as u64; rdx = (0u32.wrapping_sub(fl_r as u32)) as u32 as u64; rcx = (if (((fl_l as u32)) < ((fl_r as u32))) { 0xffffffffu32 } else { 0u32 }) as u32 as u64; rcx = ((((rcx as u32)) & (0xfffffff9u32))) as u32 as u64; fl_l = (((rcx as u32)).wrapping_add(0x8u32)) as u32 as u64; fl_r = fl_l; rcx = ((fl_l as u32)) as u32 as u64; bb = 6; continue; }
            6 => { rax = ((rcx as u32)) as u32 as u64; bb = 16; continue; }
            7 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P2)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x2u32) as u32 as u64; rax = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 15; continue; } bb = 8; continue; }
            8 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x1u32) as u32 as u64; rax = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 15; continue; } bb = 9; continue; }
            9 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = (0xcu32) as u32 as u64; rax = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 15; continue; } bb = 10; continue; }
            10 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x26u32) as u32 as u64; rax = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 15; continue; } bb = 11; continue; }
            11 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x22u32) as u32 as u64; rax = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 15; continue; } bb = 12; continue; }
            12 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x24u32) as u32 as u64; rax = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 15; continue; } bb = 13; continue; }
            13 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x26u32) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 15; continue; } bb = 14; continue; }
            14 => { rax = (0x8u32) as u32 as u64; bb = 16; continue; }
            15 => { fl_l = ((((rax as u32)) | (0xffffffffu32))) as u32 as u64; fl_r = fl_l; rax = ((fl_l as u32)) as u32 as u64; bb = 16; continue; }
            16 => { rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rcx = (((rcx) ^ (rsp))) as u64; rsp = ((rsp).wrapping_add(0x50u64)) as u64; rbx = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e1e7f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001e97380; const P1: u64 = 0x7001e1da0c; const P2: u64 = 0x7001e1dca0; const P3: u64 = 0x7001e69080; const P4: u64 = 0x7001e69480; const P5: u64 = 0x7001e1e94a; const P6: u64 = 0x7001e6649c;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 17>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64); rbp = (rsp) as u64; rsp = ((rsp).wrapping_sub(0x80u64)) as u64; rax = (crate::simd_ops::ru64(P0)) as u64; rax = (((rax) ^ (rsp))) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x8u64), (rax) as u64); r8 = (((r8) | (0xffffffffffffffffu64))) as u64; xmm0 = 0u128; rdi = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rbp.wrapping_sub(0x58u64), (r8) as u64); crate::simd_ops::wu64(rbp.wrapping_sub(0x18u64), (rdi) as u64); rbx = (rcx) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x10u64), (rdi) as u64); crate::simd_ops::wu64(rbp.wrapping_sub(0x50u64), (r8) as u64); crate::simd_ops::wu64(rbp.wrapping_sub(0x48u64), (r8) as u64); crate::simd_ops::wu128(rbp.wrapping_sub(0x28u64), (xmm0) as u128); bb = 1; continue; }
            1 => { r8 = ((r8).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru16(rdx.wrapping_add(r8.wrapping_mul(2)))) as u16 as u64; fl_r = ((rdi as u16)) as u16 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { bb = 1; continue; } bb = 2; continue; }
            2 => { rcx = ((rbp.wrapping_sub(0x28u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x18u64))) as u64; r9 = (0x2fu32) as u32 as u64; rdx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x10u64))) as u64; r8 = (crate::simd_ops::ru64(rbp.wrapping_sub(0x28u64))) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 3; continue; }
            3 => { fl_l = (rdx) as u64 as u64; fl_r = (0x7u64) as u64 as u64; rax = ((rbp.wrapping_sub(0x28u64)) as u64) as u64; rax = (if (((fl_l as u64)) > ((fl_r as u64))) { r8 } else { rax }) as u64; fl_l = (crate::simd_ops::ru16(rax.wrapping_add(rcx.wrapping_mul(2)).wrapping_sub(0x2u64))) as u16 as u64; fl_r = ((r9 as u16)) as u16 as u64; if (((fl_l as u16)) == ((fl_r as u16))) { bb = 9; continue; } bb = 4; continue; }
            4 => { fl_l = (rdx) as u64 as u64; fl_r = (0x7u64) as u64 as u64; rax = ((rbp.wrapping_sub(0x28u64)) as u64) as u64; rax = (if (((fl_l as u64)) > ((fl_r as u64))) { r8 } else { rax }) as u64; fl_l = (crate::simd_ops::ru16(rax.wrapping_add(rcx.wrapping_mul(2)).wrapping_sub(0x2u64))) as u16 as u64; fl_r = (0x5cu16) as u16 as u64; if (((fl_l as u16)) == ((fl_r as u16))) { bb = 9; continue; } bb = 5; continue; }
            5 => { fl_l = (rcx) as u64 as u64; fl_r = (rdx) as u64 as u64; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 7; continue; } bb = 6; continue; }
            6 => { rax = ((rcx.wrapping_add(0x1u64)) as u64) as u64; fl_l = (rdx) as u64 as u64; fl_r = (0x7u64) as u64 as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x18u64), (rax) as u64); rax = ((rbp.wrapping_sub(0x28u64)) as u64) as u64; rax = (if (((fl_l as u64)) > ((fl_r as u64))) { r8 } else { rax }) as u64; crate::simd_ops::wu32(rax.wrapping_add(rcx.wrapping_mul(2)), ((r9 as u32)) as u32); bb = 8; continue; }
            7 => { r8 = (r8 & !0xffu64) | (((crate::simd_ops::ru8(rbp.wrapping_sub(0x60u64))) as u8 as u64)); rcx = ((rbp.wrapping_sub(0x28u64)) as u64) as u64; rdx = (0x1u32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 8; continue; }
            8 => { r8 = (crate::simd_ops::ru64(rbp.wrapping_sub(0x28u64))) as u64; rdx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x10u64))) as u64; bb = 9; continue; }
            9 => { fl_l = (rdx) as u64 as u64; fl_r = (0x7u64) as u64 as u64; rcx = ((rbp.wrapping_sub(0x28u64)) as u64) as u64; r9 = ((rbp.wrapping_sub(0x38u64)) as u64) as u64; rcx = (if (((fl_l as u64)) > ((fl_r as u64))) { r8 } else { rcx }) as u64; rdx = ((rbp.wrapping_sub(0x30u64)) as u64) as u64; r8 = ((rbp.wrapping_sub(0x40u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P3)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 11; continue; } bb = 10; continue; }
            10 => { rax = (crate::simd_ops::ru64(rbp.wrapping_sub(0x40u64))) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x58u64), (rax) as u64); rax = (crate::simd_ops::ru64(rbp.wrapping_sub(0x38u64))) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x50u64), (rax) as u64); rax = (crate::simd_ops::ru64(rbp.wrapping_sub(0x30u64))) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x48u64), (rax) as u64); bb = 11; continue; }
            11 => { xmm0 = (crate::simd_ops::ru128(rbp.wrapping_sub(0x58u64))) as u128; rdx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x10u64))) as u64; xmm1 = ((crate::simd_ops::ru64(rbp.wrapping_sub(0x48u64))) as u64) as u128; crate::simd_ops::wu128(rbx, (xmm0) as u128); crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), ((xmm1 as u64)) as u64); fl_l = (rdx) as u64 as u64; fl_r = (0x7u64) as u64 as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 16; continue; } bb = 12; continue; }
            12 => { rcx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x28u64))) as u64; rdx = ((rdx.wrapping_mul(2).wrapping_add(0x2u64)) as u64) as u64; rax = (rcx) as u64; fl_l = (rdx) as u64 as u64; fl_r = (0x1000u64) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 15; continue; } bb = 13; continue; }
            13 => { rcx = (crate::simd_ops::ru64(rcx.wrapping_sub(0x8u64))) as u64; rdx = ((rdx).wrapping_add(0x27u64)) as u64; rax = ((rax).wrapping_sub(rcx)) as u64; rax = ((rax).wrapping_add(0xfffffffffffffff8u64)) as u64; fl_l = (rax) as u64 as u64; fl_r = (0x1fu64) as u64 as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 15; continue; } bb = 14; continue; }
            14 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P4)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::lift_ops::trap(0x80000003, P5, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            15 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 16; continue; }
            16 => { rax = (rbx) as u64; rcx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x8u64))) as u64; fl_l = (((rcx) ^ (rsp))) as u64 as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; r11 = ((rsp.wrapping_add(0x80u64)) as u64) as u64; rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x10u64))) as u64; rdi = (crate::simd_ops::ru64(r11.wrapping_add(0x20u64))) as u64; rsp = (r11) as u64; rbp = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e1e980(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001e69248; const P1: u64 = 0x7001e69260;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    rax = (rdx) as u64;
    r8 = (0u32) as u32 as u64;
    fl_l = 0; fl_r = 0;
    rdx = (rcx) as u64;
    rcx = (rax) as u64;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P0)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    rcx = (0u32) as u32 as u64;
    fl_l = 0; fl_r = 0;
    fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64;
    if !(((((fl_l as u8)) & ((fl_r as u8))) != 0)) { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P1)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rcx = ((rax as u32)) as u32 as u64; }
    rax = ((rcx as u32)) as u32 as u64;
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e1e9b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = (0u32) as u32 as u64;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu16(rcx, ((rax as u16)) as u16);
    rax = (rcx) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e1e9c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001e97380; const P1: u64 = 0x7001e4ea4c; const P2: u64 = 0x7001e6be34; const P3: u64 = 0x7001e695d8;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 73>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64);
    rsp = ((rsp).wrapping_sub(0x240u64)) as u64;
    rax = (crate::simd_ops::ru64(P0)) as u64;
    rax = (((rax) ^ (rsp))) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x230u64), (rax) as u64);
    rbx = (rcx) as u64;
    rdx = ((rsp.wrapping_add(0x20u64)) as u64) as u64;
    rcx = (0x104u32) as u32 as u64;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64;
    rcx = ((P2) as u64) as u64;
    r8 = ((rsp.wrapping_add(0x20u64)) as u64) as u64;
    rdx = (0x104u32) as u32 as u64;
    r8 = (if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { rcx } else { r8 }) as u64;
    rcx = (rbx) as u64;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P3)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    rax = (rbx) as u64;
    rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x230u64))) as u64;
    rcx = (((rcx) ^ (rsp))) as u64;
    rsp = ((rsp).wrapping_add(0x240u64)) as u64;
    rbx = core::ptr::read_unaligned(rsp as *const u64);
    rsp = rsp.wrapping_add(8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e1ea30(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001e69270; const P1: u64 = 0x7001e69030;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rbx) as u64);
    rsp = rsp.wrapping_sub(8);
    core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64);
    rsp = ((rsp).wrapping_sub(0x40u64)) as u64;
    rbx = (rdx) as u64;
    rdi = (rcx) as u64;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P0)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    rcx = (0u32) as u32 as u64;
    fl_l = 0; fl_r = 0;
    r8 = (rdi) as u64;
    fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64;
    rcx = (rcx & !0xffu64) | (((if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { 1u8 } else { 0u8 }) as u8 as u64));
    crate::simd_ops::lk_and_u64(rsp.wrapping_add(0x38u64), (0x0u64) as u64);
    crate::simd_ops::lk_and_u64(rsp.wrapping_add(0x30u64), (0x0u64) as u64);
    r9 = ((((r9 as u32)) | (0xffffffffu32))) as u32 as u64;
    crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), (0x104u32) as u32);
    rdx = (0u32) as u32 as u64;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rbx) as u64);
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P1)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64;
    rsp = ((rsp).wrapping_add(0x40u64)) as u64;
    rdi = core::ptr::read_unaligned(rsp as *const u64);
    rsp = rsp.wrapping_add(8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e1ea80(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001e69270; const P1: u64 = 0x7001e69018;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rbx) as u64);
    rsp = rsp.wrapping_sub(8);
    core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64);
    rsp = ((rsp).wrapping_sub(0x30u64)) as u64;
    rbx = (rdx) as u64;
    rdi = (rcx) as u64;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P0)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    rcx = (0u32) as u32 as u64;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), (0x104u32) as u32);
    fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rbx) as u64);
    r8 = (rdi) as u64;
    rcx = (rcx & !0xffu64) | (((if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { 1u8 } else { 0u8 }) as u8 as u64));
    r9 = ((((r9 as u32)) | (0xffffffffu32))) as u32 as u64;
    rdx = (0u32) as u32 as u64;
    fl_l = 0; fl_r = 0;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P1)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64;
    rsp = ((rsp).wrapping_add(0x30u64)) as u64;
    rdi = core::ptr::read_unaligned(rsp as *const u64);
    rsp = rsp.wrapping_add(8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e1ec7c(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x7001e16460;
    let (mut rax, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    crate::simd_ops::lk_and_u64(rcx, (0x0u64) as u64);
    rbx = (rcx) as u64;
    crate::simd_ops::lk_and_u64(rcx.wrapping_add(P0), (0x0u64) as u64);
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    rax = (rbx) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = core::ptr::read_unaligned(rsp as *const u64);
    rsp = rsp.wrapping_add(8);
    return rax;
}
