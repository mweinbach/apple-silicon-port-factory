#![allow(unused_imports, unused_variables, unused_mut, unused_assignments, non_snake_case, dead_code, clippy::all, unreachable_code, unused_parens)]

#[inline(never)]
pub unsafe extern "win64" fn fn_1412ea3a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14397ff18; const P1: u64 = 0x142645790; const P2: u64 = 0x1426455a8; const P3: u64 = 0x143a00180; const P4: u64 = 0x14204f860; const P5: u64 = 0x142442de0; const P6: u64 = 0x14240aa00; const P7: u64 = 0x14240ab10; const P8: u64 = 0x142645598;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x1412ea3a0, funcinfo_rva: 0x36959c0, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0x38, conts: &[] };
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rbp) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r15) as u64); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rsi = (crate::simd_ops::ru64(rcx)) as u64; crate::simd_ops::wu64(rcx, (0x0u64) as u64); fl_l = (rsi) as u64 as u64; fl_r = (rsi) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 14; continue; } bb = 1; continue; }
            1 => { r15 = (crate::simd_ops::ru64(P0)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1412ea3d2); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P1)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdi = ((rax as u32)) as u32 as u64; rcx = (crate::simd_ops::ru32(r15.wrapping_add(0x10u64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 3; continue; } bb = 2; continue; }
            2 => { rcx = ((r15.wrapping_add(0x8u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1412ea3e6); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P2)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu32(r15.wrapping_add(0x10u64), ((rdi as u32)) as u32); bb = 3; continue; }
            3 => { crate::simd_ops::lk_add_u32(r15.wrapping_add(0x14u64), (1) as u32); rax = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rdx = (crate::simd_ops::ru32(r15.wrapping_add(0x20u64))) as u32 as u64; rbp = (crate::simd_ops::ru64(r15.wrapping_add(0x18u64))) as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 12; continue; } bb = 4; continue; }
            4 => { rcx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; bb = 5; continue; }
            5 => { r14 = (((rcx) as i64).wrapping_mul((0x68u64) as i64) as u64) as u64; rdi = (crate::simd_ops::ru64(r14.wrapping_add(rbp))) as u64; fl_l = (rdi) as u64 as u64; fl_r = (rsi) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 8; continue; } bb = 6; continue; }
            6 => { rax = (((rax as u32)).wrapping_add(1)) as u32 as u64; rcx = ((((rax as u32)) as i32 as i64) as u64) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rdx) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 5; continue; } bb = 7; continue; }
            7 => { bb = 12; continue; }
            8 => { crate::simd_ops::wu8(r14.wrapping_add(rbp).wrapping_add(0x38u64), (0x0u8) as u8); rax = (crate::simd_ops::ru32(P3)) as u32 as u64; crate::simd_ops::wu32(r14.wrapping_add(rbp).wrapping_add(0x64u64), ((rax as u32)) as u32); fl_l = (crate::simd_ops::ru8(r14.wrapping_add(rbp).wrapping_add(0x39u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 12; continue; } bb = 9; continue; }
            9 => { fl_l = (rdi) as u64 as u64; fl_r = (rdi) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 10; continue; }
            10 => { rcx = (rdi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1412ea451); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = (rdi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1412ea459); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 11; continue; }
            11 => { crate::simd_ops::wu64(r14.wrapping_add(rbp), (0x0u64) as u64); rdx = (crate::simd_ops::ru64(r15.wrapping_add(0x18u64))) as u64; rax = (crate::simd_ops::ru32(r15.wrapping_add(0x20u64))) as u32 as u64; rsi = (((rax) as i64).wrapping_mul((0x68u64) as i64) as u64) as u64; rsi = ((rsi).wrapping_add(rdx)) as u64; rdi = ((rdx.wrapping_add(r14)) as u64) as u64; rax = (crate::simd_ops::ru64(rsi.wrapping_sub(0x68u64))) as u64; crate::simd_ops::wu64(rdi, (rax) as u64); let y_addr: u64 = rsi.wrapping_sub(0x60u64); xmm0 = core::ptr::read_unaligned(y_addr as *const u128); ymm0_hi = core::ptr::read_unaligned(y_addr.wrapping_add(16) as *const u128); let y_addr: u64 = rdi.wrapping_add(0x8u64); core::ptr::write_unaligned(y_addr as *mut u128, xmm0); core::ptr::write_unaligned(y_addr.wrapping_add(16) as *mut u128, ymm0_hi); xmm1 = (crate::simd_ops::ru128(rsi.wrapping_sub(0x40u64))) as u128; crate::simd_ops::wu128(rdi.wrapping_add(0x28u64), (xmm1) as u128); rax = ((crate::simd_ops::ru8(rsi.wrapping_sub(0x30u64))) as u32) as u32 as u64; crate::simd_ops::wu8(rdi.wrapping_add(0x38u64), ((rax as u8)) as u8); rax = ((crate::simd_ops::ru8(rsi.wrapping_sub(0x2fu64))) as u32) as u32 as u64; crate::simd_ops::wu8(rdi.wrapping_add(0x39u64), ((rax as u8)) as u8); rdx = ((rsi.wrapping_sub(0x2cu64)) as u64) as u64; rcx = ((rdi.wrapping_add(0x3cu64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1412ea4ad); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = (crate::simd_ops::ru32(rsi.wrapping_sub(0x4u64))) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x64u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(r15.wrapping_add(0x20u64))) as u32 as u64; rcx = (((rax) as i64).wrapping_mul((0x68u64) as i64) as u64) as u64; rax = (crate::simd_ops::ru64(r15.wrapping_add(0x18u64))) as u64; rax = ((rax).wrapping_add(0xffffffffffffffd4u64)) as u64; rcx = ((rcx).wrapping_add(rax)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1412ea4cb); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P7) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; fl_l = (crate::simd_ops::lk_sub_u32(r15.wrapping_add(0x20u64), (1) as u32)) as u32 as u64; bb = 12; continue; }
            12 => { fl_l = (crate::simd_ops::lk_add_u32(r15.wrapping_add(0x14u64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; if (((fl_l as u32)) != 0) { bb = 14; continue; } bb = 13; continue; }
            13 => { rax = (0xffffffffu32) as u32 as u64; let tmp_xchg: u32 = crate::simd_ops::atomic_xchg_u32(r15.wrapping_add(0x10u64), ((rax as u32)) as u32); rax = (tmp_xchg) as u32 as u64; rcx = ((r15.wrapping_add(0x8u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1412ea4e8); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P8)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 14; continue; }
            14 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1412ea510(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14220ea70; const P1: u64 = 0x10; const P2: u64 = 0xc; const P3: u64 = 0x8;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x30u64)) as u64;
    rdi = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdx);
    crate::simd_ops::wu64(rcx, rdi);
    rbx = rcx;
    rax = (0xffffffffffffffffu64) as u64;
    loop { rax = ((rax).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(rax))) as u8 as u64; fl_r = ((rdi as u8)) as u8 as u64; if !((((fl_l as u8)) != ((fl_r as u8)))) { break; } }
    crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax);
    rdx = rsp.wrapping_add(0x20);
    xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128;
    rcx = ((rcx).wrapping_add(0x8u64)) as u64;
    crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128);
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    rax = ((rbx.wrapping_add(P1)) as u64) as u64;
    crate::simd_ops::wu32(rbx.wrapping_add(P2), (0x12u32) as u32);
    crate::simd_ops::wu64(rbx, rax);
    crate::simd_ops::wu64(rax, rdi);
    crate::simd_ops::wu32(rax.wrapping_add(P3), ((rdi as u32)) as u32);
    rax = rbx;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64;
    rsp = ((rsp).wrapping_add(0x30u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1412ea5f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x18; const P2: u64 = 0x20; const P3: u64 = 0x28; const P4: u64 = 0x30; const P5: u64 = 0x38; const P6: u64 = 0x40; const P7: u64 = 0x48; const P8: u64 = 0x8; const P9: u64 = 0xc; const P10: u64 = 0x140215e80; const P11: u64 = 0x14183acc8;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rcx; r14 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rcx.wrapping_add(P0), r14); crate::simd_ops::wu64(rcx.wrapping_add(P1), r14); crate::simd_ops::wu64(rcx.wrapping_add(P2), r14); crate::simd_ops::wu64(rcx.wrapping_add(P3), r14); crate::simd_ops::wu64(rcx.wrapping_add(P4), r14); crate::simd_ops::wu64(rcx.wrapping_add(P5), r14); crate::simd_ops::wu64(rcx.wrapping_add(P6), r14); crate::simd_ops::wu64(rcx.wrapping_add(P7), r14); rax = ((rcx.wrapping_add(P0)) as u64) as u64; crate::simd_ops::wu64(rcx, rax); crate::simd_ops::wu32(rcx.wrapping_add(P8), ((r14 as u32)) as u32); crate::simd_ops::wu32(rcx.wrapping_add(P9), (0x4u32) as u32); rbp = (crate::simd_ops::ru64(rdx)) as u64; rsi = (crate::simd_ops::ru64(rdx.wrapping_add(P8))) as u64; rsi = ((rsi).wrapping_sub(rbp)) as u64; rdi = rsi; rdi = ((((rdi) as i64).wrapping_shr(4u32) as u64)) as u64; fl_l = rdi; fl_r = (0x4u64) as u64; if ((((fl_l as u64)) as i64) <= (((fl_r as u64)) as i64)) { bb = 2; continue; } bb = 1; continue; }
            1 => { rdx = rdi; rax = crate::simd_ops::call_stk(P10, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx, rax); crate::simd_ops::wu32(rbx.wrapping_add(P8), ((r14 as u32)) as u32); crate::simd_ops::wu32(rbx.wrapping_add(P9), ((rdi as u32)) as u32); bb = 2; continue; }
            2 => { r8 = rsi; rdx = rbp; rcx = rax; rax = crate::simd_ops::call_stk(P11, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu32(rbx.wrapping_add(0x8u64), ((rdi as u32)) as u32); rax = rbx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1412ea7e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1433f0fc0; const P1: u64 = 0x1433f0fe0; const P2: u64 = 0x8; const P3: u64 = 0x1412ea880; const P4: u64 = 0x141834ee8; const P5: u64 = 0x142056db0; const P6: u64 = 0x142257320; const P7: u64 = 0x141834d30;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rdi = ((rdx as u32)) as u32 as u64;
    rsi = rcx;
    rax = P0;
    crate::simd_ops::wu64(rcx, rax);
    rax = P1;
    crate::simd_ops::wu64(rcx.wrapping_add(P2), rax);
    rcx = ((rcx).wrapping_add(0x30u64)) as u64;
    r9 = P3;
    rdx = 0x2f0;
    r8 = 0x3;
    rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp);
    rcx = ((rsi.wrapping_add(P2)) as u64) as u64;
    rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp);
    rcx = rsi;
    rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp);
    fl_l = ((rdi as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64;
    if !(((((fl_l as u8)) & ((fl_r as u8))) == 0)) { rdx = 0x900; rcx = rsi; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); }
    rax = rsi;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1412ea860(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1423fd0b0; const P1: u64 = 0x2e0;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rax = rcx;
    crate::simd_ops::wu64(rcx.wrapping_add(P1), (0x0u64) as u64);
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1412ea880(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x2e0; const P1: u64 = 0x14397ff18; const P2: u64 = 0x142258080; const P3: u64 = 0x14330bca8;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rcx;
    rdx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    fl_l = rdx; fl_r = rdx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rcx = (crate::simd_ops::ru64(P1)) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); }
    rax = P3;
    crate::simd_ops::wu64(rbx, rax);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1412ea8c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1423fd0b0; const P1: u64 = 0x143428360; const P2: u64 = 0x1434277a0; const P3: u64 = 0x143428de0; const P4: u64 = 0x1434281d0;
    let (mut rax, mut fl_l, mut fl_r, mut rbp) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0; let mut ymm1_hi: u128 = 0; let mut xmm4: u128 = 0; let mut xmm5: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 21>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rbp = rsp.wrapping_sub(0x57); rsp = ((rsp).wrapping_sub(0xa0u64)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm0 = (crate::simd_ops::ru128(P1)) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rbp.wrapping_sub(0x9u64), (xmm0) as u128); xmm0 = 0u128; ymm0_hi = 0; crate::simd_ops::wu128(rbp.wrapping_add(0x17u64), (xmm0) as u128); crate::simd_ops::wu128(rbp.wrapping_add(0x37u64), (xmm0) as u128); rdx = 0; fl_l = 0; fl_r = 0; rax = (0x3ff0000000000000u64) as u64; xmm1 = 0u128; ymm1_hi = 0; crate::simd_ops::wu128(rbp.wrapping_add(0x7u64), (xmm1) as u128); xmm1 = (crate::simd_ops::ru128(P2)) as u128; ymm1_hi = 0; crate::simd_ops::wu128(rbp.wrapping_add(0x27u64), (xmm1) as u128); xmm1 = (crate::simd_ops::ru128(P1)) as u128; ymm1_hi = 0; crate::simd_ops::wu128(rbp.wrapping_add(0x47u64), (xmm1) as u128); crate::simd_ops::wu64(rcx.wrapping_add(0x2e0u64), rax); crate::simd_ops::wu64(rcx.wrapping_add(0x2e8u64), rdx); crate::simd_ops::wu64(rcx.wrapping_add(0x2f0u64), rdx); crate::simd_ops::wu64(rcx.wrapping_add(0x2f8u64), rdx); let y_addr: u64 = rbp.wrapping_add(0x17u64); xmm1 = crate::simd_ops::ru128(y_addr); ymm1_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rbp.wrapping_sub(0x9u64); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rcx.wrapping_add(0x300u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); let y_addr: u64 = rbp.wrapping_add(0x37u64); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rcx.wrapping_add(0x340u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); let y_addr: u64 = rcx.wrapping_add(0x320u64); crate::simd_ops::wu128(y_addr, xmm1); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm1_hi); xmm1 = (crate::simd_ops::ru128(P2)) as u128; ymm1_hi = 0; crate::simd_ops::wu128(rbp.wrapping_sub(0x9u64), (xmm1) as u128); crate::simd_ops::wu128(rbp.wrapping_add(0x17u64), (xmm1) as u128); xmm3 = (xmm1) as u128; xmm4 = 0u128; crate::simd_ops::wu128(rbp.wrapping_add(0x27u64), (xmm4) as u128); xmm2 = 0u128; xmm5 = 0u128; crate::simd_ops::wu64(rbp.wrapping_sub(0x11u64), ((xmm5 as u64)) as u64); crate::simd_ops::wu64(rcx.wrapping_add(0x360u64), rax); crate::simd_ops::wu64(rcx.wrapping_add(0x368u64), rdx); crate::simd_ops::wu64(rcx.wrapping_add(0x370u64), rdx); xmm0 = (crate::simd_ops::ru128(rbp.wrapping_sub(0x11u64))) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rcx.wrapping_add(0x378u64), (xmm0) as u128); xmm0 = (crate::simd_ops::ru128(rbp.wrapping_add(0x1fu64))) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rcx.wrapping_add(0x3a8u64), (xmm0) as u128); crate::simd_ops::wu64(rcx.wrapping_add(0x388u64), (((xmm1 >> 64) as u64)) as u64); crate::simd_ops::wu128(rcx.wrapping_add(0x390u64), (xmm2) as u128); xmm2 = (crate::simd_ops::ru128(P1)) as u128; crate::simd_ops::wu64(rcx.wrapping_add(0x3a0u64), ((xmm3 as u64)) as u64); crate::simd_ops::wu64(rcx.wrapping_add(0x3b8u64), (((xmm4 >> 64) as u64)) as u64); crate::simd_ops::wu128(rbp.wrapping_sub(0x9u64), (xmm2) as u128); crate::simd_ops::wu128(rbp.wrapping_add(0x47u64), (xmm2) as u128); xmm0 = 0u128; ymm0_hi = 0; crate::simd_ops::wu128(rbp.wrapping_add(0x7u64), (xmm0) as u128); xmm0 = (crate::simd_ops::ru128(P2)) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rbp.wrapping_add(0x27u64), (xmm0) as u128); xmm1 = 0u128; ymm1_hi = 0; crate::simd_ops::wu128(rbp.wrapping_add(0x17u64), (xmm1) as u128); crate::simd_ops::wu128(rbp.wrapping_add(0x37u64), (xmm1) as u128); crate::simd_ops::wu64(rcx.wrapping_add(0x3c0u64), rax); crate::simd_ops::wu64(rcx.wrapping_add(0x3c8u64), rdx); crate::simd_ops::wu64(rcx.wrapping_add(0x3d0u64), rdx); crate::simd_ops::wu64(rcx.wrapping_add(0x3d8u64), rdx); let y_addr: u64 = rbp.wrapping_add(0x17u64); xmm1 = crate::simd_ops::ru128(y_addr); ymm1_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rcx.wrapping_add(0x400u64); crate::simd_ops::wu128(y_addr, xmm1); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm1_hi); xmm1 = (crate::simd_ops::ru128(P2)) as u128; ymm1_hi = 0; crate::simd_ops::wu64(rbp.wrapping_sub(0x11u64), ((xmm5 as u64)) as u64); crate::simd_ops::wu128(rbp.wrapping_add(0x17u64), (xmm1) as u128); xmm0 = (xmm2) as u128; ymm0_hi = 0; let y_addr: u64 = rbp.wrapping_sub(0x9u64); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rcx.wrapping_add(0x3e0u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); let y_addr: u64 = rbp.wrapping_add(0x37u64); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rcx.wrapping_add(0x420u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); crate::simd_ops::wu128(rbp.wrapping_sub(0x9u64), (xmm1) as u128); xmm4 = 0u128; crate::simd_ops::wu128(rbp.wrapping_add(0x27u64), (xmm4) as u128); crate::simd_ops::wu64(rcx.wrapping_add(0x440u64), rax); crate::simd_ops::wu64(rcx.wrapping_add(0x448u64), rdx); crate::simd_ops::wu64(rcx.wrapping_add(0x450u64), rdx); xmm0 = (crate::simd_ops::ru128(rbp.wrapping_sub(0x11u64))) as u128; ymm0_hi = 0; xmm2 = 0u128; xmm3 = (xmm1) as u128; crate::simd_ops::wu128(rcx.wrapping_add(0x458u64), (xmm0) as u128); crate::simd_ops::wu128(rcx.wrapping_add(0x470u64), (xmm2) as u128); crate::simd_ops::wu64(rcx.wrapping_add(0x468u64), (((xmm1 >> 64) as u64)) as u64); crate::simd_ops::wu64(rcx.wrapping_add(0x480u64), ((xmm3 as u64)) as u64); xmm0 = (crate::simd_ops::ru128(rbp.wrapping_add(0x1fu64))) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rcx.wrapping_add(0x488u64), (xmm0) as u128); let y_addr: u64 = P3; xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); xmm2 = (crate::simd_ops::ru128(P4)) as u128; crate::simd_ops::wu64(rcx.wrapping_add(0x498u64), (((xmm4 >> 64) as u64)) as u64); crate::simd_ops::wu8(rcx.wrapping_add(0x4a0u64), ((rdx as u8)) as u8); rax = rcx; crate::simd_ops::wu64(rcx.wrapping_add(0x4b0u64), (0x3f800000u64) as u64); crate::simd_ops::wu64(rcx.wrapping_add(0x4b8u64), rdx); let y_addr: u64 = rcx.wrapping_add(0x4c0u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); crate::simd_ops::wu128(rcx.wrapping_add(0x4e0u64), (xmm2) as u128); let y_addr: u64 = P3; xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); xmm2 = (crate::simd_ops::ru128(P4)) as u128; crate::simd_ops::wu64(rcx.wrapping_add(0x4f0u64), (0x3f800000u64) as u64); crate::simd_ops::wu64(rcx.wrapping_add(0x4f8u64), rdx); let y_addr: u64 = rcx.wrapping_add(0x500u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); crate::simd_ops::wu128(rcx.wrapping_add(0x520u64), (xmm2) as u128); let y_addr: u64 = P3; xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); xmm2 = (crate::simd_ops::ru128(P4)) as u128; crate::simd_ops::wu64(rcx.wrapping_add(0x530u64), (0x3f800000u64) as u64); crate::simd_ops::wu64(rcx.wrapping_add(0x538u64), rdx); let y_addr: u64 = rcx.wrapping_add(0x540u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); let y_addr: u64 = P3; xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); crate::simd_ops::wu128(rcx.wrapping_add(0x560u64), (xmm2) as u128); xmm2 = (crate::simd_ops::ru128(P4)) as u128; crate::simd_ops::wu64(rcx.wrapping_add(0x570u64), (0x3f800000u64) as u64); crate::simd_ops::wu64(rcx.wrapping_add(0x578u64), rdx); let y_addr: u64 = rcx.wrapping_add(0x580u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); let y_addr: u64 = P3; xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); crate::simd_ops::wu128(rcx.wrapping_add(0x5a0u64), (xmm2) as u128); xmm2 = (crate::simd_ops::ru128(P4)) as u128; crate::simd_ops::wu64(rcx.wrapping_add(0x5b0u64), (0x3f800000u64) as u64); crate::simd_ops::wu64(rcx.wrapping_add(0x5b8u64), rdx); let y_addr: u64 = rcx.wrapping_add(0x5c0u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); let y_addr: u64 = P3; xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); crate::simd_ops::wu128(rcx.wrapping_add(0x5e0u64), (xmm2) as u128); xmm2 = (crate::simd_ops::ru128(P4)) as u128; crate::simd_ops::wu64(rcx.wrapping_add(0x5f0u64), (0x3f800000u64) as u64); crate::simd_ops::wu64(rcx.wrapping_add(0x5f8u64), rdx); let y_addr: u64 = rcx.wrapping_add(0x600u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); let y_addr: u64 = P3; xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); crate::simd_ops::wu128(rcx.wrapping_add(0x620u64), (xmm2) as u128); xmm2 = (crate::simd_ops::ru128(P4)) as u128; crate::simd_ops::wu64(rcx.wrapping_add(0x630u64), (0x3f800000u64) as u64); crate::simd_ops::wu64(rcx.wrapping_add(0x638u64), rdx); let y_addr: u64 = rcx.wrapping_add(0x640u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); let y_addr: u64 = P3; xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); crate::simd_ops::wu128(rcx.wrapping_add(0x660u64), (xmm2) as u128); xmm2 = (crate::simd_ops::ru128(P4)) as u128; crate::simd_ops::wu64(rcx.wrapping_add(0x670u64), (0x3f800000u64) as u64); crate::simd_ops::wu64(rcx.wrapping_add(0x678u64), rdx); let y_addr: u64 = rcx.wrapping_add(0x680u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); let y_addr: u64 = P3; xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); crate::simd_ops::wu128(rcx.wrapping_add(0x6a0u64), (xmm2) as u128); xmm2 = (crate::simd_ops::ru128(P4)) as u128; crate::simd_ops::wu64(rcx.wrapping_add(0x6b0u64), (0x3f800000u64) as u64); crate::simd_ops::wu64(rcx.wrapping_add(0x6b8u64), rdx); let y_addr: u64 = rcx.wrapping_add(0x6c0u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); let y_addr: u64 = P3; xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); crate::simd_ops::wu128(rcx.wrapping_add(0x6e0u64), (xmm2) as u128); xmm2 = (crate::simd_ops::ru128(P4)) as u128; crate::simd_ops::wu64(rcx.wrapping_add(0x6f0u64), (0x3f800000u64) as u64); crate::simd_ops::wu64(rcx.wrapping_add(0x6f8u64), rdx); let y_addr: u64 = rcx.wrapping_add(0x700u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); let y_addr: u64 = P3; xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); crate::simd_ops::wu128(rcx.wrapping_add(0x720u64), (xmm2) as u128); xmm2 = (crate::simd_ops::ru128(P4)) as u128; crate::simd_ops::wu64(rcx.wrapping_add(0x730u64), (0x3f800000u64) as u64); crate::simd_ops::wu64(rcx.wrapping_add(0x738u64), rdx); let y_addr: u64 = rcx.wrapping_add(0x740u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); let y_addr: u64 = P3; xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); crate::simd_ops::wu128(rcx.wrapping_add(0x760u64), (xmm2) as u128); xmm2 = (crate::simd_ops::ru128(P4)) as u128; crate::simd_ops::wu64(rcx.wrapping_add(0x770u64), (0x3f800000u64) as u64); crate::simd_ops::wu64(rcx.wrapping_add(0x778u64), rdx); let y_addr: u64 = rcx.wrapping_add(0x780u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); crate::simd_ops::wu128(rcx.wrapping_add(0x7a0u64), (xmm2) as u128); crate::simd_ops::wu64(rcx.wrapping_add(0x7b0u64), rdx); crate::simd_ops::wu64(rcx.wrapping_add(0x7b8u64), rdx); crate::simd_ops::wu64(rcx.wrapping_add(0x7c0u64), rdx); crate::simd_ops::wu64(rcx.wrapping_add(0x7c8u64), rdx); rsp = ((rsp).wrapping_add(0xa0u64)) as u64; rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1412ead60(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x40; const P1: u64 = 0x8; const P2: u64 = 0x10; const P3: u64 = 0x18; const P4: u64 = 0x20; const P5: u64 = 0x28; const P6: u64 = 0x30; const P7: u64 = 0x38; const P8: u64 = 0x48; const P9: u64 = 0x50; const P10: u64 = 0x58; const P11: u64 = 0x60;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 3>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x18u64)) as u64; r9 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rcx.wrapping_add(P0), (0xffffffffu32) as u32); crate::simd_ops::wu32(rcx, ((r9 as u32)) as u32); rdx = rsp; crate::simd_ops::wu64(rcx.wrapping_add(P1), r9); rax = ((r9 as u32)) as u32 as u64; crate::simd_ops::wu64(rcx.wrapping_add(P2), r9); crate::simd_ops::wu64(rcx.wrapping_add(P3), r9); crate::simd_ops::wu64(rcx.wrapping_add(P4), r9); crate::simd_ops::wu64(rcx.wrapping_add(P5), r9); crate::simd_ops::wu64(rcx.wrapping_add(P6), r9); crate::simd_ops::wu64(rcx.wrapping_add(P7), r9); crate::simd_ops::wu64(rcx.wrapping_add(P8), r9); crate::simd_ops::wu32(rcx.wrapping_add(P9), ((r9 as u32)) as u32); crate::simd_ops::wu64(rcx.wrapping_add(P10), r9); crate::simd_ops::wu8(rcx.wrapping_add(P11), ((r9 as u8)) as u8); crate::simd_ops::wu32(rcx.wrapping_add(0x64u64), ((r9 as u32)) as u32); crate::simd_ops::wu32(rsp, (0x1u32) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x4u64), (0x2u32) as u32); bb = 1; continue; }
            1 => { rax = ((((rax as u32)) | (crate::simd_ops::ru32(rdx)))) as u32 as u64; r8 = rsp.wrapping_add(0x8); rdx = ((rdx).wrapping_add(0x4u64)) as u64; crate::simd_ops::wu32(rcx.wrapping_add(0x64u64), ((rax as u32)) as u32); fl_l = rdx; fl_r = r8; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::wu64(rcx.wrapping_add(0x68u64), r9); rax = rcx; crate::simd_ops::wu64(rcx.wrapping_add(0x78u64), r9); crate::simd_ops::wu64(rcx.wrapping_add(0x70u64), r9); crate::simd_ops::wu64(rcx.wrapping_add(0x88u64), r9); crate::simd_ops::wu64(rcx.wrapping_add(0x80u64), r9); crate::simd_ops::wu64(rcx.wrapping_add(0x98u64), r9); crate::simd_ops::wu64(rcx.wrapping_add(0x90u64), r9); crate::simd_ops::wu64(rcx.wrapping_add(0xa8u64), r9); crate::simd_ops::wu64(rcx.wrapping_add(0xa0u64), r9); crate::simd_ops::wu64(rcx.wrapping_add(0xb8u64), r9); crate::simd_ops::wu64(rcx.wrapping_add(0xb0u64), r9); crate::simd_ops::wu64(rcx.wrapping_add(0xc8u64), r9); crate::simd_ops::wu64(rcx.wrapping_add(0xc0u64), r9); crate::simd_ops::wu64(rcx.wrapping_add(0xd8u64), r9); rsp = ((rsp).wrapping_add(0x18u64)) as u64; return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1412eae40(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0x18; const P3: u64 = 0x1c; const P4: u64 = 0x20;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 3>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x18u64)) as u64;
    r9 = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu32(rsp, (0x1u32) as u32);
    crate::simd_ops::wu64(rcx, r9);
    rdx = rsp;
    crate::simd_ops::wu32(rcx.wrapping_add(P0), ((r9 as u32)) as u32);
    rax = ((r9 as u32)) as u32 as u64;
    crate::simd_ops::wu64(rcx.wrapping_add(P1), r9);
    crate::simd_ops::wu8(rcx.wrapping_add(P2), ((r9 as u8)) as u8);
    crate::simd_ops::wu32(rcx.wrapping_add(P3), ((r9 as u32)) as u32);
    crate::simd_ops::wu32(rsp.wrapping_add(0x4u64), (0x2u32) as u32);
    loop { rax = ((((rax as u32)) | (crate::simd_ops::ru32(rdx)))) as u32 as u64; r8 = rsp.wrapping_add(0x8); rdx = ((rdx).wrapping_add(0x4u64)) as u64; crate::simd_ops::wu32(rcx.wrapping_add(P3), ((rax as u32)) as u32); fl_l = rdx; fl_r = r8; if !((((fl_l as u64)) != ((fl_r as u64)))) { break; } }
    crate::simd_ops::wu64(rcx.wrapping_add(P4), r9);
    rax = rcx;
    rsp = ((rsp).wrapping_add(0x18u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1412eaef0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x20; const P1: u64 = 0x10; const P2: u64 = 0x18; const P3: u64 = 0xfffffffffffffff8; const P4: u64 = 0x142535070; const P5: u64 = 0x1426456c8; const P6: u64 = 0x1426453a0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rax = ((crate::exe::tls_pointer() as u64)) as u64; rdi = rcx; rcx = 0x6348; rbx = 0; fl_l = 0; fl_r = 0; rdx = (crate::simd_ops::ru64(rax)) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(rdx))) as u64; r8 = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rdx = (crate::simd_ops::ru64(r8.wrapping_add(P1))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 1; continue; }
            1 => { fl_l = ((rdx as u8)) as u8 as u64; fl_r = (0xfu8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 5; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::lk_add_u32(r8.wrapping_add(P2), (1) as u32)) as u32 as u64; rax = (crate::simd_ops::ru64(rdx)) as u64; crate::simd_ops::wu64(r8.wrapping_add(P1), rax); bb = 3; continue; }
            3 => { rbx = rdx; bb = 4; continue; }
            4 => { rax = 0; fl_l = 0; fl_r = 0; xmm0 = 0u128; crate::simd_ops::wu128(rbx, (xmm0) as u128); crate::simd_ops::wu64(rbx.wrapping_add(P1), rax); rax = rdi; crate::simd_ops::wu64(rdi, rbx); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            5 => { rdx = 0x18; r8 = ((rdx.wrapping_add(P3)) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 3; continue; } bb = 6; continue; }
            6 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rax; rdx = 0xdeadd00d; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P6), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 4; continue; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1412eaf80(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142535070; const P1: u64 = 0x1426456c8; const P2: u64 = 0x1426453a0; const P3: u64 = 0x142409b50; const P4: u64 = 0x142327360; const P5: u64 = 0x14240ab10;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 31>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0xe0u64)) as u64; r14 = r9; r15 = r8; rdi = rcx; rcx = 0x6348; rax = ((crate::exe::tls_pointer() as u64)) as u64; rdx = (crate::simd_ops::ru64(rax)) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(rdx))) as u64; rdx = (crate::simd_ops::ru64(rcx.wrapping_add(0x138u64))) as u64; rbx = (crate::simd_ops::ru64(rdx.wrapping_add(0x10u64))) as u64; rax = rbx; rax = ((((rax as u32)) & (0xfu32))) as u32 as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::lk_add_u32(rdx.wrapping_add(0x18u64), (1) as u32)) as u32 as u64; rax = (crate::simd_ops::ru64(rbx)) as u64; crate::simd_ops::wu64(rdx.wrapping_add(0x10u64), rax); bb = 5; continue; }
            3 => { rdx = 0x130; r8 = 0x10; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rbx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp); rcx = rax; rdx = 0xdeadd00d; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); bb = 5; continue; }
            5 => { crate::simd_ops::wu64(rsp.wrapping_add(0x108u64), rbx); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x120u64))) as u64; rsi = (crate::simd_ops::ru32(rax)) as u32 as u64; rax = (crate::simd_ops::ru64(r14)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), (0x10000010u64) as u64); crate::simd_ops::wu8(rsp.wrapping_add(0x58u64), (0x0u8) as u8); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 7; continue; } bb = 6; continue; }
            6 => { crate::simd_ops::wu32(rsp.wrapping_add(0x54u64), ((rax as u32)) as u32); crate::simd_ops::wu8(rsp.wrapping_add(0x58u64), ((rax as u8)) as u8); bb = 10; continue; }
            7 => { r9 = (0xffffffffffffffffu64) as u64; bb = 8; continue; }
            8 => { r9 = ((r9).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru8(rax.wrapping_add(r9))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 8; continue; } bb = 9; continue; }
            9 => { crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), r9); r8 = 0; fl_l = 0; fl_r = 0; rdx = rsp.wrapping_add(0x40); rcx = rsp.wrapping_add(0x50); rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 10; continue; }
            10 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x128u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); r9 = ((rsi as u32)) as u32 as u64; rdx = (crate::simd_ops::ru64(r15)) as u64; rcx = rbx; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rdi, rax); rcx = rsp.wrapping_add(0x50); rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rax = rdi; r11 = rsp.wrapping_add(0xe0); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x20u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x30u64))) as u64; rsp = r11; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1412eb160(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x18; const P1: u64 = 0x14209d670; const P2: u64 = 0x58; const P3: u64 = 0x1411e7fc0; const P4: u64 = 0x142442de0; const P5: u64 = 0x8;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = rcx; rbx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rcx = rbx; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rcx = ((rbx.wrapping_add(P2)) as u64) as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rcx = rbx; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); bb = 2; continue; }
            2 => { rbx = (crate::simd_ops::ru64(rdi.wrapping_add(P5))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { rcx = rbx; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rcx = ((rbx.wrapping_add(P2)) as u64) as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rcx = rbx; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); bb = 4; continue; }
            4 => { rbx = (crate::simd_ops::ru64(rdi)) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { rcx = rbx; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rcx = ((rbx.wrapping_add(P2)) as u64) as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rcx = rbx; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); bb = 6; continue; }
            6 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1412eb1f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x28; const P2: u64 = 0x10; const P3: u64 = 0x48; const P4: u64 = 0x18; const P5: u64 = 0x68; const P6: u64 = 0x30; const P7: u64 = 0x50; const P8: u64 = 0x70; const P9: u64 = 0x38; const P10: u64 = 0x58; const P11: u64 = 0x78;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut xmm4: u128 = 0; let mut xmm5: u128 = 0; let mut xmm6: u128 = 0; let mut xmm7: u128 = 0; let mut xmm8: u128 = 0; let mut xmm9: u128 = 0; let mut xmm10: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x58u64)) as u64; crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm6) as u128); rax = rcx; crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm7) as u128); xmm7 = ((crate::simd_ops::ru64(rcx)) as u64) as u128; xmm1 = (xmm7 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm7 as u64) * f64::from_bits(crate::simd_ops::ru64(rdx.wrapping_add(P0)))).to_bits() as u128); crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm8) as u128); xmm8 = ((crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64) as u128; xmm0 = (xmm8 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm8 as u64) * f64::from_bits(crate::simd_ops::ru64(rdx.wrapping_add(P1)))).to_bits() as u128); xmm2 = (xmm1 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm1 as u64) + f64::from_bits(xmm0 as u64)).to_bits() as u128); crate::simd_ops::wu128(rsp.wrapping_add(0x10u64), (xmm9) as u128); xmm9 = ((crate::simd_ops::ru64(rcx.wrapping_add(P2))) as u64) as u128; xmm1 = (xmm9 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm9 as u64) * f64::from_bits(crate::simd_ops::ru64(rdx.wrapping_add(P3)))).to_bits() as u128); xmm3 = (xmm2 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm2 as u64) + f64::from_bits(xmm1 as u64)).to_bits() as u128); xmm1 = (xmm7 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm7 as u64) * f64::from_bits(crate::simd_ops::ru64(rdx.wrapping_add(P2)))).to_bits() as u128); crate::simd_ops::wu128(rsp, (xmm10) as u128); xmm10 = ((crate::simd_ops::ru64(rcx.wrapping_add(P4))) as u64) as u128; xmm0 = (xmm10 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm10 as u64) * f64::from_bits(crate::simd_ops::ru64(rdx.wrapping_add(P5)))).to_bits() as u128); xmm6 = (xmm3 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm3 as u64) + f64::from_bits(xmm0 as u64)).to_bits() as u128); xmm0 = (xmm8 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm8 as u64) * f64::from_bits(crate::simd_ops::ru64(rdx.wrapping_add(P6)))).to_bits() as u128); xmm2 = (xmm1 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm1 as u64) + f64::from_bits(xmm0 as u64)).to_bits() as u128); xmm1 = (xmm9 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm9 as u64) * f64::from_bits(crate::simd_ops::ru64(rdx.wrapping_add(P7)))).to_bits() as u128); xmm0 = (xmm10 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm10 as u64) * f64::from_bits(crate::simd_ops::ru64(rdx.wrapping_add(P8)))).to_bits() as u128); xmm3 = (xmm2 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm2 as u64) + f64::from_bits(xmm1 as u64)).to_bits() as u128); xmm1 = (xmm7 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm7 as u64) * f64::from_bits(crate::simd_ops::ru64(rdx.wrapping_add(P4)))).to_bits() as u128); xmm5 = (xmm3 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm3 as u64) + f64::from_bits(xmm0 as u64)).to_bits() as u128); xmm0 = (xmm8 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm8 as u64) * f64::from_bits(crate::simd_ops::ru64(rdx.wrapping_add(P9)))).to_bits() as u128); xmm2 = (xmm1 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm1 as u64) + f64::from_bits(xmm0 as u64)).to_bits() as u128); xmm1 = (xmm9 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm9 as u64) * f64::from_bits(crate::simd_ops::ru64(rdx.wrapping_add(P10)))).to_bits() as u128); xmm0 = (xmm10 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm10 as u64) * f64::from_bits(crate::simd_ops::ru64(rdx.wrapping_add(P11)))).to_bits() as u128); xmm3 = (xmm2 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm2 as u64) + f64::from_bits(xmm1 as u64)).to_bits() as u128); xmm1 = (xmm8 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm8 as u64) * f64::from_bits(crate::simd_ops::ru64(rdx.wrapping_add(0x20u64)))).to_bits() as u128); xmm8 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; xmm4 = (xmm3 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm3 as u64) + f64::from_bits(xmm0 as u64)).to_bits() as u128); xmm0 = (xmm7 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm7 as u64) * f64::from_bits(crate::simd_ops::ru64(rdx))).to_bits() as u128); xmm7 = (crate::simd_ops::ru128(rsp.wrapping_add(0x30u64))) as u128; xmm2 = (xmm1 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm1 as u64) + f64::from_bits(xmm0 as u64)).to_bits() as u128); xmm1 = (xmm9 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm9 as u64) * f64::from_bits(crate::simd_ops::ru64(rdx.wrapping_add(0x40u64)))).to_bits() as u128); xmm0 = (xmm10 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm10 as u64) * f64::from_bits(crate::simd_ops::ru64(rdx.wrapping_add(0x60u64)))).to_bits() as u128); xmm9 = (crate::simd_ops::ru128(rsp.wrapping_add(0x10u64))) as u128; xmm10 = (crate::simd_ops::ru128(rsp)) as u128; xmm3 = (xmm2 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm2 as u64) + f64::from_bits(xmm1 as u64)).to_bits() as u128); xmm1 = (xmm3 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm3 as u64) + f64::from_bits(xmm0 as u64)).to_bits() as u128); crate::simd_ops::wu64(rcx.wrapping_add(0x8u64), ((xmm6 as u64)) as u64); xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x40u64))) as u128; crate::simd_ops::wu64(rcx, ((xmm1 as u64)) as u64); crate::simd_ops::wu64(rcx.wrapping_add(0x10u64), ((xmm5 as u64)) as u64); crate::simd_ops::wu64(rcx.wrapping_add(0x18u64), ((xmm4 as u64)) as u64); rsp = ((rsp).wrapping_add(0x58u64)) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1412eb320(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x18; const P1: u64 = 0x14209d670; const P2: u64 = 0x58; const P3: u64 = 0x1411e7fc0; const P4: u64 = 0x142442de0; const P5: u64 = 0x10; const P6: u64 = 0x8;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = rcx; rbx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rcx = rbx; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rcx = ((rbx.wrapping_add(P2)) as u64) as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rcx = rbx; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); bb = 2; continue; }
            2 => { rbx = (crate::simd_ops::ru64(rdi.wrapping_add(P5))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { rcx = rbx; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rcx = ((rbx.wrapping_add(P2)) as u64) as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rcx = rbx; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); bb = 4; continue; }
            4 => { rbx = (crate::simd_ops::ru64(rdi.wrapping_add(P6))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { rcx = rbx; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rcx = ((rbx.wrapping_add(P2)) as u64) as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rcx = rbx; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); bb = 6; continue; }
            6 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1412eb400(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14222a2a0; const P1: u64 = 0x30; const P2: u64 = 0x1433f11b8; const P3: u64 = 0x38; const P4: u64 = 0x40; const P5: u64 = 0x44; const P6: u64 = 0x48; const P7: u64 = 0x50; const P8: u64 = 0x60; const P9: u64 = 0x70; const P10: u64 = 0x80; const P11: u64 = 0x78; const P12: u64 = 0x1433f1168;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rcx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu8(rbx.wrapping_add(P1), (0x0u8) as u8); rax = P2; crate::simd_ops::wu64(rbx, rax); rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rbx.wrapping_add(P3), rcx); crate::simd_ops::wu32(rbx.wrapping_add(P4), ((rcx as u32)) as u32); crate::simd_ops::wu8(rbx.wrapping_add(P5), ((rcx as u8)) as u8); crate::simd_ops::wu64(rbx.wrapping_add(P6), rcx); crate::simd_ops::wu32(rbx.wrapping_add(P7), ((rcx as u32)) as u32); crate::simd_ops::wu64(rbx.wrapping_add(P8), (0x4u64) as u64); crate::simd_ops::wu64(rbx.wrapping_add(P9), rcx); crate::simd_ops::wu64(rbx.wrapping_add(P10), rcx); crate::simd_ops::wu64(rbx.wrapping_add(P11), (0xcu64) as u64); crate::simd_ops::wu64(rbx.wrapping_add(0x88u64), rcx); rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x60u64))) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x58u64), (0x4000u64) as u64); rax = ((rax).wrapping_shl(14u32)) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x68u64), rax); rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x78u64))) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x70u64), (0x800u64) as u64); rax = ((rax).wrapping_shl(11u32)) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x80u64), rax); rax = P12; crate::simd_ops::wu64(rbx, rax); rax = rbx; crate::simd_ops::wu64(rbx.wrapping_add(0x90u64), rcx); xmm0 = 0u128; ymm0_hi = 0; let y_addr: u64 = rbx.wrapping_add(0x98u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); crate::simd_ops::wu64(rbx.wrapping_add(0x98u64), rcx); crate::simd_ops::wu64(rbx.wrapping_add(0xa0u64), rcx); crate::simd_ops::wu64(rbx.wrapping_add(0xa8u64), rcx); crate::simd_ops::wu64(rbx.wrapping_add(0xb0u64), rcx); crate::simd_ops::wu64(rbx.wrapping_add(0xb8u64), rcx); crate::simd_ops::wu128(rbx.wrapping_add(0xc0u64), (xmm0) as u128); crate::simd_ops::wu64(rbx.wrapping_add(0xc0u64), rcx); crate::simd_ops::wu64(rbx.wrapping_add(0xc8u64), rcx); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1412eb4e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x3f; const P2: u64 = 0x14126eeb0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x40u64)) as u64;
    rdx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    rbx = rcx;
    crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rdx);
    rax = ((rdx.wrapping_add(P1)) as u64) as u64;
    rdx = 0;
    fl_l = 0; fl_r = 0;
    rax = ((rax).wrapping_shr(6u32)) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax);
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    rcx = rsp.wrapping_add(0x20);
    crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax);
    rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp);
    rax = rbx;
    rsp = ((rsp).wrapping_add(0x40u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1412eb590(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0xffffffffffffffff; const P1: u64 = 0x1;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rdx = 0;
    fl_l = 0; fl_r = 0;
    fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x1;
    if (((fl_l as u32)) <= ((fl_r as u32))) { rax = 0x1; rax = (((rax as u32)).wrapping_shl(((((rdx as u32)) as u32) & 31))) as u32 as u64; return rax; }
    rax = ((rcx.wrapping_add(P0)) as u32) as u32 as u64;
    let bit_in: u32 = (rax as u32);
    if bit_in != 0 { rcx = (((31u32.wrapping_sub(bit_in.leading_zeros())) as u32)) as u32 as u64; }
    fl_l = bit_in as u64;
    if !((((fl_l as u32)) == 0)) { rdx = ((rcx.wrapping_add(P1)) as u32) as u32 as u64; }
    rax = 0x1;
    rax = (((rax as u32)).wrapping_shl(((((rdx as u32)) as u32) & 31))) as u32 as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1412ebc10(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x20; const P1: u64 = 0x29; const P2: u64 = 0x10; const P3: u64 = 0x8; const P4: u64 = 0x14222b6e0;
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
    if !((((fl_l as u8)) != ((fl_r as u8)))) { rdx = (crate::simd_ops::ru64(rcx.wrapping_add(P2))) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P3))) as u64; crate::simd_ops::wu8(rsp.wrapping_add(0x20u64), (0x0u8) as u8); rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu32(rbx.wrapping_add(P0), (0x0u32) as u32); }
    rsp = ((rsp).wrapping_add(0x30u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1412ebc50(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x29; const P1: u64 = 0x20; const P2: u64 = 0x10; const P3: u64 = 0x8; const P4: u64 = 0x14222b6e0;
    let (mut rax, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x30u64)) as u64;
    rax = ((crate::simd_ops::ru8(rcx.wrapping_add(P0))) as u32) as u32 as u64;
    rbx = rcx;
    r8 = (crate::simd_ops::ru32(rcx.wrapping_add(P1))) as u32 as u64;
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
pub unsafe extern "win64" fn fn_1412ebe80(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0xc; const P2: u64 = 0x1401deb40; const P3: u64 = 0x14183acc2;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rdx; rdi = rcx; rsi = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(P1))) as u32 as u64; fl_l = rdx; fl_r = rax; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 3; continue; } bb = 1; continue; }
            1 => { rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); r8 = rbx; fl_l = r8; fl_r = rsi; r8 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 6; continue; } bb = 2; continue; }
            2 => { r8 = ((r8.wrapping_mul(4)) as u64) as u64; rcx = (crate::simd_ops::ru32(rdi.wrapping_add(P0))) as u32 as u64; rax = (crate::simd_ops::ru64(rdi)) as u64; rcx = ((rax.wrapping_add(rcx.wrapping_mul(4))) as u64) as u64; rdx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu32(rdi.wrapping_add(P0), ((rbx as u32)) as u32); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            3 => { fl_l = rbx; fl_r = rsi; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 7; continue; } bb = 4; continue; }
            4 => { r8 = rbx; fl_l = r8; fl_r = rsi; r8 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 6; continue; } bb = 5; continue; }
            5 => { r8 = ((r8.wrapping_mul(4)) as u64) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rcx = ((rax.wrapping_add(rsi.wrapping_mul(4))) as u64) as u64; rdx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 6; continue; }
            6 => { crate::simd_ops::wu32(rdi.wrapping_add(P0), ((rbx as u32)) as u32); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            7 => { if (((fl_l as u64)) == ((fl_r as u64))) { bb = 9; continue; } bb = 8; continue; }
            8 => { crate::simd_ops::wu32(rcx.wrapping_add(P0), ((rbx as u32)) as u32); bb = 9; continue; }
            9 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1412ebf30(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1401de830; const P1: u64 = 0x142442de0; const P2: u64 = 0x1425371a0; const P3: u64 = 0x142533200;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = rcx; rbx = (crate::simd_ops::ru64(rcx)) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x8u64))) as u32 as u64; rsi = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rsi = ((rsi).wrapping_shl(5u32)) as u64; rsi = ((rsi).wrapping_add(rbx)) as u64; fl_l = rbx; fl_r = rsi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 5; continue; } bb = 1; continue; }
            1 => { bb = 2; continue; }
            2 => { rcx = rbx.wrapping_add(0x80); rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rcx = rbx.wrapping_add(0x28); rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rcx = rbx.wrapping_add(0x18); rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rbx)) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 4; continue; }
            4 => { rbx = ((rbx).wrapping_add(0xa0u64)) as u64; fl_l = rbx; fl_r = rsi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 5; continue; }
            5 => { r9 = (crate::simd_ops::ru64(rdi)) as u64; fl_l = r9; fl_r = r9; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 13; continue; } bb = 6; continue; }
            6 => { rcx = r9.wrapping_sub(0x1); rcx = (((rcx) & (0xfffffffffe000000u64))) as u64; rax = r9; rax = ((rax).wrapping_sub(rcx)) as u64; rax = ((rax).wrapping_shr(16u32)) as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rdx = rcx.wrapping_add(0x108); rdx = ((rdx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x4u64))) as u32 as u64; rdx = ((rdx).wrapping_sub(rax)) as u64; rax = ((crate::exe::teb_self() as u64)) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(rcx.wrapping_add(0x100u64))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 11; continue; } bb = 7; continue; }
            7 => { fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(0xeu64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 10; continue; } bb = 8; continue; }
            8 => { rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(r9, rax); crate::simd_ops::wu64(rdx.wrapping_add(0x20u64), r9); fl_l = (crate::simd_ops::lk_add_u32(rdx.wrapping_add(0x18u64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; if (((fl_l as u32)) != 0) { bb = 13; continue; } bb = 9; continue; }
            9 => { rcx = rdx; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 13; continue; }
            10 => { r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 12; continue; }
            11 => { r8 = 0; fl_l = 0; fl_r = 0; bb = 12; continue; }
            12 => { rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 13; continue; }
            13 => { rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rdi, rax); crate::simd_ops::wu64(rdi.wrapping_add(0x8u64), rax); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1412ec030(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x30; const P2: u64 = 0x1401de830; const P3: u64 = 0x142442de0; const P4: u64 = 0xffffffffffffffff; const P5: u64 = 0x108; const P6: u64 = 0x4; const P7: u64 = 0x100; const P8: u64 = 0xe; const P9: u64 = 0x20; const P10: u64 = 0x18; const P11: u64 = 0x1425371a0; const P12: u64 = 0x142533200;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = rcx; rbx = (crate::simd_ops::ru64(rcx)) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; rsi = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rsi = ((rsi).wrapping_shl(4u32)) as u64; rsi = ((rsi).wrapping_add(rbx)) as u64; fl_l = rbx; fl_r = rsi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 5; continue; } bb = 1; continue; }
            1 => { bb = 2; continue; }
            2 => { rcx = ((rbx.wrapping_add(P1)) as u64) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rbx)) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 4; continue; }
            4 => { rbx = ((rbx).wrapping_add(0x50u64)) as u64; fl_l = rbx; fl_r = rsi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 5; continue; }
            5 => { r9 = (crate::simd_ops::ru64(rdi)) as u64; fl_l = r9; fl_r = r9; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 13; continue; } bb = 6; continue; }
            6 => { rcx = ((r9.wrapping_add(P4)) as u64) as u64; rcx = (((rcx) & (0xfffffffffe000000u64))) as u64; rax = r9; rax = ((rax).wrapping_sub(rcx)) as u64; rax = ((rax).wrapping_shr(16u32)) as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rdx = ((rcx.wrapping_add(P5)) as u64) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(P6))) as u32 as u64; rdx = ((rdx).wrapping_sub(rax)) as u64; rax = ((crate::exe::teb_self() as u64)) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(rcx.wrapping_add(P7))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 11; continue; } bb = 7; continue; }
            7 => { fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(P8))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 10; continue; } bb = 8; continue; }
            8 => { rax = (crate::simd_ops::ru64(rdx.wrapping_add(P9))) as u64; crate::simd_ops::wu64(r9, rax); crate::simd_ops::wu64(rdx.wrapping_add(P9), r9); fl_l = (crate::simd_ops::lk_add_u32(rdx.wrapping_add(P10), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; if (((fl_l as u32)) != 0) { bb = 13; continue; } bb = 9; continue; }
            9 => { rcx = rdx; rax = crate::simd_ops::call_stk(P11, rax, rcx, rdx, r8, r9, rsp); bb = 13; continue; }
            10 => { r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 12; continue; }
            11 => { r8 = 0; fl_l = 0; fl_r = 0; bb = 12; continue; }
            12 => { rax = crate::simd_ops::call_stk(P12, rax, rcx, rdx, r8, r9, rsp); bb = 13; continue; }
            13 => { rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rdi, rax); crate::simd_ops::wu64(rdi.wrapping_add(0x8u64), rax); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1412ec110(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143868280; const P1: u64 = 0x1411db210; const P2: u64 = 0x14209d670; const P3: u64 = 0x142075af0; const P4: u64 = 0x142075bc0; const P5: u64 = 0x1411e7fc0; const P6: u64 = 0x142442de0;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 19>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x80u64)) as u64; r14 = rcx; rbp = (crate::simd_ops::ru64(r8.wrapping_add(0x8u64))) as u64; rax = rbp; rax = (((rax) as i64).wrapping_mul((rdx) as i64) as u64) as u64; rdi = (crate::simd_ops::ru64(r8.wrapping_add(0x10u64))) as u64; fl_l = rax; fl_r = rdi; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rcx = ((rdi.wrapping_add(rax)) as u64) as u64; rdx = 0; fl_l = 0; fl_r = 0; rax = rcx.wrapping_sub(0x1); let d = (rdi) as u128; let n = ((rdx as u128) << 64) | (rax as u128); if d != 0 { rax = (n / d) as u64; rdx = (n % d) as u64; } rdi = rcx.wrapping_sub(0x1); fl_l = rdi; fl_r = rdx; rdi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; bb = 2; continue; }
            2 => { rsi = (crate::simd_ops::ru64(r14)) as u64; rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x10u64))) as u64; fl_l = rax; fl_r = rdi; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 5; continue; } bb = 3; continue; }
            3 => { fl_l = (crate::simd_ops::ru8(P0)) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 13; continue; } bb = 4; continue; }
            4 => { fl_l = rax; fl_r = rdi; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 13; continue; } bb = 5; continue; }
            5 => { fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 6; continue; }
            6 => { rcx = rsp.wrapping_add(0xa0); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0xa0u64))) as u64; rcx = rsi; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu32(rsp.wrapping_add(0x40u64), (0x1u32) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x44u64), (0x10u32) as u32); rbx = 0; fl_l = 0; fl_r = 0; r9 = ((rbx as u32)) as u32 as u64; rax = rsp.wrapping_add(0x40); bb = 7; continue; }
            7 => { r9 = ((((r9 as u32)) | (crate::simd_ops::ru32(rax)))) as u32 as u64; rax = ((rax).wrapping_add(0x4u64)) as u64; rcx = rsp.wrapping_add(0x48); fl_l = rax; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 7; continue; } bb = 8; continue; }
            8 => { crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rsi); crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rbx as u32)) as u32); r8 = 0; fl_l = 0; fl_r = 0; rdx = rbp; rcx = rdi; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); fl_l = rdi; fl_r = (0x400000u64) as u64; rbx = (rbx & !0xffu64) | (((if (((fl_l as u64)) > ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); rbx = (((rbx as u32)).wrapping_add(0x2u32)) as u32 as u64; crate::simd_ops::wu32(rsi.wrapping_add(0x50u64), ((rbx as u32)) as u32); r8 = (crate::simd_ops::ru64(r14)) as u64; rax = (crate::simd_ops::ru64(r8.wrapping_add(0x10u64))) as u64; fl_l = rdi; fl_r = rax; rax = (if (((fl_l as u64)) < ((fl_r as u64))) { rdi } else { rax }) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); r9 = 0; fl_l = 0; fl_r = 0; rdx = 0; fl_l = 0; fl_r = 0; rcx = rsi; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(r14)) as u64; crate::simd_ops::wu64(r14, rsi); fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 13; continue; } bb = 9; continue; }
            9 => { rcx = rbx; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rcx = rbx.wrapping_add(0x58); rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rcx = rbx; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); bb = 13; continue; }
            10 => { rcx = rsi; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu32(rsp.wrapping_add(0x60u64), (0x1u32) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x64u64), (0x10u32) as u32); rbx = 0; fl_l = 0; fl_r = 0; r9 = ((rbx as u32)) as u32 as u64; rax = rsp.wrapping_add(0x60); bb = 11; continue; }
            11 => { r9 = ((((r9 as u32)) | (crate::simd_ops::ru32(rax)))) as u32 as u64; rax = ((rax).wrapping_add(0x4u64)) as u64; rcx = rsp.wrapping_add(0x68); fl_l = rax; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 11; continue; } bb = 12; continue; }
            12 => { crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rsi); crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rbx as u32)) as u32); r8 = 0; fl_l = 0; fl_r = 0; rdx = rbp; rcx = rdi; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); fl_l = rdi; fl_r = (0x400000u64) as u64; rbx = (rbx & !0xffu64) | (((if (((fl_l as u64)) > ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); fl_l = (((rbx as u32)).wrapping_add(0x2u32)) as u32 as u64; fl_r = fl_l; rbx = ((fl_l as u32)) as u32 as u64; crate::simd_ops::wu32(rsi.wrapping_add(0x50u64), ((rbx as u32)) as u32); bb = 13; continue; }
            13 => { r11 = rsp.wrapping_add(0x80); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x28u64))) as u64; rbp = (crate::simd_ops::ru64(r11.wrapping_add(0x30u64))) as u64; rsp = r11; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1412ec400(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142537280; const P1: u64 = 0x142535070; const P2: u64 = 0x1426456c8; const P3: u64 = 0x1426453a0; const P4: u64 = 0x1412ebf30;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0; let mut ymm1_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rsi = rdx; rdi = rcx; rbp = (crate::simd_ops::ru32(rcx.wrapping_add(0x8u64))) as u32 as u64; rdx = ((rdx.wrapping_add(rdx.wrapping_mul(4))) as u64) as u64; rdx = ((rdx).wrapping_shl(5u32)) as u64; r10 = 0x6348; rax = ((crate::exe::tls_pointer() as u64)) as u64; r8 = (crate::simd_ops::ru64(rax)) as u64; r10 = (crate::simd_ops::ru64(r10.wrapping_add(r8))) as u64; r14 = 0; fl_l = 0; fl_r = 0; fl_l = ((rdx) & ((rdx).wrapping_sub(1))) as u64; rax = ((fl_l as u64)) as u64; if (((fl_l as u64)) != 0) { bb = 5; continue; } bb = 1; continue; }
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
            15 => { r9 = (crate::simd_ops::ru64(rdi)) as u64; rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x8u64))) as u32 as u64; r8 = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; r8 = ((r8).wrapping_shl(5u32)) as u64; r8 = ((r8).wrapping_add(r9)) as u64; fl_l = r9; fl_r = r8; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 18; continue; } bb = 16; continue; }
            16 => { rdx = r9.wrapping_add(0x24); rcx = rbx.wrapping_add(0x24); bb = 17; continue; }
            17 => { rax = (crate::simd_ops::ru64(r9)) as u64; crate::simd_ops::wu64(r9, r14); crate::simd_ops::wu64(rcx.wrapping_sub(0x24u64), rax); rax = (crate::simd_ops::ru64(rdx.wrapping_sub(0x1cu64))) as u64; crate::simd_ops::wu64(rcx.wrapping_sub(0x1cu64), rax); rax = (crate::simd_ops::ru64(rdx.wrapping_sub(0x14u64))) as u64; crate::simd_ops::wu64(rcx.wrapping_sub(0x14u64), rax); crate::simd_ops::wu64(rcx.wrapping_sub(0xcu64), r14); crate::simd_ops::wu64(rcx.wrapping_sub(0x4u64), r14); rax = (crate::simd_ops::ru64(rdx.wrapping_sub(0xcu64))) as u64; crate::simd_ops::wu64(rcx.wrapping_sub(0xcu64), rax); rax = (crate::simd_ops::ru32(rdx.wrapping_sub(0x4u64))) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_sub(0x4u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdx)) as u32 as u64; crate::simd_ops::wu32(rcx, ((rax as u32)) as u32); crate::simd_ops::wu64(rdx.wrapping_sub(0xcu64), r14); crate::simd_ops::wu64(rdx.wrapping_sub(0x4u64), r14); crate::simd_ops::wu64(rcx.wrapping_add(0x4u64), r14); crate::simd_ops::wu64(rcx.wrapping_add(0xcu64), r14); rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x4u64))) as u64; crate::simd_ops::wu64(rcx.wrapping_add(0x4u64), rax); rax = (crate::simd_ops::ru32(rdx.wrapping_add(0xcu64))) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(0xcu64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x10u64))) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(0x10u64), ((rax as u32)) as u32); crate::simd_ops::wu64(rdx.wrapping_add(0x4u64), r14); crate::simd_ops::wu64(rdx.wrapping_add(0xcu64), r14); rax = ((crate::simd_ops::ru16(rdx.wrapping_add(0x14u64))) as u32) as u32 as u64; crate::simd_ops::wu16(rcx.wrapping_add(0x14u64), ((rax as u16)) as u16); rax = ((crate::simd_ops::ru16(rdx.wrapping_add(0x16u64))) as u32) as u32 as u64; crate::simd_ops::wu16(rcx.wrapping_add(0x16u64), ((rax as u16)) as u16); let y_addr: u64 = rdx.wrapping_add(0x1cu64); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rcx.wrapping_add(0x1cu64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); let y_addr: u64 = rdx.wrapping_add(0x3cu64); xmm1 = crate::simd_ops::ru128(y_addr); ymm1_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rcx.wrapping_add(0x3cu64); crate::simd_ops::wu128(y_addr, xmm1); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm1_hi); crate::simd_ops::wu64(rcx.wrapping_add(0x5cu64), r14); crate::simd_ops::wu64(rcx.wrapping_add(0x64u64), r14); rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x5cu64))) as u64; crate::simd_ops::wu64(rcx.wrapping_add(0x5cu64), rax); rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x64u64))) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(0x64u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x68u64))) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(0x68u64), ((rax as u32)) as u32); crate::simd_ops::wu64(rdx.wrapping_add(0x5cu64), r14); crate::simd_ops::wu64(rdx.wrapping_add(0x64u64), r14); rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x6cu64))) as u64; crate::simd_ops::wu64(rcx.wrapping_add(0x6cu64), rax); rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x74u64))) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(0x74u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x78u64))) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(0x78u64), ((rax as u32)) as u32); rcx = rcx.wrapping_add(0xa0); r9 = ((r9).wrapping_add(0xa0u64)) as u64; rdx = rdx.wrapping_add(0xa0); fl_l = r9; fl_r = r8; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 17; continue; } bb = 18; continue; }
            18 => { rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rdi, rbx); crate::simd_ops::wu32(rdi.wrapping_add(0x8u64), ((rbp as u32)) as u32); crate::simd_ops::wu32(rdi.wrapping_add(0xcu64), ((rsi as u32)) as u32); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1412ec640(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142537280; const P1: u64 = 0x142535070; const P2: u64 = 0x1426456c8; const P3: u64 = 0x1426453a0; const P4: u64 = 0x1412ec030;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rsi = rdx; rdi = rcx; rbp = (crate::simd_ops::ru32(rcx.wrapping_add(0x8u64))) as u32 as u64; rdx = ((rdx.wrapping_add(rdx.wrapping_mul(4))) as u64) as u64; rdx = ((rdx).wrapping_shl(4u32)) as u64; r10 = 0x6348; rax = ((crate::exe::tls_pointer() as u64)) as u64; r8 = (crate::simd_ops::ru64(rax)) as u64; r10 = (crate::simd_ops::ru64(r10.wrapping_add(r8))) as u64; r14 = 0; fl_l = 0; fl_r = 0; fl_l = ((rdx) & ((rdx).wrapping_sub(1))) as u64; rax = ((fl_l as u64)) as u64; if (((fl_l as u64)) != 0) { bb = 5; continue; } bb = 1; continue; }
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
            15 => { r8 = (crate::simd_ops::ru64(rdi)) as u64; rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x8u64))) as u32 as u64; r9 = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; r9 = ((r9).wrapping_shl(4u32)) as u64; r9 = ((r9).wrapping_add(r8)) as u64; fl_l = r8; fl_r = r9; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 18; continue; } bb = 16; continue; }
            16 => { rdx = r8.wrapping_add(0x3c); rcx = rbx.wrapping_add(0x3c); bb = 17; continue; }
            17 => { rax = (crate::simd_ops::ru64(r8)) as u64; crate::simd_ops::wu64(r8, r14); crate::simd_ops::wu64(rcx.wrapping_sub(0x3cu64), rax); rax = ((crate::simd_ops::ru16(rdx.wrapping_sub(0x34u64))) as u32) as u32 as u64; crate::simd_ops::wu16(rcx.wrapping_sub(0x34u64), ((rax as u16)) as u16); rax = ((crate::simd_ops::ru16(rdx.wrapping_sub(0x32u64))) as u32) as u32 as u64; crate::simd_ops::wu16(rcx.wrapping_sub(0x32u64), ((rax as u16)) as u16); rax = (crate::simd_ops::ru64(rdx.wrapping_sub(0x30u64))) as u64; crate::simd_ops::wu64(rcx.wrapping_sub(0x30u64), rax); rax = (crate::simd_ops::ru64(rdx.wrapping_sub(0x24u64))) as u64; crate::simd_ops::wu64(rcx.wrapping_sub(0x24u64), rax); xmm0 = ((crate::simd_ops::ru64(rdx.wrapping_sub(0x1cu64))) as u64) as u128; crate::simd_ops::wu64(rcx.wrapping_sub(0x1cu64), ((xmm0 as u64)) as u64); rax = (crate::simd_ops::ru32(rdx.wrapping_sub(0x14u64))) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_sub(0x14u64), ((rax as u32)) as u32); crate::simd_ops::wu64(rcx.wrapping_sub(0xcu64), r14); crate::simd_ops::wu64(rcx.wrapping_sub(0x4u64), r14); rax = (crate::simd_ops::ru64(rdx.wrapping_sub(0xcu64))) as u64; crate::simd_ops::wu64(rcx.wrapping_sub(0xcu64), rax); rax = (crate::simd_ops::ru32(rdx.wrapping_sub(0x4u64))) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_sub(0x4u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdx)) as u32 as u64; crate::simd_ops::wu32(rcx, ((rax as u32)) as u32); crate::simd_ops::wu64(rdx.wrapping_sub(0xcu64), r14); crate::simd_ops::wu64(rdx.wrapping_sub(0x4u64), r14); rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x4u64))) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(0x4u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x8u64))) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(0x8u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdx.wrapping_add(0xcu64))) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(0xcu64), ((rax as u32)) as u32); rcx = rcx.wrapping_add(0x50); r8 = ((r8).wrapping_add(0x50u64)) as u64; rdx = rdx.wrapping_add(0x50); fl_l = r8; fl_r = r9; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 17; continue; } bb = 18; continue; }
            18 => { rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rdi, rbx); crate::simd_ops::wu32(rdi.wrapping_add(0x8u64), ((rbp as u32)) as u32); crate::simd_ops::wu32(rdi.wrapping_add(0xcu64), ((rsi as u32)) as u32); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1412ec820(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1408746b0; const P1: u64 = 0x14183acc2;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = r8; rdi = rdx; rsi = rcx; rbp = (crate::simd_ops::ru32(rcx.wrapping_add(0x8u64))) as u32 as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(0xcu64))) as u32 as u64; fl_l = rdx; fl_r = rax; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 7; continue; } bb = 1; continue; }
            1 => { rdx = ((rax as u32)) as u32 as u64; rdx = ((rdx).wrapping_shr(1u32)) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; fl_l = rdx; fl_r = rdi; rdx = (if (((fl_l as u64)) < ((fl_r as u64))) { rdi } else { rdx }) as u64; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rdx = rdi; fl_l = rdx; fl_r = rbp; rdx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 12; continue; } bb = 2; continue; }
            2 => { rcx = (crate::simd_ops::ru32(rsi.wrapping_add(0x8u64))) as u32 as u64; rax = (crate::simd_ops::ru64(rsi)) as u64; r9 = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = (crate::simd_ops::ru64(rbx)) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 5; continue; } bb = 3; continue; }
            3 => { r8 = ((rdx.wrapping_mul(8)) as u64) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = r9; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu32(rsi.wrapping_add(0x8u64), ((rdi as u32)) as u32); bb = 15; continue; }
            4 => { bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru64(rbx)) as u64; crate::simd_ops::wu64(r9, rax); r9 = r9.wrapping_add(0x8); fl_l = rdx; fl_r = (0x1u64) as u64; rdx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 5; continue; } bb = 6; continue; }
            6 => { crate::simd_ops::wu32(rsi.wrapping_add(0x8u64), ((rdi as u32)) as u32); bb = 15; continue; }
            7 => { fl_l = rdi; fl_r = rbp; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 13; continue; } bb = 8; continue; }
            8 => { fl_l = rdx; fl_r = rbp; rdx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 12; continue; } bb = 9; continue; }
            9 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rcx = ((rax.wrapping_add(rbp.wrapping_mul(8))) as u64) as u64; fl_l = (crate::simd_ops::ru64(r8)) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 11; continue; } bb = 10; continue; }
            10 => { r8 = ((rdx.wrapping_mul(8)) as u64) as u64; rdx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu32(rsi.wrapping_add(0x8u64), ((rdi as u32)) as u32); bb = 15; continue; }
            11 => { rax = (crate::simd_ops::ru64(r8)) as u64; crate::simd_ops::wu64(rcx, rax); rcx = rcx.wrapping_add(0x8); fl_l = rdx; fl_r = (0x1u64) as u64; rdx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 11; continue; } bb = 12; continue; }
            12 => { crate::simd_ops::wu32(rsi.wrapping_add(0x8u64), ((rdi as u32)) as u32); bb = 15; continue; }
            13 => { if (((fl_l as u64)) == ((fl_r as u64))) { bb = 15; continue; } bb = 14; continue; }
            14 => { crate::simd_ops::wu32(rcx.wrapping_add(0x8u64), ((rdi as u32)) as u32); bb = 15; continue; }
            15 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1412ec980(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xc; const P1: u64 = 0x8; const P2: u64 = 0x10; const P3: u64 = 0x140215e80; const P4: u64 = 0x14183acc8; const P5: u64 = 0x1403b8050;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; rbx = rcx; rbp = (crate::simd_ops::ru32(rcx.wrapping_add(P1))) as u32 as u64; rdi = ((rax as u32)) as u32 as u64; rdi = ((rdi).wrapping_shr(1u32)) as u64; rdi = ((rdi).wrapping_add(rax)) as u64; fl_l = rdi; fl_r = rdx; rdi = (if (((fl_l as u64)) < ((fl_r as u64))) { rdx } else { rdi }) as u64; fl_l = rdi; fl_r = (0x4u64) as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 3; continue; } bb = 1; continue; }
            1 => { rsi = ((rcx.wrapping_add(P2)) as u64) as u64; fl_l = (crate::simd_ops::ru64(rcx)) as u64; fl_r = rsi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 3; continue; } bb = 2; continue; }
            2 => { rdi = 0x4; bb = 4; continue; }
            3 => { rdx = rdi; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rsi = rax; bb = 4; continue; }
            4 => { r8 = (crate::simd_ops::ru32(rbx.wrapping_add(P1))) as u32 as u64; rcx = rsi; rdx = (crate::simd_ops::ru64(rbx)) as u64; r8 = ((r8).wrapping_shl(4u32)) as u64; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rcx = rbx; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx, rsi); rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; crate::simd_ops::wu32(rbx.wrapping_add(P1), ((rbp as u32)) as u32); rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; crate::simd_ops::wu32(rbx.wrapping_add(P0), ((rdi as u32)) as u32); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1412ecb90(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1411253b0;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(0xcu64))) as u32 as u64; rbx = r8; r10 = (crate::simd_ops::ru32(rcx.wrapping_add(0x8u64))) as u32 as u64; rbx = ((rbx).wrapping_sub(rdx)) as u64; rbx = ((((rbx) as i64).wrapping_shr(6u32) as u64)) as u64; rbp = r8; rsi = rdx; rdi = rcx; fl_l = rbx; fl_r = rax; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 5; continue; } bb = 1; continue; }
            1 => { rdx = ((rax as u32)) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(0x8u64), (0x0u32) as u32); rdx = ((rdx).wrapping_shr(1u32)) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; fl_l = rdx; fl_r = rbx; rdx = (if (((fl_l as u64)) < ((fl_r as u64))) { rbx } else { rdx }) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r8 = (crate::simd_ops::ru64(rdi)) as u64; fl_l = rsi; fl_r = rbp; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 17; continue; } bb = 2; continue; }
            2 => { rdx = r8.wrapping_add(0x28); rcx = rsi.wrapping_add(0x28); bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru32(rcx.wrapping_sub(0x28u64))) as u32 as u64; rcx = rcx.wrapping_add(0x40); crate::simd_ops::wu32(r8, ((rax as u32)) as u32); r8 = r8.wrapping_add(0x40); xmm0 = ((crate::simd_ops::ru64(rcx.wrapping_sub(0x58u64))) as u64) as u128; crate::simd_ops::wu64(rdx.wrapping_sub(0x18u64), ((xmm0 as u64)) as u64); rax = (crate::simd_ops::ru32(rcx.wrapping_sub(0x50u64))) as u32 as u64; crate::simd_ops::wu32(rdx.wrapping_sub(0x10u64), ((rax as u32)) as u32); rdx = rdx.wrapping_add(0x40); xmm0 = ((crate::simd_ops::ru64(rcx.wrapping_sub(0x4cu64))) as u64) as u128; crate::simd_ops::wu64(rdx.wrapping_sub(0x4cu64), ((xmm0 as u64)) as u64); rax = (crate::simd_ops::ru32(rcx.wrapping_sub(0x44u64))) as u32 as u64; crate::simd_ops::wu32(rdx.wrapping_sub(0x44u64), ((rax as u32)) as u32); xmm0 = ((crate::simd_ops::ru64(rcx.wrapping_sub(0x40u64))) as u64) as u128; crate::simd_ops::wu64(rdx.wrapping_sub(0x40u64), ((xmm0 as u64)) as u64); rax = (crate::simd_ops::ru32(rcx.wrapping_sub(0x38u64))) as u32 as u64; crate::simd_ops::wu32(rdx.wrapping_sub(0x38u64), ((rax as u32)) as u32); xmm0 = ((crate::simd_ops::ru64(rcx.wrapping_sub(0x34u64))) as u64) as u128; crate::simd_ops::wu64(rdx.wrapping_sub(0x34u64), ((xmm0 as u64)) as u64); rax = (crate::simd_ops::ru32(rcx.wrapping_sub(0x2cu64))) as u32 as u64; crate::simd_ops::wu32(rdx.wrapping_sub(0x2cu64), ((rax as u32)) as u32); rax = rcx.wrapping_sub(0x28); fl_l = rax; fl_r = rbp; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } bb = 4; continue; }
            4 => { bb = 17; continue; }
            5 => { rcx = (crate::simd_ops::ru64(rcx)) as u64; fl_l = rbx; fl_r = r10; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 14; continue; } bb = 6; continue; }
            6 => { r9 = r10; r9 = ((r9).wrapping_shl(6u32)) as u64; r9 = ((r9).wrapping_add(rsi)) as u64; fl_l = r10; fl_r = r10; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 7; continue; }
            7 => { rax = (crate::simd_ops::ru32(rdx)) as u32 as u64; r11 = r10.wrapping_sub(0x1); crate::simd_ops::wu32(rcx, ((rax as u32)) as u32); xmm0 = ((crate::simd_ops::ru64(rdx.wrapping_add(0x10u64))) as u64) as u128; crate::simd_ops::wu64(rcx.wrapping_add(0x10u64), ((xmm0 as u64)) as u64); rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x18u64))) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(0x18u64), ((rax as u32)) as u32); xmm0 = ((crate::simd_ops::ru64(rdx.wrapping_add(0x1cu64))) as u64) as u128; crate::simd_ops::wu64(rcx.wrapping_add(0x1cu64), ((xmm0 as u64)) as u64); rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x24u64))) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(0x24u64), ((rax as u32)) as u32); xmm0 = ((crate::simd_ops::ru64(rdx.wrapping_add(0x28u64))) as u64) as u128; crate::simd_ops::wu64(rcx.wrapping_add(0x28u64), ((xmm0 as u64)) as u64); rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x30u64))) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(0x30u64), ((rax as u32)) as u32); xmm0 = ((crate::simd_ops::ru64(rdx.wrapping_add(0x34u64))) as u64) as u128; crate::simd_ops::wu64(rcx.wrapping_add(0x34u64), ((xmm0 as u64)) as u64); rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x3cu64))) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(0x3cu64), ((rax as u32)) as u32); rcx = ((rcx).wrapping_add(0x40u64)) as u64; fl_l = r11; fl_r = r11; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 8; continue; }
            8 => { r8 = rcx.wrapping_add(0x28); fl_l = ((rdx).wrapping_add(0x28u64)) as u64; fl_r = fl_l; rdx = ((fl_l as u64)) as u64; bb = 9; continue; }
            9 => { rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x18u64))) as u32 as u64; rdx = rdx.wrapping_add(0x40); crate::simd_ops::wu32(rcx, ((rax as u32)) as u32); r8 = r8.wrapping_add(0x40); xmm0 = ((crate::simd_ops::ru64(rdx.wrapping_sub(0x18u64))) as u64) as u128; crate::simd_ops::wu64(r8.wrapping_sub(0x58u64), ((xmm0 as u64)) as u64); rax = (crate::simd_ops::ru32(rdx.wrapping_sub(0x10u64))) as u32 as u64; rcx = ((rcx).wrapping_add(0x40u64)) as u64; crate::simd_ops::wu32(r8.wrapping_sub(0x50u64), ((rax as u32)) as u32); xmm0 = ((crate::simd_ops::ru64(rdx.wrapping_sub(0xcu64))) as u64) as u128; crate::simd_ops::wu64(r8.wrapping_sub(0x4cu64), ((xmm0 as u64)) as u64); rax = (crate::simd_ops::ru32(rdx.wrapping_sub(0x4u64))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_sub(0x44u64), ((rax as u32)) as u32); xmm0 = ((crate::simd_ops::ru64(rdx)) as u64) as u128; crate::simd_ops::wu64(r8.wrapping_sub(0x40u64), ((xmm0 as u64)) as u64); rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x8u64))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_sub(0x38u64), ((rax as u32)) as u32); xmm0 = ((crate::simd_ops::ru64(rdx.wrapping_add(0xcu64))) as u64) as u128; crate::simd_ops::wu64(r8.wrapping_sub(0x34u64), ((xmm0 as u64)) as u64); rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x14u64))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_sub(0x2cu64), ((rax as u32)) as u32); fl_l = r11; fl_r = (0x1u64) as u64; r11 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 9; continue; } bb = 10; continue; }
            10 => { r8 = rbx; fl_l = r8; fl_r = r10; r8 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 17; continue; } bb = 11; continue; }
            11 => { rdx = r9.wrapping_add(0x28); fl_l = rcx; fl_r = r9; rcx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; bb = 12; continue; }
            12 => { rax = (crate::simd_ops::ru32(rdx.wrapping_sub(0x28u64))) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(rdx).wrapping_sub(0x28u64), ((rax as u32)) as u32); xmm0 = ((crate::simd_ops::ru64(rdx.wrapping_sub(0x18u64))) as u64) as u128; crate::simd_ops::wu64(rcx.wrapping_add(rdx).wrapping_sub(0x18u64), ((xmm0 as u64)) as u64); rax = (crate::simd_ops::ru32(rdx.wrapping_sub(0x10u64))) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(rdx).wrapping_sub(0x10u64), ((rax as u32)) as u32); xmm0 = ((crate::simd_ops::ru64(rdx.wrapping_sub(0xcu64))) as u64) as u128; crate::simd_ops::wu64(rcx.wrapping_add(rdx).wrapping_sub(0xcu64), ((xmm0 as u64)) as u64); rax = (crate::simd_ops::ru32(rdx.wrapping_sub(0x4u64))) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(rdx).wrapping_sub(0x4u64), ((rax as u32)) as u32); xmm0 = ((crate::simd_ops::ru64(rdx)) as u64) as u128; crate::simd_ops::wu64(rcx.wrapping_add(rdx), ((xmm0 as u64)) as u64); rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x8u64))) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(rdx).wrapping_add(0x8u64), ((rax as u32)) as u32); xmm0 = ((crate::simd_ops::ru64(rdx.wrapping_add(0xcu64))) as u64) as u128; crate::simd_ops::wu64(rcx.wrapping_add(rdx).wrapping_add(0xcu64), ((xmm0 as u64)) as u64); rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x14u64))) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(rdx).wrapping_add(0x14u64), ((rax as u32)) as u32); rdx = rdx.wrapping_add(0x40); fl_l = r8; fl_r = (0x1u64) as u64; r8 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 12; continue; } bb = 13; continue; }
            13 => { bb = 17; continue; }
            14 => { fl_l = rsi; fl_r = rbp; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 17; continue; } bb = 15; continue; }
            15 => { r8 = rcx.wrapping_add(0x28); fl_l = ((rdx).wrapping_add(0x28u64)) as u64; fl_r = fl_l; rdx = ((fl_l as u64)) as u64; bb = 16; continue; }
            16 => { rax = (crate::simd_ops::ru32(rdx.wrapping_sub(0x28u64))) as u32 as u64; rdx = rdx.wrapping_add(0x40); crate::simd_ops::wu32(rcx, ((rax as u32)) as u32); rcx = rcx.wrapping_add(0x40); xmm0 = ((crate::simd_ops::ru64(rdx.wrapping_sub(0x58u64))) as u64) as u128; crate::simd_ops::wu64(r8.wrapping_sub(0x18u64), ((xmm0 as u64)) as u64); rax = (crate::simd_ops::ru32(rdx.wrapping_sub(0x50u64))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_sub(0x10u64), ((rax as u32)) as u32); r8 = r8.wrapping_add(0x40); xmm0 = ((crate::simd_ops::ru64(rdx.wrapping_sub(0x4cu64))) as u64) as u128; crate::simd_ops::wu64(r8.wrapping_sub(0x4cu64), ((xmm0 as u64)) as u64); rax = (crate::simd_ops::ru32(rdx.wrapping_sub(0x44u64))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_sub(0x44u64), ((rax as u32)) as u32); xmm0 = ((crate::simd_ops::ru64(rdx.wrapping_sub(0x40u64))) as u64) as u128; crate::simd_ops::wu64(r8.wrapping_sub(0x40u64), ((xmm0 as u64)) as u64); rax = (crate::simd_ops::ru32(rdx.wrapping_sub(0x38u64))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_sub(0x38u64), ((rax as u32)) as u32); xmm0 = ((crate::simd_ops::ru64(rdx.wrapping_sub(0x34u64))) as u64) as u128; crate::simd_ops::wu64(r8.wrapping_sub(0x34u64), ((xmm0 as u64)) as u64); rax = (crate::simd_ops::ru32(rdx.wrapping_sub(0x2cu64))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_sub(0x2cu64), ((rax as u32)) as u32); rax = rdx.wrapping_sub(0x28); fl_l = rax; fl_r = rbp; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 16; continue; } bb = 17; continue; }
            17 => { rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x8u64), ((rbx as u32)) as u32); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1412ece40(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142537280; const P1: u64 = 0x142535070; const P2: u64 = 0x1426456c8; const P3: u64 = 0x1426453a0; const P4: u64 = 0x14065f150;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rsi = rdx; rdi = rcx; rbp = (crate::simd_ops::ru32(rcx.wrapping_add(0x8u64))) as u32 as u64; rdx = ((rdx).wrapping_shl(5u32)) as u64; r10 = 0x6348; rax = ((crate::exe::tls_pointer() as u64)) as u64; r8 = (crate::simd_ops::ru64(rax)) as u64; r10 = (crate::simd_ops::ru64(r10.wrapping_add(r8))) as u64; r14 = 0; fl_l = 0; fl_r = 0; fl_l = ((rdx) & ((rdx).wrapping_sub(1))) as u64; rax = ((fl_l as u64)) as u64; if (((fl_l as u64)) != 0) { bb = 5; continue; } bb = 1; continue; }
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
            15 => { rcx = (crate::simd_ops::ru64(rdi)) as u64; r8 = (crate::simd_ops::ru32(rdi.wrapping_add(0x8u64))) as u32 as u64; r8 = ((r8).wrapping_shl(5u32)) as u64; r8 = ((r8).wrapping_add(rcx)) as u64; fl_l = rcx; fl_r = r8; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 18; continue; } bb = 16; continue; }
            16 => { rdx = rbx.wrapping_add(0xc); bb = 17; continue; }
            17 => { crate::simd_ops::wu64(rdx.wrapping_sub(0xcu64), r14); crate::simd_ops::wu64(rdx.wrapping_sub(0x4u64), r14); rax = (crate::simd_ops::ru64(rcx)) as u64; crate::simd_ops::wu64(rdx.wrapping_sub(0xcu64), rax); rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x8u64))) as u32 as u64; crate::simd_ops::wu32(rdx.wrapping_sub(0x4u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rcx.wrapping_add(0xcu64))) as u32 as u64; crate::simd_ops::wu32(rdx, ((rax as u32)) as u32); crate::simd_ops::wu64(rcx, r14); crate::simd_ops::wu64(rcx.wrapping_add(0x8u64), r14); xmm0 = ((crate::simd_ops::ru64(rcx.wrapping_add(0x10u64))) as u64) as u128; crate::simd_ops::wu64(rdx.wrapping_add(0x4u64), ((xmm0 as u64)) as u64); rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x18u64))) as u32 as u64; crate::simd_ops::wu32(rdx.wrapping_add(0xcu64), ((rax as u32)) as u32); rdx = rdx.wrapping_add(0x20); rcx = ((rcx).wrapping_add(0x20u64)) as u64; fl_l = rcx; fl_r = r8; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 17; continue; } bb = 18; continue; }
            18 => { rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rdi, rbx); crate::simd_ops::wu32(rdi.wrapping_add(0x8u64), ((rbp as u32)) as u32); crate::simd_ops::wu32(rdi.wrapping_add(0xcu64), ((rsi as u32)) as u32); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1412ecfd0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x143427178; const P2: u64 = 0x38; const P3: u64 = 0x14183acc2; const P4: u64 = 0xffffffffffffffd0; const P5: u64 = 0xffffffffffffffd8; const P6: u64 = 0xffffffffffffffe0; const P7: u64 = 0xffffffffffffffe8; const P8: u64 = 0x143159a38; const P9: u64 = 0xffffffffffffffec; const P10: u64 = 0x143159a40; const P11: u64 = 0xfffffffffffffff4;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut xmm6: u128 = 0; let mut xmm7: u128 = 0; let mut xmm8: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x50u64)) as u64; crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm6) as u128); crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm7) as u128); crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm8) as u128); rsi = r8; rax = (crate::simd_ops::ru64(rcx)) as u64; rdi = (crate::simd_ops::ru32(rax.wrapping_add(P0))) as u32 as u64; rdi = ((rdi).wrapping_shl(8u32)) as u64; rdi = ((rdi).wrapping_add(crate::simd_ops::ru64(rax))) as u64; fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { xmm7 = ((crate::simd_ops::ru32(P1)) as u32) as u128; xmm8 = 0u128; xmm6 = 0u128; rbx = ((rdi.wrapping_add(P2)) as u64) as u64; rbp = 0; fl_l = 0; fl_r = 0; bb = 2; continue; }
            2 => { rdx = 0; fl_l = 0; fl_r = 0; r8 = 0x100; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rbx.wrapping_add(P4), rbp); crate::simd_ops::wu64(rbx.wrapping_add(P5), rbp); crate::simd_ops::wu64(rbx.wrapping_add(P6), rbp); crate::simd_ops::wu32(rbx.wrapping_add(P7), ((rbp as u32)) as u32); xmm0 = ((crate::simd_ops::ru64(P8)) as u64) as u128; crate::simd_ops::wu64(rbx.wrapping_add(P9), ((xmm0 as u64)) as u64); rax = (crate::simd_ops::ru32(P10)) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(P11), ((rax as u32)) as u32); crate::simd_ops::wu32(rbx.wrapping_sub(0x8u64), ((xmm7 as u32)) as u32); crate::simd_ops::wu32(rbx.wrapping_sub(0x4u64), ((xmm7 as u32)) as u32); crate::simd_ops::wu32(rbx, ((xmm7 as u32)) as u32); crate::simd_ops::wu128(rbx.wrapping_add(0x18u64), (xmm8) as u128); crate::simd_ops::wu128(rbx.wrapping_add(0x28u64), (xmm8) as u128); crate::simd_ops::wu128(rbx.wrapping_add(0x38u64), (xmm8) as u128); crate::simd_ops::wu128(rbx.wrapping_add(0x48u64), (xmm6) as u128); crate::simd_ops::wu128(rbx.wrapping_add(0x58u64), (xmm6) as u128); crate::simd_ops::wu128(rbx.wrapping_add(0x68u64), (xmm6) as u128); crate::simd_ops::wu128(rbx.wrapping_add(0x78u64), (xmm6) as u128); crate::simd_ops::wu128(rbx.wrapping_add(0x88u64), (xmm6) as u128); crate::simd_ops::wu128(rbx.wrapping_add(0x98u64), (xmm6) as u128); rdi = ((rdi).wrapping_add(0x100u64)) as u64; rbx = rbx.wrapping_add(0x100); fl_l = rsi; fl_r = (0x1u64) as u64; rsi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 3; continue; }
            3 => { rax = rdi; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x40u64))) as u128; xmm7 = (crate::simd_ops::ru128(rsp.wrapping_add(0x30u64))) as u128; xmm8 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; rsp = ((rsp).wrapping_add(0x50u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1412ed0f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x143428240; const P1: u64 = 0x10; const P2: u64 = 0x20;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    xmm0 = (crate::simd_ops::ru128(P0)) as u128;
    xmm1 = 0u128;
    crate::simd_ops::wu128(rcx, (xmm1) as u128);
    crate::simd_ops::wu128(rcx.wrapping_add(P1), (xmm0) as u128);
    crate::simd_ops::wu128(rcx.wrapping_add(P2), (xmm1) as u128);
    rax = rcx;
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1412ed110(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x1401de830; const P2: u64 = 0xffffffffffffffff; const P3: u64 = 0x108; const P4: u64 = 0x4; const P5: u64 = 0x100; const P6: u64 = 0xe; const P7: u64 = 0x20; const P8: u64 = 0x18; const P9: u64 = 0x1425371a0; const P10: u64 = 0x142533200;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = rcx; rbx = (crate::simd_ops::ru64(rcx)) as u64; rsi = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; rsi = ((rsi).wrapping_shl(8u32)) as u64; rsi = ((rsi).wrapping_add(rbx)) as u64; fl_l = rbx; fl_r = rsi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rcx = ((rbx.wrapping_add(P0)) as u64) as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rbx = ((rbx).wrapping_add(0x100u64)) as u64; fl_l = rbx; fl_r = rsi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } bb = 2; continue; }
            2 => { r9 = (crate::simd_ops::ru64(rdi)) as u64; fl_l = r9; fl_r = r9; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 3; continue; }
            3 => { rcx = ((r9.wrapping_add(P2)) as u64) as u64; rcx = (((rcx) & (0xfffffffffe000000u64))) as u64; rax = r9; rax = ((rax).wrapping_sub(rcx)) as u64; rax = ((rax).wrapping_shr(16u32)) as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rdx = ((rcx.wrapping_add(P3)) as u64) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(P4))) as u32 as u64; rdx = ((rdx).wrapping_sub(rax)) as u64; rax = ((crate::exe::teb_self() as u64)) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(rcx.wrapping_add(P5))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 8; continue; } bb = 4; continue; }
            4 => { fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(P6))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 7; continue; } bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru64(rdx.wrapping_add(P7))) as u64; crate::simd_ops::wu64(r9, rax); crate::simd_ops::wu64(rdx.wrapping_add(P7), r9); fl_l = (crate::simd_ops::lk_add_u32(rdx.wrapping_add(P8), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; if (((fl_l as u32)) != 0) { bb = 10; continue; } bb = 6; continue; }
            6 => { rcx = rdx; rax = crate::simd_ops::call_stk(P9, rax, rcx, rdx, r8, r9, rsp); bb = 10; continue; }
            7 => { r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 9; continue; }
            8 => { r8 = 0; fl_l = 0; fl_r = 0; bb = 9; continue; }
            9 => { rax = crate::simd_ops::call_stk(P10, rax, rcx, rdx, r8, r9, rsp); bb = 10; continue; }
            10 => { rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rdi, rax); crate::simd_ops::wu64(rdi.wrapping_add(P0), rax); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1412ed1e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142537280; const P1: u64 = 0x142535070; const P2: u64 = 0x1426456c8; const P3: u64 = 0x1426453a0; const P4: u64 = 0x1412ed110;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rsi = rdx; rdi = rcx; rbp = (crate::simd_ops::ru32(rcx.wrapping_add(0x8u64))) as u32 as u64; rdx = ((rdx).wrapping_shl(8u32)) as u64; r10 = 0x6348; rax = ((crate::exe::tls_pointer() as u64)) as u64; r8 = (crate::simd_ops::ru64(rax)) as u64; r10 = (crate::simd_ops::ru64(r10.wrapping_add(r8))) as u64; r14 = 0; fl_l = 0; fl_r = 0; fl_l = ((rdx) & ((rdx).wrapping_sub(1))) as u64; rax = ((fl_l as u64)) as u64; if (((fl_l as u64)) != 0) { bb = 5; continue; } bb = 1; continue; }
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
            15 => { r9 = (crate::simd_ops::ru64(rdi)) as u64; r8 = (crate::simd_ops::ru32(rdi.wrapping_add(0x8u64))) as u32 as u64; r8 = ((r8).wrapping_shl(8u32)) as u64; r8 = ((r8).wrapping_add(r9)) as u64; fl_l = r9; fl_r = r8; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 18; continue; } bb = 16; continue; }
            16 => { rdx = r9.wrapping_add(0x14); rcx = rbx.wrapping_add(0x14); bb = 17; continue; }
            17 => { rax = (crate::simd_ops::ru64(r9)) as u64; crate::simd_ops::wu64(rcx.wrapping_sub(0x14u64), rax); crate::simd_ops::wu64(rcx.wrapping_sub(0xcu64), r14); crate::simd_ops::wu64(rcx.wrapping_sub(0x4u64), r14); rax = (crate::simd_ops::ru64(rdx.wrapping_sub(0xcu64))) as u64; crate::simd_ops::wu64(rcx.wrapping_sub(0xcu64), rax); rax = (crate::simd_ops::ru32(rdx.wrapping_sub(0x4u64))) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_sub(0x4u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdx)) as u32 as u64; crate::simd_ops::wu32(rcx, ((rax as u32)) as u32); crate::simd_ops::wu64(rdx.wrapping_sub(0xcu64), r14); crate::simd_ops::wu64(rdx.wrapping_sub(0x4u64), r14); xmm0 = ((crate::simd_ops::ru64(rdx.wrapping_add(0x4u64))) as u64) as u128; ymm0_hi = 0; crate::simd_ops::wu64(rcx.wrapping_add(0x4u64), ((xmm0 as u64)) as u64); rax = (crate::simd_ops::ru32(rdx.wrapping_add(0xcu64))) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(0xcu64), ((rax as u32)) as u32); xmm0 = (crate::simd_ops::ru128(rdx.wrapping_add(0x10u64))) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rcx.wrapping_add(0x10u64), (xmm0) as u128); xmm1 = ((crate::simd_ops::ru64(rdx.wrapping_add(0x20u64))) as u64) as u128; crate::simd_ops::wu64(rcx.wrapping_add(0x20u64), ((xmm1 as u64)) as u64); rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x28u64))) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(0x28u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x2cu64))) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(0x2cu64), ((rax as u32)) as u32); let y_addr: u64 = rdx.wrapping_add(0x3cu64); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rcx.wrapping_add(0x3cu64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); xmm1 = (crate::simd_ops::ru128(rdx.wrapping_add(0x5cu64))) as u128; crate::simd_ops::wu128(rcx.wrapping_add(0x5cu64), (xmm1) as u128); let y_addr: u64 = rdx.wrapping_add(0x6cu64); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rcx.wrapping_add(0x6cu64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); xmm1 = (crate::simd_ops::ru128(rdx.wrapping_add(0x8cu64))) as u128; crate::simd_ops::wu128(rcx.wrapping_add(0x8cu64), (xmm1) as u128); let y_addr: u64 = rdx.wrapping_add(0x9cu64); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rcx.wrapping_add(0x9cu64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); xmm1 = (crate::simd_ops::ru128(rdx.wrapping_add(0xbcu64))) as u128; crate::simd_ops::wu128(rcx.wrapping_add(0xbcu64), (xmm1) as u128); xmm0 = (crate::simd_ops::ru128(rdx.wrapping_add(0xccu64))) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rcx.wrapping_add(0xccu64), (xmm0) as u128); xmm0 = (crate::simd_ops::ru128(rdx.wrapping_add(0xdcu64))) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rcx.wrapping_add(0xdcu64), (xmm0) as u128); rcx = rcx.wrapping_add(0x100); r9 = ((r9).wrapping_add(0x100u64)) as u64; rdx = rdx.wrapping_add(0x100); fl_l = r9; fl_r = r8; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 17; continue; } bb = 18; continue; }
            18 => { rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rdi, rbx); crate::simd_ops::wu32(rdi.wrapping_add(0x8u64), ((rbp as u32)) as u32); crate::simd_ops::wu32(rdi.wrapping_add(0xcu64), ((rsi as u32)) as u32); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1412ed420(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x20;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut ymm0_hi: u128 = 0; let mut ymm2_hi: u128 = 0; let mut xmm4: u128 = 0; let mut xmm5: u128 = 0;
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    xmm0 = (crate::simd_ops::ru128(rdx.wrapping_add(P0))) as u128; ymm0_hi = 0;
    xmm2 = (crate::simd_ops::ru128(rdx)) as u128; ymm2_hi = 0;
    xmm3 = (crate::simd_ops::ru128(rdx.wrapping_add(P1))) as u128;
    xmm1 = crate::simd_ops::vmulps(xmm0, xmm0);
    xmm0 = crate::simd_ops::vmulps(xmm2, xmm2); ymm0_hi = 0;
    xmm2 = crate::simd_ops::vaddps(xmm1, xmm0); ymm2_hi = 0;
    xmm1 = crate::simd_ops::vmulps(xmm3, xmm3);
    xmm2 = crate::simd_ops::vaddps(xmm2, xmm1); ymm2_hi = 0;
    xmm4 = crate::simd_ops::vsqrtps(xmm2);
    xmm2 = crate::simd_ops::vrcpps(xmm4); ymm2_hi = 0;
    xmm1 = crate::simd_ops::vmulps(xmm2, xmm2);
    xmm2 = crate::simd_ops::vaddps(xmm2, xmm2); ymm2_hi = 0;
    xmm5 = 0u128;
    xmm0 = crate::simd_ops::vcmpps(xmm4, xmm5, 0x0u8); ymm0_hi = 0;
    xmm3 = crate::simd_ops::vmulps(xmm1, xmm4);
    xmm4 = crate::simd_ops::vsubps(xmm2, xmm3);
    xmm2 = crate::simd_ops::vblendvps(xmm4, xmm5, xmm0); ymm2_hi = 0;
    let y_addr: u64 = rdx; xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16));
    let y_addr: u64 = rcx; crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi);
    xmm1 = (crate::simd_ops::ru128(rdx.wrapping_add(P1))) as u128;
    crate::simd_ops::wu128(rcx.wrapping_add(P1), (xmm1) as u128);
    let ins_v: u128 = xmm2; xmm2 = xmm2; ymm2_hi = ins_v;
    let (y_lo, y_hi): (u128, u128) = (crate::simd_ops::vmulps(xmm2, crate::simd_ops::ru128(rcx)), crate::simd_ops::vmulps(ymm2_hi, crate::simd_ops::ru128((rcx).wrapping_add(16)))); xmm0 = y_lo; ymm0_hi = y_hi;
    let y_addr: u64 = rcx; crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi);
    xmm0 = crate::simd_ops::vmulps(xmm2, crate::simd_ops::ru128(rcx.wrapping_add(P1))); ymm0_hi = 0;
    crate::simd_ops::wu128(rcx.wrapping_add(P1), (xmm0) as u128);
    rax = rcx;
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1412ed6e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142407c80; const P1: u64 = 0x143c4aa00; const P2: u64 = 0x4c; const P3: u64 = 0x38; const P4: u64 = 0x30; const P5: u64 = 0x142237090;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut xmm6: u128 = 0; let mut xmm7: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x50u64)) as u64; crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm6) as u128); rbp = r8; crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm7) as u128); rsi = rdx; rdi = rcx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = ((rax as u32)) as u32 as u64; r14 = P1; rbx = 0; fl_l = 0; fl_r = 0; rax = (crate::simd_ops::ru64(r14.wrapping_add(rax.wrapping_mul(8)))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru32(rax)) as u32 as u64; bb = 3; continue; }
            2 => { rax = ((rbx as u32)) as u32 as u64; bb = 3; continue; }
            3 => { rcx = (((rax) as i64).wrapping_mul((0xd8u64) as i64) as u64) as u64; xmm7 = ((crate::simd_ops::ru32(rcx.wrapping_add(rdi).wrapping_add(P2))) as u32) as u128; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = ((rax as u32)) as u32 as u64; rax = (crate::simd_ops::ru64(r14.wrapping_add(rax.wrapping_mul(8)))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru32(rax)) as u32 as u64; bb = 6; continue; }
            5 => { rax = ((rbx as u32)) as u32 as u64; bb = 6; continue; }
            6 => { rcx = (((rax) as i64).wrapping_mul((0xd8u64) as i64) as u64) as u64; xmm6 = ((crate::simd_ops::ru32(rcx.wrapping_add(rdi).wrapping_add(P3))) as u32) as u128; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = ((rax as u32)) as u32 as u64; rcx = (crate::simd_ops::ru64(r14.wrapping_add(rax.wrapping_mul(8)))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 7; continue; }
            7 => { rbx = (crate::simd_ops::ru32(rcx)) as u32 as u64; bb = 8; continue; }
            8 => { rax = ((rbx as u32)) as u32 as u64; rcx = rsi; rdx = (((rax) as i64).wrapping_mul((0xd8u64) as i64) as u64) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), (0x1u32) as u32); xmm3 = (xmm7) as u128; xmm2 = (xmm6) as u128; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbp); rdx = (crate::simd_ops::ru64(rdx.wrapping_add(rdi).wrapping_add(P4))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rax = rsi; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x40u64))) as u128; xmm7 = (crate::simd_ops::ru128(rsp.wrapping_add(0x30u64))) as u128; rsp = ((rsp).wrapping_add(0x50u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1412ed7d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1412ed800; const P1: u64 = 0x141834ee8;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    r9 = P0;
    rdx = 0xd8;
    r8 = 0x13;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1412ed800(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1401de830; const P1: u64 = 0x88; const P2: u64 = 0x78; const P3: u64 = 0x68;
    let (mut rax, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rcx;
    rcx = ((rcx).wrapping_add(0x98u64)) as u64;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rcx = ((rbx.wrapping_add(P1)) as u64) as u64;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rcx = ((rbx.wrapping_add(P2)) as u64) as u64;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rcx = ((rbx.wrapping_add(P3)) as u64) as u64;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1412ed840(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x100; const P1: u64 = 0x10; const P2: u64 = 0x18; const P3: u64 = 0x142535070; const P4: u64 = 0x1426456c8; const P5: u64 = 0x1426453a0; const P6: u64 = 0x14183acc2; const P7: u64 = 0x8;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rax = ((crate::exe::tls_pointer() as u64)) as u64; rdx = 0x6348; rcx = (crate::simd_ops::ru64(rax)) as u64; rax = (crate::simd_ops::ru64(rdx.wrapping_add(rcx))) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64; rbx = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { fl_l = ((rbx as u8)) as u8 as u64; fl_r = (0xfu8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::lk_add_u32(rcx.wrapping_add(P2), (1) as u32)) as u32 as u64; rax = (crate::simd_ops::ru64(rbx)) as u64; crate::simd_ops::wu64(rcx.wrapping_add(P1), rax); bb = 5; continue; }
            3 => { rdx = 0xf8; r8 = 0x10; rcx = rax; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rbx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp); rcx = rax; rdx = 0xdeadd00d; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp); bb = 5; continue; }
            5 => { rcx = ((rbx.wrapping_add(P1)) as u64) as u64; rdx = 0; fl_l = 0; fl_r = 0; r8 = 0xe8; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); rax = rbx; crate::simd_ops::wu64(rbx, (0x0u64) as u64); crate::simd_ops::wu64(rbx.wrapping_add(P7), (0x0u64) as u64); rsp = ((rsp).wrapping_add(0x30u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1412ed9a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x4; const P2: u64 = 0x142535070; const P3: u64 = 0x1426456c8; const P4: u64 = 0x1426453a0; const P5: u64 = 0x14204dbf0;
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
    rdx = rsi;
    rcx = rbx;
    rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp);
    crate::simd_ops::wu64(rdi, rax);
    rax = rdi;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64;
    rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64;
    rsp = ((rsp).wrapping_add(0x40u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1412edae0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xc; const P1: u64 = 0x8; const P2: u64 = 0x140874640; const P3: u64 = 0x4;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = rdx; rbx = rcx; rax = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; r9 = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; r9 = ((r9).wrapping_shl(2u32)) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(P1))) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rcx = ((rcx).wrapping_shl(2u32)) as u64; fl_l = rcx; fl_r = r9; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { r10 = (0x2aaaaaaaaaaaaaabu64) as u64; rax = r10; let prod = (rax as i64 as i128).wrapping_mul((rcx) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; rdx = ((((rdx) as i64).wrapping_shr(1u32) as u64)) as u64; r8 = rdx; r8 = ((r8).wrapping_shr(63u32)) as u64; rdx = ((rdx).wrapping_add(1)) as u64; r8 = ((r8).wrapping_add(rdx)) as u64; rax = r10; let prod = (rax as i64 as i128).wrapping_mul((r9) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; rcx = rdx; rcx = ((((rcx) as i64).wrapping_shr(1u32) as u64)) as u64; rax = rcx; rax = ((rax).wrapping_shr(63u32)) as u64; rcx = ((rcx).wrapping_add(rax)) as u64; rdx = rcx; rdx = ((rdx).wrapping_shr(1u32)) as u64; rdx = ((rdx).wrapping_add(rcx)) as u64; fl_l = rdx; fl_r = r8; rdx = (if (((fl_l as u64)) < ((fl_r as u64))) { r8 } else { rdx }) as u64; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(P1))) as u32 as u64; rdx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rcx = (crate::simd_ops::ru64(rbx)) as u64; xmm0 = ((crate::simd_ops::ru32(rdi.wrapping_add(P3))) as u32) as u128; rax = (crate::simd_ops::ru32(rdi)) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(rdx.wrapping_mul(4)), ((rax as u32)) as u32); crate::simd_ops::wu32(rcx.wrapping_add(rdx.wrapping_mul(4)).wrapping_add(P3), ((xmm0 as u32)) as u32); crate::simd_ops::wu32(rcx.wrapping_add(rdx.wrapping_mul(4)).wrapping_add(P1), (0x0u32) as u32); crate::simd_ops::lk_add_u32(rbx.wrapping_add(P1), (1) as u32); rax = (crate::simd_ops::ru32(rbx.wrapping_add(P1))) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = (crate::simd_ops::ru64(rbx)) as u64; rax = ((rax.wrapping_add(rcx.wrapping_mul(4))) as u64) as u64; rax = ((rax).wrapping_add(0xfffffffffffffff4u64)) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1412edba0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xc; const P1: u64 = 0x8; const P2: u64 = 0x1412ee6f0; const P3: u64 = 0x20; const P4: u64 = 0x40; const P5: u64 = 0x48;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rcx; rax = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; r9 = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; r9 = ((r9).wrapping_shl(4u32)) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(P1))) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; fl_l = rcx; fl_r = r9; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { r10 = (0x6666666666666667u64) as u64; rax = r10; let prod = (rax as i64 as i128).wrapping_mul((rcx) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; rdx = ((((rdx) as i64).wrapping_shr(5u32) as u64)) as u64; r8 = rdx; r8 = ((r8).wrapping_shr(63u32)) as u64; rdx = ((rdx).wrapping_add(1)) as u64; r8 = ((r8).wrapping_add(rdx)) as u64; rax = r10; let prod = (rax as i64 as i128).wrapping_mul((r9) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; rcx = rdx; rcx = ((((rcx) as i64).wrapping_shr(5u32) as u64)) as u64; rax = rcx; rax = ((rax).wrapping_shr(63u32)) as u64; rcx = ((rcx).wrapping_add(rax)) as u64; rdx = rcx; rdx = ((rdx).wrapping_shr(1u32)) as u64; rdx = ((rdx).wrapping_add(rcx)) as u64; fl_l = rdx; fl_r = r8; rdx = (if (((fl_l as u64)) < ((fl_r as u64))) { r8 } else { rdx }) as u64; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(P1))) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rbx))) as u64; xmm0 = 0u128; ymm0_hi = 0; let y_addr: u64 = rcx; crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); let y_addr: u64 = rcx.wrapping_add(P3); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); crate::simd_ops::wu128(rcx.wrapping_add(P4), (xmm0) as u128); rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rcx.wrapping_add(P4), rax); crate::simd_ops::wu64(rcx.wrapping_add(P5), rax); crate::simd_ops::lk_add_u32(rbx.wrapping_add(P1), (1) as u32); rax = (crate::simd_ops::ru32(rbx.wrapping_add(P1))) as u32 as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rax = ((rax).wrapping_add(0xffffffffffffffb0u64)) as u64; rax = ((rax).wrapping_add(crate::simd_ops::ru64(rbx))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1412ee380(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1402111b0; const P1: u64 = 0x14183acc8; const P2: u64 = 0x1401de830;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x28u64)) as u64; r15 = r9; r12 = (0x2aaaaaaaaaaaaaabu64) as u64; r15 = ((r15).wrapping_sub(r8)) as u64; rax = r12; rbx = rdx; r13 = r8; let prod = (rax as i64 as i128).wrapping_mul((r15) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; r14 = rcx; rsi = rdx; rsi = ((((rsi) as i64).wrapping_shr(1u32) as u64)) as u64; rax = rsi; rax = ((rax).wrapping_shr(63u32)) as u64; fl_l = ((rsi).wrapping_add(rax)) as u64; fl_r = fl_l; rsi = ((fl_l as u64)) as u64; if (((fl_l as u64)) == 0) { bb = 14; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rdi = rcx.wrapping_add(0x8); rcx = (crate::simd_ops::ru64(rcx)) as u64; fl_l = rsi; fl_r = (0x1u64) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru32(rdi)) as u32 as u64; crate::simd_ops::wu8(rsp.wrapping_add(0x80u64), ((rsi as u8)) as u8); rdx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = ((rcx.wrapping_add(rdx.wrapping_mul(4))) as u64) as u64; fl_l = rbx; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 4; continue; } bb = 3; continue; }
            3 => { crate::simd_ops::wu8(rsp.wrapping_add(0x80u64), (0x0u8) as u8); bb = 4; continue; }
            4 => { r9 = (crate::simd_ops::ru32(rdi)) as u32 as u64; r11 = (crate::simd_ops::ru32(r14.wrapping_add(0xcu64))) as u32 as u64; r8 = ((r11 as u32)) as u32 as u64; r8 = ((r8).wrapping_sub(r9)) as u64; rax = ((r9.wrapping_add(r9.wrapping_mul(2))) as u64) as u64; r10 = ((rax.wrapping_mul(4)) as u64) as u64; rax = r12; let prod = (rax as i64 as i128).wrapping_mul((r10) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; rbp = rdx; rbp = ((((rbp) as i64).wrapping_shr(1u32) as u64)) as u64; rax = rbp; rax = ((rax).wrapping_shr(63u32)) as u64; rbp = ((rbp).wrapping_add(rax)) as u64; rbp = ((rbp).wrapping_add(rsi)) as u64; fl_l = rsi; fl_r = r8; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 9; continue; } bb = 5; continue; }
            5 => { rdx = rbx; rax = r12; rdx = ((rdx).wrapping_sub(rcx)) as u64; let prod = (rax as i64 as i128).wrapping_mul((rdx) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; rcx = rdx; rcx = ((((rcx) as i64).wrapping_shr(1u32) as u64)) as u64; rax = rcx; rax = ((rax).wrapping_shr(63u32)) as u64; rcx = ((rcx).wrapping_add(rax)) as u64; rax = ((r11 as u32)) as u32 as u64; rax = ((rax).wrapping_shr(1u32)) as u64; rax = ((rax).wrapping_add(r11)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), rcx); fl_l = rax; fl_r = rbp; rax = (if (((fl_l as u64)) < ((fl_r as u64))) { rbp } else { rax }) as u64; rdx = rax; crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), rax); rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); r12 = rax; r8 = r15; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rdx = r13; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rcx = ((r12.wrapping_add(rcx.wrapping_mul(4))) as u64) as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru8(rsp.wrapping_add(0x80u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; rdx = (crate::simd_ops::ru64(r14)) as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 7; continue; } bb = 6; continue; }
            6 => { rcx = (crate::simd_ops::ru32(rdi)) as u32 as u64; r8 = ((rcx.wrapping_add(rcx.wrapping_mul(2))) as u64) as u64; rcx = r12; fl_l = ((r8).wrapping_shl(2u32)) as u64; r8 = ((fl_l as u64)) as u64; bb = 8; continue; }
            7 => { r8 = rbx; rcx = r12; r8 = ((r8).wrapping_sub(rdx)) as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru32(rdi)) as u32 as u64; rdx = rbx; r8 = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; r8 = ((r8).wrapping_shl(2u32)) as u64; r8 = ((r8).wrapping_sub(rbx)) as u64; r8 = ((r8).wrapping_add(crate::simd_ops::ru64(r14))) as u64; fl_l = ((rax).wrapping_add(rsi)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rcx = ((r12.wrapping_add(rcx.wrapping_mul(4))) as u64) as u64; bb = 8; continue; }
            8 => { rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rcx = r14; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; crate::simd_ops::wu32(r14.wrapping_add(0xcu64), ((rax as u32)) as u32); crate::simd_ops::wu64(r14, r12); bb = 13; continue; }
            9 => { r14 = ((rcx.wrapping_add(r10)) as u64) as u64; rax = ((rsi.wrapping_add(rsi.wrapping_mul(2))) as u64) as u64; r8 = r14; rcx = ((rax.wrapping_mul(4)) as u64) as u64; r8 = ((r8).wrapping_sub(rbx)) as u64; rax = r12; let prod = (rax as i64 as i128).wrapping_mul((r8) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; rdx = ((((rdx) as i64).wrapping_shr(1u32) as u64)) as u64; rax = rdx; rax = ((rax).wrapping_shr(63u32)) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; fl_l = rsi; fl_r = rdx; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 11; continue; } bb = 10; continue; }
            10 => { r8 = rcx; rdx = r14; rdx = ((rdx).wrapping_sub(rcx)) as u64; rcx = r14; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); r8 = (((rsi) as i64).wrapping_mul((0xfffffffffffffff4u64) as i64) as u64) as u64; r8 = ((r8).wrapping_sub(rbx)) as u64; r8 = ((r8).wrapping_add(r14)) as u64; fl_l = r14; fl_r = r8; r14 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; rcx = r14; bb = 12; continue; }
            11 => { fl_l = ((rcx).wrapping_add(rbx)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; bb = 12; continue; }
            12 => { rdx = rbx; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); r8 = r15; rdx = r13; rcx = rbx; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 13; continue; }
            13 => { crate::simd_ops::wu32(rdi, ((rbp as u32)) as u32); rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; bb = 14; continue; }
            14 => { rsp = ((rsp).wrapping_add(0x28u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1412ee580(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142407c80; const P1: u64 = 0x143c4aa00; const P2: u64 = 0x1422391f0; const P3: u64 = 0x1426455a8; const P4: u64 = 0x142645598; const P5: u64 = 0x142237040; const P6: u64 = 0x142645580;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; r14 = rdx; rbp = rcx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = ((rax as u32)) as u32 as u64; r8 = P1; r8 = (crate::simd_ops::ru64(r8.wrapping_add(rax.wrapping_mul(8)))) as u64; fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru32(r8)) as u32 as u64; fl_r = 0x2; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 3; continue; } bb = 2; continue; }
            2 => { rcx = r14; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            3 => { crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rdi); rdi = (crate::simd_ops::ru64(rbp.wrapping_add(0x168u64))) as u64; rcx = rdi.wrapping_add(0x18); (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0x20u64))) as u64; rsi = (crate::simd_ops::ru64(rdi.wrapping_add(0x28u64))) as u64; r8 = rsi.wrapping_add(0x48); fl_l = r8; fl_r = (crate::simd_ops::ru64(rdi.wrapping_add(0x10u64))) as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 12; continue; } bb = 4; continue; }
            4 => { fl_l = rsi; fl_r = rdx; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 9; continue; } bb = 5; continue; }
            5 => { bb = 6; continue; }
            6 => { fl_l = r8; fl_r = rdx; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 9; continue; } bb = 7; continue; }
            7 => { rcx = (crate::simd_ops::ru32(rdi.wrapping_add(0x30u64))) as u32 as u64; rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x34u64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 9; continue; } bb = 8; continue; }
            8 => { rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0x20u64))) as u64; fl_l = rsi; fl_r = rdx; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 6; continue; } bb = 9; continue; }
            9 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x10u64))) as u64; rax = ((rax).wrapping_sub(0x8u64)) as u64; fl_l = rsi; fl_r = rax; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 11; continue; } bb = 10; continue; }
            10 => { rax = (0xffffffffffu64) as u64; crate::simd_ops::wu64(rsi, rax); bb = 11; continue; }
            11 => { rsi = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; r8 = rsi.wrapping_add(0x48); bb = 12; continue; }
            12 => { fl_l = rsi; fl_r = rdx; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 16; continue; } bb = 13; continue; }
            13 => { fl_l = r8; fl_r = rdx; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 16; continue; } bb = 14; continue; }
            14 => { rcx = (crate::simd_ops::ru32(rdi.wrapping_add(0x30u64))) as u32 as u64; rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x34u64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 16; continue; } bb = 15; continue; }
            15 => { rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0x20u64))) as u64; fl_l = rsi; fl_r = rdx; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 13; continue; } bb = 16; continue; }
            16 => { rax = (0x69fafafafaffu64) as u64; rcx = rdi.wrapping_add(0x18); crate::simd_ops::wu64(rsi, rax); let tmp_xchg: u64 = crate::simd_ops::atomic_xchg_u64(rdi.wrapping_add(0x28u64), r8); r8 = (tmp_xchg) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); let y_addr: u64 = r14; xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); xmm1 = (crate::simd_ops::ru128(r14.wrapping_add(0x20u64))) as u128; crate::simd_ops::wu32(rsi.wrapping_add(0x8u64), (0x0u32) as u32); rax = P5; crate::simd_ops::wu64(rsi.wrapping_add(0xcu64), rax); let y_addr: u64 = rsi.wrapping_add(0x14u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); crate::simd_ops::wu128(rsi.wrapping_add(0x34u64), (xmm1) as u128); rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x168u64))) as u64; crate::simd_ops::wu64(rsi, (0x3cu64) as u64); crate::simd_ops::lk_add_u32(rax.wrapping_add(0x30u64), (1) as u32); fl_l = (crate::simd_ops::ru8(rbp.wrapping_add(0x198u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 18; continue; } bb = 17; continue; }
            17 => { rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x1a0u64))) as u64; r8 = 0; fl_l = 0; fl_r = 0; rdx = ((r8.wrapping_add(0x1u64)) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P6), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 18; continue; }
            18 => { rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1412eef50(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1412f1870; const P1: u64 = 0x1412f0cc0;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0; let mut ymm1_hi: u128 = 0; let mut xmm6: u128 = 0; let mut xmm7: u128 = 0; let mut xmm8: u128 = 0; let mut xmm9: u128 = 0; let mut xmm10: u128 = 0; let mut xmm11: u128 = 0; let mut xmm12: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 45>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = (rsp) as u64; crate::simd_ops::wu8(rax.wrapping_add(0x20u64), ((r9 as u8)) as u8); crate::simd_ops::wu64(rax.wrapping_add(0x18u64), (r8) as u64); crate::simd_ops::wu64(rax.wrapping_add(0x10u64), (rdx) as u64); crate::simd_ops::wu64(rax.wrapping_add(0x8u64), (rcx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r12) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r13) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r15) as u64); rbp = ((rax.wrapping_sub(0x68u64)) as u64) as u64; rsp = ((rsp).wrapping_sub(0x128u64)) as u64; r11 = (crate::simd_ops::ru64(rdx)) as u64; r13 = (rdx) as u64; r9 = (crate::simd_ops::ru64(r8)) as u64; rsi = (r8) as u64; crate::simd_ops::wu128(rax.wrapping_sub(0x58u64), (xmm6) as u128); rdi = (0x6666666666666667u64) as u64; crate::simd_ops::wu128(rax.wrapping_sub(0x68u64), (xmm7) as u128); r10 = (r9) as u64; crate::simd_ops::wu128(rax.wrapping_sub(0x78u64), (xmm8) as u128); r10 = ((r10).wrapping_sub(r11)) as u64; crate::simd_ops::wu128(rax.wrapping_sub(0x88u64), (xmm9) as u128); r12 = ((r9.wrapping_sub(0xa0u64)) as u64) as u64; crate::simd_ops::wu128(rax.wrapping_sub(0x98u64), (xmm10) as u128); rcx = (r12) as u64; crate::simd_ops::wu128(rax.wrapping_sub(0xa8u64), (xmm11) as u128); rcx = ((rcx).wrapping_sub(r11)) as u64; crate::simd_ops::wu128(rax.wrapping_sub(0xb8u64), (xmm12) as u128); rax = (rdi) as u64; let prod = (rax as i64 as i128).wrapping_mul((r10) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; rax = (rdi) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), (r11) as u64); rdx = ((((rdx) as i64).wrapping_shr(6u32) as u64)) as u64; r8 = (rdx) as u64; r8 = ((r8).wrapping_shr(63u32)) as u64; rdx = ((rdx).wrapping_add(r8)) as u64; r8 = ((rsp.wrapping_add(0x30u64)) as u64) as u64; rdx = ((((rdx) as i64).wrapping_shr(1u32) as u64)) as u64; rbx = ((rdx.wrapping_add(rdx.wrapping_mul(4))) as u64) as u64; let prod = (rax as i64 as i128).wrapping_mul((rcx) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; rbx = ((rbx).wrapping_shl(5u32)) as u64; rdx = ((((rdx) as i64).wrapping_shr(6u32) as u64)) as u64; rbx = ((rbx).wrapping_add(r11)) as u64; rax = (rdx) as u64; rax = ((rax).wrapping_shr(63u32)) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; fl_l = (rdx) as u64 as u64; fl_r = (0x28u64) as u64 as u64; if ((((fl_l as u64)) as i64) <= (((fl_r as u64)) as i64)) { bb = 2; continue; } bb = 1; continue; }
            1 => { rcx = ((rdx.wrapping_add(0x1u64)) as u64) as u64; rcx = ((((rcx) as i64).wrapping_shr(3u32) as u64)) as u64; rdx = ((rsp.wrapping_add(0x40u64)) as u64) as u64; r15 = ((rcx.wrapping_add(rcx.wrapping_mul(4))) as u64) as u64; rsi = ((rcx.wrapping_add(rcx.wrapping_mul(4))) as u64) as u64; r15 = ((r15).wrapping_shl(6u32)) as u64; rsi = ((rsi).wrapping_shl(5u32)) as u64; rcx = ((rsp.wrapping_add(0x38u64)) as u64) as u64; rax = ((r15.wrapping_add(r11)) as u64) as u64; r14 = ((rsi.wrapping_add(r11)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (rax) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), (r14) as u64); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = ((rsi.wrapping_add(rbx)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (rax) as u64); r8 = ((rsp.wrapping_add(0x30u64)) as u64) as u64; rax = (rbx) as u64; rdx = ((rsp.wrapping_add(0x40u64)) as u64) as u64; rax = ((rax).wrapping_sub(rsi)) as u64; rcx = ((rsp.wrapping_add(0x38u64)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), (rax) as u64); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdi = (r12) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (r12) as u64); rdi = ((rdi).wrapping_sub(rsi)) as u64; r8 = ((rsp.wrapping_add(0x30u64)) as u64) as u64; r12 = ((r12).wrapping_sub(r15)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), (rdi) as u64); rdx = ((rsp.wrapping_add(0x40u64)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), (r12) as u64); rcx = ((rsp.wrapping_add(0x38u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; r8 = ((rsp.wrapping_add(0x30u64)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (rdi) as u64); rdx = ((rsp.wrapping_add(0x40u64)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), (rbx) as u64); rcx = ((rsp.wrapping_add(0x38u64)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), (r14) as u64); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rsi = (crate::simd_ops::ru64(rbp.wrapping_add(0x80u64))) as u64; bb = 3; continue; }
            2 => { rdx = ((rsp.wrapping_add(0x40u64)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (r12) as u64); rcx = ((rsp.wrapping_add(0x38u64)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), (rbx) as u64); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 3; continue; }
            3 => { r9 = (crate::simd_ops::ru64(r13)) as u64; r8 = ((rbx.wrapping_add(0xa0u64)) as u64) as u64; fl_l = (r9) as u64 as u64; fl_r = (rbx) as u64 as u64; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 8; continue; } bb = 4; continue; }
            4 => { rax = ((rbx.wrapping_sub(0x98u64)) as u64) as u64; bb = 5; continue; }
            5 => { rcx = (crate::simd_ops::ru32(rax)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rax.wrapping_add(0xa0u64))) as u32 as u64; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 8; continue; } bb = 6; continue; }
            6 => { if ((((fl_l as u32)) as i32) > (((fl_r as u32)) as i32)) { bb = 8; continue; } bb = 7; continue; }
            7 => { rbx = ((rbx).wrapping_sub(0xa0u64)) as u64; rax = ((rax).wrapping_sub(0xa0u64)) as u64; fl_l = (r9) as u64 as u64; fl_r = (rbx) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 5; continue; } bb = 8; continue; }
            8 => { rdx = (crate::simd_ops::ru64(rsi)) as u64; fl_l = (r8) as u64 as u64; fl_r = (rdx) as u64 as u64; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 13; continue; } bb = 9; continue; }
            9 => { rcx = (crate::simd_ops::ru32(rbx.wrapping_add(0x8u64))) as u32 as u64; bb = 10; continue; }
            10 => { fl_l = (crate::simd_ops::ru32(r8.wrapping_add(0x8u64))) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 13; continue; } bb = 11; continue; }
            11 => { if ((((fl_l as u32)) as i32) > (((fl_r as u32)) as i32)) { bb = 13; continue; } bb = 12; continue; }
            12 => { r8 = ((r8).wrapping_add(0xa0u64)) as u64; fl_l = (r8) as u64 as u64; fl_r = (rdx) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 10; continue; } bb = 13; continue; }
            13 => { r9 = (r8) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), (r8) as u64); r10 = (rbx) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), (rbx) as u64); bb = 14; continue; }
            14 => { fl_l = (r9) as u64 as u64; fl_r = (crate::simd_ops::ru64(rsi)) as u64 as u64; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 23; continue; } bb = 15; continue; }
            15 => { rax = ((r8.wrapping_add(0x20u64)) as u64) as u64; r12 = ((r8.wrapping_add(0x10u64)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), (rax) as u64); rcx = ((r9.wrapping_add(0x38u64)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), (r12) as u64); r15 = ((r8.wrapping_add(0x44u64)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (rcx) as u64); r14 = ((r8.wrapping_add(0x38u64)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), (r15) as u64); rdx = ((r8.wrapping_add(0x2cu64)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), (r14) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), (rdx) as u64); r10 = ((r8.wrapping_add(0x58u64)) as u64) as u64; bb = 16; continue; }
            16 => { r11 = (crate::simd_ops::ru32(rbx.wrapping_add(0x8u64))) as u32 as u64; fl_l = ((r11 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rcx.wrapping_sub(0x30u64))) as u32 as u64; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 21; continue; } bb = 17; continue; }
            17 => { if ((((fl_l as u32)) as i32) > (((fl_r as u32)) as i32)) { bb = 22; continue; } bb = 18; continue; }
            18 => { fl_l = (r8) as u64 as u64; fl_r = (r9) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 20; continue; } bb = 19; continue; }
            19 => { let y_addr: u64 = r10; xmm0 = core::ptr::read_unaligned(y_addr as *const u128); ymm0_hi = core::ptr::read_unaligned(y_addr.wrapping_add(16) as *const u128); xmm6 = ((crate::simd_ops::ru64(rax)) as u64) as u128; xmm12 = (crate::simd_ops::ru128(r12)) as u128; r12 = (crate::simd_ops::ru32(rax.wrapping_add(0x8u64))) as u32 as u64; rcx = (r8) as u64; rax = (crate::simd_ops::ru32(r14.wrapping_add(0x8u64))) as u32 as u64; xmm7 = ((crate::simd_ops::ru64(rdx)) as u64) as u128; xmm8 = ((crate::simd_ops::ru64(r14)) as u64) as u128; xmm9 = ((crate::simd_ops::ru64(r15)) as u64) as u128; xmm11 = ((crate::simd_ops::ru32(r8.wrapping_add(0xcu64))) as u32) as u128; r13 = (crate::simd_ops::ru32(rdx.wrapping_add(0x8u64))) as u32 as u64; rdx = (r9) as u64; xmm10 = ((crate::simd_ops::ru64(r10.wrapping_add(0x40u64))) as u64) as u128; r11 = (crate::simd_ops::ru32(r8)) as u32 as u64; rdi = (crate::simd_ops::ru32(r8.wrapping_add(0x4u64))) as u32 as u64; rsi = (crate::simd_ops::ru32(r8.wrapping_add(0x8u64))) as u32 as u64; r14 = (crate::simd_ops::ru32(r8.wrapping_add(0x50u64))) as u32 as u64; let y_addr: u64 = rsp.wrapping_add(0x68u64); core::ptr::write_unaligned(y_addr as *mut u128, xmm0); core::ptr::write_unaligned(y_addr.wrapping_add(16) as *mut u128, ymm0_hi); let y_addr: u64 = r10.wrapping_add(0x20u64); xmm0 = core::ptr::read_unaligned(y_addr as *const u128); ymm0_hi = core::ptr::read_unaligned(y_addr.wrapping_add(16) as *const u128); crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(r15.wrapping_add(0x8u64))) as u32 as u64; r15 = (crate::simd_ops::ru32(r8.wrapping_add(0x54u64))) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x88u64), ((rax as u32)) as u32); let y_addr: u64 = rbp.wrapping_sub(0x78u64); core::ptr::write_unaligned(y_addr as *mut u128, xmm0); core::ptr::write_unaligned(y_addr.wrapping_add(16) as *mut u128, ymm0_hi); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x20u64))) as u32 as u64; let y_addr: u64 = rsp.wrapping_add(0x68u64); xmm0 = core::ptr::read_unaligned(y_addr as *const u128); ymm0_hi = core::ptr::read_unaligned(y_addr.wrapping_add(16) as *const u128); let y_addr: u64 = rbp.wrapping_sub(0x78u64); xmm1 = core::ptr::read_unaligned(y_addr as *const u128); ymm1_hi = core::ptr::read_unaligned(y_addr.wrapping_add(16) as *const u128); rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; crate::simd_ops::wu32(rcx.wrapping_sub(0x30u64), ((rsi as u32)) as u32); rsi = (crate::simd_ops::ru64(rbp.wrapping_add(0x80u64))) as u64; crate::simd_ops::wu32(rcx.wrapping_sub(0x2cu64), ((xmm11 as u32)) as u32); crate::simd_ops::wu32(rcx.wrapping_sub(0x34u64), ((rdi as u32)) as u32); crate::simd_ops::wu128(rcx.wrapping_sub(0x28u64), (xmm12) as u128); crate::simd_ops::wu32(r9, ((r11 as u32)) as u32); crate::simd_ops::wu64(rcx, ((xmm8 as u64)) as u64); crate::simd_ops::wu32(rcx.wrapping_add(0x8u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x88u64))) as u32 as u64; crate::simd_ops::wu64(rcx.wrapping_sub(0x18u64), ((xmm6 as u64)) as u64); crate::simd_ops::wu64(rcx.wrapping_sub(0xcu64), ((xmm7 as u64)) as u64); crate::simd_ops::wu64(rcx.wrapping_add(0xcu64), ((xmm9 as u64)) as u64); crate::simd_ops::wu32(rcx.wrapping_add(0x14u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; crate::simd_ops::wu32(rcx.wrapping_sub(0x10u64), ((r12 as u32)) as u32); r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; crate::simd_ops::wu32(rcx.wrapping_sub(0x4u64), ((r13 as u32)) as u32); let y_addr: u64 = rcx.wrapping_add(0x20u64); core::ptr::write_unaligned(y_addr as *mut u128, xmm0); core::ptr::write_unaligned(y_addr.wrapping_add(16) as *mut u128, ymm0_hi); let y_addr: u64 = rcx.wrapping_add(0x40u64); core::ptr::write_unaligned(y_addr as *mut u128, xmm1); core::ptr::write_unaligned(y_addr.wrapping_add(16) as *mut u128, ymm1_hi); crate::simd_ops::wu32(rcx.wrapping_add(0x18u64), ((r14 as u32)) as u32); r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; crate::simd_ops::wu32(rcx.wrapping_add(0x1cu64), ((r15 as u32)) as u32); r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; crate::simd_ops::wu64(rcx.wrapping_add(0x60u64), ((xmm10 as u64)) as u64); bb = 20; continue; }
            20 => { r12 = ((r12).wrapping_add(0xa0u64)) as u64; rax = ((rax).wrapping_add(0xa0u64)) as u64; rdx = ((rdx).wrapping_add(0xa0u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), (rax) as u64); r14 = ((r14).wrapping_add(0xa0u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), (rdx) as u64); r15 = ((r15).wrapping_add(0xa0u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), (r14) as u64); r8 = ((r8).wrapping_add(0xa0u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), (r15) as u64); fl_l = ((r10).wrapping_add(0xa0u64)) as u64 as u64; fl_r = fl_l; r10 = ((fl_l as u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), (r12) as u64); bb = 21; continue; }
            21 => { rcx = ((rcx).wrapping_add(0xa0u64)) as u64; r9 = ((r9).wrapping_add(0xa0u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (rcx) as u64); fl_l = (r9) as u64 as u64; fl_r = (crate::simd_ops::ru64(rsi)) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 16; continue; } bb = 22; continue; }
            22 => { r10 = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; r13 = (crate::simd_ops::ru64(rbp.wrapping_add(0x78u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), (r9) as u64); bb = 23; continue; }
            23 => { rax = (crate::simd_ops::ru64(r13)) as u64; fl_l = (rax) as u64 as u64; fl_r = (r10) as u64 as u64; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 31; continue; } bb = 24; continue; }
            24 => { rcx = ((r10.wrapping_sub(0x68u64)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (rcx) as u64); r9 = ((rbx.wrapping_add(0x38u64)) as u64) as u64; bb = 25; continue; }
            25 => { rdx = (crate::simd_ops::ru32(rcx.wrapping_sub(0x30u64))) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(r9.wrapping_sub(0x30u64))) as u32 as u64; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 29; continue; } bb = 26; continue; }
            26 => { if ((((fl_l as u32)) as i32) > (((fl_r as u32)) as i32)) { bb = 30; continue; } bb = 27; continue; }
            27 => { rbx = ((rbx).wrapping_sub(0xa0u64)) as u64; rdx = ((rcx.wrapping_sub(0x38u64)) as u64) as u64; r9 = ((r9).wrapping_sub(0xa0u64)) as u64; fl_l = (rbx) as u64 as u64; fl_r = (rdx) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 29; continue; } bb = 28; continue; }
            28 => { let y_addr: u64 = r9.wrapping_add(0x20u64); xmm0 = core::ptr::read_unaligned(y_addr as *const u128); ymm0_hi = core::ptr::read_unaligned(y_addr.wrapping_add(16) as *const u128); rax = (crate::simd_ops::ru32(r9.wrapping_add(0x8u64))) as u32 as u64; rcx = (rbx) as u64; xmm11 = ((crate::simd_ops::ru32(r9.wrapping_sub(0x2cu64))) as u32) as u128; xmm12 = (crate::simd_ops::ru128(r9.wrapping_sub(0x28u64))) as u128; xmm6 = ((crate::simd_ops::ru64(r9.wrapping_sub(0x18u64))) as u64) as u128; xmm7 = ((crate::simd_ops::ru64(r9.wrapping_sub(0xcu64))) as u64) as u128; xmm8 = ((crate::simd_ops::ru64(r9)) as u64) as u128; xmm9 = ((crate::simd_ops::ru64(r9.wrapping_add(0xcu64))) as u64) as u128; xmm10 = ((crate::simd_ops::ru64(r9.wrapping_add(0x60u64))) as u64) as u128; r11 = (crate::simd_ops::ru32(rbx)) as u32 as u64; rdi = (crate::simd_ops::ru32(r9.wrapping_sub(0x34u64))) as u32 as u64; rsi = (crate::simd_ops::ru32(r9.wrapping_sub(0x30u64))) as u32 as u64; r12 = (crate::simd_ops::ru32(r9.wrapping_sub(0x10u64))) as u32 as u64; r13 = (crate::simd_ops::ru32(r9.wrapping_sub(0x4u64))) as u32 as u64; r14 = (crate::simd_ops::ru32(r9.wrapping_add(0x18u64))) as u32 as u64; r15 = (crate::simd_ops::ru32(r9.wrapping_add(0x1cu64))) as u32 as u64; let y_addr: u64 = rbp.wrapping_sub(0x78u64); core::ptr::write_unaligned(y_addr as *mut u128, xmm0); core::ptr::write_unaligned(y_addr.wrapping_add(16) as *mut u128, ymm0_hi); let y_addr: u64 = r9.wrapping_add(0x40u64); xmm0 = core::ptr::read_unaligned(y_addr as *const u128); ymm0_hi = core::ptr::read_unaligned(y_addr.wrapping_add(16) as *const u128); crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(r9.wrapping_add(0x14u64))) as u32 as u64; let y_addr: u64 = rsp.wrapping_add(0x68u64); core::ptr::write_unaligned(y_addr as *mut u128, xmm0); core::ptr::write_unaligned(y_addr.wrapping_add(16) as *mut u128, ymm0_hi); crate::simd_ops::wu32(rbp.wrapping_add(0x88u64), ((rax as u32)) as u32); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x20u64))) as u32 as u64; let y_addr: u64 = rbp.wrapping_sub(0x78u64); xmm0 = core::ptr::read_unaligned(y_addr as *const u128); ymm0_hi = core::ptr::read_unaligned(y_addr.wrapping_add(16) as *const u128); let y_addr: u64 = rsp.wrapping_add(0x68u64); xmm1 = core::ptr::read_unaligned(y_addr as *const u128); ymm1_hi = core::ptr::read_unaligned(y_addr.wrapping_add(16) as *const u128); crate::simd_ops::wu32(rcx.wrapping_sub(0x2cu64), ((xmm11 as u32)) as u32); crate::simd_ops::wu32(rcx.wrapping_sub(0x34u64), ((rdi as u32)) as u32); crate::simd_ops::wu32(rcx.wrapping_sub(0x30u64), ((rsi as u32)) as u32); crate::simd_ops::wu128(rcx.wrapping_sub(0x28u64), (xmm12) as u128); crate::simd_ops::wu32(rdx, ((r11 as u32)) as u32); crate::simd_ops::wu64(rcx, ((xmm8 as u64)) as u64); crate::simd_ops::wu32(rcx.wrapping_add(0x8u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x88u64))) as u32 as u64; crate::simd_ops::wu64(rcx.wrapping_sub(0x18u64), ((xmm6 as u64)) as u64); crate::simd_ops::wu64(rcx.wrapping_sub(0xcu64), ((xmm7 as u64)) as u64); crate::simd_ops::wu64(rcx.wrapping_add(0xcu64), ((xmm9 as u64)) as u64); crate::simd_ops::wu32(rcx.wrapping_sub(0x4u64), ((r13 as u32)) as u32); r13 = (crate::simd_ops::ru64(rbp.wrapping_add(0x78u64))) as u64; crate::simd_ops::wu32(rcx.wrapping_add(0x14u64), ((rax as u32)) as u32); crate::simd_ops::wu32(rcx.wrapping_sub(0x10u64), ((r12 as u32)) as u32); let y_addr: u64 = rcx.wrapping_add(0x20u64); core::ptr::write_unaligned(y_addr as *mut u128, xmm0); core::ptr::write_unaligned(y_addr.wrapping_add(16) as *mut u128, ymm0_hi); let y_addr: u64 = rcx.wrapping_add(0x40u64); core::ptr::write_unaligned(y_addr as *mut u128, xmm1); core::ptr::write_unaligned(y_addr.wrapping_add(16) as *mut u128, ymm1_hi); crate::simd_ops::wu64(rcx.wrapping_add(0x60u64), ((xmm10 as u64)) as u64); crate::simd_ops::wu32(rcx.wrapping_add(0x18u64), ((r14 as u32)) as u32); crate::simd_ops::wu32(rcx.wrapping_add(0x1cu64), ((r15 as u32)) as u32); bb = 29; continue; }
            29 => { rax = (crate::simd_ops::ru64(r13)) as u64; rcx = ((rcx).wrapping_sub(0xa0u64)) as u64; r10 = ((r10).wrapping_sub(0xa0u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (rcx) as u64); fl_l = (rax) as u64 as u64; fl_r = (r10) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 25; continue; } bb = 30; continue; }
            30 => { r9 = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsi = (crate::simd_ops::ru64(rbp.wrapping_add(0x80u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), (r10) as u64); bb = 31; continue; }
            31 => { rcx = (crate::simd_ops::ru64(rsi)) as u64; fl_l = (r10) as u64 as u64; fl_r = (rax) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 36; continue; } bb = 32; continue; }
            32 => { fl_l = (r9) as u64 as u64; fl_r = (rcx) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 42; continue; } bb = 33; continue; }
            33 => { fl_l = (r8) as u64 as u64; fl_r = (r9) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 35; continue; } bb = 34; continue; }
            34 => { let y_addr: u64 = rbx.wrapping_add(0x58u64); xmm0 = core::ptr::read_unaligned(y_addr as *const u128); ymm0_hi = core::ptr::read_unaligned(y_addr.wrapping_add(16) as *const u128); rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x40u64))) as u32 as u64; rdx = (r8) as u64; xmm11 = ((crate::simd_ops::ru32(rbx.wrapping_add(0xcu64))) as u32) as u128; xmm12 = (crate::simd_ops::ru128(rbx.wrapping_add(0x10u64))) as u128; xmm6 = ((crate::simd_ops::ru64(rbx.wrapping_add(0x20u64))) as u64) as u128; xmm7 = ((crate::simd_ops::ru64(rbx.wrapping_add(0x2cu64))) as u64) as u128; xmm8 = ((crate::simd_ops::ru64(rbx.wrapping_add(0x38u64))) as u64) as u128; xmm9 = ((crate::simd_ops::ru64(rbx.wrapping_add(0x44u64))) as u64) as u128; xmm10 = ((crate::simd_ops::ru64(rbx.wrapping_add(0x98u64))) as u64) as u128; r11 = (crate::simd_ops::ru32(rbx)) as u32 as u64; rcx = (rbx) as u64; rdi = (crate::simd_ops::ru32(rbx.wrapping_add(0x4u64))) as u32 as u64; rsi = (crate::simd_ops::ru32(rbx.wrapping_add(0x8u64))) as u32 as u64; r12 = (crate::simd_ops::ru32(rbx.wrapping_add(0x28u64))) as u32 as u64; r13 = (crate::simd_ops::ru32(rbx.wrapping_add(0x34u64))) as u32 as u64; r14 = (crate::simd_ops::ru32(rbx.wrapping_add(0x50u64))) as u32 as u64; r15 = (crate::simd_ops::ru32(rbx.wrapping_add(0x54u64))) as u32 as u64; let y_addr: u64 = rbp.wrapping_sub(0x78u64); core::ptr::write_unaligned(y_addr as *mut u128, xmm0); core::ptr::write_unaligned(y_addr.wrapping_add(16) as *mut u128, ymm0_hi); let y_addr: u64 = rbx.wrapping_add(0x78u64); xmm0 = core::ptr::read_unaligned(y_addr as *const u128); ymm0_hi = core::ptr::read_unaligned(y_addr.wrapping_add(16) as *const u128); crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x4cu64))) as u32 as u64; let y_addr: u64 = rsp.wrapping_add(0x68u64); core::ptr::write_unaligned(y_addr as *mut u128, xmm0); core::ptr::write_unaligned(y_addr.wrapping_add(16) as *mut u128, ymm0_hi); crate::simd_ops::wu32(rbp.wrapping_add(0x88u64), ((rax as u32)) as u32); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x20u64))) as u32 as u64; let y_addr: u64 = rbp.wrapping_sub(0x78u64); xmm0 = core::ptr::read_unaligned(y_addr as *const u128); ymm0_hi = core::ptr::read_unaligned(y_addr.wrapping_add(16) as *const u128); let y_addr: u64 = rsp.wrapping_add(0x68u64); xmm1 = core::ptr::read_unaligned(y_addr as *const u128); ymm1_hi = core::ptr::read_unaligned(y_addr.wrapping_add(16) as *const u128); crate::simd_ops::wu32(r8.wrapping_add(0xcu64), ((xmm11 as u32)) as u32); crate::simd_ops::wu32(r8, ((r11 as u32)) as u32); crate::simd_ops::wu32(r8.wrapping_add(0x4u64), ((rdi as u32)) as u32); crate::simd_ops::wu32(r8.wrapping_add(0x8u64), ((rsi as u32)) as u32); crate::simd_ops::wu128(r8.wrapping_add(0x10u64), (xmm12) as u128); crate::simd_ops::wu64(r8.wrapping_add(0x38u64), ((xmm8 as u64)) as u64); crate::simd_ops::wu32(r8.wrapping_add(0x40u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x88u64))) as u32 as u64; crate::simd_ops::wu64(r8.wrapping_add(0x20u64), ((xmm6 as u64)) as u64); crate::simd_ops::wu64(r8.wrapping_add(0x2cu64), ((xmm7 as u64)) as u64); crate::simd_ops::wu64(r8.wrapping_add(0x44u64), ((xmm9 as u64)) as u64); crate::simd_ops::wu32(r8.wrapping_add(0x4cu64), ((rax as u32)) as u32); crate::simd_ops::wu32(r8.wrapping_add(0x28u64), ((r12 as u32)) as u32); crate::simd_ops::wu32(r8.wrapping_add(0x34u64), ((r13 as u32)) as u32); let y_addr: u64 = r8.wrapping_add(0x58u64); core::ptr::write_unaligned(y_addr as *mut u128, xmm0); core::ptr::write_unaligned(y_addr.wrapping_add(16) as *mut u128, ymm0_hi); let y_addr: u64 = r8.wrapping_add(0x78u64); core::ptr::write_unaligned(y_addr as *mut u128, xmm1); core::ptr::write_unaligned(y_addr.wrapping_add(16) as *mut u128, ymm1_hi); crate::simd_ops::wu64(r8.wrapping_add(0x98u64), ((xmm10 as u64)) as u64); crate::simd_ops::wu32(r8.wrapping_add(0x50u64), ((r14 as u32)) as u32); crate::simd_ops::wu32(r8.wrapping_add(0x54u64), ((r15 as u32)) as u32); bb = 35; continue; }
            35 => { let y_addr: u64 = rbx.wrapping_add(0x58u64); xmm0 = core::ptr::read_unaligned(y_addr as *const u128); ymm0_hi = core::ptr::read_unaligned(y_addr.wrapping_add(16) as *const u128); rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x40u64))) as u32 as u64; r8 = ((r8).wrapping_add(0xa0u64)) as u64; xmm11 = ((crate::simd_ops::ru32(rbx.wrapping_add(0xcu64))) as u32) as u128; xmm12 = (crate::simd_ops::ru128(rbx.wrapping_add(0x10u64))) as u128; xmm6 = ((crate::simd_ops::ru64(rbx.wrapping_add(0x20u64))) as u64) as u128; xmm7 = ((crate::simd_ops::ru64(rbx.wrapping_add(0x2cu64))) as u64) as u128; xmm8 = ((crate::simd_ops::ru64(rbx.wrapping_add(0x38u64))) as u64) as u128; xmm9 = ((crate::simd_ops::ru64(rbx.wrapping_add(0x44u64))) as u64) as u128; xmm10 = ((crate::simd_ops::ru64(rbx.wrapping_add(0x98u64))) as u64) as u128; r11 = (crate::simd_ops::ru32(rbx)) as u32 as u64; rdx = (r9) as u64; rdi = (crate::simd_ops::ru32(rbx.wrapping_add(0x4u64))) as u32 as u64; rcx = (rbx) as u64; rsi = (crate::simd_ops::ru32(rbx.wrapping_add(0x8u64))) as u32 as u64; r12 = (crate::simd_ops::ru32(rbx.wrapping_add(0x28u64))) as u32 as u64; r13 = (crate::simd_ops::ru32(rbx.wrapping_add(0x34u64))) as u32 as u64; r14 = (crate::simd_ops::ru32(rbx.wrapping_add(0x50u64))) as u32 as u64; r15 = (crate::simd_ops::ru32(rbx.wrapping_add(0x54u64))) as u32 as u64; let y_addr: u64 = rbp.wrapping_sub(0x78u64); core::ptr::write_unaligned(y_addr as *mut u128, xmm0); core::ptr::write_unaligned(y_addr.wrapping_add(16) as *mut u128, ymm0_hi); let y_addr: u64 = rbx.wrapping_add(0x78u64); xmm0 = core::ptr::read_unaligned(y_addr as *const u128); ymm0_hi = core::ptr::read_unaligned(y_addr.wrapping_add(16) as *const u128); crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x4cu64))) as u32 as u64; let y_addr: u64 = rsp.wrapping_add(0x68u64); core::ptr::write_unaligned(y_addr as *mut u128, xmm0); core::ptr::write_unaligned(y_addr.wrapping_add(16) as *mut u128, ymm0_hi); crate::simd_ops::wu32(rbp.wrapping_add(0x88u64), ((rax as u32)) as u32); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x20u64))) as u32 as u64; fl_l = ((rbx).wrapping_add(0xa0u64)) as u64 as u64; fl_r = fl_l; rbx = ((fl_l as u64)) as u64; let y_addr: u64 = rbp.wrapping_sub(0x78u64); xmm0 = core::ptr::read_unaligned(y_addr as *const u128); ymm0_hi = core::ptr::read_unaligned(y_addr.wrapping_add(16) as *const u128); let y_addr: u64 = rsp.wrapping_add(0x68u64); xmm1 = core::ptr::read_unaligned(y_addr as *const u128); ymm1_hi = core::ptr::read_unaligned(y_addr.wrapping_add(16) as *const u128); crate::simd_ops::wu32(r9.wrapping_add(0xcu64), ((xmm11 as u32)) as u32); crate::simd_ops::wu32(r9, ((r11 as u32)) as u32); crate::simd_ops::wu32(r9.wrapping_add(0x4u64), ((rdi as u32)) as u32); crate::simd_ops::wu32(r9.wrapping_add(0x8u64), ((rsi as u32)) as u32); crate::simd_ops::wu128(r9.wrapping_add(0x10u64), (xmm12) as u128); crate::simd_ops::wu64(r9.wrapping_add(0x38u64), ((xmm8 as u64)) as u64); crate::simd_ops::wu32(r9.wrapping_add(0x40u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x88u64))) as u32 as u64; crate::simd_ops::wu64(r9.wrapping_add(0x20u64), ((xmm6 as u64)) as u64); crate::simd_ops::wu64(r9.wrapping_add(0x2cu64), ((xmm7 as u64)) as u64); crate::simd_ops::wu64(r9.wrapping_add(0x44u64), ((xmm9 as u64)) as u64); crate::simd_ops::wu32(r9.wrapping_add(0x4cu64), ((rax as u32)) as u32); crate::simd_ops::wu32(r9.wrapping_add(0x28u64), ((r12 as u32)) as u32); crate::simd_ops::wu32(r9.wrapping_add(0x34u64), ((r13 as u32)) as u32); let y_addr: u64 = r9.wrapping_add(0x58u64); core::ptr::write_unaligned(y_addr as *mut u128, xmm0); core::ptr::write_unaligned(y_addr.wrapping_add(16) as *mut u128, ymm0_hi); let y_addr: u64 = r9.wrapping_add(0x78u64); core::ptr::write_unaligned(y_addr as *mut u128, xmm1); core::ptr::write_unaligned(y_addr.wrapping_add(16) as *mut u128, ymm1_hi); crate::simd_ops::wu64(r9.wrapping_add(0x98u64), ((xmm10 as u64)) as u64); crate::simd_ops::wu32(r9.wrapping_add(0x50u64), ((r14 as u32)) as u32); crate::simd_ops::wu32(r9.wrapping_add(0x54u64), ((r15 as u32)) as u32); bb = 41; continue; }
            36 => { r10 = ((r10).wrapping_add(0xffffffffffffff60u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), (r10) as u64); fl_l = (r9) as u64 as u64; fl_r = (rcx) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 40; continue; } bb = 37; continue; }
            37 => { rbx = ((rbx).wrapping_sub(0xa0u64)) as u64; fl_l = (r10) as u64 as u64; fl_r = (rbx) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 39; continue; } bb = 38; continue; }
            38 => { let y_addr: u64 = r10.wrapping_add(0x58u64); xmm0 = core::ptr::read_unaligned(y_addr as *const u128); ymm0_hi = core::ptr::read_unaligned(y_addr.wrapping_add(16) as *const u128); rax = (crate::simd_ops::ru32(r10.wrapping_add(0x50u64))) as u32 as u64; rdx = (rbx) as u64; xmm6 = ((crate::simd_ops::ru32(r10.wrapping_add(0xcu64))) as u32) as u128; xmm11 = (crate::simd_ops::ru128(r10.wrapping_add(0x10u64))) as u128; xmm7 = ((crate::simd_ops::ru64(r10.wrapping_add(0x20u64))) as u64) as u128; xmm8 = ((crate::simd_ops::ru64(r10.wrapping_add(0x2cu64))) as u64) as u128; xmm9 = ((crate::simd_ops::ru64(r10.wrapping_add(0x38u64))) as u64) as u128; xmm10 = ((crate::simd_ops::ru64(r10.wrapping_add(0x44u64))) as u64) as u128; xmm12 = ((crate::simd_ops::ru64(r10.wrapping_add(0x98u64))) as u64) as u128; r11 = (crate::simd_ops::ru32(r10)) as u32 as u64; rcx = (r10) as u64; rdi = (crate::simd_ops::ru32(r10.wrapping_add(0x4u64))) as u32 as u64; rsi = (crate::simd_ops::ru32(r10.wrapping_add(0x8u64))) as u32 as u64; r14 = (crate::simd_ops::ru32(r10.wrapping_add(0x28u64))) as u32 as u64; r15 = (crate::simd_ops::ru32(r10.wrapping_add(0x34u64))) as u32 as u64; r12 = (crate::simd_ops::ru32(r10.wrapping_add(0x40u64))) as u32 as u64; r13 = (crate::simd_ops::ru32(r10.wrapping_add(0x4cu64))) as u32 as u64; let y_addr: u64 = rbp.wrapping_sub(0x78u64); core::ptr::write_unaligned(y_addr as *mut u128, xmm0); core::ptr::write_unaligned(y_addr.wrapping_add(16) as *mut u128, ymm0_hi); let y_addr: u64 = r10.wrapping_add(0x78u64); xmm0 = core::ptr::read_unaligned(y_addr as *const u128); ymm0_hi = core::ptr::read_unaligned(y_addr.wrapping_add(16) as *const u128); crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(r10.wrapping_add(0x54u64))) as u32 as u64; let y_addr: u64 = rsp.wrapping_add(0x68u64); core::ptr::write_unaligned(y_addr as *mut u128, xmm0); core::ptr::write_unaligned(y_addr.wrapping_add(16) as *mut u128, ymm0_hi); crate::simd_ops::wu32(rbp.wrapping_add(0x88u64), ((rax as u32)) as u32); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x20u64))) as u32 as u64; let y_addr: u64 = rbp.wrapping_sub(0x78u64); xmm0 = core::ptr::read_unaligned(y_addr as *const u128); ymm0_hi = core::ptr::read_unaligned(y_addr.wrapping_add(16) as *const u128); let y_addr: u64 = rsp.wrapping_add(0x68u64); xmm1 = core::ptr::read_unaligned(y_addr as *const u128); ymm1_hi = core::ptr::read_unaligned(y_addr.wrapping_add(16) as *const u128); crate::simd_ops::wu32(rbx.wrapping_add(0xcu64), ((xmm6 as u32)) as u32); crate::simd_ops::wu32(rbx, ((r11 as u32)) as u32); crate::simd_ops::wu32(rbx.wrapping_add(0x4u64), ((rdi as u32)) as u32); crate::simd_ops::wu32(rbx.wrapping_add(0x8u64), ((rsi as u32)) as u32); crate::simd_ops::wu128(rbx.wrapping_add(0x10u64), (xmm11) as u128); crate::simd_ops::wu64(rbx.wrapping_add(0x20u64), ((xmm7 as u64)) as u64); crate::simd_ops::wu64(rbx.wrapping_add(0x2cu64), ((xmm8 as u64)) as u64); crate::simd_ops::wu64(rbx.wrapping_add(0x38u64), ((xmm9 as u64)) as u64); crate::simd_ops::wu64(rbx.wrapping_add(0x44u64), ((xmm10 as u64)) as u64); crate::simd_ops::wu32(rbx.wrapping_add(0x28u64), ((r14 as u32)) as u32); crate::simd_ops::wu32(rbx.wrapping_add(0x34u64), ((r15 as u32)) as u32); crate::simd_ops::wu32(rbx.wrapping_add(0x40u64), ((r12 as u32)) as u32); crate::simd_ops::wu32(rbx.wrapping_add(0x4cu64), ((r13 as u32)) as u32); let y_addr: u64 = rbx.wrapping_add(0x58u64); core::ptr::write_unaligned(y_addr as *mut u128, xmm0); core::ptr::write_unaligned(y_addr.wrapping_add(16) as *mut u128, ymm0_hi); crate::simd_ops::wu32(rbx.wrapping_add(0x50u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x88u64))) as u32 as u64; let y_addr: u64 = rbx.wrapping_add(0x78u64); core::ptr::write_unaligned(y_addr as *mut u128, xmm1); core::ptr::write_unaligned(y_addr.wrapping_add(16) as *mut u128, ymm1_hi); crate::simd_ops::wu32(rbx.wrapping_add(0x54u64), ((rax as u32)) as u32); crate::simd_ops::wu64(rbx.wrapping_add(0x98u64), ((xmm12 as u64)) as u64); bb = 39; continue; }
            39 => { let y_addr: u64 = rbx.wrapping_add(0x58u64); xmm0 = core::ptr::read_unaligned(y_addr as *const u128); ymm0_hi = core::ptr::read_unaligned(y_addr.wrapping_add(16) as *const u128); rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x40u64))) as u32 as u64; r8 = ((r8).wrapping_sub(0xa0u64)) as u64; xmm11 = ((crate::simd_ops::ru32(rbx.wrapping_add(0xcu64))) as u32) as u128; xmm12 = (crate::simd_ops::ru128(rbx.wrapping_add(0x10u64))) as u128; xmm6 = ((crate::simd_ops::ru64(rbx.wrapping_add(0x20u64))) as u64) as u128; xmm7 = ((crate::simd_ops::ru64(rbx.wrapping_add(0x2cu64))) as u64) as u128; xmm8 = ((crate::simd_ops::ru64(rbx.wrapping_add(0x38u64))) as u64) as u128; xmm9 = ((crate::simd_ops::ru64(rbx.wrapping_add(0x44u64))) as u64) as u128; xmm10 = ((crate::simd_ops::ru64(rbx.wrapping_add(0x98u64))) as u64) as u128; r11 = (crate::simd_ops::ru32(rbx)) as u32 as u64; rdx = (r8) as u64; rdi = (crate::simd_ops::ru32(rbx.wrapping_add(0x4u64))) as u32 as u64; rcx = (rbx) as u64; rsi = (crate::simd_ops::ru32(rbx.wrapping_add(0x8u64))) as u32 as u64; r12 = (crate::simd_ops::ru32(rbx.wrapping_add(0x28u64))) as u32 as u64; r13 = (crate::simd_ops::ru32(rbx.wrapping_add(0x34u64))) as u32 as u64; r14 = (crate::simd_ops::ru32(rbx.wrapping_add(0x50u64))) as u32 as u64; r15 = (crate::simd_ops::ru32(rbx.wrapping_add(0x54u64))) as u32 as u64; let y_addr: u64 = rbp.wrapping_sub(0x78u64); core::ptr::write_unaligned(y_addr as *mut u128, xmm0); core::ptr::write_unaligned(y_addr.wrapping_add(16) as *mut u128, ymm0_hi); let y_addr: u64 = rbx.wrapping_add(0x78u64); xmm0 = core::ptr::read_unaligned(y_addr as *const u128); ymm0_hi = core::ptr::read_unaligned(y_addr.wrapping_add(16) as *const u128); crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x4cu64))) as u32 as u64; let y_addr: u64 = rsp.wrapping_add(0x68u64); core::ptr::write_unaligned(y_addr as *mut u128, xmm0); core::ptr::write_unaligned(y_addr.wrapping_add(16) as *mut u128, ymm0_hi); crate::simd_ops::wu32(rbp.wrapping_add(0x88u64), ((rax as u32)) as u32); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x20u64))) as u32 as u64; let y_addr: u64 = rbp.wrapping_sub(0x78u64); xmm0 = core::ptr::read_unaligned(y_addr as *const u128); ymm0_hi = core::ptr::read_unaligned(y_addr.wrapping_add(16) as *const u128); let y_addr: u64 = rsp.wrapping_add(0x68u64); xmm1 = core::ptr::read_unaligned(y_addr as *const u128); ymm1_hi = core::ptr::read_unaligned(y_addr.wrapping_add(16) as *const u128); crate::simd_ops::wu32(r8.wrapping_add(0x8u64), ((rsi as u32)) as u32); rsi = (crate::simd_ops::ru64(rbp.wrapping_add(0x80u64))) as u64; crate::simd_ops::wu32(r8.wrapping_add(0xcu64), ((xmm11 as u32)) as u32); crate::simd_ops::wu32(r8, ((r11 as u32)) as u32); crate::simd_ops::wu32(r8.wrapping_add(0x4u64), ((rdi as u32)) as u32); crate::simd_ops::wu128(r8.wrapping_add(0x10u64), (xmm12) as u128); crate::simd_ops::wu64(r8.wrapping_add(0x38u64), ((xmm8 as u64)) as u64); crate::simd_ops::wu32(r8.wrapping_add(0x40u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x88u64))) as u32 as u64; crate::simd_ops::wu64(r8.wrapping_add(0x20u64), ((xmm6 as u64)) as u64); crate::simd_ops::wu64(r8.wrapping_add(0x2cu64), ((xmm7 as u64)) as u64); crate::simd_ops::wu64(r8.wrapping_add(0x44u64), ((xmm9 as u64)) as u64); crate::simd_ops::wu32(r8.wrapping_add(0x34u64), ((r13 as u32)) as u32); r13 = (crate::simd_ops::ru64(rbp.wrapping_add(0x78u64))) as u64; crate::simd_ops::wu32(r8.wrapping_add(0x4cu64), ((rax as u32)) as u32); crate::simd_ops::wu32(r8.wrapping_add(0x28u64), ((r12 as u32)) as u32); let y_addr: u64 = r8.wrapping_add(0x58u64); core::ptr::write_unaligned(y_addr as *mut u128, xmm0); core::ptr::write_unaligned(y_addr.wrapping_add(16) as *mut u128, ymm0_hi); let y_addr: u64 = r8.wrapping_add(0x78u64); core::ptr::write_unaligned(y_addr as *mut u128, xmm1); core::ptr::write_unaligned(y_addr.wrapping_add(16) as *mut u128, ymm1_hi); crate::simd_ops::wu64(r8.wrapping_add(0x98u64), ((xmm10 as u64)) as u64); crate::simd_ops::wu32(r8.wrapping_add(0x50u64), ((r14 as u32)) as u32); crate::simd_ops::wu32(r8.wrapping_add(0x54u64), ((r15 as u32)) as u32); bb = 14; continue; }
            40 => { let y_addr: u64 = r9.wrapping_add(0x58u64); xmm0 = core::ptr::read_unaligned(y_addr as *const u128); ymm0_hi = core::ptr::read_unaligned(y_addr.wrapping_add(16) as *const u128); rax = (crate::simd_ops::ru32(r9.wrapping_add(0x40u64))) as u32 as u64; rdx = (r10) as u64; xmm11 = ((crate::simd_ops::ru32(r9.wrapping_add(0xcu64))) as u32) as u128; xmm12 = (crate::simd_ops::ru128(r9.wrapping_add(0x10u64))) as u128; xmm6 = ((crate::simd_ops::ru64(r9.wrapping_add(0x20u64))) as u64) as u128; xmm7 = ((crate::simd_ops::ru64(r9.wrapping_add(0x2cu64))) as u64) as u128; xmm8 = ((crate::simd_ops::ru64(r9.wrapping_add(0x38u64))) as u64) as u128; xmm9 = ((crate::simd_ops::ru64(r9.wrapping_add(0x44u64))) as u64) as u128; xmm10 = ((crate::simd_ops::ru64(r9.wrapping_add(0x98u64))) as u64) as u128; r11 = (crate::simd_ops::ru32(r9)) as u32 as u64; rcx = (r9) as u64; rdi = (crate::simd_ops::ru32(r9.wrapping_add(0x4u64))) as u32 as u64; rsi = (crate::simd_ops::ru32(r9.wrapping_add(0x8u64))) as u32 as u64; r12 = (crate::simd_ops::ru32(r9.wrapping_add(0x28u64))) as u32 as u64; r13 = (crate::simd_ops::ru32(r9.wrapping_add(0x34u64))) as u32 as u64; r14 = (crate::simd_ops::ru32(r9.wrapping_add(0x50u64))) as u32 as u64; r15 = (crate::simd_ops::ru32(r9.wrapping_add(0x54u64))) as u32 as u64; let y_addr: u64 = rbp.wrapping_sub(0x78u64); core::ptr::write_unaligned(y_addr as *mut u128, xmm0); core::ptr::write_unaligned(y_addr.wrapping_add(16) as *mut u128, ymm0_hi); let y_addr: u64 = r9.wrapping_add(0x78u64); xmm0 = core::ptr::read_unaligned(y_addr as *const u128); ymm0_hi = core::ptr::read_unaligned(y_addr.wrapping_add(16) as *const u128); crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(r9.wrapping_add(0x4cu64))) as u32 as u64; let y_addr: u64 = rsp.wrapping_add(0x68u64); core::ptr::write_unaligned(y_addr as *mut u128, xmm0); core::ptr::write_unaligned(y_addr.wrapping_add(16) as *mut u128, ymm0_hi); crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), (r10) as u64); crate::simd_ops::wu32(rbp.wrapping_add(0x88u64), ((rax as u32)) as u32); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x20u64))) as u32 as u64; let y_addr: u64 = rbp.wrapping_sub(0x78u64); xmm0 = core::ptr::read_unaligned(y_addr as *const u128); ymm0_hi = core::ptr::read_unaligned(y_addr.wrapping_add(16) as *const u128); let y_addr: u64 = rsp.wrapping_add(0x68u64); xmm1 = core::ptr::read_unaligned(y_addr as *const u128); ymm1_hi = core::ptr::read_unaligned(y_addr.wrapping_add(16) as *const u128); crate::simd_ops::wu32(r10.wrapping_add(0xcu64), ((xmm11 as u32)) as u32); crate::simd_ops::wu32(r10, ((r11 as u32)) as u32); crate::simd_ops::wu32(r10.wrapping_add(0x4u64), ((rdi as u32)) as u32); crate::simd_ops::wu32(r10.wrapping_add(0x8u64), ((rsi as u32)) as u32); crate::simd_ops::wu128(r10.wrapping_add(0x10u64), (xmm12) as u128); crate::simd_ops::wu64(r10.wrapping_add(0x38u64), ((xmm8 as u64)) as u64); crate::simd_ops::wu32(r10.wrapping_add(0x40u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x88u64))) as u32 as u64; crate::simd_ops::wu64(r10.wrapping_add(0x20u64), ((xmm6 as u64)) as u64); crate::simd_ops::wu64(r10.wrapping_add(0x2cu64), ((xmm7 as u64)) as u64); crate::simd_ops::wu64(r10.wrapping_add(0x44u64), ((xmm9 as u64)) as u64); crate::simd_ops::wu32(r10.wrapping_add(0x4cu64), ((rax as u32)) as u32); crate::simd_ops::wu32(r10.wrapping_add(0x28u64), ((r12 as u32)) as u32); crate::simd_ops::wu32(r10.wrapping_add(0x34u64), ((r13 as u32)) as u32); let y_addr: u64 = r10.wrapping_add(0x58u64); core::ptr::write_unaligned(y_addr as *mut u128, xmm0); core::ptr::write_unaligned(y_addr.wrapping_add(16) as *mut u128, ymm0_hi); let y_addr: u64 = r10.wrapping_add(0x78u64); core::ptr::write_unaligned(y_addr as *mut u128, xmm1); core::ptr::write_unaligned(y_addr.wrapping_add(16) as *mut u128, ymm1_hi); crate::simd_ops::wu64(r10.wrapping_add(0x98u64), ((xmm10 as u64)) as u64); crate::simd_ops::wu32(r10.wrapping_add(0x50u64), ((r14 as u32)) as u32); crate::simd_ops::wu32(r10.wrapping_add(0x54u64), ((r15 as u32)) as u32); bb = 41; continue; }
            41 => { rsi = (crate::simd_ops::ru64(rbp.wrapping_add(0x80u64))) as u64; fl_l = ((r9).wrapping_add(0xa0u64)) as u64 as u64; fl_r = fl_l; r9 = ((fl_l as u64)) as u64; r13 = (crate::simd_ops::ru64(rbp.wrapping_add(0x78u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), (r9) as u64); bb = 14; continue; }
            42 => { rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x70u64))) as u64; crate::simd_ops::wu64(rax, (rbx) as u64); crate::simd_ops::wu64(rax.wrapping_add(0x8u64), (r8) as u64); r11 = ((rsp.wrapping_add(0x128u64)) as u64) as u64; xmm6 = (crate::simd_ops::ru128(r11.wrapping_sub(0x18u64))) as u128; xmm7 = (crate::simd_ops::ru128(r11.wrapping_sub(0x28u64))) as u128; xmm8 = (crate::simd_ops::ru128(r11.wrapping_sub(0x38u64))) as u128; xmm9 = (crate::simd_ops::ru128(r11.wrapping_sub(0x48u64))) as u128; xmm10 = (crate::simd_ops::ru128(r11.wrapping_sub(0x58u64))) as u128; xmm11 = (crate::simd_ops::ru128(r11.wrapping_sub(0x68u64))) as u128; xmm12 = (crate::simd_ops::ru128(r11.wrapping_sub(0x78u64))) as u128; rsp = (r11) as u64; r15 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r13 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r12 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rsi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rbx = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rbp = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1412efa00(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0; let mut ymm1_hi: u128 = 0; let mut xmm4: u128 = 0; let mut xmm5: u128 = 0; let mut xmm6: u128 = 0; let mut xmm7: u128 = 0; let mut xmm8: u128 = 0; let mut ymm8_hi: u128 = 0; let mut xmm9: u128 = 0; let mut ymm9_hi: u128 = 0; let mut xmm10: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 21>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { r11 = rsp; crate::simd_ops::wu8(r11.wrapping_add(0x20u64), ((r9 as u8)) as u8); crate::simd_ops::wu64(r11.wrapping_add(0x18u64), r8); crate::simd_ops::wu64(r11.wrapping_add(0x10u64), rdx); crate::simd_ops::wu64(r11.wrapping_add(0x8u64), rcx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = ((rsp).wrapping_sub(0xa0u64)) as u64; rax = (crate::simd_ops::ru64(r8)) as u64; rsi = rcx; r9 = (crate::simd_ops::ru64(rdx)) as u64; fl_l = r9; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 20; continue; } bb = 1; continue; }
            1 => { r9 = ((r9).wrapping_add(0xa0u64)) as u64; fl_l = r9; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 20; continue; } bb = 2; continue; }
            2 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0xb8u64))) as u64; crate::simd_ops::wu64(r11.wrapping_sub(0x10u64), rbx); crate::simd_ops::wu64(r11.wrapping_sub(0x18u64), rbp); crate::simd_ops::wu64(r11.wrapping_sub(0x20u64), rdi); crate::simd_ops::wu64(r11.wrapping_sub(0x28u64), r12); crate::simd_ops::wu64(r11.wrapping_sub(0x30u64), r13); crate::simd_ops::wu64(r11.wrapping_sub(0x38u64), r14); crate::simd_ops::wu64(r11.wrapping_sub(0x40u64), r15); crate::simd_ops::wu128(rsp.wrapping_add(0x50u64), (xmm6) as u128); crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm7) as u128); crate::simd_ops::wu128(r11.wrapping_sub(0x78u64), (xmm8) as u128); r11 = r9.wrapping_add(0x38); crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm9) as u128); crate::simd_ops::wu128(rsp.wrapping_add(0x10u64), (xmm10) as u128); bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru32(r11.wrapping_add(0x18u64))) as u32 as u64; rax = r9; rbx = (crate::simd_ops::ru32(r11.wrapping_add(0x1cu64))) as u32 as u64; r10 = (crate::simd_ops::ru32(r11.wrapping_add(0x14u64))) as u32 as u64; rbp = (crate::simd_ops::ru32(r9)) as u32 as u64; r14 = (crate::simd_ops::ru32(r11.wrapping_sub(0x34u64))) as u32 as u64; rdi = (crate::simd_ops::ru32(r11.wrapping_sub(0x30u64))) as u32 as u64; xmm2 = ((crate::simd_ops::ru32(r11.wrapping_sub(0x2cu64))) as u32) as u128; xmm3 = (crate::simd_ops::ru128(r11.wrapping_sub(0x28u64))) as u128; xmm4 = ((crate::simd_ops::ru64(r11.wrapping_sub(0x18u64))) as u64) as u128; r15 = (crate::simd_ops::ru32(r11.wrapping_sub(0x10u64))) as u32 as u64; xmm5 = ((crate::simd_ops::ru64(r11.wrapping_sub(0xcu64))) as u64) as u128; r12 = (crate::simd_ops::ru32(r11.wrapping_sub(0x4u64))) as u32 as u64; xmm6 = ((crate::simd_ops::ru64(r11)) as u64) as u128; r13 = (crate::simd_ops::ru32(r11.wrapping_add(0x8u64))) as u32 as u64; xmm7 = ((crate::simd_ops::ru64(r11.wrapping_add(0xcu64))) as u64) as u128; let y_addr: u64 = r11.wrapping_add(0x20u64); xmm8 = crate::simd_ops::ru128(y_addr); ymm8_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = r11.wrapping_add(0x40u64); xmm9 = crate::simd_ops::ru128(y_addr); ymm9_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); xmm10 = ((crate::simd_ops::ru64(r11.wrapping_add(0x60u64))) as u64) as u128; crate::simd_ops::wu32(rsp, ((rcx as u32)) as u32); rcx = (crate::simd_ops::ru64(rsi)) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0xc8u64), ((r10 as u32)) as u32); fl_l = ((rbx as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rcx.wrapping_add(0x54u64))) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 5; continue; } bb = 4; continue; }
            4 => { rdx = (rdx & !0xffu64) | (((if ((((fl_l as u32)) as i32) > (((fl_r as u32)) as i32)) { 1u8 } else { 0u8 }) as u8 as u64)); bb = 6; continue; }
            5 => { fl_l = ((rdi as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rcx.wrapping_add(0x8u64))) as u32 as u64; rdx = (rdx & !0xffu64) | (((if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { 1u8 } else { 0u8 }) as u8 as u64)); bb = 6; continue; }
            6 => { fl_l = ((rdx as u8)) as u8 as u64; fl_r = ((rdx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 11; continue; } bb = 7; continue; }
            7 => { fl_l = rcx; fl_r = r9; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 10; continue; } bb = 8; continue; }
            8 => { r8 = r11.wrapping_add(0x68); bb = 9; continue; }
            9 => { rax = ((rax).wrapping_add(0xffffffffffffff60u64)) as u64; r8 = r8.wrapping_sub(0xa0); rdx = (crate::simd_ops::ru32(rax)) as u32 as u64; crate::simd_ops::wu32(r8, ((rdx as u32)) as u32); rdx = (crate::simd_ops::ru32(rax.wrapping_add(0x4u64))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(0x4u64), ((rdx as u32)) as u32); rdx = (crate::simd_ops::ru32(rax.wrapping_add(0x8u64))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(0x8u64), ((rdx as u32)) as u32); rdx = (crate::simd_ops::ru32(rax.wrapping_add(0xcu64))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(0xcu64), ((rdx as u32)) as u32); xmm0 = (crate::simd_ops::ru128(rax.wrapping_add(0x10u64))) as u128; ymm0_hi = 0; crate::simd_ops::wu128(r8.wrapping_add(0x10u64), (xmm0) as u128); xmm0 = ((crate::simd_ops::ru64(rax.wrapping_add(0x20u64))) as u64) as u128; ymm0_hi = 0; crate::simd_ops::wu64(r8.wrapping_add(0x20u64), ((xmm0 as u64)) as u64); rdx = (crate::simd_ops::ru32(rax.wrapping_add(0x28u64))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(0x28u64), ((rdx as u32)) as u32); xmm0 = ((crate::simd_ops::ru64(rax.wrapping_add(0x2cu64))) as u64) as u128; ymm0_hi = 0; crate::simd_ops::wu64(r8.wrapping_add(0x2cu64), ((xmm0 as u64)) as u64); rdx = (crate::simd_ops::ru32(rax.wrapping_add(0x34u64))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(0x34u64), ((rdx as u32)) as u32); xmm0 = ((crate::simd_ops::ru64(rax.wrapping_add(0x38u64))) as u64) as u128; ymm0_hi = 0; crate::simd_ops::wu64(r8.wrapping_add(0x38u64), ((xmm0 as u64)) as u64); rdx = (crate::simd_ops::ru32(rax.wrapping_add(0x40u64))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(0x40u64), ((rdx as u32)) as u32); xmm0 = ((crate::simd_ops::ru64(rax.wrapping_add(0x44u64))) as u64) as u128; ymm0_hi = 0; crate::simd_ops::wu64(r8.wrapping_add(0x44u64), ((xmm0 as u64)) as u64); rdx = (crate::simd_ops::ru32(rax.wrapping_add(0x4cu64))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(0x4cu64), ((rdx as u32)) as u32); rdx = (crate::simd_ops::ru32(rax.wrapping_add(0x50u64))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(0x50u64), ((rdx as u32)) as u32); rdx = (crate::simd_ops::ru32(rax.wrapping_add(0x54u64))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(0x54u64), ((rdx as u32)) as u32); let y_addr: u64 = rax.wrapping_add(0x58u64); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = r8.wrapping_add(0x58u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); xmm1 = ((crate::simd_ops::ru64(rax.wrapping_add(0x78u64))) as u64) as u128; ymm1_hi = 0; crate::simd_ops::wu64(r8.wrapping_add(0x78u64), ((xmm1 as u64)) as u64); xmm0 = (crate::simd_ops::ru128(rax.wrapping_add(0x58u64))) as u128; ymm0_hi = 0; crate::simd_ops::wu128(r8.wrapping_add(0x58u64), (xmm0) as u128); let y_addr: u64 = rax.wrapping_add(0x58u64); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = r8.wrapping_add(0x58u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); let y_addr: u64 = rax.wrapping_add(0x58u64); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = r8.wrapping_add(0x58u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); let y_addr: u64 = rax.wrapping_add(0x78u64); xmm1 = crate::simd_ops::ru128(y_addr); ymm1_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = r8.wrapping_add(0x78u64); crate::simd_ops::wu128(y_addr, xmm1); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm1_hi); xmm0 = ((crate::simd_ops::ru64(rax.wrapping_add(0x98u64))) as u64) as u128; ymm0_hi = 0; crate::simd_ops::wu64(r8.wrapping_add(0x98u64), ((xmm0 as u64)) as u64); fl_l = rcx; fl_r = rax; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 9; continue; } bb = 10; continue; }
            10 => { rax = (crate::simd_ops::ru64(rsi)) as u64; crate::simd_ops::wu32(rax.wrapping_add(0xcu64), ((xmm2 as u32)) as u32); crate::simd_ops::wu32(rax, ((rbp as u32)) as u32); crate::simd_ops::wu32(rax.wrapping_add(0x4u64), ((r14 as u32)) as u32); crate::simd_ops::wu32(rax.wrapping_add(0x8u64), ((rdi as u32)) as u32); crate::simd_ops::wu128(rax.wrapping_add(0x10u64), (xmm3) as u128); crate::simd_ops::wu64(rax.wrapping_add(0x44u64), ((xmm7 as u64)) as u64); crate::simd_ops::wu32(rax.wrapping_add(0x4cu64), ((r10 as u32)) as u32); bb = 18; continue; }
            11 => { r8 = r11; bb = 12; continue; }
            12 => { r8 = r8.wrapping_sub(0xa0); fl_l = ((rbx as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(r8.wrapping_add(0x1cu64))) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 14; continue; } bb = 13; continue; }
            13 => { rcx = (rcx & !0xffu64) | (((if ((((fl_l as u32)) as i32) > (((fl_r as u32)) as i32)) { 1u8 } else { 0u8 }) as u8 as u64)); bb = 15; continue; }
            14 => { fl_l = ((rdi as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(r8.wrapping_sub(0x30u64))) as u32 as u64; rcx = (rcx & !0xffu64) | (((if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { 1u8 } else { 0u8 }) as u8 as u64)); bb = 15; continue; }
            15 => { fl_l = ((rcx as u8)) as u8 as u64; fl_r = ((rcx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 17; continue; } bb = 16; continue; }
            16 => { rcx = (crate::simd_ops::ru32(r8.wrapping_sub(0x38u64))) as u32 as u64; crate::simd_ops::wu32(rax, ((rcx as u32)) as u32); rcx = (crate::simd_ops::ru32(r8.wrapping_sub(0x34u64))) as u32 as u64; crate::simd_ops::wu32(rax.wrapping_add(0x4u64), ((rcx as u32)) as u32); rcx = (crate::simd_ops::ru32(r8.wrapping_sub(0x30u64))) as u32 as u64; crate::simd_ops::wu32(rax.wrapping_add(0x8u64), ((rcx as u32)) as u32); rcx = (crate::simd_ops::ru32(r8.wrapping_sub(0x2cu64))) as u32 as u64; crate::simd_ops::wu32(rax.wrapping_add(0xcu64), ((rcx as u32)) as u32); xmm0 = (crate::simd_ops::ru128(r8.wrapping_sub(0x28u64))) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rax.wrapping_add(0x10u64), (xmm0) as u128); xmm0 = ((crate::simd_ops::ru64(r8.wrapping_sub(0x18u64))) as u64) as u128; ymm0_hi = 0; crate::simd_ops::wu64(rax.wrapping_add(0x20u64), ((xmm0 as u64)) as u64); rcx = (crate::simd_ops::ru32(r8.wrapping_sub(0x10u64))) as u32 as u64; crate::simd_ops::wu32(rax.wrapping_add(0x28u64), ((rcx as u32)) as u32); xmm0 = ((crate::simd_ops::ru64(r8.wrapping_sub(0xcu64))) as u64) as u128; ymm0_hi = 0; crate::simd_ops::wu64(rax.wrapping_add(0x2cu64), ((xmm0 as u64)) as u64); rcx = (crate::simd_ops::ru32(r8.wrapping_sub(0x4u64))) as u32 as u64; crate::simd_ops::wu32(rax.wrapping_add(0x34u64), ((rcx as u32)) as u32); xmm0 = ((crate::simd_ops::ru64(r8)) as u64) as u128; ymm0_hi = 0; crate::simd_ops::wu64(rax.wrapping_add(0x38u64), ((xmm0 as u64)) as u64); rcx = (crate::simd_ops::ru32(r8.wrapping_add(0x8u64))) as u32 as u64; crate::simd_ops::wu32(rax.wrapping_add(0x40u64), ((rcx as u32)) as u32); xmm0 = ((crate::simd_ops::ru64(r8.wrapping_add(0xcu64))) as u64) as u128; ymm0_hi = 0; crate::simd_ops::wu64(rax.wrapping_add(0x44u64), ((xmm0 as u64)) as u64); rcx = (crate::simd_ops::ru32(r8.wrapping_add(0x14u64))) as u32 as u64; crate::simd_ops::wu32(rax.wrapping_add(0x4cu64), ((rcx as u32)) as u32); rcx = (crate::simd_ops::ru32(r8.wrapping_add(0x18u64))) as u32 as u64; crate::simd_ops::wu32(rax.wrapping_add(0x50u64), ((rcx as u32)) as u32); rcx = (crate::simd_ops::ru32(r8.wrapping_add(0x1cu64))) as u32 as u64; crate::simd_ops::wu32(rax.wrapping_add(0x54u64), ((rcx as u32)) as u32); let y_addr: u64 = r8.wrapping_add(0x20u64); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rax.wrapping_add(0x58u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); xmm1 = ((crate::simd_ops::ru64(r8.wrapping_add(0x40u64))) as u64) as u128; ymm1_hi = 0; crate::simd_ops::wu64(rax.wrapping_add(0x78u64), ((xmm1 as u64)) as u64); xmm0 = (crate::simd_ops::ru128(r8.wrapping_add(0x20u64))) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rax.wrapping_add(0x58u64), (xmm0) as u128); let y_addr: u64 = r8.wrapping_add(0x20u64); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rax.wrapping_add(0x58u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); let y_addr: u64 = r8.wrapping_add(0x20u64); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rax.wrapping_add(0x58u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); let y_addr: u64 = r8.wrapping_add(0x40u64); xmm1 = crate::simd_ops::ru128(y_addr); ymm1_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rax.wrapping_add(0x78u64); crate::simd_ops::wu128(y_addr, xmm1); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm1_hi); xmm0 = ((crate::simd_ops::ru64(r8.wrapping_add(0x60u64))) as u64) as u128; ymm0_hi = 0; crate::simd_ops::wu64(rax.wrapping_add(0x98u64), ((xmm0 as u64)) as u64); rax = r8.wrapping_sub(0x38); bb = 12; continue; }
            17 => { rcx = (crate::simd_ops::ru32(rsp.wrapping_add(0xc8u64))) as u32 as u64; crate::simd_ops::wu32(rax.wrapping_add(0xcu64), ((xmm2 as u32)) as u32); crate::simd_ops::wu32(rax, ((rbp as u32)) as u32); crate::simd_ops::wu32(rax.wrapping_add(0x4u64), ((r14 as u32)) as u32); crate::simd_ops::wu32(rax.wrapping_add(0x8u64), ((rdi as u32)) as u32); crate::simd_ops::wu128(rax.wrapping_add(0x10u64), (xmm3) as u128); crate::simd_ops::wu64(rax.wrapping_add(0x44u64), ((xmm7 as u64)) as u64); crate::simd_ops::wu32(rax.wrapping_add(0x4cu64), ((rcx as u32)) as u32); bb = 18; continue; }
            18 => { rcx = (crate::simd_ops::ru32(rsp)) as u32 as u64; r9 = ((r9).wrapping_add(0xa0u64)) as u64; crate::simd_ops::wu64(rax.wrapping_add(0x20u64), ((xmm4 as u64)) as u64); crate::simd_ops::wu64(rax.wrapping_add(0x2cu64), ((xmm5 as u64)) as u64); crate::simd_ops::wu64(rax.wrapping_add(0x38u64), ((xmm6 as u64)) as u64); crate::simd_ops::wu32(rax.wrapping_add(0x28u64), ((r15 as u32)) as u32); r11 = ((r11).wrapping_add(0xa0u64)) as u64; crate::simd_ops::wu32(rax.wrapping_add(0x34u64), ((r12 as u32)) as u32); crate::simd_ops::wu32(rax.wrapping_add(0x40u64), ((r13 as u32)) as u32); let y_addr: u64 = rax.wrapping_add(0x58u64); crate::simd_ops::wu128(y_addr, xmm8); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm8_hi); let y_addr: u64 = rax.wrapping_add(0x78u64); crate::simd_ops::wu128(y_addr, xmm9); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm9_hi); crate::simd_ops::wu64(rax.wrapping_add(0x98u64), ((xmm10 as u64)) as u64); crate::simd_ops::wu32(rax.wrapping_add(0x54u64), ((rbx as u32)) as u32); crate::simd_ops::wu32(rax.wrapping_add(0x50u64), ((rcx as u32)) as u32); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xc0u64))) as u64; rax = (crate::simd_ops::ru64(rax)) as u64; fl_l = r9; fl_r = rax; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } bb = 19; continue; }
            19 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0xb0u64))) as u64; xmm10 = (crate::simd_ops::ru128(rsp.wrapping_add(0x10u64))) as u128; xmm9 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; ymm9_hi = 0; xmm8 = (crate::simd_ops::ru128(rsp.wrapping_add(0x30u64))) as u128; ymm8_hi = 0; xmm7 = (crate::simd_ops::ru128(rsp.wrapping_add(0x40u64))) as u128; xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x50u64))) as u128; r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; r13 = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x88u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x90u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x98u64))) as u64; crate::simd_ops::wu64(rsi, rax); rax = rsi; rsp = ((rsp).wrapping_add(0xa0u64)) as u64; rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            20 => { crate::simd_ops::wu64(rcx, rax); rax = rsi; rsp = ((rsp).wrapping_add(0xa0u64)) as u64; rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1412efe20(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1412f18e0; const P1: u64 = 0x1412f0cc0;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0; let mut ymm1_hi: u128 = 0; let mut xmm6: u128 = 0; let mut xmm7: u128 = 0; let mut xmm8: u128 = 0; let mut xmm9: u128 = 0; let mut xmm10: u128 = 0; let mut xmm11: u128 = 0; let mut xmm12: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 45>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = (rsp) as u64; crate::simd_ops::wu8(rax.wrapping_add(0x20u64), ((r9 as u8)) as u8); crate::simd_ops::wu64(rax.wrapping_add(0x18u64), (r8) as u64); crate::simd_ops::wu64(rax.wrapping_add(0x10u64), (rdx) as u64); crate::simd_ops::wu64(rax.wrapping_add(0x8u64), (rcx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r12) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r13) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r15) as u64); rbp = ((rax.wrapping_sub(0x68u64)) as u64) as u64; rsp = ((rsp).wrapping_sub(0x128u64)) as u64; r11 = (crate::simd_ops::ru64(rdx)) as u64; r13 = (rdx) as u64; r9 = (crate::simd_ops::ru64(r8)) as u64; rdi = (r8) as u64; crate::simd_ops::wu128(rax.wrapping_sub(0x58u64), (xmm6) as u128); rsi = (0x6666666666666667u64) as u64; crate::simd_ops::wu128(rax.wrapping_sub(0x68u64), (xmm7) as u128); r10 = (r9) as u64; crate::simd_ops::wu128(rax.wrapping_sub(0x78u64), (xmm8) as u128); r10 = ((r10).wrapping_sub(r11)) as u64; crate::simd_ops::wu128(rax.wrapping_sub(0x88u64), (xmm9) as u128); r12 = ((r9.wrapping_sub(0xa0u64)) as u64) as u64; crate::simd_ops::wu128(rax.wrapping_sub(0x98u64), (xmm10) as u128); rcx = (r12) as u64; crate::simd_ops::wu128(rax.wrapping_sub(0xa8u64), (xmm11) as u128); rcx = ((rcx).wrapping_sub(r11)) as u64; crate::simd_ops::wu128(rax.wrapping_sub(0xb8u64), (xmm12) as u128); rax = (rsi) as u64; let prod = (rax as i64 as i128).wrapping_mul((r10) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; rax = (rsi) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (r11) as u64); rdx = ((((rdx) as i64).wrapping_shr(6u32) as u64)) as u64; r8 = (rdx) as u64; r8 = ((r8).wrapping_shr(63u32)) as u64; rdx = ((rdx).wrapping_add(r8)) as u64; r8 = ((rsp.wrapping_add(0x40u64)) as u64) as u64; rdx = ((((rdx) as i64).wrapping_shr(1u32) as u64)) as u64; rbx = ((rdx.wrapping_add(rdx.wrapping_mul(4))) as u64) as u64; let prod = (rax as i64 as i128).wrapping_mul((rcx) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; rbx = ((rbx).wrapping_shl(5u32)) as u64; rdx = ((((rdx) as i64).wrapping_shr(6u32) as u64)) as u64; rbx = ((rbx).wrapping_add(r11)) as u64; rax = (rdx) as u64; rax = ((rax).wrapping_shr(63u32)) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; fl_l = (rdx) as u64 as u64; fl_r = (0x28u64) as u64 as u64; if ((((fl_l as u64)) as i64) <= (((fl_r as u64)) as i64)) { bb = 2; continue; } bb = 1; continue; }
            1 => { rcx = ((rdx.wrapping_add(0x1u64)) as u64) as u64; rcx = ((((rcx) as i64).wrapping_shr(3u32) as u64)) as u64; rdx = ((rsp.wrapping_add(0x38u64)) as u64) as u64; r15 = ((rcx.wrapping_add(rcx.wrapping_mul(4))) as u64) as u64; rsi = ((rcx.wrapping_add(rcx.wrapping_mul(4))) as u64) as u64; r15 = ((r15).wrapping_shl(6u32)) as u64; rsi = ((rsi).wrapping_shl(5u32)) as u64; rcx = ((rsp.wrapping_add(0x30u64)) as u64) as u64; rax = ((r11.wrapping_add(r15)) as u64) as u64; r14 = ((rsi.wrapping_add(r11)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), (rax) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), (r14) as u64); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = ((rsi.wrapping_add(rbx)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), (rax) as u64); r8 = ((rsp.wrapping_add(0x40u64)) as u64) as u64; rax = (rbx) as u64; rdx = ((rsp.wrapping_add(0x38u64)) as u64) as u64; rax = ((rax).wrapping_sub(rsi)) as u64; rcx = ((rsp.wrapping_add(0x30u64)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (rax) as u64); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdi = (r12) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), (r12) as u64); rdi = ((rdi).wrapping_sub(rsi)) as u64; r8 = ((rsp.wrapping_add(0x40u64)) as u64) as u64; r12 = ((r12).wrapping_sub(r15)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), (rdi) as u64); rdx = ((rsp.wrapping_add(0x38u64)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (r12) as u64); rcx = ((rsp.wrapping_add(0x30u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; r8 = ((rsp.wrapping_add(0x40u64)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), (rdi) as u64); rdx = ((rsp.wrapping_add(0x38u64)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), (rbx) as u64); rcx = ((rsp.wrapping_add(0x30u64)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (r14) as u64); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdi = (crate::simd_ops::ru64(rbp.wrapping_add(0x80u64))) as u64; bb = 3; continue; }
            2 => { rdx = ((rsp.wrapping_add(0x38u64)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), (r12) as u64); rcx = ((rsp.wrapping_add(0x30u64)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), (rbx) as u64); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 3; continue; }
            3 => { r11 = (crate::simd_ops::ru64(r13)) as u64; r8 = ((rbx.wrapping_add(0xa0u64)) as u64) as u64; fl_l = (r11) as u64 as u64; fl_r = (rbx) as u64 as u64; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 14; continue; } bb = 4; continue; }
            4 => { rcx = ((rbx.wrapping_sub(0x98u64)) as u64) as u64; bb = 5; continue; }
            5 => { rdx = (crate::simd_ops::ru32(rcx.wrapping_add(0x4cu64))) as u32 as u64; r9 = (crate::simd_ops::ru32(rcx.wrapping_add(0xecu64))) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((r9 as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 7; continue; } bb = 6; continue; }
            6 => { rax = (rax & !0xffu64) | (((if ((((fl_l as u32)) as i32) > (((fl_r as u32)) as i32)) { 1u8 } else { 0u8 }) as u8 as u64)); bb = 8; continue; }
            7 => { rax = (crate::simd_ops::ru32(rcx.wrapping_add(0xa0u64))) as u32 as u64; fl_l = (crate::simd_ops::ru32(rcx)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; rax = (rax & !0xffu64) | (((if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { 1u8 } else { 0u8 }) as u8 as u64)); bb = 8; continue; }
            8 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 14; continue; } bb = 9; continue; }
            9 => { fl_l = ((r9 as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 11; continue; } bb = 10; continue; }
            10 => { rax = (rax & !0xffu64) | (((if ((((fl_l as u32)) as i32) > (((fl_r as u32)) as i32)) { 1u8 } else { 0u8 }) as u8 as u64)); bb = 12; continue; }
            11 => { rax = (crate::simd_ops::ru32(rcx)) as u32 as u64; fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(0xa0u64))) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; rax = (rax & !0xffu64) | (((if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { 1u8 } else { 0u8 }) as u8 as u64)); bb = 12; continue; }
            12 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 14; continue; } bb = 13; continue; }
            13 => { rbx = ((rbx).wrapping_sub(0xa0u64)) as u64; rcx = ((rcx).wrapping_sub(0xa0u64)) as u64; fl_l = (r11) as u64 as u64; fl_r = (rbx) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 5; continue; } bb = 14; continue; }
            14 => { r10 = (crate::simd_ops::ru64(rdi)) as u64; fl_l = (r8) as u64 as u64; fl_r = (r10) as u64 as u64; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 25; continue; } bb = 15; continue; }
            15 => { r9 = (crate::simd_ops::ru32(rbx.wrapping_add(0x54u64))) as u32 as u64; bb = 16; continue; }
            16 => { rcx = (crate::simd_ops::ru32(r8.wrapping_add(0x54u64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((r9 as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 18; continue; } bb = 17; continue; }
            17 => { rax = (rax & !0xffu64) | (((if ((((fl_l as u32)) as i32) > (((fl_r as u32)) as i32)) { 1u8 } else { 0u8 }) as u8 as u64)); bb = 19; continue; }
            18 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x8u64))) as u32 as u64; fl_l = (crate::simd_ops::ru32(r8.wrapping_add(0x8u64))) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; rax = (rax & !0xffu64) | (((if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { 1u8 } else { 0u8 }) as u8 as u64)); bb = 19; continue; }
            19 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 25; continue; } bb = 20; continue; }
            20 => { fl_l = ((r9 as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 22; continue; } bb = 21; continue; }
            21 => { rax = (rax & !0xffu64) | (((if ((((fl_l as u32)) as i32) > (((fl_r as u32)) as i32)) { 1u8 } else { 0u8 }) as u8 as u64)); bb = 23; continue; }
            22 => { rax = (crate::simd_ops::ru32(r8.wrapping_add(0x8u64))) as u32 as u64; fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x8u64))) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; rax = (rax & !0xffu64) | (((if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { 1u8 } else { 0u8 }) as u8 as u64)); bb = 23; continue; }
            23 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 25; continue; } bb = 24; continue; }
            24 => { r8 = ((r8).wrapping_add(0xa0u64)) as u64; fl_l = (r8) as u64 as u64; fl_r = (r10) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 16; continue; } bb = 25; continue; }
            25 => { r9 = (r8) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), (r8) as u64); r10 = (rbx) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), (rbx) as u64); bb = 26; continue; }
            26 => { fl_l = (r9) as u64 as u64; fl_r = (crate::simd_ops::ru64(rdi)) as u64 as u64; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 41; continue; } bb = 27; continue; }
            27 => { rsi = ((r8.wrapping_add(0x58u64)) as u64) as u64; r12 = ((r8.wrapping_add(0x10u64)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), (rsi) as u64); rcx = ((r9.wrapping_add(0x38u64)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), (r12) as u64); r13 = ((r8.wrapping_add(0x44u64)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), (rcx) as u64); r15 = ((r8.wrapping_add(0x38u64)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), (r13) as u64); r14 = ((r8.wrapping_add(0x2cu64)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (r15) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), (r14) as u64); r10 = ((r8.wrapping_add(0x20u64)) as u64) as u64; bb = 28; continue; }
            28 => { rdx = (crate::simd_ops::ru32(rbx.wrapping_add(0x54u64))) as u32 as u64; rdi = ((rcx.wrapping_sub(0x30u64)) as u64) as u64; r11 = (crate::simd_ops::ru32(rcx.wrapping_add(0x1cu64))) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), (rdi) as u64); fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((r11 as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 30; continue; } bb = 29; continue; }
            29 => { rax = (rax & !0xffu64) | (((if ((((fl_l as u32)) as i32) > (((fl_r as u32)) as i32)) { 1u8 } else { 0u8 }) as u8 as u64)); bb = 31; continue; }
            30 => { rax = (crate::simd_ops::ru32(rdi)) as u32 as u64; fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x8u64))) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), (rdi) as u64); rax = (rax & !0xffu64) | (((if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { 1u8 } else { 0u8 }) as u8 as u64)); bb = 31; continue; }
            31 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 39; continue; } bb = 32; continue; }
            32 => { fl_l = ((r11 as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 34; continue; } bb = 33; continue; }
            33 => { rax = (rax & !0xffu64) | (((if ((((fl_l as u32)) as i32) > (((fl_r as u32)) as i32)) { 1u8 } else { 0u8 }) as u8 as u64)); bb = 35; continue; }
            34 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x8u64))) as u32 as u64; fl_l = (crate::simd_ops::ru32(rdi)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; rax = (rax & !0xffu64) | (((if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { 1u8 } else { 0u8 }) as u8 as u64)); bb = 35; continue; }
            35 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 40; continue; } bb = 36; continue; }
            36 => { fl_l = (r8) as u64 as u64; fl_r = (r9) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 38; continue; } bb = 37; continue; }
            37 => { rax = (crate::simd_ops::ru32(r15.wrapping_add(0x8u64))) as u32 as u64; rdx = (r9) as u64; xmm12 = (crate::simd_ops::ru128(r12)) as u128; xmm7 = ((crate::simd_ops::ru64(r14)) as u64) as u128; xmm8 = ((crate::simd_ops::ru64(r15)) as u64) as u128; xmm11 = ((crate::simd_ops::ru32(r8.wrapping_add(0xcu64))) as u32) as u128; xmm6 = ((crate::simd_ops::ru64(r10)) as u64) as u128; r13 = (crate::simd_ops::ru32(r14.wrapping_add(0x8u64))) as u32 as u64; rcx = (r8) as u64; r11 = (crate::simd_ops::ru32(r8)) as u32 as u64; rdi = (crate::simd_ops::ru32(r8.wrapping_add(0x4u64))) as u32 as u64; rsi = (crate::simd_ops::ru32(r8.wrapping_add(0x8u64))) as u32 as u64; r12 = (crate::simd_ops::ru32(r10.wrapping_add(0x8u64))) as u32 as u64; r14 = (crate::simd_ops::ru32(r8.wrapping_add(0x50u64))) as u32 as u64; r15 = (crate::simd_ops::ru32(r8.wrapping_add(0x54u64))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; xmm9 = ((crate::simd_ops::ru64(rax)) as u64) as u128; rax = (crate::simd_ops::ru32(rax.wrapping_add(0x8u64))) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x88u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; let y_addr: u64 = rax; xmm0 = core::ptr::read_unaligned(y_addr as *const u128); ymm0_hi = core::ptr::read_unaligned(y_addr.wrapping_add(16) as *const u128); xmm10 = ((crate::simd_ops::ru64(rax.wrapping_add(0x40u64))) as u64) as u128; let y_addr: u64 = rsp.wrapping_add(0x70u64); core::ptr::write_unaligned(y_addr as *mut u128, xmm0); core::ptr::write_unaligned(y_addr.wrapping_add(16) as *mut u128, ymm0_hi); let y_addr: u64 = rax.wrapping_add(0x20u64); xmm0 = core::ptr::read_unaligned(y_addr as *const u128); ymm0_hi = core::ptr::read_unaligned(y_addr.wrapping_add(16) as *const u128); let y_addr: u64 = rbp.wrapping_sub(0x70u64); core::ptr::write_unaligned(y_addr as *mut u128, xmm0); core::ptr::write_unaligned(y_addr.wrapping_add(16) as *mut u128, ymm0_hi); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; let y_addr: u64 = rsp.wrapping_add(0x70u64); xmm0 = core::ptr::read_unaligned(y_addr as *const u128); ymm0_hi = core::ptr::read_unaligned(y_addr.wrapping_add(16) as *const u128); let y_addr: u64 = rbp.wrapping_sub(0x70u64); xmm1 = core::ptr::read_unaligned(y_addr as *const u128); ymm1_hi = core::ptr::read_unaligned(y_addr.wrapping_add(16) as *const u128); crate::simd_ops::wu32(rcx.wrapping_sub(0x2cu64), ((xmm11 as u32)) as u32); crate::simd_ops::wu32(rax, ((rsi as u32)) as u32); rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x20u64))) as u32 as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; crate::simd_ops::wu32(r9, ((r11 as u32)) as u32); crate::simd_ops::wu32(rcx.wrapping_sub(0x34u64), ((rdi as u32)) as u32); crate::simd_ops::wu128(rcx.wrapping_sub(0x28u64), (xmm12) as u128); crate::simd_ops::wu64(rcx, ((xmm8 as u64)) as u64); crate::simd_ops::wu32(rcx.wrapping_add(0x8u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x88u64))) as u32 as u64; crate::simd_ops::wu64(rcx.wrapping_sub(0x18u64), ((xmm6 as u64)) as u64); crate::simd_ops::wu64(rcx.wrapping_sub(0xcu64), ((xmm7 as u64)) as u64); crate::simd_ops::wu64(rcx.wrapping_add(0xcu64), ((xmm9 as u64)) as u64); crate::simd_ops::wu32(rcx.wrapping_sub(0x10u64), ((r12 as u32)) as u32); r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; crate::simd_ops::wu32(rcx.wrapping_sub(0x4u64), ((r13 as u32)) as u32); r13 = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; crate::simd_ops::wu32(rcx.wrapping_add(0x14u64), ((rax as u32)) as u32); let y_addr: u64 = rcx.wrapping_add(0x20u64); core::ptr::write_unaligned(y_addr as *mut u128, xmm0); core::ptr::write_unaligned(y_addr.wrapping_add(16) as *mut u128, ymm0_hi); let y_addr: u64 = rcx.wrapping_add(0x40u64); core::ptr::write_unaligned(y_addr as *mut u128, xmm1); core::ptr::write_unaligned(y_addr.wrapping_add(16) as *mut u128, ymm1_hi); crate::simd_ops::wu32(rcx.wrapping_add(0x18u64), ((r14 as u32)) as u32); r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; crate::simd_ops::wu32(rcx.wrapping_add(0x1cu64), ((r15 as u32)) as u32); r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu64(rcx.wrapping_add(0x60u64), ((xmm10 as u64)) as u64); bb = 38; continue; }
            38 => { r12 = ((r12).wrapping_add(0xa0u64)) as u64; r14 = ((r14).wrapping_add(0xa0u64)) as u64; r15 = ((r15).wrapping_add(0xa0u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), (r14) as u64); r13 = ((r13).wrapping_add(0xa0u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (r15) as u64); r8 = ((r8).wrapping_add(0xa0u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), (r13) as u64); r10 = ((r10).wrapping_add(0xa0u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), (r12) as u64); fl_l = ((rsi).wrapping_add(0xa0u64)) as u64 as u64; fl_r = fl_l; rsi = ((fl_l as u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), (rsi) as u64); bb = 39; continue; }
            39 => { rdi = (crate::simd_ops::ru64(rbp.wrapping_add(0x80u64))) as u64; rcx = ((rcx).wrapping_add(0xa0u64)) as u64; r9 = ((r9).wrapping_add(0xa0u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), (rcx) as u64); fl_l = (r9) as u64 as u64; fl_r = (crate::simd_ops::ru64(rdi)) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 28; continue; } bb = 40; continue; }
            40 => { r10 = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; r13 = (crate::simd_ops::ru64(rbp.wrapping_add(0x78u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), (r9) as u64); bb = 41; continue; }
            41 => { rax = (crate::simd_ops::ru64(r13)) as u64; fl_l = (rax) as u64 as u64; fl_r = (r10) as u64 as u64; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 55; continue; } bb = 42; continue; }
            42 => { rdx = ((r10.wrapping_sub(0x68u64)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), (rdx) as u64); r9 = ((rbx.wrapping_add(0x38u64)) as u64) as u64; bb = 43; continue; }
            43 => { r11 = (crate::simd_ops::ru32(rdx.wrapping_add(0x1cu64))) as u32 as u64; rsi = ((rdx.wrapping_sub(0x30u64)) as u64) as u64; rdi = (crate::simd_ops::ru32(r9.wrapping_add(0x1cu64))) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), (rsi) as u64); fl_l = ((r11 as u32)) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 45; continue; } bb = 44; continue; }
            44 => { rcx = (rcx & !0xffu64) | (((if ((((fl_l as u32)) as i32) > (((fl_r as u32)) as i32)) { 1u8 } else { 0u8 }) as u8 as u64)); bb = 46; continue; }
            45 => { rcx = (crate::simd_ops::ru32(r9.wrapping_sub(0x30u64))) as u32 as u64; fl_l = (crate::simd_ops::ru32(rsi)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), (rsi) as u64); rcx = (rcx & !0xffu64) | (((if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { 1u8 } else { 0u8 }) as u8 as u64)); bb = 46; continue; }
            46 => { fl_l = ((rcx as u8)) as u8 as u64; fl_r = ((rcx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 53; continue; } bb = 47; continue; }
            47 => { fl_l = ((rdi as u32)) as u32 as u64; fl_r = ((r11 as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 49; continue; } bb = 48; continue; }
            48 => { rcx = (rcx & !0xffu64) | (((if ((((fl_l as u32)) as i32) > (((fl_r as u32)) as i32)) { 1u8 } else { 0u8 }) as u8 as u64)); bb = 50; continue; }
            49 => { rcx = (crate::simd_ops::ru32(rsi)) as u32 as u64; fl_l = (crate::simd_ops::ru32(r9.wrapping_sub(0x30u64))) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; rcx = (rcx & !0xffu64) | (((if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { 1u8 } else { 0u8 }) as u8 as u64)); bb = 50; continue; }
            50 => { fl_l = ((rcx as u8)) as u8 as u64; fl_r = ((rcx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 54; continue; } bb = 51; continue; }
            51 => { rbx = ((rbx).wrapping_sub(0xa0u64)) as u64; rcx = ((rdx.wrapping_sub(0x38u64)) as u64) as u64; r9 = ((r9).wrapping_sub(0xa0u64)) as u64; fl_l = (rbx) as u64 as u64; fl_r = (rcx) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 53; continue; } bb = 52; continue; }
            52 => { let y_addr: u64 = r9.wrapping_add(0x20u64); xmm0 = core::ptr::read_unaligned(y_addr as *const u128); ymm0_hi = core::ptr::read_unaligned(y_addr.wrapping_add(16) as *const u128); rax = (crate::simd_ops::ru32(r9.wrapping_add(0x8u64))) as u32 as u64; rdx = (rcx) as u64; xmm11 = ((crate::simd_ops::ru32(r9.wrapping_sub(0x2cu64))) as u32) as u128; xmm12 = (crate::simd_ops::ru128(r9.wrapping_sub(0x28u64))) as u128; xmm6 = ((crate::simd_ops::ru64(r9.wrapping_sub(0x18u64))) as u64) as u128; xmm7 = ((crate::simd_ops::ru64(r9.wrapping_sub(0xcu64))) as u64) as u128; xmm8 = ((crate::simd_ops::ru64(r9)) as u64) as u128; xmm9 = ((crate::simd_ops::ru64(r9.wrapping_add(0xcu64))) as u64) as u128; xmm10 = ((crate::simd_ops::ru64(r9.wrapping_add(0x60u64))) as u64) as u128; r11 = (crate::simd_ops::ru32(rbx)) as u32 as u64; rcx = (rbx) as u64; rdi = (crate::simd_ops::ru32(r9.wrapping_sub(0x34u64))) as u32 as u64; rsi = (crate::simd_ops::ru32(r9.wrapping_sub(0x30u64))) as u32 as u64; r12 = (crate::simd_ops::ru32(r9.wrapping_sub(0x10u64))) as u32 as u64; r13 = (crate::simd_ops::ru32(r9.wrapping_sub(0x4u64))) as u32 as u64; r14 = (crate::simd_ops::ru32(r9.wrapping_add(0x18u64))) as u32 as u64; r15 = (crate::simd_ops::ru32(r9.wrapping_add(0x1cu64))) as u32 as u64; let y_addr: u64 = rbp.wrapping_sub(0x70u64); core::ptr::write_unaligned(y_addr as *mut u128, xmm0); core::ptr::write_unaligned(y_addr.wrapping_add(16) as *mut u128, ymm0_hi); let y_addr: u64 = r9.wrapping_add(0x40u64); xmm0 = core::ptr::read_unaligned(y_addr as *const u128); ymm0_hi = core::ptr::read_unaligned(y_addr.wrapping_add(16) as *const u128); crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(r9.wrapping_add(0x14u64))) as u32 as u64; let y_addr: u64 = rsp.wrapping_add(0x70u64); core::ptr::write_unaligned(y_addr as *mut u128, xmm0); core::ptr::write_unaligned(y_addr.wrapping_add(16) as *mut u128, ymm0_hi); crate::simd_ops::wu32(rbp.wrapping_add(0x88u64), ((rax as u32)) as u32); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rcx = (rdx) as u64; let y_addr: u64 = rbp.wrapping_sub(0x70u64); xmm0 = core::ptr::read_unaligned(y_addr as *const u128); ymm0_hi = core::ptr::read_unaligned(y_addr.wrapping_add(16) as *const u128); let y_addr: u64 = rsp.wrapping_add(0x70u64); xmm1 = core::ptr::read_unaligned(y_addr as *const u128); ymm1_hi = core::ptr::read_unaligned(y_addr.wrapping_add(16) as *const u128); crate::simd_ops::wu32(rdx, ((r11 as u32)) as u32); rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; crate::simd_ops::wu32(rax, ((rsi as u32)) as u32); rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x20u64))) as u32 as u64; crate::simd_ops::wu32(rdx.wrapping_sub(0x2cu64), ((xmm11 as u32)) as u32); crate::simd_ops::wu32(rdx.wrapping_sub(0x34u64), ((rdi as u32)) as u32); crate::simd_ops::wu128(rdx.wrapping_sub(0x28u64), (xmm12) as u128); crate::simd_ops::wu64(rdx, ((xmm8 as u64)) as u64); crate::simd_ops::wu32(rdx.wrapping_add(0x8u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x88u64))) as u32 as u64; crate::simd_ops::wu64(rdx.wrapping_sub(0x18u64), ((xmm6 as u64)) as u64); crate::simd_ops::wu64(rdx.wrapping_sub(0xcu64), ((xmm7 as u64)) as u64); crate::simd_ops::wu64(rdx.wrapping_add(0xcu64), ((xmm9 as u64)) as u64); crate::simd_ops::wu32(rdx.wrapping_sub(0x4u64), ((r13 as u32)) as u32); r13 = (crate::simd_ops::ru64(rbp.wrapping_add(0x78u64))) as u64; crate::simd_ops::wu32(rdx.wrapping_add(0x14u64), ((rax as u32)) as u32); crate::simd_ops::wu32(rdx.wrapping_sub(0x10u64), ((r12 as u32)) as u32); let y_addr: u64 = rdx.wrapping_add(0x20u64); core::ptr::write_unaligned(y_addr as *mut u128, xmm0); core::ptr::write_unaligned(y_addr.wrapping_add(16) as *mut u128, ymm0_hi); let y_addr: u64 = rdx.wrapping_add(0x40u64); core::ptr::write_unaligned(y_addr as *mut u128, xmm1); core::ptr::write_unaligned(y_addr.wrapping_add(16) as *mut u128, ymm1_hi); crate::simd_ops::wu64(rdx.wrapping_add(0x60u64), ((xmm10 as u64)) as u64); crate::simd_ops::wu32(rdx.wrapping_add(0x18u64), ((r14 as u32)) as u32); crate::simd_ops::wu32(rdx.wrapping_add(0x1cu64), ((r15 as u32)) as u32); bb = 53; continue; }
            53 => { rax = (crate::simd_ops::ru64(r13)) as u64; rdx = ((rdx).wrapping_sub(0xa0u64)) as u64; r10 = ((r10).wrapping_sub(0xa0u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), (rdx) as u64); fl_l = (rax) as u64 as u64; fl_r = (r10) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 43; continue; } bb = 54; continue; }
            54 => { r9 = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), (r10) as u64); bb = 55; continue; }
            55 => { rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x80u64))) as u64; rcx = (crate::simd_ops::ru64(rcx)) as u64; fl_l = (r10) as u64 as u64; fl_r = (rax) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 60; continue; } bb = 56; continue; }
            56 => { fl_l = (r9) as u64 as u64; fl_r = (rcx) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 66; continue; } bb = 57; continue; }
            57 => { fl_l = (r8) as u64 as u64; fl_r = (r9) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 59; continue; } bb = 58; continue; }
            58 => { let y_addr: u64 = rbx.wrapping_add(0x58u64); xmm0 = core::ptr::read_unaligned(y_addr as *const u128); ymm0_hi = core::ptr::read_unaligned(y_addr.wrapping_add(16) as *const u128); rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x40u64))) as u32 as u64; rdx = (r8) as u64; xmm11 = ((crate::simd_ops::ru32(rbx.wrapping_add(0xcu64))) as u32) as u128; xmm12 = (crate::simd_ops::ru128(rbx.wrapping_add(0x10u64))) as u128; xmm6 = ((crate::simd_ops::ru64(rbx.wrapping_add(0x20u64))) as u64) as u128; xmm7 = ((crate::simd_ops::ru64(rbx.wrapping_add(0x2cu64))) as u64) as u128; xmm8 = ((crate::simd_ops::ru64(rbx.wrapping_add(0x38u64))) as u64) as u128; xmm9 = ((crate::simd_ops::ru64(rbx.wrapping_add(0x44u64))) as u64) as u128; xmm10 = ((crate::simd_ops::ru64(rbx.wrapping_add(0x98u64))) as u64) as u128; r11 = (crate::simd_ops::ru32(rbx)) as u32 as u64; rcx = (rbx) as u64; rdi = (crate::simd_ops::ru32(rbx.wrapping_add(0x4u64))) as u32 as u64; rsi = (crate::simd_ops::ru32(rbx.wrapping_add(0x8u64))) as u32 as u64; r12 = (crate::simd_ops::ru32(rbx.wrapping_add(0x28u64))) as u32 as u64; r13 = (crate::simd_ops::ru32(rbx.wrapping_add(0x34u64))) as u32 as u64; r14 = (crate::simd_ops::ru32(rbx.wrapping_add(0x50u64))) as u32 as u64; r15 = (crate::simd_ops::ru32(rbx.wrapping_add(0x54u64))) as u32 as u64; let y_addr: u64 = rbp.wrapping_sub(0x70u64); core::ptr::write_unaligned(y_addr as *mut u128, xmm0); core::ptr::write_unaligned(y_addr.wrapping_add(16) as *mut u128, ymm0_hi); let y_addr: u64 = rbx.wrapping_add(0x78u64); xmm0 = core::ptr::read_unaligned(y_addr as *const u128); ymm0_hi = core::ptr::read_unaligned(y_addr.wrapping_add(16) as *const u128); crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x4cu64))) as u32 as u64; let y_addr: u64 = rsp.wrapping_add(0x70u64); core::ptr::write_unaligned(y_addr as *mut u128, xmm0); core::ptr::write_unaligned(y_addr.wrapping_add(16) as *mut u128, ymm0_hi); crate::simd_ops::wu32(rbp.wrapping_add(0x88u64), ((rax as u32)) as u32); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x20u64))) as u32 as u64; let y_addr: u64 = rbp.wrapping_sub(0x70u64); xmm0 = core::ptr::read_unaligned(y_addr as *const u128); ymm0_hi = core::ptr::read_unaligned(y_addr.wrapping_add(16) as *const u128); let y_addr: u64 = rsp.wrapping_add(0x70u64); xmm1 = core::ptr::read_unaligned(y_addr as *const u128); ymm1_hi = core::ptr::read_unaligned(y_addr.wrapping_add(16) as *const u128); crate::simd_ops::wu32(r8.wrapping_add(0xcu64), ((xmm11 as u32)) as u32); crate::simd_ops::wu32(r8, ((r11 as u32)) as u32); crate::simd_ops::wu32(r8.wrapping_add(0x4u64), ((rdi as u32)) as u32); crate::simd_ops::wu32(r8.wrapping_add(0x8u64), ((rsi as u32)) as u32); crate::simd_ops::wu128(r8.wrapping_add(0x10u64), (xmm12) as u128); crate::simd_ops::wu64(r8.wrapping_add(0x38u64), ((xmm8 as u64)) as u64); crate::simd_ops::wu32(r8.wrapping_add(0x40u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x88u64))) as u32 as u64; crate::simd_ops::wu64(r8.wrapping_add(0x20u64), ((xmm6 as u64)) as u64); crate::simd_ops::wu64(r8.wrapping_add(0x2cu64), ((xmm7 as u64)) as u64); crate::simd_ops::wu64(r8.wrapping_add(0x44u64), ((xmm9 as u64)) as u64); crate::simd_ops::wu32(r8.wrapping_add(0x4cu64), ((rax as u32)) as u32); crate::simd_ops::wu32(r8.wrapping_add(0x28u64), ((r12 as u32)) as u32); crate::simd_ops::wu32(r8.wrapping_add(0x34u64), ((r13 as u32)) as u32); let y_addr: u64 = r8.wrapping_add(0x58u64); core::ptr::write_unaligned(y_addr as *mut u128, xmm0); core::ptr::write_unaligned(y_addr.wrapping_add(16) as *mut u128, ymm0_hi); let y_addr: u64 = r8.wrapping_add(0x78u64); core::ptr::write_unaligned(y_addr as *mut u128, xmm1); core::ptr::write_unaligned(y_addr.wrapping_add(16) as *mut u128, ymm1_hi); crate::simd_ops::wu64(r8.wrapping_add(0x98u64), ((xmm10 as u64)) as u64); crate::simd_ops::wu32(r8.wrapping_add(0x50u64), ((r14 as u32)) as u32); crate::simd_ops::wu32(r8.wrapping_add(0x54u64), ((r15 as u32)) as u32); bb = 59; continue; }
            59 => { let y_addr: u64 = rbx.wrapping_add(0x58u64); xmm0 = core::ptr::read_unaligned(y_addr as *const u128); ymm0_hi = core::ptr::read_unaligned(y_addr.wrapping_add(16) as *const u128); rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x40u64))) as u32 as u64; r8 = ((r8).wrapping_add(0xa0u64)) as u64; xmm11 = ((crate::simd_ops::ru32(rbx.wrapping_add(0xcu64))) as u32) as u128; xmm12 = (crate::simd_ops::ru128(rbx.wrapping_add(0x10u64))) as u128; xmm6 = ((crate::simd_ops::ru64(rbx.wrapping_add(0x20u64))) as u64) as u128; xmm7 = ((crate::simd_ops::ru64(rbx.wrapping_add(0x2cu64))) as u64) as u128; xmm8 = ((crate::simd_ops::ru64(rbx.wrapping_add(0x38u64))) as u64) as u128; xmm9 = ((crate::simd_ops::ru64(rbx.wrapping_add(0x44u64))) as u64) as u128; xmm10 = ((crate::simd_ops::ru64(rbx.wrapping_add(0x98u64))) as u64) as u128; r11 = (crate::simd_ops::ru32(rbx)) as u32 as u64; rdx = (r9) as u64; rdi = (crate::simd_ops::ru32(rbx.wrapping_add(0x4u64))) as u32 as u64; rcx = (rbx) as u64; rsi = (crate::simd_ops::ru32(rbx.wrapping_add(0x8u64))) as u32 as u64; r12 = (crate::simd_ops::ru32(rbx.wrapping_add(0x28u64))) as u32 as u64; r13 = (crate::simd_ops::ru32(rbx.wrapping_add(0x34u64))) as u32 as u64; r14 = (crate::simd_ops::ru32(rbx.wrapping_add(0x50u64))) as u32 as u64; r15 = (crate::simd_ops::ru32(rbx.wrapping_add(0x54u64))) as u32 as u64; let y_addr: u64 = rbp.wrapping_sub(0x70u64); core::ptr::write_unaligned(y_addr as *mut u128, xmm0); core::ptr::write_unaligned(y_addr.wrapping_add(16) as *mut u128, ymm0_hi); let y_addr: u64 = rbx.wrapping_add(0x78u64); xmm0 = core::ptr::read_unaligned(y_addr as *const u128); ymm0_hi = core::ptr::read_unaligned(y_addr.wrapping_add(16) as *const u128); crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x4cu64))) as u32 as u64; let y_addr: u64 = rsp.wrapping_add(0x70u64); core::ptr::write_unaligned(y_addr as *mut u128, xmm0); core::ptr::write_unaligned(y_addr.wrapping_add(16) as *mut u128, ymm0_hi); crate::simd_ops::wu32(rbp.wrapping_add(0x88u64), ((rax as u32)) as u32); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x20u64))) as u32 as u64; fl_l = ((rbx).wrapping_add(0xa0u64)) as u64 as u64; fl_r = fl_l; rbx = ((fl_l as u64)) as u64; let y_addr: u64 = rbp.wrapping_sub(0x70u64); xmm0 = core::ptr::read_unaligned(y_addr as *const u128); ymm0_hi = core::ptr::read_unaligned(y_addr.wrapping_add(16) as *const u128); let y_addr: u64 = rsp.wrapping_add(0x70u64); xmm1 = core::ptr::read_unaligned(y_addr as *const u128); ymm1_hi = core::ptr::read_unaligned(y_addr.wrapping_add(16) as *const u128); crate::simd_ops::wu32(r9.wrapping_add(0xcu64), ((xmm11 as u32)) as u32); crate::simd_ops::wu32(r9, ((r11 as u32)) as u32); crate::simd_ops::wu32(r9.wrapping_add(0x4u64), ((rdi as u32)) as u32); crate::simd_ops::wu32(r9.wrapping_add(0x8u64), ((rsi as u32)) as u32); crate::simd_ops::wu128(r9.wrapping_add(0x10u64), (xmm12) as u128); crate::simd_ops::wu64(r9.wrapping_add(0x38u64), ((xmm8 as u64)) as u64); crate::simd_ops::wu32(r9.wrapping_add(0x40u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x88u64))) as u32 as u64; crate::simd_ops::wu64(r9.wrapping_add(0x20u64), ((xmm6 as u64)) as u64); crate::simd_ops::wu64(r9.wrapping_add(0x2cu64), ((xmm7 as u64)) as u64); crate::simd_ops::wu64(r9.wrapping_add(0x44u64), ((xmm9 as u64)) as u64); crate::simd_ops::wu32(r9.wrapping_add(0x4cu64), ((rax as u32)) as u32); crate::simd_ops::wu32(r9.wrapping_add(0x28u64), ((r12 as u32)) as u32); crate::simd_ops::wu32(r9.wrapping_add(0x34u64), ((r13 as u32)) as u32); let y_addr: u64 = r9.wrapping_add(0x58u64); core::ptr::write_unaligned(y_addr as *mut u128, xmm0); core::ptr::write_unaligned(y_addr.wrapping_add(16) as *mut u128, ymm0_hi); let y_addr: u64 = r9.wrapping_add(0x78u64); core::ptr::write_unaligned(y_addr as *mut u128, xmm1); core::ptr::write_unaligned(y_addr.wrapping_add(16) as *mut u128, ymm1_hi); crate::simd_ops::wu64(r9.wrapping_add(0x98u64), ((xmm10 as u64)) as u64); crate::simd_ops::wu32(r9.wrapping_add(0x50u64), ((r14 as u32)) as u32); crate::simd_ops::wu32(r9.wrapping_add(0x54u64), ((r15 as u32)) as u32); bb = 65; continue; }
            60 => { r10 = ((r10).wrapping_add(0xffffffffffffff60u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), (r10) as u64); fl_l = (r9) as u64 as u64; fl_r = (rcx) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 64; continue; } bb = 61; continue; }
            61 => { rbx = ((rbx).wrapping_sub(0xa0u64)) as u64; fl_l = (r10) as u64 as u64; fl_r = (rbx) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 63; continue; } bb = 62; continue; }
            62 => { let y_addr: u64 = r10.wrapping_add(0x58u64); xmm0 = core::ptr::read_unaligned(y_addr as *const u128); ymm0_hi = core::ptr::read_unaligned(y_addr.wrapping_add(16) as *const u128); rax = (crate::simd_ops::ru32(r10.wrapping_add(0x50u64))) as u32 as u64; rdx = (rbx) as u64; xmm6 = ((crate::simd_ops::ru32(r10.wrapping_add(0xcu64))) as u32) as u128; xmm11 = (crate::simd_ops::ru128(r10.wrapping_add(0x10u64))) as u128; xmm7 = ((crate::simd_ops::ru64(r10.wrapping_add(0x20u64))) as u64) as u128; xmm8 = ((crate::simd_ops::ru64(r10.wrapping_add(0x2cu64))) as u64) as u128; xmm9 = ((crate::simd_ops::ru64(r10.wrapping_add(0x38u64))) as u64) as u128; xmm10 = ((crate::simd_ops::ru64(r10.wrapping_add(0x44u64))) as u64) as u128; xmm12 = ((crate::simd_ops::ru64(r10.wrapping_add(0x98u64))) as u64) as u128; r11 = (crate::simd_ops::ru32(r10)) as u32 as u64; rcx = (r10) as u64; rdi = (crate::simd_ops::ru32(r10.wrapping_add(0x4u64))) as u32 as u64; rsi = (crate::simd_ops::ru32(r10.wrapping_add(0x8u64))) as u32 as u64; r14 = (crate::simd_ops::ru32(r10.wrapping_add(0x28u64))) as u32 as u64; r15 = (crate::simd_ops::ru32(r10.wrapping_add(0x34u64))) as u32 as u64; r12 = (crate::simd_ops::ru32(r10.wrapping_add(0x40u64))) as u32 as u64; r13 = (crate::simd_ops::ru32(r10.wrapping_add(0x4cu64))) as u32 as u64; let y_addr: u64 = rbp.wrapping_sub(0x70u64); core::ptr::write_unaligned(y_addr as *mut u128, xmm0); core::ptr::write_unaligned(y_addr.wrapping_add(16) as *mut u128, ymm0_hi); let y_addr: u64 = r10.wrapping_add(0x78u64); xmm0 = core::ptr::read_unaligned(y_addr as *const u128); ymm0_hi = core::ptr::read_unaligned(y_addr.wrapping_add(16) as *const u128); crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(r10.wrapping_add(0x54u64))) as u32 as u64; let y_addr: u64 = rsp.wrapping_add(0x70u64); core::ptr::write_unaligned(y_addr as *mut u128, xmm0); core::ptr::write_unaligned(y_addr.wrapping_add(16) as *mut u128, ymm0_hi); crate::simd_ops::wu32(rbp.wrapping_add(0x88u64), ((rax as u32)) as u32); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x20u64))) as u32 as u64; let y_addr: u64 = rbp.wrapping_sub(0x70u64); xmm0 = core::ptr::read_unaligned(y_addr as *const u128); ymm0_hi = core::ptr::read_unaligned(y_addr.wrapping_add(16) as *const u128); let y_addr: u64 = rsp.wrapping_add(0x70u64); xmm1 = core::ptr::read_unaligned(y_addr as *const u128); ymm1_hi = core::ptr::read_unaligned(y_addr.wrapping_add(16) as *const u128); crate::simd_ops::wu32(rbx.wrapping_add(0xcu64), ((xmm6 as u32)) as u32); crate::simd_ops::wu32(rbx, ((r11 as u32)) as u32); crate::simd_ops::wu32(rbx.wrapping_add(0x4u64), ((rdi as u32)) as u32); crate::simd_ops::wu32(rbx.wrapping_add(0x8u64), ((rsi as u32)) as u32); crate::simd_ops::wu128(rbx.wrapping_add(0x10u64), (xmm11) as u128); crate::simd_ops::wu64(rbx.wrapping_add(0x20u64), ((xmm7 as u64)) as u64); crate::simd_ops::wu64(rbx.wrapping_add(0x2cu64), ((xmm8 as u64)) as u64); crate::simd_ops::wu64(rbx.wrapping_add(0x38u64), ((xmm9 as u64)) as u64); crate::simd_ops::wu64(rbx.wrapping_add(0x44u64), ((xmm10 as u64)) as u64); crate::simd_ops::wu32(rbx.wrapping_add(0x28u64), ((r14 as u32)) as u32); crate::simd_ops::wu32(rbx.wrapping_add(0x34u64), ((r15 as u32)) as u32); crate::simd_ops::wu32(rbx.wrapping_add(0x40u64), ((r12 as u32)) as u32); crate::simd_ops::wu32(rbx.wrapping_add(0x4cu64), ((r13 as u32)) as u32); let y_addr: u64 = rbx.wrapping_add(0x58u64); core::ptr::write_unaligned(y_addr as *mut u128, xmm0); core::ptr::write_unaligned(y_addr.wrapping_add(16) as *mut u128, ymm0_hi); crate::simd_ops::wu32(rbx.wrapping_add(0x50u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x88u64))) as u32 as u64; let y_addr: u64 = rbx.wrapping_add(0x78u64); core::ptr::write_unaligned(y_addr as *mut u128, xmm1); core::ptr::write_unaligned(y_addr.wrapping_add(16) as *mut u128, ymm1_hi); crate::simd_ops::wu32(rbx.wrapping_add(0x54u64), ((rax as u32)) as u32); crate::simd_ops::wu64(rbx.wrapping_add(0x98u64), ((xmm12 as u64)) as u64); bb = 63; continue; }
            63 => { let y_addr: u64 = rbx.wrapping_add(0x58u64); xmm0 = core::ptr::read_unaligned(y_addr as *const u128); ymm0_hi = core::ptr::read_unaligned(y_addr.wrapping_add(16) as *const u128); rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x40u64))) as u32 as u64; r8 = ((r8).wrapping_sub(0xa0u64)) as u64; xmm11 = ((crate::simd_ops::ru32(rbx.wrapping_add(0xcu64))) as u32) as u128; xmm12 = (crate::simd_ops::ru128(rbx.wrapping_add(0x10u64))) as u128; xmm6 = ((crate::simd_ops::ru64(rbx.wrapping_add(0x20u64))) as u64) as u128; xmm7 = ((crate::simd_ops::ru64(rbx.wrapping_add(0x2cu64))) as u64) as u128; xmm8 = ((crate::simd_ops::ru64(rbx.wrapping_add(0x38u64))) as u64) as u128; xmm9 = ((crate::simd_ops::ru64(rbx.wrapping_add(0x44u64))) as u64) as u128; xmm10 = ((crate::simd_ops::ru64(rbx.wrapping_add(0x98u64))) as u64) as u128; r11 = (crate::simd_ops::ru32(rbx)) as u32 as u64; rdx = (r8) as u64; rdi = (crate::simd_ops::ru32(rbx.wrapping_add(0x4u64))) as u32 as u64; rcx = (rbx) as u64; rsi = (crate::simd_ops::ru32(rbx.wrapping_add(0x8u64))) as u32 as u64; r12 = (crate::simd_ops::ru32(rbx.wrapping_add(0x28u64))) as u32 as u64; r13 = (crate::simd_ops::ru32(rbx.wrapping_add(0x34u64))) as u32 as u64; r14 = (crate::simd_ops::ru32(rbx.wrapping_add(0x50u64))) as u32 as u64; r15 = (crate::simd_ops::ru32(rbx.wrapping_add(0x54u64))) as u32 as u64; let y_addr: u64 = rbp.wrapping_sub(0x70u64); core::ptr::write_unaligned(y_addr as *mut u128, xmm0); core::ptr::write_unaligned(y_addr.wrapping_add(16) as *mut u128, ymm0_hi); let y_addr: u64 = rbx.wrapping_add(0x78u64); xmm0 = core::ptr::read_unaligned(y_addr as *const u128); ymm0_hi = core::ptr::read_unaligned(y_addr.wrapping_add(16) as *const u128); crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x4cu64))) as u32 as u64; let y_addr: u64 = rsp.wrapping_add(0x70u64); core::ptr::write_unaligned(y_addr as *mut u128, xmm0); core::ptr::write_unaligned(y_addr.wrapping_add(16) as *mut u128, ymm0_hi); crate::simd_ops::wu32(rbp.wrapping_add(0x88u64), ((rax as u32)) as u32); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x20u64))) as u32 as u64; let y_addr: u64 = rbp.wrapping_sub(0x70u64); xmm0 = core::ptr::read_unaligned(y_addr as *const u128); ymm0_hi = core::ptr::read_unaligned(y_addr.wrapping_add(16) as *const u128); let y_addr: u64 = rsp.wrapping_add(0x70u64); xmm1 = core::ptr::read_unaligned(y_addr as *const u128); ymm1_hi = core::ptr::read_unaligned(y_addr.wrapping_add(16) as *const u128); crate::simd_ops::wu32(r8.wrapping_add(0x4u64), ((rdi as u32)) as u32); rdi = (crate::simd_ops::ru64(rbp.wrapping_add(0x80u64))) as u64; crate::simd_ops::wu32(r8.wrapping_add(0xcu64), ((xmm11 as u32)) as u32); crate::simd_ops::wu32(r8, ((r11 as u32)) as u32); crate::simd_ops::wu32(r8.wrapping_add(0x8u64), ((rsi as u32)) as u32); crate::simd_ops::wu128(r8.wrapping_add(0x10u64), (xmm12) as u128); crate::simd_ops::wu64(r8.wrapping_add(0x38u64), ((xmm8 as u64)) as u64); crate::simd_ops::wu32(r8.wrapping_add(0x40u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x88u64))) as u32 as u64; crate::simd_ops::wu64(r8.wrapping_add(0x20u64), ((xmm6 as u64)) as u64); crate::simd_ops::wu64(r8.wrapping_add(0x2cu64), ((xmm7 as u64)) as u64); crate::simd_ops::wu64(r8.wrapping_add(0x44u64), ((xmm9 as u64)) as u64); crate::simd_ops::wu32(r8.wrapping_add(0x34u64), ((r13 as u32)) as u32); r13 = (crate::simd_ops::ru64(rbp.wrapping_add(0x78u64))) as u64; crate::simd_ops::wu32(r8.wrapping_add(0x4cu64), ((rax as u32)) as u32); crate::simd_ops::wu32(r8.wrapping_add(0x28u64), ((r12 as u32)) as u32); let y_addr: u64 = r8.wrapping_add(0x58u64); core::ptr::write_unaligned(y_addr as *mut u128, xmm0); core::ptr::write_unaligned(y_addr.wrapping_add(16) as *mut u128, ymm0_hi); let y_addr: u64 = r8.wrapping_add(0x78u64); core::ptr::write_unaligned(y_addr as *mut u128, xmm1); core::ptr::write_unaligned(y_addr.wrapping_add(16) as *mut u128, ymm1_hi); crate::simd_ops::wu64(r8.wrapping_add(0x98u64), ((xmm10 as u64)) as u64); crate::simd_ops::wu32(r8.wrapping_add(0x50u64), ((r14 as u32)) as u32); crate::simd_ops::wu32(r8.wrapping_add(0x54u64), ((r15 as u32)) as u32); bb = 26; continue; }
            64 => { let y_addr: u64 = r9.wrapping_add(0x58u64); xmm0 = core::ptr::read_unaligned(y_addr as *const u128); ymm0_hi = core::ptr::read_unaligned(y_addr.wrapping_add(16) as *const u128); rax = (crate::simd_ops::ru32(r9.wrapping_add(0x40u64))) as u32 as u64; rdx = (r10) as u64; xmm11 = ((crate::simd_ops::ru32(r9.wrapping_add(0xcu64))) as u32) as u128; xmm12 = (crate::simd_ops::ru128(r9.wrapping_add(0x10u64))) as u128; xmm6 = ((crate::simd_ops::ru64(r9.wrapping_add(0x20u64))) as u64) as u128; xmm7 = ((crate::simd_ops::ru64(r9.wrapping_add(0x2cu64))) as u64) as u128; xmm8 = ((crate::simd_ops::ru64(r9.wrapping_add(0x38u64))) as u64) as u128; xmm9 = ((crate::simd_ops::ru64(r9.wrapping_add(0x44u64))) as u64) as u128; xmm10 = ((crate::simd_ops::ru64(r9.wrapping_add(0x98u64))) as u64) as u128; r11 = (crate::simd_ops::ru32(r9)) as u32 as u64; rcx = (r9) as u64; rdi = (crate::simd_ops::ru32(r9.wrapping_add(0x4u64))) as u32 as u64; rsi = (crate::simd_ops::ru32(r9.wrapping_add(0x8u64))) as u32 as u64; r12 = (crate::simd_ops::ru32(r9.wrapping_add(0x28u64))) as u32 as u64; r13 = (crate::simd_ops::ru32(r9.wrapping_add(0x34u64))) as u32 as u64; r14 = (crate::simd_ops::ru32(r9.wrapping_add(0x50u64))) as u32 as u64; r15 = (crate::simd_ops::ru32(r9.wrapping_add(0x54u64))) as u32 as u64; let y_addr: u64 = rbp.wrapping_sub(0x70u64); core::ptr::write_unaligned(y_addr as *mut u128, xmm0); core::ptr::write_unaligned(y_addr.wrapping_add(16) as *mut u128, ymm0_hi); let y_addr: u64 = r9.wrapping_add(0x78u64); xmm0 = core::ptr::read_unaligned(y_addr as *const u128); ymm0_hi = core::ptr::read_unaligned(y_addr.wrapping_add(16) as *const u128); crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(r9.wrapping_add(0x4cu64))) as u32 as u64; let y_addr: u64 = rsp.wrapping_add(0x70u64); core::ptr::write_unaligned(y_addr as *mut u128, xmm0); core::ptr::write_unaligned(y_addr.wrapping_add(16) as *mut u128, ymm0_hi); crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), (r10) as u64); crate::simd_ops::wu32(rbp.wrapping_add(0x88u64), ((rax as u32)) as u32); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x20u64))) as u32 as u64; let y_addr: u64 = rbp.wrapping_sub(0x70u64); xmm0 = core::ptr::read_unaligned(y_addr as *const u128); ymm0_hi = core::ptr::read_unaligned(y_addr.wrapping_add(16) as *const u128); let y_addr: u64 = rsp.wrapping_add(0x70u64); xmm1 = core::ptr::read_unaligned(y_addr as *const u128); ymm1_hi = core::ptr::read_unaligned(y_addr.wrapping_add(16) as *const u128); crate::simd_ops::wu32(r10.wrapping_add(0xcu64), ((xmm11 as u32)) as u32); crate::simd_ops::wu32(r10, ((r11 as u32)) as u32); crate::simd_ops::wu32(r10.wrapping_add(0x4u64), ((rdi as u32)) as u32); crate::simd_ops::wu32(r10.wrapping_add(0x8u64), ((rsi as u32)) as u32); crate::simd_ops::wu128(r10.wrapping_add(0x10u64), (xmm12) as u128); crate::simd_ops::wu64(r10.wrapping_add(0x38u64), ((xmm8 as u64)) as u64); crate::simd_ops::wu32(r10.wrapping_add(0x40u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x88u64))) as u32 as u64; crate::simd_ops::wu64(r10.wrapping_add(0x20u64), ((xmm6 as u64)) as u64); crate::simd_ops::wu64(r10.wrapping_add(0x2cu64), ((xmm7 as u64)) as u64); crate::simd_ops::wu64(r10.wrapping_add(0x44u64), ((xmm9 as u64)) as u64); crate::simd_ops::wu32(r10.wrapping_add(0x4cu64), ((rax as u32)) as u32); crate::simd_ops::wu32(r10.wrapping_add(0x28u64), ((r12 as u32)) as u32); crate::simd_ops::wu32(r10.wrapping_add(0x34u64), ((r13 as u32)) as u32); let y_addr: u64 = r10.wrapping_add(0x58u64); core::ptr::write_unaligned(y_addr as *mut u128, xmm0); core::ptr::write_unaligned(y_addr.wrapping_add(16) as *mut u128, ymm0_hi); let y_addr: u64 = r10.wrapping_add(0x78u64); core::ptr::write_unaligned(y_addr as *mut u128, xmm1); core::ptr::write_unaligned(y_addr.wrapping_add(16) as *mut u128, ymm1_hi); crate::simd_ops::wu64(r10.wrapping_add(0x98u64), ((xmm10 as u64)) as u64); crate::simd_ops::wu32(r10.wrapping_add(0x50u64), ((r14 as u32)) as u32); crate::simd_ops::wu32(r10.wrapping_add(0x54u64), ((r15 as u32)) as u32); bb = 65; continue; }
            65 => { rdi = (crate::simd_ops::ru64(rbp.wrapping_add(0x80u64))) as u64; fl_l = ((r9).wrapping_add(0xa0u64)) as u64 as u64; fl_r = fl_l; r9 = ((fl_l as u64)) as u64; r13 = (crate::simd_ops::ru64(rbp.wrapping_add(0x78u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), (r9) as u64); bb = 26; continue; }
            66 => { rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x70u64))) as u64; crate::simd_ops::wu64(rax, (rbx) as u64); crate::simd_ops::wu64(rax.wrapping_add(0x8u64), (r8) as u64); r11 = ((rsp.wrapping_add(0x128u64)) as u64) as u64; xmm6 = (crate::simd_ops::ru128(r11.wrapping_sub(0x18u64))) as u128; xmm7 = (crate::simd_ops::ru128(r11.wrapping_sub(0x28u64))) as u128; xmm8 = (crate::simd_ops::ru128(r11.wrapping_sub(0x38u64))) as u128; xmm9 = (crate::simd_ops::ru128(r11.wrapping_sub(0x48u64))) as u128; xmm10 = (crate::simd_ops::ru128(r11.wrapping_sub(0x58u64))) as u128; xmm11 = (crate::simd_ops::ru128(r11.wrapping_sub(0x68u64))) as u128; xmm12 = (crate::simd_ops::ru128(r11.wrapping_sub(0x78u64))) as u128; rsp = (r11) as u64; r15 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r13 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r12 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rsi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rbx = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rbp = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1412f09b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1412f1590; const P1: u64 = 0x1412f09b0; const P2: u64 = 0x14183acc8;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut xmm6: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rax = rdx; rbx = (((r9 as u8)) as u32) as u32 as u64; rax = ((rax).wrapping_sub(rcx)) as u64; rsi = r8; rax = (((rax) & (0xfffffffffffffffcu64))) as u64; rbp = rdx; rdi = rcx; fl_l = rax; fl_r = (0x80u64) as u64; if ((((fl_l as u64)) as i64) <= (((fl_r as u64)) as i64)) { bb = 7; continue; } bb = 1; continue; }
            1 => { bb = 2; continue; }
            2 => { fl_l = rsi; fl_r = rsi; if (((((fl_l as u64)) & ((fl_r as u64))) as i64) <= 0) { bb = 12; continue; } bb = 3; continue; }
            3 => { r9 = (((rbx as u8)) as u32) as u32 as u64; rcx = rsp.wrapping_add(0x20); r8 = rbp; rdx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; rcx = rsi; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; rax = r15; rcx = ((((rcx) as i64).wrapping_shr(2u32) as u64)) as u64; rax = ((rax).wrapping_sub(rdi)) as u64; rsi = ((((rsi) as i64).wrapping_shr(1u32) as u64)) as u64; rax = (((rax) & (0xfffffffffffffffcu64))) as u64; rsi = ((rsi).wrapping_add(rcx)) as u64; r9 = (((rbx as u8)) as u32) as u32 as u64; rcx = rbp; r8 = rsi; rcx = ((rcx).wrapping_sub(r14)) as u64; rcx = (((rcx) & (0xfffffffffffffffcu64))) as u64; fl_l = rax; fl_r = rcx; if ((((fl_l as u64)) as i64) >= (((fl_r as u64)) as i64)) { bb = 5; continue; } bb = 4; continue; }
            4 => { rdx = r15; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdi = r14; bb = 6; continue; }
            5 => { rdx = rbp; rcx = r14; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbp = r15; bb = 6; continue; }
            6 => { rax = rbp; rax = ((rax).wrapping_sub(rdi)) as u64; rax = (((rax) & (0xfffffffffffffffcu64))) as u64; fl_l = rax; fl_r = (0x80u64) as u64; if ((((fl_l as u64)) as i64) > (((fl_r as u64)) as i64)) { bb = 2; continue; } bb = 7; continue; }
            7 => { fl_l = rdi; fl_r = rbp; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 44; continue; } bb = 8; continue; }
            8 => { r14 = rdi.wrapping_add(0x4); rbx = r14; fl_l = r14; fl_r = rbp; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 44; continue; } bb = 9; continue; }
            9 => { rsi = 0x4; crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm6) as u128); fl_l = rsi; fl_r = rdi; rsi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; bb = 10; continue; }
            10 => { xmm6 = ((crate::simd_ops::ru32(rbx)) as u32) as u128; xmm0 = ((crate::simd_ops::ru32(rdi)) as u32) as u128; fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = ((xmm6 as u32)) as u32 as u64; rcx = rbx; if !(f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 38; continue; } bb = 11; continue; }
            11 => { r8 = rbx.wrapping_sub(0x4); rdx = rdi; r8 = ((r8).wrapping_add(rsi)) as u64; rcx = r14; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu32(rdi, ((xmm6 as u32)) as u32); bb = 42; continue; }
            12 => { r10 = rbp; r10 = ((r10).wrapping_sub(rdi)) as u64; r10 = ((((r10) as i64).wrapping_shr(2u32) as u64)) as u64; r8 = r10; r8 = ((((r8) as i64).wrapping_shr(1u32) as u64)) as u64; fl_l = r8; fl_r = r8; if (((((fl_l as u64)) & ((fl_r as u64))) as i64) <= 0) { bb = 25; continue; } bb = 13; continue; }
            13 => { r9 = r10.wrapping_sub(0x1); fl_l = ((((r9) as i64).wrapping_shr(1u32) as u64)) as u64; r9 = ((fl_l as u64)) as u64; bb = 14; continue; }
            14 => { xmm1 = ((crate::simd_ops::ru32(rdi.wrapping_add(r8.wrapping_mul(4)).wrapping_sub(0x4u64))) as u32) as u128; r8 = ((r8).wrapping_sub(1)) as u64; rcx = r8; rdx = r8; fl_l = r8; fl_r = r9; if ((((fl_l as u64)) as i64) >= (((fl_r as u64)) as i64)) { bb = 17; continue; } bb = 15; continue; }
            15 => { bb = 16; continue; }
            16 => { rax = ((rdx.wrapping_add(rdx)) as u64) as u64; rdx = 0; fl_l = 0; fl_r = 0; xmm0 = ((crate::simd_ops::ru32(rdi.wrapping_add(rax.wrapping_mul(4)).wrapping_add(0x4u64))) as u32) as u128; fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rdi.wrapping_add(rax.wrapping_mul(4)).wrapping_add(0x8u64))) as u32 as u64; rdx = (rdx & !0xffu64) | (((if !(f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { 1u8 } else { 0u8 }) as u8 as u64)); rdx = ((rdx).wrapping_add(1)) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru32(rdi.wrapping_add(rdx.wrapping_mul(4)))) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(rcx.wrapping_mul(4)), ((rax as u32)) as u32); rcx = rdx; fl_l = rdx; fl_r = r9; if ((((fl_l as u64)) as i64) < (((fl_r as u64)) as i64)) { bb = 16; continue; } bb = 17; continue; }
            17 => { fl_l = rcx; fl_r = r9; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 20; continue; } bb = 18; continue; }
            18 => { fl_l = ((r10 as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 20; continue; } bb = 19; continue; }
            19 => { rax = (crate::simd_ops::ru32(rdi.wrapping_add(r10.wrapping_mul(4)).wrapping_sub(0x4u64))) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(rcx.wrapping_mul(4)), ((rax as u32)) as u32); rcx = r10.wrapping_sub(0x1); bb = 20; continue; }
            20 => { fl_l = r8; fl_r = rcx; if ((((fl_l as u64)) as i64) >= (((fl_r as u64)) as i64)) { bb = 24; continue; } bb = 21; continue; }
            21 => { bb = 22; continue; }
            22 => { rax = rcx.wrapping_sub(0x1); rax = ((((rax) as i64).wrapping_shr(1u32) as u64)) as u64; xmm0 = ((crate::simd_ops::ru32(rdi.wrapping_add(rax.wrapping_mul(4)))) as u32) as u128; fl_l = ((xmm1 as u32)) as u32 as u64; fl_r = ((xmm0 as u32)) as u32 as u64; if !(f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 24; continue; } bb = 23; continue; }
            23 => { crate::simd_ops::wu32(rdi.wrapping_add(rcx.wrapping_mul(4)), ((xmm0 as u32)) as u32); rcx = rax; fl_l = r8; fl_r = rax; if ((((fl_l as u64)) as i64) < (((fl_r as u64)) as i64)) { bb = 22; continue; } bb = 24; continue; }
            24 => { crate::simd_ops::wu32(rdi.wrapping_add(rcx.wrapping_mul(4)), ((xmm1 as u32)) as u32); fl_l = r8; fl_r = r8; if (((((fl_l as u64)) & ((fl_r as u64))) as i64) > 0) { bb = 14; continue; } bb = 25; continue; }
            25 => { fl_l = r10; fl_r = (0x2u64) as u64; if ((((fl_l as u64)) as i64) < (((fl_r as u64)) as i64)) { bb = 44; continue; } bb = 26; continue; }
            26 => { rsi = 0x4; r10 = rbp.wrapping_sub(0x4); fl_l = rsi; fl_r = rdi; rsi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; bb = 27; continue; }
            27 => { xmm1 = ((crate::simd_ops::ru32(r10)) as u32) as u128; rax = (crate::simd_ops::ru32(rdi)) as u32 as u64; r8 = r10; r8 = ((r8).wrapping_sub(rdi)) as u64; crate::simd_ops::wu32(r10, ((rax as u32)) as u32); r8 = ((((r8) as i64).wrapping_shr(2u32) as u64)) as u64; rcx = 0; fl_l = 0; fl_r = 0; rdx = 0; fl_l = 0; fl_r = 0; r9 = r8.wrapping_sub(0x1); r9 = ((((r9) as i64).wrapping_shr(1u32) as u64)) as u64; fl_l = r9; fl_r = r9; if (((((fl_l as u64)) & ((fl_r as u64))) as i64) <= 0) { bb = 29; continue; } bb = 28; continue; }
            28 => { rax = ((rdx.wrapping_add(rdx)) as u64) as u64; rdx = 0; fl_l = 0; fl_r = 0; xmm0 = ((crate::simd_ops::ru32(rdi.wrapping_add(rax.wrapping_mul(4)).wrapping_add(0x4u64))) as u32) as u128; fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rdi.wrapping_add(rax.wrapping_mul(4)).wrapping_add(0x8u64))) as u32 as u64; rdx = (rdx & !0xffu64) | (((if !(f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { 1u8 } else { 0u8 }) as u8 as u64)); rdx = ((rdx).wrapping_add(1)) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru32(rdi.wrapping_add(rdx.wrapping_mul(4)))) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(rcx.wrapping_mul(4)), ((rax as u32)) as u32); rcx = rdx; fl_l = rdx; fl_r = r9; if ((((fl_l as u64)) as i64) < (((fl_r as u64)) as i64)) { bb = 28; continue; } bb = 29; continue; }
            29 => { fl_l = rcx; fl_r = r9; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 32; continue; } bb = 30; continue; }
            30 => { fl_l = ((r8 as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 32; continue; } bb = 31; continue; }
            31 => { rax = (crate::simd_ops::ru32(rdi.wrapping_add(r8.wrapping_mul(4)).wrapping_sub(0x4u64))) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(rcx.wrapping_mul(4)), ((rax as u32)) as u32); rcx = r8.wrapping_sub(0x1); bb = 32; continue; }
            32 => { fl_l = rcx; fl_r = rcx; if (((((fl_l as u64)) & ((fl_r as u64))) as i64) <= 0) { bb = 36; continue; } bb = 33; continue; }
            33 => { bb = 34; continue; }
            34 => { rax = rcx.wrapping_sub(0x1); rax = ((((rax) as i64).wrapping_shr(1u32) as u64)) as u64; xmm0 = ((crate::simd_ops::ru32(rdi.wrapping_add(rax.wrapping_mul(4)))) as u32) as u128; fl_l = ((xmm1 as u32)) as u32 as u64; fl_r = ((xmm0 as u32)) as u32 as u64; if !(f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 36; continue; } bb = 35; continue; }
            35 => { crate::simd_ops::wu32(rdi.wrapping_add(rcx.wrapping_mul(4)), ((xmm0 as u32)) as u32); rcx = rax; fl_l = rax; fl_r = rax; if (((((fl_l as u64)) & ((fl_r as u64))) as i64) > 0) { bb = 34; continue; } bb = 36; continue; }
            36 => { r10 = ((r10).wrapping_sub(0x4u64)) as u64; crate::simd_ops::wu32(rdi.wrapping_add(rcx.wrapping_mul(4)), ((xmm1 as u32)) as u32); rax = ((r10.wrapping_add(rsi)) as u64) as u64; rax = (((rax) & (0xfffffffffffffffcu64))) as u64; fl_l = rax; fl_r = (0x8u64) as u64; if ((((fl_l as u64)) as i64) >= (((fl_r as u64)) as i64)) { bb = 27; continue; } bb = 37; continue; }
            37 => { bb = 44; continue; }
            38 => { rax = rbx.wrapping_sub(0x4); xmm0 = ((crate::simd_ops::ru32(rax)) as u32) as u128; fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = ((xmm6 as u32)) as u32 as u64; if !(f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 41; continue; } bb = 39; continue; }
            39 => { bb = 40; continue; }
            40 => { crate::simd_ops::wu32(rcx, ((xmm0 as u32)) as u32); xmm0 = ((crate::simd_ops::ru32(rax.wrapping_sub(0x4u64))) as u32) as u128; rcx = rax; rax = ((rax).wrapping_sub(0x4u64)) as u64; fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = ((xmm6 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 40; continue; } bb = 41; continue; }
            41 => { crate::simd_ops::wu32(rcx, ((xmm6 as u32)) as u32); bb = 42; continue; }
            42 => { rbx = ((rbx).wrapping_add(0x4u64)) as u64; fl_l = rbx; fl_r = rbp; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 10; continue; } bb = 43; continue; }
            43 => { xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x30u64))) as u128; bb = 44; continue; }
            44 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1412f0cc0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x4; const P1: u64 = 0x8; const P2: u64 = 0xc; const P3: u64 = 0x10; const P4: u64 = 0x20; const P5: u64 = 0x28; const P6: u64 = 0x2c; const P7: u64 = 0x34; const P8: u64 = 0x38; const P9: u64 = 0x40; const P10: u64 = 0x44; const P11: u64 = 0x4c;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut ymm0_hi: u128 = 0; let mut ymm1_hi: u128 = 0;
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = (crate::simd_ops::ru32(rdx)) as u32 as u64; crate::simd_ops::wu32(rcx, ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(P0), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdx.wrapping_add(P1))) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(P1), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdx.wrapping_add(P2))) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(P2), ((rax as u32)) as u32); xmm0 = (crate::simd_ops::ru128(rdx.wrapping_add(P3))) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rcx.wrapping_add(P3), (xmm0) as u128); xmm0 = ((crate::simd_ops::ru64(rdx.wrapping_add(P4))) as u64) as u128; ymm0_hi = 0; crate::simd_ops::wu64(rcx.wrapping_add(P4), ((xmm0 as u64)) as u64); rax = (crate::simd_ops::ru32(rdx.wrapping_add(P5))) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(P5), ((rax as u32)) as u32); xmm0 = ((crate::simd_ops::ru64(rdx.wrapping_add(P6))) as u64) as u128; ymm0_hi = 0; crate::simd_ops::wu64(rcx.wrapping_add(P6), ((xmm0 as u64)) as u64); rax = (crate::simd_ops::ru32(rdx.wrapping_add(P7))) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(P7), ((rax as u32)) as u32); xmm0 = ((crate::simd_ops::ru64(rdx.wrapping_add(P8))) as u64) as u128; ymm0_hi = 0; crate::simd_ops::wu64(rcx.wrapping_add(P8), ((xmm0 as u64)) as u64); rax = (crate::simd_ops::ru32(rdx.wrapping_add(P9))) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(P9), ((rax as u32)) as u32); xmm0 = ((crate::simd_ops::ru64(rdx.wrapping_add(P10))) as u64) as u128; ymm0_hi = 0; crate::simd_ops::wu64(rcx.wrapping_add(P10), ((xmm0 as u64)) as u64); rax = (crate::simd_ops::ru32(rdx.wrapping_add(P11))) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(0x4cu64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x50u64))) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(0x50u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x54u64))) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(0x54u64), ((rax as u32)) as u32); rax = rcx; let y_addr: u64 = rdx.wrapping_add(0x58u64); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rcx.wrapping_add(0x58u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); xmm1 = ((crate::simd_ops::ru64(rdx.wrapping_add(0x78u64))) as u64) as u128; ymm1_hi = 0; crate::simd_ops::wu64(rcx.wrapping_add(0x78u64), ((xmm1 as u64)) as u64); xmm0 = (crate::simd_ops::ru128(rdx.wrapping_add(0x58u64))) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rcx.wrapping_add(0x58u64), (xmm0) as u128); let y_addr: u64 = rdx.wrapping_add(0x58u64); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rcx.wrapping_add(0x58u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); let y_addr: u64 = rdx.wrapping_add(0x58u64); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rcx.wrapping_add(0x58u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); let y_addr: u64 = rdx.wrapping_add(0x78u64); xmm1 = crate::simd_ops::ru128(y_addr); ymm1_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rcx.wrapping_add(0x78u64); crate::simd_ops::wu128(y_addr, xmm1); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm1_hi); xmm0 = ((crate::simd_ops::ru64(rdx.wrapping_add(0x98u64))) as u64) as u128; ymm0_hi = 0; crate::simd_ops::wu64(rcx.wrapping_add(0x98u64), ((xmm0 as u64)) as u64); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1412f0d80(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1412f0cc0;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0; let mut ymm1_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; r11 = r8.wrapping_sub(0x1); rdi = r9; r11 = ((((r11) as i64).wrapping_shr(1u32) as u64)) as u64; rsi = r8; r10 = rdx; r14 = rcx; rbp = rdx; rbx = rdx; fl_l = rdx; fl_r = r11; if ((((fl_l as u64)) as i64) >= (((fl_r as u64)) as i64)) { bb = 3; continue; } bb = 1; continue; }
            1 => { bb = 2; continue; }
            2 => { r9 = (crate::simd_ops::ru64(r14)) as u64; r8 = ((rbx.wrapping_add(rbx)) as u64) as u64; rcx = 0; fl_l = 0; fl_r = 0; rdx = ((r8.wrapping_add(r8.wrapping_mul(4))) as u64) as u64; rdx = ((rdx).wrapping_shl(5u32)) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(r9).wrapping_add(0xa8u64))) as u32 as u64; fl_l = (crate::simd_ops::ru32(rdx.wrapping_add(r9).wrapping_add(0x148u64))) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; rcx = (rcx & !0xffu64) | (((if ((((fl_l as u32)) as i32) >= (((fl_r as u32)) as i32)) { 1u8 } else { 0u8 }) as u8 as u64)); rbx = rcx.wrapping_add(0x1); rbx = ((rbx).wrapping_add(r8)) as u64; rcx = ((r10.wrapping_add(r10.wrapping_mul(4))) as u64) as u64; rcx = ((rcx).wrapping_shl(5u32)) as u64; r10 = rbx; rdx = ((rbx.wrapping_add(rbx.wrapping_mul(4))) as u64) as u64; rdx = ((rdx).wrapping_shl(5u32)) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(r9))) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(r9), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdx.wrapping_add(r9).wrapping_add(0x4u64))) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(r9).wrapping_add(0x4u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdx.wrapping_add(r9).wrapping_add(0x8u64))) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(r9).wrapping_add(0x8u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdx.wrapping_add(r9).wrapping_add(0xcu64))) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(r9).wrapping_add(0xcu64), ((rax as u32)) as u32); xmm0 = (crate::simd_ops::ru128(rdx.wrapping_add(r9).wrapping_add(0x10u64))) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rcx.wrapping_add(r9).wrapping_add(0x10u64), (xmm0) as u128); xmm0 = ((crate::simd_ops::ru64(rdx.wrapping_add(r9).wrapping_add(0x20u64))) as u64) as u128; ymm0_hi = 0; crate::simd_ops::wu64(rcx.wrapping_add(r9).wrapping_add(0x20u64), ((xmm0 as u64)) as u64); rax = (crate::simd_ops::ru32(rdx.wrapping_add(r9).wrapping_add(0x28u64))) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(r9).wrapping_add(0x28u64), ((rax as u32)) as u32); xmm0 = ((crate::simd_ops::ru64(rdx.wrapping_add(r9).wrapping_add(0x2cu64))) as u64) as u128; ymm0_hi = 0; crate::simd_ops::wu64(rcx.wrapping_add(r9).wrapping_add(0x2cu64), ((xmm0 as u64)) as u64); rax = (crate::simd_ops::ru32(rdx.wrapping_add(r9).wrapping_add(0x34u64))) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(r9).wrapping_add(0x34u64), ((rax as u32)) as u32); xmm0 = ((crate::simd_ops::ru64(rdx.wrapping_add(r9).wrapping_add(0x38u64))) as u64) as u128; ymm0_hi = 0; crate::simd_ops::wu64(rcx.wrapping_add(r9).wrapping_add(0x38u64), ((xmm0 as u64)) as u64); rax = (crate::simd_ops::ru32(rdx.wrapping_add(r9).wrapping_add(0x40u64))) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(r9).wrapping_add(0x40u64), ((rax as u32)) as u32); xmm0 = ((crate::simd_ops::ru64(rdx.wrapping_add(r9).wrapping_add(0x44u64))) as u64) as u128; ymm0_hi = 0; crate::simd_ops::wu64(rcx.wrapping_add(r9).wrapping_add(0x44u64), ((xmm0 as u64)) as u64); rax = (crate::simd_ops::ru32(rdx.wrapping_add(r9).wrapping_add(0x4cu64))) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(r9).wrapping_add(0x4cu64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdx.wrapping_add(r9).wrapping_add(0x50u64))) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(r9).wrapping_add(0x50u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdx.wrapping_add(r9).wrapping_add(0x54u64))) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(r9).wrapping_add(0x54u64), ((rax as u32)) as u32); let y_addr: u64 = rdx.wrapping_add(r9).wrapping_add(0x58u64); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rcx.wrapping_add(r9).wrapping_add(0x58u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); xmm1 = ((crate::simd_ops::ru64(rdx.wrapping_add(r9).wrapping_add(0x78u64))) as u64) as u128; ymm1_hi = 0; crate::simd_ops::wu64(rcx.wrapping_add(r9).wrapping_add(0x78u64), ((xmm1 as u64)) as u64); xmm0 = (crate::simd_ops::ru128(rdx.wrapping_add(r9).wrapping_add(0x58u64))) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rcx.wrapping_add(r9).wrapping_add(0x58u64), (xmm0) as u128); let y_addr: u64 = rdx.wrapping_add(r9).wrapping_add(0x58u64); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rcx.wrapping_add(r9).wrapping_add(0x58u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); let y_addr: u64 = rdx.wrapping_add(r9).wrapping_add(0x58u64); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rcx.wrapping_add(r9).wrapping_add(0x58u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); let y_addr: u64 = rdx.wrapping_add(r9).wrapping_add(0x78u64); xmm1 = crate::simd_ops::ru128(y_addr); ymm1_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rcx.wrapping_add(r9).wrapping_add(0x78u64); crate::simd_ops::wu128(y_addr, xmm1); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm1_hi); xmm0 = ((crate::simd_ops::ru64(rdx.wrapping_add(r9).wrapping_add(0x98u64))) as u64) as u128; ymm0_hi = 0; crate::simd_ops::wu64(rcx.wrapping_add(r9).wrapping_add(0x98u64), ((xmm0 as u64)) as u64); fl_l = rbx; fl_r = r11; if ((((fl_l as u64)) as i64) < (((fl_r as u64)) as i64)) { bb = 2; continue; } bb = 3; continue; }
            3 => { fl_l = rbx; fl_r = r11; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 6; continue; } bb = 4; continue; }
            4 => { fl_l = ((rsi as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { r8 = (crate::simd_ops::ru64(r14)) as u64; rax = ((rsi.wrapping_add(rsi.wrapping_mul(4))) as u64) as u64; rcx = ((r10.wrapping_add(r10.wrapping_mul(4))) as u64) as u64; rax = ((rax).wrapping_shl(5u32)) as u64; rcx = ((rcx).wrapping_shl(5u32)) as u64; rcx = ((rcx).wrapping_add(r8)) as u64; rdx = r8.wrapping_sub(0xa0); rdx = ((rdx).wrapping_add(rax)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r10 = rsi.wrapping_sub(0x1); bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru64(r14)) as u64; r9 = r10.wrapping_sub(0x1); r9 = ((((r9) as i64).wrapping_shr(1u32) as u64)) as u64; fl_l = rbp; fl_r = r10; if ((((fl_l as u64)) as i64) >= (((fl_r as u64)) as i64)) { bb = 10; continue; } bb = 7; continue; }
            7 => { bb = 8; continue; }
            8 => { rcx = (crate::simd_ops::ru32(rdi.wrapping_add(0x8u64))) as u32 as u64; r8 = ((r9.wrapping_add(r9.wrapping_mul(4))) as u64) as u64; r8 = ((r8).wrapping_shl(5u32)) as u64; fl_l = (crate::simd_ops::ru32(r8.wrapping_add(rax).wrapping_add(0x8u64))) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) >= (((fl_r as u32)) as i32)) { bb = 10; continue; } bb = 9; continue; }
            9 => { rcx = (crate::simd_ops::ru32(r8.wrapping_add(rax))) as u32 as u64; rdx = ((r10.wrapping_add(r10.wrapping_mul(4))) as u64) as u64; rdx = ((rdx).wrapping_shl(5u32)) as u64; r10 = r9; r9 = ((r9).wrapping_sub(1)) as u64; r9 = ((((r9) as i64).wrapping_shr(1u32) as u64)) as u64; crate::simd_ops::wu32(rdx.wrapping_add(rax), ((rcx as u32)) as u32); rcx = (crate::simd_ops::ru32(r8.wrapping_add(rax).wrapping_add(0x4u64))) as u32 as u64; crate::simd_ops::wu32(rdx.wrapping_add(rax).wrapping_add(0x4u64), ((rcx as u32)) as u32); rcx = (crate::simd_ops::ru32(r8.wrapping_add(rax).wrapping_add(0x8u64))) as u32 as u64; crate::simd_ops::wu32(rdx.wrapping_add(rax).wrapping_add(0x8u64), ((rcx as u32)) as u32); rcx = (crate::simd_ops::ru32(r8.wrapping_add(rax).wrapping_add(0xcu64))) as u32 as u64; crate::simd_ops::wu32(rdx.wrapping_add(rax).wrapping_add(0xcu64), ((rcx as u32)) as u32); xmm0 = (crate::simd_ops::ru128(r8.wrapping_add(rax).wrapping_add(0x10u64))) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rdx.wrapping_add(rax).wrapping_add(0x10u64), (xmm0) as u128); xmm0 = ((crate::simd_ops::ru64(r8.wrapping_add(rax).wrapping_add(0x20u64))) as u64) as u128; ymm0_hi = 0; crate::simd_ops::wu64(rdx.wrapping_add(rax).wrapping_add(0x20u64), ((xmm0 as u64)) as u64); rcx = (crate::simd_ops::ru32(r8.wrapping_add(rax).wrapping_add(0x28u64))) as u32 as u64; crate::simd_ops::wu32(rdx.wrapping_add(rax).wrapping_add(0x28u64), ((rcx as u32)) as u32); xmm0 = ((crate::simd_ops::ru64(r8.wrapping_add(rax).wrapping_add(0x2cu64))) as u64) as u128; ymm0_hi = 0; crate::simd_ops::wu64(rdx.wrapping_add(rax).wrapping_add(0x2cu64), ((xmm0 as u64)) as u64); rcx = (crate::simd_ops::ru32(r8.wrapping_add(rax).wrapping_add(0x34u64))) as u32 as u64; crate::simd_ops::wu32(rdx.wrapping_add(rax).wrapping_add(0x34u64), ((rcx as u32)) as u32); xmm0 = ((crate::simd_ops::ru64(r8.wrapping_add(rax).wrapping_add(0x38u64))) as u64) as u128; ymm0_hi = 0; crate::simd_ops::wu64(rdx.wrapping_add(rax).wrapping_add(0x38u64), ((xmm0 as u64)) as u64); rcx = (crate::simd_ops::ru32(r8.wrapping_add(rax).wrapping_add(0x40u64))) as u32 as u64; crate::simd_ops::wu32(rdx.wrapping_add(rax).wrapping_add(0x40u64), ((rcx as u32)) as u32); xmm0 = ((crate::simd_ops::ru64(r8.wrapping_add(rax).wrapping_add(0x44u64))) as u64) as u128; ymm0_hi = 0; crate::simd_ops::wu64(rdx.wrapping_add(rax).wrapping_add(0x44u64), ((xmm0 as u64)) as u64); rcx = (crate::simd_ops::ru32(r8.wrapping_add(rax).wrapping_add(0x4cu64))) as u32 as u64; crate::simd_ops::wu32(rdx.wrapping_add(rax).wrapping_add(0x4cu64), ((rcx as u32)) as u32); rcx = (crate::simd_ops::ru32(r8.wrapping_add(rax).wrapping_add(0x50u64))) as u32 as u64; crate::simd_ops::wu32(rdx.wrapping_add(rax).wrapping_add(0x50u64), ((rcx as u32)) as u32); rcx = (crate::simd_ops::ru32(r8.wrapping_add(rax).wrapping_add(0x54u64))) as u32 as u64; crate::simd_ops::wu32(rdx.wrapping_add(rax).wrapping_add(0x54u64), ((rcx as u32)) as u32); let y_addr: u64 = r8.wrapping_add(rax).wrapping_add(0x58u64); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rdx.wrapping_add(rax).wrapping_add(0x58u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); xmm1 = ((crate::simd_ops::ru64(r8.wrapping_add(rax).wrapping_add(0x78u64))) as u64) as u128; ymm1_hi = 0; crate::simd_ops::wu64(rdx.wrapping_add(rax).wrapping_add(0x78u64), ((xmm1 as u64)) as u64); xmm0 = (crate::simd_ops::ru128(r8.wrapping_add(rax).wrapping_add(0x58u64))) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rdx.wrapping_add(rax).wrapping_add(0x58u64), (xmm0) as u128); let y_addr: u64 = r8.wrapping_add(rax).wrapping_add(0x58u64); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rdx.wrapping_add(rax).wrapping_add(0x58u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); let y_addr: u64 = r8.wrapping_add(rax).wrapping_add(0x58u64); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rdx.wrapping_add(rax).wrapping_add(0x58u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); let y_addr: u64 = r8.wrapping_add(rax).wrapping_add(0x78u64); xmm1 = crate::simd_ops::ru128(y_addr); ymm1_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rdx.wrapping_add(rax).wrapping_add(0x78u64); crate::simd_ops::wu128(y_addr, xmm1); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm1_hi); xmm0 = ((crate::simd_ops::ru64(r8.wrapping_add(rax).wrapping_add(0x98u64))) as u64) as u128; ymm0_hi = 0; crate::simd_ops::wu64(rdx.wrapping_add(rax).wrapping_add(0x98u64), ((xmm0 as u64)) as u64); fl_l = rbp; fl_r = r10; if ((((fl_l as u64)) as i64) < (((fl_r as u64)) as i64)) { bb = 8; continue; } bb = 10; continue; }
            10 => { rcx = ((r10.wrapping_add(r10.wrapping_mul(4))) as u64) as u64; rdx = rdi; rcx = ((rcx).wrapping_shl(5u32)) as u64; rcx = ((rcx).wrapping_add(rax)) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1412f10d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1412f0cc0;
    let (mut rax, mut r10, mut r11, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0; let mut ymm1_hi: u128 = 0; let mut xmm6: u128 = 0; let mut xmm7: u128 = 0; let mut xmm8: u128 = 0; let mut xmm9: u128 = 0; let mut xmm10: u128 = 0; let mut xmm11: u128 = 0; let mut xmm12: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 27>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = rsp; crate::simd_ops::wu64(rax.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rax.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rax.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rax.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0xd0u64)) as u64; let y_addr: u64 = rcx.wrapping_add(0x58u64); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); r8 = (crate::simd_ops::ru32(rcx)) as u32 as u64; r9 = (crate::simd_ops::ru32(rcx.wrapping_add(0x4u64))) as u32 as u64; r10 = (crate::simd_ops::ru32(rcx.wrapping_add(0x8u64))) as u32 as u64; rdi = (crate::simd_ops::ru32(rcx.wrapping_add(0x28u64))) as u32 as u64; rsi = (crate::simd_ops::ru32(rcx.wrapping_add(0x34u64))) as u32 as u64; rbp = (crate::simd_ops::ru32(rcx.wrapping_add(0x40u64))) as u32 as u64; r14 = (crate::simd_ops::ru32(rcx.wrapping_add(0x4cu64))) as u32 as u64; r11 = (crate::simd_ops::ru32(rcx.wrapping_add(0x50u64))) as u32 as u64; rbx = (crate::simd_ops::ru32(rcx.wrapping_add(0x54u64))) as u32 as u64; crate::simd_ops::wu128(rax.wrapping_sub(0x18u64), (xmm6) as u128); xmm6 = ((crate::simd_ops::ru64(rcx.wrapping_add(0x20u64))) as u64) as u128; crate::simd_ops::wu128(rax.wrapping_sub(0x28u64), (xmm7) as u128); xmm7 = ((crate::simd_ops::ru64(rcx.wrapping_add(0x2cu64))) as u64) as u128; crate::simd_ops::wu128(rax.wrapping_sub(0x38u64), (xmm8) as u128); xmm8 = ((crate::simd_ops::ru64(rcx.wrapping_add(0x38u64))) as u64) as u128; crate::simd_ops::wu128(rax.wrapping_sub(0x48u64), (xmm9) as u128); xmm9 = ((crate::simd_ops::ru64(rcx.wrapping_add(0x44u64))) as u64) as u128; crate::simd_ops::wu128(rax.wrapping_sub(0x58u64), (xmm10) as u128); xmm10 = ((crate::simd_ops::ru64(rcx.wrapping_add(0x98u64))) as u64) as u128; crate::simd_ops::wu128(rax.wrapping_sub(0x68u64), (xmm11) as u128); xmm11 = ((crate::simd_ops::ru32(rcx.wrapping_add(0xcu64))) as u32) as u128; crate::simd_ops::wu128(rax.wrapping_sub(0x78u64), (xmm12) as u128); xmm12 = (crate::simd_ops::ru128(rcx.wrapping_add(0x10u64))) as u128; let y_addr: u64 = rsp.wrapping_add(0x20u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); let y_addr: u64 = rcx.wrapping_add(0x78u64); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rsp.wrapping_add(0x40u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); let y_addr: u64 = rsp.wrapping_add(0x20u64); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rsp.wrapping_add(0x40u64); xmm1 = crate::simd_ops::ru128(y_addr); ymm1_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); crate::simd_ops::wu32(rdx.wrapping_add(0xcu64), ((xmm11 as u32)) as u32); crate::simd_ops::wu128(rdx.wrapping_add(0x10u64), (xmm12) as u128); crate::simd_ops::wu32(rdx, ((r8 as u32)) as u32); crate::simd_ops::wu32(rdx.wrapping_add(0x4u64), ((r9 as u32)) as u32); crate::simd_ops::wu32(rdx.wrapping_add(0x8u64), ((r10 as u32)) as u32); crate::simd_ops::wu64(rdx.wrapping_add(0x20u64), ((xmm6 as u64)) as u64); crate::simd_ops::wu64(rdx.wrapping_add(0x2cu64), ((xmm7 as u64)) as u64); crate::simd_ops::wu64(rdx.wrapping_add(0x38u64), ((xmm8 as u64)) as u64); crate::simd_ops::wu64(rdx.wrapping_add(0x44u64), ((xmm9 as u64)) as u64); crate::simd_ops::wu32(rdx.wrapping_add(0x28u64), ((rdi as u32)) as u32); crate::simd_ops::wu32(rdx.wrapping_add(0x34u64), ((rsi as u32)) as u32); crate::simd_ops::wu32(rdx.wrapping_add(0x40u64), ((rbp as u32)) as u32); crate::simd_ops::wu32(rdx.wrapping_add(0x4cu64), ((r14 as u32)) as u32); let y_addr: u64 = rdx.wrapping_add(0x58u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); let y_addr: u64 = rdx.wrapping_add(0x78u64); crate::simd_ops::wu128(y_addr, xmm1); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm1_hi); crate::simd_ops::wu64(rdx.wrapping_add(0x98u64), ((xmm10 as u64)) as u64); crate::simd_ops::wu32(rdx.wrapping_add(0x50u64), ((r11 as u32)) as u32); crate::simd_ops::wu32(rdx.wrapping_add(0x54u64), ((rbx as u32)) as u32); r11 = rsp.wrapping_add(0xd0); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x10u64))) as u64; rbp = (crate::simd_ops::ru64(r11.wrapping_add(0x18u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x20u64))) as u64; rdi = (crate::simd_ops::ru64(r11.wrapping_add(0x28u64))) as u64; xmm6 = (crate::simd_ops::ru128(r11.wrapping_sub(0x10u64))) as u128; xmm7 = (crate::simd_ops::ru128(r11.wrapping_sub(0x20u64))) as u128; xmm8 = (crate::simd_ops::ru128(r11.wrapping_sub(0x30u64))) as u128; xmm9 = (crate::simd_ops::ru128(r11.wrapping_sub(0x40u64))) as u128; xmm10 = (crate::simd_ops::ru128(r11.wrapping_sub(0x50u64))) as u128; xmm11 = (crate::simd_ops::ru128(r11.wrapping_sub(0x60u64))) as u128; xmm12 = (crate::simd_ops::ru128(r11.wrapping_sub(0x70u64))) as u128; rsp = r11; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1412f1220(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1412f0cc0;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0; let mut ymm1_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rsi = r8.wrapping_sub(0x1); rbx = r9; rsi = ((((rsi) as i64).wrapping_shr(1u32) as u64)) as u64; r14 = r8; r11 = rdx; rdi = rcx; r15 = rdx; r10 = rdx; fl_l = rdx; fl_r = rsi; if ((((fl_l as u64)) as i64) >= (((fl_r as u64)) as i64)) { bb = 6; continue; } bb = 1; continue; }
            1 => { bb = 2; continue; }
            2 => { r8 = (crate::simd_ops::ru64(rdi)) as u64; r10 = ((r10).wrapping_add(r10)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = ((r10.wrapping_add(r10.wrapping_mul(4))) as u64) as u64; rcx = ((rcx).wrapping_shl(5u32)) as u64; rcx = ((rcx).wrapping_add(r8)) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x194u64))) as u32 as u64; r9 = (crate::simd_ops::ru32(rcx.wrapping_add(0xf4u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((r9 as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rdx = (rdx & !0xffu64) | (((if ((((fl_l as u32)) as i32) <= (((fl_r as u32)) as i32)) { 1u8 } else { 0u8 }) as u8 as u64)); bb = 5; continue; }
            4 => { rax = (crate::simd_ops::ru32(rcx.wrapping_add(0xa8u64))) as u32 as u64; fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(0x148u64))) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; rdx = (rdx & !0xffu64) | (((if ((((fl_l as u32)) as i32) >= (((fl_r as u32)) as i32)) { 1u8 } else { 0u8 }) as u8 as u64)); bb = 5; continue; }
            5 => { rdx = ((rdx).wrapping_add(1)) as u64; rcx = ((r11.wrapping_add(r11.wrapping_mul(4))) as u64) as u64; rcx = ((rcx).wrapping_shl(5u32)) as u64; r10 = ((r10).wrapping_add(rdx)) as u64; r11 = r10; rdx = ((r10.wrapping_add(r10.wrapping_mul(4))) as u64) as u64; rdx = ((rdx).wrapping_shl(5u32)) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(r8))) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(r8), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdx.wrapping_add(r8).wrapping_add(0x4u64))) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(r8).wrapping_add(0x4u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdx.wrapping_add(r8).wrapping_add(0x8u64))) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(r8).wrapping_add(0x8u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdx.wrapping_add(r8).wrapping_add(0xcu64))) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(r8).wrapping_add(0xcu64), ((rax as u32)) as u32); xmm0 = (crate::simd_ops::ru128(rdx.wrapping_add(r8).wrapping_add(0x10u64))) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rcx.wrapping_add(r8).wrapping_add(0x10u64), (xmm0) as u128); xmm0 = ((crate::simd_ops::ru64(rdx.wrapping_add(r8).wrapping_add(0x20u64))) as u64) as u128; ymm0_hi = 0; crate::simd_ops::wu64(rcx.wrapping_add(r8).wrapping_add(0x20u64), ((xmm0 as u64)) as u64); rax = (crate::simd_ops::ru32(rdx.wrapping_add(r8).wrapping_add(0x28u64))) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(r8).wrapping_add(0x28u64), ((rax as u32)) as u32); xmm0 = ((crate::simd_ops::ru64(rdx.wrapping_add(r8).wrapping_add(0x2cu64))) as u64) as u128; ymm0_hi = 0; crate::simd_ops::wu64(rcx.wrapping_add(r8).wrapping_add(0x2cu64), ((xmm0 as u64)) as u64); rax = (crate::simd_ops::ru32(rdx.wrapping_add(r8).wrapping_add(0x34u64))) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(r8).wrapping_add(0x34u64), ((rax as u32)) as u32); xmm0 = ((crate::simd_ops::ru64(rdx.wrapping_add(r8).wrapping_add(0x38u64))) as u64) as u128; ymm0_hi = 0; crate::simd_ops::wu64(rcx.wrapping_add(r8).wrapping_add(0x38u64), ((xmm0 as u64)) as u64); rax = (crate::simd_ops::ru32(rdx.wrapping_add(r8).wrapping_add(0x40u64))) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(r8).wrapping_add(0x40u64), ((rax as u32)) as u32); xmm0 = ((crate::simd_ops::ru64(rdx.wrapping_add(r8).wrapping_add(0x44u64))) as u64) as u128; ymm0_hi = 0; crate::simd_ops::wu64(rcx.wrapping_add(r8).wrapping_add(0x44u64), ((xmm0 as u64)) as u64); rax = (crate::simd_ops::ru32(rdx.wrapping_add(r8).wrapping_add(0x4cu64))) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(r8).wrapping_add(0x4cu64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdx.wrapping_add(r8).wrapping_add(0x50u64))) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(r8).wrapping_add(0x50u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdx.wrapping_add(r8).wrapping_add(0x54u64))) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(r8).wrapping_add(0x54u64), ((rax as u32)) as u32); let y_addr: u64 = rdx.wrapping_add(r8).wrapping_add(0x58u64); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rcx.wrapping_add(r8).wrapping_add(0x58u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); xmm1 = ((crate::simd_ops::ru64(rdx.wrapping_add(r8).wrapping_add(0x78u64))) as u64) as u128; ymm1_hi = 0; crate::simd_ops::wu64(rcx.wrapping_add(r8).wrapping_add(0x78u64), ((xmm1 as u64)) as u64); xmm0 = (crate::simd_ops::ru128(rdx.wrapping_add(r8).wrapping_add(0x58u64))) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rcx.wrapping_add(r8).wrapping_add(0x58u64), (xmm0) as u128); let y_addr: u64 = rdx.wrapping_add(r8).wrapping_add(0x58u64); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rcx.wrapping_add(r8).wrapping_add(0x58u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); let y_addr: u64 = rdx.wrapping_add(r8).wrapping_add(0x58u64); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rcx.wrapping_add(r8).wrapping_add(0x58u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); let y_addr: u64 = rdx.wrapping_add(r8).wrapping_add(0x78u64); xmm1 = crate::simd_ops::ru128(y_addr); ymm1_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rcx.wrapping_add(r8).wrapping_add(0x78u64); crate::simd_ops::wu128(y_addr, xmm1); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm1_hi); xmm0 = ((crate::simd_ops::ru64(rdx.wrapping_add(r8).wrapping_add(0x98u64))) as u64) as u128; ymm0_hi = 0; crate::simd_ops::wu64(rcx.wrapping_add(r8).wrapping_add(0x98u64), ((xmm0 as u64)) as u64); fl_l = r10; fl_r = rsi; if ((((fl_l as u64)) as i64) < (((fl_r as u64)) as i64)) { bb = 2; continue; } bb = 6; continue; }
            6 => { fl_l = r10; fl_r = rsi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 9; continue; } bb = 7; continue; }
            7 => { fl_l = ((r14 as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 9; continue; } bb = 8; continue; }
            8 => { r8 = (crate::simd_ops::ru64(rdi)) as u64; rax = ((r14.wrapping_add(r14.wrapping_mul(4))) as u64) as u64; rcx = ((r11.wrapping_add(r11.wrapping_mul(4))) as u64) as u64; rax = ((rax).wrapping_shl(5u32)) as u64; rcx = ((rcx).wrapping_shl(5u32)) as u64; rcx = ((rcx).wrapping_add(r8)) as u64; rdx = r8.wrapping_sub(0xa0); rdx = ((rdx).wrapping_add(rax)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r11 = r14.wrapping_sub(0x1); bb = 9; continue; }
            9 => { rax = (crate::simd_ops::ru64(rdi)) as u64; r10 = r11.wrapping_sub(0x1); r10 = ((((r10) as i64).wrapping_shr(1u32) as u64)) as u64; fl_l = r15; fl_r = r11; if ((((fl_l as u64)) as i64) >= (((fl_r as u64)) as i64)) { bb = 16; continue; } bb = 10; continue; }
            10 => { bb = 11; continue; }
            11 => { r8 = ((r10.wrapping_add(r10.wrapping_mul(4))) as u64) as u64; r8 = ((r8).wrapping_shl(5u32)) as u64; rcx = (crate::simd_ops::ru32(r8.wrapping_add(rax).wrapping_add(0x54u64))) as u32 as u64; r9 = ((r8.wrapping_add(rax)) as u64) as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rbx.wrapping_add(0x54u64))) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 13; continue; } bb = 12; continue; }
            12 => { rcx = (rcx & !0xffu64) | (((if ((((fl_l as u32)) as i32) > (((fl_r as u32)) as i32)) { 1u8 } else { 0u8 }) as u8 as u64)); bb = 14; continue; }
            13 => { rcx = (crate::simd_ops::ru32(rbx.wrapping_add(0x8u64))) as u32 as u64; fl_l = (crate::simd_ops::ru32(r9.wrapping_add(0x8u64))) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; rcx = (rcx & !0xffu64) | (((if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { 1u8 } else { 0u8 }) as u8 as u64)); bb = 14; continue; }
            14 => { fl_l = ((rcx as u8)) as u8 as u64; fl_r = ((rcx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 16; continue; } bb = 15; continue; }
            15 => { rcx = (crate::simd_ops::ru32(r8.wrapping_add(rax))) as u32 as u64; rdx = ((r11.wrapping_add(r11.wrapping_mul(4))) as u64) as u64; rdx = ((rdx).wrapping_shl(5u32)) as u64; r11 = r10; r10 = ((r10).wrapping_sub(1)) as u64; r10 = ((((r10) as i64).wrapping_shr(1u32) as u64)) as u64; crate::simd_ops::wu32(rdx.wrapping_add(rax), ((rcx as u32)) as u32); rcx = (crate::simd_ops::ru32(r8.wrapping_add(rax).wrapping_add(0x4u64))) as u32 as u64; crate::simd_ops::wu32(rdx.wrapping_add(rax).wrapping_add(0x4u64), ((rcx as u32)) as u32); rcx = (crate::simd_ops::ru32(r9.wrapping_add(0x8u64))) as u32 as u64; crate::simd_ops::wu32(rdx.wrapping_add(rax).wrapping_add(0x8u64), ((rcx as u32)) as u32); rcx = (crate::simd_ops::ru32(r8.wrapping_add(rax).wrapping_add(0xcu64))) as u32 as u64; crate::simd_ops::wu32(rdx.wrapping_add(rax).wrapping_add(0xcu64), ((rcx as u32)) as u32); xmm0 = (crate::simd_ops::ru128(r8.wrapping_add(rax).wrapping_add(0x10u64))) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rdx.wrapping_add(rax).wrapping_add(0x10u64), (xmm0) as u128); xmm0 = ((crate::simd_ops::ru64(r8.wrapping_add(rax).wrapping_add(0x20u64))) as u64) as u128; ymm0_hi = 0; crate::simd_ops::wu64(rdx.wrapping_add(rax).wrapping_add(0x20u64), ((xmm0 as u64)) as u64); rcx = (crate::simd_ops::ru32(r8.wrapping_add(rax).wrapping_add(0x28u64))) as u32 as u64; crate::simd_ops::wu32(rdx.wrapping_add(rax).wrapping_add(0x28u64), ((rcx as u32)) as u32); xmm0 = ((crate::simd_ops::ru64(r8.wrapping_add(rax).wrapping_add(0x2cu64))) as u64) as u128; ymm0_hi = 0; crate::simd_ops::wu64(rdx.wrapping_add(rax).wrapping_add(0x2cu64), ((xmm0 as u64)) as u64); rcx = (crate::simd_ops::ru32(r8.wrapping_add(rax).wrapping_add(0x34u64))) as u32 as u64; crate::simd_ops::wu32(rdx.wrapping_add(rax).wrapping_add(0x34u64), ((rcx as u32)) as u32); xmm0 = ((crate::simd_ops::ru64(r8.wrapping_add(rax).wrapping_add(0x38u64))) as u64) as u128; ymm0_hi = 0; crate::simd_ops::wu64(rdx.wrapping_add(rax).wrapping_add(0x38u64), ((xmm0 as u64)) as u64); rcx = (crate::simd_ops::ru32(r8.wrapping_add(rax).wrapping_add(0x40u64))) as u32 as u64; crate::simd_ops::wu32(rdx.wrapping_add(rax).wrapping_add(0x40u64), ((rcx as u32)) as u32); xmm0 = ((crate::simd_ops::ru64(r8.wrapping_add(rax).wrapping_add(0x44u64))) as u64) as u128; ymm0_hi = 0; crate::simd_ops::wu64(rdx.wrapping_add(rax).wrapping_add(0x44u64), ((xmm0 as u64)) as u64); rcx = (crate::simd_ops::ru32(r8.wrapping_add(rax).wrapping_add(0x4cu64))) as u32 as u64; crate::simd_ops::wu32(rdx.wrapping_add(rax).wrapping_add(0x4cu64), ((rcx as u32)) as u32); rcx = (crate::simd_ops::ru32(r8.wrapping_add(rax).wrapping_add(0x50u64))) as u32 as u64; crate::simd_ops::wu32(rdx.wrapping_add(rax).wrapping_add(0x50u64), ((rcx as u32)) as u32); rcx = (crate::simd_ops::ru32(r8.wrapping_add(rax).wrapping_add(0x54u64))) as u32 as u64; crate::simd_ops::wu32(rdx.wrapping_add(rax).wrapping_add(0x54u64), ((rcx as u32)) as u32); let y_addr: u64 = r8.wrapping_add(rax).wrapping_add(0x58u64); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rdx.wrapping_add(rax).wrapping_add(0x58u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); xmm1 = ((crate::simd_ops::ru64(r8.wrapping_add(rax).wrapping_add(0x78u64))) as u64) as u128; ymm1_hi = 0; crate::simd_ops::wu64(rdx.wrapping_add(rax).wrapping_add(0x78u64), ((xmm1 as u64)) as u64); xmm0 = (crate::simd_ops::ru128(r8.wrapping_add(rax).wrapping_add(0x58u64))) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rdx.wrapping_add(rax).wrapping_add(0x58u64), (xmm0) as u128); let y_addr: u64 = r8.wrapping_add(rax).wrapping_add(0x58u64); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rdx.wrapping_add(rax).wrapping_add(0x58u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); let y_addr: u64 = r8.wrapping_add(rax).wrapping_add(0x58u64); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rdx.wrapping_add(rax).wrapping_add(0x58u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); let y_addr: u64 = r8.wrapping_add(rax).wrapping_add(0x78u64); xmm1 = crate::simd_ops::ru128(y_addr); ymm1_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rdx.wrapping_add(rax).wrapping_add(0x78u64); crate::simd_ops::wu128(y_addr, xmm1); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm1_hi); xmm0 = ((crate::simd_ops::ru64(r8.wrapping_add(rax).wrapping_add(0x98u64))) as u64) as u128; ymm0_hi = 0; crate::simd_ops::wu64(rdx.wrapping_add(rax).wrapping_add(0x98u64), ((xmm0 as u64)) as u64); fl_l = r15; fl_r = r11; if ((((fl_l as u64)) as i64) < (((fl_r as u64)) as i64)) { bb = 11; continue; } bb = 16; continue; }
            16 => { rcx = ((r11.wrapping_add(r11.wrapping_mul(4))) as u64) as u64; rdx = rbx; rcx = ((rcx).wrapping_shl(5u32)) as u64; rcx = ((rcx).wrapping_add(rax)) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1412f1590(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1412f1980;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = r8; rsi = rdx; rax = ((rax).wrapping_sub(rdx)) as u64; rdi = r8; rax = ((((rax) as i64).wrapping_shr(3u32) as u64)) as u64; r14 = rcx; r8 = ((r8).wrapping_add(0xfffffffffffffffcu64)) as u64; rcx = rsi; rbx = ((rdx.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rdx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = rbx.wrapping_add(0x4); fl_l = rsi; fl_r = rbx; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 5; continue; } bb = 1; continue; }
            1 => { xmm1 = ((crate::simd_ops::ru32(rbx)) as u32) as u128; bb = 2; continue; }
            2 => { rax = rbx.wrapping_sub(0x4); xmm0 = ((crate::simd_ops::ru32(rax)) as u32) as u128; fl_l = ((xmm1 as u32)) as u32 as u64; fl_r = ((xmm0 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 5; continue; } bb = 3; continue; }
            3 => { fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = ((xmm1 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 5; continue; } bb = 4; continue; }
            4 => { rbx = rax; xmm1 = (xmm0) as u128; fl_l = rsi; fl_r = rax; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 2; continue; } bb = 5; continue; }
            5 => { fl_l = rdx; fl_r = rdi; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 10; continue; } bb = 6; continue; }
            6 => { xmm1 = ((crate::simd_ops::ru32(rbx)) as u32) as u128; bb = 7; continue; }
            7 => { xmm0 = ((crate::simd_ops::ru32(rdx)) as u32) as u128; fl_l = ((xmm1 as u32)) as u32 as u64; fl_r = ((xmm0 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 10; continue; } bb = 8; continue; }
            8 => { fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = ((xmm1 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 10; continue; } bb = 9; continue; }
            9 => { rdx = ((rdx).wrapping_add(0x4u64)) as u64; fl_l = rdx; fl_r = rdi; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 7; continue; } bb = 10; continue; }
            10 => { rcx = rdx; r9 = rbx; bb = 11; continue; }
            11 => { fl_l = rcx; fl_r = rdi; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 42; continue; } bb = 12; continue; }
            12 => { rax = rdi; rax = ((rax).wrapping_sub(rcx)) as u64; rax = ((rax).wrapping_add(0x3u64)) as u64; rax = (((rax) & (0xfffffffffffffffcu64))) as u64; fl_l = rax; fl_r = (0x10u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 35; continue; } bb = 13; continue; }
            13 => { r8 = rcx.wrapping_add(0x8); bb = 14; continue; }
            14 => { xmm0 = ((crate::simd_ops::ru32(rbx)) as u32) as u128; xmm1 = ((crate::simd_ops::ru32(rcx)) as u32) as u128; fl_l = ((xmm1 as u32)) as u32 as u64; fl_r = ((xmm0 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 19; continue; } bb = 15; continue; }
            15 => { fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = ((xmm1 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 42; continue; } bb = 16; continue; }
            16 => { fl_l = rdx; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 18; continue; } bb = 17; continue; }
            17 => { xmm0 = ((crate::simd_ops::ru32(rdx)) as u32) as u128; crate::simd_ops::wu32(rdx, ((xmm1 as u32)) as u32); crate::simd_ops::wu32(rcx, ((xmm0 as u32)) as u32); bb = 18; continue; }
            18 => { fl_l = ((rdx).wrapping_add(0x4u64)) as u64; fl_r = fl_l; rdx = ((fl_l as u64)) as u64; bb = 19; continue; }
            19 => { xmm0 = ((crate::simd_ops::ru32(rbx)) as u32) as u128; rax = r8.wrapping_sub(0x4); xmm1 = ((crate::simd_ops::ru32(rax)) as u32) as u128; fl_l = ((xmm1 as u32)) as u32 as u64; fl_r = ((xmm0 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 24; continue; } bb = 20; continue; }
            20 => { fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = ((xmm1 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 55; continue; } bb = 21; continue; }
            21 => { fl_l = rdx; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 23; continue; } bb = 22; continue; }
            22 => { xmm0 = ((crate::simd_ops::ru32(rdx)) as u32) as u128; crate::simd_ops::wu32(rdx, ((xmm1 as u32)) as u32); crate::simd_ops::wu32(rax, ((xmm0 as u32)) as u32); bb = 23; continue; }
            23 => { fl_l = ((rdx).wrapping_add(0x4u64)) as u64; fl_r = fl_l; rdx = ((fl_l as u64)) as u64; bb = 24; continue; }
            24 => { xmm0 = ((crate::simd_ops::ru32(rbx)) as u32) as u128; xmm1 = ((crate::simd_ops::ru32(r8)) as u32) as u128; fl_l = ((xmm1 as u32)) as u32 as u64; fl_r = ((xmm0 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 29; continue; } bb = 25; continue; }
            25 => { fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = ((xmm1 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 56; continue; } bb = 26; continue; }
            26 => { fl_l = rdx; fl_r = r8; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 28; continue; } bb = 27; continue; }
            27 => { xmm0 = ((crate::simd_ops::ru32(rdx)) as u32) as u128; crate::simd_ops::wu32(rdx, ((xmm1 as u32)) as u32); crate::simd_ops::wu32(r8, ((xmm0 as u32)) as u32); bb = 28; continue; }
            28 => { fl_l = ((rdx).wrapping_add(0x4u64)) as u64; fl_r = fl_l; rdx = ((fl_l as u64)) as u64; bb = 29; continue; }
            29 => { xmm0 = ((crate::simd_ops::ru32(rbx)) as u32) as u128; rax = r8.wrapping_add(0x4); xmm1 = ((crate::simd_ops::ru32(rax)) as u32) as u128; fl_l = ((xmm1 as u32)) as u32 as u64; fl_r = ((xmm0 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 34; continue; } bb = 30; continue; }
            30 => { fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = ((xmm1 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 55; continue; } bb = 31; continue; }
            31 => { fl_l = rdx; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 33; continue; } bb = 32; continue; }
            32 => { xmm0 = ((crate::simd_ops::ru32(rdx)) as u32) as u128; crate::simd_ops::wu32(rdx, ((xmm1 as u32)) as u32); crate::simd_ops::wu32(rax, ((xmm0 as u32)) as u32); bb = 33; continue; }
            33 => { fl_l = ((rdx).wrapping_add(0x4u64)) as u64; fl_r = fl_l; rdx = ((fl_l as u64)) as u64; bb = 34; continue; }
            34 => { rcx = ((rcx).wrapping_add(0x10u64)) as u64; rax = rdi.wrapping_sub(0xc); r8 = ((r8).wrapping_add(0x10u64)) as u64; fl_l = rcx; fl_r = rax; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 14; continue; } bb = 35; continue; }
            35 => { fl_l = rcx; fl_r = rdi; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 42; continue; } bb = 36; continue; }
            36 => { xmm0 = ((crate::simd_ops::ru32(rbx)) as u32) as u128; xmm1 = ((crate::simd_ops::ru32(rcx)) as u32) as u128; fl_l = ((xmm1 as u32)) as u32 as u64; fl_r = ((xmm0 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 41; continue; } bb = 37; continue; }
            37 => { fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = ((xmm1 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 42; continue; } bb = 38; continue; }
            38 => { fl_l = rdx; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 40; continue; } bb = 39; continue; }
            39 => { xmm0 = ((crate::simd_ops::ru32(rdx)) as u32) as u128; crate::simd_ops::wu32(rdx, ((xmm1 as u32)) as u32); crate::simd_ops::wu32(rcx, ((xmm0 as u32)) as u32); bb = 40; continue; }
            40 => { fl_l = ((rdx).wrapping_add(0x4u64)) as u64; fl_r = fl_l; rdx = ((fl_l as u64)) as u64; bb = 41; continue; }
            41 => { rcx = ((rcx).wrapping_add(0x4u64)) as u64; fl_l = rcx; fl_r = rdi; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 36; continue; } bb = 42; continue; }
            42 => { fl_l = r9; fl_r = rsi; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 50; continue; } bb = 43; continue; }
            43 => { rax = r9.wrapping_sub(0x4); bb = 44; continue; }
            44 => { xmm1 = ((crate::simd_ops::ru32(rax)) as u32) as u128; xmm0 = ((crate::simd_ops::ru32(rbx)) as u32) as u128; fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = ((xmm1 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 48; continue; } bb = 45; continue; }
            45 => { fl_l = ((xmm1 as u32)) as u32 as u64; fl_r = ((xmm0 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 49; continue; } bb = 46; continue; }
            46 => { rbx = ((rbx).wrapping_sub(0x4u64)) as u64; fl_l = rbx; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 48; continue; } bb = 47; continue; }
            47 => { xmm0 = ((crate::simd_ops::ru32(rbx)) as u32) as u128; crate::simd_ops::wu32(rbx, ((xmm1 as u32)) as u32); crate::simd_ops::wu32(rax, ((xmm0 as u32)) as u32); bb = 48; continue; }
            48 => { r9 = ((r9).wrapping_sub(0x4u64)) as u64; rax = ((rax).wrapping_sub(0x4u64)) as u64; fl_l = rsi; fl_r = r9; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 44; continue; } bb = 49; continue; }
            49 => { fl_l = r9; fl_r = rsi; bb = 50; continue; }
            50 => { if (((fl_l as u64)) != ((fl_r as u64))) { bb = 57; continue; } bb = 51; continue; }
            51 => { fl_l = rcx; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 62; continue; } bb = 52; continue; }
            52 => { fl_l = rdx; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 54; continue; } bb = 53; continue; }
            53 => { xmm0 = ((crate::simd_ops::ru32(rbx)) as u32) as u128; rax = (crate::simd_ops::ru32(rdx)) as u32 as u64; crate::simd_ops::wu32(rbx, ((rax as u32)) as u32); crate::simd_ops::wu32(rdx, ((xmm0 as u32)) as u32); bb = 54; continue; }
            54 => { xmm0 = ((crate::simd_ops::ru32(rbx)) as u32) as u128; rax = (crate::simd_ops::ru32(rcx)) as u32 as u64; rdx = ((rdx).wrapping_add(0x4u64)) as u64; crate::simd_ops::wu32(rbx, ((rax as u32)) as u32); rbx = ((rbx).wrapping_add(0x4u64)) as u64; crate::simd_ops::wu32(rcx, ((xmm0 as u32)) as u32); fl_l = ((rcx).wrapping_add(0x4u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; bb = 11; continue; }
            55 => { rcx = rax; bb = 42; continue; }
            56 => { rcx = r8; bb = 42; continue; }
            57 => { r9 = ((r9).wrapping_add(0xfffffffffffffffcu64)) as u64; fl_l = rcx; fl_r = rdi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 61; continue; } bb = 58; continue; }
            58 => { rbx = ((rbx).wrapping_sub(0x4u64)) as u64; fl_l = r9; fl_r = rbx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 60; continue; } bb = 59; continue; }
            59 => { xmm0 = ((crate::simd_ops::ru32(r9)) as u32) as u128; rax = (crate::simd_ops::ru32(rbx)) as u32 as u64; crate::simd_ops::wu32(r9, ((rax as u32)) as u32); crate::simd_ops::wu32(rbx, ((xmm0 as u32)) as u32); bb = 60; continue; }
            60 => { rax = (crate::simd_ops::ru32(rdx.wrapping_sub(0x4u64))) as u32 as u64; fl_l = rdx; fl_r = (0x4u64) as u64; rdx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; xmm0 = ((crate::simd_ops::ru32(rbx)) as u32) as u128; crate::simd_ops::wu32(rbx, ((rax as u32)) as u32); crate::simd_ops::wu32(rdx, ((xmm0 as u32)) as u32); bb = 11; continue; }
            61 => { xmm0 = ((crate::simd_ops::ru32(rcx)) as u32) as u128; rax = (crate::simd_ops::ru32(r9)) as u32 as u64; crate::simd_ops::wu32(rcx, ((rax as u32)) as u32); fl_l = ((rcx).wrapping_add(0x4u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; crate::simd_ops::wu32(r9, ((xmm0 as u32)) as u32); bb = 11; continue; }
            62 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rax = r14; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; crate::simd_ops::wu64(r14, rbx); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu64(r14.wrapping_add(0x8u64), rdx); rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1412f18e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x54; const P1: u64 = 0x8; const P2: u64 = 0x1412f10d0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rdx; rsi = rcx; rdx = (crate::simd_ops::ru64(rcx)) as u64; rdi = r8; rcx = (crate::simd_ops::ru64(rbx)) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = (rax & !0xffu64) | (((if ((((fl_l as u32)) as i32) > (((fl_r as u32)) as i32)) { 1u8 } else { 0u8 }) as u8 as u64)); bb = 3; continue; }
            2 => { rax = (crate::simd_ops::ru32(rdx.wrapping_add(P1))) as u32 as u64; fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(P1))) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; rax = (rax & !0xffu64) | (((if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { 1u8 } else { 0u8 }) as u8 as u64)); bb = 3; continue; }
            3 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 5; continue; }
            5 => { rcx = (crate::simd_ops::ru64(rdi)) as u64; rdx = (crate::simd_ops::ru64(rbx)) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 7; continue; } bb = 6; continue; }
            6 => { rax = (rax & !0xffu64) | (((if ((((fl_l as u32)) as i32) > (((fl_r as u32)) as i32)) { 1u8 } else { 0u8 }) as u8 as u64)); bb = 8; continue; }
            7 => { rax = (crate::simd_ops::ru32(rdx.wrapping_add(P1))) as u32 as u64; fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(P1))) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; rax = (rax & !0xffu64) | (((if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { 1u8 } else { 0u8 }) as u8 as u64)); bb = 8; continue; }
            8 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 14; continue; } bb = 9; continue; }
            9 => { rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rbx)) as u64; rdx = (crate::simd_ops::ru64(rsi)) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 11; continue; } bb = 10; continue; }
            10 => { rax = (rax & !0xffu64) | (((if ((((fl_l as u32)) as i32) > (((fl_r as u32)) as i32)) { 1u8 } else { 0u8 }) as u8 as u64)); bb = 12; continue; }
            11 => { rax = (crate::simd_ops::ru32(rdx.wrapping_add(P1))) as u32 as u64; fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(P1))) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; rax = (rax & !0xffu64) | (((if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { 1u8 } else { 0u8 }) as u8 as u64)); bb = 12; continue; }
            12 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 14; continue; } bb = 13; continue; }
            13 => { rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 14; continue; }
            14 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1412f1980(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { xmm1 = ((crate::simd_ops::ru32(rcx)) as u32) as u128; rax = r8; r9 = r8; rax = ((rax).wrapping_sub(rcx)) as u64; rax = ((((rax) as i64).wrapping_shr(2u32) as u64)) as u64; fl_l = rax; fl_r = (0x28u64) as u64; if ((((fl_l as u64)) as i64) <= (((fl_r as u64)) as i64)) { bb = 21; continue; } bb = 1; continue; }
            1 => { rax = ((rax).wrapping_add(1)) as u64; rax = ((((rax) as i64).wrapping_shr(3u32) as u64)) as u64; r11 = ((rax.wrapping_mul(8)) as u64) as u64; rax = ((rax.wrapping_mul(4)) as u64) as u64; xmm0 = ((crate::simd_ops::ru32(rax.wrapping_add(rcx))) as u32) as u128; fl_l = ((xmm1 as u32)) as u32 as u64; fl_r = ((xmm0 as u32)) as u32 as u64; if !(f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 3; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::wu32(rax.wrapping_add(rcx), ((xmm1 as u32)) as u32); crate::simd_ops::wu32(rcx, ((xmm0 as u32)) as u32); bb = 3; continue; }
            3 => { xmm0 = ((crate::simd_ops::ru32(r11.wrapping_add(rcx))) as u32) as u128; xmm1 = ((crate::simd_ops::ru32(rax.wrapping_add(rcx))) as u32) as u128; fl_l = ((xmm1 as u32)) as u32 as u64; fl_r = ((xmm0 as u32)) as u32 as u64; if !(f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 6; continue; } bb = 4; continue; }
            4 => { crate::simd_ops::wu32(r11.wrapping_add(rcx), ((xmm1 as u32)) as u32); crate::simd_ops::wu32(rax.wrapping_add(rcx), ((xmm0 as u32)) as u32); xmm2 = ((crate::simd_ops::ru32(rcx)) as u32) as u128; fl_l = ((xmm2 as u32)) as u32 as u64; fl_r = ((xmm0 as u32)) as u32 as u64; if !(f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 6; continue; } bb = 5; continue; }
            5 => { crate::simd_ops::wu32(rax.wrapping_add(rcx), ((xmm2 as u32)) as u32); crate::simd_ops::wu32(rcx, ((xmm0 as u32)) as u32); bb = 6; continue; }
            6 => { xmm0 = ((crate::simd_ops::ru32(rdx)) as u32) as u128; r8 = rdx; r8 = ((r8).wrapping_sub(rax)) as u64; xmm1 = ((crate::simd_ops::ru32(r8)) as u32) as u128; fl_l = ((xmm1 as u32)) as u32 as u64; fl_r = ((xmm0 as u32)) as u32 as u64; if !(f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 8; continue; } bb = 7; continue; }
            7 => { crate::simd_ops::wu32(rdx, ((xmm1 as u32)) as u32); crate::simd_ops::wu32(r8, ((xmm0 as u32)) as u32); bb = 8; continue; }
            8 => { xmm0 = ((crate::simd_ops::ru32(rax.wrapping_add(rdx))) as u32) as u128; xmm1 = ((crate::simd_ops::ru32(rdx)) as u32) as u128; fl_l = ((xmm1 as u32)) as u32 as u64; fl_r = ((xmm0 as u32)) as u32 as u64; if !(f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 11; continue; } bb = 9; continue; }
            9 => { crate::simd_ops::wu32(rax.wrapping_add(rdx), ((xmm1 as u32)) as u32); crate::simd_ops::wu32(rdx, ((xmm0 as u32)) as u32); xmm2 = ((crate::simd_ops::ru32(r8)) as u32) as u128; fl_l = ((xmm2 as u32)) as u32 as u64; fl_r = ((xmm0 as u32)) as u32 as u64; if !(f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 11; continue; } bb = 10; continue; }
            10 => { crate::simd_ops::wu32(rdx, ((xmm2 as u32)) as u32); crate::simd_ops::wu32(r8, ((xmm0 as u32)) as u32); bb = 11; continue; }
            11 => { r8 = r9; r10 = r9; r8 = ((r8).wrapping_sub(rax)) as u64; r10 = ((r10).wrapping_sub(r11)) as u64; xmm0 = ((crate::simd_ops::ru32(r8)) as u32) as u128; xmm1 = ((crate::simd_ops::ru32(r10)) as u32) as u128; fl_l = ((xmm1 as u32)) as u32 as u64; fl_r = ((xmm0 as u32)) as u32 as u64; if !(f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 13; continue; } bb = 12; continue; }
            12 => { crate::simd_ops::wu32(r8, ((xmm1 as u32)) as u32); crate::simd_ops::wu32(r10, ((xmm0 as u32)) as u32); bb = 13; continue; }
            13 => { xmm0 = ((crate::simd_ops::ru32(r9)) as u32) as u128; xmm1 = ((crate::simd_ops::ru32(r8)) as u32) as u128; fl_l = ((xmm1 as u32)) as u32 as u64; fl_r = ((xmm0 as u32)) as u32 as u64; if !(f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 16; continue; } bb = 14; continue; }
            14 => { crate::simd_ops::wu32(r9, ((xmm1 as u32)) as u32); crate::simd_ops::wu32(r8, ((xmm0 as u32)) as u32); xmm2 = ((crate::simd_ops::ru32(r10)) as u32) as u128; fl_l = ((xmm2 as u32)) as u32 as u64; fl_r = ((xmm0 as u32)) as u32 as u64; if !(f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 16; continue; } bb = 15; continue; }
            15 => { crate::simd_ops::wu32(r8, ((xmm2 as u32)) as u32); crate::simd_ops::wu32(r10, ((xmm0 as u32)) as u32); bb = 16; continue; }
            16 => { xmm0 = ((crate::simd_ops::ru32(rdx)) as u32) as u128; xmm1 = ((crate::simd_ops::ru32(rax.wrapping_add(rcx))) as u32) as u128; fl_l = ((xmm1 as u32)) as u32 as u64; fl_r = ((xmm0 as u32)) as u32 as u64; if !(f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 18; continue; } bb = 17; continue; }
            17 => { crate::simd_ops::wu32(rdx, ((xmm1 as u32)) as u32); crate::simd_ops::wu32(rax.wrapping_add(rcx), ((xmm0 as u32)) as u32); bb = 18; continue; }
            18 => { xmm0 = ((crate::simd_ops::ru32(r8)) as u32) as u128; xmm1 = ((crate::simd_ops::ru32(rdx)) as u32) as u128; fl_l = ((xmm1 as u32)) as u32 as u64; fl_r = ((xmm0 as u32)) as u32 as u64; if !(f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 26; continue; } bb = 19; continue; }
            19 => { crate::simd_ops::wu32(r8, ((xmm1 as u32)) as u32); crate::simd_ops::wu32(rdx, ((xmm0 as u32)) as u32); xmm2 = ((crate::simd_ops::ru32(rax.wrapping_add(rcx))) as u32) as u128; fl_l = ((xmm2 as u32)) as u32 as u64; fl_r = ((xmm0 as u32)) as u32 as u64; if !(f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 26; continue; } bb = 20; continue; }
            20 => { crate::simd_ops::wu32(rdx, ((xmm2 as u32)) as u32); crate::simd_ops::wu32(rax.wrapping_add(rcx), ((xmm0 as u32)) as u32); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            21 => { xmm0 = ((crate::simd_ops::ru32(rdx)) as u32) as u128; fl_l = ((xmm1 as u32)) as u32 as u64; fl_r = ((xmm0 as u32)) as u32 as u64; if !(f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 23; continue; } bb = 22; continue; }
            22 => { crate::simd_ops::wu32(rdx, ((xmm1 as u32)) as u32); crate::simd_ops::wu32(rcx, ((xmm0 as u32)) as u32); bb = 23; continue; }
            23 => { xmm0 = ((crate::simd_ops::ru32(r8)) as u32) as u128; xmm1 = ((crate::simd_ops::ru32(rdx)) as u32) as u128; fl_l = ((xmm1 as u32)) as u32 as u64; fl_r = ((xmm0 as u32)) as u32 as u64; if !(f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 26; continue; } bb = 24; continue; }
            24 => { crate::simd_ops::wu32(r8, ((xmm1 as u32)) as u32); crate::simd_ops::wu32(rdx, ((xmm0 as u32)) as u32); xmm2 = ((crate::simd_ops::ru32(rcx)) as u32) as u128; fl_l = ((xmm2 as u32)) as u32 as u64; fl_r = ((xmm0 as u32)) as u32 as u64; if !(f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 26; continue; } bb = 25; continue; }
            25 => { crate::simd_ops::wu32(rdx, ((xmm2 as u32)) as u32); crate::simd_ops::wu32(rcx, ((xmm0 as u32)) as u32); bb = 26; continue; }
            26 => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1412f1b20(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xc; const P1: u64 = 0x10; const P2: u64 = 0x4; const P3: u64 = 0x8;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 0>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x28u64))) as u32 as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(P0), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x30u64))) as u32 as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(P1), ((rax as u32)) as u32);
    rax = rcx;
    crate::simd_ops::wu32(rcx, ((rdx as u32)) as u32);
    crate::simd_ops::wu32(rcx.wrapping_add(P2), ((r8 as u32)) as u32);
    crate::simd_ops::wu8(rcx.wrapping_add(P3), ((r9 as u8)) as u8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1412f1b40(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143bbdde0; const P1: u64 = 0x143bbdde4; const P2: u64 = 0x143bbddd8; const P3: u64 = 0x140fab9a0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = r8; rbx = rdx; r9 = (crate::simd_ops::ru32(P0)) as u32 as u64; rcx = r8; rcx = ((rcx).wrapping_sub(rdx)) as u64; rax = (0x6666666666666667u64) as u64; let prod = (rax as i64 as i128).wrapping_mul((rcx) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; rsi = rdx; rsi = ((((rsi) as i64).wrapping_shr(3u32) as u64)) as u64; rax = rsi; rax = ((rax).wrapping_shr(63u32)) as u64; rsi = ((rsi).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru32(P1)) as u32 as u64; fl_l = rsi; fl_r = rax; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 4; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu32(P0, (0x0u32) as u32); rdx = ((rax as u32)) as u32 as u64; rdx = ((rdx).wrapping_shr(1u32)) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; fl_l = rdx; fl_r = rsi; rdx = (if (((fl_l as u64)) < ((fl_r as u64))) { rsi } else { rdx }) as u64; rcx = P2; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru64(P2)) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 16; continue; } bb = 2; continue; }
            2 => { xmm0 = (crate::simd_ops::ru128(rbx)) as u128; crate::simd_ops::wu128(rcx, (xmm0) as u128); rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x10u64))) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(0x10u64), ((rax as u32)) as u32); rcx = rcx.wrapping_add(0x14); rbx = ((rbx).wrapping_add(0x14u64)) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 3; continue; }
            3 => { bb = 16; continue; }
            4 => { fl_l = rsi; fl_r = r9; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 13; continue; } bb = 5; continue; }
            5 => { rax = ((r9.wrapping_add(r9.wrapping_mul(4))) as u64) as u64; rax = ((rbx.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rdx = (crate::simd_ops::ru64(P2)) as u64; fl_l = r9; fl_r = r9; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 6; continue; }
            6 => { xmm0 = (crate::simd_ops::ru128(rbx)) as u128; crate::simd_ops::wu128(rdx, (xmm0) as u128); rcx = (crate::simd_ops::ru32(rbx.wrapping_add(0x10u64))) as u32 as u64; crate::simd_ops::wu32(rdx.wrapping_add(0x10u64), ((rcx as u32)) as u32); rdx = ((rdx).wrapping_add(0x14u64)) as u64; r8 = r9.wrapping_sub(0x1); fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 7; continue; }
            7 => { bb = 8; continue; }
            8 => { rbx = rbx.wrapping_add(0x14); xmm0 = (crate::simd_ops::ru128(rbx)) as u128; crate::simd_ops::wu128(rdx, (xmm0) as u128); rcx = (crate::simd_ops::ru32(rbx.wrapping_add(0x10u64))) as u32 as u64; crate::simd_ops::wu32(rdx.wrapping_add(0x10u64), ((rcx as u32)) as u32); rdx = ((rdx).wrapping_add(0x14u64)) as u64; fl_l = r8; fl_r = (0x1u64) as u64; r8 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 8; continue; } bb = 9; continue; }
            9 => { r8 = rsi; fl_l = r8; fl_r = r9; r8 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 16; continue; } bb = 10; continue; }
            10 => { fl_l = rdx; fl_r = rax; rdx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; bb = 11; continue; }
            11 => { xmm0 = (crate::simd_ops::ru128(rax)) as u128; crate::simd_ops::wu128(rdx.wrapping_add(rax), (xmm0) as u128); rcx = (crate::simd_ops::ru32(rax.wrapping_add(0x10u64))) as u32 as u64; crate::simd_ops::wu32(rdx.wrapping_add(rax).wrapping_add(0x10u64), ((rcx as u32)) as u32); rax = rax.wrapping_add(0x14); fl_l = r8; fl_r = (0x1u64) as u64; r8 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 11; continue; } bb = 12; continue; }
            12 => { bb = 16; continue; }
            13 => { rcx = (crate::simd_ops::ru64(P2)) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 16; continue; } bb = 14; continue; }
            14 => { bb = 15; continue; }
            15 => { xmm0 = (crate::simd_ops::ru128(rbx)) as u128; crate::simd_ops::wu128(rcx, (xmm0) as u128); rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x10u64))) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(0x10u64), ((rax as u32)) as u32); rcx = rcx.wrapping_add(0x14); rbx = ((rbx).wrapping_add(0x14u64)) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 15; continue; } bb = 16; continue; }
            16 => { crate::simd_ops::wu32(P0, ((rsi as u32)) as u32); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1412f1cc0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = (crate::simd_ops::ru64(rdx)) as u64;
    r8 = (crate::simd_ops::ru64(rcx)) as u64;
    crate::simd_ops::wu64(rcx, rax);
    crate::simd_ops::wu64(rdx, r8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1412f1d90(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142242fb0; const P1: u64 = 0x141834d30;
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
pub unsafe extern "win64" fn fn_1412f1dd0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x40; const P1: u64 = 0x10; const P2: u64 = 0x18; const P3: u64 = 0xffffffffffffffd8; const P4: u64 = 0x142535070; const P5: u64 = 0x1426456c8; const P6: u64 = 0x1426453a0; const P7: u64 = 0x1433f18d8; const P8: u64 = 0x8; const P9: u64 = 0x20; const P10: u64 = 0x28; const P11: u64 = 0x30;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rax = ((crate::exe::tls_pointer() as u64)) as u64; rbx = 0; fl_l = 0; fl_r = 0; rdx = 0x6348; rcx = (crate::simd_ops::ru64(rax)) as u64; rax = (crate::simd_ops::ru64(rdx.wrapping_add(rcx))) as u64; rdx = (crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64; rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P1))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { fl_l = ((rcx as u8)) as u8 as u64; fl_r = (0xfu8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::lk_add_u32(rdx.wrapping_add(P2), (1) as u32)) as u32 as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; crate::simd_ops::wu64(rdx.wrapping_add(P1), rax); bb = 5; continue; }
            3 => { rdx = 0x38; rcx = rax; r8 = ((rdx.wrapping_add(P3)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rcx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp); rcx = rax; rdx = 0xdeadd00d; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P6), rax), rax, rcx, rdx, r8, r9, rsp); rcx = rbx; bb = 5; continue; }
            5 => { rax = P7; crate::simd_ops::wu64(rcx.wrapping_add(P8), rbx); crate::simd_ops::wu64(rcx, rax); rax = rcx; crate::simd_ops::wu64(rcx.wrapping_add(P1), rbx); crate::simd_ops::wu64(rcx.wrapping_add(P2), rbx); crate::simd_ops::wu64(rcx.wrapping_add(P9), rbx); crate::simd_ops::wu64(rcx.wrapping_add(P10), rbx); crate::simd_ops::wu32(rcx.wrapping_add(P11), ((rbx as u32)) as u32); rsp = ((rsp).wrapping_add(0x30u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1412f1f50(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x58; const P1: u64 = 0x10; const P2: u64 = 0x18; const P3: u64 = 0xffffffffffffffc0; const P4: u64 = 0x142535070; const P5: u64 = 0x1426456c8; const P6: u64 = 0x1426453a0; const P7: u64 = 0x1433f1940; const P8: u64 = 0x8; const P9: u64 = 0x20; const P10: u64 = 0x28; const P11: u64 = 0x2c;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rax = ((crate::exe::tls_pointer() as u64)) as u64; rbx = 0; fl_l = 0; fl_r = 0; rdx = 0x6348; rcx = (crate::simd_ops::ru64(rax)) as u64; rax = (crate::simd_ops::ru64(rdx.wrapping_add(rcx))) as u64; rdx = (crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64; rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P1))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { fl_l = ((rcx as u8)) as u8 as u64; fl_r = (0xfu8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::lk_add_u32(rdx.wrapping_add(P2), (1) as u32)) as u32 as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; crate::simd_ops::wu64(rdx.wrapping_add(P1), rax); bb = 5; continue; }
            3 => { rdx = 0x50; rcx = rax; r8 = ((rdx.wrapping_add(P3)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rcx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp); rcx = rax; rdx = 0xdeadd00d; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P6), rax), rax, rcx, rdx, r8, r9, rsp); rcx = rbx; bb = 5; continue; }
            5 => { rax = P7; crate::simd_ops::wu16(rcx.wrapping_add(P8), ((rbx as u16)) as u16); crate::simd_ops::wu64(rcx, rax); rax = rcx; crate::simd_ops::wu64(rcx.wrapping_add(P1), rbx); crate::simd_ops::wu64(rcx.wrapping_add(P2), rbx); crate::simd_ops::wu64(rcx.wrapping_add(P9), rbx); crate::simd_ops::wu32(rcx.wrapping_add(P10), (0xffffffffu32) as u32); crate::simd_ops::wu64(rcx.wrapping_add(P11), rbx); crate::simd_ops::wu64(rcx.wrapping_add(0x34u64), rbx); crate::simd_ops::wu64(rcx.wrapping_add(0x3cu64), rbx); crate::simd_ops::wu64(rcx.wrapping_add(0x44u64), rbx); rsp = ((rsp).wrapping_add(0x30u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1412f2000(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1433f19a8; const P1: u64 = 0x142243b40; const P2: u64 = 0x142442de0; const P3: u64 = 0x1401de830; const P4: u64 = 0x14204f860; const P5: u64 = 0x14206b020; const P6: u64 = 0x142201e30; const P7: u64 = 0x1433f1818; const P8: u64 = 0x141834d30;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; r14 = ((rdx as u32)) as u32 as u64; rsi = rcx; rax = P0; crate::simd_ops::wu64(rcx, rax); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rbx = rsi.wrapping_add(0x58); rbp = 0x400; r15 = 0; fl_l = 0; fl_r = 0; bb = 1; continue; }
            1 => { rdi = (crate::simd_ops::ru64(rbx)) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(rdi)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = rdi; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = rdi; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 3; continue; }
            3 => { crate::simd_ops::wu64(rbx, r15); rbx = ((rbx).wrapping_add(0x8u64)) as u64; fl_l = rbp; fl_r = (0x1u64) as u64; rbp = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } bb = 4; continue; }
            4 => { rbx = (crate::simd_ops::ru64(rsi.wrapping_add(0x50u64))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { rcx = rbx.wrapping_add(0x10); rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rcx = rbx; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 6; continue; }
            6 => { rbx = (crate::simd_ops::ru64(rsi.wrapping_add(0x48u64))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 7; continue; }
            7 => { rcx = rbx; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rcx = rbx; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 8; continue; }
            8 => { rbx = (crate::simd_ops::ru64(rsi.wrapping_add(0x40u64))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 9; continue; }
            9 => { rdi = (crate::simd_ops::ru64(rbx)) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 10; continue; }
            10 => { rcx = rdi; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rcx = rdi; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 11; continue; }
            11 => { rcx = rbx; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 12; continue; }
            12 => { rcx = rsi.wrapping_add(0x28); rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 14; continue; } bb = 13; continue; }
            13 => { rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 14; continue; }
            14 => { rax = P7; crate::simd_ops::wu64(rsi, rax); fl_l = ((r14 as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 16; continue; } bb = 15; continue; }
            15 => { rdx = 0x2058; rcx = rsi; rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); bb = 16; continue; }
            16 => { rax = rsi; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1412f2120(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x18; const P1: u64 = 0x10; const P2: u64 = 0x142537280; const P3: u64 = 0x1426456c8; const P4: u64 = 0x1426453a0; const P5: u64 = 0x8; const P6: u64 = 0xc;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rax = ((crate::exe::tls_pointer() as u64)) as u64; rbx = rcx; rcx = 0x6348; rdx = (crate::simd_ops::ru64(rax)) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(rdx))) as u64; r8 = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rdx = (crate::simd_ops::ru64(r8.wrapping_add(P1))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { r9 = 0; fl_l = 0; fl_r = 0; r8 = 0; fl_l = 0; fl_r = 0; rdx = ((r9.wrapping_add(P1)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rdx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 4; continue; } bb = 2; continue; }
            2 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); rcx = rax; rdx = 0xdeadd00d; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp); rdx = 0; fl_l = 0; fl_r = 0; bb = 4; continue; }
            3 => { fl_l = (crate::simd_ops::lk_add_u32(r8.wrapping_add(P0), (1) as u32)) as u32 as u64; rax = (crate::simd_ops::ru64(rdx)) as u64; crate::simd_ops::wu64(r8.wrapping_add(P1), rax); bb = 4; continue; }
            4 => { crate::simd_ops::wu32(rdx.wrapping_add(P5), (0xffffffffu32) as u32); rax = rbx; crate::simd_ops::wu32(rdx.wrapping_add(P6), (0x0u32) as u32); crate::simd_ops::wu64(rdx, (0x0u64) as u64); crate::simd_ops::wu64(rbx, rdx); rsp = ((rsp).wrapping_add(0x30u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1412f2260(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x48; const P1: u64 = 0x10; const P2: u64 = 0x40; const P3: u64 = 0x142537280; const P4: u64 = 0x1426456c8; const P5: u64 = 0x1426453a0; const P6: u64 = 0x18; const P7: u64 = 0x14204f920;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x1412f2260, funcinfo_rva: 0x36800b0, handler: 1, shape: 2, frame_reg: 0, establisher_off: 0x38, conts: &[] };
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (rsi) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rdi) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rdx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rsi = (r9) as u64; r14 = (r8) as u64; rdi = (rcx) as u64; rbx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rcx = (0x6348u32) as u32 as u64; rax = ((crate::exe::tls_pointer() as u64)) as u64; rdx = (crate::simd_ops::ru64(rax)) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(rdx))) as u64; r10 = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rdx = (crate::simd_ops::ru64(r10.wrapping_add(P1))) as u64; fl_l = (rdx) as u64 as u64; fl_r = (rdx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { r9 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; r8 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rdx = ((rbx.wrapping_add(P2)) as u32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1412f22b0); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rdx = (rax) as u64; fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 4; continue; } bb = 2; continue; }
            2 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1412f22bd); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P4)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rcx = (rax) as u64; rdx = (0xdeadd00du32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1412f22cb); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P5)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 5; continue; }
            3 => { fl_l = (crate::simd_ops::lk_add_u32(r10.wrapping_add(P6), (1) as u32)) as u32 as u64; rax = (crate::simd_ops::ru64(rdx)) as u64; crate::simd_ops::wu64(r10.wrapping_add(P1), (rax) as u64); bb = 4; continue; }
            4 => { rbx = (rdx) as u64; bb = 5; continue; }
            5 => { crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), (rbx) as u64); r8 = (rsi) as u64; rdx = (r14) as u64; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1412f22ef); crate::simd_ops::replace_rax_in(rax); rax = match eh.call(0x1412f22ef, (P7) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8) { Ok(v) => v, Err(l) => l.unknown(0x1412f2260) }; crate::simd_ops::wu64(rdi, (rax) as u64); rax = (rdi) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1412f23d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1412f2920; const P1: u64 = 0x1412f23d0; const P2: u64 = 0x1412f27c0; const P3: u64 = 0x1412f2cd0;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 17>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rbp = rsp; rsp = ((rsp).wrapping_sub(0x50u64)) as u64; r10 = (crate::simd_ops::ru64(rdx)) as u64; rbx = (((r9 as u8)) as u32) as u32 as u64; r9 = (crate::simd_ops::ru64(rcx)) as u64; rax = r10; rax = ((rax).wrapping_sub(r9)) as u64; r14 = r8; r15 = rdx; r12 = rcx; fl_l = rax; fl_r = (0x294u64) as u64; if ((((fl_l as u64)) as i64) < (((fl_r as u64)) as i64)) { bb = 7; continue; } bb = 1; continue; }
            1 => { r13 = (0x6666666666666667u64) as u64; bb = 2; continue; }
            2 => { crate::simd_ops::wu64(rbp.wrapping_add(0x40u64), r10); crate::simd_ops::wu64(rbp.wrapping_add(0x48u64), r9); fl_l = r14; fl_r = r14; if (((((fl_l as u64)) & ((fl_r as u64))) as i64) <= 0) { bb = 14; continue; } bb = 3; continue; }
            3 => { r9 = (((rbx as u8)) as u32) as u32 as u64; r8 = rbp.wrapping_add(0x40); rdx = rbp.wrapping_add(0x48); rcx = rbp.wrapping_sub(0x20); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r10 = (crate::simd_ops::ru64(r15)) as u64; rcx = r14; rdi = (crate::simd_ops::ru64(rbp.wrapping_sub(0x18u64))) as u64; rax = r13; rsi = (crate::simd_ops::ru64(rbp.wrapping_sub(0x20u64))) as u64; r9 = (crate::simd_ops::ru64(r12)) as u64; rcx = ((((rcx) as i64).wrapping_shr(2u32) as u64)) as u64; r14 = ((((r14) as i64).wrapping_shr(1u32) as u64)) as u64; r14 = ((r14).wrapping_add(rcx)) as u64; rcx = r10; rcx = ((rcx).wrapping_sub(rdi)) as u64; let prod = (rax as i64 as i128).wrapping_mul((rcx) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; rcx = rsi; r8 = rdx; rcx = ((rcx).wrapping_sub(r9)) as u64; r8 = ((((r8) as i64).wrapping_shr(3u32) as u64)) as u64; rax = r8; rax = ((rax).wrapping_shr(63u32)) as u64; r8 = ((r8).wrapping_add(rax)) as u64; rax = r13; let prod = (rax as i64 as i128).wrapping_mul((rcx) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; rcx = rbp.wrapping_add(0x48); rdx = ((((rdx) as i64).wrapping_shr(3u32) as u64)) as u64; rax = rdx; rax = ((rax).wrapping_shr(63u32)) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; fl_l = rdx; fl_r = r8; r8 = r14; rdx = rbp.wrapping_add(0x40); if ((((fl_l as u64)) as i64) >= (((fl_r as u64)) as i64)) { bb = 5; continue; } bb = 4; continue; }
            4 => { crate::simd_ops::wu64(rbp.wrapping_add(0x48u64), r9); r9 = (((rbx as u8)) as u32) as u32 as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x40u64), rsi); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(r12, rdi); bb = 6; continue; }
            5 => { r9 = (((rbx as u8)) as u32) as u32 as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x40u64), r10); crate::simd_ops::wu64(rbp.wrapping_add(0x48u64), rdi); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(r15, rsi); bb = 6; continue; }
            6 => { r10 = (crate::simd_ops::ru64(r15)) as u64; r9 = (crate::simd_ops::ru64(r12)) as u64; rax = r10; rax = ((rax).wrapping_sub(r9)) as u64; fl_l = rax; fl_r = (0x294u64) as u64; if ((((fl_l as u64)) as i64) >= (((fl_r as u64)) as i64)) { bb = 2; continue; } bb = 7; continue; }
            7 => { fl_l = r9; fl_r = r10; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 25; continue; } bb = 8; continue; }
            8 => { rcx = r9.wrapping_add(0x14); fl_l = rcx; fl_r = r10; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 25; continue; } bb = 9; continue; }
            9 => { r11 = (crate::simd_ops::ru32(rcx.wrapping_add(0x10u64))) as u32 as u64; rdx = rcx; xmm0 = ((crate::simd_ops::ru32(r9.wrapping_add(0x10u64))) as u32) as u128; xmm2 = (crate::simd_ops::ru128(rcx)) as u128; xmm1 = (((r11 as u32)) as u32) as u128; fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = ((xmm1 as u32)) as u32 as u64; if !(f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 20; continue; } bb = 10; continue; }
            10 => { fl_l = r9; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 13; continue; } bb = 11; continue; }
            11 => { rax = rcx; r8 = rcx.wrapping_add(0x14); bb = 12; continue; }
            12 => { xmm0 = (crate::simd_ops::ru128(rax.wrapping_sub(0x14u64))) as u128; rdx = (crate::simd_ops::ru32(rax.wrapping_sub(0x4u64))) as u32 as u64; r8 = r8.wrapping_sub(0x14); rax = ((rax).wrapping_add(0xffffffffffffffecu64)) as u64; crate::simd_ops::wu128(r8, (xmm0) as u128); crate::simd_ops::wu32(r8.wrapping_add(0x10u64), ((rdx as u32)) as u32); fl_l = rax; fl_r = r9; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 12; continue; } bb = 13; continue; }
            13 => { crate::simd_ops::wu128(r9, (xmm2) as u128); crate::simd_ops::wu32(r9.wrapping_add(0x10u64), ((r11 as u32)) as u32); bb = 24; continue; }
            14 => { r8 = (((rbx as u8)) as u32) as u32 as u64; rdx = rbp.wrapping_add(0x40); rcx = rbp.wrapping_add(0x48); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru64(r15)) as u64; rsi = (crate::simd_ops::ru64(r12)) as u64; rax = rcx; rax = ((rax).wrapping_sub(rsi)) as u64; fl_l = rax; fl_r = (0x28u64) as u64; if ((((fl_l as u64)) as i64) < (((fl_r as u64)) as i64)) { bb = 25; continue; } bb = 15; continue; }
            15 => { bb = 16; continue; }
            16 => { rdi = rcx.wrapping_sub(0x14); rax = r13; rcx = ((rcx).wrapping_sub(rsi)) as u64; let prod = (rax as i64 as i128).wrapping_mul((rcx) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; rcx = rdi; rdx = ((((rdx) as i64).wrapping_shr(3u32) as u64)) as u64; rax = rdx; rax = ((rax).wrapping_shr(63u32)) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; fl_l = rdx; fl_r = (0x2u64) as u64; if ((((fl_l as u64)) as i64) < (((fl_r as u64)) as i64)) { bb = 18; continue; } bb = 17; continue; }
            17 => { rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x10u64))) as u32 as u64; r9 = rbp.wrapping_sub(0x20); xmm0 = (crate::simd_ops::ru128(rdi)) as u128; crate::simd_ops::wu32(rbp.wrapping_sub(0x10u64), ((rax as u32)) as u32); rcx = ((rcx).wrapping_sub(rsi)) as u64; rax = (crate::simd_ops::ru32(rsi.wrapping_add(0x10u64))) as u32 as u64; crate::simd_ops::wu128(rbp.wrapping_sub(0x20u64), (xmm0) as u128); xmm0 = (crate::simd_ops::ru128(rsi)) as u128; crate::simd_ops::wu128(rdi, (xmm0) as u128); crate::simd_ops::wu32(rdi.wrapping_add(0x10u64), ((rax as u32)) as u32); rax = r13; let prod = (rax as i64 as i128).wrapping_mul((rcx) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; rcx = rbp.wrapping_add(0x40); crate::simd_ops::wu64(rbp.wrapping_add(0x40u64), rsi); rdx = ((((rdx) as i64).wrapping_shr(3u32) as u64)) as u64; r8 = rdx; crate::simd_ops::wu8(rsp.wrapping_add(0x20u64), ((rbx as u8)) as u8); r8 = ((r8).wrapping_shr(63u32)) as u64; r8 = ((r8).wrapping_add(rdx)) as u64; rdx = 0; fl_l = 0; fl_r = 0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rdi; bb = 18; continue; }
            18 => { rax = rcx; rax = ((rax).wrapping_sub(rsi)) as u64; fl_l = rax; fl_r = (0x28u64) as u64; if ((((fl_l as u64)) as i64) >= (((fl_r as u64)) as i64)) { bb = 16; continue; } bb = 19; continue; }
            19 => { bb = 25; continue; }
            20 => { xmm0 = ((crate::simd_ops::ru32(rcx.wrapping_sub(0x4u64))) as u32) as u128; fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = ((xmm1 as u32)) as u32 as u64; rax = rcx.wrapping_sub(0x14); if !(f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 23; continue; } bb = 21; continue; }
            21 => { bb = 22; continue; }
            22 => { xmm0 = (crate::simd_ops::ru128(rax)) as u128; r8 = (crate::simd_ops::ru32(rax.wrapping_add(0x10u64))) as u32 as u64; crate::simd_ops::wu128(rdx, (xmm0) as u128); crate::simd_ops::wu32(rdx.wrapping_add(0x10u64), ((r8 as u32)) as u32); rdx = rax; xmm0 = ((crate::simd_ops::ru32(rax.wrapping_sub(0x4u64))) as u32) as u128; rax = ((rax).wrapping_sub(0x14u64)) as u64; fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = ((xmm1 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 22; continue; } bb = 23; continue; }
            23 => { crate::simd_ops::wu128(rdx, (xmm2) as u128); crate::simd_ops::wu32(rdx.wrapping_add(0x10u64), ((r11 as u32)) as u32); bb = 24; continue; }
            24 => { rcx = ((rcx).wrapping_add(0x14u64)) as u64; fl_l = rcx; fl_r = r10; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 9; continue; } bb = 25; continue; }
            25 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0xa0u64))) as u64; rsp = ((rsp).wrapping_add(0x50u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1412f27c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 1>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rdx = (crate::simd_ops::ru64(rdx)) as u64; rax = (0x6666666666666667u64) as u64; rdx = ((rdx).wrapping_sub(crate::simd_ops::ru64(rcx))) as u64; r14 = rcx; let prod = (rax as i64 as i128).wrapping_mul((rdx) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; rbx = rdx; rbx = ((((rbx) as i64).wrapping_shr(3u32) as u64)) as u64; rax = rbx; rax = ((rax).wrapping_shr(63u32)) as u64; rbx = ((rbx).wrapping_add(rax)) as u64; r10 = rbx; r10 = ((((r10) as i64).wrapping_shr(1u32) as u64)) as u64; fl_l = r10; fl_r = r10; if (((((fl_l as u64)) & ((fl_r as u64))) as i64) <= 0) { bb = 13; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); r11 = ((r10.wrapping_add(r10.wrapping_mul(4))) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rdi = rbx.wrapping_sub(0x1); rdi = ((((rdi) as i64).wrapping_shr(1u32) as u64)) as u64; fl_l = ((r11).wrapping_shl(2u32)) as u64; r11 = ((fl_l as u64)) as u64; bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(r14)) as u64; r11 = r11.wrapping_sub(0x14); r10 = ((r10).wrapping_sub(1)) as u64; r9 = r10; r8 = r10; rsi = (crate::simd_ops::ru32(r11.wrapping_add(rax).wrapping_add(0x10u64))) as u32 as u64; xmm2 = (crate::simd_ops::ru128(r11.wrapping_add(rax))) as u128; fl_l = r10; fl_r = rdi; if ((((fl_l as u64)) as i64) >= (((fl_r as u64)) as i64)) { bb = 4; continue; } bb = 3; continue; }
            3 => { rdx = ((r8.wrapping_add(r8)) as u64) as u64; r8 = 0; fl_l = 0; fl_r = 0; rcx = ((rdx.wrapping_add(rdx.wrapping_mul(4))) as u64) as u64; xmm0 = ((crate::simd_ops::ru32(rax.wrapping_add(rcx.wrapping_mul(4)).wrapping_add(0x24u64))) as u32) as u128; fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rax.wrapping_add(rcx.wrapping_mul(4)).wrapping_add(0x38u64))) as u32 as u64; r8 = (r8 & !0xffu64) | (((if !(f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { 1u8 } else { 0u8 }) as u8 as u64)); r8 = ((r8).wrapping_add(1)) as u64; r8 = ((r8).wrapping_add(rdx)) as u64; rcx = ((r8.wrapping_add(r8.wrapping_mul(4))) as u64) as u64; rdx = (crate::simd_ops::ru32(rax.wrapping_add(rcx.wrapping_mul(4)).wrapping_add(0x10u64))) as u32 as u64; xmm0 = (crate::simd_ops::ru128(rax.wrapping_add(rcx.wrapping_mul(4)))) as u128; rcx = ((r9.wrapping_add(r9.wrapping_mul(4))) as u64) as u64; r9 = r8; crate::simd_ops::wu128(rax.wrapping_add(rcx.wrapping_mul(4)), (xmm0) as u128); crate::simd_ops::wu32(rax.wrapping_add(rcx.wrapping_mul(4)).wrapping_add(0x10u64), ((rdx as u32)) as u32); fl_l = r8; fl_r = rdi; if ((((fl_l as u64)) as i64) < (((fl_r as u64)) as i64)) { bb = 3; continue; } bb = 4; continue; }
            4 => { fl_l = r9; fl_r = rdi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 7; continue; } bb = 5; continue; }
            5 => { fl_l = ((rbx as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 7; continue; } bb = 6; continue; }
            6 => { rcx = ((rbx.wrapping_add(rbx.wrapping_mul(4))) as u64) as u64; xmm0 = (crate::simd_ops::ru128(rax.wrapping_add(rcx.wrapping_mul(4)).wrapping_sub(0x14u64))) as u128; rdx = (crate::simd_ops::ru32(rax.wrapping_add(rcx.wrapping_mul(4)).wrapping_sub(0x4u64))) as u32 as u64; rcx = ((r9.wrapping_add(r9.wrapping_mul(4))) as u64) as u64; crate::simd_ops::wu128(rax.wrapping_add(rcx.wrapping_mul(4)), (xmm0) as u128); crate::simd_ops::wu32(rax.wrapping_add(rcx.wrapping_mul(4)).wrapping_add(0x10u64), ((rdx as u32)) as u32); r9 = rbx.wrapping_sub(0x1); bb = 7; continue; }
            7 => { r8 = r9.wrapping_sub(0x1); r8 = ((((r8) as i64).wrapping_shr(1u32) as u64)) as u64; fl_l = r10; fl_r = r9; if ((((fl_l as u64)) as i64) >= (((fl_r as u64)) as i64)) { bb = 11; continue; } bb = 8; continue; }
            8 => { xmm1 = (((rsi as u32)) as u32) as u128; bb = 9; continue; }
            9 => { rdx = ((r8.wrapping_add(r8.wrapping_mul(4))) as u64) as u64; fl_l = ((xmm1 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rax.wrapping_add(rdx.wrapping_mul(4)).wrapping_add(0x10u64))) as u32 as u64; if !(f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 11; continue; } bb = 10; continue; }
            10 => { xmm0 = (crate::simd_ops::ru128(rax.wrapping_add(rdx.wrapping_mul(4)))) as u128; rdx = (crate::simd_ops::ru32(rax.wrapping_add(rdx.wrapping_mul(4)).wrapping_add(0x10u64))) as u32 as u64; rcx = ((r9.wrapping_add(r9.wrapping_mul(4))) as u64) as u64; r9 = r8; r8 = ((r8).wrapping_sub(1)) as u64; r8 = ((((r8) as i64).wrapping_shr(1u32) as u64)) as u64; crate::simd_ops::wu128(rax.wrapping_add(rcx.wrapping_mul(4)), (xmm0) as u128); crate::simd_ops::wu32(rax.wrapping_add(rcx.wrapping_mul(4)).wrapping_add(0x10u64), ((rdx as u32)) as u32); fl_l = r10; fl_r = r9; if ((((fl_l as u64)) as i64) < (((fl_r as u64)) as i64)) { bb = 9; continue; } bb = 11; continue; }
            11 => { rcx = ((r9.wrapping_add(r9.wrapping_mul(4))) as u64) as u64; crate::simd_ops::wu128(rax.wrapping_add(rcx.wrapping_mul(4)), (xmm2) as u128); crate::simd_ops::wu32(rax.wrapping_add(rcx.wrapping_mul(4)).wrapping_add(0x10u64), ((rsi as u32)) as u32); fl_l = r10; fl_r = r10; if (((((fl_l as u64)) & ((fl_r as u64))) as i64) > 0) { bb = 2; continue; } bb = 12; continue; }
            12 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x18u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x10u64))) as u64; bb = 13; continue; }
            13 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1412f2cd0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 0>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rbx = r8.wrapping_sub(0x1); r10 = rdx; rbx = ((((rbx) as i64).wrapping_shr(1u32) as u64)) as u64; rsi = rcx; rdi = rdx; r11 = rdx; fl_l = rdx; fl_r = rbx; if ((((fl_l as u64)) as i64) >= (((fl_r as u64)) as i64)) { bb = 3; continue; } bb = 1; continue; }
            1 => { bb = 2; continue; }
            2 => { rdx = (crate::simd_ops::ru64(rsi)) as u64; rcx = ((r11.wrapping_add(r11)) as u64) as u64; r11 = 0; fl_l = 0; fl_r = 0; rax = ((rcx.wrapping_add(rcx.wrapping_mul(4))) as u64) as u64; xmm0 = ((crate::simd_ops::ru32(rdx.wrapping_add(rax.wrapping_mul(4)).wrapping_add(0x24u64))) as u32) as u128; fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rdx.wrapping_add(rax.wrapping_mul(4)).wrapping_add(0x38u64))) as u32 as u64; r11 = (r11 & !0xffu64) | (((if !(f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { 1u8 } else { 0u8 }) as u8 as u64)); r11 = ((r11).wrapping_add(1)) as u64; r11 = ((r11).wrapping_add(rcx)) as u64; rax = ((r11.wrapping_add(r11.wrapping_mul(4))) as u64) as u64; rcx = (crate::simd_ops::ru32(rdx.wrapping_add(rax.wrapping_mul(4)).wrapping_add(0x10u64))) as u32 as u64; xmm0 = (crate::simd_ops::ru128(rdx.wrapping_add(rax.wrapping_mul(4)))) as u128; rax = ((r10.wrapping_add(r10.wrapping_mul(4))) as u64) as u64; r10 = r11; crate::simd_ops::wu128(rdx.wrapping_add(rax.wrapping_mul(4)), (xmm0) as u128); crate::simd_ops::wu32(rdx.wrapping_add(rax.wrapping_mul(4)).wrapping_add(0x10u64), ((rcx as u32)) as u32); fl_l = r11; fl_r = rbx; if ((((fl_l as u64)) as i64) < (((fl_r as u64)) as i64)) { bb = 2; continue; } bb = 3; continue; }
            3 => { fl_l = r11; fl_r = rbx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 6; continue; } bb = 4; continue; }
            4 => { fl_l = ((r8 as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { rdx = (crate::simd_ops::ru64(rsi)) as u64; rax = ((r8.wrapping_add(r8.wrapping_mul(4))) as u64) as u64; xmm0 = (crate::simd_ops::ru128(rdx.wrapping_add(rax.wrapping_mul(4)).wrapping_sub(0x14u64))) as u128; rcx = (crate::simd_ops::ru32(rdx.wrapping_add(rax.wrapping_mul(4)).wrapping_sub(0x4u64))) as u32 as u64; rax = ((r10.wrapping_add(r10.wrapping_mul(4))) as u64) as u64; crate::simd_ops::wu128(rdx.wrapping_add(rax.wrapping_mul(4)), (xmm0) as u128); crate::simd_ops::wu32(rdx.wrapping_add(rax.wrapping_mul(4)).wrapping_add(0x10u64), ((rcx as u32)) as u32); r10 = r8.wrapping_sub(0x1); bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru64(rsi)) as u64; r8 = r10.wrapping_sub(0x1); r8 = ((((r8) as i64).wrapping_shr(1u32) as u64)) as u64; fl_l = rdi; fl_r = r10; if ((((fl_l as u64)) as i64) >= (((fl_r as u64)) as i64)) { bb = 10; continue; } bb = 7; continue; }
            7 => { xmm1 = ((crate::simd_ops::ru32(r9.wrapping_add(0x10u64))) as u32) as u128; bb = 8; continue; }
            8 => { rdx = ((r8.wrapping_add(r8.wrapping_mul(4))) as u64) as u64; fl_l = ((xmm1 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rax.wrapping_add(rdx.wrapping_mul(4)).wrapping_add(0x10u64))) as u32 as u64; if !(f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 10; continue; } bb = 9; continue; }
            9 => { xmm0 = (crate::simd_ops::ru128(rax.wrapping_add(rdx.wrapping_mul(4)))) as u128; rdx = (crate::simd_ops::ru32(rax.wrapping_add(rdx.wrapping_mul(4)).wrapping_add(0x10u64))) as u32 as u64; rcx = ((r10.wrapping_add(r10.wrapping_mul(4))) as u64) as u64; r10 = r8; r8 = ((r8).wrapping_sub(1)) as u64; r8 = ((((r8) as i64).wrapping_shr(1u32) as u64)) as u64; crate::simd_ops::wu128(rax.wrapping_add(rcx.wrapping_mul(4)), (xmm0) as u128); crate::simd_ops::wu32(rax.wrapping_add(rcx.wrapping_mul(4)).wrapping_add(0x10u64), ((rdx as u32)) as u32); fl_l = rdi; fl_r = r10; if ((((fl_l as u64)) as i64) < (((fl_r as u64)) as i64)) { bb = 8; continue; } bb = 10; continue; }
            10 => { xmm0 = (crate::simd_ops::ru128(r9)) as u128; rcx = (crate::simd_ops::ru32(r9.wrapping_add(0x10u64))) as u32 as u64; rdx = ((r10.wrapping_add(r10.wrapping_mul(4))) as u64) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x8u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x10u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x18u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu128(rax.wrapping_add(rdx.wrapping_mul(4)), (xmm0) as u128); crate::simd_ops::wu32(rax.wrapping_add(rdx.wrapping_mul(4)).wrapping_add(0x10u64), ((rcx as u32)) as u32); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1412f2df0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 0>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdi); r9 = (crate::simd_ops::ru64(rcx)) as u64; rbx = r8; r10 = (crate::simd_ops::ru64(rdx)) as u64; r11 = rdx; rdi = rcx; xmm0 = ((crate::simd_ops::ru32(r9.wrapping_add(P0))) as u32) as u128; fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(r10.wrapping_add(P0))) as u32 as u64; if !(f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 2; continue; } bb = 1; continue; }
            1 => { xmm0 = (crate::simd_ops::ru128(r9)) as u128; xmm1 = (crate::simd_ops::ru128(r10)) as u128; r8 = (crate::simd_ops::ru32(r10.wrapping_add(P0))) as u32 as u64; crate::simd_ops::wu128(r10, (xmm0) as u128); rax = (crate::simd_ops::ru32(r9.wrapping_add(P0))) as u32 as u64; crate::simd_ops::wu32(r10.wrapping_add(P0), ((rax as u32)) as u32); crate::simd_ops::wu128(r9, (xmm1) as u128); crate::simd_ops::wu32(r9.wrapping_add(P0), ((r8 as u32)) as u32); bb = 2; continue; }
            2 => { rdx = (crate::simd_ops::ru64(rdx)) as u64; r8 = (crate::simd_ops::ru64(rbx)) as u64; xmm0 = ((crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32) as u128; fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(r8.wrapping_add(P0))) as u32 as u64; if !(f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 5; continue; } bb = 3; continue; }
            3 => { xmm0 = (crate::simd_ops::ru128(rdx)) as u128; xmm1 = (crate::simd_ops::ru128(r8)) as u128; rcx = (crate::simd_ops::ru32(r8.wrapping_add(P0))) as u32 as u64; crate::simd_ops::wu128(r8, (xmm0) as u128); rax = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(P0), ((rax as u32)) as u32); crate::simd_ops::wu128(rdx, (xmm1) as u128); crate::simd_ops::wu32(rdx.wrapping_add(P0), ((rcx as u32)) as u32); rdx = (crate::simd_ops::ru64(rdi)) as u64; r8 = (crate::simd_ops::ru64(r11)) as u64; xmm0 = ((crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32) as u128; fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(r8.wrapping_add(P0))) as u32 as u64; if !(f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 5; continue; } bb = 4; continue; }
            4 => { xmm0 = (crate::simd_ops::ru128(rdx)) as u128; xmm1 = (crate::simd_ops::ru128(r8)) as u128; rcx = (crate::simd_ops::ru32(r8.wrapping_add(P0))) as u32 as u64; crate::simd_ops::wu128(r8, (xmm0) as u128); rax = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(P0), ((rax as u32)) as u32); crate::simd_ops::wu128(rdx, (xmm1) as u128); crate::simd_ops::wu32(rdx.wrapping_add(P0), ((rcx as u32)) as u32); bb = 5; continue; }
            5 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x8u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x10u64))) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1412f2f10(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143cf19e0; const P1: u64 = 0x1433f1d08; const P2: u64 = 0x143cf19e8; const P3: u64 = 0x14220ea70; const P4: u64 = 0x143cf19ec; const P5: u64 = 0x143cf19f0; const P6: u64 = 0x143428ba0; const P7: u64 = 0x143429660; const P8: u64 = 0x143cf1a10; const P9: u64 = 0x143cf1a30; const P10: u64 = 0x143cf1a50; const P11: u64 = 0x143cf1a70; const P12: u64 = 0x143cf1a90; const P13: u64 = 0x143cf1ab0; const P14: u64 = 0x143cf1ad0; const P15: u64 = 0x143cf1af0; const P16: u64 = 0x143cf1b10; const P17: u64 = 0x143cf1b30; const P18: u64 = 0x143cf1b50;
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut ymm1_hi: u128 = 0; let mut ymm3_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rbx = ((r8 as u32)) as u32 as u64; rdi = P0; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rdi); crate::simd_ops::wu64(P0, (0x0u64) as u64); rax = P1; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (0x1du64) as u64); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128); rdx = rsp.wrapping_add(0x20); rcx = P2; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu32(P4, ((rbx as u32)) as u32); rax = P5; crate::simd_ops::wu64(P0, rax); let y_addr: u64 = P6; xmm3 = crate::simd_ops::ru128(y_addr); ymm3_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = P7; xmm1 = crate::simd_ops::ru128(y_addr); ymm1_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = P5; crate::simd_ops::wu128(y_addr, xmm3); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm3_hi); let y_addr: u64 = P8; crate::simd_ops::wu128(y_addr, xmm1); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm1_hi); let y_addr: u64 = P9; crate::simd_ops::wu128(y_addr, xmm3); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm3_hi); let y_addr: u64 = P10; crate::simd_ops::wu128(y_addr, xmm1); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm1_hi); let y_addr: u64 = P11; crate::simd_ops::wu128(y_addr, xmm3); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm3_hi); let y_addr: u64 = P12; crate::simd_ops::wu128(y_addr, xmm1); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm1_hi); let y_addr: u64 = P13; crate::simd_ops::wu128(y_addr, xmm3); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm3_hi); let y_addr: u64 = P14; crate::simd_ops::wu128(y_addr, xmm1); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm1_hi); let y_addr: u64 = P15; crate::simd_ops::wu128(y_addr, xmm3); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm3_hi); let y_addr: u64 = P16; crate::simd_ops::wu128(y_addr, xmm1); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm1_hi); let y_addr: u64 = P17; crate::simd_ops::wu128(y_addr, xmm3); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm3_hi); let y_addr: u64 = P18; crate::simd_ops::wu128(y_addr, xmm1); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm1_hi); rax = rdi; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1412f3000(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142537280; const P1: u64 = 0x1426456c8; const P2: u64 = 0x1426453a0; const P3: u64 = 0x142076410;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x60u64)) as u64; rax = ((crate::exe::tls_pointer() as u64)) as u64; rdi = rcx; rcx = 0x6348; rbx = 0; fl_l = 0; fl_r = 0; rsi = r9; r14 = r8; rdx = (crate::simd_ops::ru64(rax)) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(rdx))) as u64; r10 = (crate::simd_ops::ru64(rcx.wrapping_add(0x48u64))) as u64; rdx = (crate::simd_ops::ru64(r10.wrapping_add(0x10u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { r9 = 0; fl_l = 0; fl_r = 0; rdx = ((rbx.wrapping_add(0x40u64)) as u32) as u32 as u64; r8 = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rdx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 4; continue; } bb = 2; continue; }
            2 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp); rcx = rax; rdx = 0xdeadd00d; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); bb = 5; continue; }
            3 => { fl_l = (crate::simd_ops::lk_add_u32(r10.wrapping_add(0x18u64), (1) as u32)) as u32 as u64; rax = (crate::simd_ops::ru64(rdx)) as u64; crate::simd_ops::wu64(r10.wrapping_add(0x10u64), rax); bb = 4; continue; }
            4 => { rbx = rdx; bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xc0u64))) as u64; r9 = (crate::simd_ops::ru64(rsp.wrapping_add(0x90u64))) as u64; r8 = ((crate::simd_ops::ru8(rsi)) as u32) as u32 as u64; rcx = (crate::simd_ops::ru32(rax)) as u32 as u64; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xb8u64))) as u64; r9 = (crate::simd_ops::ru32(r9)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x48u64), ((rcx as u32)) as u32); rcx = (crate::simd_ops::ru32(rax)) as u32 as u64; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xb0u64))) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x40u64), ((rcx as u32)) as u32); rcx = rbx; rdx = (crate::simd_ops::ru32(rax)) as u32 as u64; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xa8u64))) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x38u64), ((rdx as u32)) as u32); rdx = (crate::simd_ops::ru32(rax)) as u32 as u64; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xa0u64))) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((rdx as u32)) as u32); rdx = (crate::simd_ops::ru32(rax)) as u32 as u64; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x98u64))) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((rdx as u32)) as u32); rdx = ((crate::simd_ops::ru8(rax)) as u32) as u32 as u64; crate::simd_ops::wu8(rsp.wrapping_add(0x20u64), ((rdx as u8)) as u8); rdx = ((crate::simd_ops::ru8(r14)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rdi, rax); rax = rdi; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; rsp = ((rsp).wrapping_add(0x60u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1412f3110(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x158; const P1: u64 = 0x10; const P2: u64 = 0x18; const P3: u64 = 0x142077690; const P4: u64 = 0x142535070; const P5: u64 = 0x1426456c8; const P6: u64 = 0x1426453a0;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rax = ((crate::exe::tls_pointer() as u64)) as u64; rdi = rcx; rcx = 0x6348; rbx = 0; fl_l = 0; fl_r = 0; rsi = r9; rbp = r8; rdx = (crate::simd_ops::ru64(rax)) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(rdx))) as u64; r10 = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rdx = (crate::simd_ops::ru64(r10.wrapping_add(P1))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 1; continue; }
            1 => { fl_l = ((rdx as u8)) as u8 as u64; fl_r = (0xfu8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 5; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::lk_add_u32(r10.wrapping_add(P2), (1) as u32)) as u32 as u64; rax = (crate::simd_ops::ru64(rdx)) as u64; crate::simd_ops::wu64(r10.wrapping_add(P1), rax); bb = 3; continue; }
            3 => { rbx = rdx; bb = 4; continue; }
            4 => { r8 = ((crate::simd_ops::ru8(rsi)) as u32) as u32 as u64; rdx = rbp; rcx = rbx; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rdi, rax); rax = rdi; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            5 => { rdx = 0x150; r8 = 0x10; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rdx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 3; continue; } bb = 6; continue; }
            6 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp); rcx = rax; rdx = 0xdeadd00d; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P6), rax), rax, rcx, rdx, r8, r9, rsp); bb = 4; continue; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1412f3220(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xb8; const P1: u64 = 0x14397ff58; const P2: u64 = 0x1412f4210; const P3: u64 = 0x142428ce0; const P4: u64 = 0x141834d30;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rdi = ((rdx as u32)) as u32 as u64;
    rbx = rcx;
    rdx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    fl_l = rdx; fl_r = rdx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rcx = (crate::simd_ops::ru64(P1)) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); }
    rcx = rbx;
    rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp);
    fl_l = ((rdi as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64;
    if !(((((fl_l as u8)) & ((fl_r as u8))) == 0)) { rdx = 0xc0; rcx = rbx; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); }
    rax = rbx;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1412f3320(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x3; const P1: u64 = 0x4; const P2: u64 = 0x8; const P3: u64 = 0x1412f3490; const P4: u64 = 0x1402013d0; const P5: u64 = 0x10; const P6: u64 = 0x6; const P7: u64 = 0x141834fc0; const P8: u64 = 0xa8; const P9: u64 = 0xb0; const P10: u64 = 0xb8; const P11: u64 = 0xc0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rbx = rcx; crate::simd_ops::lk_and_u32(rcx, (0xff000008u32) as u32); crate::simd_ops::lk_or_u32(rcx, (0x8u32) as u32); crate::simd_ops::wu8(rcx.wrapping_add(P0), (0x8u8) as u8); rdi = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rcx.wrapping_add(P1), ((rdi as u32)) as u32); crate::simd_ops::wu8(rcx.wrapping_add(P2), ((rdi as u8)) as u8); rcx = ((rcx).wrapping_add(0x48u64)) as u64; rax = P3; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); r9 = P4; rdx = ((rdi.wrapping_add(P5)) as u32) as u32 as u64; r8 = ((rdi.wrapping_add(P6)) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm0 = 0u128; ymm0_hi = 0; let y_addr: u64 = rbx.wrapping_add(P8); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); crate::simd_ops::wu64(rbx.wrapping_add(P8), rdi); crate::simd_ops::wu64(rbx.wrapping_add(P9), rdi); crate::simd_ops::wu64(rbx.wrapping_add(P10), rdi); crate::simd_ops::wu64(rbx.wrapping_add(P11), rdi); crate::simd_ops::lk_and_u32(rbx.wrapping_add(0xc8u64), (0xff000010u32) as u32); crate::simd_ops::lk_or_u32(rbx.wrapping_add(0xc8u64), (0x10u32) as u32); crate::simd_ops::wu8(rbx.wrapping_add(0xcbu64), (0x10u8) as u8); crate::simd_ops::wu32(rbx.wrapping_add(0xccu64), ((rdi as u32)) as u32); crate::simd_ops::wu8(rbx.wrapping_add(0xd0u64), ((rdi as u8)) as u8); crate::simd_ops::wu64(rbx.wrapping_add(0x150u64), rdi); crate::simd_ops::wu64(rbx.wrapping_add(0x158u64), rdi); crate::simd_ops::wu64(rbx.wrapping_add(0x160u64), rdi); crate::simd_ops::wu64(rbx.wrapping_add(0x168u64), rdi); crate::simd_ops::wu64(rbx.wrapping_add(0x170u64), rdi); crate::simd_ops::wu64(rbx.wrapping_add(0x178u64), rdi); crate::simd_ops::wu64(rbx.wrapping_add(0x180u64), rdi); rax = rbx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1412f3400(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xe0; const P1: u64 = 0x10; const P2: u64 = 0x18; const P3: u64 = 0x142535070; const P4: u64 = 0x1426456c8; const P5: u64 = 0x1426453a0; const P6: u64 = 0x142398520;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x1412f3400, funcinfo_rva: 0x3696c08, handler: 1, shape: 2, frame_reg: 0, establisher_off: 0x38, conts: &[] };
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rcx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rbx = (r8) as u64; r9 = (0x6348u32) as u32 as u64; rax = ((crate::exe::tls_pointer() as u64)) as u64; rcx = (crate::simd_ops::ru64(rax)) as u64; r9 = (crate::simd_ops::ru64(r9.wrapping_add(rcx))) as u64; rdx = (crate::simd_ops::ru64(r9.wrapping_add(P0))) as u64; rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P1))) as u64; rax = (rcx) as u64; rax = ((((rax as u32)) & (0xfu32))) as u32 as u64; fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::lk_add_u32(rdx.wrapping_add(P2), (1) as u32)) as u32 as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; crate::simd_ops::wu64(rdx.wrapping_add(P1), (rax) as u64); bb = 5; continue; }
            3 => { rdx = (0xd8u32) as u32 as u64; r8 = (0x10u32) as u32 as u64; rcx = (r9) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1412f3458); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rcx = (rax) as u64; fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1412f3465); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P4)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rcx = (rax) as u64; rdx = (0xdeadd00du32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1412f3473); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P5)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rcx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; bb = 5; continue; }
            5 => { crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), (rcx) as u64); rdx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1412f3483); crate::simd_ops::replace_rax_in(rax); rax = match eh.call(0x1412f3483, (P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8) { Ok(v) => v, Err(l) => l.unknown(0x1412f3400) }; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rbx = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1412f34c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1401de830; const P1: u64 = 0xc8; const P2: u64 = 0x14240ab10; const P3: u64 = 0xb8; const P4: u64 = 0x1412f3e50; const P5: u64 = 0xa8; const P6: u64 = 0x1412f3d90; const P7: u64 = 0x48; const P8: u64 = 0x1412f3490; const P9: u64 = 0xfffffffffffffff6; const P10: u64 = 0x141834ee8;
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rdi = rcx;
    rcx = ((rcx).wrapping_add(0x150u64)) as u64;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rcx = ((rdi.wrapping_add(P1)) as u64) as u64;
    rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp);
    rcx = ((rdi.wrapping_add(P3)) as u64) as u64;
    rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp);
    rcx = ((rdi.wrapping_add(P5)) as u64) as u64;
    rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp);
    rcx = ((rdi.wrapping_add(P7)) as u64) as u64;
    r9 = P8;
    rdx = 0x10;
    r8 = ((rdx.wrapping_add(P9)) as u32) as u32 as u64;
    rax = crate::simd_ops::call_stk(P10, rax, rcx, rdx, r8, r9, rsp);
    rcx = rdi;
    rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp);
    rax = rdi;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1412f3cc0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x1412f4410; const P2: u64 = 0xffffffffffffffff; const P3: u64 = 0x108; const P4: u64 = 0x4; const P5: u64 = 0x100; const P6: u64 = 0xe; const P7: u64 = 0x20; const P8: u64 = 0x18; const P9: u64 = 0x1425371a0; const P10: u64 = 0x142533200;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = (crate::simd_ops::ru64(rcx)) as u64; rdi = rcx; rax = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; rsi = (((rax) as i64).wrapping_mul((0xe8u64) as i64) as u64) as u64; rsi = ((rsi).wrapping_add(rbx)) as u64; fl_l = rbx; fl_r = rsi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rcx = rbx; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rbx = ((rbx).wrapping_add(0xe8u64)) as u64; fl_l = rbx; fl_r = rsi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } bb = 2; continue; }
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
pub unsafe extern "win64" fn fn_1412f3e50(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x14240ab10; const P2: u64 = 0xffffffffffffffff; const P3: u64 = 0x108; const P4: u64 = 0x4; const P5: u64 = 0x100; const P6: u64 = 0xe; const P7: u64 = 0x20; const P8: u64 = 0x18; const P9: u64 = 0x1425371a0; const P10: u64 = 0x142533200;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = (crate::simd_ops::ru64(rcx)) as u64; rdi = rcx; rax = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; rsi = (((rax) as i64).wrapping_mul((0x98u64) as i64) as u64) as u64; rsi = ((rsi).wrapping_add(rbx)) as u64; fl_l = rbx; fl_r = rsi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rcx = rbx; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rbx = ((rbx).wrapping_add(0x98u64)) as u64; fl_l = rbx; fl_r = rsi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } bb = 2; continue; }
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
pub unsafe extern "win64" fn fn_1412f3f20(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142407c80; const P1: u64 = 0x143c4aa00; const P2: u64 = 0x1412f3400; const P3: u64 = 0x14397ff58; const P4: u64 = 0x1412f4210; const P5: u64 = 0x1426455a8; const P6: u64 = 0x142645598; const P7: u64 = 0x14224bd90; const P8: u64 = 0x142645580;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x1412f3f20, funcinfo_rva: 0x3696c60, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0x28, conts: &[] };
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rbp) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (rsi) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; r14 = (rdx) as u64; rbp = (rcx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1412f3f40); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = ((rax as u32)) as u32 as u64; r8 = ((P1) as u64) as u64; r8 = (crate::simd_ops::ru64(r8.wrapping_add(rax.wrapping_mul(8)))) as u64; fl_l = (r8) as u64 as u64; fl_r = (r8) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru32(r8)) as u32 as u64; fl_r = (0x2u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 5; continue; } bb = 2; continue; }
            2 => { rbx = (crate::simd_ops::ru64(r14)) as u64; r8 = (crate::simd_ops::ru64(r14.wrapping_add(0x8u64))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1412f3f64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0xb8u64))) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0xb8u64), (rax) as u64); fl_l = (rdx) as u64 as u64; fl_r = (rdx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru64(P3)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1412f3f84); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 4; continue; }
            4 => { bb = 20; continue; }
            5 => { rdi = (crate::simd_ops::ru64(rbp.wrapping_add(0x168u64))) as u64; rcx = ((rdi.wrapping_add(0x18u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1412f3f9a); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P5)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0x20u64))) as u64; rsi = (crate::simd_ops::ru64(rdi.wrapping_add(0x28u64))) as u64; r8 = ((rsi.wrapping_add(0x28u64)) as u64) as u64; fl_l = (r8) as u64 as u64; fl_r = (crate::simd_ops::ru64(rdi.wrapping_add(0x10u64))) as u64 as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 14; continue; } bb = 6; continue; }
            6 => { fl_l = (rsi) as u64 as u64; fl_r = (rdx) as u64 as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 11; continue; } bb = 7; continue; }
            7 => { bb = 8; continue; }
            8 => { fl_l = (r8) as u64 as u64; fl_r = (rdx) as u64 as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 11; continue; } bb = 9; continue; }
            9 => { rcx = (crate::simd_ops::ru32(rdi.wrapping_add(0x30u64))) as u32 as u64; rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x34u64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 11; continue; } bb = 10; continue; }
            10 => { rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0x20u64))) as u64; fl_l = (rsi) as u64 as u64; fl_r = (rdx) as u64 as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 8; continue; } bb = 11; continue; }
            11 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x10u64))) as u64; rax = ((rax).wrapping_sub(0x8u64)) as u64; fl_l = (rsi) as u64 as u64; fl_r = (rax) as u64 as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 13; continue; } bb = 12; continue; }
            12 => { rax = (0xffffffffffu64) as u64; crate::simd_ops::wu64(rsi, (rax) as u64); bb = 13; continue; }
            13 => { rsi = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; r8 = ((rsi.wrapping_add(0x28u64)) as u64) as u64; bb = 14; continue; }
            14 => { fl_l = (rsi) as u64 as u64; fl_r = (rdx) as u64 as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 18; continue; } bb = 15; continue; }
            15 => { fl_l = (r8) as u64 as u64; fl_r = (rdx) as u64 as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 18; continue; } bb = 16; continue; }
            16 => { rcx = (crate::simd_ops::ru32(rdi.wrapping_add(0x30u64))) as u32 as u64; rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x34u64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 18; continue; } bb = 17; continue; }
            17 => { rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0x20u64))) as u64; fl_l = (rsi) as u64 as u64; fl_r = (rdx) as u64 as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 15; continue; } bb = 18; continue; }
            18 => { rax = (0x69fafafafaffu64) as u64; crate::simd_ops::wu64(rsi, (rax) as u64); let tmp_xchg: u64 = crate::simd_ops::atomic_xchg_u64(rdi.wrapping_add(0x28u64), (r8) as u64); r8 = (tmp_xchg) as u64; rcx = ((rdi.wrapping_add(0x18u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1412f4032); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P6)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu32(rsi.wrapping_add(0x8u64), (0x0u32) as u32); rax = ((P7) as u64) as u64; crate::simd_ops::wu64(rsi.wrapping_add(0xcu64), (rax) as u64); xmm0 = (crate::simd_ops::ru128(r14)) as u128; crate::simd_ops::wu128(rsi.wrapping_add(0x14u64), (xmm0) as u128); rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x168u64))) as u64; crate::simd_ops::wu64(rsi, (0x1cu64) as u64); crate::simd_ops::lk_add_u32(rax.wrapping_add(0x30u64), (1) as u32); fl_l = (crate::simd_ops::ru8(rbp.wrapping_add(0x198u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 20; continue; } bb = 19; continue; }
            19 => { r8 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rdx = ((r8.wrapping_add(0x1u64)) as u32) as u32 as u64; rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x1a0u64))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1412f407d); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P8)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 20; continue; }
            20 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1412f43b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x58; const P1: u64 = 0x1401de830; const P2: u64 = 0x48; const P3: u64 = 0x14240ab10;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    fl_l = rdx; fl_r = r8;
    if (((fl_l as u64)) == ((fl_r as u64))) { return rax; }
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rsi = r8;
    rbx = rdx;
    loop { rcx = ((rbx.wrapping_add(P0)) as u64) as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rcx = ((rbx.wrapping_add(P2)) as u64) as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rcx = rbx; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rbx = ((rbx).wrapping_add(0x78u64)) as u64; fl_l = rbx; fl_r = rsi; if !((((fl_l as u64)) != ((fl_r as u64)))) { break; } }
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1412f4410(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x58; const P1: u64 = 0x98; const P2: u64 = 0x1411d7a20; const P3: u64 = 0x88; const P4: u64 = 0x8; const P5: u64 = 0x1401de400; const P6: u64 = 0xfffffffffffffff8; const P7: u64 = 0x141834ee8; const P8: u64 = 0x142442de0; const P9: u64 = 0x48; const P10: u64 = 0x14240ab10;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rcx;
    rdi = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    fl_l = rdi; fl_r = rdi;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rcx = ((rdi.wrapping_add(P1)) as u64) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rcx = ((rdi.wrapping_add(P3)) as u64) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rcx = ((rdi.wrapping_add(P4)) as u64) as u64; r9 = P5; rdx = 0x10; r8 = ((rdx.wrapping_add(P6)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); rcx = rdi; rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); }
    rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P9))) as u64;
    fl_l = rcx; fl_r = rcx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); }
    rcx = rbx;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return crate::simd_ops::tail_stk(P10, rax, rcx, rdx, r8, r9, rsp);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1412f47f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xc0; const P1: u64 = 0x10; const P2: u64 = 0x18; const P3: u64 = 0x142535070; const P4: u64 = 0x1426456c8; const P5: u64 = 0x1426453a0; const P6: u64 = 0x142428d70; const P7: u64 = 0x1433f29a0;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x38u64)) as u64; rax = ((crate::exe::tls_pointer() as u64)) as u64; rdx = 0x6348; rcx = (crate::simd_ops::ru64(rax)) as u64; rax = (crate::simd_ops::ru64(rdx.wrapping_add(rcx))) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64; r10 = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; fl_l = r10; fl_r = r10; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { fl_l = ((r10 as u8)) as u8 as u64; fl_r = (0xfu8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::lk_add_u32(rcx.wrapping_add(P2), (1) as u32)) as u32 as u64; rax = (crate::simd_ops::ru64(r10)) as u64; crate::simd_ops::wu64(rcx.wrapping_add(P1), rax); bb = 5; continue; }
            3 => { rdx = 0xb8; r8 = 0x10; rcx = rax; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); r10 = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp); rcx = rax; rdx = 0xdeadd00d; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp); r10 = 0; fl_l = 0; fl_r = 0; bb = 5; continue; }
            5 => { rcx = r10; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); r11 = P7; rax = r10; crate::simd_ops::wu64(r10, r11); rsp = ((rsp).wrapping_add(0x38u64)) as u64; return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1412f4880(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x142535360; const P2: u64 = 0x1426456c8; const P3: u64 = 0x1426453a0; const P4: u64 = 0x8; const P5: u64 = 0xc; const P6: u64 = 0x1433f29a0; const P7: u64 = 0x142428d70;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x28u64)) as u64; rdx = (((r9) as i64).wrapping_mul((0xb8u64) as i64) as u64) as u64; rcx = 0x11; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbx); fl_l = rdx; fl_r = rdx; rbx = r9; rax = ((rdx.wrapping_add(P0)) as u64) as u64; rdx = 0x10; rcx = (if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { rax } else { rcx }) as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); rcx = rax; rdx = 0xdeadd00d; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); rax = 0; fl_l = 0; fl_r = 0; bb = 2; continue; }
            2 => { crate::simd_ops::wu64(rax, (0xb8u64) as u64); r11 = ((rax.wrapping_add(P0)) as u64) as u64; crate::simd_ops::wu32(rax.wrapping_add(P4), ((rbx as u32)) as u32); crate::simd_ops::wu32(rax.wrapping_add(P5), (0x8u32) as u32); fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 3; continue; }
            3 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); r10 = r11; rdi = P6; bb = 4; continue; }
            4 => { rcx = r10; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(r10, rdi); r10 = ((r10).wrapping_add(0xb8u64)) as u64; fl_l = rbx; fl_r = (0x1u64) as u64; rbx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 4; continue; } bb = 5; continue; }
            5 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; bb = 6; continue; }
            6 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = r11; rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1412f4cf0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x140; const P1: u64 = 0x10; const P2: u64 = 0x18; const P3: u64 = 0x142535070; const P4: u64 = 0x1426456c8; const P5: u64 = 0x1426453a0; const P6: u64 = 0x8; const P7: u64 = 0xc; const P8: u64 = 0x1433f2940; const P9: u64 = 0x14183acc2; const P10: u64 = 0x1412f5870;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { r11 = rsp; crate::simd_ops::wu64(r11.wrapping_add(0x18u64), rbx); crate::simd_ops::wu64(r11.wrapping_add(0x20u64), rsi); crate::simd_ops::wu64(r11.wrapping_add(0x10u64), rdx); crate::simd_ops::wu64(r11.wrapping_add(0x8u64), rcx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x50u64)) as u64; rsi = rcx; crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), (0x0u32) as u32); rax = r11.wrapping_add(0x10); crate::simd_ops::wu64(r11.wrapping_sub(0x20u64), rax); crate::simd_ops::wu64(r11.wrapping_sub(0x18u64), (0x0u64) as u64); r9 = 0x6348; rax = ((crate::exe::tls_pointer() as u64)) as u64; rdx = (crate::simd_ops::ru64(rax)) as u64; r9 = (crate::simd_ops::ru64(r9.wrapping_add(rdx))) as u64; rcx = (crate::simd_ops::ru64(r9.wrapping_add(P0))) as u64; rdi = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; rax = rdi; rax = ((((rax as u32)) & (0xfu32))) as u32 as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::lk_add_u32(rcx.wrapping_add(P2), (1) as u32)) as u32 as u64; rax = (crate::simd_ops::ru64(rdi)) as u64; crate::simd_ops::wu64(rcx.wrapping_add(P1), rax); bb = 5; continue; }
            3 => { rdx = 0x138; r8 = 0x10; rcx = r9; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rdi = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp); rcx = rax; rdx = 0xdeadd00d; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp); bb = 5; continue; }
            5 => { crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rdi); crate::simd_ops::wu32(rdi.wrapping_add(P6), (0x1u32) as u32); crate::simd_ops::wu32(rdi.wrapping_add(P7), (0x1u32) as u32); rax = P8; crate::simd_ops::wu64(rdi, rax); rbx = ((rdi.wrapping_add(P1)) as u64) as u64; rdx = 0; fl_l = 0; fl_r = 0; r8 = 0x128; rcx = rbx; rax = crate::simd_ops::call_stk(P9, rax, rcx, rdx, r8, r9, rsp); rcx = rbx; rax = crate::simd_ops::call_stk(P10, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rsi.wrapping_add(P6), (0x0u64) as u64); crate::simd_ops::wu64(rsi, rbx); crate::simd_ops::wu64(rsi.wrapping_add(P6), rdi); rax = rsi; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; rsp = ((rsp).wrapping_add(0x50u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1412f4df0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143881d98; const P1: u64 = 0x14243b0b0; const P2: u64 = 0x1412f5940; const P3: u64 = 0x14028e490; const P4: u64 = 0x1406fe890; const P5: u64 = 0x1412f57d0; const P6: u64 = 0x1401e7b40; const P7: u64 = 0x1401e7b20; const P8: u64 = 0x1433f24b8; const P9: u64 = 0x140d37180; const P10: u64 = 0x140d38160; const P11: u64 = 0x1433f2498; const P12: u64 = 0x1412f4fd0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x1412f4df0, funcinfo_rva: 0x3696d74, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0x68, conts: &[] };
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rbp) as u64); crate::simd_ops::wu8(rsp.wrapping_add(0x18u64), ((r8 as u8)) as u8); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = ((rsp).wrapping_sub(0x50u64)) as u64; rbx = (rdx) as u64; rbp = (rcx) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), (0x3u32) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), (0x3u32) as u32); r8 = (crate::simd_ops::ru64(P0)) as u64; rdx = (rcx) as u64; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1412f4e2a); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x338u64))) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = ((rsp.wrapping_add(0x80u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1412f4e41); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x48u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x80u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x4u32) as u32 as u64; if ((((fl_l as u32)) as i32) > (((fl_r as u32)) as i32)) { bb = 12; continue; } bb = 1; continue; }
            1 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 4; continue; } bb = 2; continue; }
            2 => { rdi = (rbp) as u64; rsi = ((rax as u32)) as u32 as u64; bb = 3; continue; }
            3 => { rdx = (rbx) as u64; rcx = (rdi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1412f4e66); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rdi = ((rdi).wrapping_add(0x30u64)) as u64; fl_l = (rsi) as u64 as u64; fl_r = (0x1u64) as u64 as u64; rsi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } bb = 4; continue; }
            4 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x338u64))) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = ((rsp.wrapping_add(0x78u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1412f4e84); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x48u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rax = (((crate::simd_ops::ru32(rsp.wrapping_add(0x78u64))) as i32 as i64) as u64) as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x4u32) as u32 as u64; if ((((fl_l as u32)) as i32) > (((fl_r as u32)) as i32)) { bb = 11; continue; } bb = 5; continue; }
            5 => { rsi = (rax) as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 10; continue; } bb = 6; continue; }
            6 => { rdi = ((rbp.wrapping_add(0xc0u64)) as u64) as u64; r14 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; bb = 7; continue; }
            7 => { crate::simd_ops::wu32(rsp.wrapping_add(0x80u64), ((r14 as u32)) as u32); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x338u64))) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = ((rsp.wrapping_add(0x80u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1412f4eca); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x48u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rdx = (((crate::simd_ops::ru32(rsp.wrapping_add(0x80u64))) as i32 as i64) as u64) as u64; rcx = (rdi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1412f4ed8); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rdx = (((crate::simd_ops::ru32(rsp.wrapping_add(0x80u64))) as i32 as i64) as u64) as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 9; continue; } bb = 8; continue; }
            8 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x340u64))) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; r8 = (rdx) as u64; rdx = (crate::simd_ops::ru64(rdi)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1412f4ef9); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x28u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 9; continue; }
            9 => { rdi = ((rdi).wrapping_add(0x10u64)) as u64; fl_l = (rsi) as u64 as u64; fl_r = (0x1u64) as u64 as u64; rsi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 7; continue; } bb = 10; continue; }
            10 => { rcx = ((rbp.wrapping_add(0x100u64)) as u64) as u64; rdx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1412f4f10); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rcx = ((rbp.wrapping_add(0x118u64)) as u64) as u64; rdx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1412f4f1f); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rdx = (crate::simd_ops::ru32(rbx.wrapping_add(0x8u64))) as u32 as u64; rcx = ((rdx.wrapping_add(rdx.wrapping_mul(2))) as u64) as u64; rcx = ((rcx).wrapping_add(rcx)) as u64; rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = (crate::simd_ops::ru32(rax.wrapping_add(rcx.wrapping_mul(8)).wrapping_sub(0x8u64))) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x328u64), ((rcx as u32)) as u32); rax = ((rdx.wrapping_sub(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x8u64), ((rax as u32)) as u32); rax = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x88u64))) as u64; rsp = ((rsp).wrapping_add(0x50u64)) as u64; r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rsi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            11 => { r8 = (crate::simd_ops::ru64(rbx.wrapping_add(0x330u64))) as u64; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1412f4f63); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); r9 = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1412f4f6c); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P7) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (r8) as u64); r8 = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; rdx = ((P8) as u64) as u64; rcx = ((rsp.wrapping_add(0x40u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1412f4f86); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P9) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rcx = (rax) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1412f4f8f); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P10) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 12; continue; }
            12 => { r8 = (crate::simd_ops::ru64(rbx.wrapping_add(0x330u64))) as u64; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1412f4f9f); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); r9 = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1412f4fa8); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P7) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (r8) as u64); r8 = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; rdx = ((P11) as u64) as u64; rcx = ((rsp.wrapping_add(0x40u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1412f4fc2); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P9) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rcx = (rax) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1412f4fcb); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P10) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); crate::lift_ops::trap(0x80000003, P12, rcx, rsp); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1412f4fe0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142535070; const P1: u64 = 0x1426456c8; const P2: u64 = 0x1426453a0; const P3: u64 = 0x1433f2ab8; const P4: u64 = 0x143159a38; const P5: u64 = 0x143159a40; const P6: u64 = 0x143427178; const P7: u64 = 0x1401de400; const P8: u64 = 0x140f35bf0; const P9: u64 = 0x141834fc0; const P10: u64 = 0x1412cf770; const P11: u64 = 0x140200580;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdx); crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x50u64)) as u64; rsi = rcx; crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), (0x0u32) as u32); rax = rsp.wrapping_add(0x78); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), (0x0u64) as u64); r9 = 0x6348; rax = ((crate::exe::tls_pointer() as u64)) as u64; rdx = (crate::simd_ops::ru64(rax)) as u64; r9 = (crate::simd_ops::ru64(r9.wrapping_add(rdx))) as u64; rcx = (crate::simd_ops::ru64(r9.wrapping_add(0x120u64))) as u64; rdi = (crate::simd_ops::ru64(rcx.wrapping_add(0x10u64))) as u64; rax = rdi; rax = ((((rax as u32)) & (0xfu32))) as u32 as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::lk_add_u32(rcx.wrapping_add(0x18u64), (1) as u32)) as u32 as u64; rax = (crate::simd_ops::ru64(rdi)) as u64; crate::simd_ops::wu64(rcx.wrapping_add(0x10u64), rax); bb = 5; continue; }
            3 => { rdx = 0x118; r8 = 0x10; rcx = r9; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdi = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rax; rdx = 0xdeadd00d; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 5; continue; }
            5 => { crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rdi); crate::simd_ops::wu32(rdi.wrapping_add(0x8u64), (0x1u32) as u32); crate::simd_ops::wu32(rdi.wrapping_add(0xcu64), (0x1u32) as u32); rax = P3; crate::simd_ops::wu64(rdi, rax); rbx = rdi.wrapping_add(0x10); crate::simd_ops::wu64(rsp.wrapping_add(0x80u64), rbx); xmm0 = ((crate::simd_ops::ru64(P4)) as u64) as u128; crate::simd_ops::wu64(rbx, ((xmm0 as u64)) as u64); rax = (crate::simd_ops::ru32(P5)) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x8u64), ((rax as u32)) as u32); xmm0 = ((crate::simd_ops::ru32(P6)) as u32) as u128; crate::simd_ops::wu32(rbx.wrapping_add(0xcu64), ((xmm0 as u32)) as u32); crate::simd_ops::wu32(rbx.wrapping_add(0x10u64), ((xmm0 as u32)) as u32); crate::simd_ops::wu32(rbx.wrapping_add(0x14u64), ((xmm0 as u32)) as u32); crate::simd_ops::wu32(rbx.wrapping_add(0x18u64), (0x4u32) as u32); crate::simd_ops::wu32(rbx.wrapping_add(0x1cu64), (0x8u32) as u32); crate::simd_ops::wu32(rbx.wrapping_add(0x20u64), (0x80u32) as u32); crate::simd_ops::wu32(rbx.wrapping_add(0x24u64), (0x40800000u32) as u32); rcx = rbx.wrapping_add(0x28); rax = P7; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); r9 = P8; rdx = 0x30; r8 = ((rdx.wrapping_sub(0x2cu64)) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P9, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rbx.wrapping_add(0xe8); rax = P10; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); r9 = P11; rdx = 0x8; r8 = ((rdx.wrapping_sub(0x4u64)) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P9, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rsi.wrapping_add(0x8u64), (0x0u64) as u64); crate::simd_ops::wu64(rsi, rbx); crate::simd_ops::wu64(rsi.wrapping_add(0x8u64), rdi); rax = rsi; rsp = ((rsp).wrapping_add(0x50u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1412f5160(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142407c80; const P1: u64 = 0x143c4aa00; const P2: u64 = 0x14224ec60; const P3: u64 = 0x1426455a8; const P4: u64 = 0x142645598; const P5: u64 = 0x14224e3a0; const P6: u64 = 0x142645580;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; r14 = rdx; rbp = rcx; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rax = ((rax as u32)) as u32 as u64; rdx = P1; rdx = (crate::simd_ops::ru64(rdx.wrapping_add(rax.wrapping_mul(8)))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru32(rdx)) as u32 as u64; fl_r = 0x2; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 3; continue; } bb = 2; continue; }
            2 => { rcx = r14; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 17; continue; }
            3 => { rdi = (crate::simd_ops::ru64(rbp.wrapping_add(0x168u64))) as u64; rcx = rdi.wrapping_add(0x18); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0x20u64))) as u64; rsi = (crate::simd_ops::ru64(rdi.wrapping_add(0x28u64))) as u64; r8 = rsi.wrapping_add(0x48); fl_l = r8; fl_r = (crate::simd_ops::ru64(rdi.wrapping_add(0x10u64))) as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 11; continue; } bb = 4; continue; }
            4 => { fl_l = rsi; fl_r = rdx; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 8; continue; } bb = 5; continue; }
            5 => { fl_l = r8; fl_r = rdx; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 8; continue; } bb = 6; continue; }
            6 => { rcx = (crate::simd_ops::ru32(rdi.wrapping_add(0x30u64))) as u32 as u64; rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x34u64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 8; continue; } bb = 7; continue; }
            7 => { rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0x20u64))) as u64; fl_l = rsi; fl_r = rdx; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 5; continue; } bb = 8; continue; }
            8 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x10u64))) as u64; rax = ((rax).wrapping_sub(0x8u64)) as u64; fl_l = rsi; fl_r = rax; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 10; continue; } bb = 9; continue; }
            9 => { rax = (0xffffffffffu64) as u64; crate::simd_ops::wu64(rsi, rax); bb = 10; continue; }
            10 => { rsi = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; r8 = rsi.wrapping_add(0x48); bb = 11; continue; }
            11 => { fl_l = rsi; fl_r = rdx; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 15; continue; } bb = 12; continue; }
            12 => { fl_l = r8; fl_r = rdx; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 15; continue; } bb = 13; continue; }
            13 => { rcx = (crate::simd_ops::ru32(rdi.wrapping_add(0x30u64))) as u32 as u64; rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x34u64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 15; continue; } bb = 14; continue; }
            14 => { rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0x20u64))) as u64; fl_l = rsi; fl_r = rdx; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 12; continue; } bb = 15; continue; }
            15 => { rax = (0x69fafafafaffu64) as u64; crate::simd_ops::wu64(rsi, rax); let tmp_xchg: u64 = crate::simd_ops::atomic_xchg_u64(rdi.wrapping_add(0x28u64), r8); r8 = (tmp_xchg) as u64; rcx = rdi.wrapping_add(0x18); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp); rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rsi.wrapping_add(0x8u64), ((rcx as u32)) as u32); rax = P5; crate::simd_ops::wu64(rsi.wrapping_add(0xcu64), rax); rax = (crate::simd_ops::ru64(r14)) as u64; crate::simd_ops::wu64(rsi.wrapping_add(0x14u64), rax); crate::simd_ops::wu64(rsi.wrapping_add(0x1cu64), rcx); crate::simd_ops::wu64(rsi.wrapping_add(0x24u64), rcx); rax = (crate::simd_ops::ru64(r14.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(rsi.wrapping_add(0x1cu64), rax); rax = (crate::simd_ops::ru64(r14.wrapping_add(0x10u64))) as u64; crate::simd_ops::wu64(rsi.wrapping_add(0x24u64), rax); crate::simd_ops::wu64(r14.wrapping_add(0x8u64), rcx); crate::simd_ops::wu64(r14.wrapping_add(0x10u64), rcx); crate::simd_ops::wu64(rsi.wrapping_add(0x2cu64), rcx); crate::simd_ops::wu64(rsi.wrapping_add(0x34u64), rcx); rax = (crate::simd_ops::ru64(r14.wrapping_add(0x18u64))) as u64; crate::simd_ops::wu64(rsi.wrapping_add(0x2cu64), rax); rax = (crate::simd_ops::ru64(r14.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(rsi.wrapping_add(0x34u64), rax); crate::simd_ops::wu64(r14.wrapping_add(0x18u64), rcx); crate::simd_ops::wu64(r14.wrapping_add(0x20u64), rcx); rax = (crate::simd_ops::ru64(r14.wrapping_add(0x28u64))) as u64; crate::simd_ops::wu64(rsi.wrapping_add(0x3cu64), rax); rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x168u64))) as u64; crate::simd_ops::wu64(rsi, (0x3cu64) as u64); crate::simd_ops::lk_add_u32(rax.wrapping_add(0x30u64), (1) as u32); fl_l = (crate::simd_ops::ru8(rbp.wrapping_add(0x198u64))) as u8 as u64; fl_r = ((rcx as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 17; continue; } bb = 16; continue; }
            16 => { r8 = 0; fl_l = 0; fl_r = 0; rdx = ((rcx.wrapping_add(0x1u64)) as u32) as u32 as u64; rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x1a0u64))) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P6), rax), rax, rcx, rdx, r8, r9, rsp); bb = 17; continue; }
            17 => { rbx = (crate::simd_ops::ru64(r14.wrapping_add(0x20u64))) as u64; rdi = 0xffffffff; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 21; continue; } bb = 18; continue; }
            18 => { rax = ((rdi as u32)) as u32 as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rbx.wrapping_add(0x8u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 21; continue; } bb = 19; continue; }
            19 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rax = ((rdi as u32)) as u32 as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rbx.wrapping_add(0xcu64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 21; continue; } bb = 20; continue; }
            20 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 21; continue; }
            21 => { rbx = (crate::simd_ops::ru64(r14.wrapping_add(0x10u64))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 25; continue; } bb = 22; continue; }
            22 => { rax = ((rdi as u32)) as u32 as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rbx.wrapping_add(0x8u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 25; continue; } bb = 23; continue; }
            23 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); let xa_src: u32 = (rdi as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rbx.wrapping_add(0xcu64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rdi = (xa_old) as u32 as u64; fl_l = ((rdi as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 25; continue; } bb = 24; continue; }
            24 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 25; continue; }
            25 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1412f5360(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142407c80; const P1: u64 = 0x143c4aa00; const P2: u64 = 0x14397ff10; const P3: u64 = 0x14239af80; const P4: u64 = 0x142255e30; const P5: u64 = 0x1426455a8; const P6: u64 = 0x142645598; const P7: u64 = 0x14224e2e0; const P8: u64 = 0x142645580;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; r14 = rdx; rbp = rcx; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rax = ((rax as u32)) as u32 as u64; rdx = P1; rdx = (crate::simd_ops::ru64(rdx.wrapping_add(rax.wrapping_mul(8)))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru32(rdx)) as u32 as u64; fl_r = 0x2; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 3; continue; } bb = 2; continue; }
            2 => { rdi = (crate::simd_ops::ru64(P2)) as u64; rdx = (crate::simd_ops::ru64(r14.wrapping_add(0x18u64))) as u64; rcx = (crate::simd_ops::ru64(r14.wrapping_add(0x8u64))) as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); r8 = r14.wrapping_add(0x8); rdx = (crate::simd_ops::ru64(r14)) as u64; rcx = rdi; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); bb = 17; continue; }
            3 => { rdi = (crate::simd_ops::ru64(rbp.wrapping_add(0x168u64))) as u64; rcx = rdi.wrapping_add(0x18); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp); rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0x20u64))) as u64; rsi = (crate::simd_ops::ru64(rdi.wrapping_add(0x28u64))) as u64; r8 = rsi.wrapping_add(0x40); fl_l = r8; fl_r = (crate::simd_ops::ru64(rdi.wrapping_add(0x10u64))) as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 11; continue; } bb = 4; continue; }
            4 => { fl_l = rsi; fl_r = rdx; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 8; continue; } bb = 5; continue; }
            5 => { fl_l = r8; fl_r = rdx; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 8; continue; } bb = 6; continue; }
            6 => { rcx = (crate::simd_ops::ru32(rdi.wrapping_add(0x30u64))) as u32 as u64; rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x34u64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 8; continue; } bb = 7; continue; }
            7 => { rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0x20u64))) as u64; fl_l = rsi; fl_r = rdx; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 5; continue; } bb = 8; continue; }
            8 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x10u64))) as u64; rax = ((rax).wrapping_sub(0x8u64)) as u64; fl_l = rsi; fl_r = rax; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 10; continue; } bb = 9; continue; }
            9 => { rax = (0xffffffffffu64) as u64; crate::simd_ops::wu64(rsi, rax); bb = 10; continue; }
            10 => { rsi = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; r8 = rsi.wrapping_add(0x40); bb = 11; continue; }
            11 => { fl_l = rsi; fl_r = rdx; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 15; continue; } bb = 12; continue; }
            12 => { fl_l = r8; fl_r = rdx; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 15; continue; } bb = 13; continue; }
            13 => { rcx = (crate::simd_ops::ru32(rdi.wrapping_add(0x30u64))) as u32 as u64; rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x34u64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 15; continue; } bb = 14; continue; }
            14 => { rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0x20u64))) as u64; fl_l = rsi; fl_r = rdx; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 12; continue; } bb = 15; continue; }
            15 => { rax = (0x69fafafafaffu64) as u64; crate::simd_ops::wu64(rsi, rax); let tmp_xchg: u64 = crate::simd_ops::atomic_xchg_u64(rdi.wrapping_add(0x28u64), r8); r8 = (tmp_xchg) as u64; rcx = rdi.wrapping_add(0x18); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P6), rax), rax, rcx, rdx, r8, r9, rsp); rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rsi.wrapping_add(0x8u64), ((rcx as u32)) as u32); rax = P7; crate::simd_ops::wu64(rsi.wrapping_add(0xcu64), rax); rax = (crate::simd_ops::ru64(r14)) as u64; crate::simd_ops::wu64(rsi.wrapping_add(0x14u64), rax); crate::simd_ops::wu64(rsi.wrapping_add(0x1cu64), rcx); crate::simd_ops::wu64(rsi.wrapping_add(0x24u64), rcx); rax = (crate::simd_ops::ru64(r14.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(rsi.wrapping_add(0x1cu64), rax); rax = (crate::simd_ops::ru64(r14.wrapping_add(0x10u64))) as u64; crate::simd_ops::wu64(rsi.wrapping_add(0x24u64), rax); crate::simd_ops::wu64(r14.wrapping_add(0x8u64), rcx); crate::simd_ops::wu64(r14.wrapping_add(0x10u64), rcx); crate::simd_ops::wu64(rsi.wrapping_add(0x2cu64), rcx); crate::simd_ops::wu64(rsi.wrapping_add(0x34u64), rcx); rax = (crate::simd_ops::ru64(r14.wrapping_add(0x18u64))) as u64; crate::simd_ops::wu64(rsi.wrapping_add(0x2cu64), rax); rax = (crate::simd_ops::ru64(r14.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(rsi.wrapping_add(0x34u64), rax); crate::simd_ops::wu64(r14.wrapping_add(0x18u64), rcx); crate::simd_ops::wu64(r14.wrapping_add(0x20u64), rcx); rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x168u64))) as u64; crate::simd_ops::wu64(rsi, (0x34u64) as u64); crate::simd_ops::lk_add_u32(rax.wrapping_add(0x30u64), (1) as u32); fl_l = (crate::simd_ops::ru8(rbp.wrapping_add(0x198u64))) as u8 as u64; fl_r = ((rcx as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 17; continue; } bb = 16; continue; }
            16 => { r8 = 0; fl_l = 0; fl_r = 0; rdx = ((rcx.wrapping_add(0x1u64)) as u32) as u32 as u64; rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x1a0u64))) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P8), rax), rax, rcx, rdx, r8, r9, rsp); bb = 17; continue; }
            17 => { rbx = (crate::simd_ops::ru64(r14.wrapping_add(0x20u64))) as u64; rdi = 0xffffffff; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 21; continue; } bb = 18; continue; }
            18 => { rax = ((rdi as u32)) as u32 as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rbx.wrapping_add(0x8u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 21; continue; } bb = 19; continue; }
            19 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rax = ((rdi as u32)) as u32 as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rbx.wrapping_add(0xcu64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 21; continue; } bb = 20; continue; }
            20 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 21; continue; }
            21 => { rbx = (crate::simd_ops::ru64(r14.wrapping_add(0x10u64))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 25; continue; } bb = 22; continue; }
            22 => { rax = ((rdi as u32)) as u32 as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rbx.wrapping_add(0x8u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 25; continue; } bb = 23; continue; }
            23 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); let xa_src: u32 = (rdi as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rbx.wrapping_add(0xcu64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rdi = (xa_old) as u32 as u64; fl_l = ((rdi as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 25; continue; } bb = 24; continue; }
            24 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 25; continue; }
            25 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1412f5570(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142407c80; const P1: u64 = 0x143c4aa00; const P2: u64 = 0x14397ff10; const P3: u64 = 0x142255d10; const P4: u64 = 0x1426455a8; const P5: u64 = 0x142645598; const P6: u64 = 0x14224e2b0; const P7: u64 = 0x142645580;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rdx; rbp = rcx; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rax = ((rax as u32)) as u32 as u64; rdx = P1; rdx = (crate::simd_ops::ru64(rdx.wrapping_add(rax.wrapping_mul(8)))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru32(rdx)) as u32 as u64; fl_r = 0x2; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 3; continue; } bb = 2; continue; }
            2 => { rcx = (crate::simd_ops::ru64(P2)) as u64; rdx = rbx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(P3, rax, rcx, rdx, r8, r9, rsp); }
            3 => { crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rsi); rsi = (crate::simd_ops::ru64(rbp.wrapping_add(0x168u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rdi); crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), r14); rcx = rsi.wrapping_add(0x18); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp); rdx = (crate::simd_ops::ru64(rsi.wrapping_add(0x20u64))) as u64; r14 = (crate::simd_ops::ru64(rsi.wrapping_add(0x28u64))) as u64; r8 = r14.wrapping_add(0x20); fl_l = r8; fl_r = (crate::simd_ops::ru64(rsi.wrapping_add(0x10u64))) as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 12; continue; } bb = 4; continue; }
            4 => { fl_l = r14; fl_r = rdx; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 9; continue; } bb = 5; continue; }
            5 => { bb = 6; continue; }
            6 => { fl_l = r8; fl_r = rdx; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 9; continue; } bb = 7; continue; }
            7 => { rcx = (crate::simd_ops::ru32(rsi.wrapping_add(0x30u64))) as u32 as u64; rax = (crate::simd_ops::ru32(rsi.wrapping_add(0x34u64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 9; continue; } bb = 8; continue; }
            8 => { rdx = (crate::simd_ops::ru64(rsi.wrapping_add(0x20u64))) as u64; fl_l = r14; fl_r = rdx; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 6; continue; } bb = 9; continue; }
            9 => { rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x10u64))) as u64; rax = ((rax).wrapping_sub(0x8u64)) as u64; fl_l = r14; fl_r = rax; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 11; continue; } bb = 10; continue; }
            10 => { rax = (0xffffffffffu64) as u64; crate::simd_ops::wu64(r14, rax); bb = 11; continue; }
            11 => { r14 = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; r8 = r14.wrapping_add(0x20); bb = 12; continue; }
            12 => { fl_l = r14; fl_r = rdx; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 16; continue; } bb = 13; continue; }
            13 => { fl_l = r8; fl_r = rdx; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 16; continue; } bb = 14; continue; }
            14 => { rcx = (crate::simd_ops::ru32(rsi.wrapping_add(0x30u64))) as u32 as u64; rax = (crate::simd_ops::ru32(rsi.wrapping_add(0x34u64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 16; continue; } bb = 15; continue; }
            15 => { rdx = (crate::simd_ops::ru64(rsi.wrapping_add(0x20u64))) as u64; fl_l = r14; fl_r = rdx; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 13; continue; } bb = 16; continue; }
            16 => { rax = (0x69fafafafaffu64) as u64; rcx = rsi.wrapping_add(0x18); crate::simd_ops::wu64(r14, rax); let tmp_xchg: u64 = crate::simd_ops::atomic_xchg_u64(rsi.wrapping_add(0x28u64), r8); r8 = (tmp_xchg) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp); rax = P6; crate::simd_ops::wu32(r14.wrapping_add(0x8u64), (0x0u32) as u32); crate::simd_ops::wu64(r14.wrapping_add(0xcu64), rax); crate::simd_ops::wu64(r14.wrapping_add(0x14u64), rbx); rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x168u64))) as u64; crate::simd_ops::wu64(r14, (0x14u64) as u64); crate::simd_ops::lk_add_u32(rax.wrapping_add(0x30u64), (1) as u32); fl_l = (crate::simd_ops::ru8(rbp.wrapping_add(0x198u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 18; continue; } bb = 17; continue; }
            17 => { rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x1a0u64))) as u64; r8 = 0; fl_l = 0; fl_r = 0; rdx = ((r8.wrapping_add(0x1u64)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P7), rax), rax, rcx, rdx, r8, r9, rsp); bb = 18; continue; }
            18 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1412f56d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x14239ac30; const P2: u64 = 0xe8; const P3: u64 = 0x1412cf770; const P4: u64 = 0xfffffffffffffffc; const P5: u64 = 0x141834ee8; const P6: u64 = 0x28; const P7: u64 = 0x1401de400; const P8: u64 = 0xffffffffffffffd4;
    let (mut rax, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = ((rcx.wrapping_add(P0)) as u64) as u64;
    rcx = rbx;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rcx = ((rbx.wrapping_add(P2)) as u64) as u64;
    r9 = P3;
    rdx = 0x8;
    r8 = ((rdx.wrapping_add(P4)) as u32) as u32 as u64;
    rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp);
    rcx = ((rbx.wrapping_add(P6)) as u64) as u64;
    r9 = P7;
    rdx = 0x30;
    r8 = ((rdx.wrapping_add(P8)) as u32) as u32 as u64;
    rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1412f5720(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0xc0; const P2: u64 = 0x1401de400; const P3: u64 = 0xfffffffffffffff4; const P4: u64 = 0x141834ee8; const P5: u64 = 0xffffffffffffffd4;
    let (mut rax, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = ((rcx.wrapping_add(P0)) as u64) as u64;
    rcx = ((rbx.wrapping_add(P1)) as u64) as u64;
    r9 = P2;
    rdx = 0x10;
    r8 = ((rdx.wrapping_add(P3)) as u32) as u32 as u64;
    rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp);
    r9 = P2;
    rdx = 0x30;
    r8 = ((rdx.wrapping_add(P5)) as u32) as u32 as u64;
    rcx = rbx;
    rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1412f57a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1433f2940; const P1: u64 = 0x141834d30;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rax = P0;
    rbx = rcx;
    crate::simd_ops::wu64(rcx, rax);
    fl_l = ((rdx as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64;
    if !(((((fl_l as u8)) & ((fl_r as u8))) == 0)) { rdx = 0x138; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); }
    rax = rbx;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1412f5870(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1401de400; const P1: u64 = 0x140f35bf0; const P2: u64 = 0xffffffffffffffd4; const P3: u64 = 0x141834fc0; const P4: u64 = 0xc0; const P5: u64 = 0x1402013d0; const P6: u64 = 0xfffffffffffffff4; const P7: u64 = 0x143159a38; const P8: u64 = 0x100; const P9: u64 = 0x143159a40; const P10: u64 = 0x108; const P11: u64 = 0x143427178;
    let (mut rax, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rbx = rcx; rax = P0; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); r9 = P1; rdx = 0x30; r8 = ((rdx.wrapping_add(P2)) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = ((rbx.wrapping_add(P4)) as u64) as u64; rax = P0; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); r9 = P5; rdx = 0x10; r8 = ((rdx.wrapping_add(P6)) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm0 = ((crate::simd_ops::ru64(P7)) as u64) as u128; crate::simd_ops::wu64(rbx.wrapping_add(P8), ((xmm0 as u64)) as u64); rax = (crate::simd_ops::ru32(P9)) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(P10), ((rax as u32)) as u32); xmm0 = ((crate::simd_ops::ru32(P11)) as u32) as u128; crate::simd_ops::wu32(rbx.wrapping_add(0x10cu64), ((xmm0 as u32)) as u32); crate::simd_ops::wu32(rbx.wrapping_add(0x110u64), ((xmm0 as u32)) as u32); crate::simd_ops::wu32(rbx.wrapping_add(0x114u64), ((xmm0 as u32)) as u32); crate::simd_ops::wu32(rbx.wrapping_add(0x118u64), (0x4u32) as u32); crate::simd_ops::wu32(rbx.wrapping_add(0x11cu64), (0x8u32) as u32); crate::simd_ops::wu32(rbx.wrapping_add(0x120u64), (0x80u32) as u32); crate::simd_ops::wu32(rbx.wrapping_add(0x124u64), (0x40800000u32) as u32); rax = rbx; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1412f5940(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143881da0; const P1: u64 = 0x14243b0b0; const P2: u64 = 0x14086b290;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); crate::simd_ops::wu8(rsp.wrapping_add(0x18u64), ((r8 as u8)) as u8); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; r8 = (crate::simd_ops::ru64(P0)) as u64; rsi = rdx; rdx = rcx; crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), (0x1u32) as u32); r14 = rcx; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), (0x1u32) as u32); rcx = rsi; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rsi.wrapping_add(0x338u64))) as u64; rdx = rsp.wrapping_add(0x60); crate::simd_ops::wu32(rsp.wrapping_add(0x60u64), (0x0u32) as u32); rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x48u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rdx = (((crate::simd_ops::ru32(rsp.wrapping_add(0x60u64))) as i32 as i64) as u64) as u64; rcx = r14; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rax = (((crate::simd_ops::ru32(rsp.wrapping_add(0x60u64))) as i32 as i64) as u64) as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rcx = (crate::simd_ops::ru64(rsi.wrapping_add(0x340u64))) as u64; r8 = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rdx = (crate::simd_ops::ru64(r14)) as u64; r8 = ((r8).wrapping_shl(2u32)) as u64; r9 = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(r9.wrapping_add(0x28u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 2; continue; }
            2 => { rdi = (crate::simd_ops::ru64(rsi.wrapping_add(0x338u64))) as u64; rdx = r14.wrapping_add(0x10); rcx = rdi; rax = (crate::simd_ops::ru64(rdi)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x48u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rdi)) as u64; rdx = r14.wrapping_add(0x14); rcx = rdi; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x48u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rdi)) as u64; rdx = r14.wrapping_add(0x18); rcx = rdi; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x48u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rdi = (crate::simd_ops::ru64(rsi.wrapping_add(0x338u64))) as u64; rdx = r14.wrapping_add(0x1c); rcx = rdi; rax = (crate::simd_ops::ru64(rdi)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x48u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rdi)) as u64; rdx = r14.wrapping_add(0x20); rcx = rdi; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x48u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rdi)) as u64; rdx = r14.wrapping_add(0x24); rcx = rdi; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x48u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rsi.wrapping_add(0x338u64))) as u64; rdx = r14.wrapping_add(0x28); rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x48u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rdx = (crate::simd_ops::ru32(rsi.wrapping_add(0x8u64))) as u32 as u64; rax = (crate::simd_ops::ru64(rsi)) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rcx = ((rdx.wrapping_add(rdx.wrapping_mul(2))) as u64) as u64; rcx = ((rcx).wrapping_add(rcx)) as u64; rcx = (crate::simd_ops::ru32(rax.wrapping_add(rcx.wrapping_mul(8)).wrapping_sub(0x8u64))) as u32 as u64; rax = ((rdx.wrapping_sub(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu32(rsi.wrapping_add(0x8u64), ((rax as u32)) as u32); rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rsi.wrapping_add(0x328u64), ((rcx as u32)) as u32); rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1412f5a70(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1433f2b18; const P1: u64 = 0x143a08298; const P2: u64 = 0x14240d480; const P3: u64 = 0x14397ff08; const P4: u64 = 0x28; const P5: u64 = 0x1401de830; const P6: u64 = 0x18; const P7: u64 = 0x141834d30;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = ((rdx as u32)) as u32 as u64;
    rdi = rcx;
    rax = P0;
    crate::simd_ops::wu64(rcx, rax);
    rdx = rcx;
    rcx = (crate::simd_ops::ru64(P1)) as u64;
    rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp);
    crate::simd_ops::wu64(P3, (0x0u64) as u64);
    rcx = ((rdi.wrapping_add(P4)) as u64) as u64;
    rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp);
    rcx = ((rdi.wrapping_add(P6)) as u64) as u64;
    rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp);
    fl_l = ((rbx as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64;
    if !(((((fl_l as u8)) & ((fl_r as u8))) == 0)) { rdx = 0x38; rcx = rdi; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); }
    rax = rdi;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1412f5b70(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x19; const P2: u64 = 0x1c; const P3: u64 = 0x10; const P4: u64 = 0x1401fc200; const P5: u64 = 0x20; const P6: u64 = 0x18; const P7: u64 = 0x140208500; const P8: u64 = 0x140200130;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x50u64)) as u64; rsi = rdx; rdi = rcx; rbx = (crate::simd_ops::ru64(rcx)) as u64; rax = (crate::simd_ops::ru64(rbx.wrapping_add(P0))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); rbp = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rsp.wrapping_add(0x38u64), ((rbp as u32)) as u32); r8 = rbx; fl_l = (crate::simd_ops::ru8(rax.wrapping_add(P1))) as u8 as u64; fl_r = ((rbp as u8)) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 6; continue; } bb = 1; continue; }
            1 => { r9 = (crate::simd_ops::ru32(rdx)) as u32 as u64; bb = 2; continue; }
            2 => { crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); fl_l = (crate::simd_ops::ru32(rax.wrapping_add(P2))) as u32 as u64; fl_r = ((r9 as u32)) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 4; continue; } bb = 3; continue; }
            3 => { crate::simd_ops::wu32(rsp.wrapping_add(0x38u64), ((rbp as u32)) as u32); rax = (crate::simd_ops::ru64(rax.wrapping_add(P3))) as u64; bb = 5; continue; }
            4 => { crate::simd_ops::wu32(rsp.wrapping_add(0x38u64), (0x1u32) as u32); r8 = rax; rax = (crate::simd_ops::ru64(rax)) as u64; bb = 5; continue; }
            5 => { fl_l = (crate::simd_ops::ru8(rax.wrapping_add(P1))) as u8 as u64; fl_r = ((rbp as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 2; continue; } bb = 6; continue; }
            6 => { fl_l = (crate::simd_ops::ru8(r8.wrapping_add(P1))) as u8 as u64; fl_r = ((rbp as u8)) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 8; continue; } bb = 7; continue; }
            7 => { rax = (crate::simd_ops::ru32(r8.wrapping_add(P2))) as u32 as u64; fl_l = (crate::simd_ops::ru32(rdx)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 10; continue; } bb = 8; continue; }
            8 => { rax = (0x666666666666666u64) as u64; fl_l = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 11; continue; } bb = 9; continue; }
            9 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rbp); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r8 = rax; rax = (crate::simd_ops::ru32(rsi)) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(P2), ((rax as u32)) as u32); crate::simd_ops::wu32(r8.wrapping_add(P5), ((rbp as u32)) as u32); crate::simd_ops::wu64(r8, rbx); crate::simd_ops::wu64(r8.wrapping_add(P0), rbx); crate::simd_ops::wu64(r8.wrapping_add(P3), rbx); crate::simd_ops::wu16(r8.wrapping_add(P6), (0x0u16) as u16); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x30u64))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128); rdx = rsp.wrapping_add(0x20); rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r8 = rax; bb = 10; continue; }
            10 => { rax = ((r8.wrapping_add(P5)) as u64) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rsp = ((rsp).wrapping_add(0x50u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            11 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P8, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::lift_ops::trap(0x80000003, 0x1412f5c5f, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1412f5c80(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0xffffffffffffffff; const P2: u64 = 0x108; const P3: u64 = 0x4; const P4: u64 = 0x100; const P5: u64 = 0xe; const P6: u64 = 0x20; const P7: u64 = 0x18; const P8: u64 = 0x1425371a0; const P9: u64 = 0x142533200; const P10: u64 = 0x8; const P11: u64 = 0xc;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rcx; r9 = (crate::simd_ops::ru64(rcx)) as u64; fl_l = r9; fl_r = r9; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 1; continue; }
            1 => { rax = ((rcx.wrapping_add(P0)) as u64) as u64; fl_l = r9; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 9; continue; } bb = 2; continue; }
            2 => { rcx = ((r9.wrapping_add(P1)) as u64) as u64; rcx = (((rcx) & (0xfffffffffe000000u64))) as u64; rax = r9; rax = ((rax).wrapping_sub(rcx)) as u64; rax = ((rax).wrapping_shr(16u32)) as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rdx = ((rcx.wrapping_add(P2)) as u64) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(P3))) as u32 as u64; rdx = ((rdx).wrapping_sub(rax)) as u64; rax = ((crate::exe::teb_self() as u64)) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(rcx.wrapping_add(P4))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 7; continue; } bb = 3; continue; }
            3 => { fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(P5))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 6; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(rdx.wrapping_add(P6))) as u64; crate::simd_ops::wu64(r9, rax); crate::simd_ops::wu64(rdx.wrapping_add(P6), r9); fl_l = (crate::simd_ops::lk_add_u32(rdx.wrapping_add(P7), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; if (((fl_l as u32)) != 0) { bb = 9; continue; } bb = 5; continue; }
            5 => { rcx = rdx; rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); bb = 9; continue; }
            6 => { r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 8; continue; }
            7 => { r8 = 0; fl_l = 0; fl_r = 0; bb = 8; continue; }
            8 => { rax = crate::simd_ops::call_stk(P9, rax, rcx, rdx, r8, r9, rsp); bb = 9; continue; }
            9 => { rax = ((rbx.wrapping_add(P0)) as u64) as u64; crate::simd_ops::wu64(rbx, rax); crate::simd_ops::wu32(rbx.wrapping_add(P10), (0x0u32) as u32); crate::simd_ops::wu32(rbx.wrapping_add(P11), (0x4u32) as u32); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1412f5d40(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14307e950; const P1: u64 = 0x142645870; const P2: u64 = 0x1401fc200; const P3: u64 = 0x143426fc8; const P4: u64 = 0x140695aa0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; r15 = r8; r14 = rdx; rsi = rcx; rcx = ((crate::simd_ops::ru8(r8)) as u32) as u32 as u64; rax = (0xcbf29ce484222325u64) as u64; rcx = (((rcx) ^ (rax))) as u64; rdx = (0x100000001b3u64) as u64; rcx = (((rcx) as i64).wrapping_mul((rdx) as i64) as u64) as u64; rax = ((crate::simd_ops::ru8(r8.wrapping_add(0x1u64))) as u32) as u32 as u64; rcx = (((rcx) ^ (rax))) as u64; rcx = (((rcx) as i64).wrapping_mul((rdx) as i64) as u64) as u64; rax = ((crate::simd_ops::ru8(r8.wrapping_add(0x2u64))) as u32) as u32 as u64; rcx = (((rcx) ^ (rax))) as u64; rcx = (((rcx) as i64).wrapping_mul((rdx) as i64) as u64) as u64; rax = ((crate::simd_ops::ru8(r8.wrapping_add(0x3u64))) as u32) as u32 as u64; rcx = (((rcx) ^ (rax))) as u64; rcx = (((rcx) as i64).wrapping_mul((rdx) as i64) as u64) as u64; rax = ((crate::simd_ops::ru8(r8.wrapping_add(0x4u64))) as u32) as u32 as u64; rcx = (((rcx) ^ (rax))) as u64; rcx = (((rcx) as i64).wrapping_mul((rdx) as i64) as u64) as u64; rax = ((crate::simd_ops::ru8(r8.wrapping_add(0x5u64))) as u32) as u32 as u64; rcx = (((rcx) ^ (rax))) as u64; rcx = (((rcx) as i64).wrapping_mul((rdx) as i64) as u64) as u64; rax = ((crate::simd_ops::ru8(r8.wrapping_add(0x6u64))) as u32) as u32 as u64; rcx = (((rcx) ^ (rax))) as u64; rcx = (((rcx) as i64).wrapping_mul((rdx) as i64) as u64) as u64; rax = ((crate::simd_ops::ru8(r8.wrapping_add(0x7u64))) as u32) as u32 as u64; rcx = (((rcx) ^ (rax))) as u64; rcx = (((rcx) as i64).wrapping_mul((rdx) as i64) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x80u64), rcx); r8 = rsi.wrapping_add(0x30); rax = (crate::simd_ops::ru64(r8)) as u64; rax = (((rax) & (rcx))) as u64; rax = ((rax).wrapping_add(rax)) as u64; rcx = (crate::simd_ops::ru64(rsi.wrapping_add(0x18u64))) as u64; rbx = (crate::simd_ops::ru64(rcx.wrapping_add(rax.wrapping_mul(8)).wrapping_add(0x8u64))) as u64; rdx = rsi.wrapping_add(0x8); r12 = (crate::simd_ops::ru64(rdx)) as u64; fl_l = rbx; fl_r = r12; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rbx = r12; crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), r8); rbp = r12; bb = 8; continue; }
            2 => { rcx = (crate::simd_ops::ru64(rcx.wrapping_add(rax.wrapping_mul(8)))) as u64; rax = (crate::simd_ops::ru64(r15)) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 5; continue; } bb = 3; continue; }
            3 => { fl_l = rbx; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 7; continue; } bb = 4; continue; }
            4 => { rbx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } bb = 5; continue; }
            5 => { crate::simd_ops::wu64(r14, rbx); crate::simd_ops::wu8(r14.wrapping_add(0x8u64), (0x0u8) as u8); bb = 6; continue; }
            6 => { rax = r14; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            7 => { r12 = rbx; rax = rsi.wrapping_add(0x30); crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), rax); rbp = rbx; bb = 8; continue; }
            8 => { r13 = rsi.wrapping_add(0x18); rax = (0x666666666666666u64) as u64; fl_l = (crate::simd_ops::ru64(rsi.wrapping_add(0x10u64))) as u64; fl_r = rax; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 10; continue; } bb = 9; continue; }
            9 => { rcx = P0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::lift_ops::trap(0x80000003, 0x1412f5e73, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            10 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdx); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (0x0u64) as u64); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdi = rax; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax); rcx = (crate::simd_ops::ru64(r15)) as u64; crate::simd_ops::wu64(rax.wrapping_add(0x10u64), rcx); rdx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rax.wrapping_add(0x18u64), rdx); crate::simd_ops::wu64(rax.wrapping_add(0x20u64), rdx); rcx = (crate::simd_ops::ru64(r15.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(rax.wrapping_add(0x18u64), rcx); rax = (crate::simd_ops::ru64(r15.wrapping_add(0x10u64))) as u64; crate::simd_ops::wu64(rdi.wrapping_add(0x20u64), rax); crate::simd_ops::wu64(r15.wrapping_add(0x8u64), rdx); crate::simd_ops::wu64(r15.wrapping_add(0x10u64), rdx); rcx = (crate::simd_ops::ru64(rsi.wrapping_add(0x10u64))) as u64; xmm2 = ((crate::simd_ops::ru32(rsi)) as u32) as u128; fl_l = ((rcx).wrapping_add(0x1u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; xmm1 = 0u128; if ((((fl_l as u64)) as i64) < 0) { bb = 12; continue; } bb = 11; continue; }
            11 => { xmm1 = (xmm1 & !0xffff_ffffu128) | ((((rcx) as i64) as f32).to_bits() as u128); bb = 13; continue; }
            12 => { rax = rcx; rax = ((rax).wrapping_shr(1u32)) as u64; rcx = ((((rcx as u32)) & (0x1u32))) as u32 as u64; fl_l = (((rax) | (rcx))) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; xmm1 = (xmm1 & !0xffff_ffffu128) | ((((rax) as i64) as f32).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) + f32::from_bits(xmm1 as u32)).to_bits() as u128); bb = 13; continue; }
            13 => { rdx = (crate::simd_ops::ru64(rsi.wrapping_add(0x38u64))) as u64; xmm0 = 0u128; fl_l = rdx; fl_r = rdx; if (((((fl_l as u64)) & ((fl_r as u64))) as i64) < 0) { bb = 15; continue; } bb = 14; continue; }
            14 => { xmm0 = (xmm0 & !0xffff_ffffu128) | ((((rdx) as i64) as f32).to_bits() as u128); bb = 16; continue; }
            15 => { rcx = rdx; rcx = ((rcx).wrapping_shr(1u32)) as u64; rax = rdx; rax = ((((rax as u32)) & (0x1u32))) as u32 as u64; fl_l = (((rcx) | (rax))) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; xmm0 = (xmm0 & !0xffff_ffffu128) | ((((rcx) as i64) as f32).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) + f32::from_bits(xmm0 as u32)).to_bits() as u128); bb = 16; continue; }
            16 => { xmm0 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) / f32::from_bits(xmm0 as u32)).to_bits() as u128); fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = ((xmm2 as u32)) as u32 as u64; rax = (rax & !0xffu64) | (((if (f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { 1u8 } else { 0u8 }) as u8 as u64)); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 32; continue; } bb = 17; continue; }
            17 => { xmm0 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) / f32::from_bits(xmm2 as u32)).to_bits() as u128); xmm1 = 0u128; xmm1 = crate::simd_ops::vroundss(xmm1, (xmm0 as u32), 0x2u8); rcx = 0; fl_l = 0; fl_r = 0; xmm0 = ((crate::simd_ops::ru32(P3)) as u32) as u128; fl_l = ((xmm1 as u32)) as u32 as u64; fl_r = ((xmm0 as u32)) as u32 as u64; if !(f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { bb = 20; continue; } bb = 18; continue; }
            18 => { xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) - f32::from_bits(xmm0 as u32)).to_bits() as u128); fl_l = ((xmm1 as u32)) as u32 as u64; fl_r = ((xmm0 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { bb = 20; continue; } bb = 19; continue; }
            19 => { rax = (0x8000000000000000u64) as u64; rcx = rax; bb = 20; continue; }
            20 => { rax = (crate::simd_ops::cvt_f32_i64((f32::from_bits(xmm1 as u32)))) as u64; rax = ((rax).wrapping_add(rcx)) as u64; rcx = 0x8; fl_l = rax; fl_r = rcx; rcx = (if (((fl_l as u64)) > ((fl_r as u64))) { rax } else { rcx }) as u64; fl_l = rdx; fl_r = rcx; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 24; continue; } bb = 21; continue; }
            21 => { fl_l = rdx; fl_r = (0x200u64) as u64; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 23; continue; } bb = 22; continue; }
            22 => { rdx = ((rdx.wrapping_mul(8)) as u64) as u64; fl_l = rdx; fl_r = rcx; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 24; continue; } bb = 23; continue; }
            23 => { rdx = rcx; bb = 24; continue; }
            24 => { rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru64(rsi.wrapping_add(0x30u64))) as u64; r8 = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; rcx = (((rcx) & (r8))) as u64; rcx = ((rcx).wrapping_add(rcx)) as u64; rdx = (crate::simd_ops::ru64(rsi.wrapping_add(0x18u64))) as u64; rax = (crate::simd_ops::ru64(rdx.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x8u64))) as u64; rbx = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; fl_l = rax; fl_r = rbx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 26; continue; } bb = 25; continue; }
            25 => { crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (0x0u64) as u64); bb = 30; continue; }
            26 => { rdx = (crate::simd_ops::ru64(rdx.wrapping_add(rcx.wrapping_mul(8)))) as u64; rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x10u64))) as u64; fl_l = rcx; fl_r = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 29; continue; } bb = 27; continue; }
            27 => { rbx = rax; fl_l = rax; fl_r = rdx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 31; continue; } bb = 28; continue; }
            28 => { rax = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; fl_l = rcx; fl_r = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 27; continue; } bb = 29; continue; }
            29 => { rbx = (crate::simd_ops::ru64(rax)) as u64; bb = 30; continue; }
            30 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbx); rbp = rbx; r13 = rsi.wrapping_add(0x18); rax = rsi.wrapping_add(0x30); r12 = rbx; bb = 33; continue; }
            31 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (0x0u64) as u64); rbp = rbx; r13 = rsi.wrapping_add(0x18); rax = rsi.wrapping_add(0x30); r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; bb = 33; continue; }
            32 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; r8 = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; bb = 33; continue; }
            33 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; crate::simd_ops::lk_add_u64(rsi.wrapping_add(0x10u64), (1) as u64); crate::simd_ops::wu64(rdi, r12); crate::simd_ops::wu64(rdi.wrapping_add(0x8u64), rdx); crate::simd_ops::wu64(rdx, rdi); crate::simd_ops::wu64(rbx.wrapping_add(0x8u64), rdi); rcx = (crate::simd_ops::ru64(r13)) as u64; rax = (crate::simd_ops::ru64(rax)) as u64; rax = (((rax) & (r8))) as u64; rax = ((rax).wrapping_add(rax)) as u64; r8 = (crate::simd_ops::ru64(rcx.wrapping_add(rax.wrapping_mul(8)))) as u64; fl_l = r8; fl_r = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 35; continue; } bb = 34; continue; }
            34 => { crate::simd_ops::wu64(rcx.wrapping_add(rax.wrapping_mul(8)), rdi); bb = 38; continue; }
            35 => { fl_l = r8; fl_r = rbp; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 37; continue; } bb = 36; continue; }
            36 => { crate::simd_ops::wu64(rcx.wrapping_add(rax.wrapping_mul(8)), rdi); bb = 39; continue; }
            37 => { fl_l = (crate::simd_ops::ru64(rcx.wrapping_add(rax.wrapping_mul(8)).wrapping_add(0x8u64))) as u64; fl_r = rdx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 39; continue; } bb = 38; continue; }
            38 => { crate::simd_ops::wu64(rcx.wrapping_add(rax.wrapping_mul(8)).wrapping_add(0x8u64), rdi); bb = 39; continue; }
            39 => { crate::simd_ops::wu64(r14, rdi); crate::simd_ops::wu8(r14.wrapping_add(0x8u64), (0x1u8) as u8); bb = 6; continue; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1412f6080(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x20; const P2: u64 = 0xc; const P3: u64 = 0x1401e66f0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; crate::simd_ops::wu64(rax, (0x0u64) as u64); rdi = (crate::simd_ops::ru64(rdx)) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rsi); bb = 2; continue; }
            2 => { rbx = (crate::simd_ops::ru64(rdi.wrapping_add(P1))) as u64; rsi = (crate::simd_ops::ru64(rdi)) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 3; continue; }
            3 => { rax = 0xffffffff; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rbx.wrapping_add(P0), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 6; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rax = 0xffffffff; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rbx.wrapping_add(P2), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 6; continue; }
            6 => { rdx = rdi; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rdi = rsi; fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 2; continue; } bb = 7; continue; }
            7 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; bb = 8; continue; }
            8 => { rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1412f6100(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x20; const P2: u64 = 0xc; const P3: u64 = 0x1401fb4d0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rsi = rcx; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 1; continue; }
            1 => { rbx = (crate::simd_ops::ru64(rbx.wrapping_add(P1))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rdi); rdi = 0xffffffff; rax = ((rdi as u32)) as u32 as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rbx.wrapping_add(P0), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 5; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); let xa_src: u32 = (rdi as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rbx.wrapping_add(P2), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rdi = (xa_old) as u32 as u64; fl_l = ((rdi as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 5; continue; }
            5 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; bb = 6; continue; }
            6 => { rcx = rsi; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(P3, rax, rcx, rdx, r8, r9, rsp); }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1412f62f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14183acc8; const P1: u64 = 0x1401ffc30; const P2: u64 = 0x1401de830;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; r13 = r9; rbx = r8; r14 = rdx; rdi = rcx; rcx = (crate::simd_ops::ru64(rcx)) as u64; rbx = ((rbx).wrapping_sub(rcx)) as u64; rbx = ((((rbx) as i64).wrapping_shr(5u32) as u64)) as u64; r12 = (crate::simd_ops::ru32(rdi.wrapping_add(0x8u64))) as u32 as u64; rdx = ((r12 as u32)) as u32 as u64; rdx = ((rdx).wrapping_shl(5u32)) as u64; r8 = (crate::simd_ops::ru32(rdi.wrapping_add(0xcu64))) as u32 as u64; rsi = rbx; rsi = ((rsi).wrapping_shl(5u32)) as u64; rax = ((r8 as u32)) as u32 as u64; rax = ((rax).wrapping_shl(5u32)) as u64; fl_l = rdx; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 4; continue; } bb = 1; continue; }
            1 => { rbx = ((rsi.wrapping_add(rcx)) as u64) as u64; rax = ((rdx.wrapping_add(rcx)) as u64) as u64; fl_l = rbx; fl_r = rax; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } bb = 2; continue; }
            2 => { let y_addr: u64 = r9; xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rdx.wrapping_add(rcx); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); fl_l = (crate::simd_ops::lk_add_u32(rdi.wrapping_add(0x8u64), (1) as u32)) as u32 as u64; bb = 7; continue; }
            3 => { let y_addr: u64 = rax.wrapping_sub(0x20u64); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rax; crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); rcx = (crate::simd_ops::ru32(rdi.wrapping_add(0x8u64))) as u32 as u64; rcx = ((rcx).wrapping_shl(5u32)) as u64; rdx = (crate::simd_ops::ru64(rdi)) as u64; rax = rcx; rax = ((rax).wrapping_sub(rbx)) as u64; r8 = rdx.wrapping_sub(0x20); r8 = ((r8).wrapping_add(rax)) as u64; rcx = ((rcx).wrapping_sub(r8)) as u64; rcx = ((rcx).wrapping_add(rdx)) as u64; rdx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); let y_addr: u64 = r13; xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rbx; crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); fl_l = (crate::simd_ops::lk_add_u32(rdi.wrapping_add(0x8u64), (1) as u32)) as u32 as u64; bb = 7; continue; }
            4 => { r12 = ((r12).wrapping_add(1)) as u64; rbp = r8; rbp = ((rbp).wrapping_shr(1u32)) as u64; rbp = ((rbp).wrapping_add(r8)) as u64; fl_l = rbp; fl_r = r12; rbp = (if (((fl_l as u64)) < ((fl_r as u64))) { r12 } else { rbp }) as u64; rdx = rbp; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r15 = rax; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { r8 = rsi; rdx = (crate::simd_ops::ru64(rdi)) as u64; rcx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 6; continue; }
            6 => { rbx = ((rsi.wrapping_add(r15)) as u64) as u64; let y_addr: u64 = r13; xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rbx; crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); rax = (crate::simd_ops::ru64(rdi)) as u64; rdx = ((rsi.wrapping_add(rax)) as u64) as u64; r8 = (crate::simd_ops::ru32(rdi.wrapping_add(0x8u64))) as u32 as u64; r8 = ((r8).wrapping_shl(5u32)) as u64; r8 = ((r8).wrapping_sub(rdx)) as u64; r8 = ((r8).wrapping_add(rax)) as u64; rcx = rbx.wrapping_add(0x20); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rdi, r15); crate::simd_ops::wu32(rdi.wrapping_add(0x8u64), ((r12 as u32)) as u32); crate::simd_ops::wu32(rdi.wrapping_add(0xcu64), ((rbp as u32)) as u32); bb = 7; continue; }
            7 => { rax = r14; crate::simd_ops::wu64(r14, rbx); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1412f6440(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x20; const P1: u64 = 0x8; const P2: u64 = 0xc; const P3: u64 = 0x1401e66f0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rsi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    rsi = rdx;
    fl_l = rbx; fl_r = rbx;
    if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { rdx = rsi; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(P3, rax, rcx, rdx, r8, r9, rsp); }
    crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rdi);
    rdi = 0xffffffff;
    rax = ((rdi as u32)) as u32 as u64;
    let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rbx.wrapping_add(P1), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64;
    rax = (xa_old) as u32 as u64;
    fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1;
    if (((fl_l as u32)) != ((fl_r as u32))) { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rdx = rsi; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(P3, rax, rcx, rdx, r8, r9, rsp); }
    rax = (crate::simd_ops::ru64(rbx)) as u64;
    rcx = rbx;
    rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp);
    let xa_src: u32 = (rdi as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rbx.wrapping_add(P2), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64;
    rdi = (xa_old) as u32 as u64;
    fl_l = ((rdi as u32)) as u32 as u64; fl_r = 0x1;
    if !((((fl_l as u32)) != ((fl_r as u32)))) { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); }
    rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rdx = rsi;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rsi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return crate::simd_ops::tail_stk(P3, rax, rcx, rdx, r8, r9, rsp);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1412f64a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142537280; const P1: u64 = 0x142535070; const P2: u64 = 0x1426456c8; const P3: u64 = 0x1426453a0; const P4: u64 = 0x1412f6170;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rax = ((crate::exe::tls_pointer() as u64)) as u64; rsi = rdx; rbp = (crate::simd_ops::ru32(rcx.wrapping_add(0x8u64))) as u32 as u64; rdx = ((rdx.wrapping_add(rdx.wrapping_mul(4))) as u64) as u64; r10 = 0x6348; r14 = 0; fl_l = 0; fl_r = 0; rdx = ((rdx).wrapping_shl(3u32)) as u64; rdi = rcx; r8 = (crate::simd_ops::ru64(rax)) as u64; fl_l = ((rdx) & ((rdx).wrapping_sub(1))) as u64; rax = ((fl_l as u64)) as u64; r10 = (crate::simd_ops::ru64(r10.wrapping_add(r8))) as u64; if (((fl_l as u64)) != 0) { bb = 5; continue; } bb = 1; continue; }
            1 => { rax = rdx.wrapping_sub(0x10); fl_l = rax; fl_r = (0x3f0u64) as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 5; continue; } bb = 2; continue; }
            2 => { rax = rdx; rax = ((rax).wrapping_shr(3u32)) as u64; r8 = (crate::simd_ops::ru64(r10.wrapping_add(rax.wrapping_mul(8)).wrapping_add(0x8u64))) as u64; rbx = (crate::simd_ops::ru64(r8.wrapping_add(0x10u64))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { r9 = 0; fl_l = 0; fl_r = 0; r8 = 0; fl_l = 0; fl_r = 0; rcx = r10; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); bb = 12; continue; }
            4 => { fl_l = (crate::simd_ops::lk_add_u32(r8.wrapping_add(0x18u64), (1) as u32)) as u32 as u64; rax = (crate::simd_ops::ru64(rbx)) as u64; crate::simd_ops::wu64(r8.wrapping_add(0x10u64), rax); bb = 13; continue; }
            5 => { rax = (0x7fffffffffffffffu64) as u64; fl_l = rdx; fl_r = rax; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 7; continue; } bb = 6; continue; }
            6 => { rbx = r14; bb = 13; continue; }
            7 => { rax = rdx.wrapping_sub(0x10); fl_l = rax; fl_r = (0x3f0u64) as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 11; continue; } bb = 8; continue; }
            8 => { rax = rdx; rax = ((rax).wrapping_shr(3u32)) as u64; rcx = (crate::simd_ops::ru64(r10.wrapping_add(rax.wrapping_mul(8)).wrapping_add(0x8u64))) as u64; rbx = (crate::simd_ops::ru64(rcx.wrapping_add(0x10u64))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 9; continue; }
            9 => { fl_l = ((rbx as u8)) as u8 as u64; fl_r = (0xfu8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 11; continue; } bb = 10; continue; }
            10 => { fl_l = (crate::simd_ops::lk_add_u32(rcx.wrapping_add(0x18u64), (1) as u32)) as u32 as u64; rax = (crate::simd_ops::ru64(rbx)) as u64; crate::simd_ops::wu64(rcx.wrapping_add(0x10u64), rax); bb = 13; continue; }
            11 => { r8 = 0x10; rcx = r10; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 12; continue; }
            12 => { rbx = rax; bb = 13; continue; }
            13 => { fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 15; continue; } bb = 14; continue; }
            14 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); rcx = rax; rdx = 0xdeadd00d; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); rbx = r14; bb = 15; continue; }
            15 => { r8 = (crate::simd_ops::ru64(rdi)) as u64; rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x8u64))) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; r9 = ((r8.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = r8; fl_r = r9; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 18; continue; } bb = 16; continue; }
            16 => { rdx = r8.wrapping_add(0x20); rcx = rbx.wrapping_add(0x20); bb = 17; continue; }
            17 => { rax = (crate::simd_ops::ru64(r8)) as u64; rdx = rdx.wrapping_add(0x28); crate::simd_ops::wu64(rcx.wrapping_sub(0x18u64), r14); r8 = ((r8).wrapping_add(0x28u64)) as u64; crate::simd_ops::wu64(rcx.wrapping_sub(0x10u64), r14); crate::simd_ops::wu64(rcx.wrapping_sub(0x20u64), rax); rcx = rcx.wrapping_add(0x28); rax = (crate::simd_ops::ru64(rdx.wrapping_sub(0x40u64))) as u64; crate::simd_ops::wu64(rcx.wrapping_sub(0x40u64), rax); rax = (crate::simd_ops::ru64(rdx.wrapping_sub(0x38u64))) as u64; crate::simd_ops::wu64(rcx.wrapping_sub(0x38u64), rax); crate::simd_ops::wu64(rdx.wrapping_sub(0x40u64), r14); crate::simd_ops::wu64(rdx.wrapping_sub(0x38u64), r14); crate::simd_ops::wu64(rcx.wrapping_sub(0x30u64), r14); crate::simd_ops::wu64(rcx.wrapping_sub(0x28u64), r14); rax = (crate::simd_ops::ru64(rdx.wrapping_sub(0x30u64))) as u64; crate::simd_ops::wu64(rcx.wrapping_sub(0x30u64), rax); rax = (crate::simd_ops::ru64(rdx.wrapping_sub(0x28u64))) as u64; crate::simd_ops::wu64(rcx.wrapping_sub(0x28u64), rax); crate::simd_ops::wu64(rdx.wrapping_sub(0x30u64), r14); crate::simd_ops::wu64(rdx.wrapping_sub(0x28u64), r14); fl_l = r8; fl_r = r9; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 17; continue; } bb = 18; continue; }
            18 => { rcx = rdi; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rdi, rbx); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x8u64), ((rbp as u32)) as u32); rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; crate::simd_ops::wu32(rdi.wrapping_add(0xcu64), ((rsi as u32)) as u32); rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1412f67b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x228; const P1: u64 = 0x10; const P2: u64 = 0x18; const P3: u64 = 0x14204f540; const P4: u64 = 0x142535070; const P5: u64 = 0x1426456c8; const P6: u64 = 0x1426453a0;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x1412f67b0, funcinfo_rva: 0x369725c, handler: 1, shape: 2, frame_reg: 0, establisher_off: 0x48, conts: &[] };
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (rsi) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rdi) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rdx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rsi = (r9) as u64; r14 = (r8) as u64; rdi = (rcx) as u64; rbx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rcx = (0x6348u32) as u32 as u64; rax = ((crate::exe::tls_pointer() as u64)) as u64; rdx = (crate::simd_ops::ru64(rax)) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(rdx))) as u64; r10 = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rdx = (crate::simd_ops::ru64(r10.wrapping_add(P1))) as u64; rax = (rdx) as u64; rax = ((((rax as u32)) & (0xfu32))) as u32 as u64; fl_l = (rdx) as u64 as u64; fl_r = (rdx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 1; continue; }
            1 => { fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 5; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::lk_add_u32(r10.wrapping_add(P2), (1) as u32)) as u32 as u64; rax = (crate::simd_ops::ru64(rdx)) as u64; crate::simd_ops::wu64(r10.wrapping_add(P1), (rax) as u64); bb = 3; continue; }
            3 => { rbx = (rdx) as u64; bb = 4; continue; }
            4 => { crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), (rbx) as u64); r8 = (rsi) as u64; rdx = (r14) as u64; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1412f6821); crate::simd_ops::replace_rax_in(rax); rax = match eh.call(0x1412f6821, (P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8) { Ok(v) => v, Err(l) => l.unknown(0x1412f67b0) }; crate::simd_ops::wu64(rdi, (rax) as u64); rax = (rdi) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            5 => { rdx = (0x220u32) as u32 as u64; r8 = (0x10u32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1412f684d); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rdx = (rax) as u64; fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 3; continue; } bb = 6; continue; }
            6 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1412f685a); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P5)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rcx = (rax) as u64; rdx = (0xdeadd00du32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1412f6868); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P6)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 4; continue; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1412f6a00(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142257ee0; const P1: u64 = 0x141834d30;
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
    if !(((((fl_l as u8)) & ((fl_r as u8))) == 0)) { rdx = 0x38; rcx = rdi; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); }
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rax = rdi;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1412f6a40(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x48; const P1: u64 = 0x10; const P2: u64 = 0x40; const P3: u64 = 0x142537280; const P4: u64 = 0x1426456c8; const P5: u64 = 0x1426453a0; const P6: u64 = 0x18; const P7: u64 = 0x14204fce0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x1412f6a40, funcinfo_rva: 0x36972c4, handler: 1, shape: 2, frame_reg: 0, establisher_off: 0x28, conts: &[] };
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rcx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = (r9) as u64; rdi = (r8) as u64; rdx = (0x6348u32) as u32 as u64; rax = ((crate::exe::tls_pointer() as u64)) as u64; rcx = (crate::simd_ops::ru64(rax)) as u64; rax = (crate::simd_ops::ru64(rdx.wrapping_add(rcx))) as u64; rdx = (crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64; rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P1))) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { r9 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; r8 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rdx = ((rcx.wrapping_add(P2)) as u32) as u32 as u64; rcx = (rax) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1412f6a83); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rcx = (rax) as u64; fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 4; continue; } bb = 2; continue; }
            2 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1412f6a90); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P4)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rcx = (rax) as u64; rdx = (0xdeadd00du32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1412f6a9e); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P5)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rcx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; bb = 4; continue; }
            3 => { fl_l = (crate::simd_ops::lk_add_u32(rdx.wrapping_add(P6), (1) as u32)) as u32 as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; crate::simd_ops::wu64(rdx.wrapping_add(P1), (rax) as u64); bb = 4; continue; }
            4 => { crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (rcx) as u64); r8 = (crate::simd_ops::ru64(rbx)) as u64; rdx = (rdi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1412f6abd); crate::simd_ops::replace_rax_in(rax); rax = match eh.call(0x1412f6abd, (P7) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8) { Ok(v) => v, Err(l) => l.unknown(0x1412f6a40) }; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1412f6b70(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14240a730; const P1: u64 = 0x14240ab10;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; r14 = rcx; rdi = (crate::simd_ops::ru64(rcx)) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x8u64))) as u32 as u64; rsi = (((rax) as i64).wrapping_mul((0x68u64) as i64) as u64) as u64; rsi = ((rsi).wrapping_add(rdi)) as u64; fl_l = rdi; fl_r = rsi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 10; continue; } bb = 1; continue; }
            1 => { bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru64(rdi)) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rdi = ((rdi).wrapping_add(0x68u64)) as u64; fl_l = rdi; fl_r = rsi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 4; continue; }
            4 => { fl_l = rdi; fl_r = rsi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 10; continue; } bb = 5; continue; }
            5 => { rbx = rdi.wrapping_add(0x68); fl_l = rbx; fl_r = rsi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 10; continue; } bb = 6; continue; }
            6 => { rbp = rdi.wrapping_add(0x3c); bb = 7; continue; }
            7 => { rax = (crate::simd_ops::ru64(rbx)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 8; continue; }
            8 => { crate::simd_ops::wu64(rdi, rax); let y_addr: u64 = rbx.wrapping_add(0x8u64); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rbp.wrapping_sub(0x34u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); xmm1 = (crate::simd_ops::ru128(rbx.wrapping_add(0x28u64))) as u128; crate::simd_ops::wu128(rbp.wrapping_sub(0x14u64), (xmm1) as u128); rax = ((crate::simd_ops::ru8(rbx.wrapping_add(0x38u64))) as u32) as u32 as u64; crate::simd_ops::wu8(rbp.wrapping_sub(0x4u64), ((rax as u8)) as u8); rax = ((crate::simd_ops::ru8(rbx.wrapping_add(0x39u64))) as u32) as u32 as u64; crate::simd_ops::wu8(rbp.wrapping_sub(0x3u64), ((rax as u8)) as u8); rdx = rbx.wrapping_add(0x3c); rcx = rbp; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x64u64))) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x28u64), ((rax as u32)) as u32); rdi = ((rdi).wrapping_add(0x68u64)) as u64; fl_l = ((rbp).wrapping_add(0x68u64)) as u64; fl_r = fl_l; rbp = ((fl_l as u64)) as u64; bb = 9; continue; }
            9 => { rbx = ((rbx).wrapping_add(0x68u64)) as u64; fl_l = rbx; fl_r = rsi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 7; continue; } bb = 10; continue; }
            10 => { rbx = (crate::simd_ops::ru64(r14)) as u64; rax = (crate::simd_ops::ru32(r14.wrapping_add(0x8u64))) as u32 as u64; rsi = (((rax) as i64).wrapping_mul((0x68u64) as i64) as u64) as u64; rsi = ((rsi).wrapping_add(rbx)) as u64; rcx = rsi; rcx = ((rcx).wrapping_sub(rdi)) as u64; rax = (0x4ec4ec4ec4ec4ec5u64) as u64; let prod = (rax as i64 as i128).wrapping_mul((rcx) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; rbp = rdx; rbp = ((((rbp) as i64).wrapping_shr(5u32) as u64)) as u64; rax = rbp; rax = ((rax).wrapping_shr(63u32)) as u64; rbp = ((rbp).wrapping_add(rax)) as u64; fl_l = rdi; fl_r = rsi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 18; continue; } bb = 11; continue; }
            11 => { fl_l = rdi; fl_r = rbx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 15; continue; } bb = 12; continue; }
            12 => { fl_l = rbx; fl_r = rsi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 14; continue; } bb = 13; continue; }
            13 => { rcx = rbx.wrapping_add(0x3c); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = ((rbx).wrapping_add(0x68u64)) as u64; fl_l = rbx; fl_r = rsi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 13; continue; } bb = 14; continue; }
            14 => { crate::simd_ops::wu32(r14.wrapping_add(0x8u64), (0x0u32) as u32); bb = 18; continue; }
            15 => { rax = (crate::simd_ops::ru32(r14.wrapping_add(0x8u64))) as u32 as u64; rbx = (((rax) as i64).wrapping_mul((0x68u64) as i64) as u64) as u64; rbx = ((rbx).wrapping_add(crate::simd_ops::ru64(r14))) as u64; fl_l = rdi; fl_r = rbx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 17; continue; } bb = 16; continue; }
            16 => { rcx = rdi.wrapping_add(0x3c); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdi = ((rdi).wrapping_add(0x68u64)) as u64; fl_l = rdi; fl_r = rbx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 16; continue; } bb = 17; continue; }
            17 => { fl_l = (crate::simd_ops::ru32(r14.wrapping_add(0x8u64))) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; crate::simd_ops::wu32(r14.wrapping_add(0x8u64), ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32); bb = 18; continue; }
            18 => { rax = rbp; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1412f6cc0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x3c; const P2: u64 = 0x14240ab10; const P3: u64 = 0xffffffffffffffff; const P4: u64 = 0x108; const P5: u64 = 0x4; const P6: u64 = 0x100; const P7: u64 = 0xe; const P8: u64 = 0x20; const P9: u64 = 0x18; const P10: u64 = 0x1425371a0; const P11: u64 = 0x142533200;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = (crate::simd_ops::ru64(rcx)) as u64; rdi = rcx; rax = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; rsi = (((rax) as i64).wrapping_mul((0x68u64) as i64) as u64) as u64; rsi = ((rsi).wrapping_add(rbx)) as u64; fl_l = rbx; fl_r = rsi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rcx = ((rbx.wrapping_add(P1)) as u64) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rbx = ((rbx).wrapping_add(0x68u64)) as u64; fl_l = rbx; fl_r = rsi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } bb = 2; continue; }
            2 => { r9 = (crate::simd_ops::ru64(rdi)) as u64; fl_l = r9; fl_r = r9; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 3; continue; }
            3 => { rcx = ((r9.wrapping_add(P3)) as u64) as u64; rax = r9; rcx = (((rcx) & (0xfffffffffe000000u64))) as u64; rax = ((rax).wrapping_sub(rcx)) as u64; rax = ((rax).wrapping_shr(16u32)) as u64; rdx = ((rcx.wrapping_add(P4)) as u64) as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(P5))) as u32 as u64; rdx = ((rdx).wrapping_sub(rax)) as u64; rax = ((crate::exe::teb_self() as u64)) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(rcx.wrapping_add(P6))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 8; continue; } bb = 4; continue; }
            4 => { fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(P7))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 7; continue; } bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru64(rdx.wrapping_add(P8))) as u64; crate::simd_ops::wu64(r9, rax); fl_l = (crate::simd_ops::lk_add_u32(rdx.wrapping_add(P9), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; crate::simd_ops::wu64(rdx.wrapping_add(P8), r9); if (((fl_l as u32)) != 0) { bb = 10; continue; } bb = 6; continue; }
            6 => { rcx = rdx; rax = crate::simd_ops::call_stk(P10, rax, rcx, rdx, r8, r9, rsp); bb = 10; continue; }
            7 => { r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 9; continue; }
            8 => { r8 = 0; fl_l = 0; fl_r = 0; bb = 9; continue; }
            9 => { rax = crate::simd_ops::call_stk(P11, rax, rcx, rdx, r8, r9, rsp); bb = 10; continue; }
            10 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = 0; fl_l = 0; fl_r = 0; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; crate::simd_ops::wu64(rdi, rax); crate::simd_ops::wu64(rdi.wrapping_add(0x8u64), rax); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1412f6d90(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14240ab10; const P1: u64 = 0x14240a730;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = r9; rbx = r8; r12 = rdx; r13 = rcx; fl_l = r8; fl_r = r9; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 14; continue; } bb = 1; continue; }
            1 => { rsi = (crate::simd_ops::ru64(rcx)) as u64; r15 = rcx.wrapping_add(0x8); fl_l = rbx; fl_r = rsi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 6; continue; } bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru32(r15)) as u32 as u64; rbp = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rbp = ((rbp).wrapping_shl(4u32)) as u64; rbp = ((rbp).wrapping_add(rsi)) as u64; fl_l = r9; fl_r = rbp; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 6; continue; } bb = 3; continue; }
            3 => { fl_l = rsi; fl_r = rbp; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 5; continue; } bb = 4; continue; }
            4 => { rcx = rsi.wrapping_add(0x20); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rsi = ((rsi).wrapping_add(0x50u64)) as u64; fl_l = rsi; fl_r = rbp; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 4; continue; } bb = 5; continue; }
            5 => { crate::simd_ops::wu32(r15, (0x0u32) as u32); crate::simd_ops::wu64(r12, rbx); bb = 15; continue; }
            6 => { r14 = rbx; rax = (crate::simd_ops::ru32(r15)) as u32 as u64; rbp = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rbp = ((rbp).wrapping_shl(4u32)) as u64; rbp = ((rbp).wrapping_add(rsi)) as u64; fl_l = rdi; fl_r = rbp; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 10; continue; } bb = 7; continue; }
            7 => { rsi = r8.wrapping_add(0x1d); r15 = rdi; fl_l = r15; fl_r = rbx; r15 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; bb = 8; continue; }
            8 => { rax = (crate::simd_ops::ru64(rsi.wrapping_add(r15).wrapping_sub(0x1du64))) as u64; crate::simd_ops::wu64(r14, rax); xmm0 = (crate::simd_ops::ru128(r15.wrapping_add(rsi).wrapping_sub(0x15u64))) as u128; crate::simd_ops::wu128(rsi.wrapping_sub(0x15u64), (xmm0) as u128); rax = (crate::simd_ops::ru32(r15.wrapping_add(rsi).wrapping_sub(0x5u64))) as u32 as u64; crate::simd_ops::wu32(rsi.wrapping_sub(0x5u64), ((rax as u32)) as u32); rax = ((crate::simd_ops::ru8(r15.wrapping_add(rsi).wrapping_sub(0x1u64))) as u32) as u32 as u64; crate::simd_ops::wu8(rsi.wrapping_sub(0x1u64), ((rax as u8)) as u8); rax = ((crate::simd_ops::ru8(r15.wrapping_add(rsi))) as u32) as u32 as u64; crate::simd_ops::wu8(rsi, ((rax as u8)) as u8); rdx = r15.wrapping_add(0x3); rdx = ((rdx).wrapping_add(rsi)) as u64; rcx = rsi.wrapping_add(0x3); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru32(r15.wrapping_add(rsi).wrapping_add(0x2bu64))) as u32 as u64; crate::simd_ops::wu32(rsi.wrapping_add(0x2bu64), ((rax as u32)) as u32); r14 = ((r14).wrapping_add(0x50u64)) as u64; rsi = ((rsi).wrapping_add(0x50u64)) as u64; rax = r15.wrapping_sub(0x1d); rax = ((rax).wrapping_add(rsi)) as u64; fl_l = rax; fl_r = rbp; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 8; continue; } bb = 9; continue; }
            9 => { r15 = r13.wrapping_add(0x8); bb = 10; continue; }
            10 => { rax = (crate::simd_ops::ru32(r15)) as u32 as u64; rsi = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rsi = ((rsi).wrapping_shl(4u32)) as u64; rsi = ((rsi).wrapping_add(crate::simd_ops::ru64(r13))) as u64; fl_l = r14; fl_r = rsi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 13; continue; } bb = 11; continue; }
            11 => { rcx = r14.wrapping_add(0x20); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r14 = ((r14).wrapping_add(0x50u64)) as u64; fl_l = r14; fl_r = rsi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 11; continue; } bb = 12; continue; }
            12 => { r15 = r13.wrapping_add(0x8); bb = 13; continue; }
            13 => { rdi = ((rdi).wrapping_sub(rbx)) as u64; rax = (0x9999999999999999u64) as u64; let prod = (rax as i64 as i128).wrapping_mul((rdi) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; rdx = ((((rdx) as i64).wrapping_shr(5u32) as u64)) as u64; rax = rdx; rax = ((rax).wrapping_shr(63u32)) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; fl_l = (crate::simd_ops::lk_add_u32(r15, ((rdx as u32)) as u32)) as u32 as u64; fl_r = fl_l; crate::simd_ops::wu64(r12, rbx); bb = 15; continue; }
            14 => { crate::simd_ops::wu64(rdx, rbx); bb = 15; continue; }
            15 => { rax = r12; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1412f6f00(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x20; const P2: u64 = 0x14240ab10; const P3: u64 = 0xffffffffffffffff; const P4: u64 = 0x108; const P5: u64 = 0x4; const P6: u64 = 0x100; const P7: u64 = 0xe; const P8: u64 = 0x18; const P9: u64 = 0x1425371a0; const P10: u64 = 0x142533200;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = (crate::simd_ops::ru64(rcx)) as u64; rdi = rcx; rax = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; rsi = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rsi = ((rsi).wrapping_shl(4u32)) as u64; rsi = ((rsi).wrapping_add(rbx)) as u64; fl_l = rbx; fl_r = rsi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 3; continue; } bb = 1; continue; }
            1 => { bb = 2; continue; }
            2 => { rcx = ((rbx.wrapping_add(P1)) as u64) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rbx = ((rbx).wrapping_add(0x50u64)) as u64; fl_l = rbx; fl_r = rsi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 3; continue; }
            3 => { r9 = (crate::simd_ops::ru64(rdi)) as u64; fl_l = r9; fl_r = r9; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 4; continue; }
            4 => { rcx = ((r9.wrapping_add(P3)) as u64) as u64; rax = r9; rcx = (((rcx) & (0xfffffffffe000000u64))) as u64; rax = ((rax).wrapping_sub(rcx)) as u64; rax = ((rax).wrapping_shr(16u32)) as u64; rdx = ((rcx.wrapping_add(P4)) as u64) as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(P5))) as u32 as u64; rdx = ((rdx).wrapping_sub(rax)) as u64; rax = ((crate::exe::teb_self() as u64)) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(rcx.wrapping_add(P6))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 9; continue; } bb = 5; continue; }
            5 => { fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(P7))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 8; continue; } bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru64(rdx.wrapping_add(P1))) as u64; crate::simd_ops::wu64(r9, rax); fl_l = (crate::simd_ops::lk_add_u32(rdx.wrapping_add(P8), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; crate::simd_ops::wu64(rdx.wrapping_add(P1), r9); if (((fl_l as u32)) != 0) { bb = 11; continue; } bb = 7; continue; }
            7 => { rcx = rdx; rax = crate::simd_ops::call_stk(P9, rax, rcx, rdx, r8, r9, rsp); bb = 11; continue; }
            8 => { r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 10; continue; }
            9 => { r8 = 0; fl_l = 0; fl_r = 0; bb = 10; continue; }
            10 => { rax = crate::simd_ops::call_stk(P10, rax, rcx, rdx, r8, r9, rsp); bb = 11; continue; }
            11 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = 0; fl_l = 0; fl_r = 0; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; crate::simd_ops::wu64(rdi, rax); crate::simd_ops::wu64(rdi.wrapping_add(P0), rax); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1412f6fd0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142537280; const P1: u64 = 0x142535070; const P2: u64 = 0x1426456c8; const P3: u64 = 0x1426453a0; const P4: u64 = 0x1412f73f0; const P5: u64 = 0x1412f6cc0;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; r15 = rdx; rsi = rcx; r12 = (crate::simd_ops::ru32(rcx.wrapping_add(0x8u64))) as u32 as u64; rdx = (((rdx) as i64).wrapping_mul((0x68u64) as i64) as u64) as u64; r10 = 0x6348; rax = ((crate::exe::tls_pointer() as u64)) as u64; r8 = (crate::simd_ops::ru64(rax)) as u64; r10 = (crate::simd_ops::ru64(r10.wrapping_add(r8))) as u64; r14 = 0; fl_l = 0; fl_r = 0; fl_l = ((rdx) & ((rdx).wrapping_sub(1))) as u64; rax = ((fl_l as u64)) as u64; if (((fl_l as u64)) != 0) { bb = 5; continue; } bb = 1; continue; }
            1 => { rax = rdx.wrapping_sub(0x10); fl_l = rax; fl_r = (0x3f0u64) as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 5; continue; } bb = 2; continue; }
            2 => { rax = rdx.wrapping_add(0x7); rax = ((rax).wrapping_shr(3u32)) as u64; r9 = (crate::simd_ops::ru64(r10.wrapping_add(rax.wrapping_mul(8)).wrapping_add(0x8u64))) as u64; r8 = (crate::simd_ops::ru64(r9.wrapping_add(0x10u64))) as u64; fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { r9 = 0; fl_l = 0; fl_r = 0; rcx = r10; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 12; continue; }
            4 => { fl_l = (crate::simd_ops::lk_add_u32(r9.wrapping_add(0x18u64), (1) as u32)) as u32 as u64; rax = (crate::simd_ops::ru64(r8)) as u64; crate::simd_ops::wu64(r9.wrapping_add(0x10u64), rax); bb = 13; continue; }
            5 => { rax = (0x7fffffffffffffffu64) as u64; fl_l = rdx; fl_r = rax; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 7; continue; } bb = 6; continue; }
            6 => { r8 = r14; bb = 13; continue; }
            7 => { rax = rdx.wrapping_sub(0x10); fl_l = rax; fl_r = (0x3f0u64) as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 11; continue; } bb = 8; continue; }
            8 => { rax = rdx.wrapping_add(0x7); rax = ((rax).wrapping_shr(3u32)) as u64; rcx = (crate::simd_ops::ru64(r10.wrapping_add(rax.wrapping_mul(8)).wrapping_add(0x8u64))) as u64; r8 = (crate::simd_ops::ru64(rcx.wrapping_add(0x10u64))) as u64; rax = r8; rax = ((((rax as u32)) & (0xfu32))) as u32 as u64; fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 9; continue; }
            9 => { fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 11; continue; } bb = 10; continue; }
            10 => { fl_l = (crate::simd_ops::lk_add_u32(rcx.wrapping_add(0x18u64), (1) as u32)) as u32 as u64; rax = (crate::simd_ops::ru64(r8)) as u64; crate::simd_ops::wu64(rcx.wrapping_add(0x10u64), rax); bb = 13; continue; }
            11 => { r8 = 0x10; rcx = r10; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 12; continue; }
            12 => { r8 = rax; bb = 13; continue; }
            13 => { fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 15; continue; } bb = 14; continue; }
            14 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rax; rdx = 0xdeadd00d; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 16; continue; }
            15 => { r14 = r8; bb = 16; continue; }
            16 => { rbx = (crate::simd_ops::ru64(rsi)) as u64; rax = (crate::simd_ops::ru32(rsi.wrapping_add(0x8u64))) as u32 as u64; rbp = (((rax) as i64).wrapping_mul((0x68u64) as i64) as u64) as u64; rbp = ((rbp).wrapping_add(rbx)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), r14); rdi = r14; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), r14); fl_l = rbx; fl_r = rbp; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 19; continue; } bb = 17; continue; }
            17 => { bb = 18; continue; }
            18 => { rax = (crate::simd_ops::ru64(rbx)) as u64; crate::simd_ops::wu64(rdi, rax); let y_addr: u64 = rbx.wrapping_add(0x8u64); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rdi.wrapping_add(0x8u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); xmm1 = (crate::simd_ops::ru128(rbx.wrapping_add(0x28u64))) as u128; crate::simd_ops::wu128(rdi.wrapping_add(0x28u64), (xmm1) as u128); rcx = rdi.wrapping_add(0x38); rdx = rbx.wrapping_add(0x38); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdi = ((rdi).wrapping_add(0x68u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rdi); rbx = ((rbx).wrapping_add(0x68u64)) as u64; fl_l = rbx; fl_r = rbp; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 18; continue; } bb = 19; continue; }
            19 => { rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rsi, r14); crate::simd_ops::wu32(rsi.wrapping_add(0x8u64), ((r12 as u32)) as u32); crate::simd_ops::wu32(rsi.wrapping_add(0xcu64), ((r15 as u32)) as u32); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1412f7170(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142537280; const P1: u64 = 0x142535070; const P2: u64 = 0x1426456c8; const P3: u64 = 0x1426453a0; const P4: u64 = 0x1412f73f0; const P5: u64 = 0x1412f6f00;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; r15 = rdx; r14 = rcx; r12 = (crate::simd_ops::ru32(rcx.wrapping_add(0x8u64))) as u32 as u64; rdx = ((rdx.wrapping_add(rdx.wrapping_mul(4))) as u64) as u64; rdx = ((rdx).wrapping_shl(4u32)) as u64; r10 = 0x6348; rax = ((crate::exe::tls_pointer() as u64)) as u64; r8 = (crate::simd_ops::ru64(rax)) as u64; r10 = (crate::simd_ops::ru64(r10.wrapping_add(r8))) as u64; rbp = 0; fl_l = 0; fl_r = 0; fl_l = ((rdx) & ((rdx).wrapping_sub(1))) as u64; rax = ((fl_l as u64)) as u64; if (((fl_l as u64)) != 0) { bb = 5; continue; } bb = 1; continue; }
            1 => { rax = rdx.wrapping_sub(0x10); fl_l = rax; fl_r = (0x3f0u64) as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 5; continue; } bb = 2; continue; }
            2 => { rax = rdx.wrapping_add(0x7); rax = ((rax).wrapping_shr(3u32)) as u64; r9 = (crate::simd_ops::ru64(r10.wrapping_add(rax.wrapping_mul(8)).wrapping_add(0x8u64))) as u64; r8 = (crate::simd_ops::ru64(r9.wrapping_add(0x10u64))) as u64; fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { r9 = 0; fl_l = 0; fl_r = 0; rcx = r10; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 12; continue; }
            4 => { fl_l = (crate::simd_ops::lk_add_u32(r9.wrapping_add(0x18u64), (1) as u32)) as u32 as u64; rax = (crate::simd_ops::ru64(r8)) as u64; crate::simd_ops::wu64(r9.wrapping_add(0x10u64), rax); bb = 13; continue; }
            5 => { rax = (0x7fffffffffffffffu64) as u64; fl_l = rdx; fl_r = rax; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 7; continue; } bb = 6; continue; }
            6 => { r8 = rbp; bb = 13; continue; }
            7 => { rax = rdx.wrapping_sub(0x10); fl_l = rax; fl_r = (0x3f0u64) as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 11; continue; } bb = 8; continue; }
            8 => { rax = rdx.wrapping_add(0x7); rax = ((rax).wrapping_shr(3u32)) as u64; rcx = (crate::simd_ops::ru64(r10.wrapping_add(rax.wrapping_mul(8)).wrapping_add(0x8u64))) as u64; r8 = (crate::simd_ops::ru64(rcx.wrapping_add(0x10u64))) as u64; rax = r8; rax = ((((rax as u32)) & (0xfu32))) as u32 as u64; fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 9; continue; }
            9 => { fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 11; continue; } bb = 10; continue; }
            10 => { fl_l = (crate::simd_ops::lk_add_u32(rcx.wrapping_add(0x18u64), (1) as u32)) as u32 as u64; rax = (crate::simd_ops::ru64(r8)) as u64; crate::simd_ops::wu64(rcx.wrapping_add(0x10u64), rax); bb = 13; continue; }
            11 => { r8 = 0x10; rcx = r10; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 12; continue; }
            12 => { r8 = rax; bb = 13; continue; }
            13 => { fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 15; continue; } bb = 14; continue; }
            14 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rax; rdx = 0xdeadd00d; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 16; continue; }
            15 => { rbp = r8; bb = 16; continue; }
            16 => { rbx = (crate::simd_ops::ru64(r14)) as u64; rax = (crate::simd_ops::ru32(r14.wrapping_add(0x8u64))) as u32 as u64; rsi = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rsi = ((rsi).wrapping_shl(4u32)) as u64; rsi = ((rsi).wrapping_add(rbx)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbp); rdi = rbp; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rbp); fl_l = rbx; fl_r = rsi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 19; continue; } bb = 17; continue; }
            17 => { bb = 18; continue; }
            18 => { rax = (crate::simd_ops::ru64(rbx)) as u64; crate::simd_ops::wu64(rdi, rax); xmm0 = (crate::simd_ops::ru128(rbx.wrapping_add(0x8u64))) as u128; crate::simd_ops::wu128(rdi.wrapping_add(0x8u64), (xmm0) as u128); rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x18u64))) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x18u64), ((rax as u32)) as u32); rcx = rdi.wrapping_add(0x1c); rdx = rbx.wrapping_add(0x1c); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdi = ((rdi).wrapping_add(0x50u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rdi); rbx = ((rbx).wrapping_add(0x50u64)) as u64; fl_l = rbx; fl_r = rsi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 18; continue; } bb = 19; continue; }
            19 => { rcx = r14; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(r14, rbp); crate::simd_ops::wu32(r14.wrapping_add(0x8u64), ((r12 as u32)) as u32); crate::simd_ops::wu32(r14.wrapping_add(0xcu64), ((r15 as u32)) as u32); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1412f7370(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x20; const P2: u64 = 0x14240ab10;
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
    loop { rcx = ((rbx.wrapping_add(P1)) as u64) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rbx = ((rbx).wrapping_add(0x50u64)) as u64; fl_l = rbx; fl_r = rdi; if !((((fl_l as u64)) != ((fl_r as u64)))) { break; } }
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1412f7450(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14241d6f0; const P1: u64 = 0x140280170; const P2: u64 = 0x1402826f0; const P3: u64 = 0x141835030;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rdi = rdx; rbx = rcx; rdx = (crate::simd_ops::ru64(rdx)) as u64; crate::simd_ops::wu64(rcx, rdx); rsi = 0x1; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 1; continue; }
            1 => { rax = ((rsi as u32)) as u32 as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rdx.wrapping_add(0x24u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; rax = (((rax as u32)).wrapping_add(1)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rsi as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 4; continue; } bb = 2; continue; }
            2 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x28u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); bb = 4; continue; }
            4 => { rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x8u64), rdx); fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 5; continue; }
            5 => { rax = ((rsi as u32)) as u32 as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rdx.wrapping_add(0x24u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; rax = (((rax as u32)).wrapping_add(1)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 8; continue; } bb = 6; continue; }
            6 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x28u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 7; continue; }
            7 => { rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); bb = 8; continue; }
            8 => { rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0x10u64))) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), rdx); fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 9; continue; }
            9 => { rax = ((rsi as u32)) as u32 as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rdx.wrapping_add(0x24u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; rax = (((rax as u32)).wrapping_add(1)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 12; continue; } bb = 10; continue; }
            10 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x28u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 11; continue; }
            11 => { rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); bb = 12; continue; }
            12 => { rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0x18u64))) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x18u64), rdx); fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 16; continue; } bb = 13; continue; }
            13 => { rax = ((rsi as u32)) as u32 as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rdx.wrapping_add(0x24u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; rax = (((rax as u32)).wrapping_add(1)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 16; continue; } bb = 14; continue; }
            14 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x28u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 16; continue; } bb = 15; continue; }
            15 => { rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); bb = 16; continue; }
            16 => { rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x20u64), rdx); fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 20; continue; } bb = 17; continue; }
            17 => { rax = ((rsi as u32)) as u32 as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rdx.wrapping_add(0x24u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; rax = (((rax as u32)).wrapping_add(1)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 20; continue; } bb = 18; continue; }
            18 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x28u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 20; continue; } bb = 19; continue; }
            19 => { rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); bb = 20; continue; }
            20 => { rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0x28u64))) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x28u64), rdx); fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 24; continue; } bb = 21; continue; }
            21 => { rax = ((rsi as u32)) as u32 as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rdx.wrapping_add(0x24u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; rax = (((rax as u32)).wrapping_add(1)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 24; continue; } bb = 22; continue; }
            22 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x28u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 24; continue; } bb = 23; continue; }
            23 => { rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); bb = 24; continue; }
            24 => { rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x30u64), rdx); fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 28; continue; } bb = 25; continue; }
            25 => { rax = ((rsi as u32)) as u32 as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rdx.wrapping_add(0x24u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; rax = (((rax as u32)).wrapping_add(1)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 28; continue; } bb = 26; continue; }
            26 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x28u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 28; continue; } bb = 27; continue; }
            27 => { rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); bb = 28; continue; }
            28 => { rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0x38u64))) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x38u64), rdx); fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 32; continue; } bb = 29; continue; }
            29 => { rax = ((rsi as u32)) as u32 as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rdx.wrapping_add(0x24u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; rax = (((rax as u32)).wrapping_add(1)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 32; continue; } bb = 30; continue; }
            30 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x28u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 32; continue; } bb = 31; continue; }
            31 => { rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); bb = 32; continue; }
            32 => { rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0x40u64))) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x40u64), rdx); fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 36; continue; } bb = 33; continue; }
            33 => { rax = ((rsi as u32)) as u32 as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rdx.wrapping_add(0x24u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; rax = (((rax as u32)).wrapping_add(1)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 36; continue; } bb = 34; continue; }
            34 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x28u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 36; continue; } bb = 35; continue; }
            35 => { rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); bb = 36; continue; }
            36 => { rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0x48u64))) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x48u64), rdx); fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 40; continue; } bb = 37; continue; }
            37 => { let xa_src: u32 = (rsi as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rdx.wrapping_add(0x24u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rsi = (xa_old) as u32 as u64; fl_l = ((rsi as u32)) as u32 as u64; fl_r = ((rsi as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 40; continue; } bb = 38; continue; }
            38 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x28u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 40; continue; } bb = 39; continue; }
            39 => { rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); bb = 40; continue; }
            40 => { rdx = rdi.wrapping_add(0x50); rcx = rbx.wrapping_add(0x50); rax = P1; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax); rax = P2; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); r9 = 0x19; r8 = ((r9.wrapping_sub(0x11u64)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rax = rbx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1412f7650(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143bbcff0; const P1: u64 = 0x14241d6f0; const P2: u64 = 0x143bbcff8; const P3: u64 = 0x143bbd000; const P4: u64 = 0x143bbd008; const P5: u64 = 0x143bbd010; const P6: u64 = 0x143bbd018; const P7: u64 = 0x143bbd020; const P8: u64 = 0x143bbd028; const P9: u64 = 0x143bbd030; const P10: u64 = 0x143bbd038;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rdx; rdx = (crate::simd_ops::ru64(P0)) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 1; continue; }
            1 => { rax = 0xffffffff; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rdx.wrapping_add(0x24u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 4; continue; } bb = 2; continue; }
            2 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x28u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 4; continue; }
            4 => { rsi = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(P0, rsi); rax = (crate::simd_ops::ru64(rbx)) as u64; crate::simd_ops::wu64(rbx, rsi); rdx = (crate::simd_ops::ru64(P2)) as u64; crate::simd_ops::wu64(P0, rax); fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 5; continue; }
            5 => { rax = 0xffffffff; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rdx.wrapping_add(0x24u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 8; continue; } bb = 6; continue; }
            6 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x28u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 7; continue; }
            7 => { rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 8; continue; }
            8 => { crate::simd_ops::wu64(P2, rsi); rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x8u64), rsi); rdx = (crate::simd_ops::ru64(P3)) as u64; crate::simd_ops::wu64(P2, rax); fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 9; continue; }
            9 => { rax = 0xffffffff; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rdx.wrapping_add(0x24u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 12; continue; } bb = 10; continue; }
            10 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x28u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 11; continue; }
            11 => { rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 12; continue; }
            12 => { crate::simd_ops::wu64(P3, rsi); rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), rsi); rdx = (crate::simd_ops::ru64(P4)) as u64; crate::simd_ops::wu64(P3, rax); fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 16; continue; } bb = 13; continue; }
            13 => { rax = 0xffffffff; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rdx.wrapping_add(0x24u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 16; continue; } bb = 14; continue; }
            14 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x28u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 16; continue; } bb = 15; continue; }
            15 => { rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 16; continue; }
            16 => { crate::simd_ops::wu64(P4, rsi); rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x18u64))) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x18u64), rsi); rdx = (crate::simd_ops::ru64(P5)) as u64; crate::simd_ops::wu64(P4, rax); fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 20; continue; } bb = 17; continue; }
            17 => { rax = 0xffffffff; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rdx.wrapping_add(0x24u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 20; continue; } bb = 18; continue; }
            18 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x28u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 20; continue; } bb = 19; continue; }
            19 => { rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 20; continue; }
            20 => { crate::simd_ops::wu64(P5, rsi); rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x20u64), rsi); rdx = (crate::simd_ops::ru64(P6)) as u64; crate::simd_ops::wu64(P5, rax); fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 24; continue; } bb = 21; continue; }
            21 => { rax = 0xffffffff; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rdx.wrapping_add(0x24u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 24; continue; } bb = 22; continue; }
            22 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x28u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 24; continue; } bb = 23; continue; }
            23 => { rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 24; continue; }
            24 => { crate::simd_ops::wu64(P6, rsi); rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x28u64))) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x28u64), rsi); rdx = (crate::simd_ops::ru64(P7)) as u64; crate::simd_ops::wu64(P6, rax); fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 28; continue; } bb = 25; continue; }
            25 => { rax = 0xffffffff; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rdx.wrapping_add(0x24u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 28; continue; } bb = 26; continue; }
            26 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x28u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 28; continue; } bb = 27; continue; }
            27 => { rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 28; continue; }
            28 => { crate::simd_ops::wu64(P7, rsi); rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x30u64), rsi); rdx = (crate::simd_ops::ru64(P8)) as u64; crate::simd_ops::wu64(P7, rax); fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 32; continue; } bb = 29; continue; }
            29 => { rax = 0xffffffff; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rdx.wrapping_add(0x24u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 32; continue; } bb = 30; continue; }
            30 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x28u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 32; continue; } bb = 31; continue; }
            31 => { rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 32; continue; }
            32 => { crate::simd_ops::wu64(P8, rsi); rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x38u64))) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x38u64), rsi); rdx = (crate::simd_ops::ru64(P9)) as u64; crate::simd_ops::wu64(P8, rax); fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 36; continue; } bb = 33; continue; }
            33 => { rax = 0xffffffff; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rdx.wrapping_add(0x24u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 36; continue; } bb = 34; continue; }
            34 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x28u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 36; continue; } bb = 35; continue; }
            35 => { rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 36; continue; }
            36 => { crate::simd_ops::wu64(P9, rsi); rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x40u64))) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x40u64), rsi); rdx = (crate::simd_ops::ru64(P10)) as u64; crate::simd_ops::wu64(P9, rax); fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 40; continue; } bb = 37; continue; }
            37 => { rax = 0xffffffff; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rdx.wrapping_add(0x24u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 40; continue; } bb = 38; continue; }
            38 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x28u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 40; continue; } bb = 39; continue; }
            39 => { rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 40; continue; }
            40 => { crate::simd_ops::wu64(P10, rsi); rbp = P0; rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x48u64))) as u64; rdi = rsi; crate::simd_ops::wu64(rbx.wrapping_add(0x48u64), rsi); crate::simd_ops::wu64(P10, rax); bb = 41; continue; }
            41 => { rdx = (crate::simd_ops::ru64(rdi.wrapping_add(rbp).wrapping_add(0x50u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 45; continue; } bb = 42; continue; }
            42 => { rax = 0xffffffff; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rdx.wrapping_add(0x24u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 45; continue; } bb = 43; continue; }
            43 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x28u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 45; continue; } bb = 44; continue; }
            44 => { rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 45; continue; }
            45 => { crate::simd_ops::wu64(rdi.wrapping_add(rbp).wrapping_add(0x50u64), rsi); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(rdi).wrapping_add(0x50u64))) as u64; crate::simd_ops::wu64(rbx.wrapping_add(rdi).wrapping_add(0x50u64), rsi); crate::simd_ops::wu64(rdi.wrapping_add(rbp).wrapping_add(0x50u64), rcx); rdi = ((rdi).wrapping_add(0x8u64)) as u64; fl_l = rdi; fl_r = (0xc8u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 41; continue; } bb = 46; continue; }
            46 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rax = rbp; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1412f7950(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142407c80; const P1: u64 = 0x143c4aa00; const P2: u64 = 0x1422589f0; const P3: u64 = 0x1426455a8; const P4: u64 = 0x142645598; const P5: u64 = 0x142258630; const P6: u64 = 0x1412f7450; const P7: u64 = 0x142645580; const P8: u64 = 0x1410d7660;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbp = rdx; r15 = rcx; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rax = ((rax as u32)) as u32 as u64; r8 = P1; r8 = (crate::simd_ops::ru64(r8.wrapping_add(rax.wrapping_mul(8)))) as u64; fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru32(r8)) as u32 as u64; fl_r = 0x2; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 3; continue; } bb = 2; continue; }
            2 => { rcx = rbp; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 18; continue; }
            3 => { rdi = (crate::simd_ops::ru64(r15.wrapping_add(0x168u64))) as u64; rcx = rdi.wrapping_add(0x18); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0x20u64))) as u64; rsi = (crate::simd_ops::ru64(rdi.wrapping_add(0x28u64))) as u64; r8 = rsi.wrapping_add(0x130); fl_l = r8; fl_r = (crate::simd_ops::ru64(rdi.wrapping_add(0x10u64))) as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 12; continue; } bb = 4; continue; }
            4 => { fl_l = rsi; fl_r = rdx; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 9; continue; } bb = 5; continue; }
            5 => { bb = 6; continue; }
            6 => { fl_l = r8; fl_r = rdx; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 9; continue; } bb = 7; continue; }
            7 => { rcx = (crate::simd_ops::ru32(rdi.wrapping_add(0x30u64))) as u32 as u64; rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x34u64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 9; continue; } bb = 8; continue; }
            8 => { rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0x20u64))) as u64; fl_l = rsi; fl_r = rdx; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 6; continue; } bb = 9; continue; }
            9 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x10u64))) as u64; rax = ((rax).wrapping_sub(0x8u64)) as u64; fl_l = rsi; fl_r = rax; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 11; continue; } bb = 10; continue; }
            10 => { rax = (0xffffffffffu64) as u64; crate::simd_ops::wu64(rsi, rax); bb = 11; continue; }
            11 => { rsi = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; r8 = rsi.wrapping_add(0x130); bb = 12; continue; }
            12 => { fl_l = rsi; fl_r = rdx; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 16; continue; } bb = 13; continue; }
            13 => { fl_l = r8; fl_r = rdx; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 16; continue; } bb = 14; continue; }
            14 => { rcx = (crate::simd_ops::ru32(rdi.wrapping_add(0x30u64))) as u32 as u64; rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x34u64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 16; continue; } bb = 15; continue; }
            15 => { rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0x20u64))) as u64; fl_l = rsi; fl_r = rdx; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 13; continue; } bb = 16; continue; }
            16 => { rax = (0x69fafafafaffu64) as u64; crate::simd_ops::wu64(rsi, rax); let tmp_xchg: u64 = crate::simd_ops::atomic_xchg_u64(rdi.wrapping_add(0x28u64), r8); r8 = (tmp_xchg) as u64; rcx = rdi.wrapping_add(0x18); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu32(rsi.wrapping_add(0x8u64), (0x0u32) as u32); rax = P5; crate::simd_ops::wu64(rsi.wrapping_add(0xcu64), rax); rcx = rsi.wrapping_add(0x14); rdx = rbp; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(r15.wrapping_add(0x168u64))) as u64; crate::simd_ops::wu64(rsi, (0x124u64) as u64); crate::simd_ops::lk_add_u32(rax.wrapping_add(0x30u64), (1) as u32); fl_l = (crate::simd_ops::ru8(r15.wrapping_add(0x198u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 18; continue; } bb = 17; continue; }
            17 => { r8 = 0; fl_l = 0; fl_r = 0; rdx = ((r8.wrapping_add(0x1u64)) as u32) as u32 as u64; rcx = (crate::simd_ops::ru64(r15.wrapping_add(0x1a0u64))) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P7), rax), rax, rcx, rdx, r8, r9, rsp); bb = 18; continue; }
            18 => { rcx = rbp; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(P8, rax, rcx, rdx, r8, r9, rsp); }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1412f7c50(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0x18; const P3: u64 = 0x20; const P4: u64 = 0x28; const P5: u64 = 0x30; const P6: u64 = 0x38; const P7: u64 = 0x40; const P8: u64 = 0x48; const P9: u64 = 0x50; const P10: u64 = 0x58; const P11: u64 = 0x60;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rcx, rax); crate::simd_ops::wu64(rcx.wrapping_add(P0), rax); crate::simd_ops::wu64(rcx.wrapping_add(P1), rax); crate::simd_ops::wu64(rcx.wrapping_add(P2), rax); crate::simd_ops::wu64(rcx.wrapping_add(P3), rax); crate::simd_ops::wu64(rcx.wrapping_add(P4), rax); crate::simd_ops::wu64(rcx.wrapping_add(P5), rax); crate::simd_ops::wu64(rcx.wrapping_add(P6), rax); crate::simd_ops::wu64(rcx.wrapping_add(P7), rax); crate::simd_ops::wu64(rcx.wrapping_add(P8), rax); crate::simd_ops::wu64(rcx.wrapping_add(P9), rax); crate::simd_ops::wu64(rcx.wrapping_add(P10), rax); crate::simd_ops::wu64(rcx.wrapping_add(P11), rax); crate::simd_ops::wu64(rcx.wrapping_add(0x68u64), rax); crate::simd_ops::wu64(rcx.wrapping_add(0x70u64), rax); crate::simd_ops::wu64(rcx.wrapping_add(0x78u64), rax); crate::simd_ops::wu64(rcx.wrapping_add(0x80u64), rax); crate::simd_ops::wu64(rcx.wrapping_add(0x88u64), rax); crate::simd_ops::wu64(rcx.wrapping_add(0x90u64), rax); crate::simd_ops::wu64(rcx.wrapping_add(0x98u64), rax); crate::simd_ops::wu64(rcx.wrapping_add(0xa0u64), rax); crate::simd_ops::wu64(rcx.wrapping_add(0xa8u64), rax); crate::simd_ops::wu64(rcx.wrapping_add(0xb0u64), rax); crate::simd_ops::wu64(rcx.wrapping_add(0xb8u64), rax); crate::simd_ops::wu64(rcx.wrapping_add(0xc0u64), rax); crate::simd_ops::wu64(rcx.wrapping_add(0xc8u64), rax); crate::simd_ops::wu64(rcx.wrapping_add(0xd0u64), rax); crate::simd_ops::wu64(rcx.wrapping_add(0xd8u64), rax); crate::simd_ops::wu64(rcx.wrapping_add(0xe0u64), rax); crate::simd_ops::wu64(rcx.wrapping_add(0xe8u64), rax); crate::simd_ops::wu64(rcx.wrapping_add(0xf0u64), rax); crate::simd_ops::wu64(rcx.wrapping_add(0xf8u64), rax); crate::simd_ops::wu64(rcx.wrapping_add(0x100u64), rax); crate::simd_ops::wu64(rcx.wrapping_add(0x108u64), rax); crate::simd_ops::wu64(rcx.wrapping_add(0x110u64), rax); crate::simd_ops::wu64(rcx.wrapping_add(0x118u64), rax); crate::simd_ops::wu64(rcx.wrapping_add(0x120u64), rax); crate::simd_ops::wu64(rcx.wrapping_add(0x128u64), rax); crate::simd_ops::wu64(rcx.wrapping_add(0x130u64), rax); crate::simd_ops::wu64(rcx.wrapping_add(0x138u64), rax); crate::simd_ops::wu64(rcx.wrapping_add(0x140u64), rax); crate::simd_ops::wu64(rcx.wrapping_add(0x148u64), rax); rax = rcx; return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1412f7d50(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1401de830; const P1: u64 = 0xd8; const P2: u64 = 0xc0; const P3: u64 = 0xb0; const P4: u64 = 0x1433f3308; const P5: u64 = 0x18; const P6: u64 = 0x1412eb320;
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rdi = rcx;
    rcx = ((rcx).wrapping_add(0xe8u64)) as u64;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rcx = ((rdi.wrapping_add(P1)) as u64) as u64;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rcx = ((rdi.wrapping_add(P2)) as u64) as u64;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rcx = ((rdi.wrapping_add(P3)) as u64) as u64;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rax = P4;
    crate::simd_ops::wu64(rdi.wrapping_add(P5), rax);
    rcx = ((rdi.wrapping_add(P5)) as u64) as u64;
    rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp);
    rcx = rdi;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1412f7dc0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1401de830;
    let (mut rax, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rcx; rcx = ((rcx).wrapping_add(0x140u64)) as u64; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rcx = rbx.wrapping_add(0x130); rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rcx = rbx.wrapping_add(0x120); rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rcx = rbx.wrapping_add(0x110); rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rcx = rbx.wrapping_add(0x100); rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rcx = rbx.wrapping_add(0xf0); rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rcx = rbx.wrapping_add(0xe0); rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rcx = rbx.wrapping_add(0xd0); rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rcx = rbx.wrapping_add(0xc0); rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rcx = rbx.wrapping_add(0xa8); rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rcx = rbx.wrapping_add(0x98); rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rcx = rbx.wrapping_add(0x88); rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rcx = rbx.wrapping_add(0x78); rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rcx = rbx.wrapping_add(0x68); rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rcx = rbx.wrapping_add(0x58); rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rcx = rbx.wrapping_add(0x48); rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rcx = rbx.wrapping_add(0x38); rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rcx = rbx.wrapping_add(0x28); rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rcx = rbx.wrapping_add(0x18); rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rcx = rbx; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1412f7ec0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x318; const P1: u64 = 0x14209d670; const P2: u64 = 0x58; const P3: u64 = 0x1411e7fc0; const P4: u64 = 0x142442de0; const P5: u64 = 0x308; const P6: u64 = 0x1401de830; const P7: u64 = 0x2f8; const P8: u64 = 0x2e8; const P9: u64 = 0x2d8; const P10: u64 = 0x2c8; const P11: u64 = 0x220; const P12: u64 = 0x1433f3340; const P13: u64 = 0x1412eb320;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = rcx; rbx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rcx = rbx; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rcx = ((rbx.wrapping_add(P2)) as u64) as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rcx = rbx; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); bb = 2; continue; }
            2 => { rcx = ((rdi.wrapping_add(P5)) as u64) as u64; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); rcx = ((rdi.wrapping_add(P7)) as u64) as u64; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); rcx = ((rdi.wrapping_add(P8)) as u64) as u64; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); rcx = ((rdi.wrapping_add(P9)) as u64) as u64; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); rcx = ((rdi.wrapping_add(P10)) as u64) as u64; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); rcx = ((rdi.wrapping_add(P11)) as u64) as u64; rsi = P12; crate::simd_ops::wu64(rcx, rsi); rax = crate::simd_ops::call_stk(P13, rax, rcx, rdx, r8, r9, rsp); rcx = rdi.wrapping_add(0x180); crate::simd_ops::wu64(rcx, rsi); rax = crate::simd_ops::call_stk(P13, rax, rcx, rdx, r8, r9, rsp); rcx = rdi.wrapping_add(0x170); rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rdi.wrapping_add(0xd8u64), rsi); rcx = rdi.wrapping_add(0xd8); rax = crate::simd_ops::call_stk(P13, rax, rcx, rdx, r8, r9, rsp); rcx = rdi.wrapping_add(0xc8); rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rdi.wrapping_add(0x30u64), rsi); rcx = rdi.wrapping_add(0x30); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(P13, rax, rcx, rdx, r8, r9, rsp); }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1412f8080(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1433f3340; const P1: u64 = 0x1412eb320; const P2: u64 = 0x141834d30;
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
    if !(((((fl_l as u8)) & ((fl_r as u8))) == 0)) { rdx = 0xa0; rcx = rdi; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); }
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rax = rdi;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1412f8110(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x60; const P1: u64 = 0x90; const P2: u64 = 0x68; const P3: u64 = 0xffffffffffffffff; const P4: u64 = 0x8; const P5: u64 = 0x10; const P6: u64 = 0x143868280; const P7: u64 = 0x142261020; const P8: u64 = 0x20; const P9: u64 = 0x29; const P10: u64 = 0x1422605c0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; r8 = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rbx = rcx; rax = (crate::simd_ops::ru32(rcx.wrapping_add(P1))) as u32 as u64; r9 = (crate::simd_ops::ru64(rcx.wrapping_add(P2))) as u64; rax = (((rax) as i64).wrapping_mul((r8) as i64) as u64) as u64; fl_l = rax; fl_r = r9; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rcx = ((r9.wrapping_add(rax)) as u64) as u64; rdx = 0; fl_l = 0; fl_r = 0; rax = ((rcx.wrapping_add(P3)) as u64) as u64; let d = (r9) as u128; let n = ((rdx as u128) << 64) | (rax as u128); if d != 0 { rax = (n / d) as u64; rdx = (n % d) as u64; } r9 = ((rcx.wrapping_add(P3)) as u64) as u64; fl_l = r9; fl_r = rdx; r9 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(P4))) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(P5))) as u64; fl_l = rcx; fl_r = r9; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 5; continue; } bb = 3; continue; }
            3 => { fl_l = (crate::simd_ops::ru8(P6)) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 6; continue; } bb = 4; continue; }
            4 => { fl_l = rcx; fl_r = r9; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rdx = r9; rcx = ((rbx.wrapping_add(P4)) as u64) as u64; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); bb = 6; continue; }
            6 => { r8 = (crate::simd_ops::ru32(rbx.wrapping_add(P8))) as u32 as u64; fl_l = ((r8 as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 9; continue; } bb = 7; continue; }
            7 => { fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(P9))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 9; continue; } bb = 8; continue; }
            8 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P5))) as u64; r9 = 0x30; rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P4))) as u64; crate::simd_ops::wu8(rsp.wrapping_add(0x20u64), (0x0u8) as u8); rax = crate::simd_ops::call_stk(P10, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu32(rbx.wrapping_add(P8), (0x0u32) as u32); bb = 9; continue; }
            9 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1412f81b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x90; const P1: u64 = 0x142229a00; const P2: u64 = 0x68; const P3: u64 = 0xffffffffffffffff; const P4: u64 = 0x60; const P5: u64 = 0x98; const P6: u64 = 0x1412f8d90; const P7: u64 = 0x4;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = ((rcx.wrapping_add(P0)) as u64) as u64; rsi = rcx; rcx = rbx; rdi = ((rdx as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0xffffffff; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 6; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), r14); r14 = (crate::simd_ops::ru32(rbx)) as u32 as u64; r9 = ((r14.wrapping_add(rdi)) as u32) as u32 as u64; rcx = ((r9.wrapping_add(r9.wrapping_mul(2))) as u64) as u64; crate::simd_ops::wu32(rbx, ((r9 as u32)) as u32); r8 = (crate::simd_ops::ru64(rsi.wrapping_add(P2))) as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; fl_l = rcx; fl_r = r8; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 3; continue; } bb = 2; continue; }
            2 => { rax = ((r8.wrapping_add(P3)) as u64) as u64; rdx = 0; fl_l = 0; fl_r = 0; rax = ((rax).wrapping_add(rcx)) as u64; let d = (r8) as u128; let n = ((rdx as u128) << 64) | (rax as u128); if d != 0 { rax = (n / d) as u64; rdx = (n % d) as u64; } r8 = ((r8).wrapping_sub(1)) as u64; r8 = ((r8).wrapping_add(rcx)) as u64; fl_l = r8; fl_r = rdx; r8 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; bb = 3; continue; }
            3 => { rdx = 0; fl_l = 0; fl_r = 0; rax = r8; let d = (crate::simd_ops::ru64(rsi.wrapping_add(P4))) as u128; let n = ((rdx as u128) << 64) | (rax as u128); if d != 0 { rax = (n / d) as u64; rdx = (n % d) as u64; } rdi = rax; fl_l = ((r9 as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 5; continue; } bb = 4; continue; }
            4 => { rcx = ((rsi.wrapping_add(P5)) as u64) as u64; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru32(rbx)) as u32 as u64; crate::simd_ops::wu32(rax, ((rcx as u32)) as u32); rcx = ((rdi as u32)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rbx)) as u32 as u64; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; crate::simd_ops::wu32(rax.wrapping_add(P7), ((rcx as u32)) as u32); bb = 5; continue; }
            5 => { rax = ((r14 as u32)) as u32 as u64; crate::simd_ops::wu32(rbx, ((rdi as u32)) as u32); r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; bb = 6; continue; }
            6 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1412f8250(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14222a2a0; const P1: u64 = 0x30; const P2: u64 = 0x1433f3340; const P3: u64 = 0x38; const P4: u64 = 0x40; const P5: u64 = 0x44; const P6: u64 = 0x48; const P7: u64 = 0x50; const P8: u64 = 0x60; const P9: u64 = 0x70; const P10: u64 = 0x80; const P11: u64 = 0x78;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rcx;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    crate::simd_ops::wu8(rbx.wrapping_add(P1), (0x0u8) as u8);
    rax = P2;
    crate::simd_ops::wu64(rbx, rax);
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu64(rbx.wrapping_add(P3), rax);
    crate::simd_ops::wu32(rbx.wrapping_add(P4), ((rax as u32)) as u32);
    crate::simd_ops::wu8(rbx.wrapping_add(P5), ((rax as u8)) as u8);
    crate::simd_ops::wu64(rbx.wrapping_add(P6), rax);
    crate::simd_ops::wu32(rbx.wrapping_add(P7), ((rax as u32)) as u32);
    crate::simd_ops::wu64(rbx.wrapping_add(P8), (0x30u64) as u64);
    crate::simd_ops::wu64(rbx.wrapping_add(P9), rax);
    crate::simd_ops::wu64(rbx.wrapping_add(P10), rax);
    crate::simd_ops::wu64(rbx.wrapping_add(P11), (0x38u64) as u64);
    crate::simd_ops::wu64(rbx.wrapping_add(0x88u64), rax);
    rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x60u64))) as u64;
    rax = ((rax).wrapping_shl(14u32)) as u64;
    crate::simd_ops::wu64(rbx.wrapping_add(0x68u64), rax);
    crate::simd_ops::wu64(rbx.wrapping_add(0x58u64), (0x4000u64) as u64);
    rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x78u64))) as u64;
    rax = ((rax).wrapping_shl(11u32)) as u64;
    crate::simd_ops::wu64(rbx.wrapping_add(0x80u64), rax);
    rax = rbx;
    crate::simd_ops::wu64(rbx.wrapping_add(0x70u64), (0x800u64) as u64);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1412f82e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1424069a0; const P1: u64 = 0x143868298; const P2: u64 = 0x143426a48; const P3: u64 = 0x14209d670; const P4: u64 = 0x142075af0; const P5: u64 = 0x14209c560;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut xmm6: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 17>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = rsp; rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x70u64)) as u64; crate::simd_ops::wu64(rax.wrapping_add(0x10u64), rsi); rdi = rcx; rsi = (crate::simd_ops::ru64(rcx.wrapping_add(0x80u64))) as u64; crate::simd_ops::wu64(rax.wrapping_add(0x18u64), r14); crate::simd_ops::wu128(rax.wrapping_sub(0x28u64), (xmm6) as u128); rax = ((rdx as u32)) as u32 as u64; rax = (((rax) as i64).wrapping_mul((crate::simd_ops::ru64(rcx.wrapping_add(0x78u64))) as i64) as u64) as u64; rbp = ((rdx as u32)) as u32 as u64; r15 = ((rdx as u32)) as u32 as u64; fl_l = rax; fl_r = rsi; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rcx = ((rsi.wrapping_add(rax)) as u64) as u64; rdx = 0; fl_l = 0; fl_r = 0; rax = rcx.wrapping_sub(0x1); let d = (rsi) as u128; let n = ((rdx as u128) << 64) | (rax as u128); if d != 0 { rax = (n / d) as u64; rdx = (n % d) as u64; } rsi = rcx.wrapping_sub(0x1); fl_l = rsi; fl_r = rdx; rsi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; bb = 2; continue; }
            2 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r14 = (crate::simd_ops::ru64(rdi.wrapping_add(0x10u64))) as u64; xmm6 = (xmm0) as u128; rax = (crate::simd_ops::ru64(r14.wrapping_add(0x10u64))) as u64; fl_l = rax; fl_r = rsi; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 6; continue; } bb = 3; continue; }
            3 => { fl_l = (crate::simd_ops::ru8(P1)) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 9; continue; } bb = 4; continue; }
            4 => { fl_l = rax; fl_r = rsi; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 9; continue; } bb = 5; continue; }
            5 => { xmm1 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm0 as u64) - f64::from_bits(crate::simd_ops::ru64(rdi.wrapping_add(0x88u64)))).to_bits() as u128); fl_l = ((xmm1 as u64)) as u64; fl_r = (crate::simd_ops::ru64(P2)) as u64; if !(f64::from_bits(fl_l) > f64::from_bits(fl_r)) { bb = 9; continue; } bb = 6; continue; }
            6 => { rcx = r14; crate::simd_ops::wu64(rsp.wrapping_add(0x90u64), rbx); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rsp.wrapping_add(0x40u64), (0x1u32) as u32); r9 = ((rbx as u32)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x44u64), (0x10u32) as u32); rax = rsp.wrapping_add(0x40); bb = 7; continue; }
            7 => { r9 = ((((r9 as u32)) | (crate::simd_ops::ru32(rax)))) as u32 as u64; rcx = rsp.wrapping_add(0x48); rax = ((rax).wrapping_add(0x4u64)) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 7; continue; } bb = 8; continue; }
            8 => { crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), r14); r8 = 0; fl_l = 0; fl_r = 0; rdx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rbx as u32)) as u32); rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = rsi; fl_r = (0x400000u64) as u64; rbx = (rbx & !0xffu64) | (((if (((fl_l as u64)) > ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); fl_l = (((rbx as u32)).wrapping_add(0x2u32)) as u32 as u64; fl_r = fl_l; rbx = ((fl_l as u32)) as u32 as u64; crate::simd_ops::wu32(r14.wrapping_add(0x50u64), ((rbx as u32)) as u32); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x90u64))) as u64; crate::simd_ops::wu64(rdi.wrapping_add(0x88u64), ((xmm6 as u64)) as u64); bb = 9; continue; }
            9 => { xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x60u64))) as u128; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0xa0u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x98u64))) as u64; fl_l = ((rbp as u32)) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 11; continue; } bb = 10; continue; }
            10 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x10u64))) as u64; r9 = 0x2; r8 = (((r15) as i64).wrapping_mul((0x38u64) as i64) as u64) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rdi.wrapping_add(0x38u64), rax); crate::simd_ops::wu8(rdi.wrapping_add(0x30u64), (0x1u8) as u8); crate::simd_ops::wu32(rdi.wrapping_add(0x20u64), ((rbp as u32)) as u32); bb = 11; continue; }
            11 => { rsp = ((rsp).wrapping_add(0x70u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1412f8420(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x20; const P1: u64 = 0x29; const P2: u64 = 0x10; const P3: u64 = 0x8; const P4: u64 = 0x1422605c0;
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
    if !((((fl_l as u8)) != ((fl_r as u8)))) { rdx = (crate::simd_ops::ru64(rcx.wrapping_add(P2))) as u64; r9 = 0x30; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P3))) as u64; crate::simd_ops::wu8(rsp.wrapping_add(0x20u64), (0x0u8) as u8); rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu32(rbx.wrapping_add(P0), (0x0u32) as u32); }
    rsp = ((rsp).wrapping_add(0x30u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1412f8460(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x18; const P1: u64 = 0x3ff; const P2: u64 = 0x10; const P3: u64 = 0x14209d670; const P4: u64 = 0x1; const P5: u64 = 0xc; const P6: u64 = 0x142075af0; const P7: u64 = 0x50; const P8: u64 = 0x14209c560; const P9: u64 = 0x48; const P10: u64 = 0x44; const P11: u64 = 0x24;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 4; continue; } bb = 1; continue; }
            1 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x48u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), rbp); rdi = rcx; rsi = ((rdx as u32)) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), r14); crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), r15); r15 = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rax = ((rsi.wrapping_add(P1)) as u64) as u64; rax = (((rax) & (0xfffffffffffffc00u64))) as u64; rbp = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rbp = ((rbp).wrapping_shl(2u32)) as u64; fl_l = (crate::simd_ops::ru64(r15.wrapping_add(P2))) as u64; fl_r = rbp; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 3; continue; } bb = 2; continue; }
            2 => { rcx = r15; crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), rbx); rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rbx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), r15); r8 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rbx as u32)) as u32); rcx = rbp; r9 = ((rbx.wrapping_add(P4)) as u32) as u32 as u64; rdx = ((rbx.wrapping_add(P5)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); fl_l = rbp; fl_r = (0x400000u64) as u64; rbx = (rbx & !0xffu64) | (((if (((fl_l as u64)) > ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); fl_l = (((rbx as u32)).wrapping_add(0x2u32)) as u32 as u64; fl_r = fl_l; rbx = ((fl_l as u32)) as u32 as u64; crate::simd_ops::wu32(r15.wrapping_add(P7), ((rbx as u32)) as u32); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(P0))) as u64; r8 = ((rsi.wrapping_add(rsi.wrapping_mul(2))) as u64) as u64; r8 = ((r8).wrapping_shl(2u32)) as u64; r9 = 0x2; rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; crate::simd_ops::wu64(rdi.wrapping_add(P9), rax); crate::simd_ops::wu8(rdi.wrapping_add(P10), (0x1u8) as u8); crate::simd_ops::wu32(rdi.wrapping_add(P11), ((rsi as u32)) as u32); rsp = ((rsp).wrapping_add(0x48u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); bb = 4; continue; }
            4 => { return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1412f8520(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x29; const P1: u64 = 0x20; const P2: u64 = 0x10; const P3: u64 = 0x8; const P4: u64 = 0x1422605c0;
    let (mut rax, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x30u64)) as u64;
    rax = ((crate::simd_ops::ru8(rcx.wrapping_add(P0))) as u32) as u32 as u64;
    rbx = rcx;
    r8 = (crate::simd_ops::ru32(rcx.wrapping_add(P1))) as u32 as u64;
    r9 = 0x30;
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
pub unsafe extern "win64" fn fn_1412f8560(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x98; const P1: u64 = 0x60; const P2: u64 = 0x68; const P3: u64 = 0xffffffffffffffff; const P4: u64 = 0x8; const P5: u64 = 0x10; const P6: u64 = 0x143868280; const P7: u64 = 0x142261020; const P8: u64 = 0x20; const P9: u64 = 0x29; const P10: u64 = 0x1422605c0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rbx = rcx; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 1; continue; }
            1 => { r8 = (crate::simd_ops::ru64(rbx.wrapping_add(P1))) as u64; r9 = (crate::simd_ops::ru64(rbx.wrapping_add(P2))) as u64; rax = r8; rax = (((rax) as i64).wrapping_mul((rcx) as i64) as u64) as u64; fl_l = rax; fl_r = r9; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 3; continue; } bb = 2; continue; }
            2 => { rcx = ((r9.wrapping_add(rax)) as u64) as u64; rdx = 0; fl_l = 0; fl_r = 0; rax = ((rcx.wrapping_add(P3)) as u64) as u64; let d = (r9) as u128; let n = ((rdx as u128) << 64) | (rax as u128); if d != 0 { rax = (n / d) as u64; rdx = (n % d) as u64; } r9 = ((rcx.wrapping_add(P3)) as u64) as u64; fl_l = r9; fl_r = rdx; r9 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(P4))) as u64; rdx = (crate::simd_ops::ru64(rax.wrapping_add(P5))) as u64; fl_l = rdx; fl_r = r9; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 6; continue; } bb = 4; continue; }
            4 => { fl_l = (crate::simd_ops::ru8(P6)) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 7; continue; } bb = 5; continue; }
            5 => { fl_l = rdx; fl_r = r9; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 7; continue; } bb = 6; continue; }
            6 => { rdx = r9; rcx = ((rbx.wrapping_add(P4)) as u64) as u64; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); bb = 7; continue; }
            7 => { r8 = (crate::simd_ops::ru32(rbx.wrapping_add(P8))) as u32 as u64; fl_l = ((r8 as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 10; continue; } bb = 8; continue; }
            8 => { fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(P9))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 10; continue; } bb = 9; continue; }
            9 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P5))) as u64; r9 = 0x30; rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P4))) as u64; crate::simd_ops::wu8(rsp.wrapping_add(0x20u64), (0x0u8) as u8); rax = crate::simd_ops::call_stk(P10, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu32(rbx.wrapping_add(P8), (0x0u32) as u32); bb = 10; continue; }
            10 => { rsp = ((rsp).wrapping_add(0x30u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1412f8600(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1412ec2c0;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rdx; rdi = rcx; rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x8u64))) as u32 as u64; r8 = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; r8 = ((r8.wrapping_mul(4)) as u64) as u64; rax = (0x2aaaaaaaaaaaaaabu64) as u64; let prod = (rax as i64 as i128).wrapping_mul((r8) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; rsi = rdx; rsi = ((((rsi) as i64).wrapping_shr(1u32) as u64)) as u64; rax = rsi; rax = ((rax).wrapping_shr(63u32)) as u64; rsi = ((rsi).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(0xcu64))) as u32 as u64; fl_l = rbx; fl_r = rax; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 5; continue; } bb = 1; continue; }
            1 => { rdx = ((rax as u32)) as u32 as u64; rdx = ((rdx).wrapping_shr(1u32)) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; fl_l = rdx; fl_r = rbx; rdx = (if (((fl_l as u64)) < ((fl_r as u64))) { rbx } else { rdx }) as u64; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); r10 = rbx; fl_l = r10; fl_r = rsi; r10 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 9; continue; } bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x8u64))) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = (crate::simd_ops::ru64(rdi)) as u64; rdx = ((rax.wrapping_add(rcx.wrapping_mul(4))) as u64) as u64; r8 = 0; fl_l = 0; fl_r = 0; bb = 3; continue; }
            3 => { crate::simd_ops::wu32(rdx, (0xff01ffffu32) as u32); crate::simd_ops::wu32(rdx.wrapping_add(0x4u64), ((r8 as u32)) as u32); crate::simd_ops::wu32(rdx.wrapping_add(0x8u64), (0xffffffffu32) as u32); rdx = rdx.wrapping_add(0xc); fl_l = r10; fl_r = (0x1u64) as u64; r10 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } bb = 4; continue; }
            4 => { crate::simd_ops::wu32(rdi.wrapping_add(0x8u64), ((rbx as u32)) as u32); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            5 => { fl_l = rbx; fl_r = rsi; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 10; continue; } bb = 6; continue; }
            6 => { rcx = rbx; fl_l = rcx; fl_r = rsi; rcx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 9; continue; } bb = 7; continue; }
            7 => { rax = (crate::simd_ops::ru64(rdi)) as u64; rax = ((rax).wrapping_add(r8)) as u64; r8 = 0; fl_l = 0; fl_r = 0; bb = 8; continue; }
            8 => { crate::simd_ops::wu32(rax, (0xff01ffffu32) as u32); crate::simd_ops::wu32(rax.wrapping_add(0x4u64), ((r8 as u32)) as u32); crate::simd_ops::wu32(rax.wrapping_add(0x8u64), (0xffffffffu32) as u32); rax = rax.wrapping_add(0xc); fl_l = rcx; fl_r = (0x1u64) as u64; rcx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 8; continue; } bb = 9; continue; }
            9 => { crate::simd_ops::wu32(rdi.wrapping_add(0x8u64), ((rbx as u32)) as u32); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            10 => { if (((fl_l as u64)) == ((fl_r as u64))) { bb = 12; continue; } bb = 11; continue; }
            11 => { crate::simd_ops::wu32(rcx.wrapping_add(0x8u64), ((rbx as u32)) as u32); bb = 12; continue; }
            12 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1412f89d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x140a5b830; const P1: u64 = 0x140fff1e0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rsi = r8; r8 = rdx; rbp = rcx; rbx = (crate::simd_ops::ru64(rcx.wrapping_add(0x58u64))) as u64; fl_l = rdx; fl_r = rbx; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rcx = ((rbx.wrapping_add(rdx)) as u64) as u64; rdx = 0; fl_l = 0; fl_r = 0; rax = rcx.wrapping_sub(0x1); let d = (rbx) as u128; let n = ((rdx as u128) << 64) | (rax as u128); if d != 0 { rax = (n / d) as u64; rdx = (n % d) as u64; } rbx = rcx.wrapping_sub(0x1); fl_l = rbx; fl_r = rdx; rbx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; bb = 2; continue; }
            2 => { crate::simd_ops::wu64(rbp.wrapping_add(0x90u64), r8); r14 = rbp.wrapping_add(0x98); rdi = (crate::simd_ops::ru32(r14.wrapping_add(0x8u64))) as u32 as u64; rax = (crate::simd_ops::ru32(r14.wrapping_add(0xcu64))) as u32 as u64; fl_l = rbx; fl_r = rax; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 7; continue; } bb = 3; continue; }
            3 => { rdx = rbx; rcx = r14; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru32(r14.wrapping_add(0x8u64))) as u32 as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rax = ((rax).wrapping_add(crate::simd_ops::ru64(r14))) as u64; rcx = rbx; fl_l = rcx; fl_r = rdi; rcx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 13; continue; } bb = 4; continue; }
            4 => { bb = 5; continue; }
            5 => { xmm0 = (crate::simd_ops::ru128(rsi)) as u128; crate::simd_ops::wu128(rax, (xmm0) as u128); rax = rax.wrapping_add(0x10); fl_l = rcx; fl_r = (0x1u64) as u64; rcx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 5; continue; } bb = 6; continue; }
            6 => { bb = 13; continue; }
            7 => { fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 12; continue; } bb = 8; continue; }
            8 => { rax = rdi; rax = ((rax).wrapping_shl(4u32)) as u64; rax = ((rax).wrapping_add(crate::simd_ops::ru64(r14))) as u64; rcx = rbx; fl_l = rcx; fl_r = rdi; rcx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 13; continue; } bb = 9; continue; }
            9 => { bb = 10; continue; }
            10 => { xmm0 = (crate::simd_ops::ru128(rsi)) as u128; crate::simd_ops::wu128(rax, (xmm0) as u128); rax = rax.wrapping_add(0x10); fl_l = rcx; fl_r = (0x1u64) as u64; rcx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 10; continue; } bb = 11; continue; }
            11 => { bb = 13; continue; }
            12 => { if (((fl_l as u64)) == ((fl_r as u64))) { bb = 14; continue; } bb = 13; continue; }
            13 => { crate::simd_ops::wu32(r14.wrapping_add(0x8u64), ((rbx as u32)) as u32); rdi = ((rbx as u32)) as u32 as u64; bb = 14; continue; }
            14 => { rdx = ((rdi as u32)) as u32 as u64; r8 = (crate::simd_ops::ru32(r14.wrapping_add(0xcu64))) as u32 as u64; r8 = ((r8).wrapping_shl(4u32)) as u64; rax = ((rdi as u32)) as u32 as u64; rax = ((rax).wrapping_shl(4u32)) as u64; fl_l = rax; fl_r = r8; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 16; continue; } bb = 15; continue; }
            15 => { rcx = r14; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 16; continue; }
            16 => { rcx = rbp.wrapping_add(0xa8); rdx = rbx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1412f8d90(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xc; const P1: u64 = 0x8; const P2: u64 = 0x1401deba0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rcx;
    rcx = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64;
    rcx = ((rcx).wrapping_shl(3u32)) as u64;
    rax = (crate::simd_ops::ru32(rbx.wrapping_add(P1))) as u32 as u64;
    rax = ((rax).wrapping_shl(3u32)) as u64;
    fl_l = rax; fl_r = rcx;
    if !((((fl_l as u64)) != ((fl_r as u64)))) { rax = ((((rax) as i64).wrapping_shr(3u32) as u64)) as u64; rax = ((rax).wrapping_add(1)) as u64; rcx = ((((rcx) as i64).wrapping_shr(3u32) as u64)) as u64; rdx = rcx; rdx = ((rdx).wrapping_shr(1u32)) as u64; rdx = ((rdx).wrapping_add(rcx)) as u64; fl_l = rdx; fl_r = rax; rdx = (if (((fl_l as u64)) < ((fl_r as u64))) { rax } else { rdx }) as u64; rcx = rbx; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); }
    rcx = (crate::simd_ops::ru32(rbx.wrapping_add(P1))) as u32 as u64;
    rax = (crate::simd_ops::ru64(rbx)) as u64;
    rdx = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu64(rax.wrapping_add(rcx.wrapping_mul(8)), rdx);
    crate::simd_ops::lk_add_u32(rbx.wrapping_add(P1), (1) as u32);
    rcx = (crate::simd_ops::ru32(rbx.wrapping_add(P1))) as u32 as u64;
    rax = (crate::simd_ops::ru64(rbx)) as u64;
    rax = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64;
    rax = ((rax).wrapping_add(0xfffffffffffffff8u64)) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1412f8f30(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0xc; const P2: u64 = 0x140a5b830; const P3: u64 = 0x10;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rdx; rdi = rcx; rsi = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; rcx = ((rsi as u32)) as u32 as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rax = (crate::simd_ops::ru32(rdi.wrapping_add(P1))) as u32 as u64; fl_l = rdx; fl_r = rax; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 5; continue; } bb = 1; continue; }
            1 => { rcx = rdi; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rdx = rbx; fl_l = rdx; fl_r = rsi; rdx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 11; continue; } bb = 2; continue; }
            2 => { rcx = (crate::simd_ops::ru32(rdi.wrapping_add(P0))) as u32 as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rdi))) as u64; rax = 0; fl_l = 0; fl_r = 0; bb = 3; continue; }
            3 => { crate::simd_ops::wu64(rcx, rax); crate::simd_ops::wu64(rcx.wrapping_add(P0), rax); rcx = ((rcx.wrapping_add(P3)) as u64) as u64; fl_l = rdx; fl_r = (0x1u64) as u64; rdx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } bb = 4; continue; }
            4 => { crate::simd_ops::wu32(rdi.wrapping_add(P0), ((rbx as u32)) as u32); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            5 => { fl_l = rbx; fl_r = rsi; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 10; continue; } bb = 6; continue; }
            6 => { fl_l = rdx; fl_r = rsi; rdx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 11; continue; } bb = 7; continue; }
            7 => { rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rdi))) as u64; rax = 0; fl_l = 0; fl_r = 0; bb = 8; continue; }
            8 => { crate::simd_ops::wu64(rcx, rax); crate::simd_ops::wu64(rcx.wrapping_add(P0), rax); rcx = ((rcx.wrapping_add(P3)) as u64) as u64; fl_l = rdx; fl_r = (0x1u64) as u64; rdx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 8; continue; } bb = 9; continue; }
            9 => { crate::simd_ops::wu32(rdi.wrapping_add(P0), ((rbx as u32)) as u32); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            10 => { if (((fl_l as u64)) == ((fl_r as u64))) { bb = 12; continue; } bb = 11; continue; }
            11 => { crate::simd_ops::wu32(rdi.wrapping_add(P0), ((rbx as u32)) as u32); bb = 12; continue; }
            12 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1412f9440(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0xffffffffffffffff; const P3: u64 = 0x1424069a0; const P4: u64 = 0x143868298; const P5: u64 = 0x143426a48; const P6: u64 = 0x14209d670; const P7: u64 = 0x142075af0; const P8: u64 = 0x50;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut xmm6: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 15>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x70u64)) as u64; rdx = (((rdx) as i64).wrapping_mul((crate::simd_ops::ru64(r8.wrapping_add(P0))) as i64) as u64) as u64; crate::simd_ops::wu128(rsp.wrapping_add(0x60u64), (xmm6) as u128); rdi = (crate::simd_ops::ru64(r8.wrapping_add(P1))) as u64; rbp = r9; rsi = rcx; fl_l = rdx; fl_r = rdi; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rcx = ((rdi.wrapping_add(rdx)) as u64) as u64; rdx = 0; fl_l = 0; fl_r = 0; rax = ((rcx.wrapping_add(P2)) as u64) as u64; let d = (rdi) as u128; let n = ((rdx as u128) << 64) | (rax as u128); if d != 0 { rax = (n / d) as u64; rdx = (n % d) as u64; } rdi = ((rcx.wrapping_add(P2)) as u64) as u64; fl_l = rdi; fl_r = rdx; rdi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; bb = 2; continue; }
            2 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rsi = (crate::simd_ops::ru64(rsi)) as u64; xmm6 = (xmm0) as u128; rax = (crate::simd_ops::ru64(rsi.wrapping_add(P1))) as u64; fl_l = rax; fl_r = rdi; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 6; continue; } bb = 3; continue; }
            3 => { fl_l = (crate::simd_ops::ru8(P4)) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 9; continue; } bb = 4; continue; }
            4 => { fl_l = rax; fl_r = rdi; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 9; continue; } bb = 5; continue; }
            5 => { xmm1 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm0 as u64) - f64::from_bits(crate::simd_ops::ru64(rbp))).to_bits() as u128); fl_l = ((xmm1 as u64)) as u64; fl_r = (crate::simd_ops::ru64(P5)) as u64; if !(f64::from_bits(fl_l) > f64::from_bits(fl_r)) { bb = 9; continue; } bb = 6; continue; }
            6 => { rcx = rsi; crate::simd_ops::wu64(rsp.wrapping_add(0x80u64), rbx); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rsp.wrapping_add(0x40u64), (0x1u32) as u32); r9 = ((rbx as u32)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x44u64), (0x10u32) as u32); rax = rsp.wrapping_add(0x40); bb = 7; continue; }
            7 => { r9 = ((((r9 as u32)) | (crate::simd_ops::ru32(rax)))) as u32 as u64; rcx = rsp.wrapping_add(0x48); rax = ((rax).wrapping_add(0x4u64)) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 7; continue; } bb = 8; continue; }
            8 => { crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rsi); r8 = 0; fl_l = 0; fl_r = 0; rdx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rbx as u32)) as u32); rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = rdi; fl_r = (0x400000u64) as u64; rbx = (rbx & !0xffu64) | (((if (((fl_l as u64)) > ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); fl_l = (((rbx as u32)).wrapping_add(0x2u32)) as u32 as u64; fl_r = fl_l; rbx = ((fl_l as u32)) as u32 as u64; crate::simd_ops::wu32(rsi.wrapping_add(P8), ((rbx as u32)) as u32); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; crate::simd_ops::wu64(rbp, ((xmm6 as u64)) as u64); bb = 9; continue; }
            9 => { xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x60u64))) as u128; r11 = rsp.wrapping_add(0x70); rbp = (crate::simd_ops::ru64(r11.wrapping_add(0x18u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x20u64))) as u64; rsp = r11; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1412f9ba0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0xc; const P2: u64 = 0x1411dba80; const P3: u64 = 0x10; const P4: u64 = 0xffffffffffffffe8; const P5: u64 = 0xfffffffffffffff0; const P6: u64 = 0xfffffffffffffff8;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = r9; rsi = r8; rbp = rdx; rbx = rcx; rdx = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(P1))) as u32 as u64; r10 = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; r10 = ((r10).wrapping_shl(3u32)) as u64; rax = ((rdx.wrapping_add(rdx.wrapping_mul(2))) as u64) as u64; rax = ((rax).wrapping_shl(3u32)) as u64; fl_l = rax; fl_r = r10; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rdx = ((rdx).wrapping_add(1)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(P0))) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = (crate::simd_ops::ru64(rbx)) as u64; xmm0 = 0u128; rdx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu128(rax.wrapping_add(rcx.wrapping_mul(8)), (xmm0) as u128); crate::simd_ops::wu64(rax.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(P3), rdx); crate::simd_ops::lk_add_u32(rbx.wrapping_add(P0), (1) as u32); rax = (crate::simd_ops::ru32(rbx.wrapping_add(P0))) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = (crate::simd_ops::ru64(rbx)) as u64; crate::simd_ops::wu64(rax.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(P4), rbp); crate::simd_ops::wu64(rax.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(P5), rsi); crate::simd_ops::wu64(rax.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(P6), rdi); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1412f9c30(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14229d920; const P1: u64 = 0x1418350b4;
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
    if !(((((fl_l as u8)) & ((fl_r as u8))) == 0)) { rdx = 0x6c0; r8 = 0x40; rcx = rdi; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); }
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rax = rdi;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1412f9c80(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1433f5b80; const P1: u64 = 0x14220ea70; const P2: u64 = 0xc; const P3: u64 = 0x10; const P4: u64 = 0x8; const P5: u64 = 0x18; const P6: u64 = 0x20; const P7: u64 = 0x28; const P8: u64 = 0x30; const P9: u64 = 0x38; const P10: u64 = 0x40; const P11: u64 = 0x48;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x30u64)) as u64;
    rbx = rcx;
    rdi = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu64(rcx, rdi);
    rcx = ((rcx).wrapping_add(0x8u64)) as u64;
    rax = P0;
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax);
    crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (0xcu64) as u64);
    xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128;
    crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128);
    rdx = rsp.wrapping_add(0x20);
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    crate::simd_ops::wu32(rbx.wrapping_add(P2), (0x9u32) as u32);
    rax = ((rbx.wrapping_add(P3)) as u64) as u64;
    crate::simd_ops::wu64(rbx, rax);
    crate::simd_ops::wu64(rax, rdi);
    crate::simd_ops::wu64(rax.wrapping_add(P4), rdi);
    crate::simd_ops::wu64(rax.wrapping_add(P3), rdi);
    crate::simd_ops::wu64(rax.wrapping_add(P5), rdi);
    crate::simd_ops::wu64(rax.wrapping_add(P6), rdi);
    crate::simd_ops::wu64(rax.wrapping_add(P7), rdi);
    crate::simd_ops::wu64(rax.wrapping_add(P8), rdi);
    crate::simd_ops::wu64(rax.wrapping_add(P9), rdi);
    crate::simd_ops::wu64(rax.wrapping_add(P10), rdi);
    crate::simd_ops::wu64(rax.wrapping_add(P11), rdi);
    rax = rbx;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64;
    rsp = ((rsp).wrapping_add(0x30u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1412f9ef0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14330bca8; const P1: u64 = 0x142442de0;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    rcx = (crate::simd_ops::ru64(rcx)) as u64;
    fl_l = rcx; fl_r = rcx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rax = P0; crate::simd_ops::wu64(rcx, rax); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); }
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1412f9ff0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141309b70; const P1: u64 = 0x141309e20; const P2: u64 = 0x14130a0d0; const P3: u64 = 0x141309a00; const P4: u64 = 0x142442de0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = (crate::simd_ops::ru64(rcx)) as u64;
    crate::simd_ops::wu64(rcx, rdx);
    fl_l = rbx; fl_r = rbx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rcx = rbx; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); }
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1412fa030(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141309b70; const P1: u64 = 0x141309e20; const P2: u64 = 0x14130a0d0; const P3: u64 = 0x141309a00; const P4: u64 = 0x142442de0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = (crate::simd_ops::ru64(rcx)) as u64;
    fl_l = rbx; fl_r = rbx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rcx = rbx; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); }
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1412fa0d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x14209d670; const P2: u64 = 0x58; const P3: u64 = 0x1411e7fc0; const P4: u64 = 0x142442de0; const P5: u64 = 0x8;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = (crate::simd_ops::ru64(rcx)) as u64; crate::simd_ops::wu64(rcx, rdx); fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 1; continue; }
            1 => { rdi = (crate::simd_ops::ru64(rbx.wrapping_add(P0))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { rcx = rdi; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rcx = ((rdi.wrapping_add(P2)) as u64) as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rcx = rdi; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); bb = 3; continue; }
            3 => { rdi = (crate::simd_ops::ru64(rbx.wrapping_add(P5))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rcx = rdi; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rcx = ((rdi.wrapping_add(P2)) as u64) as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rcx = rdi; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); bb = 5; continue; }
            5 => { rcx = rbx; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); bb = 6; continue; }
            6 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1412fa150(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x14209d670; const P2: u64 = 0x58; const P3: u64 = 0x1411e7fc0; const P4: u64 = 0x142442de0; const P5: u64 = 0x8;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = (crate::simd_ops::ru64(rcx)) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 1; continue; }
            1 => { rdi = (crate::simd_ops::ru64(rbx.wrapping_add(P0))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { rcx = rdi; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rcx = ((rdi.wrapping_add(P2)) as u64) as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rcx = rdi; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); bb = 3; continue; }
            3 => { rdi = (crate::simd_ops::ru64(rbx.wrapping_add(P5))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rcx = rdi; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rcx = ((rdi.wrapping_add(P2)) as u64) as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rcx = rdi; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); bb = 5; continue; }
            5 => { rcx = rbx; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); bb = 6; continue; }
            6 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1412fa1c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x20; const P1: u64 = 0x14397ff18; const P2: u64 = 0x142258080; const P3: u64 = 0x18; const P4: u64 = 0x10; const P5: u64 = 0x14204f860; const P6: u64 = 0x142442de0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = (crate::simd_ops::ru64(rcx)) as u64; crate::simd_ops::wu64(rcx, rdx); fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 1; continue; }
            1 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P0))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { rcx = (crate::simd_ops::ru64(P1)) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 3; continue; }
            3 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P3))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rcx = (crate::simd_ops::ru64(P1)) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 5; continue; }
            5 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P4))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 6; continue; }
            6 => { rcx = (crate::simd_ops::ru64(P1)) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 7; continue; }
            7 => { rdi = (crate::simd_ops::ru64(rbx)) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 8; continue; }
            8 => { rcx = rdi; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rcx = rdi; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); bb = 9; continue; }
            9 => { rcx = rbx; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); bb = 10; continue; }
            10 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1412fa250(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x20; const P1: u64 = 0x14397ff18; const P2: u64 = 0x142258080; const P3: u64 = 0x18; const P4: u64 = 0x10; const P5: u64 = 0x14204f860; const P6: u64 = 0x142442de0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = (crate::simd_ops::ru64(rcx)) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 1; continue; }
            1 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P0))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { rcx = (crate::simd_ops::ru64(P1)) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 3; continue; }
            3 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P3))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rcx = (crate::simd_ops::ru64(P1)) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 5; continue; }
            5 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P4))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 6; continue; }
            6 => { rcx = (crate::simd_ops::ru64(P1)) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 7; continue; }
            7 => { rdi = (crate::simd_ops::ru64(rbx)) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 8; continue; }
            8 => { rcx = rdi; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rcx = rdi; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); bb = 9; continue; }
            9 => { rcx = rbx; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); bb = 10; continue; }
            10 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1412fa2f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1422b57f0; const P1: u64 = 0x8; const P2: u64 = 0x1413087a0; const P3: u64 = 0x10; const P4: u64 = 0x18; const P5: u64 = 0x14397ff68; const P6: u64 = 0x1413086e0; const P7: u64 = 0x142442de0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = (crate::simd_ops::ru64(rcx)) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rcx = rdi; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rdx = (crate::simd_ops::ru64(rdi)) as u64; r8 = (crate::simd_ops::ru32(rdi.wrapping_add(P1))) as u32 as u64; r8 = ((r8).wrapping_shl(5u32)) as u64; r8 = ((r8).wrapping_add(rdx)) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu32(rdi.wrapping_add(P1), (0x0u32) as u32); rdx = (crate::simd_ops::ru64(rdi.wrapping_add(P3))) as u64; r8 = (crate::simd_ops::ru32(rdi.wrapping_add(P4))) as u32 as u64; r8 = ((r8).wrapping_shl(5u32)) as u64; r8 = ((r8).wrapping_add(rdx)) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu32(rdi.wrapping_add(P4), (0x0u32) as u32); crate::simd_ops::wu64(P5, (0x0u64) as u64); rcx = ((rdi.wrapping_add(P3)) as u64) as u64; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); rcx = rdi; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); rcx = rdi; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); bb = 2; continue; }
            2 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1412fa380(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1422b1be0; const P1: u64 = 0x143a367f8; const P2: u64 = 0x58; const P3: u64 = 0x1401de830; const P4: u64 = 0x48; const P5: u64 = 0x30; const P6: u64 = 0x1413086e0; const P7: u64 = 0x18; const P8: u64 = 0x142442de0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = (crate::simd_ops::ru64(rcx)) as u64;
    crate::simd_ops::wu64(rcx, rdx);
    fl_l = rbx; fl_r = rbx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rcx = rbx; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(P1, (0x0u64) as u64); rcx = ((rbx.wrapping_add(P2)) as u64) as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rcx = ((rbx.wrapping_add(P4)) as u64) as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rcx = ((rbx.wrapping_add(P5)) as u64) as u64; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); rcx = ((rbx.wrapping_add(P7)) as u64) as u64; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); rcx = rbx; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); rcx = rbx; rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); }
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1412fa3f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1422b1be0; const P1: u64 = 0x143a367f8; const P2: u64 = 0x58; const P3: u64 = 0x1401de830; const P4: u64 = 0x48; const P5: u64 = 0x30; const P6: u64 = 0x1413086e0; const P7: u64 = 0x18; const P8: u64 = 0x142442de0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = (crate::simd_ops::ru64(rcx)) as u64;
    fl_l = rbx; fl_r = rbx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rcx = rbx; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(P1, (0x0u64) as u64); rcx = ((rbx.wrapping_add(P2)) as u64) as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rcx = ((rbx.wrapping_add(P4)) as u64) as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rcx = ((rbx.wrapping_add(P5)) as u64) as u64; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); rcx = ((rbx.wrapping_add(P7)) as u64) as u64; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); rcx = rbx; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); rcx = rbx; rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); }
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1412fa530(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x14204f860; const P2: u64 = 0x142442de0; const P3: u64 = 0x8;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = (crate::simd_ops::ru64(rcx)) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 1; continue; }
            1 => { rdi = (crate::simd_ops::ru64(rbx.wrapping_add(P0))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { rcx = rdi; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rcx = rdi; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 3; continue; }
            3 => { rdi = (crate::simd_ops::ru64(rbx.wrapping_add(P3))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rcx = rdi; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rcx = rdi; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 5; continue; }
            5 => { rdi = (crate::simd_ops::ru64(rbx)) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 6; continue; }
            6 => { rcx = rdi; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rcx = rdi; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 7; continue; }
            7 => { rcx = rbx; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 8; continue; }
            8 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1412fa630(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14397fee8; const P1: u64 = 0x14397fee0; const P2: u64 = 0x8; const P3: u64 = 0x1412ed110; const P4: u64 = 0x142442de0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = (crate::simd_ops::ru64(rcx)) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 1; continue; }
            1 => { rcx = (crate::simd_ops::ru32(rbx)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 4; continue; } bb = 2; continue; }
            2 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 5; continue; } bb = 3; continue; }
            3 => { rax = 0; fl_l = 0; fl_r = 0; let tmp_xchg: u64 = crate::simd_ops::atomic_xchg_u64(P0, rax); rax = (tmp_xchg) as u64; bb = 5; continue; }
            4 => { rax = 0; fl_l = 0; fl_r = 0; let tmp_xchg: u64 = crate::simd_ops::atomic_xchg_u64(P1, rax); rax = (tmp_xchg) as u64; bb = 5; continue; }
            5 => { rcx = ((rbx.wrapping_add(P2)) as u64) as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rcx = rbx; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); bb = 6; continue; }
            6 => { rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1412fa6b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143a00230; const P1: u64 = 0x100; const P2: u64 = 0x1401de830; const P3: u64 = 0xf0; const P4: u64 = 0x14136c160; const P5: u64 = 0x142442de0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = (crate::simd_ops::ru64(rcx)) as u64;
    fl_l = rbx; fl_r = rbx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { crate::simd_ops::wu64(P0, (0x0u64) as u64); rcx = ((rbx.wrapping_add(P1)) as u64) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rcx = ((rbx.wrapping_add(P3)) as u64) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rcx = rbx; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rcx = rbx; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); }
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1412fa710(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14397fed8; const P1: u64 = 0x28; const P2: u64 = 0x1403b8050; const P3: u64 = 0x10; const P4: u64 = 0x1401de830; const P5: u64 = 0x8; const P6: u64 = 0x1412ec900; const P7: u64 = 0x1401e66f0; const P8: u64 = 0x142442de0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = (crate::simd_ops::ru64(rcx)) as u64;
    fl_l = rbx; fl_r = rbx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { crate::simd_ops::wu64(P0, (0x0u64) as u64); rcx = ((rbx.wrapping_add(P1)) as u64) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rcx = ((rbx.wrapping_add(P3)) as u64) as u64; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); r8 = (crate::simd_ops::ru64(rbx)) as u64; r8 = (crate::simd_ops::ru64(r8.wrapping_add(P5))) as u64; rdx = rbx; rcx = rbx; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); rdx = (crate::simd_ops::ru64(rbx)) as u64; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); rcx = rbx; rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); }
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1412fa840(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143a00218; const P1: u64 = 0x14209d670; const P2: u64 = 0x58; const P3: u64 = 0x1411e7fc0; const P4: u64 = 0x142442de0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = (crate::simd_ops::ru64(rcx)) as u64;
    fl_l = rbx; fl_r = rbx;
    if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    crate::simd_ops::wu64(P0, (0x0u64) as u64);
    rdi = (crate::simd_ops::ru64(rbx)) as u64;
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
pub unsafe extern "win64" fn fn_1412fa8a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143a00220; const P1: u64 = 0x28; const P2: u64 = 0x14397ff18; const P3: u64 = 0x142258080; const P4: u64 = 0x20; const P5: u64 = 0x142442de0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = (crate::simd_ops::ru64(rcx)) as u64;
    fl_l = rbx; fl_r = rbx;
    if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    crate::simd_ops::wu64(P0, (0x0u64) as u64);
    rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P1))) as u64;
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
pub unsafe extern "win64" fn fn_1412fa9e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1412cf610; const P1: u64 = 0xffffffffffffffff; const P2: u64 = 0x141834ee8; const P3: u64 = 0x142442de0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = (crate::simd_ops::ru64(rcx)) as u64;
    fl_l = rbx; fl_r = rbx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { r9 = P0; rdx = 0x8; r8 = ((rdx.wrapping_add(P1)) as u32) as u32 as u64; rcx = rbx; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rcx = rbx; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); }
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1412faa20(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14397fea0; const P1: u64 = 0x8; const P2: u64 = 0x1412dcde0; const P3: u64 = 0x142442de0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = (crate::simd_ops::ru64(rcx)) as u64;
    fl_l = rbx; fl_r = rbx;
    if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    crate::simd_ops::wu64(P0, (0x0u64) as u64);
    rcx = ((rbx.wrapping_add(P1)) as u64) as u64;
    rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp);
    rcx = (crate::simd_ops::ru64(rbx)) as u64;
    fl_l = rcx; fl_r = rcx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); }
    rcx = rbx;
    rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1412faaa0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14397ff88; const P1: u64 = 0x18; const P2: u64 = 0x14204db80; const P3: u64 = 0x142442de0; const P4: u64 = 0x10; const P5: u64 = 0x141303e40; const P6: u64 = 0x1411d3de0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = (crate::simd_ops::ru64(rcx)) as u64; crate::simd_ops::wu64(rcx, rdx); fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(P0, (0x0u64) as u64); rdi = (crate::simd_ops::ru64(rbx.wrapping_add(P1))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { rcx = rdi; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rcx = rdi; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 3; continue; }
            3 => { rdi = (crate::simd_ops::ru64(rbx.wrapping_add(P4))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rcx = rdi; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rcx = rdi; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 5; continue; }
            5 => { rcx = rbx; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); rcx = rbx; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 6; continue; }
            6 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1412fab20(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14397ff88; const P1: u64 = 0x18; const P2: u64 = 0x14204db80; const P3: u64 = 0x142442de0; const P4: u64 = 0x10; const P5: u64 = 0x141303e40; const P6: u64 = 0x1411d3de0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = (crate::simd_ops::ru64(rcx)) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(P0, (0x0u64) as u64); rdi = (crate::simd_ops::ru64(rbx.wrapping_add(P1))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { rcx = rdi; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rcx = rdi; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 3; continue; }
            3 => { rdi = (crate::simd_ops::ru64(rbx.wrapping_add(P4))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rcx = rdi; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rcx = rdi; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 5; continue; }
            5 => { rcx = rbx; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); rcx = rbx; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 6; continue; }
            6 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1412fab90(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143a00200; const P1: u64 = 0x20; const P2: u64 = 0x1401de830; const P3: u64 = 0x10; const P4: u64 = 0x142442de0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = (crate::simd_ops::ru64(rcx)) as u64;
    crate::simd_ops::wu64(rcx, rdx);
    fl_l = rbx; fl_r = rbx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { crate::simd_ops::wu64(P0, (0x0u64) as u64); rcx = ((rbx.wrapping_add(P1)) as u64) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rcx = ((rbx.wrapping_add(P3)) as u64) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rcx = rbx; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); }
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1412fabd0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143a00200; const P1: u64 = 0x20; const P2: u64 = 0x1401de830; const P3: u64 = 0x10; const P4: u64 = 0x142442de0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = (crate::simd_ops::ru64(rcx)) as u64;
    fl_l = rbx; fl_r = rbx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { crate::simd_ops::wu64(P0, (0x0u64) as u64); rcx = ((rbx.wrapping_add(P1)) as u64) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rcx = ((rbx.wrapping_add(P3)) as u64) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rcx = rbx; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); }
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1412fac30(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x28; const P1: u64 = 0x14209d670; const P2: u64 = 0x58; const P3: u64 = 0x1411e7fc0; const P4: u64 = 0x142442de0; const P5: u64 = 0x10; const P6: u64 = 0x1401de830;
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
pub unsafe extern "win64" fn fn_1412fad00(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14206fa90; const P1: u64 = 0x142442de0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = (crate::simd_ops::ru64(rcx)) as u64;
    fl_l = rbx; fl_r = rbx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx, (0x0u64) as u64); rcx = rbx; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); }
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1412fad30(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = ((rcx.wrapping_add(rdx.wrapping_mul(4))) as u64) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1412fad60(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x38; const P1: u64 = 0x10; const P2: u64 = 0x18; const P3: u64 = 0xffffffffffffffe0; const P4: u64 = 0x142535070; const P5: u64 = 0x1426456c8; const P6: u64 = 0x1426453a0; const P7: u64 = 0x8; const P8: u64 = 0xc; const P9: u64 = 0x20; const P10: u64 = 0x28;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rax = ((crate::exe::tls_pointer() as u64)) as u64; rbx = rcx; rcx = 0x6348; rdi = 0; fl_l = 0; fl_r = 0; rdx = (crate::simd_ops::ru64(rax)) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(rdx))) as u64; r8 = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rdx = (crate::simd_ops::ru64(r8.wrapping_add(P1))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { fl_l = ((rdx as u8)) as u8 as u64; fl_r = (0xfu8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::lk_add_u32(r8.wrapping_add(P2), (1) as u32)) as u32 as u64; rax = (crate::simd_ops::ru64(rdx)) as u64; crate::simd_ops::wu64(r8.wrapping_add(P1), rax); bb = 5; continue; }
            3 => { rdx = 0x30; r8 = ((rdx.wrapping_add(P3)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rdx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp); rcx = rax; rdx = 0xdeadd00d; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P6), rax), rax, rcx, rdx, r8, r9, rsp); rdx = rdi; bb = 5; continue; }
            5 => { crate::simd_ops::wu32(rdx.wrapping_add(P7), (0xffffffffu32) as u32); rax = rbx; crate::simd_ops::wu32(rdx.wrapping_add(P8), ((rdi as u32)) as u32); crate::simd_ops::wu64(rdx, rdi); crate::simd_ops::wu64(rdx.wrapping_add(P1), rdi); crate::simd_ops::wu64(rdx.wrapping_add(P2), rdi); crate::simd_ops::wu64(rdx.wrapping_add(P9), rdi); crate::simd_ops::wu64(rdx.wrapping_add(P10), rdi); crate::simd_ops::wu64(rbx, rdx); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1412fae00(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143a36728; const P1: u64 = 0x1418349c0; const P2: u64 = 0x1411dba80; const P3: u64 = 0x143879610; const P4: u64 = 0x1425371a0; const P5: u64 = 0x142533200;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = rsp; crate::simd_ops::wu64(rax.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rax.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rax.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rax.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; xmm0 = 0u128; crate::simd_ops::wu128(rax.wrapping_sub(0x28u64), (xmm0) as u128); r12 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rax.wrapping_sub(0x28u64), r12); crate::simd_ops::wu64(rax.wrapping_sub(0x20u64), r12); r15 = (crate::simd_ops::ru64(P0)) as u64; rax = ((crate::exe::tls_pointer() as u64)) as u64; rbx = (crate::simd_ops::ru64(rax)) as u64; rbp = 0x70; rbp = ((rbp).wrapping_add(rbx)) as u64; fl_l = (crate::simd_ops::ru8(rbp)) as u8 as u64; fl_r = ((r12 as u8)) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 2; continue; } bb = 1; continue; }
            1 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 2; continue; }
            2 => { r14 = 0x6338; r14 = ((r14).wrapping_add(rbx)) as u64; rbx = (crate::simd_ops::ru64(r14)) as u64; rdx = 0x1; rcx = rsp.wrapping_add(0x20); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = (crate::simd_ops::ru32(rsp.wrapping_add(0x28u64))) as u32 as u64; rcx = ((rdx.wrapping_add(rdx.wrapping_mul(2))) as u64) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; xmm0 = 0u128; rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu128(rsi.wrapping_add(rcx.wrapping_mul(8)), (xmm0) as u128); crate::simd_ops::wu64(rsi.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x10u64), rax); rdx = (((rdx as u32)).wrapping_add(1)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((rdx as u32)) as u32); rax = ((rdx.wrapping_add(rdx.wrapping_mul(2))) as u64) as u64; rdi = ((rsi.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rax = P3; crate::simd_ops::wu64(rdi.wrapping_sub(0x18u64), rax); crate::simd_ops::wu64(rdi.wrapping_sub(0x10u64), r15); crate::simd_ops::wu64(rdi.wrapping_sub(0x8u64), rbx); fl_l = (crate::simd_ops::ru8(rbp)) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 4; continue; } bb = 3; continue; }
            3 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 4; continue; }
            4 => { crate::simd_ops::wu64(r14, r12); rbx = rsi; fl_l = rsi; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 7; continue; } bb = 5; continue; }
            5 => { bb = 6; continue; }
            6 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; r8 = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = ((rbx).wrapping_add(0x18u64)) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 6; continue; } bb = 7; continue; }
            7 => { fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 15; continue; } bb = 8; continue; }
            8 => { r10 = rsi.wrapping_sub(0x1); r10 = (((r10) & (0xfffffffffe000000u64))) as u64; rax = ((crate::exe::teb_self() as u64)) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(r10.wrapping_add(0x100u64))) as u64; rdx = (rdx & !0xffu64) | (((if (((fl_l as u64)) == ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); r9 = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; rax = r9; rax = ((rax).wrapping_sub(r10)) as u64; rax = ((rax).wrapping_shr(16u32)) as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rcx = r10.wrapping_add(0x108); rcx = ((rcx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x4u64))) as u32 as u64; rcx = ((rcx).wrapping_sub(rax)) as u64; fl_l = ((rdx as u8)) as u8 as u64; fl_r = ((rdx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 13; continue; } bb = 9; continue; }
            9 => { fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0xeu64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 12; continue; } bb = 10; continue; }
            10 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(r9, rax); crate::simd_ops::wu64(rcx.wrapping_add(0x20u64), r9); fl_l = (crate::simd_ops::lk_add_u32(rcx.wrapping_add(0x18u64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; if (((fl_l as u32)) != 0) { bb = 15; continue; } bb = 11; continue; }
            11 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 15; continue; }
            12 => { r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 14; continue; }
            13 => { r8 = 0; fl_l = 0; fl_r = 0; bb = 14; continue; }
            14 => { rdx = rcx; rcx = r10; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 15; continue; }
            15 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}
